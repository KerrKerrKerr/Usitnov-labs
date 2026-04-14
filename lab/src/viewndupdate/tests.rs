use super::*;
use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

fn unique_temp_path(prefix: &str, extension: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock error")
        .as_nanos();
    std::env::temp_dir().join(format!("{}_{}.{}", prefix, nanos, extension))
}

#[test]
fn ui_add_commit_pending_row_updates_storage() {
    let mut state = AppState::default();

    let _ = state.update(Message::Add);
    assert!(state.last_pending);

    let _ = state.update(Message::EditingDateChanged("2024.06.01 12:00".to_string()));
    let _ = state.update(Message::EditingNameChanged("Diesel".to_string()));
    let _ = state.update(Message::EditingPriceChanged("3.99".to_string()));
    let _ = state.update(Message::EditingColorChanged("255:200:0".to_string()));
    let _ = state.update(Message::CommitPendingRow);

    assert_eq!(state.fuel_storage.len(), 1);
    assert!(!state.last_pending);
    assert_eq!(state.fuel_storage.get_all()[0].name, "Diesel");
    assert!(state.editing_date.is_empty());
    assert!(state.editing_name.is_empty());
    assert!(state.editing_price.is_empty());
    assert!(state.editing_color.is_empty());
}

#[test]
fn ui_commit_invalid_pending_row_keeps_pending_and_storage_unchanged() {
    let mut state = AppState::default();

    let _ = state.update(Message::Add);
    let _ = state.update(Message::EditingDateChanged("2024.06.01 12:00".to_string()));
    let _ = state.update(Message::EditingNameChanged("Diesel".to_string()));
    let _ = state.update(Message::EditingPriceChanged("not-a-number".to_string()));
    let _ = state.update(Message::EditingColorChanged("255:0:0".to_string()));
    let _ = state.update(Message::CommitPendingRow);

    assert_eq!(state.fuel_storage.len(), 0);
    assert!(state.last_pending);
}

#[test]
fn ui_toggle_then_delete_selected_removes_rows() {
    let mut state = AppState::default();
    state
        .fuel_storage
        .push(Fuel::from_string("Gasoline,2024.06.01 12:00,3.99,255:0:0").unwrap());
    state
        .fuel_storage
        .push(Fuel::from_string("Diesel,2024.06.01 13:00,4.50,0:255:0").unwrap());

    let _ = state.update(Message::ToggleRow(1));
    let _ = state.update(Message::DeleteSelected);

    assert_eq!(state.fuel_storage.len(), 1);
    assert_eq!(state.fuel_storage.get_all()[0].name, "Gasoline");
    assert!(state.selected_rows.is_empty());
}

#[test]
fn ui_file_selected_replaces_storage_and_clears_selection() {
    let mut state = AppState::default();
    state
        .fuel_storage
        .push(Fuel::from_string("OldFuel,2024.06.01 12:00,1.00,1:1:1").unwrap());
    state.selected_rows.insert(0);

    let file_path = unique_temp_path("fuel_input", "csv");
    fs::write(&file_path, "NewFuel,2024.06.02 14:00,2.50,10:20:30")
        .expect("failed to write test input file");

    let _ = state.update(Message::FileSelected(file_path.to_string_lossy().to_string()));

    assert_eq!(state.fuel_storage.len(), 1);
    assert_eq!(state.fuel_storage.get_all()[0].name, "NewFuel");
    assert!(state.selected_rows.is_empty());

    let _ = fs::remove_file(file_path);
}

#[test]
fn ui_execute_commands_uses_loaded_command_file() {
    let mut state = AppState::default();

    let command_path = unique_temp_path("commands", "cmd");
    fs::write(
        &command_path,
        "ADD Diesel,2024.06.01 12:00,3.99,255:0:0\nADD Gasoline,2024.06.01 13:00,4.01,0:255:0",
    )
    .expect("failed to write command file");

    state.command_path = command_path.to_string_lossy().to_string();
    let _ = state.update(Message::ExecuteCommands);

    assert_eq!(state.fuel_storage.len(), 2);
    assert_eq!(state.fuel_storage.get_all()[0].name, "Diesel");
    assert_eq!(state.fuel_storage.get_all()[1].name, "Gasoline");

    let _ = fs::remove_file(command_path);
}

#[test]
fn ui_save_now_without_path_sets_hint_message() {
    let mut state = AppState::default();
    assert!(state.path.is_empty());

    let _ = state.update(Message::SaveNow);

    assert_eq!(state.path, "No file opened. Use Save interactively...");
}
