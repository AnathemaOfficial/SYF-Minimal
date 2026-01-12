#![no_std]

// In tests, Rust enables `std` by default. We declare it explicitly so `no_std` remains valid.
#[cfg(test)]
extern crate std;

/// Pure SYF invariant computation.
///
/// Law:
///     R = (F × E) / K
///
/// Constraints:
/// - F >= 0
/// - 0 <= E <= 1
/// - K > 0
pub fn syf_core(f: f64, e: f64, k: f64) -> Result<f64, &'static str> {
    if f < 0.0 {
        return Err("F must be >= 0");
    }
    if e < 0.0 || e > 1.0 {
        return Err("E must be in [0, 1]");
    }
    if k <= 0.0 {
        return Err("K must be > 0");
    }
    Ok((f * e) / k)
}

#[cfg(test)]
mod tests {
    use super::syf_core;

    #[test]
    fn test_case_1() {
        let r = syf_core(10.0, 1.0, 2.0).unwrap();
        assert!((r - 5.0).abs() < 1e-12);
    }

    #[test]
    fn test_case_2() {
        let r = syf_core(0.0, 0.5, 1.0).unwrap();
        assert!((r - 0.0).abs() < 1e-12);
    }

    #[test]
    fn test_case_3() {
        let r = syf_core(4.0, 0.25, 2.0).unwrap();
        assert!((r - 0.5).abs() < 1e-12);
    }

    #[test]
    fn test_invalid_f() {
        assert!(syf_core(-1.0, 0.5, 1.0).is_err());
    }

    #[test]
    fn test_invalid_e_low() {
        assert!(syf_core(1.0, -0.1, 1.0).is_err());
    }

    #[test]
    fn test_invalid_e_high() {
        assert!(syf_core(1.0, 1.1, 1.0).is_err());
    }

    #[test]
    fn test_invalid_k() {
        assert!(syf_core(1.0, 0.5, 0.0).is_err());
        assert!(syf_core(1.0, 0.5, -1.0).is_err());
    }
}
