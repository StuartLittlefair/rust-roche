use pyo3::prelude::*;

#[pyfunction]
///
///  Returns circularisation radius from Verbunt & Rappaport (as fraction of
/// binary separation)
/// 
/// Arguments:
/// 
/// * `q`: Mass ratio = M2/M1
/// 
pub fn rcirc(q: f64) -> f64 {
    let lq = q.log10();
    0.0883 + lq * (-0.04858 + lq * (0.11489 + 0.020475 * lq))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rcirc_test() -> () {
        // Values from trm.roche.rcirc
        assert_eq!(rcirc(0.2), 0.17139454448755287);
    }
}
