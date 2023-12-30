use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;


fn co() -> std::io::Result<TcpStream> {
    let stream =  TcpStream::connect("51.77.193.65:4242");
    println!("Connected to server!");
    return stream;
}
fn emission(mut stream: TcpStream){
    loop {
        let message = get_user_input();
        stream.write_all(message.as_bytes());
    }
}
fn reception(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    loop {
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    // La connexion a été fermée
                    break;
                }

                let response = String::from_utf8_lossy(&buffer[..bytes_read]);
                println!("Received from Python: {}", response);
            }
            Err(err) => {
                eprintln!("Error reading from socket: {}", err);
                break;
            }
        }
    }
}



fn main() -> io::Result<()> {
    let mut connexion:TcpStream = co()?;
    let mut connexion2:TcpStream = connexion.try_clone()?;


    let thread_emission = thread::spawn(move|| {
        emission(connexion);
    });

    let thread_reception = thread::spawn(move|| {
        reception(connexion2);
    });

    thread_emission.join().expect("Error joining emission thread");
    thread_reception.join().expect("Error joining reception thread");

    Ok(())

}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}