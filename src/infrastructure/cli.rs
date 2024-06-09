use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "myapp",
    version = "1.0",
    author = "Author Name <author@example.com>",
    about = "An example CLI with nested subcommands"
)]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,

    #[arg(short, long, value_name = "VALUE")]
    pub config: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    Start,
    /*
    Start {
        #[arg(short, long, value_name = "VALUE")]
        config: Option<String>,
    },
     */
    User(UserCommandsParser),
}

// Start commands

#[derive(Parser)]
pub struct StartCommand {
    #[arg(short, long, value_name = "VALUE")]
    pub(crate) username: String,
}

// User commands

#[derive(Parser)]
pub struct UserCommandsParser {
    #[command(subcommand)]
    pub(crate) command: UserCommands,
}

#[derive(Subcommand)]
pub(crate) enum UserCommands {
    Create(CreateUserCommand),
    List,
    Delete(DeleteUserCommand),
}

#[derive(Parser)]
pub struct CreateUserCommand {
    #[arg(short, long, value_name = "VALUE")]
    pub(crate) username: String,

    #[arg(short, long, value_name = "VALUE")]
    pub(crate) email: String,
}

#[derive(Parser)]
struct ListUsersCommand {}

#[derive(Parser)]
pub struct DeleteUserCommand {
    pub(crate) username: String,
}
