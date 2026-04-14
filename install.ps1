# Install Claude Code + Codex CLI configs from the latest GitHub release.
# Also installs github-mcp-server binary.
#
# Usage:
#   # New machine setup (global, no ProjectDir needed):
#   [Net.ServicePointManager]::SecurityProtocol=3072; $f="$env:TEMP\install.ps1"; (New-Object Net.WebClient).DownloadFile('https://github.com/bpodwinski/ai-core/releases/latest/download/install.ps1',$f); &$f
#
#   # Add .mcp.json to a specific project:
#   .\install.ps1 -ProjectDir C:\Dev\myproject
[CmdletBinding()]
param(
    [string]$ProjectDir = ""
)
$ErrorActionPreference = 'Stop'
# GitHub requires TLS 1.2 — PowerShell 5.x defaults to TLS 1.0
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

$Repo      = "bpodwinski/ai-core"
$GhMcpRepo = "github/github-mcp-server"
$ZipUrl    = "https://github.com/$Repo/releases/latest/download/claude-config.zip"
$Tmp       = [System.IO.Path]::GetTempPath() + [System.IO.Path]::GetRandomFileName()
New-Item -ItemType Directory -Path $Tmp | Out-Null
$Zip       = "$Tmp\claude-config.zip"

# ── 1. Download zip ───────────────────────────────────────────────────────────
Write-Host "Downloading latest configs from $Repo..."
(New-Object Net.WebClient).DownloadFile($ZipUrl, $Zip)
Expand-Archive -Path $Zip -DestinationPath $Tmp -Force

# ── 2. Machine-level install (always, no ProjectDir needed) ───────────────────
Write-Host "Installing global Claude config -> $HOME\.claude\"
$ClaudeDir = "$HOME\.claude"
if (-not (Test-Path $ClaudeDir)) { New-Item -ItemType Directory -Path $ClaudeDir | Out-Null }
# settings.json
Copy-Item "$Tmp\.claude\settings.json" -Destination "$ClaudeDir\settings.json" -Force
# hooks/
if (Test-Path "$Tmp\.claude\hooks") {
    $HooksDir = "$ClaudeDir\hooks"
    if (-not (Test-Path $HooksDir)) { New-Item -ItemType Directory -Path $HooksDir | Out-Null }
    Copy-Item "$Tmp\.claude\hooks\*" -Destination $HooksDir -Force
}

Write-Host "Installing Codex config -> $HOME\.codex\config.toml"
$CodexDir = "$HOME\.codex"
if (-not (Test-Path $CodexDir)) { New-Item -ItemType Directory -Path $CodexDir | Out-Null }
Copy-Item "$Tmp\.codex\config.toml" -Destination "$CodexDir\config.toml" -Force

# ── 3. github-mcp-server binary ───────────────────────────────────────────────
Write-Host ""
Write-Host "Installing github-mcp-server..."
$BinDir = "$HOME\.claude\bin"
if (-not (Test-Path $BinDir)) { New-Item -ItemType Directory -Path $BinDir | Out-Null }

$Arch = if ([System.Environment]::Is64BitOperatingSystem) {
    $cpu = (Get-WmiObject Win32_Processor).Architecture
    if ($cpu -eq 12) { "arm64" } else { "x86_64" }
} else { "x86_64" }

$Asset    = "github-mcp-server_Windows_$Arch.zip"
$GhMcpUrl = "https://github.com/$GhMcpRepo/releases/latest/download/$Asset"
$GhMcpZip = "$Tmp\$Asset"

Write-Host "Downloading $Asset..."
(New-Object Net.WebClient).DownloadFile($GhMcpUrl, $GhMcpZip)

Add-Type -AssemblyName System.IO.Compression.FileSystem
$z     = [System.IO.Compression.ZipFile]::OpenRead($GhMcpZip)
$entry = $z.Entries | Where-Object { $_.Name -eq "github-mcp-server.exe" }
[System.IO.Compression.ZipFileExtensions]::ExtractToFile($entry, "$BinDir\github-mcp-server.exe", $true)
$z.Dispose()
Write-Host "github-mcp-server installed -> $BinDir"

# ── 4. Project-level install (.mcp.json) — only if -ProjectDir is given ───────
if ($ProjectDir -ne "") {
    Write-Host ""
    Write-Host "Installing .mcp.json -> $ProjectDir"
    Copy-Item "$Tmp\.mcp.json" -Destination "$ProjectDir\.mcp.json" -Force
} else {
    Write-Host ""
    Write-Host "Tip: to add .mcp.json to a project, run:"
    Write-Host "  .\install.ps1 -ProjectDir C:\path\to\project"
}

Remove-Item -Recurse -Force $Tmp
Write-Host ""
Write-Host "Done. Set GITHUB_TOKEN in %USERPROFILE%\.claude\settings.json."
