pub(crate) const SUCCESS_SYMBOL: char = '\u{2705}';
pub(crate) const ERROR_SYMBOL: char = '\u{274C}';
pub(crate) const USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " ",
    env!("CARGO_PKG_AUTHORS")
);
pub(crate) const LOCAL_DATABASE: &str = "EVE.db";
