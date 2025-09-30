// Simple verification that FMOD 2.03.09 works
// Run with: cargo run --example verify_203

use libfmod::{System, Init};

fn main() -> Result<(), libfmod::Error> {
    println!("\n🎵 FMOD 2.03.09 Verification Test\n");

    // Create and verify version
    let system = System::create()?;
    let (version, build) = system.get_version()?;

    let major = (version >> 16) & 0xFF;
    let minor = (version >> 8) & 0xFF;
    let patch = version & 0xFF;

    println!("✅ FMOD Version: {}.{:02}.{:02}", major, minor, patch);
    println!("✅ Build Number: {}", build);

    // Initialize
    system.init(512, Init::NORMAL, None)?;
    println!("✅ System initialized");

    // Clean shutdown
    system.release()?;
    println!("✅ System released\n");

    if major == 2 && minor == 3 && patch == 9 {
        println!("🎉 SUCCESS: FMOD 2.03.09 integration verified!");
    } else {
        println!("⚠️  Version mismatch - expected 2.03.09");
    }

    Ok(())
}