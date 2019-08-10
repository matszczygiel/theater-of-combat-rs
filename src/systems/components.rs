use crate::maps::*;

pub trait Component {
    fn update(&mut self) {}

    fn owner_id(&self)-> i32;
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct MovingComponent {
    def_moving_pts: i32,
    current_moving_pts: i32,
    pub occupation: Option<hexagons::HexCoordinates>,
    owner_id: i32,
}

impl Component for MovingComponent {
    fn owner_id(&self)-> i32 {
        self.owner_id
    }
}


impl MovingComponent {
    pub  fn new(owner_id: i32, moving_pts: i32) -> Self {
        MovingComponent {
            def_moving_pts: moving_pts,
            current_moving_pts: moving_pts,
            occupation: None,
            owner_id
        }
    }

    pub fn reduce_mv_pts(&mut self, cost: i32) -> Result<&mut Self, &'static str> {
        if self.current_moving_pts < cost {
            return Err("Cost of movement higher than available moving_pts.");
        }
        self.current_moving_pts -= cost;
        Ok(self)
    }

    pub fn reest_mv_pts(&mut self) {
        self.current_moving_pts = self.def_moving_pts;
    }

}
