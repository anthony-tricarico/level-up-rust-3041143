/* How to find the middle value given the length of a list?
To do that we can first sort the array.
Then we should retrieve the index of the middle value by dividing
by two and truncating to the integer part. */
fn median(a: Vec<f32>) -> Option<f32> {
    let mut b = a.clone();
    let len = b.len();
    if b.is_empty() {
        return None;
    }

    if !len.is_multiple_of(2) {
        let middle = len / 2;
        b.sort_by(|a, b| a.total_cmp(b));
        Some(b[middle])
    } else {
        let middle = len / 2;
        let before = middle - 1;
        b.sort_by(|a, b| a.total_cmp(b));
        let result = (b[middle] + b[before]) / 2.0;
        Some(result)
    }
}

fn main() {
    let answer = median(vec![1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}
