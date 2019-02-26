use super::*;

#[derive(Clone)]
pub struct XorShape {
    transform: TransformComponent,
    primary: Box<Compositable + Send + Sync>,
    secondary: Box<Compositable + Send + Sync>,
}

impl XorShape {
    pub fn new(primary: Box<Compositable + Send + Sync>, secondary: Box<Compositable + Send + Sync>) -> XorShape {
        XorShape {
            transform: TransformComponent::new(DMat4::identity()),
            primary,
            secondary,
        }
    }
}

impl Intersectable for XorShape {

    fn get_closest_intersect(&self, ray: Ray) -> Option<Intersect> {
        let mut ret_intersect: Option<Intersect> = None;
        calculate_intersects(self, ray, &mut |cur_state: usize, states: &Vec<bool>, intersect| {
            if !states[0] && !states[1] {
                ret_intersect = Some(intersect);
            }
            else if states[0] && states[1] || states[1-cur_state] {
                ret_intersect = Some(intersect.invert_normal());
            }
            else /*if states[curstate]*/ {
                ret_intersect = Some(intersect);
            }
            Control::Return;
        });
        ret_intersect
    }

    fn get_all_intersects(&self, ray: Ray) -> Vec<Intersect> {
        let mut ret_intersects: Vec<Intersect> = Vec::new();
        calculate_intersects(self, ray, &mut |cur_state: usize, states: &Vec<bool>, intersect| {
            if !states[0] && !states[1] {
                ret_intersect.push(intersect);
            }
            else if states[0] && states[1] || states[1-cur_state] {
                ret_intersect.push(intersect.invert_normal());
            }
            else /*if states[curstate]*/ {
                ret_intersect.push(intersect);
            }
            Control::Nop
        });
        ret_intersects
    }
}

impl Compositable for XorShape {
    fn get_shapes(&self) -> Vec<&(Compositable + Send + Sync)> {
        vec!(self.primary.as_ref(), self.secondary.as_ref())
    }
}

impl Transformable for XorShape {

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
