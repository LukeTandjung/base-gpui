# Contributing to Base GPUI

Thank you for contributing to Base GPUI. Bug reports, documentation improvements, component fixes, and new component work are welcome.

Base GPUI is under active development. Please discuss substantial API or architecture changes in an issue before investing in an implementation.

## Before you start

1. Search the existing issues and pull requests to avoid duplicate work.
2. Open an issue for a bug or proposed feature if one does not already exist.
3. For larger changes, describe the intended API, behavior, accessibility implications, and testing approach before writing code.

Small documentation corrections and narrowly scoped fixes do not require prior discussion.

## Development setup

The repository provides a Nix/devenv environment. If you use `direnv`, allow the environment from the repository root:

```sh
direnv allow
```

You may also use your own Rust environment. The project depends on GPUI from a pinned Git revision, so the first build may take some time.

Install the pre-commit hooks if you have [`pre-commit`](https://pre-commit.com/) available:

```sh
pre-commit install --hook-type pre-commit --hook-type commit-msg --hook-type pre-push
```

## Making changes

- Keep pull requests focused on one problem.
- Follow [CODE_STYLE.md](CODE_STYLE.md).
- Preserve the component architecture described in [docs/component-architecture.md](docs/component-architecture.md).
- Follow any component-specific `AGENTS.md` implementation notes.
- Prefer GPUI-native behavior over direct translations of React or DOM internals.
- Add or update tests for observable behavior changes and regressions.
- Update generated component documentation when the public API changes:

```sh
deno run --allow-read --allow-write scripts/generate-component-docs.mjs
```

Do not manually edit generated component guides when the source API is the authority.

## Validation

Run these checks before opening a pull request:

```sh
cargo fmt --check
ast-grep scan --report-style short
cargo check -p base-gpui
cargo test -p base-gpui
cargo clippy -p base-gpui --all-targets
```

If a full check fails for a reason unrelated to your change, mention the exact failure in the pull request and confirm whether it is reproducible on a clean `main` branch.

Changes to the showcase should also be checked from `site/`:

```sh
trunk build
```

## Commit messages

Use [Conventional Commits](https://www.conventionalcommits.org/):

```text
feat(slider): add range thumb alignment
fix(tabs): expose focused tab state
docs: clarify component architecture
```

Allowed types are `build`, `chore`, `ci`, `docs`, `feat`, `fix`, `perf`, `refactor`, `revert`, `style`, and `test`. Keep the first line at or below 100 characters.

## Pull requests

In the pull request description:

- link the related issue;
- explain the behavior before and after the change;
- describe the tests you added or ran;
- identify public API or accessibility implications;
- include screenshots or recordings for visual showcase changes.

Maintainers may request changes to keep APIs and architecture consistent across component families. Reviews should remain technical, specific, and respectful.

By contributing, you agree that your contribution is licensed under the repository's [MIT License](LICENSE).
