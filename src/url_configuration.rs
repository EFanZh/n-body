use rand::distributions::{Distribution, Standard};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum SchedulerType {
    Basic,
}

impl Distribution<SchedulerType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> SchedulerType {
        SchedulerType::Basic
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UrlConfiguration {
    pub scheduler: SchedulerType,
    pub id: u64,
}

pub fn random_url_configuration(seed: u64) -> UrlConfiguration {
    let mut rng = StdRng::seed_from_u64(seed);

    UrlConfiguration {
        scheduler: rng.gen(),
        id: rng.gen(),
    }
}
