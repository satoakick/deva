use std::fs::File;
use std::io::{BufReader, BufRead, Write};
use std::collections::HashMap;

use std::path::PathBuf;
use structopt::StructOpt;
use indoc::indoc;

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

#[derive(Debug, PartialEq)]
enum LexState {
    Declarations,
    TranslationRules,
    Functions,
}

fn parse_lex(input: &PathBuf, decls: &mut HashMap<String, String>) {
    let mut lex_state = LexState::Declarations;
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
        .filter(|line| 
            !line.as_ref().unwrap().trim().is_empty()
        )
        .for_each(|line| {
            println!("lex state is {:#?}", lex_state);

            let row = line.unwrap();
            if row.trim() == "%%" {
                match lex_state {
                   LexState::Declarations => { lex_state = LexState::TranslationRules; }, 
                   LexState::TranslationRules => { lex_state = LexState::Functions; }, 
                   _ => panic!("lex state is invalid")
                }
                println!("change lex status {}", row);
            } else if lex_state == LexState::Declarations {
                let mut iter = row.split_whitespace();
                let key = iter.next().unwrap().to_string();
                let value = iter.collect::<Vec<_>>().join(" ");

                decls.insert(key, value);
            }
        });

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

    let mut declarrations: HashMap<String, String> = HashMap::new();

    parse_lex(&input, &mut declarrations);
    println!("{:#?}", declarrations);

    output(&opt.output_file);
}

#[test]
fn declarations_test() {
    let mut input = PathBuf::new();
    input.push("./example.l");
    let mut declarations:HashMap<String, String> = HashMap::new();

    parse_lex(&input, &mut declarations);

    assert_eq!(declarations.get("delim"), Some(&r"[ \t]+".to_string()));
    assert_eq!(declarations.get("ws"), Some(&r"{delim}+".to_string()));
    assert_eq!(declarations.get("letter"), Some(&r"[A-Za-z]".to_string()));
    assert_eq!(declarations.get("digit"), Some(&r"[0-9]".to_string()));
    assert_eq!(declarations.get("ident"), Some(&r"{letter}({letter}|{digit})*".to_string()));
    assert_eq!(declarations.get("number"), Some(&r"{digit}+(\.{digit}+)?(E[+\-]?{digit}+)?".to_string()));
}
