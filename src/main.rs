use clap::Parser;

#[derive(Parser)]
#[command(name = "fcnt")]
#[command(version = "0.1.0")]
#[command(about = "Count the total number of files in a given directory.")]
struct CmdArgs {
    /// the directories (default: ./)
    directories: Vec<String>,

    /// count all regular and hidden files.
    #[arg(short = 'a')]
    all_files: bool,

    /// count the number of directories.
    #[arg(short = 'd')]
    count_dirs: bool,

    /// count the total size of files.
    #[arg(short = 's')]
    count_size: bool,

    /// counting recursively.
    #[arg(short = 'r')]
    recursively: bool,
}

fn main() {
    let args = CmdArgs::parse();
    println!("directories: {:?}", args.directories);
    println!("all_files: {:?}", args.all_files);
    println!("count_dirs: {:?}", args.count_dirs);
    println!("count_size: {:?}", args.count_size);
    println!("recursively: {:?}", args.recursively);
}
