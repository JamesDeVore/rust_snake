extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use ::piston::event_loop::*;
use ::piston::window::WindowSettings;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::*;
use std::collections::LinkedList;
use std::iter::FromIterator;


struct Game {
    gl: GlGraphics,
    snake: Snake,
}

struct Snake {
    body:LinkedList<(i32,i32)>,
    dir: Direction,
}
#[derive(Clone, PartialEq)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics;

        let RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let squares : Vec<graphics::types::Rectangle> = self.body
        .iter()
        .map(|&(x,y)| {
           graphics::rectangle::square(
            (x*20) as f64,
            (y*20) as f64,
            20_f64
          )
        }).collect();

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            squares.into_iter()
            .for_each(|square| graphics::rectangle(RED,square,transform,gl))
        });
    }
    fn update(&mut self) {
      let mut new_head = (*self.body.front().expect("no body")).clone();

        match self.dir {
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
        }

        self.body.push_front(new_head);

        self.body.pop_back().unwrap();
    }
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(GREEN, gl);
        });
        self.snake.render(&mut self.gl, arg);
    }
    fn update(&mut self) {
        self.snake.update()
    }
    fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.dir.clone();

        self.snake.dir = match btn {
            &Button::Keyboard(Key::Up) if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down) if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left) if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right) if last_direction != Direction::Left => Direction::Right,
            _ => last_direction
        }
    }
}
fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("Snake Game", [500, 500])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            body:LinkedList::from_iter((vec![(0,0),(0,1)]).into_iter()),
            dir: Direction::Down,
        },
    };

    let mut events = Events::new(EventSettings::new()).ups(5);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }
        if let Some(u) = e.update_args() {
            game.update();
        }
        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
    }
}
