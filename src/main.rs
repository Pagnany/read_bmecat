use chrono::prelude::*;

pub mod bmecat;

fn main() {
    let start_time = Local::now();

    let temp = std::fs::read_to_string("./files/nw_bmecat.xml").unwrap();

    let articles = bmecat::read_bmecat(temp);

    for article in articles {
        println!("{}", article.article_details.desc_short);
    }

    let end_time = Local::now();
    let duration = end_time.signed_duration_since(start_time);

    println!("{:?}", duration);
}
