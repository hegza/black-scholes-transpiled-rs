## 3.2.-20
### Windows, Etana
1. Copied relevant sources from transpile project.
2. Fix `pyrs` transpilation failures.
    * Pyrs called library functions with integers instead of floats, relying on Python's automatic cast.
    * Python's `float` is 64-bit, converted to `f64` instead of `f32` offered by pyrs.
    * Pyrs failed to trace parentheses from Python side to Rust. Went through all calculations and fixed the problem.
3. Add `mod` where relevant.
    * `pyrs` could not recognize local imports from library imports.
4. Make BS-functions public.
    * `pyrs` failed to track inter-module visibility.
5. Fix lifetime issue, where Python allows values to remain usable after scope ends with if-block.
    * Manually wrote a match statement, with panic on failure.
    * Uncovered UB: Python returns "None" if called with invalid arguments.

#### Map libraries to Rust
1. Remove `use numpy::float64`. Replace symbol `float64` with `f64`.
    * Because ndarray uses native Rust types as scalar output.
2. Recognize "np" and "si" as library symbols. Note difference between py-mods and rs-mods.
3. Map out conversions from `np`, case-by-case.
    * `np.log` -> `f32::ln`
    * `np.pow` -> `np.powi`
        * Only works for integer powers.
    * `np{.log,.sqrt,.exp,.pow}` -> `f32::{$1}`
4. Map out conversions from `si`, case-by-case.
    * `si.norm.cdf({x}, {loc}, {scale})` -> `statrs::distribution::Normal::new({loc}, {scale}).cdf({$x})`.
        * Regex: `si.norm.cdf\(((?:\w|-|\(|\))+),(?: *?)(\d+\.?\d*),(?: *?)(\d+\.?\d*)\)` -> `statrs::distribution::Normal::new($2, $3).unwrap().cdf($1)`
            * Capture identifier parameter with potential minus `((?:\w|-|\(|\))+)`
            * Capture decimal-literal parameter `(\d+\.?\d*)`
            * Ignore potential whitespace `(?: *?)`
        * This would fail for broadcasting {X}. Works for scalar only.
        * {loc} = {mean}, {scale} = {stddev} (verified from docs)
    * Add required dependency: `use statrs::distribution::Univariate;`.

#### Meta
1. Code compiles, but results match is not verified. Performance is not measured.
2. Manual part of transpilation took two hours with meticulous note taking and focus on automatizability.
3. Applied `cargo fix` & `cargo +nightly fix -Z unstable-options --clippy`.
4. Regression testing assertions failed. Failure attributed to incorrect parentheses error introduced by pyrs.

### Arch Linux, pylon
1. Ran `cargo run --release` through `time`. Contents: run each BS-function once. Result `0.02 user, 0.037 total`.
    * That's a 10× speedup over the interpreted plain Python version.

## 8.2.-20
### Etana
1. Wrote benchmarks naively, just call functions with fixed parameters. Quick results, no machine setup.
    * call no-div             time:   [71.751 ns 71.783 ns 71.814 ns]
    * put no-div              time:   [73.517 ns 73.574 ns 73.644 ns]
    * call div                time:   [74.824 ns 74.888 ns 74.967 ns]
    * put div                 time:   [76.616 ns 76.639 ns 76.661 ns]
2. Black box inputs. Quick results, no machine setup. Seems to have gotten slightly slower.
    * call no-div             time:   [72.237 ns 72.299 ns 72.370 ns]
    * put no-div              time:   [74.875 ns 74.930 ns 74.985 ns]
    * call div                time:   [75.022 ns 75.056 ns 75.087 ns]
    * put div                 time:   [76.899 ns 76.974 ns 77.059 ns]
