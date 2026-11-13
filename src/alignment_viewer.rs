use iced::{
    Alignment::Center, Element, Length::Fill, widget::{
        Text, button, canvas, column, container, row, space, text
    }
};
use iced::{Color, Rectangle, Renderer, Theme};
use iced_font_awesome::{fa_icon, fa_icon_solid};

use crate::{components::{COLOR_NT_A, COLOR_NT_C, COLOR_NT_G, COLOR_NT_N, COLOR_NT_T, COLOR_NT_Z}, types::MultiAlignment};


#[derive(Clone)]
pub enum Message {
    OpenFasta,
    FileSelected(Option<std::path::PathBuf>),
    ZoomMinus,
    ZoomPlus
}

#[derive(Debug)]
struct SequenceCanvas {
    seq: Vec<char>,
    char_size: f32
}

impl SequenceCanvas {
    fn compute_width(&self) -> f32 {
        self.char_size * self.seq.len() as f32
    }
    fn compute_height(&self) -> f32 {
        self.char_size
    }
}

impl <Message> canvas::Program<Message> for SequenceCanvas {    
    type State = ();

    fn draw(&self, _state: &(), renderer: &Renderer,
            _theme: &Theme, _bounds: Rectangle, _cursor: iced::mouse::Cursor
        ) -> Vec<canvas::Geometry> {


        // Set the canvas size to fit the sequence exactly, so scrollable() can work
        let w = self.char_size * self.seq.len() as f32;
        let h = self.char_size;
        let mut frame = canvas::Frame::new(renderer, iced::Size { width: w, height: h });

        for (i, nt) in self.seq.iter().enumerate() {
            let x = i as f32 * self.char_size;
            let y = 0.0;
            let nt_color = match nt {
                'a' | 'A' => COLOR_NT_A,
                'c' | 'C' => COLOR_NT_C,
                'g' | 'G' => COLOR_NT_G,
                't' | 'T' => COLOR_NT_T,
                'z' | 'Z' | '-' | ' ' => COLOR_NT_Z,
                _ => COLOR_NT_N,
            };
            // Draw background rectangle
            frame.fill_rectangle(
                iced::Point::new(x, y),
                iced::Size::new(self.char_size, self.char_size),
                nt_color,
            );
            // Draw character
            frame.fill_text(canvas::Text {
                content: nt.to_string(),
                position: iced::Point::new(x, 0.0),
                color: Color::BLACK,
                size: iced::Pixels::from(self.char_size),
                ..Default::default()
            });
        }

        vec![frame.into_geometry()]
    }
}


pub fn toolbar<'a>() -> Element<'a, Message> {
    container(
        row![
            button(
                fa_icon_solid("folder-open")
            ).on_press(Message::OpenFasta),
            button(
                fa_icon_solid("magnifying-glass-minus")
            ).on_press(Message::ZoomMinus),
            button(
                fa_icon_solid("magnifying-glass-plus")
            ).on_press(Message::ZoomPlus)
        ].spacing(10)
    )
    .width(Fill)
    .padding(6)
    .into()
}

pub fn alignment_viewer<'a>(multi_alignment: &'a MultiAlignment, nt_display_size: f32) -> Element<'a, Message> {
    let mut rows: Vec<Element<'a, Message>> = Vec::new();

    for s in multi_alignment.iter() {
        let mut cells: Vec<Element<'a, Message>> = Vec::new();
        cells.push(sequence_name(&s.id, nt_display_size).into());
        cells.push(sequence(&s.seq, nt_display_size));
        cells.push(space().width(nt_display_size).into());
        rows.push(row(cells).into());
    }

    rows.push(space().height(nt_display_size).into());
    
    column(rows).into()
}

fn sequence(seq: &Vec<char>, nt_display_size: f32) -> Element<'_, Message> {
    let c = SequenceCanvas{seq: seq.clone(), char_size: nt_display_size};
    let w = c.compute_width();
    let h = c.compute_height();
    canvas(c)
    .width(w)
    .height(h)
    .into()
}

fn sequence_name<'a>(name: &'a str, nt_display_size: f32) -> Text<'a> {
    text(name)
    .size(iced::Pixels(nt_display_size))
    .width(200)
    .height(nt_display_size)
}