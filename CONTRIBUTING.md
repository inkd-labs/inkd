# Contributing to Inkd

Thanks for taking the time to open a patch. Inkd is a small focused
protocol, and we try to keep the review cycle tight.

## Development Loop

1. Fork the repository and create a branch off `main`.
2. Install the toolchain (Rust stable, Solana CLI, Anchor 0.30, Node 20).
3. Run `cargo check --workspace` before every push.
4. Run `cd sdk && npm install && npm test` before opening a pull request.
5. Keep commits small and focused. Use Conventional Commits (`feat:`, `fix:`, `refactor:`).

## Code Style

- Rust: `cargo fmt --all` and `cargo clippy --workspace --all-targets`.
- TypeScript: 2-space indentation, `strict: true`, no default exports.
- No emoji in code, tests, or commit messages.

## Pull Request Checklist

- [x] Branch rebased on latest `main`
- [x] `cargo check` passes
- [x] `npm test` passes under `sdk/`
- [x] New public functions carry doc comments
- [x] CHANGELOG entry added under the `Unreleased` section

## Bug Reports

Please include the commit hash, Solana cluster, and the minimum transaction
sequence that reproduces the issue.
