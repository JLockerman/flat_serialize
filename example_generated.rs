#[derive(Copy, Clone, Debug)]
pub struct Basic<'a> {
    pub header: &'a u64,
    pub data_len: &'a u32,
    pub array: &'a [u16; 3],
    pub data: &'a [u8],
    pub data2: &'a [u8],
}
const _: () = {
    use std::mem::{align_of, size_of};
    let _alignment_check = [()][(0) % align_of::<u64>()];
    let _alignment_check2 = [()][(align_of::<u64>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u64>() < align_of::<u64>()) as u8 as usize];
    let _alignment_check = [()][(0 + size_of::<u64>()) % align_of::<u32>()];
    let _alignment_check2 = [()][(align_of::<u32>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u32>() < align_of::<u32>()) as u8 as usize];
    let _alignment_check = [()][(0 + size_of::<u64>() + size_of::<u32>()) % align_of::<[u16; 3]>()];
    let _alignment_check2 = [()][(align_of::<[u16; 3]>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<[u16; 3]>() < align_of::<[u16; 3]>()) as u8 as usize];
    let _alignment_check: () =
        [()][(0 + size_of::<u64>() + size_of::<u32>() + size_of::<[u16; 3]>()) % align_of::<u8>()];
    let _alignment_check2: () = [()][(align_of::<u8>() > 8) as u8 as usize];
    let _padding_check: () = [()][(size_of::<u8>() < align_of::<u8>()) as u8 as usize];
    let _alignment_check: () =
        [()][(0 + size_of::<u64>() + size_of::<u32>() + size_of::<[u16; 3]>()) % align_of::<u8>()];
    let _alignment_check2: () = [()][(align_of::<u8>()
        > if align_of::<u8>() < 8 {
            align_of::<u8>()
        } else {
            8
        }) as u8 as usize];
    let _padding_check: () = [()][(size_of::<u8>() < align_of::<u8>()) as u8 as usize];
};
impl<'a> Basic<'a> {
    pub const fn required_alignment() -> usize {
        use std::mem::align_of;
        let mut required_alignment = 1;
        let alignment = align_of::<u64>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = align_of::<u32>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = align_of::<[u16; 3]>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = align_of::<u8>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = align_of::<u8>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        required_alignment
    }
    pub const fn max_provided_alignment() -> Option<usize> {
        use std::mem::align_of;
        let mut min_size = Self::min_len();
        let mut min_align: Option<usize> = None;
        match (Some(align_of::<u8>()), min_align) {
            (None, _) => (),
            (Some(align), None) => min_align = Some(align),
            (Some(align), Some(min)) if align < min => min_align = Some(align),
            _ => (),
        }
        match (Some(align_of::<u8>()), min_align) {
            (None, _) => (),
            (Some(align), None) => min_align = Some(align),
            (Some(align), Some(min)) if align < min => min_align = Some(align),
            _ => (),
        }
        let min_align = match min_align {
            None => return None,
            Some(min_align) => min_align,
        };
        if min_size % 8 == 0 && min_align >= 8 {
            return Some(8);
        }
        if min_size % 4 == 0 && min_align >= 4 {
            return Some(4);
        }
        if min_size % 2 == 0 && min_align >= 2 {
            return Some(2);
        }
        return Some(1);
    }
    pub const fn min_len() -> usize {
        use std::mem::size_of;
        let mut size = 0;
        size += size_of::<u64>();
        size += size_of::<u32>();
        size += size_of::<[u16; 3]>();
        size += 0;
        size += 0;
        size
    }
    #[allow(unused_assignments, unused_variables)]
    #[inline(always)]
    pub unsafe fn try_ref(mut __packet_macro_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr> {
        let __packet_macro_read_len = 0usize;
        let mut header: Option<&u64> = None;
        let mut data_len: Option<&u32> = None;
        let mut array: Option<&[u16; 3]> = None;
        let mut data: Option<&[u8]> = None;
        let mut data2: Option<&[u8]> = None;
        'tryref: loop {
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u64>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u64 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                header = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u32>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u32 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                data_len = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<[u16; 3]>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &[u16; 3] =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                array = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let __packet_macro_read_len: usize = {
                let __packet_macro_count = data_len.cloned().unwrap() as usize;
                let __packet_macro_size = ::std::mem::size_of::<u8>() * __packet_macro_count;
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field_bytes, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field_ptr = __packet_macro_field_bytes.as_ptr();
                let __packet_macro_field = ::std::slice::from_raw_parts(
                    __packet_macro_field_ptr as *const u8,
                    __packet_macro_count,
                );
                debug_assert_eq!(
                    __packet_macro_field_ptr.offset(__packet_macro_size as isize) as usize,
                    __packet_macro_field
                        .as_ptr()
                        .offset(__packet_macro_count as isize) as usize
                );
                __packet_macro_bytes = __packet_macro_rem_bytes;
                data = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let __packet_macro_read_len: usize = {
                let __packet_macro_count = data_len.cloned().unwrap() as usize / 2;
                let __packet_macro_size = ::std::mem::size_of::<u8>() * __packet_macro_count;
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field_bytes, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field_ptr = __packet_macro_field_bytes.as_ptr();
                let __packet_macro_field = ::std::slice::from_raw_parts(
                    __packet_macro_field_ptr as *const u8,
                    __packet_macro_count,
                );
                debug_assert_eq!(
                    __packet_macro_field_ptr.offset(__packet_macro_size as isize) as usize,
                    __packet_macro_field
                        .as_ptr()
                        .offset(__packet_macro_count as isize) as usize
                );
                __packet_macro_bytes = __packet_macro_rem_bytes;
                data2 = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let _ref = Basic {
                header: header.unwrap(),
                data_len: data_len.unwrap(),
                array: array.unwrap(),
                data: data.unwrap(),
                data2: data2.unwrap(),
            };
            return Ok((_ref, __packet_macro_bytes));
        }
        Err(WrapErr::NotEnoughBytes(
            0 + ::std::mem::size_of::<u64>()
                + ::std::mem::size_of::<u32>()
                + ::std::mem::size_of::<[u16; 3]>()
                + (::std::mem::size_of::<u8>()
                    * ((data_len.as_ref().map(|c| **c).unwrap_or(0)) as usize))
                + (::std::mem::size_of::<u8>()
                    * ((data_len.as_ref().map(|c| **c).unwrap_or(0)) as usize / 2)),
        ))
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn fill_vec(&self, mut __packet_macro_bytes: &mut Vec<u8>) {
        __packet_macro_bytes.reserve_exact(self.len());
        let &Basic {
            header,
            data_len,
            array,
            data,
            data2,
        } = self;
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u64>();
            let __packet_field_bytes = header as *const u64 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        };
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u32>();
            let __packet_field_bytes = data_len as *const u32 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        };
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<[u16; 3]>();
            let __packet_field_bytes = array as *const [u16; 3] as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        };
        unsafe {
            let __packet_field_count = (*data_len) as usize;
            let data = &data[..__packet_field_count];
            let __packet_field_size = ::std::mem::size_of::<u8>() * __packet_field_count;
            let __packet_field_field_bytes = data.as_ptr() as *const u8;
            let __packet_field_field_slice =
                ::std::slice::from_raw_parts(__packet_field_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_field_slice)
        };
        unsafe {
            let __packet_field_count = (*data_len) as usize / 2;
            let data2 = &data2[..__packet_field_count];
            let __packet_field_size = ::std::mem::size_of::<u8>() * __packet_field_count;
            let __packet_field_field_bytes = data2.as_ptr() as *const u8;
            let __packet_field_field_slice =
                ::std::slice::from_raw_parts(__packet_field_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_field_slice)
        }
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn len(&self) -> usize {
        let &Basic {
            header,
            data_len,
            array,
            data,
            data2,
        } = self;
        0usize
            + ::std::mem::size_of::<u64>()
            + ::std::mem::size_of::<u32>()
            + ::std::mem::size_of::<[u16; 3]>()
            + (::std::mem::size_of::<u8>() * ((*data_len) as usize))
            + (::std::mem::size_of::<u8>() * ((*data_len) as usize / 2))
    }
}
unsafe impl<'a> FlattenableRef<'a> for Basic<'a> {}
#[derive(Copy, Clone)]
pub struct Nested<'a> {
    pub prefix: &'a u64,
    pub basic: Basic<'a>,
}
const _: () = {
    use std::mem::{align_of, size_of};
    let _alignment_check = [()][(0) % align_of::<u64>()];
    let _alignment_check2 = [()][(align_of::<u64>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u64>() < align_of::<u64>()) as u8 as usize];
    let _alignment_check: () = [()][(0 + size_of::<u64>()) % Basic::required_alignment()];
    let _alignment_check2: () = [()][(Basic::required_alignment() > 8) as u8 as usize];
};
impl<'a> Nested<'a> {
    pub const fn required_alignment() -> usize {
        use std::mem::align_of;
        let mut required_alignment = 1;
        let alignment = align_of::<u64>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = Basic::required_alignment();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        required_alignment
    }
    pub const fn max_provided_alignment() -> Option<usize> {
        use std::mem::align_of;
        let mut min_size = Self::min_len();
        let mut min_align: Option<usize> = None;
        match (Basic::max_provided_alignment(), min_align) {
            (None, _) => (),
            (Some(align), None) => min_align = Some(align),
            (Some(align), Some(min)) if align < min => min_align = Some(align),
            _ => (),
        }
        let min_align = match min_align {
            None => return None,
            Some(min_align) => min_align,
        };
        if min_size % 8 == 0 && min_align >= 8 {
            return Some(8);
        }
        if min_size % 4 == 0 && min_align >= 4 {
            return Some(4);
        }
        if min_size % 2 == 0 && min_align >= 2 {
            return Some(2);
        }
        return Some(1);
    }
    pub const fn min_len() -> usize {
        use std::mem::size_of;
        let mut size = 0;
        size += size_of::<u64>();
        size += Basic::min_len();
        size
    }
    #[allow(unused_assignments, unused_variables)]
    #[inline(always)]
    pub unsafe fn try_ref(mut __packet_macro_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr> {
        let __packet_macro_read_len = 0usize;
        let mut prefix: Option<&u64> = None;
        let mut basic: Option<Basic<'a>> = None;
        'tryref: loop {
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u64>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u64 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                prefix = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let __packet_macro_read_len: usize = {
                let __old_packet_macro_bytes_size = __packet_macro_bytes.len();
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    match Basic::try_ref(__packet_macro_bytes) {
                        Ok((f, b)) => (f, b),
                        Err(WrapErr::InvalidTag(offset)) => {
                            return Err(WrapErr::InvalidTag(__packet_macro_read_len + offset))
                        }
                        Err(..) => break 'tryref,
                    };
                let __packet_macro_size =
                    __old_packet_macro_bytes_size - __packet_macro_rem_bytes.len();
                __packet_macro_bytes = __packet_macro_rem_bytes;
                basic = Some(__packet_macro_field);
                __packet_macro_read_len + __packet_macro_size
            };
            let _ref = Nested {
                prefix: prefix.unwrap(),
                basic: basic.unwrap(),
            };
            return Ok((_ref, __packet_macro_bytes));
        }
        Err(WrapErr::NotEnoughBytes(
            0 + ::std::mem::size_of::<u64>() + Basic::min_len(),
        ))
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn fill_vec(&self, mut __packet_macro_bytes: &mut Vec<u8>) {
        __packet_macro_bytes.reserve_exact(self.len());
        let &Nested { prefix, basic } = self;
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u64>();
            let __packet_field_bytes = prefix as *const u64 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        };
        basic.fill_vec(__packet_macro_bytes);
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn len(&self) -> usize {
        let &Nested { prefix, basic } = self;
        0usize + ::std::mem::size_of::<u64>() + basic.len()
    }
}
unsafe impl<'a> FlattenableRef<'a> for Nested<'a> {}
#[derive(Copy, Clone)]
pub enum BasicEnum<'a> {
    First { data_len: &'a u32, data: &'a [u8] },
    Fixed { array: &'a [u16; 3] },
}
const _: () = {
    use std::mem::{align_of, size_of};
    let _alignment_check = [()][(0) % align_of::<u64>()];
    let _alignment_check2 = [()][(align_of::<u64>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u64>() < align_of::<u64>()) as u8 as usize];
    const _: () = {
        use std::mem::{align_of, size_of};
        let _alignment_check = [()][(0 + size_of::<u64>()) % align_of::<u32>()];
        let _alignment_check2 = [()][(align_of::<u32>() > 8) as u8 as usize];
        let _padding_check = [()][(size_of::<u32>() < align_of::<u32>()) as u8 as usize];
        let _alignment_check: () =
            [()][(0 + size_of::<u64>() + size_of::<u32>()) % align_of::<u8>()];
        let _alignment_check2: () = [()][(align_of::<u8>() > 8) as u8 as usize];
        let _padding_check: () = [()][(size_of::<u8>() < align_of::<u8>()) as u8 as usize];
    };
    const _: () = {
        use std::mem::{align_of, size_of};
        let _alignment_check = [()][(0 + size_of::<u64>()) % align_of::<[u16; 3]>()];
        let _alignment_check2 = [()][(align_of::<[u16; 3]>() > 8) as u8 as usize];
        let _padding_check = [()][(size_of::<[u16; 3]>() < align_of::<[u16; 3]>()) as u8 as usize];
    };
};
const _: () = {
    #[allow(dead_code)]
    enum UniquenessCheck {
        First = 2,
        Fixed = 3,
    }
};
impl<'a> BasicEnum<'a> {
    #[allow(unused_assignments, unused_variables)]
    #[inline(always)]
    pub unsafe fn try_ref(mut __packet_macro_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr> {
        let __packet_macro_read_len = 0usize;
        let mut k = None;
        'tryref_tag: loop {
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u64>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref_tag;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u64 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                k = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            match k {
                Some(&2) => {
                    let mut data_len: Option<&u32> = None;
                    let mut data: Option<&[u8]> = None;
                    'tryref_0: loop {
                        let __packet_macro_read_len: usize = {
                            let __packet_macro_size = ::std::mem::size_of::<u32>();
                            let __packet_macro_read_len =
                                __packet_macro_read_len + __packet_macro_size;
                            if __packet_macro_bytes.len() < __packet_macro_size {
                                break 'tryref_0;
                            }
                            let (__packet_macro_field, __packet_macro_rem_bytes) =
                                __packet_macro_bytes.split_at(__packet_macro_size);
                            let __packet_macro_field: &u32 =
                                ::std::mem::transmute(__packet_macro_field.as_ptr());
                            __packet_macro_bytes = __packet_macro_rem_bytes;
                            data_len = Some(__packet_macro_field);
                            __packet_macro_read_len
                        };
                        let __packet_macro_read_len: usize = {
                            let __packet_macro_count = data_len.cloned().unwrap() as usize;
                            let __packet_macro_size =
                                ::std::mem::size_of::<u8>() * __packet_macro_count;
                            let __packet_macro_read_len =
                                __packet_macro_read_len + __packet_macro_size;
                            if __packet_macro_bytes.len() < __packet_macro_size {
                                break 'tryref_0;
                            }
                            let (__packet_macro_field_bytes, __packet_macro_rem_bytes) =
                                __packet_macro_bytes.split_at(__packet_macro_size);
                            let __packet_macro_field_ptr = __packet_macro_field_bytes.as_ptr();
                            let __packet_macro_field = ::std::slice::from_raw_parts(
                                __packet_macro_field_ptr as *const u8,
                                __packet_macro_count,
                            );
                            debug_assert_eq!(
                                __packet_macro_field_ptr.offset(__packet_macro_size as isize)
                                    as usize,
                                __packet_macro_field
                                    .as_ptr()
                                    .offset(__packet_macro_count as isize)
                                    as usize
                            );
                            __packet_macro_bytes = __packet_macro_rem_bytes;
                            data = Some(__packet_macro_field);
                            __packet_macro_read_len
                        };
                        let _ref = BasicEnum::First {
                            data_len: data_len.unwrap(),
                            data: data.unwrap(),
                        };
                        return Ok((_ref, __packet_macro_bytes));
                    }
                    return Err(WrapErr::NotEnoughBytes(
                        std::mem::size_of::<u64>()
                            + ::std::mem::size_of::<u32>()
                            + (::std::mem::size_of::<u8>()
                                * ((data_len.as_ref().map(|c| **c).unwrap_or(0)) as usize)),
                    ));
                }
                Some(&3) => {
                    let mut array: Option<&[u16; 3]> = None;
                    'tryref_1: loop {
                        let __packet_macro_read_len: usize = {
                            let __packet_macro_size = ::std::mem::size_of::<[u16; 3]>();
                            let __packet_macro_read_len =
                                __packet_macro_read_len + __packet_macro_size;
                            if __packet_macro_bytes.len() < __packet_macro_size {
                                break 'tryref_1;
                            }
                            let (__packet_macro_field, __packet_macro_rem_bytes) =
                                __packet_macro_bytes.split_at(__packet_macro_size);
                            let __packet_macro_field: &[u16; 3] =
                                ::std::mem::transmute(__packet_macro_field.as_ptr());
                            __packet_macro_bytes = __packet_macro_rem_bytes;
                            array = Some(__packet_macro_field);
                            __packet_macro_read_len
                        };
                        let _ref = BasicEnum::Fixed {
                            array: array.unwrap(),
                        };
                        return Ok((_ref, __packet_macro_bytes));
                    }
                    return Err(WrapErr::NotEnoughBytes(
                        std::mem::size_of::<u64>() + ::std::mem::size_of::<[u16; 3]>(),
                    ));
                }
                _ => return Err(WrapErr::InvalidTag(0)),
            }
        }
        Err(WrapErr::NotEnoughBytes(::std::mem::size_of::<u64>()))
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn fill_vec(&self, mut __packet_macro_bytes: &mut Vec<u8>) {
        __packet_macro_bytes.reserve_exact(self.len());
        match self {
            &BasicEnum::First { data_len, data } => {
                let k: &u64 = &2;
                unsafe {
                    let __packet_field_size = ::std::mem::size_of::<u64>();
                    let __packet_field_bytes = k as *const u64 as *const u8;
                    let __packet_field_slice =
                        ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
                    __packet_macro_bytes.extend_from_slice(__packet_field_slice)
                }
                unsafe {
                    let __packet_field_size = ::std::mem::size_of::<u32>();
                    let __packet_field_bytes = data_len as *const u32 as *const u8;
                    let __packet_field_slice =
                        ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
                    __packet_macro_bytes.extend_from_slice(__packet_field_slice)
                };
                unsafe {
                    let __packet_field_count = (*data_len) as usize;
                    let data = &data[..__packet_field_count];
                    let __packet_field_size = ::std::mem::size_of::<u8>() * __packet_field_count;
                    let __packet_field_field_bytes = data.as_ptr() as *const u8;
                    let __packet_field_field_slice = ::std::slice::from_raw_parts(
                        __packet_field_field_bytes,
                        __packet_field_size,
                    );
                    __packet_macro_bytes.extend_from_slice(__packet_field_field_slice)
                }
            }
            &BasicEnum::Fixed { array } => {
                let k: &u64 = &3;
                unsafe {
                    let __packet_field_size = ::std::mem::size_of::<u64>();
                    let __packet_field_bytes = k as *const u64 as *const u8;
                    let __packet_field_slice =
                        ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
                    __packet_macro_bytes.extend_from_slice(__packet_field_slice)
                }
                unsafe {
                    let __packet_field_size = ::std::mem::size_of::<[u16; 3]>();
                    let __packet_field_bytes = array as *const [u16; 3] as *const u8;
                    let __packet_field_slice =
                        ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
                    __packet_macro_bytes.extend_from_slice(__packet_field_slice)
                }
            }
        }
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn len(&self) -> usize {
        match self {
            &BasicEnum::First { data_len, data } => {
                ::std::mem::size_of::<u64>()
                    + ::std::mem::size_of::<u32>()
                    + (::std::mem::size_of::<u8>() * ((*data_len) as usize))
            }
            &BasicEnum::Fixed { array } => {
                ::std::mem::size_of::<u64>() + ::std::mem::size_of::<[u16; 3]>()
            }
        }
    }
}
unsafe impl<'a> FlattenableRef<'a> for BasicEnum<'a> {}
#[derive(Copy, Clone)]
pub enum PaddedEnum<'a> {
    First {
        padding: &'a [u8; 3],
        data_len: &'a u32,
        data: &'a [u8],
    },
    Fixed {
        padding: &'a u8,
        array: &'a [u16; 3],
    },
}
const _: () = {
    use std::mem::{align_of, size_of};
    let _alignment_check = [()][(0) % align_of::<u8>()];
    let _alignment_check2 = [()][(align_of::<u8>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u8>() < align_of::<u8>()) as u8 as usize];
    const _: () = {
        use std::mem::{align_of, size_of};
        let _alignment_check = [()][(0 + size_of::<u8>()) % align_of::<[u8; 3]>()];
        let _alignment_check2 = [()][(align_of::<[u8; 3]>() > 8) as u8 as usize];
        let _padding_check = [()][(size_of::<[u8; 3]>() < align_of::<[u8; 3]>()) as u8 as usize];
        let _alignment_check =
            [()][(0 + size_of::<u8>() + size_of::<[u8; 3]>()) % align_of::<u32>()];
        let _alignment_check2 = [()][(align_of::<u32>() > 8) as u8 as usize];
        let _padding_check = [()][(size_of::<u32>() < align_of::<u32>()) as u8 as usize];
        let _alignment_check: () = [()]
            [(0 + size_of::<u8>() + size_of::<[u8; 3]>() + size_of::<u32>()) % align_of::<u8>()];
        let _alignment_check2: () = [()][(align_of::<u8>() > 8) as u8 as usize];
        let _padding_check: () = [()][(size_of::<u8>() < align_of::<u8>()) as u8 as usize];
    };
    const _: () = {
        use std::mem::{align_of, size_of};
        let _alignment_check = [()][(0 + size_of::<u8>()) % align_of::<u8>()];
        let _alignment_check2 = [()][(align_of::<u8>() > 8) as u8 as usize];
        let _padding_check = [()][(size_of::<u8>() < align_of::<u8>()) as u8 as usize];
        let _alignment_check =
            [()][(0 + size_of::<u8>() + size_of::<u8>()) % align_of::<[u16; 3]>()];
        let _alignment_check2 = [()][(align_of::<[u16; 3]>() > 8) as u8 as usize];
        let _padding_check = [()][(size_of::<[u16; 3]>() < align_of::<[u16; 3]>()) as u8 as usize];
    };
};
const _: () = {
    #[allow(dead_code)]
    enum UniquenessCheck {
        First = 2,
        Fixed = 3,
    }
};
impl<'a> PaddedEnum<'a> {
    #[allow(unused_assignments, unused_variables)]
    #[inline(always)]
    pub unsafe fn try_ref(mut __packet_macro_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr> {
        let __packet_macro_read_len = 0usize;
        let mut k = None;
        'tryref_tag: loop {
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u8>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref_tag;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u8 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                k = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            match k {
                Some(&2) => {
                    let mut padding: Option<&[u8; 3]> = None;
                    let mut data_len: Option<&u32> = None;
                    let mut data: Option<&[u8]> = None;
                    'tryref_0: loop {
                        let __packet_macro_read_len: usize = {
                            let __packet_macro_size = ::std::mem::size_of::<[u8; 3]>();
                            let __packet_macro_read_len =
                                __packet_macro_read_len + __packet_macro_size;
                            if __packet_macro_bytes.len() < __packet_macro_size {
                                break 'tryref_0;
                            }
                            let (__packet_macro_field, __packet_macro_rem_bytes) =
                                __packet_macro_bytes.split_at(__packet_macro_size);
                            let __packet_macro_field: &[u8; 3] =
                                ::std::mem::transmute(__packet_macro_field.as_ptr());
                            __packet_macro_bytes = __packet_macro_rem_bytes;
                            padding = Some(__packet_macro_field);
                            __packet_macro_read_len
                        };
                        let __packet_macro_read_len: usize = {
                            let __packet_macro_size = ::std::mem::size_of::<u32>();
                            let __packet_macro_read_len =
                                __packet_macro_read_len + __packet_macro_size;
                            if __packet_macro_bytes.len() < __packet_macro_size {
                                break 'tryref_0;
                            }
                            let (__packet_macro_field, __packet_macro_rem_bytes) =
                                __packet_macro_bytes.split_at(__packet_macro_size);
                            let __packet_macro_field: &u32 =
                                ::std::mem::transmute(__packet_macro_field.as_ptr());
                            __packet_macro_bytes = __packet_macro_rem_bytes;
                            data_len = Some(__packet_macro_field);
                            __packet_macro_read_len
                        };
                        let __packet_macro_read_len: usize = {
                            let __packet_macro_count = data_len.cloned().unwrap() as usize;
                            let __packet_macro_size =
                                ::std::mem::size_of::<u8>() * __packet_macro_count;
                            let __packet_macro_read_len =
                                __packet_macro_read_len + __packet_macro_size;
                            if __packet_macro_bytes.len() < __packet_macro_size {
                                break 'tryref_0;
                            }
                            let (__packet_macro_field_bytes, __packet_macro_rem_bytes) =
                                __packet_macro_bytes.split_at(__packet_macro_size);
                            let __packet_macro_field_ptr = __packet_macro_field_bytes.as_ptr();
                            let __packet_macro_field = ::std::slice::from_raw_parts(
                                __packet_macro_field_ptr as *const u8,
                                __packet_macro_count,
                            );
                            debug_assert_eq!(
                                __packet_macro_field_ptr.offset(__packet_macro_size as isize)
                                    as usize,
                                __packet_macro_field
                                    .as_ptr()
                                    .offset(__packet_macro_count as isize)
                                    as usize
                            );
                            __packet_macro_bytes = __packet_macro_rem_bytes;
                            data = Some(__packet_macro_field);
                            __packet_macro_read_len
                        };
                        let _ref = PaddedEnum::First {
                            padding: padding.unwrap(),
                            data_len: data_len.unwrap(),
                            data: data.unwrap(),
                        };
                        return Ok((_ref, __packet_macro_bytes));
                    }
                    return Err(WrapErr::NotEnoughBytes(
                        std::mem::size_of::<u8>()
                            + ::std::mem::size_of::<[u8; 3]>()
                            + ::std::mem::size_of::<u32>()
                            + (::std::mem::size_of::<u8>()
                                * ((data_len.as_ref().map(|c| **c).unwrap_or(0)) as usize)),
                    ));
                }
                Some(&3) => {
                    let mut padding: Option<&u8> = None;
                    let mut array: Option<&[u16; 3]> = None;
                    'tryref_1: loop {
                        let __packet_macro_read_len: usize = {
                            let __packet_macro_size = ::std::mem::size_of::<u8>();
                            let __packet_macro_read_len =
                                __packet_macro_read_len + __packet_macro_size;
                            if __packet_macro_bytes.len() < __packet_macro_size {
                                break 'tryref_1;
                            }
                            let (__packet_macro_field, __packet_macro_rem_bytes) =
                                __packet_macro_bytes.split_at(__packet_macro_size);
                            let __packet_macro_field: &u8 =
                                ::std::mem::transmute(__packet_macro_field.as_ptr());
                            __packet_macro_bytes = __packet_macro_rem_bytes;
                            padding = Some(__packet_macro_field);
                            __packet_macro_read_len
                        };
                        let __packet_macro_read_len: usize = {
                            let __packet_macro_size = ::std::mem::size_of::<[u16; 3]>();
                            let __packet_macro_read_len =
                                __packet_macro_read_len + __packet_macro_size;
                            if __packet_macro_bytes.len() < __packet_macro_size {
                                break 'tryref_1;
                            }
                            let (__packet_macro_field, __packet_macro_rem_bytes) =
                                __packet_macro_bytes.split_at(__packet_macro_size);
                            let __packet_macro_field: &[u16; 3] =
                                ::std::mem::transmute(__packet_macro_field.as_ptr());
                            __packet_macro_bytes = __packet_macro_rem_bytes;
                            array = Some(__packet_macro_field);
                            __packet_macro_read_len
                        };
                        let _ref = PaddedEnum::Fixed {
                            padding: padding.unwrap(),
                            array: array.unwrap(),
                        };
                        return Ok((_ref, __packet_macro_bytes));
                    }
                    return Err(WrapErr::NotEnoughBytes(
                        std::mem::size_of::<u8>()
                            + ::std::mem::size_of::<u8>()
                            + ::std::mem::size_of::<[u16; 3]>(),
                    ));
                }
                _ => return Err(WrapErr::InvalidTag(0)),
            }
        }
        Err(WrapErr::NotEnoughBytes(::std::mem::size_of::<u8>()))
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn fill_vec(&self, mut __packet_macro_bytes: &mut Vec<u8>) {
        __packet_macro_bytes.reserve_exact(self.len());
        match self {
            &PaddedEnum::First {
                padding,
                data_len,
                data,
            } => {
                let k: &u8 = &2;
                unsafe {
                    let __packet_field_size = ::std::mem::size_of::<u8>();
                    let __packet_field_bytes = k as *const u8 as *const u8;
                    let __packet_field_slice =
                        ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
                    __packet_macro_bytes.extend_from_slice(__packet_field_slice)
                }
                unsafe {
                    let __packet_field_size = ::std::mem::size_of::<[u8; 3]>();
                    let __packet_field_bytes = padding as *const [u8; 3] as *const u8;
                    let __packet_field_slice =
                        ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
                    __packet_macro_bytes.extend_from_slice(__packet_field_slice)
                };
                unsafe {
                    let __packet_field_size = ::std::mem::size_of::<u32>();
                    let __packet_field_bytes = data_len as *const u32 as *const u8;
                    let __packet_field_slice =
                        ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
                    __packet_macro_bytes.extend_from_slice(__packet_field_slice)
                };
                unsafe {
                    let __packet_field_count = (*data_len) as usize;
                    let data = &data[..__packet_field_count];
                    let __packet_field_size = ::std::mem::size_of::<u8>() * __packet_field_count;
                    let __packet_field_field_bytes = data.as_ptr() as *const u8;
                    let __packet_field_field_slice = ::std::slice::from_raw_parts(
                        __packet_field_field_bytes,
                        __packet_field_size,
                    );
                    __packet_macro_bytes.extend_from_slice(__packet_field_field_slice)
                }
            }
            &PaddedEnum::Fixed { padding, array } => {
                let k: &u8 = &3;
                unsafe {
                    let __packet_field_size = ::std::mem::size_of::<u8>();
                    let __packet_field_bytes = k as *const u8 as *const u8;
                    let __packet_field_slice =
                        ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
                    __packet_macro_bytes.extend_from_slice(__packet_field_slice)
                }
                unsafe {
                    let __packet_field_size = ::std::mem::size_of::<u8>();
                    let __packet_field_bytes = padding as *const u8 as *const u8;
                    let __packet_field_slice =
                        ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
                    __packet_macro_bytes.extend_from_slice(__packet_field_slice)
                };
                unsafe {
                    let __packet_field_size = ::std::mem::size_of::<[u16; 3]>();
                    let __packet_field_bytes = array as *const [u16; 3] as *const u8;
                    let __packet_field_slice =
                        ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
                    __packet_macro_bytes.extend_from_slice(__packet_field_slice)
                }
            }
        }
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn len(&self) -> usize {
        match self {
            &PaddedEnum::First {
                padding,
                data_len,
                data,
            } => {
                ::std::mem::size_of::<u8>()
                    + ::std::mem::size_of::<[u8; 3]>()
                    + ::std::mem::size_of::<u32>()
                    + (::std::mem::size_of::<u8>() * ((*data_len) as usize))
            }
            &PaddedEnum::Fixed { padding, array } => {
                ::std::mem::size_of::<u8>()
                    + ::std::mem::size_of::<u8>()
                    + ::std::mem::size_of::<[u16; 3]>()
            }
        }
    }
}
unsafe impl<'a> FlattenableRef<'a> for PaddedEnum<'a> {}
#[derive(Copy, Clone, Debug)]
pub struct InMacro<'a> {
    pub a: &'a u32,
    pub padding: &'a [u8; 4],
    pub b: &'a f64,
}
const _: () = {
    use std::mem::{align_of, size_of};
    let _alignment_check = [()][(0) % align_of::<u32>()];
    let _alignment_check2 = [()][(align_of::<u32>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u32>() < align_of::<u32>()) as u8 as usize];
    let _alignment_check = [()][(0 + size_of::<u32>()) % align_of::<[u8; 4]>()];
    let _alignment_check2 = [()][(align_of::<[u8; 4]>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<[u8; 4]>() < align_of::<[u8; 4]>()) as u8 as usize];
    let _alignment_check = [()][(0 + size_of::<u32>() + size_of::<[u8; 4]>()) % align_of::<f64>()];
    let _alignment_check2 = [()][(align_of::<f64>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<f64>() < align_of::<f64>()) as u8 as usize];
};
impl<'a> InMacro<'a> {
    pub const fn required_alignment() -> usize {
        use std::mem::align_of;
        let mut required_alignment = 1;
        let alignment = align_of::<u32>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = align_of::<[u8; 4]>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = align_of::<f64>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        required_alignment
    }
    pub const fn max_provided_alignment() -> Option<usize> {
        use std::mem::align_of;
        let mut min_size = Self::min_len();
        let mut min_align: Option<usize> = None;
        let min_align = match min_align {
            None => return None,
            Some(min_align) => min_align,
        };
        if min_size % 8 == 0 && min_align >= 8 {
            return Some(8);
        }
        if min_size % 4 == 0 && min_align >= 4 {
            return Some(4);
        }
        if min_size % 2 == 0 && min_align >= 2 {
            return Some(2);
        }
        return Some(1);
    }
    pub const fn min_len() -> usize {
        use std::mem::size_of;
        let mut size = 0;
        size += size_of::<u32>();
        size += size_of::<[u8; 4]>();
        size += size_of::<f64>();
        size
    }
    #[allow(unused_assignments, unused_variables)]
    #[inline(always)]
    pub unsafe fn try_ref(mut __packet_macro_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr> {
        let __packet_macro_read_len = 0usize;
        let mut a: Option<&u32> = None;
        let mut padding: Option<&[u8; 4]> = None;
        let mut b: Option<&f64> = None;
        'tryref: loop {
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u32>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u32 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                a = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<[u8; 4]>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &[u8; 4] =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                padding = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<f64>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &f64 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                b = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let _ref = InMacro {
                a: a.unwrap(),
                padding: padding.unwrap(),
                b: b.unwrap(),
            };
            return Ok((_ref, __packet_macro_bytes));
        }
        Err(WrapErr::NotEnoughBytes(
            0 + ::std::mem::size_of::<u32>()
                + ::std::mem::size_of::<[u8; 4]>()
                + ::std::mem::size_of::<f64>(),
        ))
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn fill_vec(&self, mut __packet_macro_bytes: &mut Vec<u8>) {
        __packet_macro_bytes.reserve_exact(self.len());
        let &InMacro { a, padding, b } = self;
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u32>();
            let __packet_field_bytes = a as *const u32 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        };
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<[u8; 4]>();
            let __packet_field_bytes = padding as *const [u8; 4] as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        };
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<f64>();
            let __packet_field_bytes = b as *const f64 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        }
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn len(&self) -> usize {
        let &InMacro { a, padding, b } = self;
        0usize
            + ::std::mem::size_of::<u32>()
            + ::std::mem::size_of::<[u8; 4]>()
            + ::std::mem::size_of::<f64>()
    }
}
unsafe impl<'a> FlattenableRef<'a> for InMacro<'a> {}
#[derive(Copy, Clone)]
pub struct SizeAlignTest<'a> {
    pub f: &'a u8,
}
const _: () = {
    use std::mem::{align_of, size_of};
    let _alignment_check = [()][(0) % align_of::<u8>()];
    let _alignment_check2 = [()][(align_of::<u8>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u8>() < align_of::<u8>()) as u8 as usize];
};
impl<'a> SizeAlignTest<'a> {
    pub const fn required_alignment() -> usize {
        use std::mem::align_of;
        let mut required_alignment = 1;
        let alignment = align_of::<u8>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        required_alignment
    }
    pub const fn max_provided_alignment() -> Option<usize> {
        use std::mem::align_of;
        let mut min_size = Self::min_len();
        let mut min_align: Option<usize> = None;
        let min_align = match min_align {
            None => return None,
            Some(min_align) => min_align,
        };
        if min_size % 8 == 0 && min_align >= 8 {
            return Some(8);
        }
        if min_size % 4 == 0 && min_align >= 4 {
            return Some(4);
        }
        if min_size % 2 == 0 && min_align >= 2 {
            return Some(2);
        }
        return Some(1);
    }
    pub const fn min_len() -> usize {
        use std::mem::size_of;
        let mut size = 0;
        size += size_of::<u8>();
        size
    }
    #[allow(unused_assignments, unused_variables)]
    #[inline(always)]
    pub unsafe fn try_ref(mut __packet_macro_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr> {
        let __packet_macro_read_len = 0usize;
        let mut f: Option<&u8> = None;
        'tryref: loop {
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u8>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u8 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                f = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let _ref = SizeAlignTest { f: f.unwrap() };
            return Ok((_ref, __packet_macro_bytes));
        }
        Err(WrapErr::NotEnoughBytes(0 + ::std::mem::size_of::<u8>()))
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn fill_vec(&self, mut __packet_macro_bytes: &mut Vec<u8>) {
        __packet_macro_bytes.reserve_exact(self.len());
        let &SizeAlignTest { f } = self;
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u8>();
            let __packet_field_bytes = f as *const u8 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        }
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn len(&self) -> usize {
        let &SizeAlignTest { f } = self;
        0usize + ::std::mem::size_of::<u8>()
    }
}
unsafe impl<'a> FlattenableRef<'a> for SizeAlignTest<'a> {}
#[derive(Copy, Clone)]
pub struct SizeAlignTest<'a> {
    pub f: &'a u16,
}
const _: () = {
    use std::mem::{align_of, size_of};
    let _alignment_check = [()][(0) % align_of::<u16>()];
    let _alignment_check2 = [()][(align_of::<u16>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u16>() < align_of::<u16>()) as u8 as usize];
};
impl<'a> SizeAlignTest<'a> {
    pub const fn required_alignment() -> usize {
        use std::mem::align_of;
        let mut required_alignment = 1;
        let alignment = align_of::<u16>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        required_alignment
    }
    pub const fn max_provided_alignment() -> Option<usize> {
        use std::mem::align_of;
        let mut min_size = Self::min_len();
        let mut min_align: Option<usize> = None;
        let min_align = match min_align {
            None => return None,
            Some(min_align) => min_align,
        };
        if min_size % 8 == 0 && min_align >= 8 {
            return Some(8);
        }
        if min_size % 4 == 0 && min_align >= 4 {
            return Some(4);
        }
        if min_size % 2 == 0 && min_align >= 2 {
            return Some(2);
        }
        return Some(1);
    }
    pub const fn min_len() -> usize {
        use std::mem::size_of;
        let mut size = 0;
        size += size_of::<u16>();
        size
    }
    #[allow(unused_assignments, unused_variables)]
    #[inline(always)]
    pub unsafe fn try_ref(mut __packet_macro_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr> {
        let __packet_macro_read_len = 0usize;
        let mut f: Option<&u16> = None;
        'tryref: loop {
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u16>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u16 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                f = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let _ref = SizeAlignTest { f: f.unwrap() };
            return Ok((_ref, __packet_macro_bytes));
        }
        Err(WrapErr::NotEnoughBytes(0 + ::std::mem::size_of::<u16>()))
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn fill_vec(&self, mut __packet_macro_bytes: &mut Vec<u8>) {
        __packet_macro_bytes.reserve_exact(self.len());
        let &SizeAlignTest { f } = self;
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u16>();
            let __packet_field_bytes = f as *const u16 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        }
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn len(&self) -> usize {
        let &SizeAlignTest { f } = self;
        0usize + ::std::mem::size_of::<u16>()
    }
}
unsafe impl<'a> FlattenableRef<'a> for SizeAlignTest<'a> {}
#[derive(Copy, Clone)]
pub struct SizeAlignTest<'a> {
    pub f: &'a u32,
}
const _: () = {
    use std::mem::{align_of, size_of};
    let _alignment_check = [()][(0) % align_of::<u32>()];
    let _alignment_check2 = [()][(align_of::<u32>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u32>() < align_of::<u32>()) as u8 as usize];
};
impl<'a> SizeAlignTest<'a> {
    pub const fn required_alignment() -> usize {
        use std::mem::align_of;
        let mut required_alignment = 1;
        let alignment = align_of::<u32>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        required_alignment
    }
    pub const fn max_provided_alignment() -> Option<usize> {
        use std::mem::align_of;
        let mut min_size = Self::min_len();
        let mut min_align: Option<usize> = None;
        let min_align = match min_align {
            None => return None,
            Some(min_align) => min_align,
        };
        if min_size % 8 == 0 && min_align >= 8 {
            return Some(8);
        }
        if min_size % 4 == 0 && min_align >= 4 {
            return Some(4);
        }
        if min_size % 2 == 0 && min_align >= 2 {
            return Some(2);
        }
        return Some(1);
    }
    pub const fn min_len() -> usize {
        use std::mem::size_of;
        let mut size = 0;
        size += size_of::<u32>();
        size
    }
    #[allow(unused_assignments, unused_variables)]
    #[inline(always)]
    pub unsafe fn try_ref(mut __packet_macro_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr> {
        let __packet_macro_read_len = 0usize;
        let mut f: Option<&u32> = None;
        'tryref: loop {
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u32>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u32 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                f = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let _ref = SizeAlignTest { f: f.unwrap() };
            return Ok((_ref, __packet_macro_bytes));
        }
        Err(WrapErr::NotEnoughBytes(0 + ::std::mem::size_of::<u32>()))
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn fill_vec(&self, mut __packet_macro_bytes: &mut Vec<u8>) {
        __packet_macro_bytes.reserve_exact(self.len());
        let &SizeAlignTest { f } = self;
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u32>();
            let __packet_field_bytes = f as *const u32 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        }
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn len(&self) -> usize {
        let &SizeAlignTest { f } = self;
        0usize + ::std::mem::size_of::<u32>()
    }
}
unsafe impl<'a> FlattenableRef<'a> for SizeAlignTest<'a> {}
#[derive(Copy, Clone)]
pub struct SizeAlignTest<'a> {
    pub f: &'a u64,
}
const _: () = {
    use std::mem::{align_of, size_of};
    let _alignment_check = [()][(0) % align_of::<u64>()];
    let _alignment_check2 = [()][(align_of::<u64>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u64>() < align_of::<u64>()) as u8 as usize];
};
impl<'a> SizeAlignTest<'a> {
    pub const fn required_alignment() -> usize {
        use std::mem::align_of;
        let mut required_alignment = 1;
        let alignment = align_of::<u64>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        required_alignment
    }
    pub const fn max_provided_alignment() -> Option<usize> {
        use std::mem::align_of;
        let mut min_size = Self::min_len();
        let mut min_align: Option<usize> = None;
        let min_align = match min_align {
            None => return None,
            Some(min_align) => min_align,
        };
        if min_size % 8 == 0 && min_align >= 8 {
            return Some(8);
        }
        if min_size % 4 == 0 && min_align >= 4 {
            return Some(4);
        }
        if min_size % 2 == 0 && min_align >= 2 {
            return Some(2);
        }
        return Some(1);
    }
    pub const fn min_len() -> usize {
        use std::mem::size_of;
        let mut size = 0;
        size += size_of::<u64>();
        size
    }
    #[allow(unused_assignments, unused_variables)]
    #[inline(always)]
    pub unsafe fn try_ref(mut __packet_macro_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr> {
        let __packet_macro_read_len = 0usize;
        let mut f: Option<&u64> = None;
        'tryref: loop {
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u64>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u64 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                f = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let _ref = SizeAlignTest { f: f.unwrap() };
            return Ok((_ref, __packet_macro_bytes));
        }
        Err(WrapErr::NotEnoughBytes(0 + ::std::mem::size_of::<u64>()))
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn fill_vec(&self, mut __packet_macro_bytes: &mut Vec<u8>) {
        __packet_macro_bytes.reserve_exact(self.len());
        let &SizeAlignTest { f } = self;
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u64>();
            let __packet_field_bytes = f as *const u64 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        }
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn len(&self) -> usize {
        let &SizeAlignTest { f } = self;
        0usize + ::std::mem::size_of::<u64>()
    }
}
unsafe impl<'a> FlattenableRef<'a> for SizeAlignTest<'a> {}
#[derive(Copy, Clone)]
pub struct SizeAlignTest<'a> {
    pub a: &'a u64,
    pub b: &'a u32,
    pub c: &'a u16,
}
const _: () = {
    use std::mem::{align_of, size_of};
    let _alignment_check = [()][(0) % align_of::<u64>()];
    let _alignment_check2 = [()][(align_of::<u64>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u64>() < align_of::<u64>()) as u8 as usize];
    let _alignment_check = [()][(0 + size_of::<u64>()) % align_of::<u32>()];
    let _alignment_check2 = [()][(align_of::<u32>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u32>() < align_of::<u32>()) as u8 as usize];
    let _alignment_check = [()][(0 + size_of::<u64>() + size_of::<u32>()) % align_of::<u16>()];
    let _alignment_check2 = [()][(align_of::<u16>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u16>() < align_of::<u16>()) as u8 as usize];
};
impl<'a> SizeAlignTest<'a> {
    pub const fn required_alignment() -> usize {
        use std::mem::align_of;
        let mut required_alignment = 1;
        let alignment = align_of::<u64>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = align_of::<u32>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = align_of::<u16>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        required_alignment
    }
    pub const fn max_provided_alignment() -> Option<usize> {
        use std::mem::align_of;
        let mut min_size = Self::min_len();
        let mut min_align: Option<usize> = None;
        let min_align = match min_align {
            None => return None,
            Some(min_align) => min_align,
        };
        if min_size % 8 == 0 && min_align >= 8 {
            return Some(8);
        }
        if min_size % 4 == 0 && min_align >= 4 {
            return Some(4);
        }
        if min_size % 2 == 0 && min_align >= 2 {
            return Some(2);
        }
        return Some(1);
    }
    pub const fn min_len() -> usize {
        use std::mem::size_of;
        let mut size = 0;
        size += size_of::<u64>();
        size += size_of::<u32>();
        size += size_of::<u16>();
        size
    }
    #[allow(unused_assignments, unused_variables)]
    #[inline(always)]
    pub unsafe fn try_ref(mut __packet_macro_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr> {
        let __packet_macro_read_len = 0usize;
        let mut a: Option<&u64> = None;
        let mut b: Option<&u32> = None;
        let mut c: Option<&u16> = None;
        'tryref: loop {
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u64>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u64 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                a = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u32>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u32 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                b = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u16>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u16 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                c = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let _ref = SizeAlignTest {
                a: a.unwrap(),
                b: b.unwrap(),
                c: c.unwrap(),
            };
            return Ok((_ref, __packet_macro_bytes));
        }
        Err(WrapErr::NotEnoughBytes(
            0 + ::std::mem::size_of::<u64>()
                + ::std::mem::size_of::<u32>()
                + ::std::mem::size_of::<u16>(),
        ))
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn fill_vec(&self, mut __packet_macro_bytes: &mut Vec<u8>) {
        __packet_macro_bytes.reserve_exact(self.len());
        let &SizeAlignTest { a, b, c } = self;
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u64>();
            let __packet_field_bytes = a as *const u64 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        };
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u32>();
            let __packet_field_bytes = b as *const u32 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        };
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u16>();
            let __packet_field_bytes = c as *const u16 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        }
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn len(&self) -> usize {
        let &SizeAlignTest { a, b, c } = self;
        0usize
            + ::std::mem::size_of::<u64>()
            + ::std::mem::size_of::<u32>()
            + ::std::mem::size_of::<u16>()
    }
}
unsafe impl<'a> FlattenableRef<'a> for SizeAlignTest<'a> {}
#[derive(Copy, Clone)]
pub struct SizeAlignTest<'a> {
    pub a: &'a u32,
    pub b: &'a u32,
    pub c: &'a u32,
}
const _: () = {
    use std::mem::{align_of, size_of};
    let _alignment_check = [()][(0) % align_of::<u32>()];
    let _alignment_check2 = [()][(align_of::<u32>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u32>() < align_of::<u32>()) as u8 as usize];
    let _alignment_check = [()][(0 + size_of::<u32>()) % align_of::<u32>()];
    let _alignment_check2 = [()][(align_of::<u32>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u32>() < align_of::<u32>()) as u8 as usize];
    let _alignment_check = [()][(0 + size_of::<u32>() + size_of::<u32>()) % align_of::<u32>()];
    let _alignment_check2 = [()][(align_of::<u32>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u32>() < align_of::<u32>()) as u8 as usize];
};
impl<'a> SizeAlignTest<'a> {
    pub const fn required_alignment() -> usize {
        use std::mem::align_of;
        let mut required_alignment = 1;
        let alignment = align_of::<u32>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = align_of::<u32>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = align_of::<u32>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        required_alignment
    }
    pub const fn max_provided_alignment() -> Option<usize> {
        use std::mem::align_of;
        let mut min_size = Self::min_len();
        let mut min_align: Option<usize> = None;
        let min_align = match min_align {
            None => return None,
            Some(min_align) => min_align,
        };
        if min_size % 8 == 0 && min_align >= 8 {
            return Some(8);
        }
        if min_size % 4 == 0 && min_align >= 4 {
            return Some(4);
        }
        if min_size % 2 == 0 && min_align >= 2 {
            return Some(2);
        }
        return Some(1);
    }
    pub const fn min_len() -> usize {
        use std::mem::size_of;
        let mut size = 0;
        size += size_of::<u32>();
        size += size_of::<u32>();
        size += size_of::<u32>();
        size
    }
    #[allow(unused_assignments, unused_variables)]
    #[inline(always)]
    pub unsafe fn try_ref(mut __packet_macro_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr> {
        let __packet_macro_read_len = 0usize;
        let mut a: Option<&u32> = None;
        let mut b: Option<&u32> = None;
        let mut c: Option<&u32> = None;
        'tryref: loop {
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u32>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u32 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                a = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u32>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u32 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                b = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u32>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u32 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                c = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let _ref = SizeAlignTest {
                a: a.unwrap(),
                b: b.unwrap(),
                c: c.unwrap(),
            };
            return Ok((_ref, __packet_macro_bytes));
        }
        Err(WrapErr::NotEnoughBytes(
            0 + ::std::mem::size_of::<u32>()
                + ::std::mem::size_of::<u32>()
                + ::std::mem::size_of::<u32>(),
        ))
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn fill_vec(&self, mut __packet_macro_bytes: &mut Vec<u8>) {
        __packet_macro_bytes.reserve_exact(self.len());
        let &SizeAlignTest { a, b, c } = self;
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u32>();
            let __packet_field_bytes = a as *const u32 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        };
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u32>();
            let __packet_field_bytes = b as *const u32 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        };
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u32>();
            let __packet_field_bytes = c as *const u32 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        }
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn len(&self) -> usize {
        let &SizeAlignTest { a, b, c } = self;
        0usize
            + ::std::mem::size_of::<u32>()
            + ::std::mem::size_of::<u32>()
            + ::std::mem::size_of::<u32>()
    }
}
unsafe impl<'a> FlattenableRef<'a> for SizeAlignTest<'a> {}
#[derive(Copy, Clone)]
pub struct SizeAlignTest<'a> {
    pub a: &'a [u32; 3],
}
const _: () = {
    use std::mem::{align_of, size_of};
    let _alignment_check = [()][(0) % align_of::<[u32; 3]>()];
    let _alignment_check2 = [()][(align_of::<[u32; 3]>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<[u32; 3]>() < align_of::<[u32; 3]>()) as u8 as usize];
};
impl<'a> SizeAlignTest<'a> {
    pub const fn required_alignment() -> usize {
        use std::mem::align_of;
        let mut required_alignment = 1;
        let alignment = align_of::<[u32; 3]>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        required_alignment
    }
    pub const fn max_provided_alignment() -> Option<usize> {
        use std::mem::align_of;
        let mut min_size = Self::min_len();
        let mut min_align: Option<usize> = None;
        let min_align = match min_align {
            None => return None,
            Some(min_align) => min_align,
        };
        if min_size % 8 == 0 && min_align >= 8 {
            return Some(8);
        }
        if min_size % 4 == 0 && min_align >= 4 {
            return Some(4);
        }
        if min_size % 2 == 0 && min_align >= 2 {
            return Some(2);
        }
        return Some(1);
    }
    pub const fn min_len() -> usize {
        use std::mem::size_of;
        let mut size = 0;
        size += size_of::<[u32; 3]>();
        size
    }
    #[allow(unused_assignments, unused_variables)]
    #[inline(always)]
    pub unsafe fn try_ref(mut __packet_macro_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr> {
        let __packet_macro_read_len = 0usize;
        let mut a: Option<&[u32; 3]> = None;
        'tryref: loop {
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<[u32; 3]>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &[u32; 3] =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                a = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let _ref = SizeAlignTest { a: a.unwrap() };
            return Ok((_ref, __packet_macro_bytes));
        }
        Err(WrapErr::NotEnoughBytes(
            0 + ::std::mem::size_of::<[u32; 3]>(),
        ))
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn fill_vec(&self, mut __packet_macro_bytes: &mut Vec<u8>) {
        __packet_macro_bytes.reserve_exact(self.len());
        let &SizeAlignTest { a } = self;
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<[u32; 3]>();
            let __packet_field_bytes = a as *const [u32; 3] as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        }
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn len(&self) -> usize {
        let &SizeAlignTest { a } = self;
        0usize + ::std::mem::size_of::<[u32; 3]>()
    }
}
unsafe impl<'a> FlattenableRef<'a> for SizeAlignTest<'a> {}
#[derive(Copy, Clone)]
pub struct SizeAlignTest<'a> {
    pub a: &'a u32,
    pub b: &'a [u16],
}
const _: () = {
    use std::mem::{align_of, size_of};
    let _alignment_check = [()][(0) % align_of::<u32>()];
    let _alignment_check2 = [()][(align_of::<u32>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u32>() < align_of::<u32>()) as u8 as usize];
    let _alignment_check: () = [()][(0 + size_of::<u32>()) % align_of::<u16>()];
    let _alignment_check2: () = [()][(align_of::<u16>() > 8) as u8 as usize];
    let _padding_check: () = [()][(size_of::<u16>() < align_of::<u16>()) as u8 as usize];
};
impl<'a> SizeAlignTest<'a> {
    pub const fn required_alignment() -> usize {
        use std::mem::align_of;
        let mut required_alignment = 1;
        let alignment = align_of::<u32>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = align_of::<u16>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        required_alignment
    }
    pub const fn max_provided_alignment() -> Option<usize> {
        use std::mem::align_of;
        let mut min_size = Self::min_len();
        let mut min_align: Option<usize> = None;
        match (Some(align_of::<u16>()), min_align) {
            (None, _) => (),
            (Some(align), None) => min_align = Some(align),
            (Some(align), Some(min)) if align < min => min_align = Some(align),
            _ => (),
        }
        let min_align = match min_align {
            None => return None,
            Some(min_align) => min_align,
        };
        if min_size % 8 == 0 && min_align >= 8 {
            return Some(8);
        }
        if min_size % 4 == 0 && min_align >= 4 {
            return Some(4);
        }
        if min_size % 2 == 0 && min_align >= 2 {
            return Some(2);
        }
        return Some(1);
    }
    pub const fn min_len() -> usize {
        use std::mem::size_of;
        let mut size = 0;
        size += size_of::<u32>();
        size += 0;
        size
    }
    #[allow(unused_assignments, unused_variables)]
    #[inline(always)]
    pub unsafe fn try_ref(mut __packet_macro_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr> {
        let __packet_macro_read_len = 0usize;
        let mut a: Option<&u32> = None;
        let mut b: Option<&[u16]> = None;
        'tryref: loop {
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u32>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u32 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                a = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let __packet_macro_read_len: usize = {
                let __packet_macro_count = a.cloned().unwrap() as usize;
                let __packet_macro_size = ::std::mem::size_of::<u16>() * __packet_macro_count;
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field_bytes, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field_ptr = __packet_macro_field_bytes.as_ptr();
                let __packet_macro_field = ::std::slice::from_raw_parts(
                    __packet_macro_field_ptr as *const u16,
                    __packet_macro_count,
                );
                debug_assert_eq!(
                    __packet_macro_field_ptr.offset(__packet_macro_size as isize) as usize,
                    __packet_macro_field
                        .as_ptr()
                        .offset(__packet_macro_count as isize) as usize
                );
                __packet_macro_bytes = __packet_macro_rem_bytes;
                b = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let _ref = SizeAlignTest {
                a: a.unwrap(),
                b: b.unwrap(),
            };
            return Ok((_ref, __packet_macro_bytes));
        }
        Err(WrapErr::NotEnoughBytes(
            0 + ::std::mem::size_of::<u32>()
                + (::std::mem::size_of::<u16>()
                    * ((a.as_ref().map(|c| **c).unwrap_or(0)) as usize)),
        ))
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn fill_vec(&self, mut __packet_macro_bytes: &mut Vec<u8>) {
        __packet_macro_bytes.reserve_exact(self.len());
        let &SizeAlignTest { a, b } = self;
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u32>();
            let __packet_field_bytes = a as *const u32 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        };
        unsafe {
            let __packet_field_count = (*a) as usize;
            let b = &b[..__packet_field_count];
            let __packet_field_size = ::std::mem::size_of::<u16>() * __packet_field_count;
            let __packet_field_field_bytes = b.as_ptr() as *const u8;
            let __packet_field_field_slice =
                ::std::slice::from_raw_parts(__packet_field_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_field_slice)
        }
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn len(&self) -> usize {
        let &SizeAlignTest { a, b } = self;
        0usize + ::std::mem::size_of::<u32>() + (::std::mem::size_of::<u16>() * ((*a) as usize))
    }
}
unsafe impl<'a> FlattenableRef<'a> for SizeAlignTest<'a> {}
#[derive(Copy, Clone)]
pub struct SizeAlignTest<'a> {
    pub a: &'a u32,
    pub b: &'a [u32],
}
const _: () = {
    use std::mem::{align_of, size_of};
    let _alignment_check = [()][(0) % align_of::<u32>()];
    let _alignment_check2 = [()][(align_of::<u32>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u32>() < align_of::<u32>()) as u8 as usize];
    let _alignment_check: () = [()][(0 + size_of::<u32>()) % align_of::<u32>()];
    let _alignment_check2: () = [()][(align_of::<u32>() > 8) as u8 as usize];
    let _padding_check: () = [()][(size_of::<u32>() < align_of::<u32>()) as u8 as usize];
};
impl<'a> SizeAlignTest<'a> {
    pub const fn required_alignment() -> usize {
        use std::mem::align_of;
        let mut required_alignment = 1;
        let alignment = align_of::<u32>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = align_of::<u32>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        required_alignment
    }
    pub const fn max_provided_alignment() -> Option<usize> {
        use std::mem::align_of;
        let mut min_size = Self::min_len();
        let mut min_align: Option<usize> = None;
        match (Some(align_of::<u32>()), min_align) {
            (None, _) => (),
            (Some(align), None) => min_align = Some(align),
            (Some(align), Some(min)) if align < min => min_align = Some(align),
            _ => (),
        }
        let min_align = match min_align {
            None => return None,
            Some(min_align) => min_align,
        };
        if min_size % 8 == 0 && min_align >= 8 {
            return Some(8);
        }
        if min_size % 4 == 0 && min_align >= 4 {
            return Some(4);
        }
        if min_size % 2 == 0 && min_align >= 2 {
            return Some(2);
        }
        return Some(1);
    }
    pub const fn min_len() -> usize {
        use std::mem::size_of;
        let mut size = 0;
        size += size_of::<u32>();
        size += 0;
        size
    }
    #[allow(unused_assignments, unused_variables)]
    #[inline(always)]
    pub unsafe fn try_ref(mut __packet_macro_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr> {
        let __packet_macro_read_len = 0usize;
        let mut a: Option<&u32> = None;
        let mut b: Option<&[u32]> = None;
        'tryref: loop {
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u32>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u32 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                a = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let __packet_macro_read_len: usize = {
                let __packet_macro_count = a.cloned().unwrap() as usize;
                let __packet_macro_size = ::std::mem::size_of::<u32>() * __packet_macro_count;
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field_bytes, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field_ptr = __packet_macro_field_bytes.as_ptr();
                let __packet_macro_field = ::std::slice::from_raw_parts(
                    __packet_macro_field_ptr as *const u32,
                    __packet_macro_count,
                );
                debug_assert_eq!(
                    __packet_macro_field_ptr.offset(__packet_macro_size as isize) as usize,
                    __packet_macro_field
                        .as_ptr()
                        .offset(__packet_macro_count as isize) as usize
                );
                __packet_macro_bytes = __packet_macro_rem_bytes;
                b = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let _ref = SizeAlignTest {
                a: a.unwrap(),
                b: b.unwrap(),
            };
            return Ok((_ref, __packet_macro_bytes));
        }
        Err(WrapErr::NotEnoughBytes(
            0 + ::std::mem::size_of::<u32>()
                + (::std::mem::size_of::<u32>()
                    * ((a.as_ref().map(|c| **c).unwrap_or(0)) as usize)),
        ))
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn fill_vec(&self, mut __packet_macro_bytes: &mut Vec<u8>) {
        __packet_macro_bytes.reserve_exact(self.len());
        let &SizeAlignTest { a, b } = self;
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u32>();
            let __packet_field_bytes = a as *const u32 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        };
        unsafe {
            let __packet_field_count = (*a) as usize;
            let b = &b[..__packet_field_count];
            let __packet_field_size = ::std::mem::size_of::<u32>() * __packet_field_count;
            let __packet_field_field_bytes = b.as_ptr() as *const u8;
            let __packet_field_field_slice =
                ::std::slice::from_raw_parts(__packet_field_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_field_slice)
        }
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn len(&self) -> usize {
        let &SizeAlignTest { a, b } = self;
        0usize + ::std::mem::size_of::<u32>() + (::std::mem::size_of::<u32>() * ((*a) as usize))
    }
}
unsafe impl<'a> FlattenableRef<'a> for SizeAlignTest<'a> {}
#[derive(Copy, Clone)]
pub struct SizeAlignTest<'a> {
    pub a: &'a u32,
    pub b: &'a [u32],
    pub c: &'a u32,
}
const _: () = {
    use std::mem::{align_of, size_of};
    let _alignment_check = [()][(0) % align_of::<u32>()];
    let _alignment_check2 = [()][(align_of::<u32>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u32>() < align_of::<u32>()) as u8 as usize];
    let _alignment_check: () = [()][(0 + size_of::<u32>()) % align_of::<u32>()];
    let _alignment_check2: () = [()][(align_of::<u32>() > 8) as u8 as usize];
    let _padding_check: () = [()][(size_of::<u32>() < align_of::<u32>()) as u8 as usize];
    let _alignment_check = [()][(0 + size_of::<u32>()) % align_of::<u32>()];
    let _alignment_check2 = [()][(align_of::<u32>()
        > if align_of::<u32>() < 8 {
            align_of::<u32>()
        } else {
            8
        }) as u8 as usize];
    let _padding_check = [()][(size_of::<u32>() < align_of::<u32>()) as u8 as usize];
};
impl<'a> SizeAlignTest<'a> {
    pub const fn required_alignment() -> usize {
        use std::mem::align_of;
        let mut required_alignment = 1;
        let alignment = align_of::<u32>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = align_of::<u32>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = align_of::<u32>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        required_alignment
    }
    pub const fn max_provided_alignment() -> Option<usize> {
        use std::mem::align_of;
        let mut min_size = Self::min_len();
        let mut min_align: Option<usize> = None;
        match (Some(align_of::<u32>()), min_align) {
            (None, _) => (),
            (Some(align), None) => min_align = Some(align),
            (Some(align), Some(min)) if align < min => min_align = Some(align),
            _ => (),
        }
        let min_align = match min_align {
            None => return None,
            Some(min_align) => min_align,
        };
        if min_size % 8 == 0 && min_align >= 8 {
            return Some(8);
        }
        if min_size % 4 == 0 && min_align >= 4 {
            return Some(4);
        }
        if min_size % 2 == 0 && min_align >= 2 {
            return Some(2);
        }
        return Some(1);
    }
    pub const fn min_len() -> usize {
        use std::mem::size_of;
        let mut size = 0;
        size += size_of::<u32>();
        size += 0;
        size += size_of::<u32>();
        size
    }
    #[allow(unused_assignments, unused_variables)]
    #[inline(always)]
    pub unsafe fn try_ref(mut __packet_macro_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr> {
        let __packet_macro_read_len = 0usize;
        let mut a: Option<&u32> = None;
        let mut b: Option<&[u32]> = None;
        let mut c: Option<&u32> = None;
        'tryref: loop {
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u32>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u32 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                a = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let __packet_macro_read_len: usize = {
                let __packet_macro_count = a.cloned().unwrap() as usize;
                let __packet_macro_size = ::std::mem::size_of::<u32>() * __packet_macro_count;
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field_bytes, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field_ptr = __packet_macro_field_bytes.as_ptr();
                let __packet_macro_field = ::std::slice::from_raw_parts(
                    __packet_macro_field_ptr as *const u32,
                    __packet_macro_count,
                );
                debug_assert_eq!(
                    __packet_macro_field_ptr.offset(__packet_macro_size as isize) as usize,
                    __packet_macro_field
                        .as_ptr()
                        .offset(__packet_macro_count as isize) as usize
                );
                __packet_macro_bytes = __packet_macro_rem_bytes;
                b = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u32>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u32 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                c = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let _ref = SizeAlignTest {
                a: a.unwrap(),
                b: b.unwrap(),
                c: c.unwrap(),
            };
            return Ok((_ref, __packet_macro_bytes));
        }
        Err(WrapErr::NotEnoughBytes(
            0 + ::std::mem::size_of::<u32>()
                + (::std::mem::size_of::<u32>()
                    * ((a.as_ref().map(|c| **c).unwrap_or(0)) as usize))
                + ::std::mem::size_of::<u32>(),
        ))
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn fill_vec(&self, mut __packet_macro_bytes: &mut Vec<u8>) {
        __packet_macro_bytes.reserve_exact(self.len());
        let &SizeAlignTest { a, b, c } = self;
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u32>();
            let __packet_field_bytes = a as *const u32 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        };
        unsafe {
            let __packet_field_count = (*a) as usize;
            let b = &b[..__packet_field_count];
            let __packet_field_size = ::std::mem::size_of::<u32>() * __packet_field_count;
            let __packet_field_field_bytes = b.as_ptr() as *const u8;
            let __packet_field_field_slice =
                ::std::slice::from_raw_parts(__packet_field_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_field_slice)
        };
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u32>();
            let __packet_field_bytes = c as *const u32 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        }
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn len(&self) -> usize {
        let &SizeAlignTest { a, b, c } = self;
        0usize
            + ::std::mem::size_of::<u32>()
            + (::std::mem::size_of::<u32>() * ((*a) as usize))
            + ::std::mem::size_of::<u32>()
    }
}
unsafe impl<'a> FlattenableRef<'a> for SizeAlignTest<'a> {}
#[derive(Copy, Clone)]
pub struct NestedA<'a> {
    pub a: &'a u32,
    pub b: &'a u16,
}
const _: () = {
    use std::mem::{align_of, size_of};
    let _alignment_check = [()][(0) % align_of::<u32>()];
    let _alignment_check2 = [()][(align_of::<u32>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u32>() < align_of::<u32>()) as u8 as usize];
    let _alignment_check = [()][(0 + size_of::<u32>()) % align_of::<u16>()];
    let _alignment_check2 = [()][(align_of::<u16>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u16>() < align_of::<u16>()) as u8 as usize];
};
impl<'a> NestedA<'a> {
    pub const fn required_alignment() -> usize {
        use std::mem::align_of;
        let mut required_alignment = 1;
        let alignment = align_of::<u32>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = align_of::<u16>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        required_alignment
    }
    pub const fn max_provided_alignment() -> Option<usize> {
        use std::mem::align_of;
        let mut min_size = Self::min_len();
        let mut min_align: Option<usize> = None;
        let min_align = match min_align {
            None => return None,
            Some(min_align) => min_align,
        };
        if min_size % 8 == 0 && min_align >= 8 {
            return Some(8);
        }
        if min_size % 4 == 0 && min_align >= 4 {
            return Some(4);
        }
        if min_size % 2 == 0 && min_align >= 2 {
            return Some(2);
        }
        return Some(1);
    }
    pub const fn min_len() -> usize {
        use std::mem::size_of;
        let mut size = 0;
        size += size_of::<u32>();
        size += size_of::<u16>();
        size
    }
    #[allow(unused_assignments, unused_variables)]
    #[inline(always)]
    pub unsafe fn try_ref(mut __packet_macro_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr> {
        let __packet_macro_read_len = 0usize;
        let mut a: Option<&u32> = None;
        let mut b: Option<&u16> = None;
        'tryref: loop {
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u32>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u32 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                a = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u16>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u16 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                b = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let _ref = NestedA {
                a: a.unwrap(),
                b: b.unwrap(),
            };
            return Ok((_ref, __packet_macro_bytes));
        }
        Err(WrapErr::NotEnoughBytes(
            0 + ::std::mem::size_of::<u32>() + ::std::mem::size_of::<u16>(),
        ))
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn fill_vec(&self, mut __packet_macro_bytes: &mut Vec<u8>) {
        __packet_macro_bytes.reserve_exact(self.len());
        let &NestedA { a, b } = self;
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u32>();
            let __packet_field_bytes = a as *const u32 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        };
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u16>();
            let __packet_field_bytes = b as *const u16 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        }
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn len(&self) -> usize {
        let &NestedA { a, b } = self;
        0usize + ::std::mem::size_of::<u32>() + ::std::mem::size_of::<u16>()
    }
}
unsafe impl<'a> FlattenableRef<'a> for NestedA<'a> {}
#[derive(Copy, Clone)]
pub struct SizeAlignTest<'a> {
    pub a: &'a u32,
    pub b: NestedA<'a>,
}
const _: () = {
    use std::mem::{align_of, size_of};
    let _alignment_check = [()][(0) % align_of::<u32>()];
    let _alignment_check2 = [()][(align_of::<u32>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u32>() < align_of::<u32>()) as u8 as usize];
    let _alignment_check: () = [()][(0 + size_of::<u32>()) % NestedA::required_alignment()];
    let _alignment_check2: () = [()][(NestedA::required_alignment() > 8) as u8 as usize];
};
impl<'a> SizeAlignTest<'a> {
    pub const fn required_alignment() -> usize {
        use std::mem::align_of;
        let mut required_alignment = 1;
        let alignment = align_of::<u32>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = NestedA::required_alignment();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        required_alignment
    }
    pub const fn max_provided_alignment() -> Option<usize> {
        use std::mem::align_of;
        let mut min_size = Self::min_len();
        let mut min_align: Option<usize> = None;
        match (NestedA::max_provided_alignment(), min_align) {
            (None, _) => (),
            (Some(align), None) => min_align = Some(align),
            (Some(align), Some(min)) if align < min => min_align = Some(align),
            _ => (),
        }
        let min_align = match min_align {
            None => return None,
            Some(min_align) => min_align,
        };
        if min_size % 8 == 0 && min_align >= 8 {
            return Some(8);
        }
        if min_size % 4 == 0 && min_align >= 4 {
            return Some(4);
        }
        if min_size % 2 == 0 && min_align >= 2 {
            return Some(2);
        }
        return Some(1);
    }
    pub const fn min_len() -> usize {
        use std::mem::size_of;
        let mut size = 0;
        size += size_of::<u32>();
        size += NestedA::min_len();
        size
    }
    #[allow(unused_assignments, unused_variables)]
    #[inline(always)]
    pub unsafe fn try_ref(mut __packet_macro_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr> {
        let __packet_macro_read_len = 0usize;
        let mut a: Option<&u32> = None;
        let mut b: Option<NestedA<'a>> = None;
        'tryref: loop {
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u32>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u32 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                a = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let __packet_macro_read_len: usize = {
                let __old_packet_macro_bytes_size = __packet_macro_bytes.len();
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    match NestedA::try_ref(__packet_macro_bytes) {
                        Ok((f, b)) => (f, b),
                        Err(WrapErr::InvalidTag(offset)) => {
                            return Err(WrapErr::InvalidTag(__packet_macro_read_len + offset))
                        }
                        Err(..) => break 'tryref,
                    };
                let __packet_macro_size =
                    __old_packet_macro_bytes_size - __packet_macro_rem_bytes.len();
                __packet_macro_bytes = __packet_macro_rem_bytes;
                b = Some(__packet_macro_field);
                __packet_macro_read_len + __packet_macro_size
            };
            let _ref = SizeAlignTest {
                a: a.unwrap(),
                b: b.unwrap(),
            };
            return Ok((_ref, __packet_macro_bytes));
        }
        Err(WrapErr::NotEnoughBytes(
            0 + ::std::mem::size_of::<u32>() + NestedA::min_len(),
        ))
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn fill_vec(&self, mut __packet_macro_bytes: &mut Vec<u8>) {
        __packet_macro_bytes.reserve_exact(self.len());
        let &SizeAlignTest { a, b } = self;
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u32>();
            let __packet_field_bytes = a as *const u32 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        };
        b.fill_vec(__packet_macro_bytes);
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn len(&self) -> usize {
        let &SizeAlignTest { a, b } = self;
        0usize + ::std::mem::size_of::<u32>() + b.len()
    }
}
unsafe impl<'a> FlattenableRef<'a> for SizeAlignTest<'a> {}
#[derive(Copy, Clone)]
pub struct SizeAlignTest<'a> {
    pub a: &'a u64,
    pub b: NestedA<'a>,
}
const _: () = {
    use std::mem::{align_of, size_of};
    let _alignment_check = [()][(0) % align_of::<u64>()];
    let _alignment_check2 = [()][(align_of::<u64>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u64>() < align_of::<u64>()) as u8 as usize];
    let _alignment_check: () = [()][(0 + size_of::<u64>()) % NestedA::required_alignment()];
    let _alignment_check2: () = [()][(NestedA::required_alignment() > 8) as u8 as usize];
};
impl<'a> SizeAlignTest<'a> {
    pub const fn required_alignment() -> usize {
        use std::mem::align_of;
        let mut required_alignment = 1;
        let alignment = align_of::<u64>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = NestedA::required_alignment();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        required_alignment
    }
    pub const fn max_provided_alignment() -> Option<usize> {
        use std::mem::align_of;
        let mut min_size = Self::min_len();
        let mut min_align: Option<usize> = None;
        match (NestedA::max_provided_alignment(), min_align) {
            (None, _) => (),
            (Some(align), None) => min_align = Some(align),
            (Some(align), Some(min)) if align < min => min_align = Some(align),
            _ => (),
        }
        let min_align = match min_align {
            None => return None,
            Some(min_align) => min_align,
        };
        if min_size % 8 == 0 && min_align >= 8 {
            return Some(8);
        }
        if min_size % 4 == 0 && min_align >= 4 {
            return Some(4);
        }
        if min_size % 2 == 0 && min_align >= 2 {
            return Some(2);
        }
        return Some(1);
    }
    pub const fn min_len() -> usize {
        use std::mem::size_of;
        let mut size = 0;
        size += size_of::<u64>();
        size += NestedA::min_len();
        size
    }
    #[allow(unused_assignments, unused_variables)]
    #[inline(always)]
    pub unsafe fn try_ref(mut __packet_macro_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr> {
        let __packet_macro_read_len = 0usize;
        let mut a: Option<&u64> = None;
        let mut b: Option<NestedA<'a>> = None;
        'tryref: loop {
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u64>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u64 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                a = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let __packet_macro_read_len: usize = {
                let __old_packet_macro_bytes_size = __packet_macro_bytes.len();
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    match NestedA::try_ref(__packet_macro_bytes) {
                        Ok((f, b)) => (f, b),
                        Err(WrapErr::InvalidTag(offset)) => {
                            return Err(WrapErr::InvalidTag(__packet_macro_read_len + offset))
                        }
                        Err(..) => break 'tryref,
                    };
                let __packet_macro_size =
                    __old_packet_macro_bytes_size - __packet_macro_rem_bytes.len();
                __packet_macro_bytes = __packet_macro_rem_bytes;
                b = Some(__packet_macro_field);
                __packet_macro_read_len + __packet_macro_size
            };
            let _ref = SizeAlignTest {
                a: a.unwrap(),
                b: b.unwrap(),
            };
            return Ok((_ref, __packet_macro_bytes));
        }
        Err(WrapErr::NotEnoughBytes(
            0 + ::std::mem::size_of::<u64>() + NestedA::min_len(),
        ))
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn fill_vec(&self, mut __packet_macro_bytes: &mut Vec<u8>) {
        __packet_macro_bytes.reserve_exact(self.len());
        let &SizeAlignTest { a, b } = self;
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u64>();
            let __packet_field_bytes = a as *const u64 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        };
        b.fill_vec(__packet_macro_bytes);
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn len(&self) -> usize {
        let &SizeAlignTest { a, b } = self;
        0usize + ::std::mem::size_of::<u64>() + b.len()
    }
}
unsafe impl<'a> FlattenableRef<'a> for SizeAlignTest<'a> {}
#[derive(Copy, Clone)]
pub struct SizeAlignTest<'a> {
    pub a: &'a u64,
    pub b: NestedA<'a>,
    pub c: &'a u8,
}
const _: () = {
    use std::mem::{align_of, size_of};
    let _alignment_check = [()][(0) % align_of::<u64>()];
    let _alignment_check2 = [()][(align_of::<u64>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u64>() < align_of::<u64>()) as u8 as usize];
    let _alignment_check: () = [()][(0 + size_of::<u64>()) % NestedA::required_alignment()];
    let _alignment_check2: () = [()][(NestedA::required_alignment() > 8) as u8 as usize];
    let _alignment_check = [()][(0 + size_of::<u64>() + NestedA::min_len()) % align_of::<u8>()];
    let _alignment_check2 = [()][(align_of::<u8>()
        > match NestedA::max_provided_alignment() {
            Some(align) => {
                if align < 8 {
                    align
                } else {
                    8
                }
            }
            None => 8,
        }) as u8 as usize];
    let _padding_check = [()][(size_of::<u8>() < align_of::<u8>()) as u8 as usize];
};
impl<'a> SizeAlignTest<'a> {
    pub const fn required_alignment() -> usize {
        use std::mem::align_of;
        let mut required_alignment = 1;
        let alignment = align_of::<u64>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = NestedA::required_alignment();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = align_of::<u8>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        required_alignment
    }
    pub const fn max_provided_alignment() -> Option<usize> {
        use std::mem::align_of;
        let mut min_size = Self::min_len();
        let mut min_align: Option<usize> = None;
        match (NestedA::max_provided_alignment(), min_align) {
            (None, _) => (),
            (Some(align), None) => min_align = Some(align),
            (Some(align), Some(min)) if align < min => min_align = Some(align),
            _ => (),
        }
        let min_align = match min_align {
            None => return None,
            Some(min_align) => min_align,
        };
        if min_size % 8 == 0 && min_align >= 8 {
            return Some(8);
        }
        if min_size % 4 == 0 && min_align >= 4 {
            return Some(4);
        }
        if min_size % 2 == 0 && min_align >= 2 {
            return Some(2);
        }
        return Some(1);
    }
    pub const fn min_len() -> usize {
        use std::mem::size_of;
        let mut size = 0;
        size += size_of::<u64>();
        size += NestedA::min_len();
        size += size_of::<u8>();
        size
    }
    #[allow(unused_assignments, unused_variables)]
    #[inline(always)]
    pub unsafe fn try_ref(mut __packet_macro_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr> {
        let __packet_macro_read_len = 0usize;
        let mut a: Option<&u64> = None;
        let mut b: Option<NestedA<'a>> = None;
        let mut c: Option<&u8> = None;
        'tryref: loop {
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u64>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u64 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                a = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let __packet_macro_read_len: usize = {
                let __old_packet_macro_bytes_size = __packet_macro_bytes.len();
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    match NestedA::try_ref(__packet_macro_bytes) {
                        Ok((f, b)) => (f, b),
                        Err(WrapErr::InvalidTag(offset)) => {
                            return Err(WrapErr::InvalidTag(__packet_macro_read_len + offset))
                        }
                        Err(..) => break 'tryref,
                    };
                let __packet_macro_size =
                    __old_packet_macro_bytes_size - __packet_macro_rem_bytes.len();
                __packet_macro_bytes = __packet_macro_rem_bytes;
                b = Some(__packet_macro_field);
                __packet_macro_read_len + __packet_macro_size
            };
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u8>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u8 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                c = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let _ref = SizeAlignTest {
                a: a.unwrap(),
                b: b.unwrap(),
                c: c.unwrap(),
            };
            return Ok((_ref, __packet_macro_bytes));
        }
        Err(WrapErr::NotEnoughBytes(
            0 + ::std::mem::size_of::<u64>() + NestedA::min_len() + ::std::mem::size_of::<u8>(),
        ))
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn fill_vec(&self, mut __packet_macro_bytes: &mut Vec<u8>) {
        __packet_macro_bytes.reserve_exact(self.len());
        let &SizeAlignTest { a, b, c } = self;
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u64>();
            let __packet_field_bytes = a as *const u64 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        };
        b.fill_vec(__packet_macro_bytes);
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u8>();
            let __packet_field_bytes = c as *const u8 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        }
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn len(&self) -> usize {
        let &SizeAlignTest { a, b, c } = self;
        0usize + ::std::mem::size_of::<u64>() + b.len() + ::std::mem::size_of::<u8>()
    }
}
unsafe impl<'a> FlattenableRef<'a> for SizeAlignTest<'a> {}
#[derive(Copy, Clone)]
pub struct SizeAlignTest<'a> {
    pub a: NestedA<'a>,
    pub b: &'a u8,
    pub c: &'a u8,
    pub f: NestedA<'a>,
}
const _: () = {
    use std::mem::{align_of, size_of};
    let _alignment_check: () = [()][(0) % NestedA::required_alignment()];
    let _alignment_check2: () = [()][(NestedA::required_alignment() > 8) as u8 as usize];
    let _alignment_check = [()][(0 + NestedA::min_len()) % align_of::<u8>()];
    let _alignment_check2 = [()][(align_of::<u8>()
        > match NestedA::max_provided_alignment() {
            Some(align) => {
                if align < 8 {
                    align
                } else {
                    8
                }
            }
            None => 8,
        }) as u8 as usize];
    let _padding_check = [()][(size_of::<u8>() < align_of::<u8>()) as u8 as usize];
    let _alignment_check = [()][(0 + NestedA::min_len() + size_of::<u8>()) % align_of::<u8>()];
    let _alignment_check2 = [()][(align_of::<u8>()
        > match NestedA::max_provided_alignment() {
            Some(align) => {
                if align < 8 {
                    align
                } else {
                    8
                }
            }
            None => 8,
        }) as u8 as usize];
    let _padding_check = [()][(size_of::<u8>() < align_of::<u8>()) as u8 as usize];
    let _alignment_check: () = [()][(0 + NestedA::min_len() + size_of::<u8>() + size_of::<u8>())
        % NestedA::required_alignment()];
    let _alignment_check2: () = [()][(NestedA::required_alignment()
        > match NestedA::max_provided_alignment() {
            Some(align) => {
                if align < 8 {
                    align
                } else {
                    8
                }
            }
            None => 8,
        }) as u8 as usize];
};
impl<'a> SizeAlignTest<'a> {
    pub const fn required_alignment() -> usize {
        use std::mem::align_of;
        let mut required_alignment = 1;
        let alignment = NestedA::required_alignment();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = align_of::<u8>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = align_of::<u8>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = NestedA::required_alignment();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        required_alignment
    }
    pub const fn max_provided_alignment() -> Option<usize> {
        use std::mem::align_of;
        let mut min_size = Self::min_len();
        let mut min_align: Option<usize> = None;
        match (NestedA::max_provided_alignment(), min_align) {
            (None, _) => (),
            (Some(align), None) => min_align = Some(align),
            (Some(align), Some(min)) if align < min => min_align = Some(align),
            _ => (),
        }
        match (NestedA::max_provided_alignment(), min_align) {
            (None, _) => (),
            (Some(align), None) => min_align = Some(align),
            (Some(align), Some(min)) if align < min => min_align = Some(align),
            _ => (),
        }
        let min_align = match min_align {
            None => return None,
            Some(min_align) => min_align,
        };
        if min_size % 8 == 0 && min_align >= 8 {
            return Some(8);
        }
        if min_size % 4 == 0 && min_align >= 4 {
            return Some(4);
        }
        if min_size % 2 == 0 && min_align >= 2 {
            return Some(2);
        }
        return Some(1);
    }
    pub const fn min_len() -> usize {
        use std::mem::size_of;
        let mut size = 0;
        size += NestedA::min_len();
        size += size_of::<u8>();
        size += size_of::<u8>();
        size += NestedA::min_len();
        size
    }
    #[allow(unused_assignments, unused_variables)]
    #[inline(always)]
    pub unsafe fn try_ref(mut __packet_macro_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr> {
        let __packet_macro_read_len = 0usize;
        let mut a: Option<NestedA<'a>> = None;
        let mut b: Option<&u8> = None;
        let mut c: Option<&u8> = None;
        let mut f: Option<NestedA<'a>> = None;
        'tryref: loop {
            let __packet_macro_read_len: usize = {
                let __old_packet_macro_bytes_size = __packet_macro_bytes.len();
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    match NestedA::try_ref(__packet_macro_bytes) {
                        Ok((f, b)) => (f, b),
                        Err(WrapErr::InvalidTag(offset)) => {
                            return Err(WrapErr::InvalidTag(__packet_macro_read_len + offset))
                        }
                        Err(..) => break 'tryref,
                    };
                let __packet_macro_size =
                    __old_packet_macro_bytes_size - __packet_macro_rem_bytes.len();
                __packet_macro_bytes = __packet_macro_rem_bytes;
                a = Some(__packet_macro_field);
                __packet_macro_read_len + __packet_macro_size
            };
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u8>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u8 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                b = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u8>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u8 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                c = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let __packet_macro_read_len: usize = {
                let __old_packet_macro_bytes_size = __packet_macro_bytes.len();
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    match NestedA::try_ref(__packet_macro_bytes) {
                        Ok((f, b)) => (f, b),
                        Err(WrapErr::InvalidTag(offset)) => {
                            return Err(WrapErr::InvalidTag(__packet_macro_read_len + offset))
                        }
                        Err(..) => break 'tryref,
                    };
                let __packet_macro_size =
                    __old_packet_macro_bytes_size - __packet_macro_rem_bytes.len();
                __packet_macro_bytes = __packet_macro_rem_bytes;
                f = Some(__packet_macro_field);
                __packet_macro_read_len + __packet_macro_size
            };
            let _ref = SizeAlignTest {
                a: a.unwrap(),
                b: b.unwrap(),
                c: c.unwrap(),
                f: f.unwrap(),
            };
            return Ok((_ref, __packet_macro_bytes));
        }
        Err(WrapErr::NotEnoughBytes(
            0 + NestedA::min_len()
                + ::std::mem::size_of::<u8>()
                + ::std::mem::size_of::<u8>()
                + NestedA::min_len(),
        ))
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn fill_vec(&self, mut __packet_macro_bytes: &mut Vec<u8>) {
        __packet_macro_bytes.reserve_exact(self.len());
        let &SizeAlignTest { a, b, c, f } = self;
        a.fill_vec(__packet_macro_bytes);
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u8>();
            let __packet_field_bytes = b as *const u8 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        };
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u8>();
            let __packet_field_bytes = c as *const u8 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        };
        f.fill_vec(__packet_macro_bytes);
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn len(&self) -> usize {
        let &SizeAlignTest { a, b, c, f } = self;
        0usize + a.len() + ::std::mem::size_of::<u8>() + ::std::mem::size_of::<u8>() + f.len()
    }
}
unsafe impl<'a> FlattenableRef<'a> for SizeAlignTest<'a> {}
#[derive(Copy, Clone)]
pub struct NestedB<'a> {
    pub a: &'a u32,
    pub b: &'a [u16],
}
const _: () = {
    use std::mem::{align_of, size_of};
    let _alignment_check = [()][(0) % align_of::<u32>()];
    let _alignment_check2 = [()][(align_of::<u32>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u32>() < align_of::<u32>()) as u8 as usize];
    let _alignment_check: () = [()][(0 + size_of::<u32>()) % align_of::<u16>()];
    let _alignment_check2: () = [()][(align_of::<u16>() > 8) as u8 as usize];
    let _padding_check: () = [()][(size_of::<u16>() < align_of::<u16>()) as u8 as usize];
};
impl<'a> NestedB<'a> {
    pub const fn required_alignment() -> usize {
        use std::mem::align_of;
        let mut required_alignment = 1;
        let alignment = align_of::<u32>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = align_of::<u16>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        required_alignment
    }
    pub const fn max_provided_alignment() -> Option<usize> {
        use std::mem::align_of;
        let mut min_size = Self::min_len();
        let mut min_align: Option<usize> = None;
        match (Some(align_of::<u16>()), min_align) {
            (None, _) => (),
            (Some(align), None) => min_align = Some(align),
            (Some(align), Some(min)) if align < min => min_align = Some(align),
            _ => (),
        }
        let min_align = match min_align {
            None => return None,
            Some(min_align) => min_align,
        };
        if min_size % 8 == 0 && min_align >= 8 {
            return Some(8);
        }
        if min_size % 4 == 0 && min_align >= 4 {
            return Some(4);
        }
        if min_size % 2 == 0 && min_align >= 2 {
            return Some(2);
        }
        return Some(1);
    }
    pub const fn min_len() -> usize {
        use std::mem::size_of;
        let mut size = 0;
        size += size_of::<u32>();
        size += 0;
        size
    }
    #[allow(unused_assignments, unused_variables)]
    #[inline(always)]
    pub unsafe fn try_ref(mut __packet_macro_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr> {
        let __packet_macro_read_len = 0usize;
        let mut a: Option<&u32> = None;
        let mut b: Option<&[u16]> = None;
        'tryref: loop {
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u32>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u32 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                a = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let __packet_macro_read_len: usize = {
                let __packet_macro_count = a.cloned().unwrap() as usize;
                let __packet_macro_size = ::std::mem::size_of::<u16>() * __packet_macro_count;
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field_bytes, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field_ptr = __packet_macro_field_bytes.as_ptr();
                let __packet_macro_field = ::std::slice::from_raw_parts(
                    __packet_macro_field_ptr as *const u16,
                    __packet_macro_count,
                );
                debug_assert_eq!(
                    __packet_macro_field_ptr.offset(__packet_macro_size as isize) as usize,
                    __packet_macro_field
                        .as_ptr()
                        .offset(__packet_macro_count as isize) as usize
                );
                __packet_macro_bytes = __packet_macro_rem_bytes;
                b = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let _ref = NestedB {
                a: a.unwrap(),
                b: b.unwrap(),
            };
            return Ok((_ref, __packet_macro_bytes));
        }
        Err(WrapErr::NotEnoughBytes(
            0 + ::std::mem::size_of::<u32>()
                + (::std::mem::size_of::<u16>()
                    * ((a.as_ref().map(|c| **c).unwrap_or(0)) as usize)),
        ))
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn fill_vec(&self, mut __packet_macro_bytes: &mut Vec<u8>) {
        __packet_macro_bytes.reserve_exact(self.len());
        let &NestedB { a, b } = self;
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u32>();
            let __packet_field_bytes = a as *const u32 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        };
        unsafe {
            let __packet_field_count = (*a) as usize;
            let b = &b[..__packet_field_count];
            let __packet_field_size = ::std::mem::size_of::<u16>() * __packet_field_count;
            let __packet_field_field_bytes = b.as_ptr() as *const u8;
            let __packet_field_field_slice =
                ::std::slice::from_raw_parts(__packet_field_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_field_slice)
        }
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn len(&self) -> usize {
        let &NestedB { a, b } = self;
        0usize + ::std::mem::size_of::<u32>() + (::std::mem::size_of::<u16>() * ((*a) as usize))
    }
}
unsafe impl<'a> FlattenableRef<'a> for NestedB<'a> {}
#[derive(Copy, Clone)]
pub struct SizeAlignTest<'a> {
    pub a: &'a u32,
    pub b: NestedB<'a>,
}
const _: () = {
    use std::mem::{align_of, size_of};
    let _alignment_check = [()][(0) % align_of::<u32>()];
    let _alignment_check2 = [()][(align_of::<u32>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u32>() < align_of::<u32>()) as u8 as usize];
    let _alignment_check: () = [()][(0 + size_of::<u32>()) % NestedB::required_alignment()];
    let _alignment_check2: () = [()][(NestedB::required_alignment() > 8) as u8 as usize];
};
impl<'a> SizeAlignTest<'a> {
    pub const fn required_alignment() -> usize {
        use std::mem::align_of;
        let mut required_alignment = 1;
        let alignment = align_of::<u32>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = NestedB::required_alignment();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        required_alignment
    }
    pub const fn max_provided_alignment() -> Option<usize> {
        use std::mem::align_of;
        let mut min_size = Self::min_len();
        let mut min_align: Option<usize> = None;
        match (NestedB::max_provided_alignment(), min_align) {
            (None, _) => (),
            (Some(align), None) => min_align = Some(align),
            (Some(align), Some(min)) if align < min => min_align = Some(align),
            _ => (),
        }
        let min_align = match min_align {
            None => return None,
            Some(min_align) => min_align,
        };
        if min_size % 8 == 0 && min_align >= 8 {
            return Some(8);
        }
        if min_size % 4 == 0 && min_align >= 4 {
            return Some(4);
        }
        if min_size % 2 == 0 && min_align >= 2 {
            return Some(2);
        }
        return Some(1);
    }
    pub const fn min_len() -> usize {
        use std::mem::size_of;
        let mut size = 0;
        size += size_of::<u32>();
        size += NestedB::min_len();
        size
    }
    #[allow(unused_assignments, unused_variables)]
    #[inline(always)]
    pub unsafe fn try_ref(mut __packet_macro_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr> {
        let __packet_macro_read_len = 0usize;
        let mut a: Option<&u32> = None;
        let mut b: Option<NestedB<'a>> = None;
        'tryref: loop {
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u32>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u32 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                a = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let __packet_macro_read_len: usize = {
                let __old_packet_macro_bytes_size = __packet_macro_bytes.len();
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    match NestedB::try_ref(__packet_macro_bytes) {
                        Ok((f, b)) => (f, b),
                        Err(WrapErr::InvalidTag(offset)) => {
                            return Err(WrapErr::InvalidTag(__packet_macro_read_len + offset))
                        }
                        Err(..) => break 'tryref,
                    };
                let __packet_macro_size =
                    __old_packet_macro_bytes_size - __packet_macro_rem_bytes.len();
                __packet_macro_bytes = __packet_macro_rem_bytes;
                b = Some(__packet_macro_field);
                __packet_macro_read_len + __packet_macro_size
            };
            let _ref = SizeAlignTest {
                a: a.unwrap(),
                b: b.unwrap(),
            };
            return Ok((_ref, __packet_macro_bytes));
        }
        Err(WrapErr::NotEnoughBytes(
            0 + ::std::mem::size_of::<u32>() + NestedB::min_len(),
        ))
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn fill_vec(&self, mut __packet_macro_bytes: &mut Vec<u8>) {
        __packet_macro_bytes.reserve_exact(self.len());
        let &SizeAlignTest { a, b } = self;
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u32>();
            let __packet_field_bytes = a as *const u32 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        };
        b.fill_vec(__packet_macro_bytes);
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn len(&self) -> usize {
        let &SizeAlignTest { a, b } = self;
        0usize + ::std::mem::size_of::<u32>() + b.len()
    }
}
unsafe impl<'a> FlattenableRef<'a> for SizeAlignTest<'a> {}
#[derive(Copy, Clone)]
pub struct SizeAlignTest<'a> {
    pub a: &'a u64,
    pub b: NestedB<'a>,
}
const _: () = {
    use std::mem::{align_of, size_of};
    let _alignment_check = [()][(0) % align_of::<u64>()];
    let _alignment_check2 = [()][(align_of::<u64>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u64>() < align_of::<u64>()) as u8 as usize];
    let _alignment_check: () = [()][(0 + size_of::<u64>()) % NestedB::required_alignment()];
    let _alignment_check2: () = [()][(NestedB::required_alignment() > 8) as u8 as usize];
};
impl<'a> SizeAlignTest<'a> {
    pub const fn required_alignment() -> usize {
        use std::mem::align_of;
        let mut required_alignment = 1;
        let alignment = align_of::<u64>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = NestedB::required_alignment();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        required_alignment
    }
    pub const fn max_provided_alignment() -> Option<usize> {
        use std::mem::align_of;
        let mut min_size = Self::min_len();
        let mut min_align: Option<usize> = None;
        match (NestedB::max_provided_alignment(), min_align) {
            (None, _) => (),
            (Some(align), None) => min_align = Some(align),
            (Some(align), Some(min)) if align < min => min_align = Some(align),
            _ => (),
        }
        let min_align = match min_align {
            None => return None,
            Some(min_align) => min_align,
        };
        if min_size % 8 == 0 && min_align >= 8 {
            return Some(8);
        }
        if min_size % 4 == 0 && min_align >= 4 {
            return Some(4);
        }
        if min_size % 2 == 0 && min_align >= 2 {
            return Some(2);
        }
        return Some(1);
    }
    pub const fn min_len() -> usize {
        use std::mem::size_of;
        let mut size = 0;
        size += size_of::<u64>();
        size += NestedB::min_len();
        size
    }
    #[allow(unused_assignments, unused_variables)]
    #[inline(always)]
    pub unsafe fn try_ref(mut __packet_macro_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr> {
        let __packet_macro_read_len = 0usize;
        let mut a: Option<&u64> = None;
        let mut b: Option<NestedB<'a>> = None;
        'tryref: loop {
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u64>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u64 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                a = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let __packet_macro_read_len: usize = {
                let __old_packet_macro_bytes_size = __packet_macro_bytes.len();
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    match NestedB::try_ref(__packet_macro_bytes) {
                        Ok((f, b)) => (f, b),
                        Err(WrapErr::InvalidTag(offset)) => {
                            return Err(WrapErr::InvalidTag(__packet_macro_read_len + offset))
                        }
                        Err(..) => break 'tryref,
                    };
                let __packet_macro_size =
                    __old_packet_macro_bytes_size - __packet_macro_rem_bytes.len();
                __packet_macro_bytes = __packet_macro_rem_bytes;
                b = Some(__packet_macro_field);
                __packet_macro_read_len + __packet_macro_size
            };
            let _ref = SizeAlignTest {
                a: a.unwrap(),
                b: b.unwrap(),
            };
            return Ok((_ref, __packet_macro_bytes));
        }
        Err(WrapErr::NotEnoughBytes(
            0 + ::std::mem::size_of::<u64>() + NestedB::min_len(),
        ))
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn fill_vec(&self, mut __packet_macro_bytes: &mut Vec<u8>) {
        __packet_macro_bytes.reserve_exact(self.len());
        let &SizeAlignTest { a, b } = self;
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u64>();
            let __packet_field_bytes = a as *const u64 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        };
        b.fill_vec(__packet_macro_bytes);
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn len(&self) -> usize {
        let &SizeAlignTest { a, b } = self;
        0usize + ::std::mem::size_of::<u64>() + b.len()
    }
}
unsafe impl<'a> FlattenableRef<'a> for SizeAlignTest<'a> {}
#[derive(Copy, Clone)]
pub struct SizeAlignTest<'a> {
    pub a: &'a u64,
    pub b: NestedB<'a>,
    pub c: &'a u8,
}
const _: () = {
    use std::mem::{align_of, size_of};
    let _alignment_check = [()][(0) % align_of::<u64>()];
    let _alignment_check2 = [()][(align_of::<u64>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u64>() < align_of::<u64>()) as u8 as usize];
    let _alignment_check: () = [()][(0 + size_of::<u64>()) % NestedB::required_alignment()];
    let _alignment_check2: () = [()][(NestedB::required_alignment() > 8) as u8 as usize];
    let _alignment_check = [()][(0 + size_of::<u64>() + NestedB::min_len()) % align_of::<u8>()];
    let _alignment_check2 = [()][(align_of::<u8>()
        > match NestedB::max_provided_alignment() {
            Some(align) => {
                if align < 8 {
                    align
                } else {
                    8
                }
            }
            None => 8,
        }) as u8 as usize];
    let _padding_check = [()][(size_of::<u8>() < align_of::<u8>()) as u8 as usize];
};
impl<'a> SizeAlignTest<'a> {
    pub const fn required_alignment() -> usize {
        use std::mem::align_of;
        let mut required_alignment = 1;
        let alignment = align_of::<u64>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = NestedB::required_alignment();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = align_of::<u8>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        required_alignment
    }
    pub const fn max_provided_alignment() -> Option<usize> {
        use std::mem::align_of;
        let mut min_size = Self::min_len();
        let mut min_align: Option<usize> = None;
        match (NestedB::max_provided_alignment(), min_align) {
            (None, _) => (),
            (Some(align), None) => min_align = Some(align),
            (Some(align), Some(min)) if align < min => min_align = Some(align),
            _ => (),
        }
        let min_align = match min_align {
            None => return None,
            Some(min_align) => min_align,
        };
        if min_size % 8 == 0 && min_align >= 8 {
            return Some(8);
        }
        if min_size % 4 == 0 && min_align >= 4 {
            return Some(4);
        }
        if min_size % 2 == 0 && min_align >= 2 {
            return Some(2);
        }
        return Some(1);
    }
    pub const fn min_len() -> usize {
        use std::mem::size_of;
        let mut size = 0;
        size += size_of::<u64>();
        size += NestedB::min_len();
        size += size_of::<u8>();
        size
    }
    #[allow(unused_assignments, unused_variables)]
    #[inline(always)]
    pub unsafe fn try_ref(mut __packet_macro_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr> {
        let __packet_macro_read_len = 0usize;
        let mut a: Option<&u64> = None;
        let mut b: Option<NestedB<'a>> = None;
        let mut c: Option<&u8> = None;
        'tryref: loop {
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u64>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u64 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                a = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let __packet_macro_read_len: usize = {
                let __old_packet_macro_bytes_size = __packet_macro_bytes.len();
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    match NestedB::try_ref(__packet_macro_bytes) {
                        Ok((f, b)) => (f, b),
                        Err(WrapErr::InvalidTag(offset)) => {
                            return Err(WrapErr::InvalidTag(__packet_macro_read_len + offset))
                        }
                        Err(..) => break 'tryref,
                    };
                let __packet_macro_size =
                    __old_packet_macro_bytes_size - __packet_macro_rem_bytes.len();
                __packet_macro_bytes = __packet_macro_rem_bytes;
                b = Some(__packet_macro_field);
                __packet_macro_read_len + __packet_macro_size
            };
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u8>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u8 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                c = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let _ref = SizeAlignTest {
                a: a.unwrap(),
                b: b.unwrap(),
                c: c.unwrap(),
            };
            return Ok((_ref, __packet_macro_bytes));
        }
        Err(WrapErr::NotEnoughBytes(
            0 + ::std::mem::size_of::<u64>() + NestedB::min_len() + ::std::mem::size_of::<u8>(),
        ))
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn fill_vec(&self, mut __packet_macro_bytes: &mut Vec<u8>) {
        __packet_macro_bytes.reserve_exact(self.len());
        let &SizeAlignTest { a, b, c } = self;
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u64>();
            let __packet_field_bytes = a as *const u64 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        };
        b.fill_vec(__packet_macro_bytes);
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u8>();
            let __packet_field_bytes = c as *const u8 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        }
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn len(&self) -> usize {
        let &SizeAlignTest { a, b, c } = self;
        0usize + ::std::mem::size_of::<u64>() + b.len() + ::std::mem::size_of::<u8>()
    }
}
unsafe impl<'a> FlattenableRef<'a> for SizeAlignTest<'a> {}
#[derive(Copy, Clone)]
pub struct SizeAlignTest<'a> {
    pub a: &'a u8,
    pub b: &'a u8,
    pub c: &'a u8,
    pub d: &'a u8,
    pub e: NestedB<'a>,
}
const _: () = {
    use std::mem::{align_of, size_of};
    let _alignment_check = [()][(0) % align_of::<u8>()];
    let _alignment_check2 = [()][(align_of::<u8>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u8>() < align_of::<u8>()) as u8 as usize];
    let _alignment_check = [()][(0 + size_of::<u8>()) % align_of::<u8>()];
    let _alignment_check2 = [()][(align_of::<u8>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u8>() < align_of::<u8>()) as u8 as usize];
    let _alignment_check = [()][(0 + size_of::<u8>() + size_of::<u8>()) % align_of::<u8>()];
    let _alignment_check2 = [()][(align_of::<u8>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u8>() < align_of::<u8>()) as u8 as usize];
    let _alignment_check =
        [()][(0 + size_of::<u8>() + size_of::<u8>() + size_of::<u8>()) % align_of::<u8>()];
    let _alignment_check2 = [()][(align_of::<u8>() > 8) as u8 as usize];
    let _padding_check = [()][(size_of::<u8>() < align_of::<u8>()) as u8 as usize];
    let _alignment_check: () =
        [()][(0 + size_of::<u8>() + size_of::<u8>() + size_of::<u8>() + size_of::<u8>())
            % NestedB::required_alignment()];
    let _alignment_check2: () = [()][(NestedB::required_alignment() > 8) as u8 as usize];
};
impl<'a> SizeAlignTest<'a> {
    pub const fn required_alignment() -> usize {
        use std::mem::align_of;
        let mut required_alignment = 1;
        let alignment = align_of::<u8>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = align_of::<u8>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = align_of::<u8>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = align_of::<u8>();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        let alignment = NestedB::required_alignment();
        if alignment > required_alignment {
            required_alignment = alignment;
        }
        required_alignment
    }
    pub const fn max_provided_alignment() -> Option<usize> {
        use std::mem::align_of;
        let mut min_size = Self::min_len();
        let mut min_align: Option<usize> = None;
        match (NestedB::max_provided_alignment(), min_align) {
            (None, _) => (),
            (Some(align), None) => min_align = Some(align),
            (Some(align), Some(min)) if align < min => min_align = Some(align),
            _ => (),
        }
        let min_align = match min_align {
            None => return None,
            Some(min_align) => min_align,
        };
        if min_size % 8 == 0 && min_align >= 8 {
            return Some(8);
        }
        if min_size % 4 == 0 && min_align >= 4 {
            return Some(4);
        }
        if min_size % 2 == 0 && min_align >= 2 {
            return Some(2);
        }
        return Some(1);
    }
    pub const fn min_len() -> usize {
        use std::mem::size_of;
        let mut size = 0;
        size += size_of::<u8>();
        size += size_of::<u8>();
        size += size_of::<u8>();
        size += size_of::<u8>();
        size += NestedB::min_len();
        size
    }
    #[allow(unused_assignments, unused_variables)]
    #[inline(always)]
    pub unsafe fn try_ref(mut __packet_macro_bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr> {
        let __packet_macro_read_len = 0usize;
        let mut a: Option<&u8> = None;
        let mut b: Option<&u8> = None;
        let mut c: Option<&u8> = None;
        let mut d: Option<&u8> = None;
        let mut e: Option<NestedB<'a>> = None;
        'tryref: loop {
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u8>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u8 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                a = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u8>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u8 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                b = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u8>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u8 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                c = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let __packet_macro_read_len: usize = {
                let __packet_macro_size = ::std::mem::size_of::<u8>();
                let __packet_macro_read_len = __packet_macro_read_len + __packet_macro_size;
                if __packet_macro_bytes.len() < __packet_macro_size {
                    break 'tryref;
                }
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    __packet_macro_bytes.split_at(__packet_macro_size);
                let __packet_macro_field: &u8 =
                    ::std::mem::transmute(__packet_macro_field.as_ptr());
                __packet_macro_bytes = __packet_macro_rem_bytes;
                d = Some(__packet_macro_field);
                __packet_macro_read_len
            };
            let __packet_macro_read_len: usize = {
                let __old_packet_macro_bytes_size = __packet_macro_bytes.len();
                let (__packet_macro_field, __packet_macro_rem_bytes) =
                    match NestedB::try_ref(__packet_macro_bytes) {
                        Ok((f, b)) => (f, b),
                        Err(WrapErr::InvalidTag(offset)) => {
                            return Err(WrapErr::InvalidTag(__packet_macro_read_len + offset))
                        }
                        Err(..) => break 'tryref,
                    };
                let __packet_macro_size =
                    __old_packet_macro_bytes_size - __packet_macro_rem_bytes.len();
                __packet_macro_bytes = __packet_macro_rem_bytes;
                e = Some(__packet_macro_field);
                __packet_macro_read_len + __packet_macro_size
            };
            let _ref = SizeAlignTest {
                a: a.unwrap(),
                b: b.unwrap(),
                c: c.unwrap(),
                d: d.unwrap(),
                e: e.unwrap(),
            };
            return Ok((_ref, __packet_macro_bytes));
        }
        Err(WrapErr::NotEnoughBytes(
            0 + ::std::mem::size_of::<u8>()
                + ::std::mem::size_of::<u8>()
                + ::std::mem::size_of::<u8>()
                + ::std::mem::size_of::<u8>()
                + NestedB::min_len(),
        ))
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn fill_vec(&self, mut __packet_macro_bytes: &mut Vec<u8>) {
        __packet_macro_bytes.reserve_exact(self.len());
        let &SizeAlignTest { a, b, c, d, e } = self;
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u8>();
            let __packet_field_bytes = a as *const u8 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        };
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u8>();
            let __packet_field_bytes = b as *const u8 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        };
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u8>();
            let __packet_field_bytes = c as *const u8 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        };
        unsafe {
            let __packet_field_size = ::std::mem::size_of::<u8>();
            let __packet_field_bytes = d as *const u8 as *const u8;
            let __packet_field_slice =
                ::std::slice::from_raw_parts(__packet_field_bytes, __packet_field_size);
            __packet_macro_bytes.extend_from_slice(__packet_field_slice)
        };
        e.fill_vec(__packet_macro_bytes);
    }
    #[allow(unused_assignments, unused_variables)]
    pub fn len(&self) -> usize {
        let &SizeAlignTest { a, b, c, d, e } = self;
        0usize
            + ::std::mem::size_of::<u8>()
            + ::std::mem::size_of::<u8>()
            + ::std::mem::size_of::<u8>()
            + ::std::mem::size_of::<u8>()
            + e.len()
    }
}
unsafe impl<'a> FlattenableRef<'a> for SizeAlignTest<'a> {}
