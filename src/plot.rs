use crate::graph::Graph;
use plotters::prelude::*;
use std::error::Error;

pub trait Plot {
    fn plot(&self, filename: &str) -> Result<(), Box<dyn Error>>;
}

impl Plot for Graph {
    fn plot(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let pos = match &self.pos {
            Some(p) => p,
            None => return Err("Impossible to plot the graph: no coords available".into()),
        };
        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;

        for i in 0..self.n_nodes() {
            let x = pos[[i, 0]];
            let y = pos[[i, 1]];
            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }
        }
        let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .margin(20)
            .build_cartesian_2d((min_x - 0.5)..(max_x + 0.5), (min_y - 0.5)..(max_y + 0.5))?;

        for i in 0..self.n_edges() {
            let src = self.edge_index[[0, i]];
            let dst = self.edge_index[[1, i]];

            let x1 = pos[[src, 0]];
            let y1 = pos[[src, 1]];
            let x2 = pos[[dst, 0]];
            let y2 = pos[[dst, 1]];

            chart.draw_series(std::iter::once(PathElement::new(
                vec![(x1, y1), (x2, y2)],
                &BLACK.mix(0.5),
            )))?;
        }

        for i in 0..self.n_nodes() {
            let x = pos[[i, 0]];
            let y = pos[[i, 1]];

            chart.draw_series(std::iter::once(Circle::new((x, y), 8, RED.filled())))?;
        }

        root.present()?;
        println!("Graphe exporté avec succès vers {}", filename);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_graph_plot() {
        let nodes = array![[0.0], [0.0], [0.0]];
        let edge_index = array![[0, 1, 2], [1, 2, 0]];
        let pos = array![[0.0, 0.0], [1.0, 2.0], [2.0, 0.0]];

        let graph = Graph::new(nodes, edge_index, None, None, Some(pos));

        let filename = "test_output.png";

        let plot_result = graph.plot(filename);

        assert!(plot_result.is_ok(), "La méthode plot a retourné une erreur");
        assert!(
            Path::new(filename).exists(),
            "Le fichier image n'a pas été créé"
        );

        let _ = fs::remove_file(filename);
    }

    #[test]
    fn test_graph_plot_without_pos() {
        let nodes = array![[0.0], [0.0]];
        let edge_index = array![[0], [1]];

        let graph = Graph::new(nodes, edge_index, None, None, None);

        let plot_result = graph.plot("should_fail.png");

        assert!(
            plot_result.is_err(),
            "Le plot aurait dû échouer sans coordonnées"
        );
    }
}
