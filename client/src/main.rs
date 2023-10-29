// Libraries used in the Client Program
use std::fs;
use std::io::{Read, stdin, Write};
use std::net::TcpStream;
use std::fs::File;

fn main() -> std::io::Result<()> {
    // Displaying the files available to send in the directory of the client
    println!("Files on the server: \n");
let mut files_on_server: Vec<String> = Vec::new();
    // reading files from the current directory
    if let Ok(entries) = fs::read_dir(".") {
        // For each entry in the directory, the code checks if it's a file.
        // If it is, the file's name is extracted from the path and added to the files_on_server vector.
        // The file names are also displayed on the console.
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    let file_name_tmp = path.display().to_string().trim_start_matches(".\\").to_string();
                    files_on_server.push(file_name_tmp.clone());
                     println!("{}", file_name_tmp);
                }
            }
        }
    } else {
        println!("Error reading the current directory.");
    }

    // Now taking an input from the client to select which file to send to the Server
    println!("Enter the file name to send: ");
    let mut file_name = String::new();
    stdin().read_line(&mut file_name).expect("Failed to read user input");


    let mut file_exist: bool = false;
    // At the same time checking if the entered file is available in the directory of the client
    for file in files_on_server.iter()
    {
        if file_name.trim().eq(file)
        {
            file_exist = true;
        }
    };

    // If the file exist then attempting to send the file
    if file_exist
    {
        // Opening the file first
        let mut file = File::open(file_name.trim())?;
        // Connect to the peer connection of server
        let mut stream = TcpStream::connect("127.0.0.1:8080")?;
        // displaying the size of the file we just sent
        let bytes_copied = std::io::copy(&mut file, &mut stream)?;
        println!("Sent {} bytes", bytes_copied);
    }
    else {
        println!("Invalid file name.")
    }


    Ok(())
}
