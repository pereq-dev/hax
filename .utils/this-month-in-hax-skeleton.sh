#!/usr/bin/env bash
# This script creates a skeleton blog post for the "This Month in hax" blog series.
# It writes a new markdown file, and outputs a PR body.
# This script is an helper for the github action workflow "this-month-in-hax.yml".

set -e

# Go to the folder of blog posts
cd $(git rev-parse --show-toplevel)/docs/blog/posts/this-month-in-hax

# By default, use `pereq-dev/hax`, and the month and year from two weeks ago
repo="--repo pereq-dev/hax"
month=$(date -d "14 days ago" +'%m')
year=$(date -d "14 days ago" +'%Y')

# Set date formatting to English
export LC_ALL=C

# Parse command line arguments.
all_args=("$@")
while [ $# -gt 0 ]; do
    case "$1" in
    -r | --repo) repo="--repo $2"; shift ;;
    -m | --month) month=$2; shift ;;
    -y | --year) year=$2; shift ;;
    --author) author=$2; shift ;;
    esac
    shift
done

report() {
    # Calculate the first day of the month
    start=$(date -u -d "$year-$month-01" +"%Y-%m-%dT%H:%M:%SZ")

    # Get the next month
    end=$(date -u -d "$year-$month-01 + 1 month - 1 day" +"%Y-%m-%dT%H:%M:%SZ")

    # Get all closed PRs with number, title, and description
    pr_data=$(
        gh pr list $repo --state merged --limit 1000 \
            --json number,title,url,author,mergedAt \
            --jq "map(select(.mergedAt >= \"$start\" and .mergedAt <= \"$end\" and .author.login != \"app/dependabot\")) | .[] | {number, title, url, author}" | jq -s
    )

    echo "In $(date -d "$year-$month-01" +"%B"), we successfully merged **$(echo "$pr_data" | jq -r 'length') pull requests**!"
    echo ""
    echo "<DESCRIPTION>"
    echo ""
    echo "### Full list of PRs"

    # Extract markdown list with jq
    echo "$pr_data" | jq -r '.[] | . | "* \\#\(.number): [\(.title)](\(.url))"'

    echo ""
    echo "### Contributors"
    # Extract markdown list of authors with jq
    echo "$pr_data" | jq -r 'map(.author.login) | unique | .[] | "* [@\(.)](https://github.com/\(.))"'
}

# Available authors, and their GH handles
authors_and_handles() {
    sort -u <<AUTHORS | sed '/^[[:space:]]*$/d'
maxime:maximebuyse
clement:clementblaudeau
alex:abentkamp
tobias:treiher
AUTHORS
}
authors() {
    authors_and_handles | cut -d: -f1
}
handle_of() {
    authors_and_handles | grep "^$1:" | cut -d: -f2
}

find_last_blog_authors() {
    N=$(authors_and_handles | wc -l)
    N=$((N - 1))
    ls -t1 | head -n$N | xargs awk '/^authors:/,/^---/{ if ($0 ~ /^  - /) { sub(/^  - /, ""); print } }' | sort -u
}

pick_author() {
    diff <(authors) <(find_last_blog_authors) | grep '^< ' | cut -d' ' -f2 | shuf -n1
}

author=$(pick_author)

BLOG_POST_FILE="$year-$month.md"


cat << HEADER > $BLOG_POST_FILE
---
authors:
  - $author
title: "This Month in Hax: $(date -d "$year-$month-15" +"%B %Y")"
date: $(date +"%Y-%m-%d")
---

HEADER
report >> $BLOG_POST_FILE

BLOG_POST="$(cat $BLOG_POST_FILE)"

# Go to root
cd $(git rev-parse --show-toplevel)
BRANCH="this-month-in-hax-blog-post-$year-$month"
echo $BRANCH > this-month-in-hax-branch

# Echo the author's handle
cat <<MESSAGE > this-month-in-hax-issue.yml
---
title: Write This Month in Hax
assignees: $(handle_of $author)
---

This is an auto-generated issue for the "This Month in hax" blog series.

Branch [\`$BRANCH\`](https://github.com/pereq-dev/hax/tree/$BRANCH) have been created with the following template:
\`\`\`md
$(echo "$BLOG_POST")
\`\`\`

It is an skeleton blog post with the list of PRs pushed in $(date -d "$year-$month-01" +"%B %Y") and a list of contributor.

Suggested person to pick this draft PR: @$(handle_of $author)

## Action Items
 - [ ] Write the blog article
 - [ ] Release a new version of hax
    - [ ] Follow \`PUBLISHING.md\`
    - [ ] Create Github release
MESSAGE
