// Libraries used in the Server program
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs::File;
use std::io::copy;
use std::thread;


fn main() {
    // First to set a TCP Server by creating a TCPListener,we bind the server to the IP Address and Port
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");
    println!("Listening for incoming connections on port 8080...");

    // now we make a loop for the incoming connections
    for stream in listener.incoming() {
        // Here we are using enums for Error Handling of the stream that if it fails it will print an error
        match stream {
            Ok(stream) => {
                // If the Stream is received we will print from where the connection is made
                println!("Accepted connection from: {}", stream.peer_addr().unwrap());
                // we are cloning the stream in a variable so we can use it later to pass it in the function
                let mut stream_clone = stream.try_clone().expect("Clone failed");
                // spawning a thread for the incoming connection and passing a parameter to the function
                thread::spawn(move || {
                    handle_client(stream_clone);
                });
            }
            // printing the Error
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    // Handling incoming file transfer

    // creating a new file with a certain name to save it in our local directory of the server machine
    let mut file = File::create("received_file.txt").expect("File creation failed");
    // displaying the size of the file we just downloaded which the client send to the server
    // copy function takes two parameters one with which its gonna read and the second in which its gonna write
    let bytes_copied = copy(&mut stream, &mut file).expect("File copy failed");
    println!("Received {} bytes", bytes_copied);
}
