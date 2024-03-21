pub mod notion;

use clap::Parser;
use dotenv::dotenv;
use notion::Notion;
use reqwest::Client;

/// Notion to-do list garden keeper 🪴
#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = "🪴 A Notion gardener that trims completed items from your todo lists"
)]
struct Args {
    /// Notion page to use
    #[arg(short, long, env)]
    page_id: String,

    /// Notion token to use
    #[arg(short, long, env)]
    api_token: String,

    /// Notion to-do delete marker
    #[arg(short, long, env)]
    delete_marker: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let args = Args::parse();
    let notion_token = args.api_token;

    let mut cursor: Option<String> = None;
    let mut has_more = true;
    let mut notion_sdk = Notion::new(Client::new(), notion_token.clone());

    println!("To-Do:");
    while has_more {
        let res = notion_sdk
            .get_child_blocks(args.page_id.to_string(), cursor)
            .await?;

        for block in res.results.into_iter().filter(|b| b.block_type == "to_do") {
            if let Some(to_do) = block.to_do {
                let content: Vec<String> = to_do
                    .rich_text
                    .into_iter()
                    .map(|t| t.text.content)
                    .collect();

                let delete_marker = "🗑️".to_string();
                if to_do.checked && content.join("").contains(&delete_marker) {
                    println!("Deleting: {}", block.id);
                    notion_sdk.delete_block(block.id.to_string()).await?;
                } else {
                    println!("🔲 {}", content.join(""));
                }
            }
        }

        cursor = res.next_cursor;
        has_more = res.has_more;
    }

    Ok(())
}
