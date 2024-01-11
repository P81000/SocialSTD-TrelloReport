use serde_derive::Deserialize;
use dotenv::dotenv;
use std::env;
use reqwest;

#[derive(Debug, Deserialize)]
pub struct Trello {
    key: String,
    token: String,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct Board {
    pub id: String,
    pub name: String,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct List {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub desc: String,
}

#[allow(non_snake_case)]
impl Trello {
    pub fn new() -> Self {
        dotenv().ok();
        let key = env::var("TRELLO_KEY").unwrap();
        let token = env::var("TRELLO_TOKEN").unwrap();

        Trello { key, token }
    }

    pub async fn getBoards(&self) -> Result<Vec<Board>, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.trello.com/1/members/me/boards?key={}&token={}",
            self.key, self.token
            );

        let response = reqwest::get(&url).await?;
        let body = response.text().await?;

        let boards: Result<Vec<Board>, _> = serde_json::from_str(&body)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>);

        Ok(boards?)
    }
    
    pub async fn getLists(&self, boardId: String) -> Result<Vec<List>, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.trello.com/1/boards/{}/lists?key={}&token={}",
            boardId, self.key, self.token
        );

        let response = reqwest::get(&url).await?;
        let body = response.text().await?;

        let lists: Result<Vec<List>, _> = serde_json::from_str(&body)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>);
        let extLists: Vec<List> = lists?
            .into_iter()
            .map(|list| List {
                id: list.id,
                name: list.name,
            })
            .collect();

        Ok(extLists)
    }
    
    pub async fn getCards(&self, listId: String) -> Result<Vec<Card>, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.trello.com/1/lists/{}/cards?key={}&token={}",
            listId, self.key, self.token
        );

        let response = reqwest::get(&url).await?;
        let body = response.text().await?;

        let cards: Result<Vec<Card>, _> = serde_json::from_str(&body)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>);
        let extCards: Vec<Card> = cards?
            .into_iter()
            .map(|card| Card {
                id: card.id,
                name: card.name,
                desc: card.desc,
            })
            .collect();

        Ok(extCards)
    }
}
