//! S-HOST-REGISTRY / WP-4 — `myc run` prim-registry install path.
//!
//! White-box: [`crate::run_prim_registry`] is the single construction point used by
//! `interpreter_for` before evaluation.

use crate::*;
use mycelium_interp::PrimRegistry;

/// Floor op names from `install_default_host_ops` (sys-host INSTALL_HOST_OPS / host_registry).
const FLOOR_OPS: &[&str] = &[
    "time_mono_nanos",
    "time_wall_nanos",
    "rand_fill",
    "process_spawn",
    "process_wait",
    "process_kill",
];

/// Without host install (`--no-default-features`), the run registry grants no `wild:` ops.
#[cfg(not(feature = "host-registry"))]
#[test]
fn run_prim_registry_empty_without_host_feature() {
    assert!(!host_registry_enabled());
    let reg = run_prim_registry();
    for name in FLOOR_OPS {
        assert!(
            !reg.has_host(name),
            "without host-registry feature, `{name}` must not be granted (empty-by-design)"
        );
        assert!(
            !reg.has_host(&format!("wild:{name}")),
            "qualified wild:{name} must also be absent"
        );
        assert!(
            reg.get(&format!("wild:{name}")).is_none(),
            "get(wild:{name}) must be None so eval fails loud (UnknownPrim / host capability)"
        );
    }
}

/// Bare `with_builtins` (no install path) still has zero host ops — the loud-failure baseline.
#[test]
fn with_builtins_alone_grants_no_host_ops() {
    let reg = PrimRegistry::with_builtins();
    assert!(
        !reg.has_host("fs_read"),
        "default builtins must not grant wild:fs_read"
    );
    assert!(reg.get("wild:fs_read").is_none());
    for name in FLOOR_OPS {
        assert!(!reg.has_host(name), "builtins must not grant {name}");
    }
}

/// Default / `host-registry` feature: floor ops present after `install_default_host_ops`.
#[cfg(feature = "host-registry")]
#[test]
fn run_prim_registry_has_host_after_default_install() {
    assert!(
        host_registry_enabled(),
        "default build enables host-registry"
    );
    let reg = run_prim_registry();
    for name in FLOOR_OPS {
        assert!(
            reg.has_host(name),
            "after install_default_host_ops, has_host({name}) must be true"
        );
        assert!(
            reg.has_host(&format!("wild:{name}")),
            "qualified wild:{name} must also resolve"
        );
        assert!(
            reg.get(&format!("wild:{name}")).is_some(),
            "get(wild:{name}) must be Some after install"
        );
    }
}

/// `run_prim_registry` is what the run path uses (not a bare `Interpreter::default`).
#[test]
fn run_prim_registry_is_callable_for_smoke() {
    let reg = run_prim_registry();
    // Builtins always present (smoke: pure prims still registered).
    let names = reg.names();
    assert!(
        !names.is_empty(),
        "with_builtins should register pure prims; names={names:?}"
    );
}
