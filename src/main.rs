use clap::{App, Arg};

fn main() {
    let matches = mk_app().get_matches();
}

fn mk_app() -> App<'static, 'static> {
    let app = App::new("proinit")
        .version("0.1.0")
        .author("RisuSofos <risusofos@gmail.com>")
        .about("Initializes project workspaces for various project types")
        .arg(Arg::with_name("Web Page")
            .long("web-page")
            .value_name("output directory")
            .help("Initializes a webpage project")
            .takes_value(true)
        )
        .usage("proinit [options] <output_file>");
    app
}