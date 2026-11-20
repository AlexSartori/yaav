use std::{sync::Arc, time::Instant};
use iced::mouse::ScrollDelta;
use iced::widget::canvas::{Cache, Path, Stroke};
use iced::{Color, Element, Length, Point, Rectangle, Size, Theme, keyboard, mouse};
use iced::widget::canvas;
use crate::hershey;
use crate::{
    components::{COLOR_NT_A, COLOR_NT_C, COLOR_NT_G, COLOR_NT_N, COLOR_NT_T, COLOR_NT_Z},
    types::MultiAlignment,
};


const MIN_ZOOM: f32 = 6.0;
const MAX_ZOOM: f32 = 36.0;
const MIN_ZOOM_FOR_TEXT: f32 = 12.0;


#[derive(Debug,Clone)]
pub enum Message {
    KBEvent(keyboard::Event),
    MultiAlignmentReady(Result<MultiAlignment,String>),
    MultiAlignmentStartLoading(Result<std::path::PathBuf,String>),
    AlignmentStateChanged(AlignmentEvent),
    OpenFasta,
    ZoomMinus,
    ZoomPlus,
    Test
}

#[derive(Debug,Clone)]
pub enum AlignmentEvent {
    ScrollTo(Point),
    ZoomBy(f32)
}

#[derive(Debug)]
pub struct AlignmentState {
    pub sequences: Arc<MultiAlignment>,
    pub max_len: usize,

    pub cell_size: f32,
    pub scroll_offset: Point,
    pub scroll_speed: f32,
    pub hovered: Option<(usize, usize)>, // (row, col)

    // Base grid cache (cleared only on scroll, zoom, or data change)
    grid_cache: canvas::Cache,
    // Overlay cache (cleared on hover/selection changes)
    overlay_cache: canvas::Cache,
}

impl AlignmentState {
    pub fn new(sequences: Arc<MultiAlignment>, cell_size: f32, scroll_speed: f32) -> Self {
        let max_len = sequences.iter().map(|s| s.seq.len()).max().unwrap_or(0);
        Self {
            sequences,
            max_len,
            cell_size: cell_size,
            scroll_speed: scroll_speed,
            scroll_offset: Point::ORIGIN,
            hovered: None,
            grid_cache: canvas::Cache::new(),
            overlay_cache: Cache::new(),
        }
    }

    pub fn update(&mut self, event: AlignmentEvent) {
        match event {
            AlignmentEvent::ScrollTo(p) => {
                self.scroll_offset = p;
                self.grid_cache.clear();
                self.overlay_cache.clear();
            },
            AlignmentEvent::ZoomBy(z) => {
                self.cell_size = (self.cell_size + z).clamp(MIN_ZOOM, MAX_ZOOM);
                self.grid_cache.clear();
                self.overlay_cache.clear();
            }
        }
    }
}

struct AlignmentProgram<'a> {
    state: &'a AlignmentState
}

#[derive(Default)]
struct CanvasLocalState {
    current_hover: Option<(usize, usize)>,
    kb_mods: keyboard::Modifiers
}

impl<'a> canvas::Program<AlignmentEvent> for AlignmentProgram<'a> {
    type State = CanvasLocalState;

