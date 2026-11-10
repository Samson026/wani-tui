use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Summary {
    pub data: Reviews,
}

#[derive(Debug, Deserialize)]
pub struct Reviews {
    pub reviews: Vec<Review>,
}

#[derive(Debug, Deserialize)]
pub struct Review {
    pub subject_ids: Vec<u32>,
}

// subject data
#[derive(Debug, Deserialize)]
pub struct Subject {
    pub data: SujectData,
}

#[derive(Debug, Deserialize)]
pub struct SujectData {
    pub auxiliary_meanings: Vec<Meaing>,
    pub characters: String,
}

#[derive(Debug, Deserialize)]
pub struct Meaing {
    pub r#type: String,
    pub meaning: String,
}
