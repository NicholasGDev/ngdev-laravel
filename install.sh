#!/usr/bin/env bash
# install.sh — Instala o ngdev globalmente em /usr/local/bin
# Uso:
#   ./install.sh              (usa binário pré-compilado se existir)
#   ./install.sh --build      (compila do zero com cargo)
#   ./install.sh --uninstall  (remove o binário)

set -euo pipefail

BINARY_NAME="ngdev"
INSTALL_DIR="/usr/local/bin"
TARGET="x86_64-unknown-linux-musl"
RELEASE_BIN="./target/${TARGET}/release/${BINARY_NAME}"
FALLBACK_BIN="./target/release/${BINARY_NAME}"

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; NC='\033[0m'

info()    { echo -e "${GREEN}[ngdev]${NC} $*"; }
warn()    { echo -e "${YELLOW}[ngdev]${NC} $*"; }
err()     { echo -e "${RED}[ngdev]${NC} $*" >&2; exit 1; }

# ── Desinstalar ───────────────────────────────────────────────────────────────
if [[ "${1:-}" == "--uninstall" ]]; then
    if [[ -f "${INSTALL_DIR}/${BINARY_NAME}" ]]; then
        rm -f "${INSTALL_DIR}/${BINARY_NAME}"
        info "Removido: ${INSTALL_DIR}/${BINARY_NAME}"
    else
        warn "Nao encontrado: ${INSTALL_DIR}/${BINARY_NAME}"
    fi
    exit 0
fi

# ── Build ─────────────────────────────────────────────────────────────────────
if [[ "${1:-}" == "--build" ]]; then
    info "Compilando (release estatico)..."
    if ! command -v cargo &>/dev/null; then
        # Tenta carregar rustup
        source "$HOME/.cargo/env" 2>/dev/null || err "cargo nao encontrado. Instale em https://rustup.rs"
    fi
    rustup target add "${TARGET}" 2>/dev/null || true
    RUSTFLAGS="-C target-feature=+crt-static" \
        cargo build --release --target "${TARGET}"
fi

# ── Seleciona binário ─────────────────────────────────────────────────────────
if [[ -f "${RELEASE_BIN}" ]]; then
    BIN="${RELEASE_BIN}"
    info "Usando binario estatico: ${BIN}"
elif [[ -f "${FALLBACK_BIN}" ]]; then
    BIN="${FALLBACK_BIN}"
    warn "Usando binario dinamico (execute --build para gerar o estatico): ${BIN}"
else
    err "Binario nao encontrado. Execute: ./install.sh --build"
fi

# ── Instala ───────────────────────────────────────────────────────────────────
if [[ ! -w "${INSTALL_DIR}" ]]; then
    info "Requer sudo para instalar em ${INSTALL_DIR}..."
    sudo install -m 755 "${BIN}" "${INSTALL_DIR}/${BINARY_NAME}"
else
    install -m 755 "${BIN}" "${INSTALL_DIR}/${BINARY_NAME}"
fi

info "Instalado: ${INSTALL_DIR}/${BINARY_NAME}"
info "Execute:   ${BINARY_NAME}"
