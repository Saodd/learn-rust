use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (keyword, filepath) = parse_args()?;
    let mut client = learn_rust::minigrep::Client::new(&keyword, &filepath)?;
    if env::var("IGNORE_CASE").is_ok() {
        client.set_ignore_case();
    }
    for line in client.search() {
        println!("{}", line)
    }
    Ok(())
}

fn parse_args() -> Result<(String, String), &'static str> {
    let mut args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err("you should input exact 3 args: xxx keyword filepath");
    }
    let keywords = args.remove(1);
    let filepath = args.remove(1);

    return Ok((keywords, filepath));
}
