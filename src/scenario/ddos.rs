use crate::function_utils::subprocess_run;

use crate::function_utils;

pub(crate) fn ping(ip: String){
    let cmd= format!("ping {} -l 65500",ip);

    let sortie = subprocess_run(&*cmd);
}


