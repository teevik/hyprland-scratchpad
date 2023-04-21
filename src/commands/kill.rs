use clap::Args;
use hyprland::data::Clients;
use hyprland::dispatch::{Dispatch, DispatchType, WindowIdentifier};
use hyprland::prelude::*;

/// Kills all clients in a special workspace
#[derive(Args)]
pub struct KillCommand {
    /// The special workspace to kill (without `special:`)
    name: String,
}

impl KillCommand {
    pub fn run(self) -> anyhow::Result<()> {
        let Self { name } = self;

        let clients = Clients::get()?.collect();

        for client in clients {
            if client.workspace.name == format!("special:{name}") {
                println!("Killing {:?}", client);

                Dispatch::call(DispatchType::CloseWindow(WindowIdentifier::ProcessId(
                    client.pid,
                )))?;
            }
        }

        Ok(())
    }
}
