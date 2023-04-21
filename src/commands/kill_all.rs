use clap::Args;
use hyprland::data::Clients;
use hyprland::dispatch::{Dispatch, DispatchType, WindowIdentifier};
use hyprland::prelude::*;

/// Kills all clients in all special workspaces (not `special`)
#[derive(Args)]
pub struct KillAllCommand;

impl KillAllCommand {
    pub fn run(self) -> anyhow::Result<()> {
        let clients = Clients::get()?.collect();

        for client in clients {
            if client.workspace.name.starts_with("special:") {
                println!("Killing {:?}", client);

                Dispatch::call(DispatchType::CloseWindow(WindowIdentifier::ProcessId(
                    client.pid,
                )))?;
            }
        }

        Ok(())
    }
}
