use std::fmt::{Debug, Error, Formatter};
use std::rc::Rc;

pub struct Callback<A>(pub Option<Rc<dyn Fn(A)>>);

impl<A> Callback<A> {
  pub fn send(&self, value: A) {
    if let Some(callback) = &self.0 {
      callback(value)
    }
  }

  pub fn is_empty(&self) -> bool {
    self.0.is_none()
  }
}

impl<A> Default for Callback<A> {
  fn default() -> Self {
    Callback(None)
  }
}

impl<A> Clone for Callback<A> {
  fn clone(&self) -> Self {
    Callback(self.0.clone())
  }
}

impl<A> Debug for Callback<A> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
    write!(f, "Callback()")
  }
}

impl<A, F: Fn(A) + 'static> From<F> for Callback<A> {
  fn from(func: F) -> Self {
    Callback(Some(Rc::new(func)))
  }
}
