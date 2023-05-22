use std::ops::{Add, Div, Mul};

use bevy::render::view::ViewSet;
use ndarray::{s, Array1, Array3};
use num_traits::Num;

pub fn bilerp(matrix: &Array3<f32>, x: f32, y: f32) -> Array1<f32> {
    let shape = matrix.shape();
    let i = x as usize;
    let j = y as usize;

    let value_i_j = matrix.slice(s![i, j, ..]);
    let value_i_1_j = if i + 1 >= shape[0] {
        value_i_j
    } else {
        matrix.slice(s![i + 1, j, ..])
    };
    let value_i_j_1 = if j + 1 >= shape[1] {
        value_i_j
    } else {
        matrix.slice(s![i, j + 1, ..])
    };
    let value_i_1_j_1 = {
        let mut i_1 = if i + 1 > shape[0] { i } else { i + 1 };
        let mut j_1 = if j + 1 > shape[1] { j } else { j + 1 };
        matrix.slice(s![i_1, j_1, ..])
    };

    ((i + 1) as f32 - x) * ((j + 1) as f32 - y) * &value_i_j.to_owned()
        + (x - (i) as f32) * (y - j as f32) * &value_i_1_j.to_owned()
        + ((i + 1) as f32 - x) * (y - j as f32) * &value_i_j_1.to_owned()
        + (x - i as f32) * (y - j as f32) * &value_i_1_j_1.to_owned()
}

pub fn advect(vf: &Array3<f32>, qf: &Array3<f32>, new_qf: &mut Array3<f32>) {
  // vf.
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
