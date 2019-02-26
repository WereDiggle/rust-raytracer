use euler::{dvec3, DVec3, DMat4};
use geometry::{Intersectable, Intersect, Ray, matrix::*, Transformable, TransformComponent};

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

/*
    'outer: for a in A.into_iter() {
        while let Some(b) = some_b {
            if a.2.distance < b.2.distance {
                all_intersects.push(a);
                continue 'outer;
            }
            else {
                all_intersects.push(b);
                some_b = b_iter.next();
            }
        }
        all_intersects.push(a);
    }
    */
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

#[derive(Clone)]
pub struct BaseShape {
    transform: TransformComponent,
    primitive: Box<Intersectable + Send + Sync>,
}

impl BaseShape {
    pub fn new(matrix: DMat4, primitive: Box<Intersectable + Send + Sync>) -> BaseShape {
        BaseShape {
            transform: TransformComponent::new(matrix),
            primitive,
        }
    }
}

impl Compositable for BaseShape {
    fn get_shapes(&self) -> Vec<&(Compositable + Send + Sync)> {
        vec!(self)
    }
}

impl Transformable for BaseShape {

    fn set_transform(&mut self, trans: DMat4) {
        self.transform.set_transform(trans);
    }

    fn get_transform(&self) -> DMat4 {
        self.transform.get_transform()
    }

    fn get_inverse_transform(&self) -> DMat4 {
        self.transform.get_inverse_transform()
    }
}

impl Intersectable for BaseShape {

    fn get_closest_intersect(&self, ray: Ray) -> Option<Intersect> {
        let ray = ray.transform(self.transform.get_inverse_transform());
        if let Some(intersect) = self.primitive.get_closest_intersect(ray) {
            return Some(intersect.transform(self.transform.get_transform()));
        }
        None
    }

    fn get_all_intersects(&self, ray: Ray) -> Vec<Intersect> {
        let ray = ray.transform(self.transform.get_inverse_transform());
        let intersects = self.primitive.get_all_intersects(ray);
        intersects.into_iter().map(|inter| inter.transform(self.transform.get_transform())).collect()
    }
}

#[derive(Clone)]
pub struct SubtractShape {
    transform: TransformComponent,
    positive: Box<Compositable + Send + Sync>,
    negative: Box<Compositable + Send + Sync>,
}

impl SubtractShape {
    pub fn new(positive: Box<Compositable + Send + Sync>, negative: Box<Compositable + Send + Sync>) -> SubtractShape {
        SubtractShape {
            transform: TransformComponent::new(DMat4::identity()),
            positive,
            negative,
        }
    }
}

impl Compositable for SubtractShape {
    fn get_shapes(&self) -> Vec<&(Compositable + Send + Sync)> {
        vec!(self.positive.as_ref(), self.negative.as_ref())
    }
}

impl Transformable for SubtractShape {

    fn set_transform(&mut self, trans: DMat4) {
        self.transform.set_transform(trans);
    }

    fn get_transform(&self) -> DMat4 {
        self.transform.get_transform()
    }

    fn get_inverse_transform(&self) -> DMat4 {
        self.transform.get_inverse_transform()
    }
}

impl Intersectable for SubtractShape {

    fn get_closest_intersect(&self, ray: Ray) -> Option<Intersect> {
        let mut ret_intersect: Option<Intersect> = None;
        calculate_intersects(self, ray, &mut |cur_state: usize, states: &Vec<bool>, intersect| {
            if !states[1] && cur_state == 0 {
                ret_intersect = Some(intersect);
                return Control::Return;
            }
            else if states[0] && cur_state == 1 {
                ret_intersect = Some(intersect.invert_normal());
                return Control::Return;
            }
            Control::Nop
        });
        ret_intersect
    }

    fn get_all_intersects(&self, ray: Ray) -> Vec<Intersect> {
        let mut ret_intersects: Vec<Intersect> = Vec::new();
        calculate_intersects(self, ray, &mut |cur_state: usize, states: &Vec<bool>, intersect| {
            if !states[1] && cur_state == 0 {
                ret_intersects.push(intersect);
            }
            else if states[0] && cur_state == 1 {
                ret_intersects.push(intersect.invert_normal());
            }
            Control::Nop
        });
        ret_intersects
    }
}
