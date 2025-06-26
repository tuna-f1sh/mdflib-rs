// Wrapper header for bindgen to parse
// This should include the necessary headers for the exported functions

#pragma once

#ifdef __cplusplus
extern "C" {
#endif

// Include the mdflib headers - adjust paths as needed
// These would normally come from the installed library or bundled source
// #include <mdf/mdfreader.h>
// #include <mdf/mdfwriter.h>
// #include <mdf/mdffactory.h>
// #include <mdf/canmessage.h>

// For now, we'll define the essential types and functions based on MdfExport.cpp
// In a real implementation, you'd want to include the actual headers

// Forward declarations for opaque pointer types
typedef struct MdfReader MdfReader;
typedef struct MdfWriter MdfWriter;
typedef struct MdfFile MdfFile;
typedef struct IHeader IHeader;
typedef struct IDataGroup IDataGroup;
typedef struct IChannelGroup IChannelGroup;
typedef struct IChannel IChannel;
typedef struct IChannelObserver IChannelObserver;
typedef struct CanMessage CanMessage;

// Enums from the mdflib API
typedef enum {
    MDF_WRITER_TYPE_MDF4 = 0,
    MDF_WRITER_TYPE_MDF3 = 1
} MdfWriterType;

typedef enum {
    CHANNEL_TYPE_FIXED_LENGTH = 0,
    CHANNEL_TYPE_VARIABLE_LENGTH = 1,
    CHANNEL_TYPE_MASTER = 2,
    CHANNEL_TYPE_VIRTUAL_MASTER = 3,
    CHANNEL_TYPE_SYNC = 4,
    CHANNEL_TYPE_MAX_LENGTH = 5,
    CHANNEL_TYPE_VIRTUAL_DATA = 6
} ChannelType;

typedef enum {
    CHANNEL_DATA_TYPE_UNSIGNED_INT = 0,
    CHANNEL_DATA_TYPE_SIGNED_INT = 1,
    CHANNEL_DATA_TYPE_FLOAT = 2,
    CHANNEL_DATA_TYPE_STRING = 3,
    CHANNEL_DATA_TYPE_BYTE_ARRAY = 4
} ChannelDataType;

// Function declarations matching MdfExport.cpp
// MdfReader functions
MdfReader* MdfReaderInit(const char* filename);
void MdfReaderUnInit(MdfReader* reader);
int MdfReaderIsOk(MdfReader* reader);
int MdfReaderOpen(MdfReader* reader);
void MdfReaderClose(MdfReader* reader);
int MdfReaderReadHeader(MdfReader* reader);
int MdfReaderReadMeasurementInfo(MdfReader* reader);
int MdfReaderReadEverythingButData(MdfReader* reader);
int MdfReaderReadData(MdfReader* reader, IDataGroup* group);

// MdfWriter functions
MdfWriter* MdfWriterInit(MdfWriterType type, const char* filename);
void MdfWriterUnInit(MdfWriter* writer);
int MdfWriterInitMeasurement(MdfWriter* writer);
int MdfWriterFinalizeMeasurement(MdfWriter* writer);

// Add more function declarations as needed...

#ifdef __cplusplus
}
#endif
