#[macro_use]
extern crate lazy_static;
extern crate cursive;

use cursive::traits::*;
use cursive::{Cursive, Printer, theme, event, Vec2};
use cursive::views::{Canvas, Panel, OnEventView};
use cursive::theme::{BaseColor, ColorStyle};

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

#[derive(Clone, Copy)]
struct CanvasState {
    pellet_coord: Vec2,
    snake_coord: Vec2
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    let mut siv = Cursive::default();

    let initial_canvas_state = CanvasState {
        pellet_coord: Vec2::from((10, 4)),
        snake_coord: Vec2::from((0, 1))
    };

    let canvas = Canvas::new(initial_canvas_state)
        .with_draw(move |_, printer: &Printer| {
            draw_blocks(printer,
                        initial_canvas_state.pellet_coord,
                        initial_canvas_state.snake_coord);
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
    let snake_coord= canvas_state.snake_coord;

    canvas_state.snake_coord = Vec2::from(match direction {
        Direction::Up => (snake_coord.x, snake_coord.y - 1),
        Direction::Down => (snake_coord.x, snake_coord.y + 1),
        Direction::Left => (snake_coord.x - 2, snake_coord.y),
        Direction::Right => (snake_coord.x + 2, snake_coord.y)
    });

    let curr_state = canvas_state.clone();

    if is_out_of_bounds(curr_state.snake_coord) {
        // Oh man you did a bad
        // TODO: transition to endgame state
    }

    view.set_draw(move |_, printer: &Printer| {
        draw_blocks(printer,
                    curr_state.pellet_coord,
                    curr_state.snake_coord)
    });
}

fn is_out_of_bounds(pos: Vec2) -> bool {
    // NOTE: since the coordinates are currently represented as Vec2's -> XY<usize>
    // they can't be negative and these "pos.<x|y> < 0" conditions are redundant
    // TODO: refactor CanvasState s.t. we can use negative coordinates
    pos.x < 0 || pos.x >= CANVAS_SIZE.x || pos.y < 0 || pos.y >= CANVAS_SIZE.y
}

fn draw_blocks(printer: &Printer, pellet_coord: Vec2, snake_coord: Vec2) {
    printer.with_color(*PELLET_COLOR, |printer| {
        printer.print(pellet_coord, *BLOCK);
    });

    printer.with_color(*SNAKE_COLOR, |printer| {
        printer.print(snake_coord, *BLOCK);
    });
}