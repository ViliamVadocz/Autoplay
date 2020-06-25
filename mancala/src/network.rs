use crate::agent::Agent;
use crate::game::Game;

use rand;
use rand_distr::{Distribution, Open01};
use rulinalg::{matrix::Matrix, vector::Vector};
use std::cmp::Ordering::Less;

pub struct Network {
    weights: Vec<Matrix<f64>>,
    biases: Vec<Vector<f64>>,
    activation_func: fn(f64) -> f64,
}

impl Network {
    pub fn from(
        layer_sizes: &[usize],
        activation_func: fn(f64) -> f64,
    ) -> Result<Network, &'static str> {
        if *layer_sizes
            .first()
            .ok_or("a network with no nodes is not a network")?
            != 112
            && *layer_sizes
                .last()
                .ok_or("a network with no nodes is not a network")?
                != 1
        {
            return Err("network should have 112 input nodes (board repr as bits) and 1 output node (static eval)");
        }
        let len = layer_sizes.len();
        let layer_shapes = layer_sizes[0..(len - 1)]
            .iter()
            .zip(layer_sizes[1..len].iter());

        let mut rng = rand::thread_rng();
        let distr = Open01;

        // ugly ew
        let weights = layer_shapes
            .clone()
            .map(|(&rows, &cols)| {
                Matrix::new(
                    rows,
                    cols,
                    distr
                        .sample_iter(&mut rng)
                        .take(rows * cols)
                        .collect::<Vec<f64>>(),
                )
            })
            .collect();
        let biases = layer_shapes
            .map(|(&rows, _)| {
                Vector::from(distr.sample_iter(&mut rng).take(rows).collect::<Vec<f64>>())
            })
            .collect();

        Ok(Network {
            weights,
            biases,
            activation_func,
        })
    }

    // pub fn from_save(file_path) -> Network {
    //     Network {}
    // }

    fn feedforward(&self, mut activations: Vector<f64>) -> f64 {
        for (w, b) in self.weights.iter().zip(self.biases.iter()) {
            activations = w * activations + b;
            for node in activations.iter_mut() {
                *node = (self.activation_func)(*node);
            }
        }
        activations.data()[0]
    }

    fn create_input(game: &Game) -> Vector<f64> {
        // each number's bits are a separate input neuron
        let mut input: Vec<f64> = Vec::new();
        for &x in game.board.iter() {
            let mut num = x;
            // create an input vector from bits
            for _ in 0..8 {
                input.push((num & 1) as f64);
                num >>= 1;
            }
        }
        Vector::from(input)
    }
}

impl Agent for Network {
    fn evaluate_game(&self, game: Game) -> Result<f64, &'static str> {
        let input = Self::create_input(&game);
        let output = self.feedforward(input);

        // handle nan
        if output.is_nan() {
            Err("encountered nan")
        } else {
            Ok(output)
        }
    }
}

// example activation func
pub fn rect_lin_unit(x: f64) -> f64 {
    match x.partial_cmp(&0.0).unwrap() {
        Less => 0.0,
        _ => x,
    }
}
