#[derive(Copy, Clone)]
pub enum Color {
    RGB(u8, u8, u8),
    Red, Orange, Yellow, Green, Cyan, Blue, Purple
}

impl From<Color> for (u8, u8, u8) {
    fn from(value: Color) -> Self {
        match value {
            Color::RGB(r, g, b) => (r, g, b),
            Color::Red => (255, 0, 0),
            Color::Orange => (255, 165, 0),
            Color::Yellow => (255, 255, 0),
            Color::Green => (0, 128, 0),
            Color::Cyan => (0, 255, 255),
            Color::Blue => (0, 0, 255),
            Color::Purple => (128, 0, 128),
        }
    }
}


pub trait Colorful<T> {
    fn fg(&self, color: Color) -> T;
    fn bg(&self, color: Color) -> T;
    fn bold(&self) -> T;
    fn italic(&self) -> T;
    fn underline(&self) -> T;
}


impl Colorful<String> for String {
    fn fg(&self, color: Color) -> String {
        let (r, g, b) = color.into();
        format!("\x1B[38;2;{r};{g};{b}m{self}\x1B[0m")
    }
    fn bg(&self, color: Color) -> String {
        let (r, g, b) = color.into();
        format!("\x1B[48;2;{r};{g};{b}m{self}\x1B[0m")
    }
    fn bold(&self) -> String {
        format!("\x1B[1m{self}\x1B[0m")
    }
    fn italic(&self) -> String {
        format!("\x1B[3m{self}\x1B[0m")
    }
    fn underline(&self) -> String {
        format!("\x1B[4m{self}\x1B[0m")
    }
}

impl Colorful<String> for str {
    fn bg(&self, color: Color) -> String {
        self.to_owned().bg(color)
    }
    fn fg(&self, color: Color) -> String {
        self.to_owned().fg(color)
    }
    fn bold(&self) -> String {
        self.to_owned().bold()
    }
    fn italic(&self) -> String {
        self.to_owned().italic()
    }
    fn underline(&self) -> String {
        self.to_owned().underline()
    }
}

#[cfg(test)]
mod test {
    use crate::rainbow::{Color, Colorful};

    #[test]
    fn test() {
        println!("{}", "hello".bold().italic().underline());
    }
}