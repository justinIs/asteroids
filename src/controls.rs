use crate::ship;

pub struct Controls {
    pub ship_controls: ship::ShipControls,
    pub fire: bool,
    pub pause: bool,
    pub using_touch: bool,
}

impl Controls {
    pub fn any_press(&self) -> bool {
        true
    }
}
