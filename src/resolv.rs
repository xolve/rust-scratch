use std::net::SocketAddr;
use std::net::UdpSocket;
use std::time::Duration;

use clap::Command;
use clap::Arg;

use trust_dns_client::op::Message;
use trust_dns_client::op::MessageType;
use trust_dns_client::op::OpCode;
use trust_dns_client::op::Query;
use trust_dns_client::rr::RecordType;
use trust_dns_client::rr::domain::Name;
use trust_dns_client::serialize::binary::BinEncoder;
use trust_dns_proto::serialize::binary::BinEncodable;

fn main() {
    let app = Command::new("resolv")
        .about("A sample DNS program")
        .arg(Arg::with_name("dns-server").short('s').default_value("8.8.8.8"))
        .arg(Arg::with_name("domain-name").short('d').takes_value(true).required(true))
        .get_matches();

    let domain_name = app.value_of("domain-name").unwrap();
    let domain_name = Name::from_ascii(domain_name).unwrap();

    let dns_server = app.value_of("dns-server").unwrap();
    let dns_server: SocketAddr = format!("{}:53", dns_server).parse().expect("invalid address");

    let mut request: Vec<u8> = Vec::with_capacity(512);
    
    let mut msg = Message::new();
    msg.set_id(rand::random::<u16>())
        .set_message_type(MessageType::Query)
        .add_query(Query::query(domain_name, RecordType::A))
        .set_op_code(OpCode::Query)
        .set_recursion_desired(true);

    let mut encoder = BinEncoder::new(&mut request);
    msg.emit(&mut encoder).unwrap();

    let localhost = UdpSocket::bind("0.0.0.0:0")
        .expect("Cannot bind to local socket");

    let timeout = Duration::from_secs(3);
    localhost.set_read_timeout(Some(timeout)).unwrap();
    localhost.set_nonblocking(false).unwrap();

    let _amt = localhost.send_to(&request, dns_server)
        .expect("error sending DNS request");

    let mut response: Vec<u8> = vec![0; 512];
    let (_amt, _remote) = localhost.recv_from(&mut response).expect("timeout reached");
    let dns_message = Message::from_vec(&response).expect("unable to parse response");

    for answer in dns_message.answers() {
        println!("{:?}", answer);
    }
}