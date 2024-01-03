use function_utils::subprocess_run;
use ipnetwork::IpNetwork;

extern crate regex;
use regex::Regex;
use crate::function_utils;



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

pub(crate) fn scan(ips: Vec<Vec<String>>) {

    //let ips = vec!["127.0.0.1", "127.0.0.2", "8.8.8.8","8.8.8.7"];

    for ip in ips.iter() {
        for i in ip.iter() {
            if up_or_not(i) {
                println!("{} is UP", i)
            }else{
                //println!("{} is DOWN", i)
            }
        }
    }
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