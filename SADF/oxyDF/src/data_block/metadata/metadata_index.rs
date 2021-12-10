pub struct MetadataIndexTable {
    num_indices:u16,
    indices: Vec<MetadataIndexBlock>
}

#[derive(Debug)]
struct MetadataIndexBlock {
    //This struct represents the raw data format used by the SADF spec for the
    //metadataindex table entries.
    idn: u16,
    table_start: u64,
    table_length: u64
}