use chrono::prelude::*;

fn main() {
    let start_time = Local::now();

    let temp = std::fs::read_to_string("./files/Boschimp.xml").unwrap();
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
                article.article_details.desc_short = descen.text().unwrap_or("").to_string();
            }
            "DESCRIPTION_LONG" => {
                article.article_details.desc_long = descen.text().unwrap_or("").to_string();
            }
            "EAN" => {
                article.article_details.ean = descen.text().unwrap_or("").to_string();
            }
            "SUPPLIER_ALT_AID" => {
                article.article_details.supplier_alt_id = descen.text().unwrap_or("").to_string();
            }
            "MANUFACTURER_AID" => {
                article.article_details.manufacturer_id = descen.text().unwrap_or("").to_string();
            }
            "MANUFACTURER_NAME" => {
                article.article_details.manufacturer_name = descen.text().unwrap_or("").to_string();
            }
            "MANUFACTURER_TYPE_DESC" => {
                article.article_details.manufacturer_type_desc =
                    descen.text().unwrap_or("").to_string();
            }
            "ERP_GROUP_BUYER" => {
                article.article_details.erp_group_buyer = descen.text().unwrap_or("").to_string();
            }
            "ERP_GROUP_SUPPLIER" => {
                article.article_details.erp_group_supplier =
                    descen.text().unwrap_or("").to_string();
            }
            "DELIVERY_TIME" => {
                article.article_details.deliver_time = descen.text().unwrap_or("").to_string();
            }
            "REMARKS" => {
                article.article_details.remarks = descen.text().unwrap_or("").to_string();
            }
            "SEGMENT" => {
                article.article_details.segment = descen.text().unwrap_or("").to_string();
            }
            "ARTICLE_ORDER_DETAILS" => {
                article.article_details.article_order = descen.text().unwrap_or("").to_string();
            }

            "ARTICLE_FEATURES" => {
                article
                    .article_feature_groups
                    .push(create_article_features(descen));
            }
            "MIME_INFO" => {
                article.mime_infos = create_mime_info(descen);
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
            "REFERENCE_FEATURE_GROUP_NAME" => {
                article_feature_group.group_name = descen2.text().unwrap_or("").to_string();
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
                        "FORDER" => {
                            article_feature.order = descen3.text().unwrap_or("").to_string();
                        }
                        "FDESCR" => {
                            article_feature.descr = descen3.text().unwrap_or("").to_string();
                        }
                        "FVALUE_DETAILS" => {
                            article_feature.value_details =
                                descen3.text().unwrap_or("").to_string();
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

fn create_mime_info(node: roxmltree::Node) -> Vec<Mime> {
    let mut mime_infos = Vec::new();
    for descen in node.descendants() {
        match descen.tag_name().name() {
            "MIME" => {
                mime_infos.push(create_mime(descen));
            }
            _ => (),
        }
    }
    mime_infos
}

fn create_mime(node: roxmltree::Node) -> Mime {
    let mut mime = Mime {
        ..Default::default()
    };
    for descen in node.descendants() {
        match descen.tag_name().name() {
            "MIME_TYPE" => {
                mime.mime_type = descen.text().unwrap_or("").to_string();
            }
            "MIME_SOURCE" => {
                mime.mime_source = descen.text().unwrap_or("").to_string();
            }
            "MIME_DESCR" => {
                mime.mime_descr = descen.text().unwrap_or("").to_string();
            }
            "MIME_ALT" => {
                mime.mime_alt = descen.text().unwrap_or("").to_string();
            }
            "MIME_PURPOSE" => {
                mime.mime_purpose = descen.text().unwrap_or("").to_string();
            }
            "MIME_ORDER" => {
                mime.mime_order = descen.text().unwrap_or("").to_string();
            }
            _ => (),
        }
    }
    mime
}

#[derive(Debug, Clone)]
struct Article {
    id: String,
    article_details: ArtikelDetails,
    article_feature_groups: Vec<ArticleFeatureGroup>,
    mime_infos: Vec<Mime>,
}

impl Default for Article {
    fn default() -> Self {
        Article {
            id: "".to_string(),
            article_details: ArtikelDetails {
                ..Default::default()
            },
            article_feature_groups: Vec::new(),
            mime_infos: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct ArtikelDetails {
    desc_short: String,
    desc_long: String,
    ean: String,
    supplier_alt_id: String,
    manufacturer_id: String,
    manufacturer_name: String,
    manufacturer_type_desc: String,
    erp_group_buyer: String,
    erp_group_supplier: String,
    deliver_time: String,
    remarks: String,
    segment: String,
    article_order: String,
}

impl Default for ArtikelDetails {
    fn default() -> Self {
        ArtikelDetails {
            desc_short: "".to_string(),
            desc_long: "".to_string(),
            ean: "".to_string(),
            supplier_alt_id: "".to_string(),
            manufacturer_id: "".to_string(),
            manufacturer_name: "".to_string(),
            manufacturer_type_desc: "".to_string(),
            erp_group_buyer: "".to_string(),
            erp_group_supplier: "".to_string(),
            deliver_time: "".to_string(),
            remarks: "".to_string(),
            segment: "".to_string(),
            article_order: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
struct ArticleFeatureGroup {
    sys_name: String,
    group_id: String,
    group_name: String,
    article_features: Vec<ArticleFeature>,
}

impl Default for ArticleFeatureGroup {
    fn default() -> Self {
        ArticleFeatureGroup {
            sys_name: "".to_string(),
            group_id: "".to_string(),
            group_name: "".to_string(),
            article_features: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct ArticleFeature {
    name: String,
    value: String,
    unit: String,
    order: String,
    descr: String,
    value_details: String,
}

impl Default for ArticleFeature {
    fn default() -> Self {
        ArticleFeature {
            name: "".to_string(),
            value: "".to_string(),
            unit: "".to_string(),
            order: "".to_string(),
            descr: "".to_string(),
            value_details: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
struct ArticleOrderDetails {}

#[derive(Debug, Clone)]
struct ArticlePriceDetails {}

#[derive(Debug, Clone)]
struct Mime {
    mime_type: String,
    mime_source: String,
    mime_descr: String,
    mime_alt: String,
    mime_purpose: String,
    mime_order: String,
}

impl Default for Mime {
    fn default() -> Self {
        Mime {
            mime_type: "".to_string(),
            mime_source: "".to_string(),
            mime_descr: "".to_string(),
            mime_alt: "".to_string(),
            mime_purpose: "".to_string(),
            mime_order: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
struct UserDefinedExtensions {}
