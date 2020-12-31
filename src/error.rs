/// TODO: add error enum for more descriptive errors
pub type AdrResult<T = ()> = Result<T, std::io::Error>;
