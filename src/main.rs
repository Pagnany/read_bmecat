use chrono::prelude::*;

mod bmecat;

fn main() {
    let start_time = Local::now();

    let temp = std::fs::read_to_string("./files/nw_bmecat.xml").unwrap();

    let articles = bmecat::read_bmecat(temp);

    let mut count = 0;

    for article in articles {
        println!("{}", article.id);
        println!("{:?}", bmecat::get_article_pictures(&article));
        count += 1;

        if count > 10 {
            break;
        }
    }

    let end_time = Local::now();
    let duration = end_time.signed_duration_since(start_time);

    println!("{:?}", duration);
}
