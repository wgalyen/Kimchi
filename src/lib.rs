#[deny(missing_docs)]

/**
* `kimchi` is a library for checking links.
* "Hello world" example:
* ```
* use std::error::Error;
*
* #[tokio::main]
* async fn main() -> Result<(), Box<dyn Error>> {
*   let response = kimchi::check("https://github.com/wgalyen/kimchi").await?;
*   println!("{}", response);
*   Ok(())
* }
* ```
*
* For more specific use-cases you can build a kimchi client yourself,
* using the `ClientBuilder` which can be used to
* configure and run your own link checker and grants full flexibility:
*
* ```
* use kimchi::{ClientBuilder, Status};
* use std::error::Error;
*
* #[tokio::main]
* async fn main() -> Result<(), Box<dyn Error>> {
*   let client = ClientBuilder::default().build()?;
*   let response = client.check("https://github.com/wgalyen/kimchi").await?;
*   assert!(matches!(response.status, Status::Ok(_)));
*   Ok(())
* }
* ```
*/
mod client;
mod client_pool;
mod excludes;
mod types;
mod uri;

pub mod collector;
pub mod extract;
pub mod test_utils;

pub use client::check;
pub use client::ClientBuilder;
pub use client_pool::ClientPool;
pub use collector::Input;
pub use excludes::Excludes;
pub use types::*;
pub use uri::Uri;
