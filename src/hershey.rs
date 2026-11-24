use iced::Point;
use iced::widget::canvas::path::Builder;

/// Coordinates in eighths of width. (M, 0) means "move_to" next point. Also, the first
/// vertex of every non-empty slice is implicitly a move_to.
const M: u8 = 255;
const GLYPH_LUT: [&[(u8, u8)]; 128] = {
    let mut t: [&[(u8, u8)]; 128] = [&[]; 128];

    t[b'A' as usize] = &[(0, 8), (4, 0), (8, 8), (M, 0), (2, 4), (6, 4)];
    t[b'C' as usize] = &[(8, 2), (6, 0), (2, 0), (0, 2), (0, 6), (2, 8), (6, 8), (8, 6)];
    t[b'D' as usize] = &[(0, 0), (6, 0), (8, 2), (8, 6), (6, 8), (0, 8), (0, 0)];
    t[b'E' as usize] = &[(8, 0), (0, 0), (0, 8), (8, 8), (M, 0), (0, 4), (6, 4)];
    t[b'F' as usize] = &[(0, 8), (0, 0), (8, 0), (M, 0), (0, 4), (6, 4)];
    t[b'G' as usize] = &[(8, 2), (6, 0), (2, 0), (0, 2), (0, 6), (2, 8), (8, 8), (8, 4), (4, 4)];
    t[b'H' as usize] = &[(0, 0), (0, 8), (M, 0), (8, 0), (8, 8), (M, 0), (0, 4), (8, 4)];
    t[b'I' as usize] = &[(4, 0), (4, 8), (M, 0), (2, 0), (6, 0), (M, 0), (2, 8), (6, 8)];
    t[b'K' as usize] = &[(0, 0), (0, 8), (M, 0), (8, 0), (0, 4), (8, 8)];
    t[b'L' as usize] = &[(0, 0), (0, 8), (8, 8)];
    t[b'M' as usize] = &[(0, 8), (0, 0), (4, 4), (8, 0), (8, 8)];
    t[b'N' as usize] = &[(0, 8), (0, 0), (8, 8), (8, 0)];
    t[b'P' as usize] = &[(0, 8), (0, 0), (6, 0), (8, 1), (8, 3), (6, 4), (0, 4)];
    t[b'Q' as usize] = &[(2, 0), (6, 0), (8, 2), (8, 6), (6, 8), (2, 8), (0, 6), (0, 2), (2, 0), (M, 0), (4, 6), (8, 8)];
    t[b'R' as usize] = &[(0, 8), (0, 0), (6, 0), (8, 1), (8, 3), (6, 4), (0, 4), (M, 0), (4, 4), (8, 8)];
    t[b'S' as usize] = &[(8, 1), (6, 0), (2, 0), (0, 1), (0, 3), (8, 5), (8, 7), (6, 8), (2, 8), (0, 7)];
    t[b'T' as usize] = &[(4, 8), (4, 0), (M, 0), (0, 0), (8, 0)];
    t[b'V' as usize] = &[(0, 0), (4, 8), (8, 0)];
    t[b'W' as usize] = &[(0, 0), (2, 8), (4, 4), (6, 8), (8, 0)];
    t[b'Y' as usize] = &[(0, 0), (4, 4), (8, 0), (M, 0), (4, 4), (4, 8)];
    t[b'B' as usize] = &[(0, 0), (6, 0), (8, 1), (8, 3), (6, 4), (0, 4),(M, 0), (6, 4), (8, 5), (8, 7), (6, 8), (0, 8), (0, 0)];
    t[b'O' as usize] = &[(2, 0), (6, 0), (8, 2), (8, 6), (6, 8), (2, 8), (0, 6), (0, 2), (2, 0)];
    t[b'U' as usize] = &[(0, 0), (0, 6), (2, 8), (6, 8), (8, 6), (8, 0)];
    t[b'X' as usize] = &[(0, 0), (8, 8), (M, 0), (8, 0), (0, 8)];
    t[b'Z' as usize] = &[(0, 0), (8, 0), (0, 8), (8, 8)];
    t[b'*' as usize] = &[(4, 0), (4, 8), (M, 0), (0, 2), (8, 6), (M, 0), (8, 2), (0, 6)];
    t[b'-' as usize] = &[(0, 4), (8, 4)];
    t[b'?' as usize] = &[(0, 4), (0, 2), (2, 0), (6, 0), (8, 2), (8, 4), (4, 4), (4, 6), (M, 0), (4, 7), (4, 8)];

    // Mirror uppercase entries into lowercase ASCII slots (`a..=z`) for branchless case-insensitivity
    let mut c = b'A';
    while c <= b'Z' {
        t[(c + 32) as usize] = t[c as usize];
        c += 1;
    }

    t
};


#[inline(always)]
pub fn push_glyph(b: &mut Builder, c: u8, x0: f32, y0: f32, cs: f32) {
    let cmds = GLYPH_LUT[(c & 0x7F) as usize];
    if cmds.is_empty() {
        return;
    }

    let step = cs * 0.125;
    let mut move_next = true;

    for &(dx, dy) in cmds {
        if dx == M {
            move_next = true;
            continue;
        }
        let pt = Point {
            x: x0 + (dx as f32) * step,
            y: y0 + (dy as f32) * step,
        };
        if move_next {
            b.move_to(pt);
            move_next = false;
        } else {
            b.line_to(pt);
        }
    }
}

// #[inline]
// pub fn aa_path(aa: u8, x0: f32, y0: f32, cs: f32) -> Path {
//     Path::new(|b| push_aa_glyph(b, aa, x0, y0, cs))
// }