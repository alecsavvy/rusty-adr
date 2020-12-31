use crate::error::AdrResult as Result;
use mdbook::Config as MdBookConfig;

pub struct AdrConfig {}

/// Returns a pair of config structs for Adr and MDBook
pub async fn read_config() -> Result<(AdrConfig, MdBookConfig)> {
    let adr_conf = AdrConfig {};
    let mdb_conf = MdBookConfig::from_disk("adr.toml").unwrap();
    Ok((adr_conf, mdb_conf))
}
