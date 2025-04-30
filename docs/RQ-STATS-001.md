# REQ-STATS-001: ARITHMETIC MEAN

Function name: mean
Input: 'data' slice &[f64]
Output: Option<f64>
Description:
    Function calculates and returns arithmetic mean of given data.
        If the slice is empty, it will return None.
        If the slice contains one or more elements, it will return mean as Some(value)
