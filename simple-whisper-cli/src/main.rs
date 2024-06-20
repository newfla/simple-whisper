use std::{path::PathBuf, str::FromStr};

use clap::{Parser, Subcommand};
use simple_whisper::{Language, Model, WhisperBuilder};
use strum::IntoEnumIterator;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Debug, Subcommand)]
enum Commands {
    /// Provide information on supported languages
    Languages {
        #[command(subcommand)]
        sub_command: LangCommands,
    },
    /// Provide information on supported models
    Models {
        #[command(subcommand)]
        sub_command: ModelCommands,
    },
    /// Transcribe audio file
    Transcribe {
        /// The audio file
        file: PathBuf,

        /// Which whisper model to use
        model: Model,

        /// Audio language
        language: Language,

        /// Ignore cached model files
        #[arg(long, required = false)]
        ignore_cache: bool,
    },
}

#[derive(Debug, Subcommand)]
enum LangCommands {
    /// List supported languages
    List,

    /// Check if a language is supported by providing its code
    Check {
        /// The code associated to the language
        code: String,
    },
}

#[derive(Debug, Subcommand)]
enum ModelCommands {
    /// List supported models
    List,
    /// Download a model by providing its code
    Download {
        /// The code associated to the model
        code: String,

        /// Ignore cached model files
        #[arg(long, required = false)]
        ignore_cache: bool,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Languages { sub_command } => match sub_command {
            LangCommands::List => {
                for lang in Language::iter() {
                    println!("{lang}")
                }
            }
            LangCommands::Check { code } => match Language::from_str(&code) {
                Ok(lang) => println!("{lang} is supported"),
                Err(_) => println!("{code} not associated to any supported language"),
            },
        },
        Commands::Transcribe {
            file,
            model,
            language,
            ignore_cache,
        } => {
            match WhisperBuilder::default()
            .language(language)
            .model(model)
            .force_download(ignore_cache)
            .build() {
                Ok(model) => {
                    let _ = model.transcribe(file).await;
                },
                Err(err) => println!("{err} occured\nAborting!")
            }
        },
        Commands::Models { sub_command } => match sub_command {
            ModelCommands::List => {
                for model in Model::iter() {
                    println!("{model}")
                }
            }
            ModelCommands::Download { code, ignore_cache } => match Model::from_str(&code) {
                Ok(model) => {
                    if let Err(err) = model.download_model(true, ignore_cache).await {
                        println!("Error {err}.\nAborting!");
                    } else {
                        println!("Download completed");
                    }
                }
                Err(_) => println!("{code} not associated to any supported model"),
            },
        },
    }
}
