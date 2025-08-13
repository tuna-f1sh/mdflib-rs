/*
 * Comprehensive C export wrapper for mdflib
 * Based on MdfExport.cpp from mdflib
 */

// #include "mdf_c_wrapper.h"

#include <mdf/canmessage.h>
#include <mdf/etag.h>
#include <mdf/iattachment.h>
#include <mdf/ichannel.h>
#include <mdf/ichannelarray.h>
#include <mdf/ichannelconversion.h>
#include <mdf/ichannelgroup.h>
#include <mdf/ichannelobserver.h>
#include <mdf/idatagroup.h>
#include <mdf/ievent.h>
#include <mdf/ifilehistory.h>
#include <mdf/imetadata.h>
#include <mdf/isourceinformation.h>
#include <mdf/mdffactory.h>
#include <mdf/mdffile.h>
#include <mdf/mdfreader.h>
#include <mdf/mdfwriter.h>
#include <mdf/mdflogstream.h>

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

// Global function pointers for C-style callbacks
typedef void (*MdfCLogFunction1)(MdfLogSeverity severity, const uint8_t* text);
typedef void (*MdfCLogFunction2)(MdfLogSeverity severity, const char* function, const uint8_t* text);
static MdfCLogFunction1 g_c_log_function1 = nullptr;
static MdfCLogFunction2 g_c_log_function2 = nullptr;

// C++ wrapper for the MdfLogFunction1 callback
void MdfLogWrapper1(const MdfLocation &location, MdfLogSeverity severity, const std::string& text) {
    if (g_c_log_function1) {
        MdfLocation c_location = {location.line, location.column, location.file.c_str(), location.function.c_str()};
        g_c_log_function1(static_cast<MdfLogSeverity>(severity), (const uint8_t*) text.c_str());
    }
}

// C++ wrapper for the MdfLogFunction2 callback
void MdfLogWrapper2(MdfLogSeverity severity, const std::string& function, const std::string& text) {
    if (g_c_log_function2) {
        g_c_log_function2(static_cast<MdfLogSeverity>(severity), function.c_str(), (const uint8_t*) text.c_str());
    }
}

// Function to set the C-style log function 1
EXPORT void MdfSetLogFunction1(MdfCLogFunction1 func) {
    g_c_log_function1 = func;
    if (func) {
        MdfLogStream::SetLogFunction1(MdfLogWrapper1);
    } else {
        MdfLogStream::SetLogFunction1(nullptr);
    }
}

// Function to set the C-style log function 2
EXPORT void MdfSetLogFunction2(MdfCLogFunction2 func) {
    g_c_log_function2 = func;
    if (func) {
        MdfLogStream::SetLogFunction2(MdfLogWrapper2);
    } else {
        MdfLogStream::SetLogFunction2(nullptr);
    }
}

// MdfReader functions
EXPORT MdfReader *MdfReaderInit(const char *filename) {
  return new MdfReader(filename);
}

EXPORT void MdfReaderUnInit(MdfReader *reader) { delete reader; }

EXPORT int64_t MdfReaderGetIndex(MdfReader *reader) { return reader->Index(); }

EXPORT bool MdfReaderIsOk(MdfReader *reader) { return reader->IsOk(); }

EXPORT bool MdfReaderIsFinalized(MdfReader *reader) {
  return reader->IsFinalized();
}

EXPORT const MdfFile *MdfReaderGetFile(MdfReader *reader) {
  return reader->GetFile();
}

EXPORT const IHeader *MdfReaderGetHeader(MdfReader *reader) {
  return reader->GetHeader();
}

EXPORT const IDataGroup *MdfReaderGetDataGroup(MdfReader *reader,
                                               size_t index) {
  return reader->GetDataGroup(index);
}

EXPORT size_t MdfReaderGetDataGroupCount(MdfReader *reader) {
  const auto *file = reader->GetFile();
  if (!file)
    return 0;
  DataGroupList groups;
  file->DataGroups(groups);
  return groups.size();
}

EXPORT bool MdfReaderOpen(MdfReader *reader) { return reader->Open(); }

EXPORT void MdfReaderClose(MdfReader *reader) { reader->Close(); }

EXPORT bool MdfReaderReadHeader(MdfReader *reader) {
  return reader->ReadHeader();
}

EXPORT bool MdfReaderReadMeasurementInfo(MdfReader *reader) {
  return reader->ReadMeasurementInfo();
}

EXPORT bool MdfReaderReadEverythingButData(MdfReader *reader) {
  return reader->ReadEverythingButData();
}

EXPORT bool MdfReaderReadData(MdfReader *reader, IDataGroup *group) {
  return reader->ReadData(*group);
}

// MdfWriter functions
EXPORT MdfWriter *MdfWriterInit(MdfWriterType type, const char *filename) {
  auto *writer = MdfFactory::CreateMdfWriterEx(type);
  if (!writer)
    return nullptr;
  writer->Init(filename);
  return writer;
}

EXPORT void MdfWriterUnInit(MdfWriter *writer) { delete writer; }

EXPORT MdfFile *MdfWriterGetFile(MdfWriter *writer) {
  return writer->GetFile();
}

EXPORT IHeader *MdfWriterGetHeader(MdfWriter *writer) {
  return writer->Header();
}

EXPORT bool MdfWriterIsFileNew(MdfWriter *writer) {
  return writer->IsFileNew();
}

EXPORT bool MdfWriterGetCompressData(MdfWriter *writer) {
  return writer->CompressData();
}

EXPORT void MdfWriterSetCompressData(MdfWriter *writer, bool compress) {
  writer->CompressData(compress);
}

EXPORT double MdfWriterGetPreTrigTime(MdfWriter *writer) {
  return writer->PreTrigTime();
}

EXPORT void MdfWriterSetPreTrigTime(MdfWriter *writer, double pre_trig_time) {
  writer->PreTrigTime(pre_trig_time);
}

EXPORT uint64_t MdfWriterGetStartTime(MdfWriter *writer) {
  return writer->StartTime();
}

EXPORT uint64_t MdfWriterGetStopTime(MdfWriter *writer) {
  return writer->StopTime();
}

EXPORT uint16_t MdfWriterGetBusType(MdfWriter *writer) {
  return writer->BusType();
}

EXPORT void MdfWriterSetBusType(MdfWriter *writer, uint16_t type) {
  writer->BusType(type);
}

EXPORT bool MdfWriterCreateBusLogConfiguration(MdfWriter *writer) {
  return writer->CreateBusLogConfiguration();
}

EXPORT IDataGroup *MdfWriterCreateDataGroup(MdfWriter *writer) {
  return writer->CreateDataGroup();
}

EXPORT bool MdfWriterInitMeasurement(MdfWriter *writer) {
  return writer->InitMeasurement();
}

EXPORT void MdfWriterSaveSample(MdfWriter *writer, const IChannelGroup *group,
                                uint64_t time) {
  writer->SaveSample(*group, time);
}

EXPORT void MdfWriterSaveCanMessage(MdfWriter *writer, const IChannelGroup *group,
                                    uint64_t time, const CanMessage *message) {
  writer->SaveCanMessage(*group, time, *message);
}

EXPORT void MdfWriterStartMeasurement(MdfWriter *writer, uint64_t start_time) {
  writer->StartMeasurement(start_time);
}

EXPORT void MdfWriterStopMeasurement(MdfWriter *writer, uint64_t stop_time) {
  writer->StopMeasurement(stop_time);
}

