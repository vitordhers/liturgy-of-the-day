use agnus_daily_error::AgnusDailyError;
use html::{Audio, Div, Video};
use invidious::Format;
use leptos::*;
use logging::log;
use utils::{is_webkit, unix_to_hours_secs_mins};

use super::config::VolumeConfigCtx;

#[derive(Copy, Clone, PartialEq)]
pub enum PlaybackState {
    Playing,
    Paused,
    Loading,
    Initial,
}

#[derive(Clone, Copy)]
pub struct PlayerState {
    format: RwSignal<Option<Format>>,
    pub playback_state: RwSignal<PlaybackState>,
    video_ready: RwSignal<bool>,
    audio_ready: RwSignal<bool>,
    pub volume: RwSignal<f64>,
    pub current_time: RwSignal<f64>,
    pub duration: RwSignal<f64>,
    pub current_time_str: RwSignal<String>,
    pub duration_str: RwSignal<String>,
    pub audio_player_ref: NodeRef<Audio>,
    pub video_container_ref: NodeRef<Div>,
    pub video_controls_ref: NodeRef<Div>,
    pub video_player_ref: NodeRef<Video>,
}

impl PlayerState {
    pub fn init() -> Self {
        let volume_config_ctx = expect_context::<VolumeConfigCtx>();

        let format = create_rw_signal(None);
        let playback_state = create_rw_signal(PlaybackState::Initial);
        let video_ready = create_rw_signal(false);
        let audio_ready = create_rw_signal(false);
        let volume = create_rw_signal(volume_config_ctx.config.get_untracked());
        let current_time_str = create_rw_signal(String::from("0:00"));
        let duration_str = create_rw_signal(String::from("0:00"));
        let current_time = create_rw_signal(0f64);
        let duration = create_rw_signal(0f64);
        let audio_player_ref = create_node_ref();
        let video_container_ref = create_node_ref();
        let video_player_ref = create_node_ref();
        let video_controls_ref = create_node_ref();

        Self {
            format,
            playback_state,
            video_ready,
            audio_ready,
            volume,
            current_time,
            current_time_str,
            duration,
            duration_str,
            audio_player_ref,
            video_container_ref,
            video_controls_ref,
            video_player_ref,
        }
    }

    pub fn ready(&self) -> Result<bool, AgnusDailyError> {
        if cfg!(feature = "ssr") {
            return Ok(false);
        }

        let video = self.audio_player_ref.get().expect("video element to exist");
        let audio = self.audio_player_ref.get().expect("audio element to exist");

        let ready = if is_webkit() {
            if self
                .format
                .get()
                .map_or(false, |format| format.is_audio_only())
            {
                audio.ready_state() >= 3
            } else if self.format.get().map_or(false, |format| format.is_legacy()) {
                video.ready_state() >= 3
            } else {
                video.ready_state() >= 3 && audio.ready_state() >= 3
            }
        } else {
            if self
                .format
                .get()
                .map_or(false, |format| format.is_audio_only())
            {
                self.audio_ready.get() && audio.ready_state() >= 3
            } else if self.format.get().map_or(false, |format| format.is_legacy()) {
                self.video_ready.get() && video.ready_state() >= 3
            } else {
                self.video_ready.get()
                    && self.audio_ready.get()
                    && video.ready_state() >= 3
                    && audio.ready_state() >= 3
            }
        };

        Ok(ready)
    }

