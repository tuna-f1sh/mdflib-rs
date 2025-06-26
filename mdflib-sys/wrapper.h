// Wrapper header for bindgen to parse mdflib C exports
#pragma once

#ifdef __cplusplus
extern "C" {
#endif

// Standard C types
#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

// When building bundled, include the actual mdflib headers
#ifdef MDF_BUNDLED_BUILD
    // Include actual mdflib headers
    #include <mdf/mdfreader.h>
    #include <mdf/mdfwriter.h>
    #include <mdf/mdffactory.h>
    #include <mdf/canmessage.h>
    #include <mdf/ichannelgroup.h>
    #include <mdf/idatagroup.h>
    #include <mdf/ievent.h>
    #include <mdf/ifilehistory.h>
    
    // The actual exported functions should be available from MdfExport.cpp
    // We need to declare them here for bindgen
#endif

// Forward declarations for opaque pointer types
// These represent C++ objects that we'll handle as opaque pointers in Rust
typedef struct MdfReader MdfReader;
typedef struct MdfWriter MdfWriter;
typedef struct MdfFile MdfFile;
typedef struct IHeader IHeader;
typedef struct IDataGroup IDataGroup;
typedef struct IChannelGroup IChannelGroup;
typedef struct IChannel IChannel;
typedef struct IChannelObserver IChannelObserver;
typedef struct IChannelArray IChannelArray;
typedef struct IChannelConversion IChannelConversion;
typedef struct ISourceInformation ISourceInformation;
typedef struct IAttachment IAttachment;
typedef struct IFileHistory IFileHistory;
typedef struct IEvent IEvent;
typedef struct IMetaData IMetaData;
typedef struct ETag ETag;
typedef struct CanMessage CanMessage;

// Enums from mdflib (matching MdfExport.cpp)
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
    CHANNEL_SYNC_TYPE_NONE = 0,
    CHANNEL_SYNC_TYPE_TIME = 1,
    CHANNEL_SYNC_TYPE_ANGLE = 2,
    CHANNEL_SYNC_TYPE_DISTANCE = 3,
    CHANNEL_SYNC_TYPE_INDEX = 4
} ChannelSyncType;

typedef enum {
    CHANNEL_DATA_TYPE_UNSIGNED_INT = 0,
    CHANNEL_DATA_TYPE_SIGNED_INT = 1,
    CHANNEL_DATA_TYPE_FLOAT = 2,
    CHANNEL_DATA_TYPE_STRING = 3,
    CHANNEL_DATA_TYPE_BYTE_ARRAY = 4,
    CHANNEL_DATA_TYPE_MIME_SAMPLE = 5,
    CHANNEL_DATA_TYPE_MIME_STREAM = 6,
    CHANNEL_DATA_TYPE_CAN_OPEN_DATE = 7,
    CHANNEL_DATA_TYPE_CAN_OPEN_TIME = 8,
    CHANNEL_DATA_TYPE_COMPLEX = 9
} ChannelDataType;

typedef enum {
    CONVERSION_TYPE_NONE = 0,
    CONVERSION_TYPE_LINEAR = 1,
    CONVERSION_TYPE_RATIONAL = 2,
    CONVERSION_TYPE_ALGEBRAIC = 3,
    CONVERSION_TYPE_VALUE_TO_VALUE_INTERPOLATION = 4,
    CONVERSION_TYPE_VALUE_TO_VALUE = 5,
    CONVERSION_TYPE_VALUE_RANGE_TO_VALUE = 6,
    CONVERSION_TYPE_VALUE_TO_TEXT = 7,
    CONVERSION_TYPE_VALUE_RANGE_TO_TEXT = 8,
    CONVERSION_TYPE_TEXT_TO_VALUE = 9,
    CONVERSION_TYPE_TEXT_TO_TEXT = 10,
    CONVERSION_TYPE_BITFIELD_TEXT_TABLE = 11
} ConversionType;

typedef enum {
    ARRAY_TYPE_ARRAY = 0,
    ARRAY_TYPE_SCALING_AXIS = 1,
    ARRAY_TYPE_LOOK_UP = 2
} ArrayType;

typedef enum {
    ARRAY_STORAGE_CN_TEMPLATE = 0,
    ARRAY_STORAGE_CG_TEMPLATE = 1,
    ARRAY_STORAGE_DG_TEMPLATE = 2
} ArrayStorage;

