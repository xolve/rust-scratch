use chrono::{DateTime, Local};
use clap::{Arg, Command};

fn main() {
    let app = Command::new("clock")
        .arg(
            Arg::with_name("command")
                .takes_value(true)
                .possible_values(["get", "set"])
                .default_value("get"),
        )
        .arg(
            Arg::with_name("format")
                .long("use-standard")
                .takes_value(true)
                .default_value("timestamp"),
        );
}

struct Clock;

impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }

    fn set() -> ! {
        todo!()
    }
}
