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
        let e = self.exponent();
        if e >= 0 { self }
        else {
            let i: i64 = m / 10i64.pow((-e) as u32);
            if m >= 0 {
                Decimal64::from(i as i32)
            } else {
                Decimal64::from((i-1) as i32)
            }
        }
    }
    fn ceil(self: Self) -> Self {
        -(-self).floor()
    }
    fn decomp(self: Self) -> (i32, Self) {
        let mut f: Self = Self::from(self.coefficient() as i32);
        let mut e: i32 = self.exponent();
        let ten = Self::from(10);
        let one = Self::from(1);
        while f.abs() >= ten {
            f /= ten; e += 1;
        }
        while f.abs() < one {
            f *= ten; e -= 1;
        }
        (e, f)
    }
    fn nice(self: Self, round: bool) -> Self {
        // adapted from: https://github.com/cenfun/nice-ticks/blob/master/src/index.js
        let (e, m) = self.decomp();
        let (is_neg, m) = if m.is_negative() {(true, -m)} else {(false, m)};
        let nm = if round {
            if m < (Self::from(3)/Self::from(2)) {Self::from(1)}
            else if m < Self::from(3) {Self::from(2)}
            else if m < Self::from(7) {Self::from(5)}
            else                      {Self::from(10)}
        } else {
            if      m <= Self::from(1) {Self::from(1)}
            else if m < Self::from(2)  {Self::from(2)}
            else if m < Self::from(5)  {Self::from(5)}
            else                       {Self::from(10)}
        };
        let out = 
            if e > 0 { nm * Self::from(10i32.pow(e as u32)) }
            else     { nm / Self::from(10i32.pow((-e) as u32 )) };
        if is_neg {-out} else {out}
    }
}

/// Creates nicely rounded min/max from the input
pub fn dec_nice_range(min: Decimal64, max: Decimal64) -> (Decimal64, Decimal64)
{
    let (min,max) = 
        if min == max { (min,min+Decimal64::from(1)) } 
        else if min > max { (max, min) }
        else { (min, max) };
    let r = (max - min).nice(false);
    let d = r.nice(true) / Decimal64::from(20);
    let s = (min / d).floor() * d;
    let e = (max / d).ceil() * d;
    (s, e)
}

/*********************************************** F64Nice ***********************************************/

/// Provides the function `nice` and `compact_str`
pub trait Nice {
    /// Round to a near 'nice' number
    fn nice(self: Self, round: bool) -> Self;
    /// round nth digit
    fn round_to(self: Self, n: u32) -> Self;
    /// Generate a compact string, using the \ notation.
    fn compact_str(self: Self) -> String;
}

impl Nice 