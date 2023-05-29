default:
    just -l

run:
    cargo run

    cargo build -p crt-core
    cargo build -p crt-alloc
    cargo build -p crt-std

    cargo build -p crt-runtime
    target/debug/crt-runtime

crt:
    target/debug/crt-runtime
