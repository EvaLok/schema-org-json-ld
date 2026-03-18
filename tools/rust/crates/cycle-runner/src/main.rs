use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod close_out;
mod git;
mod review_body;
mod runner;
mod startup;
mod steps;

#[derive(Parser, Debug)]
#[command(name = "cycle-runner", about = "Top-level orchestrator cycle automation")]
struct Cli {
    /// Repository root path
    #[arg(long, default_value = ".", global = true)]
    repo_root: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run startup sequence: cycle-start, pipeline-check, housekeeping, status
    Startup {
        /// Orchestrator issue number
        #[arg(long)]
        issue: u64,

        /// Model name override (defaults to tools/config.json)
        #[arg(long)]
        model: Option<String>,

        /// Show what would run without executing
        #[arg(long)]
        dry_run: bool,
    },
    /// Run close-out sequence: C4.1 through C8
    CloseOut {
        /// Orchestrator issue number
        #[arg(long)]
        issue: u64,

        /// Cycle number override (reads from state.json if omitted)
        #[arg(long)]
        cycle: Option<u64>,

        /// Show what would run without executing
        #[arg(long)]
        dry_run: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Commands::Startup {
            issue,
            model,
            dry_run,
        } => startup::run(&cli.repo_root, issue, model.as_deref(), dry_run),
        Commands::CloseOut {
            issue,
            cycle,
            dry_run,
        } => close_out::run(&cli.repo_root, issue, cycle, dry_run),
    };
    if let Err(error) = result {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn help_text_includes_subcommands() {
        let mut cmd = Cli::command();
        let mut output = Vec::new();
        cmd.write_long_help(&mut output).unwrap();
        let help = String::from_utf8(output).unwrap();
        assert!(help.contains("startup"));
        assert!(help.contains("close-out"));
    }

    #[test]
    fn startup_help_includes_flags() {
        let cmd = Cli::command();
        let startup = cmd
            .get_subcommands()
            .find(|c| c.get_name() == "startup")
            .expect("startup subcommand should exist");
        let mut output = Vec::new();
        startup.clone().write_long_help(&mut output).unwrap();
        let help = String::from_utf8(output).unwrap();
        assert!(help.contains("--issue"));
        assert!(help.contains("--dry-run"));
    }

    #[test]
    fn close_out_help_includes_flags() {
        let cmd = Cli::command();
        let close_out = cmd
            .get_subcommands()
            .find(|c| c.get_name() == "close-out")
            .expect("close-out subcommand should exist");
        let mut output = Vec::new();
        close_out.clone().write_long_help(&mut output).unwrap();
        let help = String::from_utf8(output).unwrap();
        assert!(help.contains("--issue"));
        assert!(help.contains("--cycle"));
        assert!(help.contains("--dry-run"));
    }
}
