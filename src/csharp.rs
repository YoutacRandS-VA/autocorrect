// autocorrect: false
use super::*;
use pest::iterators::Pair;
use pest::Parser as P;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "peg/csharp.pest"]
struct CSharpParser;

pub fn format_csharp(text: &str, lint: bool) -> String {
  let result = CSharpParser::parse(Rule::item, text);
  match result {
    Ok(items) => {
      let mut out = String::new();
      for item in items {
        format_csharp_pair(&mut out, item, lint);
      }
      return out;
    }
    Err(_err) => {
      return String::from(text);
    }
  }
}

fn format_csharp_pair(text: &mut String, item: Pair<Rule>, lint: bool) {
  let (line, col) = item.as_span().start_pos().line_col();
  let part = item.as_str();

  match item.as_rule() {
    Rule::string | Rule::comment => format_or_lint(text, part, true, lint, line, col),
    Rule::item => {
      for sub in item.into_inner() {
        format_csharp_pair(text, sub, lint);
      }
    }
    _ => format_or_lint(text, part, true, lint, line, col),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_format_csharp() {
    let example = r###"
/**
 * 第1行注释
 * 第2行注释
 */
public String helloWorld(stirng name) {
  // 第3行注释
  string singleLineString = "第1个字符串string";
  string stringLiteral = $"这是stringLiteral {name}!";

  string quotation = @"
  这是多行string第1行
  这是多行string第2行
  ";
}
"###;

    let expect = r###"
/**
 * 第 1 行注释
 * 第 2 行注释
 */
public String helloWorld(stirng name) {
  // 第 3 行注释
  string singleLineString = "第 1 个字符串 string";
  string stringLiteral = $"这是 stringLiteral {name}!";

  string quotation = @"
  这是多行 string 第 1 行
  这是多行 string 第 2 行
  ";
}
"###;

    assert_eq!(expect, format_csharp(example, false));
  }
}