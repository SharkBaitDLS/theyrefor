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
         <div class="tile is-ancestor is-vertical">
            <div class="tile is-child hero">
               <div class="hero-body container pb-0">
                  <h1 class="title is-1">{ "My Man" }</h1>
                  <h2 class="subtitle">{ "Discord entrance announcements and soundboard" }</h2>
               </div>
            </div>
            <div class="tile is-parent container">
               <div class="tile is-parent">
                  <div class="tile is-child box">
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
               <div class="tile is-parent">
                  <div class="tile is-child box">
                     <p class="title">{ "Server Admins" }</p>
                     <div class="content">
                        <article class="message is-info block">
                           <div class="message-header">
                              <p>{ "Coming Soon!" }</p>
                           </div>
                           <div class="message-body">
                              { "This feature will be available at a later date." }
                           </div>
                        </article>
                     </div>
                  </div>
               </div>
            </div>
         </div>
      }
   }
}
