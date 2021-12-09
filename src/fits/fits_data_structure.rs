/*
    This module contains a list of basic (read: NO implementations)
    data structures found in a FITS file. These are defined as per
    the FITS 4.1 standard of the International Astronomical Union.

    The only struct that contains raw fits byte data is the DataBlock.
    All other structs CANNOT contain raw data or borrowings thereof.

    If a data structure must be able to access certain static variables,
    (for instance, to check if it was corrupted or not) they must be
    declared in the space directly above the struct.

    Traits must are specified below the structure that they apply to
*/
pub struct DataBlock <'life> {
    /*
        This struct contains borrows 2880 bytes of data
        from somewhere (implementation specific source
        of raw fits data).

        This struct is intended to be used to divide the
        FITS file into appropriatly sized blocks, which
        may then be parsed using to structs that implement
        the ParseDataBlock trait.
    */

    data: &'life [u8; 2880]
}

trait ParseDataBlock {
    /*
        This trait is implemented by
            - Header Data Units
            - Data Units
            - Extensions
        The implementation MUST consume/kill the
        DataBlock that it is given ownership of
        through this trait.
        (This is guaranteed by rust awesomeness anyway)
    */
    fn parse_data_block(data: DataBlock) -> Self;
}

/*
    Static vars for HeaderDataUnit:
        - As per section 3.2 of the FITSv4.1 standard, only
            certain ASCII values may appear as byte values in
            the HDU. These are characters 0x20 to 0x7e

*/
pub static HDU_allowed_ASCII: [u8; 96] = [
    32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
    42, 43, 44, 45, 46, 47, 48, 49, 50, 51,
    52, 53, 54, 55, 56, 57, 58, 59, 60, 61,
    62, 63, 64, 65, 66, 67, 68, 69, 70, 71,
    72, 73, 74, 75, 76, 77, 78, 79, 80, 81,
    82, 83, 84, 85, 86, 87, 88, 89, 90, 91,
    92, 93, 94, 95, 96, 97, 98, 99, 100, 101,
    102, 103, 104, 105, 106, 107, 108, 109, 110, 111,
    112, 113, 114, 115, 116, 117, 118, 119, 120, 121,
    122, 123, 124, 125, 126, 127,
 ];

pub struct HeaderDataUnit {
    simple: KeyWord<bool>,
    bitpix: KeyWord<i64>,
    n_axis: KeyWord<i64>,
    n_axis_N: Option<Vec<KeyWord<i64>>>,
    other: Option<Vec<KeyWord<>>
}

pub struct KeyWord<Type> {
    name: String,
    val: Type
}