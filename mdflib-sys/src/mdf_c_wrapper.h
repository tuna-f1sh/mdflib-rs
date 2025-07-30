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
typedef struct IChannelObserver IChannelObserver;

enum class MdfWriterType : int {
  Mdf3Basic = 0, ///< Basic MDF version 3 writer.
  Mdf4Basic = 1,  ///< Basic MDF version 4 writer.
  MdfBusLogger = 2, ///< Specialized bus logger writer.
  MdfConverter = 3, ///< MDF writer for MDF 4 conversion applications.
};

enum class WriteState : uint8_t {
  Create,     ///< Only at first measurement
  Init,       ///< Start work thread and start collecting samples
  StartMeas,  ///< Start saving samples to file
  StopMeas,   ///< Stop saving samples. OK to
  Finalize    ///< OK to add new DG and CG blocks
};

/** \brief Enumerate that defines type of bus. Only relevant for bus logging.
 *
 * Enumerate that is used when doing bus logging. The enumerate is used when
 * creating default channel and channel group names.
 */
enum class MdfBusType : int {
  CAN = 0x01,       ///< CAN or CAN-FD bus
  LIN = 0x02,       ///< LIN bus
  FlexRay = 0x04,   ///< FlexRay bus
  MOST = 0x08,      ///< MOST bus
  Ethernet = 0x10,  ///< Ethernet bus
  UNKNOWN = 0x00    ///< Unknown bus type (Default)
};

/** \brief Enumerate that defines how the raw data is stored. By default
 * the fixed length record is stored. Only used when doing bus logging.
 *
 * The fixed length storage is using one SD-block per byte array. The SD block
 * is temporary stored in primary memory instead of store it on disc. This
 * storage type is not recommended for bus logging.
 *
 * The variable length storage uses an extra CG-record for byte array data.
 * The storage type is used for bus logging where payload data is more than 8
 * byte.
 *
 * The maximum length storage shall be used when payload data is 8 bytes or
 * less. It is typically used when logging CAN messages which have 0-8 data
 * payload.
 */
enum class MdfStorageType : int {
  FixedLengthStorage,  ///< The default is to use fixed length records.
  VlsdStorage,         ///< Using variable length storage.
  MlsdStorage,         ///< Using maximum length storage
};

/** \brief Channel functional type.
 *
 * Most channels are marked as 'FixedLength' which means that its
 * size in a record is fixed. This works well with most data types but
 * byte arrays and strings may change its size. Instead are these data types
 * marked as 'Variable Length'. Avoid writing variable length data as it
 * may allocate a lot of memory as it flush at the end of the measurement.
 *
 * One channel in channel group (IChannelGroup), should be marked as a master
 * channel. This channel is typical relative sample time with seconds as
 * unit. The master channel is typical used on the X-axis when plotting data.
 *
 * The 'VirtualMaster' channel can be used if the sample number is linear
 * related to the sample time. The channel conversion (CC) block should
 * define the sample number to time conversion.
 *
 * The 'Sync' channel is used to synchronize an attachment block (file).
 *
 * The 'MaxLength' type is typical used when storing CAN byte array where
 * another channel stores actual bytes stored in a sample. For CAN the size
 * in the max record size is 8 bytes.
 *
 * The 'VirtualData' is similar to the 'VirtualMaster' channel but related to
 * data. Good luck to find a use of this type.
 */
enum class ChannelType : uint8_t {
  FixedLength = 0,    ///< Fixed length data (default type)
  VariableLength = 1, ///< Variable length data
  Master = 2,         ///< Master channel
  VirtualMaster = 3,  ///< Virtual master channel
  Sync = 4,           ///< Synchronize channel
  MaxLength = 5,      ///< Max length channel
  VirtualData = 6     ///< Virtual data channel
};

/** \brief Synchronization type
 *
 * Defines the synchronization type. The type is 'None' for fixed length
 * channel but should be set for master and synchronization channels.
 */
enum class ChannelSyncType : uint8_t {
  None = 0,     ///< No synchronization (default value)
  Time = 1,     ///< Time type
  Angle = 2,    ///< Angle type
  Distance = 3, ///< Distance type
  Index = 4     ///< Sample number
};

