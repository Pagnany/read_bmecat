use anyhow::{Ok, Result};
use chrono::prelude::*;
use encoding_rs::WINDOWS_1252;
use odbc::*;
use odbc_safe::AutocommitOn;
use std::fs;
use std::io;
use std::path::Path;
use unicode_segmentation::UnicodeSegmentation;
mod bmecat;

fn main() -> Result<()> {
    let start_time = Local::now();

    println!("Reading BMEcat file...");
    //let temp = fs::read_to_string("./files/Boschimp.xml").expect("Can't read file");
    let temp = fs::read_to_string("./files/nw_bmecat.xml").expect("Can't read file");
    //let temp fs::read_to_string("./files/ELTEN BMEcat 1.2.xml").expect("Can't read file");

    println!("Parsing BMEcat file...");
    let bmecat_catalog = bmecat::read_bmecat(temp);
    let articles_count = bmecat_catalog.article.len();

    println!("Copy Tables");
    let _ = copy_files(Path::new("./foxprodbs/"), Path::new("C:/vfpdb/"));

    println!("Connecting to database...");
    // connection to table
    let env = create_environment_v3().map_err(|e| e.unwrap())?;
    let buffer = r#"Driver={Microsoft Visual FoxPro Driver};SourceType=DBF;SourceDB=c:\vfpdb\;Exclusive=No;Collate=Machine;NULL=NO;DELETED=YES;BACKGROUNDFETCH=NO;"#;
    let conn = env.connect_with_connection_string(&buffer)?;

    let mut tempcounter = 0;

    println!("Inserting articles into database...");
    bmecat_catalog
        .article
        .iter()
        .enumerate()
        .for_each(|(i, article)| {
            if tempcounter == 1000 {
                println!("{} of {}", i, articles_count);
                tempcounter = 0;
            }
            tempcounter += 1;
            insert_article(&conn, article).unwrap();
            insert_mime_article(&conn, article).unwrap();
            insert_article_feature_groups(&conn, article).unwrap();
            insert_article_order_details(&conn, article).unwrap();
            insert_article_price_details(&conn, article).unwrap();
        });

    let end_time = Local::now();
    let duration = end_time.signed_duration_since(start_time);

    println!("{:?}", duration);

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

fn insert_article<'env>(
    conn: &Connection<'env, AutocommitOn>,
    article: &crate::bmecat::Article,
) -> Result<()> {
    let stmt = Statement::with_parent(conn)?;

    let mut sql_text = "INSERT INTO article VALUES (".to_string();
    sql_text.push_str(&format!("'{}',", shorten_string(&article.id, 50)));
    sql_text.push_str(&format!(
        "'{}',",
        shorten_string(&article.article_details.desc_short, 254)
    ));
    sql_text.push_str(&format!(
        "'{}',",
        shorten_string(&article.article_details.desc_long, 250)
    ));
    sql_text.push_str(&format!(
        "'{}',",
        shorten_string(&article.article_details.ean, 13)
    ));
    sql_text.push_str(&format!(
        "'{}',",
        shorten_string(&article.article_details.supplier_alt_id, 50)
    ));
    sql_text.push_str(&format!(
        "'{}',",
        shorten_string(&article.article_details.manufacturer_name, 100)
    ));
    sql_text.push_str(&format!(
        "'{}',",
        shorten_string(&article.article_details.manufacturer_type_desc, 100)
    ));
    sql_text.push_str(&format!(
        "'{}',",
        shorten_string(&article.article_details.erp_group_buyer, 50)
    ));
    sql_text.push_str(&format!(
        "'{}',",
        shorten_string(&article.article_details.erp_group_supplier, 50)
    ));
    sql_text.push_str(&format!(
        "'{}',",
        shorten_string(&article.article_details.deliver_time, 15)
    ));
    sql_text.push_str(&format!(
        "'{}',",
        shorten_string(&article.article_details.remarks, 254)
    ));
    sql_text.push_str(&format!(
        "'{}',",
        shorten_string(&article.article_details.segment, 100)
    ));
    sql_text.push_str(&format!(
        "'{}'",
        shorten_string(&article.article_details.article_order, 10)
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

fn insert_mime_article<'env>(
    conn: &Connection<'env, AutocommitOn>,
    article: &crate::bmecat::Article,
) -> Result<()> {
    for mime in &article.mime_infos {
        let stmt = Statement::with_parent(conn)?;

        let mut sql_text = "INSERT INTO mime VALUES (".to_string();
        sql_text.push_str(&format!("'{}',", shorten_string(&article.id, 200)));
        sql_text.push_str(&format!("'{}',", shorten_string(&mime.mime_type, 30)));
        sql_text.push_str(&format!("'{}',", shorten_string(&mime.mime_source, 254)));
        sql_text.push_str(&format!("'{}',", shorten_string(&mime.mime_descr, 254)));
        sql_text.push_str(&format!("'{}',", shorten_string(&mime.mime_alt, 50)));
        sql_text.push_str(&format!("'{}',", shorten_string(&mime.mime_purpose, 20)));
        sql_text.push_str(&format!("'{}'", shorten_string(&mime.mime_order, 100)));
        sql_text.push_str(")");

        let (encode, _, _) = WINDOWS_1252.encode(&sql_text);
        let s = unsafe { String::from_utf8_unchecked(encode.to_vec()) };
        match stmt.exec_direct(&s)? {
            Data(_) => (),
            NoData(_) => (),
        }
    }
    Ok(())
}

fn insert_article_feature_groups<'env>(
    conn: &Connection<'env, AutocommitOn>,
    article: &crate::bmecat::Article,
) -> Result<()> {
    let mut i = 0;
    for feature_group in &article.article_feature_groups {
        let stmt = Statement::with_parent(conn)?;

        let feature_gr_id = format!("{}-{}", shorten_string(&article.id, 250), i.to_string());

        let mut sql_text = "INSERT INTO article_feature_group VALUES (".to_string();
        sql_text.push_str(&format!("'{}',", shorten_string(&article.id, 250)));
        sql_text.push_str(&format!("'{}',", feature_gr_id));
        sql_text.push_str(&format!(
            "'{}',",
            shorten_string(&feature_group.sys_name, 50)
        ));
        sql_text.push_str(&format!(
            "'{}',",
            shorten_string(&feature_group.group_id, 60)
        ));
        sql_text.push_str(&format!(
            "'{}'",
            shorten_string(&feature_group.group_name, 60)
        ));
        sql_text.push_str(")");

        let (encode, _, _) = WINDOWS_1252.encode(&sql_text);
        let s = unsafe { String::from_utf8_unchecked(encode.to_vec()) };
        match stmt.exec_direct(&s)? {
            Data(_) => (),
            NoData(_) => (),
        }
        insert_article_feature(conn, feature_gr_id, &feature_group.article_features)?;
        i += 1;
    }
    Ok(())
}

fn insert_article_feature(
    conn: &Connection<AutocommitOn>,
    feature_gr_id: String,
    article_features: &Vec<crate::bmecat::ArticleFeature>,
) -> Result<()> {
    let mut i = 0;
    for article_feature in article_features {
        let stmt = Statement::with_parent(conn)?;

        let feature_id = format!("{}-{}", feature_gr_id, i.to_string());

        let mut sql_text = "INSERT INTO article_feature VALUES (".to_string();
        sql_text.push_str(&format!("'{}',", feature_gr_id));
        sql_text.push_str(&format!("'{}',", &feature_id));
        sql_text.push_str(&format!("'{}',", shorten_string(&article_feature.name, 60)));
        sql_text.push_str(&format!("'{}',", shorten_string(&article_feature.unit, 20)));
        sql_text.push_str(&format!(
            "'{}',",
            shorten_string(&article_feature.order, 10)
        ));
        sql_text.push_str(&format!(
            "'{}',",
            shorten_string(&article_feature.descr, 250)
        ));
        sql_text.push_str(&format!(
            "'{}'",
            shorten_string(&article_feature.value_details, 250)
        ));
        sql_text.push_str(")");

        let (encode, _, _) = WINDOWS_1252.encode(&sql_text);
        let s = unsafe { String::from_utf8_unchecked(encode.to_vec()) };
        match stmt.exec_direct(&s)? {
            Data(_) => (),
            NoData(_) => (),
        }
        if article_feature.article_variants.article_variant.len() > 0 {
            insert_article_variants(conn, &feature_id, &article_feature.article_variants)?;
        }
        insert_article_feature_value(conn, &feature_id, &article_feature.value)?;
        i += 1;
    }
    Ok(())
}

fn insert_article_feature_value(
    conn: &Connection<AutocommitOn>,
    feature_id: &String,
    feature_values: &Vec<String>,
) -> Result<()> {
    for value in feature_values {
        let stmt = Statement::with_parent(conn)?;

        let mut sql_text = "INSERT INTO article_feature_value VALUES (".to_string();
        sql_text.push_str(&format!("'{}',", feature_id));
        sql_text.push_str(&format!("'{}'", shorten_string(&value, 60)));
        sql_text.push_str(")");

        let (encode, _, _) = WINDOWS_1252.encode(&sql_text);
        let s = unsafe { String::from_utf8_unchecked(encode.to_vec()) };
        match stmt.exec_direct(&s)? {
            Data(_) => (),
            NoData(_) => (),
        }
    }
    Ok(())
}

fn insert_article_order_details(
    conn: &Connection<AutocommitOn>,
    article: &crate::bmecat::Article,
) -> Result<()> {
    let stmt = Statement::with_parent(conn)?;

    let mut sql_text = "INSERT INTO article_order_details VALUES (".to_string();
    sql_text.push_str(&format!("'{}',", shorten_string(&article.id, 50)));
    sql_text.push_str(&format!(
        "'{}',",
        shorten_string(&article.article_order_details.order_unit, 3)
    ));
    sql_text.push_str(&format!(
        "'{}',",
        shorten_string(&article.article_order_details.content_unit, 3)
    ));
    sql_text.push_str(&format!(
        "'{}',",
        shorten_string(&article.article_order_details.no_cu_per_ou, 10)
    ));
    sql_text.push_str(&format!(
        "'{}',",
        shorten_string(&article.article_order_details.price_quantity, 10)
    ));
    sql_text.push_str(&format!(
        "'{}',",
        shorten_string(&article.article_order_details.quantity_min, 10)
    ));
    sql_text.push_str(&format!(
        "'{}'",
        shorten_string(&article.article_order_details.quantity_interval, 10)
    ));
    sql_text.push_str(")");

    let (encode, _, _) = WINDOWS_1252.encode(&sql_text);
    let s = unsafe { String::from_utf8_unchecked(encode.to_vec()) };
    match stmt.exec_direct(&s)? {
        Data(_) => (),
        NoData(_) => (),
    }

    Ok(())
}

fn insert_article_price_details(
    conn: &Connection<AutocommitOn>,
    article: &crate::bmecat::Article,
) -> Result<()> {
    let mut i = 0;
    for article_price_detail in &article.article_price_details {
        let stmt = Statement::with_parent(conn)?;

        let article_price_detail_id = format!("{}-{}", &article.id, i.to_string());

        let mut sql_text = "INSERT INTO article_price_details VALUES (".to_string();
        sql_text.push_str(&format!("'{}',", &article.id));
        sql_text.push_str(&format!("'{}',", &article_price_detail_id));
        sql_text.push_str(&format!(
            "'{}',",
            shorten_string(&article_price_detail.start_date, 20)
        ));
        sql_text.push_str(&format!(
            "'{}',",
            shorten_string(&article_price_detail.end_date, 20)
        ));
        sql_text.push_str(&format!(
            "'{}'",
            shorten_string(&article_price_detail.daily_price, 10)
        ));
        sql_text.push_str(")");

        let (encode, _, _) = WINDOWS_1252.encode(&sql_text);
        let s = unsafe { String::from_utf8_unchecked(encode.to_vec()) };
        match stmt.exec_direct(&s)? {
            Data(_) => (),
            NoData(_) => (),
        }
        insert_article_prices(
            conn,
            &article_price_detail.article_prices,
            article_price_detail_id,
        )?;
        i += 1;
    }
    Ok(())
}

fn insert_article_prices(
    conn: &Connection<AutocommitOn>,
    article_prices: &Vec<crate::bmecat::ArticlePrice>,
    article_price_detail_id: String,
) -> Result<()> {
    let mut i = 0;
    for article_price in article_prices {
        let stmt = Statement::with_parent(conn)?;

        let price_id = format!("{}-{}", &article_price_detail_id, i.to_string());

        let mut sql_text = "INSERT INTO article_price VALUES (".to_string();
        sql_text.push_str(&format!("'{}',", &price_id));
        sql_text.push_str(&format!("'{}',", &article_price_detail_id));
        sql_text.push_str(&format!(
            "'{}',",
            shorten_string(&article_price.price_amount, 10)
        ));
        sql_text.push_str(&format!(
            "'{}',",
            shorten_string(&article_price.price_currency, 3)
        ));
        sql_text.push_str(&format!("'{}',", shorten_string(&article_price.tax, 10)));
        sql_text.push_str(&format!(
            "'{}',",
            shorten_string(&article_price.price_factor, 10)
        ));
        sql_text.push_str(&format!(
            "'{}',",
            shorten_string(&article_price.lower_bound, 10)
        ));
        sql_text.push_str(&format!(
            "'{}'",
            shorten_string(&article_price.price_type, 50)
        ));

        sql_text.push_str(")");

        let (encode, _, _) = WINDOWS_1252.encode(&sql_text);
        let s = unsafe { String::from_utf8_unchecked(encode.to_vec()) };
        match stmt.exec_direct(&s)? {
            Data(_) => (),
            NoData(_) => (),
        }

        for territory in &article_price.territory {
            let stmt = Statement::with_parent(conn)?;
            let mut sql_text = "INSERT INTO article_price_territory VALUES (".to_string();
            sql_text.push_str(&format!("'{}',", &price_id));
            sql_text.push_str(&format!("'{}'", &territory));
            sql_text.push_str(")");

            let (encode, _, _) = WINDOWS_1252.encode(&sql_text);
            let s = unsafe { String::from_utf8_unchecked(encode.to_vec()) };
            match stmt.exec_direct(&s)? {
                Data(_) => (),
                NoData(_) => (),
            }
        }
        i += 1;
    }
    Ok(())
}

fn insert_article_variants(
    conn: &Connection<AutocommitOn>,
    feature_id: &String,
    article_variants: &crate::bmecat::ArticleVariants,
) -> Result<()> {
    let stmt = Statement::with_parent(conn)?;

    let variant_id = format!("{}-{}", feature_id, 0.to_string());

    let mut sql_text = "INSERT INTO article_variants VALUES (".to_string();
    sql_text.push_str(&format!("'{}',", feature_id));
    sql_text.push_str(&format!(
        "'{}'",
        shorten_string(&article_variants.vorder, 10)
    ));
    sql_text.push_str(")");

    let (encode, _, _) = WINDOWS_1252.encode(&sql_text);
    let s = unsafe { String::from_utf8_unchecked(encode.to_vec()) };
    match stmt.exec_direct(&s)? {
        Data(_) => (),
        NoData(_) => (),
    }
    insert_article_variants_values(conn, &variant_id, &article_variants.article_variant)?;
    Ok(())
}

fn insert_article_variants_values(
    conn: &Connection<AutocommitOn>,
    variant_id: &String,
    article_variants: &Vec<crate::bmecat::ArticleVariant>,
) -> Result<()> {
    let mut i = 0;
    for article_variant in article_variants {
        let stmt = Statement::with_parent(conn)?;

        let variant_value_id = format!("{}-{}", variant_id, i.to_string());

        let mut sql_text = "INSERT INTO article_variant VALUES (".to_string();
        sql_text.push_str(&format!("'{}',", variant_id));
        sql_text.push_str(&format!("'{}',", &variant_value_id));
        sql_text.push_str(&format!(
            "'{}',",
            shorten_string(&article_variant.value, 60)
        ));
        sql_text.push_str(&format!(
            "'{}'",
            shorten_string(&article_variant.supplier_aid_supplement, 32)
        ));
        sql_text.push_str(")");

        let (encode, _, _) = WINDOWS_1252.encode(&sql_text);
        let s = unsafe { String::from_utf8_unchecked(encode.to_vec()) };
        match stmt.exec_direct(&s)? {
            Data(_) => (),
            NoData(_) => (),
        }
        i += 1;
    }
    Ok(())
}

fn copy_files(source_dir: &Path, dest_dir: &Path) -> Result<()> {
    for entry in fs::read_dir(source_dir)? {
        let entry = entry?;
        let source_path = entry.path();

        let file_name = source_path
            .file_name()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid file name"))?;

        let dest_path = dest_dir.join(file_name);

        if entry.file_type()?.is_file() {
            fs::copy(&source_path, &dest_path)?;
        }
    }
    Ok(())
}
