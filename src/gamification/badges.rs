use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum Badge {
    FirstCommit,
    Polyglot,
    GuildMember,
    RaidLeader,
}