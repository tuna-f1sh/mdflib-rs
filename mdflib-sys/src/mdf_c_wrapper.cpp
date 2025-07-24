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
#include <mdf/mdffile.h>
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
EXPORT MdfReader* MdfReaderInit(const char* filename) {
    return new MdfReader(filename);
}

EXPORT void MdfReaderUnInit(MdfReader* reader) {
    delete reader;
}

EXPORT int64_t MdfReaderGetIndex(MdfReader* reader) {
    return reader->Index();
}

EXPORT bool MdfReaderIsOk(MdfReader* reader) {
    return reader->IsOk();
}

EXPORT bool MdfReaderIsFinalized(MdfReader* reader) {
    return reader->IsFinalized();
}

EXPORT const MdfFile* MdfReaderGetFile(MdfReader* reader) {
    return reader->GetFile();
}

EXPORT const IHeader* MdfReaderGetHeader(MdfReader* reader) {
    return reader->GetHeader();
}

EXPORT const IDataGroup* MdfReaderGetDataGroup(MdfReader* reader, size_t index) {
    return reader->GetDataGroup(index);
}

EXPORT size_t MdfReaderGetDataGroupCount(MdfReader* reader) {
    const auto* file = reader->GetFile();
    if (!file) return 0;
    DataGroupList groups;
    file->DataGroups(groups);
    return groups.size();
}

EXPORT bool MdfReaderOpen(MdfReader* reader) {
    return reader->Open();
}

EXPORT void MdfReaderClose(MdfReader* reader) {
    reader->Close();
}

EXPORT bool MdfReaderReadHeader(MdfReader* reader) {
    return reader->ReadHeader();
}

EXPORT bool MdfReaderReadMeasurementInfo(MdfReader* reader) {
    return reader->ReadMeasurementInfo();
}

EXPORT bool MdfReaderReadEverythingButData(MdfReader* reader) {
    return reader->ReadEverythingButData();
}

EXPORT bool MdfReaderReadData(MdfReader* reader, IDataGroup* group) {
    return reader->ReadData(*group);
}

// MdfWriter functions
EXPORT MdfWriter* MdfWriterInit(MdfWriterType type, const char* filename) {
    auto* writer = MdfFactory::CreateMdfWriterEx(type);
    if (!writer) return nullptr;
    writer->Init(filename);
    return writer;
}

EXPORT void MdfWriterUnInit(MdfWriter* writer) {
    delete writer;
}

EXPORT MdfFile* MdfWriterGetFile(MdfWriter* writer) {
    return writer->GetFile();
}

EXPORT IHeader* MdfWriterGetHeader(MdfWriter* writer) {
    return writer->Header();
}

EXPORT bool MdfWriterIsFileNew(MdfWriter* writer) {
    return writer->IsFileNew();
}

EXPORT bool MdfWriterGetCompressData(MdfWriter* writer) {
    return writer->CompressData();
}

EXPORT void MdfWriterSetCompressData(MdfWriter* writer, bool compress) {
    writer->CompressData(compress);
}

EXPORT double MdfWriterGetPreTrigTime(MdfWriter* writer) {
    return writer->PreTrigTime();
}

EXPORT void MdfWriterSetPreTrigTime(MdfWriter* writer, double pre_trig_time) {
    writer->PreTrigTime(pre_trig_time);
}

EXPORT uint64_t MdfWriterGetStartTime(MdfWriter* writer) {
    return writer->StartTime();
}

EXPORT uint64_t MdfWriterGetStopTime(MdfWriter* writer) {
    return writer->StopTime();
}

EXPORT uint16_t MdfWriterGetBusType(MdfWriter* writer) {
    return writer->BusType();
}

EXPORT void MdfWriterSetBusType(MdfWriter* writer, uint16_t type) {
    writer->BusType(type);
}

EXPORT bool MdfWriterCreateBusLogConfiguration(MdfWriter* writer) {
    return writer->CreateBusLogConfiguration();
}