    pub fn play(&self) -> Result<(), AgnusDailyError> {
        let video = self.audio_player_ref.get().expect("video element to exist");
        let audio = self.audio_player_ref.get().expect("audio element to exist");

        if self.ready()? {
            audio.set_volume(self.volume.get());
            let video_play = video.play();
            video.set_current_time(audio.current_time());
            let audio_play = audio.play();
            match (audio_play, video_play) {
                (Ok(_), Ok(_)) => {
                    self.playback_state.set(PlaybackState::Playing);
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub fn resume(&self) -> Result<(), AgnusDailyError> {
        let video = self.audio_player_ref.get().expect("video element to exist");
        let audio = self.audio_player_ref.get().expect("audio element to exist");

        if self.playback_state.get() == PlaybackState::Loading
            || self.playback_state.get() == PlaybackState::Paused
        {
            if is_webkit() {
                let video_play = video.play();
                let audio_play = audio.play();
                match (audio_play, video_play) {
                    (Ok(_), Ok(_)) => {
                        self.playback_state.set(PlaybackState::Playing);
                    }
                    _ => {}
                }
            } else {
                audio.set_volume(self.volume.get());
                let video_play = video.play();
                audio.set_current_time(video.current_time());
                let audio_play = audio.play();
                match (audio_play, video_play) {
                    (Ok(_), Ok(_)) => {
                        self.playback_state.set(PlaybackState::Playing);
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn pause(&self) -> Result<(), AgnusDailyError> {
        let video = self.audio_player_ref.get().expect("video element to exist");
        let audio = self.audio_player_ref.get().expect("audio element to exist");

        let video_pause = video.pause();
        let audio_pause = audio.pause();
        match (audio_pause, video_pause) {
            (Ok(_), Ok(_)) => {
                self.playback_state.set(PlaybackState::Paused);
            }
            _ => {}
        }

        Ok(())
    }

    pub fn toggle_playback(&self) -> Result<(), AgnusDailyError> {
        match self.playback_state.get() {
            PlaybackState::Playing => self.pause()?,
            PlaybackState::Paused => self.resume()?,
            PlaybackState::Loading => (),
            PlaybackState::Initial => self.play()?,
        }
        Ok(())
    }

    pub fn set_video_ready(&self, ready: bool) -> Result<(), AgnusDailyError> {
        self.video_ready.set(ready);
        if self.ready()? && self.playback_state.get() != PlaybackState::Initial {
            self.resume()?;
        }
        Ok(())
    }

    pub fn set_audio_ready(&self, ready: bool) -> Result<(), AgnusDailyError> {
        self.audio_ready.set(ready);
        if self.ready()? && self.playback_state.get() != PlaybackState::Initial {
            self.resume()?;
        }
        Ok(())
    }

    pub fn sync(&self) -> Result<(), AgnusDailyError> {
        if let Some(Some(format)) = self.format.try_get() {
            if !format.is_audio_only() && !format.is_legacy() {
                let video = self.audio_player_ref.get().expect("video element to exist");
                let audio = self.audio_player_ref.get().expect("audio element to exist");

                let video_time = video.current_time();
                let audio_time = audio.current_time();

                let initial_start = video_time < 3.0 || audio_time < 3.0;
                let out_of_sync =
                    video_time > audio_time + 0.125 || video_time + 0.125 < audio_time;
                if !initial_start && out_of_sync {
                    video.set_current_time(audio_time);
                }
            }
        }

        Ok(())
    }

    pub fn seek(&self, time: f64) -> Result<(), AgnusDailyError> {
        let video = self.audio_player_ref.get().expect("video element to exist");
        let audio = self.audio_player_ref.get().expect("audio element to exist");

        self.pause()?;
        self.set_video_ready(false)?;
        self.playback_state.set(PlaybackState::Loading);

        match is_webkit() {
            true => {
                video.set_current_time(time);
                audio.set_current_time(time);
            }
            false => {
                self.set_audio_ready(false)?;
                let fast_seek_video = video.fast_seek(time);
                let fast_seek_audio = audio.fast_seek(time);
                if fast_seek_audio.is_err() || fast_seek_video.is_err() {
                    video.set_current_time(time);
                    audio.set_current_time(time);
                }
            }
        }

        self.current_time.set(time);
        self.current_time_str.set(unix_to_hours_secs_mins(time));
        self.pause()?;
        self.play()?;
        Ok(())
    }

    pub fn update_time(&self) -> Result<(), AgnusDailyError> {
        let video = self.audio_player_ref.get().expect("video element to exist");
        let audio = self.audio_player_ref.get().expect("audio element to exist");

        let current_time = video.current_time();
        let total_time = video.duration();
        self.current_time.set(current_time);
        self.duration.set(total_time);
        self.current_time_str
            .set(unix_to_hours_secs_mins(current_time));
        self.duration_str.set(unix_to_hours_secs_mins(total_time));
        Ok(())
    }

    pub fn change_format(&self, format: Format) -> Result<(), AgnusDailyError> {
        let video = self.audio_player_ref.get().expect("video element to exist");
        let audio = self.audio_player_ref.get().expect("audio element to exist");

        let current_time = video.current_time();
        video.set_src(&format.video_url().unwrap_or_default());
        audio.set_src(&format.audio_url().unwrap_or_default());
        self.pause()?;
        self.set_video_ready(false)?;
        self.playback_state.set(PlaybackState::Loading);
        self.format.set(Some(format));
        video.set_current_time(current_time);
        audio.set_current_time(current_time);
        self.current_time.set(current_time);
        self.current_time_str
            .set(unix_to_hours_secs_mins(current_time));
        Ok(())
    }

    pub fn set_volume(&self, volume: f64) -> Result<(), AgnusDailyError> {
        let video = self.audio_player_ref.get().expect("video element to exist");
        let audio = self.audio_player_ref.get().expect("audio element to exist");

        video.set_volume(volume);
        audio.set_volume(volume);
        self.volume.set(volume);
        expect_context::<VolumeConfigCtx>()
            .update_config
            .set(volume);

        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct VideoTime {
    pub current: RwSignal<String>,
    pub total: RwSignal<String>,
    pub current_ms: RwSignal<f64>,
    pub total_ms: RwSignal<f64>,
}

#[derive(Clone, Copy)]
pub struct PlayerStyle {
    pub controls_visible: RwSignal<bool>,
    pub full_window: RwSignal<bool>,
    pub fullscreen: RwSignal<bool>,
}

impl PlayerStyle {
    pub fn init() -> Self {
        let controls_visible = create_rw_signal(false);
        let full_window = create_rw_signal(false);
        let fullscreen = create_rw_signal(false);

        Self {
            controls_visible,
            full_window,
            fullscreen,
        }
    }
}
