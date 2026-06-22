use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Fetch the input for a given year and day
    Fetch {
        /// The year to fetch
        year: u32,
        /// The day to fetch
        day: u32,
    },
    /// Run the solution for a given year and day
    Run {
        /// The year to run
        year: u32,
        /// The day to run
        day: u32,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Fetch { year, day } => {
            println!("Fetching year {} day {}", year, day);
        }
        Commands::Run { year, day } => {
            println!("Running year {} day {}", year, day);
        }
    }
}
