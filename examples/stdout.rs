// Usage:
// cargo run --release --example stdout 2> /dev/null | pacat --raw --format=s16ne --channels=1
//
// cargo run --release --example stdout 2> /dev/null > audio.raw
// sox -t raw -e signed-integer -b 16 -r 44100 -c 1 audio.raw audio.wav

use std::{f64::consts::PI, io::Write};

use impulser::{
    model::{Point, TruncatedWave},
    normalize, remove_dc_offset,
    render::Render,
};

fn main() {
    let sample_rate = 44100;
    let speed = 340.0;
    let decay = 1.0;

    let mut points = vec![
        Point::new(1.0, -1.01),
        Point::new(1.0, 1.0),
        Point::new(-1.0, 1.0),
        Point::new(-1.0, -0.1),
        Point::new(0.5, -1.0),
    ];
    points.push(points[0]);
    let room = impulser::room::Room::new(points);
    let src = Point::new(0.1, 0.0);
    let dst = Point::new(0.0, 0.5);

    let duration = 2;
    let ch = 1;
    let mut buffer = vec![0.0; duration * sample_rate * ch];

    // let start = std::time::Instant::now();
    Render {
        dst,
        cutoff_distance: duration as f64 * speed,
        hit: |src: TruncatedWave| room.hit(src),
    }
    .render2(src.into(), &mut |src| {
        let d = src.distance_with(dst);
        let p = d * sample_rate as f64 / speed;
        let i = p.floor() as usize;
        let power = d.powf(-decay);
        if ch == 1 {
            if i < duration * sample_rate - 1 {
                buffer[i] += power * (1.0 - p.fract());
                buffer[i + 1] += power * p.fract();
            }
        } else if ch == 2 {
            let a = (src.point - dst).atan2();
            let r = (a.cos() + 1.0) / 2.0;
            let l = ((a + PI).cos() + 1.0) / 2.0;
            if i < duration * sample_rate - 1 {
                buffer[i * 2] += power * l * (1.0 - p.fract());
                buffer[i * 2 + 1] += power * r * (1.0 - p.fract());
                buffer[i * 2 + 2] += power * l * p.fract();
                buffer[i * 2 + 3] += power * r * p.fract();
            }
        }
    });
    // dbg!(start.elapsed());

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
