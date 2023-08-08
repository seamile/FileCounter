mod cmdargs;
mod output;
mod walker;

use clap::Parser;
use cmdargs::{CmdArgParser, OrderBy};
use walker::Counter;

fn main() {
    // parse cmd-line args and get directories
    let args = CmdArgParser::parse();

    // walk all files
    let directories = args.get_directories();
    let filter = args.get_regex();
    let with_dir = filter.is_none();
    let mut counters = Vec::<Counter>::new();
    if args.non_recursive {
        for dirpath in directories {
            if let Ok((_, counter)) = walker::walk(
                &dirpath,
                args.all_files,
                args.with_size,
                filter.clone(),
                args.verbose,
            ) {
                counters.push(counter);
            };
        }
    } else {
        counters = walker::parallel_walk(
            directories,
            args.all_files,
            args.with_size,
            filter,
            args.verbose,
            args.get_threads_num(),
        );
    }

    match args.order_by {
        Some(OrderBy::Name) => {
            counters.sort_by(|c1, c2| c1.dirpath.cmp(&c2.dirpath));
        }
        Some(OrderBy::Count) => {
            counters.sort_by(|c1, c2| c2.n_files.cmp(&c1.n_files));
        }
        Some(OrderBy::Size) => {
            counters.sort_by(|c1, c2| c2.size().cmp(&c1.size()));
        }
        None => {}
    }

    Counter::output(&counters, with_dir, args.with_size);
}
