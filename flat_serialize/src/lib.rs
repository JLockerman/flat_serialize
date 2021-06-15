
use std::{mem::{MaybeUninit, align_of, size_of}, slice};

#[derive(Debug)]
pub enum WrapErr {
    NotEnoughBytes(usize),
    InvalidTag(usize),
}

// TODO add a Metadata argument to try_ref and type to the trait for more
//      advanced deserialization?
pub unsafe trait FlattenableRef<'a>: Sized + 'a {}

/// For a type to be `FlatSerializable` it must contain no pointers, have no
/// interior padding, must have a `size >= alignmen` and must have
/// `size % align = 0`. Use `#[derive(FlatSerializable)]` to implement this.
pub unsafe trait FlatSerializable<'input>: Sized + 'input {
    const MIN_LEN: usize;
    const REQUIRED_ALIGNMENT: usize;
    const MAX_PROVIDED_ALIGNMENT: Option<usize>;
    const TRIVIAL_COPY: bool = false;


    unsafe fn try_ref(input: &'input [u8]) -> Result<(Self, &'input [u8]), WrapErr>;
    fn fill_vec(&self, input: &mut Vec<u8>) {
        let start = input.len();
        let my_len = self.len();
        input.reserve(my_len);
        // simulate unstable spare_capacity_mut()
        let slice = unsafe {
            slice::from_raw_parts_mut(
                input.as_mut_ptr().add(input.len()) as *mut MaybeUninit<u8>,
                my_len,
            )
        };
        let rem = unsafe {
            self.fill_slice(slice)
        };
        debug_assert_eq!(rem.len(), 0);
        unsafe {
            input.set_len(start + my_len);
        }
    }
    #[must_use]
    unsafe fn fill_slice<'out>(&self, input: &'out mut [MaybeUninit<u8>])
    -> &'out mut [MaybeUninit<u8>];
    fn len(&self) -> usize;
}

macro_rules! impl_flat_serializable {
    ($($typ:ty)+) => {
        $(
            unsafe impl<'i> FlatSerializable<'i> for $typ {
                const MIN_LEN: usize = size_of::<Self>();
                const REQUIRED_ALIGNMENT: usize = align_of::<Self>();
                const MAX_PROVIDED_ALIGNMENT: Option<usize> = None;
                const TRIVIAL_COPY: bool = true;

                #[inline(always)]
                unsafe fn try_ref(input: &'i [u8])
                -> Result<(Self, &'i [u8]), WrapErr> {
                    let size = size_of::<Self>();
                    if input.len() < size {
                        return Err(WrapErr::NotEnoughBytes(size))
                    }
                    let (field, rem) = input.split_at(size);
                    let field = field.as_ptr().cast::<Self>();
                    Ok((field.read_unaligned(), rem))
                }

                #[inline(always)]
                unsafe fn fill_slice<'out>(&self, input: &'out mut [MaybeUninit<u8>])
                -> &'out mut [MaybeUninit<u8>] {
                    let size = size_of::<Self>();
                    let (input, rem) = input.split_at_mut(size);
                    let bytes = (self as *const Self).cast::<MaybeUninit<u8>>();
                    let bytes = slice::from_raw_parts(bytes, size);
                    // emulate write_slice_cloned()
                    // for i in 0..size {
                    //     input[i] = MaybeUninit::new(bytes[i])
                    // }
                    input.copy_from_slice(bytes);
                    rem
                }

                #[inline(always)]
                fn len(&self) -> usize {
                    size_of::<Self>()
                }
            }
        )+
    };
}

impl_flat_serializable!(bool);
impl_flat_serializable!(i8 u8 i16 u16 i32 u32 i64 u64 i128 u128);
impl_flat_serializable!(f32 f64 ordered_float::OrderedFloat<f32> ordered_float::OrderedFloat<f64>);

