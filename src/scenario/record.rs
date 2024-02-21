use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound;
use std::sync::{Arc, Mutex};

pub(crate) fn record(){
    let host = cpal::default_host();
    let device = host.default_input_device().expect("no input device available");
    let mut supported_configs_range = device.supported_input_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range.next()
        .expect("no supported config?!")
        .with_max_sample_rate();
    let sample_format = supported_config.sample_format();
    let config = supported_config.config();

    // Créer un fichier WAV pour l'enregistrement
    let spec = hound::WavSpec {
        channels: config.channels as u16,
        sample_rate: config.sample_rate.0,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let samples: Arc<Mutex<Vec<i16>>> = Arc::new(Mutex::new(Vec::new()));

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
    let stream = match sample_format {
        cpal::SampleFormat::F32 => {
            let samples = Arc::clone(&samples);
            device.build_input_stream(
                &config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
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
        cpal::SampleFormat::I16 => {
            let samples = Arc::clone(&samples);
            device.build_input_stream(
                &config,
                move |data: &[i16], _: &cpal::InputCallbackInfo| {
                    let mut samples = samples.lock().unwrap();
                    samples.extend_from_slice(data);
                },
                err_fn,
                None
            )
        },
        cpal::SampleFormat::U16 => {
            let samples = Arc::clone(&samples);
            device.build_input_stream(
                &config,
                move |data: &[u16], _: &cpal::InputCallbackInfo| {
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
    stream.play().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(10));
    drop(stream);
    // Écrire les échantillons dans le fichier WAV
    let samples = samples.lock().unwrap();
    let mut writer = hound::WavWriter::create("enregistrement.wav", spec).unwrap();
    for sample in samples.iter() {
        writer.write_sample(*sample).unwrap();
    }
    writer.finalize().unwrap();
    println!("Enregistrement terminé")
}