// 点定义
#[derive(Debug)]
pub struct Vertex<'a> {
    id: usize,
    name: &'a str,
}

impl Vertex<'_> {
    pub fn new(id: usize, name: &'static str) -> Self {
        Self { id, name }
    }
}

// 边定义
#[derive(Debug, Clone)]
pub struct Edge {
    edge: bool,
}

impl Edge {
    fn new() -> Self {
        Self { edge: false }
    }
    fn set_edge() -> Self {
        Edge { edge: true }
    }
}

// 图定义
#[derive(Debug)]
pub struct Graph {
    nodes: usize,
    graph: Vec<Vec<Edge>>,
}

impl Graph {
    pub fn new(nodes: usize) -> Self {
        Self {
            nodes,
            graph: vec![vec![Edge::new(); nodes]; nodes],
        }
    }

    pub fn len(&self) -> usize {
        self.nodes
    }

    pub fn is_empty(&self) -> bool {
        0 == self.nodes
    }

    pub fn add_edge(&mut self, n1: &Vertex, n2: &Vertex) {
        if n1.id < self.nodes && n2.id < self.nodes {
            self.graph[n1.id][n2.id] = Edge::set_edge();
        } else {
            panic!("error");
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Graph, Vertex};

    #[test]
    fn test_graph_matrix() {
        let mut g = Graph::new(4);

        let n1 = Vertex::new(0, "n1");
        let n2 = Vertex::new(1, "n2");
        let n3 = Vertex::new(2, "n3");
        let n4 = Vertex::new(3, "n4");

        g.add_edge(&n1, &n2);
        g.add_edge(&n1, &n3);
        g.add_edge(&n2, &n3);
        g.add_edge(&n2, &n4);
        g.add_edge(&n3, &n4);
        g.add_edge(&n3, &n1);

        println!("{:#?}", g);
        assert_eq!(g.is_empty(), false);
        assert_eq!(g.len(), 4)
    }
}