    fn update(
        &self,
        state: &mut Self::State,
        event: &canvas::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<canvas::Action<AlignmentEvent>> {
        match event {
            canvas::Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                let next_hover = cursor.position_in(bounds).and_then(|pos| {
                    let col = ((pos.x + self.state.scroll_offset.x) / self.state.cell_size) as usize;
                    let row = ((pos.y + self.state.scroll_offset.y) / self.state.cell_size) as usize;
                    if row < self.state.sequences.len() && col < self.state.sequences[row].seq.len() {
                        Some((row, col))
                    } else {
                        None
                    }
                });

                if state.current_hover != next_hover {
                    state.current_hover = next_hover;
                    self.state.overlay_cache.clear();
                    return Some(canvas::Action::request_redraw());
                }
                None
            }
            canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                // if let Some((row, col)) = *current_hover {
                //     return Some(canvas::Action::publish((self.on_select)(row, col)));
                // }
                None
            }
            canvas::Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                let cs = self.state.cell_size;

                let (mut dx, mut dy): (f32, f32) = match delta {
                    ScrollDelta::Lines{x, y} => (x * cs, y * cs),
                    ScrollDelta::Pixels{x, y} => (*x, *y)
                };

                if state.kb_mods.control() {
                    return Some(
                        canvas::Action::publish(AlignmentEvent::ZoomBy(if dy > 0.0 {1.0} else {-1.0}))
                    )
                }

                if state.kb_mods.shift() {
                    (dx, dy) = (dy, dx);
                }

                let max_x = self.state.max_len as f32 * cs;
                let max_y = self.state.sequences.len() as f32 * cs;

                let p = self.state.scroll_offset;
                let x2 = (p.x - dx*self.state.scroll_speed).clamp(0.0, max_x);
                let y2 = (p.y - dy*self.state.scroll_speed).clamp(0.0, max_y);
                
                Some(
                    canvas::Action::publish(AlignmentEvent::ScrollTo(Point { x: x2, y: y2 }))
                )
            }
            canvas::Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
                state.kb_mods = *modifiers;
                None
            }
            // Note: Wire Mouse::WheelScrolled to your app's `AlignmentState::set_scroll`
            // or `zoom_by` by publishing a message from here.
            _ => None,
        }
    }


    fn draw(
        &self,
        state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let cs = self.state.cell_size as f32;
        let (scroll_x, scroll_y) = (self.state.scroll_offset.x, self.state.scroll_offset.y);

        let grid_geometry = self.state.grid_cache.draw(renderer, bounds.size(), |frame| {
            println!("Alora {:?}", Instant::now());
            
            let first_row = (scroll_y / cs).floor().max(0.0) as usize;
            let last_row = ((scroll_y + bounds.height) / cs)
                .ceil()
                .min(self.state.sequences.len() as f32) as usize;

            let first_col = (scroll_x / cs).floor().max(0.0) as usize;
            let last_col = ((scroll_x + bounds.width) / cs)
                .ceil()
                .min(self.state.max_len as f32) as usize;

            for row_idx in first_row..last_row {
                let row = &self.state.sequences[row_idx];
                let y = row_idx as f32 * cs - scroll_y;
                let row_end_col = last_col.min(row.seq.len());

                for col_idx in first_col..row_end_col {
                    let nt = row.seq[col_idx];
                    let x = col_idx as f32 * cs - scroll_x;
                    let rect_pos = Point::new(x, y);
                    let nt_color = match nt {
                        'a' | 'A' => COLOR_NT_A,
                        'c' | 'C' => COLOR_NT_C,
                        'g' | 'G' => COLOR_NT_G,
                        't' | 'T' => COLOR_NT_T,
                        'z' | 'Z' | '-' | ' ' => COLOR_NT_Z,
                        _ => COLOR_NT_N,
                    };
                
                    frame.fill_rectangle(
                        rect_pos,
                        iced::Size::new(cs, cs),
                        nt_color,
                    );

                    if cs >= MIN_ZOOM_FOR_TEXT {
                        let stroke = Stroke::default().with_color(Color::BLACK);
                        for p in hershey::get_scaled_character(nt, x, y-2.0, cs as usize) {
                            frame.stroke(&p, stroke);
                        }
                    }
                }
            }
        });

        let overlay_geometry = self.state.overlay_cache.draw(
            renderer, bounds.size(), |frame| {
                if let Some((row, col)) = state.current_hover {
                    let x = col as f32 * cs - scroll_x;
                    let y = row as f32 * cs - scroll_y;

                    let highlight = Path::rectangle(Point::new(x, y), Size::new(cs, cs));
                    frame.stroke(
                        &highlight,
                        Stroke::default()
                            .with_color(Color::WHITE)
                            .with_width(2.0),
                    );
                }
            });

        vec![grid_geometry, overlay_geometry]
    }
}


pub struct AlignmentView<'a, Message> {
    state: &'a AlignmentState,
    on_event: Box<dyn Fn(AlignmentEvent) -> Message + 'a>,
    width: Length,
    height: Length,
}

impl<'a, Message> AlignmentView<'a, Message> {
    pub fn new(state: &'a AlignmentState, on_event: impl Fn(AlignmentEvent) -> Message + 'a) -> Self {
        Self {
            state,
            on_event: Box::new(on_event),
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }
}

impl<'a, Message: 'a> From<AlignmentView<'a, Message>> for Element<'a, Message> {
    fn from(view: AlignmentView<'a, Message>) -> Self {
        let canvas: Element<'a, AlignmentEvent> = canvas::Canvas::new(
            AlignmentProgram { state: view.state }
        ).width(view.width)
        .height(view.height)
        .into();

        canvas.map(view.on_event)
    }
}

// fn sequence_name(name: &str, nt_display_size: u8) -> Text<'_> {
//     text(name)
//     .size(iced::Pixels(nt_display_size as f32))
//     .width(200)
//     .height(nt_display_size as f32)
// }