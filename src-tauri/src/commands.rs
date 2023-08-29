use anyhow::{self, Result};
use cfg_if::cfg_if;
use std::process::Command;
use tauri;

use crate::{notify, process::vrchat_exist};

mod world_id {
    pub const JAPAN_SHRINE: &str = "wrld_736bad27-4663-4346-a345-26e1e859d94e";
}

pub fn vrc_exec(world_id: &str) -> Result<()> {
    let scheme = format!("\"vrchat://launch?ref=vrchat.com?id={}\"", world_id);
    if vrchat_exist() {
        anyhow::bail!("VRChat is already running.");
    }
    cfg_if! {
        if #[cfg(windows)] {
            Command::new("powershell")
                .args(&[
                    "/C",
                    "start",
                    scheme.as_str(),
                ])
                .spawn()?;
        } else {
            anyhow::bail!("This OS is not supported yet.")
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn command1() {
    // vrchat://launch?ref=vrchat.com&id=wrld_736bad27-4663-4346-a345-26e1e859d94e:mosco~group(grp_55a159da-da85-4bf3-893d-65fc50abe6c1)~groupAccessType(public)~region(eu)&shortName=aacyru06

    if let Err(err) = vrc_exec(world_id::JAPAN_SHRINE) {
        if let Err(_err) = notify::default()
            .summary("Failed to exec vrchat")
            .body(&err.to_string())
            .show()
        {
            // explicitly ignore error here
        };
    };
}
