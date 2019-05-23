#[macro_use]
extern crate lazy_static;
extern crate cursive;
extern crate rand;

use std::collections::vec_deque::VecDeque;
use rand::Rng;
use rand::seq::SliceRandom;
use cursive::traits::*;
use cursive::{Cursive, Printer, theme, direction, event, Vec2, XY};
use cursive::views::{Canvas, Panel, OnEventView, Dialog, ViewRef};
use cursive::theme::{BaseColor, ColorStyle};
use std::thread;
use std::time::Duration;

lazy_static! {
    // it would be nice if we could retrieve the size of the canvas from
    // the views::Canvas object
    static ref CANVAS_SIZE: Vec2 = Vec2::from((30, 15));
    static ref BLOCK: &'static str = "██";
    static ref PELLET_COLOR: ColorStyle = theme::ColorStyle::new(
        theme::Color::Light(BaseColor::Red),
        theme::PaletteColor::View
    );
    static ref SNAKE_COLOR: ColorStyle = theme::ColorStyle::new(
        theme::Color::Dark(BaseColor::Green),
        theme::PaletteColor::View
    );
}

#[derive(Clone)]
struct CanvasState {
    pellet_coord: Vec2,
    snake_coords: VecDeque<XY<isize>>,
    last_direction: direction::Absolute,
}

impl CanvasState {
    pub fn new() -> Self {
        let mut snake_coords: VecDeque<XY<isize>> = VecDeque::new();
        snake_coords.push_front(XY::new(0, 1));

        CanvasState {
            pellet_coord: Vec2::from((10, 4)),
            snake_coords,
            last_direction: direction::Absolute::None
        }
    }

    pub fn is_out_of_bounds(&self) -> bool {
        let head = self.snake_head();
        head.x < 0 || head.x as usize >= CANVAS_SIZE.x || head.y < 0 || head.y as usize >= CANVAS_SIZE.y
    }

    pub fn is_overlapping(&self) -> bool {
        let head = self.snake_head();

        // look at coords excluding head
        for idx in 1..self.snake_coords.len() {
            if *head == self.snake_coords[idx] {
                return true
            }
        }
        return false
    }

    pub fn update(&mut self, direction: direction::Absolute) {
        let snake_head = self.snake_head();

        let new_head: XY<isize> = XY::from(match direction {
            direction::Absolute::Up => (snake_head.x, snake_head.y - 1),
            direction::Absolute::Down => (snake_head.x, snake_head.y + 1),
            direction::Absolute::Left => (snake_head.x - 2, snake_head.y),
            direction::Absolute::Right => (snake_head.x + 2, snake_head.y),
            direction::Absolute::None => panic!("Can't move in no direction")
        });

        self.last_direction = direction;
        self.snake_coords.push_front(new_head);

        if new_head.map(|v| v as usize) == self.pellet_coord {
            // we've reached a pellet!
            // Grow snake size by 1 by not removing the butt, and reset pellet location somewhere else
            self.pellet_coord = self.next_pellet_coord()
        } else {
            self.snake_coords.pop_back();
        }
    }


    // returns a vector of snake coordinates in usize
    // Is there an easier/idiomatic way to map XY<isize> to XY<usize>?
    pub fn snake_coords(&self) -> Vec<Vec2> {
        self.snake_coords.iter().map(|xy| xy.map(|v| v as usize)).collect()
    }

    fn snake_head(&self) -> &XY<isize> {
        self.snake_coords.front().expect("Snake should always have a head")
    }


    fn next_pellet_coord(&self) -> Vec2 {
        let mut canvas_coords: Vec<Vec2> = vec![];
        for x in (0..CANVAS_SIZE.x).step_by(BLOCK.len()) {
            for y in 0..CANVAS_SIZE.y {
                canvas_coords.push(Vec2::from((x, y)));
            }
        }

        let open_coords: Vec<&Vec2> = canvas_coords.iter().filter(|&coord| {
            !(self.snake_coords.contains(&coord.map(|v| v as isize)) || self.pellet_coord == *coord)
        }).collect();

        **open_coords.choose(&mut rand::thread_rng()).expect("How are there no open spots left OMG")
    }
}

fn main() {
    let mut siv: Cursive = Cursive::default();

    let initial_canvas_state = CanvasState::new();
    let curr_state = initial_canvas_state.clone();
    let canvas = Canvas::new(initial_canvas_state)
        .with_draw(move |_, printer: &Printer| {
            draw_blocks(printer,
                        &curr_state.pellet_coord,
                        &curr_state.snake_coords());
        })
        .with_id("canvas")
        .fixed_size(*CANVAS_SIZE);

    siv.add_layer(
        OnEventView::new(
            Panel::new(canvas).title("Snake"))
            .on_event('q', |s| s.quit())
            .on_event(event::Key::Esc, |s| s.quit())
            .on_event(event::Key::Up, |s| move_snake(s, direction::Absolute::Up))
            .on_event(event::Key::Down, |s| move_snake(s, direction::Absolute::Down))
            .on_event(event::Key::Left, |s| move_snake(s, direction::Absolute::Left))
            .on_event(event::Key::Right, |s| move_snake(s, direction::Absolute::Right))
    );

    let cb_sink = siv.cb_sink().clone();

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(1000));
            cb_sink.send(Box::new(|s: &mut Cursive| {
                let mut view: ViewRef<Canvas<CanvasState>> = s.find_id("canvas").unwrap();
                view.set_draw(move |_, printer: &Printer| {
                    draw_blocks(printer,
                                &XY::new(1, rand::thread_rng().gen_range(0, CANVAS_SIZE.y)),
                                &vec![XY::new(0, 1)]);
                });
            })).unwrap();
        }
    });

    siv.set_fps(2);

    siv.run();
}

fn move_snake(s: &mut Cursive, direction: direction::Absolute) {
    let mut view: ViewRef<Canvas<CanvasState>> = s.find_id("canvas").unwrap();
    let canvas_state = view.state_mut();

    if opposite_direction(direction, canvas_state.last_direction) &&
        canvas_state.snake_coords.len() > 1 {
        return;
    }

    canvas_state.update(direction);

    let curr_state = canvas_state.clone();
    if curr_state.is_out_of_bounds() || curr_state.is_overlapping() {
        // Oh man you did a bad
        s.add_layer(Dialog::text("Game Over")
            .button("Quit", Cursive::quit));
    } else {
        view.set_draw(move |_, printer: &Printer| {
            draw_blocks(printer,
                        &curr_state.pellet_coord,
                        &curr_state.snake_coords())
        });
    }
}

fn opposite_direction(curr_direction: direction::Absolute, last_direction: direction::Absolute) -> bool {
    (curr_direction == direction::Absolute::Up && last_direction == direction::Absolute::Down) ||
    (curr_direction == direction::Absolute::Down && last_direction == direction::Absolute::Up) ||
    (curr_direction == direction::Absolute::Right && last_direction == direction::Absolute::Left) ||
    (curr_direction == direction::Absolute::Left && last_direction == direction::Absolute::Right)
}

fn draw_blocks(printer: &Printer, pellet_coord: &Vec2, snake_coords: &Vec<Vec2>) {
    printer.with_color(*PELLET_COLOR, |printer| {
        printer.print(pellet_coord, &*BLOCK);
    });

    printer.with_color(*SNAKE_COLOR, |printer| {
        for coord in snake_coords {
            printer.print(coord, &*BLOCK);
        }
    });
}