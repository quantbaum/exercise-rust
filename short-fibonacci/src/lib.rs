/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    vec![]
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    // unimplemented!("create a zeroized buffer of {} bytes", count)
    vec![0; count]
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let mut vector = create_buffer(5);
    vector[0] = 1;
    vector[1] = 1;
    for idx in 2..vector.len(){
        vector[idx] = vector[idx-1] + vector[idx-2];
    }
    vector
}
