//! Table/file agreement only; schema and archive validation live in NodeCraft itself.
use std::{collections::BTreeSet, path::Path};

#[test]
fn compiled_snapshot_matches_the_current_index_files() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert_eq!(
        std::fs::read(root.join("index.json")).unwrap(),
        nodecraft_template_data::INDEX.as_bytes()
    );
    let mut compiled = BTreeSet::new();
    for (name, bytes) in nodecraft_template_data::FILES {
        assert!(compiled.insert((*name).to_owned()), "duplicate static file");
        assert_eq!(std::fs::read(root.join(name)).unwrap(), *bytes);
    }
    // This extracts filenames only: the pinned NodeCraft validator owns all schema rules.
    let index: serde_json::Value = serde_json::from_str(nodecraft_template_data::INDEX).unwrap();
    let listed: BTreeSet<String> = index["templates"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| entry["file"].as_str().unwrap().to_owned())
        .collect();
    assert_eq!(
        compiled, listed,
        "keep the static offline set in step with the current index"
    );
    // Older immutable archives remain in Git without inflating the compiled current set.
}
