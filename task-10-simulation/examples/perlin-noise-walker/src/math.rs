pub mod random {
    type PartialProfile = (f64, f64);

    /// Perlin noise value sets based on [posting by user9144436](https://stackoverflow.com/a/60772438).
    pub struct PerlinProfile {
        profile_1: PartialProfile,
        profile_e: PartialProfile,
        profile_pi: PartialProfile
    }

    impl PerlinProfile {
        const MAX_PROFILE_1: PartialProfile = (3.2, 1.3);
        const MAX_PROFILE_PI: PartialProfile = (1.9, 0.7);
        const MAX_PROFILE_E: PartialProfile = (1.2, 1.7);

        fn partial_profile(max_profile: PartialProfile) -> PartialProfile {
            let (max_amplitude, max_scale) = max_profile;
            (Self::sample_factor(max_amplitude), Self::sample_factor(max_scale))
        }

        fn sample_factor(max: f64) -> f64 {
            rand::random::<f64>() * max
        }
    }

    /// Create value sets for perlin noise functions.
    pub fn generate_profile() -> PerlinProfile {
        PerlinProfile {
            profile_1: PerlinProfile::partial_profile(PerlinProfile::MAX_PROFILE_1),
            profile_e: PerlinProfile::partial_profile(PerlinProfile::MAX_PROFILE_E),
            profile_pi: PerlinProfile::partial_profile(PerlinProfile::MAX_PROFILE_PI)
        }
    }

    /// Perlin noise using a function that is newer periodic.
    pub fn perlin_noise(x: f64, profile: &PerlinProfile) -> f64 {
        use std::f64::consts::{E, PI};

        let periodic_1 = partial_noise(x, &profile.profile_1, 1.0);
        let periodic_e = partial_noise(x, &profile.profile_e, E);
        let periodic_pi = partial_noise(x, &profile.profile_pi, PI);

        -periodic_1 + periodic_pi - periodic_e
    }

    fn partial_noise(x: f64, profile: &PartialProfile, factor: f64) -> f64 {
        let (amplitude, scale) = profile;
        amplitude * f64::sin(scale * factor * x)
    }
}