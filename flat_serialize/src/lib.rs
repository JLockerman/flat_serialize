#[derive(Debug)]
pub enum WrapErr {
    NotEnoughBytes(usize),
    InvalidTag(usize),
}

// TODO add a Metadata argument to try_ref and type to the trait for more
//      advanced deserialization?
pub trait FlattenableRef<'a>: Sized + 'a {
    unsafe fn try_ref(bytes: &'a [u8]) -> Result<(Self, &'a [u8]), WrapErr>;
    fn fill_vec(&self, vec: &mut Vec<u8>);
    fn len(&self) -> usize;
    fn min_len() -> usize {
        unsafe {
            match Self::try_ref(&[]) {
                Ok(..) => 0,
                Err(WrapErr::NotEnoughBytes(b)) => b,
                _ => unreachable!(),
            }
        }
    }
}

pub unsafe trait FlatSerializable: Sized {
    unsafe fn try_ref<'a>(bytes: &'a [u8]) -> Result<(&'a Self, &'a [u8]), WrapErr> {
        let size = ::std::mem::size_of::<Self>();
        if bytes.len() < size {
            return Err(WrapErr::NotEnoughBytes(size));
        }
        let (field, rem_bytes) = bytes.split_at(size);
        let field: &Self = ::std::mem::transmute(field.as_ptr());
        Ok((field, rem_bytes))
    }
    unsafe fn try_slice<'a>(
        bytes: &'a [u8],
        count: usize,
    ) -> Result<(&'a [Self], &'a [u8]), WrapErr> {
        let size = ::std::mem::size_of::<Self>() * count;
        if bytes.len() < size {
            return Err(WrapErr::NotEnoughBytes(size));
        }
        let (field_bytes, rem_bytes) = bytes.split_at(size);
        let field_ptr = field_bytes.as_ptr();
        let field = ::std::slice::from_raw_parts(field_ptr as *const Self, count);
        debug_assert_eq!(
            field_ptr.offset(size as isize) as usize,
            field.as_ptr().offset(count as isize) as usize
        );
        Ok((field, rem_bytes))
    }
    fn fill_vec(&self, vec: &mut Vec<u8>) {
        unsafe {
            let size = ::std::mem::size_of::<Self>();
            let bytes = self as *const Self as *const u8;
            let slice = ::std::slice::from_raw_parts(bytes, size);
            vec.extend_from_slice(slice)
        }
    }
    fn min_len() -> usize {
        ::std::mem::size_of::<Self>()
    }
}

#[macro_export]
macro_rules! impl_flat_serializable {
    ($($typ:ty)+) => {
        $(
            unsafe impl FlatSerializable for $typ {
                unsafe fn try_ref<'a>(bytes: &'a [u8])
                -> Result<(&'a Self, &'a [u8]), WrapErr> {
                    let size = ::std::mem::size_of::<Self>();
                    if bytes.len() < size {
                        return Err(WrapErr::NotEnoughBytes(size))
                    }
                    let (field, rem_bytes) =
                        bytes.split_at(size);
                    let field: &Self =
                        ::std::mem::transmute(field.as_ptr());
                    Ok((field, rem_bytes))
                }
                fn fill_vec(&self, vec: &mut Vec<u8>) {
                    unsafe {
                        let size = ::std::mem::size_of::<Self>();
                        let bytes = self as *const Self as *const u8;
                        let slice =
                            ::std::slice::from_raw_parts(bytes, size);
                        vec.extend_from_slice(slice)
                    }
                }
                fn min_len() -> usize {
                    ::std::mem::size_of::<Self>()
                }
            }
        )+
    };
}

// impl_flat_serializable!(bool);
// impl_flat_serializable!(i8 u8 i32 u32 i64 u64 i128 u128);
// impl_flat_serializable!(f32 f64);

// TODO holdover blanket impl until const generics are stable and we can
//      implement for all arrays
unsafe impl<T> FlatSerializable for T where T: Sized {}

#[cfg(test)]
mod tests {
    use crate::*;

    use flat_serialize_macro::flat_serialize;

