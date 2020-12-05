# XMC NBI GenericNbiClient (Rust)

GenericNbiClient sends a query to the GraphQL-based API provided by the Northbound Interface (NBI) of [Extreme Management Center (XMC)](https://www.extremenetworks.com/product/extreme-management-center/) and prints the raw JSON response to stdout.

**Notice**: This project has surfaced as a proof of concept and is _not maintained_ anymore. If you need a production ready NBI client, please refer to [the Go version of GenericNbiClient](https://gitlab.com/rbrt-weiler/xmc-nbi-genericnbiclient-go).

## Branches

This project uses two defined branches:

* `master` is the primary development branch. Code within `master` may be broken at any time.
* `stable` is reserved for code that compiles without errors and is tested. Track `stable` if you just want to use the software.

Other branches, for example for developing specific features, may be created and deleted at any time.

## Compiling

Use `cargo run` to build (with debug information) and run the tool directly or `cargo build --release` to compile an optimized binary without debug information.

Tested with cargo 1.48.0 / rustc 1.48.0.

## Usage

`xmc-nbi-genericnbiclient-rs -h` (either in ./target/debug/ or ./targt/release/):

```text
USAGE:
    xmc-nbi-genericnbiclient-rs [FLAGS] [OPTIONS] --host <STRING>

FLAGS:
    -h, --help             Prints help information
    -I, --insecurehttps    Do not validate HTTPS certificates
    -V, --version          Prints version information

OPTIONS:
    -H, --host <STRING>        XMC Hostname / IP
    -T, --httptimeout <INT>    Timeout for HTTP(S) connections [default: 5]
    -W, --password <STRING>    Password for HTTP auth [default: ]
    -P, --port <INT>           HTTP port where XMC is listening [default: 8443]
    -Q, --query <STRING>       GraphQL query to send to XMC [default: query { network { devices { up ip sysName nickName
                               } } }]
    -U, --username <STRING>    Username for HTTP auth [default: admin]
```

## Source

The original project is [hosted at GitLab](https://gitlab.com/rbrt-weiler/xmc-nbi-genericnbiclient-rs), with a [copy over at GitHub](https://github.com/rbrt-weiler/xmc-nbi-genericnbiclient-rs) for the folks over there. Additionally, there is a project at GitLab which [collects all available clients](https://gitlab.com/rbrt-weiler/xmc-nbi-clients).
