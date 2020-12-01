fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i: usize| {
            v.iter()
                .map(|inner: &Vec<T>| inner[i].clone())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub fn init_vec(m: usize, n: usize) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let mut a: Vec<Vec<f64>> = vec![vec![0.0; n]; m];
    let mut b: Vec<Vec<f64>> = vec![vec![0.0; m]; n];

    for i in 1..m + 1 {
        for j in 1..n + 1 {
            a[i - 1][j - 1] = 12.2 * i as f64 - 3.8 * j as f64;
            b[j - 1][i - 1] = 65.1 + 3.3 * j as f64 - 20.2 * i as f64;
        }
    }
    b = transpose(b);
    return (a, b);
}
