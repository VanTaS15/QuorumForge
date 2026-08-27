use bio999::adjudicate::Policy;

#[test]
fn non_finite_minimum_mass_is_rejected() {
    for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let p = Policy {
            minimum_mass: bad,
            ..Policy::default()
        };
        let err = p.validate().expect_err("must be rejected");
        assert!(err.contains("minimum_mass"), "{err}");
    }
}

#[test]
fn non_finite_ratios_are_rejected() {
    for bad in [f64::NAN, f64::INFINITY] {
        let p = Policy {
            consensus_threshold: bad,
            ..Policy::default()
        };
        assert!(p
            .validate()
            .expect_err("threshold")
            .contains("consensus_threshold"));
        let p = Policy {
            dissent_ceiling: bad,
            ..Policy::default()
        };
        assert!(p
            .validate()
            .expect_err("ceiling")
            .contains("dissent_ceiling"));
    }
}

#[test]
fn the_default_policy_is_valid() {
    Policy::default()
        .validate()
        .expect("default policy must be valid");
}
