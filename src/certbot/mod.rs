mod logic;
mod tester;

use crate::{
    certbot::{logic::run_regular, tester::run_test},
    config::Config,
    run_options::certbot_run_options::CertbotRunOptions,
    services::helpers::build_client,
};

pub fn run(options: CertbotRunOptions) -> Result<(), Box<dyn std::error::Error>> {
    options
        .check()
        .unwrap_or_else(|e| panic!("Arguments provided are not correct: {}", e));

    println!("Starting with args: {:?}", &options);

    let config = Config::from_file_or_default(&options.config_file_path)?;

    println!("Starting with config: {:?}", config);

    let domain_controllers = config.into_domain_controllers();

    let proxy = options.proxy.clone();
    let client_maker = Box::new(|| build_client(proxy.clone()));

    match options.test_run {
        true => run_test(client_maker.as_ref(), domain_controllers),
        false => run_regular(
            client_maker.as_ref(),
            options.into_simplified(),
            domain_controllers,
        ),
    }?;

    Ok(())
}
