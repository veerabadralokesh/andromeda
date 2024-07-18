static MINUTE_DEGREES: f64 = 6.0;
static HOUR_DEGREES: f64 = 30.0;
static MINUTE_HOUR_DEGREES: f64 = 0.5;
static DEGREES: f64 = 360.0;
impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let (h, m) = (hour as f64, minutes as f64);
        let small_dial = h * HOUR_DEGREES + m * MINUTE_HOUR_DEGREES;
        let large_dial = m * MINUTE_DEGREES;
        let diff = (small_dial - large_dial).abs();
        if diff < 180.0 {
            diff
        } else {
            DEGREES - diff
        }
    }
}

