# NodeCraft templates

Original, versioned workflow examples for [NodeCraft](https://github.com/CypherNaught-0x/nodecraft).
Each ordinary `.ncraft` archive includes its own source assets. Adding a template never runs a
model; a later paid run follows NodeCraft's normal quote, budget and consent flow.

## License and authors

Content, original fixtures and the data package are **MIT OR Apache-2.0**, at your option.
Contributions use the same terms. Keep LICENSE-MIT, LICENSE-APACHE, NOTICE and any content-specific
attribution when redistributing. Initial authors and source history are preserved in Git;
[SOURCE.md](SOURCE.md) records the original NodeCraft revision and migration.
[fixtures/README.md](fixtures/README.md) describes the original media and reproduction recipe.
No vendor workflows, prompts or media were copied.

## Layout

- `index.json`: the current catalog, with mandatory `schema_version: 1` and one current record
  per unique template ID. Each record has id, title, positive version, description, feature tags,
  initial canvas ID, BLAKE3 digest and immutable archive path.
- `templates/<id>/<version>/<blake3>.ncraft`: directly openable projects. Published bytes and
  versions are immutable; keep old paths available when publishing a new version.
- `src/lib.rs`: dependency-free static index/archive bytes, explicitly maintained with the index.
  NodeCraft pins a full Git revision for its compiled offline fallback. No build script downloads
  or generates content. Updates to this catalog can be fetched independently by a running app.
- `fixtures/`: original media used by the offline author/semantic tests.

## Authoring and validation

Start from a small original public-safe project. Use a new lowercase/digit/hyphen/underscore ID,
short title/description and useful feature tags. Give each version distinct graph/canvas IDs;
never reuse another template's IDs. Include all assets, no credentials, private paths or account
information. Describe what the example does and any configured-provider/native-resource needs.

The client, author validation and CI use the **same NodeCraft production index/archive validator**.
The pinned validator source revision is recorded in `.github/workflows/validate.yml`; validation
is not a second implementation of the schema. In a checkout of that exact NodeCraft revision:

```sh
TEMPLATE953_CATALOG_ROOT=/absolute/path/to/nodecraft-templates \
  CARGO_INCREMENTAL=0 cargo test -p nodecraft-engine --test template_catalog -- --include-ignored
```

This validates the index version/bounds, every digest and ordinary archive, graph/asset integrity
and installed node kinds. Its negative tests prove unsupported schema and bad digest rejection.
Run `CARGO_INCREMENTAL=0 cargo test --offline` here as well: it checks that this data package's
compiled table contains the same archive files and index bytes. Neither test calls a provider,
reads credentials or contacts ComfyUI. NodeCraft keeps semantic mock/native execution tests for
the pinned offline snapshot.

## Publishing an update

1. Branch and change an example; increment its positive version when its archive changes.
2. Place the archive at its new immutable path; compute BLAKE3 from those exact archive bytes.
   Update the index record, metadata and static byte table together. Do not remove older archives.
3. Run the exact pinned validator and package checks above. Review the actual graph and media;
   use safe examples and add attribution for any permissible external contribution.
4. Open a pull request. CI validates the complete current index and checks that published
   `(id, version)` contents were not replaced. Merge only after checks/review pass.
5. The default branch index publishes the update to clients; clients fetch it first and retrieve
   only a selected missing archive. An unreachable or invalid update leaves their bundled or
   previously verified version usable. No app release is required for catalog updates.
6. To update a future app's compiled offline set, make a separate NodeCraft PR that changes its
   immutable template-data Git pin/lockfile and passes its full gates. Do not use a branch pin.

Do not turn this repository into an arbitrary third-party plugin or personal-media library.
