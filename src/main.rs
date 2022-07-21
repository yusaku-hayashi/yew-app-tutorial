use reqwasm::http::Request;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[derive(Clone, Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
    on_click: Callback<Video>,
    selected_vid: Option<Video>,
}

#[function_component(VideosList)]
fn videos_list(
    VideosListProps {
        videos,
        on_click,
        selected_vid,
    }: &VideosListProps,
) -> Html {
    html! {
        <div class={"videos_list"}>
        {
            videos
                .iter()
                .map(|video| {
                    let on_video_select = {
                        let on_click = on_click.clone();
                        let video = video.clone();
                        Callback::from(move |_| on_click.emit(video.clone()))
                    };

                    html! {
                        <button
                            onclick={on_video_select}
                            class={
                                match selected_vid {
                                    Some(selected_vid) if selected_vid == video => "selected",
                                    _ => ""
                                }
                            }
                        >
                            {format!("{}: {}", video.speaker, video.title)}
                        </button>
                    }
                })
                .collect::<Html>()
        }
        </div>
    }
}

#[derive(Clone, Properties, PartialEq)]
struct VideosDetailsProps {
    video: Video,
}

#[function_component(VideoDetails)]
fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div class={"video_details"}>
            <h3>{ video.title.clone() }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    let videos = use_state(|| vec![]);

    {
        let videos = videos.clone();
        use_effect_with_deps(
            move |_| {
                let videos = videos.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    videos.set(fetched_videos);
                });
                || ()
            },
            (),
        );
    }

    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };

    let details = selected_video.as_ref().map(|video| {
        html! {
            <VideoDetails video={video.clone()} />
        }
    });

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{ "Videos to watch" }</h3>
                <VideosList
                    videos={(*videos).clone()}
                    on_click={on_video_select.clone()}
                    selected_vid={
                        let selected_video = selected_video.clone();
                        (*selected_video).clone()
                    }
                />
            </div>
            { for details }
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
