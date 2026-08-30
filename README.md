# archivelist

List contents of tar, zip, and other archives.

## Install

```console
cargo build --release
sudo cp target/release/archivelist /usr/local/bin/
```

## Usage

```console
archivelist release.tar.gz
archivelist backup.zip
```

Output:

```
     1234  src/main.rs
     5678  README.md
      901  Cargo.toml
```
