pub fn deref<T>(addr: usize) -> T {
  unsafe { (addr as *const T).read() }
}
