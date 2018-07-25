use std::collections::HashMap;

// use ::console::{log, log_u32};
use ::byte_order::ByteOrder;

/*
pub struct Pixel {
    red: u32,
    green: u32,
    blue: u32,
    alpha: u32,
}

pub struct Image {
    pixels: [Pixel],
}
*/

#[derive(Hash, Eq, Debug, PartialEq, Clone, Copy)]
pub enum IfdEntryTag {
    NewSubfileType,
    SubfileType,
    ImageWidth,
    ImageLength,
    BitsPerSample,
    Compression,
    PhotometricInterpretation,
    Threshholding,
    CellWidth,
    CellLength,
    FillOrder,
    ImageDescription,
    Make,
    Model,
    StripOffsets,
    Orientation,
    SamplesPerPixel,
    RowsPerStrip,
    StripByteCounts,
    MinSampleValue,
    MaxSampleValue,
    XResolution,
    YResolution,
    PlanarConfiguration,
    FreeOffsets,
    FreeByteCounts,
    GrayResponseUnit,
    GrayResponseCurve,
    ResolutionUnit,
    Software,
    DateTime,
    Artist,
    HostComputer,
    ColorMap,
    TileWidth,
    TileLength,
    TileOffsets,
    TileByteCounts,
    SubIFDs,
    ExtraSamples,
    YCbCrCoefficients,
    YCbCrSubSampling,
    YCbCrPositioning,
    ReferenceBlackWhite,
    XMP,
    CFARepeatPatternDim,
    CFAPattern,
    Copyright,
    EXIF,
    ImageNumber,

    // DNG Tags:
    DNGVersion,
    DNGBackwardVersion,
    UniqueCameraModel,
    LocalizedCameraModel,
    CFAPlaneColor,
    CFALayout,
    LinearizationTable,
    BlackLevelRepeatDim,
    BlackLevel,
    BlackLevelDeltaH,
    BlackLevelDeltaV,
    WhiteLevel,
    DefaultScale,
    DefaultCropOrigin,
    DefaultCropSize,
    ColorMatrix1,
    ColorMatrix2,
    CameraCalibration1,
    CameraCalibration2,
    ReductionMatrix1,
    ReductionMatrix2,
    AnalogBalance,
    AsShotNeutral,
    AsShotWhiteXY,
    BaselineExposure,
    BaselineNoise,
    BaselineSharpness,
    BayerGreenSplit,
    LinearResponseLimit,
    CameraSerialNumber,
    LensInfo,
    ChromaBlurRadius,
    AntiAliasStrength,
    ShadowScale,
    DNGPrivateData,
    MakerNoteSafety,
    CalibrationIlluminant1,
    CalibrationIlluminant2,
    BestQualityScale,
    RawDataUniqueID,
    OriginalRawFileName,
    OriginalRawFileData,
    ActiveArea,
    MaskedAreas,
    AsShotICCProfile,
    AsShotPreProfileMatrix,
    CurrentICCProfile,
    CurrentPreProfileMatrix,
    ColorimetricReference,
    CameraCalibrationSignature,
    ProfileCalibrationSignature,
    ExtraCameraProfiles,
    AsShotProfileName,
    NoiseReductionApplied,
    ProfileName,
    ProfileHueSatMapDims,
    ProfileHueSatMapData1,
    ProfileHueSatMapData2,
    ProfileToneCurve,
    ProfileEmbedPolicy,
    ProfileCopyright,
    ForwardMatrix1,
    ForwardMatrix2,
    PreviewApplicationName,
    PreviewApplicationVersion,
    PreviewSettingsName,
    PreviewSettingsDigest,
    PreviewColorSpace,
    PreviewDateTime,
    RawImageDigest,
    OriginalRawFileDigest,
    SubTileBlockSize,
    RowInterleaveFactor,
    ProfileLookTableDims,
    ProfileLookTableData,
    OpcodeList1,
    OpcodeList2,
    OpcodeList3,
    NoiseProfile,
    OriginalDefaultFinalSize,
    OriginalBestQualityFinalSize,
    OriginalDefaultCropSize,
    ProfileHueSatMapEncoding,
    ProfileLookTableEncoding,
    BaselineExposureOffset,
    DefaultBlackRender,
    NewRawImageDigest,
    RawToPreviewGain,
    CacheVersion,
    DefaultUserCrop,
    Other(u16),
}

