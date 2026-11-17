use topcoat::context::Cx;
use topcoat::router::page;
use topcoat::view::{view, View};

#[page("/about")]
async fn about(cx: &Cx) -> topcoat::Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html>
            <head>
                <title>"about"</title>
                topcoat::dev::script()
            </head>
            <body>
               "This is page about me"
            </body>
        </html>
    })
}