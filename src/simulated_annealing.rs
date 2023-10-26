use rand::Rng;

use crate::point::Point;
use crate::vehicle::Vehicle;
use crate::utils;

pub struct SimulatedAnnealing {
    no_of_customer: usize,
    no_of_vehicle: usize,
    depo: Point,
    vehicle: Vehicle,
    customer_list: Vec<Point>,
    distance_matrix: Vec<Vec<f64>>,
}
impl SimulatedAnnealing {
    pub fn new(
        no_of_customer: usize,
        no_of_vehicle: usize,
        depo: Point,
        vehicle: Vehicle,
        customer_list: Vec<Point>,
        distance_matrix: Vec<Vec<f64>>,
    ) -> Self {
        Self {
            no_of_customer,
            no_of_vehicle,
            depo,
            vehicle,
            customer_list,
            distance_matrix,
        }
    }

    pub fn run(
        &self,
        initial_temp: f64,
        final_temp: f64,
        cooling_factor: f64,
        no_of_iteration: usize,
    ) -> Vec<Vec<Point>> {
        let route_list = utils::generate_random_route_list(
            self.no_of_customer,
            self.no_of_vehicle,
            &self.depo,
            &self.vehicle,
            &self.customer_list,
            &self.distance_matrix,
        );
        utils::print_route_id(&route_list);
        let mut best_route_list = route_list.clone();
        let mut current_route_list = route_list.clone();
        let mut neighbour_route_list = route_list.clone();

        let mut cost_of_best_route_list = utils::calculate_max_distance_of_all_route(&route_list, &self.distance_matrix);
        let mut cost_of_current_route_list = cost_of_best_route_list;

        let mut current_temp = initial_temp;

        while current_temp > final_temp {
            let mut iteration = 0;
            while iteration < no_of_iteration {
                iteration += 1;
                neighbour_route_list = Self::get_neighbour(&self, &current_route_list);

                let cost_of_neighbour_route_list = utils::calculate_max_distance_of_all_route(&neighbour_route_list, &self.distance_matrix);

                let cost_diff = cost_of_neighbour_route_list - cost_of_current_route_list;
                if cost_diff < 0.0 || (-cost_diff / current_temp).exp() > rand::random() {
                    current_route_list = neighbour_route_list.clone();
                    cost_of_current_route_list = cost_of_neighbour_route_list;
                    if cost_of_current_route_list < cost_of_best_route_list {
                        best_route_list = current_route_list.clone();
                        cost_of_best_route_list = cost_of_current_route_list;
                    }
                }
            }
            current_temp *= cooling_factor;
        }

        best_route_list
    }

    fn get_neighbour(&self, route_list: &Vec<Vec<Point>>) -> Vec<Vec<Point>> {
        let random_num = rand::random::<usize>() % 3;

        match random_num {
            0 => Self::mutate_insertion(&self, route_list),
            1 => Self::mutate_swap(&self, route_list),
            2 => Self::mutate_inversion(&self, route_list),
            _ => route_list.clone(),
        }
    }

    fn mutate_insertion(&self, route_list: &Vec<Vec<Point>>) -> Vec<Vec<Point>> {
        let mut neighbour_route_list = route_list.clone();
        let mut rng = rand::thread_rng();

        // let mut feasible = false;

        let first_route_index = rng.gen_range(0..neighbour_route_list.len());
        let mut first_route = neighbour_route_list[first_route_index].clone();

        let second_route_index = rng.gen_range(0..neighbour_route_list.len());
        let mut second_route = neighbour_route_list[second_route_index].clone();

        if first_route_index == second_route_index {
            return neighbour_route_list;
        }

        if first_route.len() >= 3 && second_route.len() >= 2{
            let customer_index = rng.gen_range(1..first_route.len() - 1);
            let customer = first_route[customer_index].clone();

            first_route.remove(customer_index);
            
            let index = rng.gen_range(1..second_route.len());
            second_route.insert(index, customer.clone());

        }

        neighbour_route_list[first_route_index] = first_route;
        neighbour_route_list[second_route_index] = second_route;

        neighbour_route_list
    }

    fn mutate_swap(&self, route_list: &Vec<Vec<Point>>) -> Vec<Vec<Point>> {
        let mut neighbour_route_list = route_list.clone();

        let mut rng = rand::thread_rng();

        let first_route_index = rng.gen_range(0..neighbour_route_list.len());
        let second_route_index = rng.gen_range(0..neighbour_route_list.len());

        let mut first_route = neighbour_route_list[first_route_index].clone();
        let mut second_route = neighbour_route_list[second_route_index].clone();

        if first_route_index == second_route_index
            || first_route.len() <= 3
            || second_route.len() <= 3
        {
            return neighbour_route_list;
        }

        let first_route_customer_index = rng.gen_range(1..first_route.len() - 1);
        let second_route_customer_index = rng.gen_range(1..second_route.len() - 1);
        let first_route_customer = first_route[first_route_customer_index].clone();
        let second_route_customer = second_route[second_route_customer_index].clone();

        first_route.remove(first_route_customer_index);
        second_route.remove(second_route_customer_index);
        first_route.insert(first_route_customer_index, second_route_customer.clone());
        second_route.insert(second_route_customer_index, first_route_customer.clone());

        neighbour_route_list[first_route_index] = first_route;
        neighbour_route_list[second_route_index] = second_route;

        neighbour_route_list
    }

    fn mutate_inversion(&self, route_list: &Vec<Vec<Point>>) -> Vec<Vec<Point>> {
        let mut neighbour_route_list = route_list.clone();

        let mut rng = rand::thread_rng();
        // let mut feasible = false;

        let route_index = rng.gen_range(0..neighbour_route_list.len());
        let mut route = neighbour_route_list[route_index].clone();

        if route.len() <= 4 {
            return neighbour_route_list;
        }

        let customer_start = rng.gen_range(1..route.len() - 3);
        let count = rng.gen_range(2..route.len() - 1 - customer_start);

        route[customer_start..=customer_start + count].reverse();

        neighbour_route_list[route_index] = route;

        neighbour_route_list
    }
}
