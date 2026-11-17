use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::{ScrolledWindow, TextBuffer, TextView, WrapMode};
use libadwaita::{TabBar, TabPage, TabView};

use crate::model::document::DocumentMetadata;
use crate::storage::recovery::TabSnapshot;

pub struct TabPageData {
    pub view: TextView,
    pub buffer: TextBuffer,
    pub metadata: Rc<RefCell<DocumentMetadata>>,
}

pub struct EditorTabManager {
    tab_view: TabView,
    tab_bar: TabBar,
    pages_data: Rc<RefCell<HashMap<String, TabPageData>>>, // indexed by DocumentMetadata.id
}

impl EditorTabManager {
    pub fn new() -> Self {
        let tab_view = TabView::new();
        let tab_bar = TabBar::new();
        tab_bar.set_view(Some(&tab_view));
        tab_bar.set_autohide(false);

        Self {
            tab_view,
            tab_bar,
            pages_data: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn tab_view(&self) -> &TabView {
        &self.tab_view
    }

    pub fn tab_bar(&self) -> &TabBar {
        &self.tab_bar
    }

    /// Crea y añade una nueva pestaña con texto inicial opcional y metadatos.
    pub fn add_tab(
        &self,
        metadata: DocumentMetadata,
        content: &str,
        select: bool,
    ) -> TabPage {
        let buffer = TextBuffer::new(None);
        buffer.set_text(content);

        let view = TextView::builder()
            .buffer(&buffer)
            .wrap_mode(WrapMode::WordChar)
            .monospace(false)
            .left_margin(12)
            .right_margin(12)
            .top_margin(10)
            .bottom_margin(10)
            .build();

        let id = metadata.id.clone();

        let scrolled = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Automatic)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .child(&view)
            .build();
        
        // Asignamos el ID único al widget contenedor para búsqueda directa y determinista
        scrolled.set_widget_name(&id);

        let page = self.tab_view.append(&scrolled);
        page.set_title(&metadata.title);

        let metadata_rc = Rc::new(RefCell::new(metadata));

        // Detectar cambios en el buffer para marcar como modificado (dirty)
        let page_clone = page.clone();
        let meta_clone = metadata_rc.clone();
        buffer.connect_changed(move |_| {
            let mut meta = meta_clone.borrow_mut();
            if !meta.is_dirty {
                meta.is_dirty = true;
                page_clone.set_indicator_icon(Some(&gtk4::gio::ThemedIcon::new("media-record-symbolic")));
                page_clone.set_title(&format!("* {}", meta.title));
            }
        });

        self.pages_data.borrow_mut().insert(
            id,
            TabPageData {
                view,
                buffer,
                metadata: metadata_rc,
            },
        );

        if select {
            self.tab_view.set_selected_page(&page);
        }

        page
    }

    /// Devuelve los datos de la pestaña activa actualmente.
    pub fn get_active_data(&self) -> Option<TabPageData> {
        let page = self.tab_view.selected_page()?;
        let scrolled = page.child().downcast::<ScrolledWindow>().ok()?;
        let id = scrolled.widget_name().to_string();

        let pages = self.pages_data.borrow();
        let data = pages.get(&id)?;

        Some(TabPageData {
            view: data.view.clone(),
            buffer: data.buffer.clone(),
            metadata: data.metadata.clone(),
        })
    }

    /// Guarda el contenido actual del documento activo en disco.
    pub fn save_current_to_path(&self, path: PathBuf) -> Result<(), String> {
        let active = self.get_active_data().ok_or("No hay pestaña activa")?;
        let (start, end) = active.buffer.bounds();
        let content = active.buffer.text(&start, &end, false).to_string();

        fs::write(&path, content).map_err(|e| format!("Error guardando archivo: {}", e))?;

        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("documento.txt")
            .to_string();

        let mut meta = active.metadata.borrow_mut();
        meta.file_path = Some(path);
        meta.title = file_name.clone();
        meta.is_dirty = false;

        if let Some(page) = self.tab_view.selected_page() {
            page.set_title(&file_name);
            page.set_indicator_icon(None::<&gtk4::gio::ThemedIcon>);
        }

        Ok(())
    }

    /// Obtiene snapshots de todas las pestañas para el autoguardado de recuperación.
    pub fn collect_snapshots(&self) -> Vec<TabSnapshot> {
        let n = self.tab_view.n_pages();
        let mut snapshots = Vec::new();
        let pages = self.pages_data.borrow();

        for i in 0..n {
            let page = self.tab_view.nth_page(i);
            if let Some(scrolled) = page.child().downcast::<ScrolledWindow>().ok() {
                let id = scrolled.widget_name().to_string();
                if let Some(data) = pages.get(&id) {
                    let (start, end) = data.buffer.bounds();
                    let content = data.buffer.text(&start, &end, false).to_string();
                    let meta = data.metadata.borrow().clone();
                    snapshots.push(TabSnapshot {
                        metadata: meta,
                        content,
                    });
                }
            }
        }

        snapshots
    }

    pub fn active_index(&self) -> usize {
        if let Some(page) = self.tab_view.selected_page() {
            self.tab_view.page_position(&page) as usize
        } else {
            0
        }
    }

    pub fn remove_page_data(&self, page: &TabPage) -> Option<String> {
        if let Some(scrolled) = page.child().downcast::<ScrolledWindow>().ok() {
            let id = scrolled.widget_name().to_string();
            self.pages_data.borrow_mut().remove(&id);
            return Some(id);
        }
        None
    }
}
