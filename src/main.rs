use std::{io, io::Read};

fn main() -> Result<(), Box<std::error::Error + 'static>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let out: String = input.split('-').filter_map(|n| n.chars().next()).collect();
    println!("{}", out);
    Ok(())
}
