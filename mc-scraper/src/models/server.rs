use anyhow::Result;
use percent_encoding::percent_decode_str;
use scrap::{Html, Selector};

pub struct Server {
    pub episode_id: i64,
    pub name: String,
    pub url: String,
}

impl Server {
    pub fn extract_many(doc: &Html) -> Result<Vec<Self>> {
        let options_sel = "ul.TPlayerNv li";
        let options_sel = Selector::parse(options_sel).unwrap();
        let options = doc.select(&options_sel);

        let iframes_sel = "div.TPlayer iframe";
        let iframes_sel = Selector::parse(iframes_sel).unwrap();
        let iframes = doc.select(&iframes_sel);

        let options_iframes = options.zip(iframes);

        let mut servers = Vec::with_capacity(8);
        for (option, iframe) in options_iframes {
            let server_name = option
                .value()
                .attr("title")
                .unwrap()
                .to_string();

            let iframe_src = iframe.value().attr("src").unwrap();
            let iframe_src = url::Url::parse(iframe_src)?;

            let queries = iframe_src.query_pairs();

            for (query, value) in queries {
                if query.to_string() == "url" {
                    let url = value.to_string();
                    let url = percent_decode_str(&url).decode_utf8()?.to_string();

                    let server = Server {
                        episode_id: 0,
                        name: server_name,
                        url,
                    };

                    servers.push(server);

                    break;
                }
            }
        }

        Ok(servers)
    }
}
