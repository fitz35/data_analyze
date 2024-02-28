
pub mod stats_serie;

/// get the outliers of the given data (Mask)
/// return the vector of bool, true if the corresponding data is an outlier
pub fn get_outliers(
    data_to_filter: &Vec<f32>,
) -> Vec<bool>
{

    let q1_q3 = calculate_q1_q3(data_to_filter);
    let (lower_bound, upper_bound) = calculate_bounds(&q1_q3);

    let mut outliers = Vec::new();

    for (_, d) in data_to_filter.iter().enumerate() {
        outliers.push(*d < lower_bound || *d > upper_bound);
    }

    outliers.into()
}

/// use the inverted_cdf method to get the q1 and q3
pub fn calculate_q1_q3(data: &Vec<f32>) -> (f32, f32) {
    if data.len() == 0 {
        return (f32::NAN, f32::NAN);
    }

    // Sort the data first
    let mut sorted_data = data.clone();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let freq_q1 = (data.len() as f32) / 4.0;
    let freq_q3 = (data.len() as f32) * 3.0 / 4.0;

    let q1_index = freq_q1.ceil() as usize - 1;

        
    let q3_index =  freq_q3.ceil() as usize - 1;

    
    let q1 = sorted_data[q1_index];
    let q3 = sorted_data[q3_index];

    (q1, q3)
}


pub fn calculate_bounds(q1_q3: &(f32, f32)) -> (f32, f32)
{
    // Calculate the lower and upper bounds for outliers
    let iqr = q1_q3.1.clone() - q1_q3.0.clone();
    let lower_bound = q1_q3.0.clone() - (1.5 * iqr);
    let upper_bound = q1_q3.1.clone() + (1.5 * iqr);

    (lower_bound, upper_bound)
}