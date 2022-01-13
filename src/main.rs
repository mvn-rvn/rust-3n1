use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{LineStyle, LineJoin};
use plotlib::view::ContinuousView;
use std::io;

fn main() {
    //make data vector
    let mut data: Vec<(f64, f64)> = Vec::new();
    //make x value
    let mut x: f64 = 1.0;
    //ask user for input
    let mut input = String::new();
    println!("Input number: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read input.");
    //convert input to x
    let mut y = input
        .trim()
        .parse::<f64>()
        .unwrap();
    //main 3n+1 loop
    loop {
        data.push((x, y));

        if y == 1.0 {
            break;

        } else if y % 2.0 == 0.0 {
            y /= 2.0;

        } else {
            y *= 3.0;
            y += 1.0;

        }
        x += 1.0;
    }
    //make graph
    let graph = Plot::new(data)
        .line_style(
            LineStyle::new()
                .colour("red")
                .linejoin(LineJoin::Round)
        );
    //export graph to svg
    let out = ContinuousView::new().add(graph);
    Page::single(&out)
        .save("out.svg")
        .unwrap();
}