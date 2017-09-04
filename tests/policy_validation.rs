use quorumforge::adjudicate::Policy;

#[test]
fn non_finite_minimum_mass_is_rejected() {
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let p = Policy { minimum_mass: bad, ..Policy::default() };
        let err = p.validate().expect_err("must be rejected");
        assert!(err.contains("minimum_mass"), "{err}");
    }
}

#[test]
