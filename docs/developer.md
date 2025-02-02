# Debugging / Development information

This is just a loose list of things that can come in useful for debugging and development. This is on top of anything written in the [setup](./setup.md) documentation.

## Use latest stable Rust

OPCUA for Rust will always track quite close to the most stable version of Rust, therefore ensure your toolchain is kept up to date.

## Review / Acceptance Criteria

All code that is submitted via merge request / pull is required to be:

a) Developed in the style of the existing code.
b) Formatted with Rustfmt.
c) Extensively tested via unit tests and if necessary integration tests.
d) Serve a purpose related to the project, e.g. fix a bug, or add a feature required by the spec.

## Style guide

Some styling hints and design philosophy are provided in [design](./design.md).

### Rustfmt

Rustfmt will be used to format the sources and ensure a consistent style. Install rustfmt like so:

```
rustup component add rustfmt
```

Ensure you run `cargo fmt` on any changes you make. e.g.

```
cargo fmt --all
```

## CLion / RustRover

Note: JetBrains is deprecating Rust support in CLion (a bad idea IMO) for a standalone RustRover IDE. So this information is only for older installations of CLion.

CLion has very good Rust support. Install the `rust` and `toml` plugins and choose to use them with your existing Rust toolchain.

1. Enable "Use rustfmt instead of built-in formatter"
2. Enable "Run rustfmt on save"

If you are using RustRover then you get Rust out of the box but you need to ensure you use Rustfmt as your formatter. Using RustRover also prevents you from using some of the 3rd party samples and might also impact on some debugging scenarios, e.g. into OpenSSL code.

## Visual Studio Code

Visual Studio Code has pretty decent Rust support these days. You will need to install `rust-analyzer`, `Even Better TOML` extensions to make it work. You might also install `CodeLLDB` if you intend to debug.

## Wireshark

This is a useful link to follow about setting up [Wireshark for OPC UA](https://opcconnect.opcfoundation.org/2017/02/analyzing-opc-ua-communications-with-wireshark/). This allows you to capture network traffic and see how clients and servers are talking to each other. Wireshark has an OPC UA filter that decodes the binary traffic and tells you what requests and responses were being sent.

The only thing to add to the article is that most of the samples run on port 4855, so you should edit the settings for OPC UA and add port `4855` so that when you capture traffic and filter on `opcua` you see the port.