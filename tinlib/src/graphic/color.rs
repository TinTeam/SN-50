//! Color implementation and manipulation.

/// A color representation with red, green and blue values.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    /// Creates a Color with red, green and blue values.
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    /// Creates a Color from a hexadecimal value.
    pub fn new_from_hex(value: u32) -> Self {
        Self {
            red: ((value & 0x00ff_0000) >> 16) as u8,
            green: ((value & 0x0000_ff00) >> 8) as u8,
            blue: (value & 0x0000_00ff) as u8,
        }
    }

    /// Get red value.
    ///
    pub fn red(self) -> u8 {
        self.red
    }

    /// Get green value.
    pub fn green(self) -> u8 {
        self.green
    }

    /// Get blue value.
    pub fn blue(self) -> u8 {
        self.blue
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}

impl From<[u8; 3]> for Color {
    fn from(array: [u8; 3]) -> Self {
        Self::new(array[0], array[1], array[2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_new() {
        let color = Color::new(1, 2, 3);

        assert_eq!(color.red, 1);
        assert_eq!(color.green, 2);
        assert_eq!(color.blue, 3);
    }

    #[test]
    fn test_color_new_from_hex() {
        let color = Color::new_from_hex(0x7bc950);

        assert_eq!(color.red, 123);
        assert_eq!(color.green, 201);
        assert_eq!(color.blue, 80);
    }

    #[test]
    fn test_color_red_green_blue() {
        let color = Color::new(1, 2, 3);

        assert_eq!(color.red(), 1);
        assert_eq!(color.green(), 2);
        assert_eq!(color.blue(), 3);
    }

    #[test]
    fn test_color_from_tuple() {
        let tuple = (1u8, 2u8, 3u8);
        let color = Color::from(tuple);

        assert_eq!(color.red, tuple.0);
        assert_eq!(color.green, tuple.1);
        assert_eq!(color.blue, tuple.2);
    }

    #[test]
    fn test_color_from_array() {
        let array = [1u8, 2u8, 3u8];
        let color = Color::from(array);

        assert_eq!(color.red, array[0]);
        assert_eq!(color.green, array[1]);
        assert_eq!(color.blue, array[2]);
    }
}
