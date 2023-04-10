use std::f64::consts::PI;
use std::f64::consts::E;
use num_complex::Complex;
mod vectorTools;

struct Signal {
    pub id : String,
    pub series : Vec<f64>,
    pub length : f64,
}

impl Signal {
    pub fn len(&self) -> usize {
        self.series.len()
    }
    pub fn fourier_transform(&self) -> Vec<Complex<F>> {
        let mut transformed_vector = Vec::new();
        for i in 0..self.len() {
            let mut push:Complex<F> = 0.;
            for j in 0..self.len(){
                let push = push + self[i].num_complex::Complex::exp()
            }
            transformed_vector.push(push);
        }
        transformed_vector

    }
}