<div align="center">
<h2>Colorism</h2>
<p>A library to use terminal ANSI colors</p>
<img src="https://img.shields.io/crates/v/colorism.svg" />
</div>

&nbsp;

## ❗️ Install:
```bash
cargo add colorism
```

&nbsp;

## 🚀 Usage:

Using the foreground method:
```rust
// Import the fore method and RESET
use colorism::{foreground::Fore, util::RESET};

// Use RESET on all string ends, if you don't, the colors will escape to your terminal and will be really ugly, but not danger.
fn main() {
    // Green regular text
    println!("{}Hello, world!{}", Fore::color(Fore::Green), RESET);

    // Green bold text
    println!("{}Hello, world!{}", Fore::color(Fore::BdGreen), RESET);
}
```

&nbsp;

Using the background method:
```rust
use colorism::{background::Back, util::RESET};

// Use RESET on all string ends, if you don't, the colors will escape to your terminal and will be really ugly, but not danger.
fn main() {
    // Green background, white text
    println!("{}Hello, world!{}", Back::color(Back::Green), RESET);

    // Green background, white bold text
    println!("{}Hello, world!{}", Back::color(Fore::BdGreen), RESET);
}
```

&nbsp;

Using the utils:
```rust
// Import the util and (We will use Style to styling texts) RESET
use colorism::util::{Style, RESET};

// Use RESET on all string ends, if you don't, the colors will escape to your terminal and will be really ugly, but not danger.
fn main() {
    // Simple bold text
    println!("{}I am a text{}", Style::text(Style::Bold), RESET);

    // Simple underline text
    println!("{}I am a text{}", Style::text(Style::Underline), RESET);
}
```
