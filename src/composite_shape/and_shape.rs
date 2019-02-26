use super::*;

#[derive(Clone)]
pub struct AndShape {
    transform: TransformComponent,
    primary: Box<Compositable + Send + Sync>,
    secondary: Box<Compositable + Send + Sync>,
}

impl AndShape {
    pub fn new(primary: Box<Compositable + Send + Sync>, secondary: Box<Compositable + Send + Sync>) -> AndShape {
        AndShape {
            transform: TransformComponent::new(DMat4::identity()),
            primary,
            secondary,
        }
    }
}

impl Intersectable for AndShape {

    fn get_closest_intersect(&self, ray: Ray) -> Option<Intersect> {
        let mut ret_intersect: Option<Intersect> = None;
        calculate_intersects(self, ray, &mut |cur_state: usize, states: &Vec<bool>, intersect| {
            if states[1-curstate] || (states[0] && states[1]) {
                ret_intersect = Some(intersect);
                return Control::Return;
            }
            Control::Nop
        });
        ret_intersect
    }

    fn get_all_intersects(&self, ray: Ray) -> Vec<Intersect> {
        let mut ret_intersects: Vec<Intersect> = Vec::new();
        calculate_intersects(self, ray, &mut |cur_state: usize, states: &Vec<bool>, intersect| {
            if states[1-curstate] || (states[0] && states[1]) {
                ret_intersect.push(intersect);
            }
            Control::Nop
        });
        ret_intersects
    }
}

impl Compositable for AndShape {
    fn get_shapes(&self) -> Vec<&(Compositable + Send + Sync)> {
        vec!(self.primary.as_ref(), self.secondary.as_ref())
    }
}

impl Transformable for AndShape {

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
