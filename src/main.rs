mod builtin;
mod interp;
mod parser;
mod tast;
mod ty;

use anyhow::{anyhow, Context};
use std::fs;
use std::path::PathBuf;

mod cli {
    use clap::{App, Arg, ArgGroup};

    pub fn mk<'a, 'b>() -> App<'a, 'b> {
        let dump_ast = Arg::with_name("dump-ast").long("--ddump-ast");
        let debug_group = ArgGroup::with_name("debug")
            .args(&["dump-ast"])
            .multiple(false);

        let files = Arg::with_name("file").required(true);
        App::new("fangc")
            .arg(dump_ast)
            .group(debug_group)
            .arg(files)
    }
}

fn main() -> anyhow::Result<()> {
    let args = cli::mk().get_matches();

    let file: PathBuf = args
        .value_of_lossy("file")
        .map(|f| PathBuf::from(f.to_string()))
        .unwrap();
    let file =
        fs::read_to_string(&file).context(anyhow!("File doesn't exist: {}", file.display()))?;

    let ast = parser::parse(&file);

    if let Err(errors) = ast {
        for err in errors {
            eprintln!("{}", parser::WithCode::new(&file, &err));
        }
        std::process::exit(1);
    }

    if args.is_present("dump-ast") {
        println!("{:#?}", ast);
    }

    Ok(())
}
