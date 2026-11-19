use std::fs;
use std::rc::Rc;

use gtk4::gdk::{Key, ModifierType};
use gtk4::prelude::*;
use gtk4::{
    Box as GtkBox, Button, EventControllerKey, MenuButton, Orientation,
    PopoverMenu, gio, glib,
};
use libadwaita::prelude::*;
use libadwaita::{Application, ApplicationWindow, Banner, HeaderBar};

use crate::i18n::I18n;
use crate::model::document::DocumentMetadata;
use crate::storage::recovery::RecoveryManager;
use crate::ui::dialogs::DialogHelper;
use crate::ui::search_bar::{EditorSearchBar, SearchDirection};
use crate::ui::tab_view::EditorTabManager;
use crate::utils::clipboard::ClipboardUtil;

pub struct PapelApp {
    window: ApplicationWindow,
    tab_manager: Rc<EditorTabManager>,
    search_bar: Rc<EditorSearchBar>,
    recovery_mgr: Rc<RecoveryManager>,
    banner: Banner,
}

impl PapelApp {
    pub fn build(app: &Application) -> Self {
        let i18n = I18n::get();

        let window = ApplicationWindow::builder()
            .application(app)
            .title(i18n.app_title)
            .default_width(850)
            .default_height(600)
            .build();

        let recovery_mgr = Rc::new(RecoveryManager::new().expect("Error al inicializar recovery"));
        let tab_manager = Rc::new(EditorTabManager::new());
        let search_bar = Rc::new(EditorSearchBar::new());

        let banner = Banner::builder()
            .title(i18n.banner_recovered_title)
            .button_label(i18n.banner_understood_btn)
            .build();

        let banner_clone = banner.clone();
        banner.connect_button_clicked(move |_| {
            banner_clone.set_revealed(false);
        });

        // Contenedor principal vertical
        let main_box = GtkBox::new(Orientation::Vertical, 0);

        // HeaderBar
        let header = HeaderBar::new();

        // Botón Nueva Pestaña (+)
        let new_tab_btn = Button::from_icon_name("tab-new-symbolic");
        new_tab_btn.set_tooltip_text(Some(i18n.new_tab_tooltip));
        header.pack_start(&new_tab_btn);

        // Botón Abrir
        let open_btn = Button::from_icon_name("document-open-symbolic");
        open_btn.set_tooltip_text(Some(i18n.open_tooltip));
        header.pack_start(&open_btn);

        // Botón Guardar
        let save_btn = Button::from_icon_name("document-save-symbolic");
        save_btn.set_tooltip_text(Some(i18n.save_tooltip));
        header.pack_start(&save_btn);

        // Botón Búsqueda
        let search_btn = Button::from_icon_name("edit-find-symbolic");
        search_btn.set_tooltip_text(Some(i18n.search_tooltip));
        header.pack_end(&search_btn);

        // Menú secundario (Hamburguesa)
        let menu_model = gio::Menu::new();
        menu_model.append(Some(i18n.restore_session), Some("app.restore-session"));
        menu_model.append(Some(i18n.save_as), Some("app.save-as"));
        menu_model.append(Some(i18n.close_tab), Some("app.close-tab"));

        let edit_section = gio::Menu::new();
        edit_section.append(Some(i18n.cut), Some("app.cut"));
        edit_section.append(Some(i18n.copy), Some("app.copy"));
        edit_section.append(Some(i18n.paste), Some("app.paste"));
        menu_model.append_section(Some(i18n.edit_section), &edit_section);

        let help_section = gio::Menu::new();
        help_section.append(Some(i18n.about_app), Some("app.about"));
        menu_model.append_section(None, &help_section);

        let popover = PopoverMenu::from_model(Some(&menu_model));
        let menu_btn = MenuButton::builder()
            .icon_name("open-menu-symbolic")
            .popover(&popover)
            .tooltip_text(i18n.more_options_tooltip)
            .build();
        header.pack_end(&menu_btn);

        main_box.append(&header);
        main_box.append(&banner);
        main_box.append(tab_manager.tab_bar());
        main_box.append(search_bar.widget());

        let tab_view = tab_manager.tab_view().clone();
        tab_view.set_vexpand(true);
        main_box.append(&tab_view);

        window.set_content(Some(&main_box));

        let app_instance = Self {
            window,
            tab_manager,
            search_bar,
            recovery_mgr,
            banner,
        };

        app_instance.setup_actions(app);
        app_instance.setup_events(&new_tab_btn, &open_btn, &save_btn, &search_btn);
        app_instance.setup_keyboard_shortcuts();
        app_instance.setup_autosave();
        app_instance.load_initial_session();

        app_instance
    }

