use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::StreamConfig;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::fs::File;
use std::io::Write;

pub(crate) fn record() {
    // Choisissez l'hôte par défaut
    let host = cpal::default_host();

    // Sélectionnez le périphérique d'entrée (microphone) par défaut
    let device = host.default_input_device().expect("Aucun périphérique d'entrée trouvé");

    // Obtenez la configuration par défaut du périphérique
    let config = device.default_input_config().expect("Erreur lors de l'obtention de la configuration");

    // Créez un fichier pour enregistrer les données audio
    let file = Arc::new(Mutex::new(File::create("C:\\Users\\jiull\\Downloads\\enregistrement_audio.wav")
        .expect("Impossible de créer le fichier")));

    // Clonez le fichier pour le partager avec le callback
    let file_clone = file.clone();

    // Créez un flux avec la configuration souhaitée et spécifiez la latence
    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            // Ici, vous pouvez traiter ou enregistrer les données audio

            // Verrouillez le fichier pour éviter les conflits d'accès concurrents
            let mut file = file_clone.lock().unwrap();

            // Écrivez les données audio dans le fichier
            for &sample in data {
                let sample_i16 = (sample * i16::max_value() as f32) as i16;
                file.write_all(&sample_i16.to_le_bytes())
                    .expect("Erreur lors de l'écriture des données dans le fichier");
            }
        },
        move |err| {
            eprintln!("Erreur lors de la capture audio: {:?}", err);
        },
        Some(Duration::from_secs(10)), // Spécifiez la latence (10 secondes dans cet exemple)
    ).expect("Erreur lors de la création du flux");

    // Démarrez le flux
    stream.play().expect("Erreur lors du démarrage du flux");

    // Attendez 10 secondes
    std::thread::sleep(Duration::from_secs(10));

    // Arrêtez le flux
    stream.pause().expect("Erreur lors de l'arrêt du flux");
    println!("Enregistrement terminé");
}
