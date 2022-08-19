use chrono::{DateTime, Local, TimeZone};
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
                .possible_values(["rfc2822", "rfc3339", "timestamp"])
                .default_value("timestamp"),
        )
        .arg(
            Arg::with_name("datetime")
               .help("When <action> is 'set' apply <datetime>, else ignore.")
        );

        let args = app.get_matches();

        let command = args.value_of("command").unwrap();
        let format = args.value_of("format").unwrap();

        if command == "set" {
            todo!()
        }

        let now = Clock::get();
        match format {
            "timestamp" => println!("{}", now.timestamp()),
            "rfc2822" => println!("{}", now.to_rfc2822()),
            "rfc3339" => println!("{}", now.to_rfc3339()),
            _ => unreachable!(),
        }
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