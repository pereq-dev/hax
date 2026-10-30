#!/usr/bin/env bash
# Keeps one open issue per label current, for workflows that report a state
# rather than an event. Unlike the `open-issue-once` action, which opens an
# issue per event and never touches it again, the issue is edited on every
# run and closed once the state is resolved.
#
#   .utils/upsert-issue.sh <label> <title> <body-file>
#       Creates the label if missing, then edits the body of the open issue
#       with the label, or creates the issue.
#   .utils/upsert-issue.sh --resolve <label> [<comment>]
#       Closes the open issue with the label, if any, with the comment.
#
# Needs `gh` authenticated for the repository (GH_TOKEN in a workflow).
set -euo pipefail

# GitHub rejects bodies above 65536 characters.
max_body=60000

usage() {
  echo "usage: $0 <label> <title> <body-file> | $0 --resolve <label> [<comment>]" >&2
  exit 2
}

open_issue() {
  gh issue list --label "$1" --state open --limit 1 --json number --jq '.[0].number // empty'
}

if [ "${1:-}" = "--resolve" ]; then
  label="${2:-}"
  [ -n "$label" ] || usage
  comment="${3:-Resolved by the latest run.}"
  number="$(open_issue "$label")"
  if [ -z "$number" ]; then
    echo "no open issue with label $label"
    exit 0
  fi
  gh issue close "$number" --comment "$comment"
  echo "closed #$number"
  exit 0
fi

label="${1:-}"
title="${2:-}"
body_file="${3:-}"
if [ -z "$label" ] || [ -z "$title" ] || [ ! -f "$body_file" ]; then
  usage
fi

body="$(mktemp)"
trap 'rm -f "$body"' EXIT
if [ "$(wc -c <"$body_file")" -gt "$max_body" ]; then
  # `iconv -c` drops a character the cut splits, and fails when it does.
  head -c "$max_body" "$body_file" | iconv -c -f UTF-8 -t UTF-8 >"$body" || true
  # Closes a code fence the cut leaves open.
  if [ $(($(grep -c '^```' "$body" || true) % 2)) -eq 1 ]; then
    printf '\n```\n' >>"$body"
  fi
  printf '\n\n*Truncated to %s bytes; the full output is in the workflow log.*\n' \
    "$max_body" >>"$body"
else
  cp "$body_file" "$body"
fi

# Created only when missing, so that a colour chosen by a maintainer stays.
# Any error other than a missing label fails the script with its message.
if ! lookup="$(gh api "repos/{owner}/{repo}/labels/$label" --silent 2>&1)"; then
  if ! grep -q 'HTTP 404' <<<"$lookup"; then
    echo "$lookup" >&2
    exit 1
  fi
  gh label create "$label" --description "Kept current by a workflow" >/dev/null
fi
number="$(open_issue "$label")"
if [ -n "$number" ]; then
  gh issue edit "$number" --title "$title" --body-file "$body" >/dev/null
  echo "updated #$number"
else
  gh issue create --label "$label" --title "$title" --body-file "$body"
fi
