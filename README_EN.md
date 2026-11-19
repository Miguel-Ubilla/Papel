<div align="center">

# 📄 Papel

**A modern, lightweight, crash-resilient text editor for Ubuntu / GNOME, built with Rust.**

[🇬🇧 English](README_EN.md) | [🇪🇸 Español](README.md)

[![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange.svg?logo=rust)](https://www.rust-lang.org/)
[![GTK4](https://img.shields.io/badge/GTK-4.14-blue.svg?logo=gnome)](https://www.gtk.org/)
[![Libadwaita](https://img.shields.io/badge/Libadwaita-1.5-blue.svg)](https://gnome.pages.gitlab.gnome.org/libadwaita/)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![Download .deb](https://img.shields.io/badge/Download-.deb%20(325%20KB)-brightgreen.svg)](https://github.com/Miguel-Ubilla/Papel/raw/main/dist/papel_0.2.0-beta_amd64.deb)

*As fast and clean as a classic notepad, with the modern elegance of GNOME and bulletproof power-loss protection.*

</div>

---

## ✨ Key Features

* 📑 **Native Tabs (`AdwTabView` & `AdwTabBar`):** Work across multiple documents simultaneously with draggable tabs, individual tab close buttons, and unsaved changes indicators (`*`).
* ⚡ **Atomic Data Protection (Crash & Power Loss Safe):**
  * Automatic periodic background snapshots every 3 seconds.
  * Atomic disk writes using `fsync` and atomic renaming, ensuring an unexpected power cut or system shutdown never leaves a corrupt file.
  * Automatic session recovery upon reopening after a crash or power cut, plus a manual menu option (*"Restore previous session"*).
* 🔍 **Interactive Search with Visual Highlights:**
  * Quick shortcut `Ctrl + F`.
  * Highlights **all matches in yellow** and the active match in **vibrant orange**.
  * Seamless navigation with `Enter` (next) and `Shift + Enter` (previous), displaying real-time counters (`3/12`).
* ✂️ **Standard Editing & Clipboard:** Full Cut, Copy, and Paste support via context menu and universal keyboard shortcuts.
* 🌐 **Automatic Multi-Language Support (i18n):** Automatically adapts to your Operating System language (English, Spanish, Portuguese, and French supported).
* 🎨 **Native Ubuntu / GNOME Design:** Adapts automatically to system light/dark style schemes and typography.
* 📦 **Ultra Lightweight:** Compiled native binary (~1 MB) that starts in milliseconds and consumes minimal RAM.

---

## ⌨️ Keyboard Shortcuts

| Shortcut | Action |
| :--- | :--- |
| `Ctrl + N` | New tab (blank document) |
| `Ctrl + O` | Open existing file |
| `Ctrl + S` | Save active document |
| `Ctrl + Shift + S` | Save As... (choose location) |
| `Ctrl + W` | Close active tab |
| `Ctrl + F` | Open / Toggle find bar |
| `Enter` (in search) | Jump to next match |
| `Shift + Enter` (in search) | Jump to previous match |
| `Esc` | Close search and clear highlights |
| `Ctrl + C` | Copy selected text |
| `Ctrl + X` | Cut selected text |
| `Ctrl + V` | Paste clipboard text |

---

## 🚀 Installation on Ubuntu / Debian

### Option 1: Direct `.deb` package download (Recommended)

Precompiled packages are available in the [`dist/`](dist/) folder:

📦 **[Download papel_0.2.0-beta_amd64.deb](https://github.com/Miguel-Ubilla/Papel/raw/main/dist/papel_0.2.0-beta_amd64.deb)** *(325 KB)*

#### How to install after downloading?
* **Graphically:** Double-click the downloaded `.deb` file to open and install it with the Ubuntu Software Center.
* **Via terminal:**
  ```bash
  sudo apt install ./dist/papel_0.2.0-beta_amd64.deb
  ```

---

### Option 2: Build and install locally from source

#### 1. Prerequisites
You need the **Rust** toolchain and the **GTK4 / Libadwaita** development headers:

```bash
# Ubuntu system packages
sudo apt update
sudo apt install -y build-essential libgtk-4-dev libadwaita-1-dev

# Rust compiler (if not installed yet)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### 2. Clone the repository
```bash
git clone https://github.com/Miguel-Ubilla/Papel.git
cd Papel
```

#### 3. Local user installation (no sudo required)
```bash
chmod +x install.sh
./install.sh
```

The script compiles the release-optimized binary and registers the desktop launcher and icon in your Ubuntu application menu.

To run it anytime from terminal:
```bash
papel
# Or open a file directly:
papel myfile.txt
```

---

## 📦 How to build your own `.deb` package

To build a fresh redistributable Debian/Ubuntu package:

```bash
chmod +x build-deb.sh
./build-deb.sh
```

This creates `dist/papel_0.2.0-beta_amd64.deb` ready to be shared.

---

## 🧱 Software Architecture (Clean Code & SRP)

The project adheres strictly to the Single Responsibility Principle:

```
papel/
├── src/
│   ├── main.rs              # Program entrypoint
│   ├── lib.rs               # Application lifecycle & GTK setup
│   ├── app.rs               # Main window, menus, actions & shortcuts
│   ├── i18n.rs              # OS locale detection & multi-language translations
│   ├── model/
│   │   └── document.rs      # Document representation and metadata
│   ├── storage/
│   │   └── recovery.rs      # Atomic persistence and power-loss recovery engine
│   ├── ui/
│   │   ├── tab_view.rs      # Native tabs with AdwTabView / TabBar
│   │   ├── search_bar.rs    # Interactive search engine & visual highlighter
│   │   └── dialogs.rs       # In-process file chooser dialogs
│   └── utils/
│       └── clipboard.rs     # Clipboard helper utilities
├── assets/                  # High-resolution SVG icons & .desktop file
├── dist/                    # Ready-to-install .deb packages
├── tests/                   # Automated recovery & i18n unit tests
├── install.sh               # Local user installer / updater script
├── build-deb.sh             # .deb package builder
└── Cargo.toml               # Rust package metadata & dependencies
```

---

## 📄 License

This project is licensed under the **MIT License**, a fully permissive open-source license. You are free to use, modify, distribute, and integrate it without restrictions. See the [LICENSE](LICENSE) file for details.

