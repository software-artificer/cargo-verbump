# cargo-verbump
A small CLI tool to update version inside of the `Cargo.toml` file. Useful for
CI pipelines.

## Examples
Update the version to `0.1.0` in the `Cargo.toml` file in the current
directory:

```console
garry@pond $ cargo-verbump 0.1.0
```

Update the version to `1.0.1` in the `/foo/bar/Cargo.toml` file:
```console
garry@pond $ cargo-verbump -p /foo/bar/Cargo.toml 1.0.1
```

## Installation

### From binary
Simply download the archive with prebuilt binary from the respective release
page, unpack and use on your system.

### Via cargo install
```
cargo install --tag v0.0.1 --git https://github.com/software-artificer/cargo-verbump
```

### From source
1. Clone the repository
2. Use `cargo run` to run directly, or `cargo build` to produce a binary.
