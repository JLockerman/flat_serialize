
#[derive(Debug)]
pub enum WrapErr {
    NotEnoughBytes(usize),
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
        let mut output = vec![];
        Basic::Ref{ header, data, data2, array }.fill_vec(&mut output);
        assert_eq!(output, bytes);
    }
}
