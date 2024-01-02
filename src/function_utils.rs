use std::borrow::Cow;
use std::process::Command;

pub(crate) fn subprocess_run(cmd: &str) -> String {

    let output = Command::new("powershell")
    .args(&[cmd])
    .output()
    .expect("Erreur lors de l'exécution de la commande.");

    //return
    String::from_utf8_lossy(&*output.stdout).to_string()
}
