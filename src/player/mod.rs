mod stream;
mod video_player_components;

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::{component, IntoView};
use video_player_components::VideoPlayerControll;
use web_sys::{Event, HtmlDivElement, HtmlVideoElement, MediaSource, Url};
use stream::stream_video;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

#[component]
pub fn VideoPlayer(video_id: String) -> impl IntoView {
    let video_ref = NodeRef::new();
    let container_ref = NodeRef::new();

    let codec = "video/mp4; codecs=\"avc1.640028, mp4a.40.2\"";

    let load_video = move |_| {
        leptos::logging::log!("Load Video");
        let video: HtmlVideoElement = video_ref.get().unwrap();
        let video_id_clone = video_id.clone();

        let media_source = MediaSource::new().unwrap();
        let media_source_clone = media_source.clone();

        let on_source_open = Closure::wrap(Box::new(move |_event: Event| {
            let source_buffer = media_source_clone.add_source_buffer(codec).unwrap();

            let video_id_clone = video_id_clone.clone();
            spawn_local(async move {
                let mut buffer_pos = 0;
                loop {
                    let mut chunk = stream_video(video_id_clone.clone(), Some(buffer_pos))
                        .await
                        .unwrap();
                    leptos::logging::log!("{:?}", chunk[0]);
                    source_buffer
                        .append_buffer_with_u8_array(chunk.as_mut_slice())
                        .expect("Failed to append buffer");
                    buffer_pos += chunk.len() as u64;
                }
            });
        }) as Box<dyn FnMut(_)>);

        media_source.set_onsourceopen(Some(on_source_open.as_ref().unchecked_ref()));
        on_source_open.forget();

        let url = Url::create_object_url_with_source(&media_source).unwrap();
        video.set_src("https://videocdn.cdnpk.net/videos/5d68cbdc-50f0-4e3f-b62c-4747bbc79837/horizontal/previews/videvo_watermarked/large.mp4");
    };

    video_ref.on_load(load_video);

    view! {
        <div node_ref=container_ref class="w-screen h-screen flex item-center justify-center overflow-hidden object-contain" tabindex=-1>
            <video node_ref=video_ref controls=false class="w-screen object-contain"/>

            <VideoPlayerControll video_ref=video_ref container_ref=container_ref/>
        </div>
    }
}