EXPORT bool MdfWriterFinalizeMeasurement(MdfWriter *writer) {
  return writer->FinalizeMeasurement();
}

// MdfFile functions
EXPORT size_t MdfFileGetName(const MdfFile *file, char *name,
                             size_t max_length) {
  const std::string &file_name = file->Name();
  size_t copy_length = std::min(file_name.length(), max_length - 1);
  if (name && max_length > 0) {
    std::memcpy(name, file_name.c_str(), copy_length);
    name[copy_length] = '\0';
  }
  return file_name.length();
}

EXPORT void MdfFileSetName(MdfFile *file, const char *name) {
  file->Name(name);
}

EXPORT size_t MdfFileGetFileName(const MdfFile *file, char *filename,
                                 size_t max_length) {
  const std::string &file_name = file->FileName();
  size_t copy_length = std::min(file_name.length(), max_length - 1);
  if (filename && max_length > 0) {
    std::memcpy(filename, file_name.c_str(), copy_length);
    filename[copy_length] = '\0';
  }
  return file_name.length();
}

EXPORT void MdfFileSetFileName(MdfFile *file, const char *filename) {
  file->FileName(filename);
}

EXPORT size_t MdfFileGetVersion(const MdfFile *file, char *version,
                                size_t max_length) {
  const std::string &ver = file->Version();
  size_t copy_length = std::min(ver.length(), max_length - 1);
  if (version && max_length > 0) {
    std::memcpy(version, ver.c_str(), copy_length);
    version[copy_length] = '\0';
  }
  return ver.length();
}

EXPORT int MdfFileGetMainVersion(const MdfFile *file) {
  return file->MainVersion();
}

EXPORT int MdfFileGetMinorVersion(const MdfFile *file) {
  return file->MinorVersion();
}

EXPORT void MdfFileSetMinorVersion(MdfFile *file, int minor) {
  file->MinorVersion(minor);
}

EXPORT void MdfFileSetProgramId(MdfFile *file, const char *program_id) {
  file->ProgramId(program_id);
}

EXPORT size_t MdfFileGetProgramId(const MdfFile *file, char *buffer,
                                       size_t max_length) {
  const std::string &program_id = file->ProgramId();
  size_t copy_length = std::min(program_id.length(), max_length - 1);
  if (buffer && max_length > 0) {
    std::memcpy(buffer, program_id.c_str(), copy_length);
    buffer[copy_length] = '\0';
  }
  return program_id.length();
}

// TODO these need file stream
// EXPORT void MdfFileReadHeader(MdfFile *file) {
//   file->ReadHeader();
// }
//
// EXPORT void MdfFileReadMeasurementInfo(MdfFile *file) {
//   file->ReadMeasurementInfo();
// }
//
// EXPORT bool MdfFileWrite(MdfFile *file) {
//   return file->Write();
// }
//
// EXPORT void MdfFileReadEverythingButData(MdfFile *file) {
//   file->ReadEverythingButData();
// }

EXPORT bool MdfFileIsFinalizedDone(const MdfFile *file) {
  return file->IsFinalizedDone();
}

EXPORT const IHeader *MdfFileGetHeader(const MdfFile *file) {
  return file->Header();
}

EXPORT bool MdfFileGetIsMdf4(const MdfFile *file) { return file->IsMdf4(); }

EXPORT size_t MdfFileGetDataGroupCount(const MdfFile *file) {
  DataGroupList groups;
  file->DataGroups(groups);
  return groups.size();
}

EXPORT IDataGroup *MdfFileGetDataGroupByIndex(const MdfFile *file,
                                                    size_t index) {
  DataGroupList groups;
  file->DataGroups(groups);
  if (index >= groups.size())
    return nullptr;
  return groups[index];
}

EXPORT IDataGroup *MdfFileCreateDataGroup(MdfFile *file) {
  return file->CreateDataGroup();
}

EXPORT size_t MdfFileGetDataGroups(const MdfFile *file,
                                    IDataGroup *dest[], size_t max_count) {
  if (!file || !dest)
    return 0;

  std::vector<IDataGroup *> temp_list;
  file->DataGroups(temp_list);

  size_t copy_count = std::min(temp_list.size(), max_count);
  for (size_t i = 0; i < copy_count; ++i) {
    dest[i] = temp_list[i];
  }
  return temp_list.size();
}

EXPORT size_t MdfFileGetAttachments(const MdfFile *file,
                                    const IAttachment *attachments[],
                                    size_t max_count) {
  if (!file || !attachments)
    return 0;

  std::vector<const IAttachment *> temp_list;
  file->Attachments(temp_list);

  size_t copy_count = std::min(temp_list.size(), max_count);
  for (size_t i = 0; i < copy_count; ++i) {
    attachments[i] = temp_list[i];
  }
  return temp_list.size();
}

EXPORT IAttachment *MdfFileCreateAttachment(MdfFile *file) {
  return file ? file->CreateAttachment() : nullptr;
}

EXPORT IDataGroup* MdfFileFindParentDataGroup(const MdfFile *file,
                                                    const IChannel &channel) {
  return file->FindParentDataGroup(channel);
}

// IDataGroup functions
EXPORT uint64_t DataGroupGetIndex(const IDataGroup *group) {
  return group->Index();
}

EXPORT size_t DataGroupGetDescription(const IDataGroup *group,
                                      char *description, size_t max_length) {
  const std::string &desc = group->Description();
  size_t copy_length = std::min(desc.length(), max_length - 1);
  if (description && max_length > 0) {
    std::memcpy(description, desc.c_str(), copy_length);
    description[copy_length] = '\0';
  }
  return desc.length();
}

EXPORT void DataGroupSetDescription(IDataGroup *group,
                                    const char *description) {
  group->Description(description);
}

EXPORT size_t DataGroupGetChannelGroupCount(const IDataGroup *group) {
  const auto &channel_groups = group->ChannelGroups();
  return channel_groups.size();
}

EXPORT IChannelGroup *
DataGroupGetChannelGroupByIndex(const IDataGroup *group, size_t index) {
  const auto &channel_groups = group->ChannelGroups();
  if (index >= channel_groups.size())
    return nullptr;
  return channel_groups[index];
}

EXPORT IChannelGroup *DataGroupGetChannelGroupByName(
    const IDataGroup *group, const char *name) {
  return group->GetChannelGroup(name);
}

EXPORT IChannelGroup *DataGroupCreateChannelGroup(IDataGroup *group) {
  return group->CreateChannelGroup();
}

EXPORT void DataGroupClearData(IDataGroup *group) {
  group->ClearData();
}

// IChannelGroup functions
EXPORT uint64_t ChannelGroupGetIndex(const IChannelGroup *group) {
  return group->Index();
}

EXPORT size_t ChannelGroupGetName(const IChannelGroup *group, char *name,
                                  size_t max_length) {
  const std::string &group_name = group->Name();
  size_t copy_length = std::min(group_name.length(), max_length - 1);
  if (name && max_length > 0) {
    std::memcpy(name, group_name.c_str(), copy_length);
    name[copy_length] = '\0';
  }
  return group_name.length();
}

EXPORT void ChannelGroupSetName(IChannelGroup *group, const char *name) {
  group->Name(name);
}

