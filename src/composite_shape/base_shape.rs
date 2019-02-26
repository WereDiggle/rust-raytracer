use super::*;

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