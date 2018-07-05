use clap::{App, Arg, SubCommand};

pub fn get<'a, 'b>() -> App<'a, 'b> {
    return App::new("dlrun")
        .version(crate_version!())
        .author("Julien Roncaglia <julien@roncaglia.fr>")
        .about("Download & run installers for container creation")
        .arg(
            Arg::with_name("url")
                .index(1)
                .value_name("URL")
                .help(
                    "URL of the binary to download",
                )
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("hash")
                .index(2)
                .value_name("HASH")
                .help(
                    "Hash (SHA256) of the binary file",
                )
                .takes_value(true)
                .required(false),
        );
}
