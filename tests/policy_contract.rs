use quorumforge::adjudicate::Policy;

#[test]
fn default_policy_is_valid() {
    Policy::default().validate().expect("default policy must be valid");
}

#[test]
fn ratios_outside_the_unit_interval_are_rejected() {
    let p = Policy { consensus_threshold: 1.5, ..Policy::default() };
    assert!(p.validate().expect_err("threshold").contains("consensus_threshold"));
    let p = Policy { dissent_ceiling: -0.1, ..Policy::default() };
    assert!(p.validate().expect_err("ceiling").contains("dissent_ceiling"));
}

#[test]
fn negative_minimum_mass_is_rejected() {
    let p = Policy { minimum_mass: -1.0, ..Policy::default() };
    assert!(p.validate().expect_err("mass").contains("minimum_mass"));
}
