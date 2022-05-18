use std::sync::Arc;

use reqwest::{cookie::CookieStore, Client, Url};
use serde::{Deserialize, Serialize};

#[cfg(feature = "ts")]
use ts_rs::TS;

type StdResult<T, E> = std::result::Result<T, E>;

/// Result type used by this crate. This is equivalent
/// to `std::result::Result<T, mojang_api::Error>`.
pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    Status(NicoError),
    Http(reqwest::Error),
    Json(serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "ts", ts(export))]
pub struct NicoResult<T> {
    pub meta: NicoMeta,
    pub data: T,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "ts", ts(export))]
pub struct NicoError {
    pub meta: NicoMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct NicoMeta {
    pub status: usize,
    pub error_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "ts", ts(export))]
pub struct MylistsResponse {
    #[serde(default)]
    pub mylists: Vec<Mylist>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct Mylist {
    pub id: usize,
    pub is_public: bool,
    pub name: String,
    pub description: String,
    pub default_sort_key: String,
    pub default_sort_order: String,
    pub items_count: usize,
    pub owner: Owner,
    #[serde(default)]
    pub sample_items: Vec<Item>,
    pub follower_count: usize,
    pub created_at: String,
    pub is_following: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    pub owner_type: String,
    pub id: Option<String>,
    pub name: Option<String>,
    pub icon_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub item_id: usize,
    pub watch_id: String,
    pub description: String,
    pub added_at: String,
    pub status: String,
    pub video: Video,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct Video {
    #[cfg_attr(feature = "ts", ts(rename = "type"))]
    #[serde(rename(deserialize = "type"))]
    pub video_type: String,
    pub id: String,
    pub title: String,
    pub registered_at: String,
    pub count: Count,
    pub thumbnail: Thumbnail,
    pub duration: usize,
    pub short_description: String,
    pub latest_comment_summary: String,
    pub is_channel_video: bool,
    pub is_payment_required: bool,
    pub playback_position: Option<f32>,
    pub owner: Owner,
    pub require_sensitive_masking: bool,
    pub video_live: Option<String>,
    // #[cfg_attr(feature = "ts", ts(rename = "9d091f87"))]
    #[serde(rename(deserialize = "9d091f87"))]
    pub n_9d091f87: bool,
    // #[cfg_attr(feature = "ts", ts(rename = "acf68865"))]
    #[serde(rename(deserialize = "acf68865"))]
    pub n_acf68865: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct Count {
    pub view: usize,
    pub comment: usize,
    pub mylist: usize,
    pub like: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    pub url: String,
    pub middle_url: Option<String>,
    pub large_url: Option<String>,
    pub listing_url: Option<String>,
    pub n_hd_url: Option<String>,
}

pub async fn get_my_mylists<C: CookieStore + 'static>(
    cookie_store: Arc<C>,
    sample_item_count: usize,
) -> Result<NicoResult<MylistsResponse>> {
    let url = format!(
        "https://nvapi.nicovideo.jp/v1/users/me/mylists?sampleItemCount={}",
        sample_item_count
    )
    .parse::<Url>()
    .expect("This is illegal");

    let client = Client::builder()
        .cookie_provider(cookie_store)
        .build()
        .map_err(Error::Http)?;
    let response = client
        .get(url)
        .header("X-Frontend-Id", "6")
        .send()
        .await
        .map_err(Error::Http)?;
    let status_code = response.status();
    let string = response.text().await.map_err(Error::Http)?;

    if status_code.as_u16() > 299u16 {
        let err: NicoError = serde_json::from_str(&string).map_err(Error::Json)?;
        return Err(Error::Status(err));
    }

    let response = serde_json::from_str(&string).map_err(Error::Json)?;

    Ok(response)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "ts", ts(export))]
pub struct MylistResponse {
    pub mylist: MylistDetail,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "ts", derive(TS))]
