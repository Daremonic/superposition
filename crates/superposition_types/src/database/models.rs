pub mod cac;
#[cfg(feature = "experimentation")]
pub mod experimentation;

use std::str::FromStr;

use chrono::NaiveDateTime;
#[cfg(feature = "diesel_derives")]
use diesel::{
    sql_types::Text, AsChangeset, AsExpression, FromSqlRow, Insertable, QueryId,
    Queryable, Selectable,
};
use serde::{Deserialize, Serialize};
#[cfg(all(
    feature = "diesel_derives",
    not(feature = "disable_db_data_validation")
))]
use superposition_derives::TextFromSql;
#[cfg(all(feature = "diesel_derives", feature = "disable_db_data_validation"))]
use superposition_derives::TextFromSqlNoValidation;
#[cfg(feature = "diesel_derives")]
use superposition_derives::TextToSql;

#[cfg(feature = "diesel_derives")]
use super::superposition_schema::superposition::*;
#[cfg(feature = "disable_db_data_validation")]
use super::DisableDBValidation;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(try_from = "String")]
#[cfg_attr(
    feature = "diesel_derives",
    derive(AsExpression, FromSqlRow, TextToSql)
)]
#[cfg_attr(
    all(
        feature = "diesel_derives",
        not(feature = "disable_db_data_validation")
    ),
    derive(TextFromSql)
)]
#[cfg_attr(
    all(feature = "diesel_derives", feature = "disable_db_data_validation"),
    derive(TextFromSqlNoValidation)
)]
#[cfg_attr(feature = "diesel_derives", diesel(sql_type = Text))]
pub struct ChangeReason(String);
const CHANGE_REASON_CHAR_LIMIT: usize = 255;

impl Default for ChangeReason {
    fn default() -> Self {
        Self(String::from("Change Reason not provided"))
    }
}

#[cfg(feature = "disable_db_data_validation")]
impl DisableDBValidation for ChangeReason {
    type Source = String;
    fn from_db_unvalidated(data: Self::Source) -> Self {
        // Defaulting, to convert "" entries to Self::default
        Self::try_from(data).unwrap_or_default()
    }
}

impl From<&ChangeReason> for String {
    fn from(value: &ChangeReason) -> String {
        value.0.clone()
    }
}

impl TryFrom<String> for ChangeReason {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(String::from("Empty reason not allowed"));
        }
        let len = value.len();
        if len > CHANGE_REASON_CHAR_LIMIT {
            return Err(format!(
                "Reason longer than {CHANGE_REASON_CHAR_LIMIT} characters not allowed, current length: {len}",
            ));
        }
        Ok(Self(value))
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(try_from = "String")]
#[cfg_attr(
    feature = "diesel_derives",
    derive(AsExpression, FromSqlRow, TextToSql)
)]
#[cfg_attr(
    all(
        feature = "diesel_derives",
        not(feature = "disable_db_data_validation")
    ),
    derive(TextFromSql)
)]
#[cfg_attr(
    all(feature = "diesel_derives", feature = "disable_db_data_validation"),
    derive(TextFromSqlNoValidation)
)]
#[cfg_attr(feature = "diesel_derives", diesel(sql_type = Text))]
pub struct Description(String);
const DESCRIPTION_CHAR_LIMIT: usize = 1024;

impl Default for Description {
    fn default() -> Self {
        Self(String::from("Description not provided"))
    }
}

#[cfg(feature = "disable_db_data_validation")]
impl DisableDBValidation for Description {
    type Source = String;
    fn from_db_unvalidated(data: Self::Source) -> Self {
        // Defaulting, to convert "" entries to Self::default
        Self::try_from(data).unwrap_or_default()
    }
}

impl From<&Description> for String {
    fn from(value: &Description) -> String {
        value.0.clone()
    }
}

impl TryFrom<String> for Description {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(String::from("Empty description not allowed"));
        }
        let len = value.len();
        if len > DESCRIPTION_CHAR_LIMIT {
            return Err(format!(
                "Description longer than {DESCRIPTION_CHAR_LIMIT} characters not allowed, current length: {len}",
            ));
        }
        Ok(Self(value))
    }
}

#[derive(
    Debug, Clone, Copy, PartialEq, Deserialize, Serialize, strum_macros::Display,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(
    feature = "diesel_derives",
    derive(diesel_derive_enum::DbEnum, QueryId)
)]
#[cfg_attr(feature = "diesel_derives", DbValueStyle = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(
    feature = "diesel_derives",
    ExistingTypePath = "crate::database::superposition_schema::superposition::sql_types::OrgStatus"
)]
pub enum OrgStatus {
    Active,
    Inactive,
    PendingKyb,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(
    feature = "diesel_derives",
    derive(Queryable, Selectable, Insertable, AsChangeset)
)]
#[cfg_attr(feature = "diesel_derives", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(feature = "diesel_derives", diesel(primary_key(id)))]
#[cfg_attr(feature = "diesel_derives", diesel(treat_none_as_null = true))]
pub struct Organisation {
    pub id: String,
    pub name: String,
    pub country_code: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub created_by: String,
    pub admin_email: String,
    pub status: OrgStatus,
    pub sector: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub updated_by: String,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Deserialize, Serialize, strum_macros::Display,
)]
#[serde(rename_all = "UPPERCASE")]
#[strum(serialize_all = "UPPERCASE")]
#[cfg_attr(
    feature = "diesel_derives",
    derive(diesel_derive_enum::DbEnum, QueryId)
)]
#[cfg_attr(feature = "diesel_derives", DbValueStyle = "UPPERCASE")]
#[cfg_attr(
    feature = "diesel_derives",
    ExistingTypePath = "crate::database::superposition_schema::superposition::sql_types::WorkspaceStatus"
)]
pub enum WorkspaceStatus {
    ENABLED,
    DISABLED,
}

impl FromStr for WorkspaceStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ENABLED" => Ok(WorkspaceStatus::ENABLED),
            "DISABLED" => Ok(WorkspaceStatus::DISABLED),
            _ => Err(format!("Invalid enum string: {}", s)),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[cfg_attr(
    feature = "diesel_derives",
    derive(Queryable, Selectable, Insertable, AsChangeset)
)]
#[cfg_attr(feature = "diesel_derives", diesel(check_for_backend(diesel::pg::Pg)))]
#[cfg_attr(
    feature = "diesel_derives",
    diesel(primary_key(organisation_id, workspace_name))
)]
pub struct Workspace {
    pub organisation_id: String,
    pub organisation_name: String,
    pub workspace_name: String,
    pub workspace_schema_name: String,
    pub workspace_status: WorkspaceStatus,
    pub workspace_admin_email: String,
    pub created_by: String,
    pub last_modified_by: String,
    pub last_modified_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub mandatory_dimensions: Option<Vec<String>>,
}
