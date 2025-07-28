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
typedef struct IChannelArray IChannelArray;
typedef struct IChannelConversion IChannelConversion;
typedef struct ISourceInformation ISourceInformation;
typedef struct IAttachment IAttachment;
typedef struct IFileHistory IFileHistory;
typedef struct IEvent IEvent;
typedef struct ETag ETag;
typedef struct IMetaData IMetaData;
typedef struct CanMessage CanMessage;

// Enums from mdflib
typedef enum {
  MdfWriterType_Mdf3,
  MdfWriterType_Mdf4,
  MdfWriterType_BusLogger
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
EXPORT bool MdfReaderIsFinalized(MdfReader* reader);
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
EXPORT size_t MdfFileGetAttachments(const MdfFile* file, const IAttachment* attachments[], size_t max_count);
EXPORT IAttachment* MdfFileCreateAttachment(MdfFile* file);

// IDataGroup functions
EXPORT uint64_t DataGroupGetIndex(const IDataGroup* group);
EXPORT size_t DataGroupGetDescription(const IDataGroup* group, char* description, size_t max_length);
EXPORT void DataGroupSetDescription(IDataGroup* group, const char* description);
EXPORT size_t DataGroupGetChannelGroupCount(const IDataGroup* group);
EXPORT IChannelGroup* DataGroupGetChannelGroupByIndex(const IDataGroup* group, size_t index);
EXPORT IChannelGroup* DataGroupGetChannelGroupByName(const IDataGroup* group, const char* name);
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
EXPORT const IMetaData* ChannelGroupGetMetaData(const IChannelGroup* group);
EXPORT IMetaData* ChannelGroupCreateMetaData(IChannelGroup* group);
EXPORT const ISourceInformation* ChannelGroupGetSourceInformation(const IChannelGroup* group);
EXPORT ISourceInformation* ChannelGroupCreateSourceInformation(IChannelGroup* group);

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
EXPORT const IMetaData* ChannelGetMetaData(const IChannel* channel);
EXPORT IMetaData* ChannelCreateMetaData(IChannel* channel);
EXPORT const ISourceInformation* ChannelGetSourceInformation(const IChannel* channel);
EXPORT ISourceInformation* ChannelCreateSourceInformation(IChannel* channel);
EXPORT const IChannelConversion* ChannelGetChannelConversion(const IChannel* channel);
EXPORT IChannelConversion* ChannelCreateChannelConversion(IChannel* channel);
EXPORT const IChannelArray* ChannelGetChannelArray(const IChannel* channel);
EXPORT IChannelArray* ChannelCreateChannelArray(IChannel* channel);

// IHeader functions
EXPORT size_t IHeaderGetMeasurementId(const IHeader* header, char* id, size_t max_length);
EXPORT void IHeaderSetMeasurementId(IHeader* header, const char* id);
EXPORT size_t IHeaderGetRecorderId(const IHeader* header, char* id, size_t max_length);
EXPORT void IHeaderSetRecorderId(IHeader* header, const char* id);
EXPORT int64_t IHeaderGetRecorderIndex(const IHeader* header);
EXPORT void IHeaderSetRecorderIndex(IHeader* header, int64_t index);
EXPORT bool IHeaderGetStartAngle(const IHeader* header, double* angle);
EXPORT void IHeaderSetStartAngle(IHeader* header, double angle);
EXPORT bool IHeaderGetStartDistance(const IHeader* header, double* distance);
EXPORT void IHeaderSetStartDistance(IHeader* header, double distance);
EXPORT size_t IHeaderGetAuthor(const IHeader* header, char* author, size_t max_length);
EXPORT void IHeaderSetAuthor(IHeader* header, const char* author);
EXPORT size_t IHeaderGetDepartment(const IHeader* header, char* department, size_t max_length);
EXPORT void IHeaderSetDepartment(IHeader* header, const char* department);
EXPORT size_t IHeaderGetProject(const IHeader* header, char* project, size_t max_length);
EXPORT void IHeaderSetProject(IHeader* header, const char* project);
EXPORT size_t IHeaderGetSubject(const IHeader* header, char* subject, size_t max_length);
EXPORT void IHeaderSetSubject(IHeader* header, const char* subject);
EXPORT size_t IHeaderGetDescription(const IHeader* header, char* description, size_t max_length);
EXPORT void IHeaderSetDescription(IHeader* header, const char* description);
EXPORT uint64_t IHeaderGetStartTime(const IHeader* header);
EXPORT void IHeaderSetStartTime(IHeader* header, uint64_t start_time);
EXPORT const IMetaData* IHeaderGetMetaData(const IHeader* header);
EXPORT IMetaData* IHeaderCreateMetaData(IHeader* header);
EXPORT size_t IHeaderGetAttachments(const IHeader* header, const IAttachment* attachments[], size_t max_count);
EXPORT IAttachment* IHeaderCreateAttachment(IHeader* header);
EXPORT size_t IHeaderGetFileHistories(const IHeader* header, const IFileHistory* histories[], size_t max_count);
EXPORT IFileHistory* IHeaderCreateFileHistory(IHeader* header);
EXPORT size_t IHeaderGetEvents(const IHeader* header, const IEvent* events[], size_t max_count);
EXPORT IEvent* IHeaderCreateEvent(IHeader* header);
EXPORT IDataGroup *IHeaderLastDataGroup(IHeader *header);
EXPORT size_t IHeaderGetDataGroups(const IHeader *header, const IDataGroup *groups[], size_t max_count);

// ISourceInformation functions
EXPORT uint64_t SourceInformationGetIndex(const ISourceInformation* source);
EXPORT size_t SourceInformationGetName(const ISourceInformation* source, char* name, size_t max_length);
EXPORT void SourceInformationSetName(ISourceInformation* source, const char* name);
EXPORT size_t SourceInformationGetDescription(const ISourceInformation* source, char* description, size_t max_length);
EXPORT void SourceInformationSetDescription(ISourceInformation* source, const char* description);
EXPORT size_t SourceInformationGetPath(const ISourceInformation* source, char* path, size_t max_length);
EXPORT void SourceInformationSetPath(ISourceInformation* source, const char* path);
EXPORT uint8_t SourceInformationGetType(const ISourceInformation* source);
EXPORT void SourceInformationSetType(ISourceInformation* source, uint8_t type);
EXPORT uint8_t SourceInformationGetBus(const ISourceInformation* source);
EXPORT void SourceInformationSetBus(ISourceInformation* source, uint8_t bus);
EXPORT uint8_t SourceInformationGetFlags(const ISourceInformation* source);
EXPORT void SourceInformationSetFlags(ISourceInformation* source, uint8_t flags);
EXPORT const IMetaData* SourceInformationGetMetaData(const ISourceInformation* source);
EXPORT IMetaData* SourceInformationCreateMetaData(ISourceInformation* source);

// IAttachment functions
EXPORT uint64_t AttachmentGetIndex(const IAttachment* attachment);
EXPORT uint16_t AttachmentGetCreatorIndex(const IAttachment* attachment);
EXPORT void AttachmentSetCreatorIndex(IAttachment* attachment, uint16_t index);
EXPORT bool AttachmentGetEmbedded(const IAttachment* attachment);
EXPORT void AttachmentSetEmbedded(IAttachment* attachment, bool embedded);
EXPORT bool AttachmentGetCompressed(const IAttachment* attachment);
EXPORT void AttachmentSetCompressed(IAttachment* attachment, bool compressed);
EXPORT bool AttachmentGetMd5(const IAttachment* attachment, char* md5, size_t max_length);
EXPORT size_t AttachmentGetFileName(const IAttachment* attachment, char* name, size_t max_length);
EXPORT void AttachmentSetFileName(IAttachment* attachment, const char* name);
EXPORT size_t AttachmentGetFileType(const IAttachment* attachment, char* type, size_t max_length);
EXPORT void AttachmentSetFileType(IAttachment* attachment, const char* type);
EXPORT const IMetaData* AttachmentGetMetaData(const IAttachment* attachment);
EXPORT IMetaData* AttachmentCreateMetaData(IAttachment* attachment);

// IEvent functions
EXPORT uint64_t EventGetIndex(const IEvent* event);
EXPORT size_t EventGetName(const IEvent* event, char* name, size_t max_length);
EXPORT void EventSetName(IEvent* event, const char* name);
EXPORT size_t EventGetDescription(const IEvent* event, char* description, size_t max_length);
EXPORT void EventSetDescription(IEvent* event, const char* description);
EXPORT size_t EventGetGroupName(const IEvent* event, char* group, size_t max_length);
EXPORT void EventSetGroupName(IEvent* event, const char* group);
EXPORT uint8_t EventGetType(const IEvent* event);
EXPORT void EventSetType(IEvent* event, uint8_t type);
EXPORT uint8_t EventGetSync(const IEvent* event);
EXPORT void EventSetSync(IEvent* event, uint8_t type);
EXPORT uint8_t EventGetRange(const IEvent* event);
EXPORT void EventSetRange(IEvent* event, uint8_t type);
EXPORT uint8_t EventGetCause(const IEvent* event);
EXPORT void EventSetCause(IEvent* event, uint8_t cause);
EXPORT uint16_t EventGetCreatorIndex(const IEvent* event);
EXPORT void EventSetCreatorIndex(IEvent* event, uint16_t index);
EXPORT int64_t EventGetSyncValue(const IEvent* event);
EXPORT void EventSetSyncValue(IEvent* event, int64_t value);
EXPORT double EventGetSyncFactor(const IEvent* event);
EXPORT void EventSetSyncFactor(IEvent* event, double factor);
EXPORT double EventGetPreTrig(const IEvent* event);
EXPORT void EventSetPreTrig(IEvent* event, double time);
EXPORT double EventGetPostTrig(const IEvent* event);
EXPORT void EventSetPostTrig(IEvent* event, double time);
EXPORT const IMetaData* EventGetMetaData(const IEvent* event);

// IFileHistory functions
EXPORT uint64_t FileHistoryGetIndex(const IFileHistory* file_history);
EXPORT uint64_t FileHistoryGetTime(const IFileHistory* file_history);
EXPORT void FileHistorySetTime(IFileHistory* file_history, uint64_t time);
EXPORT const IMetaData* FileHistoryGetMetaData(const IFileHistory* file_history);
EXPORT size_t FileHistoryGetDescription(const IFileHistory* file_history, char* desc, size_t max_length);
EXPORT void FileHistorySetDescription(IFileHistory* file_history, const char* desc);
EXPORT size_t FileHistoryGetToolName(const IFileHistory* file_history, char* name, size_t max_length);
EXPORT void FileHistorySetToolName(IFileHistory* file_history, const char* name);
EXPORT size_t FileHistoryGetToolVendor(const IFileHistory* file_history, char* vendor, size_t max_length);
EXPORT void FileHistorySetToolVendor(IFileHistory* file_history, const char* vendor);
EXPORT size_t FileHistoryGetToolVersion(const IFileHistory* file_history, char* version, size_t max_length);
EXPORT void FileHistorySetToolVersion(IFileHistory* file_history, const char* version);
EXPORT size_t FileHistoryGetUserName(const IFileHistory* file_history, char* user, size_t max_length);
EXPORT void FileHistorySetUserName(IFileHistory* file_history, const char* user);

// IMetaData functions
EXPORT size_t MetaDataGetPropertyAsString(const IMetaData* metadata, const char* index, char* prop, size_t max_length);
EXPORT void MetaDataSetPropertyAsString(IMetaData* metadata, const char* index, const char* prop);
EXPORT double MetaDataGetPropertyAsFloat(const IMetaData* metadata, const char* index);
EXPORT void MetaDataSetPropertyAsFloat(IMetaData* metadata, const char* index, double prop);
EXPORT size_t MetaDataGetXmlSnippet(const IMetaData* metadata, char* xml, size_t max_length);
EXPORT void MetaDataSetXmlSnippet(IMetaData* metadata, const char* xml);
EXPORT size_t MetaDataGetProperties(const IMetaData* metadata, ETag* properties[], size_t max_count);
EXPORT size_t MetaDataGetCommonProperties(const IMetaData* metadata, ETag* properties[], size_t max_count);
EXPORT void MetaDataAddCommonProperty(IMetaData* metadata, ETag* tag);

// ETag functions
EXPORT ETag* ETagInit();
EXPORT void ETagUnInit(ETag* etag);
EXPORT size_t ETagGetName(const ETag* etag, char* name, size_t max_length);
EXPORT void ETagSetName(ETag* etag, const char* name);
EXPORT size_t ETagGetDescription(const ETag* etag, char* desc, size_t max_length);
EXPORT void ETagSetDescription(ETag* etag, const char* desc);
EXPORT size_t ETagGetUnit(const ETag* etag, char* unit, size_t max_length);
EXPORT void ETagSetUnit(ETag* etag, const char* unit);
EXPORT size_t ETagGetUnitRef(const ETag* etag, char* unit, size_t max_length);
EXPORT void ETagSetUnitRef(ETag* etag, const char* unit);
EXPORT size_t ETagGetType(const ETag* etag, char* type, size_t max_length);
EXPORT void ETagSetType(ETag* etag, const char* type);
EXPORT uint8_t ETagGetDataType(const ETag* etag);
EXPORT void ETagSetDataType(ETag* etag, uint8_t type);
EXPORT size_t ETagGetLanguage(const ETag* etag, char* language, size_t max_length);
EXPORT void ETagSetLanguage(ETag* etag, const char* language);
EXPORT bool ETagGetReadOnly(const ETag* etag);
EXPORT void ETagSetReadOnly(ETag* etag, bool read_only);
EXPORT size_t ETagGetValueAsString(const ETag* etag, char* value, size_t max_length);
EXPORT void ETagSetValueAsString(ETag* etag, const char* value);
EXPORT double ETagGetValueAsFloat(const ETag* etag);
EXPORT void ETagSetValueAsFloat(ETag* etag, double value);
EXPORT bool ETagGetValueAsBoolean(const ETag* etag);
EXPORT void ETagSetValueAsBoolean(ETag* etag, bool value);
EXPORT int64_t ETagGetValueAsSigned(const ETag* etag);
EXPORT void ETagSetValueAsSigned(ETag* etag, int64_t value);
EXPORT uint64_t ETagGetValueAsUnsigned(const ETag* etag);
EXPORT void ETagSetValueAsUnsigned(ETag* etag, uint64_t value);

// IChannelArray functions
EXPORT uint64_t ChannelArrayGetIndex(const IChannelArray* array);
EXPORT uint8_t ChannelArrayGetType(const IChannelArray* array);
EXPORT void ChannelArraySetType(IChannelArray* array, uint8_t type);
EXPORT uint8_t ChannelArrayGetStorage(const IChannelArray* array);
EXPORT void ChannelArraySetStorage(IChannelArray* array, uint8_t storage);
EXPORT uint32_t ChannelArrayGetFlags(const IChannelArray* array);
EXPORT void ChannelArraySetFlags(IChannelArray* array, uint32_t flags);
EXPORT uint64_t ChannelArrayGetNofElements(const IChannelArray* array);
EXPORT void ChannelArraySetNofElements(IChannelArray* array, uint64_t elements);

// IChannelConversion functions
EXPORT uint64_t ChannelConversionGetIndex(const IChannelConversion* conversion);
EXPORT size_t ChannelConversionGetName(const IChannelConversion* conversion, char* name, size_t max_length);
EXPORT void ChannelConversionSetName(IChannelConversion* conversion, const char* name);
EXPORT size_t ChannelConversionGetDescription(const IChannelConversion* conversion, char* desc, size_t max_length);
EXPORT void ChannelConversionSetDescription(IChannelConversion* conversion, const char* desc);
EXPORT size_t ChannelConversionGetUnit(const IChannelConversion* conversion, char* unit, size_t max_length);
EXPORT void ChannelConversionSetUnit(IChannelConversion* conversion, const char* unit);
EXPORT uint8_t ChannelConversionGetType(const IChannelConversion* conversion);
EXPORT void ChannelConversionSetType(IChannelConversion* conversion, uint8_t type);
EXPORT bool ChannelConversionIsPrecisionUsed(const IChannelConversion* conversion);
EXPORT uint8_t ChannelConversionGetPrecision(const IChannelConversion* conversion);
EXPORT bool ChannelConversionIsRangeUsed(const IChannelConversion* conversion);
EXPORT double ChannelConversionGetRangeMin(const IChannelConversion* conversion);
EXPORT double ChannelConversionGetRangeMax(const IChannelConversion* conversion);
EXPORT void ChannelConversionSetRange(IChannelConversion* conversion, double min, double max);
EXPORT uint16_t ChannelConversionGetFlags(const IChannelConversion* conversion);
EXPORT size_t ChannelConversionGetFormula(const IChannelConversion* conversion, char* formula, size_t max_length);
EXPORT void ChannelConversionSetFormula(IChannelConversion* conversion, const char* formula);
EXPORT double ChannelConversionGetParameterAsDouble(const IChannelConversion* conversion, uint16_t index);
EXPORT void ChannelConversionSetParameterAsDouble(IChannelConversion* conversion, uint16_t index, double parameter);
EXPORT uint64_t ChannelConversionGetParameterAsUInt64(const IChannelConversion* conversion, uint16_t index);
EXPORT void ChannelConversionSetParameterAsUInt64(IChannelConversion* conversion, uint16_t index, uint64_t parameter);
EXPORT const IMetaData* ChannelConversionGetMetaData(const IChannelConversion* conversion);
EXPORT IMetaData* ChannelConversionCreateMetaData(IChannelConversion* conversion);

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
EXPORT uint32_t CanMessageGetBusChannel(const CanMessage* can);
EXPORT void CanMessageSetBusChannel(CanMessage* can, uint32_t busChannel);

#ifdef __cplusplus
}
#endif

#endif // MDF_C_WRAPPER_H
