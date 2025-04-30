///Author: Christian Clemente de Frutos - Date: 04/28/2025
///Calculates the arithmetic mean of a slice of `f64` values.
///
///# Arguments
///
///* `data` - A slice of floating-point numbers
///
///# Returns
///
///* `Some(mean)` if the slice is not empty
///* `None` if the slice is empty
///
///# Exampes
///
///```
///use stats_lib::mean;
///
///let data = [1.0, 2.0, 3.0];
///assert_eq!(mean(&data), Some(2.0));
///```



pub fn mean(data: &[f64]) -> Option<f64> {
    if data.is_empty() {
        return None;
    }
    let sum: f64 = data.iter().sum();
    Some(sum / data.len() as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean_with_non_empty_slice(){
        let data = [1.0, 2.0, 3.0];
        assert_eq!(mean(&data), Some(2.0));
    }

    #[test]
    fn test_mean_with_empty_slice(){
        let data: [f64; 0] = [];
        assert_eq!(mean(&data), None);
    }
}
