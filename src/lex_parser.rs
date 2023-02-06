use std::{collections::HashMap, path::PathBuf, fs::File, io::{BufReader, BufRead}, };

#[derive(Debug, PartialEq)]
enum LexState {
    Declarations,
    TranslationRules,
    Functions,
    Finish,
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

                    let cloned_key = &key.clone();
                    self.declarations.insert(key, value);

                    // With the assumption that the value which should be resolved exist,
                    // try to resolve reference declarations.
                    let mut resolved = String::new();
                    Self::resolve_referece(&cloned_key, &mut resolved, &self.declarations);
                    self.declarations.entry(cloned_key.to_string())
                                     .and_modify(|v| *v = resolved);

                }
            });

        println!("declarations {:?}", self.declarations);

        self.parser_state = LexState::Finish;
    }

    fn is_parse_finish(&self) -> bool {
        return self.parser_state == LexState::Finish;
    }

    fn resolve_referece(key: &String, resolved: &mut String, decl_map: &HashMap<String, String>) {
        // TODO: Impl memorize
        // if memo.contains(key) { resolved.push_str(memo.get(key).unwrap()); return; }

        match decl_map.get(key) {
            Some(value) => {
                if !value.contains("{") || !value.contains("}") {
                    resolved.push_str(&value);
                    // TODO: Impl memorize
                    // memo.insert(value);
                    return;
                } else {
                    let mut is_in_bracket = false;

                    let mut tmp_key = String::new();
                    value.chars().for_each(|c| {
                        match c {
                            '{' => {
                                if is_in_bracket {
                                    panic!("invalid parse: duplicate :{{");
                                }
                                is_in_bracket = true;
                                tmp_key.clear();
                            },
                            '}' => {
                                if !is_in_bracket {
                                    panic!("invalid parse: duplicate :}}");
                                }
                                is_in_bracket = false;
                                Self::resolve_referece(&tmp_key, resolved, decl_map);
                            },
                            _ => {
                                if is_in_bracket {
                                    tmp_key.push(c);
                                } else {
                                    resolved.push(c);
                                }
                            }
                        }
                    });
                }
            },
            None => (),
        }
    }
}


mod tests {
    use super::*;

    #[test]
    fn declarations_test() {
        let mut input = super::PathBuf::new();
        input.push("./example.l");

        let mut parser = LexParser::new(input);

        parser.exec();

        assert_eq!(parser.declarations.get("delim"), Some(&r"[ \t]+".to_string()));
        assert_eq!(parser.declarations.get("ws"), Some(&r"[ \t]++".to_string()));
        assert_eq!(parser.declarations.get("letter"), Some(&r"[A-Za-z]".to_string()));
        assert_eq!(parser.declarations.get("digit"), Some(&r"[0-9]".to_string()));
        assert_eq!(parser.declarations.get("ident"), Some(&r"[A-Za-z]([A-Za-z]|[0-9])*".to_string()));
        assert_eq!(parser.declarations.get("number"), Some(&r"[0-9]+(\.[0-9]+)?(E[+\-]?[0-9]+)?".to_string()));
    }

    #[test]
    fn resolve_reference_test() {
        let mut decl_map = HashMap::new();
        decl_map.insert("2".to_string(), "3".to_string());
        decl_map.insert("1".to_string(), "{2}".to_string());
        decl_map.insert("c".to_string(), "bar".to_string());
        decl_map.insert("b".to_string(), "{c}{1}".to_string());
        decl_map.insert("a".to_string(), "{b}foo{b}".to_string());
        let mut resolved = String::new();

        LexParser::resolve_referece(&"a".to_string(), &mut resolved, &decl_map);

        assert_eq!(resolved, "bar3foobar3".to_string());
    }

    #[test]
    fn extract_curly_brackets_empty_test() {
        let mut decl_map = HashMap::new();
        decl_map.insert("a".to_string(), "{b}foo{b}".to_string());
        let mut resolved = String::new();
        LexParser::resolve_referece(&"hoge".to_string(), &mut resolved, &decl_map);
        assert_eq!(resolved, "".to_string());
    }

    #[test]
    #[should_panic(expected = "invalid parse: duplicate :{")]
    fn extract_curly_brackets_duplicate_error_test() {
        let mut decl_map = HashMap::new();
        decl_map.insert("a".to_string(), "{{b}".to_string());
        let mut resolved = String::new();
        LexParser::resolve_referece(&"a".to_string(), &mut resolved, &decl_map);
    }

    #[test]
    #[should_panic(expected = "invalid parse: duplicate :}")]
    fn extract_curly_brackets_duplicate_error_test_2() {
        let mut decl_map = HashMap::new();
        decl_map.insert("a".to_string(), "{b}}".to_string());
        let mut resolved = String::new();

        LexParser::resolve_referece(&"a".to_string(), &mut resolved, &decl_map);
    }

}