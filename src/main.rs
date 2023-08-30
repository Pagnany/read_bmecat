use chrono::prelude::*;

fn main() {
    let start_time = Local::now();

    let temp = std::fs::read_to_string("./files/ELTEN BMEcat 1.2.xml").unwrap();
    //let temp = std::fs::read_to_string("./files/nw_bmecat.xml").unwrap();
    println!("XML in memory");
    let doc = roxmltree::Document::parse(&temp).unwrap();

    for node in doc.descendants() {
        match node.tag_name().name() {
            "ARTICLE" => {
                create_artikel(node.descendants());
                return;
            }
            _ => (),
        }
    }

    let end_time = Local::now();
    let duration = end_time.signed_duration_since(start_time);

    println!("Dauer: {:?}", duration);
}

fn create_artikel(node: roxmltree::Descendants) {
    let mut artikel = Artikel {
        ..Default::default()
    };
    for descen in node {
        match descen.tag_name().name() {
            "SUPPLIER_AID" => {
                artikel.id = descen.text().unwrap_or("").to_string();
            }
            "DESCRIPTION_SHORT" => {
                artikel.desc_short = descen.text().unwrap_or("").to_string();
            }
            "DESCRIPTION_LONG" => {
                artikel.desc_long = descen.text().unwrap_or("").to_string();
            }
            "EAN" => {
                artikel.ean = descen.text().unwrap_or("").to_string();
            }
            "DELIVERY_TIME" => {
                artikel.deliver_time = descen.text().unwrap_or("").to_string();
            }
            "ARTICLE_FEATURES" => {
                for feature in descen.descendants() {
                    match feature.tag_name().name() {
                        "FEATURE" => {
                            let mut artikel_feature = ArtikelFeature {
                                ..Default::default()
                            };
                            for feature_descen in feature.descendants() {
                                match feature_descen.tag_name().name() {
                                    "FNAME" => {
                                        artikel_feature.name =
                                            feature_descen.text().unwrap_or("").to_string();
                                    }
                                    "FVALUE" => {
                                        artikel_feature.value =
                                            feature_descen.text().unwrap_or("").to_string();
                                    }
                                    "FUNIT" => {
                                        artikel_feature.unit =
                                            feature_descen.text().unwrap_or("").to_string();
                                    }
                                    _ => (),
                                }
                            }
                            artikel.artikel_features.push(artikel_feature);
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
    println!("{:?}", artikel);
}

#[derive(Debug, Clone)]
struct Artikel {
    id: String,
    desc_short: String,
    desc_long: String,
    ean: String,
    deliver_time: String,
    artikel_features: Vec<ArtikelFeature>,
}

impl Default for Artikel {
    fn default() -> Self {
        Artikel {
            id: "".to_string(),
            desc_short: "".to_string(),
            desc_long: "".to_string(),
            ean: "".to_string(),
            deliver_time: "".to_string(),
            artikel_features: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct ArtikelPrice {
    id: String,
    price: String,
}

#[derive(Debug, Clone)]
struct ArtikelFeature {
    name: String,
    value: String,
    unit: String,
}

impl Default for ArtikelFeature {
    fn default() -> Self {
        ArtikelFeature {
            name: "".to_string(),
            value: "".to_string(),
            unit: "".to_string(),
        }
    }
}
