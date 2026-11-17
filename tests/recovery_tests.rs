use std::fs;
use std::path::PathBuf;
use uuid::Uuid;
use papel::model::document::DocumentMetadata;
use papel::storage::recovery::{RecoveryManager, TabSnapshot};

#[test]
fn test_recovery_lifecycle() {
    let test_dir = PathBuf::from("/tmp").join(format!("papel_test_{}", Uuid::new_v4()));
    let mgr = RecoveryManager::with_dir(test_dir.clone()).expect("Failed to initialize RecoveryManager");

    // Crear dos pestañas de prueba
    let tab1 = TabSnapshot {
        metadata: DocumentMetadata::new_empty("Borrador 1".to_string()),
        content: "Texto importante que no debe perderse ante corte de luz".to_string(),
    };
    let tab2 = TabSnapshot {
        metadata: DocumentMetadata::new_empty("Borrador 2".to_string()),
        content: "Otro texto crítico con datos no guardados".to_string(),
    };

    let tabs = vec![tab1.clone(), tab2.clone()];
    mgr.save_session_atomic(&tabs, 1).expect("Failed to save session");

    // Verificar que se recupera correctamente
    let (recovered, active_idx) = mgr.load_recovered_session().expect("Session should be recoverable");
    assert_eq!(recovered.len(), 2);
    assert_eq!(active_idx, 1);
    assert_eq!(recovered[0].content, tab1.content);
    assert_eq!(recovered[1].content, tab2.content);
    assert_eq!(recovered[0].metadata.title, "Borrador 1");

    // Eliminar una pestaña y verificar
    mgr.remove_tab_recovery(&tab1.metadata.id);
    mgr.clear_session();

    // Limpiar directorio de test
    let _ = fs::remove_dir_all(test_dir);
}
