use crate::formats::items::{ColorInfo, Container, QualityLabel, Resolution};
use crate::formats::{AudioFormat, AudioQuality, VideoFormat};
use agnus_daily_error::AgnusDailyError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdaptiveFormat {
    pub init: String,
    pub index: String,
    pub bitrate: String,
    pub url: String,
    pub itag: String,
    pub r#type: String,
    pub clen: String,
    pub lmt: String,
    pub projection_type: Option<String>,
    pub fps: Option<u8>,
    pub audio_quality: Option<AudioQuality>,
    pub audio_sample_rate: Option<u32>,
    pub audio_channels: Option<u32>,
    pub container: Option<Container>,
    pub encoding: Option<String>,
    pub resolution: Option<Resolution>,
    pub quality_label: Option<QualityLabel>,
    pub color_info: Option<ColorInfo>,
}

impl TryFrom<AdaptiveFormat> for AudioFormat {
    type Error = AgnusDailyError;

    fn try_from(adaptive_format: AdaptiveFormat) -> Result<Self, Self::Error> {
        let audio_quality = adaptive_format
            .audio_quality
            .ok_or(Self::Error::format_parse())?;
        let audio_sample_rate = adaptive_format
            .audio_sample_rate
            .ok_or(Self::Error::format_parse())?;
        let audio_channels = adaptive_format
            .audio_channels
            .ok_or(Self::Error::format_parse())?;

        Ok(AudioFormat {
            init: adaptive_format.init,
            index: adaptive_format.index,
            bitrate: adaptive_format.bitrate,
            url: adaptive_format.url,
            itag: adaptive_format.itag,
            r#type: adaptive_format.r#type,
            clen: adaptive_format.clen,
            lmt: adaptive_format.lmt,
            projection_type: adaptive_format.projection_type,
            container: adaptive_format.container,
            encoding: adaptive_format.encoding,
            audio_quality,
            audio_sample_rate,
            audio_channels,
        })
    }
}

impl TryFrom<AdaptiveFormat> for VideoFormat {
    type Error = AgnusDailyError;

    fn try_from(adaptive_format: AdaptiveFormat) -> Result<Self, Self::Error> {
        let resolution = adaptive_format
            .resolution
            .ok_or(Self::Error::format_parse())?;
        let quality_label = adaptive_format
            .quality_label
            .ok_or(Self::Error::format_parse())?;
        let fps = adaptive_format.fps.ok_or(Self::Error::format_parse())?;

        Ok(VideoFormat {
            init: adaptive_format.init,
            index: adaptive_format.index,
            bitrate: adaptive_format.bitrate,
            url: adaptive_format.url,
            itag: adaptive_format.itag,
            r#type: adaptive_format.r#type,
            clen: adaptive_format.clen,
            lmt: adaptive_format.lmt,
            projection_type: adaptive_format.projection_type,
            container: adaptive_format.container,
            encoding: adaptive_format.encoding,
            fps,
            resolution,
            quality_label,
            color_info: adaptive_format.color_info,
        })
    }
}
