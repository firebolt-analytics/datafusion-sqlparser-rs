# Extensible SQL Lexer and Parser for Rust Agents Guidelines

## General Agent Workflow
1. You will write unit tests to ensure your code change is working as expected.
2. You will run the commands in the Pre Commit Checks section below to ensure your change is ready for a pull request.
3. When instructed to open a PR, you will follow the instructions in the Pull Request Guidelines section below.

## General Coding Guidelines
1. Refrain from adding conditions on specific dialects, such as `dialect_is!(...)` or `dialect_of!(... | ...)`. Instead, define a new function in the `Dialect` trait that describes the condition, so that dialects can turn this condition on more easily.
2. Make targeted code changes and refrain from refactoring, unless it's absolutely required.

## Unit Tests Guidelines
- New unit tests should be added to the `tests` module in the corresponding dialect file (e.g., `tests/sqlparser_redshift.rs` for Redshift), and should be placed at the end of the file.
- If the new functionality is gated using a dialect function, and the SQL is likely relevant in most dialects, tests should be placed under `tests/sqlparser_common.rs`.
- When testing a multi-line SQL statement, use a raw string literal, i.e. `r#"..."#` to preserve formatting.
- The parser builds an abstract syntax tree (AST) from the SQL statement and has functionality to display the tree as SQL. Use the following template for simple unit tests where you expect the SQL created from the AST to be the same as the input SQL:
```rust
<dialect>().verified_stmt(r#"..."#);
```
For example: `snowflake().verified_stmt(r#"SELECT * FROM my_table"#)`. Use `one_statement_parses_to` instead of `verified_stmt` when you expect the SQL created by the AST to differ than the input SQL. For example:
```rust
snowflake().one_statement_parses_to(
    "SELECT * FROM my_table t",
    "SELECT * FROM my_table AS t",
)
```

## Analyzing Parsing Issues
You can try to simplify the SQL statement to identify the root cause of the parsing issue. This may involve removing certain clauses or components of the SQL statement to see if it can be parsed successfully. Additionally, you can compare the problematic SQL statement with similar statements that are parsed correctly to identify any differences that may be causing the issue.

## Pre Commit Checks
Run the following commands before you commit to ensure the change will pass the CI process:
```bash
cargo test --all-features
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
```

## Pull Request Guidelines
1. PR title should follow this format: `<DIALECT>: <SHORT DESCRIPTION>`, For example, `Showflake: Add support for casting to VARIANT`.
2. Make the PR comment short, provide an example of what was not working and a short description of the fix. Be succint.

## Matching Upstream Conventions

This is a fork of `apache/datafusion-sqlparser-rs` and changes here are expected to go upstream. Everything an agent produces — code, comments, commit messages, PR descriptions, tests — should be indistinguishable in style and length from what is already in the repository. Upstream reviewers reject verbose contributions, and LLM agents default to writing far more prose than this project uses. When in doubt, read three neighbouring commits or functions and match them.

1. **Comments**: explain a non-obvious *why* in one or two lines, then stop. A typical upstream parser fix adds 5–7 comment lines in total. Do not narrate what the code does, restate the commit message, or leave a design essay above a function.
2. **Commit messages**: subject line in the existing style (`<Dialect>: <short description>`, or `Parser: <short description>` for dialect-independent parser changes). Most upstream commits have no body at all; the longest run to about 20 lines. If a body is warranted, state the bug, a minimal example, and the fix — nothing else.
3. **PR descriptions**: same budget as the commit body. See the Pull Request Guidelines above.
4. **Tests**: follow the Unit Tests Guidelines above and match the assertion style of the tests already in the file. Add the cases that pin the behaviour and no more; a long table of near-duplicate inputs is noise. Do not add commentary to a test that the test name and SQL already convey.
5. **No ticket references or internal identifiers** in code, comments, tests, or commit messages. Firebolt ticket IDs (`FB-XXXX`) are meaningless upstream. Describe the problem instead.
6. **Do not restate measurements you did not take.** Timings, call counts and "all N dialects affected" claims belong in a commit body only if you measured them; keep them to one clause.

### Reviewing Agents

Review comments follow the same budget as everything else — a few sentences, pointing at the specific line and the specific consequence.

1. Do not summarise the diff back to the author, restate the PR description, or open with praise.
2. One comment per real issue. Skip style nits that `cargo fmt` and `cargo clippy` already enforce.
3. Say plainly whether a finding is a correctness bug, a behaviour change, or a suggestion — and if it is a behaviour change, name an input whose result changes.
4. If a review finding is right, fix it and say so in one or two sentences. Do not write a post-mortem.
