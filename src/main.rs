mod wireworld;
use crate::wireworld::*;

use nannou::prelude::*;

use wasm_bindgen::prelude::wasm_bindgen;

use std::cell::RefCell;
use async_std::task::block_on;

use nannou::wgpu::{Backends, DeviceDescriptor, Limits};
use wasm_bindgen::{throw_str, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[wasm_bindgen]
pub async fn main_web() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    block_on(async {
        let model = get_model().await;
        run_app(model).await;
    });
}
pub fn main() {
    block_on(async {
        let model = get_model().await;
        run_app(model).await;
    });
}


struct Model {
    grid: Grid,
    update_last_millis: u128,
    update_every_millis: u128,
    paint_current: CellState,
    is_paused: bool
}

#[wasm_bindgen]
pub async fn fetch_file_content(url: &str) -> Result<String, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;

    let text = JsFuture::from(resp.text()?).await?;
    Ok(text.as_string().unwrap())
}

async fn get_model() -> Model {
    let mut grid;
    #[cfg(target_arch = "wasm32")]
    {
        grid = Grid::from_text(fetch_file_content("./grid.txt").await.unwrap())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        grid = Grid::from_file("grid.txt").unwrap();
    }

    Model {
        grid: grid,
        update_last_millis: 0,
        update_every_millis: 100,
        paint_current: CellState::Conductor,
        is_paused: false
    }
}

async fn create_window(app: &App) {
    let device_desc = DeviceDescriptor {
        limits: Limits {
            max_texture_dimension_2d: 8192,
            ..Limits::downlevel_webgl2_defaults()
        },
        ..Default::default()
    };

    app.new_window()
        .device_descriptor(device_desc)
        .title("WireWorld")
        .view(view)
        .build_async()
        .await
        .unwrap();
}

async fn run_app(model: Model) {
    block_on(async {
        thread_local!(static MODEL: RefCell<Option<Model>> = Default::default());
        MODEL.with(|m| m.borrow_mut().replace(model));

        app::Builder::new_async(|app| {
            Box::new(async move {
                create_window(app).await;
                MODEL.with(|m| m.borrow_mut().take().unwrap())
            })
        })
            .size(1024, 1024)
            .backends(Backends::PRIMARY | Backends::GL)
            .event(event)
            .run_async()
            .await;
    })

}

fn event(app: &App, model: &mut Model, event: Event) {
    if let Event::WindowEvent {id: _, simple: window_event } = event {
        if window_event.is_none() { return }
        let window_event_unwrapped = window_event.unwrap();
        match window_event_unwrapped {
            WindowEvent::MousePressed(MouseButton::Left) => {
                let mouse_grid_pos = mouse_to_grid(app, model);
                model.grid.set_cell(mouse_grid_pos.0, mouse_grid_pos.1, &model.paint_current);
            }
            WindowEvent::MouseMoved(_) =>
            {
                if app.mouse.buttons.left().is_down(){
                    let mouse_grid_pos = mouse_to_grid(app, model);
                    model.grid.set_cell(mouse_grid_pos.0, mouse_grid_pos.1, &model.paint_current);
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
                    Key::Space => model.is_paused = !model.is_paused,
                    _ => {}
                }
            }
            WindowEvent::DroppedFile(path) => {
                let new_grid = Grid::from_file(path.to_str().unwrap());
                if new_grid.is_ok() {
                    model.grid = new_grid.unwrap();
                }
            }
            _ => { }
        }
    } else if let Event::Update(update_event) = event {
        if model.is_paused {
            return;
        }
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

    let cell_width = app.window_rect().w() / model.grid.get_width() as f32;
    let cell_height = app.window_rect().h() / model.grid.get_height() as f32;

    let mouse_grid_pos = mouse_to_grid(app, model);

    let paint_color: Srgb<u8> = Grid::cell_to_color(&model.paint_current);

    for y in 0..model.grid.get_height() {
        for x in 0..model.grid.get_width() {

            let cell_x = app.window_rect().left() + cell_width * x as f32 + cell_width/2.0;
            let cell_y = app.window_rect().top() - cell_height * (y+1) as f32 + cell_height/2.0;

            if mouse_grid_pos.0 == x && mouse_grid_pos.1 == y {
                draw.rect().no_fill().stroke(paint_color).stroke_weight(3.0).w(cell_width).h(cell_height).x(cell_x).y(cell_y);
            }

            let cell: &CellState = model.grid.get_cell(x as i32, y as i32);

            if *cell == CellState::Empty {
                continue;
            }

            let color: Srgb<u8> = Grid::cell_to_color(cell);
            draw.rect().color(color).w(cell_width).h(cell_height).x(cell_x).y(cell_y);
        }
    }

    if model.is_paused {
        draw.text("Pause").font_size(30).align_text_middle_y();
    }

    draw.to_frame(app, &frame).unwrap();
}