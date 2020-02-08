#![allow(non_snake_case)]

#[cfg(test)]
#[macro_use]
extern crate approx;

mod black_scholes_dp;
mod black_scholes_ndp;

#[cfg(test)]
mod test_regression;

pub use black_scholes_dp::black_scholes_call_div;
pub use black_scholes_dp::black_scholes_put_div;
pub use black_scholes_ndp::euro_vanilla_call;
pub use black_scholes_ndp::euro_vanilla_put;
