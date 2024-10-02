use std::fmt;
use statrs::distribution::{ChiSquared, ContinuousCDF};
type Series = Vec<f64>;
type Occurrences = Vec<Row>;

fn persean_criterion(a: f64, intervals: usize, unknown_parameters: usize) -> Result<f64, Box<dyn std::error::Error>> {
    let freedom = intervals - unknown_parameters - 1;
    let chi_squared = ChiSquared::new(freedom as f64)?;
    Ok(chi_squared.inverse_cdf(1.0 - a))
}

pub struct Row {
    pub x: f64,
    pub occurrences: usize,
}

impl Row {
    fn new(x: f64, occurrences: usize) -> Self {
        Self { x, occurrences }
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.occurrences)
    }
}

pub struct Histogram {
    pub occurrences: Occurrences,
    pub step: f64,
    pub sample_size: usize,
}

impl Histogram {
    // Количество групп по формуле Стерджесса 1 + |_log2(n)_|
    fn calculate_number_of_groups(number_of_measurements: usize) -> usize {
        1 + (number_of_measurements as f64).log2().floor() as usize
    }

    fn calculate_histogram_group_number_and_step(series: &Series) -> (usize, f64) {
        let amount_of_groups = Self::calculate_number_of_groups(series.len());
        let length_of_range = unsafe { series.last().unwrap_unchecked() - series.first().unwrap() };
        // I believe that nobody pass empty vector as argument
        let histogram_step = length_of_range / amount_of_groups as f64;
        (amount_of_groups, histogram_step)
    }

    /// All checks must be performed on caller side
    pub unsafe fn from_sorted_series(series: &Series) -> Self {
        let (amount_of_groups, histogram_step) =
            Self::calculate_histogram_group_number_and_step(series);
        let mut occurrences = Vec::with_capacity(amount_of_groups);
        let start_of_histogram = *series.first().unwrap_unchecked();
        let mut range_start = start_of_histogram + histogram_step;
        let mut occurrences_in_range: usize = 0;
        for value in series.iter() {
            if *value < range_start {
                occurrences_in_range += 1;
            } else {
                occurrences.push(Row::new(
                    range_start - histogram_step / 2.0,
                    occurrences_in_range,
                ));
                range_start += histogram_step;
                occurrences_in_range = 1;
            }
            // println!("{}",range_start);
        }
        // Since float is awful for comparing additional check required
        if occurrences.len() < occurrences.capacity() {
            occurrences.push(Row::new(
                range_start - histogram_step / 2.0,
                occurrences_in_range,
            ));
        } else {
            // Add 1 occurrency to last element because of [xk...xn] on the last range
            // Equality may fail sometimes
            // Non-zero array required
            occurrences.last_mut().unwrap_unchecked().occurrences += 1;
        }

        Histogram {
            occurrences,
            step: histogram_step,
            sample_size: series.len(),
        }
    }

    #[allow(dead_code)]
    pub unsafe fn from_sorted_series_with_given_groups(series: &Series, amount_of_groups: usize) -> Self {
        let length_of_range = unsafe { series.last().unwrap_unchecked() - series.first().unwrap() };
        // I believe that nobody pass empty vector as argument
        let histogram_step = length_of_range / amount_of_groups as f64;
        let mut occurrences = Vec::with_capacity(amount_of_groups);
        let start_of_histogram = *series.first().unwrap_unchecked();
        let mut range_start = start_of_histogram + histogram_step;
        let mut occurrences_in_range: usize = 0;
        for value in series.iter() {
            if *value < range_start {
                occurrences_in_range += 1;
            } else {
                occurrences.push(Row::new(
                    range_start - histogram_step / 2.0,
                    occurrences_in_range,
                ));
                range_start += histogram_step;
                occurrences_in_range = 1;
            }
            // println!("{}",range_start);
        }
        // Since float is awful for comparing additional check required
        if occurrences.len() < occurrences.capacity() {
            occurrences.push(Row::new(
                range_start - histogram_step / 2.0,
                occurrences_in_range,
            ));
        } else {
            // Add 1 occurrency to last element because of [xk...xn] on the last range
            // Equality may fail sometimes
            // Non-zero array required
            occurrences.last_mut().unwrap_unchecked().occurrences += 1;
        }

        Histogram {
            occurrences,
            step: histogram_step,
            sample_size: series.len(),
        }
    }
}

pub mod sample_analysys {
    use statrs::distribution::ContinuousCDF;

    use super::persean_criterion;
    use super::Histogram;
    use super::Series;

    impl Histogram {
        fn calculate_expected_value(&self) -> f64 {
            self.occurrences
                .iter()
                .fold(0.0, |acc, x| acc + x.x * x.occurrences as f64)
                / (self.sample_size as f64)
        }

        fn calculate_expected_value_and_variance(&self) -> (f64, f64) {
            let expected_value = self.calculate_expected_value();
            let variance = self
                .occurrences
                .iter()
                .fold(0.0, |acc, x| acc + x.x * x.x * x.occurrences as f64)
                / (self.sample_size as f64)
                - expected_value * expected_value;
            (expected_value, variance)
        }

        pub fn calculate_expected_value_variance_and_unbiased_variance(&self) -> (f64, f64, f64) {
            let (expected_value, variance) = self.calculate_expected_value_and_variance();
            let unbiased_variance =
                variance * (self.sample_size as f64) / (self.sample_size - 1) as f64;
            (expected_value, variance, unbiased_variance)
        }

        pub fn return_persean_test(&self, distribution: impl ContinuousCDF<f64, f64>, a:f64 ,unknown_parameters: usize) -> (f64,f64) {
            let critical_value = self
                .occurrences
                .iter()
                .fold(0.0, |acc, x| {
                    let theoretical_probability = distribution.cdf(x.x);
                    let frequency = x.occurrences as f64 / self.sample_size as f64;
                    acc + (frequency - theoretical_probability).powi(2)/theoretical_probability
                });
                println!("{},{},{}",a, self.occurrences.len(), unknown_parameters);
            (critical_value, persean_criterion(a, self.occurrences.len(), unknown_parameters).unwrap())
        }

        pub fn use_persean_test(values: (f64, f64)) -> bool {
            values.0 < values.1
        }

    }

    #[allow(dead_code)]
    fn calculate_expected_value(series: &Series) -> f64 {
        series.iter().sum::<f64>() / (series.len() as f64)
    }

    #[allow(dead_code)]
    fn calculate_expected_value_and_variance(series: &Series) -> (f64, f64) {
        let expected_value = calculate_expected_value(series);
        let variance = series.iter().map(|x| x * x).sum::<f64>() / (series.len() as f64)
            - expected_value * expected_value;
        (expected_value, variance)
    }

    #[allow(dead_code)]
    fn calculate_expected_value_variance_and_unbiased_variance(series: &Series) -> (f64, f64, f64) {
        let (expected_value, variance) = calculate_expected_value_and_variance(series);
        let unbiased_variance = variance * (series.len() as f64) / (series.len() - 1) as f64;
        (expected_value, variance, unbiased_variance)
    }
}
