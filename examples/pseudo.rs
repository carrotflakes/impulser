use std::io::Write;

use impulser::*;
use rand::Rng;

fn main() {
    let sample_rate = 44100;

    let mut rng = rand::thread_rng();

    let duration = 5;
    let mut buffer = vec![0.0; duration * sample_rate];

    for _ in 0..10000 {
        let d = rng.gen_range(0.001f64..1.0).powf(0.7) * duration as f64;
        let p = d * sample_rate as f64;
        let i = p.floor() as usize;
        let power = d.powf(-1.0);
        if i < buffer.len() - 1 {
            buffer[i] += power * (1.0 - p.fract());
            buffer[i + 1] += power * p.fract();
        }
    }

    remove_dc_offset(&mut buffer);
    normalize(&mut buffer);

    let buf: Vec<_> = buffer
        .iter()
        .flat_map(|x| ((x * i16::MAX as f64).round() as i16).to_ne_bytes())
        .collect();

    let mut stdout = std::io::stdout().lock();
    stdout.write_all(&buf).unwrap();
    stdout.flush().unwrap();
}
