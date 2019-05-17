#[macro_use]
extern crate lazy_static;
extern crate cursive;

use cursive::traits::*;
use cursive::{Cursive, Printer, theme};
use cursive::views::{Canvas, Panel};
use cursive::theme::{BaseColor, ColorStyle};

lazy_static! {
    static ref CANVAS_SIZE: (u32, u32) = (30, 15);
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

fn main() {
    let mut siv = Cursive::default();

    let canvas = Canvas::new(())
        .with_draw(|_, printer: &Printer| {
            printer.with_color(*PELLET_COLOR, |printer| {
                printer.print((10, 4), *BLOCK);
            });

            printer.with_color(*SNAKE_COLOR, |printer| {
                printer.print((0, 1), *BLOCK);
            });
        })
        .with_id("canvas")
        .fixed_size(*CANVAS_SIZE);

    siv.add_layer(Panel::new(canvas).title("Snake"));
    siv.run();
}