pub use clap_complete;
pub use clap;
pub use oscript_derive::oscript_main;
pub use clap::*;

#[cfg(feature = "tokio")]
pub use tokio;

#[cfg(feature = "tokio")]
pub use oscript_derive::oscript_async_main;