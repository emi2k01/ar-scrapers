table! {
    anime_pages (id) {
        id -> Nullable<Integer>,
        title -> Nullable<Text>,
        url -> Nullable<Text>,
        html -> Nullable<Text>,
        html_len -> Nullable<Integer>,
    }
}

table! {
    animes (id) {
        id -> Nullable<Integer>,
        title -> Nullable<Text>,
        synopsis -> Nullable<Text>,
        status -> Nullable<Text>,
        release_date -> Nullable<Text>,
        kind -> Nullable<Text>,
    }
}

table! {
    animes_genres (id) {
        id -> Nullable<Integer>,
        anime_id -> Nullable<Integer>,
        genre_id -> Nullable<Integer>,
    }
}

table! {
    browse_pages (id) {
        id -> Nullable<Integer>,
        page -> Nullable<Integer>,
        url -> Nullable<Text>,
        html -> Nullable<Text>,
        html_len -> Nullable<Integer>,
    }
}

table! {
    episode_pages (id) {
        id -> Nullable<Integer>,
        anime_id -> Nullable<Integer>,
        number -> Nullable<Integer>,
        url -> Nullable<Text>,
        html -> Nullable<Text>,
        html_len -> Nullable<Integer>,
    }
}

table! {
    episodes (id) {
        id -> Nullable<Integer>,
        anime_id -> Nullable<Integer>,
        name -> Nullable<Text>,
    }
}

table! {
    genres (id) {
        id -> Nullable<Integer>,
        genre -> Nullable<Text>,
    }
}

table! {
    servers (id) {
        id -> Nullable<Integer>,
        episode_id -> Nullable<Integer>,
        name -> Nullable<Text>,
        url -> Nullable<Text>,
    }
}

joinable!(animes_genres -> animes (anime_id));
joinable!(animes_genres -> genres (genre_id));
joinable!(episode_pages -> anime_pages (anime_id));
joinable!(episodes -> animes (anime_id));
joinable!(servers -> episodes (episode_id));

allow_tables_to_appear_in_same_query!(
    anime_pages,
    animes,
    animes_genres,
    browse_pages,
    episode_pages,
    episodes,
    genres,
    servers,
);
