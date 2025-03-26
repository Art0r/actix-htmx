use actix_web::{body::MessageBody, 
    dev::{ServiceRequest, ServiceResponse}, 
    middleware::Next, Error};

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {

    // Call the next middleware or handler
    let res = next.call(req).await?;

    Ok(res)
}

