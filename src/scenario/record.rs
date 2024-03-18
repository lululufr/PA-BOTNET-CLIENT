#[cfg(feature = "libraries")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
#[cfg(feature = "libraries")]
use hound;
#[cfg(feature = "libraries")]
use std::sync::{Arc, Mutex};

#[cfg(feature = "libraries")]
fn record (){
    //Définition de l'hôte
    let host = cpal::default_host();
    //Selection de l'entrée audio
    let device = host.default_input_device().expect("Aucun périphérique d'entrée audio disponible");
    //Configuration d'entrée supportées
    let mut supported_configs_range = device.supported_input_configs()
        .expect("Erreur lors de la récupération des configurations d'entrée audio");
    //Recherche de la configuration supportée
    let supported_config = supported_configs_range.find(|config| {
        matches!(config.sample_format(), cpal::SampleFormat::F32 | cpal::SampleFormat::I16 | cpal::SampleFormat::U16)
    }).expect("no supported config?!")
        .with_max_sample_rate(); // Taux d'échantillonage maximal

    //Enregistrement du format d'échantillonage
    let sample_format = supported_config.sample_format();
    //Configuration supportée pour le flux
    let config = supported_config.config();

    //Spécifications du fichier .wav
    let spec = hound::WavSpec {
        channels: config.channels as u16,
        sample_rate: config.sample_rate.0,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    //Vecteur pour stocker les échantillons
    let samples: Arc<Mutex<Vec<i16>>> = Arc::new(Mutex::new(Vec::new()));

    let err_fn = |err| eprintln!("Une erreur est survenue: {}", err);
    //Flux d'entrée audio
    let stream = match sample_format {
        cpal::SampleFormat::F32 => { //Echantillonage de type flottant
            let samples = Arc::clone(&samples);
            device.build_input_stream(
                &config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| { // Fonction de rappel pour le traitement audio
                    let mut samples = samples.lock().unwrap();
                    for &sample in data {
                        let sample_int = (sample * i16::MAX as f32) as i16;
                        samples.push(sample_int);
                    }
                },
                err_fn,
                None
            )
        },
        cpal::SampleFormat::I16 => { //Echantillonage de type 16 bits
            let samples = Arc::clone(&samples);
            device.build_input_stream(
                &config,
                move |data: &[i16], _: &cpal::InputCallbackInfo| {  //Ajoute directement les echantillons au vecteur
                    let mut samples = samples.lock().unwrap();
                    samples.extend_from_slice(data);
                },
                err_fn,
                None
            )
        },
        cpal::SampleFormat::U16 => { // Echantillonage de type 16 bits non signé
            let samples = Arc::clone(&samples);
            device.build_input_stream(
                &config,
                move |data: &[u16], _: &cpal::InputCallbackInfo| { //Conversion de u16 en i16
                    let mut samples = samples.lock().unwrap();
                    for &sample in data {
                        let sample = sample as i16;
                        samples.push(sample);
                    }
                },
                err_fn,
                None
            )
        },
        _ => panic!("Format d'échantillonnage non pris en charge!"),
    }.unwrap();
    //Démarrage de l'enregistrement
    stream.play().unwrap();
    //Enregistrement pendant X secondes
    std::thread::sleep(std::time::Duration::from_secs(10));
    //Arrêt et libération du flux
    drop(stream);
    // Écrire les échantillons dans le fichier WAV
    let samples = samples.lock().unwrap();
    let mut writer = hound::WavWriter::create("enregistrement.wav", spec).unwrap();
    for sample in samples.iter() {
        writer.write_sample(*sample).unwrap();
    }
    //Finalisation du fichier :)
    writer.finalize().unwrap();
    println!("Enregistrement terminé")
}

#[cfg(not(feature = "libraries"))]
fn record (){
    println!("La fonctionnalité d'enregistrement audio n'est pas prise en charge sur ce système");
}
