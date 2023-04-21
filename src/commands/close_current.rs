use clap::Args;
use hyprland::data::Client;
use hyprland::dispatch::{Dispatch, DispatchType};
use hyprland::prelude::*;

/// Closes a special workspace if inside one
#[derive(Args)]
pub struct CloseCurrentCommand;

impl CloseCurrentCommand {
    pub fn run(self) -> anyhow::Result<()> {
        let active_client = Client::get_active()?;
        let Some(active_client) = active_client else { return Ok(()) };

        if let Some(name) = active_client.workspace.name.strip_prefix("special:") {
            Dispatch::call(DispatchType::ToggleSpecialWorkspace(Some(name)))?;
        }

        Ok(())
    }
}