    pub fn present(&self) {
        self.window.present();
    }

    fn setup_events(
        &self,
        new_tab_btn: &Button,
        open_btn: &Button,
        save_btn: &Button,
        search_btn: &Button,
    ) {
        let i18n = I18n::get();
        let tab_mgr = self.tab_manager.clone();
        new_tab_btn.connect_clicked(move |_| {
            let meta = DocumentMetadata::new_empty(i18n.untitled.to_string());
            tab_mgr.add_tab(meta, "", true);
        });

        let tab_mgr2 = self.tab_manager.clone();
        let win_clone = self.window.clone();
        open_btn.connect_clicked(move |_| {
            let tab_mgr_inner = tab_mgr2.clone();
            DialogHelper::show_open_dialog(&win_clone, move |path| {
                if let Ok(content) = fs::read_to_string(&path) {
                    let title = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or(i18n.default_doc_filename)
                        .to_string();
                    let meta = DocumentMetadata::from_file(path, title);
                    tab_mgr_inner.add_tab(meta, &content, true);
                }
            });
        });

        let tab_mgr3 = self.tab_manager.clone();
        let win_clone2 = self.window.clone();
        save_btn.connect_clicked(move |_| {
            Self::trigger_save(&tab_mgr3, &win_clone2, false);
        });

        let sb = self.search_bar.clone();
        search_btn.connect_clicked(move |_| {
            sb.toggle();
        });

        // Búsqueda interactiva en el SearchEntry con resaltado visual completo
        let tab_mgr4 = self.tab_manager.clone();
        let sb_clone = self.search_bar.clone();
        self.search_bar.entry().connect_search_changed(move |entry| {
            let query = entry.text().to_string();
            if let Some(active) = tab_mgr4.get_active_data() {
                if let Some((idx, total)) = sb_clone.execute_search(&active.view, &query, SearchDirection::StayOrFindNearCursor) {
                    if total > 0 {
                        sb_clone.set_matches_text(Some(&format!("{}/{}", idx, total)));
                    } else {
                        sb_clone.set_matches_text(Some(i18n.zero_matches));
                    }
                } else {
                    sb_clone.set_matches_text(None);
                }
            }
        });

        let tab_mgr5 = self.tab_manager.clone();
        let sb_clone2 = self.search_bar.clone();
        self.search_bar.entry().connect_next_match(move |entry| {
            let query = entry.text().to_string();
            if let Some(active) = tab_mgr5.get_active_data() {
                if let Some((idx, total)) = sb_clone2.execute_search(&active.view, &query, SearchDirection::Next) {
                    if total > 0 {
                        sb_clone2.set_matches_text(Some(&format!("{}/{}", idx, total)));
                    } else {
                        sb_clone2.set_matches_text(Some(i18n.zero_matches));
                    }
                }
            }
        });

        let tab_mgr6 = self.tab_manager.clone();
        let sb_clone3 = self.search_bar.clone();
        self.search_bar.entry().connect_previous_match(move |entry| {
            let query = entry.text().to_string();
            if let Some(active) = tab_mgr6.get_active_data() {
                if let Some((idx, total)) = sb_clone3.execute_search(&active.view, &query, SearchDirection::Previous) {
                    if total > 0 {
                        sb_clone3.set_matches_text(Some(&format!("{}/{}", idx, total)));
                    } else {
                        sb_clone3.set_matches_text(Some(i18n.zero_matches));
                    }
                }
            }
        });

        // Limpiar resaltados al cerrar la barra de búsqueda
        let tab_mgr_clear = self.tab_manager.clone();
        let sb_clear = self.search_bar.clone();
        self.search_bar.widget().connect_search_mode_enabled_notify(move |sb_widget| {
            if !sb_widget.is_search_mode() {
                if let Some(active) = tab_mgr_clear.get_active_data() {
                    EditorSearchBar::clear_highlights(&active.buffer);
                }
                sb_clear.set_matches_text(None);
            }
        });

        // Actualizar búsqueda al cambiar de pestaña activa
        let sb_tab_switch = self.search_bar.clone();
        let tab_mgr_switch = self.tab_manager.clone();
        self.tab_manager.tab_view().connect_selected_page_notify(move |_| {
            if let Some(active) = tab_mgr_switch.get_active_data() {
                if sb_tab_switch.widget().is_search_mode() {
                    let query = sb_tab_switch.entry().text().to_string();
                    if !query.is_empty() {
                        if let Some((idx, total)) = sb_tab_switch.execute_search(&active.view, &query, SearchDirection::StayOrFindNearCursor) {
                            if total > 0 {
                                sb_tab_switch.set_matches_text(Some(&format!("{}/{}", idx, total)));
                            } else {
                                sb_tab_switch.set_matches_text(Some(i18n.zero_matches));
                            }
                        }
                    }
                }
            }
        });

        // Cerrar pestaña en TabView
        let tab_mgr7 = self.tab_manager.clone();
        let recovery_mgr_clone = self.recovery_mgr.clone();
        self.tab_manager.tab_view().connect_close_page(move |view, page| {
            if let Some(id) = tab_mgr7.remove_page_data(page) {
                recovery_mgr_clone.remove_tab_recovery(&id);
            }
            view.close_page_finish(page, true);

            // Si se cerraron todas las pestañas, abrir una vacía
            if view.n_pages() == 0 {
                let meta = DocumentMetadata::new_empty(i18n.untitled.to_string());
                tab_mgr7.add_tab(meta, "", true);
            }

            glib::Propagation::Stop
        });
    }

