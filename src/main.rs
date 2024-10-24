mod cmdargs;
mod output;
mod walker;

use clap::Parser;
use cmdargs::{CmdArgParser, OrderBy};
use walker::Counter;

fn main() {
    // parse cmd-line args and get directories
    let args = CmdArgParser::parse();
    let with_size =
        args.with_size || args.order_by == Some(OrderBy::S) || args.order_by == Some(OrderBy::Size);
    let with_dir =
        args.with_dir || args.order_by == Some(OrderBy::D) || args.order_by == Some(OrderBy::Dir);

    // walk all files
    let directories = args.get_directories();
    let filter = args.get_regex();
    let mut counters = Vec::<Counter>::new();
    if args.non_recursive {
        for dirpath in directories {
            if let Ok((_, counter)) = walker::walk(
                &dirpath,
                args.all_files,
                with_size,
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
            with_size,
            filter,
            args.verbose,
            args.get_threads_num(),
        );
    }

    match args.order_by {
        Some(OrderBy::Name) | Some(OrderBy::N) => {
            counters.sort_by(|c1, c2| c1.dirpath.cmp(&c2.dirpath));
        }
        Some(OrderBy::File) | Some(OrderBy::F) => {
            counters.sort_by(|c1, c2| c2.n_files.cmp(&c1.n_files));
        }
        Some(OrderBy::Dir) | Some(OrderBy::D) => {
            counters.sort_by(|c1, c2| c2.n_dirs.cmp(&c1.n_dirs));
        }
        Some(OrderBy::Size) | Some(OrderBy::S) => {
            counters.sort_by(|c1, c2| c2.size().cmp(&c1.size()));
        }
        None => {}
    }

    Counter::output(&counters, with_dir, with_size);
}
