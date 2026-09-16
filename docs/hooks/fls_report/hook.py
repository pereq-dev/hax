"""mkdocs hook adding the FLS report pages to the build as generated files,
so that nothing is written into the tree."""

import sys
from pathlib import Path

from mkdocs.plugins import event_priority
from mkdocs.structure.files import File

sys.path.insert(0, str(Path(__file__).parent))
import report  # noqa: E402

REPO_ROOT = Path(__file__).resolve().parents[3]


# Runs before the navigation plugins collect the files, which have the
# default priority.
@event_priority(100)
def on_files(files, config):
    if not report.inputs_present(REPO_ROOT):
        return files
    data = report.load(REPO_ROOT)
    pages = {
        f"manual/{name}/support.md": report.render_backend(data, name)
        for name in data.backends
    }
    pages["manual/fls-coverage.md"] = report.render_coverage(data)
    for src_uri, content in pages.items():
        files.append(File.generated(config, src_uri, content=content))
    return files
