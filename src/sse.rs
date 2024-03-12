use eventsource_client::*;
use futures_util::StreamExt;

pub struct ServerEvents {
    client: Box<dyn Client>,
}

impl ServerEvents {
    pub fn new(url: &str) -> Result<Self> {
        let client = ClientBuilder::for_url(url).map(|cb| cb.build())?;

        Ok(Self {
            client: Box::new(client),
        })
    }

    pub async fn listen(
        &self,
        stream_event: impl Fn(String, Option<String>),
        stream_err: impl Fn(Error),
        keep_alive_friendly: bool,
    ) {
        self.stream(keep_alive_friendly)
            .for_each(|event| {
                match event {
                    Ok((event_type, maybe_data)) => stream_event(event_type, maybe_data),
                    Err(x) => stream_err(x),
                }
                futures_util::future::ready(())
            })
            .await
    }

    pub fn stream(
        &self,
        keep_alive_friendly: bool,
    ) -> impl futures_util::Stream<Item = Result<(String, Option<String>)>> {
        Box::pin(self.client.stream().filter_map(move |event| async move {
            match event {
                Ok(SSE::Event(ev)) => {
                    if ev.event_type == "keep-alive" && !keep_alive_friendly {
                        return None;
                    }

                    if ev.data == "null" {
                        return Some(Ok((ev.event_type, None)));
                    }

                    Some(Ok((ev.event_type, Some(ev.data))))
                }
                Ok(SSE::Comment(_)) => None,
                Err(x) => Some(Err(x)),
            }
        }))
    }
}
