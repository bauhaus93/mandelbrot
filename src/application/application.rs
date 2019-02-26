
use allegro;
use allegro::{ Core, Display, EventQueue, Timer, Color };
use allegro_primitives::PrimitivesAddon;

use crate::mandelbrot::Mandelbrot;
use super::application_error::ApplicationError;

pub struct Application {
    stop: bool,
    needs_update: bool,
    mandelbrot: Mandelbrot,
    update_timer: Timer, 
    event_queue: EventQueue,
    primitives: PrimitivesAddon,
    display: Display,
    core: Core,
}

impl Application {
    pub fn new(screen_size: [i32; 2], update_frequency: i32) -> Result<Application, ApplicationError> {
        let core = match Core::init() {
            Ok(e) => e,
            Err(s) => return Err(ApplicationError::Allegro(s))
        };

        match core.install_keyboard() {
            Ok(e) => e,
            Err(_) => return Err(ApplicationError::Allegro("Could not install keyboard".to_string()))
        }

        let display = match Display::new(&core, screen_size[0], screen_size[1]) {
            Ok(e) => e,
            Err(_) => return Err(ApplicationError::Allegro("Could not create display".to_string()))
        };

        let event_queue = match EventQueue::new(&core) {
            Ok(e) => e,
            Err(_) => return Err(ApplicationError::Allegro("Could not create event queue".to_string()))
        };

        let update_timer = match Timer::new(&core, 1.0 / update_frequency as f64) {
            Ok(e) => e,
            Err(_) => return Err(ApplicationError::Allegro("Could not update create timer".to_string()))
        };
        let primitives_addon = match PrimitivesAddon::init(&core) {
            Ok(e) => e,
            Err(s) => return Err(ApplicationError::Allegro(s))
        };

        event_queue.register_event_source(display.get_event_source());
        match core.get_keyboard_event_source() {
            Some(s) => event_queue.register_event_source(s),
            None => return Err(ApplicationError::Allegro("No keyboard event source".to_string()))
        }
        event_queue.register_event_source(update_timer.get_event_source());

        let mut mandelbrot = Mandelbrot::default();
        mandelbrot.set_shape(screen_size);
        let app = Self {
            stop: false,
            needs_update: true,
            mandelbrot: mandelbrot,
            update_timer: update_timer,
            event_queue: event_queue,
            primitives: primitives_addon,
            display: display,
            core: core
        };
        Ok(app)
    }

    pub fn run(&mut self) {
        self.update_timer.start();
        while !self.stop {
            match self.event_queue.wait_for_event() {
                allegro::KeyDown { keycode: k, .. } => {
                    self.handle_keydown(k);
                },
                allegro::TimerTick { .. } => {
                    if self.needs_update {
                        self.update();
                        self.needs_update = false;
                    }
                },
                allegro::DisplayResize { width, height, .. } => {
                    info!("W = {}, H = {}", width, height);
                    self.mandelbrot.set_shape([width, height]);
                    self.needs_update = true;
                }
                allegro::DisplayClose { .. } => {
                    self.stop = true;
                },
                _ => {}
            }
        }
        self.update_timer.stop(); 
    }

    fn handle_keydown(&mut self, key: allegro::KeyCode) {
        const MOVE_PERC: f64 = 0.33;
        match key {
            allegro::KeyCode::W => {
                self.mandelbrot.move_center([0., -MOVE_PERC]);
                self.needs_update = true;
             },
            allegro::KeyCode::A => {
                self.mandelbrot.move_center([-MOVE_PERC, 0.]);
                self.needs_update = true;
            },
            allegro::KeyCode::S => {
                self.mandelbrot.move_center([0., MOVE_PERC]);
                self.needs_update = true;
            },
            allegro::KeyCode::D => {
                self.mandelbrot.move_center([MOVE_PERC, 0.]);
                self.needs_update = true;
            },
            allegro::KeyCode::E => {
                self.mandelbrot.zoom(0.8);
                self.needs_update = true;
            },
            allegro::KeyCode::R => {
                self.mandelbrot.mod_depth(5);
                self.needs_update = true;
            },
            _ => {}
        }
    }

    fn update(&mut self) {
        self.update_timer.stop();
        self.mandelbrot.print_stats();
        let pixels = self.mandelbrot.create_pixels();
        let shape = self.mandelbrot.get_shape();
        self.render(pixels, shape);
        self.update_timer.start();
    }

    fn render(&mut self, pixels: Vec<[u8; 3]>, shape: [i32; 2]) {
        self.core.clear_to_color(Color::from_rgb(0, 0, 0));
        let mut iter = pixels.iter();
        for y in 0..shape[1] {
            for x in 0..shape[0] {
                match iter.next() {
                    Some(p) => {
                        self.core.draw_pixel(x as f32, y as f32, Color::from_rgb(p[0], p[1], p[2]));
                    },
                    None => unreachable!()
                }
            }
        }
        self.core.flip_display();
    }

}
