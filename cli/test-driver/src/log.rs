//! Lightweight logging layer that multiplexes progress reporting across a CI
//! renderer and an interactive TUI renderer.

use anyhow::Result;
use colored::Colorize;
use hax_types::cli_options::BackendName;
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    collections::HashMap,
    fmt::Display,
    future::Future,
    io::IsTerminal,
    sync::{Arc, LazyLock},
    time::{Duration, SystemTime},
};
use tokio::sync::{mpsc, oneshot};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
/// Job categorisations used by the renderer implementations.
pub enum JobKind {
    ResolveTests,
    BackendJob {
        kind: BackendJobKind,
        backend: BackendName,
    },
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
/// Backend-specific job classification.
pub enum BackendJobKind {
    CargoHaxSerialize,
    CargoHaxInto {
        test: String,
    },
    Verification {
        test: String,
    },
    /// A phase of the FLS suite that spans all tests of a backend, e.g. one `lake build` pass.
    Phase {
        name: String,
    },
    /// The number of backend jobs for a given backend. (each can report a CargoHaxSerialize, a CargoHaxInto and a Verification)
    NumberBackendJobs(usize),
}

#[derive(Clone, Debug)]
/// Individual payload sent through the logging channel.
pub enum ReportMessage {
    Start,
    Message(String),
    End(Arc<Result<()>>),
    /// The entire stdout a command produced. Display only if we are clueless about an error.
    Stdout(String),
    /// The entire stderr a command produced. Display only if we are clueless about an error.
    Stderr(String),
}

#[derive(Clone, Debug)]
/// A report for a job.
pub struct JobReport {
    pub job: JobKind,
    pub message: Option<ReportMessage>,
    pub timestamp: SystemTime,
}

impl Display for JobKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobKind::ResolveTests => write!(f, "discover tests"),
            JobKind::BackendJob { kind, backend } => match kind {
                BackendJobKind::CargoHaxSerialize => write!(f, "cargo: {backend}"),
                BackendJobKind::CargoHaxInto { test } => write!(f, "engine: {backend}/{test}"),
                BackendJobKind::Verification { test } => {
                    write!(f, "verification: {backend}: {test}")
                }
                BackendJobKind::Phase { name } => write!(f, "{backend}: {name}"),
                BackendJobKind::NumberBackendJobs(_) => Ok(()),
            },
        }
    }
}

impl JobKind {
    /// Sends a message to the logger.
    pub fn report(&self, message: ReportMessage) {
        send(JobReport {
            job: self.clone(),
            message: Some(message),
            timestamp: SystemTime::now(),
        })
    }
    /// Signals progress for a job that has no additional payload.
    pub fn report_no_message(&self) {
        send(JobReport {
            job: self.clone(),
            message: None,
            timestamp: SystemTime::now(),
        })
    }

    /// Sends a human readable message.
    pub fn report_message(&self, message: impl AsRef<str>) {
        self.report(ReportMessage::Message(message.as_ref().to_string()))
    }
    /// Runs an async job while taking care of reporting lifecycle messages.
    pub async fn run<R, F: Future<Output = Result<R>>>(
        self,
        f: impl Fn(JobKind) -> F,
    ) -> Result<R> {
        self.report(ReportMessage::Start);
        let result = f(self.clone()).await;
        self.report(ReportMessage::End(Arc::new(match &result {
            Err(err) => Err(anyhow::Error::msg(err.to_string())),
            _ => Ok(()),
        })));
        result
    }
}

trait ReportRenderer: Send {
    fn handle(&mut self, report: &JobReport);
    fn finish(&mut self);
}

enum RendererKind {
    Ci,
    Tui,
}

fn build_renderers() -> Vec<Box<dyn ReportRenderer + Send>> {
    let stdout_is_terminal = std::io::stdout().is_terminal();
    let kinds = detect_renderer_kinds(stdout_is_terminal);
    let mut renderers: Vec<Box<dyn ReportRenderer + Send>> = Vec::new();

    for kind in kinds {
        match kind {
            RendererKind::Ci => renderers.push(Box::new(CiRenderer::new())),
            RendererKind::Tui => renderers.push(Box::new(TuiRenderer::new(stdout_is_terminal))),
        }
    }

    if renderers.is_empty() {
        renderers.push(Box::new(CiRenderer::new()));
    }

    renderers
}

fn detect_renderer_kinds(stdout_is_terminal: bool) -> Vec<RendererKind> {
    if let Some(from_env) = renderers_from_env()
        && !from_env.is_empty()
    {
        return from_env;
    }

    if std::env::var_os("CI").is_some() || !stdout_is_terminal {
        vec![RendererKind::Ci]
    } else {
        vec![RendererKind::Tui]
    }
}

