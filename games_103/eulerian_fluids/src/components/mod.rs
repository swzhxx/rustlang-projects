use std::ops::{Deref, DerefMut};

use bevy::prelude::Component;
use num_traits::Num;

use crate::field::CField;

#[derive(Debug, Component)]
pub struct VelocityField<T: Num>(CField<T>);

#[derive(Debug, Component)]
pub struct PressureField<T: Num>(CField<T>);

#[derive(Debug, Component)]
pub struct ColorField<T: Num>(CField<T>);

// impl<T> Deref for VelocityField<T>
// where
//     T: Num,
// {
//     type Target = CField<T>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl<T> DerefMut for VelocityField<T>
// where
//     T: Num,
// {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

macro_rules! ref_impl_field {
    ($name:ident) => {
        impl<T> Deref for $name<T>
        where
            T: Num,
        {
            type Target = CField<T>;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

macro_rules! ref_mut_impl_field {
    ($name:ident) => {
        impl<T> DerefMut for $name<T>
        where
            T: Num,
        {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}
ref_impl_field!(VelocityField);
ref_mut_impl_field!(VelocityField);

ref_impl_field!(PressureField);
ref_mut_impl_field!(PressureField);

ref_impl_field!(ColorField);
ref_mut_impl_field!(ColorField);