EXPORT size_t ChannelGroupGetDescription(const IChannelGroup *group,
                                         char *description, size_t max_length) {
  const std::string &desc = group->Description();
  size_t copy_length = std::min(desc.length(), max_length - 1);
  if (description && max_length > 0) {
    std::memcpy(description, desc.c_str(), copy_length);
    description[copy_length] = '\0';
  }
  return desc.length();
}

EXPORT void ChannelGroupSetDescription(IChannelGroup *group,
                                       const char *description) {
  group->Description(description);
}

EXPORT uint64_t ChannelGroupGetNofSamples(const IChannelGroup *group) {
  return group->NofSamples();
}

EXPORT void ChannelGroupSetNofSamples(IChannelGroup *group, uint64_t samples) {
  group->NofSamples(samples);
}

EXPORT size_t ChannelGroupGetChannelCount(const IChannelGroup *group) {
  const auto &channels = group->Channels();
  return channels.size();
}

EXPORT const IChannel *ChannelGroupGetChannelByIndex(const IChannelGroup *group,
                                                     size_t index) {
  const auto &channels = group->Channels();
  if (index >= channels.size())
    return nullptr;
  return channels[index];
}

EXPORT const IChannel *ChannelGroupGetChannelByName(
    const IChannelGroup *group, const char *name) {
  return group->GetChannel(name);
}

EXPORT IChannel *ChannelGroupCreateChannel(IChannelGroup *group) {
  return group->CreateChannel();
}

EXPORT const IMetaData *ChannelGroupGetMetaData(const IChannelGroup *group) {
  return group ? group->MetaData() : nullptr;
}

EXPORT IMetaData *ChannelGroupCreateMetaData(IChannelGroup *group) {
  return group ? group->CreateMetaData() : nullptr;
}

EXPORT const ISourceInformation *
ChannelGroupGetSourceInformation(const IChannelGroup *group) {
  return group ? group->SourceInformation() : nullptr;
}

EXPORT ISourceInformation *
ChannelGroupCreateSourceInformation(IChannelGroup *group) {
  return group ? group->CreateSourceInformation() : nullptr;
}

EXPORT uint8_t ChannelGroupGetBusType(const IChannelGroup *group) {
  return (uint8_t) group->GetBusType();
}

// IHeader functions
EXPORT size_t IHeaderGetMeasurementId(const IHeader *header, char *id,
                                      size_t max_length) {
  const std::string &measurement_id = header->MeasurementId();
  size_t copy_length = std::min(measurement_id.length(), max_length - 1);
  if (id && max_length > 0) {
    std::memcpy(id, measurement_id.c_str(), copy_length);
    id[copy_length] = '\0';
  }
  return measurement_id.length();
}

EXPORT void IHeaderSetMeasurementId(IHeader *header, const char *id) {
  header->MeasurementId(id);
}

EXPORT size_t IHeaderGetRecorderId(const IHeader *header, char *id,
                                   size_t max_length) {
  const std::string &recorder_id = header->RecorderId();
  size_t copy_length = std::min(recorder_id.length(), max_length - 1);
  if (id && max_length > 0) {
    std::memcpy(id, recorder_id.c_str(), copy_length);
    id[copy_length] = '\0';
  }
  return recorder_id.length();
}

EXPORT void IHeaderSetRecorderId(IHeader *header, const char *id) {
  header->RecorderId(id);
}

EXPORT int64_t IHeaderGetRecorderIndex(const IHeader *header) {
  return header->RecorderIndex();
}

EXPORT void IHeaderSetRecorderIndex(IHeader *header, int64_t index) {
  header->RecorderIndex(index);
}

EXPORT bool IHeaderGetStartAngle(const IHeader *header, double *angle) {
  if (header->StartAngle().has_value()) {
    *angle = header->StartAngle().value();
    return true;
  }
  return false;
}

EXPORT void IHeaderSetStartAngle(IHeader *header, double angle) {
  header->StartAngle(angle);
}

EXPORT bool IHeaderGetStartDistance(const IHeader *header, double *distance) {
  if (header->StartDistance().has_value()) {
    *distance = header->StartDistance().value();
    return true;
  }
  return false;
}

EXPORT void IHeaderSetStartDistance(IHeader *header, double distance) {
  header->StartDistance(distance);
}

EXPORT size_t IHeaderGetAuthor(const IHeader *header, char *author,
                               size_t max_length) {
  const std::string &val = header->Author();
  size_t copy_length = std::min(val.length(), max_length - 1);
  if (author && max_length > 0) {
    std::memcpy(author, val.c_str(), copy_length);
    author[copy_length] = '\0';
  }
  return val.length();
}

EXPORT void IHeaderSetAuthor(IHeader *header, const char *author) {
  header->Author(author);
}

EXPORT size_t IHeaderGetDepartment(const IHeader *header, char *department,
                                   size_t max_length) {
  const std::string &val = header->Department();
  size_t copy_length = std::min(val.length(), max_length - 1);
  if (department && max_length > 0) {
    std::memcpy(department, val.c_str(), copy_length);
    department[copy_length] = '\0';
  }
  return val.length();
}

EXPORT void IHeaderSetDepartment(IHeader *header, const char *department) {
  header->Department(department);
}

EXPORT size_t IHeaderGetProject(const IHeader *header, char *project,
                                size_t max_length) {
  const std::string &val = header->Project();
  size_t copy_length = std::min(val.length(), max_length - 1);
  if (project && max_length > 0) {
    std::memcpy(project, val.c_str(), copy_length);
    project[copy_length] = '\0';
  }
  return val.length();
}

EXPORT void IHeaderSetProject(IHeader *header, const char *project) {
  header->Project(project);
}

EXPORT size_t IHeaderGetSubject(const IHeader *header, char *subject,
                                size_t max_length) {
  const std::string &val = header->Subject();
  size_t copy_length = std::min(val.length(), max_length - 1);
  if (subject && max_length > 0) {
    std::memcpy(subject, val.c_str(), copy_length);
    subject[copy_length] = '\0';
  }
  return val.length();
}

EXPORT void IHeaderSetSubject(IHeader *header, const char *subject) {
  header->Subject(subject);
}

EXPORT size_t IHeaderGetDescription(const IHeader *header, char *description,
                                    size_t max_length) {
  const std::string &val = header->Description();
  size_t copy_length = std::min(val.length(), max_length - 1);
  if (description && max_length > 0) {
    std::memcpy(description, val.c_str(), copy_length);
    description[copy_length] = '\0';
  }
  return val.length();
}

EXPORT void IHeaderSetDescription(IHeader *header, const char *description) {
  header->Description(description);
}

EXPORT uint64_t IHeaderGetStartTime(const IHeader *header) {
  return header->StartTime();
}

EXPORT void IHeaderSetStartTime(IHeader *header, uint64_t start_time) {
  header->StartTime(start_time);
}

EXPORT const IMetaData *IHeaderGetMetaData(const IHeader *header) {
  return header ? header->MetaData() : nullptr;
}

EXPORT IMetaData *IHeaderCreateMetaData(IHeader *header) {
  return header ? header->CreateMetaData() : nullptr;
}

EXPORT size_t IHeaderGetAttachments(const IHeader *header,
                                    const IAttachment *attachments[],
                                    size_t max_count) {
  if (!header || !attachments)
    return 0;

  auto temp_list = header->Attachments();

  size_t copy_count = std::min(temp_list.size(), max_count);
  for (size_t i = 0; i < copy_count; ++i) {
    attachments[i] = temp_list[i];
  }
  return temp_list.size();
}