fn renderers_from_env() -> Option<Vec<RendererKind>> {
    let value = std::env::var("HAX_REPORT_RENDERERS").ok()?;
    let parsed = parse_renderer_list(&value);
    if parsed.is_empty() {
        None
    } else {
        Some(parsed)
    }
}

fn parse_renderer_list(value: &str) -> Vec<RendererKind> {
    let mut kinds = Vec::new();

    for part in value.split(',') {
        let part = part.trim().to_ascii_lowercase();
        if part.is_empty() {
            continue;
        }

        match part.as_str() {
            "ci" => kinds.push(RendererKind::Ci),
            "tui" => kinds.push(RendererKind::Tui),
            "auto" => {}
            other => {
                eprintln!(
                    "Unknown renderer `{other}` in HAX_REPORT_RENDERERS; falling back to defaults"
                );
            }
        }
    }

    kinds
}

fn is_test_job(job: &JobKind) -> bool {
    matches!(
        job,
        JobKind::BackendJob {
            kind: BackendJobKind::Verification { .. } | BackendJobKind::CargoHaxInto { .. },
            ..
        }
    )
}

fn format_duration(duration: Duration) -> String {
    if duration.as_secs() >= 1 {
        format!("{:.2}s", duration.as_secs_f64())
    } else if duration.as_millis() >= 1 {
        format!("{}ms", duration.as_millis())
    } else if duration.as_micros() >= 1 {
        format!("{}us", duration.as_micros())
    } else {
        "0s".to_string()
    }
}

fn format_job_title(job: &JobKind) -> String {
    match job {
        JobKind::ResolveTests => "discover tests".to_string(),
        JobKind::BackendJob { kind, backend } => match kind {
            BackendJobKind::CargoHaxSerialize => format!("{backend} metadata"),
            BackendJobKind::CargoHaxInto { test } => format!("{backend}/{test}"),
            BackendJobKind::Verification { test } => {
                format!("verification {backend}/{test}")
            }
            BackendJobKind::Phase { name } => format!("{backend}: {name}"),
            BackendJobKind::NumberBackendJobs(_) => backend.to_string(),
        },
    }
}

struct CiRenderer {
    jobs: HashMap<JobKind, CiJobState>,
}

impl CiRenderer {
    fn new() -> Self {
        Self {
            jobs: HashMap::new(),
        }
    }

    fn state_entry(&mut self, job: &JobKind, timestamp: SystemTime) -> &mut CiJobState {
        self.jobs.entry(job.clone()).or_insert_with(|| CiJobState {
            started_at: timestamp,
            messages: Vec::new(),
            stdout: None,
            stderr: None,
        })
    }

    fn handle_meta(&mut self, report: &JobReport) {
        if let JobKind::BackendJob {
            kind: BackendJobKind::NumberBackendJobs(_),
            ..
        } = &report.job
        {
            // No-op for CI renderer; metadata is consumed by the TUI renderer.
        }
    }

    fn print_message(message: &str) {
        let mut lines = message.lines();
        if let Some(first) = lines.next() {
            println!("  → {first}");
        } else {
            println!("  →");
        }
        for line in lines {
            println!("    {line}");
        }
    }

    fn print_labeled_block(label: &str, content: &str) {
        println!("  {label}:");
        let mut lines = content.lines();
        if let Some(first) = lines.next() {
            println!("    → {first}");
        } else {
            println!("    →");
        }
        for line in lines {
            println!("      {line}");
        }
    }
}

struct CiJobState {
    started_at: SystemTime,
    messages: Vec<String>,
    stdout: Option<String>,
    stderr: Option<String>,
}

