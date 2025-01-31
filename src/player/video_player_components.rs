use leptos::{
    ev::{click, keydown, pause, play, timeupdate},
    html::{Div, Video},
    prelude::*,
};
use leptos_use::{use_document, use_event_listener};
use web_sys::{DomRect, Event, HtmlDivElement, HtmlVideoElement};

#[component]
pub fn VideoPlayerControll(
    container_ref: NodeRef<Div>,
    video_ref: NodeRef<Video>,
) -> impl IntoView {
    view! {
      <div class="absolute bottom-0 left-0 w-full p-4 bg-gradient-to-t from-black to-transparent">
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

                    {/* Title and Episode Info */}
                    <div class="text-sm text-center">
                        <p class="font-bold text-neutral-200">"The Night Agent"</p>
                        <p class="text-gray-400">"Flg. 1 Anrufverfolgung"</p>
                    </div>

                    {/* Right Controls */}
                    <div class="flex items-center gap-4">
                        <VideoPlayerControllInfo video_ref=video_ref/>
                        <VideoPlayerControllSubtitle video_ref=video_ref/>
                        <VideoPlayerControllSpeed video_ref=video_ref/>
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

     _ = use_event_listener(progress_bar_ref, click, move |event| {
        let rect: DomRect = event_target::<HtmlDivElement>(&event).get_bounding_client_rect();
        let click_x = event.client_x() as f64 - rect.left();
        let width = rect.width();
        seek_video(click_x, width);
    });

    view! {
      <div
        node_ref=progress_bar_ref
        class="relative w-full h-1 bg-gray-600 cursor-pointer"
      >
          <div
            class="absolute top-0 left-0 h-full bg-indigo-700"
            style={move || format!("width: {}%", video_percent())}
          />
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

    view! {
      <IconButton on_click=video_play>
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
        leptos::logging::log!("Set time to {}", new_time);
        video.set_current_time(new_time);
    };

    _ = use_event_listener(use_document(), keydown, move |event| {
        if event.key() == "ArrowLeft" {
            action();
        }
    });

    view! {
      <IconButton on_click=action>
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
        leptos::logging::log!("Set time to {}", new_time);
        video.set_current_time(new_time);
    };

    _ = use_event_listener(use_document(), keydown, move |event| {
        if event.key() == "ArrowRight" {
            action();
        }
    });

    view! {
      <IconButton on_click=action>
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="size-6">
            <path d="M5.055 7.06C3.805 6.347 2.25 7.25 2.25 8.69v8.122c0 1.44 1.555 2.343 2.805 1.628L12 14.471v2.34c0 1.44 1.555 2.343 2.805 1.628l7.108-4.061c1.26-.72 1.26-2.536 0-3.256l-7.108-4.061C13.555 6.346 12 7.249 12 8.689v2.34L5.055 7.061Z" />
        </svg>
      </IconButton>
    }
}

#[component]
fn VideoPlayerControllAudio(video_ref: NodeRef<Video>) -> impl IntoView {
    let (is_mute, set_mute) = signal(false);

    let mute = move || {
        let video: HtmlVideoElement = video_ref.get_untracked().unwrap();
        video.set_muted(!video.muted());
        set_mute(video.muted());
    };

    _ = use_event_listener(use_document(), keydown, move |event| {
        if event.key() == "M" || event.key() == "m" {
            mute();
        }
    });

    view! {
      <IconButton on_click=mute>
          <Show when=move || is_mute() >
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="size-6">
              <path d="M13.5 4.06c0-1.336-1.616-2.005-2.56-1.06l-4.5 4.5H4.508c-1.141 0-2.318.664-2.66 1.905A9.76 9.76 0 0 0 1.5 12c0 .898.121 1.768.35 2.595.341 1.24 1.518 1.905 2.659 1.905h1.93l4.5 4.5c.945.945 2.561.276 2.561-1.06V4.06ZM17.78 9.22a.75.75 0 1 0-1.06 1.06L18.44 12l-1.72 1.72a.75.75 0 1 0 1.06 1.06l1.72-1.72 1.72 1.72a.75.75 0 1 0 1.06-1.06L20.56 12l1.72-1.72a.75.75 0 1 0-1.06-1.06l-1.72 1.72-1.72-1.72Z" />
            </svg>
          </Show>

          <Show when=move || !is_mute() >
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="size-6">
              <path d="M13.5 4.06c0-1.336-1.616-2.005-2.56-1.06l-4.5 4.5H4.508c-1.141 0-2.318.664-2.66 1.905A9.76 9.76 0 0 0 1.5 12c0 .898.121 1.768.35 2.595.341 1.24 1.518 1.905 2.659 1.905h1.93l4.5 4.5c.945.945 2.561.276 2.561-1.06V4.06ZM18.584 5.106a.75.75 0 0 1 1.06 0c3.808 3.807 3.808 9.98 0 13.788a.75.75 0 0 1-1.06-1.06 8.25 8.25 0 0 0 0-11.668.75.75 0 0 1 0-1.06Z" />
              <path d="M15.932 7.757a.75.75 0 0 1 1.061 0 6 6 0 0 1 0 8.486.75.75 0 0 1-1.06-1.061 4.5 4.5 0 0 0 0-6.364.75.75 0 0 1 0-1.06Z" />
            </svg>
          </Show>
      </IconButton>
    }
}

#[component]
fn VideoPlayerControllInfo(video_ref: NodeRef<Video>) -> impl IntoView {
    let action = move || {
        let _video: HtmlVideoElement = video_ref.get_untracked().unwrap();
    };

    view! {
      <IconButton on_click=action>
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
      <IconButton on_click=action>
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="size-6">
            <path fill-rule="evenodd" d="M4.848 2.771A49.144 49.144 0 0 1 12 2.25c2.43 0 4.817.178 7.152.52 1.978.292 3.348 2.024 3.348 3.97v6.02c0 1.946-1.37 3.678-3.348 3.97a48.901 48.901 0 0 1-3.476.383.39.39 0 0 0-.297.17l-2.755 4.133a.75.75 0 0 1-1.248 0l-2.755-4.133a.39.39 0 0 0-.297-.17 48.9 48.9 0 0 1-3.476-.384c-1.978-.29-3.348-2.024-3.348-3.97V6.741c0-1.946 1.37-3.68 3.348-3.97ZM6.75 8.25a.75.75 0 0 1 .75-.75h9a.75.75 0 0 1 0 1.5h-9a.75.75 0 0 1-.75-.75Zm.75 2.25a.75.75 0 0 0 0 1.5H12a.75.75 0 0 0 0-1.5H7.5Z" clip-rule="evenodd" />
          </svg>
      </IconButton>
    }
}

#[component]
fn VideoPlayerControllSpeed(video_ref: NodeRef<Video>) -> impl IntoView {
    let action = move || {
        let _video: HtmlVideoElement = video_ref.get_untracked().unwrap();
    };

    view! {
      <IconButton on_click=action>
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="size-6">
            <path fill-rule="evenodd" d="M12 2.25c-5.385 0-9.75 4.365-9.75 9.75s4.365 9.75 9.75 9.75 9.75-4.365 9.75-9.75S17.385 2.25 12 2.25ZM12.75 6a.75.75 0 0 0-1.5 0v6c0 .414.336.75.75.75h4.5a.75.75 0 0 0 0-1.5h-3.75V6Z" clip-rule="evenodd" />
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

    view! {
      <IconButton on_click=toggle_fullscreen>
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
pub fn IconButton<F>(children: ChildrenFn, on_click: F) -> impl IntoView
where
    F: Fn() + 'static,
{
    view! {
        <button class="p-2 text-neutral-200 rounded-full hover:bg-neutral-800 focus:outline-none" on:click=move |_| on_click()>
            {children()}
        </button>
    }
}
