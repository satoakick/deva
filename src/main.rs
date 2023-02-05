use std::fs::File;
use std::io::Write;

use std::path::PathBuf;
use structopt::StructOpt;
use indoc::indoc;

use deva::lex_parser::LexParser;

/// Welcome to deva!
#[derive(StructOpt, Debug)]
#[structopt(name = "deva")]
struct Opt {
    /// input file
    #[structopt(name = "input", short = "i", long = "--input", parse(from_os_str))]
    input_file: PathBuf,

    /// output file
    #[structopt(name = "output", short ="o", long = "--output", parse(from_os_str))]
    output_file: PathBuf,
}

fn output(output: &PathBuf) {
    let deps = format!(indoc! {r#"
        // dependencies
        // use std::collection;
    "#});

    let source = format!(indoc! {r#"
        // Write Here, place the source for analyzing which will be generated.
        println!("foo bar");
    "#});

    let template = format!(indoc! {r#"
        {deps}
        fn main() {{
            {source}
        }}
    "#}, source=source, deps=deps);

    let mut result = File::create(output).unwrap();
    result.write_all(template.as_bytes());

}

fn main() {
    let opt = Opt::from_args();
    let input = opt.input_file;
    let mut lex_parser = LexParser::new(input);

    lex_parser.exec();
    println!("{:#?}", lex_parser.declarations);

    output(&opt.output_file);
}
