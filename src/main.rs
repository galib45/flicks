// mod ui;

use std::error::Error;
use reqwest::{blocking::Client, Url};
use scraper::{Html, Selector};
//use crate::ui::UI;

struct Media {
	title: String, year: String,
	href: String, media_id: String 
}

fn main() -> Result<(), Box<dyn Error>> {
	let base = Url::parse("https://sflix.to")?;
	let url = base.join("/search/young-sheldon")?;
	let client = Client::new();
	let html = client.get(url).send()?.text()?;
	let document = Html::parse_document(&html);
	let item_selector = Selector::parse("div.flw-item")?;
	let ahref_selector = Selector::parse("a.film-poster-ahref")?;
	let year_selector = Selector::parse("span.fdi-item")?;
	let (mut ahref, mut title, mut href, mut year, mut media_id);
	for item in document.select(&item_selector) {
		ahref = item.select(&ahref_selector).next().unwrap();
		title = ahref.attr("title").unwrap().to_string();
		href = ahref.attr("href").unwrap().to_string();
		media_id = href.split('-').last().unwrap();
		year = item.select(&year_selector).next().unwrap().inner_html();
		println!("{}, {}, {}, {}", title, year, href, media_id);
	}

	let items = vec![
            "Game of Thrones",
            "Breaking Bad",
            "The Sopranos",
            "Friends",
            "The Office (US)",
            "Stranger Things",
            "The Crown",
            "The Mandalorian",
            "Chernobyl",
            "The Witcher",
            "Game of Thrones",
            "Breaking Bad",
            "The Sopranos",
            "Friends",
            "The Office (US)",
            "Stranger Things",
            "The Crown",
            "The Mandalorian",
            "Chernobyl",
            "The Witcher",
            "Game of Thrones",
            "Breaking Bad",
            "The Sopranos",
            "Friends",
            "The Office (US)",
            "Stranger Things",
            "The Crown",
            "The Mandalorian",
            "Chernobyl",
            "The Witcher",
        ];
    // let mut ui = UI::new(&items);
    // ui.run()?;
	println!("{:?}", items);
    Ok(())
}
