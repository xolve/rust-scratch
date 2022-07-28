extern crate pest;

#[macro_use]
extern crate pest_derive;

use pest::Parser;

use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "ini.pest"]
pub struct INIParser;

fn main() {
    let file_content = include_str!("sample.ini");

    let x = INIParser::parse(Rule::file, file_content)
        .expect("error in parsing")
        .next()
        .unwrap();

    let mut properties: HashMap<&str, HashMap<&str, &str>> = Default::default();

    let mut current_section_name = "";
    for line in x.into_inner() {
        match line.as_rule() {
            Rule::section => {
                current_section_name = line.into_inner().next().unwrap().as_str();
            },
            Rule::property => {
                let mut inner = line.into_inner();
                let p = inner.next().unwrap().as_str();
                let v = inner.next().unwrap().as_str();

                let section = properties.entry(current_section_name).or_insert(Default::default());
                section.insert(p, v);
            },
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    println!("{:#?}", properties);
}