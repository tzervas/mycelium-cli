//! Real-process integration tests (spawns the built `myc` binary via `CARGO_BIN_EXE_myc`) — the
//! only reliable way to assert on actual stdout/stderr *stream* separation and content, since the
//! in-crate white-box unit tests (`src/tests/`) call library functions directly and cannot observe
//! which fd an `eprintln!`/`println!` landed on.
//!
//! Covers: S-CLI-VERSION-MYC (stdout-only, single-line `--version`/`-V`) and S-WILD-DISCLOSURE-CLI
//! (the wild-boundary banner is present on stderr for a wild-containing program, absent for a
//! wild-free one).

use std::path::PathBuf;
use std::process::Command;

fn myc_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_myc"))
}

fn fixture_manifest(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(format!("tests/fixtures/{name}/mycelium-proj.toml"))
}

/// S-CLI-VERSION-MYC checkable_done, run for real: `out=$(myc --version 2>&1); rc=$?` gives rc=0
/// and `out` equals `mycelium-cli <version>` (matching `[workspace.package].version`, measured
/// `0.464.0`) — plus (adversarial checklist) confirms the emission is STDOUT-only, single line, not
/// also/instead stderr.
#[test]
fn version_flag_prints_stdout_only_single_line_and_exits_ok() {
    let out = Command::new(myc_bin())
        .arg("--version")
        .output()
        .expect("myc --version spawns");
    assert!(out.status.success(), "exit code must be 0: {out:?}");
    let stdout = String::from_utf8(out.stdout).expect("utf8 stdout");
    let stderr = String::from_utf8(out.stderr).expect("utf8 stderr");
    assert_eq!(
        stdout,
        format!("{} {}\n", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
        "stdout must be EXACTLY `mycelium-cli <version>` (one line) — an ap-workflows consumer \
         does a plain substring match on captured stdout"
    );
    assert!(
        stderr.is_empty(),
        "must not ALSO emit anything on stderr: {stderr:?}"
    );
}

/// `-V` is the same as `--version`.
#[test]
fn version_short_flag_matches_long_flag() {
    let long = Command::new(myc_bin())
        .arg("--version")
        .output()
        .expect("spawns");
    let short = Command::new(myc_bin()).arg("-V").output().expect("spawns");
    assert_eq!(long.stdout, short.stdout);
    assert!(short.status.success());
}

/// S-CLI-VERSION-MYC: must work from an arbitrary cwd with no `mycelium-proj.toml` present — before
/// any manifest/project resolution.
#[test]
fn version_flag_works_with_no_manifest_in_cwd() {
    let empty_dir = std::env::temp_dir().join(format!(
        "myc-cli-version-nomanifest-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&empty_dir).unwrap();
    let out = Command::new(myc_bin())
        .arg("--version")
        .current_dir(&empty_dir)
        .output()
        .expect("spawns");
    assert!(
        out.status.success(),
        "must succeed with no manifest present in cwd: {out:?}"
    );
}

/// S-WILD-DISCLOSURE-CLI checkable_done, run for real: the wild-boundary banner appears on STDERR
/// (not stdout) for a wild-containing program, `run` still exits 0. Gated on `host-registry`
/// (default-on; CI never builds `--no-default-features` — `.github/workflows/ci.yml` runs plain
/// `cargo test` and `cargo test --features net-host` only): `time_mono_nanos` is only actually
/// GRANTED via `install_default_host_ops`, so without the feature `eval_core` fails for a
/// documented, unrelated reason (`host-op-not-registered`) — the banner itself still fires first
/// either way, since it is unconditional on `contains_wild`, independent of host installation.
#[cfg(feature = "host-registry")]
#[test]
fn run_discloses_wild_boundary_on_stderr_for_a_wild_program() {
    let out = Command::new(myc_bin())
        .arg("run")
        .arg("--config")
        .arg(fixture_manifest("run-wild-main"))
        .output()
        .expect("myc run spawns");
    assert!(out.status.success(), "still exits 0: {out:?}");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("wild") && stderr.contains("AUDITED"),
        "the wild-boundary banner must appear on stderr: {stderr}"
    );
}

/// The complement: no banner line for a plain, wild-free program (no stderr noise regression).
#[test]
fn run_has_no_wild_banner_for_a_plain_program() {
    let out = Command::new(myc_bin())
        .arg("run")
        .arg("--config")
        .arg(fixture_manifest("run-single-nodule"))
        .output()
        .expect("myc run spawns");
    assert!(out.status.success(), "exits 0: {out:?}");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        !stderr.contains("AUDITED"),
        "a wild-free program must NOT print the wild-boundary banner: {stderr}"
    );
}

/// S-RUN-COREVALUE checkable_done, run for real end-to-end through the actual binary: a
/// `Unit`-returning `main` now exits 0 (was rc=65 `error[myc-run-eval]: the program evaluated to a
/// data value; use eval_core for the data fragment` before this fix).
#[test]
fn run_a_unit_main_exits_ok_through_the_real_binary() {
    let out = Command::new(myc_bin())
        .arg("run")
        .arg("--config")
        .arg(fixture_manifest("run-unit-main"))
        .output()
        .expect("myc run spawns");
    assert!(
        out.status.success(),
        "a Unit-returning main must exit 0, not the old rc=65 data-refusal: {out:?}"
    );
}
