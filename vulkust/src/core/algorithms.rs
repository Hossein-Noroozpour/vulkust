use std::cmp::Ordering;

pub(crate) fn merge_all_sorted<T, F>(vectors: &[&[T]], mut compare: F) -> Vec<T>
where
    F: FnMut(&T, &T) -> Ordering,
    T: Clone,
{
    let vectors_count = vectors.len();
    let mut result_len = 0;
    let mut max_len = 0;
    for v in vectors {
        result_len += v.len();
        if v.len() > max_len {
            max_len = v.len();
        }
    }
    result_len += 1;
    max_len += 1;
    let mut temp_vecs = [Vec::with_capacity(max_len), Vec::with_capacity(max_len)];
    let mut result = Vec::with_capacity(result_len);
    for e in vectors[0] {
        temp_vecs[0].push(e.clone());
    }
    for i in 1..vectors_count {
        let si = i & 1;
        let psi = (!si) & 1;
        let es = vectors[i];
        let mut pi = 0;
        for e in es {
            while pi < temp_vecs[psi].len() {
                if compare(&temp_vecs[psi][pi], e) == Ordering::Less {
                    let d = temp_vecs[psi][pi].clone();
                    temp_vecs[si].push(d);
                } else {
                    break;
                }
                pi += 1;
            }
            temp_vecs[si].push(e.clone());
        }
    }
    for k in &temp_vecs[(!vectors_count) & 1] {
        result.push(k.clone());
    }
    return result;
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn check() {
//         let mut
//         check_power_of_two(3 * 1024);
//     }
// }

// s = 5.172 / 6.87
