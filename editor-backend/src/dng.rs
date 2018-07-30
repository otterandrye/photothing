use std::collections::HashMap;

// use ::console::log;
use ::byte_order::{BufferReader, ByteOrder};

#[derive(Debug)]
pub enum Compression {
    Uncompressed,
    Huffman,
    JpegCompressed,
    Deflate,
    PackBits,
    LossyJpeg,
    Unknown(u16),
}

impl Compression {
    fn from_u16(val: u16) -> Compression {
        match val {
            1 => Compression::Uncompressed,
            2 => Compression::Huffman,
            7 => Compression::JpegCompressed,
            8 => Compression::Deflate,
            32773 => Compression::PackBits,
            34892 => Compression::LossyJpeg,
            _ => Compression::Unknown(val),
        }
    }
}

#[derive(Debug)]
pub struct Strip {}

#[derive(Debug)]
pub enum NewSubfileType {
    Original,
    Preview,
    Unknown(u32),
}

impl NewSubfileType {
    fn from_u32(val: u32) -> NewSubfileType {
        match val {
            0 => NewSubfileType::Original,
            1 => NewSubfileType::Preview,
            _ => NewSubfileType::Unknown(val),
        }
    }
}

#[derive(Debug)]
pub enum PhotometricInterpretation {
    WhiteIsZero,
    BlackIsZero,
    RGB,
    YCbCr,
    ColorFilterArray,
    LinearRaw,
    Unknown(u16),
}

impl PhotometricInterpretation {
    fn from_u16(val: u16) -> PhotometricInterpretation {
        match val {
            0 => PhotometricInterpretation::WhiteIsZero,
            1 => PhotometricInterpretation::BlackIsZero,
            2 => PhotometricInterpretation::RGB,
            6 => PhotometricInterpretation::YCbCr,
            32803 => PhotometricInterpretation::ColorFilterArray,
            34892 => PhotometricInterpretation::LinearRaw,
            _ => PhotometricInterpretation::Unknown(val),
        }
    }
}

#[derive(Debug)]
pub enum PlanarConfiguration {
    Chunky,
    Planar,
    Unknown(u16),
}

impl PlanarConfiguration {
    fn from_u16(val: u16) -> PlanarConfiguration {
        match val {
            1 => PlanarConfiguration::Chunky,
            2 => PlanarConfiguration::Planar,
            _ => PlanarConfiguration::Unknown(val),
        }
    }
}

#[derive(Debug)]
pub enum Illumination {
    Unknown,
    Daylight,
    Fluorescent,
    Tungsten,
    Flash,
    FineWeather,
    CloudyWeather,
    Shade,
    DaylightFluorescent,
    DayWhiteFluorescent,
    CoolWhiteFluorescent,
    WhiteFluorescent,
    StandardLightA,
    StandardLightB,
    StandardLightC,
    D55,
    D65,
    D75,
    D50,
    StudioTungsten,
    Other,
}

impl Illumination {
    fn from_u16(val: u16) -> Illumination {
        match val {
            0 => Illumination::Unknown,
            1 => Illumination::Daylight,
            2 => Illumination::Fluorescent,
            3 => Illumination::Tungsten,
            4 => Illumination::Flash,
            9 => Illumination::FineWeather,
            10 => Illumination::CloudyWeather,
            11 => Illumination::Shade,
            12 => Illumination::DaylightFluorescent,
            13 => Illumination::DayWhiteFluorescent,
            14 => Illumination::CoolWhiteFluorescent,
            15 => Illumination::WhiteFluorescent,
            17 => Illumination::StandardLightA,
            18 => Illumination::StandardLightB,
            19 => Illumination::StandardLightC,
            20 => Illumination::D55,
            21 => Illumination::D65,
            22 => Illumination::D75,
            23 => Illumination::D50,
            24 => Illumination::StudioTungsten,
            255 => Illumination::Other,
            _ => Illumination::Other,
        }
    }
}

#[derive(Debug)]
pub enum CFALayout {
    Rectangular,
    StaggeredA,
    StaggeredB,
    StaggeredC,
    StaggeredD,
    StaggeredE,
    StaggeredF,
    StaggeredG,
    StaggeredH,
}

