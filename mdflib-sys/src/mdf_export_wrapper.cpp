/*
 * Comprehensive C export wrapper for mdflib
 * Based on MdfExport.cpp from mdflib
 */

#include <mdf/ichannelgroup.h>
#include <mdf/idatagroup.h>
#include <mdf/ievent.h>
#include <mdf/ifilehistory.h>
#include <mdf/mdffactory.h>
#include <mdf/mdfreader.h>
#include <mdf/mdfwriter.h>
#include <mdf/canmessage.h>
#include <mdf/ichannel.h>
#include <mdf/ichannelconversion.h>
#include <mdf/ichannelarray.h>
#include <mdf/isourceinformation.h>
#include <mdf/iattachment.h>
#include <mdf/imetadata.h>

using namespace mdf;

// Export macros for different platforms
#if defined(_WIN32)
#define EXPORT __declspec(dllexport)
#elif defined(__linux__) || defined(__APPLE__) || defined(__CYGWIN__)
#define EXPORT __attribute__((visibility("default")))
#else
#define EXPORT
#endif

extern "C" {

// MdfReader functions
EXPORT mdf::MdfReader* MdfReaderInit(const char* filename) {
    return new MdfReader(filename);
}

EXPORT void MdfReaderUnInit(mdf::MdfReader* reader) {
    delete reader;
}

EXPORT int64_t MdfReaderGetIndex(mdf::MdfReader* reader) {
    return reader->Index();
}

EXPORT bool MdfReaderIsOk(mdf::MdfReader* reader) {
    return reader->IsOk();
}

EXPORT const mdf::MdfFile* MdfReaderGetFile(mdf::MdfReader* reader) {
    return reader->GetFile();
}

EXPORT const mdf::IHeader* MdfReaderGetHeader(mdf::MdfReader* reader) {
    return reader->GetHeader();
}

EXPORT const mdf::IDataGroup* MdfReaderGetDataGroup(mdf::MdfReader* reader, size_t index) {
    return reader->GetDataGroup(index);
}

EXPORT size_t MdfReaderGetDataGroupCount(mdf::MdfReader* reader) {
    const auto* file = reader->GetFile();
    if (!file) return 0;
    const auto& groups = file->DataGroups();
    return groups.size();
}

EXPORT bool MdfReaderOpen(mdf::MdfReader* reader) {
    return reader->Open();
}

EXPORT void MdfReaderClose(mdf::MdfReader* reader) {
    reader->Close();
}

EXPORT bool MdfReaderReadHeader(mdf::MdfReader* reader) {
    return reader->ReadHeader();
}

EXPORT bool MdfReaderReadMeasurementInfo(mdf::MdfReader* reader) {
    return reader->ReadMeasurementInfo();
}

EXPORT bool MdfReaderReadEverythingButData(mdf::MdfReader* reader) {
    return reader->ReadEverythingButData();
}

EXPORT bool MdfReaderReadData(mdf::MdfReader* reader, mdf::IDataGroup* group) {
    return reader->ReadData(*group);
}

// MdfWriter functions
EXPORT mdf::MdfWriter* MdfWriterInit(MdfWriterType type, const char* filename) {
    auto* writer = MdfFactory::CreateMdfWriterEx(type);
    if (!writer) return nullptr;
    writer->Init(filename);
    return writer;
}

EXPORT void MdfWriterUnInit(mdf::MdfWriter* writer) {
    delete writer;
}

EXPORT mdf::MdfFile* MdfWriterGetFile(mdf::MdfWriter* writer) {
    return writer->GetFile();
}

EXPORT mdf::IHeader* MdfWriterGetHeader(mdf::MdfWriter* writer) {
    return writer->Header();
}

EXPORT bool MdfWriterIsFileNew(mdf::MdfWriter* writer) {
    return writer->IsFileNew();
}

EXPORT bool MdfWriterGetCompressData(mdf::MdfWriter* writer) {
    return writer->CompressData();
}

EXPORT void MdfWriterSetCompressData(mdf::MdfWriter* writer, bool compress) {
    writer->CompressData(compress);
}

EXPORT double MdfWriterGetPreTrigTime(mdf::MdfWriter* writer) {
    return writer->PreTrigTime();
}

EXPORT void MdfWriterSetPreTrigTime(mdf::MdfWriter* writer, double pre_trig_time) {
    writer->PreTrigTime(pre_trig_time);
}

EXPORT uint64_t MdfWriterGetStartTime(mdf::MdfWriter* writer) {
    return writer->StartTime();
}

EXPORT uint64_t MdfWriterGetStopTime(mdf::MdfWriter* writer) {
    return writer->StopTime();
}

EXPORT uint16_t MdfWriterGetBusType(mdf::MdfWriter* writer) {
    return writer->BusType();
}

EXPORT void MdfWriterSetBusType(mdf::MdfWriter* writer, uint16_t type) {
    writer->BusType(type);
}

EXPORT bool MdfWriterCreateBusLogConfiguration(mdf::MdfWriter* writer) {
    return writer->CreateBusLogConfiguration();
}

EXPORT mdf::IDataGroup* MdfWriterCreateDataGroup(mdf::MdfWriter* writer) {
    return writer->CreateDataGroup();
}

EXPORT bool MdfWriterInitMeasurement(mdf::MdfWriter* writer) {
    return writer->InitMeasurement();
}

EXPORT void MdfWriterSaveSample(mdf::MdfWriter* writer, mdf::IChannelGroup* group, uint64_t time) {
    writer->SaveSample(*group, time);
}

EXPORT void MdfWriterSaveCanMessage(mdf::MdfWriter* writer, mdf::IChannelGroup* group, uint64_t time, mdf::CanMessage* message) {
    writer->SaveCanMessage(*group, time, *message);
}

EXPORT void MdfWriterStartMeasurement(mdf::MdfWriter* writer, uint64_t start_time) {
    writer->StartMeasurement(start_time);
}

EXPORT void MdfWriterStopMeasurement(mdf::MdfWriter* writer, uint64_t stop_time) {
    writer->StopMeasurement(stop_time);
}

EXPORT bool MdfWriterFinalizeMeasurement(mdf::MdfWriter* writer) {
    return writer->FinalizeMeasurement();
}

// MdfFile functions
EXPORT size_t MdfFileGetName(mdf::MdfFile* file, char* name, size_t max_length) {
    const std::string& file_name = file->Name();
    size_t copy_length = std::min(file_name.length(), max_length - 1);
    if (name && max_length > 0) {
        std::memcpy(name, file_name.c_str(), copy_length);
        name[copy_length] = '\0';
    }
    return file_name.length();
}

EXPORT void MdfFileSetName(mdf::MdfFile* file, const char* name) {
    file->Name(name);
}

EXPORT size_t MdfFileGetFileName(mdf::MdfFile* file, char* filename, size_t max_length) {
    const std::string& file_name = file->FileName();
    size_t copy_length = std::min(file_name.length(), max_length - 1);
    if (filename && max_length > 0) {
        std::memcpy(filename, file_name.c_str(), copy_length);
        filename[copy_length] = '\0';
    }
    return file_name.length();
}

EXPORT void MdfFileSetFileName(mdf::MdfFile* file, const char* filename) {
    file->FileName(filename);
}

EXPORT size_t MdfFileGetVersion(mdf::MdfFile* file, char* version, size_t max_length) {
    const std::string& ver = file->Version();
    size_t copy_length = std::min(ver.length(), max_length - 1);
    if (version && max_length > 0) {
        std::memcpy(version, ver.c_str(), copy_length);
        version[copy_length] = '\0';
    }
    return ver.length();
}

EXPORT int MdfFileGetMainVersion(mdf::MdfFile* file) {
    return file->MainVersion();
}

EXPORT int MdfFileGetMinorVersion(mdf::MdfFile* file) {
    return file->MinorVersion();
}

EXPORT void MdfFileSetMinorVersion(mdf::MdfFile* file, int minor) {
    file->MinorVersion(minor);
}

EXPORT const mdf::IHeader* MdfFileGetHeader(mdf::MdfFile* file) {
    return file->Header();
}

EXPORT bool MdfFileGetIsMdf4(mdf::MdfFile* file) {
    return file->IsMdf4();
}

EXPORT size_t MdfFileGetDataGroupCount(mdf::MdfFile* file) {
    const auto& groups = file->DataGroups();
    return groups.size();
}

EXPORT const mdf::IDataGroup* MdfFileGetDataGroupByIndex(mdf::MdfFile* file, size_t index) {
    const auto& groups = file->DataGroups();
    if (index >= groups.size()) return nullptr;
    return groups[index].get();
}

EXPORT mdf::IDataGroup* MdfFileCreateDataGroup(mdf::MdfFile* file) {
    return file->CreateDataGroup();
}

// IDataGroup functions
EXPORT uint64_t DataGroupGetIndex(const mdf::IDataGroup* group) {
    return group->Index();
}

EXPORT size_t DataGroupGetName(const mdf::IDataGroup* group, char* name, size_t max_length) {
    const std::string& group_name = group->Name();
    size_t copy_length = std::min(group_name.length(), max_length - 1);
    if (name && max_length > 0) {
        std::memcpy(name, group_name.c_str(), copy_length);
        name[copy_length] = '\0';
    }
    return group_name.length();
}

EXPORT void DataGroupSetName(mdf::IDataGroup* group, const char* name) {
    group->Name(name);
}

EXPORT size_t DataGroupGetDescription(const mdf::IDataGroup* group, char* description, size_t max_length) {
    const std::string& desc = group->Description();
    size_t copy_length = std::min(desc.length(), max_length - 1);
    if (description && max_length > 0) {
        std::memcpy(description, desc.c_str(), copy_length);
        description[copy_length] = '\0';
    }
    return desc.length();
}

EXPORT void DataGroupSetDescription(mdf::IDataGroup* group, const char* description) {
    group->Description(description);
}

EXPORT size_t DataGroupGetChannelGroupCount(const mdf::IDataGroup* group) {
    const auto& channel_groups = group->ChannelGroups();
    return channel_groups.size();
}

EXPORT const mdf::IChannelGroup* DataGroupGetChannelGroupByIndex(const mdf::IDataGroup* group, size_t index) {
    const auto& channel_groups = group->ChannelGroups();
    if (index >= channel_groups.size()) return nullptr;
    return channel_groups[index].get();
}

EXPORT mdf::IChannelGroup* DataGroupCreateChannelGroup(mdf::IDataGroup* group) {
    return group->CreateChannelGroup();
}

// IChannelGroup functions
EXPORT uint64_t ChannelGroupGetIndex(const mdf::IChannelGroup* group) {
    return group->Index();
}

EXPORT size_t ChannelGroupGetName(const mdf::IChannelGroup* group, char* name, size_t max_length) {
    const std::string& group_name = group->Name();
    size_t copy_length = std::min(group_name.length(), max_length - 1);
    if (name && max_length > 0) {
        std::memcpy(name, group_name.c_str(), copy_length);
        name[copy_length] = '\0';
    }
    return group_name.length();
}

EXPORT void ChannelGroupSetName(mdf::IChannelGroup* group, const char* name) {
    group->Name(name);
}

EXPORT size_t ChannelGroupGetDescription(const mdf::IChannelGroup* group, char* description, size_t max_length) {
    const std::string& desc = group->Description();
    size_t copy_length = std::min(desc.length(), max_length - 1);
    if (description && max_length > 0) {
        std::memcpy(description, desc.c_str(), copy_length);
        description[copy_length] = '\0';
    }
    return desc.length();
}

EXPORT void ChannelGroupSetDescription(mdf::IChannelGroup* group, const char* description) {
    group->Description(description);
}

EXPORT uint64_t ChannelGroupGetNofSamples(const mdf::IChannelGroup* group) {
    return group->NofSamples();
}

EXPORT void ChannelGroupSetNofSamples(mdf::IChannelGroup* group, uint64_t samples) {
    group->NofSamples(samples);
}

EXPORT size_t ChannelGroupGetChannelCount(const mdf::IChannelGroup* group) {
    const auto& channels = group->Channels();
    return channels.size();
}

EXPORT const mdf::IChannel* ChannelGroupGetChannelByIndex(const mdf::IChannelGroup* group, size_t index) {
    const auto& channels = group->Channels();
    if (index >= channels.size()) return nullptr;
    return channels[index].get();
}

EXPORT mdf::IChannel* ChannelGroupCreateChannel(mdf::IChannelGroup* group) {
    return group->CreateChannel();
}

// IChannel functions
EXPORT uint64_t ChannelGetIndex(const mdf::IChannel* channel) {
    return channel->Index();
}

EXPORT size_t ChannelGetName(const mdf::IChannel* channel, char* name, size_t max_length) {
    const std::string& channel_name = channel->Name();
    size_t copy_length = std::min(channel_name.length(), max_length - 1);
    if (name && max_length > 0) {
        std::memcpy(name, channel_name.c_str(), copy_length);
        name[copy_length] = '\0';
    }
    return channel_name.length();
}

EXPORT void ChannelSetName(mdf::IChannel* channel, const char* name) {
    channel->Name(name);
}

EXPORT size_t ChannelGetDisplayName(const mdf::IChannel* channel, char* display_name, size_t max_length) {
    const std::string& name = channel->DisplayName();
    size_t copy_length = std::min(name.length(), max_length - 1);
    if (display_name && max_length > 0) {
        std::memcpy(display_name, name.c_str(), copy_length);
        display_name[copy_length] = '\0';
    }
    return name.length();
}

EXPORT void ChannelSetDisplayName(mdf::IChannel* channel, const char* display_name) {
    channel->DisplayName(display_name);
}

EXPORT size_t ChannelGetDescription(const mdf::IChannel* channel, char* description, size_t max_length) {
    const std::string& desc = channel->Description();
    size_t copy_length = std::min(desc.length(), max_length - 1);
    if (description && max_length > 0) {
        std::memcpy(description, desc.c_str(), copy_length);
        description[copy_length] = '\0';
    }
    return desc.length();
}

EXPORT void ChannelSetDescription(mdf::IChannel* channel, const char* description) {
    channel->Description(description);
}

EXPORT size_t ChannelGetUnit(const mdf::IChannel* channel, char* unit, size_t max_length) {
    const std::string& unit_str = channel->Unit();
    size_t copy_length = std::min(unit_str.length(), max_length - 1);
    if (unit && max_length > 0) {
        std::memcpy(unit, unit_str.c_str(), copy_length);
        unit[copy_length] = '\0';
    }
    return unit_str.length();
}

EXPORT void ChannelSetUnit(mdf::IChannel* channel, const char* unit) {
    channel->Unit(unit);
}

EXPORT uint8_t ChannelGetType(const mdf::IChannel* channel) {
    return static_cast<uint8_t>(channel->Type());
}

EXPORT void ChannelSetType(mdf::IChannel* channel, uint8_t type) {
    channel->Type(static_cast<ChannelType>(type));
}

EXPORT uint8_t ChannelGetDataType(const mdf::IChannel* channel) {
    return static_cast<uint8_t>(channel->DataType());
}

EXPORT void ChannelSetDataType(mdf::IChannel* channel, uint8_t data_type) {
    channel->DataType(static_cast<ChannelDataType>(data_type));
}

EXPORT uint64_t ChannelGetDataBytes(const mdf::IChannel* channel) {
    return channel->DataBytes();
}

EXPORT void ChannelSetDataBytes(mdf::IChannel* channel, uint64_t bytes) {
    channel->DataBytes(bytes);
}

EXPORT bool ChannelGetChannelValue(const mdf::IChannel* channel, uint64_t sample, double* value) {
    if (!value) return false;
    return channel->GetChannelValue(sample, *value);
}

EXPORT bool ChannelGetEngValue(const mdf::IChannel* channel, uint64_t sample, double* value) {
    if (!value) return false;
    return channel->GetEngValue(sample, *value);
}

// CanMessage functions
EXPORT mdf::CanMessage* CanMessageInit() {
    return new mdf::CanMessage;
}

EXPORT void CanMessageUnInit(mdf::CanMessage* can) {
    delete can;
}

EXPORT uint32_t CanMessageGetMessageId(mdf::CanMessage* can) {
    return can->MessageId();
}

EXPORT void CanMessageSetMessageId(mdf::CanMessage* can, uint32_t msgId) {
    can->MessageId(msgId);
}

EXPORT uint32_t CanMessageGetCanId(mdf::CanMessage* can) {
    return can->CanId();
}

EXPORT bool CanMessageGetExtendedId(mdf::CanMessage* can) {
    return can->ExtendedId();
}

EXPORT void CanMessageSetExtendedId(mdf::CanMessage* can, bool extendedId) {
    can->ExtendedId(extendedId);
}

EXPORT uint8_t CanMessageGetDlc(mdf::CanMessage* can) {
    return can->Dlc();
}

EXPORT void CanMessageSetDlc(mdf::CanMessage* can, uint8_t dlc) {
    can->Dlc(dlc);
}

EXPORT size_t CanMessageGetDataLength(mdf::CanMessage* can) {
    return can->DataLength();
}

EXPORT void CanMessageSetDataLength(mdf::CanMessage* can, uint32_t dataLength) {
    can->DataLength(dataLength);
}

EXPORT size_t CanMessageGetDataBytes(mdf::CanMessage* can, uint8_t* dataList, size_t max_length) {
    const auto& data = can->DataBytes();
    size_t copy_length = std::min(data.size(), max_length);
    if (dataList && max_length > 0) {
        std::memcpy(dataList, data.data(), copy_length);
    }
    return data.size();
}

EXPORT void CanMessageSetDataBytes(mdf::CanMessage* can, const uint8_t* dataList, size_t size) {
    std::vector<uint8_t> data(dataList, dataList + size);
    can->DataBytes(data);
}

EXPORT uint64_t CanMessageGetTime(mdf::CanMessage* can) {
    return can->Time();
}

EXPORT void CanMessageSetTime(mdf::CanMessage* can, uint64_t time) {
    can->Time(time);
}

} // extern "C"
