use statrs::distribution::Univariate;

use std::*;

pub fn euro_vanilla_call(S: f64, K: f64, T: f64, r: f64, sigma: f64) -> f64 {
    let d1 = f64::ln(S / K) + r + 0.5 * sigma.powi(2) * T / sigma * f64::sqrt(T);
    let d2 = f64::ln(S / K) + r - 0.5 * sigma.powi(2) * T / sigma * f64::sqrt(T);
    let call = S * statrs::distribution::Normal::new(0.0, 1.0).unwrap().cdf(d1)
        - K * f64::exp(-(r) * T) * statrs::distribution::Normal::new(0.0, 1.0).unwrap().cdf(d2);
    return call;
}
pub fn euro_vanilla_put(S: f64, K: f64, T: f64, r: f64, sigma: f64) -> f64 {
    let d1 = f64::ln(S / K) + r + 0.5 * sigma.powi(2) * T / sigma * f64::sqrt(T);
    let d2 = f64::ln(S / K) + r - 0.5 * sigma.powi(2) * T / sigma * f64::sqrt(T);
    let put = K
        * f64::exp(-(r) * T)
        * statrs::distribution::Normal::new(0.0, 1.0)
            .unwrap()
            .cdf(-(d2))
        - S * statrs::distribution::Normal::new(0.0, 1.0)
            .unwrap()
            .cdf(-(d1));
    return put;
}
pub fn euro_vanilla(S: f64, K: f64, T: f64, r: f64, sigma: f64, option: &str) -> f64 {
    let d1 = f64::ln(S / K) + r + 0.5 * sigma.powi(2) * T / sigma * f64::sqrt(T);
    let d2 = f64::ln(S / K) + r - 0.5 * sigma.powi(2) * T / sigma * f64::sqrt(T);
    return match option {
        "call" => {
            S * statrs::distribution::Normal::new(0.0, 1.0).unwrap().cdf(d1)
                - K * f64::exp(-(r) * T)
                    * statrs::distribution::Normal::new(0.0, 1.0).unwrap().cdf(d2)
        }
        "put" => {
            K * f64::exp(-(r) * T)
                * statrs::distribution::Normal::new(0.0, 1.0)
                    .unwrap()
                    .cdf(-(d2))
                - S * statrs::distribution::Normal::new(0.0, 1.0)
                    .unwrap()
                    .cdf(-(d1))
        }
        _ => panic!(),
    };
}
