// Binds for foreground colors
//
// How to use:
//
// ----------------
// 
// use colorism::{foreground::Fore, util::RESET};
//
// fn main() {
//     println!("{}Hello!{}", Fore::color(Fore::Green), RESET);
// }

pub enum Fore {
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
    BdWhite,
}

impl Fore {
    pub fn color(color: Fore) -> &'static str {
        match color {
            Fore::Black => "\x1b[30m",
            Fore::Red => "\x1b[31m",
            Fore::Green => "\x1b[32m",
            Fore::Yellow => "\x1b[33m",
            Fore::Blue => "\x1b[34m",
            Fore::Magenta => "\x1b[35m",
            Fore::Cyan => "\x1b[36m",
            Fore::White => "\x1b[37m",
            Fore::BdBlack => "\x1b[1;30m",
            Fore::BdRed => "\x1b[1;31m",
            Fore::BdGreen => "\x1b[1;32m",
            Fore::BdYellow => "\x1b[1;33m",
            Fore::BdBlue => "\x1b[1;34m",
            Fore::BdMagenta => "\x1b[1;35m",
            Fore::BdCyan => "\x1b[1;36m",
            Fore::BdWhite => "\x1b[1;37m",
        }
    }
}
