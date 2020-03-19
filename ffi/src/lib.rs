#[no_mangle]
pub extern "C" fn euro_vanilla_put(S: f64, K: f64, T: f64, r: f64, sigma: f64) -> f64 {
    black_scholes::euro_vanilla_put(S, K, T, r, sigma)
}
