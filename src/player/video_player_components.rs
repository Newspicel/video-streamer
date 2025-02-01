use std::time::Duration;

use leptos::{
    ev::{click, keydown, mouseleave, mousemove, pause, play, progress, timeupdate},
    html::{Div, Video},
    prelude::*,
};
use leptos_use::{
    use_document, use_event_listener, use_timeout_fn, use_timestamp, UseTimeoutFnReturn,
};
use web_sys::{DomRect, Event, HtmlDivElement, HtmlVideoElement, ProgressEvent};

#[component]
pub fn VideoPlayerControll(
    container_ref: NodeRef<Div>,
    video_ref: NodeRef<Video>,
) -> impl IntoView {
    let (show_controls, set_show_controls) = signal(false);
    let last_mouse_move = StoredValue::new(use_timestamp().get_untracked());

    const SHOW_CONTROLS_TIMEOUT: f64 = 3000.0;

    let UseTimeoutFnReturn { start, .. } = use_timeout_fn(
        move |_| {
            let last_move = last_mouse_move.get_value();
            let current_time = use_timestamp().get_untracked();
            if (current_time - last_move) > SHOW_CONTROLS_TIMEOUT {
                set_show_controls(false);
            }
        },
        SHOW_CONTROLS_TIMEOUT,
    );

    _ = use_event_listener(container_ref, mousemove, move |_| {
        set_show_controls(true);
        last_mouse_move.set_value(use_timestamp().get_untracked());
        start(());
    });

    view! {
      <div class="absolute bottom-0 left-0 w-full p-4 bg-gradient-to-t from-black to-transparent
                  transition-opacity duration-300 ease-in-out"
           class:opacity-100=move || show_controls()
           class:opacity-0=move || !show_controls()
      >
          <div class="flex justify-between items-center mb-2">
              <VideoPlayerControllProgressBar video_ref=video_ref/>
          </div>
          <div class="flex justify-between items-center">
              <div class="flex items-center gap-4">
                  <VideoPlayerControllPlay video_ref=video_ref/>
                  <VideoPlayerControllBackward video_ref=video_ref/>
                  <VideoPlayerControllForward video_ref=video_ref/>
                  <VideoPlayerControllAudio video_ref=video_ref/>
              </div>

              {/* Title and Episode Info - Centered */}
              <div class="absolute left-1/2 bottom-4 transform -translate-x-1/2 text-sm text-center">
                  <p class="font-bold text-neutral-200">"The Night Agent"</p>
                  <p class="text-gray-400">"Flg. 1 Anrufverfolgung"</p>
              </div>

              {/* Right Controls */}
              <div class="flex items-center gap-4">
                  <VideoPlayerControllInfo video_ref=video_ref/>
                  //<VideoPlayerControllSubtitle video_ref=video_ref/>
                  <VideoPlayerControllOptions video_ref=video_ref/>
                  <VideoPlayerControllFullScreen container_ref=container_ref/>
              </div>
          </div>
      </div>
    }
}

