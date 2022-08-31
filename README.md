# Setup

Create a `.env` file with a discord token for the bot to use.

```ini
DISCORD_API_KEY = "<token>"
```

If you're using VSCode, you should install the recommended extensions. You can find these by typing `@reccomended` in the extensions search bar or running the `Show Recommended Extensions` command:
![](https://code.visualstudio.com/assets/docs/editor/extension-marketplace/recommendations.png)

# Building

This project is written in [Rust](https://rust-lang.org). You can install Rust from https://www.rust-lang.org/tools/install.

While editing the bot, use `cargo build` or `cargo run` for a fast build. Otherwise, use `cargo build --release` or `cargo run --release` for a release build.

# Cross-Compiling

The best way to cross-compile (for example, to run the bot on a Raspberry Pi) is [cross](https://github.com/cross-rs/cross). You can install cross with `cargo install cross`. It also requires [podman](https://podman.io), which you can install from https://podman.io/getting-started/installation.

Then you can use the `cross` command like `cargo`, for example using `cross build --release --target aarch64-unknown-linux-gnu` to build for a Raspberry Pi.
