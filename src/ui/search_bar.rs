use std::cell::Cell;
use std::rc::Rc;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{
    Box as GtkBox, Button, Label, Orientation, SearchBar, SearchEntry, TextBuffer,
    TextSearchFlags, TextView,
};
use crate::i18n::I18n;

const TAG_MATCH: &str = "search_match";
const TAG_CURRENT: &str = "search_current";

pub struct EditorSearchBar {
    container: SearchBar,
    entry: SearchEntry,
    matches_label: Label,
    current_match_idx: Rc<Cell<usize>>,
}

impl EditorSearchBar {
    pub fn new() -> Self {
        let i18n = I18n::get();
        let container = SearchBar::builder()
            .show_close_button(true)
            .build();

        let box_widget = GtkBox::new(Orientation::Horizontal, 6);
        box_widget.set_margin_start(8);
        box_widget.set_margin_end(8);
        box_widget.set_margin_top(4);
        box_widget.set_margin_bottom(4);

        let entry = SearchEntry::builder()
            .placeholder_text(i18n.search_placeholder)
            .width_request(240)
            .build();

        let prev_btn = Button::from_icon_name("go-up-symbolic");
        prev_btn.set_tooltip_text(Some(i18n.prev_match_tooltip));

        let next_btn = Button::from_icon_name("go-down-symbolic");
        next_btn.set_tooltip_text(Some(i18n.next_match_tooltip));

        let matches_label = Label::new(None);
        matches_label.add_css_class("dim-label");

        box_widget.append(&entry);
        box_widget.append(&prev_btn);
        box_widget.append(&next_btn);
        box_widget.append(&matches_label);

        container.set_child(Some(&box_widget));

        let current_match_idx = Rc::new(Cell::new(0));

        let sb = Self {
            container,
            entry,
            matches_label,
            current_match_idx,
        };

        let entry_clone = sb.entry.clone();
        prev_btn.connect_clicked(glib::clone!(
            #[weak]
            entry_clone,
            move |_| {
                entry_clone.emit_previous_match();
            }
        ));

        let entry_clone2 = sb.entry.clone();
        next_btn.connect_clicked(glib::clone!(
            #[weak]
            entry_clone2,
            move |_| {
                entry_clone2.emit_next_match();
            }
        ));

        sb
    }

    pub fn widget(&self) -> &SearchBar {
        &self.container
    }

    pub fn entry(&self) -> &SearchEntry {
        &self.entry
    }

    pub fn toggle(&self) {
        let active = self.container.is_search_mode();
        self.container.set_search_mode(!active);
        if !active {
            self.entry.grab_focus();
        }
    }

    pub fn open(&self) {
        self.container.set_search_mode(true);
        self.entry.grab_focus();
    }

    pub fn set_matches_text(&self, text: Option<&str>) {
        self.matches_label.set_text(text.unwrap_or(""));
    }

    pub fn current_match_idx(&self) -> &Rc<Cell<usize>> {
        &self.current_match_idx
    }

    /// Asegura que los tags de resaltado existan en la tabla del buffer.
    fn ensure_tags(buffer: &TextBuffer) {
        let tag_table = buffer.tag_table();
        if tag_table.lookup(TAG_MATCH).is_none() {
            let tag = gtk4::TextTag::builder()
                .name(TAG_MATCH)
                .background("#f6d32d") // Amarillo GNOME para todas las coincidencias
                .foreground("#000000") // Texto oscuro para máximo contraste
                .build();
            tag_table.add(&tag);
        }
        if tag_table.lookup(TAG_CURRENT).is_none() {
            let tag = gtk4::TextTag::builder()
                .name(TAG_CURRENT)
                .background("#ff7800") // Naranja brillante GNOME para la coincidencia activa
                .foreground("#ffffff") // Texto blanco
                .build();
            tag_table.add(&tag);
        }
    }

    /// Elimina todos los resaltados de búsqueda del buffer.
    pub fn clear_highlights(buffer: &TextBuffer) {
        Self::ensure_tags(buffer);
        let (start, end) = buffer.bounds();
        buffer.remove_tag_by_name(TAG_MATCH, &start, &end);
        buffer.remove_tag_by_name(TAG_CURRENT, &start, &end);
    }

    /// Resalta todas las coincidencias en amarillo y la activa en naranja,
    /// navegando hacia adelante, atrás o manteniendo la coincidencia según el cursor.
    pub fn execute_search(
        &self,
        view: &TextView,
        query: &str,
        direction: SearchDirection,
    ) -> Option<(usize, usize)> {
        let buffer = view.buffer();
        Self::clear_highlights(&buffer);

        if query.is_empty() {
            self.current_match_idx.set(0);
            return None;
        }

        let flags = TextSearchFlags::CASE_INSENSITIVE;
        let mut match_offsets: Vec<(i32, i32)> = Vec::new();

        let (mut scan_iter, end_bound) = buffer.bounds();
        while let Some((m_start, m_end)) = scan_iter.forward_search(query, flags, Some(&end_bound)) {
            match_offsets.push((m_start.offset(), m_end.offset()));
            scan_iter = m_end;
        }

        let total = match_offsets.len();
        if total == 0 {
            self.current_match_idx.set(0);
            return Some((0, 0));
        }

        // 1. Resaltar TODAS las apariciones en amarillo de fondo
        for &(start_off, end_off) in &match_offsets {
            let start_iter = buffer.iter_at_offset(start_off);
            let end_iter = buffer.iter_at_offset(end_off);
            buffer.apply_tag_by_name(TAG_MATCH, &start_iter, &end_iter);
        }

        // 2. Determinar cuál coincidencia es la activa
        let mut curr_idx = self.current_match_idx.get();

        match direction {
            SearchDirection::Next => {
                curr_idx = (curr_idx + 1) % total;
            }
            SearchDirection::Previous => {
                if curr_idx == 0 {
                    curr_idx = total.saturating_sub(1);
                } else {
                    curr_idx -= 1;
                }
            }
            SearchDirection::StayOrFindNearCursor => {
                let cursor_off = buffer.iter_at_mark(&buffer.get_insert()).offset();
                let mut found_nearest = 0;
                for (i, &(s_off, _)) in match_offsets.iter().enumerate() {
                    if s_off >= cursor_off {
                        found_nearest = i;
                        break;
                    }
                }
                curr_idx = found_nearest;
            }
        }

        self.current_match_idx.set(curr_idx);

        // 3. Resaltar la activa en naranja y desplazar la vista hasta ella
        let (active_start, active_end) = match_offsets[curr_idx];
        let mut start_iter = buffer.iter_at_offset(active_start);
        let end_iter = buffer.iter_at_offset(active_end);

        buffer.remove_tag_by_name(TAG_MATCH, &start_iter, &end_iter);
        buffer.apply_tag_by_name(TAG_CURRENT, &start_iter, &end_iter);

        // Mover el cursor y hacer scroll suave hasta la coincidencia
        buffer.place_cursor(&end_iter);
        view.scroll_to_iter(&mut start_iter, 0.2, false, 0.0, 0.0);

        Some((curr_idx + 1, total))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchDirection {
    Next,
    Previous,
    StayOrFindNearCursor,
}