#[component]
fn VideoPlayerControllProgressBar(video_ref: NodeRef<Video>) -> impl IntoView {
    let progress_bar_ref = NodeRef::new();
    let (video_percent, set_video_percent) = signal(0);
    let (buffered_percent, set_buffered_percent) = signal(0);
    let (hover_x, set_hover_x) = signal(0.0);
    let (hover_time, set_hover_time) = signal("00:00".to_string());
    let (show_preview, set_show_preview) = signal(false);

    let seek_video = move |click_x: f64, width: f64| {
        if let Some(video) = video_ref.get_untracked() {
            let duration = video.duration();
            let new_time = (click_x / width) * duration;
            video.set_current_time(new_time);
        }
    };

    _ = use_event_listener(video_ref, timeupdate, move |event: Event| {
        let video: HtmlVideoElement = event_target(&event);
        let duration = video.duration();
        let current_time = video.current_time();
        if duration > 0.0 {
            let percent = (current_time / duration) * 100.0;
            set_video_percent(percent as u32);
        } else {
            set_video_percent(0);
        }
    });

    _ = use_event_listener(video_ref, progress, move |event: ProgressEvent| {
        let video: HtmlVideoElement = event_target(&event);
        let duration = video.duration();
        if duration > 0.0 {
            let buffered = video.buffered();
            let mut max_buffered: f64 = 0.0;

            for i in 0..buffered.length() {
                let end = buffered.end(i).unwrap_or(0.0);
                if end > max_buffered {
                    max_buffered = end;
                }
            }

            let percent = (max_buffered / duration) * 100.0;
            set_buffered_percent((percent) as u32);
        }
    });

    _ = use_event_listener(progress_bar_ref, click, move |event| {
        let progress_bar: HtmlDivElement = progress_bar_ref
            .get_untracked()
            .expect("DOM element not found");
        let rect: DomRect = progress_bar.get_bounding_client_rect();
        let click_x = event.client_x() as f64 - rect.left();
        let width = rect.width();
        seek_video(click_x, width);
    });

    _ = use_event_listener(progress_bar_ref, mousemove, move |event| {
        let progress_bar: HtmlDivElement = progress_bar_ref
            .get_untracked()
            .expect("DOM element not found");
        let rect: DomRect = progress_bar.get_bounding_client_rect();
        let hover_x = (event.client_x() as f64 - rect.left()).clamp(0.0, rect.width());
        set_hover_x(hover_x);

        if let Some(video) = video_ref.get_untracked() {
            let duration = video.duration();
            let hover_time_sec = (hover_x / rect.width()) * duration;
            let minutes = (hover_time_sec / 60.0).floor() as u32;
            let seconds = (hover_time_sec % 60.0).floor() as u32;
            set_hover_time(format!("{:02}:{:02}", minutes, seconds));
        }
        set_show_preview(true);
    });

    _ = use_event_listener(progress_bar_ref, mouseleave, move |_| {
        set_show_preview(false);
    });

    view! {
        <div node_ref=progress_bar_ref class="relative w-full h-6 bg-transparent cursor-pointer">
            {/* Mouse Hover Preview (Timestamp & Image) */}
            <Show when=move || show_preview()>
                <div
                    class="absolute -top-20 left-0 transform -translate-x-1/2 bg-neutral-800 text-white text-xs px-2 py-1 rounded shadow-md"
                    style={move || format!("left: {}px;", hover_x())}
                >
                     <img
                        src={move || format!("https://example.com/previews/frame_{}.jpg", hover_time())}
                        alt="Preview"
                        class="w-24 h-14 mb-1 rounded"
                    />
                    {hover_time()}
                </div>
            </Show>

            {/* Clickable and Hoverable Area */}
            <div class="absolute top-0 left-0 w-full h-full bg-transparent"></div>

            {/* Progress Bar */}
            <div class="absolute top-1/2 left-0 w-full h-1 bg-neutral-600 transform -translate-y-1/2">
                <div
                    class="absolute top-0 left-0 h-full bg-neutral-500"
                    style={move || format!("width: {}%;", buffered_percent())}
                />

                <div
                    class="absolute top-0 left-0 h-full bg-indigo-700"
                    style={move || format!("width: {}%;", video_percent())}
                />
            </div>

            {/* Hover Position Indicator | */}
            <Show when=move || show_preview()>
                <div
                    class="absolute top-1/2 w-[2px] h-4 bg-white transform -translate-y-1/2"
                    style={move || format!("left: {}px;", hover_x())}
                />
            </Show>
        </div>
    }
}

