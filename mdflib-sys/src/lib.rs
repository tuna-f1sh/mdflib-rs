#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Include the generated bindings from bindgen
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CStr, CString};
    use std::fs;
    use std::os::raw::c_char;
    use std::path::Path;

    #[test]
    fn test_writer_and_reader() {
        let filename_str = "test_rust.mdf";
        let filename = CString::new(filename_str).unwrap();

        // Cleanup previous test file if it exists
        if Path::new(filename_str).exists() {
            fs::remove_file(filename_str).unwrap();
        }

        unsafe {
            // Writer part
            let writer = MdfWriterInit(MdfWriterType::Mdf4Basic, filename.as_ptr());
            assert!(!writer.is_null(), "Failed to initialize MdfWriter");

            let file = MdfWriterGetFile(writer);
            assert!(!file.is_null(), "Failed to get MdfFile from writer");

            let data_group = MdfFileCreateDataGroup(file);
            assert!(!data_group.is_null(), "Failed to create IDataGroup");

            let channel_group = DataGroupCreateChannelGroup(data_group);
            assert!(!channel_group.is_null(), "Failed to create IChannelGroup");

            let channel_name = CString::new("TestChannel").unwrap();
            let channel = ChannelGroupCreateChannel(channel_group);
            assert!(!channel.is_null(), "Failed to create IChannel");
            ChannelSetName(channel, channel_name.as_ptr());
            ChannelSetDataType(channel, ChannelDataType::UnsignedIntegerLe as u8);
            ChannelSetDataBytes(channel, 4);

            assert!(
                MdfWriterInitMeasurement(writer),
                "Failed to init measurement"
            );

            // Write a single sample
            let value: u32 = 12345;
            ChannelSetChannelValue(channel, value, true);
            MdfWriterSaveSample(writer, channel_group, 0);

            assert!(
                MdfWriterFinalizeMeasurement(writer),
                "Failed to finalize measurement"
            );
            MdfWriterUnInit(writer);

            // Reader part
            let reader = MdfReaderInit(filename.as_ptr());
            assert!(!reader.is_null(), "Failed to initialize MdfReader");
            assert!(MdfReaderIsOk(reader), "MdfReader is not OK");

            assert!(
                MdfReaderReadEverythingButData(reader),
                "Failed to read file metadata"
            );

            let read_file = MdfReaderGetFile(reader);
            assert!(!read_file.is_null());

            let dg_count = MdfFileGetDataGroupCount(read_file);
            assert_eq!(dg_count, 1, "Expected 1 data group");

            let read_dg = MdfFileGetDataGroupByIndex(read_file, 0);
            assert!(!read_dg.is_null());

            let cg_count = DataGroupGetChannelGroupCount(read_dg);
            assert_eq!(cg_count, 1, "Expected 1 channel group");

            let read_cg = DataGroupGetChannelGroupByIndex(read_dg, 0);
            assert!(!read_cg.is_null());

            let ch_count = ChannelGroupGetChannelCount(read_cg);
            assert_eq!(ch_count, 1, "Expected 1 channel");

            let read_ch = ChannelGroupGetChannelByIndex(read_cg, 0);
            assert!(!read_ch.is_null());

            let mut buffer: [c_char; 128] = [0; 128];
            ChannelGetName(read_ch, buffer.as_mut_ptr(), buffer.len());
            let read_channel_name = CStr::from_ptr(buffer.as_ptr()).to_str().unwrap();

            assert_eq!(
                read_channel_name, "TestChannel",
                "Channel name does not match"
            );

            MdfReaderUnInit(reader);
        }

        // Final cleanup
        if Path::new(filename_str).exists() {
            fs::remove_file(filename_str).unwrap();
        }
    }
}