/** \brief Channel data type.
 *
 * Defines the channel data type. Avoid defining value sizes that doesn't align
 * to a byte size.
 *
 * The Le and Be extension is related to byte order. Little endian (Intel
 * byte order) while big endian (Motorola byte order).
 */
enum class ChannelDataType : uint8_t {
  UnsignedIntegerLe= 0,  ///< Unsigned integer, little endian.
  UnsignedIntegerBe = 1, ///< Unsigned integer, big endian.
  SignedIntegerLe = 2,   ///< Signed integer, little endian.
  SignedIntegerBe = 3,   ///< Signed integer, big endian.
  FloatLe = 4,           ///< Float, little endian.
  FloatBe = 5,           ///< Float, big endian.
  StringAscii = 6,       ///< Text,  ISO-8859-1 coded
  StringUTF8 = 7,        ///< Text, UTF8 coded.
  StringUTF16Le = 8,     ///< Text, UTF16 coded little endian.
  StringUTF16Be = 9,     ///< Text, UTF16 coded big endian.
  ByteArray = 10,        ///< Byte array.
  MimeSample = 11,       ///< MIME sample byte array.
  MimeStream = 12,       ///< MIME stream byte array.
  CanOpenDate = 13,      ///< 7-byte CANOpen date.
  CanOpenTime = 14,      ///< 6-byte CANOpen time.
  ComplexLe = 15,        ///< Complex value, little endian.
  ComplexBe = 16         ///< Complex value, big endian.
};

/** \brief Channel flags. See also IChannel::Flags().
 *
 */
namespace CnFlag {
constexpr uint32_t AllValuesInvalid = 0x0001; ///< All values are invalid.
constexpr uint32_t InvalidValid = 0x0002; ///< Invalid bit is used.
constexpr uint32_t PrecisionValid = 0x0004; ///< Precision is used.
constexpr uint32_t RangeValid = 0x0008; ///< Range is used.
constexpr uint32_t LimitValid = 0x0010; ///< Limit is used.
constexpr uint32_t ExtendedLimitValid = 0x0020; ///< Extended limit is used.
constexpr uint32_t Discrete = 0x0040; ///< Discrete channel.
constexpr uint32_t Calibration = 0x0080; ///< Calibrated channel.
constexpr uint32_t Calculated = 0x0100; ///< Calculated channel.
constexpr uint32_t Virtual = 0x0200; ///< Virtual channel.
constexpr uint32_t BusEvent = 0x0400; ///< Bus event channel.
constexpr uint32_t StrictlyMonotonous = 0x0800; ///< Strict monotonously.
constexpr uint32_t DefaultX = 0x1000; ///< Default x-axis channel.
constexpr uint32_t EventSignal = 0x2000; ///< Event signal.
constexpr uint32_t VlsdDataStream = 0x4000; ///< VLSD data stream channel.
}

/** \brief Type of array.
 *
 */
enum class ArrayType : uint8_t {
  Array = 0, ///< Simple array without attributes
  ScalingAxis = 1, ///< Scaling axis.
  LookUp = 2, ///< Lookup array.
  IntervalAxis = 3, ///< Interval axis.
  ClassificationResult = 4 ///< Classification result.
};

/** \brief Type of storage. */
enum class ArrayStorage : uint8_t {
  CnTemplate = 0, ///< Channel template.
  CgTemplate = 1, ///< Channel group template.
  DgTemplate = 2  ///< Data group template.
};

/** \brief Channel array (CA) block flags. */
namespace CaFlag {
constexpr uint32_t DynamicSize = 0x0001; ///< Dynamic size
constexpr uint32_t InputQuantity = 0x0002; ///< Input quantity.
constexpr uint32_t OutputQuantity = 0x0004; ///< Output quantity.
constexpr uint32_t ComparisonQuantity = 0x0008; ///< Comparison quantity.
constexpr uint32_t Axis = 0x0010; ///< Axis
constexpr uint32_t FixedAxis = 0x0020; ///< Fixed axis.
constexpr uint32_t InverseLayout = 0x0040; ///< Inverse layout.
constexpr uint32_t LeftOpenInterval = 0x0080; ///< Left-over interval.
constexpr uint32_t StandardAxis = 0x0100; ///< Standard axis.
}  // namespace CaFlag

