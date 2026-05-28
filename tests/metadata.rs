//! Invariants on the v3 metadata catalogue.
//!
//! These run via `cargo test` with the default feature set, so the registry
//! used here reflects what `hash-bench metadata` would emit for a normal build.

use hash_bench::metadata::metadata_report;

#[test]
fn schema_version_is_v3() {
    let report = metadata_report();
    assert_eq!(report.schema_version, 3, "schema_version must be v3");
}

#[test]
fn dos_resistant_implies_keyed() {
    let report = metadata_report();
    for a in &report.algorithms {
        if a.dos_resistant {
            assert!(
                a.keyed,
                "{} [{}] is dos_resistant but not keyed",
                a.name, a.variant
            );
        }
    }
}

#[test]
fn hw_required_implies_non_empty_hw_features() {
    let report = metadata_report();
    for a in &report.algorithms {
        if a.hardware_required {
            assert!(
                !a.hardware_features.is_empty(),
                "{} [{}] is hardware_required but hardware_features is empty",
                a.name,
                a.variant
            );
        }
    }
}

#[test]
fn sw_variant_has_no_hw_features() {
    let report = metadata_report();
    for a in &report.algorithms {
        if a.variant == "sw" {
            assert!(
                !a.hardware_required,
                "{} [sw] should not be hardware_required",
                a.name,
            );
            assert!(
                a.hardware_features.is_empty(),
                "{} [sw] should not list hardware_features (found {:?})",
                a.name,
                a.hardware_features,
            );
        }
    }
}

#[test]
fn known_keyed_tags_present() {
    let report = metadata_report();
    let by_key: std::collections::HashMap<_, _> = report
        .algorithms
        .iter()
        .map(|a| ((a.name.as_str(), a.variant.as_str()), a))
        .collect();

    for name in ["SipHash-1-3", "SipHash-2-4"] {
        let a = by_key
            .get(&(name, "sw"))
            .unwrap_or_else(|| panic!("missing {name} [sw]"));
        assert!(
            a.keyed && a.dos_resistant,
            "{name} should be keyed+DoS-resistant"
        );
    }

    for name in ["HighwayHash-64", "HighwayHash-128", "HighwayHash-256"] {
        let a = by_key
            .get(&(name, "sw"))
            .unwrap_or_else(|| panic!("missing {name} [sw]"));
        assert!(
            a.keyed && a.dos_resistant,
            "{name} should be keyed+DoS-resistant"
        );
    }

    for name in ["XXH3_64", "XXH3_128"] {
        let a = by_key
            .get(&(name, "sw"))
            .unwrap_or_else(|| panic!("missing {name} [sw]"));
        assert!(a.keyed, "{name} should be keyed");
        assert!(
            !a.dos_resistant,
            "{name} should not claim DoS-resistance (no adversarial hardening)"
        );
    }
}
