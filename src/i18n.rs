use std::env;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Spanish,
    English,
    Portuguese,
    French,
}

#[derive(Debug, Clone)]
pub struct I18n {
    pub app_title: &'static str,
    pub untitled: &'static str,
    pub default_doc_filename: &'static str,

    // Tooltips y Botones de la barra principal
    pub new_tab_tooltip: &'static str,
    pub open_tooltip: &'static str,
    pub save_tooltip: &'static str,
    pub search_tooltip: &'static str,
    pub more_options_tooltip: &'static str,

    // Menú secundario
    pub restore_session: &'static str,
    pub save_as: &'static str,
    pub close_tab: &'static str,
    pub edit_section: &'static str,
    pub cut: &'static str,
    pub copy: &'static str,
    pub paste: &'static str,
    pub about_app: &'static str,

    // Banners de recuperación
    pub banner_recovered_title: &'static str,
    pub banner_restored_title: &'static str,
    pub banner_understood_btn: &'static str,

    // Barra de búsqueda
    pub search_placeholder: &'static str,
    pub prev_match_tooltip: &'static str,
    pub next_match_tooltip: &'static str,
    pub zero_matches: &'static str,

    // Diálogos de archivo
    pub open_dialog_title: &'static str,
    pub save_as_dialog_title: &'static str,
    pub btn_cancel: &'static str,
    pub btn_open: &'static str,
    pub btn_save: &'static str,

    // Acerca de
    pub about_comments: &'static str,
    pub about_copyright: &'static str,
}

