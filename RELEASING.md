# Releasing

Crate releases are published to [crates.io](https://crates.io/crates/tauri-nspanel)
from an exact `vX.Y.Z` tag. GitHub releases use the same tag.

## Initial crates.io release

Trusted Publishing can only be configured after the crate exists. Publish `2.1.0` once from a
clean checkout of the merged `v2.1` branch using a crates.io owner account:

```sh
cargo publish --locked --dry-run
cargo login
cargo publish --locked
```

After crates.io has indexed the crate, tag the same merged commit and create its GitHub release:

```sh
git tag -s v2.1.0 -m "v2.1.0"
git push origin v2.1.0
gh release create v2.1.0 --verify-tag --generate-notes
```

Publishing is permanent: an uploaded version cannot be overwritten or deleted. Confirm the
package contents with `cargo package --list` before the non-dry-run command.

## Configure Trusted Publishing

In the `tauri-nspanel` crate settings on crates.io, add a GitHub Actions trusted publisher with:

- Repository owner: `ahkohd`
- Repository: `tauri-nspanel`
- Workflow: `publish.yml`
- Environment: `crates-io`

Create a `crates-io` environment in the GitHub repository settings and protect it with required
reviewers. No long-lived crates.io token or repository secret is needed.

## Later releases

1. Update the version in `Cargo.toml` and merge the release changes into `v2.1`.
2. Create and push a signed `vX.Y.Z` tag for that merged commit.
3. Run the **Publish crate** workflow on that tag. Enter `X.Y.Z` and enable **publish**.
4. After the workflow succeeds, create the GitHub release from the same tag.

The workflow always runs `cargo publish --locked --dry-run` first. A real publish is rejected
unless the selected ref is the matching version tag.
