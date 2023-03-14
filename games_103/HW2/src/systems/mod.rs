use bevy::prelude::*;

use crate::components::{Damping, Mass, SpringK, ELV};

pub fn collision_handling() {}

const GRAVITY: Vec3 = Vec3::new(0., -9.8, 0.);
const rho: f32 = 0.995;

// 隐式积分更新
pub fn implict_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut cloths: Query<(
        Entity,
        &Mass,
        &mut ELV,
        &Handle<Mesh>,
        Option<&Damping>,
        &SpringK,
    )>,
    timer: Res<Time>,
) {
    let mut t = 0.033_f32;
    let get_gradient = |X: &Vec<Vec3>,
                        X_hat: &Vec<Vec3>,
                        G: &mut Vec<Vec3>,
                        E: &Vec<u32>,
                        L: &Vec<f32>,
                        mass: f32,
                        spring_k: f32| {
        for i in 0..G.len() {
            G[i] = 1. / t.sqrt() * mass * (X[i] - X_hat[i]) - mass * GRAVITY;
        }
        for e in 0..E.len() / 2 {
            let indexI = E[e * 2] as usize;
            let indexJ = E[e * 2 + 1] as usize;
            let xI = X[indexI];
            let xJ = X[indexJ];
            let spring = spring_k * (1. - L[e] / (xI - xJ).length()) * (xI - xJ);
            G[indexI] += spring;
            G[indexJ] -= spring;
        }
    };

    for (entity, mass, mut klv, handle_mesh, damping, spring_k) in cloths.iter_mut() {
        if let (mesh) = meshes.get_mut(handle_mesh).unwrap() {
            let mut G = vec![Vec3::default(); mesh.count_vertices()];
            let mut X: Vec<Vec3> = mesh
                .attribute(Mesh::ATTRIBUTE_POSITION)
                .unwrap()
                .as_float3()
                .unwrap()
                .iter()
                .map(|item| Vec3::new(item[0], item[1], item[2]))
                .collect();

            let mut last_X = vec![Vec3::default(); X.len()];
            let mut X_hat = vec![Vec3::default(); X.len()];

            for i in 0..klv.V.len() {
                klv.V[i] *= if damping.is_none() {
                    0.
                } else {
                    damping.unwrap().0
                };

                X_hat[i] = X[i] + t * klv.V[i];
                X[i] = X[i] + t * klv.V[i];
            }

            let mut w = 1.;
            for k in 0..32 {
                get_gradient(&X, &X_hat, &mut G, &klv.E, &klv.L, mass.0, spring_k.0);
                if k == 0 {
                    w = 1.;
                } else if k == 1 {
                    w = 2. / (2. - rho * rho)
                } else {
                    w = 4. / (4. - rho * rho * w);
                }

                for i in 1..X.len() {
                    if i == 20 {
                        continue;
                    }
                    let old_x = X[i].clone();
                    X[i] = w * (X[i] - 1. / (1. / t.sqrt() * mass.0 + 4. * spring_k.0) * G[i])
                        + (1. - w) * last_X[i];

                    last_X[i] = old_x;
                }
            }

            for i in 1..klv.V.len() {
                if i == 20 {
                    continue;
                }
                klv.V[i] += 1. / t * (X[i] - X_hat[i])
            }
            mesh.insert_attribute(
                Mesh::ATTRIBUTE_POSITION,
                X.iter()
                    .map(|item| [item.x, item.y, item.z])
                    .collect::<Vec<[f32; 3]>>(),
            );

            collision_handling()
        }
    }
}
