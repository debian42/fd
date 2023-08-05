use std::time::SystemTime;

use clap::Parser;
use colored::Colorize;
use fd::process_file;
use fd::DateTimeHolder;

// microsoft malloc
#[cfg(feature = "win_only")]
use mimalloc::MiMalloc;
#[cfg(feature = "win_only")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

/// F. P.:  Program to filter carmen/yoda logfiles(byte encoded) by datetime
#[derive(Parser, Debug)]
#[command(version)]
struct CMDArgs {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
 
    /// Use other algo
    #[arg(short, long,)]
    fast: bool,

    /// The optional start date: dd.mm.yyyy HH::MM:SS
    #[arg(short, long)]
    start: Option<String>,

    /// The optional end date: dd.mm.yyyy HH::MM:SS
    #[arg(short, long)]
    end: Option<String>,

    /// Name of the files to filter
    #[clap(value_parser)]
    files: Option<Vec<String>>,
}

fn check_consistency_of_args(args: &CMDArgs) {
    if args.start.is_none() && args.end.is_none() {
        eprintln!("{}", "Start- or End- Date must be given".bold().red());
        ::std::process::exit(1);
    }
}

#[allow(clippy::print_with_newline)]
fn main() {
    let args = CMDArgs::parse();

    check_consistency_of_args(&args);

    let start_end_date: DateTimeHolder =
        DateTimeHolder::new(args.start.as_ref(), args.end.as_ref());

    if !start_end_date.validate() {
        eprintln!(
            "{}",
            "End-Date must be greater then Start-Date".bold().red()
        );
        ::std::process::exit(1);
    }
    let now = SystemTime::now();
    if args.files.is_none() {
        process_file(
            &start_end_date,
            None,
            args.debug,
            args.fast,
            &mut std::io::stdout(),
            &mut std::io::stdin(),
        );
    } else {
        let len = args.files.as_ref().unwrap().len();
        for idx in 0..len {
            let filename = &args.files.as_ref().unwrap()[idx];
            process_file(
                &start_end_date,
                Some(filename),
                args.debug,
                args.fast,
                &mut std::io::stdout(),
                &mut std::io::stdin(),
            );
        }
    }
    if args.debug > 0 {
        let duration = now.elapsed().expect("Clock error ?!").as_millis() as u64;
        eprintln!("Processing took {} ms", duration);
    }

    ::std::process::exit(0);
}