impl CFALayout {
    fn from_u16(val: u16) -> CFALayout {
        match val {
            1 => CFALayout::Rectangular,
            2 => CFALayout::StaggeredA,
            3 => CFALayout::StaggeredB,
            4 => CFALayout::StaggeredC,
            5 => CFALayout::StaggeredD,
            6 => CFALayout::StaggeredE,
            7 => CFALayout::StaggeredF,
            8 => CFALayout::StaggeredG,
            9 => CFALayout::StaggeredH,
            _ => CFALayout::Rectangular,
        }
    }
}

#[derive(Debug)]
pub enum Orientation {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
    LeftTop,
    RightTop,
    RightBottom,
    LeftBottom,
}

impl Orientation {
    fn from_u16(val: u16) -> Orientation {
        match val {
            1 => Orientation::TopLeft,
            2 => Orientation::TopRight,
            3 => Orientation::BottomRight,
            4 => Orientation::BottomLeft,
            5 => Orientation::LeftTop,
            6 => Orientation::RightTop,
            7 => Orientation::RightBottom,
            8 => Orientation::LeftBottom,
            _ => Orientation::TopLeft,
        }
    }
}

#[derive(Debug)]
pub enum PreviewColorSpace {
    Unknown,
    GrayGamma22,
    SRGB,
    AdobeRGB,
    ProPhotoRGB,
}

impl PreviewColorSpace {
    fn from_u32(val: u32) -> PreviewColorSpace {
        match val {
            0 => PreviewColorSpace::Unknown,
            1 => PreviewColorSpace::GrayGamma22,
            2 => PreviewColorSpace::SRGB,
            3 => PreviewColorSpace::AdobeRGB,
            4 => PreviewColorSpace::ProPhotoRGB,
            _ => PreviewColorSpace::SRGB,
        }
    }
}


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
    ExposureTime,
    EXIF,
    ExposureProgram,
    ISO,
    SensitivityType,
    RecommendedExposureIndex,
    ExifVersion,
    DateTimeOriginal,
    CreateDate,
    ShutterSpeedValue,
    ExposureCompensation,
    MeteringMode,
    Flash,
    ImageNumber,
    SubSecTimeOriginal,
    SubSecTimeDigitized,
    ColorSpace,
    FocalPlaneXResolution,
    FocalPlaneYResolution,
    FocalPlaneResolutionUnit,
    CustomRendered,
    ExposureMode,
    WhiteBalance,
    SceneCaptureType,
    SerialNumber,
    LensSerialNumber,

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

#[derive(Debug)]
pub enum IfdEntryValue {
    Ascii(String),
    Byte(u8),
    ByteTriple(u8, u8, u8),
    ByteQuad(u8, u8, u8, u8),
    Short(u16),
    ShortPair(u16, u16),
    ShortTriple(u16, u16, u16),
    Int(u32),
    IntTriple(u32, u32, u32),
    IntQuad(u32, u32, u32, u32),
    Rational(u32, u32),
    RationalPair(u32, u32, u32, u32),
    RationalTriple(u32, u32, u32, u32, u32, u32),
    RationalQuad(u32, u32, u32, u32, u32, u32, u32, u32),
    SRational(i32, i32),
    Compression(Compression),
    Strips(Vec<Strip>),
    NewSubfileType(NewSubfileType),
    PhotometricInterpretation(PhotometricInterpretation),
    PlanarConfiguration(PlanarConfiguration),
    Illumination(Illumination),
    CFALayout(CFALayout),
    Orientation(Orientation),
    PreviewColorSpace(PreviewColorSpace),
    Offset(IfdEntryType, u32, usize),
}