    fn setup_actions(&self, app: &Application) {
        let i18n = I18n::get();
        let tab_mgr_restore = self.tab_manager.clone();
        let recovery_mgr_restore = self.recovery_mgr.clone();
        let banner_restore = self.banner.clone();
        let restore_action = gio::SimpleAction::new("restore-session", None);
        restore_action.connect_activate(move |_, _| {
            if let Some((recovered_tabs, active_idx)) = recovery_mgr_restore.load_recovered_session() {
                if !recovered_tabs.is_empty() {
                    for (i, tab_data) in recovered_tabs.into_iter().enumerate() {
                        let select = i == active_idx;
                        tab_mgr_restore.add_tab(tab_data.metadata, &tab_data.content, select);
                    }
                    banner_restore.set_title(i18n.banner_restored_title);
                    banner_restore.set_revealed(true);
                }
            }
        });
        app.add_action(&restore_action);

        let tab_mgr = self.tab_manager.clone();
        let win = self.window.clone();
        let save_as_action = gio::SimpleAction::new("save-as", None);
        save_as_action.connect_activate(move |_, _| {
            Self::trigger_save(&tab_mgr, &win, true);
        });
        app.add_action(&save_as_action);

        let tab_mgr2 = self.tab_manager.clone();
        let close_tab_action = gio::SimpleAction::new("close-tab", None);
        close_tab_action.connect_activate(move |_, _| {
            if let Some(page) = tab_mgr2.tab_view().selected_page() {
                tab_mgr2.tab_view().close_page(&page);
            }
        });
        app.add_action(&close_tab_action);

        // Clipboard actions
        let tab_mgr_cut = self.tab_manager.clone();
        let cut_action = gio::SimpleAction::new("cut", None);
        cut_action.connect_activate(move |_, _| {
            if let Some(active) = tab_mgr_cut.get_active_data() {
                ClipboardUtil::cut_active(&active.view);
            }
        });
        app.add_action(&cut_action);

        let tab_mgr_copy = self.tab_manager.clone();
        let copy_action = gio::SimpleAction::new("copy", None);
        copy_action.connect_activate(move |_, _| {
            if let Some(active) = tab_mgr_copy.get_active_data() {
                ClipboardUtil::copy_active(&active.view);
            }
        });
        app.add_action(&copy_action);

        let tab_mgr_paste = self.tab_manager.clone();
        let paste_action = gio::SimpleAction::new("paste", None);
        paste_action.connect_activate(move |_, _| {
            if let Some(active) = tab_mgr_paste.get_active_data() {
                ClipboardUtil::paste_active(&active.view);
            }
        });
        app.add_action(&paste_action);

        // Acción Acerca de Papel
        let win_about = self.window.clone();
        let about_action = gio::SimpleAction::new("about", None);
        about_action.connect_activate(move |_, _| {
            let about = libadwaita::AboutWindow::builder()
                .transient_for(&win_about)
                .application_name(i18n.app_title)
                .application_icon("io.github.papel.editor")
                .developer_name("Miguel Ubilla")
                .version("0.2.0-beta")
                .comments(i18n.about_comments)
                .license_type(gtk4::License::MitX11)
                .copyright(i18n.about_copyright)
                .website("https://github.com/Miguel-Ubilla/Papel")
                .issue_url("https://github.com/Miguel-Ubilla/Papel/issues")
                .build();
            about.present();
        });
        app.add_action(&about_action);
    }

