use std::sync::Arc;

use reqwest::{cookie::Jar, Client, Url};
use serde::{Deserialize, Serialize};

type StdResult<T, E> = std::result::Result<T, E>;

/// Result type used by this crate. This is equivalent
/// to `std::result::Result<T, mojang_api::Error>`.
pub type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
pub enum Error {
    Http(reqwest::Error),
    Json(serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NicoResult<T> {
    pub meta: NicoMeta,
    pub data: Option<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NicoMeta {
    pub status: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MylistsResponse {
    #[serde(default)]
    pub mylists: Vec<Mylist>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
#[serde(rename_all = "camelCase")]
pub struct Owner {
    pub owner_type: String,
    pub id: Option<String>,
    pub name: Option<String>,
    pub icon_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub item_id: usize,
    pub watch_id: String,
    pub description: String,
    pub added_at: String,
    pub status: String,
    pub video: Video,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Video {
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
    pub playback_position: Option<usize>,
    pub owner: Owner,
    pub require_sensitive_masking: bool,
    pub video_live: Option<String>,
    #[serde(rename(deserialize = "9d091f87"))]
    pub n_9d091f87: bool,
    #[serde(rename(deserialize = "acf68865"))]
    pub n_acf68865: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Count {
    pub view: usize,
    pub comment: usize,
    pub mylist: usize,
    pub like: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    pub url: String,
    pub middle_url: Option<String>,
    pub large_url: Option<String>,
    pub listing_url: Option<String>,
    pub n_hd_url: Option<String>,
}

pub async fn get_my_mylists(
    user_session: &str,
    user_session_secure: &str,
    sample_item_count: usize,
) -> Result<NicoResult<MylistsResponse>> {
    let url = format!(
        "https://nvapi.nicovideo.jp/v1/users/me/mylists?sampleItemCount={}",
        sample_item_count
    )
    .parse::<Url>()
    .expect("This is illegal");
    let cookie = format!(
        "user_session={}; user_session_secure={}",
        user_session, user_session_secure
    );
    let jar = Jar::default();
    jar.add_cookie_str(&cookie, &url);

    let client = Client::builder()
        .cookie_provider(Arc::new(jar))
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
        println!("{}", string);
    }
    let response = serde_json::from_str(&string).map_err(Error::Json)?;

    Ok(response)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MylistResponse {
    pub mylist: MylistDetail,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

pub async fn get_mylist(
    user_session: &str,
    user_session_secure: &str,
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
    let cookie = format!(
        "user_session={}; user_session_secure={}",
        user_session, user_session_secure
    );
    let jar = Jar::default();
    jar.add_cookie_str(&cookie, &url);

    let client = Client::builder()
        .cookie_provider(Arc::new(jar))
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
        println!("{}", string);
    }
    let response = serde_json::from_str(&string).map_err(Error::Json)?;

    Ok(response)
}

pub async fn get_mylist_all(
    user_session: &str,
    user_session_secure: &str,
    id: usize,
) -> Result<NicoResult<MylistResponse>> {
    let mut first_mylist = get_mylist(user_session, user_session_secure, id, 100, 1).await?;
    if let None = first_mylist.data {
        return Ok(first_mylist);
    };
    if !first_mylist
        .data
        .as_ref()
        .expect("must exist")
        .mylist
        .has_next
    {
        return Ok(first_mylist);
    }

    let mut page = 2;
    let mut extend_items = Vec::new();
    loop {
        let next_mylist = get_mylist(user_session, user_session_secure, id, 100, page).await?;
        if let Some(data) = next_mylist.data {
            extend_items.extend(data.mylist.items);
            if !data.mylist.has_next {
                break;
            }
            page += 1;
        } else {
            break;
        }
    }

    if let Some(ref mut data) = first_mylist.data {
        data.mylist.items.extend(extend_items);
    };

    Ok(first_mylist)
}

#[cfg(test)]
mod tests {
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
        let result = get_my_mylists(USER_SESSION, USER_SESSION_SECURE, 4)
            .await
            .unwrap();

        println!(
            "mylists len: {:?}",
            result.data.expect("should exist").mylists.len()
        );
    }

    #[tokio::test]
    async fn get_mylist_works() {
        let result = get_mylist(USER_SESSION, USER_SESSION_SECURE, 71381719, 100, 1)
            .await
            .unwrap();
        println!(
            "mylist len: {:?}",
            result.data.expect("should exist").mylist.items.len()
        );
    }

    #[tokio::test]
    async fn get_mylist_all_works() {
        let result = get_mylist_all(USER_SESSION, USER_SESSION_SECURE, 71381719)
            .await
            .unwrap();
        println!(
            "mylist len: {:?}",
            result.data.expect("should exist").mylist.items.len()
        );
    }
}
