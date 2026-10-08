# Releasing

Run the **Publish crate** workflow from `v2.1` and select **patch**, **minor** or **major**.
The workflow calculates the next version, updates `Cargo.toml` and `package.json`, verifies
the crate, then pushes the release commit and its `vX.Y.Z` tag before publishing to crates.io.

For example, to publish the next patch release:

```sh
gh workflow run publish.yml --ref v2.1 -f bump=patch
```

A minor release resets the patch number to zero. A major release resets both minor and patch
numbers to zero. Both manifests must contain the same stable `X.Y.Z` version before releasing.

## Trusted Publishing setup

In the `tauri-nspanel` crate settings on crates.io, add a GitHub Actions trusted publisher with:

- Repository owner: `ahkohd`
- Repository: `tauri-nspanel`
- Workflow: `publish.yml`
- Environment: `crates-io`

The GitHub `crates-io` environment must allow the `v2.1` branch, where the workflow is dispatched.
No long-lived crates.io token or repository secret is needed. The workflow passes its temporary
Trusted Publishing token to Cargo through `CARGO_REGISTRY_TOKEN`.

## Retry a failed publish

Use **Re-run failed jobs** on the original workflow run. The publish job reuses the release tag
and verified lockfile from the successful preparation job, so it does not bump the version again.
Starting a new workflow run creates another version.

After publishing succeeds, a GitHub release can be created from the same tag:

```sh
gh release create vX.Y.Z --verify-tag --generate-notes
```
