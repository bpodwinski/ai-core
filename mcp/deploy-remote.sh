#!/usr/bin/env bash
# Wrapper SSH côté serveur — restreint les commandes autorisées pour la clé mcp_deploy.
#
# Installation sur le serveur :
#   scp mcp/deploy-remote.sh user@host:~/deploy-mcp.sh
#   ssh user@host "chmod 750 ~/deploy-mcp.sh"
#
# Puis dans ~/.ssh/authorized_keys du user de déploiement :
#   command="/home/USER/deploy-mcp.sh",no-port-forwarding,no-X11-forwarding,no-agent-forwarding,no-pty ssh-ed25519 AAAA...
#
# La variable SSH_ORIGINAL_COMMAND contient la commande demandée par le client.
# Ce script autorise uniquement rsync (sync fichiers) et docker compose (restart).

set -euo pipefail

DEPLOY_PATH="${MCP_DEPLOY_PATH:-/home/$(whoami)/public_html}"

case "${SSH_ORIGINAL_COMMAND:-}" in

  # rsync en mode serveur — utilisé par deploy.ps1 pour transférer les fichiers
  rsync\ --server*)
    exec $SSH_ORIGINAL_COMMAND
    ;;

  # Restart des containers après sync
  "docker compose up -d --build --force-recreate")
    cd "$DEPLOY_PATH"
    exec docker compose up -d --build --force-recreate
    ;;

  # Commande non autorisée
  *)
    echo "deploy-mcp.sh: commande non autorisée: ${SSH_ORIGINAL_COMMAND:-<vide>}" >&2
    exit 1
    ;;

esac
