
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
        }
    }

    #[test]
    fn basic() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&33u32.to_ne_bytes());
        bytes.extend_from_slice(&6usize.to_ne_bytes());
        bytes.extend_from_slice(&[1, 3, 5, 7, 9, 11]);
        let (Basic::Ref{ header, data }, rem) = unsafe {
            Basic::Ref::try_ref(&bytes).unwrap()
        };
        assert_eq!((header, data, rem), (&33, &[1, 3, 5, 7, 9, 11][..], &[][..]));
        let (Basic::Ref{ header, data }, rem) = unsafe {
            Basic::try_ref(&bytes).unwrap()
        };
        assert_eq!((header, data, rem), (&33, &[1, 3, 5, 7, 9, 11][..], &[][..]));
        let mut output = vec![];
        Basic::Ref{ header, data }.fill_vec(&mut output);
        assert_eq!(output, bytes);
    }
}
