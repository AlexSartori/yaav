use iced::{
    Color, theme::Palette
};

macro_rules! color {
    ($r:expr, $g:expr, $b:expr) => {
        Color{r: ($r as f32)/255.0, g: ($g as f32)/255.0, b: ($b as f32)/255.0, a: 1.0}
    };
}

// const COLOR_BG:  Color = color!(43, 65, 98);
const COLOR_BG:  Color = color!(31, 47, 71);
const COLOR_PRI: Color = color!(202, 229, 255);
const COLOR_DNG: Color = color!(255, 153, 155);
const COLOR_SUC: Color = color!(170, 246, 131);
const COLOR_WRN: Color = color!(234, 220, 113);

pub const COLOR_NT_A: Color = color!(243, 165, 188);
pub const COLOR_NT_C: Color = color!(148, 184, 219);
pub const COLOR_NT_G: Color = color!(171, 207, 160);
pub const COLOR_NT_T: Color = color!(251, 209, 136);
pub const COLOR_NT_N: Color = color!(150, 150, 150);
pub const COLOR_NT_Z: Color = color!(41, 57, 81);

pub const PALETTE: Palette = Palette {
    background: COLOR_BG,
    text: COLOR_PRI,
    primary: COLOR_PRI,
    success: COLOR_SUC,
    danger: COLOR_DNG,
    warning: COLOR_WRN
};