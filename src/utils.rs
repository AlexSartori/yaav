use crate::{alignment_viewer::AlignmentState, components, types::SequenceEncoding};

pub fn update_alignment_translation(state: &mut AlignmentState, to: &SequenceEncoding) {
    match state.display_encoding {
        SequenceEncoding::Nucleotidic => match to {
            SequenceEncoding::Aminoacidic => state.sequences.iter_mut().for_each(|s| s.update_aa_translation()),
            _ => (),
        }
        _ => match to {
            _ => ()
        }
    }
}

pub fn get_base_color(c: char, is_aa: bool) -> iced::Color {
    if is_aa {
        match c {
            'A' => components::COLOR_AA_A,
            'V' => components::COLOR_AA_V,
            'L' => components::COLOR_AA_L,
            'I' => components::COLOR_AA_I,
            'M' => components::COLOR_AA_M,
            'F' => components::COLOR_AA_F,
            'Y' => components::COLOR_AA_Y,
            'W' => components::COLOR_AA_W,
            'S' => components::COLOR_AA_S,
            'T' => components::COLOR_AA_T,
            'N' => components::COLOR_AA_N,
            'Q' => components::COLOR_AA_Q,
            'K' => components::COLOR_AA_K,
            'R' => components::COLOR_AA_R,
            'H' => components::COLOR_AA_H,
            'D' => components::COLOR_AA_D,
            'E' => components::COLOR_AA_E,
            'C' => components::COLOR_AA_C,
            'G' => components::COLOR_AA_G,
            'P' => components::COLOR_AA_P,
            '*' => components::COLOR_AA_STOP,
            _ => components::COLOR_AA_X,
        }
    } else {
        match c {
            'a' | 'A' => components::COLOR_NT_A,
            'c' | 'C' => components::COLOR_NT_C,
            'g' | 'G' => components::COLOR_NT_G,
            't' | 'T' => components::COLOR_NT_T,
            'z' | 'Z' | '-' | ' ' => components::COLOR_NT_Z,
            _ => components::COLOR_NT_N,
        }
    }
}