#!/usr/bin/env python3
"""Check the CI naming convention described in CONTRIBUTING.md."""

import re
import sys
from pathlib import Path

import yaml

WORKFLOWS = Path(__file__).resolve().parent.parent / ".github" / "workflows"
KEBAB = re.compile(r"[a-z0-9]+(-[a-z0-9]+)*")
CHECK_RESULTS = "./.github/actions/check-results"
MATRIX_REF = re.compile(r"\bmatrix\.([A-Za-z0-9_-]+)")


def triggers(workflow: dict) -> dict:
    # PyYAML reads the key `on` as the boolean `True`.
    on = workflow.get(True, workflow.get("on"))
    if isinstance(on, str):
        return {on: None}
    if isinstance(on, list):
        return dict.fromkeys(on)
    return on if isinstance(on, dict) else {}


def can_be_required(workflow: dict) -> bool:
    """Whether the workflow reports on every pull request and merge queue entry."""
    on = triggers(workflow)
    return all(
        event in on and not set(on[event] or {}) & {"paths", "paths-ignore"}
        for event in ("pull_request", "merge_group")
    )


def has_matrix(job: dict) -> bool:
    strategy = job.get("strategy")
    return isinstance(strategy, dict) and strategy.get("matrix") is not None


def check_job(prefix: str, id: str, job: dict) -> list[str]:
    name = job.get("name")
    if not isinstance(name, str):
        return [f"job `{id}` has no name"]
    errors = []
    part = name.removeprefix(f"{prefix}: ")
    if part == name:
        errors.append(f"job `{id}`: `{name}` does not start with `{prefix}: `")
    elif not KEBAB.fullmatch(part.split(" (")[0]):
        errors.append(f"job `{id}`: `{name}` is not kebab-case before the matrix values")
    if has_matrix(job):
        matrix = job["strategy"]["matrix"]
        refs = set(MATRIX_REF.findall(name))
        # A matrix given by an expression has keys unknown here, and a matrix
        # of only `include` entries has no fixed dimensions.
        keys = set(matrix) - {"include", "exclude"} if isinstance(matrix, dict) else set()
        if not refs:
            errors.append(f"job `{id}`: `{name}` does not name its matrix values")
        elif keys - refs:
            missing = ", ".join(sorted(keys - refs))
            errors.append(f"job `{id}`: `{name}` does not name the matrix values {missing}")
    return errors


def check_result(prefix: str, jobs: dict) -> list[str]:
    result = jobs["result"]
    errors = []
    if result.get("name") != f"{prefix}: result":
        errors.append(f"job `result` is not named `{prefix}: result`")
    if result.get("if") != "always()":
        errors.append("job `result` does not run `if: always()`")
    needs = result.get("needs", [])
    missing = set(jobs) - {"result"} - ({needs} if isinstance(needs, str) else set(needs))
    if missing:
        errors.append(f"job `result` does not need {', '.join(sorted(missing))}")
    steps = result.get("steps") or []
    if not any(isinstance(step, dict) and step.get("uses") == CHECK_RESULTS for step in steps):
        errors.append(f"job `result` does not use `{CHECK_RESULTS}`")
    return errors


def check(path: Path) -> list[str]:
    prefix = path.stem
    errors = []
    if path.suffix != ".yml" or not KEBAB.fullmatch(prefix):
        errors.append("the file name is not kebab-case with the extension `.yml`")
    try:
        workflow = yaml.safe_load(path.read_text())
    except yaml.YAMLError as error:
        return [f"{path.name}: {error}"]
    jobs = workflow.get("jobs") if isinstance(workflow, dict) else None
    if not isinstance(jobs, dict) or not all(isinstance(job, dict) for job in jobs.values()):
        return [f"{path.name}: not a workflow with a mapping of jobs"]
    names = {}
    for id, job in jobs.items():
        errors += check_job(prefix, id, job)
        name = job.get("name")
        if not isinstance(name, str):
            continue
        if name in names:
            errors.append(f"jobs `{names[name]}` and `{id}` are both named `{name}`")
        names[name] = id
    if "result" in jobs:
        errors += check_result(prefix, jobs)
    elif can_be_required(workflow) and (len(jobs) > 1 or any(map(has_matrix, jobs.values()))):
        errors.append("the workflow can be required and has several jobs or a matrix, but no job `result`")
    return [f"{path.name}: {error}" for error in errors]


def main() -> int:
    paths = sorted(p for p in WORKFLOWS.iterdir() if p.is_file())
    errors = [error for path in paths for error in check(path)]
    for error in errors:
        print(error, file=sys.stderr)
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
