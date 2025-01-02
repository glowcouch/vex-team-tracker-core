use serde::{Deserialize, Serialize};

#[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Team {
    pub data: TeamData,
    pub notes: TeamNotes,
}

#[cfg(feature = "fake")]
pub struct FakeTeam;

#[cfg(feature = "fake")]
impl fake::Dummy<FakeTeam> for Team {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &FakeTeam, rng: &mut R) -> Self {
        use fake::Fake;
        Team {
            data: FakeTeamData.fake_with_rng(rng),
            notes: FakeTeamNotes.fake_with_rng(rng),
        }
    }
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

#[cfg(feature = "fake")]
pub struct FakeCoordinates;

#[cfg(feature = "fake")]
impl fake::Dummy<FakeCoordinates> for Coordinates {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &FakeCoordinates, rng: &mut R) -> Self {
        Coordinates {
            latitude: rng.gen_range(-180.0..180.0),
            longitude: rng.gen_range(-180.0..180.0),
        }
    }
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Location {
    pub city: String,
    pub region: Option<String>,
    pub postcode: Option<String>,
    pub country: String,
    pub coords: Coordinates,
}

#[cfg(feature = "fake")]
pub struct FakeLocation;

#[cfg(feature = "fake")]
impl fake::Dummy<FakeLocation> for Location {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &FakeLocation, rng: &mut R) -> Self {
        use fake::{
            faker::address::en::{CityName, CountryName, PostCode},
            Fake,
        };
        Location {
            city: CityName().fake_with_rng(rng),
            region: CountryName().fake_with_rng(rng),
            postcode: PostCode().fake_with_rng(rng),
            country: CountryName().fake_with_rng(rng),
            coords: FakeCoordinates.fake_with_rng(rng),
        }
    }
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct TeamData {
    pub id: u32,
    pub number: String,
    pub name: String,
    pub organization: Option<String>,
    pub location: Location,
}

#[cfg(feature = "fake")]
pub struct FakeTeamData;

#[cfg(feature = "fake")]
impl fake::Dummy<FakeTeamData> for TeamData {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &FakeTeamData, rng: &mut R) -> Self {
        use fake::{
            faker::company::en::{Buzzword, CompanyName},
            Fake,
        };
        TeamData {
            id: rng.gen_range(0..99999),
            number: format!("{}{}", rng.gen_range(0..99999), rng.gen_range('A'..'X')),
            name: Buzzword().fake_with_rng(rng),
            organization: CompanyName().fake_with_rng(rng),
            location: FakeLocation.fake_with_rng(rng),
        }
    }
}

#[derive(Default, Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Hash)]
pub struct TeamNotes {
    pub robots: Vec<Robot>,
    pub members: Vec<TeamMember>,
    pub driving: String,
    pub strategy: String,
    pub notes: String,
    pub lock: Option<String>,
}

#[cfg(feature = "fake")]
pub struct FakeTeamNotes;

#[cfg(feature = "fake")]
impl fake::Dummy<FakeTeamNotes> for TeamNotes {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_config: &FakeTeamNotes, rng: &mut R) -> Self {
        use fake::{faker::lorem::en::Sentences, Fake};
        Self {
            robots: (1..rng.gen_range(0..5))
                .map(|_| FakeRobot.fake_with_rng(rng))
                .collect(),
            members: (0..rng.gen_range(1..30))
                .map(|_| FakeTeamMember.fake_with_rng(rng))
                .collect(),
            driving: {
                let sentences: Vec<String> = Sentences(0..rng.gen_range(1..10)).fake_with_rng(rng);
                sentences.concat()
            },
            strategy: {
                let sentences: Vec<String> = Sentences(0..rng.gen_range(1..10)).fake_with_rng(rng);
                sentences.concat()
            },
            notes: {
                let sentences: Vec<String> = Sentences(0..rng.gen_range(1..10)).fake_with_rng(rng);
                sentences.concat()
            },
            lock: FakeLock.fake_with_rng(rng),
        }
    }
}

#[derive(Default, Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Hash)]
pub struct TeamMember {
    pub name: String,
    pub role: String,
}

#[cfg(feature = "fake")]
pub struct FakeTeamMember;

#[cfg(feature = "fake")]
impl fake::Dummy<FakeTeamMember> for TeamMember {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &FakeTeamMember, rng: &mut R) -> Self {
        use fake::{
            faker::{company::en::Profession, name::en::Name},
            Fake,
        };
        TeamMember {
            name: Name().fake_with_rng(rng),
            role: Profession().fake_with_rng(rng),
        }
    }
}