EXPORT IAttachment *IHeaderCreateAttachment(IHeader *header) {
  return header ? header->CreateAttachment() : nullptr;
}

EXPORT size_t IHeaderGetFileHistories(const IHeader *header,
                                      const IFileHistory *histories[],
                                      size_t max_count) {
  if (!header || !histories)
    return 0;

  auto temp_list = header->FileHistories();

  size_t copy_count = std::min(temp_list.size(), max_count);
  for (size_t i = 0; i < copy_count; ++i) {
    histories[i] = temp_list[i];
  }
  return temp_list.size();
}

EXPORT IFileHistory *IHeaderCreateFileHistory(IHeader *header) {
  return header ? header->CreateFileHistory() : nullptr;
}

EXPORT size_t IHeaderGetEvents(const IHeader *header, const IEvent *events[],
                               size_t max_count) {
  if (!header || !events)
    return 0;

  auto temp_list = header->Events();

  size_t copy_count = std::min(temp_list.size(), max_count);
  for (size_t i = 0; i < copy_count; ++i) {
    events[i] = temp_list[i];
  }
  return temp_list.size();
}

EXPORT IEvent *IHeaderCreateEvent(IHeader *header) {
  return header ? header->CreateEvent() : nullptr;
}

EXPORT IDataGroup *IHeaderCreateDataGroup(IHeader *header) {
  return header->CreateDataGroup();
}

EXPORT IDataGroup *IHeaderLastDataGroup(IHeader *header) {
  return header ? header->LastDataGroup() : nullptr;
}

EXPORT size_t IHeaderGetDataGroupCount(const IHeader *header) {
  if (!header)
    return 0;

  auto temp_list = header->DataGroups();
  return temp_list.size();
}

EXPORT size_t IHeaderGetDataGroups(const IHeader *header,
                                    const IDataGroup *groups[],
                                    size_t max_count) {
  if (!header || !groups)
    return 0;

  // std::vector<const IDataGroup *> temp_list;
  auto temp_list = header->DataGroups();

  size_t copy_count = std::min(temp_list.size(), max_count);
  for (size_t i = 0; i < copy_count; ++i) {
    groups[i] = temp_list[i];
  }
  return temp_list.size();
}

// IChannel functions
EXPORT uint64_t ChannelGetIndex(const IChannel *channel) {
  return channel->Index();
}

EXPORT size_t ChannelGetName(const IChannel *channel, char *name,
                             size_t max_length) {
  const std::string &channel_name = channel->Name();
  size_t copy_length = std::min(channel_name.length(), max_length - 1);
  if (name && max_length > 0) {
    std::memcpy(name, channel_name.c_str(), copy_length);
    name[copy_length] = '\0';
  }
  return channel_name.length();
}

EXPORT void ChannelSetName(IChannel *channel, const char *name) {
  channel->Name(name);
}

EXPORT size_t ChannelGetDisplayName(const IChannel *channel, char *display_name,
                                    size_t max_length) {
  const std::string &name = channel->DisplayName();
  size_t copy_length = std::min(name.length(), max_length - 1);
  if (display_name && max_length > 0) {
    std::memcpy(display_name, name.c_str(), copy_length);
    display_name[copy_length] = '\0';
  }
  return name.length();
}

EXPORT void ChannelSetDisplayName(IChannel *channel, const char *display_name) {
  channel->DisplayName(display_name);
}

EXPORT size_t ChannelGetDescription(const IChannel *channel, char *description,
                                    size_t max_length) {
  const std::string &desc = channel->Description();
  size_t copy_length = std::min(desc.length(), max_length - 1);
  if (description && max_length > 0) {
    std::memcpy(description, desc.c_str(), copy_length);
    description[copy_length] = '\0';
  }
  return desc.length();
}

EXPORT void ChannelSetDescription(IChannel *channel, const char *description) {
  channel->Description(description);
}

EXPORT size_t ChannelGetUnit(const IChannel *channel, char *unit,
                             size_t max_length) {
  const std::string &unit_str = channel->Unit();
  size_t copy_length = std::min(unit_str.length(), max_length - 1);
  if (unit && max_length > 0) {
    std::memcpy(unit, unit_str.c_str(), copy_length);
    unit[copy_length] = '\0';
  }
  return unit_str.length();
}

EXPORT void ChannelSetUnit(IChannel *channel, const char *unit) {
  channel->Unit(unit);
}

EXPORT uint8_t ChannelGetType(const IChannel *channel) {
  return static_cast<uint8_t>(channel->Type());
}

EXPORT void ChannelSetType(IChannel *channel, uint8_t type) {
  channel->Type(static_cast<ChannelType>(type));
}

EXPORT uint8_t ChannelGetDataType(const IChannel *channel) {
  return static_cast<uint8_t>(channel->DataType());
}

EXPORT void ChannelSetDataType(IChannel *channel, uint8_t data_type) {
  channel->DataType(static_cast<ChannelDataType>(data_type));
}

EXPORT uint64_t ChannelGetDataBytes(const IChannel *channel) {
  return channel->DataBytes();
}

EXPORT void ChannelSetDataBytes(IChannel *channel, uint64_t bytes) {
  channel->DataBytes(bytes);
}

EXPORT void ChannelSetChannelValue(IChannel *channel, uint32_t value,
                                   bool valid) {
  if (channel) {
    channel->SetChannelValue(value, valid);
  }
}

EXPORT const IMetaData *ChannelGetMetaData(const IChannel *channel) {
  return channel ? channel->MetaData() : nullptr;
}

EXPORT IMetaData *ChannelCreateMetaData(IChannel *channel) {
  return channel ? channel->CreateMetaData() : nullptr;
}

EXPORT const ISourceInformation *
ChannelGetSourceInformation(const IChannel *channel) {
  return channel ? channel->SourceInformation() : nullptr;
}

EXPORT ISourceInformation *ChannelCreateSourceInformation(IChannel *channel) {
  return channel ? channel->CreateSourceInformation() : nullptr;
}

EXPORT const IChannelConversion *
ChannelGetChannelConversion(const IChannel *channel) {
  return channel ? channel->ChannelConversion() : nullptr;
}

EXPORT IChannelConversion *ChannelCreateChannelConversion(IChannel *channel) {
  return channel ? channel->CreateChannelConversion() : nullptr;
}

EXPORT const IChannelArray *ChannelGetChannelArray(const IChannel *channel) {
  return channel ? channel->ChannelArray() : nullptr;
}

EXPORT IChannelArray *ChannelCreateChannelArray(IChannel *channel) {
  return channel ? channel->CreateChannelArray() : nullptr;
}

// CanMessage functions
EXPORT CanMessage *CanMessageInit() { return new CanMessage; }

EXPORT void CanMessageUnInit(CanMessage *can) { delete can; }

EXPORT uint32_t CanMessageGetMessageId(const CanMessage *can) {
  return can->MessageId();
}

EXPORT void CanMessageSetMessageId(CanMessage *can, uint32_t msgId) {
  can->MessageId(msgId);
}

EXPORT uint32_t CanMessageGetCanId(const CanMessage *can) { return can->CanId(); }

EXPORT bool CanMessageGetExtendedId(const CanMessage *can) {
  return can->ExtendedId();
}

EXPORT void CanMessageSetExtendedId(CanMessage *can, bool extendedId) {
  can->ExtendedId(extendedId);
}

