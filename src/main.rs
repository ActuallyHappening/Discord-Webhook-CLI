use anyhow::Context;
use clap::Parser;
use discord_webhook_cli::WebhookData;
use dotenv::dotenv;
use tracing::{info, trace};

#[derive(Parser, Debug)]
struct CliArgs {
	#[arg(short, long, )]
	username: Option<String>,

	#[arg(short, long)]
	msg: String,
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

	info!("Beginning ...");

	trace!("Parsing args ...");
	let args = CliArgs::parse();
	trace!("Args parsed: {:?}", args);


	trace!("Getting URL from environment variable {ENV_VAR} ...");
	const ENV_VAR: &str = "DISCORD_WEBHOOK_CLI_URL";
	let url = std::env::var(ENV_VAR).context(format!("URL not found in environment variable {ENV_VAR}"))?;
	trace!("Found URL from environment");

	let client = reqwest::Client::new();
	let data: WebhookData = args.into();

	trace!("Sending webhook ...");
	let response = client.post(&url).json(&data).send().await?;
	trace!("Webhook sent with response: {:?}", response);

	Ok(())
}
