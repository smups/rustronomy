mod fits {

    #[derive(Debug)]
    pub struct Fits {
        header: Vec<HeaderDataUnit>
    }

    #[derive(Debug)]
    struct HeaderDataUnit {
        keyword_records: [KeywordRecord; 36]
    }

    #[derive(Debug)]
    struct KeywordRecord {
        data: [u8; 80],
        key_word_name: str
        /*
        opt: value_indicator
        opt: value
        opt: comment
        */
    }

    impl KeywordRecord {
        pub fn new(data: &[u8; 80]) -> KeywordRecord{
            //Calculate the keyword fields
            let key_word_name = &data[0..7] as &[char; 8]; //first 8 bytes
            let value_indicator = match key_word_name{

            }


            KeywordRecord {
                data: *data, //copy the data
                key_word_name: key_word_name
            }
        }

        fn get_value_indicator(keyword_data: &[u8; 2]) -> bool {
            /*
                If bytes 9 and 10 are equal to 61 and 32 we have a
                value associated with this keyword
                UNLESS: the key_word_name equals one of the following
                    - _ (blank keyword field)
                    - HISTORY
                    - COMMENT
            */
            if keyword_data[0] == 61 && keyword_data[1] == 32 {
                return true;
            } else { 
                return false;
            }
        }

        fn get_key_word_name(data: &[u8; 8]) -> str {

        }
    }

}