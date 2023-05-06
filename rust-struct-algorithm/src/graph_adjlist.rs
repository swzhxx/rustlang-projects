use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct Vertex<T> {
    key: T,
    connects: Vec<(T, i32)>, // 邻点集合
}

impl<T: Clone + PartialEq> Vertex<T> {
    pub fn new(key: T) -> Self {
        Self {
            key,
            connects: vec![],
        }
    }

    // 判断与当前点是否相邻
    pub fn adjacent_key(&self, key: &T) -> bool {
        for (nbr, _wt) in self.connects.iter() {
            if nbr == key {
                return true;
            }
        }
        false
    }

    pub fn add_neighbor(&mut self, nbr: T, wt: i32) {
        self.connects.push((nbr, wt))
    }

    // 获取相邻的点集合
    pub fn get_connects(&self) -> Vec<&T> {
        let mut connects = vec![];
        for (nbr, _wt) in self.connects.iter() {
            connects.push(nbr)
        }
        connects
    }

    // 返回到邻点的边权重
    pub fn get_nbr_weight(&self, key: &T) -> &i32 {
        for (nbr, wt) in self.connects.iter() {
            if nbr == key {
                return wt;
            }
        }
        &0
    }
}

#[derive(Debug, Clone)]
pub struct Graph<T> {
    vertnums: u32,
    edgenums: u32,
    vertices: HashMap<T, Vertex<T>>,
}

impl<T> Graph<T>
where
    T: Hash + Eq + PartialEq + Clone,
{
    pub fn new() -> Self {
        Self {
            vertnums: 0,
            edgenums: 0,
            vertices: HashMap::<T, Vertex<T>>::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        0 == self.vertnums
    }

    pub fn vertex_num(&self) -> u32 {
        self.vertnums
    }

    pub fn edge_num(&self) -> u32 {
        self.edgenums
    }

    pub fn contains(&self, key: &T) -> bool {
        for (nbr, _vertex) in self.vertices.iter() {
            if nbr == key {
                return true;
            }
        }
        false
    }

    pub fn add_vertex(&mut self, key: &T) -> Option<Vertex<T>> {
        let vertex = Vertex::new(key.clone());
        self.vertnums += 1;
        self.vertices.insert(key.clone(), vertex)
    }

    pub fn get_vertex(&self, key: &T) -> Option<&Vertex<T>> {
        self.vertices.get(key)
    }

    // 获取所有节点的key
    pub fn vertex_keys(&self) -> Vec<T> {
        let mut keys = vec![];
        for key in self.vertices.keys() {
            keys.push(key.clone());
        }
        keys
    }

    // 删除点(同时删除边)
    pub fn remove_vertex(&mut self, key: &T) -> Option<Vertex<T>> {
        let old_vertex = self.vertices.remove(key);
        self.vertnums -= 1;
        self.edgenums -= old_vertex.clone().unwrap().get_connects().len() as u32;
        for vertex in self.vertex_keys() {
            if let Some(vt) = self.vertices.get_mut(&vertex) {
                if vt.adjacent_key(key) {
                    vt.connects.retain(|(k, _)| k != key);
                    self.edgenums -= 1;
                }
            }
        }
        old_vertex
    }

    pub fn add_edge(&mut self, from: &T, to: &T, wt: i32) {
        if !self.contains(from) {
            self.add_vertex(from);
        }
        if !self.contains(to) {
            self.add_vertex(to);
        }
        self.edgenums += 1;
        self.vertices
            .get_mut(from)
            .unwrap()
            .add_neighbor(to.clone(), wt);
    }

    pub fn adjacent(&self, from: &T, to: &T) -> bool {
        self.vertices.get(from).unwrap().adjacent_key(to)
    }
}

#[cfg(test)]
mod test {
    use super::Graph;

    #[test]
    fn test_graph() {
        let mut g = Graph::new();
        for i in 0..6 {
            g.add_vertex(&i);
        }
        assert_eq!(false, g.is_empty());
        let vertices = g.vertex_keys();
        for vertex in vertices {
            println!("Vertex {:#?}", vertex);
        }

        g.add_edge(&0, &1, 5);
        g.add_edge(&0, &5, 2);
        g.add_edge(&1, &2, 4);
        g.add_edge(&2, &3, 9);
        g.add_edge(&3, &4, 7);
        g.add_edge(&3, &5, 3);
        g.add_edge(&4, &0, 1);
        g.add_edge(&4, &4, 8);
    }
    
    
}
