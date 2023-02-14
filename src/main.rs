use clap::Parser;

#[derive(Default, Parser, Debug)]
#[clap(author = "phnaharris", version, about = "OCR tool")]
struct Args {
    /// File path
    #[clap(short = 'p', long = "path")]
    path: String,
}

fn main() {
    let args = Args::parse();

    let language = String::from("eng");
    let mut lt = leptess::LepTess::new(None, &language).unwrap();

    let _ = lt.set_image(args.path);
    let _ = lt.set_variable(leptess::Variable::TesseditPagesegMode, "3");
    // let _ = lt.set_variable(leptess::Variable::TesseditOcrEngineMode, "3");

    println!(
        "Result: \n{}",
        lt.get_utf8_text().expect("it must be a ocr result")
    );
}
