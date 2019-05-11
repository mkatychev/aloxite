use argparse::{ArgumentParser, Store, StoreTrue}; // TODO replace with clap-rs

fn parse_args() {
    let mut verbose = false;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("aloxite cli");
        ap.refer(&mut verbose).add_option(
            &["-v", "--verbose"],
            StoreTrue,
            "Print matches to stdout",
        );
    }
}
