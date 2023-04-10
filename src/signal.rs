struct Signal {
    id : String,
    series : vec<f64>,
    length : f64,
}

impl Signal {
    pub fn len(&self) -> usize {
        self.series.len()
    }
}