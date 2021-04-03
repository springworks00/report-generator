use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

// TODO:
//  3. find the best way to write each text portion onto the document
//  4. write all the sample text
//  5. create a program to output the pdf

const BODY: &str = "-- INSTRUCTIONS -------------------------------------------
(1) Create an assignment with a descriptive title under 28
letters, using the eduma command-line application.

(2) Type under 7 instructions (each under 173 letters) to 
completely finish the assignment. Instructions provided by
any other means are unallowed.

(3) Condense instructions as a whole to under 861 letters.

(4) Retype instructions without any pronouns, infinitives,
grammatical mistakes, custom abbreviations, or action verbs
(excluding each first word; each must be an action verb).

(5) Type the purpose section in under 296 letters, starting
with an infinitive, explaining the value of the assignment.
Using other infinitives or active verbs is unallowed.

(6) Submit the document with the 'grade' button, revising
and resubmitting until the lexile score is under 500.



-- PURPOSE ------------------------------------------------
To teach instructors how to create assignments. Assignments
are strictly graded and formatted, accomplishing their
information delivery through minimalism and briefness.
";

enum TextRule {
    AsgnStep,
    AsgnPurpose,
}

struct Block {
    title: String,
    lines: Vec<String>,
    rule: Rule,
}

impl Block {
    fn new() -> Self {
        Block {
            title: String::from(""),
            lines: vec![],
        }
    }
    fn set_title(&mut self) {
        // three words max
    }
    fn set_text(&mut self) {
        // break the text into lines based on available spaces.
        // cause an error if a rule is not followed
    }
}

enum DocRule {
    Asgn,
    Report,
}

struct Document {
    rule: Rule,
    blocks: Vec<Block>,
}

// if you're typing a research paper in the terminal, you should get red
// highlighted spaces if your words aren't close enough to the edge.
// the only reason for "Document" and not just Vec<Block> is the more 
// complicated document rules.


fn main() {
    let (doc, page1, layer1) = PdfDocument::new("Assignment Template", Mm(215.9), Mm(279.4), "Layer 1");
    let layer = doc.get_page(page1).get_layer(layer1);

    let font_reg = doc.add_external_font(File::open("assets/RobotoMono-Regular.ttf").unwrap()).unwrap();
    let font_bold = doc.add_external_font(File::open("assets/RobotoMono-Bold.ttf").unwrap()).unwrap();

    //let lines = vec!["Lorem Ipsum", "Borem Dipsum", "Gorum Dimsum"];
    
    //layer.use_text(BODY, 15.0, Mm(12.7), Mm(213.3), &font_reg);
    let lines: Vec<&str> = BODY.split("\n").collect();

    layer.begin_text_section();

    layer.set_font(&font_reg, 15.0);
    layer.set_line_height(20.0);
    layer.set_text_cursor(Mm(12.7), Mm(213.3));

    for line in lines {
        layer.write_text(line.clone(), &font_reg);
        layer.add_line_break();
    }

    layer.end_text_section();


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

fn load_font() {
    let text = "Lorem Ipsum";

}
