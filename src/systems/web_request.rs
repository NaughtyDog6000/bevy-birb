use bevy::prelude::*;
use bevy_mod_reqwest::{bevy_eventlistener::callbacks::ListenerInput, reqwest::Url, *};
use serde::Deserialize;

// pub fn web_request(mut client: BevyReqwest) {
//     let url = "https://httpbin.org/ip";
//     let req = client.get(url).build().unwrap();
//     // will run the callback, and remove the created entity after callback
//     client.send(
//         req,
//         On::run(|req: Listener<ReqResponse>| {
//             let res = req.as_str();
//             info!("return data: {res:?}");
//         }),
//     );
// }

/// implement the [Event] to able to use this from an eventreader, dont forget to add the event to the app
#[derive(Deserialize, Debug, Event)]
pub struct Bored {
    pub activity: String,
    pub r#type: String,
    pub participants: f32,
    pub accessibility: f32,
    pub price: f32,
    pub link: String,
}

/// this is one way to automatically turn the responses into events, which is
/// the prefered way, since it allows parallelism according to
/// [example](https://github.com/aevyrie/bevy_eventlistener/blob/main/examples/event_listeners.rs)
impl From<ListenerInput<ReqResponse>> for Bored {
    fn from(value: ListenerInput<ReqResponse>) -> Self {
        let s = value.deserialize_json().unwrap();
        s
    }
}
/// builds the http requests
pub fn send_requests(mut bevyreq: BevyReqwest) {
    info!("Sending activity request");
    let url: Url = "https://www.boredapi.com/api/activity".try_into().unwrap();
    let reqwest = bevyreq.client().get(url).build().unwrap();
    bevyreq.send(
        // the http request
        reqwest,
        // what to do when the api call is complete
        On::send_event::<Bored>(),
    );
}

/// here you can do anything with the data from the events
pub fn handle_events(mut events: EventReader<Bored>) {
    for ev in events.read() {
        info!("got respoonse: {ev:?}");
    }
}
