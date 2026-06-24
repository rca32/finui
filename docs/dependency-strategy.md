# Dependency Strategy

Finui is currently pre-release. Downstream applications should prefer a git
dependency pinned to a commit until the crates are published.

Recommended pre-release dependency shape:

```toml
finui-primitives = { git = "https://github.com/rca32/finui", rev = "<commit>" }
finui-grid = { git = "https://github.com/rca32/finui", rev = "<commit>" }
```

After the public API stabilizes, downstream applications can move to crates.io
versions for `finui-primitives` and `finui-grid`.
