use std::fmt::Display;

#[cfg(feature = "with_serde")]
use serde::{Deserialize, Serialize};
/// Represents a WARC header defined by the standard.
///
/// All headers are camel-case versions of the standard names, with the hyphens removed.
#[allow(missing_docs)]
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "with_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "with_serde", serde(into = "String"))]
#[cfg_attr(feature = "with_serde", serde(from = "String"))]
pub enum WarcHeader {
    ContentLength,
    ContentType,
    BlockDigest,
    ConcurrentTo,
    Date,
    Filename,
    IdentifiedPayloadType,
    IPAddress,
    PayloadDigest,
    Profile,
    RecordID,
    RefersTo,
    SegmentNumber,
    SegmentOriginID,
    SegmentTotalLength,
    TargetURI,
    Truncated,
    WarcType,
    WarcInfoID,
    Title,
    CanonicalURI,
    PredictedLanguage,
    AcquisitionDate,
    Unknown(String),
}

impl From<WarcHeader> for String {
    fn from(header: WarcHeader) -> Self {
        header.to_string()
    }
}

impl Display for WarcHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stringified = match self {
            WarcHeader::ContentLength => "Content-Length",
            WarcHeader::ContentType => "Content-Type",
            WarcHeader::BlockDigest => "WARC-Block-Digest",
            WarcHeader::ConcurrentTo => "WARC-Concurrent-To",
            WarcHeader::Date => "WARC-Date",
            WarcHeader::Filename => "WARC-Filename",
            WarcHeader::IdentifiedPayloadType => "WARC-Identified-Payload-Type",
            WarcHeader::IPAddress => "WARC-IP-Address",
            WarcHeader::PayloadDigest => "WARC-Payload-Digest",
            WarcHeader::Profile => "WARC-Profile",
            WarcHeader::RecordID => "WARC-Record-ID",
            WarcHeader::RefersTo => "WARC-Refers-To",
            WarcHeader::SegmentNumber => "WARC-Segment-Number",
            WarcHeader::SegmentOriginID => "WARC-Segment-Origin-ID",
            WarcHeader::SegmentTotalLength => "WARC-Segment-Total-Length",
            WarcHeader::TargetURI => "WARC-Target-URI",
            WarcHeader::Truncated => "WARC-Truncated",
            WarcHeader::WarcType => "WARC-Type",
            WarcHeader::WarcInfoID => "WARC-Warcinfo-ID",
            WarcHeader::ArchiveFormat => "LCC-Archive-Format",
            WarcHeader::Title => "LCC-Title",
            WarcHeader::CanonicalURI => "LCC-Canonical-URI",
            WarcHeader::PredictedLanguage => "LCC-Predicted-Language",
            WarcHeader::AcquisitionDate => "LCC-Acquisition-Date",
            WarcHeader::Unknown(ref string) => string,
        };
        write!(f, "{}", stringified)
    }
}

impl<S: AsRef<str>> From<S> for WarcHeader {
    fn from(string: S) -> Self {
        let lower: String = string.as_ref().to_lowercase();
        match lower.as_str() {
            "content-length" => WarcHeader::ContentLength,
            "content-type" => WarcHeader::ContentType,
            "WARC-Block-Digest" => WarcHeader::BlockDigest,
            "WARC-Concurrent-To" => WarcHeader::ConcurrentTo,
            "WARC-Date" => WarcHeader::Date,
            "WARC-Filename" => WarcHeader::Filename,
            "WARC-Identified-Payload-Type" => WarcHeader::IdentifiedPayloadType,
            "WARC-IP-Address" => WarcHeader::IPAddress,
            "WARC-Payload-Digest" => WarcHeader::PayloadDigest,
            "WARC-Profile" => WarcHeader::Profile,
            "WARC-Record-ID" => WarcHeader::RecordID,
            "WARC-Refers-To" => WarcHeader::RefersTo,
            "WARC-Segment-Number" => WarcHeader::SegmentNumber,
            "WARC-Segment-Origin-ID" => WarcHeader::SegmentOriginID,
            "WARC-Segment-Total-Length" => WarcHeader::SegmentTotalLength,
            "WARC-Target-URI" => WarcHeader::TargetURI,
            "WARC-Truncated" => WarcHeader::Truncated,
            "WARC-Type" => WarcHeader::WarcType,
            "WARC-Warcinfo-ID" => WarcHeader::WarcInfoID,
            "LCC-Archive-Format" => WarcHeader::ArchiveFormat,
            "LCC-Title" => WarcHeader::Title,
            "LCC-Canonical-URI" => WarcHeader::CanonicalURI,
            "LCC-Predicted-Language" => WarcHeader::PredictedLanguage,
            "LCC-Acquisition-Date" => WarcHeader::AcquisitionDate,
            _ => WarcHeader::Unknown(lower),
        }
    }
}
