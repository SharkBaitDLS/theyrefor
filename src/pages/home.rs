use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Home;
impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="tile is-ancestor is-vertical">
                <div class="tile is-child hero">
                    <div class="hero-body container pb-0">
                        <h1 class="title is-1">{ "My Man" }</h1>
                        <h2 class="subtitle">{ "Discord entrance announcements and soundboard" }</h2>
                    </div>
                </div>

                <div class="tile is-parent container">
                    { self.view_info_tiles() }
                </div>
            </div>
        }
    }
}
impl Home {
    fn view_info_tiles(&self) -> Html {
        html! {
            <>
                <div class="tile is-parent">
                    <div class="tile is-child box">
                        <p class="title">{ "How does it Work?" }</p>

                        <div class="content">
                            {r#"If your server admin has uploaded an entrance sound for your username, when you join a
                             voice channel the bot will play that entrance sound. To interact with the bot, you can 
                             either use its slash commands in your discord server (see the /help command for more 
                             details), or use the clips page for your server and play them using the buttons. 
                            "#}
                        </div>
                    </div>
                </div>
                <div class="tile is-parent">
                    <div class="tile is-child box">
                        <p class="title">{ "Server Admins" }</p>
                        <div class="content">
                            {r#"As a Discord server admin, you can upload clips for your server on the management page.
                            See the instructions on that page for additiona details.
                            "#}
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