typedef enum {
    SOURCE_TYPE_OTHER = 0,
    SOURCE_TYPE_ECU = 1,
    SOURCE_TYPE_BUS = 2,
    SOURCE_TYPE_I_O = 3,
    SOURCE_TYPE_TOOL = 4,
    SOURCE_TYPE_USER = 5
} SourceType;

typedef enum {
    BUS_TYPE_NONE = 0,
    BUS_TYPE_OTHER = 1,
    BUS_TYPE_CAN = 2,
    BUS_TYPE_LIN = 3,
    BUS_TYPE_MOST = 4,
    BUS_TYPE_FLEXRAY = 5,
    BUS_TYPE_K_LINE = 6,
    BUS_TYPE_ETHERNET = 7,
    BUS_TYPE_USB = 8
} BusType;

typedef enum {
    EVENT_TYPE_RECORDING = 0,
    EVENT_TYPE_RECORDING_INTERRUPT = 1,
    EVENT_TYPE_ACQUISITION_INTERRUPT = 2,
    EVENT_TYPE_START_RECORDING_TRIGGER = 3,
    EVENT_TYPE_STOP_RECORDING_TRIGGER = 4,
    EVENT_TYPE_TRIGGER = 5,
    EVENT_TYPE_MARKER = 6
} EventType;

typedef enum {
    SYNC_TYPE_TIME = 1,
    SYNC_TYPE_ANGLE = 2,
    SYNC_TYPE_DISTANCE = 3,
    SYNC_TYPE_INDEX = 4
} SyncType;

typedef enum {
    RANGE_TYPE_POINT = 0,
    RANGE_TYPE_BEGIN = 1,
    RANGE_TYPE_END = 2
} RangeType;

typedef enum {
    EVENT_CAUSE_OTHER = 0,
    EVENT_CAUSE_ERROR = 1,
    EVENT_CAUSE_TOOL = 2,
    EVENT_CAUSE_SCRIPT = 3,
    EVENT_CAUSE_USER = 4
} EventCause;

typedef enum {
    ETAG_DATA_TYPE_UNSIGNED_INT = 0,
    ETAG_DATA_TYPE_SIGNED_INT = 1,
    ETAG_DATA_TYPE_FLOAT = 2,
    ETAG_DATA_TYPE_STRING = 3,
    ETAG_DATA_TYPE_BOOLEAN = 4
} ETagDataType;

typedef enum {
    CAN_ERROR_TYPE_NONE = 0,
    CAN_ERROR_TYPE_BIT = 1,
    CAN_ERROR_TYPE_FORM = 2,
    CAN_ERROR_TYPE_STUFF = 3,
    CAN_ERROR_TYPE_CRC = 4,
    CAN_ERROR_TYPE_ACK = 5,
    CAN_ERROR_TYPE_OTHER = 6
} CanErrorType;

typedef enum {
    MDF_STORAGE_TYPE_FIXED_LENGTH_DATA = 0,
    MDF_STORAGE_TYPE_VARIABLE_LENGTH_DATA = 1,
    MDF_STORAGE_TYPE_VARIABLE_LENGTH_DATA_LIST = 2
} MdfStorageType;

// Function declarations based on MdfExport.cpp
// These should match the actual exported functions from the C++ library

// MdfReader functions
MdfReader* MdfReaderInit(const char* filename);
void MdfReaderUnInit(MdfReader* reader);
int64_t MdfReaderGetIndex(MdfReader* reader);
bool MdfReaderIsOk(MdfReader* reader);
const MdfFile* MdfReaderGetFile(MdfReader* reader);
const IHeader* MdfReaderGetHeader(MdfReader* reader);
const IDataGroup* MdfReaderGetDataGroup(MdfReader* reader, size_t index);
bool MdfReaderOpen(MdfReader* reader);
void MdfReaderClose(MdfReader* reader);
bool MdfReaderReadHeader(MdfReader* reader);
bool MdfReaderReadMeasurementInfo(MdfReader* reader);
bool MdfReaderReadEverythingButData(MdfReader* reader);
bool MdfReaderReadData(MdfReader* reader, IDataGroup* group);

