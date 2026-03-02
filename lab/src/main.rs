use chrono::{DateTime, TimeZone, Utc};
use std::{io::stdin, os::unix::thread};
use iced::{Task, widget::{Column, button, column, text}, window};
use std::process::Command;
use rfd::FileDialog;

#[derive(Default)]
struct AppState {
    path: String,
}


impl std::fmt::Display for FuelStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out: String = String::new();
        for i in &self.fuel_storage.into_iter() {
            out += &format!("{}\n",i);
        }

        write!(
            f,"{}",out
        )
        // write!(
        //     f,
        //     "Name: {}, Date: {}, Price: {:.2}",
        //     self.name,
        //     self.date.format("%Y-%m-%d %H:%M:%S"),
        //     self.price
        // )
    }
}

struct FuelStorage {
    fuel_storage: Vec<Fuel>,
}



#[derive(Debug, Clone)]
pub enum Message {
    SelectFile,
    FileSelected(String)
}

impl AppState {
    pub fn view(&self) -> Column<Message> {
        column![
            button("Open file").on_press(Message::SelectFile),

        ]
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SelectFile => {
                Task::perform(pick_file_async(), Message::FileSelected )
            }
            Message::FileSelected(path) => {
                println!("{}",path);
                self.path = path;
                Task::none()
            }
            //Message::Decrement => {
            //    self.value -= 1;
            //}
        }
    }
}

async fn pick_file_async() -> String {
    let path = FileDialog::new()
        .set_directory("~")
        .add_filter("Select fuel", &["csv","txt"])
        .set_can_create_directories(true)
        .pick_file();


    
    let p = match path {
        //format!("{:?}",Some(path));
        Some(p) => format!("{:?}",Some(p)),
        _ => String::new()
    };
    return p;
}

struct Fuel {
    name: String,
    date: DateTime<Utc>,
    price: f64, 
}

impl Fuel {
    fn new() -> Self {
        Fuel { name: String::from("Not defined"), date: Utc::now(), price: -1.0 }
    }
    fn new_param(name_: String, date_: DateTime<Utc>,price_:f64) -> Self {
        Fuel { name: name_, date: date_, price: price_ }
    }
    //Format <String>,<Time (yyyy.mm.dd hh.mm)>, <f64>
    fn from_string(self, input: &str) -> Result<Self, String> {
        
        let parts: Vec<&str> = input.split(',').collect();
        if parts.len() != 3 {
            return Err("Input must have three parts separated by commas".to_string());
        }
        let name = parts[0].trim().to_string();
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        let date_str = parts[1].trim();
        let price_str = parts[2].trim();
        let price = price_str.parse::<f64>().map_err(|e| format!("Failed to parse price: {}", e))?;
        //don;t care if deprecated
        let date = match Utc.datetime_from_str(date_str, "%Y.%m.%d %H:%M") {
            Ok(d) => d,
            Err(_) => return Err("Invalid date format. Expected yyyy.mm.dd hh.mm".to_string()),
        };



        return Ok(Fuel::new_param(name, date, price));
    }

    fn input_secure() -> Self {
        
        
        loop {
            let mut input = String::new();
            println!("Enter fuel data in the format: <String>,<Time (yyyy.mm.dd hh.mm)>, <f64>");
            stdin().read_line(&mut input).expect("Failed to read input, fatal error");
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
            "Name: {}, Date: {}, Price: {:.2}",
            self.name,
            self.date.format("%Y-%m-%d %H:%M:%S"),
            self.price
        )
    }
}


//fuel = Fuel::new_param("Gasoline".to_string(), Utc.ymd(2024, 6, 1).and_hms(12, 0, 0), 3.99);
fn main() -> iced::Result {
    iced::application(AppState::default, AppState::update, AppState::view).window_size((400, 300)).resizable(false).run()
}