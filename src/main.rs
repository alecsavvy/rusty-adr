mod commands;
mod config;
mod error;

#[async_std::main]
async fn main() -> error::AdrResult {
    // reads .toml config, combo of adr conf and mdbook conf
    let (_adr_conf, mdb_conf) = config::read_config().await?;

    let path = mdb_conf.clone().book.src;

    let _book = mdbook::MDBook::init(path)
        .with_config(mdb_conf)
        .build()
        .unwrap();

    Ok(())
}
