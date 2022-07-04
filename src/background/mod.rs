// Binds for background colors
//
// How to use:
//
// --------------------
//
// use colorism::{background::Back, util::RESET};
//
// fn main() {
//     println!("{}Hello!{}", Back::color(Back::Green), RESET);
// }

pub enum Back {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BdBlack,
    BdRed,
    BdGreen,
    BdYellow,
    BdBlue,
    BdMagenta,
    BdCyan,
    BdWhite
}

impl Back {
    pub fn color(color: Back) -> &'static str {
        match color {
            Back::Black => "\x1b[40m",
            Back::Red => "\x1b[41m",
            Back::Green => "\x1b[42m",
            Back::Yellow => "\x1b[43m",
            Back::Blue => "\x1b[44m",
            Back::Magenta => "\x1b[45m",
            Back::Cyan => "\x1b[46m",
            Back::White => "\x1b[47m",
            Back::BdBlack => "\x1b[1;40m",
            Back::BdRed => "\x1b[1;41m",
            Back::BdGreen => "\x1b[1;42m",
            Back::BdYellow => "\x1b[1;43m",
            Back::BdBlue => "\x1b[1;44m",
            Back::BdMagenta => "\x1b[1;45m",
            Back::BdCyan => "\x1b[1;46m",
            Back::BdWhite => "\x1b[1;47m",
        }
    }
}
