use std::error::Error;
use fs_err::env::var;
fn main() -> Result<(), Box<dyn Error>> {
    let message = var("MESSAGE")?;
    println!("message={:?}", message);
    Ok(())
}
