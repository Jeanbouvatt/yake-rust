use cli::{parse_cli, ParsedCli};
use yake_rust::{ResultItem, Yake};

mod cli;

fn main() {
    let ParsedCli {
        language,
        #[cfg(feature = "json_format")]
        json,
        input,
        config,
        top,
        verbose: _,
    } = parse_cli();

    let now = std::time::Instant::now();

    let keywords = Yake::new(language, config).get_n_best(&input, top);

    output_keywords(
        &keywords,
        #[cfg(feature = "json_format")]
        json,
    );

    eprintln!("Elapsed: {:.2?}", now.elapsed());
}

fn output_keywords(keywords: &[ResultItem], #[cfg(feature = "json_format")] json: bool) {
    #[cfg(feature = "json_format")]
    if json {
        match serde_json::to_string(&keywords) {
            Ok(str) => {
                println!("{}", str)
            }
            Err(e) => {
                eprintln!("Unexpected error happened while trying to serialize result to json : {:?}", e)
            }
        }
        return;
    }
    println!("{:?}", keywords);
}
