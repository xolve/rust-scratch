#[allow(unused_variables)]
fn main() {
    use std::collections::HashMap;

    let mut ranks: HashMap<usize, String> = Default::default();
    ranks.insert(1, "Aung".to_string());
    ranks.insert(2, "Bikas".to_string());
    ranks.insert(3, "Chiyoko".to_string());

    let more_names = vec!["Đình", "Edward", "Freyde"];

    let max: Vec<String> = more_names.iter().enumerate().map(|(i, name)| {
        ranks.insert(i + 4, name.to_string());
        let x = ranks.values().max().unwrap();
        x.to_string()
    }).collect();

    let max = || {
        ranks.insert(4, "Đình".to_string());
        let max_name = ranks.values().max().unwrap().clone();
        max_name
    };
}