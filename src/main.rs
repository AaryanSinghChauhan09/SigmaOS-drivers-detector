//! SigmaOS-mint-drivers-detector — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_mint_drivers_detector_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-mint-drivers-detector: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-mint-drivers-detector: ok");
}
