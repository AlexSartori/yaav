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

pub const COLOR_CELL_HOVERED: Color = color!(230, 230, 230);
pub const COLOR_CELL_SELECTED: Color = color!(200, 10, 10);

pub const COLOR_NT_A: Color = color!(243, 165, 188);
pub const COLOR_NT_C: Color = color!(148, 184, 219);
pub const COLOR_NT_G: Color = color!(171, 207, 160);
pub const COLOR_NT_T: Color = color!(251, 209, 136);
pub const COLOR_NT_N: Color = color!(150, 150, 150);
pub const COLOR_NT_Z: Color = color!(41, 57, 81);

// Grazie gemini per la palette raggruppata per tipo, è gay e bellissima
// --- IDROFOBICI / ALIFATICI (Toni Pesca, Giallo Caldo, Beige) ---
pub const COLOR_AA_A: Color = color!(247, 214, 152); // Alanina (Ala)
pub const COLOR_AA_V: Color = color!(243, 198, 138); // Valina (Val)
pub const COLOR_AA_L: Color = color!(238, 184, 132); // Leucina (Leu)
pub const COLOR_AA_I: Color = color!(234, 174, 136); // Isoleucina (Ile)
pub const COLOR_AA_M: Color = color!(245, 224, 146); // Metionina (Met)
// --- AROMATICI (Toni Verde Salvia / Menta / Acquamarina) ---
pub const COLOR_AA_F: Color = color!(164, 212, 182); // Fenilalanina (Phe)
pub const COLOR_AA_Y: Color = color!(154, 206, 194); // Tirosina (Tyr)
pub const COLOR_AA_W: Color = color!(146, 198, 188); // Triptofano (Trp)
// --- POLARI NON CARICHI (Toni Verde Chiaro / Azzurro Ghiaccio) ---
pub const COLOR_AA_S: Color = color!(184, 219, 168); // Serina (Ser)
pub const COLOR_AA_T: Color = color!(172, 212, 176); // Treonina (Thr)
pub const COLOR_AA_N: Color = color!(168, 208, 218); // Asparagina (Asn)
pub const COLOR_AA_Q: Color = color!(158, 198, 214); // Glutammina (Gln)
// --- CARICHI POSITIVAMENTE / BASICI (Toni Blu Cielo / Pervinca) ---
pub const COLOR_AA_K: Color = color!(146, 178, 224); // Lisina (Lys)
pub const COLOR_AA_R: Color = color!(138, 168, 218); // Arginina (Arg)
pub const COLOR_AA_H: Color = color!(166, 176, 226); // Istidina (His)
// --- CARICHI NEGATIVAMENTE / ACIDI (Toni Rosa / Malva / Lilla) ---
pub const COLOR_AA_D: Color = color!(241, 158, 186); // Aspartato (Asp)
pub const COLOR_AA_E: Color = color!(232, 154, 196); // Glutammato (Glu)
// --- SPECIALI (Toni Lavanda, Zolfo, Neutro) ---
pub const COLOR_AA_C: Color = color!(246, 232, 148); // Cisteina (Cys - giallo zolfo)
pub const COLOR_AA_G: Color = color!(212, 214, 218); // Glicina (Gly - grigio neutro)
pub const COLOR_AA_P: Color = color!(214, 178, 218); // Prolina (Pro - lavanda)
// --- EXTRA ---
pub const COLOR_AA_STOP: Color = color!(235, 136, 146); // Stop (*)
pub const COLOR_AA_X: Color = color!(195, 198, 202);    // Sconosciuto (X)

pub const PALETTE: Palette = Palette {
    background: COLOR_BG,
    text: COLOR_PRI,
    primary: COLOR_PRI,
    success: COLOR_SUC,
    danger: COLOR_DNG,
    warning: COLOR_WRN,
};