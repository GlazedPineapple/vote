use std::{collections::HashMap, io::Write};

use super::schema::{cast_votes, polls};
use chrono::{DateTime, Utc};
use derive_more::{AsRef, Constructor, Deref, DerefMut, From, Index, IndexMut, Into, IntoIterator};
use diesel::{
    backend::Backend,
    deserialize, serialize,
    sql_types::Text,
    types::{FromSql, ToSql},
    Expression, Queryable,
};
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use twilight_model::id::UserId;
use uuid::Uuid;

macro_rules! serde_sql_wrapper {
    ($(
        $(#[$outer:meta])*
        pub struct $name:ident($wrapped:ty);
    )*) => {
        $(
            $(#[$outer])*
            #[derive(
                Debug, Clone, PartialEq, Eq,
                Deref, AsRef, DerefMut, From, Into, Constructor,
                AsExpression, FromSqlRow,
                Serialize, Deserialize,
            )]
            pub struct $name($wrapped);

            impl Expression for $name {
                type SqlType = Text;
            }

            impl<DB: Backend> ToSql<Text, DB> for $name
            where
                String: ToSql<Text, DB>,
            {
                fn to_sql<W: Write>(&self, out: &mut serialize::Output<W, DB>) -> serialize::Result {
                    serde_json::to_string(&self.0)?.to_sql(out)
                }
            }

            impl<DB: Backend> FromSql<Text, DB> for $name
            where
                String: FromSql<Text, DB>,
            {
                fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
                    Ok($name(serde_json::from_str(&String::from_sql(bytes)?)?))
                }
            }
        )*
    };
}

serde_sql_wrapper! {
    #[derive(Index, IndexMut, IntoIterator)]
    pub struct Moderators(Vec<UserId>);

    #[derive(Index, IndexMut, IntoIterator)]
    pub struct Choices(HashMap<Uuid, Candidates>);

    #[derive(Index, IndexMut, IntoIterator)]
    pub struct Ranking(IndexSet<Uuid>);

    pub struct PollId(Uuid);
    pub struct VoteId(Uuid);
    pub struct DiscordUser(UserId);
    pub struct Timestamp(DateTime<Utc>);
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Candidates {
    pub president: String,
    pub vice_president: String,
}

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name="polls"]
pub struct PollRow {
    pub id: PollId,
    pub title: String,
    pub moderators: Moderators,
    pub choices: Choices,
    pub timestamp: Timestamp
}

#[derive(Queryable, Insertable, Debug, Clone)]
#[table_name="cast_votes"]
pub struct CastVoteRow {
    pub id: VoteId,
    pub user: DiscordUser,
    pub poll: PollId,
    pub ranking: Ranking,
    pub timestamp: Timestamp
}
