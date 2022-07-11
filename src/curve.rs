pub fn linear(x: u8, _: u8) -> u8 {
    x
}

/// Return quadratic ease in 'curve' value of `x` going outwards from middle value of `min`/`max` boundary. `0` is the implicit min.
///
/// # Examples
///
/// ```
/// use ds4linux::curve::in_quad;
///
/// assert_eq!(in_quad(75, 100), 62.5);
/// assert_eq!(in_quad(25, 100), 37.5);
///
/// // min & max & mid values remain the same
/// assert_eq!(in_quad(0, 255), 0.0);
/// assert_eq!(in_quad(255, 255), 255.0);
/// assert_eq!(in_quad(50, 100), 50.0);
///
/// ```
pub fn in_quad(x: u8, max: u8) -> f32 {
    let middle = max as f32 / 2.0;
    // Start curvature from mid point, as mid point is center value of axis
    if x as f32 > middle {
        let x = x as f32 - middle;
        let normalized = x / middle;
        ((normalized * normalized) * middle) + middle
    } else {
        let normalized = x as f32 / middle;
        (1.0 - (1.0 - normalized) * (1.0 - normalized)) * middle as f32
    }
}

/// Return quadratic ease out 'curve' value of `x` going outwards from middle value of `min`/`max` boundary. `0` is the implicit min.
///
/// # Examples
///
/// ```
/// use ds4linux::curve::out_quad;
///
/// assert_eq!(out_quad(75, 100), 87.5);
/// assert_eq!(out_quad(25, 100), 12.5);
///
/// // min & max & mid values remain the same
/// assert_eq!(out_quad(0, 255), 0.0);
/// assert_eq!(out_quad(255, 255), 255.0);
/// assert_eq!(out_quad(50, 100), 50.0);
///
/// ```
pub fn out_quad(x: u8, max: u8) -> f32 {
    let middle = max as f32 / 2.0;
    // Start curvature from mid point, as mid point is center value of axis
    if x as f32 > middle {
        let x = x as f32 - middle;
        let normalized = x / middle;
        let in_quad_val = -(normalized * (normalized - 2.0));
        (in_quad_val * middle) + middle
    } else {
        let normalized = x as f32 / middle;
        normalized * normalized * middle
    }
}

/// Return cubic ease in 'curve' value of `x` going outwards from middle value of `min`/`max` boundary. `0` is the implicit min.
///
/// # Examples
///
/// ```
/// use ds4linux::curve::cubic;
///
/// assert_eq!(cubic(75, 100), 56.25);
/// assert_eq!(cubic(25, 100), 43.75);
///
/// // min & max & mid values remain the same
/// assert_eq!(cubic(0, 255), 0.0);
/// assert_eq!(cubic(255, 255), 255.0);
/// assert_eq!(cubic(50, 100), 50.0);
///
/// ```
pub fn cubic(x: u8, max: u8) -> f32 {
    let middle = max as f32 / 2.0;
    // Start curvature from mid point, as mid point is center value of axis
    if x as f32 > middle {
        let x = x as f32 - middle;
        let normalized = x / middle;
        ((normalized * normalized * normalized) * middle) + middle
    } else {
        let normalized = (x as f32 / middle) - 1.0;
        (normalized * normalized * normalized + 1.0) * middle as f32
    }
}

pub fn in_out_quad(x: u8, max: u8) -> f32 {
    let middle = max as f32 / 2.0;
    if f32::abs(x as f32 - middle) < 0.5 * middle {
        if x as f32 > middle {
            // + on axis, ease in
            let x = (x as f32 - middle) / middle;
            ((2.0 * x * x) * middle) + middle
        } else {
            // - on axis, ease in
            let x = x as f32 / middle;
            ((-2.0 * x * x) + (4.0 * x) - 1.0) * middle
        }
    } else {
        if x as f32 > middle {
            // + on axis, ease out
            let x = (x as f32 - middle) / middle;
            ((-2.0 * x * x) + (4.0 * x) - 1.0) * middle + middle
        } else {
            // - on axis, ease out
            let x = x as f32 / middle;
            (2.0 * x * x) * middle
        }
    }
}

/// Get customized curve from `CUSTOM_CURVE_VALS` table
pub fn custom(x: u8, _: u8) -> f32 {
    CUSTOM_CURVE_VALS[x as usize]
}

