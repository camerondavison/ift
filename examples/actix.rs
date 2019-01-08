use actix;
use actix_web::{
    middleware,
    server,
    App,
    HttpRequest,
};
use env_logger;
use ift;

fn index(_req: &HttpRequest) -> &'static str { "Hello world!" }

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("hello-world");

    let mut s = server::new(|| {
        App::new()
            // enable logger
            .middleware(middleware::Logger::default())
            .resource("/index.html", |r| r.f(|_| "Hello world!"))
            .resource("/", |r| r.f(index))
    });

    for ip in ift::eval("GetPrivateInterfaces").unwrap().into_iter() {
        s = s.bind((ip, 8080)).unwrap();
    }

    println!("Started http server");
    for addr in &s.addrs() {
        println!("  listening on {}", addr)
    }

    s.start();
    let _ = sys.run();
}
