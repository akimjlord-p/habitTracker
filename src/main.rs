mod service;
mod cli;
mod db;

fn main(){
    let mut app = cli::handlers::CLi::initiate();
    app.run();
}
