mod cmdargs;
mod output;
mod walker;

use clap::Parser;
use cmdargs::CmdArgParser;
use walker::Counter;

fn main() {
    // parse cmd-line args and get directories
    let args = CmdArgParser::parse();

    // walk all files
    let directories = args.get_directories();
    let mut counters = Vec::<Counter>::new();
    if args.non_recursive {
        for dirpath in directories {
            if let Ok((_, counter)) =
                walker::walk(&dirpath, args.all_files, args.with_size, args.verbose)
            {
                counters.push(counter);
            };
        }
    } else {
        let n_thread = args.get_threads_num();
        counters = walker::parallel_walk(
            directories,
            args.all_files,
            args.with_size,
            args.verbose,
            n_thread,
        );
    }

    if args.with_size {
        counters.sort_by(|c1, c2| c2.size().cmp(&c1.size()));
    } else {
        counters.sort_by(|c1, c2| c2.n_files.cmp(&c1.n_files));
    }

    Counter::output(&counters, args.with_size);
}
