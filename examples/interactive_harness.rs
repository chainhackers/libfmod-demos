// Interactive FMOD Studio Test Harness - Real-time control for all features
// Run with: ./run_fmod.sh interactive_harness

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    cursor,
    style::{Color, Print, ResetColor, SetForegroundColor, Attribute, SetAttribute},
};
use libfmod::{Studio, StudioInit, Init, LoadBank, StopMode, EventDescription, EventInstance, Bank, Vector, Attributes3d};
use libfmod_demos;
use std::{
    collections::HashMap,
    io::{self, Write},
    time::{Duration, Instant},
};

// Harness state
struct HarnessState {
    studio: Studio,
    banks: Vec<Bank>,
    event_descriptions: Vec<(String, EventDescription)>,
    active_instances: HashMap<usize, EventInstance>,
    selected_event: usize,
    selected_parameter: usize,

    // 3D position
    listener_pos: Vector,
    source_pos: Vector,

    // Display state
    show_help: bool,
    last_update: Instant,
    frame_count: u32,
    fps: f32,
}

impl HarnessState {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Initialize Studio
        let studio = Studio::create()?;
        studio.initialize(1024, StudioInit::NORMAL, Init::NORMAL, None)?;

        // Load all banks
        let bank_dir = libfmod_demos::get_example_banks_dir()?;
        let mut banks = Vec::new();

        banks.push(studio.load_bank_file(&format!("{}/Master.bank", bank_dir), LoadBank::NORMAL)?);
        banks.push(studio.load_bank_file(&format!("{}/Master.strings.bank", bank_dir), LoadBank::NORMAL)?);
        banks.push(studio.load_bank_file(&format!("{}/SFX.bank", bank_dir), LoadBank::NORMAL)?);
        banks.push(studio.load_bank_file(&format!("{}/Music.bank", bank_dir), LoadBank::NORMAL)?);
        banks.push(studio.load_bank_file(&format!("{}/Vehicles.bank", bank_dir), LoadBank::NORMAL)?);

        // Get all events
        let mut event_descriptions = Vec::new();

        // Pre-defined events we know exist
        let event_paths = vec![
            "event:/Ambience/Country",
            "event:/Character/Player Footsteps",
            "event:/Weapons/Explosion",
            "event:/UI/Cancel",
            "event:/Vehicles/Ride-on Mower",
            "event:/Music/Level 01",
        ];

        for path in event_paths {
            if let Ok(desc) = studio.get_event(path) {
                event_descriptions.push((path.to_string(), desc));
            }
        }

        Ok(HarnessState {
            studio,
            banks,
            event_descriptions,
            active_instances: HashMap::new(),
            selected_event: 0,
            selected_parameter: 0,
            listener_pos: Vector { x: 0.0, y: 0.0, z: 0.0 },
            source_pos: Vector { x: 0.0, y: 0.0, z: -5.0 },
            show_help: false,
            last_update: Instant::now(),
            frame_count: 0,
            fps: 0.0,
        })
    }

    fn update(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.studio.update()?;

        // Calculate FPS
        self.frame_count += 1;
        let elapsed = self.last_update.elapsed();
        if elapsed >= Duration::from_secs(1) {
            self.fps = self.frame_count as f32 / elapsed.as_secs_f32();
            self.frame_count = 0;
            self.last_update = Instant::now();
        }

        Ok(())
    }

    fn play_event(&mut self, index: usize) -> Result<(), Box<dyn std::error::Error>> {
        if index >= self.event_descriptions.len() {
            return Ok(());
        }

        let (path, desc) = &self.event_descriptions[index];
        let instance = desc.create_instance()?;

        // Set 3D attributes if applicable
        let attributes = Attributes3d {
            position: self.source_pos.clone(),
            velocity: Vector { x: 0.0, y: 0.0, z: 0.0 },
            forward: Vector { x: 0.0, y: 0.0, z: 1.0 },
            up: Vector { x: 0.0, y: 1.0, z: 0.0 },
        };
        instance.set_3d_attributes(attributes).ok();

        // Set parameters based on event type
        let is_one_shot = path.contains("Footstep") || path.contains("Explosion");

        if path.contains("Vehicle") || path.contains("Ride-on Mower") {
            // Vehicles need RPM to make sound
            instance.set_parameter_by_name("RPM", 1500.0, false)?;
        } else if path.contains("Footstep") {
            // Set surface parameter for footsteps
            instance.set_parameter_by_name("Surface", 1.0, false).ok();
        }

        instance.start()?;

        // For one-shot events, release immediately after starting
        // They will play once and clean up automatically
        if is_one_shot {
            instance.release()?;
        } else {
            // For looping/continuous events, track them
            self.active_instances.insert(index, instance);
        }

        Ok(())
    }

    fn stop_event(&mut self, index: usize) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(instance) = self.active_instances.remove(&index) {
            instance.stop(StopMode::AllowFadeout)?;
            instance.release()?;
        }
        Ok(())
    }

    fn stop_all_events(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for (_idx, instance) in self.active_instances.drain() {
            instance.stop(StopMode::AllowFadeout)?;
            instance.release()?;
        }
        Ok(())
    }

    fn move_source(&mut self, dx: f32, dy: f32, dz: f32) -> Result<(), Box<dyn std::error::Error>> {
        self.source_pos.x += dx;
        self.source_pos.y += dy;
        self.source_pos.z += dz;

        // Update all active instances
        for (_idx, instance) in &self.active_instances {
            let attributes = Attributes3d {
                position: self.source_pos.clone(),
                velocity: Vector { x: 0.0, y: 0.0, z: 0.0 },
                forward: Vector { x: 0.0, y: 0.0, z: 1.0 },
                up: Vector { x: 0.0, y: 1.0, z: 0.0 },
            };
            instance.set_3d_attributes(attributes).ok();
        }

        Ok(())
    }

    fn adjust_parameter(&mut self, delta: f32) -> Result<(), Box<dyn std::error::Error>> {
        // Adjust parameter on all active instances
        for (_idx, instance) in &self.active_instances {
            // Common parameter names
            instance.set_parameter_by_name("RPM", 2000.0 + delta * 1000.0, false).ok();
            instance.set_parameter_by_name("Surface", delta, false).ok();
        }
        Ok(())
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();
}