#[component]
fn VideoPlayerControllPlay(video_ref: NodeRef<Video>) -> impl IntoView {
    let (is_playing, set_playing) = signal(false);

    let video_play = move || {
        let video: HtmlVideoElement = video_ref.get().expect("Failed to get video element");

        if video.paused() {
            _ = video.play().expect("Failed to play video");
            set_playing(true);
        } else {
            video.pause().expect("Failed to pause video");
            set_playing(false);
        }
    };

    let set_button_state = move |event| {
        set_playing(!event_target::<web_sys::HtmlVideoElement>(&event).paused());
    };

    _ = use_event_listener(video_ref, play, set_button_state);
    _ = use_event_listener(video_ref, pause, set_button_state);

    _ = use_event_listener(use_document(), keydown, move |event| {
        if event.key() == " " {
            video_play();
        }
    });

    _ = use_event_listener(video_ref, click, move |_| {
        video_play();
    });

    view! {
      <IconButton on:click= move |_| video_play()>
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="size-6">
            <path fill-rule="evenodd" d={move || {
              if is_playing() {
                "M6.75 5.25a.75.75 0 0 1 .75-.75H9a.75.75 0 0 1 .75.75v13.5a.75.75 0 0 1-.75.75H7.5a.75.75 0 0 1-.75-.75V5.25Zm7.5 0A.75.75 0 0 1 15 4.5h1.5a.75.75 0 0 1 .75.75v13.5a.75.75 0 0 1-.75.75H15a.75.75 0 0 1-.75-.75V5.25Z"
              } else {
                "M4.5 5.653c0-1.427 1.529-2.33 2.779-1.643l11.54 6.347c1.295.712 1.295 2.573 0 3.286L7.28 19.99c-1.25.687-2.779-.217-2.779-1.643V5.653Z"
              }
            }} clip-rule="evenodd" />
        </svg>
      </IconButton>
    }
}

#[component]
fn VideoPlayerControllBackward(video_ref: NodeRef<Video>) -> impl IntoView {
    let action = move || {
        let video: HtmlVideoElement = video_ref.get_untracked().unwrap();
        let new_time = (video.current_time() - 10.0).max(0.0);
        video.set_current_time(new_time);
    };

    _ = use_event_listener(use_document(), keydown, move |event| {
        if event.key() == "ArrowLeft" {
            action();
        }
    });

    view! {
      <IconButton on:click= move |_| action()>
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="size-6">
            <path d="M9.195 18.44c1.25.714 2.805-.189 2.805-1.629v-2.34l6.945 3.968c1.25.715 2.805-.188 2.805-1.628V8.69c0-1.44-1.555-2.343-2.805-1.628L12 11.029v-2.34c0-1.44-1.555-2.343-2.805-1.628l-7.108 4.061c-1.26.72-1.26 2.536 0 3.256l7.108 4.061Z" />
        </svg>
      </IconButton>
    }
}

#[component]
fn VideoPlayerControllForward(video_ref: NodeRef<Video>) -> impl IntoView {
    let action = move || {
        let video: HtmlVideoElement = video_ref.get_untracked().unwrap();
        let new_time = (video.current_time() + 10.0).min(video.duration());
        video.set_current_time(new_time);
    };

    _ = use_event_listener(use_document(), keydown, move |event| {
        if event.key() == "ArrowRight" {
            action();
        }
    });

    view! {
      <IconButton on:click= move |_| action()>
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="size-6">
            <path d="M5.055 7.06C3.805 6.347 2.25 7.25 2.25 8.69v8.122c0 1.44 1.555 2.343 2.805 1.628L12 14.471v2.34c0 1.44 1.555 2.343 2.805 1.628l7.108-4.061c1.26-.72 1.26-2.536 0-3.256l-7.108-4.061C13.555 6.346 12 7.249 12 8.689v2.34L5.055 7.061Z" />
        </svg>
      </IconButton>
    }
}

