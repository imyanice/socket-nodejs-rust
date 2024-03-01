import zmq from "zeromq";
// Creation of the publisher socket
let sock = zmq.socket("push");
// Binding the socket to the port 3000 on localhost
sock.bindSync("tcp://127.0.0.1:3000");
console.log("Publisher has established connection to 'tcp://127.0.0.1:3000'.");

sock.send("connected");

// sock.send("Hello, World!");

sock.send("exit");

sock.on("message", (msg) => {
  console.log("Message received: " + msg.toString());
});
