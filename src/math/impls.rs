
/*

Sums, products, differences
gradient(f, *varargs, **kwargs)	Return the gradient of an N-dimensional array.
cross(a, b[, axisa, axisb, axisc, axis])	Return the cross product of two (arrays of) vectors.
trapz(y[, x, dx, axis])	Integrate along the given axis using the composite trapezoidal rule.

Exponents and logarithms
exp(x[, out])	Calculate the exponential of all elements in the input array.
expm1(x[, out])	Calculate exp(x) - 1 for all elements in the array.
exp2(x[, out])	Calculate 2**p for all p in the input array.
log(x[, out])	Natural logarithm, element-wise.
log10(x[, out])	Return the base 10 logarithm of the input array, element-wise.
log2(x[, out])	Base-2 logarithm of x.
log1p(x[, out])	Return the natural logarithm of one plus the input array, element-wise.
logaddexp(x1, x2[, out])	Logarithm of the sum of exponentiations of the inputs.
logaddexp2(x1, x2[, out])	Logarithm of the sum of exponentiations of the inputs in base-2.

Other special functions
i0(x)	Modified Bessel function of the first kind, order 0.
sinc(x)	Return the sinc function.

Floating point routines
signbit(x[, out])	Returns element-wise True where signbit is set (less than zero).
copysign(x1, x2[, out])	Change the sign of x1 to that of x2, element-wise.
frexp(x[, out1, out2])	Decompose the elements of x into mantissa and twos exponent.
ldexp(x1, x2[, out])	Returns x1 * 2**x2, element-wise.

Arithmetic operations
add(x1, x2[, out])	Add arguments element-wise.
reciprocal(x[, out])	Return the reciprocal of the argument, element-wise.
negative(x[, out])	Numerical negative, element-wise.
multiply(x1, x2[, out])	Multiply arguments element-wise.
divide(x1, x2[, out])	Divide arguments element-wise.
power(x1, x2[, out])	First array elements raised to powers from second array, element-wise.
subtract(x1, x2[, out])	Subtract arguments, element-wise.
true_divide(x1, x2[, out])	Returns a true division of the inputs, element-wise.
floor_divide(x1, x2[, out])	Return the largest integer smaller or equal to the division of the inputs.
fmod(x1, x2[, out])	Return the element-wise remainder of division.
mod(x1, x2[, out])	Return element-wise remainder of division.
modf(x[, out1, out2])	Return the fractional and integral parts of an array, element-wise.
remainder(x1, x2[, out])	Return element-wise remainder of division.

Handling complex numbers
angle(z[, deg])	Return the angle of the complex argument.
real(val)	Return the real part of the elements of the array.
imag(val)	Return the imaginary part of the elements of the array.
conj(x[, out])	Return the complex conjugate, element-wise.

Miscellaneous
convolve(a, v[, mode])	Returns the discrete, linear convolution of two one-dimensional sequences.
clip(a, a_min, a_max[, out])	Clip (limit) the values in an array.
sqrt(x[, out])	Return the positive square-root of an array, element-wise.
cbrt(x[, out])	Return the cube-root of an array, element-wise.
square(x[, out])	Return the element-wise square of the input.
absolute(x[, out])	Calculate the absolute value element-wise.
fabs(x[, out])	Compute the absolute values element-wise.
sign(x[, out])	Returns an element-wise indication of the sign of a number.
maximum(x1, x2[, out])	Element-wise maximum of array elements.
minimum(x1, x2[, out])	Element-wise minimum of array elements.
fmax(x1, x2[, out])	Element-wise maximum of array elements.
fmin(x1, x2[, out])	Element-wise minimum of array elements.
nan_to_num(x)	Replace nan with zero and inf with finite numbers.
real_if_close(a[, tol])	If complex input returns a real array if complex parts are close to zero.
interp(x, xp, fp[, left, right, period])	One-dimensional linear interpolation.

*/

