use worker::*;

#[event(fetch)]
async fn fetch(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();
    let url = req.url()?;
    let query_pairs = url
        .query_pairs()
        .map(|(k, v)| (format!("{}", k), format!("{}", v)))
        .collect::<Vec<_>>();

    let mut url_to_check = None;

    for pair in query_pairs {
        if pair.0 == "url" {
            console_debug!("Found url: {}", pair.1);
            url_to_check = Some(Url::parse(&pair.1)?);
        }
    }

    let mut status = "Invalid";
    let mut message =
        "Usage: https://downornot.shantanugoel.com/?url=<URL_TO_CHECK_ALONG_WITH_HTTP(S)>"
            .to_string();
    if let Some(u) = url_to_check {
        console_debug!("Checking {}", u);
        let resp = reqwest::get(u).await;
        if let Ok(r) = resp {
            if r.status().is_success() {
                status = "Up";
            } else {
                status = "Down";
            }
            message = r.status().to_string();
        } else {
            status = "Error";
            message = resp.unwrap_err().to_string();
        }
    }
    console_debug!("Status: {}, Message: {}", status, message);
    let result = format!(
        "{{\"status\": \"{}\", \"message\": \"{}\"}}",
        status, message
    );

    Response::ok(result)
}
