use crate::model::*;

pub struct Render<I: Iterator<Item = TruncatedWave>, H: Fn(TruncatedWave) -> I> {
    pub dst: Point,
    pub cutoff_distance: f64,
    pub hit: H,
}

impl<I: Iterator<Item = TruncatedWave>, H: Fn(TruncatedWave) -> I> Render<I, H> {
    pub fn render(&self, src: TruncatedWave, on_hit: &mut impl FnMut(&TruncatedWave)) {
        let distance = src.distance_with(self.dst);
        if distance > self.cutoff_distance {
            return;
        }

        if src.hit_with_point(self.dst) {
            on_hit(&src);
        }

        for hit in (self.hit)(src.clone()) {
            self.render(hit.clone(), on_hit);
        }
    }

    pub fn render2(&self, src: TruncatedWave, on_hit: &mut impl FnMut(&TruncatedWave)) {
        let mut stack = vec![src];

        while let Some(src) = stack.pop() {
            let distance = src.distance_with(self.dst);
            if distance > self.cutoff_distance {
                continue;
            }

            if src.hit_with_point(self.dst) {
                on_hit(&src);
            }

            stack.extend((self.hit)(src.clone()));
        }
    }
}
