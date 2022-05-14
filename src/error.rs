// copied form actix example https://github.com/actix/examples/tree/master/templating/tera
use crate::state::AppState;
use actix_web::{
    body::BoxBody,
    dev::ServiceResponse,
    error::Result,
    http::{header::ContentType, StatusCode},
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    web, HttpResponse,
};

pub fn error_handlers() -> ErrorHandlers<BoxBody> {
    ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
}

// Error handler for a 404 Page not found error.
fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<BoxBody>> {
    let response = get_error_response(&res, "Page not found");
    Ok(ErrorHandlerResponse::Response(ServiceResponse::new(
        res.into_parts().0,
        response.map_into_left_body(),
    )))
}

// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> HttpResponse {
    let request = res.request();

    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.
    let fallback = |e: &str| {
        HttpResponse::build(res.status())
            .content_type(ContentType::plaintext())
            .body(e.to_string())
    };

    let tera = request.app_data::<web::Data<AppState>>().unwrap().tera.clone();
    let mut context = tera::Context::new();
    context.insert("message", error);
    context.insert("status_code", res.status().as_str());
    let body = tera.render("error/status.html.tera", &context);

    match body {
        Ok(body) => HttpResponse::build(res.status())
            .content_type(ContentType::html())
            .body(body),
        Err(_) => fallback(error),
    }
}
