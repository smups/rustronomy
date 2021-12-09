use num::{Complex};

pub enum KeywordValueTypes {
    /*
        This enum contains the various variations of data
        types that may show up in a FITS keyword. Since these
        keywords are trivial data types, the enum implements
        a Copy method
    */
    Integer (i64),
    Float (f64),
    Logic (bool),
    CharArray (String),
    ComplexInteger (Complex<i64>),
    ComplexFloat (Complex<f64>),
}

pub struct Keyword {
    /*
        Basic struct containing a Keyword name as string and a value as
        an variant of the ValueTypes enum, which contains the various
        allowed value types in the FITS format
    */
    pub name: String,
    pub value: Option<KeywordValueTypes>
}

impl Keyword {
    /*
        Implementation for the keyword struct that handles encoding and
        decoding of keywords to/from byte arrays
    */

    pub fn decode(data: [u8; 80]) -> Self {
        /*
            This function takes an 80 byte long array and converts it 
            into an instance of the Keyword struct.

            May panic if decoding errors occur, since this struct is
            only used in header decoding and corrupted headers = bad
        */

        //First we parse the first 8 bytes to find the keyword name
        let name = Keyword::data_to_string(data[0..=7].to_vec());

        //Next we check if the keyword has a value.
        //we can do this by checking bytes 8 and 9
        let has_value = if data[8] == 61 && data[9] == 32 {true} else {false};

        //Useful data vector for matching pattern down below:
        let data_bytes = data[10..=79].to_vec();

        //Set the keyword value to None if there is no value
        let value = if !has_value {None} else {
            //If we do have a value, we must parse bytes 10 through 79 to find it
            match Keyword::data_to_string(data[10..=79].to_vec()).as_str() {
                //Blank keywords
                "HISTORY" | "" | "COMMENT"
                => None,

                //Integer keywords
                "NAXIS" | "NAXIS0" | "NAXIS1" | "NAXIS2" | "NAXIS3"
                => Some(KeywordValueTypes::Integer(
                        Keyword::data_to_int(data_bytes))
                    ),

                //Logical keywords (bools)
                "LOGIC" 
                => match Keyword::data_to_string(vec![data[29]]).as_str() {
                        "T" => Some(KeywordValueTypes::Logic(true)),
                        "F" => Some(KeywordValueTypes::Logic(false)),
                        _ => None
                }

                //Badly encoded data
                _ => panic!("Invalid data type!")
            }
        };

        return  Keyword{
            name: name,
            value: value
        };
    }

    pub fn encode(self) -> [u8; 80] {
        /*
            This function converts itself into an encoded byte array.

            It consumes a Keyword instance (self)
        */

        return [0 as u8; 80];
    }

    pub fn has_value(&self) -> bool {
        //Boiler plate function to check if the keyword has an associated value
        self.value.is_none()
    }

    fn data_to_string(data: Vec<u8>) -> String {
        //Converts string byte vectors to strings
        //Checks if resulting string is FITS standard 4.1 compliant

        //I am allowing panics in the header decoding since no header = bad data
        String::from_utf8(data).unwrap()
    }

    fn data_to_int(data: Vec<u8>) -> i64 {
        0 //TODO: implement this
    }

}