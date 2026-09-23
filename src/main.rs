use std::env;
use std::process::{self, Command};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use signal_hook::consts::{SIGHUP, SIGINT, SIGTERM};
use signal_hook::iterator::Signals;

fn set_direct_scanout(val: u8) {
    if env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok() {
        let code = format!("hl.config({{ render = {{ direct_scanout = {} }} }})", val);
        let _ = Command::new("hyprctl")
            .args(["eval", &code])
            .output();
    }
}

struct ScanoutGuard;

impl Drop for ScanoutGuard {
    fn drop(&mut self) {
        set_direct_scanout(2);
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Uso: with-smooth-motion <comando> [argumentos...]");
        process::exit(1);
    }

    // 1. Suspende direct scanout no Hyprland durante a execução do jogo
    set_direct_scanout(0);
    let _guard = ScanoutGuard;

    // 2. Garante restauração caso o processo receba sinal (SIGINT, SIGTERM, SIGHUP)
    let terminated = Arc::new(AtomicBool::new(false));
    let term_clone = Arc::clone(&terminated);

    if let Ok(mut signals) = Signals::new([SIGINT, SIGTERM, SIGHUP]) {
        thread::spawn(move || {
            if let Some(_sig) = signals.into_iter().next() {
                set_direct_scanout(2);
                term_clone.store(true, Ordering::SeqCst);
                process::exit(130);
            }
        });
    }

    // 3. Executa o jogo/aplicação com NVPRESENT_ENABLE_SMOOTH_MOTION=1
    let program = &args[0];
    let program_args = &args[1..];

    match Command::new(program)
        .args(program_args)
        .env("NVPRESENT_ENABLE_SMOOTH_MOTION", "1")
        .status()
    {
        Ok(status) => {
            // Drop de _guard restaura direct_scanout = 2
            drop(_guard);
            if let Some(code) = status.code() {
                process::exit(code);
            } else {
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Erro ao executar '{}': {}", program, e);
            drop(_guard);
            process::exit(127);
        }
    }
}
