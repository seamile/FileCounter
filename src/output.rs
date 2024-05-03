use std::error::Error;
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

#[allow(unused)]
pub fn color_me(arg: &dyn Display, color: Color, effect: Effect) -> String {
    return format!("\x1b[{};{}m{}\x1b[0m", effect as u8, color as u8, arg);
}

#[allow(unused)]
pub fn title(arg: &dyn Display) -> String {
    return color_me(arg, Color::BrightGreen, Effect::Underline);
}

#[allow(unused)]
pub fn info(arg: &dyn Display) -> String {
    return color_me(arg, Color::BrightCyan, Effect::Default);
}

#[allow(unused)]
pub fn note(arg: &dyn Display) -> String {
    return color_me(arg, Color::Magenta, Effect::Default);
}

#[allow(unused)]
pub fn warn(arg: &dyn Display) -> String {
    return color_me(arg, Color::Yellow, Effect::Bold);
}

#[allow(unused)]
pub fn error(arg: &dyn Display) -> String {
    return color_me(arg, Color::Red, Effect::Bold);
}

#[allow(unused)]
pub fn strong(arg: &dyn Display) -> String {
    return color_me(arg, Color::Default, Effect::Bold);
}

pub fn print_err(err: &dyn Error, msg: &dyn Display) {
    let head = err.to_string();
    eprintln!("{}: {}", error(&head), msg)
}

pub fn fill_char(chr: char, width: usize) -> String {
    let s = vec![chr as u16; width];
    return String::from_utf16(&s).unwrap();
}

pub fn spaces(width: usize) -> String {
    return fill_char(' ', width);
}

pub fn display_width(s: &String) -> usize {
    return s
        .chars()
        .map(|c| if c as u32 > 0x2e80 { 2_usize } else { 1_usize })
        .sum::<usize>();
}

#[allow(unused)]
pub fn center_justify(s: &dyn ToString, width: usize) -> String {
    let mut string = s.to_string();
    let d_width = display_width(&string);
    if d_width < width {
        let n_fill = width - d_width;
        if n_fill % 2 == 0 {
            let fill = spaces(n_fill / 2);
            string.insert_str(string.len(), &fill);
            string.insert_str(0, &fill);
        } else {
            string.insert_str(string.len(), &spaces(n_fill / 2 + 1));
            string.insert_str(0, &spaces(n_fill / 2));
        }
    }
    return string;
}

#[allow(unused)]
pub fn left_justify(s: &dyn ToString, width: usize) -> String {
    let mut string = s.to_string();
    let d_width = display_width(&string);
    if d_width < width {
        let n_fill = width - d_width;
        string.insert_str(string.len(), &spaces(n_fill));
    }
    return string;
}

#[allow(unused)]
pub fn right_justify(s: &dyn ToString, width: usize) -> String {
    let mut string = s.to_string();
    let d_width = display_width(&string);
    if d_width < width {
        let n_fill = width - d_width;
        string.insert_str(0, &spaces(n_fill));
    }
    return string;
}

#[test]
fn test_color() {
    let s = "Hello World";
    println!(
        "yellow + underline: {}\n",
        color_me(&s, Color::Yellow, Effect::Underline)
    );
    println!("title: {}", title(&s));
    println!("info : {}", info(&s));
    println!("warn : {}", warn(&s));
    println!("error  : {}", error(&s));
}

#[test]
fn test_align() {
    let s = "HelloWorld";
    assert_eq!(center_justify(&s, 14), String::from("  HelloWorld  "));
    assert_eq!(center_justify(&s, 15), String::from("  HelloWorld   "));
    assert_eq!(left_justify(&s, 15), String::from("HelloWorld     "));
    assert_eq!(right_justify(&s, 15), String::from("     HelloWorld"));

    let t = title(
        &vec![
            left_justify(&"Name", 8),
            right_justify(&"Files", 5),
            right_justify(&"Dirs", 5),
            right_justify(&"Size", 9),
        ]
        .join(" "),
    );

    println!("{}", t);
    assert_eq!(
        t,
        "\x1b[4;92mName     Files  Dirs      Size\x1b[0m".to_string()
    );
}

#[test]
fn test_non_ascii() {
    let s1 = "Hello World!";
    let s2 = "你好 Rust";
    let s3 = "Séamile: 🌊😀";
    let s4 = "EVA，人の作り出した物";

    let aligned_s1 = right_justify(&s1, 25);
    let aligned_s2 = center_justify(&s2, 25);
    let aligned_s3 = left_justify(&s3, 25);
    let aligned_s4 = left_justify(&s4, 25);

    println!("s1 => |{}|", aligned_s1);
    println!("s2 => |{}|", aligned_s2);
    println!("s3 => |{}|", aligned_s3);
    println!("s3 => |{}|", aligned_s4);

    assert_eq!(aligned_s1.len(), 25);
    assert_eq!(aligned_s2.len(), 27);
    assert_eq!(aligned_s3.len(), 30);
    assert_eq!(aligned_s4.len(), 34);
}

#[test]
fn test_color_me() {
    assert_eq!(
        color_me(&"hello", Color::Red, Effect::Bold),
        "\x1b[1;31mhello\x1b[0m"
    );
    assert_eq!(
        color_me(&"world", Color::Green, Effect::Underline),
        "\x1b[4;32mworld\x1b[0m"
    );
}

#[test]
fn test_title() {
    assert_eq!(title(&"Welcome"), "\x1b[4;92mWelcome\x1b[0m");
}

#[test]
fn test_info() {
    assert_eq!(
        info(&"This is an information"),
        "\x1b[0;96mThis is an information\x1b[0m"
    );
}

#[test]
fn test_note() {
    assert_eq!(
        note(&"Note: this is important"),
        "\x1b[0;35mNote: this is important\x1b[0m"
    );
}

#[test]
fn test_warn() {
    assert_eq!(
        warn(&"Warning: something may go wrong"),
        "\x1b[1;33mWarning: something may go wrong\x1b[0m"
    );
}

#[test]
fn test_error() {
    assert_eq!(
        error(&"Error: something went wrong"),
        "\x1b[1;31mError: something went wrong\x1b[0m"
    );
}

#[test]
fn test_strong() {
    assert_eq!(strong(&"strong text"), "\x1b[1;99mstrong text\x1b[0m");
}

#[test]
fn test_fill_char() {
    assert_eq!(fill_char('-', 5), "-----");
    assert_eq!(fill_char('*', 3), "***");
}

#[test]
fn test_spaces() {
    assert_eq!(spaces(3), "   ");
    assert_eq!(spaces(0), "");
}

#[test]
fn test_display_width() {
    assert_eq!(display_width(&"hello".to_string()), 5);
    assert_eq!(display_width(&"你好".to_string()), 4);
    assert_eq!(display_width(&"abc你好".to_string()), 6);
}

#[test]
fn test_center_justify() {
    assert_eq!(center_justify(&"hello", 10), "  hello   ");
    assert_eq!(center_justify(&"world", 9), "  world  ");
}

#[test]
fn test_left_justify() {
    assert_eq!(left_justify(&"hello", 8), "hello   ");
    assert_eq!(left_justify(&"world", 6), "world ");
}

#[test]
fn test_right_justify() {
    assert_eq!(right_justify(&"hello", 8), "   hello");
    assert_eq!(right_justify(&"world", 6), " world");
}
