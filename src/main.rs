use crate::cli::cli;
use crate::product::TokopediaProduct;
use crate::product_scraper::scrape_product;

mod cli;
mod product;
mod product_scraper;

#[tokio::main]
async fn main() {
    let cli = cli().get_matches();
    let query = cli.get_one::<String>("query").expect("required");
    let page = cli.get_one::<String>("page").expect("required");
    let format = cli.get_one::<String>("format").expect("required");

    // change page to u32
    let page: u32 = page.parse().unwrap();

    match scrape_product(&query, &page).await {
        Ok(tokopedia_product) => match format.as_str() {
            "json" => {
                write_json(tokopedia_product);
            }
            _ => {
                eprintln!("Invalid format");
            }
        },

        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }
}

fn write_json(tokopedia_products: Vec<TokopediaProduct>) {
    let json_data = serde_json::to_string(&tokopedia_products).unwrap();
    std::fs::write("products.json", json_data).expect("Unable to write file");
}
