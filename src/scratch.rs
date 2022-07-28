#[macro_use]
extern crate serde_json;

fn main() {
    let x = json!({
        "hello": "world"
    });

    println!("{x:?}\n");
}