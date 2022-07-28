use bitvec::prelude::*;

fn main() {
    let array = [21u8];
    let bits = array.view_bits::<Lsb0>();
    println!("{:?}", bits);
}
