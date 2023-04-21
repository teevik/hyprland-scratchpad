use clap::Args;
use hyprland::data::Workspaces;
use hyprland::prelude::*;

/// Lists all special workspaces
#[derive(Args)]
pub struct ListCommand;

impl ListCommand {
    pub fn run(self) -> anyhow::Result<()> {
        let workspaces = Workspaces::get()?.collect();

        for workspace in workspaces {
            if workspace.name.starts_with("special:") {
                println!("{:?}", workspace);
            }
        }

        Ok(())
    }
}
