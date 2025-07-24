#include "mdf_c_wrapper.h"

// Include all mdflib headers
#include "mdf/canmessage.h"
#include "mdf/iattachment.h"
#include "mdf/ichannel.h"
#include "mdf/ichannelarray.h"
#include "mdf/ichannelconversion.h"
#include "mdf/ichannelgroup.h"
#include "mdf/ichannelobserver.h"
#include "mdf/idatagroup.h"
#include "mdf/ievent.h"
#include "mdf/ifilehistory.h"
#include "mdf/iheader.h"
#include "mdf/imetadata.h"
#include "mdf/isourceinformation.h"
#include "mdf/mdffactory.h"
#include "mdf/mdffile.h"
#include "mdf/mdfreader.h"
#include "mdf/mdfwriter.h"

// use mdf namespace
using namespace mdf;

extern "C" {

// MdfReader functions
MdfReader *MdfReaderInit(const char *filename) {
    auto reader = mdf::MdfFactory::CreateMdfReader(filename);
    return reinterpret_cast<MdfReader*>(reader.release());
}

void MdfReaderUnInit(MdfReader *reader) {
    delete reinterpret_cast<mdf::MdfReader*>(reader);
}

int64_t MdfReaderGetIndex(MdfReader *reader) {
    return reinterpret_cast<mdf::MdfReader*>(reader)->GetIndex();
}

bool MdfReaderIsOk(MdfReader *reader) {
    return reinterpret_cast<mdf::MdfReader*>(reader)->IsOk();
}

const MdfFile *MdfReaderGetFile(MdfReader *reader) {
    return reinterpret_cast<const MdfFile*>(reinterpret_cast<const mdf::MdfReader*>(reader)->GetFile());
}

const IHeader *MdfReaderGetHeader(MdfReader *reader) {
    return reinterpret_cast<const IHeader*>(reinterpret_cast<const mdf::MdfReader*>(reader)->GetHeader());
}

const IDataGroup *MdfReaderGetDataGroup(MdfReader *reader, size_t index) {
    return reinterpret_cast<const IDataGroup*>(reinterpret_cast<const mdf::MdfReader*>(reader)->GetDataGroup(index));
}

bool MdfReaderOpen(MdfReader *reader) {
    return reinterpret_cast<mdf::MdfReader*>(reader)->Open();
}

void MdfReaderClose(MdfReader *reader) {
    reinterpret_cast<mdf::MdfReader*>(reader)->Close();
}

bool MdfReaderReadHeader(MdfReader *reader) {
    return reinterpret_cast<mdf::MdfReader*>(reader)->ReadHeader();
}

bool MdfReaderReadMeasurementInfo(MdfReader *reader) {
    return reinterpret_cast<mdf::MdfReader*>(reader)->ReadMeasurementInfo();
}

bool MdfReaderReadEverythingButData(MdfReader *reader) {
    return reinterpret_cast<mdf::MdfReader*>(reader)->ReadEverythingButData();
}

bool MdfReaderReadData(MdfReader *reader, IDataGroup *group) {
    return reinterpret_cast<mdf::MdfReader*>(reader)->ReadData(reinterpret_cast<const mdf::IDataGroup*>(group));
}

// MdfWriter functions
MdfWriter *MdfWriterInit(MdfWriterType type, const char *filename) {
    MdfLibrary::MdfWriterType writer_type;
    // Note: MdfWriterType enum in wrapper.h has different values than in MdfExport.h
    // Assuming names are what matters.
    // wrapper.h: MDF_WRITER_TYPE_MDF4 = 0, MDF_WRITER_TYPE_MDF3 = 1
    // MdfExport.h: Mdf3Basic = 0, Mdf4Basic = 1
    switch (type) {
        case MDF_WRITER_TYPE_MDF4: // is 0
            writer_type = MdfLibrary::MdfWriterType::Mdf4Basic; // is 1
            break;
        case MDF_WRITER_TYPE_MDF3: // is 1
            writer_type = MdfLibrary::MdfWriterType::Mdf3Basic; // is 0
            break;
        default:
            return nullptr;
    }
    auto writer = mdf::MdfFactory::CreateMdfWriter(writer_type, filename);
    return reinterpret_cast<MdfWriter*>(writer.release());
}

void MdfWriterUnInit(MdfWriter *writer) {
    delete reinterpret_cast<mdf::MdfWriter*>(writer);
}

MdfFile *MdfWriterGetFile(MdfWriter *writer) {
    return reinterpret_cast<MdfFile*>(reinterpret_cast<mdf::MdfWriter*>(writer)->GetFile());
}

IHeader *MdfWriterGetHeader(MdfWriter *writer) {
    return reinterpret_cast<IHeader*>(reinterpret_cast<mdf::MdfWriter*>(writer)->GetHeader());
}

bool MdfWriterIsFileNew(MdfWriter *writer) {
    return reinterpret_cast<mdf::MdfWriter*>(writer)->IsFileNew();
}

bool MdfWriterGetCompressData(MdfWriter *writer) {
    return reinterpret_cast<mdf::MdfWriter*>(writer)->GetCompressData();
}

void MdfWriterSetCompressData(MdfWriter *writer, bool compress) {
    reinterpret_cast<mdf::MdfWriter*>(writer)->SetCompressData(compress);
}

double MdfWriterGetPreTrigTime(MdfWriter *writer) {
    return reinterpret_cast<mdf::MdfWriter*>(writer)->GetPreTrigTime();
}

void MdfWriterSetPreTrigTime(MdfWriter *writer, double pre_trig_time) {
    reinterpret_cast<mdf::MdfWriter*>(writer)->SetPreTrigTime(pre_trig_time);
}

uint64_t MdfWriterGetStartTime(MdfWriter *writer) {
    return reinterpret_cast<mdf::MdfWriter*>(writer)->GetStartTime();
}

uint64_t MdfWriterGetStopTime(MdfWriter *writer) {
    return reinterpret_cast<mdf::MdfWriter*>(writer)->GetStopTime();
}

uint16_t MdfWriterGetBusType(MdfWriter *writer) {
    return static_cast<uint16_t>(reinterpret_cast<mdf::MdfWriter*>(writer)->GetBusType());
}

void MdfWriterSetBusType(MdfWriter *writer, uint16_t type) {
    reinterpret_cast<mdf::MdfWriter*>(writer)->SetBusType(static_cast<MdfLibrary::MdfBusType>(type));
}

MdfStorageType MdfWriterGetStorageType(MdfWriter *writer) {
    return static_cast<MdfStorageType>(reinterpret_cast<mdf::MdfWriter*>(writer)->GetStorageType());
}

void MdfWriterSetStorageType(MdfWriter *writer, MdfStorageType type) {
    reinterpret_cast<mdf::MdfWriter*>(writer)->SetStorageType(static_cast<MdfLibrary::MdfStorageType>(type));
}

uint32_t MdfWriterGetMaxLength(MdfWriter *writer) {
    return reinterpret_cast<mdf::MdfWriter*>(writer)->GetMaxLength();
}

void MdfWriterSetMaxLength(MdfWriter *writer, uint32_t length) {
    reinterpret_cast<mdf::MdfWriter*>(writer)->SetMaxLength(length);
}

bool MdfWriterCreateBusLogConfiguration(MdfWriter *writer) {
    return reinterpret_cast<mdf::MdfWriter*>(writer)->CreateBusLogConfiguration();
}

IDataGroup *MdfWriterCreateDataGroup(MdfWriter *writer) {
    return reinterpret_cast<IDataGroup*>(reinterpret_cast<mdf::MdfWriter*>(writer)->CreateDataGroup());
}

bool MdfWriterInitMeasurement(MdfWriter *writer) {
    return reinterpret_cast<mdf::MdfWriter*>(writer)->InitMeasurement();
}

void MdfWriterSaveSample(MdfWriter *writer, IChannelGroup *group, uint64_t time) {
    reinterpret_cast<mdf::MdfWriter*>(writer)->SaveSample(*reinterpret_cast<mdf::IChannelGroup*>(group), time);
}

void MdfWriterSaveCanMessage(MdfWriter *writer, IChannelGroup *group, uint64_t time, CanMessage *message) {
    reinterpret_cast<mdf::MdfWriter*>(writer)->SaveCanMessage(*reinterpret_cast<mdf::IChannelGroup*>(group), time, *reinterpret_cast<mdf::CanMessage*>(message));
}

void MdfWriterStartMeasurement(MdfWriter *writer, uint64_t start_time) {
    reinterpret_cast<mdf::MdfWriter*>(writer)->StartMeasurement(start_time);
}

void MdfWriterStopMeasurement(MdfWriter *writer, uint64_t stop_time) {
    reinterpret_cast<mdf::MdfWriter*>(writer)->StopMeasurement(stop_time);
}

bool MdfWriterFinalizeMeasurement(MdfWriter *writer) {
    return reinterpret_cast<mdf::MdfWriter*>(writer)->FinalizeMeasurement();
}

// MdfFile functions
size_t MdfFileGetName(MdfFile *file, char *name) {
    if (!file) return 0;
    std::string str = reinterpret_cast<mdf::MdfFile*>(file)->Name();
    if (name) {
        strcpy(name, str.c_str());
    }
    return str.length();
}

void MdfFileSetName(MdfFile *file, const char *name) {
    if (!file) return;
    reinterpret_cast<mdf::MdfFile*>(file)->Name(name);
}

size_t MdfFileGetFileName(MdfFile *file, char *filename) {
    if (!file) return 0;
    std::string str = reinterpret_cast<mdf::MdfFile*>(file)->FileName();
    if (filename) {
        strcpy(filename, str.c_str());
    }
    return str.length();
}

void MdfFileSetFileName(MdfFile *file, const char *filename) {
    if (!file) return;
    reinterpret_cast<mdf::MdfFile*>(file)->FileName(filename);
}

size_t MdfFileGetVersion(MdfFile *file, char *version) {
    if (!file) return 0;
    std::string str = reinterpret_cast<mdf::MdfFile*>(file)->Version();
    if (version) {
        strcpy(version, str.c_str());
    }
    return str.length();
}

int MdfFileGetMainVersion(MdfFile *file) {
    if (!file) return 0;
    return reinterpret_cast<mdf::MdfFile*>(file)->MainVersion();
}

int MdfFileGetMinorVersion(MdfFile *file) {
    if (!file) return 0;
    return reinterpret_cast<mdf::MdfFile*>(file)->MinorVersion();
}

void MdfFileSetMinorVersion(MdfFile *file, int minor) {
    if (!file) return;
    reinterpret_cast<mdf::MdfFile*>(file)->MinorVersion(minor);
}

size_t MdfFileGetProgramId(MdfFile *file, char *program_id) {
    if (!file) return 0;
    std::string str = reinterpret_cast<mdf::MdfFile*>(file)->ProgramId();
    if (program_id) {
        strcpy(program_id, str.c_str());
    }
    return str.length();
}

void MdfFileSetProgramId(MdfFile *file, const char *program_id) {
    if (!file) return;
    reinterpret_cast<mdf::MdfFile*>(file)->ProgramId(program_id);
}

bool MdfFileGetFinalized(MdfFile *file, uint16_t *standard_flags, uint16_t *custom_flags) {
    if (!file || !standard_flags || !custom_flags) return false;
    return reinterpret_cast<mdf::MdfFile*>(file)->GetFinalized(*standard_flags, *custom_flags);
}

const IHeader *MdfFileGetHeader(MdfFile *file) {
    if (!file) return nullptr;
    return reinterpret_cast<const IHeader*>(reinterpret_cast<const mdf::MdfFile*>(file)->Header());
}

bool MdfFileGetIsMdf4(MdfFile *file) {
    if (!file) return false;
    return reinterpret_cast<mdf::MdfFile*>(file)->IsMdf4();
}

size_t MdfFileGetAttachments(MdfFile *file, const IAttachment **pAttachment) {
    if (!file) return 0;
    auto attachments = reinterpret_cast<mdf::MdfFile*>(file)->Attachments();
    if (pAttachment) {
        for (size_t i = 0; i < attachments.size(); ++i) {
            pAttachment[i] = reinterpret_cast<const IAttachment*>(attachments[i]);
        }
    }
    return attachments.size();
}

size_t MdfFileGetDataGroups(MdfFile *file, const IDataGroup **pDataGroup) {
    if (!file) return 0;
    auto data_groups = reinterpret_cast<mdf::MdfFile*>(file)->DataGroups();
    if (pDataGroup) {
        for (size_t i = 0; i < data_groups.size(); ++i) {
            pDataGroup[i] = reinterpret_cast<const IDataGroup*>(data_groups[i]);
        }
    }
    return data_groups.size();
}

IAttachment *MdfFileCreateAttachment(MdfFile *file) {
    if (!file) return nullptr;
    return reinterpret_cast<IAttachment*>(reinterpret_cast<mdf::MdfFile*>(file)->CreateAttachment());
}

IDataGroup *MdfFileCreateDataGroup(MdfFile *file) {
    if (!file) return nullptr;
    return reinterpret_cast<IDataGroup*>(reinterpret_cast<mdf::MdfFile*>(file)->CreateDataGroup());
}

// CanMessage functions
CanMessage *CanMessageInit(void) {
    return reinterpret_cast<CanMessage*>(new mdf::CanMessage());
}

void CanMessageUnInit(CanMessage *can) {
    delete reinterpret_cast<mdf::CanMessage*>(can);
}

uint32_t CanMessageGetMessageId(CanMessage *can) {
    if (!can) return 0;
    return reinterpret_cast<mdf::CanMessage*>(can)->MessageId();
}

void CanMessageSetMessageId(CanMessage *can, uint32_t msgId) {
    if (!can) return;
    reinterpret_cast<mdf::CanMessage*>(can)->MessageId(msgId);
}

uint32_t CanMessageGetCanId(CanMessage *can) {
    if (!can) return 0;
    return reinterpret_cast<mdf::CanMessage*>(can)->CanId();
}

bool CanMessageGetExtendedId(CanMessage *can) {
    if (!can) return false;
    return reinterpret_cast<mdf::CanMessage*>(can)->ExtendedId();
}

void CanMessageSetExtendedId(CanMessage *can, bool extendedId) {
    if (!can) return;
    reinterpret_cast<mdf::CanMessage*>(can)->ExtendedId(extendedId);
}

uint8_t CanMessageGetDlc(CanMessage *can) {
    if (!can) return 0;
    return reinterpret_cast<mdf::CanMessage*>(can)->Dlc();
}

void CanMessageSetDlc(CanMessage *can, uint8_t dlc) {
    if (!can) return;
    reinterpret_cast<mdf::CanMessage*>(can)->Dlc(dlc);
}

size_t CanMessageGetDataLength(CanMessage *can) {
    if (!can) return 0;
    return reinterpret_cast<mdf::CanMessage*>(can)->DataLength();
}

void CanMessageSetDataLength(CanMessage *can, uint32_t dataLength) {
    if (!can) return;
    reinterpret_cast<mdf::CanMessage*>(can)->DataLength(dataLength);
}

size_t CanMessageGetDataBytes(CanMessage *can, uint8_t *dataList) {
    if (!can) return 0;
    auto data = reinterpret_cast<mdf::CanMessage*>(can)->DataBytes();
    if (dataList) {
        memcpy(dataList, data.data(), data.size());
    }
    return data.size();
}

void CanMessageSetDataBytes(CanMessage *can, const uint8_t *dataList, size_t size) {
    if (!can) return;
    reinterpret_cast<mdf::CanMessage*>(can)->DataBytes({dataList, dataList + size});
}

}
