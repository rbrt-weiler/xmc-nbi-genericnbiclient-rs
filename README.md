# XMC NBI GenericNbiClient (Rust)

GenericNbiClient sends a query to the GraphQL-based API provided by the Northbound Interface (NBI) of [Extreme Management Center (XMC)](https://www.extremenetworks.com/product/extreme-management-center/) and prints the raw JSON response to stdout.

## Branches

This project uses two defined branches:

  * `master` is the primary development branch. Code within `master` may be broken at any time.
  * `stable` is reserved for code that compiles without errors and is tested. Track `stable` if you just want to use the software.

Other branches, for example for developing specific features, may be created and deleted at any time.

## Compiling

Use `cargo run` to build (with debug information) and run the tool directly or `cargo build --release` to compile an optimized binary without debug information.

Tested with cargo 1.34.0 / rustc 1.34.2.

## Usage

`xmc-nbi-genericnbiclient-rs -h` (either in ./target/debug/ or ./targt/release/):

<pre>
USAGE:
    xmc-nbi-genericnbiclient-rs [FLAGS] [OPTIONS] --host &lt;STRING&gt;

FLAGS:
    -h, --help             Prints help information
    -I, --insecurehttps    Do not validate HTTPS certificates
    -V, --version          Prints version information

OPTIONS:
    -H, --host &lt;STRING&gt;        XMC Hostname / IP
    -T, --httptimeout &lt;INT&gt;    Timeout for HTTP(S) connections [default: 5]
    -W, --password &lt;STRING&gt;    Password for HTTP auth [default: ]
    -P, --port &lt;INT&gt;           HTTP port where XMC is listening [default: 8443]
    -Q, --query &lt;STRING&gt;       GraphQL query to send to XMC [default: query { network { devices { up ip sysName nickName
                               } } }]
    -U, --username &lt;STRING&gt;    Username for HTTP auth [default: admin]
</pre>

## Source

The original project is [hosted at GitLab](https://gitlab.com/rbrt-weiler/xmc-nbi-genericnbiclient-rs), with a [copy over at GitHub](https://github.com/rbrt-weiler/xmc-nbi-genericnbiclient-rs) for the folks over there. Additionally, there is a project at GitLab which [collects all available clients](https://gitlab.com/rbrt-weiler/xmc-nbi-clients).
