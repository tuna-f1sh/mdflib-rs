#ifndef MDF_C_WRAPPER_H
#define MDF_C_WRAPPER_H

// #include <stdint.h>
// #include <stddef.h>
//
// #ifdef __cplusplus
// extern "C" {
// #endif
//
// // Opaque pointers to hide C++ implementation details
// typedef struct MdfReader MdfReader;
// typedef struct MdfFile MdfFile;
// typedef struct MdfHeader MdfHeader;
// typedef struct MdfDataGroup MdfDataGroup;
// typedef struct MdfChannelGroup MdfChannelGroup;
// typedef struct MdfChannel MdfChannel;
//
// // Error codes to be returned by wrapper functions
// typedef enum {
//     MDF_SUCCESS = 0,
//     MDF_ERROR_NULL_POINTER = 1,
//     MDF_ERROR_CPP_EXCEPTION = 2,
//     MDF_ERROR_FILE_NOT_OPEN = 3,
//     MDF_ERROR_INVALID_ARGUMENT = 4,
//     MDF_ERROR_UNKNOWN = 5,
// } MdfStatusCode;
//
// // MdfReader functions
// MdfStatusCode mdf_reader_new(MdfReader** reader, const char* filename);
// MdfStatusCode mdf_reader_free(MdfReader* reader);
// MdfStatusCode mdf_reader_read_header(MdfReader* reader);
// MdfStatusCode mdf_reader_read_everything_but_data(MdfReader* reader);
// MdfStatusCode mdf_reader_get_file(MdfReader* reader, const MdfFile** file);
// MdfStatusCode mdf_reader_get_header(MdfReader* reader, const MdfHeader** header);
// MdfStatusCode mdf_reader_get_data_group(MdfReader* reader, size_t index, const MdfDataGroup** data_group);
//
// // MdfFile functions
// MdfStatusCode mdf_file_get_data_group_count(const MdfFile* file, size_t* count);
//
// // MdfDataGroup functions
// MdfStatusCode mdf_datagroup_get_channel_group_count(const MdfDataGroup* data_group, size_t* count);
// MdfStatusCode mdf_datagroup_get_channel_group(const MdfDataGroup* data_group, size_t index, const MdfChannelGroup** channel_group);
//
// // MdfChannelGroup functions
// MdfStatusCode mdf_channelgroup_get_channel_count(const MdfChannelGroup* channel_group, size_t* count);
// MdfStatusCode mdf_channelgroup_get_channel(const MdfChannelGroup* channel_group, size_t index, const MdfChannel** channel);
// MdfStatusCode mdf_channelgroup_get_name(const MdfChannelGroup* channel_group, char* name, size_t* length);
//
//
// // MdfChannel functions
// MdfStatusCode mdf_channel_get_name(const MdfChannel* channel, char* name, size_t* length);
// MdfStatusCode mdf_channel_get_unit(const MdfChannel* channel, char* unit, size_t* length);
// MdfStatusCode mdf_channel_get_channel_value_as_float(const MdfChannel* channel, uint64_t sample, double* value, bool* valid);
//
//
// #ifdef __cplusplus
// }
// #endif

#endif // MDF_C_WRAPPER_H
