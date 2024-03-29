use bevy::prelude::*;
#[derive(Component, Reflect, Default)]
#[reflect]
pub struct Mass(pub f32);

#[derive(Component, Reflect, Default)]
#[reflect]
pub struct SpringK(pub f32);

#[derive(Component, Reflect, Default)]
#[reflect]
pub struct Rho(f32);

#[derive(Component, Reflect, Default)]
pub struct Damping(pub f32);

#[derive(Component)]
pub struct Cloth;

#[derive(Component)]
pub struct Force;

#[derive(Component)]
pub struct ELV {
    pub E: Vec<u32>,
    pub L: Vec<f32>,
    pub V: Vec<Vec3>,
}
impl ELV {
    pub fn init(triangles: &Vec<u32>, vertices: &Vec<Vec3>) -> ELV {
        let triangles_len = triangles.len();
        let mut _E = vec![0; triangles_len * 2];
        let mut i = 0;
        while (i < triangles_len) {
            _E[i * 2 + 0] = triangles[i + 0];
            _E[i * 2 + 1] = triangles[i + 1];
            _E[i * 2 + 2] = triangles[i + 1];
            _E[i * 2 + 3] = triangles[i + 2];
            _E[i * 2 + 4] = triangles[i + 2];
            _E[i * 2 + 5] = triangles[i + 0];
            i += 3;
        }
        // println!("_E before {:?}",_E);
        let mut i = 0;
        while i < _E.len() {
            if _E[i] > _E[i + 1] {
                _E.swap(i, i + 1);
            }
            i += 2;
        }
        // println!("_E len {:?}",_E.len());
        // println!("_E {:?}",_E);
        let len = _E.len();
        // println!("sort : len / 2 - 1 {:?}", len / 2 - 1);
        ELV::sort(&mut _E, 0, (len / 2) as i32 - 1);

        // 计算边数量
        let mut e_number = 0;
        let mut i = 0;
        while i < _E.len() {
            if i == 0 || _E[i + 0] != _E[i - 2] || _E[i + 1] != _E[i - 1] {
                e_number += 1;
            }
            i += 2;
        }

        // 构建边集
        let mut E = vec![0; e_number * 2];
        let mut i = 0;
        let mut e = 0;
        while i < _E.len() {
            if i == 0 || _E[i + 0] != _E[i - 2] || _E[i + 1] != _E[i - 1] {
                E[e * 2 + 0] = _E[i + 0];
                E[e * 2 + 1] = _E[i + 1];
                e += 1;
            }
            i += 2;
        }
        // println!("{:?}", E);
        // 构造边的初始长度
        let mut L = vec![0.; E.len() / 2];
        for i in 0..E.len() / 2 {
            let v0 = E[(i * 2 + 0) as usize];
            let v1 = E[(i * 2 + 1) as usize];
            let _a = vertices[v0 as usize] - vertices[v1 as usize];
            L[i] = _a.length();
        }

        // 构造初始速度
        let V = vec![Vec3::default(); vertices.len()];
        ELV { E, L, V }
    }
    fn sort(a: &mut Vec<u32>, l: i32, r: i32) {
        let mut j: i32 = 0;
        if l < r {
            j = ELV::quick_sort_partition(a, l, r);
            // println!("j {}", j);
            ELV::sort(a, l, j - 1);
            ELV::sort(a, j + 1, r);
        }
    }
    fn quick_sort_partition(a: &mut Vec<u32>, l: i32, r: i32) -> i32 {
        let pivot_0 = a[(l * 2 + 0) as usize];
        let pivot_1 = a[(l * 2 + 1) as usize];
        let mut i = l;
        let mut j = r + 1;
        loop {
            i += 1;
            while i <= r
                && (a[(i * 2) as usize] < pivot_0
                    || a[(i * 2) as usize] == pivot_0 && a[(i * 2 + 1) as usize] <= pivot_1)
            {
                i += 1;
            }
            j -= 1;
            while a[(j * 2) as usize] > pivot_0
                || a[(j * 2) as usize] == pivot_0 && a[(j * 2 + 1) as usize] > pivot_1
            {
                j -= 1;
            }
            if i >= j {
                break;
            }
            a.swap((i * 2) as usize, (j * 2) as usize);
            a.swap((i * 2 + 1) as usize, (j * 2 + 1) as usize)
        }
        a.swap((l * 2) as usize, (j * 2) as usize);
        a.swap((l * 2 + 1) as usize, (j * 2 + 1) as usize);
        j
    }
}

#[derive(Bundle)]
pub struct ClothBundle {
    pub cloth: Cloth,
    pub elv: ELV,
    pub spring_k: SpringK,
    pub mass: Mass,
}
impl Default for ClothBundle {
    fn default() -> Self {
        Self {
            spring_k: SpringK(8000.),
            cloth: Cloth,
            mass: Mass(1.),
            elv: ELV {
                E: vec![],
                L: vec![],
                V: vec![],
            },
        }
    }
}
