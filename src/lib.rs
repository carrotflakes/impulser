pub mod model;
pub mod render;
pub mod room;

pub fn remove_dc_offset(buffer: &mut [f64]) {
    let dc_offset = buffer.iter().sum::<f64>() / buffer.len() as f64;
    buffer.iter_mut().for_each(|x| *x = *x - dc_offset);
}

pub fn normalize(buffer: &mut [f64]) {
    let max = buffer.iter().map(|x| x.abs()).reduce(f64::max).unwrap();
    buffer.iter_mut().for_each(|x| *x /= max);
}
