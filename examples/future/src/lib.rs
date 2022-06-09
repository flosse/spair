// Copied from https://github.com/rustwasm/wasm-bindgen/blob/master/examples/fetch/src/lib.rs
// and modified to work with spair

use serde::{Deserialize, Serialize};
use spair::prelude::*;
use spair::web_sys::*;

/// A struct to hold some data from the github Branch API.
///
/// Note how we don't have to define every member -- serde will ignore extra
/// data when deserializing
#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub commit: Commit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub sha: String,
    pub commit: CommitDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitDetails {
    pub author: Signature,
    pub committer: Signature,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Signature {
    pub name: String,
    pub email: String,
}

struct State {
    branch: Option<Branch>,
    message: String,
}

impl State {
    fn set_data(&mut self, branch: Branch) {
        self.branch = Some(branch);
        self.message = "".to_string();
    }

    fn reset(&mut self) {
        self.branch = None;
        self.message = "Wait for your click".to_string();
    }

    fn start_fetching(&mut self) -> spair::Command<Self> {
        self.message = "Clicked! Please wait for a moment".to_string();

        spair::Future::new(async {
            let mut opts = RequestInit::new();
            opts.method("GET");

            let url = "https://api.github.com/repos/rustwasm/wasm-bindgen/branches/master";
            let request = Request::new_with_str_and_init(url, &opts)?;

            request
                .headers()
                .set("Accept", "application/vnd.github.v3+json")?;
            let window = spair::window();
            let resp_value = spair::JsFuture::from(window.fetch_with_request(&request)).await?;
            let resp: Response = resp_value.dyn_into().unwrap_throw();
            let json = spair::JsFuture::from(resp.json()?).await?;
            let branch_info: Branch = json.into_serde().unwrap_throw();

            Ok(branch_info)
        })
        .callback(
            |state: &mut Self, r: Result<Branch, spair::JsValue>| match r {
                Ok(rok) => state.set_data(rok),
                Err(rerr) => state.fetch_error(rerr),
            },
        )
    }

    fn fetch_error(&mut self, e: spair::JsValue) {
        self.message = e.as_string().unwrap_or_else(|| format!("{:?}", e));
    }
}

impl spair::Component for State {
    type Routes = ();
    fn render(&self, element: spair::Element<Self>) {
        let comp = element.comp();
        element
            .r#static("You are running `examples\\future`")
            .line_break()
            .match_if(|mi| match self.branch.as_ref() {
                Some(branch) => spair::set_arm!(mi)
                    .render(branch)
                    .button(|b| {
                        b.static_attributes()
                            .on_click(comp.handler_mut(State::reset))
                            .static_nodes()
                            .r#static("Reset");
                    })
                    .done(),
                None => spair::set_arm!(mi)
                    .button(|b| {
                        b.static_attributes()
                            .on_click(comp.handler_mut(State::start_fetching))
                            .static_nodes()
                            .r#static("Click to fetch wasm-bindgen latest commit info");
                    })
                    .done(),
            })
            .p(|p| p.render(&self.message).done());
    }
}

impl spair::Render<State> for &Branch {
    fn render(self, nodes: spair::Nodes<State>) {
        nodes
            .p(|p| {
                p.r#static("The latest commit to the wasm-bindgen ")
                    .render(&self.name)
                    .r#static(" branch is:");
            })
            .render(&self.commit);
    }
}

impl spair::Render<State> for &Commit {
    fn render(self, nodes: spair::Nodes<State>) {
        nodes.p(|p| {
            p.render(&self.sha)
                .r#static(", authored by ")
                .render(&self.commit.author.name)
                .r#static(" (")
                .render(&self.commit.author.email)
                .r#static(")");
        });
    }
}

impl spair::Application for State {
    fn init(_: &spair::Comp<Self>) -> Self {
        Self {
            branch: None,
            message: "Wait for your click".to_string(),
        }
    }
}

#[wasm_bindgen(start)]
pub fn start_fetch_example() {
    State::mount_to("root")
}