impl I18n {
    pub fn get() -> &'static Self {
        static INSTANCE: std::sync::OnceLock<I18n> = std::sync::OnceLock::new();
        INSTANCE.get_or_init(|| {
            let lang = Self::detect_system_language();
            Self::for_language(lang)
        })
    }

    /// Detecta el idioma del sistema operativo inspeccionando las variables de entorno POSIX/Linux.
    pub fn detect_system_language() -> Language {
        let locale = env::var("LC_ALL")
            .or_else(|_| env::var("LC_MESSAGES"))
            .or_else(|_| env::var("LANG"))
            .or_else(|_| env::var("LANGUAGE"))
            .unwrap_or_default()
            .to_lowercase();

        if locale.starts_with("es") {
            Language::Spanish
        } else if locale.starts_with("pt") {
            Language::Portuguese
        } else if locale.starts_with("fr") {
            Language::French
        } else {
            // Predeterminado internacional: Inglés
            Language::English
        }
    }

    pub fn for_language(lang: Language) -> Self {
        match lang {
            Language::Spanish => Self {
                app_title: "Papel",
                untitled: "Sin título",
                default_doc_filename: "documento.txt",

                new_tab_tooltip: "Nuevo documento (Ctrl+N)",
                open_tooltip: "Abrir archivo (Ctrl+O)",
                save_tooltip: "Guardar (Ctrl+S)",
                search_tooltip: "Buscar (Ctrl+F)",
                more_options_tooltip: "Opciones adicionales",

                restore_session: "Restaurar sesión anterior",
                save_as: "Guardar como...",
                close_tab: "Cerrar pestaña",
                edit_section: "Edición",
                cut: "Cortar",
                copy: "Copiar",
                paste: "Pegar",
                about_app: "Acerca de Papel",

                banner_recovered_title: "Se recuperaron documentos no guardados de una sesión anterior.",
                banner_restored_title: "Se restauraron las pestañas de la sesión respaldada.",
                banner_understood_btn: "Entendido",

                search_placeholder: "Buscar en documento...",
                prev_match_tooltip: "Anterior (Shift+Enter)",
                next_match_tooltip: "Siguiente (Enter)",
                zero_matches: "0 coincidencias",

                open_dialog_title: "Abrir archivo",
                save_as_dialog_title: "Guardar como",
                btn_cancel: "Cancelar",
                btn_open: "Abrir",
                btn_save: "Guardar",

                about_comments: "Bloc de notas moderno, rápido y seguro con pestañas, búsqueda y recuperación ante cortes de energía.",
                about_copyright: "© 2026 Miguel Ubilla. Licencia MIT (Permisiva y de libre uso).",
            },

            Language::English => Self {
                app_title: "Papel",
                untitled: "Untitled",
                default_doc_filename: "document.txt",

                new_tab_tooltip: "New document (Ctrl+N)",
                open_tooltip: "Open file (Ctrl+O)",
                save_tooltip: "Save (Ctrl+S)",
                search_tooltip: "Find (Ctrl+F)",
                more_options_tooltip: "More options",

                restore_session: "Restore previous session",
                save_as: "Save As...",
                close_tab: "Close tab",
                edit_section: "Edit",
                cut: "Cut",
                copy: "Copy",
                paste: "Paste",
                about_app: "About Papel",

                banner_recovered_title: "Unsaved documents from a previous session were recovered.",
                banner_restored_title: "Tabs from the backed-up session have been restored.",
                banner_understood_btn: "Dismiss",

                search_placeholder: "Find in document...",
                prev_match_tooltip: "Previous (Shift+Enter)",
                next_match_tooltip: "Next (Enter)",
                zero_matches: "0 matches",

                open_dialog_title: "Open File",
                save_as_dialog_title: "Save As",
                btn_cancel: "Cancel",
                btn_open: "Open",
                btn_save: "Save",

                about_comments: "Modern, lightweight and safe text editor with tabs, search, and power-loss recovery.",
                about_copyright: "© 2026 Miguel Ubilla. MIT License (Permissive and free to use).",
            },

            Language::Portuguese => Self {
                app_title: "Papel",
                untitled: "Sem título",
                default_doc_filename: "documento.txt",

                new_tab_tooltip: "Novo documento (Ctrl+N)",
                open_tooltip: "Abrir arquivo (Ctrl+O)",
                save_tooltip: "Salvar (Ctrl+S)",
                search_tooltip: "Localizar (Ctrl+F)",
                more_options_tooltip: "Mais opções",

                restore_session: "Restaurar sessão anterior",
                save_as: "Salvar como...",
                close_tab: "Fechar aba",
                edit_section: "Editar",
                cut: "Recortar",
                copy: "Copiar",
                paste: "Colar",
                about_app: "Sobre o Papel",

                banner_recovered_title: "Documentos não salvos de uma sessão anterior foram recuperados.",
                banner_restored_title: "As abas da sessão anterior foram restauradas.",
                banner_understood_btn: "Entendido",

                search_placeholder: "Localizar no documento...",
                prev_match_tooltip: "Anterior (Shift+Enter)",
                next_match_tooltip: "Próximo (Enter)",
                zero_matches: "0 correspondências",

                open_dialog_title: "Abrir Arquivo",
                save_as_dialog_title: "Salvar Como",
                btn_cancel: "Cancelar",
                btn_open: "Abrir",
                btn_save: "Salvar",

                about_comments: "Editor de texto moderno, leve e seguro com abas, busca e proteção contra quedas de energia.",
                about_copyright: "© 2026 Miguel Ubilla. Licença MIT (Permissiva e de uso livre).",
            },

            Language::French => Self {
                app_title: "Papel",
                untitled: "Sans titre",
                default_doc_filename: "document.txt",

                new_tab_tooltip: "Nouveau document (Ctrl+N)",
                open_tooltip: "Ouvrir un fichier (Ctrl+O)",
                save_tooltip: "Enregistrer (Ctrl+S)",
                search_tooltip: "Rechercher (Ctrl+F)",
                more_options_tooltip: "Plus d'options",

                restore_session: "Restaurer la session précédente",
                save_as: "Enregistrer sous...",
                close_tab: "Fermer l'onglet",
                edit_section: "Édition",
                cut: "Couper",
                copy: "Copier",
                paste: "Coller",
                about_app: "À propos de Papel",

                banner_recovered_title: "Des documents non enregistrés d'une session précédente ont été récupérés.",
                banner_restored_title: "Les onglets de la session précédente ont été restaurés.",
                banner_understood_btn: "Compris",

                search_placeholder: "Rechercher dans le document...",
                prev_match_tooltip: "Précédent (Shift+Enter)",
                next_match_tooltip: "Suivant (Enter)",
                zero_matches: "0 correspondance",

                open_dialog_title: "Ouvrir un fichier",
                save_as_dialog_title: "Enregistrer sous",
                btn_cancel: "Annuler",
                btn_open: "Ouvrir",
                btn_save: "Enregistrer",

                about_comments: "Éditeur de texte moderne, léger et sûr avec onglets, recherche et protection contre les coupures de courant.",
                about_copyright: "© 2026 Miguel Ubilla. Licence MIT (Permissive et libre d'utilisation).",
            },
        }
    }
}

