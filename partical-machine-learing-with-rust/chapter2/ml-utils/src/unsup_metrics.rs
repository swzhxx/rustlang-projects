pub fn rand_index(clusters1: &[HashSet<u64>], clusters2: &[HashSet<u64>]) -> f64 {
    todo!()
}
fn count_pairwise_cooccurence(
    clusters1: &[HashSet<u64>],
    clusters2: &[HashSet<u64>],
) -> (f64, f64, f64, f64) {
    let cont_tbl = contingency_table(&clusters1, &clusters2);
    // println!("{:?}", cont_tbl);

    let square_matrix = cont_tbl.mapv(|a| a.pow(2));
    // println!("{:?}", square_matrix);
    let sum_of_squares1 = square_matrix.into_raw_vec();
    let sum_of_squares: u64 = sum_of_squares1.iter().sum();
    // println!("{:?}", sum_of_squares);
    let c1_sum_sq_sizes = cluster_size_sequence_sqsum(clusters1);
    let c2_sum_sq_sizes = cluster_size_sequence_sqsum(clusters2);
    // println!("{:?}", c1_sum_sq_sizes);

    let c1_elements_count = elements_in_vectr(clusters1);
    let n11 = 0.5 * (sum_of_squares - c1_elements_count) as f64;
    // println!("{:?}", n11);
    let n10 = 0.5 * (c1_sum_sq_sizes - sum_of_squares) as f64;
    let n01 = 0.5 * (c2_sum_sq_sizes - sum_of_squares) as f64;
    let n00 = 0.5 * c1_elements_count as f64 * (c1_elements_count - 1) as f64 - n11 - n10 - n01;
    (n11, n10, n01, n00)
}
fn matching_elems_count(s1: &HashSet<u64>, s2: &HashSet<u64>) -> u64 {
    let common: Vec<_> = s1.intersection(s2).collect();
    common.len() as u64
}

fn contingency_table(
    clusters1: &[HashSet<u64>],
    clusters2: &[HashSet<u64>],
) -> ArrayBase<OwnedRepr<u64>, Dim<[usize; 2]>> {
    let length = clusters1.len();
    assert!(length == clusters2.len());
    let product = iproduct!(clusters1, clusters2);
    let cont_table_vec: Vec<u64> = product
        .map(|(c1, c2)| matching_elems_count(c1, c2))
        .collect();
    // println!("{:?}", cont_table_vec);
    let cont_table_mat = Array::from_shape_vec((3, 3), cont_table_vec).unwrap();
    cont_table_mat
    // let v_chunked: Vec<Vec<f64>> = cont_table_vec.chunks(length).map(|x| x.to_vec()).collect();
    // v_chunked
}

fn cluster_size_sequence_sqsum(clusters: &[HashSet<u64>]) -> u64 {
    let cluster1_size_seq: Vec<u64> = clusters.iter().map(|v| v.len() as u64).collect();
    let squares = cluster1_size_seq.iter().map(|num| num.pow(2));
    squares.sum()
}

fn elements_in_vectr(vectr: &[HashSet<u64>]) -> u64 {
    let flatten_array: Vec<u64> = vectr
        .iter()
        .flat_map(|array| array.iter())
        .cloned()
        .collect();
    flatten_array.len() as u64
}
