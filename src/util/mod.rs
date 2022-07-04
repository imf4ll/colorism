// Binds for util stuffs
//
// How to use:
//
// ----------------------
//
// use colorism::util::{Style, RESET};
//
// fn main() {
//     println!("{}test{}", Style::text(Style::Underline), RESET);
// }

pub const RESET: &str = "\x1b[m";

pub enum Style {
    Bold,
    Underline,
}

impl Style {
    pub fn text(style: Style) -> &'static str {
        match style {
            Style::Bold => "\x1b[1m",
            Style::Underline => "\x1b[4m",
        }
    }
}