impl IfdEntryTag {
    fn parse(&self, reader: &mut BufferReader, entry_type: IfdEntryType, count: u32, offset: usize) -> IfdEntryValue {
        reader.skip_to(offset);
        match *self {
            IfdEntryTag::NewSubfileType => IfdEntryValue::NewSubfileType(NewSubfileType::from_u32(reader.read_u32())),
            IfdEntryTag::PhotometricInterpretation => IfdEntryValue::PhotometricInterpretation(PhotometricInterpretation::from_u16(reader.read_u16())),
            IfdEntryTag::Compression => IfdEntryValue::Compression(Compression::from_u16(reader.read_u16())),
            IfdEntryTag::PlanarConfiguration => IfdEntryValue::PlanarConfiguration(PlanarConfiguration::from_u16(reader.read_u16())),
            IfdEntryTag::CalibrationIlluminant1
                | IfdEntryTag::CalibrationIlluminant2 => IfdEntryValue::Illumination(Illumination::from_u16(reader.read_u16())),
            IfdEntryTag::CFALayout => IfdEntryValue::CFALayout(CFALayout::from_u16(reader.read_u16())),
            IfdEntryTag::EXIF
                | IfdEntryTag::SubIFDs
                | IfdEntryTag::TileOffsets
                | IfdEntryTag::StripOffsets => IfdEntryValue::Offset(entry_type, count, offset),
            IfdEntryTag::Orientation => IfdEntryValue::Orientation(Orientation::from_u16(reader.read_u16())),
            IfdEntryTag::PreviewColorSpace => IfdEntryValue::PreviewColorSpace(PreviewColorSpace::from_u32(reader.read_u32())),
            _ => {
                if count == 1 {
                    match entry_type {
                        IfdEntryType::Long => return IfdEntryValue::Int(reader.read_u32()),
                        IfdEntryType::Short => return IfdEntryValue::Short(reader.read_u16()),
                        IfdEntryType::Byte => return IfdEntryValue::Byte(reader.read_u8()),
                        IfdEntryType::Rational => return IfdEntryValue::Rational(reader.read_u32(), reader.read_u32()),
                        IfdEntryType::SRational => return IfdEntryValue::SRational(reader.read_i32(), reader.read_i32()),
                        _ => ()
                    }
                }

                if count == 2 {
                    match entry_type {
                        IfdEntryType::Short => return IfdEntryValue::ShortPair(reader.read_u16(), reader.read_u16()),
                        IfdEntryType::Rational => return IfdEntryValue::RationalPair(reader.read_u32(), reader.read_u32(), reader.read_u32(), reader.read_u32()),
                        _ => ()
                    }
                }

                if count == 3 {
                    match entry_type {
                        IfdEntryType::Long => return IfdEntryValue::IntTriple(reader.read_u32(), reader.read_u32(), reader.read_u32()),
                        IfdEntryType::Short => return IfdEntryValue::ShortTriple(reader.read_u16(), reader.read_u16(), reader.read_u16()),
                        IfdEntryType::Byte => return IfdEntryValue::ByteTriple(reader.read_u8(), reader.read_u8(), reader.read_u8()),
                        IfdEntryType::Rational => return IfdEntryValue::RationalTriple(reader.read_u32(), reader.read_u32(), reader.read_u32(), reader.read_u32(), reader.read_u32(), reader.read_u32()),
                        _ => ()
                    }
                }

                if count == 4 {
                    match entry_type {
                        IfdEntryType::Long => return IfdEntryValue::IntQuad(reader.read_u32(), reader.read_u32(), reader.read_u32(), reader.read_u32()),
                        IfdEntryType::Byte => return IfdEntryValue::ByteQuad(reader.read_u8(), reader.read_u8(), reader.read_u8(), reader.read_u8()),
                        IfdEntryType::Rational => return IfdEntryValue::RationalQuad(reader.read_u32(), reader.read_u32(), reader.read_u32(), reader.read_u32(), reader.read_u32(), reader.read_u32(), reader.read_u32(), reader.read_u32()),
                        _ => ()
                    } 
                }

                if entry_type == IfdEntryType::Ascii {
                    let mut chars = Vec::new();
                    for _i in 0..count {
                        let character = reader.read_u8();
                        if character == 0x00 {
                            break;
                        }
                        chars.push(character);
                    }
                    return IfdEntryValue::Ascii(String::from_utf8(chars).unwrap());
                }
                IfdEntryValue::Offset(entry_type, count, offset)
            },
        }
    }

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
            33434 => IfdEntryTag::ExposureTime,
            34665 => IfdEntryTag::EXIF,
            34850 => IfdEntryTag::ExposureProgram,
            34855 => IfdEntryTag::ISO,
            34864 => IfdEntryTag::SensitivityType,
            34866 => IfdEntryTag::RecommendedExposureIndex,
            36864 => IfdEntryTag::ExifVersion,
            36867 => IfdEntryTag::DateTimeOriginal,
            36868 => IfdEntryTag::CreateDate,
            37377 => IfdEntryTag::ShutterSpeedValue,
            37380 => IfdEntryTag::ExposureCompensation,
            37383 => IfdEntryTag::MeteringMode,
            37385 => IfdEntryTag::Flash,
            37393 => IfdEntryTag::ImageNumber,
            37521 => IfdEntryTag::SubSecTimeOriginal,
            37522 => IfdEntryTag::SubSecTimeDigitized,
            40961 => IfdEntryTag::ColorSpace,
            41486 => IfdEntryTag::FocalPlaneXResolution,
            41487 => IfdEntryTag::FocalPlaneYResolution,
            41488 => IfdEntryTag::FocalPlaneResolutionUnit,
            41985 => IfdEntryTag::CustomRendered,
            41986 => IfdEntryTag::ExposureMode,
            41987 => IfdEntryTag::WhiteBalance,
            41990 => IfdEntryTag::SceneCaptureType,
            42033 => IfdEntryTag::SerialNumber,
            42037 => IfdEntryTag::LensSerialNumber,
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
    pub ifds: Vec<HashMap<IfdEntryTag, IfdEntryValue>>,
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

fn parse_ifd_list(reader: &mut BufferReader) -> Vec<HashMap<IfdEntryTag, IfdEntryValue>> {
    let mut ifds = Vec::new();

    let mut next_ifd = reader.read_u32() as usize;
    while next_ifd != 0 {
        reader.skip_to(next_ifd);
        ifds.push(parse_ifd(reader));
        next_ifd = reader.read_u32() as usize;
    }

    let mut new_ifds = Vec::new();
    for i in 0..ifds.len() {
        match ifds[i].get(&IfdEntryTag::SubIFDs) {
            Some(IfdEntryValue::Offset(_entry_type, count, offset)) => {
                for individual_offset in 0..*count {
                    reader.skip_to(offset + (individual_offset as usize * 4));
                    let this_offset = reader.read_u32() as usize;
                    reader.skip_to(this_offset);
                    new_ifds.push(parse_ifd(reader));
                }
            },
            _ => ()
        }

        match ifds[i].get(&IfdEntryTag::EXIF) {
            Some(IfdEntryValue::Offset(_entry_type, count, offset)) => {
                for individual_offset in 0..*count {
                    reader.skip_to(offset + (individual_offset as usize * 4));
                    let this_offset = reader.read_u32() as usize;
                    reader.skip_to(this_offset);
                    new_ifds.push(parse_ifd(reader));
                }
            },
            _ => ()
        }
    }

    ifds.extend(new_ifds);
    ifds
}

fn parse_ifd(reader: &mut BufferReader) -> HashMap<IfdEntryTag, IfdEntryValue> {
    let ifd_entry_count = reader.read_u16();
    let mut ifd_entries = HashMap::new();
    for _n in 0..ifd_entry_count {
        let (tag, value) = parse_ifd_entry(reader);
        ifd_entries.insert(tag, value);
    }

    ifd_entries
}

fn parse_ifd_entry(reader: &mut BufferReader) -> (IfdEntryTag, IfdEntryValue) {
    let tag = IfdEntryTag::from_u16(reader.read_u16());
    let entry_type = IfdEntryType::from_u16(reader.read_u16());
    let count = reader.read_u32();
    let offset = if is_offset(entry_type, count) {
        reader.read_u32() as usize
    } else { 
        let val = reader.offset();
        reader.read_u32();
        val
    };

    let prev_offset = reader.offset();
    let ret_val = (tag, tag.parse(reader, entry_type, count, offset));
    reader.skip_to(prev_offset);
    ret_val
}

#[derive(Debug)]
pub enum DngParseError {
    NotADng,
}

pub fn parse_dng(buffer: &[u8], _length: u32) -> Result<Dng, DngParseError> {
    let byte_order: ByteOrder = if buffer[0] == 0x49 && buffer[1] == 0x49 { ByteOrder::LittleEndian }
        else if buffer[0] == 0x4D && buffer[1] == 0x4D { ByteOrder::BigEndian }
        else { return Err(DngParseError::NotADng) };

    let mut reader = BufferReader::new(buffer, byte_order);
    reader.read_u16(); // This short is the already parsed byte order!

    if reader.read_u16() != 42 {
        return Err(DngParseError::NotADng)
    }

    Ok(Dng {
        buffer: buffer,
        ifds: parse_ifd_list(&mut reader),
        byte_order: byte_order,
    })
}