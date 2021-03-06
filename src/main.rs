extern crate clap;
extern crate reqwest;

use clap::{Arg, App};
use reqwest::{Client, Error};
use std::time::Duration;

const TOOL_NAME:&str = "XMC NBI GenericNbiClient.rs";
const TOOL_VERSION:&str = "0.2.0";

fn main() -> Result<(), Error> {
	let args = App::new(TOOL_NAME)
		.version(TOOL_VERSION)
		.about("This tool queries the XMC API and prints the raw reply (JSON) to stdout.")
		.arg(Arg::with_name("host")
			.short("H")
			.long("host")
			.value_name("STRING")
			.help("XMC Hostname / IP")
			.takes_value(true)
			.required(true))
		.arg(Arg::with_name("port")
			.short("P")
			.long("port")
			.value_name("INT")
			.default_value("8443")
			.help("HTTP port where XMC is listening")
			.takes_value(true))
		.arg(Arg::with_name("httptimeout")
			.short("T")
			.long("httptimeout")
			.value_name("INT")
			.default_value("5")
			.help("Timeout for HTTP(S) connections")
			.takes_value(true))
		.arg(Arg::with_name("insecurehttps")
			.short("I")
			.long("insecurehttps")
			.help("Do not validate HTTPS certificates"))
		.arg(Arg::with_name("username")
			.short("U")
			.long("username")
			.value_name("STRING")
			.default_value("admin")
			.help("Username for HTTP auth")
			.takes_value(true))
		.arg(Arg::with_name("password")
			.short("W")
			.long("password")
			.value_name("STRING")
			.default_value("")
			.help("Password for HTTP auth")
			.takes_value(true))
		.arg(Arg::with_name("query")
			.short("Q")
			.long("query")
			.value_name("STRING")
			.default_value("query { network { devices { up ip sysName nickName } } }")
			.help("GraphQL query to send to XMC")
			.takes_value(true))
		.get_matches();

	let xmc_host = args.value_of("host").unwrap();
	let http_port:u16 = args.value_of("port").unwrap().parse().unwrap();
	let http_timeout:u16 = args.value_of("httptimeout").unwrap().parse().unwrap();
	let mut insecure_https = false;
	if args.is_present("insecurehttps") {
		insecure_https = true;
	}
	let http_username = args.value_of("username").unwrap();
	let http_password = args.value_of("password").unwrap();
	let xmc_query = args.value_of("query").unwrap();

	let api_url = format!("https://{}:{}/nbi/graphql", xmc_host, http_port);
	let http_user_agent = format!("{}/{}", TOOL_NAME, TOOL_VERSION);

	let client = Client::builder()
		.danger_accept_invalid_certs(insecure_https)
		.timeout(Duration::from_secs(http_timeout.into()))
		.build()?;
	let mut response = client.get(&api_url)
		.header("User-Agent", http_user_agent)
		.header("Accept", "application/json")
		.basic_auth(http_username, Some(http_password))
		.query(&[("query", xmc_query)])
		.send()?;

	println!("{}", response.text()?);

	Ok(())
}