impl IfdEntryTag {
    fn from_u16(value: u16) -> IfdEntryTag {
        match value {
            254 => IfdEntryTag::NewSubfileType,
            255 => IfdEntryTag::SubfileType,
            256 => IfdEntryTag::ImageWidth,
            257 => IfdEntryTag::ImageLength,
            258 => IfdEntryTag::BitsPerSample,
            259 => IfdEntryTag::Compression,
            262 => IfdEntryTag::PhotometricInterpretation,
            263 => IfdEntryTag::Threshholding,
            264 => IfdEntryTag::CellWidth,
            265 => IfdEntryTag::CellLength,
            266 => IfdEntryTag::FillOrder,
            270 => IfdEntryTag::ImageDescription,
            271 => IfdEntryTag::Make,
            272 => IfdEntryTag::Model,
            273 => IfdEntryTag::StripOffsets,
            274 => IfdEntryTag::Orientation,
            277 => IfdEntryTag::SamplesPerPixel,
            278 => IfdEntryTag::RowsPerStrip,
            279 => IfdEntryTag::StripByteCounts,
            280 => IfdEntryTag::MinSampleValue,
            281 => IfdEntryTag::MaxSampleValue,
            282 => IfdEntryTag::XResolution,
            283 => IfdEntryTag::YResolution,
            284 => IfdEntryTag::PlanarConfiguration,
            288 => IfdEntryTag::FreeOffsets,
            289 => IfdEntryTag::FreeByteCounts,
            290 => IfdEntryTag::GrayResponseUnit,
            291 => IfdEntryTag::GrayResponseCurve,
            296 => IfdEntryTag::ResolutionUnit,
            305 => IfdEntryTag::Software,
            306 => IfdEntryTag::DateTime,
            315 => IfdEntryTag::Artist,
            316 => IfdEntryTag::HostComputer,
            320 => IfdEntryTag::ColorMap,
            322 => IfdEntryTag::TileWidth,
            323 => IfdEntryTag::TileLength,
            324 => IfdEntryTag::TileOffsets,
            325 => IfdEntryTag::TileByteCounts,
            330 => IfdEntryTag::SubIFDs,
            338 => IfdEntryTag::ExtraSamples,
            529 => IfdEntryTag::YCbCrCoefficients,
            530 => IfdEntryTag::YCbCrSubSampling,
            531 => IfdEntryTag::YCbCrPositioning,
            532 => IfdEntryTag::ReferenceBlackWhite,
            700 => IfdEntryTag::XMP,
            33421 => IfdEntryTag::CFARepeatPatternDim,
            33422 => IfdEntryTag::CFAPattern,
            33432 => IfdEntryTag::Copyright,
            34665 => IfdEntryTag::EXIF,
            37393 => IfdEntryTag::ImageNumber,
            50706 => IfdEntryTag::DNGVersion,
            50707 => IfdEntryTag::DNGBackwardVersion,
            50708 => IfdEntryTag::UniqueCameraModel,
            50709 => IfdEntryTag::LocalizedCameraModel,
            50710 => IfdEntryTag::CFAPlaneColor,
            50711 => IfdEntryTag::CFALayout,
            50712 => IfdEntryTag::LinearizationTable,
            50713 => IfdEntryTag::BlackLevelRepeatDim,
            50714 => IfdEntryTag::BlackLevel,
            50715 => IfdEntryTag::BlackLevelDeltaH,
            50716 => IfdEntryTag::BlackLevelDeltaV,
            50717 => IfdEntryTag::WhiteLevel,
            50718 => IfdEntryTag::DefaultScale,
            50719 => IfdEntryTag::DefaultCropOrigin,
            50720 => IfdEntryTag::DefaultCropSize,
            50721 => IfdEntryTag::ColorMatrix1,
            50722 => IfdEntryTag::ColorMatrix2,
            50723 => IfdEntryTag::CameraCalibration1,
            50724 => IfdEntryTag::CameraCalibration2,
            50725 => IfdEntryTag::ReductionMatrix1,
            50726 => IfdEntryTag::ReductionMatrix2,
            50727 => IfdEntryTag::AnalogBalance,
            50728 => IfdEntryTag::AsShotNeutral,
            50729 => IfdEntryTag::AsShotWhiteXY,
            50730 => IfdEntryTag::BaselineExposure,
            50731 => IfdEntryTag::BaselineNoise,
            50732 => IfdEntryTag::BaselineSharpness,
            50733 => IfdEntryTag::BayerGreenSplit,
            50734 => IfdEntryTag::LinearResponseLimit,
            50735 => IfdEntryTag::CameraSerialNumber,
            50736 => IfdEntryTag::LensInfo,
            50737 => IfdEntryTag::ChromaBlurRadius,
            50738 => IfdEntryTag::AntiAliasStrength,
            50739 => IfdEntryTag::ShadowScale,
            50740 => IfdEntryTag::DNGPrivateData,
            50741 => IfdEntryTag::MakerNoteSafety,
            50778 => IfdEntryTag::CalibrationIlluminant1,
            50779 => IfdEntryTag::CalibrationIlluminant2,
            50780 => IfdEntryTag::BestQualityScale,
            50781 => IfdEntryTag::RawDataUniqueID,
            50827 => IfdEntryTag::OriginalRawFileName,
            50828 => IfdEntryTag::OriginalRawFileData,
            50829 => IfdEntryTag::ActiveArea,
            50830 => IfdEntryTag::MaskedAreas,
            50831 => IfdEntryTag::AsShotICCProfile,
            50832 => IfdEntryTag::AsShotPreProfileMatrix,
            50833 => IfdEntryTag::CurrentICCProfile,
            50834 => IfdEntryTag::CurrentPreProfileMatrix,
            50879 => IfdEntryTag::ColorimetricReference,
            50931 => IfdEntryTag::CameraCalibrationSignature,
            50932 => IfdEntryTag::ProfileCalibrationSignature,
            50933 => IfdEntryTag::ExtraCameraProfiles,
            50934 => IfdEntryTag::AsShotProfileName,
            50935 => IfdEntryTag::NoiseReductionApplied,
            50936 => IfdEntryTag::ProfileName,
            50937 => IfdEntryTag::ProfileHueSatMapDims,
            50938 => IfdEntryTag::ProfileHueSatMapData1,
            50939 => IfdEntryTag::ProfileHueSatMapData2,
            50940 => IfdEntryTag::ProfileToneCurve,
            50941 => IfdEntryTag::ProfileEmbedPolicy,
            50942 => IfdEntryTag::ProfileCopyright,
            50964 => IfdEntryTag::ForwardMatrix1,
            50965 => IfdEntryTag::ForwardMatrix2,
            50966 => IfdEntryTag::PreviewApplicationName,
            50967 => IfdEntryTag::PreviewApplicationVersion,
            50968 => IfdEntryTag::PreviewSettingsName,
            50969 => IfdEntryTag::PreviewSettingsDigest,
            50970 => IfdEntryTag::PreviewColorSpace,
            50971 => IfdEntryTag::PreviewDateTime,
            50972 => IfdEntryTag::RawImageDigest,
            50973 => IfdEntryTag::OriginalRawFileDigest,
            50974 => IfdEntryTag::SubTileBlockSize,
            50975 => IfdEntryTag::RowInterleaveFactor,
            50981 => IfdEntryTag::ProfileLookTableDims,
            50982 => IfdEntryTag::ProfileLookTableData,
            51008 => IfdEntryTag::OpcodeList1,
            51009 => IfdEntryTag::OpcodeList2,
            51022 => IfdEntryTag::OpcodeList3,
            51041 => IfdEntryTag::NoiseProfile,
            51089 => IfdEntryTag::OriginalDefaultFinalSize,
            51090 => IfdEntryTag::OriginalBestQualityFinalSize,
            51091 => IfdEntryTag::OriginalDefaultCropSize,
            51107 => IfdEntryTag::ProfileHueSatMapEncoding,
            51108 => IfdEntryTag::ProfileLookTableEncoding,
            51109 => IfdEntryTag::BaselineExposureOffset,
            51110 => IfdEntryTag::DefaultBlackRender,
            51111 => IfdEntryTag::NewRawImageDigest,
            51112 => IfdEntryTag::RawToPreviewGain,
            51114 => IfdEntryTag::CacheVersion,
            51125 => IfdEntryTag::DefaultUserCrop,
            _ => IfdEntryTag::Other(value),
        }
    }
}

