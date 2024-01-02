use std::io::{Read, Write};
use std::net::TcpStream;
use crate::get_user_input;


pub(crate) fn co() -> std::io::Result<TcpStream> {
    let stream =  TcpStream::connect("51.77.193.65:4242");
    println!("Connexion ETABLI");
    return stream;
}

pub(crate) fn emission(mut stream: TcpStream){
    loop {
        let message = get_user_input();
        stream.write_all(message.as_bytes()).expect("Erreur lors de l'envoi du message");
    }
}

pub(crate) fn reception(mut stream: TcpStream) {
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
                eprintln!("Erreur lors de la lecture: {}", err);
                break;
            }
        }
    }
}
