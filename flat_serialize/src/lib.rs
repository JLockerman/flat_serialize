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
pub unsafe trait FlatSerializable: Sized {}

#[macro_export]
macro_rules! impl_flat_serializable {
    ($($typ:ty)+) => {
        $(
            unsafe impl FlatSerializable for $typ {}
        )+
    };
}

impl_flat_serializable!(bool);
impl_flat_serializable!(i8 u8 i16 u16 i32 u32 i64 u64 i128 u128);
impl_flat_serializable!(f32 f64 ordered_float::OrderedFloat<f32> ordered_float::OrderedFloat<f64>);

unsafe impl<T: FlatSerializable, const N: usize> FlatSerializable for [T; N] {}

#[cfg(test)]
mod tests {
    use crate::*;

    use flat_serialize_macro::{flat_serialize, FlatSerializable};

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

        assert_eq!(Basic::min_len(), 18);
        assert_eq!(Basic::required_alignment(), 8);
        assert_eq!(Basic::max_provided_alignment(), Some(1));
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
            (&6, &[1, 3, 5, 7, 9, 11][..], &[][..])
        );

        let mut output = vec![];
        BasicEnum::First { data_len, data }.fill_vec(&mut output);
        assert_eq!(output, bytes);
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
        assert_eq!((array, rem), (&[3, 6, 9], &[7][..]));

        let (array, rem) = match unsafe { BasicEnum::try_ref(&bytes).unwrap() } {
            (BasicEnum::Fixed { array }, rem) => (array, rem),
            _ => unreachable!(),
        };
        assert_eq!((array, rem), (&[3, 6, 9], &[7][..]));

        let mut output = vec![];
        BasicEnum::Fixed { array }.fill_vec(&mut output);
        assert_eq!(output, &bytes[..bytes.len() - 1]);
    }



    flat_serialize! {
        enum PaddedEnum {
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
            (&[0xf, 0xf, 0xf], &6, &[1, 3, 5, 7, 9, 11][..], &[][..])
        );

        let mut output = vec![];
        PaddedEnum::First { padding, data_len, data }.fill_vec(&mut output);
        assert_eq!(output, bytes);
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
        assert_eq!((padding, array, rem), (&0, &[3, 6, 9], &[7][..]));

        let (padding, array, rem) = match unsafe { PaddedEnum::try_ref(&bytes).unwrap() } {
            (PaddedEnum::Fixed {padding, array }, rem) => (padding, array, rem),
            _ => unreachable!(),
        };
        assert_eq!((padding, array, rem), (&0, &[3, 6, 9], &[7][..]));

        let mut output = vec![];
        PaddedEnum::Fixed { padding, array }.fill_vec(&mut output);
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

    macro_rules! check_size_align {
        (struct {
            $( $(#[$attrs: meta])*  $field:ident : $typ: tt $(<$life:lifetime>)?),*
            $(,)?
        }
            len: $min_len: expr,
            align: $required_alignment: expr,
            max: $max_provided_alignment: expr $(,)?
        ) => {
            {
                flat_serialize!{
                    struct SizeAlignTest {
                        $($(#[$attrs])* $field: $typ $(<$life>)?),*
                    }
                };
                assert_eq!(SizeAlignTest::min_len(), $min_len, "length");
                assert_eq!(SizeAlignTest::required_alignment(), $required_alignment, "required");
                assert_eq!(SizeAlignTest::max_provided_alignment(), $max_provided_alignment, "max provided");
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
            struct {
                a: u32,
                b: [u16; self.a],
            }
            len: 4,
            align: 4,
            max: Some(2),
        );

        check_size_align!(
            struct {
                a: u32,
                b: [u32; self.a],
            }
            len: 4,
            align: 4,
            max: Some(4),
        );

        check_size_align!(
            struct {
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
                b: NestedA<'a>,
            }
            len: 4 + (4 + 2),
            align: 4,
            max: None,
        );

        check_size_align!(
            struct {
                a: u64,
                #[flat_serialize::flatten]
                b: NestedA<'a>,
            }
            len: 8 + (4 + 2),
            align: 8,
            max: None,
        );

        check_size_align!(
            struct {
                a: u64,
                #[flat_serialize::flatten]
                b: NestedA<'a>,
                c: u8
            }
            len: 8 + (4 + 2) + 1,
            align: 8,
            max: None,
        );

        check_size_align!(
            struct {
                #[flat_serialize::flatten]
                a: NestedA<'a>,
                b: u8,
                c: u8,
                #[flat_serialize::flatten]
                f: NestedA<'a>,
            }
            len: (4 + 2) + 1 + 1 + (4 + 2),
            align: 4,
            max: None,
        );

        flat_serialize!{
            struct NestedB {
                a: u32,
                b: [u16; self.a],
            }
        }

        check_size_align!(
            struct {
                a: u32,
                #[flat_serialize::flatten]
                b: NestedB<'a>,
            }
            len: 4 + (4),
            align: 4,
            max: Some(2),
        );

        check_size_align!(
            struct {
                a: u64,
                #[flat_serialize::flatten]
                b: NestedB<'a>,
            }
            len: 8 + (4),
            align: 8,
            max: Some(2),
        );

        check_size_align!(
            struct {
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
            struct {
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
                a: EnumA<'a>,
            }
            len: (4 + 2),
            align: 4,
            max: Some(2),
        );

        check_size_align!(
            struct {
                #[flat_serialize::flatten]
                a: EnumA<'a>,
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
                a: EnumA<'a>,
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
                a: EnumB<'a>,
            }
            len: (4 + 2),
            align: 4,
            max: Some(1),
        );

        check_size_align!(
            struct {
                b: u64,
                #[flat_serialize::flatten]
                a: EnumB<'a>,
            }
            len: 8 + (4 + 2),
            align: 8,
            max: Some(1),
        );

        flat_serialize!{
            enum EnumC {
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
            struct {
                #[flat_serialize::flatten]
                a: EnumC<'a>,
            }
            len: (8 + 2),
            align: 8,
            max: Some(2),
        );

        check_size_align!(
            struct {
                #[flat_serialize::flatten]
                a: EnumC<'a>,
                b: u16,
            }
            len: (8 + 2) + 2,
            align: 8,
            max: Some(2),
        );

        check_size_align!(
            struct {
                b: u64,
                #[flat_serialize::flatten]
                a: EnumC<'a>,
            }
            len: 8 + (8 + 2),
            align: 8,
            max: Some(2),
        );

        flat_serialize!{
            enum EnumD {
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
            struct {
                #[flat_serialize::flatten]
                a: EnumD<'a>,
            }
            len: (4 + 2),
            align: 4,
            max: Some(1),
        );

        check_size_align!(
            struct {
                #[flat_serialize::flatten]
                a: EnumD<'a>,
                b: u8,
            }
            len: (4 + 2) + 1,
            align: 4,
            max: Some(1),
        );

        check_size_align!(
            struct {
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
    struct Foo {
        a: i32,
        b: i32,
    }

    const _:() = {
        fn check_flat_serializable_impl<T: FlatSerializable>() {}
        let _ = check_flat_serializable_impl::<Foo>;
        let _ = check_flat_serializable_impl::<[Foo; 2]>;
    };

    #[derive(FlatSerializable)]
    #[allow(dead_code)]
    #[repr(u8)]
    enum Bar {
        A,
        B,
    }

    const _:() = {
        fn check_flat_serializable_impl<T: FlatSerializable>() {}
        let _ = check_flat_serializable_impl::<Bar>;
        let _ = check_flat_serializable_impl::<[Bar; 2]>;
    };
}
