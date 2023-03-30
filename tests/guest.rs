use wit_component::ComponentEncoder;

#[test]
fn encode_guest() {
    assert!(ComponentEncoder::default()
        .validate(true)
        .module(include_bytes!(env!("CARGO_CDYLIB_FILE_GUEST")))
        .expect("failed to set core component module")
        .adapter(
            "wasi_snapshot_preview1",
            include_bytes!(env!("CARGO_CDYLIB_FILE_WASI_SNAPSHOT_PREVIEW1")),
        )
        .expect("failed to add WASI adapter")
        .encode()
        .is_ok())
}
