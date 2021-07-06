use std::fmt::Debug;
use wasm_bindgen_futures::spawn_local;

use yew::{format::Json, prelude::*};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use graphql_client::GraphQLQuery;
use serde_json::{Value, from_str};

use crate::util::{constant::ObjectId, common::gql_uri};

////////////////////////////////////////////////////
// Fetch projects data use `yew::services::fetch` //
////////////////////////////////////////////////////

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/all_projects.graphql",
    response_derives = "Debug"
)]
struct AllProjects;

#[derive(Debug)]
pub enum Msg {
    PassRequest,
    ReceiveResponse(Result<Vec<Value>, anyhow::Error>),
}

#[derive(Debug)]
pub struct Projects {
    fetch_task: Option<FetchTask>,
    list: Option<Vec<Value>>,
    link: ComponentLink<Self>,
    error: Option<String>,
}

impl Projects {
    fn view_fetching(&self) -> Html {
        if self.fetch_task.is_some() {
            html! { <p>{ "Fetching data..." }</p> }
        } else {
            html! { <p></p> }
        }
    }

    fn view_data(&self) -> Html {
        match self.list {
            Some(ref list) => {
                let projects = list.iter().map(|project| {
                    html! {
                        <div>
                            <li>
                                <strong>{ &project["subject"].as_str().unwrap() }</strong>
                            </li>
                            <ul>
                                <li>{ &project["userId"].as_str().unwrap() }</li>
                                <li>{ &project["id"].as_str().unwrap() }</li>
                                <li>
                                    <a href={ project["website"].as_str().unwrap().to_owned() }>
                                        { &project["website"].as_str().unwrap() }
                                    </a>
                                </li>
                            </ul>
                        </div>
                    }
                });

                html! {
                    <ul>
                        { for projects }
                    </ul>
                }
            }
            None => {
                html! {
                     <p>
                        { "No data." }
                     </p>
                }
            }
        }
    }

    fn view_error(&self) -> Html {
        if let Some(ref error) = self.error {
            html! { <p>{ error.clone() }</p> }
        } else {
            html! {}
        }
    }
}

impl Component for Projects {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { fetch_task: None, list: None, link, error: None }
    }

    fn rendered(&mut self, first_render: bool) {
        let link = self.link.clone();
        if first_render {
            spawn_local(async move {
                link.send_message(Msg::PassRequest);
            });
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::PassRequest => {
                // build graphql query body
                let build_query =
                    AllProjects::build_query(all_projects::Variables {});
                let query = Json(&build_query);

                // build the request
                let request = Request::post(&gql_uri())
                    .body(query)
                    .expect("Could not build request.");

                // construct a callback
                let callback = self.link.callback(
                    |response: Response<Result<String, anyhow::Error>>| {
                        let resp_body = response.into_body();
                        let resp_str = resp_body.as_ref().unwrap();

                        let projects_value: Value =
                            from_str(&resp_str).unwrap();
                        let projects_vec = projects_value["data"]
                            ["allProjects"]
                            .as_array()
                            .unwrap()
                            .to_owned();

                        Msg::ReceiveResponse(Ok(projects_vec))
                    },
                );

                // pass the request and callback to the fetch service
                let task = FetchService::fetch(request, callback)
                    .expect("failed to start request");

                // store the task so it isn't canceled immediately
                self.fetch_task = Some(task);

                // redraw so that the page displays a 'fetching...' message
                true
            }
            Msg::ReceiveResponse(data) => {
                match data {
                    Ok(projects_vec) => {
                        self.list = Some(projects_vec);
                    }
                    Err(error) => self.error = Some(error.to_string()),
                }
                self.fetch_task = None;

                // redraw so that the page displays projects data
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        let link = self.link.clone();
        spawn_local(async move {
            link.send_message(Msg::PassRequest);
        });

        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1>{ "all projects" }</h1>

                { self.view_fetching() }
                { self.view_data() }
                { self.view_error() }
            </>
        }
    }
}
