use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

fn main() {
    let (doc, page1, layer1) = PdfDocument::new("Assignment Template", Mm(215.9), Mm(279.4), "Layer 1");
    doc.save(&mut BufWriter::new(File::create("asgn_t.pdf").unwrap())).unwrap();
}
