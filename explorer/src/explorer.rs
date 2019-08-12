use allegro;
use allegro::{Color, Core, Display, EventQueue, Timer};
use allegro_primitives::PrimitivesAddon;
use chrono::Local;

use crate::mandelbrot_core::Mandelbrot;
use crate::ExplorerError;

pub struct Explorer {
    stop: bool,
    needs_update: bool,
    mandelbrot: Mandelbrot,
    update_timer: Timer,
    shape: [i32; 2],
    event_queue: EventQueue,
    primitives: PrimitivesAddon,
    display: Display,
    core: Core,
}

impl Explorer {
    pub fn new(screen_size: [i32; 2], update_frequency: i32) -> Result<Explorer, ExplorerError> {
        let core = match Core::init() {
            Ok(e) => e,
            Err(s) => return Err(ExplorerError::Allegro(s)),
        };

        match core.install_keyboard() {
            Ok(e) => e,
            Err(_) => {
                return Err(ExplorerError::Allegro(
                    "Could not install keyboard".to_string(),
                ))
            }
        }

        match core.install_mouse() {
            Ok(e) => e,
            Err(_) => {
                return Err(ExplorerError::Allegro(
                    "Could not install mouse".to_string(),
                ))
            }
        }

        let display = match Display::new(&core, screen_size[0], screen_size[1]) {
            Ok(e) => e,
            Err(_) => {
                return Err(ExplorerError::Allegro(
                    "Could not create display".to_string(),
                ))
            }
        };

        let event_queue = match EventQueue::new(&core) {
            Ok(e) => e,
            Err(_) => {
                return Err(ExplorerError::Allegro(
                    "Could not create event queue".to_string(),
                ))
            }
        };

        let update_timer = match Timer::new(&core, 1.0 / update_frequency as f64) {
            Ok(e) => e,
            Err(_) => {
                return Err(ExplorerError::Allegro(
                    "Could not update create timer".to_string(),
                ))
            }
        };

        let primitives_addon = match PrimitivesAddon::init(&core) {
            Ok(e) => e,
            Err(s) => return Err(ExplorerError::Allegro(s)),
        };

        event_queue.register_event_source(display.get_event_source());
        match core.get_keyboard_event_source() {
            Some(s) => event_queue.register_event_source(s),
            None => {
                return Err(ExplorerError::Allegro(
                    "No keyboard event source".to_string(),
                ))
            }
        }
        match core.get_mouse_event_source() {
            Some(s) => event_queue.register_event_source(s),
            None => return Err(ExplorerError::Allegro("No mouse event source".to_string())),
        }
        event_queue.register_event_source(update_timer.get_event_source());

        let mandelbrot = Mandelbrot::default();
        let app = Self {
            stop: false,
            needs_update: true,
            mandelbrot: mandelbrot,
            update_timer: update_timer,
            shape: screen_size,
            event_queue: event_queue,
            primitives: primitives_addon,
            display: display,
            core: core,
        };
        Ok(app)
    }

    pub fn run(&mut self) {
        self.update_timer.start();
        while !self.stop {
            match self.event_queue.wait_for_event() {
                allegro::KeyDown { keycode: k, .. } => {
                    self.handle_keydown(k);
                }
                allegro::MouseButtonDown { x, y, button, .. } => {
                    self.handle_mousedown([x, y], button);
                }
                allegro::TimerTick { .. } => {
                    if self.needs_update {
                        self.update();
                        self.needs_update = false;
                    }
                }
                allegro::DisplayResize { width, height, .. } => {
                    info!("W = {}, H = {}", width, height);
                    self.shape = [width, height];
                    self.needs_update = true;
                }
                allegro::DisplayClose { .. } => {
                    self.stop = true;
                }
                _ => {}
            }
        }
        self.update_timer.stop();
    }

    fn handle_keydown(&mut self, key: allegro::KeyCode) {
        match key {
            allegro::KeyCode::E => {
                self.mandelbrot.zoom(0.8);
                self.needs_update = true;
            }
            allegro::KeyCode::R => {
                self.mandelbrot.mod_depth(25);
                self.needs_update = true;
            }
            allegro::KeyCode::F1 => {
                let name = format!("{}", Local::now().format("%Y%m%d_%H%M%S"));
                let snapshot_shape = [1920, 1080];
                info!(
                    "Starting snapshot of size {}x{}",
                    snapshot_shape[0], snapshot_shape[1]
                );
                match self.mandelbrot.snapshot(&name, snapshot_shape) {
                    Ok(_) => info!("Finished snapshot!"),
                    Err(e) => error!("Snapshot: {}", e),
                }
            }
            allegro::KeyCode::F2 => {
                info!("Starting zoomed sequence...");
                match self
                    .mandelbrot
                    .snapshot_sequence_zoomed(1000, [800, 600], 0.99, "seq_")
                {
                    Ok(_) => info!("Finished zoomed sequence"),
                    Err(e) => error!("Zoomed sequence: {}", e),
                }
            },
            allegro::KeyCode::F3 => {
                self.mandelbrot.randomize_start_color();
                self.needs_update = true;
            },
            allegro::KeyCode::F4 => {
                self.mandelbrot.set_step_default();
                self.needs_update = true;
            }
            allegro::KeyCode::Escape => {
                self.stop = true;
            }
            _ => {}
        }
    }

    fn handle_mousedown(&mut self, pos: [i32; 2], button: u32) {
        info!("pos = {}/{}, button = {}", pos[0], pos[1], button);
        match button {
            1 => {
                let center_offset = [pos[0] - self.shape[0] / 2, pos[1] - self.shape[1] / 2];
                self.mandelbrot.move_center(center_offset);
                self.needs_update = true;
            }
            _ => {}
        }
    }

    fn update(&mut self) {
        self.update_timer.stop();
        self.mandelbrot.print_stats();
        let pixels = self.mandelbrot.create_pixel_triplets(self.shape);
        self.render(&pixels);
        self.update_timer.start();
    }

    fn render(&mut self, pixels: &[[u8; 3]]) {
        self.core.clear_to_color(Color::from_rgb(0, 0, 0));
        let mut iter = pixels.iter();
        for y in 0..self.shape[1] {
            for x in 0..self.shape[0] {
                match iter.next() {
                    Some(p) => {
                        self.core
                            .draw_pixel(x as f32, y as f32, Color::from_rgb(p[0], p[1], p[2]));
                    }
                    None => unreachable!(),
                }
            }
        }
        self.core.flip_display();
    }
}
