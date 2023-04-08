use anyhow::{Context, Result};
use clap::Parser;
use std::{
    fs::File,
    io::{stdout, BufRead, BufReader, Write},
};

/// concatenate and print files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// names of files to print
    #[arg(required = true)]
    file_names: Vec<String>,

    /// print line numbers
    #[arg(short = 'n')]
    print_indices: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let print = if args.print_indices {
        print_lines_with_index
    } else {
        print_lines
    };

    for file_name in args.file_names.iter() {
        let file = BufReader::new(File::open(file_name)?);
        let it = file.lines().filter_map(Result::ok);
        print(stdout(), it).with_context(|| format!("print lines from {}", file_name))?;
    }

    Ok(())
}

fn print_lines<I, R, W>(mut w: W, lines: I) -> Result<()>
where
    I: Iterator<Item = R>,
    R: AsRef<str>,
    W: Write,
{
    for line in lines {
        w.write_all(format!("{}\n", line.as_ref()).as_bytes())?;
    }
    Ok(())
}

fn print_lines_with_index<I, R, W>(mut w: W, lines: I) -> Result<()>
where
    I: Iterator<Item = R>,
    R: AsRef<str>,
    W: Write,
{
    for (index, line) in lines.enumerate() {
        w.write_all(format!("{} {}\n", index + 1, line.as_ref()).as_bytes())?;
    }
    Ok(())
}

#[test]
fn test_print_lines_empty() -> Result<()> {
    let mut output = Vec::new();
    print_lines(&mut output, Vec::<String>::new().iter())?;
    assert!(String::from_utf8(output)?.is_empty());
    Ok(())
}

#[test]
fn test_print_lines() -> Result<()> {
    let mut output = Vec::new();
    let lines = vec!["foo", "bar", "baz"];
    print_lines(&mut output, lines.iter())?;
    assert_eq!(String::from_utf8(output)?, "foo\nbar\nbaz\n");
    Ok(())
}

#[test]
fn test_print_lines_with_index() -> Result<()> {
    let mut output = Vec::new();
    let lines = vec!["foo", "bar", "baz"];
    print_lines_with_index(&mut output, lines.iter())?;
    assert_eq!(String::from_utf8(output)?, "1 foo\n2 bar\n3 baz\n");
    Ok(())
}
