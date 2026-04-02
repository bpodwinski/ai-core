$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$envFile = Join-Path $scriptDir ".env.deploy"

if (-not (Test-Path $envFile)) {
    Write-Error "Fichier $envFile manquant. Creez-le a partir de .env.deploy.example"
    exit 1
}

# Charger les variables
Get-Content $envFile | ForEach-Object {
    if ($_ -match '^([^#]\S+?)=(.+)$') {
        Set-Variable -Name $matches[1] -Value $matches[2]
    }
}

# Dossiers et fichiers a exclure
$excludeDirs = @('.git', 'node_modules', 'target')
$excludeFiles = @('.env', '.env.deploy', '.env.deploy.example')

Write-Host "==> Syncing files to $REMOTE_HOST..."

# Collecter les fichiers en excluant les dossiers et fichiers blacklistes
$items = Get-ChildItem -Path $scriptDir -Recurse -File | Where-Object {
    $rel = $_.FullName.Substring($scriptDir.Length + 1)
    $parts = $rel.Split('\')

    # Exclure si un segment du chemin matche un dossier exclu
    $inExcludedDir = $false
    foreach ($part in $parts) {
        if ($excludeDirs -contains $part) {
            $inExcludedDir = $true
            break
        }
    }

    # Exclure si le nom du fichier matche
    $isExcludedFile = $excludeFiles -contains $_.Name

    (-not $inExcludedDir) -and (-not $isExcludedFile)
}

# Copier chaque fichier via scp
foreach ($item in $items) {
    $rel = $item.FullName.Substring($scriptDir.Length + 1).Replace('\', '/')
    $remotePath = "${REMOTE_PATH}${rel}"
    $remoteDir = $remotePath -replace '/[^/]+$', ''

    & ssh -p $REMOTE_PORT -i $SSH_KEY "${REMOTE_USER}@${REMOTE_HOST}" "mkdir -p '$remoteDir'" 2>$null
    & scp -P $REMOTE_PORT -i $SSH_KEY $item.FullName "${REMOTE_USER}@${REMOTE_HOST}:${remotePath}" | Out-Null
    Write-Host "  $rel"
}

Write-Host "==> Rebuilding containers..."
& ssh -p $REMOTE_PORT -i $SSH_KEY "${REMOTE_USER}@${REMOTE_HOST}" "cd $REMOTE_PATH && docker compose up -d --build --force-recreate"

Write-Host "==> Done."
