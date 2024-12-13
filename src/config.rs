use std::sync::LazyLock;

use config::Config;

fn config() -> &'static Config {
    static CONFIG: LazyLock<Config> = LazyLock::new(|| {
        Config::builder()
            .add_source(config::File::with_name("name"))
            .add_source(config::Environment::default())
            .build()
            .unwrap()
    });

    todo!()
}

pub fn address() -> (&'static str, u16) {
    ("127.0.0.1", 9832)
}
