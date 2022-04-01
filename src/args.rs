use clap::Parser;

/// Simple program to view log patterns
#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
    /// Local log pattern file
    #[clap(short, long, default_value = "reports.json")]
    pub file: String,

    /// Namespace of app
    #[clap(long)]
    pub namespace: Option<String>,

    /// Name of app
    #[clap(long)]
    pub name: Option<String>,
}
