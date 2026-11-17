#!/usr/bin/env bash
set -e

# Colores para salida de terminal
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}==>${NC} Compilando Papel en modo optimizado (Release)..."
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
fi

cargo build --release

BIN_DIR="$HOME/.local/bin"
APP_DIR="$HOME/.local/share/applications"
ICON_DIR="$HOME/.local/share/icons/hicolor/scalable/apps"

echo -e "${BLUE}==>${NC} Preparando directorios de instalación..."
mkdir -p "$BIN_DIR"
mkdir -p "$APP_DIR"
mkdir -p "$ICON_DIR"

echo -e "${BLUE}==>${NC} Instalando binario ejecutable en $BIN_DIR/papel..."
install -m 755 target/release/papel "$BIN_DIR/papel"

echo -e "${BLUE}==>${NC} Instalando icono de la aplicación..."
cp assets/io.github.papel.editor.svg "$ICON_DIR/io.github.papel.editor.svg"

echo -e "${BLUE}==>${NC} Instalando lanzador de menú (.desktop)..."
cp assets/io.github.papel.editor.desktop "$APP_DIR/io.github.papel.editor.desktop"

# Limpieza de versión anterior con nombre previo si existiera
rm -f "$BIN_DIR/libreta" "$APP_DIR/io.github.libreta.editor.desktop" "$ICON_DIR/io.github.libreta.editor.svg" 2>/dev/null || true

# Actualizar caches del entorno de escritorio de Ubuntu
if command -v update-desktop-database &> /dev/null; then
    update-desktop-database "$APP_DIR" || true
fi

if command -v gtk-update-icon-cache &> /dev/null; then
    gtk-update-icon-cache -f -t "$HOME/.local/share/icons/hicolor" 2>/dev/null || true
fi

echo -e "${GREEN}==> ¡Instalación y actualización completada con éxito!${NC}"
echo -e "Puedes abrir Papel desde el menú de aplicaciones de Ubuntu o escribiendo 'papel' en la terminal."
