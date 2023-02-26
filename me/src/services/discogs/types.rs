// use datetime::LocalDateTime;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

// #[derive(Debug)]
// struct Record {
//     artist: String,
//     title: String,
//     pressed: String,
//     date_added: LocalDateTime,
// }

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionResponse {
    // pub pagination: Pagination,
    pub releases: Vec<Release>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub page: i64,
    pub pages: i64,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    pub items: i64,
    pub urls: Urls,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Urls {
    pub last: String,
    pub next: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Release {
    pub id: i64,
    #[serde(rename = "instance_id")]
    pub instance_id: i64,
    #[serde(rename = "date_added")]
    pub date_added: String,
    pub rating: i64,
    #[serde(rename = "basic_information")]
    pub basic_information: BasicInformation,
    #[serde(rename = "folder_id")]
    pub folder_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BasicInformation {
    pub id: i64,
    #[serde(rename = "master_id")]
    pub master_id: i64,
    #[serde(rename = "master_url")]
    pub master_url: String,
    #[serde(rename = "resource_url")]
    pub resource_url: String,
    pub thumb: String,
    #[serde(rename = "cover_image")]
    pub cover_image: String,
    pub title: String,
    pub year: i64,
    pub formats: Vec<Format>,
    pub labels: Vec<Label>,
    pub artists: Vec<Artist>,
    pub genres: Vec<String>,
    pub styles: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Format {
    pub name: String,
    pub qty: String,
    // pub text: String,
    pub descriptions: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub name: String,
    pub catno: String,
    #[serde(rename = "entity_type")]
    pub entity_type: String,
    #[serde(rename = "entity_type_name")]
    pub entity_type_name: String,
    pub id: i64,
    #[serde(rename = "resource_url")]
    pub resource_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub name: String,
    pub anv: String,
    pub join: String,
    pub role: String,
    pub tracks: String,
    pub id: i64,
    #[serde(rename = "resource_url")]
    pub resource_url: String,
}