fn draw_ui(state: &HarnessState) -> io::Result<()> {
    let mut stdout = io::stdout();

    // Clear and reset cursor
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;

    // Header
    execute!(stdout,
        SetForegroundColor(Color::Cyan),
        SetAttribute(Attribute::Bold),
        Print("========================================================================\r\n"),
        Print("| FMOD Interactive Test Harness 1.0 | FPS: "),
        ResetColor,
        Print(format!("{:3.0}", state.fps)),
        SetForegroundColor(Color::Cyan),
        Print(" | Events: "),
        ResetColor,
        Print(format!("{}/{}", state.active_instances.len(), state.event_descriptions.len())),
        SetForegroundColor(Color::Cyan),
        Print("       |\r\n"),
        Print("========================================================================\r\n"),
        ResetColor
    )?;

    // Event list
    execute!(stdout,
        SetForegroundColor(Color::Yellow),
        Print("\r\n> Available Events:\r\n"),
        ResetColor
    )?;

    for (i, (path, _desc)) in state.event_descriptions.iter().enumerate() {
        let is_active = state.active_instances.contains_key(&i);
        let is_selected = i == state.selected_event;

        if is_selected {
            execute!(stdout, SetForegroundColor(Color::Green), Print("> "))?;
        } else {
            execute!(stdout, Print("  "))?;
        }

        execute!(stdout,
            SetForegroundColor(if is_active { Color::Green } else { Color::White }),
            Print(format!("[{}] ", i + 1)),
            Print(path),
            ResetColor
        )?;

        if is_active {
            execute!(stdout,
                SetForegroundColor(Color::Green),
                Print(" [PLAYING]"),
                ResetColor
            )?;
        }

        execute!(stdout, Print("\r\n"))?;
    }

    // 3D Position display
    execute!(stdout,
        Print("\r\n"),
        SetForegroundColor(Color::Yellow),
        Print("* 3D Position:\r\n"),
        ResetColor,
        Print(format!("  Source: X:{:5.1} Y:{:5.1} Z:{:5.1}\r\n",
            state.source_pos.x, state.source_pos.y, state.source_pos.z)),
        Print(format!("  Listener: X:{:5.1} Y:{:5.1} Z:{:5.1}\r\n",
            state.listener_pos.x, state.listener_pos.y, state.listener_pos.z))
    )?;

    // Controls
    if state.show_help {
        execute!(stdout,
            Print("\r\n"),
            SetForegroundColor(Color::Cyan),
            SetAttribute(Attribute::Bold),
            Print("Controls:\r\n"),
            ResetColor,
            SetForegroundColor(Color::White),
            Print("  [1-6]     Play/Stop event\r\n"),
            Print("  [Space]   Stop all events\r\n"),
            Print("  [WASD]    Move source (X/Z)\r\n"),
            Print("  [Q/E]     Move source (Up/Down)\r\n"),
            Print("  [+/-]     Adjust parameters\r\n"),
            Print("  [R]       Reset position\r\n"),
            Print("  [H]       Toggle help\r\n"),
            Print("  [Esc]     Exit\r\n"),
            ResetColor
        )?;
    } else {
        execute!(stdout,
            Print("\r\n"),
            SetForegroundColor(Color::DarkGrey),
            Print("[H] Help  [1-6] Play  [WASD] Move  [Space] Stop All  [Esc] Exit\r\n"),
            ResetColor
        )?;
    }

    stdout.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    clear_screen();

    let mut state = HarnessState::new()?;

    println!("Initializing FMOD Studio Interactive Harness...");
    std::thread::sleep(Duration::from_millis(500));

    // Main loop
    loop {
        // Update FMOD
        state.update()?;

        // Draw UI
        draw_ui(&state)?;

        // Handle input (non-blocking)
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
                match code {
                    // Exit
                    KeyCode::Esc => break,

                    // Help
                    KeyCode::Char('h') | KeyCode::Char('H') => {
                        state.show_help = !state.show_help;
                    }

                    // Play events
                    KeyCode::Char('1') => {
                        if state.active_instances.contains_key(&0) {
                            state.stop_event(0)?;
                        } else {
                            state.play_event(0)?;
                        }
                    }
                    KeyCode::Char('2') if state.event_descriptions.len() > 1 => {
                        if state.active_instances.contains_key(&1) {
                            state.stop_event(1)?;
                        } else {
                            state.play_event(1)?;
                        }
                    }
                    KeyCode::Char('3') if state.event_descriptions.len() > 2 => {
                        if state.active_instances.contains_key(&2) {
                            state.stop_event(2)?;
                        } else {
                            state.play_event(2)?;
                        }
                    }
                    KeyCode::Char('4') if state.event_descriptions.len() > 3 => {
                        if state.active_instances.contains_key(&3) {
                            state.stop_event(3)?;
                        } else {
                            state.play_event(3)?;
                        }
                    }
                    KeyCode::Char('5') if state.event_descriptions.len() > 4 => {
                        if state.active_instances.contains_key(&4) {
                            state.stop_event(4)?;
                        } else {
                            state.play_event(4)?;
                        }
                    }
                    KeyCode::Char('6') if state.event_descriptions.len() > 5 => {
                        if state.active_instances.contains_key(&5) {
                            state.stop_event(5)?;
                        } else {
                            state.play_event(5)?;
                        }
                    }

                    // Stop all
                    KeyCode::Char(' ') => {
                        state.stop_all_events()?;
                    }

                    // 3D movement
                    KeyCode::Char('w') | KeyCode::Char('W') => {
                        let delta = if modifiers.contains(KeyModifiers::SHIFT) { 0.1 } else { 1.0 };
                        state.move_source(0.0, 0.0, -delta)?;
                    }
                    KeyCode::Char('s') | KeyCode::Char('S') => {
                        let delta = if modifiers.contains(KeyModifiers::SHIFT) { 0.1 } else { 1.0 };
                        state.move_source(0.0, 0.0, delta)?;
                    }
                    KeyCode::Char('a') | KeyCode::Char('A') => {
                        let delta = if modifiers.contains(KeyModifiers::SHIFT) { 0.1 } else { 1.0 };
                        state.move_source(-delta, 0.0, 0.0)?;
                    }
                    KeyCode::Char('d') | KeyCode::Char('D') => {
                        let delta = if modifiers.contains(KeyModifiers::SHIFT) { 0.1 } else { 1.0 };
                        state.move_source(delta, 0.0, 0.0)?;
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        let delta = if modifiers.contains(KeyModifiers::SHIFT) { 0.1 } else { 1.0 };
                        state.move_source(0.0, delta, 0.0)?;
                    }
                    KeyCode::Char('e') | KeyCode::Char('E') => {
                        let delta = if modifiers.contains(KeyModifiers::SHIFT) { 0.1 } else { 1.0 };
                        state.move_source(0.0, -delta, 0.0)?;
                    }

                    // Reset position
                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        state.source_pos = Vector { x: 0.0, y: 0.0, z: -5.0 };
                        state.move_source(0.0, 0.0, 0.0)?;
                    }

                    // Parameter adjustment
                    KeyCode::Char('+') | KeyCode::Char('=') => {
                        state.adjust_parameter(0.1)?;
                    }
                    KeyCode::Char('-') | KeyCode::Char('_') => {
                        state.adjust_parameter(-0.1)?;
                    }

                    _ => {}
                }
            }
        }
    }

    // Cleanup
    state.stop_all_events()?;
    for bank in state.banks.iter() {
        bank.unload()?;
    }
    state.studio.release()?;

    // Restore terminal
    disable_raw_mode()?;
    clear_screen();

    println!("Interactive harness closed. Thanks for testing!");

    Ok(())
}