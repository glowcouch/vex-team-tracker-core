use serde::{Deserialize, Serialize};

#[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Team {
    pub data: TeamData,
    pub notes: TeamNotes,
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Location {
    pub city: String,
    pub region: Option<String>,
    pub postcode: Option<String>,
    pub country: String,
    pub coords: Coordinates,
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct TeamData {
    pub id: u32,
    pub number: String,
    pub name: String,
    pub organization: String,
    pub location: Location,
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug, Hash)]
pub struct TeamNotes {
    pub robots: Vec<Robot>,
    pub members: Vec<TeamMember>,
    pub driving: String,
    pub strategy: String,
    pub notes: String,
    pub lock: Lock,
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug, Hash)]
pub struct TeamMember {
    pub name: String,
    pub role: String,
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug, Hash)]
pub struct Robot {
    pub images: Vec<String>,
    pub status: RobotStatus,
    pub features: String,
    pub autons: Vec<RobotAuton>,
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug, Hash)]
pub enum RobotStatus {
    #[default]
    Active,
    Inactive,
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug, Hash)]
pub struct RobotAuton {
    pub points: i32,
    pub description: String,
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug, Hash)]
pub enum Lock {
    Locked(String),
    #[default]
    Unlocked,
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Statistics {
    pub average_score: f32,
    pub average_net_score: f32,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Hash)]
pub enum SocialPlatform {
    Instagram,
    Youtube,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Hash)]
pub struct SocialAcc {
    pub platform: SocialPlatform,
    pub url: String,
    pub name: String,
}
