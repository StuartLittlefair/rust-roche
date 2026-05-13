use pyo3::prelude::*;
use crate::constants::{C, H, K};
///
/// Computes the Planck function Bnu = (2 h \nu^3/c^2)/(exp(h \nu/kT) - 1)
///  as a function of wavelength and temperature. Output units are W/m**2/Hz/sr.
///
/// Arguments:
///
/// * `wave`: wavelength in nanometres
/// * `temp`: temperature in K
///
#[pyfunction]
pub fn planck(wave: f64, temp: f64) -> f64 {
    let fac1: f64 = 2.0e27 * H * C;
    let fac2: f64 = 1.0e9 * H * C / K;

    let exponent: f64 = fac2 / (wave * temp);
    if exponent > 40.0 {
        fac1 * ((-exponent).exp()) / (wave * wave * wave)
    } else {
        fac1 / exponent.exp_m1() / (wave * wave * wave)
    }
}

///
/// Computes the logarithmic derivative of the Planck function Bnu wrt
/// wavelength (i.e. d ln(Bnu) / d ln(lambda)) as a function of wavelength and temperature
///
/// Arguments:
///
/// * `wave`: wavelength in nanometres
/// * `temp`: temperature in K
///
#[pyfunction]
pub fn dplanck(wave: f64, temp: f64) -> f64 {
    let fac2: f64 = 1.0e9 * H * C / K;

    let exponent: f64 = fac2 / (wave * temp);
    exponent / (1.0 - -exponent.exp()) - 3.0
}

///
/// Computes the logarithmic derivative of the Planck function Bnu wrt
/// T (i.e. d ln(Bnu) / d ln(T)) as a function of wavelength and temperature
///
/// Arguments:
///
/// * `wave`: wavelength in nanometres
/// * `temp`: temperature in K
#[pyfunction]
pub fn dlpdlt(wave: f64, temp: f64) -> f64 {
    let fac2: f64 = 1.0e9 * H * C / K;

    let exponent: f64 = fac2 / (wave * temp);
    exponent / (1.0 - (-exponent).exp())
}
