use crate::black_scholes_ndp::*;

#[test]
fn test_regression_wrt_python() {
    assert_abs_diff_eq!(
        euro_vanilla_call(50., 100., 1., 0.05, 0.25),
        0.027352509369436617,
        epsilon = 0.000000001
    );
    assert_abs_diff_eq!(
        euro_vanilla_put(50., 100., 1., 0.05, 0.25),
        45.15029495944084,
        epsilon = 0.000000001
    );
    assert_abs_diff_eq!(
        euro_vanilla(50., 100., 1., 0.05, 0.25, "put"),
        45.15029495944084,
        epsilon = 0.000000001
    );
}
