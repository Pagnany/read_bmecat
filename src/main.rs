use chrono::prelude::*;

fn main() {
    let start_time = Local::now();

    let temp = std::fs::read_to_string("./files/ELTEN BMEcat 1.2.xml").unwrap();
    let doc = roxmltree::Document::parse(&temp).unwrap();

    for node in doc.descendants() {
        match node.tag_name().name() {
            "ARTICLE" => {
                let temp = create_article(node);
                println!("{:?}", temp);

                // to test with only one article
                break;
            }
            _ => (),
        }
    }

    let end_time = Local::now();
    let duration = end_time.signed_duration_since(start_time);

    println!("Duration: {:?}", duration);
}

fn create_article(node: roxmltree::Node) -> Article {
    let mut article = Article {
        ..Default::default()
    };
    for descen in node.descendants() {
        match descen.tag_name().name() {
            "SUPPLIER_AID" => {
                article.id = descen.text().unwrap_or("").to_string();
            }
            "DESCRIPTION_SHORT" => {
                article.desc_short = descen.text().unwrap_or("").to_string();
            }
            "DESCRIPTION_LONG" => {
                article.desc_long = descen.text().unwrap_or("").to_string();
            }
            "EAN" => {
                article.ean = descen.text().unwrap_or("").to_string();
            }
            "DELIVERY_TIME" => {
                article.deliver_time = descen.text().unwrap_or("").to_string();
            }
            "ARTICLE_FEATURES" => {
                article
                    .article_feature_groups
                    .push(create_article_features(descen));
            }
            _ => (),
        }
    }
    article
}

fn create_article_features(descen: roxmltree::Node) -> ArticleFeatureGroup {
    let mut article_feature_group = ArticleFeatureGroup {
        ..Default::default()
    };
    for descen2 in descen.descendants() {
        match descen2.tag_name().name() {
            "REFERENCE_FEATURE_SYSTEM_NAME" => {
                article_feature_group.sys_name = descen2.text().unwrap_or("").to_string();
            }
            "REFERENCE_FEATURE_GROUP_ID" => {
                article_feature_group.group_id = descen2.text().unwrap_or("").to_string();
            }
            "FEATURE" => {
                let mut article_feature = ArticleFeature {
                    ..Default::default()
                };
                for descen3 in descen2.descendants() {
                    match descen3.tag_name().name() {
                        "FNAME" => {
                            article_feature.name = descen3.text().unwrap_or("").to_string();
                        }
                        "FVALUE" => {
                            article_feature.value = descen3.text().unwrap_or("").to_string();
                        }
                        "FUNIT" => {
                            article_feature.unit = descen3.text().unwrap_or("").to_string();
                        }
                        _ => (),
                    }
                }
                article_feature_group.article_features.push(article_feature);
            }
            _ => (),
        }
    }
    article_feature_group
}

#[derive(Debug, Clone)]
struct Article {
    id: String,
    desc_short: String,
    desc_long: String,
    ean: String,
    deliver_time: String,
    article_feature_groups: Vec<ArticleFeatureGroup>,
}

impl Default for Article {
    fn default() -> Self {
        Article {
            id: "".to_string(),
            desc_short: "".to_string(),
            desc_long: "".to_string(),
            ean: "".to_string(),
            deliver_time: "".to_string(),
            article_feature_groups: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct ArticlePrice {
    id: String,
    price: String,
}

#[derive(Debug, Clone)]
struct ArticleFeatureGroup {
    sys_name: String,
    group_id: String,
    article_features: Vec<ArticleFeature>,
}

impl Default for ArticleFeatureGroup {
    fn default() -> Self {
        ArticleFeatureGroup {
            sys_name: "".to_string(),
            group_id: "".to_string(),
            article_features: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct ArticleFeature {
    name: String,
    value: String,
    unit: String,
}

impl Default for ArticleFeature {
    fn default() -> Self {
        ArticleFeature {
            name: "".to_string(),
            value: "".to_string(),
            unit: "".to_string(),
        }
    }
}
