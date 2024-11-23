mod wireworld;
use crate::wireworld::*;

use nannou::prelude::*;

struct Model {
    grid: wireworld::Grid
}

fn model(_app: &App) -> Model {
    Model {
        grid: wireworld::Grid::new(50,50)
    }
}

fn main() {
    println!("WireWorld - Paweł Reich s193682 2024");

    nannou::app(model)
        .event(event)
        .simple_window(view)
        .size(1024, 1024)
        .run();
}
fn event(app: &App, model: &mut Model, event: Event) {

}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLUE);

    draw.to_frame(app, &frame).unwrap();
}