# 0004. Instructions for Publishing Crates to crates.io

This document provides complete instructions for publishing `lmxml` and `lmxml-derive` to [crates.io](https://crates.io), covering one-time prerequisite setup, the automated GitHub Actions release workflow, the manual CLI fallback procedure, and known publishing gotchas.

---

## 1. One-Time Setup Checklist

Before publishing can succeed, complete the following setup steps once:

### 1.1 Crates.io Account & Email Verification
1. Navigate to [crates.io](https://crates.io/) and log in using your GitHub account (`filipmalczak`).
2. Go to **Account Settings** and confirm that your email address is verified. *Crates.io rejects publishes from accounts without a verified email.*

### 1.2 Generate an API Token
1. In crates.io, go to **Account Settings** -> **API Tokens** (or [crates.io/settings/tokens](https://crates.io/settings/tokens)).
2. Click **New Token**:
   - **Name**: e.g., `github-actions-lmxml`
   - **Scopes**: Ensure both `publish-new` and `publish-update` are selected.
3. Click **Create** and copy the generated token immediately (it will only be shown once).

### 1.3 Add GitHub Actions Repository Secret
1. Go to your GitHub repository: `https://github.com/filipmalczak/lmxml-rust`.
2. Navigate to **Settings** -> **Secrets and variables** -> **Actions**.
3. Under **Repository secrets**, click **New repository secret**:
   - **Name**: `CARGO_REGISTRY_TOKEN`
   - **Secret**: Paste the crates.io API token copied in step 1.2.
4. Click **Add secret**.

### 1.4 Set Git Remote (if not already set)
If your local repository does not yet have `origin` configured:
```bash
git remote add origin git@github.com:filipmalczak/lmxml-rust.git
# Or using HTTPS:
# git remote add origin https://github.com/filipmalczak/lmxml-rust.git
```

---

## 2. Standard Release Workflow (GitHub Actions)

Once the one-time setup is complete, releasing a new version is driven entirely by pushing a semantic version tag to GitHub.

### Step 2.1: Bump Version in `Cargo.toml`
In the root `Cargo.toml`, update the version in `[workspace.package]`:
```toml
[workspace.package]
version = "0.1.0" # Set new semantic version
```

If `lmxml` depends on a new version of `lmxml-derive`, update the version requirement in `lmxml/Cargo.toml`:
```toml
[dependencies]
lmxml-derive = { path = "../lmxml-derive", version = "0.1.0" }
```

Run `cargo check` to update `Cargo.lock`:
```bash
cargo check
```

### Step 2.2: Verify Pre-flight Invariants Locally
Before tagging, run the verification suite to ensure all gates pass:
```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo publish --package lmxml-derive --dry-run
```

### Step 2.3: Commit and Merge to Master
Commit the version bump and merge into `master` following standard repository review procedures:
```bash
git add Cargo.toml Cargo.lock lmxml/Cargo.toml lmxml-derive/Cargo.toml
git commit -m "Bump version to 0.1.0"
git push origin master
```

### Step 2.4: Create and Push the Release Tag
Create an annotated tag matching the version (prefixed with `v`) and push it:
```bash
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

### Step 2.5: Monitor GitHub Actions
1. Open the GitHub repository and navigate to the **Actions** tab.
2. The **Publish Crates** workflow will trigger automatically for tag `v0.1.0`.
3. The workflow executes:
   - Verification checks (`fmt`, `clippy`, `test`).
   - `cargo publish --package lmxml-derive`.
   - Polls the crates.io API until `lmxml-derive` is live (with safety propagation delay).
   - `cargo publish --package lmxml`.
4. Once completed, verify both crates are live on crates.io:
   - [https://crates.io/crates/lmxml-derive](https://crates.io/crates/lmxml-derive)
   - [https://crates.io/crates/lmxml](https://crates.io/crates/lmxml)

---

## 3. Manual CLI Fallback Procedure

If GitHub Actions is unavailable or you prefer publishing directly from your local terminal:

### Step 3.1: Authenticate Local Cargo
```bash
cargo login <YOUR_CRATES_IO_TOKEN>
```
*(Stored in `~/.cargo/credentials.toml`)*

### Step 3.2: Verify Working Tree
Cargo prevents publishing from a dirty git tree by default:
```bash
git status
```
Ensure all release changes are committed, or append `--allow-dirty` if intentionally testing uncommitted changes.

### Step 3.3: Publish `lmxml-derive` FIRST
Because `lmxml` depends on `lmxml-derive`, publishing `lmxml` first will fail with:
`error: no matching package named lmxml-derive found in crates.io index`

Publish the derive macro crate first:
```bash
cargo publish --package lmxml-derive
```

### Step 3.4: Wait for Index Propagation
Wait approximately 30 to 60 seconds for crates.io's sparse index to update:
```bash
# Verify it is visible via the crates.io API
curl -s https://crates.io/api/v1/crates/lmxml-derive/0.1.0 | grep '"num":"0.1.0"'
```

### Step 3.5: Publish `lmxml`
Once `lmxml-derive` is indexed:
```bash
cargo publish --package lmxml
```

---

## 4. Key Gotchas & Best Practices

1. **Topological Order is Mandatory:**
   `lmxml` strips `path = "../lmxml-derive"` when packaging and requires `lmxml-derive = "0.1.0"` from crates.io. Attempting to publish both concurrently or `lmxml` first will fail.
2. **Releases are Immutable:**
   Once a version (e.g. `0.1.0`) is published to crates.io, it can never be overwritten, re-uploaded, or replaced. Any bug fix or modification requires bumping the patch version (e.g. `0.1.1`).
3. **Docs.rs Builds:**
   Docs.rs will automatically build documentation for both crates upon publication:
   - `https://docs.rs/lmxml/0.1.0`
   - `https://docs.rs/lmxml-derive/0.1.0`
4. **First Publish Ownership:**
   Crate names on crates.io are first-come, first-served. Publishing version `0.1.0` registers you as the crate owner. You can invite additional owners via `cargo owner --add <github-username>`.
