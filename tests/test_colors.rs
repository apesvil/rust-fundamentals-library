use my_library::colors::{Color, ColorString};

#[test]

fn test_red_coloring() {
    let mut s = ColorString {
        color: Color::Red,
        string: "hello".to_string(),
        colorized: String::new(),
        reset_counter: 0
    };
    s.paint();
    assert_eq!(s.colorized, "\x1b[31mhello\x1b[0m");
}