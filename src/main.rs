use chrono::prelude::*;

fn main() {
    let start_time = Local::now();

    let temp = std::fs::read_to_string("./files/ELTEN BMEcat 1.2.xml").unwrap();
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
                artikel
                    .artikel_feature_groups
                    .push(create_artikel_features(descen));
            }
            _ => (),
        }
    }
    println!("{:?}", artikel);
}

fn create_artikel_features(descen: roxmltree::Node) -> ArtikelFeatureGroup {
    let mut artikel_feature_group = ArtikelFeatureGroup {
        ..Default::default()
    };
    for descen2 in descen.descendants() {
        match descen2.tag_name().name() {
            "REFERENCE_FEATURE_SYSTEM_NAME" => {
                artikel_feature_group.sys_name = descen2.text().unwrap_or("").to_string();
            }
            "REFERENCE_FEATURE_GROUP_ID" => {
                artikel_feature_group.group_id = descen2.text().unwrap_or("").to_string();
            }
            "FEATURE" => {
                let mut artikel_feature = ArtikelFeature {
                    ..Default::default()
                };
                for descen3 in descen2.descendants() {
                    match descen3.tag_name().name() {
                        "FNAME" => {
                            artikel_feature.name = descen3.text().unwrap_or("").to_string();
                        }
                        "FVALUE" => {
                            artikel_feature.value = descen3.text().unwrap_or("").to_string();
                        }
                        "FUNIT" => {
                            artikel_feature.unit = descen3.text().unwrap_or("").to_string();
                        }
                        _ => (),
                    }
                }
                artikel_feature_group.artikel_features.push(artikel_feature);
            }
            _ => (),
        }
    }
    artikel_feature_group
}

#[derive(Debug, Clone)]
struct Artikel {
    id: String,
    desc_short: String,
    desc_long: String,
    ean: String,
    deliver_time: String,
    artikel_feature_groups: Vec<ArtikelFeatureGroup>,
}

impl Default for Artikel {
    fn default() -> Self {
        Artikel {
            id: "".to_string(),
            desc_short: "".to_string(),
            desc_long: "".to_string(),
            ean: "".to_string(),
            deliver_time: "".to_string(),
            artikel_feature_groups: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct ArtikelPrice {
    id: String,
    price: String,
}

#[derive(Debug, Clone)]
struct ArtikelFeatureGroup {
    sys_name: String,
    group_id: String,
    artikel_features: Vec<ArtikelFeature>,
}

impl Default for ArtikelFeatureGroup {
    fn default() -> Self {
        ArtikelFeatureGroup {
            sys_name: "".to_string(),
            group_id: "".to_string(),
            artikel_features: Vec::new(),
        }
    }
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