EXPORT IDataGroup* MdfWriterCreateDataGroup(MdfWriter* writer) {
    return writer->CreateDataGroup();
}

EXPORT bool MdfWriterInitMeasurement(MdfWriter* writer) {
    return writer->InitMeasurement();
}

EXPORT void MdfWriterSaveSample(MdfWriter* writer, IChannelGroup* group, uint64_t time) {
    writer->SaveSample(*group, time);
}

EXPORT void MdfWriterSaveCanMessage(MdfWriter* writer, IChannelGroup* group, uint64_t time, CanMessage* message) {
    writer->SaveCanMessage(*group, time, *message);
}

EXPORT void MdfWriterStartMeasurement(MdfWriter* writer, uint64_t start_time) {
    writer->StartMeasurement(start_time);
}

EXPORT void MdfWriterStopMeasurement(MdfWriter* writer, uint64_t stop_time) {
    writer->StopMeasurement(stop_time);
}

EXPORT bool MdfWriterFinalizeMeasurement(MdfWriter* writer) {
    return writer->FinalizeMeasurement();
}

// MdfFile functions
EXPORT size_t MdfFileGetName(const MdfFile* file, char* name, size_t max_length) {
    const std::string& file_name = file->Name();
    size_t copy_length = std::min(file_name.length(), max_length - 1);
    if (name && max_length > 0) {
        std::memcpy(name, file_name.c_str(), copy_length);
        name[copy_length] = '\0';
    }
    return file_name.length();
}

EXPORT void MdfFileSetName(MdfFile* file, const char* name) {
    file->Name(name);
}

EXPORT size_t MdfFileGetFileName(const MdfFile* file, char* filename, size_t max_length) {
    const std::string& file_name = file->FileName();
    size_t copy_length = std::min(file_name.length(), max_length - 1);
    if (filename && max_length > 0) {
        std::memcpy(filename, file_name.c_str(), copy_length);
        filename[copy_length] = '\0';
    }
    return file_name.length();
}

EXPORT void MdfFileSetFileName(MdfFile* file, const char* filename) {
    file->FileName(filename);
}

EXPORT size_t MdfFileGetVersion(const MdfFile* file, char* version, size_t max_length) {
    const std::string& ver = file->Version();
    size_t copy_length = std::min(ver.length(), max_length - 1);
    if (version && max_length > 0) {
        std::memcpy(version, ver.c_str(), copy_length);
        version[copy_length] = '\0';
    }
    return ver.length();
}

EXPORT int MdfFileGetMainVersion(const MdfFile* file) {
    return file->MainVersion();
}

EXPORT int MdfFileGetMinorVersion(const MdfFile* file) {
    return file->MinorVersion();
}

EXPORT void MdfFileSetMinorVersion(MdfFile* file, int minor) {
    file->MinorVersion(minor);
}

EXPORT const IHeader* MdfFileGetHeader(const MdfFile* file) {
    return file->Header();
}

EXPORT bool MdfFileGetIsMdf4(const MdfFile* file) {
    return file->IsMdf4();
}

EXPORT size_t MdfFileGetDataGroupCount(const MdfFile* file) {
    DataGroupList groups;
    file->DataGroups(groups);
    return groups.size();
}

EXPORT const IDataGroup* MdfFileGetDataGroupByIndex(const MdfFile* file, size_t index) {
    DataGroupList groups;
    file->DataGroups(groups);
    if (index >= groups.size()) return nullptr;
    return groups[index];
}

EXPORT IDataGroup* MdfFileCreateDataGroup(MdfFile* file) {
    return file->CreateDataGroup();
}

// IDataGroup functions
EXPORT uint64_t DataGroupGetIndex(const IDataGroup* group) {
    return group->Index();
}

