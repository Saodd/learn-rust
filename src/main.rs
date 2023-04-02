use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let client = learn_rust::minigrep::Client::new(&args)?;
    for line in client.search() {
        println!("{}", line)
    }
    Ok(())
}
