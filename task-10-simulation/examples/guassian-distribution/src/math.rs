pub mod random {
    use std::f64;

    /// Normal distribution with the [Box-Muller transform](https://en.wikipedia.org/wiki/Box%E2%80%93Muller_transform).
    /// 
    /// Generates a random normally distributed value with given mean and standard deviator.
    /// 
    /// ### Arguments
    /// - `mean`: Position of the center of the peak.
    /// - `variance`: The Guassian root mean square (the standard deviation).
    pub fn random_guassian(mean: f64, variance: f64) -> f64 {
        let (r, theta) = box_muller_deviation_coordinates(variance);

        // Get independent varaible with normal distribution as the system X coordinate.
        r * f64::cos(theta) + mean
    }

    /// Normal distribution with the [Box-Muller transform](https://en.wikipedia.org/wiki/Box%E2%80%93Muller_transform).
    /// 
    /// Get the independent noramally distributed values used in the deviation Cartesian system.
    /// 
    /// ### Arguments
    /// - `variance`: The Guassian root mean square (the standard deviation).
    fn box_muller_deviation_coordinates(variance: f64) -> (f64, f64) {
        const PI: f64 = f64::consts::PI;

        // Get unit samples of (0, 1).
        let u1: f64 = unit_sample();
        let u2 = unit_sample();

        // Get the norm of the bivariate normal variable.
        let r = variance * f64::sqrt(-2.0 * f64::log(u1, 2.0));
        let theta = 2.0 * PI * u2;

        (r, theta)
    }

    /// Calculate a uniformly distributed sample from the interval (0, 1).
    fn unit_sample() -> f64 {
        let mut sample = 0.0;
        while sample <= f64::EPSILON {
            sample = rand::random::<f64>();
        }
        sample
    }
}