#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

#[derive(Debug)]
struct Message {
    to: u64,
    conent: String,
}

#[derive(Debug)]
struct MailBox {
    messages: Vec<Message>,
}

#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: MailBox,
}

impl CubeSat {
    fn new(id: u64) -> Self {
        CubeSat { id, mailbox: MailBox { messages: vec![] } }
    }

    fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}

struct GroundStation;

impl GroundStation {
    fn send(&self, to: &mut CubeSat, message: Message) {
        to.mailbox.messages.push(message);
    }

    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat::new(sat_id)
    }
}

fn check_status(sat_id: &CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![0, 1, 2,]
}

fn main () {
    let base = GroundStation {};

    let sat_ids = fetch_sat_ids();
}