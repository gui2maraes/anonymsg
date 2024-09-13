use blindchannel::startup::run;
use std::process::ExitCode;
#[tokio::main]
async fn main() -> ExitCode {
    let _guard = blindchannel::telemetry::init_subscriber();
    let settings = match blindchannel::configuration::get_configuration() {
        Ok(s) => s,
        Err(e) => {
            tracing::error!("unable to get configuration: {e}");
            return ExitCode::FAILURE;
        }
    };
    let pool = sqlx::PgPool::connect_lazy_with(settings.database.connect_options());
    let addr = (settings.application.host, settings.application.port);
    run(addr, pool).await;
    ExitCode::SUCCESS
}