EXPORT uint8_t CanMessageGetDlc(const CanMessage *can) { return can->Dlc(); }

EXPORT void CanMessageSetDlc(CanMessage *can, uint8_t dlc) { can->Dlc(dlc); }

EXPORT size_t CanMessageGetDataLength(const CanMessage *can) {
  return can->DataLength();
}

EXPORT void CanMessageSetDataLength(CanMessage *can, uint32_t dataLength) {
  can->DataLength(dataLength);
}

EXPORT size_t CanMessageGetDataBytes(const CanMessage *can, uint8_t *dataList,
                                          size_t max_length) {
  const auto &data = can->DataBytes();
  size_t copy_length = std::min(data.size(), max_length);
  if (dataList && max_length > 0) {
    std::memcpy(dataList, data.data(), copy_length);
  }
  return data.size();
}

EXPORT void CanMessageSetDataBytes(CanMessage *can, const uint8_t *dataList,
                                   size_t size) {
  std::vector<uint8_t> data(dataList, dataList + size);
  can->DataBytes(data);
}

EXPORT uint32_t CanMessageGetBusChannel(const CanMessage *can) {
  return can->BusChannel();
}

EXPORT void CanMessageSetBusChannel(CanMessage *can, uint32_t busChannel) {
  can->BusChannel(busChannel);
}

EXPORT uint64_t CanMessageGetTimestamp(const CanMessage *can) {
  return can->Timestamp();
}

EXPORT void CanMessageSetTimestamp(CanMessage *can, uint64_t timeStamp) {
  can->Timestamp(timeStamp);
}

EXPORT uint32_t CanMessageGetCrc(const CanMessage *can) {
  return can->Crc();
}

EXPORT void CanMessageSetCrc(CanMessage *can, uint32_t crc) {
  can->Crc(crc);
}

EXPORT void CanMessageSetTypeOfMessage(CanMessage *can,
                                       uint8_t typeOfMessage) {
  can->TypeOfMessage(static_cast<MessageType>(typeOfMessage));
}

EXPORT uint8_t CanMessageGetTypeOfMessage(const CanMessage *can) {
  return static_cast<uint8_t>(can->TypeOfMessage());
}

// ISourceInformation functions
EXPORT uint64_t SourceInformationGetIndex(const ISourceInformation *source) {
  return source->Index();
}

EXPORT size_t SourceInformationGetName(const ISourceInformation *source,
                                       char *name, size_t max_length) {
  const auto &str = source->Name();
  if (name && max_length > 0) {
    size_t copy_len = std::min(str.size(), max_length - 1);
    std::memcpy(name, str.c_str(), copy_len);
    name[copy_len] = '\0';
  }
  return str.size();
}

EXPORT void SourceInformationSetName(ISourceInformation *source,
                                     const char *name) {
  source->Name(name ? name : "");
}

EXPORT size_t SourceInformationGetDescription(const ISourceInformation *source,
                                              char *description,
                                              size_t max_length) {
  const auto &str = source->Description();
  if (description && max_length > 0) {
    size_t copy_len = std::min(str.size(), max_length - 1);
    std::memcpy(description, str.c_str(), copy_len);
    description[copy_len] = '\0';
  }
  return str.size();
}

EXPORT void SourceInformationSetDescription(ISourceInformation *source,
                                            const char *description) {
  source->Description(description ? description : "");
}

EXPORT size_t SourceInformationGetPath(const ISourceInformation *source,
                                       char *path, size_t max_length) {
  const auto &str = source->Path();
  if (path && max_length > 0) {
    size_t copy_len = std::min(str.size(), max_length - 1);
    std::memcpy(path, str.c_str(), copy_len);
    path[copy_len] = '\0';
  }
  return str.size();
}

EXPORT void SourceInformationSetPath(ISourceInformation *source,
                                     const char *path) {
  source->Path(path ? path : "");
}

EXPORT uint8_t SourceInformationGetType(const ISourceInformation *source) {
  return static_cast<uint8_t>(source->Type());
}

EXPORT void SourceInformationSetType(ISourceInformation *source, uint8_t type) {
  source->Type(static_cast<SourceType>(type));
}

EXPORT uint8_t SourceInformationGetBus(const ISourceInformation *source) {
  return static_cast<uint8_t>(source->Bus());
}

EXPORT void SourceInformationSetBus(ISourceInformation *source, uint8_t bus) {
  source->Bus(static_cast<BusType>(bus));
}

EXPORT uint8_t SourceInformationGetFlags(const ISourceInformation *source) {
  return source->Flags();
}

EXPORT void SourceInformationSetFlags(ISourceInformation *source,
                                      uint8_t flags) {
  source->Flags(flags);
}

EXPORT const IMetaData *
SourceInformationGetMetaData(const ISourceInformation *source) {
  return source->MetaData();
}

EXPORT IMetaData *SourceInformationCreateMetaData(ISourceInformation *source) {
  return source->CreateMetaData();
}

// IAttachment functions
EXPORT uint64_t AttachmentGetIndex(const IAttachment *attachment) {
  return attachment->Index();
}

EXPORT uint16_t AttachmentGetCreatorIndex(const IAttachment *attachment) {
  return attachment->CreatorIndex();
}

EXPORT void AttachmentSetCreatorIndex(IAttachment *attachment, uint16_t index) {
  attachment->CreatorIndex(index);
}

EXPORT bool AttachmentGetEmbedded(const IAttachment *attachment) {
  return attachment->IsEmbedded();
}

EXPORT void AttachmentSetEmbedded(IAttachment *attachment, bool embedded) {
  attachment->IsEmbedded(embedded);
}

EXPORT bool AttachmentGetCompressed(const IAttachment *attachment) {
  return attachment->IsCompressed();
}

EXPORT void AttachmentSetCompressed(IAttachment *attachment, bool compressed) {
  attachment->IsCompressed(compressed);
}

EXPORT bool AttachmentGetMd5(const IAttachment *attachment, char *md5,
                             size_t max_length) {
  auto md5_opt = attachment->Md5();
  if (md5_opt.has_value() && md5 && max_length > 0) {
    const auto &md5_str = md5_opt.value();
    size_t copy_len = std::min(md5_str.size(), max_length - 1);
    std::memcpy(md5, md5_str.c_str(), copy_len);
    md5[copy_len] = '\0';
    return true;
  }
  return false;
}

EXPORT size_t AttachmentGetFileName(const IAttachment *attachment, char *name,
                                    size_t max_length) {
  const auto &str = attachment->FileName();
  if (name && max_length > 0) {
    size_t copy_len = std::min(str.size(), max_length - 1);
    std::memcpy(name, str.c_str(), copy_len);
    name[copy_len] = '\0';
  }
  return str.size();
}

EXPORT void AttachmentSetFileName(IAttachment *attachment, const char *name) {
  attachment->FileName(name ? name : "");
}

EXPORT size_t AttachmentGetFileType(const IAttachment *attachment, char *type,
                                    size_t max_length) {
  const auto &str = attachment->FileType();
  if (type && max_length > 0) {
    size_t copy_len = std::min(str.size(), max_length - 1);
    std::memcpy(type, str.c_str(), copy_len);
    type[copy_len] = '\0';
  }
  return str.size();
}

EXPORT void AttachmentSetFileType(IAttachment *attachment, const char *type) {
  attachment->FileType(type ? type : "");
}

