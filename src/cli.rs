use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    app_from_crate!()
        .arg(
            Arg::with_name("command")
                .help("target command")
                .required(true),
        )
        .arg(
            Arg::with_name("input-dir")
                .help("directory containing test cases")
                .required(true),
        )
        .arg(
            Arg::with_name("output-dir")
                .help("output target directory")
                .required(true),
        )
        .arg(
            Arg::with_name("threads")
                .long("threads")
                .help("Specify the number of threads to execute in parallel. default=10")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("logfile")
                .long("logfile")
                .help("Specify the file to save the log. default=log.ltsv")
                .value_name("file"),
        )
    // .arg(
    //     Arg::with_name("timeout")
    //         .long("timeout")
    //         .help("")
    //         .value_name("time"),
    // )
}
