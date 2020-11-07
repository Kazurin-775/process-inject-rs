pub fn transmute_to_bytes<T>(slice: &[T]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            slice.as_ptr() as *const u8,
            slice.len() * std::mem::size_of::<T>(),
        )
    }
}

pub unsafe fn transmute_to_bytes_mut<T>(slice: &mut [T]) -> &mut [u8] {
    std::slice::from_raw_parts_mut(
        slice.as_ptr() as *mut u8,
        slice.len() * std::mem::size_of::<T>(),
    )
}

pub unsafe fn transmute_from_bytes<T>(bytes: &[u8]) -> &[T] {
    let (head, body, tail) = bytes.align_to();
    assert!(
        head.is_empty() && tail.is_empty(),
        "the input is not properly aligned",
    );
    body
}

pub unsafe fn transmute_from_bytes_mut<T>(bytes: &mut [u8]) -> &mut [T] {
    let (head, body, tail) = bytes.align_to_mut();
    assert!(
        head.is_empty() && tail.is_empty(),
        "the input is not properly aligned",
    );
    body
}