/** \brief Type of conversion formula
 *
 * The type together with the Parameter() function defines
 * the conversion between channel and engineering value.
 *
 */
enum class ConversionType : uint8_t {
  /** \brief 1:1 conversion. No parameters needed. */
  NoConversion = 0,

  /** \brief Linear conversion. 2 parameters.
   * Eng = Ch * Par(1) + Par(0).
   */
  Linear = 1,

  /** \brief Rational function conversion. 6 parameters.
   *  Eng = (Par(0)*Ch^2 + Par(1)*Ch + Par(2)) /
   *  (Par(3)*Ch^2 + Par(4)*Ch + Par(5))
   */
  Rational = 2,
  Algebraic = 3,  ///< Text formula.

  /** \brief Value to value conversion with interpolation.
   * Defined by a list of Key value pairs.
   * Par(n) = key and Par(n+1) value.
   */
  ValueToValueInterpolation = 4,

  /** \brief Value to value conversion without interpolation.
   * Defined by a list of Key value pairs.
   * Par(n) = key and Par(n+1) value.
   */
  ValueToValue = 5,

  /** \brief Value range to value conversion without interpolation.
   * Defined by a list of Key min/max value triplets.
   * Par(3*n) = key min, Par(3*(n+1)) = key max and Par(3*(n+2)) value. Add a
   * default value last, after all the triplets.
   */
  ValueRangeToValue = 6,

  /** \brief Value to text conversion.
   * Defined by a list of key values to text string conversions. This is
   * normally used for enumerated channels.
   * Par(n) value to Ref(n) text. Add a default
   * referenced text last.
   */
  ValueToText = 7,

  /** \brief Value range to text conversion.
   * Defined by a list of key min/max values to text string conversions. This is
   * normally used for enumerated channels.
   * Par(2*n) min key, Par(2(n+1)) max key to Ref(n) value. Add a default
   * referenced text  last.
   */
  ValueRangeToText = 8,

  /** \brief Text to value conversion.
   * Defines a list of text string to value conversion.
   * Ref(n) key to Par(n) value. Add a default value last to the parameter list.
   */
  TextToValue = 9,

  /** \brief Text to text conversion.
   * Defines a list of text string to text conversion.
   * Ref(2*n) key to Ref(2*(n+1)) value.
   * Add a text value last to the parameter list.
   */
  TextToTranslation = 10,

  /** \brief Bitfield to text conversion
   * Currently unsupported conversion.
   */
  BitfieldToText = 11,
  // MDF 3 types
  Polynomial = 30,      ///< MDF 3 polynomial conversion.
  Exponential = 31,     ///< MDF 3 exponential conversion.
  Logarithmic = 32,     ///< MDF 3 logarithmic conversion.
  DateConversion = 33,  ///< MDF 3 Date conversion
  TimeConversion = 34   ///< MDF 3 Time conversion
};

/** \brief Channel conversion flags.
 *
 */
namespace CcFlag {
constexpr uint16_t PrecisionValid = 0x0001;  ///< Precision is used.
constexpr uint16_t RangeValid = 0x0002;      ///< Range is used.
constexpr uint16_t StatusString = 0x0004;    ///< Status string flag.
}  // namespace CcFlag

/** \brief Type of source information. */
enum class SourceType : uint8_t {
  Other = 0,     ///< Unknown source type.
  Ecu = 1,       ///< ECU.
  Bus = 2,       ///< Bus.
  IoDevice = 3,  ///< I/O device.
  Tool = 4,      ///< Tool.
  User = 5       ///< User.
};

/** \brief Type of bus. */
enum class BusType : uint8_t {
  None = 0,      ///< No bus (default).
  Other = 1,     ///< Unknown bus type.
  Can = 2,       ///< CAN bus.
  Lin = 3,       ///< LIN bus.
  Most = 4,      ///< MOST bus.
  FlexRay = 5,   ///< FlexRay bus.
  Kline = 6,     ///< KLINE bus.
  Ethernet = 7,  ///< EtherNet bus.
  Usb = 8        ///< USB bus.
};

/** \brief Source information flags. */
namespace SiFlag {
constexpr uint8_t Simulated = 0x01;  ///< Simulated device.
}