EXPORT size_t DataGroupGetDescription(const IDataGroup* group, char* description, size_t max_length) {
    const std::string& desc = group->Description();
    size_t copy_length = std::min(desc.length(), max_length - 1);
    if (description && max_length > 0) {
        std::memcpy(description, desc.c_str(), copy_length);
        description[copy_length] = '\0';
    }
    return desc.length();
}

EXPORT void DataGroupSetDescription(IDataGroup* group, const char* description) {
    group->Description(description);
}

EXPORT size_t DataGroupGetChannelGroupCount(const IDataGroup* group) {
    const auto& channel_groups = group->ChannelGroups();
    return channel_groups.size();
}

EXPORT const IChannelGroup* DataGroupGetChannelGroupByIndex(const IDataGroup* group, size_t index) {
    const auto& channel_groups = group->ChannelGroups();
    if (index >= channel_groups.size()) return nullptr;
    return channel_groups[index];
}

EXPORT IChannelGroup* DataGroupCreateChannelGroup(IDataGroup* group) {
    return group->CreateChannelGroup();
}

// IChannelGroup functions
EXPORT uint64_t ChannelGroupGetIndex(const IChannelGroup* group) {
    return group->Index();
}

EXPORT size_t ChannelGroupGetName(const IChannelGroup* group, char* name, size_t max_length) {
    const std::string& group_name = group->Name();
    size_t copy_length = std::min(group_name.length(), max_length - 1);
    if (name && max_length > 0) {
        std::memcpy(name, group_name.c_str(), copy_length);
        name[copy_length] = '\0';
    }
    return group_name.length();
}

EXPORT void ChannelGroupSetName(IChannelGroup* group, const char* name) {
    group->Name(name);
}

EXPORT size_t ChannelGroupGetDescription(const IChannelGroup* group, char* description, size_t max_length) {
    const std::string& desc = group->Description();
    size_t copy_length = std::min(desc.length(), max_length - 1);
    if (description && max_length > 0) {
        std::memcpy(description, desc.c_str(), copy_length);
        description[copy_length] = '\0';
    }
    return desc.length();
}

EXPORT void ChannelGroupSetDescription(IChannelGroup* group, const char* description) {
    group->Description(description);
}

EXPORT uint64_t ChannelGroupGetNofSamples(const IChannelGroup* group) {
    return group->NofSamples();
}

EXPORT void ChannelGroupSetNofSamples(IChannelGroup* group, uint64_t samples) {
    group->NofSamples(samples);
}

EXPORT size_t ChannelGroupGetChannelCount(const IChannelGroup* group) {
    const auto& channels = group->Channels();
    return channels.size();
}

EXPORT const IChannel* ChannelGroupGetChannelByIndex(const IChannelGroup* group, size_t index) {
    const auto& channels = group->Channels();
    if (index >= channels.size()) return nullptr;
    return channels[index];
}

EXPORT IChannel* ChannelGroupCreateChannel(IChannelGroup* group) {
    return group->CreateChannel();
}

// IChannel functions
EXPORT uint64_t ChannelGetIndex(const IChannel* channel) {
    return channel->Index();
}

EXPORT size_t ChannelGetName(const IChannel* channel, char* name, size_t max_length) {
    const std::string& channel_name = channel->Name();
    size_t copy_length = std::min(channel_name.length(), max_length - 1);
    if (name && max_length > 0) {
        std::memcpy(name, channel_name.c_str(), copy_length);
        name[copy_length] = '\0';
    }
    return channel_name.length();
}

EXPORT void ChannelSetName(IChannel* channel, const char* name) {
    channel->Name(name);
}

EXPORT size_t ChannelGetDisplayName(const IChannel* channel, char* display_name, size_t max_length) {
    const std::string& name = channel->DisplayName();
    size_t copy_length = std::min(name.length(), max_length - 1);
    if (display_name && max_length > 0) {
        std::memcpy(display_name, name.c_str(), copy_length);
        display_name[copy_length] = '\0';
    }
    return name.length();
}

