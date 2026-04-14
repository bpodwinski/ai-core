# Install Claude Code + Codex CLI configs from the latest GitHub release.
# Also installs github-mcp-server binary.
# Usage (from project root, or anywhere):
#   irm https://github.com/bpodwinski/ai-core/releases/latest/download/install.ps1 | iex
#   .\install.ps1
#   .\install.ps1 -ProjectDir C:\path\to\project
[CmdletBinding()]
param(
    [string]$ProjectDir = $PWD
)
$ErrorActionPreference = 'Stop'
# GitHub requires TLS 1.2 — PowerShell 5.x defaults to TLS 1.0
[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12

$Repo       = "bpodwinski/ai-core"
$GhMcpRepo  = "github/github-mcp-server"
$ZipUrl     = "https://github.com/$Repo/releases/latest/download/claude-config.zip"
$Tmp        = [System.IO.Path]::GetTempPath() + [System.IO.Path]::GetRandomFileName()
New-Item -ItemType Directory -Path $Tmp | Out-Null
$Zip        = "$Tmp\claude-config.zip"

# ── 1. Claude Code + Codex configs ───────────────────────────────────────────
Write-Host "Downloading latest configs from $Repo..."
Invoke-WebRequest -Uri $ZipUrl -OutFile $Zip -UseBasicParsing

Write-Host "Installing project configs → $ProjectDir"
Expand-Archive -Path $Zip -DestinationPath $Tmp -Force

# .mcp.json → project root
Copy-Item "$Tmp\.mcp.json" -Destination "$ProjectDir\.mcp.json" -Force

# .claude/ → project root
if (Test-Path "$Tmp\.claude") {
    Copy-Item "$Tmp\.claude" -Destination "$ProjectDir\.claude" -Recurse -Force
}

Write-Host "Installing Codex config → $HOME\.codex\config.toml"
$CodexDir = "$HOME\.codex"
if (-not (Test-Path $CodexDir)) { New-Item -ItemType Directory -Path $CodexDir | Out-Null }
Copy-Item "$Tmp\.codex\config.toml" -Destination "$CodexDir\config.toml" -Force

# ── 2. github-mcp-server binary ──────────────────────────────────────────────
Write-Host ""
Write-Host "Installing github-mcp-server..."

$BinDir = "$HOME\.claude\bin"
if (-not (Test-Path $BinDir)) { New-Item -ItemType Directory -Path $BinDir | Out-Null }

$Arch = if ([System.Environment]::Is64BitOperatingSystem) {
    $cpu = (Get-WmiObject Win32_Processor).Architecture
    if ($cpu -eq 12) { "arm64" } else { "x86_64" }
} else { "x86_64" }

$Asset      = "github-mcp-server_Windows_$Arch.zip"
$GhMcpUrl   = "https://github.com/$GhMcpRepo/releases/latest/download/$Asset"
$GhMcpZip   = "$Tmp\$Asset"

Write-Host "Downloading $Asset..."
Invoke-WebRequest -Uri $GhMcpUrl -OutFile $GhMcpZip -UseBasicParsing

Add-Type -AssemblyName System.IO.Compression.FileSystem
$zip = [System.IO.Compression.ZipFile]::OpenRead($GhMcpZip)
$entry = $zip.Entries | Where-Object { $_.Name -eq "github-mcp-server.exe" }
[System.IO.Compression.ZipFileExtensions]::ExtractToFile($entry, "$BinDir\github-mcp-server.exe", $true)
$zip.Dispose()

Write-Host "github-mcp-server installed → $BinDir"

Remove-Item -Recurse -Force $Tmp
Write-Host ""
Write-Host "Done. Set GITHUB_TOKEN in your environment or in %USERPROFILE%\.claude\settings.json."
