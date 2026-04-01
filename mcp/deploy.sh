#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ENV_FILE="$SCRIPT_DIR/.env.deploy"

if [ ! -f "$ENV_FILE" ]; then
  echo "Fichier $ENV_FILE manquant. Créez-le à partir de .env.deploy.example"
  exit 1
fi

source "$ENV_FILE"

echo "==> Syncing files to $REMOTE_HOST..."
rsync -avz --delete \
  --exclude '.env' \
  --exclude '.env.deploy' \
  --exclude '.git' \
  --exclude 'node_modules' \
  -e "ssh -p $REMOTE_PORT" \
  "$SCRIPT_DIR/" "$REMOTE_USER@$REMOTE_HOST:$REMOTE_PATH/"

echo "==> Rebuilding containers..."
ssh -p "$REMOTE_PORT" "$REMOTE_USER@$REMOTE_HOST" \
  "cd $REMOTE_PATH && docker compose up -d --build"

echo "==> Done."
