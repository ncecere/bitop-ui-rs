#!/usr/bin/env bash
# Builds a static copy of the gallery in site/, for GitHub Pages.
#
# Topcoat renders on the server and has no static export, so this serves the gallery
# and crawls it: every page reachable by a link, and the assets they load, with links
# rewritten to relative paths so the copy works under any base path. Pages that differ
# by query string (sorting and paging in the data table) are saved as separate files.
#
# What needs a live server does not work in the copy: a form submitted with text that
# no link leads to, such as a search in the filter bar, and state the server reads from
# a cookie, such as the sidebar staying collapsed after a reload.
set -euo pipefail
cd "$(dirname "$0")/.."

export PORT="${PORT:-3919}"
base="http://127.0.0.1:$PORT"

cargo build -p gallery --release
topcoat asset bundle -p gallery --release
target/release/gallery &
server=$!
trap 'kill $server 2>/dev/null || true' EXIT

for _ in $(seq 50); do
  curl -fs "$base/healthz" >/dev/null && break
  sleep 0.2
done

rm -rf site
# --mirror follows every link; --page-requisites fetches the CSS, scripts and images;
# --convert-links makes links relative; --adjust-extension saves pages as .html.
# wget exits with 8 when any request fails, so check the log for errors instead.
wget --mirror --page-requisites --convert-links --adjust-extension \
  --no-host-directories --directory-prefix=site --execute robots=off \
  --reject-regex '/healthz' --no-verbose "$base/" >site.log 2>&1 || true

if grep -E "ERROR|failed" site.log; then
  echo "the crawl had errors; see site.log" >&2
  exit 1
fi
rm site.log

# --convert-links turns a link to a place on the same page ("#billing") into a link to
# the file ("item.html#billing"), which reloads the page. Put those back.
python3 -I - site <<'PY'
import pathlib, re, sys
for page in pathlib.Path(sys.argv[1]).rglob("*.html"):
    text = page.read_text()
    fixed = re.sub(r'href="' + re.escape(page.name) + '#', 'href="#', text)
    if fixed != text:
        page.write_text(fixed)
PY

# GitHub Pages runs Jekyll unless told not to, and Jekyll drops files starting with _.
touch site/.nojekyll
echo "site/: $(find site -name '*.html' | wc -l) pages, $(du -sh site | cut -f1)"