impl ReportRenderer for CiRenderer {
    fn handle(&mut self, report: &JobReport) {
        match report.message.as_ref() {
            Some(ReportMessage::Start) => {
                self.jobs.insert(
                    report.job.clone(),
                    CiJobState {
                        started_at: report.timestamp,
                        messages: Vec::new(),
                        stdout: None,
                        stderr: None,
                    },
                );
            }
            Some(ReportMessage::Message(message)) => {
                let state = self.state_entry(&report.job, report.timestamp);
                state.messages.push(message.clone());
            }
            Some(ReportMessage::Stdout(output)) => {
                let state = self.state_entry(&report.job, report.timestamp);
                state.stdout = Some(output.clone());
            }
            Some(ReportMessage::Stderr(output)) => {
                let state = self.state_entry(&report.job, report.timestamp);
                state.stderr = Some(output.clone());
            }
            Some(ReportMessage::End(result)) => {
                let state = self.jobs.remove(&report.job).unwrap_or(CiJobState {
                    started_at: report.timestamp,
                    messages: Vec::new(),
                    stdout: None,
                    stderr: None,
                });
                let duration = report
                    .timestamp
                    .duration_since(state.started_at)
                    .unwrap_or(Duration::ZERO);

                let CiJobState {
                    messages,
                    stdout,
                    stderr,
                    ..
                } = state;

                match &**result {
                    Ok(_) => {
                        println!("PASS {} ({})", report.job, format_duration(duration));
                        if !is_test_job(&report.job) {
                            for message in &messages {
                                CiRenderer::print_message(message);
                            }
                        }
                    }
                    Err(err) => {
                        println!("FAIL {} ({})", report.job, format_duration(duration));
                        for message in &messages {
                            CiRenderer::print_message(message);
                        }
                        if messages.is_empty() || !is_test_job(&report.job) {
                            if let Some(stderr) = &stderr {
                                CiRenderer::print_labeled_block("stderr", stderr);
                            }
                            if let Some(stdout) = &stdout {
                                CiRenderer::print_labeled_block("stdout", stdout);
                            }
                        }
                        CiRenderer::print_message(&format!("error: {err}"));
                    }
                }
            }
            None => self.handle_meta(report),
        }
    }

    fn finish(&mut self) {
        for (job, state) in self.jobs.drain() {
            let duration = SystemTime::now()
                .duration_since(state.started_at)
                .unwrap_or(Duration::ZERO);
            println!("INCOMPLETE {} ({})", job, format_duration(duration));
            let CiJobState {
                messages,
                stdout,
                stderr,
                ..
            } = state;
            let mut had_messages = false;
            for message in messages {
                CiRenderer::print_message(&message);
                had_messages = true;
            }
            if !had_messages {
                if let Some(stderr) = &stderr {
                    CiRenderer::print_labeled_block("stderr", stderr);
                }
                if let Some(stdout) = &stdout {
                    CiRenderer::print_labeled_block("stdout", stdout);
                }
            }
        }
    }
}

struct TuiRenderer {
    progress: ProgressBar,
    total_tests: u64,
    completed_tests: u64,
    passed_tests: u64,
    jobs: HashMap<JobKind, TuiJobState>,
    failures: Vec<FailureDetails>,
    overall_start: Option<SystemTime>,
    interactive: bool,
    backend_expected: HashMap<BackendName, u64>,
}

impl TuiRenderer {
    fn new(interactive: bool) -> Self {
        let progress = if interactive {
            let pb = ProgressBar::new(0);
            if let Ok(style) =
                ProgressStyle::with_template("{bar:40.cyan/blue} {pos}/{len} tests {msg}")
            {
                pb.set_style(style);
            }
            pb
        } else {
            ProgressBar::hidden()
        };

        Self {
            progress,
            total_tests: 0,
            completed_tests: 0,
            passed_tests: 0,
            jobs: HashMap::new(),
            failures: Vec::new(),
            overall_start: None,
            interactive,
            backend_expected: HashMap::new(),
        }
    }

    fn update_progress_message(&mut self) {
        if self.total_tests == 0 {
            return;
        }

        let failed = self
            .failures
            .iter()
            .filter(|failure| failure.is_test)
            .count() as u64;
        let passed = self.passed_tests;
        self.progress
            .set_message(format!("passed {passed}, failed {failed}"));
    }

    fn handle_meta(&mut self, report: &JobReport) {
        if let JobKind::BackendJob {
            kind: BackendJobKind::NumberBackendJobs(count),
            backend,
        } = &report.job
        {
            let count = *count as u64;
            self.backend_expected.insert(*backend, count);
            self.total_tests = self.backend_expected.values().copied().sum();
            self.progress.set_length(self.total_tests);
            self.progress.set_position(self.completed_tests);
            self.update_progress_message();
        }
    }

    fn print_messages(progress: &ProgressBar, messages: &[String]) {
        for message in messages {
            let mut lines = message.lines();
            match lines.next() {
                Some(first) => out(progress, format!("    → {first}")),
                None => out(progress, "    →"),
            }
            for line in lines {
                out(progress, format!("      {line}"));
            }
        }
    }

