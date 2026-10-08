# Implementation Plan: Crates.io Publishing Configuration & GitHub Actions CI/CD

## Overview

This plan defines the configuration, workflows, architectural decisions, and operational instructions required to publish `lmxml` and `lmxml-derive` to [crates.io](https://crates.io) and establish automated CI/CD pipelines via GitHub Actions.

In accordance with instructions:
- **No commits will be created** during this task.
- Work is isolated on feature branch `0004-crates-publishing-and-ci`.
- Synchronization steps are outlined for repository remote setup, crates.io token creation, and initial publication.

---

## Proposed Changes

### 1. Workspace & Package Metadata Configuration

Currently:
- `Cargo.toml` (root) defines only workspace members and resolver.
- `lmxml/Cargo.toml` has minimal metadata (lacks repository, homepage, keywords, categories).
- `lmxml-derive/Cargo.toml` has minimal metadata (lacks repository, homepage, keywords, categories, documentation, and dedicated README).

#### 1.1 Root `Cargo.toml`
Enhance root `Cargo.toml` with `[workspace.package]` inheritance:
```toml
[workspace]
members = [
    "lmxml",
    "lmxml-derive",
]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["Filip Malczak <filip.malczak@gmail.com>"]
license = "MIT"
repository = "https://github.com/filipmalczak/lmxml-rust"
homepage = "https://github.com/filipmalczak/lmxml-rust"
readme = "README.md"
keywords = ["xml", "llm", "prompt", "serialization", "lmxml"]
categories = ["encoding", "text-processing"]
```

#### 1.2 `lmxml/Cargo.toml`
Update to inherit workspace metadata and define crate-specific documentation:
```toml
[package]
name = "lmxml"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true
homepage.workspace = true
readme.workspace = true
keywords.workspace = true
categories.workspace = true
description = "Language Model XML (LMXML) serialization format for Rust"
documentation = "https://docs.rs/lmxml"

[dependencies]
lmxml-derive = { path = "../lmxml-derive", version = "0.1.0" }
```

#### 1.3 `lmxml-derive/Cargo.toml`
Update to inherit workspace metadata and add dedicated README reference:
```toml
[package]
name = "lmxml-derive"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true
homepage.workspace = true
keywords.workspace = true
categories.workspace = true
description = "Derive macro for LMXMLSerializable in lmxml"
documentation = "https://docs.rs/lmxml-derive"
readme = "README.md"

[lib]
proc-macro = true

[dependencies]
proc-macro2 = "1.0"
quote = "1.0"
syn = { version = "2.0", features = ["full", "extra-traits"] }
```

#### 1.4 `lmxml-derive/README.md`
Add a dedicated README for the derive crate explaining that it is the companion proc-macro crate to `lmxml`.

---

### 2. GitHub Actions Pipelines

Create `.github/workflows/` with two distinct pipelines:

#### 2.1 Per-Commit & PR Verification Pipeline (`.github/workflows/ci.yml`)
- **Triggers**:
  - `push` on branch `master`
  - `pull_request` on branch `master`
- **Jobs**:
  - `fmt`: Runs `cargo fmt --all --check`
  - `clippy`: Runs `cargo clippy --all-targets --all-features -- -D warnings`
  - `test`: Runs `cargo test --all-targets --all-features`
  - `doc`: Runs `cargo doc --no-deps --all-features`
  - `package-dry-run`: Runs `cargo publish --package lmxml-derive --dry-run` to ensure packaging invariants hold

#### 2.2 Release Tag Publishing Pipeline (`.github/workflows/publish.yml`)
- **Triggers**:
  - `push` on tags matching `v*` (e.g. `v0.1.0`)
  - `workflow_dispatch` (manual dispatch with optional tag parameter)
- **Topological Publishing Order**:
  Because `lmxml` depends on `lmxml-derive` at compile time, `cargo publish` on `lmxml` verifies that the matching `lmxml-derive` version is present in the crates.io index.
  - Step 1: Run comprehensive tests and linting.
  - Step 2: Publish `lmxml-derive` using `CARGO_REGISTRY_TOKEN`.
  - Step 3: Wait / poll for crates.io index propagation (sleep 30-45s).
  - Step 4: Publish `lmxml` using `CARGO_REGISTRY_TOKEN`.

---

### 3. Knowledge Base & Architectural Decision

#### 3.1 Knowledge Entry: `knowledge/0004-publishing-to-crates-io.md`
A comprehensive guide detailing:
1. **One-Time Setup**:
   - Creating/verifying crates.io account.
   - Generating an API token (`publish-new` and `publish-update` scopes).
   - Adding `CARGO_REGISTRY_TOKEN` secret to GitHub Repository Settings.
   - Verifying repository remote URL.
2. **Standard Automated Release Flow**:
   - Version bump in `Cargo.toml`.
   - Local validation checks.
   - Creating and pushing git tag `vX.Y.Z`.
   - Monitoring the GitHub Actions publication run.
3. **Manual CLI Fallback Flow**:
   - `cargo login`
   - Publishing `lmxml-derive` first.
   - Waiting for index cache update.
   - Publishing `lmxml`.
4. **Troubleshooting & Gotchas**:
   - Index propagation delay ("no matching package named lmxml-derive found").
   - Immutability of crate releases on crates.io.
   - Workspace dependency version synchronization.

#### 3.2 Decision Log: `decisions/0004-crates-publishing-and-ci.md`
Document:
- Use of workspace package metadata inheritance.
- Separation of CI (testing/linting on commit) and CD (tag-based publication).
- Dependency-ordered release sequencing for workspace crates.
- API token secret conventions.

---

### 4. Synchronization Checklist for User

Since no commits will be made and the remote repository is not yet configured locally:
1. **Repository Remote**:
   - Check if remote `origin` is added: `git remote add origin git@github.com:filipmalczak/lmxml-rust.git` (or https equivalent).
2. **crates.io Account & Name Availability**:
   - Ensure you are logged into crates.io.
   - Verify ownership or availability of `lmxml` and `lmxml-derive`.
3. **GitHub Secrets**:
   - Add `CARGO_REGISTRY_TOKEN` to GitHub repository secrets.

---

## Verification Plan

### Automated / Local Tests
1. `cargo check` and `cargo test`: verify all existing 103 tests pass.
2. `cargo clippy --all-targets --all-features -- -D warnings`: ensure zero warnings.
3. `cargo fmt --check`: ensure clean code formatting.
4. `cargo publish --package lmxml-derive --dry-run`: confirm `lmxml-derive` packages without metadata warnings or errors.
5. GitHub Actions workflow syntax validation: verify YAML structure and action versions.

### Repository Status
1. Feature branch: `0004-crates-publishing-and-ci`.
2. Stage modified/created production files (`Cargo.toml`, `lmxml/Cargo.toml`, `lmxml-derive/Cargo.toml`, `lmxml-derive/README.md`, `.github/workflows/*`, `knowledge/*`, `decisions/*`).
3. Maintain uncommitted state (do NOT commit).
4. Update transcript in `transcripts/0004-crates-publishing-and-ci.md`.
