/// Calculate the magnitude of the given vector.
fn magnitude(v: &[f64; 3]) -> f64 {
    (v[0].powi(2) + v[1].powi(2) + v[2].powi(2)).sqrt()
}

/// Change the magnitude of the vector to 1.0 without changing its direction.
fn normalize(v: &mut [f64; 3]) {
    let magnitude = magnitude(v);

    v.iter_mut().for_each(|x| *x /= magnitude);
}

fn main() {
    println!(
        "Magnitude of a unit vector: {}",
        magnitude(&[0.0, 1.0, 0.0])
    );

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}
