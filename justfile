run:
    cargo fmt --all
    cargo run

build:
    cargo build --release

install: build
    sudo install -Dm 755 target/release/anid /usr/bin
    sudo install -Dm 644 anid.service /usr/lib/systemd/user
