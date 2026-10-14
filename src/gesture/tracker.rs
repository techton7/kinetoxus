//! Host-side pointer sample ring buffer and deterministic release velocity estimation.

use std::time::Duration;
use kinetocore::physics::KinematicSampleBuffer;

/// Weighting policy used for multi-sample velocity estimation across historical lookback window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VelocityWeighting {
    /// Recency-weighted moving window weighting recent displacement segments higher.
    #[default]
    RecencyWeighted,
    /// Uniform window evaluation ($v = \Delta x / \Delta t$ across full window).
    Uniform,
}

/// Configuration parameters for pointer gesture velocity tracking.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TrackerConfig {
    /// Historical lookback window duration for sample evaluation (default 100ms).
    pub lookback: Duration,
    /// Inactivity threshold beyond which samples are considered stale (default 100ms).
    pub stale_threshold: Duration,
    /// Minimum movement displacement below which velocity is filtered to 0.0 (default 2.0px).
    pub min_distance: f64,
    /// Weighting policy applied across historical samples.
    pub weighting: VelocityWeighting,
}

impl Default for TrackerConfig {
    fn default() -> Self {
        Self {
            lookback: Duration::from_millis(100),
            stale_threshold: Duration::from_millis(100),
            min_distance: 2.0,
            weighting: VelocityWeighting::RecencyWeighted,
        }
    }
}

impl TrackerConfig {
    /// Creates a new `TrackerConfig` with explicit parameters.
    pub fn new(
        lookback: Duration,
        stale_threshold: Duration,
        min_distance: f64,
        weighting: VelocityWeighting,
    ) -> Self {
        Self {
            lookback,
            stale_threshold,
            min_distance,
            weighting,
        }
    }

    /// Builder method to update the lookback window duration.
    pub fn with_lookback(mut self, lookback: Duration) -> Self {
        self.lookback = lookback;
        self
    }

    /// Builder method to update the stale threshold duration.
    pub fn with_stale_threshold(mut self, stale_threshold: Duration) -> Self {
        self.stale_threshold = stale_threshold;
        self
    }

    /// Builder method to update the minimum distance noise gate.
    pub fn with_min_distance(mut self, min_distance: f64) -> Self {
        self.min_distance = min_distance;
        self
    }

    /// Builder method to update the velocity weighting model.
    pub fn with_weighting(mut self, weighting: VelocityWeighting) -> Self {
        self.weighting = weighting;
        self
    }
}

/// Host-side pointer sample ring buffer and deterministic velocity estimator.
///
/// Wraps a fixed-capacity [`KinematicSampleBuffer`] with zero heap allocations,
/// implementing automatic stale-sample reset policies during ingestion and
/// recency-weighted release velocity estimation.
#[derive(Debug, Clone, PartialEq)]
pub struct GestureTracker<const N: usize = 16> {
    buffer: KinematicSampleBuffer<N>,
    config: TrackerConfig,
}

impl<const N: usize> Default for GestureTracker<N> {
    fn default() -> Self {
        Self::new(TrackerConfig::default())
    }
}

impl<const N: usize> GestureTracker<N> {
    /// Creates a new `GestureTracker` with the given configuration.
    pub fn new(config: TrackerConfig) -> Self {
        Self {
            buffer: KinematicSampleBuffer::new(),
            config,
        }
    }

    /// Ingests a new pointer coordinate sample `(pos, time)`.
    ///
    /// Applies the **stale-sample reset policy**:
    /// If the time delta preceding this sample exceeds `config.stale_threshold`,
    /// or if the timestamp moves backwards, all previous samples in the ring buffer
    /// are automatically cleared so stale velocity history cannot contaminate the new stroke.
    pub fn add_sample(&mut self, pos: f64, time: Duration) {
        if let Some((_, prev_time)) = self.buffer.newest()
            && (time < prev_time || (time - prev_time) > self.config.stale_threshold)
        {
            self.buffer.clear();
        }
        self.buffer.push(pos, time);
    }

