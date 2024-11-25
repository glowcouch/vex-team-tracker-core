use serde::{Deserialize, Serialize};

#[derive(Default, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Team {
    pub data: TeamData,
    pub notes: TeamNotes,
}

#[derive(Default, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct TeamData {
    pub id: u32,
    pub number: String,
    pub name: String,
    pub organization: String,
}

#[derive(Default, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct TeamNotes {
    pub robots: Vec<Robot>,
    pub members: Vec<TeamMember>,
    pub driving: String,
    pub strategy: String,
    pub notes: String,
    pub lock: Lock,
}

#[derive(Default, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct TeamMember {
    pub name: String,
    pub role: String,
}

#[derive(Default, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Robot {
    pub images: Vec<String>,
    pub status: RobotStatus,
    pub features: String,
    pub autons: Vec<RobotAuton>,
}

#[derive(Default, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum RobotStatus {
    #[default]
    Active,
    Inactive,
}

#[derive(Default, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct RobotAuton {
    pub points: i32,
    pub description: String,
}

#[derive(Default, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum Lock {
    Locked(String),
    #[default]
    Unlocked,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum SocialPlatform {
    Instagram,
    Youtube,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct SocialAcc {
    pub platform: SocialPlatform,
    pub url: String,
    pub name: String,
    pub profile_pic_url: Option<String>,
}
