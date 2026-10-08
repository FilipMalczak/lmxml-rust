# lmxml-rust

## Vision

Reimplementation of [lmxml](https://github.com/filipmalczak/lmxml) python package as Rust crate.

`lmxml` is a subset of XML meant to be repeatable and easy to read both by people and LLMs.

## General

* Do not rewrite unrelated code.
* When uncertain about architecture, ask instead of inventing one.
* Prefer simple, explicit solutions over speculative abstractions.
* Do not implement future functionality unless the current task requires it.
* Keep framework concerns separate from application/harness concerns.
* Whenever we make an architectural or project-direction decision, document it in `./decisions/`.

  * Treat this directory as an append-only decision log.
  * Create new Markdown files; never modify existing ones on your own volition.

    * You may modify existing entries if directly instructed to.
    * You may modify an existing entry to mark it as superseded; in that case modification must be limited to a rename and adding or enhancing the comment at the top of the document.
  * Name entries by prefixing them with a four-digit number.
  * If a newly created decision supersedes an existing one, even partially:

    * rename the existing file by inserting `SUPERSEDED` between its entry number and entry slug;
    * add a Markdown `>` quote at the beginning of the superseded entry pointing to the superseding one.
  * `SUPERSEDED` does not mean that the entire document is invalid. Parts of the document may remain viable.
  * An entry may be superseded by more than one entry; record all follow-ups in the superseded entry's top comment.
* Put longer-lived implementation/project knowledge that is not an architectural decision in `./knowledge/`.
* Do not use `decisions/` or `knowledge/` for temporary notes, scratch work, or investigation debris.
* DO NOT read from `backlog.md` - this file is purely developer-oriented and should not be considered by agents at all

## Planning Agents

If you are an autonomous agent without explicit Plan and Act/Execute modes, prefer to make plans unless the task at hand is trivial.

If you decide to avoid planning, explain why.

When running in **Plan** mode:

* Never execute the plan without explicit approval.
* Approval is normally the **Proceed** button or switching to **Act** mode.
* A review is not approval.
* If uncertain whether approval was given, assume it was not.

## Git

### Repository State

If a new task is started while the repository is not currently on `master`, do not proceed automatically. Ask the user how to proceed:

- switch to `master` and create the new task's feature branch from there; or
- treat the current branch as intentional and proceed without applying the normal branch-from-`master` workflow.

The user may have intentionally started the task from an existing feature branch, or may simply have forgotten that the repository is mid-task. Do not assume either case.

 Do not perform feature work directly on `master`.
Every new task must be performed on its own feature branch.

### Feature Branches

Before implementing a new task:

1. Identify the relevant decision-log entry.
2. Use that decision's four-digit number as the beginning of the feature branch name.
3. Create the branch from the current `master`.

Branch naming convention:

```text
NNNN-short-task-description
```

For example:

```text
0001-repository-structure
0002-v0-harness
0003-persistent-runs
```

The branch number must correspond to the relevant decision-log entry.

Do not reuse a feature branch for an unrelated task.

### Review and Merge

The normal workflow is:

```text
master
  │
  └── NNNN-feature
          │
          ├── implementation
          ├── tests
          └── verification
                │
                ▼
           user review
                │
                ▼
          merge into master
```

Do not merge into `master` before the user has reviewed the current task and approved the merge.

After approval:

* merge the feature branch into `master`;
* do not squash or rewrite history unless explicitly requested;
* leave the repository on `master` after the merge unless instructed otherwise.

If the user requests changes after review, continue on the existing feature branch unless they explicitly define a new task.

Do not delete past task branches unless explicitly asked to.

### Commits

Never create commits unless:

1. the user explicitly instructs you to commit, or
2. you are executing an approved plan that explicitly includes a commit.

If a commit would otherwise be appropriate, stop and ask.

When asking for commit approval, include:

* proposed commit message;
* files that would be included.

If the user explicitly authorizes the task's normal Git workflow, commits required by that workflow are authorized.

Never overwrite manual user changes.

### Staging

* Stage all production files created or modified as part of the requested work.
* Do not stage scratch files, experiments, temporary scripts, or investigation artifacts.
* Always stage everything under `./knowledge/`.
* Leave the repository in a state where it could be committed immediately.

## Task Transcripts

Whenever a new task is started and a feature branch is created, store a rudimentary transcript of the task's user and agent messages.

- Create a dedicated `./transcripts/` directory.
- For each task, create one transcript file whose filename matches the task's decision-log number and branch name.
- The transcript should contain user and agent text messages, including user prompts.
- Separate transcript entries with `======` to visualize the parts.
- Do not include tool-call logs, tool results, internal reasoning/thinking, or other operational metadata.
- Start the transcript when the task begins and append to it as the task progresses.
- Keep transcripts separate from `./decisions/` and `./knowledge/`; they are historical conversation records, not architectural decisions or project knowledge.
- Do not modify or rewrite previous transcript content except to append missing messages from the same task.
- If you've created a plan during the conversation, store copy of that plan next to the transcript under `NNNN_implementation_plan.md` (where `NNNN` is the numeric prefix shared by decision log, knowledge entries, branch name, transcript name, etc); if there is more than one plan within the conversation, first one should have no suffix and following ones should have incremental suffixes starting with 2 (`..._plan.md`, `..._plan_2.md`, `..._plan_3.md`, ...)
- Remember to drump transcript before commiting, or you'll be left with uncomitted changes to the transcript

## Rust

### Common Commands

| Task                    | Command                                                    |
| ----------------------- | ---------------------------------------------------------- |
| Build                   | `cargo build`                                              |
| Fast compile/type check | `cargo check`                                              |
| Run                     | `timeout ... cargo run`                                    |
| Run release             | `cargo run --release`                                      |
| Run all tests           | `cargo test`                                               |
| Run specific test       | `cargo test <name>`                                        |
| Format                  | `cargo fmt`                                                |
| Check formatting        | `cargo fmt --check`                                        |
| Lint                    | `cargo clippy --all-targets --all-features -- -D warnings` |
| Documentation           | `cargo doc`                                                |
| Clean                   | `cargo clean`                                              |

Prefer `cargo check` during development when execution is unnecessary.

Use `cargo run` only when runtime behavior needs verification.

Do not use a `dev` profile unless absolutely necessary. Use release builds when runtime performance matters.

### General

* Keep the project formatted with `cargo fmt`.
* Resolve Clippy warnings instead of suppressing them unless there is a documented reason.
* Prefer `cargo add` for adding dependencies when available.
* Keep `Cargo.toml` and `Cargo.lock` consistent.
* Prefer idiomatic `Result<T, E>` error handling.
* Do not use `panic!` for normal error handling.
* Prefer imports with `use` over repeatedly using fully qualified paths.
* Alias imports when names conflict.
* Avoid unnecessary abstraction layers.
* Prefer explicit types and boundaries where they improve readability or testability.

### Module Layout

Prefer the modern Rust module layout:

```text
foo.rs
foo/
```

rather than:

```text
foo/
  mod.rs
```

Use `prelude.rs` for significant subsystems when it genuinely improves ergonomics. Do not create preludes merely for convention.

## Testing

Tests are part of implementation, not a final afterthought.

During development, run the subset of tests relevant to the current change.

Before considering work complete, always run:

```text
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
```

Do not finish while any of these fail.

Never remove or weaken tests without explicit approval.

Prefer testing observable behavior and meaningful boundaries over implementation details.

## Static Analysis

Use these tools throughout development:

* `cargo check` — compilation, type checking, borrow checking;
* `cargo fmt` — formatting;
* `cargo clippy` — linting and correctness guidance.

After significant changes:

1. run `cargo check`;
2. run affected tests;
3. address Clippy findings;
4. run the complete verification suite before completion.

Treat Clippy suggestions as the default unless there is a documented reason not to follow them.

## Code Organization

Keep individual modules focused.

Prefer small, cohesive components over large files containing unrelated responsibilities.

Do not reorganize existing code merely for aesthetic reasons unless the current task requires it.

When introducing a new subsystem, prefer a clear directory boundary and document important architectural decisions in `./decisions/`.

## Documentation

Keep `AGENTS.md` focused on repository-wide working rules.

Do not turn it into a project design document.

Use:

```text
./decisions/
```

for architectural and project-direction decisions.

Use:

```text
./knowledge/
```

for durable implementation knowledge, conventions, and other information future agents should retain.

Task-specific requirements and detailed implementation plans belong in the task prompt or appropriate repository documentation, not here.

## Final Task Checklist

Before declaring a task complete:

* [ ] Work is on the correct numbered feature branch.
* [ ] Architectural decisions are documented in `./decisions/`.
* [ ] Durable implementation knowledge is documented in `./knowledge/` where appropriate.
* [ ] No unrelated code was rewritten.
* [ ] Tests pass.
* [ ] `cargo test` passes.
* [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes.
* [ ] `cargo fmt --check` passes.
* [ ] Production changes are staged.
* [ ] Scratch/temporary files are not staged.
* [ ] Repository is ready for user review.
* [ ] Do not merge into `master` until the user has reviewed and approved the task.

