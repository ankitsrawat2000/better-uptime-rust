use poem::{
    get, handler, listener::TcpListener, post, web::{Json, Path}, Route, Server
};

use request_inputs::CreateWebsiteInput;
use request_outputs::CreateWebsiteOutput;
use store::store::Store;//from the store crate the store module the Store struct
pub mod request_inputs;
pub mod request_outputs;

#[handler]
fn get_website(Path(website_id): Path<String>) -> String {
    format!("hello: {website_id}")
}

//getting json inputs, giving json outputs
#[handler]
fn create_website( Json(data): Json<CreateWebsiteInput>) -> 
Json<CreateWebsiteOutput> {
    let s = Store::default(); //assuming Store implements Default;
    let id: String = s.create_website();
    let response: CreateWebsiteOutput = CreateWebsiteOutput{
        id
    };
    //persist this in the db
    // sqlx => pg
    // diesel => prisma
    Json(response)
}


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    //specify the business details of your app
    //app.get("/hello/:name", (req, res) => {})
    let app = Route::new()
        .at("/status/:website_id", get(get_website))
        .at("/website", post(create_website));
    //creates and runs the http server
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .name("hello-world")
        .run(app)
        .await
}