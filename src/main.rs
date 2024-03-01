use zmq::*;

fn main() {
    println!("The subscriber has been started, waiting for connection...");
    // Initializes a socket on the subscriber side
    let context = Context::new();
    let socket = context.socket(REP).unwrap();

    socket.bind("tcp://127.0.0.1:3000").expect("msg");
    let mut msg = zmq::Message::new();
    loop {
        // println!("Waiting for a message...");
        // Parse the message as a string
        let reccv = socket.recv(&mut msg, 0);
        match reccv {
            Ok(_) => {
                let string = msg.as_str().unwrap();
                if string == "exit" {
                    println!("Publisher is shutting down..."); // Show a loading screen (eg)
                } else if string == "connected" {
                    println!("Publisher is connected."); // Hide a loading screen (eg)
                } else {
                    println!("NEW MESSAGE: ");
                    println!("{}", string);
                    socket.send("Message received", 0).unwrap();

                    // Or do anything else with the message
                    // Use serde_json to parse the message as a json object
                    // To then call a function to handle the parsed data
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
        // socket.send("uh nuh", 0).unwrap();
    }
}