#[component]
fn VideoPlayerControllAudio(video_ref: NodeRef<Video>) -> impl IntoView {
    let (is_mute, set_mute) = signal(false);
    let (volume, set_volume) = signal(100); // Volume in percentage

    let mute = move || {
        let video: HtmlVideoElement = video_ref.get_untracked().unwrap();
        video.set_muted(!video.muted());
        set_mute(video.muted());
    };

    let change_volume = move |new_volume: f64| {
        let video: HtmlVideoElement = video_ref.get_untracked().unwrap();
        let normalized_volume = new_volume.clamp(0.0, 1.0);
        if volume() != normalized_volume as u32 {
            video.set_volume(normalized_volume);
            set_volume((normalized_volume * 100.0) as u32);
            if video.muted() && normalized_volume > 0.0 {
                video.set_muted(false);
                set_mute(false);
            }
        }
    };

    _ = use_event_listener(use_document(), keydown, move |event| {
        if event.key() == "M" || event.key() == "m" {
            mute();
        }
    });

    view! {
        <div class="flex group">
            {/* Mute Button */}
            <IconButton on:click= move |_| mute()>
                <Show when=move || is_mute()>
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="size-6">
                        <path d="M13.5 4.06c0-1.336-1.616-2.005-2.56-1.06l-4.5 4.5H4.508c-1.141 0-2.318.664-2.66 1.905A9.76 9.76 0 0 0 1.5 12c0 .898.121 1.768.35 2.595.341 1.24 1.518 1.905 2.659 1.905h1.93l4.5 4.5c.945.945 2.561.276 2.561-1.06V4.06ZM17.78 9.22a.75.75 0 1 0-1.06 1.06L18.44 12l-1.72 1.72a.75.75 0 1 0 1.06 1.06l1.72-1.72 1.72 1.72a.75.75 0 1 0 1.06-1.06L20.56 12l1.72-1.72a.75.75 0 1 0-1.06-1.06l-1.72 1.72-1.72-1.72Z" />
                    </svg>
                </Show>

                <Show when=move || !is_mute()>
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="size-6">
                        <path d="M13.5 4.06c0-1.336-1.616-2.005-2.56-1.06l-4.5 4.5H4.508c-1.141 0-2.318.664-2.66 1.905A9.76 9.76 0 0 0 1.5 12c0 .898.121 1.768.35 2.595.341 1.24 1.518 1.905 2.659 1.905h1.93l4.5 4.5c.945.945 2.561.276 2.561-1.06V4.06ZM18.584 5.106a.75.75 0 0 1 1.06 0c3.808 3.807 3.808 9.98 0 13.788a.75.75 0 0 1-1.06-1.06 8.25 8.25 0 0 0 0-11.668.75.75 0 0 1 0-1.06Z" />
                        <path d="M15.932 7.757a.75.75 0 0 1 1.061 0 6 6 0 0 1 0 8.486.75.75 0 0 1-1.06-1.061 4.5 4.5 0 0 0 0-6.364.75.75 0 0 1 0-1.06Z" />
                    </svg>
                </Show>
            </IconButton>

            {/* Volume Control Slider  */ }
            <div class="p-2 w-24 h-2">
                    <input
                        type="range"
                        min="0"
                        max="100"
                        value=move || volume().to_string()
                        class="w-24 h-2 slider-neutral-800 rounded-lg cursor-pointer accent-indigo-700 opacity-0 transition-opacity duration-200 group-hover:opacity-100 hover:opacity-100"
                        on:input=move |event| {
                            let new_volume = event_target::<web_sys::HtmlInputElement>(&event)
                                .value()
                                .parse::<f64>()
                                .unwrap_or(100.0) / 100.0;
                            change_volume(new_volume);
                        }
                    />
            </div>
        </div>
    }
}

