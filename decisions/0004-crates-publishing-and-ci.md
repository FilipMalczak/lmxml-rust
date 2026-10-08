# 0004. Crates.io Publishing and CI/CD Pipelines

## Context

`lmxml-rust` is a multi-crate Cargo workspace comprising the core runtime library `lmxml` and the procedural derive macro `lmxml-derive`. 

To release these crates to the Rust ecosystem on [crates.io](https://crates.io) and ensure consistent quality control, we require:
1. Complete package metadata compliant with crates.io standards without duplicating attributes across members.
2. Automated continuous integration (CI) verifying formatting, lints, tests, doc generation, and package packaging on every push and pull request.
3. Automated continuous deployment (CD) publishing new versions to crates.io upon pushing semantic release tags.
4. Robust handling of the inter-crate dependency graph during publication (`lmxml` requires `lmxml-derive`).

## Decision

1. **Workspace Package Metadata Inheritance:**
   All shared package attributes (`version`, `edition`, `authors`, `license`, `repository`, `homepage`, `keywords`, `categories`, and `readme`) are centralized in the workspace root `Cargo.toml` under `[workspace.package]`. Workspace members inherit these via `<field>.workspace = true`. Crate-specific fields (such as `name`, `description`, `documentation`, and custom READMEs) are retained in individual crate manifests.

2. **Dedicated Proc-Macro Documentation:**
   A dedicated `lmxml-derive/README.md` is provided for `lmxml-derive` so that visitors navigating directly to its crates.io page receive clear guidance directing them to the primary `lmxml` crate.

3. **Separation of CI and CD Workflows:**
   - `.github/workflows/ci.yml`: Triggers on `push` to `master` and all PRs targeting `master`. Enforces `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test`, `cargo doc`, and packaging dry-runs.
   - `.github/workflows/publish.yml`: Triggers only on tag pushes matching `v*` (and via manual `workflow_dispatch`).

4. **Dependency-Ordered Topological Publication:**
   Because `lmxml` specifies a compile-time dependency on `lmxml-derive`, `cargo publish` will fail if `lmxml-derive` is not yet available in the crates.io index. The publish workflow enforces sequential execution:
   - Publishes `lmxml-derive` first.
   - Polls the crates.io API and sleeps until `lmxml-derive` is verified live and propagated across the index.
   - Publishes `lmxml` second.

5. **Token Security via Standard Secret:**
   Workflows authenticate with crates.io using the standard repository secret `CARGO_REGISTRY_TOKEN`.

## Consequences

- Crate metadata remains strictly synchronized across workspace packages with single-point version bumps.
- Breaking changes and lint regressions are blocked before merge.
- Releases can be executed reliably by creating a signed git tag without requiring local cargo publishing commands or token distribution to developer machines.
