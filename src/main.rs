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
    let mut args = env::args();

    let _ = args.next();
    let keyword = match args.next() {
        None => return Err("Didn't get keyword argument"),
        Some(arg) => arg,
    };
    let filepath = match args.next() {
        None => return Err("Didn't get filepath argument"),
        Some(arg) => arg,
    };

    return Ok((keyword, filepath));
}
