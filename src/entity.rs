use std::{collections::HashMap, sync::{atomic::AtomicU64, OnceLock}};

use crate::{hold::obj::Obj, projectiles::Projectile};

macro_rules! as_none {
    ($a:ident, $c:tt) => {
        fn $c(&mut self) -> Option<&mut dyn $a> {
            None
        }
    };
}

macro_rules! from_entity {
    ($t:tt, $as:tt) => {
        impl<'a> FromEntity<'a> for &'a mut dyn $t {
            fn from_entity(other: &'a mut &'a mut dyn Entity<'a>) -> Option<Self> where Self: Sized {
                other.$as()
            }
        }
                
    };
}

pub trait Entity<'a> {
    as_none! { Obj, as_obj }
    as_none! { Projectile, as_proj }
}

pub trait FromEntity<'a> {
    fn from_entity(other: &'a mut &'a mut dyn Entity<'a>) -> Option<Self> where Self: Sized {
        None
    }
}

from_entity! { Obj, as_obj }
from_entity! { Projectile, as_proj }

pub struct Entities<'a>(pub HashMap<u64, &'a mut dyn Entity<'a>>);

impl<'a> Entities<'a> {
    pub fn as_mut_<T: FromEntity<'a>>(s: *mut Self) -> HashMap<u64, T> {
        let mut ts = HashMap::new();
        for (id, entity) in &mut unsafe { s.as_mut().unwrap() }.0 {
            if let Some(t) = T::from_entity(entity) {
                ts.insert(*id, t);
            }
        }
        ts
    }
    pub fn as_mut<T: FromEntity<'a>>(&mut self) -> HashMap<u64, T> {
        Entities::as_mut_(self)
    }

    pub fn append<T: Entity<'a> + 'a>(&mut self, other: *mut HashMap<u64, T>) {
        self.0.extend(&mut unsafe { other.as_mut() }.unwrap().iter_mut().map(|x| (*x.0, x.1 as &mut dyn Entity)));
    }
}