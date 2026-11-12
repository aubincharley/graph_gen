mod graph;
mod params;
mod plot;
use graph::Graph;
use ndarray::{Array1, Array2, array};
use params::GlobalParams;
use plot::Plot;
use plotters::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nodes = array![[0.0], [0.0], [0.0]];

    let edge_index = array![[0, 1, 2], [1, 2, 0]];

    let mut graph = Graph::new(nodes, edge_index, None, None, None);

    graph.pos = Some(array![[0.0, 0.0], [1.0, 2.0], [2.0, 0.0]]);

    graph.plot("mon_graphe.png")?;
    Ok(())
}
