/// Return quadratic ease in 'curve' value of `x` going outwards from middle value of `min`/`max` boundary. `0` is the implicit min.
///
/// # Examples
///
/// ```
/// use ds4linux::curve::quad;
///
/// assert_eq!(quad(75, 100), 62.5);
/// assert_eq!(quad(25, 100), 37.5);
///
/// // min & max & mid values remain the same
/// assert_eq!(quad(0, 255), 0.0);
/// assert_eq!(quad(255, 255), 255.0);
/// assert_eq!(quad(50, 100), 50.0);
///
/// ```
pub fn quad(x: u8, max: u8) -> f32 {
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