    fn print_labeled_block(progress: &ProgressBar, label: &str, content: &str) {
        out(progress, format!("    {label}:"));
        let mut lines = content.lines();
        if let Some(first) = lines.next() {
            out(progress, format!("      → {first}"));
        } else {
            out(progress, "      →");
        }
        for line in lines {
            out(progress, format!("        {line}"));
        }
    }
}

/// Prints a line on stdout above the progress bar. `ProgressBar::println`
/// draws on stderr and prints nothing when stderr is not a terminal.
fn out(progress: &ProgressBar, line: impl Display) {
    progress.suspend(|| println!("{line}"));
}

struct TuiJobState {
    started_at: SystemTime,
    messages: Vec<String>,
    is_test: bool,
    stdout: Option<String>,
    stderr: Option<String>,
}

struct FailureDetails {
    job: JobKind,
    error: String,
    messages: Vec<String>,
    duration: Option<Duration>,
    is_test: bool,
    stdout: Option<String>,
    stderr: Option<String>,
}

impl ReportRenderer for TuiRenderer {
    fn handle(&mut self, report: &JobReport) {
        match report.message.as_ref() {
            Some(ReportMessage::Start) => {
                if self.overall_start.is_none() {
                    self.overall_start = Some(report.timestamp);
                }
                let is_test = is_test_job(&report.job);
                self.jobs.insert(
                    report.job.clone(),
                    TuiJobState {
                        started_at: report.timestamp,
                        messages: Vec::new(),
                        is_test,
                        stdout: None,
                        stderr: None,
                    },
                );
                if is_test {
                    self.update_progress_message();
                }
            }
            Some(ReportMessage::Message(message)) => {
                let is_test = is_test_job(&report.job);
                let state = self
                    .jobs
                    .entry(report.job.clone())
                    .or_insert_with(|| TuiJobState {
                        started_at: report.timestamp,
                        messages: Vec::new(),
                        is_test,
                        stdout: None,
                        stderr: None,
                    });
                state.messages.push(message.clone());
            }
            Some(ReportMessage::Stdout(output)) => {
                let is_test = is_test_job(&report.job);
                let state = self
                    .jobs
                    .entry(report.job.clone())
                    .or_insert_with(|| TuiJobState {
                        started_at: report.timestamp,
                        messages: Vec::new(),
                        is_test,
                        stdout: None,
                        stderr: None,
                    });
                state.stdout = Some(output.clone());
            }
            Some(ReportMessage::Stderr(output)) => {
                let is_test = is_test_job(&report.job);
                let state = self
                    .jobs
                    .entry(report.job.clone())
                    .or_insert_with(|| TuiJobState {
                        started_at: report.timestamp,
                        messages: Vec::new(),
                        is_test,
                        stdout: None,
                        stderr: None,
                    });
                state.stderr = Some(output.clone());
            }
            Some(ReportMessage::End(result)) => {
                let state = self.jobs.remove(&report.job).unwrap_or(TuiJobState {
                    started_at: report.timestamp,
                    messages: Vec::new(),
                    is_test: is_test_job(&report.job),
                    stdout: None,
                    stderr: None,
                });
                let duration = report.timestamp.duration_since(state.started_at).ok();
                let TuiJobState {
                    messages,
                    is_test,
                    stdout,
                    stderr,
                    ..
                } = state;

                if is_test {
                    self.completed_tests += 1;
                    self.progress.set_position(self.completed_tests);
                }

                match &**result {
                    // Test jobs pass silently; the messages of other jobs,
                    // such as a summary, are their output.
                    Ok(_) if is_test => self.passed_tests += 1,
                    Ok(_) => Self::print_messages(&self.progress, &messages),
                    Err(err) => {
                        let mut line = format!("Error in {}", format_job_title(&report.job));
                        if let Some(duration) = duration {
                            line.push_str(&format!(" ({})", format_duration(duration)));
                        }
                        out(&self.progress, String::new());
                        out(&self.progress, line.black().on_red().bold().to_string());
                        Self::print_messages(&self.progress, &messages);
                        // The captured output of a test job only repeats what
                        // its messages say; a failed phase, such as the
                        // canary, has no other trace of the tool's error.
                        if messages.is_empty() || !is_test {
                            if let Some(stderr) = &stderr {
                                Self::print_labeled_block(&self.progress, "stderr", stderr);
                            }
                            if let Some(stdout) = &stdout {
                                Self::print_labeled_block(&self.progress, "stdout", stdout);
                            }
                        }
                        out(&self.progress, format!("  error: {err}").red().to_string());
                        self.failures.push(FailureDetails {
                            job: report.job.clone(),
                            error: err.to_string(),
                            messages,
                            duration,
                            is_test,
                            stdout,
                            stderr,
                        });
                    }
                }

                if is_test {
                    self.update_progress_message();
                }
            }
            None => self.handle_meta(report),
        }
    }

