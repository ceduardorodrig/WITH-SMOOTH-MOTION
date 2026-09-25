use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::{self, Command};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, SystemTime};
use signal_hook::consts::{SIGINT, SIGTERM};
use signal_hook::iterator::Signals;

fn log_msg(msg: &str) {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or_default();
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/with-smooth-motion.log")
    {
        let _ = writeln!(file, "[{}] {}", now, msg);
    }
}

fn get_direct_scanout() -> Option<u8> {
    if env::var("HYPRLAND_INSTANCE_SIGNATURE").is_err() {
        return None;
    }

    let output = Command::new("hyprctl")
        .args(["getoption", "render:direct_scanout"])
        .output()
        .ok()?;

    let text = String::from_utf8_lossy(&output.stdout);
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("int:") {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                return parts[1].parse::<u8>().ok();
            }
        }
    }
    None
}

fn set_direct_scanout(val: u8) {
    if env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok() {
        let code = format!("hl.config({{ render = {{ direct_scanout = {} }} }})", val);
        let _ = Command::new("hyprctl")
            .args(["eval", &code])
            .output();
        log_msg(&format!("render:direct_scanout set to {}", val));
    }
}

struct ScanoutGuard;

impl Drop for ScanoutGuard {
    fn drop(&mut self) {
        set_direct_scanout(2);
        log_msg("ScanoutGuard drop: scanout restored to 2");
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: with-smooth-motion <command> [args...]");
        process::exit(1);
    }

    log_msg(&format!("Starting command with Smooth Motion: {:?}", args));

    // 1. Force initial deactivation of direct scanout
    set_direct_scanout(0);
    let _guard = ScanoutGuard;

    let is_running = Arc::new(AtomicBool::new(true));
    let running_watchdog = Arc::clone(&is_running);

    // 2. Watchdog: monitors every 2s whether direct_scanout was reset by workspace switch or Hyprland reload
    thread::spawn(move || {
        while running_watchdog.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_secs(2));
            if !running_watchdog.load(Ordering::Relaxed) {
                break;
            }
            if let Some(val) = get_direct_scanout() {
                if val != 0 {
                    log_msg(&format!("Watchdog detected scanout reverted to {}. Re-applying 0...", val));
                    set_direct_scanout(0);
                }
            }
        }
    });

    // 3. Spawn child process with Smooth Motion enabled
    let program = &args[0];
    let program_args = &args[1..];

    let mut child = match Command::new(program)
        .args(program_args)
        .env("NVPRESENT_ENABLE_SMOOTH_MOTION", "1")
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            log_msg(&format!("Failed to spawn '{}': {}", program, e));
            eprintln!("Failed to execute '{}': {}", program, e);
            drop(_guard);
            process::exit(127);
        }
    };

    let child_pid = child.id();
    log_msg(&format!("Child process started with PID {}", child_pid));

    // 4. Capture graceful signals and forward them to child
    let running_signals = Arc::clone(&is_running);
    if let Ok(mut signals) = Signals::new([SIGINT, SIGTERM]) {
        thread::spawn(move || {
            if let Some(sig) = signals.into_iter().next() {
                log_msg(&format!("Received signal {}. Terminating child process PID {}...", sig, child_pid));
                running_signals.store(false, Ordering::SeqCst);
                // Send SIGTERM to child via kill
                let _ = Command::new("kill")
                    .args(["-15", &child_pid.to_string()])
                    .output();
            }
        });
    }

    // 5. Wait for child process to complete
    let status_res = child.wait();
    is_running.store(false, Ordering::SeqCst);

    match status_res {
        Ok(status) => {
            log_msg(&format!("Child process finished with status: {:?}", status));
            drop(_guard);
            if let Some(code) = status.code() {
                process::exit(code);
            } else {
                process::exit(1);
            }
        }
        Err(e) => {
            log_msg(&format!("Failed to wait for child process: {}", e));
            drop(_guard);
            process::exit(1);
        }
    }
}
