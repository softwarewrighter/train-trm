# train-trm
Yew/Rust/WASM/CLI that implement Tiny Recursive Model training

agent instructions: q.v. the online paper for TRM (Tiny Recursive Model) a refinement of HRM (Hierarchical Reasoning Model)

implement this in Rust as a clap-based CLI

add a Yew-based web ui.

Demonstrate how to train the TRM.

Use Test-Driven-Development (TDD) with Red/Green testing.  Before commiting: format code, run clippy, do not bypass clippy checks, use annotations for apparently dead code that configure it appropriately for wasm test best practices, q.v., document this in docs/process.md.  Also, before committing, all tests should pass.  Do not add a feature or a fix without adding tests.  Before committing, validate .gitignore, updated docs.  When committing, use a detailed message.  Commit after each step when possible.

Start with a ./docs directory and create markdown files for: architecture, prd, design, plan, process, and status.

Proceed implementing the Rust project, following the process.
