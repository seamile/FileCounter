use std::fmt::Display;

#[allow(unused)]
pub enum Effect {
    Default = 0,
    Bold = 1,
    Dark = 2,
    Inverse = 3,
    Underline = 4,
    Blink = 5,
    Hidden = 8,
}

impl Effect {
    pub fn bold(arg: &dyn Display, color: Color) -> String {
        color_me(arg, color, Effect::Bold)
    }
    pub fn dark(arg: &dyn Display, color: Color) -> String {
        color_me(arg, color, Effect::Dark)
    }
    pub fn inverse(arg: &dyn Display, color: Color) -> String {
        color_me(arg, color, Effect::Inverse)
    }
    pub fn underline(arg: &dyn Display, color: Color) -> String {
        color_me(arg, color, Effect::Underline)
    }
    pub fn blink(arg: &dyn Display, color: Color) -> String {
        color_me(arg, color, Effect::Blink)
    }
    pub fn hidden(arg: &dyn Display, color: Color) -> String {
        color_me(arg, color, Effect::Hidden)
    }
}

#[allow(unused)]
pub enum Color {
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    White = 37,
    Grey = 90,
    BrightRed = 91,
    BrightGreen = 92,
    BrightYellow = 93,
    BrightBlue = 94,
    BrightMagenta = 95,
    BrightCyan = 96,
    BrightWhite = 97,
    Default = 99,
}

impl Color {
    pub fn black(arg: &dyn Display, effect: Effect) -> String {
        return color_me(arg, Color::Black, effect);
    }
    pub fn red(arg: &dyn Display, effect: Effect) -> String {
        return color_me(arg, Color::Red, effect);
    }
    pub fn green(arg: &dyn Display, effect: Effect) -> String {
        return color_me(arg, Color::Green, effect);
    }
    pub fn yellow(arg: &dyn Display, effect: Effect) -> String {
        return color_me(arg, Color::Yellow, effect);
    }
    pub fn blue(arg: &dyn Display, effect: Effect) -> String {
        return color_me(arg, Color::Blue, effect);
    }
    pub fn magenta(arg: &dyn Display, effect: Effect) -> String {
        return color_me(arg, Color::Magenta, effect);
    }
    pub fn cyan(arg: &dyn Display, effect: Effect) -> String {
        return color_me(arg, Color::Cyan, effect);
    }
    pub fn white(arg: &dyn Display, effect: Effect) -> String {
        return color_me(arg, Color::White, effect);
    }
    pub fn grey(arg: &dyn Display, effect: Effect) -> String {
        return color_me(arg, Color::Grey, effect);
    }
    pub fn bright_red(arg: &dyn Display, effect: Effect) -> String {
        return color_me(arg, Color::BrightRed, effect);
    }
    pub fn bright_green(arg: &dyn Display, effect: Effect) -> String {
        return color_me(arg, Color::BrightGreen, effect);
    }
    pub fn bright_yellow(arg: &dyn Display, effect: Effect) -> String {
        return color_me(arg, Color::BrightYellow, effect);
    }
    pub fn bright_blue(arg: &dyn Display, effect: Effect) -> String {
        return color_me(arg, Color::BrightBlue, effect);
    }
    pub fn bright_magenta(arg: &dyn Display, effect: Effect) -> String {
        return color_me(arg, Color::BrightMagenta, effect);
    }
    pub fn bright_cyan(arg: &dyn Display, effect: Effect) -> String {
        return color_me(arg, Color::BrightCyan, effect);
    }
    pub fn bright_white(arg: &dyn Display, effect: Effect) -> String {
        return color_me(arg, Color::BrightWhite, effect);
    }
}

pub fn color_me(arg: &dyn Display, color: Color, effect: Effect) -> String {
    return format!("\x1b[{};{}m{}\x1b[0m", effect as u8, color as u8, arg);
}

pub fn title(arg: &dyn Display) -> String {
    color_me(arg, Color::Default, Effect::Underline)
}

pub fn info(arg: &dyn Display) -> String {
    color_me(arg, Color::BrightBlue, Effect::Default)
}

pub fn warn(arg: &dyn Display) -> String {
    color_me(arg, Color::Yellow, Effect::Default)
}

pub fn err(arg: &dyn Display) -> String {
    color_me(arg, Color::Red, Effect::Default)
}

#[test]
fn test_color() {
    let s = "Hello World";
    println!(
        "yellow + underline: {}\n",
        Color::yellow(&s, Effect::Underline)
    );
    println!("title: {}", title(&s));
    println!("info: {}", info(&s));
    println!("warn: {}", warn(&s));
    println!("err: {}", err(&s));
}