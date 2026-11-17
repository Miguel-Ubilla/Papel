use gtk4::prelude::*;
use gtk4::{TextBuffer, TextView};

/// Utilidades de portapapeles para operaciones explícitas (Cortar, Copiar, Pegar).
pub struct ClipboardUtil;

impl ClipboardUtil {
    pub fn copy_active(view: &TextView) {
        let display = gtk4::gdk::Display::default().unwrap();
        let clipboard = display.clipboard();
        if let Some(buffer) = view.buffer().downcast::<TextBuffer>().ok() {
            buffer.copy_clipboard(&clipboard);
        }
    }

    pub fn cut_active(view: &TextView) {
        let display = gtk4::gdk::Display::default().unwrap();
        let clipboard = display.clipboard();
        if let Some(buffer) = view.buffer().downcast::<TextBuffer>().ok() {
            buffer.cut_clipboard(&clipboard, true);
        }
    }

    pub fn paste_active(view: &TextView) {
        let display = gtk4::gdk::Display::default().unwrap();
        let clipboard = display.clipboard();
        if let Some(buffer) = view.buffer().downcast::<TextBuffer>().ok() {
            buffer.paste_clipboard(&clipboard, None, true);
        }
    }
}

