use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record {
    artist: String,
    album: String,
    pressed: String,
    date_added: String,
}
impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
impl Record {
    pub fn from(mut c: Collection) -> Vec<Record> {
        let mut records: Vec<Record> = Vec::with_capacity(c.releases.len());
        for record in c.releases.iter_mut() {
            records.push(Record {
                artist: record.basic_information.artists[0].name.to_string(),
                album: record.basic_information.title.to_string(),
                pressed: record.basic_information.year.to_string(),
                date_added: record.date_added.to_string(),
            });
        }

        records
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Records(pub Vec<Record>);
impl std::fmt::Display for Records {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for v in &self.0 {
            write!(f, "{}\n", v)?;
        }
        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    // pub pagination: Pagination,
    pub releases: Vec<Release>,
}
impl std::fmt::Display for Collection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Collection: {:?}", self.releases)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Pagination {
    pub page: i64,
    pub pages: i64,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    pub items: i64,
    pub urls: Urls,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Urls {
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
