#[macro_use]
extern crate lazy_static;
extern crate cursive;
extern crate rand;

use cursive::traits::*;
use cursive::{Cursive, Printer, theme, event, Vec2};
use cursive::views::{Canvas, Panel, OnEventView};
use cursive::theme::{BaseColor, ColorStyle};
use std::collections::vec_deque::VecDeque;
use rand::seq::SliceRandom;

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
    snake_coords: VecDeque<Vec2>
}

impl CanvasState {
    fn snake_head(&self) -> &Vec2 {
        self.snake_coords.front().expect("Snake should always have a head")
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    let mut siv = Cursive::default();

    let mut snake_coords = VecDeque::new();
    snake_coords.push_front(Vec2::from((0, 1)));
    let initial_canvas_state = CanvasState {
        pellet_coord: Vec2::from((10, 4)),
        snake_coords
    };

    let curr_state = initial_canvas_state.clone();

    let canvas = Canvas::new(initial_canvas_state)
        .with_draw(move |_, printer: &Printer| {
            draw_blocks(printer,
                        &curr_state.pellet_coord,
                        &curr_state.snake_coords);
        })
        .with_id("canvas")
        .fixed_size(*CANVAS_SIZE);

    siv.add_layer(
        OnEventView::new(
            Panel::new(canvas).title("Snake"))
            .on_event('q', |s| s.quit())
            .on_event(event::Key::Esc, |s| s.quit())
            .on_event(event::Key::Up, |s| move_snake(s, Direction::Up))
            .on_event(event::Key::Down, |s| move_snake(s, Direction::Down))
            .on_event(event::Key::Left, |s| move_snake(s, Direction::Left))
            .on_event(event::Key::Right, |s| move_snake(s, Direction::Right))
    );
    siv.run();
}

fn move_snake(s: &mut Cursive, direction: Direction) {
    s.call_on_id(
        "canvas",
        |view| update_snake_coord(view, direction)
    );
}

fn update_snake_coord(view: &mut Canvas<CanvasState>, direction: Direction) {
    let canvas_state = view.state_mut();
    let snake_head = canvas_state.snake_head();

    let new_head = Vec2::from(match direction {
        Direction::Up => (snake_head.x, snake_head.y - 1),
        Direction::Down => (snake_head.x, snake_head.y + 1),
        Direction::Left => (snake_head.x - 2, snake_head.y),
        Direction::Right => (snake_head.x + 2, snake_head.y)
    });

    canvas_state.snake_coords.push_front(new_head);

    if new_head == canvas_state.pellet_coord {
        // we've reached a pellet!
        // Grow snake size by 1, and reset pellet location somewhere else
        canvas_state.pellet_coord = next_pellet_coord(&canvas_state.pellet_coord, &canvas_state.snake_coords)
    } else {
        canvas_state.snake_coords.pop_back();
    }

    let curr_state = canvas_state.clone();

    if is_out_of_bounds(curr_state.snake_head()) {
        // Oh man you did a bad
        // TODO: transition to endgame state
    }

    view.set_draw(move |_, printer: &Printer| {
        draw_blocks(printer,
                    &curr_state.pellet_coord,
                    &curr_state.snake_coords)
    });
}

fn is_out_of_bounds(head: &Vec2) -> bool {
    // NOTE: since the coordinates are currently represented as Vec2's -> XY<usize>
    // they can't be negative and these "head.<x|y> < 0" conditions are redundant
    // TODO: refactor CanvasState s.t. we can use negative coordinates
    head.x < 0 || head.x >= CANVAS_SIZE.x || head.y < 0 || head.y >= CANVAS_SIZE.y
}

fn next_pellet_coord(pellet_coord: &Vec2, snake_coords: &VecDeque<Vec2>) -> Vec2 {
    let mut canvas_coords: Vec<Vec2> = vec![];
    for x in (0..CANVAS_SIZE.x).step_by(2) {
        for y in 0..CANVAS_SIZE.y {
            canvas_coords.push(Vec2::from((x, y)));
        }
    }

    let open_coords: Vec<&Vec2> = canvas_coords.iter().filter(|&coord| {
        !(snake_coords.contains(coord) || pellet_coord == coord)
    }).collect();

    **open_coords.choose(&mut rand::thread_rng()).expect("How are there no open spots left OMG")
}

fn draw_blocks(printer: &Printer, pellet_coord: &Vec2, snake_coords: &VecDeque<Vec2>) {
    printer.with_color(*PELLET_COLOR, |printer| {
        printer.print(pellet_coord, &*BLOCK);
    });

    printer.with_color(*SNAKE_COLOR, |printer| {
        for coord in snake_coords {
            printer.print(coord, &*BLOCK);
        }
    });
}