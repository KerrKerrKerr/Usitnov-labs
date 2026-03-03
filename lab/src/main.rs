use chrono::{DateTime, TimeZone, Utc};
use rfd::FileDialog;
use std::collections::HashSet;
use std::io::stdin;

use iced::{Color, Theme};

mod viewnupdate;

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
}

impl std::fmt::Display for FuelStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out: String = String::new();
        for i in self.fuel_storage.iter() {
            out += &format!("{}\n", i);
        }

        write!(f, "{}", out)
    }
}

impl FuelStorage {
    pub fn new() -> Self {
        FuelStorage {
            fuel_storage: Vec::new(),
        }
    }
    
    pub fn get_all(&self) -> &[Fuel] {
        &self.fuel_storage
    }
    
    pub fn push(&mut self, fuel: Fuel) {
        self.fuel_storage.push(fuel);
    }
    
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&Fuel) -> bool,
    {
        self.fuel_storage.retain(f);
    }

    pub fn parse(&mut self, content: &str) {
        for line in content.lines() {
            println!("line: {}", line);
            let fuel = Fuel::new().from_string(line);
            if fuel.is_ok() {
                //need to hook it to logger later
                //println!("ok");
                self.fuel_storage.push(fuel.unwrap());
            } else {
                println!("err: {}", fuel.err().unwrap());
            }
        }
    }
    pub fn serialize_storage(&self) -> String {
        self.fuel_storage
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

#[derive(Default)]
pub struct FuelStorage {
    fuel_storage: Vec<Fuel>,
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectFile,
    FileSelected(String),
    SaveAs,
    SaveInteractively,
    FileSaved(String),
    Add,
    DeleteSelected,
    SaveNow,
    InputChanged(String),
    EditingDateChanged(String),
    EditingNameChanged(String),
    EditingPriceChanged(String),
    EditingColorChanged(String),
    ToggleRow(usize),
    CommitPendingRow,
    PasteNow,
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

struct Fuel {
    name: String,
    date: DateTime<Utc>,
    price: f64,
    color: (u8,u8,u8)
}

impl Fuel {
    pub fn new() -> Self {
        Fuel {
            name: String::from("Not defined"),
            date: Utc::now(),
            price: -1.0,
            color: (0,0,0)
        }
    }
    fn new_param(name_: String, date_: DateTime<Utc>, price_: f64,color_: (u8,u8,u8)) -> Self {
        Fuel {
            name: name_,
            date: date_,
            price: price_,
            color: color_,
        }
    }
    //Format <String>,<Time (yyyy.mm.dd hh.mm)>, <f64>,u8:u8:u8
    fn from_string(self, input: &str) -> Result<Self, String> {
        let parts: Vec<&str> = input.split(',').collect();
        if parts.len() != 4 {
            return Err("Input must have four parts separated by commas".to_string());
        }
        let name = parts[0].trim().to_string();
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        let date_str = parts[1].trim();
        let price_str = parts[2].trim();
        let price = price_str
            .parse::<f64>()
            .map_err(|e| format!("Failed to parse price: {}", e))?;
        let color_string = parts[3].trim();
        let color_parts: Vec<&str> = color_string.split(":").collect();
        if color_parts.len() != 3 {
            return Err("invalid color format u8:u8:u8".to_string());
        }
        let color: (u8, u8, u8) = (
            color_parts[0].parse::<u8>().map_err(|e| format!("Failed to parse color R: {}", e))?,
            color_parts[1].parse::<u8>().map_err(|e| format!("Failed to parse color G: {}", e))?,
            color_parts[2].parse::<u8>().map_err(|e| format!("Failed to parse color B: {}", e))?,
        );
        //don;t care if deprecated
        let date = match Utc.datetime_from_str(date_str, "%Y.%m.%d %H:%M") {
            Ok(d) => d,
            Err(_) => return Err("Invalid date format. Expected yyyy.mm.dd hh:mm".to_string()),
        };

        return Ok(Fuel::new_param(name, date, price,color));
    }

    fn _input_secure() -> Self {
        loop {
            let mut input = String::new();
            println!("Enter fuel data in the format: <String>,<Time (yyyy.mm.dd hh.mm)>, <f64>,u8:u8:u8");
            stdin()
                .read_line(&mut input)
                .expect("Failed to read input, fatal error");
            println!("Input: {}", input);
            match Fuel::new().from_string(&input) {
                Ok(f) => return f,
                Err(e) => {
                    println!("Error: {}", e);
                    continue;
                }
            }
        }
    }
}

impl std::fmt::Display for Fuel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}, Date: {}, Price: {:.2}, Color {}:{}:{}",
            self.name,
            self.date.format("%Y-%m-%d %H:%M:%S"),
            self.price,
            self.color.0,
            self.color.1,
            self.color.2
        )
    }
}

//fuel = Fuel::new_param("Gasoline".to_string(), Utc.ymd(2024, 6, 1).and_hms(12, 0, 0), 3.99);
fn main() -> iced::Result {
    iced::application(AppState::default, AppState::update, AppState::view)
        .theme(Theme::Light)
        .window_size((800, 600))
        .resizable(true)
        .run()
}
