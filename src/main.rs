use std::env::args;
use std::fs;
use std::io::{stdout, Write};

use wit_component::ComponentEncoder;

fn main() {
    let mut argv = args();

    let module = match (argv.next(), argv.next(), argv.next()) {
        (Some(exe), Some(module), None) => Some(fs::read(module).expect("failed to read module")),
        (Some(exe), None, None) => None,
        _ => panic!("argv[0] not set"),
    };

    let buf = ComponentEncoder::default()
        .validate(true)
        .module(
            module
                .as_deref()
                .unwrap_or(include_bytes!(env!("CARGO_CDYLIB_FILE_GUEST"))),
        )
        .expect("failed to set core component module")
        .adapter(
            "wasi_snapshot_preview1",
            include_bytes!(env!("CARGO_CDYLIB_FILE_WASI_SNAPSHOT_PREVIEW1")),
        )
        .expect("failed to add WASI adapter")
        .encode()
        .expect("failed to encode a component");

    stdout()
        .write(&buf)
        .expect("failed to write component to stdout");
}
