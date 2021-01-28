use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

// TODO:
//  1. load the Roboto Mono font into the program
//  2. attempt writing text (and bold text) on to the document
//  3. find the best way to write each text portion onto the document
//  4. write all the sample text
//  5. create a program to output the pdf

fn main() {
    let (doc, page1, layer1) = PdfDocument::new("Assignment Template", Mm(215.9), Mm(279.4), "Layer 1");
    let layer = doc.get_page(page1).get_layer(layer1);

    let title_points = vec![
        (Point::new(Mm(6.4), Mm(273.0)), false),   // 273   => top line
        (Point::new(Mm(6.4), Mm(228.6)), false),   // 228.6 => bottom line
        (Point::new(Mm(209.5), Mm(228.6)), false), // 6.4   => left edge
        (Point::new(Mm(209.5), Mm(273.0)), false), // 209.5 => right edge
    ];
    let body_points = vec![
        (Point::new(Mm(6.4), Mm(222.3)), false),   // 222.3 => top line
        (Point::new(Mm(6.4), Mm(6.4)), false),     // 6.4   => bottom line
        (Point::new(Mm(209.5), Mm(6.4)), false),   // 6.4   => left edge
        (Point::new(Mm(209.5), Mm(222.3)), false), // 209.5 => right edge
    ];
    let title_box = Line {
        points: title_points,
        is_closed: true,
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };
    let body_box = Line {
        points: body_points,
        is_closed: true,
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
    };

    layer.set_outline_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)));
    layer.set_outline_thickness(5.0);

    layer.add_shape(title_box);
    layer.add_shape(body_box);

    doc.save(&mut BufWriter::new(File::create("asgn_t.pdf").unwrap())).unwrap();
}
