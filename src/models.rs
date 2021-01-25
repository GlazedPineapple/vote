use std::{collections::HashMap, io::Write};

use super::schema::{cast_votes, polls};
use derive_more::{AsRef, Deref, DerefMut, From, Index, IndexMut, Into};
use diesel::{
    backend::Backend,
    deserialize, serialize,
    sql_types::Text,
    types::{FromSql, ToSql},
    Expression, Queryable,
};
use indexmap::IndexSet;
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
                Deref, AsRef, DerefMut, From, Into,
                AsExpression, FromSqlRow,
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
    #[derive(Index, IndexMut)]
    pub struct Moderators(Vec<UserId>);

    #[derive(Index, IndexMut)]
    pub struct Choices(HashMap<Uuid, String>);

    #[derive(Index, IndexMut)]
    pub struct Ranking(IndexSet<Uuid>);

    pub struct PollId(Uuid);
    pub struct VoteId(Uuid);
    pub struct DiscordUser(UserId);
}

#[derive(Queryable, Insertable)]
pub struct Poll {
    pub id: PollId,
    pub title: String,
    pub moderators: Moderators,
    pub choices: Choices,
}

#[derive(Queryable, Insertable)]
pub struct CastVote {
    pub id: VoteId,
    pub user: DiscordUser,
    pub poll: PollId,
    pub ranking: Ranking,
}
