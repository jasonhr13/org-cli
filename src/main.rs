use clap::{Parser, Subcommand};
use aws_sdk_ec2 as awsec2;

mod commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}


#[derive(Subcommand)]
enum Commands {
    /// allows ssh into AWS instances
    ListServers,
    /// ssh into specific server
    Ssh {
        server_name: String,
        #[arg(
            short,
            long,
            required = false,
            default_value_t = false,
            help = "Specify searching production servers."
        )]
        production: bool,
        #[arg(
            short,
            required = false,
            default_value_t = false,
            help = "specify us-west-2a"
        )]
        a: bool,
        #[arg(
            short,
            required = false,
            default_value_t = false,
            help = "specify us-west-2b."
        )]
        b: bool,
    },
}

#[tokio::main]
async fn main() -> Result<(), awsec2::Error> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::ListServers) => {
            commands::ssh_all().await;
            Ok(())
        }
        Some(Commands::Ssh { server_name, production, a, b}) => {
            commands::ssh(server_name.to_string(), *production, *a, *b).await;
            Ok(())
        }
        None => {Ok(())}
    }
}
