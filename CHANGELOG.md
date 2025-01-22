# Changelog

## [0.14.0] - 2025-01-22

First release of the async-opcua library. Version number picks up where this forked from opcua. This changelog is almost certainly incomplete, the library has in large part been rewritten.

### Common

#### Changed
 - The libraries are now named `async-opcua-*`. The root module is still `opcua`. Do not use this together with the old opcua library.
 - `ExtensionObject` is now stored as an extension of `dyn Any`.
 - We no longer depend on OpenSSL, all crypto is now done with pure rust crates.
 - Generated types and address space now targets OPC-UA version 1.05.
 - The library is separated into multiple crates. Most users should still just depend on the `async-opcua` crate with appropriate features.
 - A number of minor optimizations in the common comms layer.

#### Added
 - `async-opcua-xml`, a library for parsing a number of OPC-UA XML structures. Included in `async-opcua` if you enable the `xml` feature.
 - `async-opcua-macros`, a common macro library for `async-opcua`. Macros are re-exported depending on enabled features.
 - Basic support for custom structures.
 - Much more tooling around generated code, enough that it should be possible to implement a companion standard using the same tooling that generates the core address space. See [samples/custom-codegen](samples/custom-codegen).

#### Fixed
 - A number of deviations from the standard and other bugs related to generated types.
 - A few common issues in encoding, and opc/tcp.
 - Generated certificates are now fully compliant with the OPC-UA standard.

### Server

#### Changed
 - The server library is rewritten from scratch, and has a completely new interface. Instead of defining a single `AddressSpace` and simply mutating that, servers now define a number of `NodeManager`s which may present parts of the address space in different ways. The closest equivalent to the old behavior is adding a `SimpleNodeManager`. See [docs/server.md](docs/server.md) for details.
 - The server no longer automatically samples data from nodes. Instead, you must `notify` the server of changes to variables. The `SyncSampler` type can be used to do this with sampling, and the `SimpleNodeManager` does this automatically.
 - The server is now fully async, and does not define its own tokio runtime.

#### Added
 - It is now possible to define servers that are far more flexible than before, including storing the entire address space in databases or external systems, using notification-based mechanisms for notifications, etc.
 - Tools for managing the server runtime, including graceful shutdown notifying clients, tools for managing the service level, and more.

#### Removed
 - The web interface for the server has been completely removed.

### Client

#### Changed
 - The client is now fully async, and does not define its own tokio runtime. All services are async.

#### Added
 - The client is now able to efficiently restore subscriptions on reconnect. This can be turned off.
 - There are a few more configuration options.
 - A flexible system for request building, making it possible to automatically retry OPC-UA services.
 - A builder-pattern for creating OPC-UA connections, making the connection establishment part of the client more flexible.