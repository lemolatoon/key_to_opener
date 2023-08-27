use std::process::Command;

use tauri;

#[tauri::command]
pub async fn command1() {
    // vrchat://launch?ref=vrchat.com&id=wrld_736bad27-4663-4346-a345-26e1e859d94e:mosco~group(grp_55a159da-da85-4bf3-893d-65fc50abe6c1)~groupAccessType(public)~region(eu)&shortName=aacyru06
    // vrchat://launch?ref=vrchat.com&id=wrld_736bad27-4663-4346-a345-26e1e859d94e
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(&[
                "/C",
                "start",
                "vrchat://launch?id=wrld_736bad27-4663-4346-a345-26e1e859d94e",
            ])
            .spawn()
            .expect("failed to execute process");
    }
    #[cfg(not(target_os = "windows"))]
    {
        eprintln!("This OS is not supported yet.");
    }
}
