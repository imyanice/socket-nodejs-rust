use zmq::*;

fn main() {
    println!("The subscriber has been started, waiting for connection...");
    // Initializes a socket on the subscriber side
    let socket = zmq::Context::new().socket(SocketType::PULL).unwrap();
    // Connect the socket to localhost:3000
    socket
        .connect("tcp://localhost:3000")
        .expect("Port is already in use.");
    loop {
        // Parse the message as a string
        let msg = socket.recv_string(0).unwrap();
        match msg {
            // If the message is a string, print it/analyze it
            Ok(string) => {
                if string == "exit" {
                    println!("Publisher is shutting down...");
                } else if string == "connected" {
                    println!("Publisher is connected.");
                } else {
                    println!("NEW MESSAGE: ");
                    println!("{}", string)

                    // Or do anything else with the message
                    // Use serde_json to parse the message as a json object
                    // To then call a function to handle the parsed data
                }
            }
            // If the message is not a string, print the bytes
            Err(vec) => {
                println!("NEW BYTES: ");
                println!("{:?}", vec)
            }
        }
    }
}
