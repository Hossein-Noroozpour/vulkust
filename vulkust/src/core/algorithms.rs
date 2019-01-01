use std::cmp::Ordering;

pub(crate) fn merge_all_sorted<T, F>(vectors: &[&[T]], compare: &mut F) -> Vec<T>
where
    F: FnMut(&T, &T) -> Ordering,
    T: Clone,
{
    if vectors.len() == 1 {
        return vectors[0].to_vec();
    } else if vectors.len() > 1 {
        let l = vectors.len() >> 1;
        let v1 = merge_all_sorted(&vectors[..l], compare);
        let v2 = merge_all_sorted(&vectors[l..], compare);
        let v1l = v1.len();
        let v2l = v2.len();
        let mut result = Vec::with_capacity(v1l + v2l + 1);
        let mut v1i = 0;
        let mut v2i = 0;
        loop {
            if v1i < v1l && v2i < v2l {
                if compare(&v1[v1i], &v2[v2i]) == Ordering::Less {
                    result.push(v1[v1i].clone());
                    v1i += 1;
                } else {
                    result.push(v2[v2i].clone());
                    v2i += 1;
                }
            } else if v1i < v1l {
                for i in v1i..v1l {
                    result.push(v1[i].clone());
                }
                break;
            } else if v2i < v2l {
                for i in v2i..v2l {
                    result.push(v2[i].clone());
                }
                break;
            } else {
                break;
            }
        }
        return result;
    } else {
        vxunexpected!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn merge_all_sorted_test() {
        use rand::distributions::Distribution;
        let between1 = rand::distributions::Uniform::from(30..80);
        let between2 = rand::distributions::Uniform::from(-5f32..5f32);
        let mut rng = rand::thread_rng();
        let mut final_cmp = Vec::new();
        let mut v = Vec::new();
        for _ in 0..100 {
            let mut c = Vec::new();
            for _ in 0..between1.sample(&mut rng) {
                let f = between2.sample(&mut rng);
                final_cmp.push(f);
                c.push(f);
            }
            c.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            v.push(c);
        }
        let mut vref: Vec<&[f32]> = Vec::new();
        for e in &v {
            vref.push(e);
        }
        let mut ordfun = |a: &f32, b: &f32| a.partial_cmp(b).unwrap();
        let v1 = merge_all_sorted(&vref, &mut ordfun);
        final_cmp.sort_unstable_by(ordfun);
        assert_eq!(final_cmp, v1);
    }
}