/** \brief Type of event. */
enum class EventType : uint8_t {
  RecordingPeriod = 0,       ///< Specifies a recording period (range).
  RecordingInterrupt = 1,    ///< The recording was interrupted.
  AcquisitionInterrupt = 2,  ///< The data acquisition was interrupted.
  StartRecording = 3,        ///< Start recording event.
  StopRecording = 4,         ///< Stop recording event.
  Trigger = 5,               ///< Generic event (no range).
  Marker = 6                 ///< Another generic event (maybe range).
};

/** \brief Type of synchronization value (default time) */
enum class SyncType : uint8_t {
  SyncTime = 1,      ///< Sync value represent time (s).
  SyncAngle = 2,     ///< Sync value represent angle (rad).
  SyncDistance = 3,  ///< Sync value represent distance (m).
  SyncIndex = 4,     ///< Sync value represent sample index.
};

/** \brief Type of range. */
enum class RangeType : uint8_t {
  RangePoint = 0,  ///< Defines a point
  RangeStart = 1,  ///< First in a range.
  RangeEnd = 2     ///< Last in a range.
};

/** \brief Type of cause. */
enum class EventCause : uint8_t {
  CauseOther = 0,   ///< Unknown source.
  CauseError = 1,   ///< An error generated this event.
  CauseTool = 2,    ///< The tool generated this event.
  CauseScript = 3,  ///< A script generated this event.
  CauseUser = 4,    ///< A user generated this event.
};

/** \brief The e-tag may optional have a data type below. The value in the
 * XML file is of course string but the data type may be used for
 * interpretation of the value. Note that unit property can also be added.
 *
 * Use ISO UTC date and time formats or avoid these data types if possible
 * as they just causing problem at presentation.
 */
enum class ETagDataType : uint8_t {
  StringType = 0,   ///< Text value.
  DecimalType = 1,  ///< Decimal value (use float instead)
  IntegerType = 2,  ///< Integer value
  FloatType = 3,    ///< Floating point value
  BooleanType = 4,  ///< Boolean tru/false value
  DateType = 5,     ///< Date value according to ISO (YYYY-MM-DD).
  TimeType = 6,     ///< Time value ISO
  DateTimeType = 7  ///< Date and Time ISO string (YYYY-MM-DD hh:mm:ss)
};

enum class CanErrorType : uint8_t {
  UNKNOWN_ERROR = 0,       ///< Unspecified error.
  BIT_ERROR = 1,           ///< CAN bit error.
  FORM_ERROR = 2,          ///< CAN format error.
  BIT_STUFFING_ERROR = 3,  ///< Bit stuffing error.
  CRC_ERROR = 4,           ///< Checksum error.
  ACK_ERROR = 5            ///< Acknowledgement error.
};

enum class MessageType : int {
  CAN_DataFrame,      ///< Normal CAN message
  CAN_RemoteFrame,    ///< Remote frame message.
  CAN_ErrorFrame,     ///< Error message.
  CAN_OverloadFrame,  ///< Overload frame message.
};

#if defined(_WIN32)
#define EXPORT __declspec(dllexport)
#elif defined(__linux__) || defined(__APPLE__) || defined(__CYGWIN__)
#define EXPORT __attribute__((visibility("default")))
#else
#define EXPORT
#endif

// MDF log severities
enum class MdfLogSeverity : uint8_t {
  kTrace = 0,
  kDebug,
  kInfo,
  kNotice,
  kWarning,
  kError,
  kCritical,
  kAlert,
  kEmergency
};

// MDF log location struct
typedef struct {
    int line;
    int column;
    const char* file;
    const char* function;
} MdfLocation;

// C-compatible log function pointer types
typedef void (*MdfCLogFunction1)(MdfLogSeverity severity, const char* text);
typedef void (*MdfCLogFunction2)(MdfLogSeverity severity, const char* function, const char* text);