/// Absolute hackiest way to create curved values. Why do this? Because I suuuuuck.
const CUSTOM_CURVE_VALS: [f32; 256] = [
    0.0,
    1.9921875,
    2.96875,
    3.9296875,
    4.875,
    5.8046875,
    7.71875,
    8.6171875,
    9.5,
    10.3671875,
    11.21875,
    12.0546875,
    13.875,
    14.6796875,
    15.46875,
    16.2421875,
    17.0,
    18.7421875,
    19.46875,
    20.1796875,
    22.875,
    24.5546875,
    26.21875,
    28.8671875,
    30.5,
    32.1171875,
    34.71875,
    36.3046875,
    38.875,
    40.4296875,
    42.96875,
    44.4921875,
    46.0,
    48.4921875,
    50.96875,
    52.4296875,
    54.875,
    56.3046875,
    58.71875,
    60.1171875,
    62.5,
    64.8671875,
    66.21875,
    68.5546875,
    70.875,
    72.1796875,
    74.46875,
    76.7421875,
    78.0,
    79.2421875,
    80.46875,
    81.6796875,
    82.875,
    84.0546875,
    85.21875,
    86.3671875,
    87.5,
    88.6171875,
    89.71875,
    90.8046875,
    91.875,
    92.9296875,
    93.96875,
    94.9921875,
    96.0,
    96.9921875,
    97.96875,
    98.9296875,
    99.875,
    100.8046875,
    101.71875,
    102.6171875,
    103.5,
    104.3671875,
    105.21875,
    106.0546875,
    106.875,
    107.6796875,
    108.46875,
    109.2421875,
    110.0,
    110.7421875,
    111.46875,
    112.1796875,
    112.875,
    113.5546875,
    114.21875,
    114.8671875,
    115.5,
    116.1171875,
    116.71875,
    117.3046875,
    117.875,
    118.4296875,
    118.96875,
    119.4921875,
    120.0,
    120.4921875,
    120.96875,
    121.4296875,
    121.875,
    122.3046875,
    122.71875,
    123.1171875,
    123.5,
    123.8671875,
    124.21875,
    124.5546875,
    124.875,
    125.1796875,
    125.46875,
    125.7421875,
    126.0,
    126.2421875,
    126.46875,
    126.6796875,
    126.875,
    127.0546875,
    127.21875,
    127.3671875,
    127.5,
    127.6171875,
    127.71875,
    127.8046875,
    127.875,
    127.9296875,
    127.96875,
    127.9921875,
    128.0,
    128.0078125,
    128.03125,
    128.0703125,
    128.125,
    128.1953125,
    128.28125,
    128.3828125,
    128.5,
    128.6328125,
    128.78125,
    128.9453125,
    129.125,
    129.3203125,
    129.53125,
    129.7578125,
    130.0,
    130.2578125,
    130.53125,
    130.8203125,
    131.125,
    131.4453125,
    131.78125,
    132.1328125,
    132.5,
    132.8828125,
    133.28125,
    133.6953125,
    134.125,
    134.5703125,
    135.03125,
    135.5078125,
    136.0,
    136.5078125,
    137.03125,
    137.5703125,
    138.125,
    138.6953125,
    139.28125,
    139.8828125,
    140.5,
    141.1328125,
    141.78125,
    142.4453125,
    143.125,
    143.8203125,
    144.53125,
    145.2578125,
    146.0,
    146.7578125,
    147.53125,
    148.3203125,
    149.125,
    149.9453125,
    150.78125,
    151.6328125,
    152.5,
    153.3828125,
    154.28125,
    155.1953125,
    156.125,
    157.0703125,
    158.03125,
    159.0078125,
    160.0,
    161.0078125,
    162.03125,
    163.0703125,
    164.125,
    165.1953125,
    166.28125,
    167.3828125,
    168.5,
    169.6328125,
    170.78125,
    172.9453125,
    174.125,
    176.3203125,
    178.53125,
    180.7578125,
    182.0,
    184.2578125,
    186.53125,
    188.8203125,
    190.125,
    192.4453125,
    194.78125,
    196.1328125,
    198.5,
    200.8828125,
    202.28125,
    204.6953125,
    206.125,
    208.5703125,
    210.03125,
    212.5078125,
    214.0,
    216.5078125,
    218.03125,
    220.5703125,
    222.125,
    224.6953125,
    226.28125,
    228.8828125,
    230.5,
    232.1328125,
    234.78125,
    235.4453125,
    236.125,
    237.8203125,
    238.53125,
    239.0,
    240.2578125,
    241.0,
    242.7578125,
    243.53125,
    244.3203125,
    245.125,
    246.9453125,
    247.78125,
    248.6328125,
    249.3828125,
    250.28125,
    251.1953125,
    252.125,
    253.0,
    254.0,
    255.00,
];
