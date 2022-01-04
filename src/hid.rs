pub struct DS4Hid {
    left: bool,
}

impl From<&[u8; 64]> for DS4Hid {
    fn from(buf: &[u8; 64]) -> Self {
        let left = buf[5] == 0x6;
        Self { left }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_attrs {
        ($object:ident: $($attr:ident $op:tt $value:expr,)*) => {
            $(
                assert!($object.$attr == $value, "expected {:?} == {:?}, but was {:?}", stringify!($object.$attr), $value, $object.$attr);
            )*
        };
    }

    macro_rules! ds4hid_from_buf {
        ($($name:ident: $datastring:expr; $attr:ident $op:tt $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let mut fake_buf: [u8; 64] = [0; 64];
                    hex::decode_to_slice($datastring.replace(" ", ""), &mut fake_buf).unwrap();

                    let hid = DS4Hid::from(&fake_buf);

                    assert_attrs!(hid: $attr $op $value,);

                }
            )*
        };
    }

    ds4hid_from_buf! {
        // left_is_pressed: "01 80 7D 78 83 06 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00", (left == true),
        left_is_pressed: "01 80 7D 78 83 06 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00".to_string(); left == true,
    }

    #[test]
    fn test_DS4Hid_from_buf() {
        // let datastring = ""
        let datastring = "01 80 7D 78 83 06 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00".replace(" ", "");
        let mut fake_buf: [u8; 64] = [0; 64];
        hex::decode_to_slice(datastring, &mut fake_buf).unwrap();

        let hid = DS4Hid::from(&fake_buf);

        assert_attrs!(hid: left == true,);
    }
}
