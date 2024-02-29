
use zmq::*;

fn main() {
    println!("Start server");
    let mut socket = zmq::Context::new().socket(SocketType::PUSH).unwrap();
    socket.connect("tcp://127.0.0.1:3000").expect("TODO: panic message");
    let mut msg = Message::new();
    socket.send("dded", 0)
        .expect("TODO: panic message");
}