use chrono::{DateTime, TimeZone, Utc};
use iced::{
    Task,
    widget::{Column, button, column},
};
use rfd::FileDialog;
use std::io::stdin;

use iced::{
    Alignment, Color, Element, Length, Theme,
    widget::{container, responsive, row, scrollable, text},
};

#[derive(Default)]
struct AppState {
    path: String,
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

struct FuelStorage {
    fuel_storage: Vec<Fuel>,
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectFile,
    FileSelected(String),
    SaveAs,
    Add,
    DeleteSelected,
    SaveNow,
}

impl AppState {
    pub fn view(&self) -> Element<Message> {
        responsive(|size| {
            let is_narrow = size.width < 700.0;
            let path_display = text(format!("Файл: {}", self.path));
            let top_controls = row![
                button("Open interactively...").on_press(Message::SelectFile),
                button("Save as...").on_press(Message::SaveAs)
            ]
            .spacing(10);
            let top_bar = row![path_display, top_controls]
                .spacing(10)
                .align_y(Alignment::Center);

            //let top_bar = if is_narrow {
            //    column![path_display, top_controls].spacing(10)
            // } else {
            //     row![path_display, horizontal_space(), top_controls]
            //         .spacing(10)
            //         .align_y(Alignment::Center)
            // };

            let col_date = Length::FillPortion(1);
            let col_type = Length::FillPortion(1);
            let col_price = Length::FillPortion(1);
            let header = container(
                row![
                    text("Date").width(col_date),
                    text("Fuel type").width(col_type),
                    text("Price").width(col_price),
                ]
                .spacing(10),
            )
            .style(|_theme: &Theme| container::Style {
                background: Some(iced::Background::Color(Color::from_rgb(0.96, 0.96, 0.96))),
                ..Default::default()
            })
            .padding(5);
            let row1 = container(row![].spacing(10))
                .style(|_theme: &Theme| container::Style {
                    background: Some(iced::Background::Color(Color::from_rgb(0.2, 0.55, 0.9))),
                    text_color: Some(Color::WHITE),
                    ..Default::default()
                })
                .padding(5);
            let row2 = container(row![].spacing(10)).padding(5);
            let table =
                container(scrollable(column![header, row1, row2].spacing(1)).height(Length::Fill))
                    .style(|_theme: &Theme| container::Style {
                        background: Some(iced::Background::Color(Color::WHITE)),
                        border: iced::Border {
                            color: Color::from_rgb(0.85, 0.85, 0.85),
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        ..Default::default()
                    })
                    .height(Length::Fill);
            let left_buttons = row![
                button("Add").on_press(Message::Add),
                button("Delete selected").on_press(Message::DeleteSelected)
            ]
            .spacing(10);
            let save_button = button("Сохранить сейчас").on_press(Message::SaveNow);
            let bottom_bar = row![left_buttons, save_button]
                .spacing(10)
                .align_y(Alignment::Center);

            // let bottom_bar = if is_narrow {
            //     column![left_buttons, save_button].spacing(10)
            // } else {
            //     row![left_buttons, horizontal_space(), save_button]
            //         .spacing(10)
            //         .align_y(Alignment::Center)
            // };
            column![top_bar, table, bottom_bar]
                .spacing(15)
                .padding(20)
                .height(Length::Fill)
                .width(Length::Fill)
                .into()
        })
        .into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SelectFile => Task::perform(pick_file_async(), Message::FileSelected),
            Message::FileSelected(path) => {
                println!("{}", path);
                self.path = path;
                Task::none()
            }
            _ => Task::none(),
        }
    }
}

async fn pick_file_async() -> String {
    let path = FileDialog::new()
        .set_directory("~")
        .add_filter("Select fuel", &["csv", "txt"])
        .set_can_create_directories(true)
        .pick_file();

    let p = match path {
        //format!("{:?}",Some(path));
        Some(p) => format!("{:?}", Some(p)),
        _ => String::new(),
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
        Fuel {
            name: String::from("Not defined"),
            date: Utc::now(),
            price: -1.0,
        }
    }
    fn new_param(name_: String, date_: DateTime<Utc>, price_: f64) -> Self {
        Fuel {
            name: name_,
            date: date_,
            price: price_,
        }
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
        let price = price_str
            .parse::<f64>()
            .map_err(|e| format!("Failed to parse price: {}", e))?;
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
            "Name: {}, Date: {}, Price: {:.2}",
            self.name,
            self.date.format("%Y-%m-%d %H:%M:%S"),
            self.price
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