EXPORT void ChannelSetDisplayName(IChannel* channel, const char* display_name) {
    channel->DisplayName(display_name);
}

EXPORT size_t ChannelGetDescription(const IChannel* channel, char* description, size_t max_length) {
    const std::string& desc = channel->Description();
    size_t copy_length = std::min(desc.length(), max_length - 1);
    if (description && max_length > 0) {
        std::memcpy(description, desc.c_str(), copy_length);
        description[copy_length] = '\0';
    }
    return desc.length();
}

EXPORT void ChannelSetDescription(IChannel* channel, const char* description) {
    channel->Description(description);
}

EXPORT size_t ChannelGetUnit(const IChannel* channel, char* unit, size_t max_length) {
    const std::string& unit_str = channel->Unit();
    size_t copy_length = std::min(unit_str.length(), max_length - 1);
    if (unit && max_length > 0) {
        std::memcpy(unit, unit_str.c_str(), copy_length);
        unit[copy_length] = '\0';
    }
    return unit_str.length();
}

EXPORT void ChannelSetUnit(IChannel* channel, const char* unit) {
    channel->Unit(unit);
}

EXPORT uint8_t ChannelGetType(const IChannel* channel) {
    return static_cast<uint8_t>(channel->Type());
}

EXPORT void ChannelSetType(IChannel* channel, uint8_t type) {
    channel->Type(static_cast<ChannelType>(type));
}

EXPORT uint8_t ChannelGetDataType(const IChannel* channel) {
    return static_cast<uint8_t>(channel->DataType());
}

EXPORT void ChannelSetDataType(IChannel* channel, uint8_t data_type) {
    channel->DataType(static_cast<ChannelDataType>(data_type));
}

EXPORT uint64_t ChannelGetDataBytes(const IChannel* channel) {
    return channel->DataBytes();
}

EXPORT void ChannelSetDataBytes(IChannel* channel, uint64_t bytes) {
    channel->DataBytes(bytes);
}

EXPORT void ChannelSetChannelValue(IChannel* channel, uint32_t value, bool valid) {
    if (channel) {
        channel->SetChannelValue(value, valid);
    }
}

// CanMessage functions
EXPORT CanMessage* CanMessageInit() {
    return new CanMessage;
}

EXPORT void CanMessageUnInit(CanMessage* can) {
    delete can;
}

EXPORT uint32_t CanMessageGetMessageId(CanMessage* can) {
    return can->MessageId();
}

EXPORT void CanMessageSetMessageId(CanMessage* can, uint32_t msgId) {
    can->MessageId(msgId);
}

EXPORT uint32_t CanMessageGetCanId(CanMessage* can) {
    return can->CanId();
}

EXPORT bool CanMessageGetExtendedId(CanMessage* can) {
    return can->ExtendedId();
}

EXPORT void CanMessageSetExtendedId(CanMessage* can, bool extendedId) {
    can->ExtendedId(extendedId);
}

EXPORT uint8_t CanMessageGetDlc(CanMessage* can) {
    return can->Dlc();
}

EXPORT void CanMessageSetDlc(CanMessage* can, uint8_t dlc) {
    can->Dlc(dlc);
}

EXPORT size_t CanMessageGetDataLength(CanMessage* can) {
    return can->DataLength();
}

EXPORT void CanMessageSetDataLength(CanMessage* can, uint32_t dataLength) {
    can->DataLength(dataLength);
}

EXPORT size_t CanMessageGetDataBytes(CanMessage* can, uint8_t* dataList, size_t max_length) {
    const auto& data = can->DataBytes();
    size_t copy_length = std::min(data.size(), max_length);
    if (dataList && max_length > 0) {
        std::memcpy(dataList, data.data(), copy_length);
    }
    return data.size();
}

EXPORT void CanMessageSetDataBytes(CanMessage* can, const uint8_t* dataList, size_t size) {
    std::vector<uint8_t> data(dataList, dataList + size);
    can->DataBytes(data);
}

} // extern "C"
