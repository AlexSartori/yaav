use std::{fs, sync::Arc};

use iced::{Subscription, Task, Theme, widget::{Column, column, scrollable}};
use rfd::AsyncFileDialog;
use keybinds::Keybinds;

mod alignment_viewer;
mod toolbar;
mod types;
mod components;
mod hershey;

use crate::alignment_viewer::{Message,alignment_viewer};
use crate::toolbar::toolbar;
use crate::types::{MultiAlignment,Sequence};
use crate::components::PALETTE;


const MIN_ZOOM: u8 = 6;
const MAX_ZOOM: u8 = 36;

#[derive(Debug,Clone)]
enum KBAction {
    OpenFile
}

#[derive(Debug, Clone)]
struct AppWindow {
    multi_alignment: MultiAlignment,
    nt_display_size: u8,
    keybinds: Keybinds<KBAction>,
    is_loading: bool
}

impl Default for AppWindow {
    fn default() -> Self {
        let mut kb = Keybinds::default();
        kb.bind("Ctrl+o", KBAction::OpenFile).unwrap();

        Self {
            multi_alignment: Vec::new(),
            nt_display_size: 16,
            keybinds: kb,
            is_loading: false
        }
    }
}

impl AppWindow {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenFasta => {
                Task::perform(pick_file(), Message::MultiAlignmentStartLoading)
            },
            Message::MultiAlignmentStartLoading(Ok(p)) => {
                self.is_loading = true;
                Task::perform(read_fasta(p), Message::MultiAlignmentReady)
            },
            Message::MultiAlignmentStartLoading(Err(e)) => {
                println!("No file selected: {}", e);
                Task::none()
            },
            Message::MultiAlignmentReady(m) => {
                self.is_loading = false;

                match m {
                    Ok(m_ok) => self.multi_alignment = m_ok,
                    _ => println!("Error loading multi alignment") // TODO: alert
                }
                Task::none()
            },
            Message::ZoomMinus => {
                if self.nt_display_size > MIN_ZOOM { self.nt_display_size -= 1; }
                Task::none()
            },
            Message::ZoomPlus => {
                if self.nt_display_size < MAX_ZOOM { self.nt_display_size += 1; }
                Task::none()
            },
            Message::KBEvent(evt) => {
                if let Some(action) = self.keybinds.dispatch(evt) {
                    match action {
                        KBAction::OpenFile => {
                            // Task::done(Message::OpenFasta)
                            self.multi_alignment = vec![
                                Arc::new(Sequence::new(0, "seq_1", "ACGTAGCGGCTATAGCTCAT".chars().collect())),
                                Arc::new(Sequence::new(1, "seq_2", "ACGACGATCATGCGATGCATGCA".chars().collect())),
                                Arc::new(Sequence::new(2, "seq_3", "aCAGCTCGAGACTCTAGCA".chars().collect())),
                                Arc::new(Sequence::new(3, "seq_4", "AAAAAAAAAAAAACCCCCCCCCCCTTTTT".chars().collect())),
                            ];
                            Task::none()
                        },
                        _ => Task::none()
                    }
                } else {
                    Task::none()
                }
            },
            Message::Test => {
                println!("Test message signal received");
                Task::none()
            },
            m => {
                println!("Unhandled message: {:?}", m);
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

    pub fn subscription(&self) -> Subscription<Message> {
        iced::event::listen_with(|event, _, _| match event {
            iced::Event::Keyboard(event) => Some(Message::KBEvent(event)),
            _ => None,
        })
    }

}


async fn pick_file() -> Result<std::path::PathBuf, String> {
    let file_handle = AsyncFileDialog::new()
        .set_title("Choose a file")
        .add_filter("FASTA files", &["fa", "fas", "fasta", "fna", "ffn", "faa", "mpfa", "frn"])
        .pick_file()
        .await;

    match file_handle.map(|handle| handle.path().to_path_buf()) {
        Some(p) => Ok(p),
        _ => Err("No file selected".to_owned())
    }
}


async fn read_fasta(p: std::path::PathBuf) -> Result<MultiAlignment, String> {
    let res_contents = fs::read_to_string(p.as_path());
    if res_contents.is_err() {
        return Err("Cannot read file".to_owned());
    }
    
    let mut result = MultiAlignment::new();
    let contents = res_contents.expect("File was succesfully read but contents are somehow unavailable");
    let mut uid = 0u64;
    let mut new_seq = |h: String, s: String| {
        uid += 1;
        Arc::new(Sequence::new(uid, &h.to_string(), s.chars().collect()))
    };

    let mut curr_header = "".to_owned();
    let mut curr_seq = "".to_owned();
    for l in contents.lines() {
        if l.starts_with(">") {
            result.push(new_seq(curr_header, curr_seq));
            curr_header = l.to_owned();
            curr_seq = "".to_owned()
        } else {
            curr_seq.extend(l.chars());
        }
    }
    result.push(new_seq(curr_header, curr_seq));

    Ok(result)
}

// $env:CARGO_INCREMENTAL=0; cargo run
fn main() -> iced::Result {
    iced::application(AppWindow::default, AppWindow::update, AppWindow::view)
    .subscription(AppWindow::subscription)
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