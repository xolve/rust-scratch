use std::fmt::Display;

use rand::RngCore;

#[derive(Debug)]
struct MacAddress([u8; 6]);

impl Display for MacAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let octet = &self.0;
        write!(f,
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            octet[0], octet[1], octet[2], octet[3], octet[4], octet[5]
        )
    }
}

impl MacAddress {
    fn new() -> Self {
        let mut ret_val = MacAddress([0; 6]);
        rand::thread_rng().fill_bytes(&mut ret_val.0);
        ret_val
    }

    fn is_local(&self) -> bool {
        self.0[0] & 0b_00000010 > 0
    }

    fn is_unicast(&self) -> bool {
        self.0[0] & 0b_00000001 == 1
    }

    fn is_multicast(&self) -> bool {
        self.0[0] & 0b_00000001 == 0
    }
}

fn main() {
    let mac = MacAddress::new();
    println!("{} {} {} {}", mac, mac.is_local(), mac.is_unicast(), mac.is_multicast());
}