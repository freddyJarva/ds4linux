#[derive(Debug, Default)]
pub struct DS4State {
    left: bool,
    up: bool,
    right: bool,
    down: bool,
    square: bool,
    triangle: bool,
    circle: bool,
    cross: bool,
    l1: bool,
    l2: bool,
    l2_analog: u8,
    r1: bool,
    r2: bool,
    r2_analog: u8,
    select: bool,
    start: bool,
    touchpad: bool,
    lsx: u8,
    lsy: u8,
    rsx: u8,
    rsy: u8,
}

impl From<&[u8; 64]> for DS4State {
    fn from(buf: &[u8; 64]) -> Self {
        // Analog Sticks
        let lsx = buf[1];
        let lsy = buf[2];
        let rsx = buf[3];
        let rsy = buf[4];

        // dpad
        let mut up = false;
        let mut right = false;
        let mut down = false;
        let mut left = false;
        match buf[5] & 0x0f {
            0 => up = true,
            2 => right = true,
            4 => down = true,
            6 => left = true,
            1 => {
                up = true;
                right = true
            }
            3 => {
                right = true;
                down = true
            }
            5 => {
                down = true;
                left = true
            }
            7 => {
                left = true;
                up = true
            }
            _ => (),
        }
        // Face buttons
        let square = buf[5] & 0x10 != 0;
        let cross = buf[5] & 0x20 != 0;
        let circle = buf[5] & 0x40 != 0;
        let triangle = buf[5] & 0x80 != 0;

        Self {
            left,
            up,
            right,
            down,
            square,
            cross,
            circle,
            triangle,
            lsx,
            lsy,
            rsx,
            rsy,
            ..Default::default()
        }
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
        ($($name:ident: $datastring:expr; $($attr:ident $op:tt $value:expr;)*,)*) => {
            $(
                #[test]
                fn $name() {
                    let mut fake_buf: [u8; 64] = [0; 64];
                    hex::decode_to_slice($datastring.replace(" ", ""), &mut fake_buf).unwrap();

                    let hid = DS4State::from(&fake_buf);

                    $(
                        assert_attrs!(hid: $attr $op $value,);
                    )*
                }
            )*
        };
    }

    ds4hid_from_buf! {
        // dpad values are represented in the low 4 bits of byte index 5
        left_is_pressed:
            "01 80 7D 78 83 06 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            left == true;,
        up_is_pressed:
            "01 80 7D 78 83 00 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            up = true;,
        right_is_pressed:
            "01 80 7D 78 83 02 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            right = true;,
        down_is_pressed:
            "01 80 7D 78 83 04 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            down = true;,
        up_right_is_pressed:
            "01 80 7D 78 83 01 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            up = true; right = true; down = false; left = false;,
        right_down_is_pressed:
            "01 80 7D 78 83 03 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            up = false; right = true; down = true; left = false;,
        down_left_is_pressed:
            "01 80 7D 78 83 05 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            up = false; right = false; down = true; left = true;,
        left_up_is_pressed:
            "01 80 7D 78 83 07 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            up = true; right = false; down = false; left = true;,
        dpad_not_pressed:
            "01 80 7D 78 83 08 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            up = false; right = false; down = false; left = false;,
        // face button values are represented in the high 4 bits of byte index 5
        square_is_pressed:
            "01 80 7D 78 83 10 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            square == true; cross == false; circle == false; triangle == false;,
        cross_is_pressed:
            "01 80 7D 78 83 20 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            square == false; cross == true; circle == false; triangle == false;,
        circle_is_pressed:
            "01 80 7D 78 83 42 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            square == false; cross == false; circle == true; triangle == false;,
        triangle_is_pressed:
            "01 80 7D 78 83 84 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            square == false; cross == false; circle == false; triangle == true;,
        square_cross_is_pressed:
            "01 80 7D 78 83 31 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            square == true; cross == true; circle == false; triangle == false;,
        cross_circle_is_pressed:
            "01 80 7D 78 83 60 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            square == false; cross == true; circle == true; triangle == false;,
        circle_triangle_is_pressed:
            "01 80 7D 78 83 C0 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            square == false; cross == false; circle == true; triangle == true;,
        triangle_square_is_pressed:
            "01 80 7D 78 83 97 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            square == true; cross == false; circle == false; triangle == true;,
        face_buttons_not_pressed:
            "01 80 7D 78 83 00 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            square == false; cross == false; circle == false; triangle == false;,
        all_face_buttons_pressed:
            "01 80 7D 78 83 F0 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            square == true; cross == true; circle == true; triangle == true;,
        // analog stick values found on byte indexes 1-4, in order : lsx, lsy, rsx, rsy
        left_stick_full_left:
            "01 00 7D 78 83 F0 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            lsx == 0x0;,
        left_stick_full_right:
            "01 FF 7D 78 83 F0 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            lsx == 0xFF;,
        left_stick_full_up:
            "01 7F 00 78 83 F0 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            lsy == 0x0;,
        left_stick_full_down:
            "01 7F FF 78 83 F0 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            lsy == 0xFF;,
        right_stick_full_left:
            "01 7F 00 00 83 F0 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            rsx == 0x00;,
        right_stick_full_right:
            "01 7F FF FF 83 F0 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            rsx == 0xFF;,
        right_stick_full_up:
            "01 7F 7F 7F 00 F0 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            rsy == 0x00;,
        right_stick_full_down:
            "01 7F 7F 7F FF F0 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            rsy == 0xFF;,
    }
}
