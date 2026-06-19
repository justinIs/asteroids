use crate::ship;

pub struct Controls {
    pub ship_controls: ship::Controls,
    pub fire: bool,
    pub pause: bool,
    pub using_touch: bool,
}

impl Controls {
    pub fn any_press(&self) -> bool {
        true
    }
}
