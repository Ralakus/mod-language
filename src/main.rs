#![feature(box_syntax)]

extern crate mod_language;

use mod_language::{
  session::SESSION,
  source::SOURCE_MANAGER,
  lexer::Lexer,
  parser::Parser,
  analyzer::Analyzer,
  ansi,
};


fn main () -> std::io::Result<()> {
  if !ansi::enable() { println!("Failed to enable ansi coloring for terminal") }
  else { println!("\n{}Ansi coloring enabled for terminal{}\n", ansi::Foreground::Green, ansi::Foreground::Reset) }
  

  SESSION.init();
  SOURCE_MANAGER.init();


  let source = SOURCE_MANAGER.load("./test_scripts/item_analysis.ms")?;


  let mut lexer = Lexer::new(source);

  let stream = lexer.lex_stream();

  println!("Got token stream, dumping to ./log/stream");
  if !std::path::Path::new("./log").exists() { std::fs::create_dir("./log").expect("Failed to create ./log dir"); }
  std::fs::write("./log/stream", format!("{:#?}", stream)).expect("Failed to dump token stream to ./log/stream");


  let mut parser = Parser::new(&stream);

  let ast = parser.parse_ast();

  println!("Got ast, dumping to ./log/ast");
  std::fs::write("./log/ast", format!("{:#?}", ast)).expect("Failed to dump token ast to ./log/ast");


  let analyzer = Analyzer::new(&ast);

  let context = analyzer.analyze();

  println!("Got context, dumping to ./log/context");
  std::fs::write("./log/context", format!("{:#?}", context)).expect("Failed to dump context to ./log/context");

  let lib = context.items.get(context.lib_mod).unwrap().ref_module().unwrap();

  let export_key = lib.export_bindings.get_entry("Y").expect("Lib does not export item named Y");

  println!("Got exported item Y: {:#?}", context.items.get(export_key).expect("Lib exports invalid key for item named Y"));

  if !SESSION.messages().is_empty() {
    SESSION.print_messages();
    panic!("Error parsing items");
  }


  Ok(())
}