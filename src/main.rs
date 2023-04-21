mod commands;

use crate::commands::close_current::CloseCurrentCommand;
use clap::Parser;
use commands::kill::KillCommand;
use commands::kill_all::KillAllCommand;
use commands::list::ListCommand;
use commands::toggle_exec::ToggleExecCommand;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
enum Command {
    ToggleExec(ToggleExecCommand),
    List(ListCommand),
    Kill(KillCommand),
    KillAll(KillAllCommand),
    CloseCurrent(CloseCurrentCommand),
}

fn main() -> anyhow::Result<()> {
    let command = Command::parse();

    match command {
        Command::ToggleExec(toggle_exec_command) => toggle_exec_command.run(),
        Command::List(list_command) => list_command.run(),
        Command::Kill(kill_command) => kill_command.run(),
        Command::KillAll(kill_all_command) => kill_all_command.run(),
        Command::CloseCurrent(close_current_command) => close_current_command.run(),
    }
}
