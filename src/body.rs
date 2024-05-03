use std::fmt::Debug;
use crate::num::Num;
use crate::vector::Vector;

pub type BodyId = usize;

#[allow(dead_code)]
pub trait BodyLike: Debug + Copy + Clone + PartialEq {
    type Value;
    
    fn id(&self) -> &BodyId;
    fn pos(&self) -> &Vector;
    fn speed(&self) -> &Vector;
    fn mass(&self) -> &Num;
    fn pos_mut(&mut self) -> &mut Vector;
    fn speed_mut(&mut self) -> &mut Vector;
    fn mass_mut(&mut self) -> &mut Num;
    #[allow(unused_variables)]
    fn get_attr(&self, name: &String) -> Option<Self::Value> {
        None
    }
}


#[macro_export] macro_rules! auto_impl_body {
    ($pos: ident, $speed: ident, $mass: ident, $id: ident, $Value: tt) => {
impl BodyLike for Body {
    type Value = $Value;
    
    fn id(&self) -> &BodyId{
        &self.$id
    }
    fn pos(&self) -> &Vector {
        &self.$pos
    }
    fn speed(&self) -> &Vector {
        &self.$speed
    }
    fn mass(&self) -> &Num {
        &self.$mass
    }
    fn pos_mut(&mut self) -> &mut Vector {
        &mut self.$pos
    }
    fn speed_mut(&mut self) -> &mut Vector {
        &mut self.$speed
    }
    fn mass_mut(&mut self) -> &mut Num {
        &mut self.$mass
    }
}
    };
}