    flat_serialize! {
        #[derive(Debug)]
        struct Basic {
            header: u64,
            data_len: u32,
            array: [u16; 3],
            data: [u8; self.data_len],
            data2: [u8; self.data_len / 2],
        }
    }

    #[test]
    fn basic() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&33u64.to_ne_bytes());
        bytes.extend_from_slice(&6u32.to_ne_bytes());
        bytes.extend_from_slice(&202u16.to_ne_bytes());
        bytes.extend_from_slice(&404u16.to_ne_bytes());
        bytes.extend_from_slice(&555u16.to_ne_bytes());
        bytes.extend_from_slice(&[1, 3, 5, 7, 9, 11]);
        bytes.extend_from_slice(&[4, 4, 4]);
        let (
            Basic {
                header,
                data_len,
                data,
                data2,
                array,
            },
            rem,
        ) = unsafe { Basic::try_ref(&bytes).unwrap() };
        assert_eq!(
            (header, data_len, data, data2, array, rem),
            (
                &33,
                &6,
                &[1, 3, 5, 7, 9, 11][..],
                &[4, 4, 4][..],
                &[202, 404, 555],
                &[][..]
            )
        );

        let (
            Basic {
                header,
                data_len,
                data,
                data2,
                array,
            },
            rem,
        ) = unsafe { <Basic as FlattenableRef>::try_ref(&bytes).unwrap() };
        assert_eq!(
            (header, data_len, data, data2, array, rem),
            (
                &33,
                &6,
                &[1, 3, 5, 7, 9, 11][..],
                &[4, 4, 4][..],
                &[202, 404, 555],
                &[][..]
            )
        );

        let mut output = vec![];
        Basic {
            header,
            data_len,
            data,
            data2,
            array,
        }
        .fill_vec(&mut output);
        assert_eq!(output, bytes);

        let debug = format!(
            "{:?}",
            Basic {
                header,
                data_len,
                data,
                data2,
                array
            }
        );
        assert_eq!(debug, "Basic { header: 33, data_len: 6, array: [202, 404, 555], data: [1, 3, 5, 7, 9, 11], data2: [4, 4, 4] }");
    }

    #[test]
    #[should_panic(expected = "range end index 5 out of range for slice of length 1")]
    fn bad_len1() {
        let mut output = vec![];
        Basic {
            header: &1,
            data_len: &5,
            data: &[1],
            data2: &[2],
            array: &[0; 3],
        }
        .fill_vec(&mut output);
    }

    #[test]
    #[should_panic(expected = "range end index 2 out of range for slice of length 0")]
    fn bad_len2() {
        let mut output = vec![];
        Basic {
            header: &1,
            data_len: &5,
            data: &[1, 2, 3, 4, 5],
            data2: &[],
            array: &[0; 3],
        }
        .fill_vec(&mut output);
    }

    flat_serialize! {
        struct Nested {
            prefix: u64,
            #[flat_serialize::flatten]
            basic: Basic<'a>,
        }
    }

    #[test]
    fn nested() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&101010101u64.to_ne_bytes());
        bytes.extend_from_slice(&33u64.to_ne_bytes());
        bytes.extend_from_slice(&6u32.to_ne_bytes());
        bytes.extend_from_slice(&202u16.to_ne_bytes());
        bytes.extend_from_slice(&404u16.to_ne_bytes());
        bytes.extend_from_slice(&555u16.to_ne_bytes());
        bytes.extend_from_slice(&[1, 3, 5, 7, 9, 11]);
        bytes.extend_from_slice(&[4, 4, 4]);
        let (
            Nested {
                prefix,
                basic:
                    Basic {
                        header,
                        data_len,
                        data,
                        data2,
                        array,
                    },
            },
            rem,
        ) = unsafe { Nested::try_ref(&bytes).unwrap() };
        assert_eq!(
            (prefix, header, data_len, data, data2, array, rem),
            (
                &101010101,
                &33,
                &6,
                &[1, 3, 5, 7, 9, 11][..],
                &[4, 4, 4][..],
                &[202, 404, 555],
                &[][..]
            )
        );

        let (
            Nested {
                prefix,
                basic:
                    Basic {
                        header,
                        data_len,
                        data,
                        data2,
                        array,
                    },
            },
            rem,
        ) = unsafe { <Nested as FlattenableRef>::try_ref(&bytes).unwrap() };
        assert_eq!(
            (prefix, header, data_len, data, data2, array, rem),
            (
                &101010101,
                &33,
                &6,
                &[1, 3, 5, 7, 9, 11][..],
                &[4, 4, 4][..],
                &[202, 404, 555],
                &[][..]
            )
        );

        let mut output = vec![];
        Nested {
            prefix,
            basic: Basic {
                header,
                data_len,
                data,
                data2,
                array,
            },
        }
        .fill_vec(&mut output);
        assert_eq!(output, bytes);
    }

    flat_serialize! {
        enum BasicEnum {
            k: u8,
            First: 2 {
                data_len: usize,
                data: [u8; self.data_len],
            },
            Fixed: 3 {
                array: [u16; 3],
            },
        }
    }

    #[test]
    fn basic_enum1() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&2u8.to_ne_bytes());
        bytes.extend_from_slice(&6usize.to_ne_bytes());
        bytes.extend_from_slice(&[1, 3, 5, 7, 9, 11]);
        let (data_len, data, rem) = match unsafe { BasicEnum::try_ref(&bytes).unwrap() } {
            (BasicEnum::First { data_len, data }, rem) => (data_len, data, rem),
            _ => unreachable!(),
        };
        assert_eq!(
            (data_len, data, rem),
            (&6, &[1, 3, 5, 7, 9, 11][..], &[][..])
        );

        let (data_len, data, rem) =
            match unsafe { <BasicEnum as FlattenableRef>::try_ref(&bytes).unwrap() } {
                (BasicEnum::First { data_len, data }, rem) => (data_len, data, rem),
                _ => unreachable!(),
            };
        assert_eq!(
            (data_len, data, rem),
            (&6, &[1, 3, 5, 7, 9, 11][..], &[][..])
        );

        let mut output = vec![];
        BasicEnum::First { data_len, data }.fill_vec(&mut output);
        assert_eq!(output, bytes);
    }

    #[test]
    fn basic_enum2() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&3u8.to_ne_bytes());
        bytes.extend_from_slice(&3u16.to_ne_bytes());
        bytes.extend_from_slice(&6u16.to_ne_bytes());
        bytes.extend_from_slice(&9u16.to_ne_bytes());
        bytes.extend_from_slice(&[7]);
        let (array, rem) = match unsafe { BasicEnum::try_ref(&bytes).unwrap() } {
            (BasicEnum::Fixed { array }, rem) => (array, rem),
            _ => unreachable!(),
        };
        assert_eq!((array, rem), (&[3, 6, 9], &[7][..]));

        let (array, rem) = match unsafe { BasicEnum::try_ref(&bytes).unwrap() } {
            (BasicEnum::Fixed { array }, rem) => (array, rem),
            _ => unreachable!(),
        };
        assert_eq!((array, rem), (&[3, 6, 9], &[7][..]));

        let (array, rem) = match unsafe { <BasicEnum as FlattenableRef>::try_ref(&bytes).unwrap() }
        {
            (BasicEnum::Fixed { array }, rem) => (array, rem),
            _ => unreachable!(),
        };
        assert_eq!((array, rem), (&[3, 6, 9], &[7][..]));

        let mut output = vec![];
        BasicEnum::Fixed { array }.fill_vec(&mut output);
        assert_eq!(output, &bytes[..bytes.len() - 1]);
    }

    macro_rules! sub_macro {
        (
            $(#[$attrs: meta])?
            struct $name: ident {
                $($field:ident : $typ: tt),*
                $(,)?
            }
        ) => {
            flat_serialize_macro::flat_serialize! {
                $(#[$attrs])?
                struct $name {
                    $($field: $typ),*
                }
            }
        }
    }

    // test that sub_macros provide correct compilation
    sub_macro! {
        #[derive(Debug)]
        struct InMacro {
            a: u32,
            padding: [u8; 4], // with this commented out, the error should be on b
            b: f64,
        }
    }
}
