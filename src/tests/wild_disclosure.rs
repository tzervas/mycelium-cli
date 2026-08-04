//! S-WILD-DISCLOSURE-CLI (PKG-INTERP-CORRECTNESS) — `contains_wild`'s exhaustive walk +
//! `wild_boundary_banner`'s wording, and the end-to-end `myc run` disclosure it drives. White-box
//! access via `use crate::*`.

use crate::*;
use std::path::{Path, PathBuf};

#[cfg(feature = "host-registry")]
fn wild_main_fixture_manifest() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/run-wild-main/mycelium-proj.toml")
}

#[cfg(feature = "host-registry")]
fn wild_transitive_fixture_manifest() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/run-wild-transitive/mycelium-proj.toml")
}

fn run_hello_fixture_manifest() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/run-single-nodule/mycelium-proj.toml")
}

/// Parse + check + elaborate a single-`.myc`-source fixture's `main`, mirroring exactly what
/// `run_single_nodule` does internally (white-box), so `contains_wild` can be tested directly
/// against a REAL elaborated `Node`, not a hand-built stand-in.
fn elaborate_fixture_main(myc_path: &Path) -> Node {
    let text = std::fs::read_to_string(myc_path).expect("fixture source reads");
    let nodule = parse(&text).expect("fixture parses");
    let env = check_nodule(&nodule).expect("fixture checks clean");
    elaborate(&env, "main").expect("fixture elaborates")
}

/// The direct-call case: `main`'s own top-level body is `(wild { time_mono_nanos() }) : Unit`.
#[test]
fn contains_wild_true_for_a_direct_wild_call() {
    let myc = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/run-wild-main/run_wild_main.myc");
    assert!(contains_wild(&elaborate_fixture_main(&myc)));
}

/// The adversarial-checklist case: the only `wild` call is inside a `helper()` function `main`
/// calls — NOT textually inside `main`'s own top-level body. `contains_wild` walks the ELABORATED
/// node (where the call is resolved through `App`/`Fix`, not the surface `Expr` AST), so this must
/// still be `true` — an exhaustive walk with no `_ => false` catch-all can't silently miss it.
#[test]
fn contains_wild_true_for_a_transitively_called_wild() {
    let myc = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/run-wild-transitive/run_wild_transitive.myc");
    assert!(contains_wild(&elaborate_fixture_main(&myc)));
}

/// The common case: zero `wild` blocks anywhere — must be `false` (no stderr noise regression).
#[test]
fn contains_wild_false_for_a_wild_free_program() {
    let myc = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/run-single-nodule/run_hello.myc");
    assert!(!contains_wild(&elaborate_fixture_main(&myc)));
}

/// The banner states what is NOT verified and must never overclaim that verification now happens
/// (G2/VR-5) — a reassuring-sounding but false message would itself be a silent-wrong-type risk.
#[test]
fn the_banner_discloses_the_gap_honestly_and_does_not_overclaim() {
    let banner = wild_boundary_banner();
    assert!(
        banner.contains("not") && (banner.contains("verified") || banner.contains("checked")),
        "the banner must state plainly that verification does NOT happen: {banner}"
    );
    assert!(
        banner.contains("ADR-014") || banner.contains("VR-5"),
        "the banner should cite the governing decision it stays true to: {banner}"
    );
    assert!(
        !banner.to_lowercase().contains("verified against") || banner.contains("not"),
        "must never read as an affirmative verification claim: {banner}"
    );
}

/// End-to-end (M-908 v0 single-nodule path): the measured baseline repro
/// (`nodule proj @std-sys; fn main() => Unit !{{ffi}} = (wild {{ time_mono_nanos() }}) : Unit;`)
/// still exits 0 (`run` succeeds) — this remains a legitimately well-typed v0 program (ADR-014); the
/// fix here is disclosure, never refusal. Gated on `host-registry` (default-on; `cli_process.rs`'s
/// process-spawn tests build the real default binary regardless) — `time_mono_nanos` is only
/// actually GRANTED via `install_default_host_ops` (S-HOST-REGISTRY), so a `--no-default-features`
/// build fails this program's `eval_core` for a documented, unrelated reason
/// (`myc-run-eval`/`host-op-not-registered`), not a wild-disclosure regression.
#[cfg(feature = "host-registry")]
#[test]
fn run_still_succeeds_on_a_well_typed_wild_program() {
    let report = run(&wild_main_fixture_manifest())
        .expect("a legitimately well-typed wild program must still run, never be refused");
    assert_eq!(report.entry, "main");
}

/// Same success guarantee for the transitive-call fixture (same `host-registry` gating rationale).
#[cfg(feature = "host-registry")]
#[test]
fn run_still_succeeds_on_a_transitively_wild_program() {
    let report = run(&wild_transitive_fixture_manifest())
        .expect("a legitimately well-typed transitively-wild program must still run");
    assert_eq!(report.entry, "main");
}

/// The plain, wild-free fixture keeps running unaffected (no behavior change for the common case).
#[test]
fn run_still_succeeds_on_a_plain_wild_free_program() {
    let report = run(&run_hello_fixture_manifest()).expect("the wild-free fixture still runs");
    assert_eq!(report.entry, "main");
}
