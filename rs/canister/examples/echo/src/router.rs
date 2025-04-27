// HTTP request aur response ke types ko import karte hain
use ic_http_certification::{HttpRequest, HttpResponse};
// HTTP methods aur router functionality ko import karte hain
use oc_bots_sdk_canister::{HttpMethod::*, HttpRouter};
// LazyLock ko import karte hain jo lazy initialization provide karta hai
use std::sync::LazyLock;

// Submodules ko define karte hain
mod commands;    // Commands handling ke liye module
mod definition;  // Bot definition ke liye module

// ROUTER ko static variable ke roop mein define karte hain
// LazyLock ka use karke hum ensure karte hain ki router sirf pehli baar
// jab use hoga tabhi initialize hoga, na ki program start hone par
static ROUTER: LazyLock<HttpRouter> = LazyLock::new(init_router);

// Router ko initialize karne ke liye function
// Ye function HTTP endpoints ko define karta hai aur unhe handlers se map karta hai
fn init_router() -> HttpRouter {
    HttpRouter::default()  // Default router create karte hain
        .route("/execute_command", POST, commands::execute)  // POST requests ko /execute_command endpoint par commands::execute function se handle karte hain
        .fallback(definition::get)  // Koi bhi aur request jo match nahi hui, usko definition::get function se handle karte hain
}

// Main handler function jo har HTTP request ko process karta hai
// 'query' parameter batata hai ki ye query call hai ya update call
pub async fn handle(request: HttpRequest, query: bool) -> HttpResponse {
    // Request ko router ke through process karte hain
    // Router automatically request ko analyze karke sahi handler ko call karega
    ROUTER.handle(request, query).await
}
