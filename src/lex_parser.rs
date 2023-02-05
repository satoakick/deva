use std::{collections::HashMap, path::PathBuf, fs::File, io::{BufReader, BufRead}};

#[derive(Debug, PartialEq)]
enum LexState {
    Declarations,
    TranslationRules,
    Functions,
}
pub struct LexParser {
    input: PathBuf,
    parser_state: LexState,
    pub declarations: HashMap<String, String>,
}
impl LexParser {
    pub fn new(input: PathBuf) -> Self {
        Self {
            input: input,
            parser_state: LexState::Declarations,
            declarations: HashMap::new()
        }
    }
    pub fn exec(&mut self) {
        let file = File::open(&self.input).unwrap();
        let reader = BufReader::new(file);

        reader.lines()
            .filter(|line| 
                !line.as_ref().unwrap().trim().is_empty()
            )
            .for_each(|line| {
                println!("parser state is {:#?}", self.parser_state);

                let row = line.unwrap();
                if row.trim() == "%%" {
                    match self.parser_state {
                       LexState::Declarations => { self.parser_state = LexState::TranslationRules; }, 
                       LexState::TranslationRules => { self.parser_state = LexState::Functions; }, 
                       _ => panic!("parser state is invalid")
                    }
                    println!("change parser status {}", row);
                } else if self.parser_state == LexState::Declarations {
                    let mut iter = row.split_whitespace();
                    let key = iter.next().unwrap().to_string();
                    let value = iter.collect::<Vec<_>>().join(" ");

                    self.declarations.insert(key, value);
                }
            });
    }
}


mod tests {

    #[test]
    fn declarations_test() {
        let mut input = super::PathBuf::new();
        input.push("./example.l");

        let mut parser = super::LexParser::new(input);

        parser.exec();

        assert_eq!(parser.declarations.get("delim"), Some(&r"[ \t]+".to_string()));
        assert_eq!(parser.declarations.get("ws"), Some(&r"{delim}+".to_string()));
        assert_eq!(parser.declarations.get("letter"), Some(&r"[A-Za-z]".to_string()));
        assert_eq!(parser.declarations.get("digit"), Some(&r"[0-9]".to_string()));
        assert_eq!(parser.declarations.get("ident"), Some(&r"{letter}({letter}|{digit})*".to_string()));
        assert_eq!(parser.declarations.get("number"), Some(&r"{digit}+(\.{digit}+)?(E[+\-]?{digit}+)?".to_string()));
    }
}