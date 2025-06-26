// Comprehensive wrapper header for bindgen to parse mdflib C exports
#pragma once

#ifdef __cplusplus
extern "C" {
#endif

// Standard C types
#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

// Forward declarations for opaque pointer types
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

// Enums from mdflib
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

// MdfReader functions
MdfReader* MdfReaderInit(const char* filename);
void MdfReaderUnInit(MdfReader* reader);
int64_t MdfReaderGetIndex(MdfReader* reader);
bool MdfReaderIsOk(MdfReader* reader);
const MdfFile* MdfReaderGetFile(MdfReader* reader);
const IHeader* MdfReaderGetHeader(MdfReader* reader);
const IDataGroup* MdfReaderGetDataGroup(MdfReader* reader, size_t index);
size_t MdfReaderGetDataGroupCount(MdfReader* reader);
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
bool MdfWriterCreateBusLogConfiguration(MdfWriter* writer);
IDataGroup* MdfWriterCreateDataGroup(MdfWriter* writer);
bool MdfWriterInitMeasurement(MdfWriter* writer);
void MdfWriterSaveSample(MdfWriter* writer, IChannelGroup* group, uint64_t time);
void MdfWriterSaveCanMessage(MdfWriter* writer, IChannelGroup* group, uint64_t time, CanMessage* message);
void MdfWriterStartMeasurement(MdfWriter* writer, uint64_t start_time);
void MdfWriterStopMeasurement(MdfWriter* writer, uint64_t stop_time);
bool MdfWriterFinalizeMeasurement(MdfWriter* writer);

// MdfFile functions
size_t MdfFileGetName(MdfFile* file, char* name, size_t max_length);
void MdfFileSetName(MdfFile* file, const char* name);
size_t MdfFileGetFileName(MdfFile* file, char* filename, size_t max_length);
void MdfFileSetFileName(MdfFile* file, const char* filename);
size_t MdfFileGetVersion(MdfFile* file, char* version, size_t max_length);
int MdfFileGetMainVersion(MdfFile* file);
int MdfFileGetMinorVersion(MdfFile* file);
void MdfFileSetMinorVersion(MdfFile* file, int minor);
const IHeader* MdfFileGetHeader(MdfFile* file);
bool MdfFileGetIsMdf4(MdfFile* file);
size_t MdfFileGetDataGroupCount(MdfFile* file);
const IDataGroup* MdfFileGetDataGroupByIndex(MdfFile* file, size_t index);
IDataGroup* MdfFileCreateDataGroup(MdfFile* file);

// IDataGroup functions
uint64_t DataGroupGetIndex(const IDataGroup* group);
size_t DataGroupGetName(const IDataGroup* group, char* name, size_t max_length);
void DataGroupSetName(IDataGroup* group, const char* name);
size_t DataGroupGetDescription(const IDataGroup* group, char* description, size_t max_length);
void DataGroupSetDescription(IDataGroup* group, const char* description);
size_t DataGroupGetChannelGroupCount(const IDataGroup* group);
const IChannelGroup* DataGroupGetChannelGroupByIndex(const IDataGroup* group, size_t index);
IChannelGroup* DataGroupCreateChannelGroup(IDataGroup* group);

// IChannelGroup functions
uint64_t ChannelGroupGetIndex(const IChannelGroup* group);
size_t ChannelGroupGetName(const IChannelGroup* group, char* name, size_t max_length);
void ChannelGroupSetName(IChannelGroup* group, const char* name);
size_t ChannelGroupGetDescription(const IChannelGroup* group, char* description, size_t max_length);
void ChannelGroupSetDescription(IChannelGroup* group, const char* description);
uint64_t ChannelGroupGetNofSamples(const IChannelGroup* group);
void ChannelGroupSetNofSamples(IChannelGroup* group, uint64_t samples);
size_t ChannelGroupGetChannelCount(const IChannelGroup* group);
const IChannel* ChannelGroupGetChannelByIndex(const IChannelGroup* group, size_t index);
IChannel* ChannelGroupCreateChannel(IChannelGroup* group);

// IChannel functions
uint64_t ChannelGetIndex(const IChannel* channel);
size_t ChannelGetName(const IChannel* channel, char* name, size_t max_length);
void ChannelSetName(IChannel* channel, const char* name);
size_t ChannelGetDisplayName(const IChannel* channel, char* display_name, size_t max_length);
void ChannelSetDisplayName(IChannel* channel, const char* display_name);
size_t ChannelGetDescription(const IChannel* channel, char* description, size_t max_length);
void ChannelSetDescription(IChannel* channel, const char* description);
size_t ChannelGetUnit(const IChannel* channel, char* unit, size_t max_length);
void ChannelSetUnit(IChannel* channel, const char* unit);
uint8_t ChannelGetType(const IChannel* channel);
void ChannelSetType(IChannel* channel, uint8_t type);
uint8_t ChannelGetDataType(const IChannel* channel);
void ChannelSetDataType(IChannel* channel, uint8_t data_type);
uint64_t ChannelGetDataBytes(const IChannel* channel);
void ChannelSetDataBytes(IChannel* channel, uint64_t bytes);
bool ChannelGetChannelValue(const IChannel* channel, uint64_t sample, double* value);
bool ChannelGetEngValue(const IChannel* channel, uint64_t sample, double* value);

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
size_t CanMessageGetDataBytes(CanMessage* can, uint8_t* dataList, size_t max_length);
void CanMessageSetDataBytes(CanMessage* can, const uint8_t* dataList, size_t size);
uint64_t CanMessageGetTime(CanMessage* can);
void CanMessageSetTime(CanMessage* can, uint64_t time);

#ifdef __cplusplus
}
#endif
