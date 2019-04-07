use crate::renderer::Renderer;
use crate::scheduler::Scheduler;
use crate::universe::Universe;
use cgmath::{InnerSpace, Vector2};

pub struct BasicScheduler {
    sample_frequency: f64,
    step_size: f64,                             // Pre-calculated.
    sampled: u64,                               // State.
    last_timestamp: f64,                        // State.
    start_time: f64,                            // State.
    position_histories: Vec<Vec<Vector2<f64>>>, // Shared Buffer.
}

impl BasicScheduler {
    pub fn new<U: Universe>(sample_frequency: f64, universe: &U) -> BasicScheduler {
        BasicScheduler {
            sample_frequency: sample_frequency / 1000.0,
            step_size: sample_frequency.recip(),
            sampled: 0,
            last_timestamp: 0.0,
            start_time: 0.0,
            position_histories: universe.get_bodies().iter().map(|b| vec![b.position]).collect(),
        }
    }
}

impl Scheduler for BasicScheduler {
    fn advance<U: Universe, R: Renderer>(&mut self, mut timestamp: f64, universe: &mut U, renderer: &mut R) {
        timestamp -= self.start_time;

        // Adjust timestamp for long time running.

        let ellapsed_time = timestamp - self.last_timestamp;

        if ellapsed_time > 1000.0 {
            self.start_time += ellapsed_time - 1000.0;

            timestamp = self.last_timestamp + 1000.0;
        }

        // Collect the required drawing actions.

        let target_samples = (self.sample_frequency * timestamp) as _;

        for _ in self.sampled..target_samples {
            universe.advance(self.step_size);

            for (position_history, position) in self
                .position_histories
                .iter_mut()
                .zip(universe.get_bodies().iter().map(|b| b.position))
            {
                if (position - position_history.last().unwrap()).magnitude2() >= 1.0 {
                    position_history.push(position);
                }
            }
        }

        // Do actual drawings.

        renderer.render(&self.position_histories);

        // Discard old histories.

        for position_history in &mut self.position_histories {
            let last_index = position_history.len() - 1;

            position_history.swap(0, last_index);
            position_history.truncate(1);
        }

        self.sampled = target_samples;
        self.last_timestamp = timestamp;
    }
}
