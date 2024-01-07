
struct atkOver {
    attaque:bool,
    discret: bool,
}

struct ScanEnvoiJson {
    cible: String,
    discret: bool,
    ipup: vec<String>,
}
struct ScanReceptionJson {
    cible: String,
    discret: bool,
}

struct DdosEnvoiJson {
    attack: String,
    id: Int,
    arguments:{
        ip: String,
        time: u32,
        advanced: bool,
    }
}