#[cfg_attr(feature = "ts", ts(export))]
#[serde(rename_all = "camelCase")]
pub struct MylistDetail {
    pub id: usize,
    pub name: String,
    pub description: String,
    pub default_sort_key: String,
    pub default_sort_order: String,
    #[serde(default)]
    pub items: Vec<Item>,
    pub total_item_count: usize,
    pub has_next: bool,
    pub is_public: bool,
    pub owner: Owner,
    pub has_invisible_items: bool,
    pub follower_count: usize,
    pub is_following: bool,
}

pub async fn get_mylist<C: CookieStore + 'static>(
    cookie_store: Arc<C>,
    id: usize,
    page_size: usize,
    page: usize,
) -> Result<NicoResult<MylistResponse>> {
    let url = format!(
        "https://nvapi.nicovideo.jp/v1/users/me/mylists/{}?pageSize={}&page={}",
        id, page_size, page
    )
    .parse::<Url>()
    .expect("This is illegal");
    let client = Client::builder()
        .cookie_provider(cookie_store)
        .build()
        .map_err(Error::Http)?;
    let response = client
        .get(url)
        .header("X-Frontend-Id", "6")
        .send()
        .await
        .map_err(Error::Http)?;
    let status_code = response.status();
    let string = response.text().await.map_err(Error::Http)?;

    if status_code.as_u16() > 299u16 {
        let err: NicoError = serde_json::from_str(&string).map_err(Error::Json)?;
        return Err(Error::Status(err));
    }

    let response = serde_json::from_str(&string).map_err(Error::Json)?;

    Ok(response)
}

pub async fn get_mylist_all<C: CookieStore + 'static>(
    cookie_store: Arc<C>,
    id: usize,
) -> Result<NicoResult<MylistResponse>> {
    let mut first_mylist = get_mylist(cookie_store.clone(), id, 100, 1).await?;
    if !first_mylist.data.mylist.has_next {
        return Ok(first_mylist);
    }

    let mut page = 2;
    let mut extend_items = Vec::new();
    loop {
        let next_mylist = get_mylist(cookie_store.clone(), id, 100, page).await?;
        extend_items.extend(next_mylist.data.mylist.items);
        if !next_mylist.data.mylist.has_next {
            break;
        }
        page += 1;
    }

    first_mylist.data.mylist.items.extend(extend_items);

    Ok(first_mylist)
}

#[cfg(test)]
mod tests {
    use reqwest::cookie::Jar;

    use super::*;

    static USER_SESSION: &str = "user_session_replace_this";
    static USER_SESSION_SECURE: &str = "replace_this";

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn get_my_mylists_works() {
        let jar = Jar::default();
        jar.add_cookie_str(
            &format!(
                "user_session={}; user_session_secure={};",
                USER_SESSION, USER_SESSION_SECURE
            ),
            &Url::parse("https://nvapi.nicovideo.jp/").unwrap(),
        );

        let result = get_my_mylists(Arc::new(jar), 4).await.unwrap();

        println!("mylists len: {:?}", result.data.mylists.len());
    }

    #[tokio::test]
    async fn get_mylist_works() {
        let jar = Jar::default();
        jar.add_cookie_str(
            &format!(
                "user_session={}; user_session_secure={};",
                USER_SESSION, USER_SESSION_SECURE
            ),
            &Url::parse("https://nvapi.nicovideo.jp/").unwrap(),
        );

        let result = get_mylist(Arc::new(jar), 71381719, 100, 1).await.unwrap();
        println!("mylist len: {:?}", result.data.mylist.items.len());
    }

    #[tokio::test]
    async fn get_mylist_all_works() {
        let jar = Jar::default();
        jar.add_cookie_str(
            &format!(
                "user_session={}; user_session_secure={};",
                USER_SESSION, USER_SESSION_SECURE
            ),
            &Url::parse("https://nvapi.nicovideo.jp/").unwrap(),
        );

        let result = get_mylist_all(Arc::new(jar), 71381719).await.unwrap();
        println!("mylist len: {:?}", result.data.mylist.items.len());
    }
}
