use std::{hash::Hash, sync::Arc, time::Instant};
use iced::widget::canvas::Stroke;
use iced::{Color, Element, Rectangle, Renderer, Theme, keyboard};
use iced::widget::{
    Text, canvas, column, lazy, row, space, text
};
use crate::{
    components::{COLOR_NT_A, COLOR_NT_C, COLOR_NT_G, COLOR_NT_N, COLOR_NT_T, COLOR_NT_Z},
    types::{MultiAlignment, Sequence},
    hershey
};


#[derive(Clone)]
pub enum Message {
    KBEvent(keyboard::Event),
    OpenFasta,
    FileSelected(Option<std::path::PathBuf>),
    ZoomMinus,
    ZoomPlus
}

#[derive(Debug,Clone,Hash)]
struct SequenceCanvas {
    seq: Arc<Sequence>,
    char_size: u8
}

impl SequenceCanvas {
    fn compute_width(&self) -> f32 {
        self.char_size as f32 * self.seq.seq.len() as f32
    }
    fn compute_height(&self) -> f32 {
        self.char_size as f32
    }
}

impl <Message> canvas::Program<Message> for SequenceCanvas {    
    type State = canvas::Cache;

    fn draw(&self, cache: &Self::State, renderer: &Renderer,
            _theme: &Theme, bounds: Rectangle, _cursor: iced::mouse::Cursor
        ) -> Vec<canvas::Geometry> {

        let geometry = cache.draw(renderer, bounds.size(), |frame| {
            println!("Alora {:?}", Instant::now());
            let cs = self.char_size as f32;

            for (i, nt) in self.seq.seq.iter().enumerate() {
                let x = i as f32 * cs;
                let y = 0.0;
                let nt_color = match nt {
                    'a' | 'A' => COLOR_NT_A,
                    'c' | 'C' => COLOR_NT_C,
                    'g' | 'G' => COLOR_NT_G,
                    't' | 'T' => COLOR_NT_T,
                    'z' | 'Z' | '-' | ' ' => COLOR_NT_Z,
                    _ => COLOR_NT_N,
                };
                
                frame.fill_rectangle(
                    iced::Point::new(x, y),
                    iced::Size::new(cs, cs),
                    nt_color,
                );

                let stroke = Stroke::default().with_color(Color::BLACK);
                for p in hershey::get_scaled_character(*nt, x, y-2.0, cs as usize) {
                    frame.stroke(&p, stroke);
                }
                
                // frame.fill_text(canvas::Text {
                //     content: nt.to_string(),
                //     position: iced::Point::new(x + cs/2.0, 0.0),
                //     color: Color::BLACK,
                //     size: iced::Pixels::from(cs * 0.8),
                //     ..Default::default()
                // });
            }
        });

        vec![geometry]
    }
}

pub fn alignment_viewer<'a>(multi_alignment: &'a MultiAlignment, nt_display_size: u8) -> Element<'a, Message> {
    let mut rows: Vec<Element<'a, Message>> = Vec::new();

    for s in multi_alignment.iter() {
        let mut cells: Vec<Element<'a, Message>> = Vec::new();
        cells.push(sequence_name(&s.id, nt_display_size).into());
        cells.push(sequence(s.clone(), nt_display_size));
        cells.push(space().width(nt_display_size as f32).into());
        rows.push(row(cells).into());
    }

    rows.push(space().height(nt_display_size as f32).into());
    
    column(rows).into()
}

fn sequence<'a>(seq: Arc<Sequence>, nt_display_size: u8) -> Element<'a, Message> {
    let c: SequenceCanvas = SequenceCanvas{seq: seq, char_size: nt_display_size};
    lazy(c, |c| {
        let w = c.compute_width();
        let h = c.compute_height();
        canvas(c.clone())
        .width(w)
        .height(h)
    })
    .into()
}

fn sequence_name(name: &str, nt_display_size: u8) -> Text<'_> {
    text(name)
    .size(iced::Pixels(nt_display_size as f32))
    .width(200)
    .height(nt_display_size as f32)
}