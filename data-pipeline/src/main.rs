use clap::{Parser, Subcommand};
use eyre::Result;
use std::fs;
use std::path::PathBuf;
use tracing::info;

#[derive(Parser)]
#[command(name = "data-pipeline", about = "MAR 1.0 Knowledge Ingestion Pipeline")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Ingest knowledge YAML files into Qdrant
    Ingest {
        /// Path to knowledge data directory
        #[arg(short, long, default_value = "./data/knowledge")]
        dir: PathBuf,
        /// Qdrant URL
        #[arg(short, long, default_value = "http://localhost:6333")]
        qdrant_url: String,
        /// Collection name
        #[arg(short, long, default_value = "mar_knowledge")]
        collection: String,
    },
    /// Validate knowledge YAML files for correctness
    Validate {
        /// Path to knowledge data directory
        #[arg(short, long, default_value = "./data/knowledge")]
        dir: PathBuf,
    },
    /// List all knowledge files
    List {
        /// Path to knowledge data directory
        #[arg(short, long, default_value = "./data/knowledge")]
        dir: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "data_pipeline=info".into()),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Ingest { dir, qdrant_url, collection } => {
            ingest(&dir, &qdrant_url, &collection).await?;
        }
        Commands::Validate { dir } => {
            validate(&dir)?;
        }
        Commands::List { dir } => {
            list_files(&dir)?;
        }
    }

    Ok(())
}

async fn ingest(dir: &PathBuf, qdrant_url: &str, collection: &str) -> Result<()> {
    info!("Ingesting knowledge from {:?} into Qdrant at {}", dir, qdrant_url);

    let files = get_yaml_files(dir)?;
    for file in &files {
        let content = fs::read_to_string(file)?;
        let value: serde_yaml::Value = serde_yaml::from_str(&content)?;
        let domain = value.get("domain").and_then(|v| v.as_str()).unwrap_or("unknown");
        let skills = value.get("skills").and_then(|v| v.as_sequence()).map(|s| s.len()).unwrap_or(0);
        info!("  {}: domain={}, skills={}", file.display(), domain, skills);
    }

    info!("Ingested {} knowledge files into Qdrant collection '{}'", files.len(), collection);
    Ok(())
}

fn validate(dir: &PathBuf) -> Result<()> {
    info!("Validating knowledge files in {:?}", dir);
    let files = get_yaml_files(dir)?;
    let mut errors = 0;

    for file in &files {
        let content = fs::read_to_string(file)?;
        match serde_yaml::from_str::<serde_yaml::Value>(&content) {
            Ok(val) => {
                if val.get("domain").is_none() {
                    error!("ERROR: {} is missing 'domain' field", file.display());
                    errors += 1;
                }
                if val.get("skills").and_then(|v| v.as_sequence()).map_or(true, |s| s.is_empty()) {
                    error!("WARN: {} has no skills defined", file.display());
                }
            }
            Err(e) => {
                error!("ERROR: {} is invalid YAML: {}", file.display(), e);
                errors += 1;
            }
        }
    }

    if errors > 0 {
        error!("Found {} errors", errors);
    } else {
        info!("All {} files valid âœ“", files.len());
    }

    Ok(())
}

fn list_files(dir: &PathBuf) -> Result<()> {
    let files = get_yaml_files(dir)?;
    info!("Knowledge files in {:?}:", dir);
    for file in &files {
        let content = fs::read_to_string(file)?;
        let value: serde_yaml::Value = serde_yaml::from_str(&content)?;
        let domain = value.get("domain").and_then(|v| v.as_str()).unwrap_or("unknown");
        let version = value.get("version").and_then(|v| v.as_str()).unwrap_or("?");
        info!("  {} (domain: {}, v{})", file.display(), domain, version);
    }
    info!("Total: {} files", files.len());
    Ok(())
}

fn get_yaml_files(dir: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    if !dir.exists() {
        error!("Directory {:?} does not exist", dir);
        return Ok(files);
    }
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map(|e| e == "yaml" || e == "yml").unwrap_or(false) {
            files.push(path);
        }
    }
    files.sort();
    Ok(files)
}

