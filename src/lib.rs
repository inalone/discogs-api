pub mod discogs;
pub mod routes;
pub mod types;
pub use discogs::*;
pub use routes::*;
pub use types::*;

#[cfg(test)]
mod tests {
    use crate::Discogs;

    #[tokio::test]
    async fn artist() {
        match Discogs::new("aiko_unit_test").artist(108713).await {
            Ok(artist) => println!("{:?}", artist),
            Err(e) => println!("{}", e),
        };
    }

    #[tokio::test]
    async fn release() {
        match Discogs::new("aiko_unit_test")
            .release(249504, crate::Currency::GBP)
            .await
        {
            Ok(release) => println!("{:?}", release),
            Err(e) => println!("{}", e),
        };
    }
}
