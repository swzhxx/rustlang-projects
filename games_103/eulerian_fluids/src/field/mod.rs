use ndarray::Array3;
use num_traits::Num;

pub trait Field<T>
where
    T: Num,
{
    fn swap(&mut self);
}

#[derive(Debug)]
pub struct CField<T>
where
    T: Num,
{
    pub cur: Array3<T>,
    pub next: Array3<T>,
}

impl<T> CField<T>
where
    T: Num,
{
    pub fn new(cur: Array3<T>, next: Array3<T>) -> Self {
        Self { cur, next }
    }
}

impl<T> Field<T> for CField<T>
where
    T: Num,
{
    fn swap(&mut self) {
        std::mem::swap(&mut self.cur, &mut self.next);
    }
}

// pub struct CField<T>
// where
//     T: Num,
// {
//     cur: *const Array3<T>,
//     next: *const Array3<T>,
// }

// impl<T> CField<T>
// where
//     T: Num,
// {
//     pub fn get_cur(&self) -> &Array3<T> {
//         unsafe { &(*self.cur) as &Array3<T> }
//     }

//     pub fn get_mut_cur(&self) -> &mut Array3<T> {
//         unsafe { &mut *(self.cur as *mut Array3<T>) as &mut Array3<T> }
//     }

//     pub fn new(cur: Array3<T>, next: Array3<T>) -> Self {
//         Self {
//             cur: &cur as *const Array3<T>,
//             next: &next as *const Array3<T>,
//         }
//     }
// }

// impl<T> Field<T> for CField<T>
// where
//     T: Num,
// {
//     fn swap(&mut self) {
//         std::mem::swap(&mut self.cur, &mut self.next);
//     }
// }

#[cfg(test)]
mod test {
    use ndarray::{array, Array3};

    use super::{CField, Field};

    //     #[test]
    //     fn test_field() {
    //         let a: Array3<f32> = array![[[1.]]];

    //         let b: Array3<f32> = array![[[0.]]];

    //         let c = 5f32;
    //         let raw_c = &c as *const f32;

    //         let mut f = CField {
    //             cur: &a as *const Array3<f32>,
    //             next: &b as *const Array3<f32>,
    //         };

    //         f.swap();
    //         println!("{:?}", f.cur);
    //         println!("{:?}", f.next);
    //         f.get_mut_cur()[[0, 0, 0]] = 5.;
    //         // f.get_mut_cur()[[0, 0, 0]] = 5.;
    //         println!("{:?}", f.get_cur().get((0, 0, 0)));
    //         advect(unsafe { &*f.cur }, unsafe {
    //             &mut *(f.next as *mut Array3<f32>)
    //         });
    //     }

    #[test]
    fn test() {
        let a: Array3<f32> = array![[[1.]]];
        let b: Array3<f32> = array![[[0.]]];

        let mut f = CField::new(a, b);

        advect(&f.cur, &mut f.next);
    }

    fn advect(a: &Array3<f32>, b: &mut Array3<f32>) {}
}
