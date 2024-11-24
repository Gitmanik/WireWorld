mod wireworld;
use crate::wireworld::*;

use nannou::prelude::*;

struct Model {
    grid: wireworld::Grid,
    update_last_millis: u128,
    update_every_millis: u128,
    paint_current: CellState,
}

fn model(app: &App) -> Model {

    app.new_window()
        .title("WireWorld")
        .size(1024, 1024)
        .view(view)
        .build()
        .unwrap();

    app.set_loop_mode(LoopMode::rate_fps(60.0));

    let mut grid = wireworld::Grid::from_file("grid.txt");
    if grid.is_err() {
        grid = Ok(wireworld::Grid::new(50, 50));
    }

    Model {
        grid: grid.unwrap(),
        update_last_millis: 0,
        update_every_millis: 100,
        paint_current: CellState::Conductor,
    }
}

fn main() {
    println!("WireWorld - Paweł Reich s193682 2024");

    nannou::app(model)
        .event(event)
        .run();
}
fn event(app: &App, model: &mut Model, event: Event) {
    if let Event::WindowEvent {id: _, simple: window_event } = event {
        if window_event.is_none() { return }
        let window_event_unwrapped = window_event.unwrap();
        match window_event_unwrapped {
            WindowEvent::MousePressed(MouseButton::Left) => {
                let mouse_grid_pos = mouse_to_grid(app, model);
                model.grid.set_cell(mouse_grid_pos.0, mouse_grid_pos.1, model.paint_current.clone());
            }
            WindowEvent::MouseMoved(_) =>
            {
                if app.mouse.buttons.left().is_down(){
                    let mouse_grid_pos = mouse_to_grid(app, model);
                    model.grid.set_cell(mouse_grid_pos.0, mouse_grid_pos.1, model.paint_current.clone());
                }
            }
            WindowEvent::KeyReleased(_key) =>
            {
                match _key {
                    Key::S => model.grid.to_file("current.txt").unwrap_or(()),
                    Key::P => model.grid.pretty_print(),
                    Key::Z => model.paint_current = CellState::Conductor,
                    Key::X => model.paint_current = CellState::Head,
                    Key::C => model.paint_current = CellState::Tail,
                    Key::V => model.paint_current = CellState::Empty,
                    _ => {}
                }
            }
            WindowEvent::DroppedFile(path) => {
                println!("dropped file {:?}", path);

                let new_grid = wireworld::Grid::from_file(path.to_str().unwrap());
                if new_grid.is_ok() {
                    model.grid = new_grid.unwrap();
                }
            }
            _ => { }
        }
    } else if let Event::Update(update_event) = event {
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

fn cell_to_color(cell: CellState) -> Srgb<u8>
{
    match cell {
        CellState::Empty => BLACK,
        CellState::Head => BLUE,
        CellState::Tail => RED,
        CellState::Conductor => YELLOW,
    }
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    let mouse_grid_pos = mouse_to_grid(app, model);

    for y in 0..model.grid.get_height() {
        for x in 0..model.grid.get_width() {

            let color: Srgb<u8> = cell_to_color(model.grid.get_cell(x as i32, y as i32));

            let cell_width = app.window_rect().w() / model.grid.get_width() as f32;
            let cell_height = app.window_rect().h() / model.grid.get_height() as f32;

            let cell_x = app.window_rect().left() + cell_width * x as f32 + cell_width/2.0;
            let cell_y = app.window_rect().top() - cell_height * (y+1) as f32 + cell_height/2.0;

            draw.rect().color(color).w(cell_width).h(cell_height).x(cell_x).y(cell_y);

            if mouse_grid_pos.0 == x && mouse_grid_pos.1 == y {

                draw.rect().no_fill().stroke(cell_to_color(model.paint_current.clone())).stroke_weight(3.0).w(cell_width).h(cell_height).x(cell_x).y(cell_y);
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
}