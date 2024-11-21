use nannou::prelude::*;

fn main() {
    println!("WireWorld - Paweł Reich s193682 2024");
    nannou::sketch(view).run();
}
fn view(app: &App, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();

    // set background to blue
    draw.background().color(BLUE);

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}