$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$envFile = Join-Path $scriptDir ".env.deploy"

if (-not (Test-Path $envFile)) {
    Write-Error "Fichier $envFile manquant. Créez-le à partir de .env.deploy.example"
    exit 1
}

# Charger les variables
Get-Content $envFile | ForEach-Object {
    if ($_ -match '^([^#]\S+?)=(.+)$') {
        Set-Variable -Name $matches[1] -Value $matches[2]
    }
}

# Chemin SSH (Windows natif si disponible, sinon ssh du PATH)
$SSH_CMD = if ($env:SSH_CMD) { $env:SSH_CMD } else { "ssh" }

# Construire les chaînes en dehors de l'interpolation pour éviter
# l'ambiguïté du ":" avec la syntaxe de scope PowerShell ($env:, etc.)
$remoteTarget  = $REMOTE_USER + "@" + $REMOTE_HOST
$rsyncDest     = $remoteTarget + ":" + $REMOTE_PATH + "/"
$sshTransport  = $SSH_CMD + " -p " + $REMOTE_PORT + " -i " + $SSH_KEY

# ── 1. Sync des fichiers via rsync ────────────────────────────────────────────
Write-Host "==> Syncing files to $rsyncDest ..."

& rsync -az --delete `
    --exclude='.git/' `
    --exclude='.env' `
    --exclude='.env.*' `
    --exclude='node_modules/' `
    --exclude='servers/rust-docs/target/' `
    -e $sshTransport `
    "$scriptDir/" `
    $rsyncDest

if ($LASTEXITCODE -ne 0) {
    Write-Error "rsync a échoué (exit $LASTEXITCODE)"
    exit 1
}

# ── 2. Rebuild des containers ─────────────────────────────────────────────────
Write-Host "==> Rebuilding containers..."

& $SSH_CMD -p $REMOTE_PORT -i $SSH_KEY $remoteTarget `
    "docker compose up -d --build --force-recreate"

if ($LASTEXITCODE -ne 0) {
    Write-Error "docker compose a échoué (exit $LASTEXITCODE)"
    exit 1
}

Write-Host "==> Done."
