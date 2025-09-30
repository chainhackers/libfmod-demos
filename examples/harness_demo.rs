// Demo version of the interactive harness - shows functionality without terminal interaction
// Run with: ./run_fmod.sh harness_demo [demo_name]
// Examples:
//   ./run_fmod.sh harness_demo explosion
//   ./run_fmod.sh harness_demo spatial
//   ./run_fmod.sh harness_demo parameters
//   ./run_fmod.sh harness_demo footsteps
//   ./run_fmod.sh harness_demo all (default)

use libfmod::{Studio, StudioInit, Init, LoadBank, StopMode, Vector, Attributes3d, SpeakerMode};
use libfmod_demos;
use std::{thread, time::Duration, env};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let demo_name = if args.len() > 1 {
        args[1].as_str()
    } else {
        "all"
    };

    println!("\n==============================================================");
    println!("     FMOD Interactive Harness Demo (Non-Interactive)       ");
    println!("==============================================================\n");

    if demo_name != "all" {
        println!("Running demo: {}\n", demo_name);
    }

    // Initialize Studio with explicit stereo output configuration
    let studio = Studio::create()?;

    // Get the core system BEFORE initialization to configure audio output
    let core = studio.get_core_system()?;

    // Set speaker mode to stereo for proper 3D panning (must be done before init)
    core.set_software_format(Some(48000), Some(SpeakerMode::Stereo), Some(0))?;

    // Now initialize the studio system
    studio.initialize(1024, StudioInit::NORMAL, Init::NORMAL, None)?;

    // Get and display output info
    let output_type = core.get_output()?;
    let (sample_rate, speaker_mode, num_raw_speakers) = core.get_software_format()?;
    let stereo_channels = core.get_speaker_mode_channels(SpeakerMode::Stereo)?;
    println!("Audio Output Configuration:");
    println!("  Output Type: {:?}", output_type);
    println!("  Sample Rate: {} Hz", sample_rate);
    println!("  Speaker Mode: {:?} ({} channels)", speaker_mode, stereo_channels);
    println!("  Raw Speakers: {}", num_raw_speakers);

    // Verify 3D settings
    let (doppler, distance_factor, rolloff) = core.get_3d_settings()?;
    println!("3D Audio Settings:");
    println!("  Doppler Scale: {}", doppler);
    println!("  Distance Factor: {}", distance_factor);
    println!("  Rolloff Scale: {}", rolloff);
    println!();

    // Load banks
    let bank_dir = libfmod_demos::get_example_banks_dir()?;
    let master = studio.load_bank_file(&format!("{}/Master.bank", bank_dir), LoadBank::NORMAL)?;
    let strings = studio.load_bank_file(&format!("{}/Master.strings.bank", bank_dir), LoadBank::NORMAL)?;
    let sfx = studio.load_bank_file(&format!("{}/SFX.bank", bank_dir), LoadBank::NORMAL)?;
    let vehicles = studio.load_bank_file(&format!("{}/Vehicles.bank", bank_dir), LoadBank::NORMAL)?;

    println!("OK - Banks loaded: Master, SFX, Vehicles\n");

    // Demo 1: Event Playback
    if demo_name == "all" || demo_name == "explosion" {
    println!(">>> DEMO 1: Event Playback");
    println!("-------------------------");

    let explosion_desc = studio.get_event("event:/Weapons/Explosion")?;
    let explosion = explosion_desc.create_instance()?;

    println!("Playing explosion...");
    explosion.start()?;

    // Let explosion play fully (3 seconds)
    for _ in 0..60 {
        studio.update()?;
        thread::sleep(Duration::from_millis(50));
    }
    explosion.release()?;
    println!("OK - Explosion complete\n");
    }

    // Pause between demos
    if demo_name == "all" {
        thread::sleep(Duration::from_secs(1));
    }

    // Demo 2: 3D Spatial Audio
    if demo_name == "all" || demo_name == "spatial" {
    println!(">>> DEMO 2: 3D Spatial Audio Movement");
    println!("--------------------------------------");

    // Set up listener position - standing 5 units away from the road
    let listener_attributes = Attributes3d {
        position: Vector { x: 0.0, y: 0.0, z: 5.0 },  // 5 units back from the road
        velocity: Vector { x: 0.0, y: 0.0, z: 0.0 },
        forward: Vector { x: 0.0, y: 0.0, z: -1.0 },  // Looking toward the road
        up: Vector { x: 0.0, y: 1.0, z: 0.0 },
    };
    studio.set_listener_attributes(0, listener_attributes, None)?;

    // Also set on core system for 3D processing
    let core = studio.get_core_system()?;
    core.set_3d_listener_attributes(
        0,
        Some(Vector { x: 0.0, y: 0.0, z: 5.0 }),  // Same position as above
        Some(Vector { x: 0.0, y: 0.0, z: 0.0 }),
        Some(Vector { x: 0.0, y: 0.0, z: -1.0 }),
        Some(Vector { x: 0.0, y: 1.0, z: 0.0 })
    )?;

    // Set 3D settings optimized for stereo panning
    // doppler_scale: 1.0 (normal doppler effect)
    // distance_factor: 1.0 (1 unit = 1 meter)
    // rolloff_scale: 1.5 (stronger distance attenuation for clearer spatial effect)
    core.set_3d_settings(1.0, 1.0, 1.5)?;

    // Set the 3D number of listeners
    core.set_3d_num_listeners(1)?;

    let vehicle_desc = studio.get_event("event:/Vehicles/Ride-on Mower")?;
    let vehicle = vehicle_desc.create_instance()?;

    // CRITICAL: The vehicle needs RPM parameter to produce sound!
    vehicle.set_parameter_by_name("RPM", 2000.0, false)?;

    // Set initial 3D position before starting (positive X for left in FMOD)
    let initial_attributes = Attributes3d {
        position: Vector { x: 10.0, y: 0.0, z: 0.0 },  // Start at right (sounds left)
        velocity: Vector { x: 0.0, y: 0.0, z: 0.0 },
        forward: Vector { x: 1.0, y: 0.0, z: 0.0 },
        up: Vector { x: 0.0, y: 1.0, z: 0.0 },
    };
    vehicle.set_3d_attributes(initial_attributes)?;

    // Set volume
    vehicle.set_volume(1.5)?;

    println!("Starting vehicle engine...");
    println!("The vehicle will pan from left to right in stereo.");
    vehicle.start()?;

    // Move the vehicle in 3D space
    println!("Moving vehicle from left to right in stereo:");
    println!("(Movement will take 10 seconds)");
    println!("Listener is at (0,0,5) - standing back from the road");
    println!("Vehicle drives along the road at Z=0, from X=+10 to X=-10");
    println!("Note: In FMOD, positive X = left speaker, negative X = right speaker\n");

    for i in 0..100 {
        let progress = i as f32 / 100.0;
        // Reverse the movement to match audio: start at +10 (left), go to -10 (right)
        let x = 10.0 - (progress * 20.0);  // +10 to -10
        let z = 0.0;  // Keep on same plane as listener

        let attributes = Attributes3d {
            position: Vector { x, y: 0.0, z },
            velocity: Vector { x: -0.2, y: 0.0, z: 0.0 },  // Negative velocity
            forward: Vector { x: 1.0, y: 0.0, z: 0.0 },
            up: Vector { x: 0.0, y: 1.0, z: 0.0 },
        };

        vehicle.set_3d_attributes(attributes)?;

        // Keep RPM at constant 2000
        if i % 20 == 0 {
            vehicle.set_parameter_by_name("RPM", 2000.0, false)?;
        }

        // Visual position indicator - now matches audio direction
        if i % 2 == 0 {
            // Visual goes left to right as audio does
            let visual_pos = (progress * 40.0) as usize;
            let line = format!("{:>width$}▶", "", width = visual_pos.min(40));

            // Calculate distance from listener at (0,0,5) to vehicle at (x,0,0)
            let distance = ((x * x) + (5.0 * 5.0)).sqrt();

            print!("\r  [{:<40}] X:{:5.1} Distance:{:4.1}m ({})", line, x, distance,
                   if x > 5.0 { "Left" } else if x < -5.0 { "Right" } else { "Center" });
            use std::io::{self, Write};
            io::stdout().flush()?;
        }

        // Update both studio and core systems for 3D processing
        studio.update()?;
        core.update()?;
        thread::sleep(Duration::from_millis(100));
    }

    println!("\nStopping vehicle...");
    vehicle.stop(StopMode::AllowFadeout)?;

    // Let fadeout complete (2 seconds)
    for _ in 0..40 {
        studio.update()?;
        thread::sleep(Duration::from_millis(50));
    }
    vehicle.release()?;
    println!("OK - 3D movement complete\n");
    }

    // Pause between demos
    if demo_name == "all" {
        thread::sleep(Duration::from_secs(1));
    }

    // Demo 3: Parameter Control
    if demo_name == "all" || demo_name == "parameters" || demo_name == "rpm" {
    println!(">>> DEMO 3: Real-time Parameter Control");
    println!("----------------------------------------");

    let vehicle_desc = studio.get_event("event:/Vehicles/Ride-on Mower")?;
    let vehicle2 = vehicle_desc.create_instance()?;


    // Start with RPM at 0 (idle)
    vehicle2.set_parameter_by_name("RPM", 0.0, false)?;
    vehicle2.start()?;

    println!("Adjusting vehicle RPM:");
    println!("(Each RPM level will play for 2 seconds)");

    // RPM ranges from 0 to 2000 based on parameter description
    for rpm_level in [0.0, 250.0, 500.0, 750.0, 1000.0, 1250.0, 1500.0, 1750.0, 2000.0, 1000.0, 0.0] {
        print!("\n  RPM: {:4.0} ", rpm_level);

        // Visual RPM meter (scaled to 0-2000 range)
        let bar_length = (rpm_level / 100.0) as usize;
        for i in 0..20 {
            if i < bar_length {
                print!("█");
            } else {
                print!("░");
            }
        }
        use std::io::{self, Write};
        io::stdout().flush()?;

        vehicle2.set_parameter_by_name("RPM", rpm_level, false)?;


        // Hold each RPM level for 2 seconds
        for _ in 0..40 {
            studio.update()?;
            thread::sleep(Duration::from_millis(50));
        }
    }
    println!();

    println!("\nStopping engine...");
    vehicle2.stop(StopMode::AllowFadeout)?;
    for _ in 0..40 {
        studio.update()?;
        thread::sleep(Duration::from_millis(50));
    }
    vehicle2.release()?;
    println!("OK - Parameter control complete\n");
    }

    // Pause between demos
    if demo_name == "all" {
        thread::sleep(Duration::from_secs(1));
    }

    // Demo 4: Multiple Instances
    if demo_name == "all" || demo_name == "footsteps" {
    println!(">>> DEMO 4: Multiple Simultaneous Events");
    println!("-----------------------------------------");

    let footstep_desc = studio.get_event("event:/Character/Player Footsteps")?;

    println!("Playing footsteps pattern:");
    println!("(Walking on different surfaces - 0.5 sec per step)");
    print!("  ");

    for step in 1..=10 {
        let surface = (step % 4) as f32;
        let surface_name = match step % 4 {
            0 => "concrete",
            1 => "gravel",
            2 => "wood",
            _ => "metal",
        };

        print!(".");
        use std::io::{self, Write};
        io::stdout().flush()?;

        let footstep = footstep_desc.create_instance()?;
        footstep.set_parameter_by_name("Surface", surface, false).ok();
        footstep.start()?;
        footstep.release()?;

        // Half second between steps
        for _ in 0..10 {
            studio.update()?;
            thread::sleep(Duration::from_millis(50));
        }

        if step % 5 == 0 {
            println!("  ({})", surface_name);
            if step < 10 {
                print!("  ");
            }
        } else {
            print!(" ");
        }
        io::stdout().flush()?;
    }
    println!("\nOK - Multiple instances complete\n");
    }

    // Final pause
    if demo_name == "all" {
        thread::sleep(Duration::from_secs(1));
    }

    // Summary
    if demo_name == "all" {
    println!("==============================================================");
    println!("                    DEMO COMPLETE!                         ");
    println!("--------------------------------------------------------------");
    println!(" The interactive harness supports:                         ");
    println!("   * Real-time keyboard control                            ");
    println!("   * 3D spatial positioning (WASD + QE)                    ");
    println!("   * Parameter adjustment (+/-)                            ");
    println!("   * Multiple event instances                              ");
    println!("   * Visual feedback and status display                    ");
    println!("                                                            ");
    println!(" To use the full interactive version, run:                 ");
    println!("   ./target/debug/examples/interactive_harness             ");
    println!(" in a proper terminal with keyboard support                ");
    println!("==============================================================");
    }

    // Cleanup
    vehicles.unload()?;
    sfx.unload()?;
    strings.unload()?;
    master.unload()?;
    studio.release()?;

    Ok(())
}