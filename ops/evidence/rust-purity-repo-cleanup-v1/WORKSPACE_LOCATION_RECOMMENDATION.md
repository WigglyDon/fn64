# Workspace Location Recommendation

Recommendation: `KEEP_RUST_WORKSPACE_UNDER_RUST`

Evidence:

- `rust/` is the current Cargo workspace and sole product implementation.
- The concurrent boot lane owns Rust product source, so moving manifests or
  source would create direct overlap.
- The required gate resolves the workspace from `rust/` already.
- A root move would change commands, paths, manifests, tooling, context, and
  integration without earning new product truth.
- Repository purity is an ownership property; it does not require immediate
  physical relocation.

No workspace manifest, lockfile, crate manifest, forwarding metadata, symlink,
wrapper, or compatibility path changed. Revisit location only under a future
explicit product decision with concrete path pressure and no concurrent product
owner.
