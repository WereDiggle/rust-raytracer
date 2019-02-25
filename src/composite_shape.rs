use euler::{dvec3, DVec3, DMat4};
use geometry::{Intersectable, Intersect, Ray, matrix::*, Transformable, TransformComponent};

pub trait Compositable: Intersectable + Transformable + CompositableClone {}

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

impl Compositable for BaseShape {}

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

/*
    fn surface_normal(&self, hit_point: DVec3) -> DVec3 {
        transform_normal_with_inverse(self.transform.get_inverse_transform(), 
                                      self.primitive.surface_normal(transform_point(self.transform.get_inverse_transform(), hit_point)))
    }
    */

    fn get_all_intersects(&self, ray: Ray) -> Vec<Intersect> {
        let ray = ray.transform(self.transform.get_inverse_transform());
        let intersects = self.primitive.get_all_intersects(ray);
        intersects.into_iter().map(|inter| inter.transform(self.transform.get_transform())).collect()
    }
}

enum Control {
    Return,
    Nop,
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

    fn get_intersects<F>(&self, ray: Ray, mut func: F)
        where F: FnMut(Intersect) -> Control {

        let ray = ray.transform(self.transform.get_inverse_transform());

        // Gather all intersects
        let mut positive_intersects = self.positive.get_all_intersects(ray);
        let mut negative_intersects = self.negative.get_all_intersects(ray);

        // sort by distance
        positive_intersects.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
        negative_intersects.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

        // Handle trivial cases
        if positive_intersects.len() == 0 {
            return;
        }
        else if negative_intersects.len() == 0 {
            // we know positive_intersects.len() != 0
            for intersect in positive_intersects.into_iter() {
                match func(intersect.transform(self.transform.get_transform())) {
                    Control::Return => return,
                    Control::Nop => (),
                }
            }
            return;
        }

        // Categorize each intersect
        let positive_intersects: Vec<(Sign, Hit, Intersect)> = positive_intersects.into_iter().map(|x| (Sign::Pos, hit_direction(x), x)).collect();
        let negative_intersects: Vec<(Sign, Hit, Intersect)> = negative_intersects.into_iter().map(|x| (Sign::Neg, hit_direction(x), x)).collect();
        
        // states need to be initialized depending on the intersects
        let mut in_pos = false;
        let mut in_neg = false;
        if let Some(first_pos) = positive_intersects.first() {
            match first_pos.1 {
                Hit::Enter => in_pos = false,
                Hit::Exit => in_pos = true,
            }
        }
        if let Some(first_neg) = negative_intersects.first() {
            match first_neg.1 {
                Hit::Enter => in_neg = false,
                Hit::Exit => in_neg = true,
            }
        }

        let all_intersects = merge(positive_intersects, negative_intersects);
        for (sign, _, intersect) in all_intersects {

            // positive intersect
            if !in_neg && sign == Sign::Pos {
                match func(intersect.transform(self.transform.get_transform())) {
                    Control::Return => return,
                    Control::Nop => (),
                }
            }
            // subtracted intersect
            else if in_pos && sign == Sign::Neg {
                match func(intersect.transform(self.transform.get_transform()).invert_normal()) {
                    Control::Return => return,
                    Control::Nop => (),
                }
            }

            // Change states at the end
            match sign {
                Sign::Pos => in_pos = !in_pos,
                Sign::Neg => in_neg = !in_neg,
            }
        }
    }
}

impl Compositable for SubtractShape {}

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

#[derive(PartialEq, Copy, Clone)]
enum Hit {
    Enter,
    Exit,
}

#[derive(PartialEq, Copy, Clone)]
enum Sign {
    Pos,
    Neg,
}

fn hit_direction(intersect: Intersect) -> Hit {
    let dot = intersect.surface_normal.dot(intersect.ray.direction);
    if dot < 0.0 {Hit::Enter} else {Hit::Exit}
}

// Assumes A and B are already sorted by distance
fn merge(A: Vec<(Sign, Hit, Intersect)>, B: Vec<(Sign, Hit, Intersect)>) -> Vec<(Sign, Hit, Intersect)> {
    let mut all_intersects: Vec<(Sign, Hit, Intersect)> = Vec::with_capacity(A.len() + B.len());
    let mut b_iter = B.into_iter();
    let mut some_b = b_iter.next();
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
    all_intersects
}

impl Intersectable for SubtractShape {

    fn get_closest_intersect(&self, ray: Ray) -> Option<Intersect> {
        let mut ret_intersect: Option<Intersect> = None;
        self.get_intersects(ray, |intersect| {
            ret_intersect = Some(intersect);
            Control::Return
        });
        ret_intersect
    }

    fn get_all_intersects(&self, ray: Ray) -> Vec<Intersect> {
        let mut ret_intersects: Vec<Intersect> = Vec::new();
        self.get_intersects(ray, |intersect| {
            ret_intersects.push(intersect);
            Control::Nop
        });
        ret_intersects
    }
}
