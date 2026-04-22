use super::*;

#[test]
fn test_file_header_serialization_and_deserialization() {
    let original = FileHeader::new(
        "test_file.txt".to_string(),
        12345,
        4,
        "hash123".to_string(),
        10,
        2,
    );

    let bytes = original.to_bytes().expect("Serialization failed");
    assert_eq!(bytes.len(), HEADER_SIZE);

    let decoded = FileHeader::from_bytes(&bytes).expect("Deserialization failed");
    
    assert_eq!(decoded.magic, MAGIC_NUMBER);
    assert_eq!(decoded.version, VERSION);
    assert_eq!(decoded.original_filename, "test_file.txt");
    assert_eq!(decoded.file_size, 12345);
    assert_eq!(decoded.block_size, 4);
    assert_eq!(decoded.data_shards, 10);
    assert_eq!(decoded.parity_shards, 2);
}

#[test]
fn test_file_header_from_bytes_invalid_magic() {
    let mut original = FileHeader::new(
        "test".to_string(),
        100,
        1,
        "hash".to_string(),
        1,
        1,
    );
    // Tamper with magic number
    let mut bytes = original.to_bytes().unwrap();
    bytes[0] = 0x00;

    let result = FileHeader::from_bytes(&bytes);
    assert!(result.is_err());
}
