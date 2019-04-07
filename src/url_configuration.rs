use rand::distributions::{Distribution, Standard};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum RenderType {
    Basic,
}

impl Distribution<RenderType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> RenderType {
        RenderType::Basic
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UrlConfiguration {
    pub render_type: RenderType,
    pub id: u64,
}

pub fn random_url_configuration(seed: u64) -> UrlConfiguration {
    let mut rng = StdRng::seed_from_u64(seed);

    UrlConfiguration {
        render_type: rng.gen(),
        id: rng.gen(),
    }
}
