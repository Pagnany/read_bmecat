use anyhow::Result;
use chrono::prelude::*;
use encoding_rs::WINDOWS_1252;
use odbc::*;
use odbc_safe::AutocommitOn;
use unicode_segmentation::UnicodeSegmentation;

mod bmecat;

fn main() -> Result<()> {
    let start_time = Local::now();
    env_logger::init();

    println!("Reading BMEcat file...");
    //let temp = std::fs::read_to_string("./files/Boschimp.xml").expect("Can't read file");
    let temp = std::fs::read_to_string("./files/nw_bmecat.xml").expect("Can't read file");
    //let temp = std::fs::read_to_string("./files/ELTEN BMEcat 1.2.xml").expect("Can't read file");

    println!("Connecting to database...");
    // connection to table
    let env = create_environment_v3().map_err(|e| e.unwrap())?;
    let buffer = r#"Driver={Microsoft Visual FoxPro Driver};SourceType=DBF;SourceDB=c:\vfpdb\;Exclusive=No;Collate=Machine;NULL=NO;DELETED=YES;BACKGROUNDFETCH=NO;"#;
    let conn = env.connect_with_connection_string(&buffer)?;

    println!("Parsing BMEcat file...");
    let articles = bmecat::read_bmecat(temp);
    let articles_count = articles.len();

    println!("Inserting articles into database...");
    articles[..].iter().enumerate().for_each(|(i, article)| {
        if i % 1000 == 0 {
            println!("{} of {}", i, articles_count);
        }
        insert_article(&conn, article).unwrap();
        insert_mime_article(&conn, article).unwrap();
    });

    let end_time = Local::now();
    let duration = end_time.signed_duration_since(start_time);

    println!("{:?}", duration);

    Ok(())
}

fn _execute_statement<'env>(conn: &Connection<'env, AutocommitOn>) -> Result<()> {
    let stmt = Statement::with_parent(conn)?;

    let sql_text = "select * from Hallodatei".to_string();

    let (encode, _, _) = WINDOWS_1252.encode(&sql_text);

    let s = unsafe { String::from_utf8_unchecked(encode.to_vec()) };

    let mut count = 1;

    match stmt.exec_direct(&s)? {
        Data(mut stmt) => {
            let cols = stmt.num_result_cols()?;
            while let Some(mut cursor) = stmt.fetch()? {
                println!("Row {}:", count);
                for i in 1..=cols {
                    println!("  Column {}:", i);
                    let data = cursor.get_data::<Vec<u8>>(i as u16).unwrap().unwrap();
                    let (result, _, _) = WINDOWS_1252.decode(&data);
                    let s = result.to_string();
                    println!("    {}", s);
                }

                count += 1;
            }
        }
        NoData(_) => println!("Query executed, no data returned"),
    }
    println!("Count: {}", count);

    Ok(())
}

fn _insert_statement<'env>(conn: &Connection<'env, AutocommitOn>) -> Result<()> {
    let stmt = Statement::with_parent(conn)?;
    let sql_text =
        "INSERT INTO Hallodatei (name, number, logical, id, memo, date, datetime) VALUES ('insert Ä ü ö ß', 1.35, .F., 5, 'Hier können wir auch mal etwas längeres schreiben', DATE(), DATETIME())"
            .to_string();

    let (encode, _, _) = WINDOWS_1252.encode(&sql_text);

    let s = unsafe { String::from_utf8_unchecked(encode.to_vec()) };

    match stmt.exec_direct(&s)? {
        Data(_) => println!("Query executed, data returned"),
        NoData(_) => println!("Query executed, no data returned"),
    }

    Ok(())
}