trait TrigFuncs {
    // Trigonometric sine, element-wise.
    pub fn sin(x: &Matrix) -> Matrix;
    // Cosine element-wise.
    pub fn cos(x: &Matrix) -> Matrix;
    //Compute tangent element-wise.
    pub fn tan(x: &Matrix) -> Matrix;
    //Inverse sine, element-wise.
    pub fn arcsin(x: &Matrix) -> Matrix;
    //Trigonometric inverse cosine, element-wise.
    pub fn arccos(x: &Matrix) -> Matrix;
    //Trigonometric inverse tangent, element-wise.
    pub fn arctan(x: &Matrix) -> Matrix;
    //Given the “legs” of a right triangle, return its hypotenuse.
    pub fn hypot(x1: &Matrix, x2: &Matrix) -> Matrix;
    //Element-wise arc tangent of x1/x2 choosing the quadrant correctly.
    pub fn arctan2(x1: &Matrix, x2: &Matrix) -> Matrix;
    //Convert angles from radians to degrees.
    pub fn degrees(x: &Matrix) -> Matrix;
    //Convert angles from degrees to radians.
    pub fn radians(x: &Matrix) -> Matrix;
    // TODO:
    // Unwrap by changing deltas between values to 2*pi complement.
    // pub fn unwrap(p[, discont, axis]) -> Matrix;
    //Convert angles from degrees to radians.
    pub fn deg2rad(x: &Matrix) -> Matrix;
    //Convert angles from radians to degrees.
    pub fn rad2deg(x: &Matrix) -> Matrix;
}

trait HypberbolicTrigFuncs {
    // 	Hyperbolic sine, element-wise.
    pub fn sinh(x: &Matrix) -> Matrix;
    // 	Hyperbolic cosine, element-wise.
    pub fn cosh(x: &Matrix) -> Matrix;
    // 	Compute hyperbolic tangent element-wise.
    pub fn tanh(x: &Matrix) -> Matrix;
    // 	Inverse hyperbolic sine element-wise.
    pub fn arcsinh(x: &Matrix) -> Matrix;
    // 	Inverse hyperbolic cosine, element-wise.
    pub fn arccosh(x: &Matrix) -> Matrix;
    // 	Inverse hyperbolic tangent element-wise.
    pub fn arctanh(x: &Matrix) -> Matrix;
}

trait RoundingFuncs {
    // 	Evenly round to the given number of decimals.
    pub fn around(a[, decimals: &Matrix) -> Matrix;
    // 	Round an array to the given number of decimals.
    pub fn round_(a[, decimals: &Matrix) -> Matrix;
    // 	Round elements of the array to the nearest integer.
    pub fn rint(x: &Matrix) -> Matrix;
    // Round to nearest integer towards zero.
    pub fn fix(x: &Matrix ) -> Matrix;
    // 	Return the floor of the input, element-wise.
    pub fn floor(x: &Matrix) -> Matrix;
    // 	Return the ceiling of the input, element-wise.
    pub fn ceil(x: &Matrix) -> Matrix;
    // 	Return the truncated value of the input, element-wise.
    pub fn trunc(x: &Matrix) -> Matrix;
}

trait SumFuncs {
    // 	Sum of array elements over a given axis.
    sum(a: &Matrix) -> Matrix;
    // 	Return the sum of array elements over a given axis treating Not a Numbers (NaNs) as zero.
    nansum(a: &Matrix) -> Matrix;
    // 	Return the cumulative sum of array elements over a given axis treating Not a Numbers (NaNs) as zero.
    nancumsum(a: &Matrix) -> Matrix;
    // 	Return the cumulative sum of the elements along a given axis.
    cumsum(a: &Matrix) -> Matrix;
}

trait DifferenceFuncs {
    //	Calculate the n-th discrete difference along given axis.
    diff(a: &Matrix) -> Matrix;
    //	The differences between consecutive elements of an array.
    ediff1d(ary: &Matrix) -> Matrix;
}

trait ProductFuncs {
    //	Return the product of array elements over a given axis.
    prod(a: &Matrix) -> Matrix;
    //	Return the product of array elements over a given axis treating Not a Numbers (NaNs) as ones.
    nanprod(a: &Matrix) -> Matrix;
    //	Return the cumulative product of elements along a given axis.
    cumprod(a: &Matrix) -> Matrix;
    //	Return the cumulative product of array elements over a given axis treating Not a Numbers (NaNs) as one.
    nancumprod(a: &Matrix) -> Matrix;
}



