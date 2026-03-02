use crate::Fuel;
use crate::{AppState, FuelStorage, Message, pick_file_async};
use iced::Length::Shrink;
use iced::{
    Alignment, Color, Element, Length, Task, Theme,
    alignment::Vertical::Bottom,
    widget::{Text, button, column, container, responsive, row, scrollable, text},
};

impl Fuel {
    fn serialize_to_table(&self) -> Vec<String> {
        vec![
            self.date.to_string(),
            self.name.clone(),
            format!("{:.2}", self.price),
        ]
    }
}

impl AppState {
    pub fn view(&self) -> Element<Message> {
        responsive(|size| {
            //println!("{}", size.width);
            let is_narrow = size.width < 450.0;
            let is_narrow_2 = size.width < 280.0;
            let path_display: Text = text(format!("Opened: {}", self.path));
            let open_button = button("Open interactively...").on_press(Message::SelectFile);
            let save_button = button("Save as...").on_press(Message::SaveAs);

            let mut top_bar = row![];
            if is_narrow == false {
                top_bar = row![open_button, save_button, path_display.width(Length::Shrink)]
                    .spacing(10)
                    .align_y(Alignment::Center);
            } else {
                top_bar = row![column![path_display, open_button, save_button].spacing(10)];
            }

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

            let mut table_column = column![header].spacing(1);
            for (i, fuel) in self.fuel_storage.fuel_storage.iter().enumerate() {
                let mut row = container(
                    row![
                        text(fuel.date.to_string()).width(col_date),
                        text(fuel.name.to_string()).width(col_type),
                        text(fuel.price.to_string()).width(col_price),
                    ]
                    .spacing(10),
                )
                .style(|_theme: &Theme| container::Style {
                    background: Some(iced::Background::Color(Color::from_rgb(0.2, 0.55, 0.9))),
                    text_color: Some(Color::WHITE),
                    ..Default::default()
                })
                .padding(5);

                if i % 2 == 0 {
                    row = row.style(|_theme: &Theme| container::Style {
                        background: Some(iced::Background::Color(Color::from_rgb(
                            0.96, 0.96, 0.96,
                        ))),
                        ..Default::default()
                    });
                }

                table_column = table_column.push(row);
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
                self.path = path.clone();
                if let Some(contents) = std::fs::read_to_string(&path).ok() {
                    self.fuel_storage.parse(&contents);
                }
                Task::none()
            }
            _ => Task::none(),
        }
    }
}
