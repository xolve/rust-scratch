use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {    
    let resp = reqwest::blocking::get("http://www.gnu.org")?;
    let resp = resp.bytes().unwrap();
    let content = String::from_utf8_lossy(&resp);
    println!("content: {content}");

    Ok(())
}