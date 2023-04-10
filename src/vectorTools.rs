pub fn vector_sum(vector : vec<f644>) -> f64 {
    let mut sum = 0.;
    for i in 0..vector.len() {
        sum = sum + vector[i];
    }
    sum
}