#[derive(Debug)]
pub struct Cabbage {
    days: u32,
}

impl Cabbage {
    pub fn new() -> Self {
        return Cabbage { days: 0 };
    }

    pub fn speak(&self) -> String {
        format!("I'm {} days old!", self.days)
    }
}
