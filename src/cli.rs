use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "pkg-cmp")]
#[command(version = "0.1.0")]
#[command(about = "Command line arguments for alt linux package comparator", long_about = None)]
pub struct Args {
    #[arg(short = 'a', long = "arch")]
    arch: Option<String>,

    /// name of first comparable branch, by default - sisyphus
    #[arg(short = 'f', long = "first-branch", default_value = "sisyphus")]
    first_branch: String,

    /// name of second comparable branch, by default - p10
    #[arg(short = 's', long = "second_branch", default_value = "p10")]
    second_branch: String,
}

impl Args {
    pub fn arch(&self) -> &Option<String> {
        &self.arch
    }
    pub fn first_branch(&self) -> &str {
        self.first_branch.as_str()
    }
    pub fn second_branch(&self) -> &str {
        self.second_branch.as_str()
    }
}
