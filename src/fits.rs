mod fits {

    #[derive(Debug)]
    pub struct Fits {
        header: Vec<HeaderDataUnit>
    }

    #[derive(Debug)]
    struct HeaderDataUnit {
        /*
            A Header Data Unit (HDU) must consist of
            36 keywords total. Some of these are optional
            and may be zero. Others are not.
        */

        //Required keywords
        keyword_records: [KeywordRecord; 36]
    }

    #[derive(Debug)]
    pub struct KeywordRecord {
        //Non-optional fields
        key_word_name: String,
        value_indicator: bool,

        //Optional fields may be Null
        value: Option<[u8; 70]>,
        comment: Option<[u8; 72]>
    }

    impl KeywordRecord {
        pub fn new(data: &[u8; 80]) -> KeywordRecord{
            /*
                The keyword fields use the following layout
                - Bytes 0 - 7: keyword name (string)
                - (opt) Bytes 8 - 9: value indicator (badly stored bool)
                    * If the keyword name equals a comment type keyword,
                    there cannot be a value so the value_indicator is missing
                    from the data (but still false)
                - (opt) Bytes
            */

            //Now let's calculate these damn parameters
            let key_word_name = KeywordRecord::get_keyword_name(&data[0..7]);
            let value_indicator = match key_word_name.as_str() {
                "" => false,
                "COMMENT" => false,
                "HISTORY" => false,
                _ => KeywordRecord::get_value_indicator(&data[8..9])
            };
            let value = match value_indicator {
                false => None,
                true => slice_to_array_clone!(&data[10..79], [u8; 70])
            };
            let comment = match value_indicator {
                false => slice_to_array_clone!(&data[8..79], [u8; 72]),
                true => None
            };

            KeywordRecord {
                key_word_name: key_word_name,
                value_indicator: value_indicator,
                value: value,
                comment: comment
            }
        }

        fn get_value_indicator(keyword_data: &[u8]) -> bool {
            /*
                If bytes 9 and 10 are equal to 61 and 32 we have a
                value associated with this keyword
                UNLESS: see implementation of KeywordRecod::new() for details
            */
            if keyword_data[0] == 61 && keyword_data[1] == 32 {
                return true;
            } else { 
                return false;
            }
        }

        fn get_keyword_name(data: &[u8]) -> String {
            let mut resp = String::new();
            //Fill the name with the chars
            for b in data.iter(){
                resp.push(*b as char);
            }
            return resp;
        }

        fn get_value(data: &[u8]) -> String{
            let mut resp = String::new();
            //Fill the name with chars again
            for b in data.iter() {
                resp.push(*b as char);
            }
            return resp;
        }

    }

    trait AsPrimitive<T> {
        fn val_as_prim(&self) -> Option<T>;
        fn com_as_prim(&self) -> Option<T>;
    }

    impl AsPrimitive<String> for KeywordRecord {

        fn val_as_prim(&self) -> Option<String> {
            //If we have no value, return None
            if !self.value_indicator {return None;}

            //Now we can safely unwrap the Option
            let vals = self.value.unwrap();

            //Find the beginning and the end of the string
            let start = vals.iter().position(|x| *x == 39).unwrap() + 1;
            let stop = vals.iter().rev().position(|x| *x == 39).unwrap();

            //Fill the response string
            let mut rsp = String::new();
            for i in start..stop {
                rsp.push(vals[i] as char);
            }

            return Some(rsp);
        }

        fn com_as_prim(&self) -> Option<String> {
            //If we have no value, return None
            if self.value_indicator {return None;}

            //Now we can safely unwrap the Option
            let coms = self.comment.unwrap();

            //Find the beginning and the end of the string
            let start = coms.iter().position(|x| *x == 39).unwrap() + 1;
            let stop = coms.iter().rev().position(|x| *x == 39).unwrap();

            //Fill the response string
            let mut rsp = String::new();
            for i in start..stop {
                rsp.push(coms[i] as char);
            }

            return Some(rsp);
        }
    }

}