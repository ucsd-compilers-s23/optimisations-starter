use std::{
    env,
    fs::File,
    io::{self, Read},
};

mod parser;
mod syntax;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let in_name = &args[1];
    let out_name = &args[2];
    let mut in_contents = String::new();
    let mut in_file = File::open(in_name)?;
    in_file.read_to_string(&mut in_contents)?;
    let expr = parser::parse(&in_contents);
    println!("{expr:#?}");
    // let result = compile(&expr);

    Ok(())
}
