use std::fmt::Display;

#[derive(Debug, Default)]
pub struct DS4State {
    pub left: bool,
    pub up: bool,
    pub right: bool,
    pub down: bool,
    pub square: bool,
    pub triangle: bool,
    pub circle: bool,
    pub cross: bool,
    pub l1: bool,
    pub l2: bool,
    pub l2_analog: u8,
    pub l3: bool,
    pub r1: bool,
    pub r2: bool,
    pub r2_analog: u8,
    pub r3: bool,
    pub select: bool,
    pub start: bool,
    pub touchpad: bool,
    pub ps: bool,
    pub lsx: u8,
    pub lsy: u8,
    pub rsx: u8,
    pub rsy: u8,
}

impl DS4State {
    pub fn initial_state() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Display for DS4State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "^={} >={} v={} <={} ■={} ▲={} ●={} x={} l1={} r1={} l2={} r2={} l3={} r3={} PS={} TP={} SL={} ST={} LX={:02X} LY={:02X} RX={:02X} RY={:02X}",
            self.up as u8,
            self.right as u8,
            self.down as u8,
            self.left as u8,
            self.square as u8,
            self.triangle as u8,
            self.circle as u8,
            self.cross as u8,
            self.l1 as u8,
            self.r1 as u8,
            self.l2 as u8,
            self.r2 as u8,
            self.l3 as u8,
            self.r3 as u8,
            self.ps as u8,
            self.touchpad as u8,
            self.select as u8,
            self.start as u8,
            self.lsx as u8,
            self.lsy as u8,
            self.rsx as u8,
            self.rsy as u8,
        )
    }
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

        // bumpers/triggers
        let l1 = buf[6] & 0x01 != 0;
        let r1 = buf[6] & 0x02 != 0;
        let l2 = buf[6] & 0x04 != 0;
        let r2 = buf[6] & 0x08 != 0;
        let l3 = buf[6] & 0x40 != 0;
        let r3 = buf[6] & 0x80 != 0;

        // ps button & touchpad press
        let ps = buf[7] & 0x01 != 0;
        let touchpad = buf[7] & 0x02 != 0;

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
            l1,
            r1,
            l2,
            r2,
            l3,
            r3,
            ps,
            touchpad,
            ..Default::default()
        }
    }
}

#[allow(non_snake_case)]
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
        // l1-3 & r1-3 found on byte index 6
        l1_pressed:
            "01 7F 7F 7F FF F0 01 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            l1 == true; r1 == false; l2 == false; r2 == false; l3 == false; r3 == false;,
        r1_pressed:
            "01 7F 7F 7F FF F0 02 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            l1 == false; r1 == true; l2 == false; r2 == false; l3 == false; r3 == false;,
        l2_pressed:
            "01 7F 7F 7F FF F0 04 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            l1 == false; r1 == false; l2 == true; r2 == false; l3 == false; r3 == false;,
        r2_pressed:
            "01 7F 7F 7F FF F0 08 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            l1 == false; r1 == false; l2 == false; r2 == true; l3 == false; r3 == false;,
        l3_pressed:
            "01 7F 7F 7F FF F0 40 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            l1 == false; r1 == false; l2 == false; r2 == false; l3 == true; r3 == false;,
        r3_pressed:
            "01 7F 7F 7F FF F0 80 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            l1 == false; r1 == false; l2 == false; r2 == false; l3 == false; r3 == true;,
        all_lr_buttons_pressed:
            "01 7F 7F 7F FF F0 CF 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            l1 == true; r1 == true; l2 == true; r2 == true; l3 == true; r3 == true;,
        no_lr_buttons_pressed:
            "01 7F 7F 7F FF F0 00 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            l1 == false; r1 == false; l2 == false; r2 == false; l3 == false; r3 == false;,
        // touchpad and ps button press found on byte index 7. If reading the raw data stream it might not seem like it, because they're 'hidden' behind a incrementing timer on the same byte
        ps_button_pressed:
            "01 7F 7F 7F FF F0 80 01 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            ps == true; touchpad == false;,
        touchpad_pressed:
            "01 7F 7F 7F FF F0 80 02 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            ps == false; touchpad == true;,
        ps_button_and_touchpad_pressed:
            "01 7F 7F 7F FF F0 80 03 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            ps == true; touchpad == true;,
        ps_button_and_touchpad_NOT_pressed:
            "01 7F 7F 7F FF F0 80 00 00 00 0D AF FF E9 FF EE FF F2 FF 28 03 23 20 FF FF 00 00 00 00 00 1B 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00 00 00 80 00 00 00 00 80 00";
            ps == false; touchpad == false;,
    }
}
