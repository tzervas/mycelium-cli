# CROSS-REF — mycelium-cli

Mycelium-internal dependencies only (steer handoff §6.1; external crates stay in Cargo
metadata). Pinned revs are the fixed (buildable) tips recorded by the Phase-B wave;
content hash = git tree hash of the pinned rev.

| Interface consumed | Repo | Pinned rev | Content hash | Notes |
|---|---|---|---|---|
| mycelium-cli-common | https://github.com/tzervas/mycelium-cli-common | `c673fafdd1bcecb713e3560689af6ab152033867` | tree `(tree hash: fetch dep rev locally to resolve)` | Rust API of `mycelium-cli-common` (see monorepo `docs/api-index/INDEX.md#mycelium-cli-common`) |
| mycelium-interp | https://github.com/tzervas/mycelium-runtime | `ab9cee665b620ed80ab74ea61ea639817dc49077` | tree `(tree hash: fetch dep rev locally to resolve)` | Rust API of `mycelium-interp` (see monorepo `docs/api-index/INDEX.md#mycelium-interp`) |
| mycelium-l1 | https://github.com/tzervas/mycelium-l1 | `cd32a1ed7ab7d2be38c9c3047da7318d182b7a1c` | tree `(tree hash: fetch dep rev locally to resolve)` | Rust API of `mycelium-l1` (see monorepo `docs/api-index/INDEX.md#mycelium-l1`) |
| mycelium-proj | https://github.com/tzervas/mycelium-proj | `cfc934b32c0b7b57becbb7ddfe3c516712c6ec0e` | tree `(tree hash: fetch dep rev locally to resolve)` | Rust API of `mycelium-proj` (see monorepo `docs/api-index/INDEX.md#mycelium-proj`) |
| mycelium-spore | https://github.com/tzervas/mycelium-spore | `12f117d395545ee89ba144eb858de11f0b36ab16` | tree `(tree hash: fetch dep rev locally to resolve)` | Rust API of `mycelium-spore` (see monorepo `docs/api-index/INDEX.md#mycelium-spore`) |

**Owning docs:** see monorepo `docs/Doc-Index.md`.
**Source provenance:** extracted from `tzervas/mycelium` archive `aad96b7a…`; fixed by
the course-correction Phase B (workspace root, git pins, toolchain + supply-chain
replicas, CI v2). Full program record: monorepo
`docs/planning/course-correction-2026-07-18/PROGRAM.md`.
