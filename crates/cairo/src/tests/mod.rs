use std::{env, path::PathBuf};

use crate::{cairo_compile, compute_hash};

#[tokio::test]
async fn get_hash_test() {
    let cairo_file_path =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR env not present"))
            .join("src/tests/simple.cairo");

    let compiled_cairo = cairo_compile(cairo_file_path).await.unwrap();
    let hash = compute_hash(compiled_cairo.path().to_path_buf())
        .await
        .unwrap();

    assert!(hex::encode(hash) == "479cf5e104346d5f90ffc7cd80fae23591a3ecb130d0d20170a430f5f4ade7");
}
