# CROSS-REF — mycelium-cli

Mycelium-internal dependencies only (steer handoff §6.1; external crates stay in Cargo
metadata). Pinned revs are the fixed (buildable) tips recorded by the Phase-B wave;
content hash = git tree hash of the pinned rev.

| Interface consumed | Repo | Pinned rev | Content hash | Notes |
|---|---|---|---|---|
| mycelium-cli-common | https://github.com/tzervas/mycelium-cli-common | `c673fafdd1bcecb713e3560689af6ab152033867` | tree `c6fc7df3db94db6de08a7004729b1c37a7f5fc76` | Rust API of `mycelium-cli-common` (see monorepo `docs/api-index/INDEX.md#mycelium-cli-common`) |
| mycelium-interp | https://github.com/tzervas/mycelium-runtime | `0a284a79573ecfdab3db719c5f76ee9abcd155c1` | tree `(tree hash: fetch dep rev locally to resolve)` | WP-4 co-dev: runtime#11 HostCallRegistry / register_host |
| mycelium-l1 | https://github.com/tzervas/mycelium-l1 | `2b92f54349eb0d4f67e32e983874df76908b9ab6` | tree `(tree hash: fetch dep rev locally to resolve)` | Rust API of `mycelium-l1` (see monorepo `docs/api-index/INDEX.md#mycelium-l1`) |
| mycelium-proj | https://github.com/tzervas/mycelium-proj | `20b8a6d264ac728e81cfe8cd90cec8d2a91370be` | tree `(tree hash: fetch dep rev locally to resolve)` | Rust API of `mycelium-proj` (see monorepo `docs/api-index/INDEX.md#mycelium-proj`) |
| mycelium-spore | https://github.com/tzervas/mycelium-spore | `283f9fd901607841d5302d5935d15d873a32eef7` | tree `(tree hash: fetch dep rev locally to resolve)` | Rust API of `mycelium-spore` (see monorepo `docs/api-index/INDEX.md#mycelium-spore`) |
| mycelium-std-sys-host | https://github.com/tzervas/mycelium-std-sys-host | `067da5070e0fea796a6ca030c845c140985cab69` | tree `(feature host-registry → dep host-registry)` | WP-4 co-dev: std-sys-host#7 install_default_host_ops |

**Owning docs:** see monorepo `docs/Doc-Index.md`.
**Source provenance:** extracted from `tzervas/mycelium` archive `aad96b7a…`; fixed by
the course-correction Phase B (workspace root, git pins, toolchain + supply-chain
replicas, CI v2). Full program record: monorepo
`docs/planning/course-correction-2026-07-18/PROGRAM.md`.
