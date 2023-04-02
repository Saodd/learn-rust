#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    pub fn new(width: u32, height: u32) -> Rectangle {
        return Rectangle {
            width,
            height,
        };
    }
}


#[cfg(test)]
mod tests {
    use crate::Rectangle;

    #[test]
    fn can_hold() {
        let small = Rectangle {
            width: 2,
            height: 4,
        };
        let big = Rectangle {
            width: 3,
            height: 5,
        };

        assert!(big.can_hold(&small));
        assert!(!small.can_hold(&big));
    }
}
