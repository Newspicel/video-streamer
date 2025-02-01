mod stream;
mod video_player_components;

use leptos::prelude::*;
use leptos::IntoView;
use video_player_components::VideoPlayerControll;
use web_sys::HtmlVideoElement;


#[component]
pub fn VideoPlayer(video_id: String) -> impl IntoView {
    let video_ref = NodeRef::new();
    let container_ref = NodeRef::new();

    let load_video = move |video: HtmlVideoElement| {
        leptos::logging::log!("Load Video: {}", video_id);
        video.set_src("http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4");
    };

    video_ref.on_load(load_video);

    view! {
        <div node_ref=container_ref class="w-screen h-screen flex item-center justify-center overflow-hidden object-contain select-none">
            <video node_ref=video_ref controls=false class="w-screen object-contain"/>

            <VideoPlayerControll video_ref=video_ref container_ref=container_ref/>
        </div>
    }
}
