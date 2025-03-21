use bio::io::fasta;
use clap::Parser;
use std::collections::HashSet;
use std::error::Error;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about,long_about = None)]

struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    remove_list: String,

    #[arg(long)]
    output_not_matched: String,

    #[arg(long)]
    output_matched: String,
}

// load a fasta file
// then remove specific records

fn readtxtfile(filename: &str) -> Result<String, Box<dyn Error>> {
    fs::read_to_string(filename).map_err(|e| e.into())
}

fn main() -> Result<(), Box<dyn Error>> {
    // load the args parser
    let args = Args::parse();

    // load data input
    let fastafile = fasta::Reader::from_file(&args.input)?;
    let recids = readtxtfile(&args.remove_list)?;

    let remove_list: HashSet<&str> = recids
        .lines()
        .map(|lines| lines.trim().split_whitespace().next().unwrap_or(""))
        .filter(|id| !id.is_empty())
        .collect();

    // create a FASTA writer

    let mut cleaned_ref = fasta::Writer::to_file(&args.output_not_matched)?;
    let mut preserved = fasta::Writer::to_file(&args.output_matched)?;
    // elaborate an output variable to then write out into a file

    // Goal: pick the IDs of the txt file and use them to create a new FASTA file without those IDs

    for result in fastafile.records() {
        let record = result?;
        let ids = record.id();
        //let seq = record.seq();

        if !remove_list.contains(ids) {
            println!("Saving record {} to {}", ids, args.output_not_matched);
            cleaned_ref.write(ids, record.desc(), record.seq())?;
        } else {
            println!("Saving matched sequence {} to {}", ids, args.output_matched);
            preserved.write(ids, record.desc(), record.seq())?;
        }
    }

    // first print each record in the file
    Ok(())
}
