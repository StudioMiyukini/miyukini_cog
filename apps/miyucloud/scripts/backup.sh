#!/usr/bin/env bash
set -euo pipefail

BACKUP_DIR="${BACKUP_DIR:-/var/backups/miyucloud}"
STORAGE_DIR="${STORAGE_DIR:-/var/lib/miyucloud/storage}"
DB_PATH="${DB_PATH:-/var/lib/miyucloud/miyucloud.db}"
RETENTION_DAYS="${RETENTION_DAYS:-14}"

mkdir -p "$BACKUP_DIR"
ts="$(date +%Y%m%d-%H%M%S)"
archive="$BACKUP_DIR/miyucloud-backup-$ts.tar.gz"

echo "[backup] creating archive: $archive"
tar -czf "$archive" "$DB_PATH" "$STORAGE_DIR"

echo "[backup] pruning archives older than $RETENTION_DAYS days"
find "$BACKUP_DIR" -type f -name "miyucloud-backup-*.tar.gz" -mtime "+$RETENTION_DAYS" -delete

echo "[backup] done"
