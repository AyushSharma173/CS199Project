extern crate ndarray;
use pyo3::prelude::Python;
use numpy::{ToPyArray, PyArray};

struct diffeq; 

impl diffeq{

    pub fn RungeKutta(model: &dyn Fn(f64, Vec<f64>, HashMap) -> i32, initial_conditions: Vec<f64>, t_initial: f64, t_final: , dt: f64)->Vec<Vec<f64>>
    {
        let mut X : Vec<Vec<f64>> = Vector::new();
        let mut t = t_initial;
        let n : i64 = (t_final - t_initial)/dt;
        X.push(initial_conditions);
        
        for k in 0..(n-1) {
            let k1 = dt*model(t, X[k]);
            let k2 = dt*model(t+dt/2, x[:,k] + k1/2);
            let k3 = dt*model(t + dt, X
        }
    }

    pub fn LotkaVolteraModel(x: f64, parameters: HashMap) -> {
  ws
    }

}
