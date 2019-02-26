use euler::{DMat4};
use geometry::{Intersectable, Intersect, Ray, Transformable, TransformComponent};

pub mod base_shape;
pub mod subtract_shape;

pub use self::base_shape::*;
pub use self::subtract_shape::*;

#[derive(PartialEq, Copy, Clone)]
enum Hit {
    Enter,
    Exit,
}

enum Control {
    Return,
    Nop,
}

fn hit_direction(intersect: Intersect) -> Hit {
    let dot = intersect.surface_normal.dot(intersect.ray.direction);
    if dot < 0.0 {Hit::Enter} else {Hit::Exit}
}

// Assumes A and B are already sorted by distance
fn merge(A: Vec<(usize, Hit, Intersect)>, B: Vec<(usize, Hit, Intersect)>) -> Vec<(usize, Hit, Intersect)> {
    let mut all_intersects: Vec<(usize, Hit, Intersect)> = Vec::with_capacity(A.len() + B.len());

    let mut b_iter = B.into_iter();
    let mut some_b = b_iter.next();

    let mut a_iter = A.into_iter();
    let mut some_a = a_iter.next();

    for _ in 0..all_intersects.capacity() {
        // Take the shortest distance one
        // TODO: refactor this ugly mess
        let shortest = if let Some(a) = some_a {
            if let Some(b) = some_b {
                if a.2.distance < b.2.distance {
                    some_a = a_iter.next();
                    a
                }
                else {
                    some_b = b_iter.next();
                    b
                }
            }
            else {
                some_a = a_iter.next();
                a
            }
        }
        else {
            if let Some(b) = some_b {
                some_b = b_iter.next();
                b
            }
            else { break; }
        };
        all_intersects.push(shortest);
    }

    all_intersects
}

// General function for calculating the intersections of any boolean operation
fn calculate_intersects(comp: &(Compositable + Send + Sync), ray: Ray, mut check_get: &mut FnMut(usize, &Vec<bool>, Intersect) -> Control) 
{

    let ray = ray.transform(comp.get_inverse_transform());

    // Gather all intersects and sort by distance
    let shapes = comp.get_shapes();
    let mut shape_intersects: Vec<Vec<Intersect>> = Vec::with_capacity(shapes.len());
    for shape in shapes.iter() {
        let mut intersects = shape.get_all_intersects(ray);
        intersects.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
        shape_intersects.push(intersects);
    }

    // Categorize each intersect
    let mut temp_intersects: Vec<Vec<(usize, Hit, Intersect)>> = Vec::with_capacity(shapes.len());
    for (i, shape_sects) in shape_intersects.into_iter().enumerate() {
        let mut intersects: Vec<(usize, Hit, Intersect)> = shape_sects.into_iter().map(|x| (i, hit_direction(x), x)).collect();
        temp_intersects.push(intersects);
    }
    // I want to keep this name
    let shape_intersects = temp_intersects;
    
    // states need to be initialized depending on the intersects
    let mut states: Vec<bool> = shape_intersects.iter().map(|vec| {
        if let Some(first_pos) = vec.first() {
            match first_pos.1 {
                Hit::Enter => false,
                Hit::Exit => true,
            }
        }
        else {
            false
        }
    }).collect();

    let all_intersects = shape_intersects.iter().fold(Vec::new(), |acc, x| merge(acc, x.to_vec()));
    for (cur_state, _, intersect) in all_intersects {
        
        // check_get will test the states and put the intersect where it needs to be
        match check_get(cur_state, &states, intersect.transform(comp.get_transform())) {
            Control::Return => return,
            Control::Nop => (),
        }

        // Change states at the end
        states[cur_state] = !states[cur_state];
    }
}

pub trait Compositable: Intersectable + Transformable + CompositableClone {
    fn get_shapes(&self) -> Vec<&(Compositable + Send + Sync)>;
}

pub trait CompositableClone {
    // Seems to be the only way to get this one to clone,
    // Due to Compositable also being bound to Intersectable and Transformable
    fn clone_compositable_box(&self) -> Box<Compositable + Send + Sync>;
}

impl<T> CompositableClone for T
where
    T: 'static + Compositable + Send + Sync + Clone
{
    fn clone_compositable_box(&self) -> Box<Compositable + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Box<Compositable + Send + Sync> {
    fn clone(&self) -> Box<Compositable + Send + Sync> {
        self.clone_compositable_box()
    }
}

