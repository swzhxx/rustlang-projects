use bevy::render::view::ViewSet;
use ndarray::{s, Array1, Array3};
use num_traits::Num;

pub fn sample<'a>(qf: &'a Array3<f32>, u: usize, v: usize) -> Vec<f32> {
    let shape = qf.shape();
    let u = shape[0].max(0).min(u - 1);
    let v = shape[1].max(0).min(v - 1);
    let binding = qf.slice(s![shape[0], shape[1], ..]);
    let res = binding.as_slice().unwrap();
    res.to_owned()
}

pub fn lerp(vl: &[f32], vr: &[f32], frac: f32) -> Vec<f32> {
    let vl = Array1::from(vl.to_owned());
    let vr = Array1::from(vl.to_owned());
    let result = &vl + frac * (&vr - &vl);
    result.to_vec()
}

pub fn bilerp(matrix: &Array3<f32>, u: f32, v: f32) -> Vec<f32> {
    let s = u - 0.5;
    let t = v - 0.5;
    let iu = s as usize;
    let iv = t as usize;
    let fu = s - iu as f32;
    let fv = t - iv as f32;
    let a = sample(matrix, iu, iv);
    let b = sample(matrix, iu + 1, iv);
    let c = sample(matrix, iu, iv + 1);
    let d = sample(matrix, iu + 1, iv + 1);
    lerp(&lerp(&a, &b, fu), &lerp(&c, &d, fu), fv)
}

pub fn advect(vf: &Array3<f32>, qf: &Array3<f32>, new_qf: &mut Array3<f32>) {
    let shape = vf.shape();
    for i in 0..shape[0] {
        for j in 0..shape[1] {
            let _i = i as f32 + 0.5;
            let _j = j as f32 + 0.5;
            let res = bilerp(qf, _i, _j);
            new_qf[(i, j, 0)] = res[0];
            new_qf[(i, j, 1)] = res[1];
            new_qf[(i, j, 2)] = res[2];
        }
    }
}

#[cfg(test)]
mod test {

    use ndarray::{array, s};

    use super::bilerp;

    #[test]
    fn test_bilerp() {
        let a = array![[[1., 1.]], [[1., 1.]], [[1., 1.]]];
        let b = &a;
        let c = b.slice(s![1, 1, ..]);
        let c = c.to_owned();
        3. + &c;
        &c + 3.;
    }
}
