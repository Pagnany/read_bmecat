pub fn read_bmecat(file: String) -> Vec<Article> {
    let doc = roxmltree::Document::parse(&file).unwrap();

    let mut articles = Vec::new();
    for node in doc.descendants() {
        match node.tag_name().name() {
            "T_NEW_CATALOG" => {
                for descen in node.descendants() {
                    match descen.tag_name().name() {
                        "ARTICLE" => {
                            articles.push(create_article(descen));
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
    articles
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
            "MIME_INFO" => {
                article.mime_infos = create_article_mime_info(descen);
            }
            _ => (),
        }
    }
    article
}

fn create_article_mime_info(node: roxmltree::Node) -> Vec<Mime> {
    let mut mime_infos = Vec::new();
    for descen in node.descendants() {
        match descen.tag_name().name() {
            "MIME" => {
                mime_infos.push(create_article_mime(descen));
            }
            _ => (),
        }
    }
    mime_infos
}

fn create_article_mime(node: roxmltree::Node) -> Mime {
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

#[derive(Debug, Clone, Default)]
pub struct Article {
    pub id: String,
    pub mime_infos: Vec<Mime>,
}

impl Article {
    pub fn get_pictures(&self) -> Vec<String> {
        let mut pictures = Vec::new();
        for mime in &self.mime_infos {
            if mime.mime_type == "image/jpeg" || mime.mime_type == "image/png" {
                pictures.push(mime.mime_source.clone());
            }
        }
        pictures
    }
}

#[derive(Debug, Clone, Default)]
pub struct Mime {
    pub mime_type: String,
    pub mime_source: String,
    pub mime_descr: String,
    pub mime_alt: String,
    pub mime_purpose: String,
    pub mime_order: String,
}
