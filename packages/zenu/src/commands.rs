use clap::{Args, Subcommand};

use crate::mongo_client::{list_databases, truncate_outbound_requests_results};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ZenuArgs {
    // Taken from https://github.com/clap-rs/clap/blob/master/examples/git-derive.rs
    #[command(subcommand)]
    commands: ZenuCommands,
}

#[derive(Debug, Subcommand)]
enum ZenuCommands {
    Test,
    ListDatabases,
    TruncateOutboundRequests,
}

pub async fn run_zenu_command(args: ZenuArgs) {
    match args.commands {
        ZenuCommands::Test => {
            println!("Zenu test command");
        }
        ZenuCommands::ListDatabases => {
            list_databases().await.expect("Failed to list databases");
        }
        ZenuCommands::TruncateOutboundRequests => {
            truncate_outbound_requests_results()
                .await
                .expect("Failed to truncate outbound requests results");
        }
    }
}
