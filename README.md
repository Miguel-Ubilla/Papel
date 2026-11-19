<div align="center">

# 📄 Papel

**Un editor de texto moderno, ligero y a prueba de fallos para Ubuntu / GNOME, escrito en Rust.**

[🇪🇸 Español](README.md) | [🇬🇧 English](README_EN.md)

[![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange.svg?logo=rust)](https://www.rust-lang.org/)
[![GTK4](https://img.shields.io/badge/GTK-4.14-blue.svg?logo=gnome)](https://www.gtk.org/)
[![Libadwaita](https://img.shields.io/badge/Libadwaita-1.5-blue.svg)](https://gnome.pages.gitlab.gnome.org/libadwaita/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![Download .deb](https://img.shields.io/badge/Descargar-.deb%20(325%20KB)-brightgreen.svg)](https://github.com/Miguel-Ubilla/Papel/raw/main/dist/papel_0.2.0-beta_amd64.deb)

*Rápido como un bloc de notas tradicional, pero con la elegancia nativa de GNOME y protección total contra cortes de energía.*

</div>

---

## ✨ Características Principales

* 📑 **Pestañas Nativas (`AdwTabView` & `AdwTabBar`):** Trabaja en múltiples documentos a la vez con pestañas arrastrables, cierre individual y alertas de modificaciones sin guardar (`*`).
* ⚡ **Protección de Datos Atómica (Crash & Power Loss Safe):**
  * Respaldo automático periódico cada 3 segundos en segundo plano.
  * Escritura atómica a disco con `fsync` y renombramiento seguro para garantizar que un corte de electricidad repentino nunca corrompa tu archivo.
  * Recuperación automática al reabrir la app tras un apagón con aviso visual, o manual desde el menú (*"Restaurar sesión anterior"*).
* 🔍 **Búsqueda Interactiva con Resaltado:**
  * Atajo rápido `Ctrl + F`.
  * Resalta **todas las apariciones en amarillo** y la coincidencia activa en **naranja vibrante**.
  * Navegación fluida con `Enter` (siguiente) y `Shift + Enter` (anterior) con conteo en tiempo real (`3/12`).
* ✂️ **Edición Estándar y Portapapeles:** Soporte total de Copiar, Cortar y Pegar mediante menú contextual y atajos universales.
* 🌐 **Soporte Multi-Idioma Automático (i18n):** Se adapta solo al idioma de tu Sistema Operativo (Español, Inglés, Portugués y Francés).
* 🎨 **Diseño Moderno de Ubuntu:** Integración 100% nativa con esquemas claros y oscuros del sistema y tipografías del entorno.
* 📦 **Super Ligero:** Binario nativo compilado de ~1 MB que inicia en milisegundos y consume una fracción ínfima de memoria RAM.

---

## ⌨️ Atajos de Teclado

| Atajo | Acción |
| :--- | :--- |
| `Ctrl + N` | Nueva pestaña (documento en blanco) |
| `Ctrl + O` | Abrir archivo existente |
| `Ctrl + S` | Guardar documento activo |
| `Ctrl + Shift + S` | Guardar como... (elegir destino) |
| `Ctrl + W` | Cerrar pestaña activa |
| `Ctrl + F` | Abrir / Alternar barra de búsqueda |
| `Enter` (en búsqueda) | Ir a la siguiente coincidencia |
| `Shift + Enter` (en búsqueda) | Ir a la coincidencia anterior |
| `Esc` | Cerrar búsqueda y limpiar resaltados |
| `Ctrl + C` | Copiar texto seleccionado |
| `Ctrl + X` | Cortar texto seleccionado |
| `Ctrl + V` | Pegar texto del portapapeles |

---

## 🚀 Instalación en Ubuntu

### Opción 1: Descarga directa del paquete `.deb` (Recomendado)

El paquete oficial precompilado está disponible en la carpeta [`dist/`](dist/):

📦 **[Descargar papel_0.2.0-beta_amd64.deb](https://github.com/Miguel-Ubilla/Papel/raw/main/dist/papel_0.2.0-beta_amd64.deb)** *(325 KB)*

#### ¿Cómo instalarlo tras descargarlo?
* **Gráficamente:** Haz doble clic sobre el archivo descargado para abrirlo con el instalador de paquetes de Ubuntu.
* **O desde la terminal:**
  ```bash
  sudo apt install ./dist/papel_0.2.0-beta_amd64.deb
  ```

---

### Opción 2: Compilar e instalar localmente desde código fuente

#### 1. Requisitos previos
Necesitas tener instalado el compilador de **Rust** y las cabeceras de desarrollo de **GTK4 / Libadwaita**:

```bash
# Dependencias en Ubuntu
sudo apt update
sudo apt install -y build-essential libgtk-4-dev libadwaita-1-dev

# Compilador de Rust (si aún no lo tienes instalado)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### 2. Clonar el repositorio
```bash
git clone https://github.com/Miguel-Ubilla/Papel.git
cd Papel
```

#### 3. Instalación local de usuario (sin necesidad de root/sudo)
```bash
chmod +x install.sh
./install.sh
```

El script compilará el binario en modo optimizado e integrará el icono y el lanzador en tu menú de aplicaciones de Ubuntu en segundos.

---

## 📦 Cómo generar tu propio paquete `.deb`

Para construir un nuevo instalador `.deb` redistribuible para Debian / Ubuntu:

```bash
chmod +x build-deb.sh
./build-deb.sh
```

Esto generará el archivo `dist/papel_0.2.0-beta_amd64.deb` listo para compartir.

---

## 🧱 Arquitectura del Código (Clean Code & SRP)

El proyecto sigue una estricta separación de responsabilidades:

```
papel/
├── src/
│   ├── main.rs              # Punto de entrada de ejecución
│   ├── lib.rs               # API de la aplicación y ciclo de vida GTK
│   ├── app.rs               # Ventana principal, menús, acciones y atajos
│   ├── model/
│   │   └── document.rs      # Modelo del documento y metadatos
│   ├── storage/
│   │   └── recovery.rs      # Motor atómico de autoguardado y persistencia
│   ├── ui/
│   │   ├── tab_view.rs      # Pestañas nativas con AdwTabView / TabBar
│   │   ├── search_bar.rs    # Motor de búsqueda interactivo y resaltado visual
│   │   └── dialogs.rs       # Diálogos de abrir y guardar archivos
│   └── utils/
│       └── clipboard.rs     # Utilidades de portapapeles
├── assets/                  # Iconos SVG y lanzador .desktop
├── tests/                   # Pruebas automatizadas de recuperación
├── install.sh               # Instalador/actualizador local
├── build-deb.sh             # Generador de paquete .deb
└── Cargo.toml               # Configuración del paquete y dependencias en Rust
```

---

## 📄 Licencia

Este proyecto está bajo la **Licencia MIT**, una licencia de código abierto totalmente permisiva. Puedes usarlo, modificarlo, redistribuirlo y utilizarlo de forma libre y sin restricciones. Consulta el archivo [LICENSE](LICENSE) para más información.
