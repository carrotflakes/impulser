// Usage:
// cargo run --release --example stdout 2> /dev/null | pacat --raw --format=s16ne --channels=1
//
// cargo run --release --example stdout 2> /dev/null > audio.raw
// sox -t raw -e signed-integer -b 16 -r 44100 -c 1 audio.raw audio.wav

use std::io::Write;

use impulser::{
    model::{Point, TruncatedWave},
    normalize, remove_dc_offset,
    render::Render,
};

fn main() {
    let sample_rate = 44100;
    let speed = 34.0;
    let decay = 1.0;

    let room = impulser::room::Room::new(vec![
        Point::new(1.0, -1.01),
        Point::new(1.0, 1.0),
        Point::new(-1.0, 1.0),
        Point::new(-1.0, -0.1),
    ]);
    // let room = impulser::room::Room::new(vec![
    //     Point::new(1.0, -1.2),
    //     Point::new(1.0, 1.0),
    //     Point::new(-10.0, 1.1),
    // ]);
    let src = Point::new(0.1, 0.0);
    let dst = Point::new(0.0, 0.5);

    let duration = 2;
    let mut buffer = vec![0.0; duration * sample_rate];

    Render {
        dst,
        cutoff_distance: duration as f64 * speed,
        hit: |src: TruncatedWave| room.hit(src),
    }
    .render(src.into(), &mut |src| {
        let d = src.distance_with(dst);
        let i = (d * sample_rate as f64 / speed) as usize;
        if i < buffer.len() {
            buffer[i] += d.powf(-decay);
        }
    });

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
