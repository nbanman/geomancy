use geomancy::make_pattern::make_pattern;
use svg::{
    Document,
    node::element::{Path, path::Data},
};

fn main() {
    let lines = make_pattern(3, false, false, false);

    let mut data = Data::new();
    for [from, to] in lines {
        data = data.move_to((from.x, from.y)).line_to((to.x, to.y));
    }
    data = data.close();

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 0.1)
        .set("d", data);

    let document = Document::new()
        .set("viewBox", (-20, -20, 40, 40))
        .set("width", "30cm")
        .set("height", "30cm")
        .add(path);

    svg::save("image.svg", &document).unwrap();
}