    /// Clears and resets all samples in the buffer.
    pub fn reset(&mut self) {
        self.buffer.clear();
    }

    /// Returns the number of active samples stored in the buffer.
    #[inline]
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Returns `true` if the buffer contains no samples.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Returns the newest sample's position coordinate, if available.
    #[inline]
    pub fn latest_position(&self) -> Option<f64> {
        self.buffer.newest().map(|(pos, _)| pos)
    }

    /// Returns the newest sample's timestamp, if available.
    #[inline]
    pub fn latest_time(&self) -> Option<Duration> {
        self.buffer.newest().map(|(_, time)| time)
    }

    /// Access reference to the active tracker configuration.
    #[inline]
    pub fn config(&self) -> &TrackerConfig {
        &self.config
    }

    /// Access mutable reference to the active tracker configuration.
    #[inline]
    pub fn config_mut(&mut self) -> &mut TrackerConfig {
        &mut self.config
    }

    /// Estimate release velocity using the newest sample timestamp as query time.
    pub fn estimate_velocity(&self) -> f64 {
        match self.buffer.newest() {
            Some((_, latest_time)) => self.estimate_velocity_at(latest_time),
            None => 0.0,
        }
    }

    /// Estimate release velocity evaluated at explicit release timestamp `current_time`.
    pub fn estimate_velocity_at(&self, current_time: Duration) -> f64 {
        if self.buffer.len() < 2 {
            return 0.0;
        }

        let (latest_pos, latest_time) = match self.buffer.newest() {
            Some(s) => s,
            None => return 0.0,
        };

        // 1. Stale hold suppression: gap between latest sample and release timestamp
        if current_time > latest_time && (current_time - latest_time) > self.config.stale_threshold {
            return 0.0;
        }

        // 2. Stale hold suppression: gap between newest sample and preceding sample
        let (_, prev_time) = match self.buffer.sample_from_newest(1) {
            Some(s) => s,
            None => return 0.0,
        };
        if latest_time > prev_time && (latest_time - prev_time) > self.config.stale_threshold {
            return 0.0;
        }

        match self.config.weighting {
            VelocityWeighting::Uniform => self.buffer.estimate_velocity_at(
                current_time,
                self.config.lookback,
                self.config.stale_threshold,
                self.config.min_distance,
            ),
            VelocityWeighting::RecencyWeighted => {
                self.estimate_recency_weighted_velocity(latest_pos, latest_time)
            }
        }
    }

