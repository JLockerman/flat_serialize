
#[derive(Debug)]
pub enum WrapErr {
    NotEnoughBytes(usize),
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
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    use flat_serialize_macro::flat_serialize;

    flat_serialize!{
        struct Basic {
            header: u32,
            data_len: usize,
            data: [u8; self.data_len],
            data2: [u8; self.data_len / 2],
            array: [u16; 3],
        }
    }

    #[test]
    fn basic() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&33u32.to_ne_bytes());
        bytes.extend_from_slice(&6usize.to_ne_bytes());
        bytes.extend_from_slice(&[1, 3, 5, 7, 9, 11]);
        bytes.extend_from_slice(&[4, 4, 4]);
        bytes.extend_from_slice(&202u16.to_ne_bytes());
        bytes.extend_from_slice(&404u16.to_ne_bytes());
        bytes.extend_from_slice(&555u16.to_ne_bytes());
        let (Basic::Ref{ header, data, data2, array }, rem) = unsafe {
            Basic::Ref::try_ref(&bytes).unwrap()
        };
        assert_eq!(
            (header, data, data2, array, rem),
            (&33,
                &[1, 3, 5, 7, 9, 11][..],
                &[4, 4, 4][..],
                &[202, 404, 555],
                &[][..])
        );
        let (Basic::Ref{ header, data, data2, array }, rem) = unsafe {
            Basic::try_ref(&bytes).unwrap()
        };
        assert_eq!(
            (header, data, data2, array, rem),
            (&33,
                &[1, 3, 5, 7, 9, 11][..],
                &[4, 4, 4][..],
                &[202, 404, 555],
                &[][..])
        );
        let (Basic::Ref{ header, data, data2, array }, rem) = unsafe {
            <Basic::Ref as FlattenableRef>::try_ref(&bytes).unwrap()
        };
        assert_eq!(
            (header, data, data2, array, rem),
            (&33,
                &[1, 3, 5, 7, 9, 11][..],
                &[4, 4, 4][..],
                &[202, 404, 555],
                &[][..])
        );
        let mut output = vec![];
        Basic::Ref{ header, data, data2, array }.fill_vec(&mut output);
        assert_eq!(output, bytes);
    }

    flat_serialize!{
        struct nested {
            prefix: u64,
            #[flat_serialize::flatten]
            basic: Basic::Ref<'a>,
        }
    }

    #[test]
    fn nested() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&101010101u64.to_ne_bytes());
        bytes.extend_from_slice(&33u32.to_ne_bytes());
        bytes.extend_from_slice(&6usize.to_ne_bytes());
        bytes.extend_from_slice(&[1, 3, 5, 7, 9, 11]);
        bytes.extend_from_slice(&[4, 4, 4]);
        bytes.extend_from_slice(&202u16.to_ne_bytes());
        bytes.extend_from_slice(&404u16.to_ne_bytes());
        bytes.extend_from_slice(&555u16.to_ne_bytes());
        let (nested::Ref{ prefix, basic: Basic::Ref{ header, data, data2, array }}, rem) = unsafe {
            nested::Ref::try_ref(&bytes).unwrap()
        };
        assert_eq!(
            (prefix, header, data, data2, array, rem),
            (&101010101,
                &33,
                &[1, 3, 5, 7, 9, 11][..],
                &[4, 4, 4][..],
                &[202, 404, 555],
                &[][..])
        );
        let (nested::Ref{ prefix, basic: Basic::Ref{ header, data, data2, array }}, rem) = unsafe {
            nested::try_ref(&bytes).unwrap()
        };
        assert_eq!(
            (prefix, header, data, data2, array, rem),
            (&101010101,
                &33,
                &[1, 3, 5, 7, 9, 11][..],
                &[4, 4, 4][..],
                &[202, 404, 555],
                &[][..])
        );
        let (nested::Ref{ prefix, basic: Basic::Ref{ header, data, data2, array }}, rem) = unsafe {
            <nested::Ref as FlattenableRef>::try_ref(&bytes).unwrap()
        };
        assert_eq!(
            (prefix, header, data, data2, array, rem),
            (&101010101,
                &33,
                &[1, 3, 5, 7, 9, 11][..],
                &[4, 4, 4][..],
                &[202, 404, 555],
                &[][..])
        );
        let mut output = vec![];
        nested::Ref{ prefix, basic: Basic::Ref{ header, data, data2, array }}.fill_vec(&mut output);
        assert_eq!(output, bytes);
    }
}
