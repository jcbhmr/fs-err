use std::error::Error;
// use fs_err::fs::File;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("foo.txt")?;
    println!("{:?}", file);
    Ok(())
}
