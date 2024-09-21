pub fn as_ref<T>(addr: usize) -> &'static T {
  unsafe { &*(addr as *const T) }
}

pub fn as_ref_mut<T>(addr: usize) -> &'static mut T {
  unsafe { &mut *(addr as *mut T) }
}

pub fn ptr<T>(r: &T) -> usize {
  r as *const T as usize
}

pub fn read<T>(addr: usize) -> T {
  unsafe { (addr as *const T).read() }
}