EXPORT const IMetaData *AttachmentGetMetaData(const IAttachment *attachment) {
  return attachment->MetaData();
}

EXPORT IMetaData *AttachmentCreateMetaData(IAttachment *attachment) {
  return attachment->CreateMetaData();
}

// IEvent functions
EXPORT uint64_t EventGetIndex(const IEvent *event) { return event->Index(); }

EXPORT size_t EventGetName(const IEvent *event, char *name, size_t max_length) {
  const auto &str = event->Name();
  if (name && max_length > 0) {
    size_t copy_len = std::min(str.size(), max_length - 1);
    std::memcpy(name, str.c_str(), copy_len);
    name[copy_len] = '\0';
  }
  return str.size();
}

EXPORT void EventSetName(IEvent *event, const char *name) {
  event->Name(name ? name : "");
}

EXPORT size_t EventGetDescription(const IEvent *event, char *description,
                                  size_t max_length) {
  const auto &str = event->Description();
  if (description && max_length > 0) {
    size_t copy_len = std::min(str.size(), max_length - 1);
    std::memcpy(description, str.c_str(), copy_len);
    description[copy_len] = '\0';
  }
  return str.size();
}

EXPORT void EventSetDescription(IEvent *event, const char *description) {
  event->Description(description ? description : "");
}

EXPORT size_t EventGetGroupName(const IEvent *event, char *group,
                                size_t max_length) {
  const auto &str = event->GroupName();
  if (group && max_length > 0) {
    size_t copy_len = std::min(str.size(), max_length - 1);
    std::memcpy(group, str.c_str(), copy_len);
    group[copy_len] = '\0';
  }
  return str.size();
}

EXPORT void EventSetGroupName(IEvent *event, const char *group) {
  event->GroupName(group ? group : "");
}

EXPORT uint8_t EventGetType(const IEvent *event) {
  return static_cast<uint8_t>(event->Type());
}

EXPORT void EventSetType(IEvent *event, uint8_t type) {
  event->Type(static_cast<EventType>(type));
}

EXPORT uint8_t EventGetSync(const IEvent *event) {
  return static_cast<uint8_t>(event->Sync());
}

EXPORT void EventSetSync(IEvent *event, uint8_t type) {
  event->Sync(static_cast<SyncType>(type));
}

EXPORT uint8_t EventGetRange(const IEvent *event) {
  return static_cast<uint8_t>(event->Range());
}

EXPORT void EventSetRange(IEvent *event, uint8_t type) {
  event->Range(static_cast<RangeType>(type));
}

EXPORT uint8_t EventGetCause(const IEvent *event) {
  return static_cast<uint8_t>(event->Cause());
}

EXPORT void EventSetCause(IEvent *event, uint8_t cause) {
  event->Cause(static_cast<EventCause>(cause));
}

EXPORT uint16_t EventGetCreatorIndex(const IEvent *event) {
  return event->CreatorIndex();
}

EXPORT void EventSetCreatorIndex(IEvent *event, uint16_t index) {
  event->CreatorIndex(index);
}

EXPORT int64_t EventGetSyncValue(const IEvent *event) {
  return event->SyncValue();
}

EXPORT void EventSetSyncValue(IEvent *event, int64_t value) {
  event->SyncValue(value);
}

EXPORT double EventGetSyncFactor(const IEvent *event) {
  return event->SyncFactor();
}

EXPORT void EventSetSyncFactor(IEvent *event, double factor) {
  event->SyncFactor(factor);
}

EXPORT double EventGetPreTrig(const IEvent *event) { return event->PreTrig(); }

EXPORT void EventSetPreTrig(IEvent *event, double time) {
  event->PreTrig(time);
}

EXPORT double EventGetPostTrig(const IEvent *event) {
  return event->PostTrig();
}

EXPORT void EventSetPostTrig(IEvent *event, double time) {
  event->PostTrig(time);
}

EXPORT const IMetaData *EventGetMetaData(const IEvent *event) {
  return event->MetaData();
}

// IFileHistory functions
EXPORT uint64_t FileHistoryGetIndex(const IFileHistory *file_history) {
  return file_history->Index();
}

EXPORT uint64_t FileHistoryGetTime(const IFileHistory *file_history) {
  return file_history->Time();
}

EXPORT void FileHistorySetTime(IFileHistory *file_history, uint64_t time) {
  file_history->Time(time);
}

EXPORT const IMetaData *
FileHistoryGetMetaData(const IFileHistory *file_history) {
  return file_history->MetaData();
}

EXPORT size_t FileHistoryGetDescription(const IFileHistory *file_history,
                                        char *desc, size_t max_length) {
  const auto &str = file_history->Description();
  if (desc && max_length > 0) {
    size_t copy_len = std::min(str.size(), max_length - 1);
    std::memcpy(desc, str.c_str(), copy_len);
    desc[copy_len] = '\0';
  }
  return str.size();
}

EXPORT void FileHistorySetDescription(IFileHistory *file_history,
                                      const char *desc) {
  file_history->Description(desc ? desc : "");
}

EXPORT size_t FileHistoryGetToolName(const IFileHistory *file_history,
                                     char *name, size_t max_length) {
  const auto &str = file_history->ToolName();
  if (name && max_length > 0) {
    size_t copy_len = std::min(str.size(), max_length - 1);
    std::memcpy(name, str.c_str(), copy_len);
    name[copy_len] = '\0';
  }
  return str.size();
}

EXPORT void FileHistorySetToolName(IFileHistory *file_history,
                                   const char *name) {
  file_history->ToolName(name ? name : "");
}

EXPORT size_t FileHistoryGetToolVendor(const IFileHistory *file_history,
                                       char *vendor, size_t max_length) {
  const auto &str = file_history->ToolVendor();
  if (vendor && max_length > 0) {
    size_t copy_len = std::min(str.size(), max_length - 1);
    std::memcpy(vendor, str.c_str(), copy_len);
    vendor[copy_len] = '\0';
  }
  return str.size();
}

EXPORT void FileHistorySetToolVendor(IFileHistory *file_history,
                                     const char *vendor) {
  file_history->ToolVendor(vendor ? vendor : "");
}

EXPORT size_t FileHistoryGetToolVersion(const IFileHistory *file_history,
                                        char *version, size_t max_length) {
  const auto &str = file_history->ToolVersion();
  if (version && max_length > 0) {
    size_t copy_len = std::min(str.size(), max_length - 1);
    std::memcpy(version, str.c_str(), copy_len);
    version[copy_len] = '\0';
  }
  return str.size();
}

EXPORT void FileHistorySetToolVersion(IFileHistory *file_history,
                                      const char *version) {
  file_history->ToolVersion(version ? version : "");
}

EXPORT size_t FileHistoryGetUserName(const IFileHistory *file_history,
                                     char *user, size_t max_length) {
  const auto &str = file_history->UserName();
  if (user && max_length > 0) {
    size_t copy_len = std::min(str.size(), max_length - 1);
    std::memcpy(user, str.c_str(), copy_len);
    user[copy_len] = '\0';
  }
  return str.size();
}

EXPORT void FileHistorySetUserName(IFileHistory *file_history,
                                   const char *user) {
  file_history->UserName(user ? user : "");
}

