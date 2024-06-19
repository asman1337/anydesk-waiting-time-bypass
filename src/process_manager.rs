use std::process::{Command, Stdio};

use log::{error, info};
use sysinfo::System;

pub struct ProcessManager;

pub enum ProcessAction {
    Kill,
    Start,
}

#[allow(dead_code)]
impl ProcessManager {
    pub fn kill_process(process_name: &str) {
        let mut system = System::new_all();
        system.refresh_all();

        for (pid, process) in system.processes() {
            if process.name() == process_name {
                info!("Killing process with name: {}, PID: {}", process_name, pid);
                if process.kill() {
                    info!(
                        "Successfully killed process: {}, PID: {}",
                        process_name, pid
                    );
                } else {
                    error!("Failed to kill process: {}, PID: {}", process_name, pid);
                }
            }
        }
    }

    pub fn start_process(command: &str, args: &[&str]) {
        info!("Starting process: {} with args: {:?}", command, args);
        match Command::new(command)
            .args(args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
        {
            Ok(mut child) => {
                info!("Successfully started process: {}", command);
                match child.wait() {
                    Ok(status) => info!("Process {} exited with status: {}", command, status),
                    Err(e) => error!("Failed to wait on process: {}. Error: {}", command, e),
                }
            }
            Err(e) => error!("Failed to start process: {}. Error: {}", command, e),
        }
    }

    #[allow(unused_doc_comments)]
    pub fn manage_anydesk(action: ProcessAction) {
        #[cfg(target_os = "windows")]
        let process_name = "AnyDesk.exe";

        #[cfg(target_os = "linux")]
        let process_name = "anydesk";

        #[cfg(target_os = "macos")]
        let process_name = "AnyDesk";

        match action {
            ProcessAction::Kill => Self::kill_process(process_name),
            /// Fixme - Start action needs proper implementation, We will implement later
            ProcessAction::Start => {
                #[cfg(target_os = "windows")]
                Self::start_process("AnyDesk.exe", &[]);

                #[cfg(target_os = "linux")]
                Self::start_process("anydesk", &[]);

                #[cfg(target_os = "macos")]
                Self::start_process("/Applications/AnyDesk.app/Contents/MacOS/AnyDesk", &[]);
            }
        }
    }

    pub fn run_with_admin(command: &str, args: &[&str]) {
        #[cfg(target_os = "windows")]
        {
            let status = Command::new("runas")
                .arg("/user:Administrator")
                .arg(format!("{} {}", command, args.join(" ")))
                .status()
                .expect("Failed to execute command with admin privileges");

            if status.success() {
                info!("Successfully executed command with admin privileges: {} {:?}", command, args);
            } else {
                error!("Failed to execute command with admin privileges: {} {:?}", command, args);
            }
        }

        #[cfg(target_os = "unix")]
        {
            let status = Command::new("sudo")
                .arg(command)
                .args(args)
                .status()
                .expect("Failed to execute command with admin privileges");

            if status.success() {
                info!("Successfully executed command with admin privileges: {} {:?}", command, args);
            } else {
                error!("Failed to execute command with admin privileges: {} {:?}", command, args);
            }
        }
    }
}