use dsp::transforms::{clarke, inverse_clarke};
use easybench::bench;

fn clarke_bench() {
    println!("clarke {}", bench(|| clarke((1.0, 2.0, -3.0))));
}

fn inverse_clarke_bench() {
    println!(
        "inverse clarke {}",
        bench(|| inverse_clarke((1.0, 2.0, -3.0)))
    );
}

fn main() {
    clarke_bench();
    inverse_clarke_bench();
}
