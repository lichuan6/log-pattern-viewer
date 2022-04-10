use clap::Parser;

/// Simple program to view log patterns
#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
    /// Local log pattern file
    #[clap(short, long)]
    pub from_local: Option<String>,

    /// Namespace of app
    #[clap(long)]
    pub namespace: Option<String>,

    /// Name of app
    #[clap(long)]
    pub name: Option<String>,

    /// Year of report
    #[clap(short, long)]
    pub year: Option<i32>,

    /// Month of report
    #[clap(short, long)]
    pub month: Option<i32>,

    /// aws profile name
    #[clap(short, long)]
    pub profile: Option<String>,

    /// aws region name
    #[clap(long)]
    pub region: Option<String>,
}