#[component]
fn VideoPlayerControllInfo(video_ref: NodeRef<Video>) -> impl IntoView {
    let action = move || {
        let _video: HtmlVideoElement = video_ref.get_untracked().unwrap();
    };

    view! {
      <IconButton on:click= move |_| action()>
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="size-6">
            <path d="M5.566 4.657A4.505 4.505 0 0 1 6.75 4.5h10.5c.41 0 .806.055 1.183.157A3 3 0 0 0 15.75 3h-7.5a3 3 0 0 0-2.684 1.657ZM2.25 12a3 3 0 0 1 3-3h13.5a3 3 0 0 1 3 3v6a3 3 0 0 1-3 3H5.25a3 3 0 0 1-3-3v-6ZM5.25 7.5c-.41 0-.806.055-1.184.157A3 3 0 0 1 6.75 6h10.5a3 3 0 0 1 2.683 1.657A4.505 4.505 0 0 0 18.75 7.5H5.25Z" />
          </svg>
      </IconButton>
    }
}

#[component]
fn VideoPlayerControllSubtitle(video_ref: NodeRef<Video>) -> impl IntoView {
    let action = move || {
        let _video: HtmlVideoElement = video_ref.get_untracked().unwrap();
    };

    view! {
      <IconButton on:click= move |_| action()>
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="size-6">
            <path fill-rule="evenodd" d="M4.848 2.771A49.144 49.144 0 0 1 12 2.25c2.43 0 4.817.178 7.152.52 1.978.292 3.348 2.024 3.348 3.97v6.02c0 1.946-1.37 3.678-3.348 3.97a48.901 48.901 0 0 1-3.476.383.39.39 0 0 0-.297.17l-2.755 4.133a.75.75 0 0 1-1.248 0l-2.755-4.133a.39.39 0 0 0-.297-.17 48.9 48.9 0 0 1-3.476-.384c-1.978-.29-3.348-2.024-3.348-3.97V6.741c0-1.946 1.37-3.68 3.348-3.97ZM6.75 8.25a.75.75 0 0 1 .75-.75h9a.75.75 0 0 1 0 1.5h-9a.75.75 0 0 1-.75-.75Zm.75 2.25a.75.75 0 0 0 0 1.5H12a.75.75 0 0 0 0-1.5H7.5Z" clip-rule="evenodd" />
          </svg>
      </IconButton>
    }
}

