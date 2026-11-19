use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use gtk4::gio;
use gtk4::prelude::*;
#[allow(deprecated)]
use gtk4::{FileChooserAction, FileChooserDialog, ResponseType, Window};

use crate::i18n::I18n;

#[allow(deprecated)]
pub struct DialogHelper;

#[allow(deprecated)]
impl DialogHelper {
    pub fn show_open_dialog<F>(parent: &impl IsA<Window>, on_file_selected: F)
    where
        F: FnOnce(PathBuf) + 'static,
    {
        let i18n = I18n::get();
        let dialog = FileChooserDialog::new(
            Some(i18n.open_dialog_title),
            Some(parent),
            FileChooserAction::Open,
            &[
                (i18n.btn_cancel, ResponseType::Cancel),
                (i18n.btn_open, ResponseType::Accept),
            ],
        );
        dialog.set_modal(true);

        let initial_dir = dirs::document_dir().or_else(dirs::home_dir);
        if let Some(dir) = initial_dir {
            let gio_dir = gio::File::for_path(dir);
            let _ = dialog.set_current_folder(Some(&gio_dir));
        }

        let cb = Rc::new(RefCell::new(Some(on_file_selected)));
        dialog.connect_response(move |dialog, response| {
            if response == ResponseType::Accept {
                if let Some(file) = dialog.file() {
                    if let Some(path) = file.path() {
                        if let Some(callback) = cb.borrow_mut().take() {
                            callback(path);
                        }
                    }
                }
            }
            dialog.destroy();
        });

        dialog.present();
    }

    pub fn show_save_as_dialog<F>(
        parent: &impl IsA<Window>,
        current_name: &str,
        existing_path: Option<&Path>,
        on_file_selected: F,
    ) where
        F: FnOnce(PathBuf) + 'static,
    {
        let i18n = I18n::get();
        let dialog = FileChooserDialog::new(
            Some(i18n.save_as_dialog_title),
            Some(parent),
            FileChooserAction::Save,
            &[
                (i18n.btn_cancel, ResponseType::Cancel),
                (i18n.btn_save, ResponseType::Accept),
            ],
        );
        dialog.set_modal(true);

        // Carpeta de destino
        let initial_dir = if let Some(p) = existing_path.and_then(|p| p.parent()) {
            Some(p.to_path_buf())
        } else {
            dirs::document_dir().or_else(dirs::home_dir)
        };

        if let Some(dir) = initial_dir {
            let gio_dir = gio::File::for_path(dir);
            let _ = dialog.set_current_folder(Some(&gio_dir));
        }

        dialog.set_current_name(current_name);

        let cb = Rc::new(RefCell::new(Some(on_file_selected)));
        dialog.connect_response(move |dialog, response| {
            if response == ResponseType::Accept {
                if let Some(file) = dialog.file() {
                    if let Some(path) = file.path() {
                        if let Some(callback) = cb.borrow_mut().take() {
                            callback(path);
                        }
                    }
                }
            }
            dialog.destroy();
        });

        dialog.present();
    }
}
