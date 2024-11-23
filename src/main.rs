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
fn mouse_to_grid(app: &App, model: &Model) -> (u32, u32)
{
    let mut mouse_pos = app.mouse.position();
    mouse_pos.x = map_range(mouse_pos.x, app.window_rect().left(), app.window_rect().right(), 0.0, model.grid.get_width() as f32);
    mouse_pos.y = map_range(mouse_pos.y, app.window_rect().top(), app.window_rect().bottom(), 0.0, model.grid.get_height() as f32);

    let clicked_x = mouse_pos.x.floor() as u32;
    let clicked_y = mouse_pos.y.floor() as u32;

    (clicked_x, clicked_y)
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for y in 0..model.grid.get_height() {
        for x in 0..model.grid.get_width() {
            let color: Srgb<u8>;
            match model.grid.get_cell(x,y) {
                CellState::Empty => color = BLACK,
                CellState::Head => color = RED,
                CellState::Tail => color = BLUE,
                CellState::Conductor => color = YELLOW,
            }

            let cell_width = app.window_rect().w() / model.grid.get_width() as f32;
            let cell_height = app.window_rect().h() / model.grid.get_height() as f32;

            let cell_x = app.window_rect().left() + cell_width * x as f32 + cell_width/2.0;
            let cell_y = app.window_rect().top() - cell_height * (y+1) as f32 + cell_height/2.0;

            draw.rect().color(color).w(cell_width).h(cell_height).x(cell_x).y(cell_y);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}