# Publishing to crates.io

The Rust SDK is published as the single `frontal-sdk` package. Before the first release, complete the runtime implementation, establish and document the supported Rust version, update the changelog, and review the generated contract matrix.

Run `cargo publish --dry-run` for the root package, create the matching protected Git tag, and publish the `frontal-sdk` package. Keep credentials in protected repository secrets and use the registry's recommended signing or trusted-publishing mechanism where available.
