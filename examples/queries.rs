use std::env;

const ACCESS_TOKEN: &'static str = "TOKEN";
#[tokio::main]
async fn main() {
    env_logger::init();
    let token = env::var(ACCESS_TOKEN)
        .expect(format!("{} not specified in environment", ACCESS_TOKEN).as_str());
    let client = trading212::Client::new(&token, trading212::Target::Live);
    // match client.get_exchanges().await {
    //     Ok(exchanges) => {
    //         println!("Got {} exchanges", exchanges.len());
    //         for exchange in exchanges {
    //             println!("Exchange: {:?}", exchange.name);
    //         }
    //     }
    //     Err(e) => {
    //         println!("Failed to get exchanges: {:?}", e)
    //     }
    // };

    // match client.get_instruments().await {
    //     Ok(instruments) => {
    //         println!("Got {} instruments", instruments.len());
    //         for instrument in instruments {
    //             println!("Instrument: {:?}", instrument.name);
    //         }
    //     }
    //     Err(e) => {
    //         println!("Failed to get instruments: {:?}", e)
    //     }
    // };

    // match client.get_all_pies().await {
    //     Ok(pies) => {
    //         println!("Got {} pies", pies.len());
    //         for pie in pies {
    //             println!("Pie: {:?}", pie.id);
    //             match client.get_pie(pie.id).await {
    //                 Ok(piedetail) => {
    //                     println!("Got pie: {:?}", piedetail.clone().settings.unwrap().name);
    //                     match client
    //                         .delete_pie(piedetail.clone().settings.unwrap().id)
    //                         .await
    //                     {
    //                         Ok(_) => {
    //                             println!(
    //                                 "Deleted pie: {:?}",
    //                                 piedetail.clone().settings.unwrap().name
    //                             );
    //                         }
    //                         Err(e) => {
    //                             println!("Failed to delete pie: {:?}", e)
    //                         }
    //                     }
    //                 }
    //                 Err(e) => {
    //                     println!("Failed to get pie: {:?}", e)
    //                 }
    //             }
    //         }
    //     }
    //     Err(e) => {
    //         println!("Failed to get pies: {:?}", e)
    //     }
    // };
    //
    // let new_pie = trading212::models::PieRequest {
    //     dividend_cash_action: trading212::models::dividend_cache_action::DividendCashAction::Reinvest,
    //     end_date: time::OffsetDateTime::now_utc(),
    //     goal: 0.0,
    //     icon: trading212::models::icon::Icon::Burger,
    //     instrument_shares: std::collections::HashMap::from([("AAPL_US_EQ".to_string(), 1.0)]),
    //     name: "Test from API".to_string(),
    // };
    // match client.create_pie(new_pie).await {
    //     Ok(pie) => {
    //         println!("Got pie: {:?}", pie.clone().settings.unwrap().name);
    //         // match client
    //         //     .delete_pie(pie.clone().settings.unwrap().id)
    //         //     .await
    //         // {
    //         //     Ok(_) => {
    //         //         println!("Deleted pie: {:?}", pie.settings.unwrap().name);
    //         //     }
    //         //     Err(e) => {
    //         //         println!("Failed to delete pie: {:?}", e)
    //         //     }
    //         // }
    //         let mut updated_pie: trading212::models::PieRequest = pie.clone().try_into().unwrap();
    //         updated_pie.name = "Updated from API".to_string();
    //         match client
    //             .update_pie(pie.clone().settings.unwrap().id, updated_pie)
    //             .await
    //         {
    //             Ok(pie2) => {
    //                 println!("Updated pie: {:?}", pie2.settings.unwrap().name);
    //             }
    //             Err(e) => {
    //                 println!("Failed to update pie: {:?}", e)
    //             }
    //         }
    //     }
    //     Err(e) => {
    //         println!("Failed to get pie: {:?}", e)
    //     }
    // }

    // match client.get_all_orders().await {
    //     Ok(orders) => {
    //         println!("Got {} orders", orders.len());
    //         for order in orders {
    //             println!("Order: {:?}", order.id);
    //         }
    //     }
    //     Err(e) => {
    //         println!("Failed to get orders: {:?}", e)
    //     }
    // };

    // match client
    //     .place_limit_order(trading212::models::LimitRequest {
    //         limit_price: Some(100.0),
    //         quantity: Some(1.0),
    //         ticker: Some("AAPL_US_EQ".to_string()),
    //         time_validity: Some(TimeValidity::Day),
    //     })
    //     .await
    // {
    //     Ok(order) => {
    //         println!("Got order: {:?}", order.id);
    //     }
    //     Err(e) => {
    //         println!("Failed to place order: {:?}", e)
    //     }
    // };
    //
    // match client
    //     .place_market_order(trading212::models::MarketRequest {
    //         quantity: Some(1.0),
    //         ticker: Some("AAPL_US_EQ".to_string()),
    //     })
    //     .await
    // {
    //     Ok(order) => {
    //         println!("Got market order: {:?}", order.id);
    //     }
    //     Err(e) => {
    //         println!("Failed to place order: {:?}", e)
    //     }
    // };
    //
    // match client
    //     .place_stop_order(trading212::models::StopRequest {
    //         stop_price: Some(100.0),
    //         quantity: Some(1.0),
    //         ticker: Some("AAPL_US_EQ".to_string()),
    //         time_validity: Some(TimeValidity::Day),
    //     })
    //     .await
    // {
    //     Ok(order) => {
    //         println!("Got stop order: {:?}", order.id);
    //     }
    //     Err(e) => {
    //         println!("Failed to place order: {:?}", e)
    //     }
    // };
    //
    // match client
    //     .place_stop_limit_order(trading212::models::StopLimitRequest {
    //         stop_price: Some(102.0),
    //         limit_price: Some(201.0),
    //         quantity: Some(1.0),
    //         ticker: Some("AAPL_US_EQ".to_string()),
    //         time_validity: Some(TimeValidity::Day),
    //     })
    //     .await
    // {
    //     Ok(order) => {
    //         println!("Got stop limit order: {:?}", order.id);
    //     }
    //     Err(e) => {
    //         println!("Failed to place order: {:?}", e)
    //     }
    // };
    //
    // match client.get_all_orders().await {
    //     Ok(orders) => {
    //         println!("Got {} orders", orders.len());
    //         for order in orders {
    //             match client.cancel_order(order.id).await {
    //                 Ok(_) => {
    //                     println!("Cancelled order: {:?}", order.id);
    //                     tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    //                 }
    //                 Err(e) => {
    //                     println!("Failed to cancel order: {:?}", e)
    //                 }
    //             }
    //             println!("Order: {:?}", order.id);
    //         }
    //     }
    //     Err(e) => {
    //         println!("Failed to get orders: {:?}", e)
    //     }
    // };

    // match client.get_account_cash().await {
    //     Ok(cash) => {
    //         println!("Got cash: {:?}", cash);
    //     }
    //     Err(e) => {
    //         println!("Failed to get cash: {:?}", e)
    //     }
    // };
    //
    // match client.get_account_metadata().await {
    //     Ok(metadata) => {
    //         println!("Got metadata: {:?}", metadata);
    //     }
    //     Err(e) => {
    //         println!("Failed to get metadata: {:?}", e)
    //     }
    // };
    //
    // match client.get_all_open_positions().await {
    //     Ok(positions) => {
    //         println!("Got {} positions", positions.len());
    //         for position in positions {
    //             println!("Position: {:?}", position);
    //         }
    //     }
    //     Err(e) => {
    //         println!("Failed to get positions: {:?}", e)
    //     }
    // };
    //
    // match client.get_historical_orders(None, None, None).await {
    //     Ok(orders) => {
    //         println!("Got {} orders", orders.items.clone().unwrap().len());
    //         for order in orders.items.unwrap() {
    //             println!("Order: {:#?}", order);
    //         }
    //     }
    //     Err(e) => {
    //         println!("Failed to get orders: {:?}", e)
    //     }
    // };

    // match client.get_paid_dividends(None, None, None).await {
    //     Ok(dividends) => {
    //         println!("Got {} dividends", dividends.items.clone().unwrap().len());
    //         for dividend in dividends.items.unwrap() {
    //             println!("Dividend: {:#?}", dividend);
    //         }
    //     }
    //     Err(e) => {
    //         println!("Failed to get dividends: {:?}", e)
    //     }
    // };

    // match client.export_list().await {
    //     Ok(list) => {
    //         println!("Got list: {:?}", list);
    //     }
    //     Err(e) => {
    //         println!("Failed to get list: {:?}", e)
    //     }
    // };

    // match client
    //     .export_csv(trading212::models::PublicReportRequest {
    //         data_included: trading212::models::ReportDataIncluded {
    //             include_dividends: true,
    //             include_interest: false,
    //             include_orders: false,
    //             include_transactions: false,
    //         },
    //         time_from: OffsetDateTime::now_utc() - time::Duration::days(365),
    //         time_to: OffsetDateTime::now_utc(),
    //     })
    //     .await
    // {
    //     Ok(csv) => {
    //         println!("Got csv: {:?}", csv);
    //     }
    //     Err(e) => {
    //         println!("Failed to get csv: {:?}", e)
    //     }
    // };

    // match client.transaction_list(None, None).await {
    //     Ok(transactions) => {
    //         println!(
    //             "Got {} transactions",
    //             transactions.items.clone().unwrap().len()
    //         );
    //         for transaction in transactions.items.unwrap() {
    //             println!("Transaction: {:#?}", transaction);
    //         }
    //     }
    //     Err(e) => {
    //         println!("Failed to get transactions: {:?}", e)
    //     }
    // };
}
