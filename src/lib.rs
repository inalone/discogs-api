pub mod database;
pub mod discogs;
pub use database::*;
pub use discogs::*;

#[cfg(test)]
mod tests {
    use crate::{Artist, Discogs};

    #[tokio::test]
    async fn test112() {
        let mut discogs = Discogs::new("aiko_unit_test");
        match Artist::get(&mut discogs, 81013).await {
            Ok(artist) => println!("{:?}", artist),
            Err(e) => println!("{}", e),
        };
    }
}