    fn finish(&mut self) {
        for (job, state) in self.jobs.drain() {
            let duration = SystemTime::now().duration_since(state.started_at).ok();
            let mut line = format!("Incomplete job {job}");
            if let Some(duration) = duration {
                line.push_str(&format!(" ({})", format_duration(duration)));
            }
            out(&self.progress, line);
            let TuiJobState {
                messages,
                is_test,
                stdout,
                stderr,
                ..
            } = state;
            Self::print_messages(&self.progress, &messages);
            if messages.is_empty() {
                if let Some(stderr) = &stderr {
                    Self::print_labeled_block(&self.progress, "stderr", stderr);
                }
                if let Some(stdout) = &stdout {
                    Self::print_labeled_block(&self.progress, "stdout", stdout);
                }
            }
            self.failures.push(FailureDetails {
                job,
                error: "job did not finish".to_string(),
                messages,
                duration,
                is_test,
                stdout,
                stderr,
            });
        }

        if self.interactive {
            self.progress.finish_and_clear();
        } else {
            self.progress.finish();
        }

        if self.total_tests > 0 {
            let failed = self
                .failures
                .iter()
                .filter(|failure| failure.is_test)
                .count() as u64;
            let passed = self.passed_tests;
            println!(
                "Summary: {passed}/{total} tests passed, {failed} failed.",
                total = self.total_tests
            );
        }

        let other_failures: Vec<_> = self
            .failures
            .iter()
            .filter(|failure| !failure.is_test)
            .collect();
        if !other_failures.is_empty() {
            println!("Additional failed tasks:");
            for failure in other_failures {
                let duration = failure
                    .duration
                    .map(format_duration)
                    .unwrap_or_else(|| "unknown".to_string());
                println!("  - {} ({duration}): {}", failure.job, failure.error);
                for message in &failure.messages {
                    let mut lines = message.lines();
                    if let Some(first) = lines.next() {
                        println!("    → {first}");
                    } else {
                        println!("    →");
                    }
                    for line in lines {
                        println!("      {line}");
                    }
                }
                if failure.messages.is_empty() {
                    if let Some(stderr) = &failure.stderr {
                        println!("    stderr:");
                        let mut lines = stderr.lines();
                        if let Some(first) = lines.next() {
                            println!("      → {first}");
                        } else {
                            println!("      →");
                        }
                        for line in lines {
                            println!("        {line}");
                        }
                    }
                    if let Some(stdout) = &failure.stdout {
                        println!("    stdout:");
                        let mut lines = stdout.lines();
                        if let Some(first) = lines.next() {
                            println!("      → {first}");
                        } else {
                            println!("      →");
                        }
                        for line in lines {
                            println!("        {line}");
                        }
                    }
                }
            }
        }

        if let Some(started_at) = self.overall_start
            && let Ok(total_duration) = SystemTime::now().duration_since(started_at)
        {
            println!("Total time: {}", format_duration(total_duration));
        }
    }
}

/// What the logging task receives: a report to render, or a request to
/// render everything received so far, finish the renderers, and answer.
enum Event {
    Report(JobReport),
    Flush(oneshot::Sender<()>),
}

static REPORTS: LazyLock<mpsc::UnboundedSender<Event>> = LazyLock::new(|| {
    let (tx, mut rx) = mpsc::unbounded_channel::<Event>();
    tokio::spawn(async move {
        let mut renderers = build_renderers();
        while let Some(event) = rx.recv().await {
            match event {
                Event::Report(report) => {
                    for renderer in renderers.iter_mut() {
                        renderer.handle(&report);
                    }
                }
                Event::Flush(done) => {
                    for renderer in renderers.iter_mut() {
                        renderer.finish();
                    }
                    let _ = done.send(());
                }
            }
        }
    });
    tx
});

fn send(report: JobReport) {
    REPORTS.send(Event::Report(report)).unwrap()
}

/// Waits until every report sent so far is rendered, then finishes the
/// renderers: open jobs count as incomplete failures, and the summary is
/// printed. Called once before exit: the logging task runs independently of
/// the jobs, so the process must not exit before this.
pub async fn flush() {
    let (tx, rx) = oneshot::channel();
    if REPORTS.send(Event::Flush(tx)).is_ok() {
        let _ = rx.await;
    }
}
