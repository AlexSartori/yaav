use std::{fs, sync::Arc};

use iced::{Task, Theme, widget::{Column, column, scrollable}};
use rfd::AsyncFileDialog;

mod alignment_viewer;
mod types;
mod components;

use crate::alignment_viewer::{Message,alignment_viewer,toolbar};
use crate::types::{MultiAlignment,Sequence};
use crate::components::PALETTE;


const MIN_ZOOM: u8 = 6;
const MAX_ZOOM: u8 = 36;

#[derive(Debug, Clone)]
struct AppWindow {
    multi_alignment: MultiAlignment,
    nt_display_size: u8,
}

impl Default for AppWindow {
    fn default() -> Self {
        AppWindow { multi_alignment: Vec::new(), nt_display_size: 16 }
    }
}

impl AppWindow {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenFasta => {
                Task::perform(pick_file(), Message::FileSelected)
            },
            Message::FileSelected(path) => {
                if let Some(p) = path {
                    match fs::read_to_string(p.as_path()) {
                        Ok(contents) => {
                            self.multi_alignment = read_fasta(contents);
                        },
                        Err(err) => {
                            println!("Cannot open file {}", err);
                        }
                    }
                } else {
                    println!("Invalid file path");
                }
                Task::none()
            }
            Message::ZoomMinus => {
                if self.nt_display_size > MIN_ZOOM { self.nt_display_size -= 1; }
                Task::none()
            },
            Message::ZoomPlus => {
                if self.nt_display_size < MAX_ZOOM { self.nt_display_size += 1; }
                Task::none()
            }
        }
    }

    pub fn view(&'_ self) -> Column<'_, Message> {
        column![
            toolbar(),
            scrollable(
                alignment_viewer(&self.multi_alignment, self.nt_display_size)
            ).direction(scrollable::Direction::Both{
                vertical: scrollable::Scrollbar::new(),
                horizontal: scrollable::Scrollbar::new()
            }).height(iced::Length::Fill)
            .width(iced::Length::Fill)
        ]
        .spacing(4)
        .padding(4)
        .into()
    }

}


async fn pick_file() -> Option<std::path::PathBuf> {
    let file_handle = AsyncFileDialog::new()
        .set_title("Choose a file")
        .add_filter("FASTA files", &["fa", "fas", "fasta", "fna", "ffn", "faa", "mpfa", "frn"])
        .pick_file()
        .await;

    file_handle.map(|handle| handle.path().to_path_buf())
}


fn read_fasta(contents: String) -> MultiAlignment {
    let mut result = MultiAlignment::new();

    let mut curr_header = "".to_owned();
    let mut curr_seq = "".to_owned();
    for l in contents.lines() {
        if l.starts_with(">") {
            result.push(
                Arc::new(Sequence::new(&curr_header, curr_seq.chars().collect()))
            );
            curr_header = l.to_owned();
            curr_seq = "".to_owned()
        } else {
            curr_seq.extend(l.chars());
        }
    }
    result.push(
        Arc::new(Sequence::new(&curr_header, curr_seq.chars().collect()))
    );

    result
}

// $env:CARGO_INCREMENTAL=0; cargo run
fn main() -> iced::Result {
    iced::application(AppWindow::default, AppWindow::update, AppWindow::view)
    .title("Sweggheti sweg")
    .theme(Theme::custom("alesc", PALETTE))
    .run()
}

// #[test]
// fn it_counts_properly() {
//     let mut counter = Counter { value: 0 };

//     counter.update(Message::Increment);
//     counter.update(Message::Increment);
//     // counter.update(Message::Decrement);

//     assert_eq!(counter.value, 2);
// }