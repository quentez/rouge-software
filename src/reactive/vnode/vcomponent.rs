use std::{any::TypeId, marker::PhantomData};

use crate::reactive::component::Component;

pub struct VComponent<C: Component> {
  parent: PhantomData<C>,
  pub model_type: TypeId,
}
