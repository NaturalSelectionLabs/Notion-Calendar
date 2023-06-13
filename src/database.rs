use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct DatabaseResponse {
    // Define the structure of the data you expect to receive from the API
    // based on your Notion database schema.
    // Adjust the fields according to your specific database schema.
    // object: String,
    pub results: Vec<DatabaseEntry>,
    // ... other fields from the response
}

#[derive(Debug, Deserialize)]
pub struct DatabaseEntry {
    // Define the structure of each entry in the Notion database.
    // Adjust the fields according to your specific database schema.
    pub object: String,
    pub id: String,
    #[serde(rename = "created_time")]
    pub created_time: String,
    #[serde(rename = "last_edited_time")]
    pub last_edited_time: String,
    #[serde(rename = "created_by")]
    pub created_by: CreatedBy,
    #[serde(rename = "last_edited_by")]
    pub last_edited_by: LastEditedBy,
    pub cover: Value,
    pub icon: Option<Icon>,
    pub parent: Parent,
    pub archived: bool,
    pub properties: Properties,
    pub url: String,
    #[serde(rename = "public_url")]
    pub public_url: Value,
    // ... other fields from the database entry
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedBy {
    pub object: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastEditedBy {
    pub object: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon {
    #[serde(rename = "type")]
    pub type_field: String,
    pub emoji: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parent {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "database_id")]
    pub database_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    #[serde(rename = "Content")]
    pub content: Content,
    #[serde(rename = "Status")]
    pub status: Status,
    #[serde(rename = "URL")]
    pub url: Url,
    #[serde(rename = "Publish date")]
    pub publish_date: PublishDate,
    #[serde(rename = "Poster")]
    pub poster: Poster,
    #[serde(rename = "Reviewer")]
    pub reviewer: Reviewer,
    #[serde(rename = "Name")]
    pub name: Name,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "rich_text")]
    pub rich_text: Vec<RichText>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichText {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: Text,
    pub annotations: Annotations,
    #[serde(rename = "plain_text")]
    pub plain_text: String,
    pub href: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub content: String,
    pub link: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotations {
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub underline: bool,
    pub code: bool,
    pub color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub select: Option<Select>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Select {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Url {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishDate {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub date: Option<Date>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Date {
    pub start: String,
    pub end: Option<String>,
    #[serde(rename = "time_zone")]
    pub time_zone: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Poster {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub checkbox: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reviewer {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub people: Vec<People2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct People2 {
    pub object: String,
    pub id: String,
    pub name: Option<String>,
    #[serde(rename = "avatar_url")]
    pub avatar_url: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub person: Option<Person2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Person2 {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub title: Vec<Title>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: Option<Text2>,
    pub annotations: Annotations2,
    #[serde(rename = "plain_text")]
    pub plain_text: String,
    pub href: Option<String>,
    pub mention: Option<Mention>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text2 {
    pub content: String,
    pub link: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotations2 {
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub underline: bool,
    pub code: bool,
    pub color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mention {
    #[serde(rename = "type")]
    pub type_field: String,
    pub page: Page,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page2 {}
