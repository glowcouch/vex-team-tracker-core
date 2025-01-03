use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Hash)]
pub struct Team {
    pub lock: Lock,
}

#[derive(Default, Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Hash)]
pub struct Note {
    pub team: u32,
    pub name: String,
    pub content: String,
}

#[cfg(feature = "fake")]
pub struct FakeNotes;

#[cfg(feature = "fake")]
impl fake::Dummy<FakeNotes> for Note {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_config: &FakeNotes, rng: &mut R) -> Self {
        use fake::{
            faker::{company::en::Buzzword, lorem::en::Sentences},
            Fake,
        };
        Self {
            team: rng.gen_range(0..99999),
            name: Buzzword().fake_with_rng(rng),
            content: {
                let sentences: Vec<String> = Sentences(0..rng.gen_range(1..10)).fake_with_rng(rng);
                sentences.concat()
            },
        }
    }
}

#[derive(Default, Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Hash)]
pub struct Person {
    pub team: u32,
    pub name: String,
    pub role: String,
}

#[cfg(feature = "fake")]
pub struct FakePerson;

#[cfg(feature = "fake")]
impl fake::Dummy<FakePerson> for Person {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &FakePerson, rng: &mut R) -> Self {
        use fake::{
            faker::{company::en::Profession, name::en::Name},
            Fake,
        };
        Person {
            team: rng.gen_range(0..99999),
            name: Name().fake_with_rng(rng),
            role: Profession().fake_with_rng(rng),
        }
    }
}

#[derive(Default, Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Hash)]
pub struct Robot {
    pub team: u32,
    pub images: Vec<String>,
    pub status: RobotStatus,
    pub features: String,
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
            team: rng.gen_range(0..99999),
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
    pub robot: String,
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
            robot: {
                let mut range: Vec<char> = ('a'..='z').collect::<Vec<char>>();
                range.append(&mut ('0'..='9').collect());
                (0..20)
                    .map(|_| range.get(rng.gen_range(0..range.len())).unwrap().to_owned())
                    .collect()
            },
            points: rng.gen_range(0..100),
            description: {
                let sentences: Vec<String> = Sentences(0..rng.gen_range(1..5)).fake_with_rng(rng);
                sentences.concat()
            },
        }
    }
}

#[derive(Default, Clone, Eq, PartialEq, Serialize, Deserialize, Debug, Hash)]
#[serde(tag = "t", content = "u")]
pub enum Lock {
    #[default]
    Unlocked,
    Locked(String),
}

#[cfg(feature = "fake")]
pub struct FakeLock;

#[cfg(feature = "fake")]
impl fake::Dummy<FakeLock> for Lock {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &FakeLock, rng: &mut R) -> Self {
        match rng.gen_bool(0.8) {
            true => Self::Unlocked,
            false => Self::Locked({
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
