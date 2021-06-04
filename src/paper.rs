// return x coordinates of n points in [0,l] that are equidistant in curve length.
pub fn project(fp: impl Fn(f64) -> f64, l: f64, n: usize, epsilon: f64) -> Vec<f64> {
    let curve_length = |x| (fp(x).powi(2) + 1.0).sqrt();
    let tot_l = integrate(&curve_length, 0.0, l, epsilon);
    // println!("tot_l: {}", tot_l);
    let delta = tot_l / (n as f64);

    let mut points = vec![0.0; n];
    points[0] = next_point(&curve_length, 0.0, delta/2.0, epsilon);
    for i in 1..n {
        points[i] = next_point(&curve_length, points[i-1], delta, epsilon);
    }
    assert!(points[n-1] < l);

    points
}

fn next_point(l: &impl Fn(f64) -> f64, start: f64, step: f64, epsilon: f64) -> f64 {
    // binary search for where it could be
    // we seek x s.t. integral(start, x) == step, or at least decently close to that
    let mut hi = start + step;
    let mut lo = start;
    while hi - lo > epsilon {
        // println!("hi: {}, lo: {}", hi, lo);
        let mid = (hi + lo) / 2.0;
        let v = integrate(l, start, mid, epsilon);
        if v < step {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    (hi + lo) / 2.0
}

// integrate f on [a,b], using delta as the step size.
fn integrate(f: &impl Fn(f64) -> f64, a: f64, b: f64, delta: f64) -> f64 {
    let mut riemann_sum = 0.0;
    let mut x = a;
    while x + delta < b {
        riemann_sum += f(x) * delta;
        x += delta;
    }
    riemann_sum += f(x) * (b - x);
    return riemann_sum;
}