// TODO ensure perf
unsafe impl<'i, T, const N: usize> FlatSerializable<'i> for [T; N]
where T: FlatSerializable<'i> + 'i {
    const MIN_LEN: usize = {T::MIN_LEN * N};
    const REQUIRED_ALIGNMENT: usize = T::REQUIRED_ALIGNMENT;
    const MAX_PROVIDED_ALIGNMENT: Option<usize> = T::MAX_PROVIDED_ALIGNMENT;
    const TRIVIAL_COPY: bool = T::TRIVIAL_COPY;

    #[inline(always)]
    unsafe fn try_ref(mut input: &'i [u8])
    -> Result<(Self, &'i [u8]), WrapErr> {
        // TODO can we simplify based on T::TRIVIAL_COPY?
        if T::TRIVIAL_COPY {
            if input.len() < (T::MIN_LEN * N) {
                return Err(WrapErr::NotEnoughBytes(T::MIN_LEN * N))
            }
        }
        let mut output: [MaybeUninit<T>; N] = MaybeUninit::uninit().assume_init();
        for i in 0..N {
            let (val, rem) = T::try_ref(input)?;
            output[i] = MaybeUninit::new(val);
            input = rem;
        }
        let output = (&output as * const [MaybeUninit<T>; N])
            .cast::<[T; N]>().read();
        Ok((output, input))
    }

    #[inline(always)]
    unsafe fn fill_slice<'out>(&self, input: &'out mut [MaybeUninit<u8>])
    -> &'out mut [MaybeUninit<u8>] {
        let size = if Self::TRIVIAL_COPY {
            Self::MIN_LEN
        } else {
            self.len()
        };
        let (mut input, rem) = input.split_at_mut(size);
        input = &mut input[..size];
        // TODO is there a way to force a memcopy for trivial cases?

        for val in self {
            input = val.fill_slice(input);
        }
        debug_assert_eq!(input.len(), 0);
        rem
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.iter().map(T::len).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    use flat_serialize_macro::{flat_serialize, FlatSerializable};

    flat_serialize! {
        #[derive(Debug)]
        struct Basic<'input> {
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
            (header, data_len, array, data, data2, rem),
            (
                33,
                6,
                [202, 404, 555],
                &[1, 3, 5, 7, 9, 11][..],
                &[4, 4, 4][..],
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

        assert_eq!(Basic::MIN_LEN, 18);
        assert_eq!(Basic::REQUIRED_ALIGNMENT, 8);
        assert_eq!(Basic::MAX_PROVIDED_ALIGNMENT, Some(1));
        assert_eq!(Basic::TRIVIAL_COPY, false);

        for i in 0..bytes.len()-1 {
            let res = unsafe {
                Basic::try_ref(&bytes[..i])
            };
            assert!(matches!(res, Err(WrapErr::NotEnoughBytes(..))), "{:?}", res);
        }
    }

    #[test]
    #[should_panic(expected = "range end index 5 out of range for slice of length 1")]
    fn bad_len1() {
        let mut output = vec![];
        Basic {
            header: 1,
            data_len: 5,
            array: [0; 3],
            data: &[1],
            data2: &[2],
        }
        .fill_vec(&mut output);
    }

    #[test]
    #[should_panic(expected = "range end index 2 out of range for slice of length 0")]
    fn bad_len2() {
        let mut output = vec![];
        Basic {
            header: 1,
            data_len: 5,
            array: [0; 3],
            data: &[1, 2, 3, 4, 5],
            data2: &[],
        }
        .fill_vec(&mut output);
    }

    flat_serialize! {
        #[derive(Debug)]
        struct Optional {
            header: u64,
            optional_field: u32 if self.header != 1,
            non_optional_field: u16,
        }
    }

    const _TEST_NO_VARIABLE_LEN_NO_LIFETIME: Optional = Optional {
        header: 0,
        optional_field: None,
        non_optional_field: 0,
    };

    #[test]
    fn optional_present() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&101010101u64.to_ne_bytes());
        bytes.extend_from_slice(&30u32.to_ne_bytes());
        bytes.extend_from_slice(&6u16.to_ne_bytes());
        let (
            Optional {
                header,
                optional_field,
                non_optional_field,
            },
            rem,
        ) = unsafe { Optional::try_ref(&bytes).unwrap() };
        assert_eq!(
            (header, optional_field, non_optional_field, rem),
            (
                101010101,
                Some(30),
                6,
                &[][..]
            )
        );

        let mut output = vec![];
        Optional {
            header,
            optional_field,
            non_optional_field,
        }
        .fill_vec(&mut output);
        assert_eq!(output, bytes);

        for i in 0..bytes.len()-1 {
            let res = unsafe {
                Optional::try_ref(&bytes[..i])
            };
            assert!(matches!(res, Err(WrapErr::NotEnoughBytes(..))), "{:?}", res);
        }
    }

    #[test]
    fn optional_absent() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&1u64.to_ne_bytes());
        bytes.extend_from_slice(&7u16.to_ne_bytes());
        let (
            Optional {
                header,
                optional_field,
                non_optional_field,
            },
            rem,
        ) = unsafe { Optional::try_ref(&bytes).unwrap() };
        assert_eq!(
            (header, optional_field, non_optional_field, rem),
            (
                1,
                None,
                7,
                &[][..]
            )
        );

        let mut output = vec![];
        Optional {
            header,
            optional_field,
            non_optional_field,
        }
        .fill_vec(&mut output);
        assert_eq!(output, bytes);

        for i in 0..bytes.len()-1 {
            let res = unsafe {
                Optional::try_ref(&bytes[..i])
            };
            assert!(matches!(res, Err(WrapErr::NotEnoughBytes(..))), "{:?}", res);
        }
    }

    flat_serialize! {
        #[derive(Debug)]
        struct Nested<'a> {
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
                        array,
                        data,
                        data2,
                    },
            },
            rem,
        ) = unsafe { Nested::try_ref(&bytes).unwrap() };
        assert_eq!(
            (prefix, header, data_len, array, data, data2, rem),
            (
                101010101,
                33,
                6,
                [202, 404, 555],
                &[1, 3, 5, 7, 9, 11][..],
                &[4, 4, 4][..],
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

        for i in 0..bytes.len()-1 {
            let res = unsafe {
                Nested::try_ref(&bytes[..i])
            };
            assert!(matches!(res, Err(WrapErr::NotEnoughBytes(..))), "{:?}", res);
        }
    }

    flat_serialize! {
        #[derive(Debug)]
        enum BasicEnum<'input> {
            k: u64,
            First: 2 {
                data_len: u32,
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
        bytes.extend_from_slice(&2u64.to_ne_bytes());
        bytes.extend_from_slice(&6u32.to_ne_bytes());
        bytes.extend_from_slice(&[1, 3, 5, 7, 9, 11]);
        let (data_len, data, rem) = match unsafe { BasicEnum::try_ref(&bytes).unwrap() } {
            (BasicEnum::First { data_len, data }, rem) => (data_len, data, rem),
            _ => unreachable!(),
        };
        assert_eq!(
            (data_len, data, rem),
            (6, &[1, 3, 5, 7, 9, 11][..], &[][..])
        );

        let mut output = vec![];
        BasicEnum::First { data_len, data }.fill_vec(&mut output);
        assert_eq!(output, bytes);

        for i in 0..bytes.len()-1 {
            let res = unsafe {
                BasicEnum::try_ref(&bytes[..i])
            };
            assert!(matches!(res, Err(WrapErr::NotEnoughBytes(..))), "{:?}", res);
        }
    }

    #[test]
    fn basic_enum2() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&3u64.to_ne_bytes());
        bytes.extend_from_slice(&3u16.to_ne_bytes());
        bytes.extend_from_slice(&6u16.to_ne_bytes());
        bytes.extend_from_slice(&9u16.to_ne_bytes());
        bytes.extend_from_slice(&[7]);
        let (array, rem) = match unsafe { BasicEnum::try_ref(&bytes).unwrap() } {
            (BasicEnum::Fixed { array }, rem) => (array, rem),
            _ => unreachable!(),
        };
        assert_eq!((array, rem), ([3, 6, 9], &[7][..]));

        let (array, rem) = match unsafe { BasicEnum::try_ref(&bytes).unwrap() } {
            (BasicEnum::Fixed { array }, rem) => (array, rem),
            _ => unreachable!(),
        };
        assert_eq!((array, rem), ([3, 6, 9], &[7][..]));

        let mut output = vec![];
        BasicEnum::Fixed { array }.fill_vec(&mut output);
        assert_eq!(output, &bytes[..bytes.len() - 1]);

        for i in 0..bytes.len()-1 {
            let res = unsafe {
                BasicEnum::try_ref(&bytes[..i])
            };
            assert!(matches!(res, Err(WrapErr::NotEnoughBytes(..))), "{:?}", res);
        }
    }



    flat_serialize! {
        #[derive(Debug)]
        enum PaddedEnum<'input> {
            k: u8,
            First: 2 {
                padding: [u8; 3],
                data_len: u32,
                data: [u8; self.data_len],
            },
            Fixed: 3 {
                padding: u8,
                array: [u16; 3],
            },
        }
    }

    #[test]
    fn padded_enum1() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&2u8.to_ne_bytes());
        bytes.extend_from_slice(&[0xf, 0xf, 0xf]);
        bytes.extend_from_slice(&6u32.to_ne_bytes());
        bytes.extend_from_slice(&[1, 3, 5, 7, 9, 11]);
        let (padding, data_len, data, rem) = match unsafe { PaddedEnum::try_ref(&bytes).unwrap() } {
            (PaddedEnum::First {  padding, data_len, data }, rem) => (padding, data_len, data, rem),
            _ => unreachable!(),
        };
        assert_eq!(
            (padding, data_len, data, rem),
            ([0xf, 0xf, 0xf], 6, &[1, 3, 5, 7, 9, 11][..], &[][..])
        );

        let mut output = vec![];
        PaddedEnum::First { padding, data_len, data }.fill_vec(&mut output);
        assert_eq!(output, bytes);

        for i in 0..bytes.len()-1 {
            let res = unsafe {
                PaddedEnum::try_ref(&bytes[..i])
            };
            assert!(matches!(res, Err(WrapErr::NotEnoughBytes(..))), "{:?}", res);
        }
    }

    #[test]
    fn padded_enum2() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&3u8.to_ne_bytes());
        bytes.extend_from_slice(&[0]);
        bytes.extend_from_slice(&3u16.to_ne_bytes());
        bytes.extend_from_slice(&6u16.to_ne_bytes());
        bytes.extend_from_slice(&9u16.to_ne_bytes());
        bytes.extend_from_slice(&[7]);
        let (padding, array, rem) = match unsafe { PaddedEnum::try_ref(&bytes).unwrap() } {
            (PaddedEnum::Fixed { padding, array }, rem) => (padding, array, rem),
            _ => unreachable!(),
        };
        assert_eq!((padding, array, rem), (0, [3, 6, 9], &[7][..]));

        let (padding, array, rem) = match unsafe { PaddedEnum::try_ref(&bytes).unwrap() } {
            (PaddedEnum::Fixed {padding, array }, rem) => (padding, array, rem),
            _ => unreachable!(),
        };
        assert_eq!((padding, array, rem), (0, [3, 6, 9], &[7][..]));

        let mut output = vec![];
        PaddedEnum::Fixed { padding, array }.fill_vec(&mut output);
        assert_eq!(output, &bytes[..bytes.len() - 1]);

        for i in 0..bytes.len()-1 {
            let res = unsafe {
                PaddedEnum::try_ref(&bytes[..i])
            };
            assert!(matches!(res, Err(WrapErr::NotEnoughBytes(..))), "{:?}", res);
        }
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

    #[test]
    fn test_no_refrence() {
        flat_serialize!{
            struct NoLifetime {
                val: i64,
            }
        }

        let _: NoLifetime = NoLifetime{ val: 3 };

        flat_serialize!{
            struct NestedNoLifetime {
                #[flat_serialize::flatten]
                nested: NoLifetime,
            }
        }

        let _: NestedNoLifetime = NestedNoLifetime{ nested: NoLifetime{ val: 3 } };

        flat_serialize!{
            enum ENoLifetime {
                tag: i64,
                Variant: 1 {
                    val: i64,
                },
            }
        }

        let _: ENoLifetime = ENoLifetime::Variant{ val: 2 };

        flat_serialize!{
            enum NestedENoLifetime {
                tag: i64,
                Variant: 2 {
                    #[flat_serialize::flatten]
                    val: ENoLifetime,
                },
            }
        }

        let _: NestedENoLifetime = NestedENoLifetime::Variant{val: ENoLifetime::Variant{ val: 2 }};
    }

    macro_rules! check_size_align {
        (struct $($dec_life:lifetime)? {
            $( $(#[$attrs: meta])*  $field:ident : $typ: tt $(<$life:lifetime>)?),*
            $(,)?
        }
            len: $min_len: expr,
            align: $required_alignment: expr,
            max: $max_provided_alignment: expr $(,)?
        ) => {
            {
                flat_serialize!{
                    struct SizeAlignTest $(<$dec_life>)? {
                        $($(#[$attrs])* $field: $typ $(<$life>)?),*
                    }
                };
                assert_eq!(SizeAlignTest::MIN_LEN, $min_len, "length");
                assert_eq!(SizeAlignTest::REQUIRED_ALIGNMENT, $required_alignment, "required");
                assert_eq!(SizeAlignTest::MAX_PROVIDED_ALIGNMENT, $max_provided_alignment, "max provided");
                assert_eq!(SizeAlignTest::TRIVIAL_COPY, false, "trivial copy");
            }
        }
    }

    #[test]
    fn test_size_align_struct() {
        check_size_align!(
            struct {
                f: u8,
            }
            len: 1,
            align: 1,
            max: None,
        );


        check_size_align!(
            struct {
                f: u16,
            }
            len: 2,
            align: 2,
            max: None,
        );

        check_size_align!(
            struct {
                f: u32,
            }
            len: 4,
            align: 4,
            max: None,
        );

        check_size_align!(
            struct {
                f: u64,
            }
            len: 8,
            align: 8,
            max: None,
        );

        check_size_align!(
            struct {
                a: u64,
                b: u32,
                c: u16,
            }
            len: 8 + 4 + 2,
            align: 8,
            max: None,
        );

        check_size_align!(
            struct {
                a: u32,
                b: u32,
                c: u32,
            }
            len: 4 + 4 + 4,
            align: 4,
            max: None,
        );

        check_size_align!(
            struct {
                a: [u32; 3],
            }
            len: 4 * 3,
            align: 4,
            max: None,
        );

        check_size_align!(
            struct 'a {
                a: u32,
                b: [u16; self.a],
            }
            len: 4,
            align: 4,
            max: Some(2),
        );

        check_size_align!(
            struct 'a {
                a: u32,
                b: [u32; self.a],
            }
            len: 4,
            align: 4,
            max: Some(4),
        );

        check_size_align!(
            struct 'a {
                a: u32,
                b: [u32; self.a],
                c: u32,
            }
            len: 4 + 4,
            align: 4,
            max: Some(4),
        );

        flat_serialize!{
            struct NestedA {
                a: u32,
                b: u16,
            }
        }

        check_size_align!(
            struct {
                a: u32,
                #[flat_serialize::flatten]
                b: NestedA,
            }
            len: 4 + (4 + 2),
            align: 4,
            max: None,
        );

        check_size_align!(
            struct {
                a: u64,
                #[flat_serialize::flatten]
                b: NestedA,
            }
            len: 8 + (4 + 2),
            align: 8,
            max: None,
        );

        check_size_align!(
            struct {
                a: u64,
                #[flat_serialize::flatten]
                b: NestedA,
                c: u8
            }
            len: 8 + (4 + 2) + 1,
            align: 8,
            max: None,
        );

        check_size_align!(
            struct {
                #[flat_serialize::flatten]
                a: NestedA,
                b: u8,
                c: u8,
                #[flat_serialize::flatten]
                f: NestedA,
            }
            len: (4 + 2) + 1 + 1 + (4 + 2),
            align: 4,
            max: None,
        );

        flat_serialize!{
            struct NestedB<'input> {
                a: u32,
                b: [u16; self.a],
            }
        }

        check_size_align!(
            struct 'a {
                a: u32,
                #[flat_serialize::flatten]
                b: NestedB<'a>,
            }
            len: 4 + (4),
            align: 4,
            max: Some(2),
        );

        check_size_align!(
            struct 'a {
                a: u64,
                #[flat_serialize::flatten]
                b: NestedB<'a>,
            }
            len: 8 + (4),
            align: 8,
            max: Some(2),
        );

        check_size_align!(
            struct 'a {
                a: u64,
                #[flat_serialize::flatten]
                b: NestedB<'a>,
                c: u8
            }
            len: 8 + (4) + 1,
            align: 8,
            max: Some(1),
        );

        check_size_align!(
            struct 'a {
                a: u8,
                b: u8,
                c: u8,
                d: u8,
                #[flat_serialize::flatten]
                e: NestedB<'a>,
            }
            len: 4 + (4),
            align: 4,
            max: Some(2),
        );
    }

    #[test]
    fn test_size_align_enum() {

        flat_serialize!{
            enum EnumA {
                tag: u32,
                A: 1 {
                    a: u32,
                },
                B: 2 {
                    a: u16,
                },
            }
        }

        check_size_align!(
            struct {
                #[flat_serialize::flatten]
                a: EnumA,
            }
            len: (4 + 2),
            align: 4,
            max: Some(2),
        );

        check_size_align!(
            struct {
                #[flat_serialize::flatten]
                a: EnumA,
                b: u16,
            }
            len: (4 + 2) + 2,
            align: 4,
            max: Some(2),
        );

        check_size_align!(
            struct {
                b: u64,
                #[flat_serialize::flatten]
                a: EnumA,
            }
            len: 8 + (4 + 2),
            align: 8,
            max: Some(2),
        );

        flat_serialize!{
            enum EnumB {
                tag: u32,
                A: 1 {
                    a: [u8; 5],
                },
                B: 2 {
                    a: u16,
                },
            }
        }

        check_size_align!(
            struct {
                #[flat_serialize::flatten]
                a: EnumB,
            }
            len: (4 + 2),
            align: 4,
            max: Some(1),
        );

        check_size_align!(
            struct {
                b: u64,
                #[flat_serialize::flatten]
                a: EnumB,
            }
            len: 8 + (4 + 2),
            align: 8,
            max: Some(1),
        );

        flat_serialize!{
            enum EnumC<'input> {
                tag: u64,
                A: 1 {
                    a: u64,
                },
                B: 2 {
                    a: u16,
                    b: [u16; self.a],
                },
            }
        }

        check_size_align!(
            struct 'a {
                #[flat_serialize::flatten]
                a: EnumC<'a>,
            }
            len: (8 + 2),
            align: 8,
            max: Some(2),
        );

        check_size_align!(
            struct 'a {
                #[flat_serialize::flatten]
                a: EnumC<'a>,
                b: u16,
            }
            len: (8 + 2) + 2,
            align: 8,
            max: Some(2),
        );

        check_size_align!(
            struct 'a {
                b: u64,
                #[flat_serialize::flatten]
                a: EnumC<'a>,
            }
            len: 8 + (8 + 2),
            align: 8,
            max: Some(2),
        );

        flat_serialize!{
            enum EnumD<'input> {
                tag: u32,
                A: 1 {
                    a: u16,
                },
                B: 2 {
                    a: u32,
                    b: [u8; self.a],
                },
            }
        }

        check_size_align!(
            struct 'a {
                #[flat_serialize::flatten]
                a: EnumD<'a>,
            }
            len: (4 + 2),
            align: 4,
            max: Some(1),
        );

        check_size_align!(
            struct 'a {
                #[flat_serialize::flatten]
                a: EnumD<'a>,
                b: u8,
            }
            len: (4 + 2) + 1,
            align: 4,
            max: Some(1),
        );

        check_size_align!(
            struct 'a {
                b: u64,
                #[flat_serialize::flatten]
                a: EnumD<'a>,
            }
            len: 8 + (4 + 2),
            align: 8,
            max: Some(1),
        );
    }

    #[derive(FlatSerializable)]
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Foo {
        a: i32,
        b: i32,
    }

    const _:() = {
        fn check_flat_serializable_impl<'a, T: FlatSerializable<'a>>() {}
        let _ = check_flat_serializable_impl::<Foo>;
        let _ = check_flat_serializable_impl::<[Foo; 2]>;
    };

    #[test]
    fn foo() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&33i32.to_ne_bytes());
        bytes.extend_from_slice(&100000001i32.to_ne_bytes());

        let (Foo {a, b}, rem) = unsafe {
            Foo::try_ref(&bytes).unwrap()
        };
        assert_eq!(
            (a, b, rem),
            (33, 100000001, &[][..]),
        );

        let mut output = vec![];
        Foo { a, b }.fill_vec(&mut output);
        assert_eq!(output, bytes);

        assert_eq!(Foo::MIN_LEN, 8);
        assert_eq!(Foo::REQUIRED_ALIGNMENT, 4);
        assert_eq!(Foo::MAX_PROVIDED_ALIGNMENT, None);
        assert_eq!(Foo::TRIVIAL_COPY, true);

        for i in 0..bytes.len()-1 {
            let res = unsafe {
                Foo::try_ref(&bytes[..i])
            };
            assert!(matches!(res, Err(WrapErr::NotEnoughBytes(..))), "{:?}", res);
        }
    }

    #[derive(FlatSerializable)]
    #[allow(dead_code)]
    #[repr(u16)]
    #[derive(Debug, Copy, Clone)]
    enum Bar {
        A = 0,
        B = 1111,
    }

    const _:() = {
        fn check_flat_serializable_impl<'a, T: FlatSerializable<'a>>() {}
        let _ = check_flat_serializable_impl::<Bar>;
        let _ = check_flat_serializable_impl::<[Bar; 2]>;
    };

    #[test]
    fn fs_enum_a() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&0u16.to_ne_bytes());

        let (
            val,
            rem,
        ) = unsafe { Bar::try_ref(&bytes).unwrap() };
        assert_eq!(
            (val as u16, rem),
            (Bar::A as u16, &[][..])
        );

        let mut output = vec![];
        val.fill_vec(&mut output);
        assert_eq!(output, bytes);

        assert_eq!(Bar::MIN_LEN, 2);
        assert_eq!(Bar::REQUIRED_ALIGNMENT, 2);
        assert_eq!(Bar::MAX_PROVIDED_ALIGNMENT, None);
        assert_eq!(Bar::TRIVIAL_COPY, true);

        for i in 0..bytes.len()-1 {
            let res = unsafe {
                Bar::try_ref(&bytes[..i])
            };
            assert!(matches!(res, Err(WrapErr::NotEnoughBytes(..))), "{:?}", res);
        }
    }

    #[test]
    fn fs_enum_b() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&1111u16.to_ne_bytes());

        let (
            val,
            rem,
        ) = unsafe { Bar::try_ref(&bytes).unwrap() };
        assert_eq!(
            (val as u16, rem),
            (Bar::B as u16, &[][..])
        );

        let mut output = vec![];
        val.fill_vec(&mut output);
        assert_eq!(output, bytes);

        for i in 0..bytes.len()-1 {
            let res = unsafe {
                Bar::try_ref(&bytes[..i])
            };
            assert!(matches!(res, Err(WrapErr::NotEnoughBytes(..))), "{:?}", res);
        }
    }

    #[test]
    fn fs_enum_non() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&1u16.to_ne_bytes());

        let res= unsafe { Bar::try_ref(&bytes) };
        assert!(matches!(res, Err(WrapErr::InvalidTag(0))));
    }
}
