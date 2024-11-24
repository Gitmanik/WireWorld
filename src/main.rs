mod wireworld;
use crate::wireworld::*;

use nannou::prelude::*;

struct Model {
    grid: wireworld::Grid,
    update_last_millis: u128,
    update_every_millis: u128,
}

fn model(_app: &App) -> Model {
    let mut model = Model {
        grid: wireworld::Grid::new(50,50),
        update_last_millis: 0,
        update_every_millis: 100,
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
    if let Event::WindowEvent {id: _, simple: window_event } = event {
        if window_event.is_none() { return }
        let window_event_unwrapped = window_event.unwrap();
        match window_event_unwrapped {
            WindowEvent::MouseMoved(_) =>
            {
                if app.mouse.buttons.left().is_down(){
                    let mouse_grid_pos = mouse_to_grid(&app, &model);
                    model.grid.set_cell(mouse_grid_pos.0, mouse_grid_pos.1, CellState::Conductor);
                }
            }
            _ => { }
        }
    }else if let Event::Update(update_event) = event {
        if update_event.since_start.as_millis() - model.update_last_millis > model.update_every_millis
        {
            model.update_last_millis = update_event.since_start.as_millis();
            model.grid.tick();
        }
    }
}

fn mouse_to_grid(app: &App, model: &Model) -> (u32, u32) {
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

    let mouse_grid_pos = mouse_to_grid(&app, &model);

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


            if mouse_grid_pos.0 == x as u32 && mouse_grid_pos.1 == y as u32 {
                draw.rect().no_fill().stroke(WHITE).stroke_weight(3.0).w(cell_width).h(cell_height).x(cell_x).y(cell_y);
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
}