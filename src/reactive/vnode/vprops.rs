use std::{
  any::{Any, TypeId},
  sync::atomic::{AtomicBool, Ordering},
};

pub struct VProps {
  valid: AtomicBool,
  type_id: TypeId,
  data: *mut (),
}

impl VProps {
  // fn null() -> Self {
  //   VProps {
  //     valid: AtomicBool::new(false),
  //     type_id: TypeId::of::<()>(),
  //     data: std::ptr::null_mut(),
  //   }
  // }

  pub fn new<Props: Any>(props: Props) -> Self {
    VProps {
      valid: AtomicBool::new(true),
      type_id: TypeId::of::<Props>(),
      data: Box::into_raw(Box::new(props)) as *mut (),
    }
  }

  pub fn unwrap<Props: Any>(&self) -> Props {
    if !self.valid.swap(false, Ordering::SeqCst) {
      panic!("tried to unwrap AnyProps of type {:?} twice", self.type_id)
    }

    if self.type_id != TypeId::of::<Props>() {
      panic!(
        "passed type {:?} to constructor expecting type {:?}",
        self.type_id,
        TypeId::of::<Props>()
      )
    }

    #[allow(unsafe_code)]
    unsafe {
      *Box::from_raw(self.data as *mut Props)
    }
  }
}
