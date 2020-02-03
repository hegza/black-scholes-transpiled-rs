## 3.2.-20
### Windows, Etana
1. Copied relevant sources from transpile project.
2. Fix `pyrs` transpilation failures.
    * Pyrs called library functions with integers instead of floats, relying on Python's automatic cast.
    * Python's `float` is 64-bit, converted to `f64` instead of `f32` offered by pyrs.
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
        * This would fail for broadcasting {X}. Works for scalar only.
        * {loc} = {mean}, {scale} = {stddev} (verified from docs)
    * Add required dependency: `use statrs::distribution::Univariate;`.

#### Meta
1. Code compiles, but results match is not verified. Performance is not measured.
2. Manual part of transpilation took two hours with meticulous note taking and focus on automatizability.
3. Applied `cargo fix` & `cargo +nightly fix -Z unstable-options --clippy`.