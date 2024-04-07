use std::io::{BufReader, BufWriter};
use yaserde::{YaDeserialize, YaSerialize};
use yaserde_derive::{YaDeserialize, YaSerialize};

// Define structs matching the XML structure
#[derive(Debug, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(rename = "PacketBrainTemplate")]
pub struct PacketBrainTemplate {
    #[yaserde(rename = "Namespace")]
    pub namespace: Namespace,
}

#[derive(Debug, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(rename = "Namespace")]
pub struct Namespace {
    #[yaserde(attribute)]
    pub name: String,
    #[yaserde(rename = "PacketDef")]
    pub packet_def: PacketDef,
}

#[derive(Debug, PartialEq, YaDeserialize, YaSerialize)]
#[yaserde(rename = "PacketDef")]
pub struct PacketDef {
    #[yaserde(attribute)]
    pub name: String,
    #[yaserde(rename = "Field_string")]
    pub destination_file_location: String,
    #[yaserde(rename = "Field_int64")]
    pub timestamp_ms: i64,
    #[yaserde(rename = "Field_int64", skip_serializing_if = "Option::is_none")]
    pub chunk_index: Option<i64>,
    #[yaserde(rename = "Field_bool", skip_serializing_if = "Option::is_none")]
    pub is_last_chunk: Option<bool>,
}

fn main() {
    // Serialization example
    let data = PacketBrainTemplate {
        namespace: Namespace {
            name: "publisher".to_string(),
            packet_def: PacketDef {
                name: "PublisherKafkaKey".to_string(),
                destination_file_location: "/path/to/file".to_string(),
                timestamp_ms: 1673054400000,
                chunk_index: Some(1),
                is_last_chunk: Some(true),
            },
        },
    };

    let mut writer = BufWriter::new(Vec::new());
    data.serialize(&mut writer).unwrap();
    let serialized_xml = String::from_utf8(writer.into_inner()).unwrap();
    println!("Serialized XML:\n{}", serialized_xml);

    // Deserialization example
    let reader = BufReader::new(serialized_xml.as_bytes());
    let deserialized_data: PacketBrainTemplate = YaDeserialize::deserialize(reader).unwrap();
    println!("Deserialized data:\n{:?}", deserialized_data);
}
