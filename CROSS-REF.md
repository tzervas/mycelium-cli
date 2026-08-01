# CROSS-REF — mycelium-cli

Mycelium-internal dependencies only (steer handoff §6.1; external crates stay in Cargo
metadata). Pinned revs are the fixed (buildable) tips recorded by the Phase-B wave;
content hash = git tree hash of the pinned rev.

| Interface consumed | Repo | Pinned rev | Content hash | Notes |
|---|---|---|---|---|
| mycelium-cli-common | https://github.com/tzervas/mycelium-cli-common | `c673fafdd1bcecb713e3560689af6ab152033867` | tree `c6fc7df3db94db6de08a7004729b1c37a7f5fc76` | Rust API of `mycelium-cli-common` (see monorepo `docs/api-index/INDEX.md#mycelium-cli-common`) |
| mycelium-interp | https://github.com/tzervas/mycelium-runtime | `4af6d0051f4da4961a108869c56407e32b9a372b` | tree `(tree hash: fetch dep rev locally to resolve)` | runtime main: HostCallRegistry / register_host (runtime#11) |
| mycelium-l1 | https://github.com/tzervas/mycelium-l1 | `2b92f54349eb0d4f67e32e983874df76908b9ab6` | tree `(tree hash: fetch dep rev locally to resolve)` | Rust API of `mycelium-l1` (see monorepo `docs/api-index/INDEX.md#mycelium-l1`) |
| mycelium-proj | https://github.com/tzervas/mycelium-proj | `20b8a6d264ac728e81cfe8cd90cec8d2a91370be` | tree `(tree hash: fetch dep rev locally to resolve)` | Rust API of `mycelium-proj` (see monorepo `docs/api-index/INDEX.md#mycelium-proj`) |
| mycelium-spore | https://github.com/tzervas/mycelium-spore | `283f9fd901607841d5302d5935d15d873a32eef7` | tree `(tree hash: fetch dep rev locally to resolve)` | Rust API of `mycelium-spore` (see monorepo `docs/api-index/INDEX.md#mycelium-spore`) |
| mycelium-std-sys-host | https://github.com/tzervas/mycelium-std-sys-host | `78fe4ad93c8fd5dd408b50502a9c3d4c48e8893d` | tree `(feature host-registry → dep host-registry)` | main: install_default_host_ops + process floor |
| mycelium-std-net | https://github.com/tzervas/mycelium-std-net | `1b6782541453d52ca1aa3deee24e08d3babc7313` | tree `(feature net-host → dep host-registry)` | WP-6: install_http_host_ops / wild:http_request (interp on runtime main) |

**Owning docs:** see monorepo `docs/Doc-Index.md`.
**Source provenance:** extracted from `tzervas/mycelium` archive `aad96b7a…`; fixed by
the course-correction Phase B (workspace root, git pins, toolchain + supply-chain
replicas, CI v2). Full program record: monorepo
`docs/planning/course-correction-2026-07-18/PROGRAM.md`.
