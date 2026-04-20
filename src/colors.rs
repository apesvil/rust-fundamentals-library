//! Colorized output utilities for the terminal using ANSI escape codes.
//! # Examples:
//! ```
//! use my_library::colors::*;
//! println!("{}{}{}", red("Red"), green("Green"), blue("Blue"));
//! ```

/// Returns a string with the ANSI escape code for red.
/// # Examples:
/// ```
/// use my_library::colors::*;
/// println!("{}", red("Red"));
/// ```
pub fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}

/// Returns a string with the ANSI escape code for green.
/// # Examples:
/// ```
/// use my_library::colors::*;
/// println!("{}", green("Green"));
/// ```
pub fn green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", s)
}

/// Returns a string with the ANSI escape code for blue.
/// # Examples:
/// ```
/// use my_library::colors::*;
/// println!("{}", blue("Blue"));
/// ```
pub fn blue(s: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", s)
}

/// Returns a string with the ANSI escape code for bold.
/// # Examples:
/// ```
/// use my_library::colors::*;
/// println!("{}", bold("Bold"));
/// ```
pub fn bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}

/// Returns a string with the ANSI escape code for resetting the color.
/// # Examples:
/// ```
/// use my_library::colors::*;
/// println!("{}", reset("Reset"));
/// ```
pub fn reset(s: &str) -> String {
    format!("\x1b[0m{}\x1b[0m", s)
}

/// An enum representing the available colors for colorizing strings.
/// # Examples:
/// ```
/// use my_library::colors::Color;
/// let color = Color::Red;
/// ```
pub enum Color{
    Red,
    Green,
    Blue,
    Bold,
}

/// A struct that contains a color, a string, a colorized version of the string, and a reset counter.
/// # Examples:
/// ```
/// use my_library::colors::{Color, ColorString};
/// let mut color_string = ColorString { color: Color::Red, string: String::from("Hello"), colorized: String::new(), reset_counter: 0 };
/// println!("{}", color_string.string);
/// ```
pub struct ColorString {
    pub color: Color,
    pub string: String,
    pub colorized: String,
    pub reset_counter: usize,
}

/// Methods for the ColorString struct.
impl ColorString {
    /// This method uses the `color` and `string` fields to create a colorized string and assigns it to the `colorized` field.
    /// # Examples:
    /// ```
    /// use my_library::colors::{Color, ColorString};
    /// let mut color_string = ColorString { color: Color::Red, string: String::from("Hello"), colorized: String::new(), reset_counter: 0 };
    /// color_string.paint();
    /// println!("{}", color_string.colorized);
    /// ```
    pub fn paint(&mut self) {
        match self.color {
            Color::Red => self.colorized = red(&self.string),
            Color::Green => self.colorized = green(&self.string),
            Color::Blue => self.colorized = blue(&self.string),
            Color::Bold => self.colorized = bold(&self.string),
        };
    }

    /// This method resets the colorized string to the original string with the reset ANSI escape code and increments the reset counter.
    /// # Examples:
    /// ```
    /// use my_library::colors::{Color, ColorString};
    /// let mut color_string = ColorString { color: Color::Red, string: String::from("Hello"), colorized: String::new(), reset_counter: 0 };
    /// color_string.paint();
    /// println!("{}", color_string.colorized);
    /// color_string.reset();
    /// println!("{}", color_string.colorized);
    /// ```
    pub fn reset(&mut self) {
        self.colorized = reset(&self.string);
        self.reset_counter += 1;
    }

}
