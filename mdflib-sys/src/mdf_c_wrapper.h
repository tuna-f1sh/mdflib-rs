#ifndef MDF_C_WRAPPER_H
#define MDF_C_WRAPPER_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// Opaque pointers to hide C++ implementation details
typedef struct MdfReader MdfReader;
typedef struct MdfWriter MdfWriter;
typedef struct MdfFile MdfFile;
typedef struct IHeader IHeader;
typedef struct IDataGroup IDataGroup;
typedef struct IChannelGroup IChannelGroup;
typedef struct IChannel IChannel;
typedef struct CanMessage CanMessage;

// Enums from mdflib
typedef enum {
    MdfWriterType_Mdf3,
    MdfWriterType_Mdf4
} MdfWriterType;

typedef enum {
    ChannelType_FixedLength = 0,
    ChannelType_VariableLength = 1,
    ChannelType_Master = 2,
    ChannelType_VirtualMaster = 3,
    ChannelType_Sync = 4,
    ChannelType_MaxLength = 5,
    ChannelType_VirtualData = 6
} ChannelType;

typedef enum {
    ChannelDataType_UnsignedIntegerLe = 0,
    ChannelDataType_UnsignedIntegerBe = 1,
    ChannelDataType_SignedIntegerLe = 2,
    ChannelDataType_SignedIntegerBe = 3,
    ChannelDataType_FloatLe = 4,
    ChannelDataType_FloatBe = 5,
    ChannelDataType_String = 6,
    ChannelDataType_ByteArray = 7,
    ChannelDataType_CanOpenDate = 9,
    ChannelDataType_CanOpenTime = 10
} ChannelDataType;


#if defined(_WIN32)
#define EXPORT __declspec(dllexport)
#elif defined(__linux__) || defined(__APPLE__) || defined(__CYGWIN__)
#define EXPORT __attribute__((visibility("default")))
#else
#define EXPORT
#endif

// MdfReader functions
EXPORT MdfReader* MdfReaderInit(const char* filename);
EXPORT void MdfReaderUnInit(MdfReader* reader);
EXPORT int64_t MdfReaderGetIndex(MdfReader* reader);
EXPORT bool MdfReaderIsOk(MdfReader* reader);
EXPORT const MdfFile* MdfReaderGetFile(MdfReader* reader);
EXPORT const IHeader* MdfReaderGetHeader(MdfReader* reader);
EXPORT const IDataGroup* MdfReaderGetDataGroup(MdfReader* reader, size_t index);
EXPORT size_t MdfReaderGetDataGroupCount(MdfReader* reader);
EXPORT bool MdfReaderOpen(MdfReader* reader);
EXPORT void MdfReaderClose(MdfReader* reader);
EXPORT bool MdfReaderReadHeader(MdfReader* reader);
EXPORT bool MdfReaderReadMeasurementInfo(MdfReader* reader);
EXPORT bool MdfReaderReadEverythingButData(MdfReader* reader);
EXPORT bool MdfReaderReadData(MdfReader* reader, IDataGroup* group);

// MdfWriter functions
EXPORT MdfWriter* MdfWriterInit(MdfWriterType type, const char* filename);
EXPORT void MdfWriterUnInit(MdfWriter* writer);
EXPORT MdfFile* MdfWriterGetFile(MdfWriter* writer);
EXPORT IHeader* MdfWriterGetHeader(MdfWriter* writer);
EXPORT bool MdfWriterIsFileNew(MdfWriter* writer);
EXPORT bool MdfWriterGetCompressData(MdfWriter* writer);
EXPORT void MdfWriterSetCompressData(MdfWriter* writer, bool compress);
EXPORT double MdfWriterGetPreTrigTime(MdfWriter* writer);
EXPORT void MdfWriterSetPreTrigTime(MdfWriter* writer, double pre_trig_time);
EXPORT uint64_t MdfWriterGetStartTime(MdfWriter* writer);
EXPORT uint64_t MdfWriterGetStopTime(MdfWriter* writer);
EXPORT uint16_t MdfWriterGetBusType(MdfWriter* writer);
EXPORT void MdfWriterSetBusType(MdfWriter* writer, uint16_t type);
EXPORT bool MdfWriterCreateBusLogConfiguration(MdfWriter* writer);
EXPORT IDataGroup* MdfWriterCreateDataGroup(MdfWriter* writer);
EXPORT bool MdfWriterInitMeasurement(MdfWriter* writer);
EXPORT void MdfWriterSaveSample(MdfWriter* writer, IChannelGroup* group, uint64_t time);
EXPORT void MdfWriterSaveCanMessage(MdfWriter* writer, IChannelGroup* group, uint64_t time, CanMessage* message);
EXPORT void MdfWriterStartMeasurement(MdfWriter* writer, uint64_t start_time);
EXPORT void MdfWriterStopMeasurement(MdfWriter* writer, uint64_t stop_time);
EXPORT bool MdfWriterFinalizeMeasurement(MdfWriter* writer);

