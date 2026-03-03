use crate::Fuel;
use crate::{AppState, FuelStorage, Message, pick_file_async, save_file_async};
use chrono::Utc;
use iced::widget::{text_input};
use iced::{
    Alignment, Color, Element, Length, Task, Theme,
    widget::{Text, button, column, container, responsive, row, scrollable, text},
};

impl AppState {

    
    pub fn view(&self) -> Element<'_, Message> {
        responsive(|size| {
            //println!("{}", size.width);
            let is_narrow = size.width < 450.0;
            let is_narrow_2 = size.width < 280.0;
            let path_display: Text = text(format!("Opened: {}", self.path));
            let open_button = button("Open interactively...").on_press(Message::SelectFile);
            let save_button = button("Save interactively...").on_press(Message::SaveInteractively);

            let mut top_bar = row![];
            if is_narrow == false {
                top_bar = row![open_button, save_button, path_display.width(Length::Shrink)]
                    .spacing(10)
                    .align_y(Alignment::Center);
            } else {
                top_bar = row![column![path_display, open_button, save_button].spacing(10)];
            }

            let col_date = Length::FillPortion(4);
            let col_type = Length::FillPortion(3);
            let col_price = Length::FillPortion(1);
            let col_color = Length::FillPortion(2);
            let header = container(
                row![
                    text("Date").width(col_date),
                    text("Fuel type").width(col_type),
                    text("Price").width(col_price),
                    text("Color").width(col_color),
                ]
                .spacing(10),
            )
            .style(|_theme: &Theme| container::Style {
                background: Some(iced::Background::Color(Color::from_rgb(0.96, 0.96, 0.96))),
                ..Default::default()
            })
            .padding(5);

            let mut table_column = column![header].spacing(1);
            for (i, fuel) in self.fuel_storage.get_all().iter().enumerate() {
                let is_selected = self.selected_rows.contains(&i);
                
                let base_bg =   Color::from_rgb8(fuel.color.0, fuel.color.1, fuel.color.2);
                
                
                
                let row_background = if is_selected {
                    Color::from_rgb(0.2, 0.6, 0.2)
                } else {
                    base_bg
                };
                let row_text_color = if is_selected {
                    Color::WHITE
                } else {
                    Color::BLACK
                };

                let row_content = row![
                    text(fuel.date.to_string()).width(col_date),
                    text(fuel.name.to_string()).width(col_type),
                    text(fuel.price.to_string()).width(col_price),
                    text(format!("{}:{}:{}", fuel.color.0, fuel.color.1, fuel.color.2)).width(col_color),
                ]
                .spacing(10);

                let row_button = button(row_content)
                    .on_press(Message::ToggleRow(i))
                    .style(move |_theme: &Theme, _status| button::Style {
                        background: Some(iced::Background::Color(row_background)),
                        text_color: row_text_color,
                        border: iced::Border {
                            color: Color::from_rgb(0.9, 0.9, 0.9),
                            width: 1.0,
                            radius: 0.0.into(),
                        },
                        ..Default::default()
                    })
                    .padding(5);

                table_column = table_column.push(row_button);
            }

            if self.last_pending {
                let editing_row = container(
                    row![
                        button("Add").on_press(Message::CommitPendingRow),
                        button("Now").on_press(Message::PasteNow),
                        text_input("yyyy.mm.dd hh.mm", &self.editing_date)
                            .on_input(Message::EditingDateChanged)
                            .width(col_date),
                        text_input("name", &self.editing_name)
                            .on_input(Message::EditingNameChanged)
                            .width(col_type),
                        text_input("price", &self.editing_price)
                            .on_input(Message::EditingPriceChanged)
                            .width(col_price),
                        text_input("r:g:b", &self.editing_color)
                            .on_input(Message::EditingColorChanged)
                            .width(col_color),
                    ]
                    .spacing(10),
                )
                .padding(5);

                table_column = table_column.push(editing_row);
            }

            let table = container(scrollable(table_column).height(Length::Fill))
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

            let add_button = button("Add").on_press(Message::Add);
            let delete_selected_button =
                button("Delete selected").on_press(Message::DeleteSelected);

            let save_button = button("Save now").on_press(Message::SaveNow);
            let mut bottom_bar = row![];
            if !is_narrow_2 {
                bottom_bar = row![add_button, delete_selected_button, save_button]
                    .spacing(10)
                    .align_y(Alignment::Center);
            } else {
                bottom_bar =
                    row![column![add_button, delete_selected_button, save_button].spacing(10)]
            }

            let some_button = button("Generic button");
            let some_input_text = text_input("placeholder", &self.some_string).on_input(Message::Dummy);
            let just_row = row![some_button,some_input_text];

            column![top_bar, table, bottom_bar, just_row]
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
            Message::SaveInteractively => Task::perform(save_file_async(), Message::FileSaved),
            Message::FileSaved(path) => {
                if path.trim().is_empty() {
                    Task::none()
                } else {
                    self.path = path.clone();
                    let contents = self.fuel_storage.serialize_storage();
                    let _ = std::fs::write(&path, contents);
                    Task::none()
                }
            }
            Message::FileSelected(path) => {
                self.path = path.clone();
                if let Some(contents) = std::fs::read_to_string(&path).ok() {
                    self.fuel_storage.parse(&contents);
                }
                self.selected_rows.clear();
                Task::none()
            }
            Message::Add => {
                self.add_pressed = true;
                if !self.last_pending {
                    self.last_pending = true;
                    self.editing_date.clear();
                    self.editing_name.clear();
                    self.editing_price.clear();
                    self.editing_color.clear();
                }
                Task::none()
            }
            Message::InputChanged(value) => {
                self.input_form = value;
                Task::none()
            }
            Message::EditingDateChanged(value) => {
                self.editing_date = value;
                Task::none()
            }
            Message::EditingNameChanged(value) => {
                self.editing_name = value;
                Task::none()
            }
            Message::EditingPriceChanged(value) => {
                self.editing_price = value;
                Task::none()
            }
            Message::EditingColorChanged(value) => {
                self.editing_color = value;
                Task::none()
            }
            Message::CommitPendingRow => {
                let input_line = format!(
                    "{},{},{},{}",
                    &self.editing_name, &self.editing_date, &self.editing_price, &self.editing_color
                );
                if let Ok(fuel) = Fuel::new().from_string(&input_line) {
                    self.fuel_storage.push(fuel);
                    self.last_pending = false;
                    self.editing_date.clear();
                    self.editing_name.clear();
                    self.editing_price.clear();
                    self.editing_color.clear();
                }
                Task::none()
            }
            Message::PasteNow => {
                self.editing_date = Utc::now().format("%Y.%m.%d %H:%M").to_string();
                Task::none()
            }
            Message::SaveNow => {
                if self.path.trim().is_empty() {
                    self.path = "No file opened. Use Save interactively...".to_string();
                } else {
                    let contents = self.fuel_storage.serialize_storage();
                    let _ = std::fs::write(&self.path, contents);
                }
                Task::none()
            }
            Message::DeleteSelected => {
                if !self.selected_rows.is_empty() {
                    let mut index = 0usize;
                    self.fuel_storage.retain(|_| {
                        let keep = !self.selected_rows.contains(&index);
                        index += 1;
                        keep
                    });
                    self.selected_rows.clear();
                }
                Task::none()
            }
            Message::ToggleRow(index) => {
                if self.selected_rows.contains(&index) {
                    self.selected_rows.remove(&index);
                } else {
                    self.selected_rows.insert(index);
                }
                Task::none()
            } Message::Dummy(soem_str) => {
                self.some_string = soem_str;
                Task::none()
            }
            _ => {
                println!("Tried invoking undimplemented message, idk which one though");
                Task::none()},
        }
    }
}