#[component]
fn VideoPlayerControllOptions(video_ref: NodeRef<Video>) -> impl IntoView {
    let (is_show_menu, set_show_menu) = signal(false);
    let (settings_page, set_settings_page) = signal("main".to_string());

    let open_menu = move || {
        set_settings_page("main".to_string());
        set_show_menu(true);
    };

    let close_menu = move || {
        set_show_menu(false);
    };

    let go_to_page = move |page: &str| set_settings_page(page.to_string());
    let go_back = move || set_settings_page("main".to_string());

    view! {
        <Show when=move || is_show_menu()>
            <div class="absolute bottom-24 right-4 w-60 bg-neutral-800/90 rounded-lg shadow-lg p-4 text-neutral-200">
                <Show when=move || settings_page() == "main">
                    <div>
                        <div class="flex items-center justify-between mb-4">
                            <p class="text-sm font-medium">Untertitel (1)</p>
                            <button class="text-blue-500 text-sm" on:click=move |_| go_to_page("subtitles")>Aus ></button>
                        </div>
                        <div class="flex items-center justify-between mb-4">
                            <p class="text-sm font-medium">Wiedergabegeschwindigkeit</p>
                            <button class="text-blue-500 text-sm" on:click=move |_| go_to_page("playback_speed")>Benutzerdefiniert (1) ></button>
                        </div>
                        <div class="flex items-center justify-between mb-4">
                            <p class="text-sm font-medium">Ruhemodus-Timer</p>
                            <button class="text-blue-500 text-sm" on:click=move |_| go_to_page("sleep_timer")>Aus ></button>
                        </div>
                        <div class="flex items-center justify-between">
                            <p class="text-sm font-medium">Qualität</p>
                            <button class="text-blue-500 text-sm" on:click=move |_| go_to_page("quality")>Automatisch (1080p Premium HD) ></button>
                        </div>
                    </div>
                </Show>

                <Show when=move || settings_page() == "subtitles">
                    <div>
                        <div class="flex items-center mb-4">
                            <button class="text-blue-500 text-sm mr-4" on:click=move |_| go_back()>{"<"}</button>
                            <p class="text-sm font-medium">Untertitel</p>
                        </div>
                        <div class="flex flex-col gap-2">
                            <div class="flex items-center justify-between">
                                <p class="text-sm">Deutsch</p>
                                <input type="radio" name="subtitle" class="cursor-pointer" />
                            </div>
                            <div class="flex items-center justify-between">
                                <p class="text-sm">Englisch</p>
                                <input type="radio" name="subtitle" class="cursor-pointer" />
                            </div>
                            <div class="flex items-center justify-between">
                                <p class="text-sm">Aus</p>
                                <input type="radio" name="subtitle" class="cursor-pointer" checked=true />
                            </div>
                        </div>
                    </div>
                </Show>

                <Show when=move || settings_page() == "playback_speed">
                    <div>
                        <div class="flex items-center mb-4">
                            <button class="text-blue-500 text-sm mr-4" on:click=move |_| go_back()>{"<"}</button>
                            <p class="text-sm font-medium">Wiedergabegeschwindigkeit</p>
                        </div>
                        <div class="flex flex-col gap-2">
                            <div class="flex items-center justify-between">
                                <p class="text-sm">0.5x</p>
                                <input type="radio" name="speed" class="cursor-pointer" />
                            </div>
                            <div class="flex items-center justify-between">
                                <p class="text-sm">1.0x (Standard)</p>
                                <input type="radio" name="speed" class="cursor-pointer" checked=true />
                            </div>
                            <div class="flex items-center justify-between">
                                <p class="text-sm">1.5x</p>
                                <input type="radio" name="speed" class="cursor-pointer" />
                            </div>
                            <div class="flex items-center justify-between">
                                <p class="text-sm">2.0x</p>
                                <input type="radio" name="speed" class="cursor-pointer" />
                            </div>
                        </div>
                    </div>
                </Show>
            </div>
        </Show>

        <IconButton on:click=move |_| open_menu()>
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="size-6">
                <path fill-rule="evenodd" d="M11.078 2.25c-.917 0-1.699.663-1.85 1.567L9.05 4.889c-.02.12-.115.26-.297.348a7.493 7.493 0 0 0-.986.57c-.166.115-.334.126-.45.083L6.3 5.508a1.875 1.875 0 0 0-2.282.819l-.922 1.597a1.875 1.875 0 0 0 .432 2.385l.84.692c.095.078.17.229.154.43a7.598 7.598 0 0 0 0 1.139c.015.2-.059.352-.153.43l-.841.692a1.875 1.875 0 0 0-.432 2.385l.922 1.597a1.875 1.875 0 0 0 2.282.818l1.019-.382c.115-.043.283-.031.45.082.312.214.641.405.985.57.182.088.277.228.297.35l.178 1.071c.151.904.933 1.567 1.85 1.567h1.844c.916 0 1.699-.663 1.85-1.567l.178-1.072c.02-.12.114-.26.297-.349.344-.165.673-.356.985-.57.167-.114.335-.125.45-.082l1.02.382a1.875 1.875 0 0 0 2.28-.819l.923-1.597a1.875 1.875 0 0 0-.432-2.385l-.84-.692c-.095-.078-.17-.229-.154-.43a7.614 7.614 0 0 0 0-1.139c-.016-.2.059-.352.153-.43l.84-.692c.708-.582.891-1.59.433-2.385l-.922-1.597a1.875 1.875 0 0 0-2.282-.818l-1.02.382c-.114.043-.282.031-.449-.083a7.49 7.49 0 0 0-.985-.57c-.183-.087-.277-.227-.297-.348l-.179-1.072a1.875 1.875 0 0 0-1.85-1.567h-1.843ZM12 15.75a3.75 3.75 0 1 0 0-7.5 3.75 3.75 0 0 0 0 7.5Z" clip-rule="evenodd" />
            </svg>
        </IconButton>
    }
}
            
                
#[component]
fn VideoPlayerControllFullScreen(container_ref: NodeRef<Div>) -> impl IntoView {
    let (is_fullscreen, set_fullscreen) = signal(false);

    let toggle_fullscreen = move || {
        let container: HtmlDivElement = container_ref.get_untracked().unwrap();

        if is_fullscreen() {
            document().exit_fullscreen();
            set_fullscreen(false);
        } else {
            container
                .request_fullscreen()
                .expect("Failed to request fullscreen");
            set_fullscreen(true);
        }
    };

    _ = use_event_listener(use_document(), keydown, move |event| {
        if event.key() == "f" || event.key() == "F" {
            toggle_fullscreen();
        }
    });

    view! {
      <IconButton on:click= move |_| toggle_fullscreen()>
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="size-6">
              <path fill-rule="evenodd" d={move || {
                  if is_fullscreen() {
                      "M3.22 3.22a.75.75 0 0 1 1.06 0l3.97 3.97V4.5a.75.75 0 0 1 1.5 0V9a.75.75 0 0 1-.75.75H4.5a.75.75 0 0 1 0-1.5h2.69L3.22 4.28a.75.75 0 0 1 0-1.06Zm17.56 0a.75.75 0 0 1 0 1.06l-3.97 3.97h2.69a.75.75 0 0 1 0 1.5H15a.75.75 0 0 1-.75-.75V4.5a.75.75 0 0 1 1.5 0v2.69l3.97-3.97a.75.75 0 0 1 1.06 0ZM3.75 15a.75.75 0 0 1 .75-.75H9a.75.75 0 0 1 .75.75v4.5a.75.75 0 0 1-1.5 0v-2.69l-3.97 3.97a.75.75 0 0 1-1.06-1.06l3.97-3.97H4.5a.75.75 0 0 1-.75-.75Zm10.5 0a.75.75 0 0 1 .75-.75h4.5a.75.75 0 0 1 0 1.5h-2.69l3.97 3.97a.75.75 0 1 1-1.06 1.06l-3.97-3.97v2.69a.75.75 0 0 1-1.5 0V15Z"
                  } else {
                      "M15 3.75a.75.75 0 0 1 .75-.75h4.5a.75.75 0 0 1 .75.75v4.5a.75.75 0 0 1-1.5 0V5.56l-3.97 3.97a.75.75 0 1 1-1.06-1.06l3.97-3.97h-2.69a.75.75 0 0 1-.75-.75Zm-12 0A.75.75 0 0 1 3.75 3h4.5a.75.75 0 0 1 0 1.5H5.56l3.97 3.97a.75.75 0 0 1-1.06 1.06L4.5 5.56v2.69a.75.75 0 0 1-1.5 0v-4.5Zm11.47 11.78a.75.75 0 1 1 1.06-1.06l3.97 3.97v-2.69a.75.75 0 0 1 1.5 0v4.5a.75.75 0 0 1-.75.75h-4.5a.75.75 0 0 1 0-1.5h2.69l-3.97-3.97Zm-4.94-1.06a.75.75 0 0 1 0 1.06L5.56 19.5h2.69a.75.75 0 0 1 0 1.5h-4.5a.75.75 0 0 1-.75-.75v-4.5a.75.75 0 0 1 1.5 0v2.69l3.97-3.97a.75.75 0 0 1 1.06 0Z"
                  }
              }} clip-rule="evenodd" />
          </svg>
      </IconButton>
    }
}

#[component]
pub fn IconButton(children: ChildrenFn) -> impl IntoView{
    let button_ref = NodeRef::new();
    view! {
        <button node_ref=button_ref class="p-2 text-neutral-200 rounded-full hover:bg-neutral-800 focus:outline-none" on:click=move |_| {
          _ = button_ref.get_untracked().expect("Failed to get button element").blur();
        }> {children()} </button>
    }
}
