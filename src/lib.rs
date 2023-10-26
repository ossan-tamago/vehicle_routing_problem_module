use pyo3::prelude::*;
mod simulated_annealing;
mod utils;
mod point;
mod vehicle;

use point::Point;

/// A Python module implemented in Rust.
#[pymodule]
fn vehicle_routing_problem_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(simulated_annealing_fn, m)?)?;
    Ok(())
}

#[pyfunction]
fn simulated_annealing_fn(
    no_of_customer: usize,
    no_of_vehicle: usize,
    vehicle_speed: f64,
    depo: [f64; 3],
    customer_list: Vec<[f64; 3]>,
    // security_list: Vec<[f64; 4]>,
    distance_matrix: Vec<Vec<f64>>,
    initial_temp: f64,
    final_temp: f64,
    cooling_factor: f64,
    no_of_iteration: usize
) -> PyResult<Vec<Vec<usize>>> {
    let points_list = utils::points_from_list(&customer_list);
    println!("points_list OK!");
    let depo = Point::new_depo(0, depo[0], depo[1]);
    let vehicle = vehicle::Vehicle::new(0.0, vehicle_speed);
    println!("simulated_annealing Start!");
    let simulated_annealing = simulated_annealing::SimulatedAnnealing::new(
        no_of_customer,
        no_of_vehicle,
        depo,
        vehicle,
        points_list,
        distance_matrix,
    );
    let route_list = simulated_annealing.run(
        initial_temp,
        final_temp,
        cooling_factor,
        no_of_iteration,
    );
    let list = utils::list_from_points(&route_list);

    Ok(list)
}