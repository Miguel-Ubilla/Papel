#!/usr/bin/env bash
set -e

GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

VERSION="0.1.0-beta"
ARCH="amd64"
PKG_NAME="papel"
OUTPUT_DIR="dist"
DEB_NAME="${PKG_NAME}_${VERSION}_${ARCH}.deb"
DEB_PATH="${OUTPUT_DIR}/${DEB_NAME}"
STAGE_DIR="/tmp/papel_deb_stage"

mkdir -p "$OUTPUT_DIR"

echo -e "${BLUE}==>${NC} Compilando Papel en modo optimizado (Release)..."
if [ -f "$HOME/.cargo/env" ]; then
    source "$HOME/.cargo/env"
fi
cargo build --release

echo -e "${BLUE}==>${NC} Creando estructura para el paquete .deb..."
rm -rf "$STAGE_DIR"
mkdir -p "$STAGE_DIR/DEBIAN"
mkdir -p "$STAGE_DIR/usr/bin"
mkdir -p "$STAGE_DIR/usr/share/applications"
mkdir -p "$STAGE_DIR/usr/share/icons/hicolor/scalable/apps"

# Archivo de control de Debian
cat <<EOF > "$STAGE_DIR/DEBIAN/control"
Package: ${PKG_NAME}
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: ${ARCH}
Depends: libc6, libgtk-4-1, libadwaita-1-0
Maintainer: Miguel Ubilla
Homepage: https://github.com/Miguel-Ubilla/Papel
Description: Bloc de notas nativo, rapido y seguro para Ubuntu
 Papel es un editor de texto gráfico moderno escrito en Rust con GTK4 y Libadwaita.
 Incluye soporte para pestañas múltiples, búsqueda con resaltado visual interactivo
 y un sistema de autoguardado atómico que protege tus documentos ante cortes de energía.
EOF

chmod 755 "$STAGE_DIR/DEBIAN"
chmod 644 "$STAGE_DIR/DEBIAN/control"

# Copiar binario y assets
install -m 755 target/release/papel "$STAGE_DIR/usr/bin/papel"
install -m 644 assets/io.github.papel.editor.desktop "$STAGE_DIR/usr/share/applications/io.github.papel.editor.desktop"
install -m 644 assets/io.github.papel.editor.svg "$STAGE_DIR/usr/share/icons/hicolor/scalable/apps/io.github.papel.editor.svg"

echo -e "${BLUE}==>${NC} Empaquetando ${DEB_PATH} con dpkg-deb..."
dpkg-deb --build --root-owner-group "$STAGE_DIR" "$DEB_PATH"

rm -rf "$STAGE_DIR"

echo -e "${GREEN}==> ¡Paquete .deb generado con éxito!${NC}"
echo -e "Archivo creado: ${GREEN}${DEB_PATH}${NC}"
echo -e "Puedes instalarlo con:"
echo -e "  sudo apt install ./${DEB_PATH}"
echo -e "o haciendo doble clic sobre él en tu explorador de archivos."
