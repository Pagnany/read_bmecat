use chrono::prelude::*;

mod bmecat;

fn main() {
    let start_time = Local::now();

    let temp = std::fs::read_to_string("./files/nw_bmecat.xml").expect("Can't read file");

    let articles = bmecat::read_bmecat(temp);

    article_pictures_to_csv(articles.clone());

    let end_time = Local::now();
    let duration = end_time.signed_duration_since(start_time);

    println!("{:?}", duration);
}

fn article_pictures_to_csv(articles: Vec<bmecat::Article>) {
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b';')
        .from_path("./files/article_pictures.csv")
        .expect("Can't create file");

    for article in articles {
        let pictures = bmecat::get_article_pictures(&article);

        for picture in pictures {
            wtr.write_record(&[article.id.clone(), picture])
                .expect("Can't write record");
        }
    }
    wtr.flush().expect("Can't flush");
}