// IMetaData functions
EXPORT size_t MetaDataGetPropertyAsString(const IMetaData *metadata,
                                          const char *index, char *prop,
                                          size_t max_length) {
  std::string prop_str = metadata->StringProperty(index);
  if (prop && max_length > 0) {
    size_t copy_len = std::min(prop_str.size(), max_length - 1);
    std::memcpy(prop, prop_str.c_str(), copy_len);
    prop[copy_len] = '\0';
  }
  return prop_str.size();
}

EXPORT void MetaDataSetPropertyAsString(IMetaData *metadata, const char *index,
                                        const char *prop) {
  metadata->StringProperty(index, prop ? prop : "");
}

EXPORT double MetaDataGetPropertyAsFloat(const IMetaData *metadata,
                                         const char *index) {
  return metadata->FloatProperty(index);
}

EXPORT void MetaDataSetPropertyAsFloat(IMetaData *metadata, const char *index,
                                       double prop) {
  metadata->FloatProperty(index, prop);
}

EXPORT size_t MetaDataGetXmlSnippet(const IMetaData *metadata, char *xml,
                                    size_t max_length) {
  const auto &str = metadata->XmlSnippet();
  if (xml && max_length > 0) {
    size_t copy_len = std::min(str.size(), max_length - 1);
    std::memcpy(xml, str.c_str(), copy_len);
    xml[copy_len] = '\0';
  }
  return str.size();
}

EXPORT void MetaDataSetXmlSnippet(IMetaData *metadata, const char *xml) {
  metadata->XmlSnippet(xml ? xml : "");
}

EXPORT size_t MetaDataGetProperties(const IMetaData *metadata,
                                    ETag *properties[], size_t max_count) {
  if (!metadata || !properties)
    return 0;

  auto temp_list = metadata->Properties();

  size_t copy_count = std::min(temp_list.size(), max_count);
  for (size_t i = 0; i < copy_count; ++i) {
    properties[i] = &temp_list[i];
  }
  return temp_list.size();
}

EXPORT size_t MetaDataGetCommonProperties(const IMetaData *metadata,
                                          ETag *properties[],
                                          size_t max_count) {
  if (!metadata || !properties)
    return 0;

  auto temp_list =
      metadata->Properties(); // Use the general Properties for reading

  size_t copy_count = std::min(temp_list.size(), max_count);
  for (size_t i = 0; i < copy_count; ++i) {
    properties[i] = &temp_list[i];
  }
  return temp_list.size();
}

EXPORT void MetaDataAddCommonProperty(IMetaData *metadata, ETag *tag) {
  if (metadata && tag) {
    std::vector<ETag> tag_list = {*tag};
    metadata->CommonProperties(tag_list);
  }
}

// ETag functions
EXPORT ETag *ETagInit() { return new ETag(); }

EXPORT void ETagUnInit(ETag *etag) { delete etag; }

EXPORT size_t ETagGetName(const ETag *etag, char *name, size_t max_length) {
  const auto &str = etag->Name();
  if (name && max_length > 0) {
    size_t copy_len = std::min(str.size(), max_length - 1);
    std::memcpy(name, str.c_str(), copy_len);
    name[copy_len] = '\0';
  }
  return str.size();
}

EXPORT void ETagSetName(ETag *etag, const char *name) {
  etag->Name(name ? name : "");
}

EXPORT size_t ETagGetDescription(const ETag *etag, char *desc,
                                 size_t max_length) {
  const auto &str = etag->Description();
  if (desc && max_length > 0) {
    size_t copy_len = std::min(str.size(), max_length - 1);
    std::memcpy(desc, str.c_str(), copy_len);
    desc[copy_len] = '\0';
  }
  return str.size();
}

EXPORT void ETagSetDescription(ETag *etag, const char *desc) {
  etag->Description(desc ? desc : "");
}

EXPORT size_t ETagGetUnit(const ETag *etag, char *unit, size_t max_length) {
  const auto &str = etag->Unit();
  if (unit && max_length > 0) {
    size_t copy_len = std::min(str.size(), max_length - 1);
    std::memcpy(unit, str.c_str(), copy_len);
    unit[copy_len] = '\0';
  }
  return str.size();
}

EXPORT void ETagSetUnit(ETag *etag, const char *unit) {
  etag->Unit(unit ? unit : "");
}

EXPORT size_t ETagGetUnitRef(const ETag *etag, char *unit, size_t max_length) {
  const auto &str = etag->UnitRef();
  if (unit && max_length > 0) {
    size_t copy_len = std::min(str.size(), max_length - 1);
    std::memcpy(unit, str.c_str(), copy_len);
    unit[copy_len] = '\0';
  }
  return str.size();
}

EXPORT void ETagSetUnitRef(ETag *etag, const char *unit) {
  etag->UnitRef(unit ? unit : "");
}

EXPORT size_t ETagGetType(const ETag *etag, char *type, size_t max_length) {
  const auto &str = etag->Type();
  if (type && max_length > 0) {
    size_t copy_len = std::min(str.size(), max_length - 1);
    std::memcpy(type, str.c_str(), copy_len);
    type[copy_len] = '\0';
  }
  return str.size();
}

EXPORT void ETagSetType(ETag *etag, const char *type) {
  etag->Type(type ? type : "");
}

EXPORT uint8_t ETagGetDataType(const ETag *etag) {
  return static_cast<uint8_t>(etag->DataType());
}

EXPORT void ETagSetDataType(ETag *etag, uint8_t type) {
  etag->DataType(static_cast<ETagDataType>(type));
}

EXPORT size_t ETagGetLanguage(const ETag *etag, char *language,
                              size_t max_length) {
  const auto &str = etag->Language();
  if (language && max_length > 0) {
    size_t copy_len = std::min(str.size(), max_length - 1);
    std::memcpy(language, str.c_str(), copy_len);
    language[copy_len] = '\0';
  }
  return str.size();
}

EXPORT void ETagSetLanguage(ETag *etag, const char *language) {
  etag->Language(language ? language : "");
}

EXPORT bool ETagGetReadOnly(const ETag *etag) { return etag->ReadOnly(); }

EXPORT void ETagSetReadOnly(ETag *etag, bool read_only) {
  etag->ReadOnly(read_only);
}

EXPORT size_t ETagGetValueAsString(const ETag *etag, char *value,
                                   size_t max_length) {
  const auto str = etag->Value<std::string>();
  if (value && max_length > 0) {
    size_t copy_len = std::min(str.size(), max_length - 1);
    std::memcpy(value, str.c_str(), copy_len);
    value[copy_len] = '\0';
  }
  return str.size();
}

EXPORT void ETagSetValueAsString(ETag *etag, const char *value) {
  etag->Value(std::string(value ? value : ""));
}

EXPORT double ETagGetValueAsFloat(const ETag *etag) {
  return etag->Value<double>();
}

EXPORT void ETagSetValueAsFloat(ETag *etag, double value) {
  etag->Value(value);
}

EXPORT bool ETagGetValueAsBoolean(const ETag *etag) {
  return etag->Value<bool>();
}

EXPORT void ETagSetValueAsBoolean(ETag *etag, bool value) {
  etag->Value(value);
}

EXPORT int64_t ETagGetValueAsSigned(const ETag *etag) {
  return etag->Value<int64_t>();
}

EXPORT void ETagSetValueAsSigned(ETag *etag, int64_t value) {
  etag->Value(value);
}

EXPORT uint64_t ETagGetValueAsUnsigned(const ETag *etag) {
  return etag->Value<uint64_t>();
}

