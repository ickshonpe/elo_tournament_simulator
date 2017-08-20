pub fn interleave_slices<T: Clone>(xs: &[T], ys: &[T]) -> Vec<T> {
    let mut out = Vec::with_capacity(xs.len() + ys.len());
    let (shorter, longer) =
        if xs.len() < ys.len() {
            (xs, ys)
        } else {
            (ys, xs)
        };
    for i in 0 .. shorter.len() {
        out.push(longer[i].clone());
        out.push(shorter[i].clone());
    }
    for element in &longer[shorter.len() .. ] {
        out.push(element.clone());
    }
    out
}