fn main() {
    // Converting data types with parse
    let _guess: u32 = "42".parse().expect("Not a number!");

    // Floating-Point Types
    {
        let _x = 2.0; // f64
        let _y: f32 = 3.0; // f32
    }

    // Numeric Operations
    {
        // addition
        let _summ = 5 + 10;

        // subtraction
        let _difference = 95.5 - 4.3;

        // multiplication
        let _product = 4 * 30;

        // division
        let _quotient = 56.7 / 32.2;
        let _floored = 2 / 3; // Results in 0

        // remainder
        let _remainder = 43 % 5;
    }

    // The Boolean Type
    {
        let _t = true;
        let _f: bool = false; // with explicit type annotation
    }

    // The Character Type
    {
        let _c = 'z';
        let _z: char = 'ℤ'; // with explicit type annotation
        let _heart_eyed_cat = '😻';
    }

    // Compound Types

    // The Tuple Type
    {
        let tup: (i32, f64, u8) = (500, 6.4, 1);

        // access a tuple element directly by using a period (.) followed by the index of the value
        let _five_hundred = tup.0;
        let _six_point_four = tup.1;
        let _one = tup.2;

        // destructuring
        let (_x, y, _z) = tup;

        println!("The value of y is: {y}");
    }

    // The Array Type
    {
        // same
        let _a = [1, 2, 3, 4, 5];
        let a: [i32; 5] = [1, 2, 3, 4, 5];

        // accessing array
        let _first = a[0];
        let _second = a[1];

        // auto fill
        let _b = [3; 5];
        let _b = [3, 3, 3, 3, 3];
    }
}
