///! Some utility function for producing nice y-axis ranges
use dec::Decimal64;


/*********************************************** DecNice ***********************************************/

/// Provides the functions `abs`, `floor`, `ceil`, `decomp` and `nice`
pub trait Dec64Nice {
    /// Return absolute value
    fn abs(self: Self) -> Self;
    /// Return floor (largest integer smaller than input)
    fn floor(self: Self) -> Self;
    /// Return ceiling (smallest integer larger than input)
    fn ceil(self: Self) -> Self;
    /// Return exponent and mantissa such that 1.0 <= mantissa < 10.0
    fn decomp(self: Self) -> (i32, Self);
    /// Round to a near 'nice' number
    fn nice(self: Self, round: bool) -> Self;
}

impl Dec64Nice for Decimal64 {
    fn abs(self: Self) -> Self {
        if self.is_negative() { -self } else { self }
    }
    fn floor(self: Self) -> Self {
        let m = self.coefficient();
        let e = self.exp