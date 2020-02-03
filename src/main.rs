use std::collections::HashMap;
use std::*;

mod black_scholes_dp;
mod black_scholes_ndp;

use black_scholes_dp::*;
use black_scholes_ndp::*;
fn main() {
    euro_vanilla_call(50, 100, 1, 0.05, 0.25);
    euro_vanilla_put(50, 100, 1, 0.05, 0.25);
    euro_vanilla(50, 100, 1, 0.05, 0.25, "put");
    black_scholes_call_div(50, 100, 1, 0.05, 0.06, 0.25);
    black_scholes_put_div(50, 100, 1, 0.05, 0.06, 0.25);
    euro_vanilla_dividend(50, 100, 1, 0.05, 0.06, 0.25, "put");
}
