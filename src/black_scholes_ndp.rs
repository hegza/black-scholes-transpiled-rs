use std::collections::HashMap;
use std::*;

use numpy::float64;
fn euro_vanilla_call(S: i32, K: i32, T: i32, r: f32, sigma: f32) -> float64 {
    let d1 = np.log(S / K) + r + 0.5 * sigma.pow(2) * T / sigma * np.sqrt(T);
    let d2 = np.log(S / K) + r - 0.5 * sigma.pow(2) * T / sigma * np.sqrt(T);
    let call = S * si.norm.cdf(d1, 0.0, 1.0) - K * np.exp(-(r) * T) * si.norm.cdf(d2, 0.0, 1.0);
    return call;
}
fn euro_vanilla_put(S: i32, K: i32, T: i32, r: f32, sigma: f32) -> float64 {
    let d1 = np.log(S / K) + r + 0.5 * sigma.pow(2) * T / sigma * np.sqrt(T);
    let d2 = np.log(S / K) + r - 0.5 * sigma.pow(2) * T / sigma * np.sqrt(T);
    let put =
        K * np.exp(-(r) * T) * si.norm.cdf(-(d2), 0.0, 1.0) - S * si.norm.cdf(-(d1), 0.0, 1.0);
    return put;
}
fn euro_vanilla(S: i32, K: i32, T: i32, r: f32, sigma: f32, option: &str) -> float64 {
    let d1 = np.log(S / K) + r + 0.5 * sigma.pow(2) * T / sigma * np.sqrt(T);
    let d2 = np.log(S / K) + r - 0.5 * sigma.pow(2) * T / sigma * np.sqrt(T);
    if option == "call" {
        let mut result =
            S * si.norm.cdf(d1, 0.0, 1.0) - K * np.exp(-(r) * T) * si.norm.cdf(d2, 0.0, 1.0);
    }
    if option == "put" {
        let mut result =
            K * np.exp(-(r) * T) * si.norm.cdf(-(d2), 0.0, 1.0) - S * si.norm.cdf(-(d1), 0.0, 1.0);
    }
    return result;
}
