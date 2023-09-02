use anyhow::Context;
use clap::Parser;
use discord_webhook_cli::WebhookData;
use dotenv::dotenv;
use tracing::{info, trace};

#[derive(Parser, Debug)]
struct CliArgs {

	#[arg(long, env = "DISCORD_WEBHOOK_CLI_URL")]
	url: String,

	#[arg(short, long, env = "DISCORD_WEBHOOK_CLI_MSG")]
	msg: String,

	#[arg(short, long, env = "DISCORD_WEBHOOK_CLI_USERNAME")]
	username: Option<String>,
}

impl From<CliArgs> for WebhookData {
	fn from(value: CliArgs) -> Self {
		Self {
			content: value.msg,
			username: value.username,
		}
	}
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	tracing_subscriber::fmt::init();
	dotenv().ok();
	human_panic::setup_panic!();

	panic!("WHAT IS GOIN GON?");

	info!("Beginning ...");

	trace!("Parsing args ...");
	let args = CliArgs::parse();
	trace!("Args parsed: {:?}", &args);
	let url = args.url.clone();

	let client = reqwest::Client::new();
	let data: WebhookData = args.into();

	trace!("Sending webhook ...");
	let response = client.post(url).json(&data).send().await?;
	trace!("Webhook sent with response: {:?}", response);

	Ok(())
}
