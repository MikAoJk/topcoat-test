# topcoat-test
Testing https://github.com/tokio-rs/topcoat
https://github.com/tokio-rs/topcoat/blob/main/docs/manual_setup.md

### Prerequisites
Install the following tools and verify that they are available:

| Tool | Check installation | Installation guide |
| --- | --- | --- |
| Rust | `bash rustc --version` | [Rust setup](https://rust-lang.org/tools/install/) |
| Cargo | `cargo --version` | [Rust setup](https://rust-lang.org/tools/install/)  |
> [!NOTE]
> Rust minimum version that is required se [Cargo.toml](Cargo.toml#L5)

## Run the development server

Install the Topcoat CLI for automatic rebuilds:

```bash
cargo install topcoat-cli
```

Make sure Cargo's binary directory is on your `PATH` so you can run `topcoat`. You can also invoke it as `cargo topcoat`.

Stop the app if it is still running, then run this from the project root:

```bash
topcoat dev
```

The dev server builds and starts the app, then rebuilds it when source files change. Pages that include `topcoat::dev::script()` update when the new build is ready. Press `r` in the terminal to rebuild manually.

For ways to reduce rebuild times, see the Cargo book's [build performance chapter](https://doc.rust-lang.org/cargo/guide/build-performance.html).