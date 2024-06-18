use yew::{html, Component, Context, Html};

pub struct Home;
impl Component for Home {
   type Message = ();
   type Properties = ();

   fn create(_ctx: &Context<Self>) -> Self {
      Self
   }

   fn view(&self, _ctx: &Context<Self>) -> Html {
      html! {
         <div class="container">
            <div class="columns is-centered">
               <div class="hero">
                  <div class="hero-body is-centered">
                     <h1 class="title is-1">{ "My Man" }</h1>
                     <h2 class="subtitle">{ "Discord entrance announcements and soundboard" }</h2>
                  </div>
               </div>
            </div>
            <div class="grid">
               <div class="cell">
                  <div class="box">
                     <p class="title">{ "How does it Work?" }</p>
                     <div class="content">
                        {
                           "If your server admin has uploaded an entrance sound for your username, when you join a
                           voice channel the bot will play that entrance sound. To interact with the bot, you can
                           either use its slash commands in your discord server (see the /help command for more
                           details), or use the clips page for your server and play them using the buttons."
                        }
                     </div>
                  </div>
               </div>
               <div class="cell">
                  <div class="box">
                     <p class="title">{ "Server Admins" }</p>
                     <div class="content">
                        { "If you are an admin of a server or have been given a role that grants access to
                           administer the bot, you can use the Manage Servers page to upload, preview, or
                           delete user entrance sounds or soundboard clips."
                        }
                     </div>
                  </div>
               </div>
            </div>
         </div>
      }
   }
}
