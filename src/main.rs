use web_metrics::get_web_metrics;

mod config_file;

#[tokio::main]
async fn main() {
    let env_settings :config_file::EnvsSettings = config_file::get_envs().expect("Unable to get env values");

    env_logger::init();

    let file_settings :config_file::Config = config_file::read_config(&env_settings.settings_filepath)
        .expect("Unable to read settings file");

    get_web_metrics(
        file_settings.endpoints,
        file_settings.port,
        file_settings.retry
    )
        .await
        .expect("Unable to get metrics");
}