// Functions to set the log callbacks
EXPORT void MdfSetLogFunction1(MdfCLogFunction1 func);
EXPORT void MdfSetLogFunction2(MdfCLogFunction2 func);

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
EXPORT void MdfWriterSaveSample(MdfWriter* writer, const IChannelGroup* group, uint64_t time);
EXPORT void MdfWriterSaveCanMessage(MdfWriter* writer, const IChannelGroup* group, uint64_t time, const CanMessage* message);
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
EXPORT size_t MdfFileGetDataGroups(const MdfFile* file, IDataGroup* dest[], size_t max_count);
EXPORT const IDataGroup* MdfFileFindParentDataGroup(const MdfFile *file, const IChannel &channel);
EXPORT void MdfFileSetProgramId(MdfFile *file, const char *program_id);
EXPORT size_t MdfFileGetProgramId(const MdfFile *file, char *buffer, size_t max_length);
// EXPORT void MdfFileReadHeader(MdfFile *file);
// EXPORT void MdfFileReadMeasurementInfo(MdfFile *file);
// EXPORT void MdfFileReadEverythingButData(MdfFile *file);
// EXPORT bool MdfFileWrite(MdfFile *file);
EXPORT bool MdfFileIsFinalizedDone(const MdfFile *file);

// IDataGroup functions
EXPORT uint64_t DataGroupGetIndex(const IDataGroup* group);
EXPORT size_t DataGroupGetDescription(const IDataGroup* group, char* description, size_t max_length);
EXPORT void DataGroupSetDescription(IDataGroup* group, const char* description);
EXPORT size_t DataGroupGetChannelGroupCount(const IDataGroup* group);
EXPORT IChannelGroup* DataGroupGetChannelGroupByIndex(const IDataGroup* group, size_t index);
EXPORT IChannelGroup* DataGroupGetChannelGroupByName(const IDataGroup* group, const char* name);
EXPORT IChannelGroup* DataGroupCreateChannelGroup(IDataGroup* group);
EXPORT void DataGroupClearData(IDataGroup *group);

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
EXPORT const IChannel *ChannelGroupGetChannelByName(const IChannelGroup *group, const char *name);
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
EXPORT size_t IHeaderGetDataGroupCount(const IHeader *header);

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
EXPORT uint32_t CanMessageGetMessageIdConst(const CanMessage* can);
EXPORT void CanMessageSetMessageId(CanMessage* can, uint32_t msgId);
EXPORT uint32_t CanMessageGetCanId(CanMessage* can);
EXPORT uint32_t CanMessageGetCanIdConst(const CanMessage* can);
EXPORT bool CanMessageGetExtendedId(CanMessage* can);
EXPORT bool CanMessageGetExtendedIdConst(const CanMessage* can);
EXPORT void CanMessageSetExtendedId(CanMessage* can, bool extendedId);
EXPORT uint8_t CanMessageGetDlc(CanMessage* can);
EXPORT uint8_t CanMessageGetDlcConst(const CanMessage* can);
EXPORT void CanMessageSetDlc(CanMessage* can, uint8_t dlc);
EXPORT size_t CanMessageGetDataLength(CanMessage* can);
EXPORT size_t CanMessageGetDataLengthConst(const CanMessage* can);
EXPORT void CanMessageSetDataLength(CanMessage* can, uint32_t dataLength);
EXPORT size_t CanMessageGetDataBytes(CanMessage* can, uint8_t* dataList, size_t max_length);
EXPORT size_t CanMessageGetDataBytesConst(const CanMessage* can, uint8_t* dataList, size_t max_length);
EXPORT void CanMessageSetDataBytes(CanMessage* can, const uint8_t* dataList, size_t size);
EXPORT uint32_t CanMessageGetBusChannel(const CanMessage* can);
EXPORT void CanMessageSetBusChannel(CanMessage* can, uint32_t busChannel);

// IChannelObserver functions
EXPORT IChannelObserver* CreateChannelObserver(const IDataGroup* dataGroup, const IChannelGroup* channelGroup, const IChannel* channel);
EXPORT void ChannelObserverUnInit(IChannelObserver* observer);
EXPORT size_t ChannelObserverGetNofSamples(const IChannelObserver* observer);
EXPORT bool ChannelObserverGetChannelValue(const IChannelObserver* observer, size_t sample, double* value);
EXPORT bool ChannelObserverGetEngValue(const IChannelObserver* observer, size_t sample, double* value);
EXPORT bool ChannelObserverGetValid(const IChannelObserver* observer, size_t sample);

#ifdef __cplusplus
}
#endif

#endif // MDF_C_WRAPPER_H
