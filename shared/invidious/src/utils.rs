use agnus_daily_error::AgnusDailyError;

use crate::{
    AudioFormat, AudioQuality, Container, DashFormat, Format, Formats, LegacyFormat, QualityLabel,
    VideoFormat,
};

pub fn get_format(
    formats: &Formats,
    default_audio_quality: AudioQuality,
    default_video_quality: QualityLabel,
    is_webkit: bool,
) -> Result<Format, AgnusDailyError> {
    let audio_format = find_audio_format(&formats, default_audio_quality, is_webkit)?;
    let video_format = find_video_format(&formats, default_video_quality);

    let format = match video_format {
        Ok(video_format) => Format::Dash(DashFormat::new(video_format, audio_format)),
        Err(_) => match find_legacy_format(&formats) {
            Ok(legacy_format) => Format::Legacy(legacy_format),
            Err(_) => Format::Audio(audio_format),
        },
    };
    Ok(format)
}

pub fn find_video_format(
    formats: &Formats,
    default_video_quality: QualityLabel,
) -> Result<VideoFormat, AgnusDailyError> {
    // let default_video_quality = move || {
    //     expect_context::<PlayerConfigCtx>()
    //         .0
    //          .0
    //         .get()
    //         .default_video_quality
    // };

    let preferred_format = formats
        .video_formats
        .iter()
        .find(|x| x.quality_label == default_video_quality)
        .cloned();

    match preferred_format {
        Some(_) => preferred_format,
        None => formats.video_formats.first().cloned(),
    }
    .ok_or(AgnusDailyError::no_dash_video_format_available())
}

pub fn find_legacy_format(formats: &Formats) -> Result<LegacyFormat, AgnusDailyError> {
    formats
        .legacy_formats
        .last()
        .cloned()
        .ok_or(AgnusDailyError::no_legacy_format_available())
}

pub fn find_audio_format(
    formats: &Formats,
    default_audio_quality: AudioQuality,
    is_webkit: bool,
) -> Result<AudioFormat, AgnusDailyError> {
    let audio_formats = if is_webkit {
        filter_mp4_audio_formats(&formats.audio_formats)
    } else {
        formats.audio_formats.clone()
    };

    let preferred_format = audio_formats
        .iter()
        .find(|format| format.audio_quality == default_audio_quality)
        .cloned();

    match preferred_format {
        Some(_) => preferred_format,
        None => audio_formats.first().cloned(),
    }
    .ok_or(AgnusDailyError::no_audio_format_available())
}

pub fn filter_mp4_audio_formats(formats: &Vec<AudioFormat>) -> Vec<AudioFormat> {
    formats
        .iter()
        .filter_map(|format| {
            let a = format.r#type.contains("mp4");
            let b = format
                .clone()
                .container
                .map(|container| (container.eq(&Container::M4A)));

            (a && b.unwrap_or_default()).then(|| format.clone())
        })
        .collect::<Vec<AudioFormat>>()
}
