///Author: Christian Clemente de Frutos - Date: 04/28/2025
///Calculates the arithmetic median of a slice of `f64` values.
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
///use inferrous::median;
///
///let data1 = [1.0, 3.0, 2.0];
///assert_eq!(median(&data1), Ok(2.0));
///let data2 = [1.0, 3.0, 4.0, 2.0];
///assert_eq!(median(&data2), Ok(2.5));
///```



pub fn median(data: &[f64]) -> Result<f64, &'static str> {
    if data.is_empty() {
        return Err("Empty input");
    }
    if data.iter().any(|x| x.is_nan()){
        return Err("Input contains NaN")
    }    
    let mut sorted = data.to_vec(); //copy to not modify original

    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap()); //orders - TODO: NaN treatment

    let mid = sorted.len() / 2;

    if sorted.len() % 2 == 0 {
        //even
        Ok((sorted[mid - 1] + sorted[mid]) / 2.0)
    } else {
        //odd
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
        assert_eq!(median(&data), Err("Empty input"));
    }

    #[test]
    fn test_median_with_nan(){
        let data = [1.0, f64::NAN, 2.0, 3.0];
        assert_eq!(median(&data), Err("Input contains NaN"));
    }


}
