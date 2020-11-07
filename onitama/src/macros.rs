#[macro_export]
macro_rules! board {
    // no more 1s and 0s
    (@$counter:expr, $temp_bitmap:ident,) => {};
    // match 0
    (@$counter:expr, $temp_bitmap:ident, 0 $($other:tt)*) => {
        // don't do anything, just recursively call with counter + 1
        board!(@$counter + 1, $temp_bitmap, $($other)*);
    };
    // match 1
    (@$counter:expr, $temp_bitmap:ident, 1 $($other:tt)*) => {
        $temp_bitmap = $temp_bitmap | 1 << $counter; // set this position to true
        board!(@$counter + 1, $temp_bitmap, $($other)*);
    };
    // get 1s and 0s
    ($($lit:tt)*) => {
        {
            let mut temp_bitmap = 0u32;
            board!(@0u32, temp_bitmap, $($lit)*);
            temp_bitmap
        }
    };
}
