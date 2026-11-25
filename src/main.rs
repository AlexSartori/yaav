use std::{fs, path::PathBuf};

use iced::{Length, Point, Subscription, Task, Theme, widget::{self, Column, column, row, scrollable, space}};
use rfd::{AsyncFileDialog, AsyncMessageDialog, MessageButtons, MessageLevel};
use keybinds::Keybinds;

mod alignment_viewer;
mod toolbar;
mod types;
mod components;
mod hershey;
mod utils;

use crate::alignment_viewer::{AlignmentEvent, AlignmentState, AlignmentView, Message};
use crate::toolbar::toolbar;
use crate::types::{MultiAlignment,Sequence};
use crate::components::PALETTE;


const DEFAULT_ZOOM: f32 = 14.0;
const DEFAULT_SCROLL_SPEED: f32 = 4.0;
const SCROLLBAR_THICKNESS: f32 = 12.0;

#[derive(Debug,Clone)]
enum KBAction {
    OpenFile
}

#[derive(Debug)]
struct AppWindow {
    alignment_state: AlignmentState,
    keybinds: Keybinds<KBAction>,
    is_loading: bool,
    v_scrollbar_id: widget::Id,
    h_scrollbar_id: widget::Id
}

impl Default for AppWindow {
    fn default() -> Self {
        let mut kb = Keybinds::default();
        kb.bind("Ctrl+o", KBAction::OpenFile).unwrap();

        Self {
            alignment_state: AlignmentState::new(Box::new(vec![]), DEFAULT_ZOOM, DEFAULT_SCROLL_SPEED),
            keybinds: kb,
            is_loading: false,
            v_scrollbar_id: widget::Id::unique(),
            h_scrollbar_id: widget::Id::unique()
        }
    }
}

