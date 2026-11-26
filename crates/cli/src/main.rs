use std::io::{self, BufWriter, IsTerminal};

use senbonzakura_cli::Cli;

fn main() {
  let args: Vec<String> =
    std::env::args_os().skip(1).map(|arg| arg.to_string_lossy().into_owned()).collect();
  let code = senbonzakura_interpret::with_large_stack(move || {
    let terminal = io::stdin().is_terminal() && io::stdout().is_terminal();
    let mut input = io::stdin().lock();
    let mut out = BufWriter::new(io::stdout().lock());
    let mut err = io::stderr().lock();
    Cli::new(&mut input, &mut out, &mut err, terminal).run(&args)
  });
  std::process::exit(code);
}
