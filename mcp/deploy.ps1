param(
    [switch]$NoCache
)

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

$SSH_CMD = if ($env:SSH_CMD) { $env:SSH_CMD } else { "ssh" }

# Construire les chaînes sans interpolation ":" pour éviter l'ambiguïté PS
$remoteTarget = $REMOTE_USER + "@" + $REMOTE_HOST
$rsyncDest    = $remoteTarget + ":" + $REMOTE_PATH + "/"
$sshTransport = $SSH_CMD + " -p " + $REMOTE_PORT + " -i " + $SSH_KEY

# Détecter rsync — natif ou via Git for Windows
$rsyncCmd = Get-Command rsync -ErrorAction SilentlyContinue
if (-not $rsyncCmd) {
    $gitRsync = "C:\Program Files\Git\usr\bin\rsync.exe"
    if (Test-Path $gitRsync) { $rsyncCmd = $gitRsync }
}

# ── 1. Sync des fichiers ──────────────────────────────────────────────────────
if ($rsyncCmd) {
    # rsync disponible — sync efficace en une seule session SSH
    Write-Host "==> Syncing (rsync) to $rsyncDest ..."

    & $rsyncCmd -az --delete `
        --exclude='.git/' `
        --exclude='.env' `
        --exclude='.env.*' `
        --exclude='node_modules/' `
        --exclude='target/' `
        --exclude='dist/' `
        --exclude='AGENTS.md' `
        --exclude='CLAUDE.md' `
        --exclude='justfile' `
        --exclude='deploy.ps1' `
        --exclude='deploy-remote.sh' `
        --exclude='tests/' `
        --exclude='crates/mcp-configgen/src/' `
        --exclude='crates/mcp-hooks/src/' `
        -e $sshTransport `
        "$scriptDir/" `
        $rsyncDest

    if ($LASTEXITCODE -ne 0) { Write-Error "rsync a échoué (exit $LASTEXITCODE)"; exit 1 }

} else {
    # Fallback SCP — copie fichier par fichier
    Write-Host "==> Syncing (scp) to $remoteTarget ..."

    $excludeDirs     = @('.git', 'node_modules', 'target', 'dist', 'tests')
    $excludeFiles    = @('.env', '.env.deploy', '.env.deploy.example', 'AGENTS.md', 'CLAUDE.md', 'justfile', 'deploy.ps1', 'deploy-remote.sh')
    $excludeRelPaths = @('crates\mcp-configgen\src', 'crates\mcp-hooks\src')

    $items = Get-ChildItem -Path $scriptDir -Recurse -File | Where-Object {
        $rel   = $_.FullName.Substring($scriptDir.Length + 1)
        $parts = $rel.Split('\')
        $inExcludedDir     = $parts | Where-Object { $excludeDirs -contains $_ }
        $isExcludedFile    = $excludeFiles -contains $_.Name
        $inExcludedRelPath = $excludeRelPaths | Where-Object { $rel.StartsWith($_) }
        (-not $inExcludedDir) -and (-not $isExcludedFile) -and (-not $inExcludedRelPath)
    }

    foreach ($item in $items) {
        $rel        = $item.FullName.Substring($scriptDir.Length + 1).Replace('\', '/')
        $remotePath = $REMOTE_PATH + "/" + $rel
        $remoteDir  = $remotePath -replace '/[^/]+$', ''

        & $SSH_CMD -p $REMOTE_PORT -i $SSH_KEY $remoteTarget "mkdir -p '$remoteDir'" 2>$null
        & scp -P $REMOTE_PORT -i $SSH_KEY $item.FullName ($remoteTarget + ":" + $remotePath) | Out-Null
        Write-Host "  $rel"
    }
}

# ── 2. Rebuild des containers ─────────────────────────────────────────────────
Write-Host "==> Rebuilding containers..."

$buildCmd = if ($NoCache) { "docker compose build --no-cache mcp-docs && " } else { "" }
$restartCmd = "cd " + $REMOTE_PATH + " && " + $buildCmd + "docker compose up -d --build --force-recreate"
& $SSH_CMD -p $REMOTE_PORT -i $SSH_KEY $remoteTarget $restartCmd

if ($LASTEXITCODE -ne 0) { Write-Error "docker compose a échoué (exit $LASTEXITCODE)"; exit 1 }

Write-Host "==> Done."

# 3. Health check
Write-Host "==> Health check..."
$maxAttempts = 6
$delay = 5
for ($i = 1; $i -le $maxAttempts; $i++) {
    try {
        Invoke-RestMethod -Uri $HEALTH_URL -TimeoutSec 10 | Out-Null
        Write-Host "==> OK - server is up."
        exit 0
    } catch {
        if ($i -lt $maxAttempts) {
            Write-Host "  attempt $i/$maxAttempts failed, retrying in $delay s..."
            Start-Sleep -Seconds $delay
        }
    }
}
Write-Error "Health check FAILED after $maxAttempts attempts."
exit 1