// MdfFile functions
EXPORT size_t MdfFileGetName(const MdfFile* file, char* name, size_t max_length);
EXPORT void MdfFileSetName(MdfFile* file, const char* name);
EXPORT size_t MdfFileGetFileName(const MdfFile* file, char* filename, size_t max_length);
EXPORT void MdfFileSetFileName(MdfFile* file, const char* filename);
EXPORT size_t MdfFileGetVersion(const MdfFile* file, char* version, size_t max_length);
EXPORT int MdfFileGetMainVersion(const MdfFile* file);
EXPORT int MdfFileGetMinorVersion(const MdfFile* file);
EXPORT void MdfFileSetMinorVersion(MdfFile* file, int minor);
EXPORT const IHeader* MdfFileGetHeader(const MdfFile* file);
EXPORT bool MdfFileGetIsMdf4(const MdfFile* file);
EXPORT size_t MdfFileGetDataGroupCount(const MdfFile* file);
EXPORT const IDataGroup* MdfFileGetDataGroupByIndex(const MdfFile* file, size_t index);
EXPORT IDataGroup* MdfFileCreateDataGroup(MdfFile* file);

// IDataGroup functions
EXPORT uint64_t DataGroupGetIndex(const IDataGroup* group);
EXPORT size_t DataGroupGetDescription(const IDataGroup* group, char* description, size_t max_length);
EXPORT void DataGroupSetDescription(IDataGroup* group, const char* description);
EXPORT size_t DataGroupGetChannelGroupCount(const IDataGroup* group);
EXPORT const IChannelGroup* DataGroupGetChannelGroupByIndex(const IDataGroup* group, size_t index);
EXPORT IChannelGroup* DataGroupCreateChannelGroup(IDataGroup* group);

// IChannelGroup functions
EXPORT uint64_t ChannelGroupGetIndex(const IChannelGroup* group);
EXPORT size_t ChannelGroupGetName(const IChannelGroup* group, char* name, size_t max_length);
EXPORT void ChannelGroupSetName(IChannelGroup* group, const char* name);
EXPORT size_t ChannelGroupGetDescription(const IChannelGroup* group, char* description, size_t max_length);
EXPORT void ChannelGroupSetDescription(IChannelGroup* group, const char* description);
EXPORT uint64_t ChannelGroupGetNofSamples(const IChannelGroup* group);
EXPORT void ChannelGroupSetNofSamples(IChannelGroup* group, uint64_t samples);
EXPORT size_t ChannelGroupGetChannelCount(const IChannelGroup* group);
EXPORT const IChannel* ChannelGroupGetChannelByIndex(const IChannelGroup* group, size_t index);
EXPORT IChannel* ChannelGroupCreateChannel(IChannelGroup* group);

// IChannel functions
EXPORT uint64_t ChannelGetIndex(const IChannel* channel);
EXPORT size_t ChannelGetName(const IChannel* channel, char* name, size_t max_length);
EXPORT void ChannelSetName(IChannel* channel, const char* name);
EXPORT size_t ChannelGetDisplayName(const IChannel* channel, char* display_name, size_t max_length);
EXPORT void ChannelSetDisplayName(IChannel* channel, const char* display_name);
EXPORT size_t ChannelGetDescription(const IChannel* channel, char* description, size_t max_length);
EXPORT void ChannelSetDescription(IChannel* channel, const char* description);
EXPORT size_t ChannelGetUnit(const IChannel* channel, char* unit, size_t max_length);
EXPORT void ChannelSetUnit(IChannel* channel, const char* unit);
EXPORT uint8_t ChannelGetType(const IChannel* channel);
EXPORT void ChannelSetType(IChannel* channel, uint8_t type);
EXPORT uint8_t ChannelGetDataType(const IChannel* channel);
EXPORT void ChannelSetDataType(IChannel* channel, uint8_t data_type);
EXPORT uint64_t ChannelGetDataBytes(const IChannel* channel);
EXPORT void ChannelSetDataBytes(IChannel* channel, uint64_t bytes);
EXPORT void ChannelSetChannelValue(IChannel* channel, uint32_t value, bool valid);

// CanMessage functions
EXPORT CanMessage* CanMessageInit();
EXPORT void CanMessageUnInit(CanMessage* can);
EXPORT uint32_t CanMessageGetMessageId(CanMessage* can);
EXPORT void CanMessageSetMessageId(CanMessage* can, uint32_t msgId);
EXPORT uint32_t CanMessageGetCanId(CanMessage* can);
EXPORT bool CanMessageGetExtendedId(CanMessage* can);
EXPORT void CanMessageSetExtendedId(CanMessage* can, bool extendedId);
EXPORT uint8_t CanMessageGetDlc(CanMessage* can);
EXPORT void CanMessageSetDlc(CanMessage* can, uint8_t dlc);
EXPORT size_t CanMessageGetDataLength(CanMessage* can);
EXPORT void CanMessageSetDataLength(CanMessage* can, uint32_t dataLength);
EXPORT size_t CanMessageGetDataBytes(CanMessage* can, uint8_t* dataList, size_t max_length);
EXPORT void CanMessageSetDataBytes(CanMessage* can, const uint8_t* dataList, size_t size);

#ifdef __cplusplus
}
#endif

#endif // MDF_C_WRAPPER_H
