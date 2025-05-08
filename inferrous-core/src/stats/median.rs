/// Author: Christian Clemente de Frutos - Date: 04/28/2025
/// Calculates the arithmetic median of a slice of `f64` values.
///
/// # Arguments
///
/// * `data` - A slice of floating-point numbers
///
/// # Returns
///
/// * `Ok(median)` if the input is valid
/// * `Err(MedianError::EmptyInput)` if the slice is empty
/// * `Err(MedianError::ContainsNaN)` if any value is NaN
///
/// # Examples
///
/// ```
/// use inferrous_core::median;
///
/// let data1 = [1.0, 3.0, 2.0];
/// assert_eq!(median(&data1), Ok(2.0));
///
/// let data2 = [1.0, 3.0, 4.0, 2.0];
/// assert_eq!(median(&data2), Ok(2.5));
/// ```


use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum MedianError {
    #[error("The input slice is empty")]
    EmptyInput,

    #[error("The input contains NaN")]
    ContainsNaN,
}    



pub fn median(data: &[f64]) -> Result<f64, MedianError> {
    if data.is_empty() {
        return Err(MedianError::EmptyInput);
    }
    if data.iter().any(|x| x.is_nan()){
        return Err(MedianError::ContainsNaN)
    }    
    let mut sorted = data.to_vec(); //Defensive copy

    sorted.sort_by(|a, b| a.partial_cmp(b).expect("NaN check already passed."));

    let mid = sorted.len() / 2;

    if sorted.len() % 2 == 0 {
        //Even number of elements: avrg the two middle ones
        Ok((sorted[mid - 1] + sorted[mid]) / 2.0)
    } else {
        //Odd number of elements: middle value
        Ok(sorted[mid])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_with_even_length(){
        let data = [1.0, 2.0, 3.0, 4.0];
        assert_eq!(median(&data), Ok(2.5));
    }

    #[test]
    fn test_median_with_odd_length(){
        let data = [1.0, 2.0, 3.0];
        assert_eq!(median(&data), Ok(2.0));
    }

    #[test]
    fn test_median_with_empty_slice(){
        let data: [f64; 0] = [];
        assert_eq!(median(&data), Err(MedianError::EmptyInput));
    }

    #[test]
    fn test_median_with_nan(){
        let data = [1.0, f64::NAN, 2.0, 3.0];
        assert_eq!(median(&data), Err(MedianError::ContainsNaN));
    }
    #[test]
    fn test_median_with_duplicates() {
        let data = [3.0, 1.0, 2.0, 2.0];
        assert_eq!(median(&data), Ok(2.0));
    }
    #[test]
    fn test_median_with_large_slice() {
        let data: Vec<f64> = (1..=1_001).map(|x| x as f64).collect();
        assert_eq!(median(&data), Ok(501.0));
    }    


}