EXPORT void ETagSetValueAsUnsigned(ETag *etag, uint64_t value) {
  etag->Value(value);
}

// IChannelArray functions
EXPORT uint64_t ChannelArrayGetIndex(const IChannelArray *array) {
  return array->Index();
}

EXPORT uint8_t ChannelArrayGetType(const IChannelArray *array) {
  return static_cast<uint8_t>(array->Type());
}

EXPORT void ChannelArraySetType(IChannelArray *array, uint8_t type) {
  array->Type(static_cast<ArrayType>(type));
}

EXPORT uint8_t ChannelArrayGetStorage(const IChannelArray *array) {
  return static_cast<uint8_t>(array->Storage());
}

EXPORT void ChannelArraySetStorage(IChannelArray *array, uint8_t storage) {
  array->Storage(static_cast<ArrayStorage>(storage));
}

EXPORT uint32_t ChannelArrayGetFlags(const IChannelArray *array) {
  return array->Flags();
}

EXPORT void ChannelArraySetFlags(IChannelArray *array, uint32_t flags) {
  array->Flags(flags);
}

EXPORT uint64_t ChannelArrayGetNofElements(const IChannelArray *array) {
  return array->Dimensions();
}

// IChannelConversion functions
EXPORT uint64_t
ChannelConversionGetIndex(const IChannelConversion *conversion) {
  return conversion->Index();
}

EXPORT size_t ChannelConversionGetName(const IChannelConversion *conversion,
                                       char *name, size_t max_length) {
  const auto &str = conversion->Name();
  if (name && max_length > 0) {
    size_t copy_len = std::min(str.size(), max_length - 1);
    std::memcpy(name, str.c_str(), copy_len);
    name[copy_len] = '\0';
  }
  return str.size();
}

EXPORT void ChannelConversionSetName(IChannelConversion *conversion,
                                     const char *name) {
  conversion->Name(name ? name : "");
}

EXPORT size_t ChannelConversionGetDescription(
    const IChannelConversion *conversion, char *desc, size_t max_length) {
  const auto &str = conversion->Description();
  if (desc && max_length > 0) {
    size_t copy_len = std::min(str.size(), max_length - 1);
    std::memcpy(desc, str.c_str(), copy_len);
    desc[copy_len] = '\0';
  }
  return str.size();
}

EXPORT void ChannelConversionSetDescription(IChannelConversion *conversion,
                                            const char *desc) {
  conversion->Description(desc ? desc : "");
}

EXPORT size_t ChannelConversionGetUnit(const IChannelConversion *conversion,
                                       char *unit, size_t max_length) {
  const auto &str = conversion->Unit();
  if (unit && max_length > 0) {
    size_t copy_len = std::min(str.size(), max_length - 1);
    std::memcpy(unit, str.c_str(), copy_len);
    unit[copy_len] = '\0';
  }
  return str.size();
}

EXPORT void ChannelConversionSetUnit(IChannelConversion *conversion,
                                     const char *unit) {
  conversion->Unit(unit ? unit : "");
}

EXPORT uint8_t ChannelConversionGetType(const IChannelConversion *conversion) {
  return static_cast<uint8_t>(conversion->Type());
}

EXPORT void ChannelConversionSetType(IChannelConversion *conversion,
                                     uint8_t type) {
  conversion->Type(static_cast<ConversionType>(type));
}

EXPORT bool
ChannelConversionIsPrecisionUsed(const IChannelConversion *conversion) {
  return conversion->IsDecimalUsed();
}

EXPORT uint8_t
ChannelConversionGetPrecision(const IChannelConversion *conversion) {
  return conversion->Decimals();
}

EXPORT bool ChannelConversionIsRangeUsed(const IChannelConversion *conversion) {
  auto range_opt = conversion->Range();
  return range_opt.has_value();
}

EXPORT double
ChannelConversionGetRangeMin(const IChannelConversion *conversion) {
  auto range_opt = conversion->Range();
  if (range_opt.has_value()) {
    return range_opt.value().first;
  }
  return 0.0;
}

EXPORT double
ChannelConversionGetRangeMax(const IChannelConversion *conversion) {
  auto range_opt = conversion->Range();
  if (range_opt.has_value()) {
    return range_opt.value().second;
  }
  return 0.0;
}

EXPORT void ChannelConversionSetRange(IChannelConversion *conversion,
                                      double min, double max) {
  conversion->Range(min, max);
}

EXPORT uint16_t
ChannelConversionGetFlags(const IChannelConversion *conversion) {
  return conversion->Flags();
}

EXPORT size_t ChannelConversionGetFormula(const IChannelConversion *conversion,
                                          char *formula, size_t max_length) {
  const auto &str = conversion->Formula();
  if (formula && max_length > 0) {
    size_t copy_len = std::min(str.size(), max_length - 1);
    std::memcpy(formula, str.c_str(), copy_len);
    formula[copy_len] = '\0';
  }
  return str.size();
}

EXPORT void ChannelConversionSetFormula(IChannelConversion *conversion,
                                        const char *formula) {
  conversion->Formula(formula ? formula : "");
}

EXPORT double
ChannelConversionGetParameterAsDouble(const IChannelConversion *conversion,
                                      uint16_t index) {
  return conversion->Parameter(index);
}

EXPORT void
ChannelConversionSetParameterAsDouble(IChannelConversion *conversion,
                                      uint16_t index, double parameter) {
  conversion->Parameter(index, parameter);
}

EXPORT uint64_t ChannelConversionGetParameterAsUInt64(
    const IChannelConversion *conversion, uint16_t index) {
  return static_cast<uint64_t>(conversion->Parameter(index));
}

EXPORT void
ChannelConversionSetParameterAsUInt64(IChannelConversion *conversion,
                                      uint16_t index, uint64_t parameter) {
  conversion->Parameter(index, parameter);
}

EXPORT const IMetaData *
ChannelConversionGetMetaData(const IChannelConversion *conversion) {
  return conversion->MetaData();
}

EXPORT IMetaData *
ChannelConversionCreateMetaData(IChannelConversion *conversion) {
  return conversion->CreateMetaData();
}

// IChannelObserver functions
EXPORT IChannelObserver* CreateChannelObserver(const IDataGroup* dataGroup, const IChannelGroup* channelGroup, const IChannel* channel) {
  if (!dataGroup || !channelGroup || !channel) {
    return nullptr;
  }
  auto observer_ptr = mdf::CreateChannelObserver(*dataGroup, *channelGroup, *channel);
  return observer_ptr.release();
}

EXPORT void ChannelObserverUnInit(IChannelObserver* observer) {
  delete observer;
}

EXPORT size_t ChannelObserverGetNofSamples(const IChannelObserver* observer) {
  return observer ? observer->NofSamples() : 0;
}

EXPORT bool ChannelObserverGetChannelValue(const IChannelObserver* observer, size_t sample, double* value) {
  if (!observer || !value) {
    return false;
  }
  return observer->GetChannelValue(sample, *value);
}

EXPORT bool ChannelObserverGetEngValue(const IChannelObserver* observer, size_t sample, double* value) {
  if (!observer || !value) {
    return false;
  }
  return observer->GetEngValue(sample, *value);
}

EXPORT bool ChannelObserverGetValid(const IChannelObserver* observer, size_t sample) {
  if (!observer) {
    return false;
  }
  const auto& valid_list = observer->GetValidList();
  return sample < valid_list.size() && valid_list[sample];
}

} // extern "C"
