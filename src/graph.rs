use ndarray::{Array1, Array2};

pub struct Graph {
    pub nodes: Array2<f32>,
    pub edge_index: Array2<usize>,
    pub edge_attr: Option<Array2<f32>>,
    pub y: Option<Array1<f32>>,
    pub pos: Option<Array2<f32>>,
}

impl Graph {
    pub fn new(
        nodes: Array2<f32>,
        edge_index: Array2<usize>,
        edge_attr: Option<Array2<f32>>,
        y: Option<Array1<f32>>,
        pos: Option<Array2<f32>>,
    ) -> Graph {
        Graph {
            nodes,
            edge_index,
            edge_attr,
            y,
            pos,
        }
    }
    pub fn n_nodes(&self) -> usize {
        self.nodes.nrows()
    }
    pub fn n_nodes_features(&self) -> usize {
        self.nodes.ncols()
    }
    pub fn n_edges(&self) -> usize {
        self.edge_index.ncols()
    }
    pub fn n_edge_features(&self) -> Option<usize> {
        self.edge_attr.as_ref().map(|attr| attr.ncols())
    }
}