// MdfWriter functions
MdfWriter* MdfWriterInit(MdfWriterType type, const char* filename);
void MdfWriterUnInit(MdfWriter* writer);
MdfFile* MdfWriterGetFile(MdfWriter* writer);
IHeader* MdfWriterGetHeader(MdfWriter* writer);
bool MdfWriterIsFileNew(MdfWriter* writer);
bool MdfWriterGetCompressData(MdfWriter* writer);
void MdfWriterSetCompressData(MdfWriter* writer, bool compress);
double MdfWriterGetPreTrigTime(MdfWriter* writer);
void MdfWriterSetPreTrigTime(MdfWriter* writer, double pre_trig_time);
uint64_t MdfWriterGetStartTime(MdfWriter* writer);
uint64_t MdfWriterGetStopTime(MdfWriter* writer);
uint16_t MdfWriterGetBusType(MdfWriter* writer);
void MdfWriterSetBusType(MdfWriter* writer, uint16_t type);
MdfStorageType MdfWriterGetStorageType(MdfWriter* writer);
void MdfWriterSetStorageType(MdfWriter* writer, MdfStorageType type);
uint32_t MdfWriterGetMaxLength(MdfWriter* writer);
void MdfWriterSetMaxLength(MdfWriter* writer, uint32_t length);
bool MdfWriterCreateBusLogConfiguration(MdfWriter* writer);
IDataGroup* MdfWriterCreateDataGroup(MdfWriter* writer);
bool MdfWriterInitMeasurement(MdfWriter* writer);
void MdfWriterSaveSample(MdfWriter* writer, IChannelGroup* group, uint64_t time);
void MdfWriterSaveCanMessage(MdfWriter* writer, IChannelGroup* group, uint64_t time, CanMessage* message);
void MdfWriterStartMeasurement(MdfWriter* writer, uint64_t start_time);
void MdfWriterStopMeasurement(MdfWriter* writer, uint64_t stop_time);
bool MdfWriterFinalizeMeasurement(MdfWriter* writer);

// MdfFile functions
size_t MdfFileGetName(MdfFile* file, char* name);
void MdfFileSetName(MdfFile* file, const char* name);
size_t MdfFileGetFileName(MdfFile* file, char* filename);
void MdfFileSetFileName(MdfFile* file, const char* filename);
size_t MdfFileGetVersion(MdfFile* file, char* version);
int MdfFileGetMainVersion(MdfFile* file);
int MdfFileGetMinorVersion(MdfFile* file);
void MdfFileSetMinorVersion(MdfFile* file, int minor);
size_t MdfFileGetProgramId(MdfFile* file, char* program_id);
void MdfFileSetProgramId(MdfFile* file, const char* program_id);
bool MdfFileGetFinalized(MdfFile* file, uint16_t* standard_flags, uint16_t* custom_flags);
const IHeader* MdfFileGetHeader(MdfFile* file);
bool MdfFileGetIsMdf4(MdfFile* file);
size_t MdfFileGetAttachments(MdfFile* file, const IAttachment** pAttachment);
size_t MdfFileGetDataGroups(MdfFile* file, const IDataGroup** pDataGroup);
IAttachment* MdfFileCreateAttachment(MdfFile* file);
IDataGroup* MdfFileCreateDataGroup(MdfFile* file);

// Add more function declarations as we expand the API...

// CanMessage functions
CanMessage* CanMessageInit(void);
void CanMessageUnInit(CanMessage* can);
uint32_t CanMessageGetMessageId(CanMessage* can);
void CanMessageSetMessageId(CanMessage* can, uint32_t msgId);
uint32_t CanMessageGetCanId(CanMessage* can);
bool CanMessageGetExtendedId(CanMessage* can);
void CanMessageSetExtendedId(CanMessage* can, bool extendedId);
uint8_t CanMessageGetDlc(CanMessage* can);
void CanMessageSetDlc(CanMessage* can, uint8_t dlc);
size_t CanMessageGetDataLength(CanMessage* can);
void CanMessageSetDataLength(CanMessage* can, uint32_t dataLength);
size_t CanMessageGetDataBytes(CanMessage* can, uint8_t* dataList);
void CanMessageSetDataBytes(CanMessage* can, const uint8_t* dataList, size_t size);

#ifdef __cplusplus
}
#endif
