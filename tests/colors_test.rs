use colorism::{background::Back, foreground::Fore, util::{Style, RESET}};

#[test]
fn fore_green() {
    println!("Foreground green: {}test{}", Fore::color(Fore::Green), RESET);
}

#[test]
fn bold_green() {
    println!("Bold green: {}test{}", Fore::color(Fore::BdGreen), RESET);
}

#[test]
fn underline_blue() {
    println!("Underline blue: {}{}test{}", Fore::color(Fore::Blue), Style::text(Style::Underline), RESET);
}

#[test]
fn underline() {
    println!("Underline: {}test{}", Style::text(Style::Underline), RESET);
}

#[test]
fn bold() {
    println!("Bold: {}test{}", Style::text(Style::Bold), RESET);
}

#[test]
fn back_red() {
    println!("Background red: {}test{}", Back::color(Back::Red), RESET);
}

#[test]
fn back_bold_red() {
    println!("Background bold red: {}test{}", Back::color(Back::BdRed), RESET);
}