fn insert_article<'env>(
    conn: &Connection<'env, AutocommitOn>,
    article: &crate::bmecat::Article,
) -> Result<()> {
    let stmt = Statement::with_parent(conn)?;
    let mut sql_text =
        "INSERT INTO article (ID, DESC_SHORT, DESC_LONG, EAN, SUP_ALT_ID, MANUF_NAME, MANUF_TYP, ERP_GR_BY, ERP_GR_SUP, DELIV_TIME, REMAKRS, SEGMENT, ORDER) VALUES (".to_string();

    sql_text.push_str(&format!("'{}',", str_conv(&article.id)));
    sql_text.push_str(&format!(
        "'{}',",
        str_conv(&article.article_details.desc_short)
    ));

    sql_text.push_str(&format!(
        "'{}',",
        shorten_string(&article.article_details.desc_long, 250)
    ));

    sql_text.push_str(&format!("'{}',", str_conv(&article.article_details.ean)));
    sql_text.push_str(&format!(
        "'{}',",
        str_conv(&article.article_details.supplier_alt_id)
    ));
    sql_text.push_str(&format!(
        "'{}',",
        str_conv(&article.article_details.manufacturer_name)
    ));
    sql_text.push_str(&format!(
        "'{}',",
        str_conv(&article.article_details.manufacturer_type_desc)
    ));
    sql_text.push_str(&format!(
        "'{}',",
        str_conv(&article.article_details.erp_group_buyer)
    ));
    sql_text.push_str(&format!(
        "'{}',",
        str_conv(&article.article_details.erp_group_supplier)
    ));
    sql_text.push_str(&format!(
        "'{}',",
        str_conv(&article.article_details.deliver_time)
    ));

    sql_text.push_str(&format!(
        "'{}',",
        shorten_string(&article.article_details.remarks, 250)
    ));

    sql_text.push_str(&format!(
        "'{}',",
        str_conv(&article.article_details.segment)
    ));
    sql_text.push_str(&format!(
        "'{}'",
        str_conv(&article.article_details.article_order)
    ));

    sql_text.push_str(")");

    //println!("{}", sql_text);

    let (encode, _, _) = WINDOWS_1252.encode(&sql_text);
    let s = unsafe { String::from_utf8_unchecked(encode.to_vec()) };
    match stmt.exec_direct(&s)? {
        Data(_) => (),
        NoData(_) => (),
    }

    Ok(())
}

fn str_conv(string: &str) -> String {
    // all chars in Windows-1252 range
    // removes chars for SQL
    string
        .chars()
        .filter(|c| ((c >= &'!' && c <= &'ÿ') && c != &',' && c != &'\'') || c == &' ')
        .collect()
}

fn shorten_string(string: &str, max_len: usize) -> String {
    let mut temp = str_conv(string);

    if temp.len() > max_len {
        let mut truncated = temp.graphemes(true).take(max_len).collect::<String>();
        while !truncated.is_char_boundary(truncated.len()) {
            truncated.pop();
        }
        temp = truncated;
    }

    temp
}

fn insert_mime_article<'env>(
    conn: &Connection<'env, AutocommitOn>,
    article: &crate::bmecat::Article,
) -> Result<()> {
    let art_id = article.id.clone();
    for mime in article.mime_infos.clone() {
        let stmt = Statement::with_parent(conn)?;

        let mut sql_text =
            "INSERT INTO mime (ART_ID, TYPE, SOURCE, DESC, ALT, PURPOSE, ORDER) VALUES ("
                .to_string();

        sql_text.push_str(&format!("'{}',", shorten_string(&art_id, 250)));
        sql_text.push_str(&format!("'{}',", shorten_string(&mime.mime_type, 250)));
        sql_text.push_str(&format!("'{}',", shorten_string(&mime.mime_source, 250)));
        sql_text.push_str(&format!("'{}',", shorten_string(&mime.mime_descr, 500)));
        sql_text.push_str(&format!("'{}',", shorten_string(&mime.mime_alt, 250)));
        sql_text.push_str(&format!("'{}',", shorten_string(&mime.mime_purpose, 250)));
        sql_text.push_str(&format!("'{}'", shorten_string(&mime.mime_order, 250)));
        sql_text.push_str(")");

        //println!("{}", sql_text);

        let (encode, _, _) = WINDOWS_1252.encode(&sql_text);
        let s = unsafe { String::from_utf8_unchecked(encode.to_vec()) };
        match stmt.exec_direct(&s)? {
            Data(_) => (),
            NoData(_) => (),
        }
    }
    Ok(())
}
