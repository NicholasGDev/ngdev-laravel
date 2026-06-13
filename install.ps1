# install.ps1 — Instala o ngdev globalmente em %USERPROFILE%\.cargo\bin
# Uso:
#   .\install.ps1              (usa binário pré-compilado se existir)
#   .\install.ps1 --build      (compila do zero com cargo)
#   .\install.ps1 --uninstall  (remove o binário)

$ErrorActionPreference = "Stop"

$BinaryName  = "ngdev.exe"
$InstallDir  = "$env:USERPROFILE\.cargo\bin"
$ReleaseBin  = ".\target\release\$BinaryName"

function Write-Info  { param($msg) Write-Host "[ngdev] $msg" -ForegroundColor Green  }
function Write-Warn  { param($msg) Write-Host "[ngdev] $msg" -ForegroundColor Yellow }
function Write-Err   { param($msg) Write-Host "[ngdev] $msg" -ForegroundColor Red; exit 1 }

# ── Desinstalar ───────────────────────────────────────────────────────────────
if ($args[0] -eq "--uninstall") {
    $target = Join-Path $InstallDir $BinaryName
    if (Test-Path $target) {
        Remove-Item $target -Force
        Write-Info "Removido: $target"
    } else {
        Write-Warn "Nao encontrado: $target"
    }
    exit 0
}

# ── Build ─────────────────────────────────────────────────────────────────────
if ($args[0] -eq "--build") {
    Write-Info "Compilando (release)..."

    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        # Tenta carregar via rustup
        $rustupEnv = "$env:USERPROFILE\.cargo\env"
        if (Test-Path $rustupEnv) {
            . $rustupEnv
        } else {
            Write-Err "cargo nao encontrado. Instale em https://rustup.rs"
        }
    }

    cargo build --release
    if ($LASTEXITCODE -ne 0) { Write-Err "cargo build falhou." }
}

# ── Seleciona binário ─────────────────────────────────────────────────────────
if (Test-Path $ReleaseBin) {
    $Bin = $ReleaseBin
    Write-Info "Usando binario: $Bin"
} else {
    Write-Err "Binario nao encontrado. Execute: .\install.ps1 --build"
}

# ── Cria diretório de destino se não existir ──────────────────────────────────
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

# ── Instala ───────────────────────────────────────────────────────────────────
Copy-Item -Path $Bin -Destination (Join-Path $InstallDir $BinaryName) -Force

Write-Info "Instalado: $InstallDir\$BinaryName"
Write-Info "Execute:   ngdev"

# Avisa se o diretório não estiver no PATH da sessão atual
if ($env:PATH -notlike "*$InstallDir*") {
    Write-Warn "$InstallDir nao esta no PATH desta sessao."
    Write-Warn "Adicione ao seu perfil ou reinicie o terminal."
}
