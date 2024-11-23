mod wireworld;
use crate::wireworld::*;

use nannou::prelude::*;

struct Model {
    grid: wireworld::Grid
}

fn model(_app: &App) -> Model {
    let mut model = Model {
        grid: wireworld::Grid::new(50,50)
    };

    for x in 0..model.grid.get_width() {
        for y in 0..model.grid.get_height() {
            if x % 2 == y % 2 {
                model.grid.set_cell(x,y, CellState::Conductor);
            }
        }
    }

    model.grid.set_cell(1,1,CellState::Head);

    model.grid.pretty_print();

    model
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