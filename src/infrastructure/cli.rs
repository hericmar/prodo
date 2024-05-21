use clap::{Arg, ArgMatches, Command, Parser, Subcommand};

/*
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(subcommand)]
    User: UserCommands,
}

pub struct UserCommands {
    #[clap(subcommand)]
    command: UserCommandsEnum,
}

enum UserCommandsEnum {
    Create,
    Read,
    Update,
    Delete,
}
 */

#[derive(Parser)]
#[command(name = "myapp", version = "1.0", author = "Author Name <author@example.com>", about = "An example CLI with nested subcommands")]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Start,

    User(UserCommandsParser),
}

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
