use anyhow::Result;
use scrap::Html;

pub struct Anime {
    pub title: String,
    pub synopsis: String,
    pub status: String,
    pub release_date: String,
    pub kind: String,
    pub genres: Vec<String>,
}

impl Anime {
    pub fn extract(doc: &Html) -> Result<Self> {
        let parse_sel = |sel| {
            scrap::Selector::parse(sel)
                .map_err(|e| format_err!("error parsing sel [{}]: {:?}", sel, e))
        };

        let no_text_error = |el| format_err!("no text node in element in {}");

        let extract_first_text = |mut el: scrap::html::Select, el_sel| -> Result<String> {
            Ok(el
                .next()
                .ok_or(format_err!("no element matches"))?
                .text()
                .next()
                .ok_or(no_text_error(el_sel))?
                .to_string())
        };

        let title_sel = parse_sel("h1.Title")?;
        let synopsis_sel = parse_sel("div.Description")?;
        let status_sel = parse_sel("div.Type")?;
        let rel_date_and_kind_sel = parse_sel("div.after_title")?;
        let genres_sel = parse_sel("div.generos a")?;

        let title = extract_first_text(doc.select(&title_sel), "title")?;
        let synopsis = extract_first_text(doc.select(&synopsis_sel), "synopsis")?;
        let status = extract_first_text(doc.select(&status_sel), "status")?;

        let rel_date_and_kind =
            extract_first_text(doc.select(&rel_date_and_kind_sel), "release date and kind")?;
        let mut rel_date_and_kind = rel_date_and_kind.split(" | ");

        let release_date = rel_date_and_kind
            .next()
            .ok_or(format_err!("no release date"))?
            .to_string();

        let kind = rel_date_and_kind
            .next()
            .ok_or(no_text_error("no kind"))?
            .to_string();

        let genres_res = doc
            .select(&genres_sel)
            .map(|el| el.text().next().ok_or(no_text_error("genres")))
            .collect::<Vec<_>>();

        let mut genres = Vec::with_capacity(genres_res.len());
        for genre_res in genres_res {
            match genre_res {
                Ok(genre_str) => genres.push(genre_str.to_string()),
                Err(e) => return Err(e),
            }
        }

        Ok(Anime {
            title,
            synopsis,
            status,
            release_date,
            kind,
            genres,
        })
    }
}
