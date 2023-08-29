use once_cell::sync::Lazy;
use std::sync::Mutex;
use sysinfo::{ProcessExt, System, SystemExt};

static SYSTEM: Mutex<Lazy<System>> = Mutex::new(Lazy::new(|| System::new_all()));

pub fn vrchat_exist() -> bool {
    let mut system = SYSTEM.lock().unwrap();
    system.refresh_all();
    let processes = system.processes();
    for (_pid, process) in processes {
        if process.name().contains("VRChat") {
            return true;
        }
    }
    false
}
