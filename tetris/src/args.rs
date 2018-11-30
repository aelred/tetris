pub fn evil_mode() -> bool {
    ARGS.evil_mode
}

struct Args {
    evil_mode: bool,
}

lazy_static! {
    static ref ARGS: Args = {
        let args: Vec<String> = std::env::args().skip(1).collect();
        let first_arg = args.get(0);

        let evil_mode = first_arg
            .map(|arg| arg == "evil" || arg == "for-filipe")
            .unwrap_or(false);

        Args { evil_mode }
    };
}
