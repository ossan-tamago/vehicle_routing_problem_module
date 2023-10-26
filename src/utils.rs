use rand::seq::SliceRandom;

use crate::point::Point;
use crate::vehicle::Vehicle;

pub fn generate_random_route_list(
    no_of_customer: usize,
    no_of_vehicle: usize,
    depo: &Point,
    vehicle: &Vehicle,
    customer_list: &Vec<Point>,
    distance_matrix: &Vec<Vec<f64>>,
) -> Vec<Vec<Point>> {
    let mut route_list: Vec<Vec<Point>> = Vec::new();
    let mut customer_list = customer_list.clone();
    let mut rng = rand::thread_rng();
    customer_list.shuffle(&mut rng);
    for _ in 0..no_of_vehicle {
        let mut route: Vec<Point> = Vec::new();
        route.push(depo.clone());
        for _ in 0..no_of_customer / no_of_vehicle {
            let customer = customer_list.pop();
            match customer {
                Some(customer) => route.push(customer),
                None => break,
            }
        }
        route.push(depo.clone());
        route_list.push(route);
    }
    route_list
}

pub fn points_from_list(customer_list: &Vec<[f64; 3]>) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    for customer in customer_list {
        points.push(Point::new_customer( customer[0] as usize, customer[1], customer[2]));
    }
    points
}

pub fn list_from_points(points: &Vec<Vec<Point>>) -> Vec<Vec<usize>> {
    let mut list: Vec<Vec<usize>> = Vec::new();
    for route in points {
        let mut route_list: Vec<usize> = Vec::new();
        for point in route {
            route_list.push(point.id);
        }
        list.push(route_list);
    }
    list
}

pub fn calculate_total_distance_of_all_route(
    route_list: &Vec<Vec<Point>>,
    distance_matrix: &Vec<Vec<f64>>,
) -> f64 {
    let mut total_distance = 0.0;
    for route in route_list {
        total_distance += calculate_total_distance_of_a_route(route, distance_matrix);
    }
    total_distance
}

fn calculate_total_distance_of_a_route(
    route: &Vec<Point>,
    distance_matrix: &Vec<Vec<f64>>,
) -> f64 {
    let mut total_distance = 0.0;
    let mut prev_customer = &route[0];
    for customer in route {
        total_distance += distance_matrix[prev_customer.id][customer.id];
        prev_customer = customer;
    }
    total_distance
}

pub fn calculate_max_distance_of_all_route(
    route_list: &Vec<Vec<Point>>,
    distance_matrix: &Vec<Vec<f64>>,
) -> f64 {
    let mut max_distance = 0.0;
    for route in route_list {
        let distance = calculate_total_distance_of_a_route(route, distance_matrix);
        if distance > max_distance {
            max_distance = distance;
        }
    }
    max_distance
}

pub fn print_route_id(route_list: &Vec<Vec<Point>>) {
    for route in route_list {
        for customer in route {
            print!("{} ", customer.id);
        }
        println!("");
    }
}