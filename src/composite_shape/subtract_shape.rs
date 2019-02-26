use super::*;

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