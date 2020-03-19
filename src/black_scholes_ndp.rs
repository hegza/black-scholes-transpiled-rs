use statrs::distribution::{Normal, Univariate};

use std::*;

pub fn euro_vanilla_call(S: f64, K: f64, T: f64, r: f64, sigma: f64) -> f64 {
    let d1 = (f64::ln(S / K) + (r + 0.5 * sigma.powi(2)) * T) / (sigma * f64::sqrt(T));
    let d2 = (f64::ln(S / K) + (r - 0.5 * sigma.powi(2)) * T) / (sigma * f64::sqrt(T));
    let call = S * Normal::new(0.0, 1.0).unwrap().cdf(d1)
        - K * f64::exp(-(r) * T) * Normal::new(0.0, 1.0).unwrap().cdf(d2);
    call
}
pub fn euro_vanilla_put(S: f64, K: f64, T: f64, r: f64, sigma: f64) -> f64 {
    let d1 = (f64::ln(S / K) + (r + 0.5 * sigma.powi(2)) * T) / (sigma * f64::sqrt(T));
    let d2 = (f64::ln(S / K) + (r - 0.5 * sigma.powi(2)) * T) / (sigma * f64::sqrt(T));
    let put = K * f64::exp(-(r) * T) * Normal::new(0.0, 1.0).unwrap().cdf(-(d2))
        - S * Normal::new(0.0, 1.0).unwrap().cdf(-(d1));
    put
}
pub fn euro_vanilla(S: f64, K: f64, T: f64, r: f64, sigma: f64, option: &str) -> f64 {
    let d1 = (f64::ln(S / K) + (r + 0.5 * sigma.powi(2)) * T) / (sigma * f64::sqrt(T));
    let d2 = (f64::ln(S / K) + (r - 0.5 * sigma.powi(2)) * T) / (sigma * f64::sqrt(T));
    match option {
        "call" => {
            S * Normal::new(0.0, 1.0).unwrap().cdf(d1)
                - K * f64::exp(-(r) * T) * Normal::new(0.0, 1.0).unwrap().cdf(d2)
        }
        "put" => {
            K * f64::exp(-(r) * T) * Normal::new(0.0, 1.0).unwrap().cdf(-(d2))
                - S * Normal::new(0.0, 1.0).unwrap().cdf(-(d1))
        }
        _ => panic!(),
    }
}
