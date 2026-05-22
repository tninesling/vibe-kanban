#!/bin/sh
set -e

DATA_DIR="${HOME}/.local/share/vibe-kanban"
SEED_DIR="/seed"

# Seed the database and config on first run
if [ ! -f "${DATA_DIR}/db.sqlite" ]; then
  echo "Seeding database from ${SEED_DIR}..."
  mkdir -p "${DATA_DIR}"
  cp -r "${SEED_DIR}/." "${DATA_DIR}/"
fi

exec /usr/bin/tini -- /usr/local/bin/server "$@"