impl AppWindow {
    fn new() -> (Self, Task<Message>) {
        let startup_task = if let Some(arg) = std::env::args().nth(1) {
            Task::done(Message::LoadMultiAlignment(Ok(PathBuf::from(arg))))
        } else {
            Task::none()
        };

        (Self::default(), startup_task)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenFasta => {
                Task::perform(pick_file(), Message::LoadMultiAlignment)
            },
            Message::ShowError(err_msg) => {
                Task::perform(async move {
                        AsyncMessageDialog::new()
                            .set_level(MessageLevel::Error)
                            .set_title("Error")
                            .set_description(&err_msg)
                            .set_buttons(MessageButtons::Ok)
                            .show()
                            .await;
                    }, |_| Message::DialogDismissed,
                )
            },
            Message::DialogDismissed => {
                Task::none()
            },
            Message::LoadMultiAlignment(ma) => {
                match ma {
                    Ok(p) => {
                        self.is_loading = true;
                        Task::perform(read_fasta(p), Message::MultiAlignmentReady)
                    }, Err(e) => {
                        Task::done(Message::ShowError(
                            format!("Invalid file path: {}", e)
                        ))
                    }
                }
            },
            Message::MultiAlignmentReady(m) => {
                self.is_loading = false;

                match m {
                    Ok(m_ok) => {
                        self.alignment_state = AlignmentState::new(Box::new(m_ok), self.alignment_state.cell_size, self.alignment_state.scroll_speed);
                        Task::none()
                    }, Err(e) => {
                        Task::done(Message::ShowError(format!("Failed to load alignment: {}", e)))
                    }
                }
            },
            Message::ZoomMinus => {
                Task::done(Message::AlignmentStateChanged(alignment_viewer::AlignmentEvent::ZoomBy(-1.0)))
            },
            Message::ZoomPlus => {
                Task::done(Message::AlignmentStateChanged(alignment_viewer::AlignmentEvent::ZoomBy(1.0)))
            },
            Message::KBEvent(evt) => {
                if let Some(action) = self.keybinds.dispatch(evt) {
                    match action {
                        KBAction::OpenFile => {
                            // Task::done(Message::OpenFasta)
                            self.alignment_state = AlignmentState::new(Box::new(vec![
                                Sequence::new(0, "seq_1", "ACGTAGCGGCTATAGCTCAT".chars().collect()),
                                Sequence::new(1, "seq_2", "ACGACGATCATGCGATGCATGCA".chars().collect()),
                                Sequence::new(2, "seq_3", "aCAGCTCGAGACTCTAGCA".chars().collect()),
                                Sequence::new(3, "seq_4", "AAAAAAAAAAAAACCCCCCCCCCCTTTTT".chars().collect()),
                            ]), 14.0, 2.0);
                            Task::none()
                        }
                    }
                } else {
                    Task::none()
                }
            },
            Message::AlignmentStateChanged(evt) => {
                self.alignment_state.update(evt.clone());
                if let AlignmentEvent::ScrollTo(p) = evt {
                    Task::batch([
                        widget::operation::scroll_to::<Message>(
                            self.h_scrollbar_id.clone(),
                            scrollable::AbsoluteOffset{ x: p.x, y: 0.0 }
                        ),
                        widget::operation::scroll_to::<Message>(
                            self.v_scrollbar_id.clone(),
                            scrollable::AbsoluteOffset{ x: 0.0, y: p.y }
                        )
                    ])
                } else {
                    Task::none()
                }
            },
            Message::ScrolledX(viewport) => {
                let new_x = viewport.absolute_offset().x;

                if new_x != self.alignment_state.scroll_offset.x {
                    self.alignment_state.update(
                        alignment_viewer::AlignmentEvent::ScrollTo(
                            Point { x: new_x, y: self.alignment_state.scroll_offset.y }
                        )
                    );
                }

                Task::none()
            },
            Message::ScrolledY(viewport) => {
                let new_y = viewport.absolute_offset().y;

                if new_y != self.alignment_state.scroll_offset.y {
                    self.alignment_state.update(
                        alignment_viewer::AlignmentEvent::ScrollTo(
                            Point { x: self.alignment_state.scroll_offset.x, y: new_y }
                        )
                    );
                }
                
                Task::none()
            }
        }
    }

    pub fn view(&'_ self) -> Column<'_, Message> {
        let alv = AlignmentView::new(&self.alignment_state, Message::AlignmentStateChanged);
        let w = alv.get_cavas_width();
        let h = alv.get_canvas_height();

        column![
            toolbar(&self.alignment_state),
            row![
                alv,
                scrollable(space().width(1.0).height(h))
                    .id(self.v_scrollbar_id.clone())
                    .width(Length::Fixed(SCROLLBAR_THICKNESS))
                    .height(Length::Fill)
                    .direction(scrollable::Direction::Vertical(
                        scrollable::Scrollbar::new()
                            .width(SCROLLBAR_THICKNESS)
                            .scroller_width(SCROLLBAR_THICKNESS)
                    ))
                    .on_scroll(Message::ScrolledY)
            ],
            row![
                scrollable(space().height(1.0).width(w))
                    .id(self.h_scrollbar_id.clone())
                    .width(Length::Fill)
                    .height(Length::Fixed(SCROLLBAR_THICKNESS))
                    .direction(scrollable::Direction::Horizontal(
                        scrollable::Scrollbar::new()
                            .width(SCROLLBAR_THICKNESS)
                            .scroller_width(SCROLLBAR_THICKNESS)
                    ))
                    .on_scroll(Message::ScrolledX),
                space().width(SCROLLBAR_THICKNESS).height(1.0)
            ]
        ]
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
    let mut uid = 0u64;
    let mut new_seq = |h: String, s: String| {
        uid += 1;
        Sequence::new(uid, &h.to_string(), s.chars().collect())
    };
    
    match fs::read_to_string(p.as_path()) {
        Err(err) =>
            Err(format!(
                "Cannot open file {} ({})",
                p.to_str().unwrap_or("<invalid PathBuf contents>"),
                err.to_string()
            )),
        Ok(file_contents) => {
            let mut result = MultiAlignment::new();
            let mut curr_header = "".to_owned();
            let mut curr_seq = "".to_owned();
            let mut error_msg = None;

            for l in file_contents.lines() {
                if l.starts_with(">") {
                    result.push(new_seq(curr_header, curr_seq));
                    curr_header = l.to_owned();
                    curr_seq = "".to_owned()
                } else {
                    if l.chars().any(|c| !c.is_ascii_alphabetic() && c != '-') {
                        error_msg = Some("Invalid characters found in sequence".to_owned());
                        break;
                    }
                    curr_seq.extend(l.chars());
                }
            }

            result.push(new_seq(curr_header, curr_seq));

            if let Some(err) = error_msg {
                Err(err)
            } else {
                Ok(result)
            }
        }
    }
}

fn main() -> iced::Result {
    iced::application(AppWindow::new, AppWindow::update, AppWindow::view)
    .subscription(AppWindow::subscription)
    .title("YAAV")
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