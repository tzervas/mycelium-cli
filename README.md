# mycelium-cli

<!-- FLEET-BADGES:BEGIN -->
[![CI](https://github.com/tzervas/mycelium-cli/actions/workflows/fleet-ci.yml/badge.svg?branch=main)](https://github.com/tzervas/mycelium-cli/actions/workflows/fleet-ci.yml?query=branch%3Amain)
[![Security](https://github.com/tzervas/mycelium-cli/actions/workflows/fleet-security.yml/badge.svg?branch=main)](https://github.com/tzervas/mycelium-cli/actions/workflows/fleet-security.yml?query=branch%3Amain)
[![Runner](https://img.shields.io/badge/runs--on-self--hosted%20podman-informational)](https://github.com/tzervas/gha-runner-ctl)
<!-- FLEET-BADGES:END -->


Component extracted from monorepo [`tzervas/mycelium`](https://github.com/tzervas/mycelium)
at archive tip `aad96b7a425710db5e91094d4fc2ca21a129e41a` (`archive/main-pre-component-transpile-2026-07-17`).

| Field | Value |
|---|---|
| **Program** | PROGRAM-SELFHOST-DECOMPOSE-2026-07-17 Phase D |
| **Source paths** | crates/mycelium-cli |
| **License** | MIT |
| **Honesty** | Extract is mechanical copy from archive; not DN-88 production-ready dogfood; guarantee tags stay Declared/Empirical until differential upgrades |

## Build

MSRV 1.96.1. All deps are git-rev pins (no monorepo path deps).

```bash
cargo test
```

### Feature: `host-registry` (S-HOST-REGISTRY / WP-4) — **default on**

`myc run` constructs a `PrimRegistry` via `PrimRegistry::with_builtins()`, then calls
`mycelium_std_sys_host::install_default_host_ops` **before** evaluation so the audited
`@std-sys` floor ops (`time_mono_nanos`, `time_wall_nanos`, `rand_fill`, later `fs_*`) are
granted under the `wild:` namespace.

| Build | Host ops | Behaviour |
|-------|----------|-----------|
| default (`cargo test` / `cargo run`) | default table from `mycelium-std-sys-host` | `has_host` true for floor ops after install |
| `--no-default-features` | **none** | unresolved `wild:` fails loud (`UnknownPrim` / host capability not granted — G2) |

Compile without the host table (e.g. while co-developing against a tip that lacks
`install_default_host_ops`):

```bash
cargo test --no-default-features
```

Co-dev pins (WIP until merge):

- [`mycelium-runtime#11`](https://github.com/tzervas/mycelium-runtime/pull/11) — `register_host` / `has_host` / `install_host_ops`
- [`mycelium-std-sys-host#7`](https://github.com/tzervas/mycelium-std-sys-host/pull/7) — `install_default_host_ops` (`host-registry` feature on that crate)

Hub: [mycelium-lang#30](https://github.com/tzervas/mycelium-lang/issues/30).
