use chrono::prelude::*;
use std::env;

mod bmecat;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Wenn der Pfad nicht übergeben wurde.
    if args.len() != 2 {
        println!("FEHLER: Bitte geben Sie den Pfad zur BMEcat Datei an.");

        println!("Zum Schließen <Enter> drücken.");
        let mut line = String::new();
        let _ = std::io::stdin().read_line(&mut line).unwrap();
        return;
    }

    let start_time = Local::now();

    let file_string = std::fs::read_to_string(&args[1]).expect(
        "FEHLER: Datei konnte nicht geöffnet werden! Hinweis: Die Datei muss als UTF-8 encoded sein.",
    );

    println!("BMEcat wird verarbeitet... Bitte haben sie etwas Gedult.");
    let articles = bmecat::read_bmecat(file_string);

    println!("CSV Datei wird generiert...");
    article_picture_to_csv_one_line(articles);

    let end_time = Local::now();
    let duration = end_time.signed_duration_since(start_time);
    println!("{:?}", duration);

    println!("Zum Schließen <Enter> drücken.");
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line).unwrap();
}

fn article_pictures_to_csv(articles: Vec<bmecat::Article>) {
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b';')
        .from_path("./article_pictures.csv")
        .expect("Can't create file");

    for article in articles {
        let pictures = article.get_pictures();

        for picture in pictures {
            wtr.write_record(&[article.id.clone(), picture])
                .expect("Can't write record");
        }
    }
    wtr.flush().expect("Can't flush");
}

fn article_picture_to_csv_one_line(articles: Vec<bmecat::Article>) {
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b';')
        .quote_style(csv::QuoteStyle::Never)
        .from_path("./article_pictures.csv")
        .expect("Can't create file");

    for article in articles {
        let pictures = article.get_pictures();

        wtr.write_record(&[article.id.clone(), pictures.join(";")])
            .expect("Can't write record");
    }
    wtr.flush().expect("Can't flush");
}
