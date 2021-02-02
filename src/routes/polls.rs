use rocket::{
    get,
    http::{ContentType, Status},
    response::Content,
};
use rocket_contrib::uuid::Uuid as RocketUuid;
use twilight_model::id::UserId;
use uuid::Uuid;

use crate::{
    models::{Candidates, Choices, DiscordUser, Moderators, Poll, PollId, Ranking, VoteId},
    templates::PollsList,
};

#[get("/polls")]
pub fn all_polls() -> PollsList {
    PollsList {
        polls: vec![
            Poll {
                id: Uuid::new_v4().into(),
                title: "cock".into(),
                moderators: Moderators::new(
                    [69, 420, 69420].iter().copied().map(UserId).collect(),
                ),
                choices: Choices::new(
                    [
                        Candidates {
                            president: "cock".into(),
                            vice_president: "balls".into(),
                        },
                        Candidates {
                            president: "zach".into(),
                            vice_president: "stinks".into(),
                        },
                        Candidates {
                            president: "no".into(),
                            vice_president: "nope".into(),
                        },
                    ]
                    .iter()
                    .cloned()
                    .map(|c| (Uuid::new_v4(), c))
                    .collect(),
                ),
            },
            Poll {
                id: Uuid::new_v4().into(),
                title: "My PENIS :D".into(),
                moderators: Moderators::new(
                    [100, 200, 300, 400, 500]
                        .iter()
                        .copied()
                        .map(UserId)
                        .collect(),
                ),
                choices: Choices::new(
                    [
                        Candidates {
                            president: "<b>Joe</b>".into(),
                            vice_president: "Mamma".into(),
                        },
                        Candidates {
                            president: "Elliot".into(),
                            vice_president: "Smells".into(),
                        },
                        Candidates {
                            president: ":D".into(),
                            vice_president: "D:".into(),
                        },
                    ]
                    .iter()
                    .cloned()
                    .map(|c| (Uuid::new_v4(), c))
                    .collect(),
                ),
            },
        ],
    }
}

#[get("/polls/<id>")]
pub fn poll_by_id(id: RocketUuid) -> Status {
    Status::NotImplemented
}
