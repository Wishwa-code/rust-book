extern crate trpl;
use trpl::{Either,Html};


fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let title_fut_1 = page_tittle(&args[1]);
        let title_fut_2 = page_tittle(&args[2]);

        let (url,maybe_tittle) = 
            match trpl::race(title_fut_1,title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

        println!("{url} returned first");
        match maybe_tittle {
            Some(title) => println!("Its page tittle is: '{title}'"),
            None => println!("Its title could not be parsed."),
        }
    })
}

async fn page_tittle(url: &str) ->(&str, Option<String>) {
    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text)
        .select_first("title")
        .map(|title | title.inner_html());
    (url, title) 
}

/* cargo run -- https://www.google.lk https://www.youtube.lk */