#[derive(Default, Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Hash)]
pub struct Robot {
    pub images: Vec<String>,
    pub status: RobotStatus,
    pub features: String,
    pub autons: Vec<RobotAuton>,
}

#[cfg(feature = "fake")]
pub struct FakeRobot;

#[cfg(feature = "fake")]
impl fake::Dummy<FakeRobot> for Robot {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &FakeRobot, rng: &mut R) -> Self {
        use fake::{
            faker::{filesystem::en::FileExtension, lorem::en::Words},
            Fake,
        };
        Robot {
            images: (1..rng.gen_range(0..10))
                .map(|_| {
                    let id: u64 = rng.gen();
                    let extension: String = FileExtension().fake_with_rng(rng);
                    format!("{id}.{extension}")
                })
                .collect(),
            status: FakeRobotStatus.fake_with_rng(rng),
            features: {
                let words: Vec<String> = Words(0..rng.gen_range(1..5)).fake_with_rng(rng);
                words.concat()
            },
            autons: (0..rng.gen_range(1..10))
                .map(|_| FakeRobotAuton.fake_with_rng(rng))
                .collect(),
        }
    }
}

#[derive(Default, Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Hash)]
pub enum RobotStatus {
    #[default]
    Active,
    Inactive,
}

#[cfg(feature = "fake")]
pub struct FakeRobotStatus;

#[cfg(feature = "fake")]
impl fake::Dummy<FakeRobotStatus> for RobotStatus {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &FakeRobotStatus, rng: &mut R) -> Self {
        match rng.gen_bool(0.8) {
            true => Self::Active,
            false => Self::Inactive,
        }
    }
}

#[derive(Default, Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Hash)]
pub struct RobotAuton {
    pub points: i32,
    pub description: String,
}

#[cfg(feature = "fake")]
pub struct FakeRobotAuton;

#[cfg(feature = "fake")]
impl fake::Dummy<FakeRobotAuton> for RobotAuton {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &FakeRobotAuton, rng: &mut R) -> Self {
        use fake::{faker::lorem::en::Sentences, Fake};
        RobotAuton {
            points: rng.gen_range(0..100),
            description: {
                let sentences: Vec<String> = Sentences(0..rng.gen_range(1..5)).fake_with_rng(rng);
                sentences.concat()
            },
        }
    }
}

#[cfg(feature = "fake")]
pub struct FakeLock;

#[cfg(feature = "fake")]
impl fake::Dummy<FakeLock> for Option<String> {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &FakeLock, rng: &mut R) -> Self {
        match rng.gen_bool(0.8) {
            true => None,
            false => Some({
                let o: u64 = rng.gen();
                o.to_string()
            }),
        }
    }
}

#[derive(Default, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Statistics {
    pub average_score: f32,
    pub average_net_score: f32,
}

#[cfg(feature = "fake")]
pub struct FakeStatistics;

#[cfg(feature = "fake")]
impl fake::Dummy<FakeStatistics> for Statistics {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &FakeStatistics, rng: &mut R) -> Self {
        Statistics {
            average_score: rng.gen_range(0.0..20.0),
            average_net_score: rng.gen_range(-10.0..=10.0),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Hash)]
pub enum SocialPlatform {
    Instagram,
    Youtube,
}

#[cfg(feature = "fake")]
pub struct FakeSocialPlatform;

#[cfg(feature = "fake")]
impl fake::Dummy<FakeSocialPlatform> for SocialPlatform {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &FakeSocialPlatform, rng: &mut R) -> Self {
        match rng.gen_range(0..=1) {
            0 => SocialPlatform::Instagram,
            1 => SocialPlatform::Youtube,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Hash)]
pub struct SocialAcc {
    pub platform: SocialPlatform,
    pub url: String,
    pub name: String,
}

#[cfg(feature = "fake")]
pub struct FakeSocialAcc;

#[cfg(feature = "fake")]
impl fake::Dummy<FakeSocialAcc> for SocialAcc {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &FakeSocialAcc, rng: &mut R) -> Self {
        use fake::{faker::company::en::CompanyName, Fake};
        let platform: SocialPlatform = FakeSocialPlatform.fake_with_rng(rng);
        let name: String = CompanyName().fake_with_rng(rng);
        SocialAcc {
            platform: platform.clone(),
            url: match &platform {
                SocialPlatform::Instagram => {
                    format!("https://www.instagram.com/{}/", name.replace(" ", "_"))
                }
                SocialPlatform::Youtube => {
                    format!("https://www.youtube.com/@{}/", name.replace(" ", "_"))
                }
            },
            name,
        }
    }
}
