// This file was autogenerated from Opc.Ua.Types.bsd.xml by tools/schema/gen_types.js
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

use encoding::*;
#[allow(unused_imports)]
use basic_types::*;
use date_time::DateTime;
use service_types::AggregateConfiguration;

#[derive(Debug, Clone, PartialEq)]
pub struct AggregateFilterResult {
    pub revised_start_time: DateTime,
    pub revised_processing_interval: Double,
    pub revised_aggregate_configuration: AggregateConfiguration,
}

impl BinaryEncoder<AggregateFilterResult> for AggregateFilterResult {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.revised_start_time.byte_len();
        size += self.revised_processing_interval.byte_len();
        size += self.revised_aggregate_configuration.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.revised_start_time.encode(stream)?;
        size += self.revised_processing_interval.encode(stream)?;
        size += self.revised_aggregate_configuration.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let revised_start_time = DateTime::decode(stream)?;
        let revised_processing_interval = Double::decode(stream)?;
        let revised_aggregate_configuration = AggregateConfiguration::decode(stream)?;
        Ok(AggregateFilterResult {
            revised_start_time,
            revised_processing_interval,
            revised_aggregate_configuration,
        })
    }
}
