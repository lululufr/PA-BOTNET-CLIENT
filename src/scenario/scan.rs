use std::sync::mpsc::Receiver;
use function_utils::subprocess_run;
use ipnetwork::IpNetwork;

extern crate regex;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use crate::function_utils;

// a copier coller pour le json
#[derive(Serialize, Deserialize)]
struct DataReceived {
    id: u32,
    attack: String,
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

    data.id = received.id;
    data.attack = received.attack;
    data.ipup = ips;

    let json_string = serde_json::to_string(&data);

    println!("Serialized JSON: {:?}", json_string)
}

// a copier coller pour le json


fn generate_ips(base_ip: &str, subnet_mask: &str) -> Vec<String> {
    let network_str = format!("{}/{}", base_ip, subnet_mask);
    let network = network_str.parse::<IpNetwork>().expect("Invalid network");

    let mut ips = Vec::new();

    for ip in network.iter() {
        ips.push(ip.to_string());
    }
    ips
}


pub(crate) fn find_net() -> Vec<Vec<String>> {
    let tofind = subprocess_run("ipconfig");

    let re = Regex::new(r"(\b25[0-4]|\b2[0-4][0-9]|\b[01]?[0-9][0-9]?)(\.(25[0-5]|2[0-4][0-9]|[01]?[1-9][1-9]?)){3}").unwrap(); //faire en sorte que on ai la bonne interface et pas un doublons de merde avec la gateway
    let mut ips = vec! [];

            for interfaces in re.find_iter(&*tofind) {
                ips.push(generate_ips(interfaces.as_str(),"24"));
                //println!("{:?}", interfaces.as_str());
            }
            //println!("{:?}", ips)
    ips
}

pub(crate) fn scan(ips: Vec<Vec<String>>,ordre:String){

    let data_json = receive_data_json_to_str(ordre);
    let mut ips_up:Vec<String> = vec![];

    println!("ID du lancement = {}",data_json.id);

    for ip in ips.iter() {
        for i in ip.iter() {
            if up_or_not(i) {
                println!("{} is UP", i);
                ips_up.push(i.to_string());
            }else{
                //println!("{} is DOWN", i)
            }
        }
    }
    println!("fin du ID = {}",data_json.id);
    send_json_data(ips_up, data_json)
}


fn up_or_not(ip: &str) -> bool {
    let cmd= format!("ping {} -n 1 -w 1",ip);

    let sortie = subprocess_run(&*cmd);

    if sortie.contains("TTL"){
        true
    }else{
        false
    }

}