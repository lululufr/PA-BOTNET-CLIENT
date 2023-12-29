use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("51.77.193.65:8080")?;

    println!("Connected to server!");

    let mut buffer = [0; 512];

    loop {
        let message = get_user_input();
        stream.write_all(message.as_bytes())?;

        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            // La connexion a été fermée
            break;
        }

        let response = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received from Python: {}", response);

        if response.to_lowercase().contains("quitter") {
            break;
        }
    }

    Ok(())
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}