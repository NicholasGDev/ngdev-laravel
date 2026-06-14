#!/usr/bin/env bash
# install.sh — Instala o caronte globalmente em /usr/local/bin
# Uso:
#   ./install.sh              (usa binário pré-compilado se existir)
#   ./install.sh --build      (compila do zero com cargo)
#   ./install.sh --uninstall  (remove o binário)

set -euo pipefail

BINARY_NAME="caronte"
INSTALL_DIR="/usr/local/bin"
TARGET="x86_64-unknown-linux-musl"
RELEASE_BIN="./target/${TARGET}/release/${BINARY_NAME}"
FALLBACK_BIN="./target/release/${BINARY_NAME}"

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; NC='\033[0m'

info()    { echo -e "${GREEN}[caronte]${NC} $*"; }
warn()    { echo -e "${YELLOW}[caronte]${NC} $*"; }
err()     { echo -e "${RED}[caronte]${NC} $*" >&2; exit 1; }

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
        # Tenta carregar rustup da sessao atual
        source "$HOME/.cargo/env" 2>/dev/null || true
    fi

    if ! command -v cargo &>/dev/null; then
        warn "cargo nao encontrado. Instalando Rust via rustup..."
        if command -v curl &>/dev/null; then
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
        elif command -v wget &>/dev/null; then
            wget -qO- https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
        else
            err "curl ou wget nao encontrados. Instale o Rust manualmente em https://rustup.rs"
        fi
        source "$HOME/.cargo/env" 2>/dev/null || err "Rust instalado mas cargo nao encontrado. Reabra o terminal e tente novamente."
        info "Rust instalado com sucesso."
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
