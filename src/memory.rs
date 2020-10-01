pub fn transmute_to_bytes<T>(data: &[T]) -> &[u8] {
    let len = data.len() * std::mem::size_of::<T>();
    unsafe { std::slice::from_raw_parts(data.as_ptr() as *const u8, len) }
}

pub unsafe fn transmute_from_bytes<T>(data: &[u8]) -> &[T] {
    let (head, body, tail) = data.align_to();
    assert!(
        head.is_empty() && tail.is_empty(),
        "the input is not properly aligned"
    );
    body
}
