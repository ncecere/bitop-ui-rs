#!/usr/bin/env bash
# Builds the gallery, serves it, and runs the browser checks in Chromium, Firefox and
# WebKit. Pass page names to run only those, for example: scripts/e2e.sh button menu
#
# By default the browsers run in the Playwright container, so no system libraries are
# needed. Set E2E_NATIVE=1 to use a local Playwright install instead (CI does).
# Set E2E_ENGINES to a subset, for example E2E_ENGINES=chromium.
set -euo pipefail
cd "$(dirname "$0")/.."

export PORT="${PORT:-3918}"
base="http://127.0.0.1:$PORT"

cargo build -p gallery
topcoat asset bundle -p gallery
target/debug/gallery &
server=$!
trap 'kill $server 2>/dev/null || true' EXIT

for _ in $(seq 50); do
  curl -fs "$base/healthz" >/dev/null && break
  sleep 0.2
done

(cd e2e && npm ci --silent)
version=$(node -p "require('./e2e/node_modules/playwright/package.json').version")
status=0
for engine in ${E2E_ENGINES:-chromium firefox webkit}; do
  if [[ "${E2E_NATIVE:-}" == 1 ]]; then
    (cd e2e && node run.mjs "$base" "$engine" "$@") || status=1
  else
    docker run --rm --network host --ipc=host -v "$PWD/e2e:/work" -w /work \
      "mcr.microsoft.com/playwright:v$version-noble" node run.mjs "$base" "$engine" "$@" || status=1
  fi
done
exit $status
