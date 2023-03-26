mod garden;

fn main() {
    let cab = garden::cabbage::Cabbage::new();
    println!("I'm growing {:?}, it says: {}", cab, cab.speak())
}