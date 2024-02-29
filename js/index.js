var zmq = require("zeromq"),
    sock = zmq.socket("pull");

sock.bind("tcp://*:3000");
console.log("Worker connected to port 3000");

sock.on("message", function(msg) {
    console.log("work: %s", msg.toString());
});
