//! S-RUN-COREVALUE (PKG-INTERP-CORRECTNESS) — `myc run` on a `main` that evaluates to an algebraic
//! data value: `eval_core` (not `eval`), rendered via [`render_core_value`]/[`render_datum`]. White-box
//! access via `use crate::*`.

use crate::*;
use std::path::PathBuf;

/// `tests/fixtures/run-unit-main/{mycelium-proj.toml,run_unit_main.myc}` — the package's own
/// measured baseline repro: `nodule proj; fn main() => Unit = Unit;`.
fn unit_main_fixture_manifest() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/run-unit-main/mycelium-proj.toml")
}

/// `tests/fixtures/run-data-nested/{mycelium-proj.toml,run_data_nested.myc}` — a two-level nested
/// `Datum` result (`Out(In(1), 2)`).
fn data_nested_fixture_manifest() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/run-data-nested/mycelium-proj.toml")
}

/// The measured baseline this whole package item exists to fix: before the `eval`→`eval_core`
/// switch, `myc run` on this exact program refused with rc=65 `error[myc-run-eval]: the program
/// evaluated to a data value; use eval_core for the data fragment` even though `myc check` already
/// called it clean. It must now run end-to-end (checkable_done).
#[test]
fn run_a_unit_returning_main_succeeds_end_to_end() {
    let report =
        run(&unit_main_fixture_manifest()).expect("a Unit-returning main must run, not refuse");
    assert_eq!(report.entry, "main");
    assert_eq!(report.source, "run_unit_main.myc");
    // Content-addressed, never a fabricated name (ADR-003): `#<hash>#<index>` — `Unit` is a nullary
    // constructor, so no trailing `(...)`.
    assert!(
        report.rendered.starts_with('#'),
        "a data result must render content-addressed (CtorRef Display), not the old Repr Debug \
         form: {}",
        report.rendered
    );
    assert!(
        !report.rendered.contains('('),
        "a nullary constructor (Unit) has no fields, so no `(...)` should appear: {}",
        report.rendered
    );
}

/// The pre-existing `Repr` rendering (`format!(\"{value:?}\")`, the `Value { repr: ..., payload:
/// ..., meta: ... }` Rust-Debug-ish form) must stay BYTE-IDENTICAL after switching `eval`→
/// `eval_core` — only the new `Data` arm is new code. Reuses the already-committed
/// `run-single-nodule` fixture (a `Binary{8}` result, i.e. `CoreValue::Repr`), whose expected
/// substance this crate's own pre-existing `run_executes_a_committed_single_nodule_fixture_end_to_end`
/// test already pins bit-for-bit — this test additionally pins the outer `Value { ... }` Debug
/// wrapper shape survives unchanged.
#[test]
fn repr_rendering_is_unchanged_after_eval_core_switch() {
    let report = run(&PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/run-single-nodule/mycelium-proj.toml"))
    .expect("the fixture runs end-to-end");
    assert!(
        report.rendered.starts_with("Value { repr:"),
        "a Repr result's rendering must stay the pre-existing Debug form verbatim: {}",
        report.rendered
    );
}

/// `render_datum` recurses through a nested `Datum` (a data field holding another data value), not
/// just the empty-fields nullary-constructor case `run_a_unit_returning_main_succeeds_end_to_end`
/// exercises.
#[test]
fn nested_datum_renders_recursively_and_ends_end_to_end() {
    let report = run(&data_nested_fixture_manifest())
        .expect("a two-level nested data result must run and render, not crash or refuse");
    assert_eq!(report.entry, "main");
    // Outer's ctor, applied to two fields: the inner Datum (itself `#hash#i(...)`) and a plain
    // `Binary{8}` Repr Debug dump — i.e. two levels of `#...#...(` nesting.
    assert_eq!(
        report.rendered.matches("#").count().min(4),
        4,
        "expected at least two `#hash#index` ctor refs (outer + inner): {}",
        report.rendered
    );
    assert!(
        report.rendered.contains("Value { repr:"),
        "the innermost leaf fields are still Repr values, rendered in their unchanged Debug form: {}",
        report.rendered
    );
}

/// Build an N-deep chain of nested nullary-at-the-leaf [`Datum`]s
/// (`Datum(ctor, [Datum(ctor, [Datum(ctor, [...empty leaf...])])])`), reusing one hand-built
/// [`CtorRef`](mycelium_core::data::CtorRef) — a white-box helper independent of the interpreter or
/// data registry, so the depth is exactly controllable.
fn nested_datum_chain(depth: usize) -> CoreValue {
    use mycelium_core::data::CtorRef;
    use mycelium_core::id::ContentHash;

    let decl_hash =
        ContentHash::parse("blake3:deadbeefdeadbeefdeadbeefdeadbeef").expect("shape-valid stub");
    let ctor = CtorRef::new(decl_hash, 0);

    let mut cv = CoreValue::Data(Datum::new(ctor.clone(), vec![])); // the nullary leaf
    for _ in 0..depth {
        cv = CoreValue::Data(Datum::new(ctor.clone(), vec![cv]));
    }
    cv
}

/// [`render_core_value`]/[`render_datum`] over a hand-built, deeply-nested [`Datum`] — a white-box
/// unit check independent of the interpreter, well past the adversarial checklist's 100+
/// nesting-depth bar and below the fixed render ceiling: must render fully, never crash, never
/// silently truncate.
#[test]
fn render_datum_handles_deep_nesting_without_crashing() {
    let rendered = render_core_value(&nested_datum_chain(300));
    assert!(
        !rendered.is_empty(),
        "a deeply-nested Datum must render to something, never silently empty"
    );
    assert!(
        !rendered.contains("depth limit"),
        "300 < the render-depth ceiling; should render fully, not hit the bounded refusal: \
         {rendered}"
    );
}

/// Past the fixed render ceiling: [`render_datum`] must refuse EXPLICITLY (bounded, located text),
/// never a silent truncation and never a host-stack crash (the adversarial checklist's explicit
/// "graceful output or an explicit bounded refusal, never a crash" bar).
#[test]
fn render_datum_refuses_explicitly_past_the_depth_ceiling() {
    let rendered = render_core_value(&nested_datum_chain(600));
    assert!(
        rendered.contains("depth limit"),
        "past the render-depth ceiling, the refusal must be explicit in the output, not a silent \
         truncation or a crash: {rendered}"
    );
}
