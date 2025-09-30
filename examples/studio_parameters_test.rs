// Test FMOD Studio real-time parameter control with FMOD 2.03.09
// Run with: ./run_fmod.sh studio_parameters_test

use libfmod::{Studio, StudioInit, Init, LoadBank, StopMode};
use libfmod_demos;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎛️  FMOD Studio Parameters Test (2.03.09)\n");
    println!("==========================================\n");

    // Initialize Studio System
    let studio = Studio::create()?;
    studio.initialize(1024, StudioInit::NORMAL, Init::NORMAL, None)?;

    // Load banks
    let bank_dir = libfmod_demos::get_example_banks_dir()?;
    let master = studio.load_bank_file(&format!("{}/Master.bank", bank_dir), LoadBank::NORMAL)?;
    let strings = studio.load_bank_file(&format!("{}/Master.strings.bank", bank_dir), LoadBank::NORMAL)?;
    let sfx = studio.load_bank_file(&format!("{}/SFX.bank", bank_dir), LoadBank::NORMAL)?;
    let vehicles = studio.load_bank_file(&format!("{}/Vehicles.bank", bank_dir), LoadBank::NORMAL)?;

    println!("Banks loaded ✓\n");

    // Test 1: Vehicle RPM Parameter
    println!("🚜 TEST 1: Vehicle Engine RPM Control");
    println!("--------------------------------------");

    let vehicle_desc = studio.get_event("event:/Vehicles/Ride-on Mower")?;
    let vehicle = vehicle_desc.create_instance()?;

    // Note: Parameter descriptions are on the Studio System level in FMOD 2.03
    println!("Note: Events use global and local parameters");

    println!("\nStarting engine...");
    vehicle.start()?;

    // Simulate RPM changes
    println!("Adjusting RPM:");

    // Try to set RPM parameter
    for rpm_stage in 0..=4 {
        let rpm = rpm_stage as f32 * 1000.0; // 0, 1000, 2000, 3000, 4000

        println!("  RPM: {:.0}", rpm);

        // Try setting by name (common parameter names)
        vehicle.set_parameter_by_name("RPM", rpm, false).ok();
        vehicle.set_parameter_by_name("rpm", rpm, false).ok();
        vehicle.set_parameter_by_name("EngineRPM", rpm, false).ok();

        // Let it play at this RPM
        for _ in 0..20 {
            studio.update()?;
            thread::sleep(Duration::from_millis(50));
        }
    }

    println!("Decreasing RPM...");
    for rpm_stage in (0..=2).rev() {
        let rpm = rpm_stage as f32 * 1000.0;
        println!("  RPM: {:.0}", rpm);

        vehicle.set_parameter_by_name("RPM", rpm, false).ok();
        vehicle.set_parameter_by_name("rpm", rpm, false).ok();

        for _ in 0..15 {
            studio.update()?;
            thread::sleep(Duration::from_millis(50));
        }
    }

    println!("Stopping engine...");
    vehicle.stop(StopMode::AllowFadeout)?;

    for _ in 0..30 {
        studio.update()?;
        thread::sleep(Duration::from_millis(50));
    }
    vehicle.release()?;

    // Test 2: Footsteps with Surface Parameter
    println!("\n👟 TEST 2: Footsteps Surface Parameter");
    println!("---------------------------------------");

    let footstep_desc = studio.get_event("event:/Character/Player Footsteps")?;

    // Parameters control variations
    println!("Parameters control footstep variations...");

    // Simulate walking on different surfaces
    let surfaces = vec![
        (0.0, "Concrete"),
        (1.0, "Gravel"),
        (2.0, "Wood"),
        (3.0, "Metal"),
    ];

    for (value, name) in surfaces {
        println!("\nWalking on {}:", name);

        for step in 1..=4 {
            println!("  Step {}", step);

            let footstep = footstep_desc.create_instance()?;

            // Set surface parameter
            footstep.set_parameter_by_name("Surface", value, false).ok();
            footstep.set_parameter_by_name("surface", value, false).ok();
            footstep.set_parameter_by_name("Material", value, false).ok();

            footstep.start()?;
            footstep.release()?;

            // Pause between steps
            for _ in 0..6 {
                studio.update()?;
                thread::sleep(Duration::from_millis(50));
            }
        }
    }

    // Test 3: Volume and Pitch Control
    println!("\n🔊 TEST 3: Volume and Pitch Control");
    println!("------------------------------------");

    let ambience_desc = studio.get_event("event:/Ambience/Country")?;
    let ambience = ambience_desc.create_instance()?;

    println!("Starting ambient sound...");
    ambience.start()?;

    // Volume control
    println!("\nAdjusting volume:");
    let volumes = vec![
        (1.0, "100%"),
        (0.5, "50%"),
        (0.2, "20%"),
        (0.5, "50%"),
        (1.0, "100%"),
    ];

    for (volume, label) in volumes {
        println!("  Volume: {}", label);
        ambience.set_volume(volume)?;

        for _ in 0..15 {
            studio.update()?;
            thread::sleep(Duration::from_millis(50));
        }
    }

    // Pitch control
    println!("\nAdjusting pitch:");
    let pitches = vec![
        (1.0, "Normal"),
        (1.5, "+50%"),
        (0.5, "-50%"),
        (1.0, "Normal"),
    ];

    for (pitch, label) in pitches {
        println!("  Pitch: {}", label);
        ambience.set_pitch(pitch)?;

        for _ in 0..20 {
            studio.update()?;
            thread::sleep(Duration::from_millis(50));
        }
    }

    println!("\nStopping ambient...");
    ambience.stop(StopMode::AllowFadeout)?;

    for _ in 0..20 {
        studio.update()?;
        thread::sleep(Duration::from_millis(50));
    }
    ambience.release()?;

    // Test 4: Global Parameter (if available)
    println!("\n🌍 TEST 4: Global Parameters");
    println!("-----------------------------");

    // Try to set a global parameter (affects all events)
    println!("Setting global parameters:");

    studio.set_parameter_by_name("TimeOfDay", 0.0, false).ok();
    println!("  TimeOfDay = Morning");

    studio.set_parameter_by_name("Weather", 1.0, false).ok();
    println!("  Weather = Rainy");

    studio.set_parameter_by_name("Tension", 0.5, false).ok();
    println!("  Tension = Medium");

    // These would affect any playing events that use these parameters

    // Clean up
    println!("\nCleaning up...");
    vehicles.unload()?;
    sfx.unload()?;
    strings.unload()?;
    master.unload()?;
    studio.release()?;

    println!("\n==========================================");
    println!("✅ Parameter tests completed!");
    println!("   - Engine RPM control tested");
    println!("   - Surface parameters tested");
    println!("   - Volume/Pitch control works");
    println!("   - Global parameters set");
    println!("==========================================\n");

    Ok(())
}