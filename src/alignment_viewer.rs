use iced::mouse::ScrollDelta;
use iced::widget::canvas::path::Builder;
use iced::widget::canvas::{Cache, Path, Stroke};
use iced::{Color, Element, Length, Point, Rectangle, Size, Theme, keyboard, mouse};
use iced::widget::{canvas, combo_box};
use crate::alignment_viewer::SequenceEncoding::Aminoacidic;
use crate::hershey::push_glyph;
use crate::utils;
use crate::types::SequenceEncoding;
use crate::types::MultiAlignment;


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
    ZoomBy(f32),
    EncodingChanged(SequenceEncoding),
}

#[derive(Debug)]
pub struct AlignmentState {
    pub sequences: Box<MultiAlignment>,
    pub max_len: usize,
    pub display_encoding: SequenceEncoding,
    pub display_combobox: combo_box::State<SequenceEncoding>,

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
    pub fn new(sequences: Box<MultiAlignment>, cell_size: f32, scroll_speed: f32) -> Self {
        let max_len = sequences.iter().map(|s| s.original_seq.len()).max().unwrap_or(0);
        Self {
            sequences: sequences,
            max_len: max_len,
            display_encoding: SequenceEncoding::Nucleotidic,
            display_combobox: combo_box::State::new(vec![
                SequenceEncoding::Nucleotidic, SequenceEncoding::Aminoacidic
            ]),
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
                self.invalidate_cache();
            },
            AlignmentEvent::ZoomBy(z) => {
                self.cell_size = (self.cell_size + z).clamp(MIN_ZOOM, MAX_ZOOM);
                self.invalidate_cache();
            },
            AlignmentEvent::EncodingChanged(new_encoding) => {
                utils::update_alignment_translation(self, &new_encoding);
                self.display_encoding = new_encoding;
                self.invalidate_cache();
            }
        }
    }

    fn invalidate_cache(&self) {
        self.grid_cache.clear();
        self.overlay_cache.clear();
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
                    if row < self.state.sequences.len() && col < self.state.sequences[row].original_seq.len() {
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

                dx *= 2.0;

                if state.kb_mods.control() {
                    return Some(
                        canvas::Action::publish(AlignmentEvent::ZoomBy(if dy > 0.0 {1.0} else {-1.0}))
                    )
                }

                if state.kb_mods.shift() {
                    (dx, dy) = (dy, dx);
                }

                if state.kb_mods.alt() {
                    dx *= 10.0;
                    dy *= 10.0;
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
        let is_aa = self.state.display_encoding == Aminoacidic;
        let seqs: Vec<Option<&Vec<char>>> = self.state.sequences.iter().map(|s|
            if is_aa { s.aa_seq.as_ref() } else { Some(&s.original_seq) }
        ).collect();
        let cs = self.state.cell_size as f32;
        let pad = cs * 0.2;
        let (scroll_x, scroll_y) = (self.state.scroll_offset.x, self.state.scroll_offset.y);

        let grid_geometry = self.state.grid_cache.draw(renderer, bounds.size(), |frame| {
            let stroke = Stroke::default().with_color(Color::BLACK);
            let first_row = (scroll_y / cs).floor().max(0.0) as usize;
            let last_row = ((scroll_y + bounds.height) / cs)
                .ceil()
                .min(seqs.len() as f32) as usize;

            let first_col = (scroll_x / cs).floor().max(0.0) as usize;
            let last_col = ((scroll_x + bounds.width) / cs)
                .ceil()
                .min(self.state.max_len as f32) as usize;

            for row_idx in first_row..last_row {
                if seqs[row_idx].is_none() { continue; }
                let row = seqs[row_idx].unwrap();
                let y = row_idx as f32 * cs - scroll_y;
                let row_end_col = last_col.min(row.len());
                let mut txt_builder = Builder::new();

                for col_idx in first_col..row_end_col {
                    let base = row[col_idx];
                    let x = col_idx as f32 * cs - scroll_x;
                    let rect_pos = Point::new(x, y);
                    let base_color = utils::get_base_color(base, is_aa);
                
                    frame.fill_rectangle(
                        rect_pos,
                        iced::Size::new(cs, cs),
                        base_color,
                    );

                    if cs >= MIN_ZOOM_FOR_TEXT {
                        push_glyph(&mut txt_builder, base as u8, x + pad, y + pad, cs - pad*2.0);
                    }
                }

                frame.stroke(&txt_builder.build(), stroke);
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