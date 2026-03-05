#!/usr/bin/env bash
set -euo pipefail

API_URL="${API_URL:-http://127.0.0.1:11440/health}"
WEB_URL="${WEB_URL:-https://127.0.0.1:11442/health}"
TIMEOUT_SECONDS="${TIMEOUT_SECONDS:-5}"

echo "[healthcheck] api: $API_URL"
curl --fail --silent --show-error --max-time "$TIMEOUT_SECONDS" "$API_URL" >/dev/null

echo "[healthcheck] web: $WEB_URL"
curl --fail --silent --show-error --max-time "$TIMEOUT_SECONDS" --insecure "$WEB_URL" >/dev/null

echo "[healthcheck] ok"
