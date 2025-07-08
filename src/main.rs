use std::{
    io::{BufReader, Read, Write, stdout},
    path::PathBuf,
};

use candle_core::Tensor;
use candle_transformers::models::mimi::candle_nn::var_builder;
use clap::{Parser, Subcommand};
use clio::{Input, Output};

mod anathema_app;
#[derive(Parser, Debug)]
#[command(version, about)]
#[command(propagate_version = true)]
struct Args {
    #[arg(short, long, value_parser, default_value = "-")]
    input: Input,
    #[arg(short, long, value_parser, default_value = "-")]
    output: Output,
    #[arg(short, long)]
    model: PathBuf,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Chat {
        #[arg(short('p'), long, default_value = "\n>>:")]
        custom_prompt: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let device = candle_core::Device::Cpu;

    let mut file = BufReader::new(std::fs::File::open(args.model)?);

    let content = candle_core::quantized::gguf_file::Content::read(&mut file)?;

    let mut tensors = Vec::with_capacity(content.tensor_infos.len());
    for (n, i) in &content.tensor_infos {
        let n_t = content.tensor(&mut file, n, &device)?;
        tensors.push(n_t)
    }
    for (k, v) in &content.metadata {
        println!("{k},{v:?}");
    }
    stdout().flush()?;
    Ok(())
}
