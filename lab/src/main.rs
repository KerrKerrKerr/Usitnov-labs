use rfd::FileDialog;
use std::collections::HashSet;

use iced::Theme;

mod model;
mod viewnupdate;

use model::{FuelParser, FuelStorage, CommandParser, Command};

#[derive(Default)]
pub struct AppState {
    pub path: String,
    pub fuel_storage: FuelStorage,
    pub add_pressed: bool,
    pub input_form: String,
    pub last_pending: bool,
    pub editing_date: String,
    pub editing_name: String,
    pub editing_price: String,
    pub editing_color: String,
    pub selected_rows: HashSet<usize>,
    pub some_string: String,
    pub command_path: String,
}

impl FuelStorage {
    pub fn parse(&mut self, content: &str) {
        let parser = FuelParser::new();
        let new_storage = parser.parse_content(content);
        // Merge new storage into existing
        for fuel in new_storage.get_all() {
            self.push(fuel.clone());
        }
    }

    pub fn serialize_storage(&self) -> String {
        self.get_all()
            .iter()
            .map(|fuel| {
                format!(
                    "{},{},{:.2},{}:{}:{}",
                    fuel.name,
                    fuel.date.format("%Y.%m.%d %H:%M"),
                    fuel.price,
                    fuel.color.0,
                    fuel.color.1,
                    fuel.color.2
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectFile,
    FileSelected(String),
    SaveAs,
    SaveInteractively,
    FileSaved(String),
    SelectCommandFile,
    CommandFileSelected(String),
    Add,
    DeleteSelected,
    SaveNow,
    ExecuteCommands,
    InputChanged(String),
    EditingDateChanged(String),
    EditingNameChanged(String),
    EditingPriceChanged(String),
    EditingColorChanged(String),
    ToggleRow(usize),
    CommitPendingRow,
    PasteNow,
    Dummy(String),
}

pub async fn pick_file_async() -> String {
    let path = FileDialog::new()
        .set_directory("~")
        .add_filter("Select fuel", &["csv", "txt"])
        .set_can_create_directories(true)
        .pick_file();

    let p = match path {
        Some(p) => p.as_path().to_string_lossy().to_string(),
        _ => String::new(),
    };
    return p;
}

pub async fn save_file_async() -> String {
    let path = FileDialog::new()
        .set_directory("~")
        .add_filter("Select fuel", &["csv", "txt"])
        .set_can_create_directories(true)
        .save_file();

    let p = match path {
        Some(p) => p.as_path().to_string_lossy().to_string(),
        _ => String::new(),
    };
    return p;
}

pub async fn pick_command_file_async() -> String {
    let path = FileDialog::new()
        .set_directory("~")
        .add_filter("Command file", &["cmd", "txt"])
        .add_filter("All files", &["*"])
        .pick_file();

    let p = match path {
        Some(p) => p.as_path().to_string_lossy().to_string(),
        _ => String::new(),
    };
    p
}

fn main() -> iced::Result {
    // Initialize logging
    env_logger::init();
    log::info!("Application started");

    iced::application(AppState::default, AppState::update, AppState::view)
        .theme(Theme::Light)
        .window_size((800, 600))
        .resizable(true)
        .run()
}