#[derive(Eq, Debug, PartialEq, Clone, Copy)]
pub enum IfdEntryType {
    Byte,
    Ascii,
    Short,
    Long,
    Rational,
    SByte,
    Undefined,
    SShort,
    SLong,
    SRational,
    Float,
    Double,
    Unknown,
}

impl IfdEntryType {
    fn from_u16(value: u16) -> IfdEntryType {
        match value {
            1 => IfdEntryType::Byte,
            2 => IfdEntryType::Ascii,
            3 => IfdEntryType::Short,
            4 => IfdEntryType::Long,
            5 => IfdEntryType::Rational,
            6 => IfdEntryType::SByte,
            7 => IfdEntryType::Undefined,
            8 => IfdEntryType::SShort,
            9 => IfdEntryType::SLong,
            10 => IfdEntryType::SRational,
            11 => IfdEntryType::Float,
            12 => IfdEntryType::Double,
            _ => IfdEntryType::Unknown,
        }
    }
}

#[derive(Eq, Debug, PartialEq, Clone, Copy)]
pub struct IfdEntry {
    tag: IfdEntryTag,
    pub entry_type: IfdEntryType,
    pub count: u32,
    pub offset: usize,
}



pub struct Dng<'a> {
    buffer: &'a [u8],
    pub ifds: Vec<HashMap<IfdEntryTag, IfdEntry>>,
    byte_order: ByteOrder,
}

