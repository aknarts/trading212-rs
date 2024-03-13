use std::env;
use trading212::error::Error;

const ACCESS_TOKEN: &'static str = "TOKEN";
#[tokio::main]
async fn main() {
    env_logger::init();
    let token = env::var(ACCESS_TOKEN)
        .expect(format!("{} not specified in environment", ACCESS_TOKEN).as_str());
    let client = trading212::Client::new(&token, trading212::Target::Live).unwrap();

    match client.get_account_metadata().await {
        Ok(metadata) => {
            log::info!("Account currency {:#?}", metadata.currency_code);
            log::info!("Account id {:#?}", metadata.id);
        }
        Err(e) => {
            if let Error::Limit = e {
                log::warn!("Hit API limit on metadata")
            }
        }
    }

    match client.get_account_cash().await {
        Ok(cash) => {
            log::info!("Account free cash {:?}", cash.free);
            log::info!("Account pie cash {:?}", cash.pie_cash);
            log::info!("Account invested value {:?}", cash.invested);
            log::info!("Account profit loss {:?}", cash.ppl);
            log::info!("Account total value {:?}", cash.total);
        }
        Err(e) => {
            if let Error::Limit = e {
                log::warn!("Hit API limit on cash")
            }
        }
    }

    match client.get_all_open_positions().await {
        Ok(positions) => {
            for position in positions {
                log::info!(
                    "Position {:?} {:?}({:?})",
                    position.ticker,
                    position.current_price,
                    position.ppl
                );
            }
        }
        Err(e) => {
            if let Error::Limit = e {
                log::warn!("Hit API limit on positions")
            }
        }
    }

    match client.get_all_pies().await {
        Ok(pies) => {
            for pie in pies {
                match client.get_pie(pie.id).await {
                    Ok(pie_details) => {
                        log::info!(
                            "Pie {:?} {:?}",
                            pie_details.settings.name,
                            pie_details.settings.dividend_cash_action
                        );
                        for instrument in pie_details.instruments {
                            log::info!(
                                "\tInstrument {:?} {:?}({:?}) {:?}/{:?}",
                                instrument.ticker,
                                instrument.result.value,
                                instrument.result.result,
                                instrument.current_share,
                                instrument.expected_share
                            );
                        }
                    }
                    Err(e) => {
                        if let Error::Limit = e {
                            log::warn!("Hit API limit on pie details")
                        }
                    }
                }
            }
        }
        Err(e) => {
            if let Error::Limit = e {
                log::warn!("Hit API limit on pies")
            }
        }
    }
}
