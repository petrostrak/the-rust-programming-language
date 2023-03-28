// Implement IntoIterator using a custom Iterator

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

// ColorIntoIter is a proxy struct
struct ColorIntoIter {
    color: Color,
    position: u8,
}

// ColorIter is a proxy struct
struct ColorIter<'a> {
    color: &'a Color,
    position: u8,
}

impl<'a> Iterator for ColorIter<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.position {
            0 => Some(self.color.r),
            1 => Some(self.color.g),
            2 => Some(self.color.b),
            _ => None,
        };
        self.position += 1;
        next
    }
}

impl<'a> IntoIterator for &'a Color {
    type Item = u8;
    type IntoIter = ColorIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            color: &self,
            position: 0,
        }
    }
}

impl Iterator for ColorIntoIter {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.position {
            0 => Some(self.color.r),
            1 => Some(self.color.g),
            2 => Some(self.color.b),
            _ => None,
        };
        self.position += 1;
        next
    }
}

impl IntoIterator for Color {
    type Item = u8;
    type IntoIter = ColorIntoIter;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            color: self,
            position: 0,
        }
    }
}

fn main() {
    let color: Color = Color {
        r: 10,
        g: 20,
        b: 30,
    };

    for c in &color {
        println!("{}", c)
    }
    for c in &color {
        println!("{}", c)
    }
}
