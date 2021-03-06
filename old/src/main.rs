#![allow(dead_code)]

extern crate rand;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod lib;
mod ui;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use lib::type_aliases::*;
use lib::grid::grid_manager::GridManager;
use lib::organisms;
use lib::organisms::OrganismType;
use ui::selection_box::SelectionBox;
use ui::selection_box;

const CANVAS_WIDTH: u32 = 600;
const CANVAS_HEIGHT: u32 = CANVAS_WIDTH;
const GUI_WIDTH: f64 = 150.0;

const BUTTON_HEIGHT: f64 = 30.0;
const SELECTION_COLOR: Color = [0.75, 0.75, 0.75, 1.0];

const GRID_WIDTH: u32 = 200;
const GRID_HEIGHT: u32 = GRID_WIDTH;

struct Environment {
    gl: GlGraphics,
    pub buttons: Vec<SelectionBox>,
}

impl Environment {
    fn new (opengl: OpenGL) -> Self {
        Environment {
            gl: GlGraphics::new(opengl),
            buttons: selection_box::get_buttons(),
        }
    }

    fn render(&mut self, args: &RenderArgs, grid_manager: &GridManager) {
        use graphics::*;

        let organism_width: f64 = (args.width as f64 - GUI_WIDTH) / GRID_WIDTH as f64;
        let organism_height: f64 = args.height as f64/ GRID_HEIGHT as f64;
        
        let square = rectangle::square(0.0, 0.0, organism_width);

        let button_shape = rectangle::rectangle_by_corners(0.0, 0.0, GUI_WIDTH as f64, BUTTON_HEIGHT);
        let buttons = &self.buttons;
        
        self.gl.draw(args.viewport(), |c, gl|{
            
            for ((x, y), color) in grid_manager.color_enumerator() {
                let transform = c.transform.trans(x as f64 * organism_width, y as f64 * organism_height);
                rectangle(color, square, transform, gl);
            }

            for (i, button) in buttons.iter().enumerate() {
                let transform = c.transform.trans(CANVAS_WIDTH as f64, i as f64 * BUTTON_HEIGHT);
                rectangle(button.color, button_shape, transform, gl);
            }
        });
    }

    fn update(&mut self, grid_manager: &mut GridManager) {
        grid_manager.update();
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut grid_manager = GridManager::new(GRID_WIDTH, GRID_HEIGHT);           
    
    let mut window: Window = WindowSettings::new(
            "My Little Habitat",
            [CANVAS_WIDTH + GUI_WIDTH as u32, CANVAS_HEIGHT]
        ).opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut env = Environment::new(opengl);
    let mut events = Events::new(EventSettings::new());

    events.set_max_fps(60);
    events.set_ups(60);

    let width_scale = (CANVAS_WIDTH as u32 / GRID_WIDTH) as f64;
    let height_scale = (CANVAS_HEIGHT as u32 / GRID_HEIGHT) as f64;

    let mut pos_x = 0;
    let mut pos_y = 0;
    let mut mouse_pos: (f64, f64) = (0.0, 0.0);
    let mut mouse_down: bool = false;

    let mut current_selection: OrganismType = OrganismType::Plant;

    while let Some(e) = events.next(&mut window) {
        match e {
            Input::Render(render_args) => {
                env.render(&render_args, &grid_manager);
            },
            Input::Update(_) => {
                env.update(&mut grid_manager);
            },
            Input::Press(button) => {
                if button == Button::Mouse(MouseButton::Left) {
                    mouse_down = true;
                }
                match button {
                    Button::Mouse(MouseButton::Left) => mouse_down = true,
                    Button::Keyboard(Key::Space) => grid_manager = GridManager::new(GRID_WIDTH, GRID_HEIGHT),
                    _ => {}
                }
            },
            Input::Release(button) => {
                if button == Button::Mouse(MouseButton::Left) {
                    mouse_down = false;
                    if mouse_pos.0 > CANVAS_WIDTH as f64{
                        let index = (mouse_pos.1 / BUTTON_HEIGHT) as usize;
                        if index < env.buttons.len() {
                            current_selection = env.buttons[index].organism_type;
                        }
                    }
                }
            },
            Input::Move(motion) => {
                if let Motion::MouseCursor(x, y) = motion {
                    mouse_pos = (x, y);
                    if x < CANVAS_WIDTH as f64 {
                        pos_x = (x  / width_scale) as u32;
                        pos_y = (y  /  height_scale) as u32;
                    } 
                }
            },
            _ => {}
        }
        if mouse_down {
            if mouse_pos.0 < CANVAS_WIDTH as f64 {
                if let Some(organism) = organisms::get_new_organism_of_type(current_selection) {
                    grid_manager.add_to_queue(pos_x, pos_y, organism.layer);
                    grid_manager.set(pos_x, pos_y, organism);
                } 
            }
        }
    }   
}