    fn setup_keyboard_shortcuts(&self) {
        let i18n = I18n::get();
        let key_controller = EventControllerKey::new();
        let tab_mgr = self.tab_manager.clone();
        let win = self.window.clone();
        let sb = self.search_bar.clone();

        key_controller.connect_key_pressed(move |_, key, _, modifier| {
            let is_ctrl = modifier.contains(ModifierType::CONTROL_MASK);
            let is_shift = modifier.contains(ModifierType::SHIFT_MASK);

            if is_ctrl {
                match key {
                    Key::n | Key::N => {
                        let meta = DocumentMetadata::new_empty(i18n.untitled.to_string());
                        tab_mgr.add_tab(meta, "", true);
                        return glib::Propagation::Stop;
                    }
                    Key::o | Key::O => {
                        let tab_mgr_inner = tab_mgr.clone();
                        DialogHelper::show_open_dialog(&win, move |path| {
                            if let Ok(content) = fs::read_to_string(&path) {
                                let title = path
                                    .file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or(i18n.default_doc_filename)
                                    .to_string();
                                let meta = DocumentMetadata::from_file(path, title);
                                tab_mgr_inner.add_tab(meta, &content, true);
                            }
                        });
                        return glib::Propagation::Stop;
                    }
                    Key::s | Key::S => {
                        Self::trigger_save(&tab_mgr, &win, is_shift);
                        return glib::Propagation::Stop;
                    }
                    Key::w | Key::W => {
                        if let Some(page) = tab_mgr.tab_view().selected_page() {
                            tab_mgr.tab_view().close_page(&page);
                        }
                        return glib::Propagation::Stop;
                    }
                    Key::f | Key::F => {
                        sb.open();
                        return glib::Propagation::Stop;
                    }
                    _ => {}
                }
            } else if key == Key::Escape {
                if sb.widget().is_search_mode() {
                    sb.widget().set_search_mode(false);
                    return glib::Propagation::Stop;
                }
            }

            glib::Propagation::Proceed
        });

        self.window.add_controller(key_controller);
    }

    /// Autoguardado periódico en segundo plano: cada 3 segundos.
    fn setup_autosave(&self) {
        let tab_mgr = self.tab_manager.clone();
        let recovery_mgr = self.recovery_mgr.clone();

        glib::timeout_add_seconds_local(3, move || {
            let snapshots = tab_mgr.collect_snapshots();
            if !snapshots.is_empty() {
                let active_idx = tab_mgr.active_index();
                let _ = recovery_mgr.save_session_atomic(&snapshots, active_idx);
            }
            glib::ControlFlow::Continue
        });
    }

    /// Restaura sesión anterior si existía (corte de energía o fallo previo) o inicia una pestaña vacía.
    fn load_initial_session(&self) {
        let i18n = I18n::get();
        if let Some((recovered_tabs, active_idx)) = self.recovery_mgr.load_recovered_session() {
            let has_content = recovered_tabs.iter().any(|t| !t.content.is_empty() || t.metadata.is_dirty);
            if has_content {
                for (i, tab_data) in recovered_tabs.into_iter().enumerate() {
                    let select = i == active_idx;
                    self.tab_manager.add_tab(tab_data.metadata, &tab_data.content, select);
                }
                self.banner.set_revealed(true);
                return;
            }
        }

        // Si no hay sesión previa para restaurar, abrir pestaña en blanco
        let meta = DocumentMetadata::new_empty(i18n.untitled.to_string());
        self.tab_manager.add_tab(meta, "", true);
    }

    fn trigger_save(tab_mgr: &Rc<EditorTabManager>, parent_win: &ApplicationWindow, force_save_as: bool) {
        let active = match tab_mgr.get_active_data() {
            Some(a) => a,
            None => {
                eprintln!("trigger_save: No hay pestaña activa");
                return;
            }
        };

        let current_path = {
            let meta = active.metadata.borrow();
            meta.file_path.clone()
        };

        if !force_save_as && current_path.is_some() {
            if let Some(path) = current_path {
                let _ = tab_mgr.save_current_to_path(path);
            }
        } else {
            let default_name = {
                let meta = active.metadata.borrow();
                let clean = meta.title.trim_start_matches("* ").trim().to_string();
                if clean.contains('.') {
                    clean
                } else {
                    format!("{}.txt", clean)
                }
            };
            let tab_mgr_clone = tab_mgr.clone();
            let path_ref = current_path.as_deref();
            DialogHelper::show_save_as_dialog(parent_win, &default_name, path_ref, move |path| {
                let _ = tab_mgr_clone.save_current_to_path(path);
            });
        }
    }
}
