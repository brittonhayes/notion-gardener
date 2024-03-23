use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct Notion {
    pub client: Client,
    pub token: String,
}

impl Notion {
    pub fn new(client: Client, token: String) -> Self {
        Self { client, token }
    }

    pub async fn search_all_pages(
        &mut self,
        #[allow(unused_mut)] mut _cursor: Option<String>,
    ) -> Result<ListResponse, reqwest::Error> {
        self.client
            .get("https://api.notion.com/v1/search")
            .bearer_auth(&self.token)
            .header("Notion-Version", "2022-06-28")
            .send()
            .await?
            .json::<ListResponse>()
            .await
    }

    pub async fn get_child_blocks(
        &mut self,
        page_id: String,
        #[allow(unused_mut)] mut cursor: Option<String>,
    ) -> Result<ListResponse, reqwest::Error> {
        let response = self
            .client
            .get(format!(
                "https://api.notion.com/v1/blocks/{}/children",
                page_id
            ))
            .query(&[("start_cursor", cursor.clone())])
            .bearer_auth(&self.token)
            .header("Notion-Version", "2022-06-28")
            .send()
            .await
            .unwrap()
            .error_for_status();

        match response {
            Ok(_) => response.unwrap().json::<ListResponse>().await,
            Err(err) => Err(err),
        }
    }

    pub async fn delete_block(&mut self, block_id: String) -> Result<Response, reqwest::Error> {
        let response = self
            .client
            .delete(format!("https://api.notion.com/v1/blocks/{}", block_id))
            .bearer_auth(&self.token)
            .header("Notion-Version", "2022-06-28")
            .send()
            .await
            .unwrap()
            .error_for_status();

        match response {
            Ok(_) => response.unwrap().json::<Response>().await,
            Err(err) => Err(err),
        }
    }
}

/// The primitive for a notion block
#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub object: String,
    pub id: String,
    #[serde(rename = "type")]
    pub block_type: String,
    pub to_do: Option<ToDo>,
}

/// The primitive for a notion to-do block
#[derive(Debug, Serialize, Deserialize)]
pub struct ToDo {
    pub checked: bool,
    pub rich_text: Vec<Text>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Text {
    #[serde(rename = "type")]
    pub text_type: String,
    pub text: Content,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub content: String,
    pub link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListResponse {
    pub results: Vec<Block>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub id: String,
    pub object: String,
}
