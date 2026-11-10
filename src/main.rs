use std::{env, error::Error, fmt::format};

use reqwest::{Client, header::AUTHORIZATION};

use crate::types::{Subject, Summary};
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let token = env::var("WANI")?;

    let resp: Summary = client
        .get("https://api.wanikani.com/v2/summary")
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .send()
        .await?
        .json()
        .await?;

    let section = 3;

    println!("{}", resp.data.reviews[section].subject_ids.len());

    let mut subjects: Vec<Subject> = vec![];
    for review in resp.data.reviews[section].subject_ids.iter() {
        let sub: Subject = client
            .get(format!("https://api.wanikani.com/v2/subjects/{}", review))
            .header(AUTHORIZATION, format!("Bearer {token}"))
            .send()
            .await?
            .json()
            .await?;

        subjects.push(sub);
    }

    for sub in subjects.iter() {
        println!(
            "Review: {}, meaning: {}",
            sub.data.characters,
            sub.data
                .auxiliary_meanings
                .first()
                .map(|m| m.meaning.as_str())
                .unwrap_or("No meaning...")
        )
    }
    Ok(())
}
