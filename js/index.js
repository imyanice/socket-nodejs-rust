import zmq from "zeromq";

async function run() {
  const sock = new zmq.Request();

  sock.connect("tcp://127.0.0.1:3000");
  console.log("Producer bound to port 3000");
  // Send data to the Rust server

  await sock.send("AHHHHH");

  const [result] = await sock.receive();

  console.log(result.toString());
}

run();
