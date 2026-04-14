#!/usr/bin/env bash
# Wrapper SSH côté serveur — restreint les commandes autorisées pour la clé mcp_deploy.
#
# Installation sur le serveur :
#   scp deploy-remote.sh user@host:~/deploy-mcp.sh
#   ssh user@host "chmod 750 ~/deploy-mcp.sh"
#
# Puis dans ~/.ssh/authorized_keys :
#   command="/home/USER/deploy-mcp.sh",no-port-forwarding,no-X11-forwarding,no-agent-forwarding,no-pty ssh-ed25519 AAAA...

set -euo pipefail

DEPLOY_PATH="${MCP_DEPLOY_PATH:-/home/$(whoami)/web/mcp.benoitpodwinski.com/public_html}"

case "${SSH_ORIGINAL_COMMAND:-}" in

  # rsync en mode serveur (client avec rsync disponible)
  "rsync --server"*)
    exec $SSH_ORIGINAL_COMMAND
    ;;

  # SCP en mode réception (fallback pour Windows sans rsync)
  "scp -t "*)
    exec $SSH_ORIGINAL_COMMAND
    ;;

  # Création de répertoires (fallback SCP)
  "mkdir -p "*)
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
