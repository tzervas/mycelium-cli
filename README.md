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
`@std-sys` floor ops (`time_mono_nanos`, `time_wall_nanos`, `rand_fill`, `process_*`, later
`fs_*`) are granted under the `wild:` namespace.

| Build | Host ops | Behaviour |
|-------|----------|-----------|
| default (`cargo test` / `cargo run`) | default table from `mycelium-std-sys-host` | `has_host` true for floor ops after install |
| `--no-default-features` | **none** | unresolved `wild:` fails loud (`UnknownPrim` / host capability not granted — G2) |

Compile without the host table (e.g. while co-developing against a tip that lacks
`install_default_host_ops`):

```bash
cargo test --no-default-features
```

### Feature: `net-host` (S-STD-NET / WP-6) — **opt-in**

Ambient HTTPS is **not** floor. Enable `net-host` to call
`mycelium_std_net::install_http_host_ops` after the default install so catalog name
`http_request` is granted as `wild:http_request`:

```bash
cargo test --features net-host
cargo run --features net-host -- run
```

| Build | `http_request` | Behaviour |
|-------|----------------|-----------|
| default (no `net-host`) | **absent** | unresolved `wild:http_request` fails loud (G2) |
| `--features net-host` | installed | `has_host("http_request")` true after install |

Pins:

- [`mycelium-runtime`](https://github.com/tzervas/mycelium-runtime) main — `register_host` / `has_host` / `install_host_ops`
- [`mycelium-std-sys-host`](https://github.com/tzervas/mycelium-std-sys-host) main — `install_default_host_ops`
- [`mycelium-std-net`](https://github.com/tzervas/mycelium-std-net) main — `install_http_host_ops` (`host-registry` feature on that crate)

Hub: [mycelium-lang#30](https://github.com/tzervas/mycelium-lang/issues/30).