    fn estimate_recency_weighted_velocity(&self, latest_pos: f64, latest_time: Duration) -> f64 {
        let count = self.buffer.len();
        if count < 2 {
            return 0.0;
        }

        let window_min = latest_time.saturating_sub(self.config.lookback);
        let lookback_secs = self.config.lookback.as_secs_f64().max(0.001);

        let mut weighted_v_sum = 0.0f64;
        let mut weight_sum = 0.0f64;
        let mut baseline_pos = latest_pos;

        // Iterate consecutive pairs of samples from newest backwards
        for i in 0..(count - 1) {
            let newer = match self.buffer.sample_from_newest(i) {
                Some(s) => s,
                None => break,
            };
            let older = match self.buffer.sample_from_newest(i + 1) {
                Some(s) => s,
                None => break,
            };

            baseline_pos = older.0;

            let dt = (newer.1.saturating_sub(older.1)).as_secs_f64();
            if dt > 0.0 {
                let v_segment = (newer.0 - older.0) / dt;
                let mid_time = older.1 + (newer.1 - older.1) / 2;
                let age = (latest_time.saturating_sub(mid_time)).as_secs_f64();
                let fraction = (age / lookback_secs).clamp(0.0, 1.0);
                // Weight linearly from 1.0 down to 0.1 at window edge
                let weight = (1.0 - 0.9 * fraction).max(0.1);

                weighted_v_sum += weight * v_segment;
                weight_sum += weight;
            }

            if older.1 <= window_min {
                break;
            }
        }

        // Noise gate: total displacement check
        let total_disp = (latest_pos - baseline_pos).abs();
        if total_disp < self.config.min_distance || weight_sum <= 0.0 {
            return 0.0;
        }

        weighted_v_sum / weight_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracker_basic_push_and_velocity() {
        let mut tracker = GestureTracker::<16>::default();
        tracker.add_sample(0.0, Duration::from_millis(0));
        tracker.add_sample(10.0, Duration::from_millis(50));
        tracker.add_sample(20.0, Duration::from_millis(100));

        assert_eq!(tracker.len(), 3);
        assert_eq!(tracker.latest_position(), Some(20.0));
        assert_eq!(tracker.latest_time(), Some(Duration::from_millis(100)));

        let vel = tracker.estimate_velocity();
        assert!((vel - 200.0).abs() < 1e-3, "Expected 200.0 px/s, got {vel}");
    }

    #[test]
    fn test_tracker_stale_sample_reset_policy() {
        let mut tracker = GestureTracker::<16>::new(
            TrackerConfig::default().with_stale_threshold(Duration::from_millis(100)),
        );

        // First stroke
        tracker.add_sample(0.0, Duration::from_millis(0));
        tracker.add_sample(50.0, Duration::from_millis(50));
        assert_eq!(tracker.len(), 2);

        // Gap of 200ms (> stale_threshold 100ms)
        tracker.add_sample(60.0, Duration::from_millis(250));
        assert_eq!(tracker.len(), 1, "Stale gap must clear prior samples");
        assert_eq!(tracker.latest_position(), Some(60.0));

        // Add second sample of new stroke
        tracker.add_sample(80.0, Duration::from_millis(300));
        assert_eq!(tracker.len(), 2);
        let vel = tracker.estimate_velocity();
        // New stroke velocity: (80 - 60) / 0.05 = 400.0 px/s
        assert!((vel - 400.0).abs() < 1e-3, "Expected 400.0 px/s, got {vel}");
    }

    #[test]
    fn test_tracker_stale_hold_suppression() {
        let mut tracker = GestureTracker::<16>::new(
            TrackerConfig::default().with_stale_threshold(Duration::from_millis(100)),
        );

        tracker.add_sample(0.0, Duration::from_millis(0));
        tracker.add_sample(100.0, Duration::from_millis(50));

        // Release evaluated 150ms later (> stale_threshold 100ms)
        let vel = tracker.estimate_velocity_at(Duration::from_millis(200));
        assert_eq!(vel, 0.0, "Stale release must suppress velocity to 0.0");
    }

    #[test]
    fn test_tracker_noise_gate() {
        let mut tracker = GestureTracker::<16>::new(
            TrackerConfig::default().with_min_distance(5.0),
        );

        tracker.add_sample(0.0, Duration::from_millis(0));
        tracker.add_sample(2.0, Duration::from_millis(50));

        let vel = tracker.estimate_velocity();
        assert_eq!(vel, 0.0, "Displacement 2.0 < min_distance 5.0 must return 0.0");
    }

    #[test]
    fn test_tracker_recency_weighting() {
        let mut tracker = GestureTracker::<16>::new(
            TrackerConfig::default()
                .with_lookback(Duration::from_millis(100))
                .with_weighting(VelocityWeighting::RecencyWeighted),
        );

        // Slow start (0 -> 10 in 50ms = 200 px/s), fast end (10 -> 40 in 50ms = 600 px/s)
        tracker.add_sample(0.0, Duration::from_millis(0));
        tracker.add_sample(10.0, Duration::from_millis(50));
        tracker.add_sample(40.0, Duration::from_millis(100));

        let vel = tracker.estimate_velocity();
        // Since recent segment (600 px/s) has higher weight than early segment (200 px/s),
        // recency-weighted average must be strictly higher than uniform average (400 px/s).
        assert!(vel > 400.0, "Recency weighted velocity {vel} should be > 400.0");
    }
}
