use std::{fs::File, net::{Ipv6Addr}};
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {    
    let _f = File::open("/root/hello.txt")?;

    let _i = "::1".parse::<Ipv6Addr>()?;

    Ok(())
}