impl<'a> Dng<'a> {
    pub fn read_u32(&self, offset: usize) -> u32 {
        self.byte_order.read_u32(self.buffer, offset)
    }

    pub fn read_u16(&self, offset: usize) -> u16 {
        self.byte_order.read_u16(self.buffer, offset)
    }

    pub fn read_u16_be(&self, offset: usize) -> u16 {
        ByteOrder::BigEndian.read_u16(self.buffer, offset)
    }

    pub fn read_u8(&self, offset: usize) -> u8 {
        self.buffer[offset]
    }
}

fn is_offset(entry_type: IfdEntryType, count: u32) -> bool {
    match entry_type {
        IfdEntryType::Byte 
            | IfdEntryType::Ascii 
            | IfdEntryType::SByte
            | IfdEntryType::Undefined => count > 4,
        IfdEntryType::Short
            | IfdEntryType::SShort => count > 2,
        _ => count > 1,
    }
}


fn parse_ifd_list(byte_order: ByteOrder, buffer: &[u8], position: &mut usize) -> Vec<HashMap<IfdEntryTag, IfdEntry>> {
    let mut next_ifd = byte_order.read_u32(buffer, *position) as usize;
    let mut ifds = Vec::new();
    while next_ifd != 0 {
        *position = next_ifd;

        let ifd = parse_ifd(byte_order, buffer, position);
        next_ifd = byte_order.read_u32(buffer, *position) as usize;
        ifds.push(ifd);
    }


    let mut new_ifds = Vec::new();
    for i in 0..ifds.len() {
        match ifds[i].get(&IfdEntryTag::SubIFDs) {
            Some(entry) => {
                for offset in 0..entry.count {
                    let this_offset = byte_order.read_u32(buffer, entry.offset + (offset as usize * 4)) as usize;
                    new_ifds.push(parse_ifd(byte_order, buffer, &mut this_offset.clone()));
                }
            },
            None => ()
        }
    }

    ifds.extend(new_ifds);
    ifds
}


fn parse_ifd(byte_order: ByteOrder, buffer: &[u8], position: &mut usize) -> HashMap<IfdEntryTag, IfdEntry> {
    let ifd_entry_count = byte_order.read_u16(buffer, *position);
    *position += 2;
    let mut ifd_entries = HashMap::new();
    for _n in 0..ifd_entry_count {
        let entry = parse_ifd_entry(byte_order, buffer, position);
        ifd_entries.insert(entry.tag, entry);
    }

    ifd_entries
}

fn parse_ifd_entry(byte_order: ByteOrder, buffer: &[u8], position: &mut usize) -> IfdEntry {
    *position += 12;

    let entry_type = IfdEntryType::from_u16(byte_order.read_u16(buffer, *position - 10));
    let count = byte_order.read_u32(buffer, *position - 8);

    IfdEntry {
        tag: IfdEntryTag::from_u16(byte_order.read_u16(buffer, *position - 12)),
        entry_type: entry_type,
        count: count,
        offset: if is_offset(entry_type, count) { byte_order.read_u32(buffer, *position - 4) as usize } 
            else { *position - 4 },
    }
}

pub fn parse_dng(buffer: &[u8], _length: u32) -> Dng {
    let byte_order: ByteOrder = if buffer[0] == 0x49 && buffer[1] == 0x49 { ByteOrder::LittleEndian }
        else if buffer[0] == 0x4D && buffer[1] == 0x4D { ByteOrder::BigEndian }
        else { panic!("File has invalid Byte Order") };

    let mut position: usize = 2;
    let confirm_dng = byte_order.read_u16(buffer, position);
    position += 2;

    if confirm_dng != 42 {
        panic!("File is not a DNG {}", confirm_dng)
    }

    Dng {
        buffer: buffer,
        ifds: parse_ifd_list(byte_order, buffer, &mut position),
        byte_order: byte_order,
    }
}