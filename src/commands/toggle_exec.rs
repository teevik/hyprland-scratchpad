use clap::Args;
use hyprland::data::Workspaces;
use hyprland::dispatch::{Dispatch, DispatchType};
use hyprland::prelude::*;
use std::thread;
use std::time::Duration;

/// Toggles an special workspace, and runs exec for the workspace if it isn't created
#[derive(Args)]
pub struct ToggleExecCommand {
    /// Name for the special workspace (without `special:`)
    #[arg(short, long)]
    name: String,

    /// The command to exec
    #[arg(short, long)]
    exec: String,

    /// The amount of retries for trying to toggle the special workspace, runs with a 100ms interval
    #[arg(long, default_value_t = 30)]
    max_retries: u32,
}

const TIMEOUT: Duration = Duration::from_millis(100);

impl ToggleExecCommand {
    pub fn run(self) -> anyhow::Result<()> {
        let Self {
            name,
            exec,
            max_retries,
        } = self;

        let target_workspace_name = format!("special:{name}");

        let workspaces = Workspaces::get()?;
        let workspace_is_spawned = workspaces
            .iter()
            .any(|workspace| workspace.name == target_workspace_name);

        if !workspace_is_spawned {
            let exec_command = format!("[workspace {target_workspace_name}] {exec}");

            Dispatch::call(DispatchType::Exec(&exec_command))?;

            for _ in 0..max_retries {
                thread::sleep(TIMEOUT);

                let workspaces = Workspaces::get()?;
                let is_spawned = workspaces
                    .iter()
                    .any(|workspace| workspace.name == target_workspace_name);

                if is_spawned {
                    break;
                }
            }
        }

        Dispatch::call(DispatchType::ToggleSpecialWorkspace(Some(name)))?;

        Ok(())
    }
}
