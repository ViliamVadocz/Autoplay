use bitmaps::Bitmap;
use typenum::U25;

macro_rules! card {
    // no more 1s and 0s
    (@$counter:expr, $temp_bitmap:ident,) => {};
    // match 0
    (@$counter:expr, $temp_bitmap:ident, 0 $($other:tt)*) => {
        // don't do anything, just recursively call with counter + 1
        card!(@$counter + 1, $temp_bitmap, $($other)*);
    };
    // match 1
    (@$counter:expr, $temp_bitmap:ident, 1 $($other:tt)*) => {
        $temp_bitmap.set($counter, true); // set this position to true
        card!(@$counter + 1, $temp_bitmap, $($other)*);
    };
    // get 1s and 0s
    ($($lit:tt)*) => {
        let temp_bitmap: Bitmap<U25> = Bitmap::new();
        card!(@0, temp_bitmap, $($lit)*);
        temp_bitmap
    };
}

pub enum Card {
    Boar,
    Cobra,
    Crab,
    Crane,
    Dragon,
    Eel,
    Elephant,
    Frog,
    Goose,
    Horse,
    Mantis,
    Monkey,
    Ox,
    Rabbit,
    Rooster,
    Tiger,
}

impl Card {
    #[rustfmt::skip]
    fn get_moves(&self) -> Bitmap<U25> {
        match self {
            Card::Boar =>
                card!(0 0 0 0 0
                      0 0 1 0 0
                      0 1 0 1 0
                      0 0 0 0 0
                      0 0 0 0 0),
            Card::Cobra =>
                card!(0 0 0 0 0
                      0 0 0 1 0
                      0 1 0 0 0
                      0 0 0 1 0
                      0 0 0 0 0),
            Card::Crab =>
                card!(0 0 0 0 0
                      0 0 1 0 0
                      1 0 0 0 1
                      0 0 0 0 0
                      0 0 0 0 0),
            Card::Crane =>
                card!(0 0 0 0 0
                      0 0 1 0 0
                      0 0 0 0 0
                      0 1 0 1 0
                      0 0 0 0 0),
            Card::Dragon =>
                card!(0 0 0 0 0
                      1 0 0 0 1
                      0 0 0 0 0
                      0 1 0 1 0
                      0 0 0 0 0),
            Card::Eel =>
                card!(0 0 0 0 0
                      0 1 0 0 0
                      0 0 0 1 0
                      0 1 0 0 0
                      0 0 0 0 0),
            Card::Elephant =>
                card!(0 0 0 0 0
                      0 1 0 1 0
                      0 1 0 1 0
                      0 0 0 0 0
                      0 0 0 0 0),
            Card::Frog =>
                card!(0 0 0 0 0
                      0 1 0 0 0
                      1 0 0 0 0
                      0 0 0 1 0
                      0 0 0 0 0),
            Card::Goose =>
                card!(0 0 0 0 0
                      0 1 0 0 0
                      0 1 0 1 0
                      0 0 0 1 0
                      0 0 0 0 0),
            Card::Horse =>
                card!(0 0 0 0 0
                      0 0 1 0 0
                      0 1 0 0 0
                      0 0 1 0 0
                      0 0 0 0 0),
            Card::Mantis =>
                card!(0 0 0 0 0
                      0 1 0 1 0
                      0 0 0 0 0
                      0 0 1 0 0
                      0 0 0 0 0),
            Card::Monkey =>
                card!(0 0 0 0 0
                      0 1 0 1 0
                      0 0 0 0 0
                      0 1 0 1 0
                      0 0 0 0 0),
            Card::Ox =>
                card!(0 0 0 0 0
                      0 0 1 0 0
                      0 0 0 1 0
                      0 0 1 0 0
                      0 0 0 0 0),
            Card::Rabbit =>
                card!(0 0 0 0 0
                      0 0 0 1 0
                      0 0 0 0 1
                      0 1 0 0 0
                      0 0 0 0 0),
            Card::Rooster =>
                card!(0 0 0 0 0
                      0 0 0 1 0
                      0 1 0 1 0
                      0 1 0 0 0
                      0 0 0 0 0),
            Card::Tiger =>
                card!(0 0 1 0 0
                      0 0 0 0 0
                      0 0 0 0 0
                      0 0 1 0 0
                      0 0 0 0 0),
        }
    }
}
