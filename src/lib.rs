#![deny(missing_docs)]
//! A library for interacting with the Trading212 API
//!
//! <p style="background:rgba(255,181,77,0.16);padding:0.75em;">
//! This is very experimental and probably contains bugs and invalid documentation, use with caution
//! </p>
//!
//! # Example
//! ```
//! use trading212::Client;
//!
//! #[tokio::main]
//! async fn main() {
//!    let client = Client::new("token", trading212::Target::Demo).unwrap();
//!    let exchanges = client.get_exchanges().await.unwrap();
//!    println!("{:?}", exchanges);
//! }
//! ```
//!
//! The API is documented [here](https://t212public-api-docs.redoc.ly/)
//! It is bound to change as it is v0 and currently in Beta

#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

use crate::error::Error;
use derivative::Derivative;
use log::{debug, error};
use reqwest::header::HeaderValue;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod error;
pub mod models;

/// The target to use for the API 0
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Target {
    /// The demo account
    Demo,
    /// The live account
    Live,
}

/// The client to use for the API
#[derive(Derivative, Debug, Clone, Serialize, Deserialize)]
#[derivative(PartialEq)]
pub struct Client {
    target: Target,
    token: String,
    #[serde(skip_serializing, skip_deserializing)]
    #[derivative(PartialEq = "ignore")]
    client: reqwest::Client,
    proxy: Option<String>,
}

impl Client {
    /// Create a new client
    /// # Arguments
    /// * `token` - The token to use for authentication
    /// * `target` - The target to use for the API
    /// # Errors
    /// * `Error::Reqwest` - If the reqwest client can't be created
    ///
    /// # Example
    /// ```
    /// use trading212::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let client = Client::new("token", trading212::Target::Demo).unwrap();
    /// }
    /// ```

    pub fn new(token: &str, target: Target) -> Result<Self, Error> {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        const NAME: &str = env!("CARGO_PKG_NAME");

        let mut headers = reqwest::header::HeaderMap::new();
        if let Ok(value) = HeaderValue::from_str(token) {
            headers.insert(reqwest::header::AUTHORIZATION, value);
        }

        let client = reqwest::ClientBuilder::new()
            .user_agent(format!("{NAME}/{VERSION}"))
            .default_headers(headers);
        match client.build() {
            Ok(c) => Ok(Self {
                target,
                client: c,
                token: token.to_string(),
                proxy: None,
            }),
            Err(e) => Err(e.into()),
        }
    }

    /// Create a new client with a proxy
    /// # Arguments
    /// * `token` - The token to use for authentication
    /// * `target` - The target to use for the API
    /// * `proxy` - The proxy to use
    /// # Errors
    /// * `Error::Reqwest` - If the reqwest client can't be created
    pub fn new_with_proxy(token: &str, target: Target, proxy: &str) -> Result<Self, Error> {
        match Self::new(token, target) {
            Ok(c) => Ok(c.with_proxy(proxy)),
            Err(e) => Err(e),
        }
    }

    /// Add a proxy to the client
    /// # Arguments
    /// * `proxy` - The proxy to use
    /// # Returns
    /// * `Self` - The client with the proxy
    pub fn with_proxy(mut self, proxy: &str) -> Self {
        self.proxy = Some(proxy.to_string());
        self
    }

    async fn api_get<T: DeserializeOwned>(&self, rest_method: &str) -> Result<T, Error> {
        match self
            .client
            .get(format!(
                "{}https://{}.trading212.com/api/v0/{rest_method}",
                match self.proxy {
                    Some(ref p) => {
                        p.clone()
                    }
                    None => {
                        "".to_string()
                    }
                },
                match self.target {
                    Target::Demo => {
                        "demo"
                    }
                    Target::Live => {
                        "live"
                    }
                }
            ))
            .send()
            .await
        {
            Ok(resp) => {
                if resp.status() == StatusCode::UNAUTHORIZED {
                    return Err(Error::Token);
                }
                if resp.status() == StatusCode::FORBIDDEN {
                    return Err(Error::Scope);
                }
                if resp.status() == StatusCode::TOO_MANY_REQUESTS {
                    return Err(Error::Limit);
                }
                if resp.status() == StatusCode::REQUEST_TIMEOUT {
                    return Err(Error::Timeout);
                }
                match resp.text().await {
                    Ok(v) => {
                        let de: Result<T, _> = serde_json::from_str(&v);
                        match de {
                            Ok(reply) => Ok(reply),
                            Err(e) => {
                                error!("Couldn't parse reply for {} call: {}", rest_method, e);
                                debug!("Source JSON: {}", v);
                                Err(e.into())
                            }
                        }
                    }
                    Err(e) => Err(e.into()),
                }
            }
            Err(e) => Err(e.into()),
        }
    }

    async fn api_post<T: DeserializeOwned, U: Serialize + ?Sized + Sync>(
        &self,
        rest_method: &str,
        body: &U,
    ) -> Result<T, Error> {
        debug!("Posting json: {:?}", serde_json::to_string(body));
        match self
            .client
            .post(format!(
                "{}https://{}.trading212.com/api/v0/{rest_method}",
                match self.proxy {
                    Some(ref p) => {
                        p.clone()
                    }
                    None => {
                        "".to_string()
                    }
                },
                match self.target {
                    Target::Demo => {
                        "demo"
                    }
                    Target::Live => {
                        "live"
                    }
                }
            ))
            .json(body)
            .send()
            .await
        {
            Ok(resp) => {
                if resp.status() == StatusCode::UNAUTHORIZED {
                    return Err(Error::Token);
                }
                if resp.status() == StatusCode::FORBIDDEN {
                    return Err(Error::Scope);
                }
                if resp.status() == StatusCode::TOO_MANY_REQUESTS {
                    return Err(Error::Limit);
                }
                if resp.status() == StatusCode::BAD_REQUEST {
                    return Err(Error::Malformed);
                }
                if resp.status() == StatusCode::REQUEST_TIMEOUT {
                    return Err(Error::Timeout);
                }
                match resp.text().await {
                    Ok(v) => {
                        let de: Result<T, _> = serde_json::from_str(&v);
                        match de {
                            Ok(reply) => Ok(reply),
                            Err(e) => {
                                error!("Couldn't parse reply for {} call: {}", rest_method, e);
                                debug!("Source JSON: {}", v);
                                Err(e.into())
                            }
                        }
                    }
                    Err(e) => Err(e.into()),
                }
            }
            Err(e) => Err(e.into()),
        }
    }

    async fn api_delete(&self, rest_method: &str) -> Result<String, Error> {
        match self
            .client
            .delete(format!(
                "{}https://{}.trading212.com/api/v0/{rest_method}",
                match self.proxy {
                    Some(ref p) => {
                        p.clone()
                    }
                    None => {
                        "".to_string()
                    }
                },
                match self.target {
                    Target::Demo => {
                        "demo"
                    }
                    Target::Live => {
                        "live"
                    }
                }
            ))
            .send()
            .await
        {
            Ok(resp) => {
                if resp.status() == StatusCode::UNAUTHORIZED {
                    return Err(Error::Token);
                }
                if resp.status() == StatusCode::FORBIDDEN {
                    return Err(Error::Scope);
                }
                if resp.status() == StatusCode::TOO_MANY_REQUESTS {
                    return Err(Error::Limit);
                }
                if resp.status() == StatusCode::REQUEST_TIMEOUT {
                    return Err(Error::Timeout);
                }
                match resp.text().await {
                    Ok(v) => Ok(v),
                    Err(e) => Err(e.into()),
                }
            }
            Err(e) => Err(e.into()),
        }
    }

    /// Fetch all exchanges and their corresponding working schedules that your account has access to
    /// # Errors
    /// * `Error::Token` - Bad API key
    /// * `Error::Scope` - Scope( metadata ) missing for API key
    /// * `Error::Limit` - Too many requests 1 / 30s
    /// * `Error::Timeout` - Request timed out
    ///
    /// # Example
    /// ```
    /// use trading212::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let client = Client::new("token", trading212::Target::Demo).unwrap();
    ///    let exchanges = client.get_exchanges().await.unwrap();
    ///    println!("{:?}", exchanges);
    /// }
    /// ```
    pub async fn get_exchanges(&self) -> Result<Vec<models::exchange::Exchange>, Error> {
        self.api_get("equity/metadata/exchanges").await
    }

    /// Fetch all instruments that your account has access to
    /// # Errors
    /// * `Error::Token` - Bad API key
    /// * `Error::Scope` - Scope( metadata ) missing for API key
    /// * `Error::Limit` - Too many requests 1 / 50s
    /// * `Error::Timeout` - Request timed out
    /// # Example
    /// ```
    /// use trading212::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let client = Client::new("token", trading212::Target::Demo).unwrap();
    ///    let instruments = client.get_instruments().await.unwrap();
    ///    println!("{:?}", instruments);
    /// }
    /// ```
    pub async fn get_instruments(
        &self,
    ) -> Result<Vec<models::tradeable_instrument::TradeableInstrument>, Error> {
        self.api_get("equity/metadata/instruments").await
    }

    /// Fetches all pies for the account
    /// # Errors
    /// * `Error::Token` - Bad API key
    /// * `Error::Scope` - Scope( pies:read ) missing for API key
    /// * `Error::Limit` - Too many requests 1 / 30s
    /// * `Error::Timeout` - Request timed out
    /// # Example
    /// ```
    /// use trading212::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let client = Client::new("token", trading212::Target::Demo).unwrap();
    ///    let pies = client.get_all_pies().await.unwrap();
    ///    println!("{:?}", pies);
    /// }
    /// ```
    pub async fn get_all_pies(
        &self,
    ) -> Result<Vec<models::account_bucket_result_response::AccountBucketResultResponse>, Error>
    {
        self.api_get("equity/pies").await
    }

    /// Fetches a pie for the account with detailed information
    /// # Arguments
    /// * `pie_id` - The id of the pie to fetch
    /// # Errors
    /// * `Error::Token` - Bad API key
    /// * `Error::Scope` - Scope( pies:read ) missing for API key
    /// * `Error::Limit` - Too many requests 1 / 5s
    /// * `Error::Timeout` - Request timed out
    /// # Example
    /// ```
    /// use trading212::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let client = Client::new("token", trading212::Target::Demo).unwrap();
    ///    let pie = client.get_pie(123456).await.unwrap();
    ///    println!("{:?}", pie);
    /// }
    /// ```
    pub async fn get_pie(
        &self,
        pie_id: i64,
    ) -> Result<models::account_bucket_instruments_detailed_response::AccountBucketInstrumentsDetailedResponse, Error>{
        self.api_get(format!("equity/pies/{pie_id}").as_str()).await
    }

    /// Creates a pie for the account by given params
    /// # Arguments
    /// * `pie` - The pie to create
    /// # Errors
    /// * `Error::Token` - Bad API key
    /// * `Error::Scope` - Scope( pies:write ) missing for API key
    /// * `Error::Limit` - Too many requests 1 / 5s
    /// * `Error::Timeout` - Request timed out
    /// # Example
    /// ```
    /// use trading212::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let client = Client::new("token", trading212::Target::Demo).unwrap();
    ///    let pie = client.create_pie(trading212::models::pie_request::PieRequest::new()).await.unwrap();
    ///    println!("{:?}", pie);
    /// }
    /// ```
    pub async fn create_pie(
        &self,
        pie: models::pie_request::PieRequest,
    ) -> Result<models::account_bucket_instruments_detailed_response::AccountBucketInstrumentsDetailedResponse, Error>{
        self.api_post("equity/pies".to_string().as_str(), &pie)
            .await
    }

    /// Deletes a pie by given id
    /// # Arguments
    /// * `pie_id` - The id of the pie to delete
    /// # Errors
    /// * `Error::Token` - Bad API key
    /// * `Error::Scope` - Scope( pies:write ) missing for API key
    /// * `Error::Limit` - Too many requests 1 / 5s
    /// * `Error::Timeout` - Request timed out
    /// # Example
    /// ```
    /// use trading212::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let client = Client::new("token", trading212::Target::Demo).unwrap();
    ///    client.delete_pie(123456).await.unwrap();
    /// }
    /// ```
    pub async fn delete_pie(&self, pie_id: i64) -> Result<String, Error> {
        self.api_delete(format!("equity/pies/{pie_id}").as_str())
            .await
    }

    /// Updates a pie for the account by given params
    /// # Arguments
    /// * `pie_id` - The id of the pie to update
    /// * `pie` - The pie to update
    /// # Errors
    /// * `Error::Token` - Bad API key
    /// * `Error::Scope` - Scope( pies:write ) missing for API key
    /// * `Error::Limit` - Too many requests 1 / 5s
    /// * `Error::Timeout` - Request timed out
    /// # Example
    /// ```
    /// use trading212::Client;
    /// use trading212::models::pie_request::PieRequest;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let client = Client::new("token", trading212::Target::Demo).unwrap();
    ///    let pie = client.update_pie(123456, PieRequest::new()).await.unwrap();
    ///    println!("{:?}", pie);
    /// }
    /// ```
    pub async fn update_pie(
        &self,
        pie_id: i64,
        pie: models::pie_request::PieRequest,
    ) -> Result<models::account_bucket_instruments_detailed_response::AccountBucketInstrumentsDetailedResponse, Error>{
        self.api_post(format!("equity/pies/{pie_id}").as_str(), &pie)
            .await
    }

    /// Fetches all orders for the account
    /// # Errors
    /// * `Error::Token` - Bad API key
    /// * `Error::Scope` - Scope( orders:read ) missing for API key
    /// * `Error::Limit` - Too many requests 1 / 5s
    /// * `Error::Timeout` - Request timed out
    /// # Example
    /// ```
    /// use trading212::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let client = Client::new("token", trading212::Target::Demo).unwrap();
    ///    let orders = client.get_all_orders().await.unwrap();
    ///    println!("{:?}", orders);
    /// }
    /// ```
    pub async fn get_all_orders(&self) -> Result<Vec<models::order::Order>, Error> {
        self.api_get("equity/orders").await
    }

    /// Fetches an order for the account
    /// # Arguments
    /// * `order_id` - The id of the order to fetch
    /// # Errors
    /// * `Error::Token` - Bad API key
    /// * `Error::Scope` - Scope( orders:read ) missing for API key
    /// * `Error::Limit` - Too many requests 1 / 1s
    /// * `Error::Timeout` - Request timed out
    /// # Example
    /// ```
    /// use trading212::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let client = Client::new("token", trading212::Target::Demo).unwrap();
    ///    let order = client.get_order(123456).await.unwrap();
    ///    println!("{:?}", order);
    /// }
    /// ```
    pub async fn get_order(&self, order_id: i64) -> Result<models::order::Order, Error> {
        self.api_get(format!("equity/orders/{order_id}").as_str())
            .await
    }

    /// Places a limit order for the account
    /// # Arguments
    /// * `order` - The order to place
    /// # Errors
    /// * `Error::Token` - Bad API key
    /// * `Error::Scope` - Scope( orders:execute ) missing for API key
    /// * `Error::Limit` - Too many requests 1 / 2s
    /// * `Error::Timeout` - Request timed out
    /// # Example
    /// ```
    /// use trading212::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let client = Client::new("token", trading212::Target::Demo).unwrap();
    ///    let order = client.place_limit_order(trading212::models::limit_request::LimitRequest::new()).await.unwrap();
    ///    println!("{:?}", order);
    /// }
    /// ```
    pub async fn place_limit_order(
        &self,
        order: models::limit_request::LimitRequest,
    ) -> Result<models::order::Order, Error> {
        self.api_post("equity/orders/limit".to_string().as_str(), &order)
            .await
    }

    /// Places a market order for the account
    /// # Arguments
    /// * `order` - The order to place
    /// # Errors
    /// * `Error::Token` - Bad API key
    /// * `Error::Scope` - Scope( orders:execute ) missing for API key
    /// * `Error::Limit` - Too many requests 1 / 1s
    /// * `Error::Timeout` - Request timed out
    /// # Example
    /// ```
    /// use trading212::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let client = Client::new("token", trading212::Target::Demo).unwrap();
    ///    let order = client.place_market_order(trading212::models::market_request::MarketRequest::new()).await.unwrap();
    ///    println!("{:?}", order);
    /// }
    /// ```
    pub async fn place_market_order(
        &self,
        order: models::market_request::MarketRequest,
    ) -> Result<models::order::Order, Error> {
        self.api_post("equity/orders/market".to_string().as_str(), &order)
            .await
    }

    /// Places a stop order for the account
    /// # Arguments
    /// * `order` - The order to place
    /// # Errors
    /// * `Error::Token` - Bad API key
    /// * `Error::Scope` - Scope( orders:execute ) missing for API key
    /// * `Error::Limit` - Too many requests 1 / 2s
    /// * `Error::Timeout` - Request timed out
    /// # Example
    /// ```
    /// use trading212::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let client = Client::new("token", trading212::Target::Demo).unwrap();
    ///    let order = client.place_stop_order(trading212::models::stop_request::StopRequest::new()).await.unwrap();
    ///    println!("{:?}", order);
    /// }
    /// ```
    pub async fn place_stop_order(
        &self,
        order: models::stop_request::StopRequest,
    ) -> Result<models::order::Order, Error> {
        self.api_post("equity/orders/stop".to_string().as_str(), &order)
            .await
    }

    /// Places a stop limit order for the account
    /// # Arguments
    /// * `order` - The order to place
    /// # Errors
    /// * `Error::Token` - Bad API key
    /// * `Error::Scope` - Scope( orders:execute ) missing for API key
    /// * `Error::Limit` - Too many requests 1 / 2s
    /// * `Error::Timeout` - Request timed out
    /// # Example
    /// ```
    /// use trading212::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let client = Client::new("token", trading212::Target::Demo).unwrap();
    ///    let order = client.place_stop_limit_order(trading212::models::stop_limit_request::StopLimitRequest::new()).await.unwrap();
    ///    println!("{:?}", order);
    /// }
    /// ```
    pub async fn place_stop_limit_order(
        &self,
        order: models::stop_limit_request::StopLimitRequest,
    ) -> Result<models::order::Order, Error> {
        self.api_post("equity/orders/stop_limit".to_string().as_str(), &order)
            .await
    }

    /// Cancels an order for the account
    /// # Arguments
    /// * `order_id` - The id of the order to cancel
    /// # Errors
    /// * `Error::Token` - Bad API key
    /// * `Error::Scope` - Scope( orders:execute ) missing for API key
    /// * `Error::Limit` - Too many requests 1 / 1s
    /// * `Error::Timeout` - Request timed out
    /// # Example
    /// ```
    /// use trading212::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let client = Client::new("token", trading212::Target::Demo).unwrap();
    ///    client.cancel_order(123456).await.unwrap();
    /// }
    /// ```
    pub async fn cancel_order(&self, order_id: i64) -> Result<String, Error> {
        self.api_delete(format!("equity/orders/{order_id}").as_str())
            .await
    }

    /// Get account cash
    /// # Errors
    /// * `Error::Token` - Bad API key
    /// * `Error::Scope` - Scope( account ) missing for API key
    /// * `Error::Limit` - Too many requests 1 / 2s
    /// * `Error::Timeout` - Request timed out
    /// # Example
    /// ```
    /// use trading212::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let client = Client::new("token", trading212::Target::Demo).unwrap();
    ///    let cash = client.get_account_cash().await.unwrap();
    ///    println!("{:?}", cash);
    /// }
    /// ```
    pub async fn get_account_cash(&self) -> Result<models::cash::Cash, Error> {
        self.api_get("equity/account/cash").await
    }

    /// Get account metadata
    /// # Errors
    /// * `Error::Token` - Bad API key
    /// * `Error::Scope` - Scope( account ) missing for API key
    /// * `Error::Limit` - Too many requests 1 / 30s
    /// * `Error::Timeout` - Request timed out
    /// # Example
    /// ```
    /// use trading212::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let client = Client::new("token", trading212::Target::Demo).unwrap();
    ///    let metadata = client.get_account_metadata().await.unwrap();
    ///    println!("{:?}", metadata);
    /// }
    /// ```
    pub async fn get_account_metadata(&self) -> Result<models::account::Account, Error> {
        self.api_get("equity/account/info").await
    }

    /// Get all open positions
    /// # Errors
    /// * `Error::Token` - Bad API key
    /// * `Error::Scope` - Scope( portfolio ) missing for API key
    /// * `Error::Limit` - Too many requests 1 / 5s
    /// * `Error::Timeout` - Request timed out
    /// # Example
    /// ```
    /// use trading212::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let client = Client::new("token", trading212::Target::Demo).unwrap();
    ///    let positions = client.get_all_open_positions().await.unwrap();
    ///    println!("{:?}", positions);
    /// }
    /// ```
    pub async fn get_all_open_positions(&self) -> Result<Vec<models::position::Position>, Error> {
        self.api_get("equity/portfolio").await
    }

    /// Get a position by id
    /// # Arguments
    /// * `position_id` - The id of the position to fetch
    /// # Errors
    /// * `Error::Token` - Bad API key
    /// * `Error::Scope` - Scope( portfolio ) missing for API key
    /// * `Error::Limit` - Too many requests 1 / 1s
    /// * `Error::Timeout` - Request timed out
    /// # Example
    /// ```
    /// use trading212::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let client = Client::new("token", trading212::Target::Demo).unwrap();
    ///    let position = client.get_position(123456).await.unwrap();
    ///    println!("{:?}", position);
    /// }
    /// ```
    pub async fn get_position(
        &self,
        position_id: i64,
    ) -> Result<models::position::Position, Error> {
        self.api_get(format!("equity/portfolio/{position_id}").as_str())
            .await
    }

    /// Get order history
    /// # Arguments
    /// * `limit` - The number of orders to fetch
    /// * `cursor` - The cursor to use for pagination
    /// * `ticker` - The ticker to filter by
    /// # Errors
    /// * `Error::Token` - Bad API key
    /// * `Error::Scope` - Scope( history:orders ) missing for API key
    /// * `Error::Limit` - Too many requests 6 / 1m0s
    /// * `Error::Timeout` - Request timed out
    /// # Example
    /// ```
    /// use trading212::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let client = Client::new("token", trading212::Target::Demo).unwrap();
    ///    let orders = client.get_historical_orders(None, None, None).await.unwrap();
    ///    println!("{:?}", orders);
    /// }
    /// ```
    pub async fn get_historical_orders(
        &self,
        limit: Option<i32>,
        cursor: Option<i64>,
        ticker: Option<String>,
    ) -> Result<models::paginated_response_historical_order::PaginatedResponseHistoricalOrder, Error>
    {
        let mut path = "equity/history/orders".to_string();
        if limit.is_some() || cursor.is_some() || ticker.is_some() {
            path.push('?');
        }
        if let Some(limit) = limit {
            path.push_str(format!("limit={limit}",).as_str());
        }
        if let Some(cursor) = cursor {
            path.push_str(format!("&cursor={cursor}",).as_str());
        }
        if let Some(ticker) = ticker {
            path.push_str(format!("&ticker={ticker}",).as_str());
        }
        self.api_get(&path).await
    }

    /// Get dividend history
    /// # Arguments
    /// * `limit` - The number of dividends to fetch
    /// * `cursor` - The cursor to use for pagination
    /// * `ticker` - The ticker to filter by
    /// # Errors
    /// * `Error::Token` - Bad API key
    /// * `Error::Scope` - Scope( history:dividends ) missing for API key
    /// * `Error::Limit` - Too many requests 6 / 1m0s
    /// * `Error::Timeout` - Request timed out
    /// # Example
    /// ```
    /// use trading212::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let client = Client::new("token", trading212::Target::Demo).unwrap();
    ///    let dividends = client.get_paid_dividends(None, None, None).await.unwrap();
    ///    println!("{:?}", dividends);
    /// }
    /// ```
    pub async fn get_paid_dividends(
        &self,
        limit: Option<i32>,
        cursor: Option<i64>,
        ticker: Option<String>,
    ) -> Result<
        models::paginated_response_history_dividend_item::PaginatedResponseHistoryDividendItem,
        Error,
    > {
        let mut path = "history/dividends".to_string();
        if limit.is_some() || cursor.is_some() || ticker.is_some() {
            path.push('?');
        }
        if let Some(limit) = limit {
            path.push_str(format!("limit={limit}",).as_str());
        }
        if let Some(cursor) = cursor {
            path.push_str(format!("&cursor={cursor}",).as_str());
        }
        if let Some(ticker) = ticker {
            path.push_str(format!("&ticker={ticker}",).as_str());
        }
        self.api_get(&path).await
    }

    /// Get list of all exports
    /// # Errors
    /// * `Error::Token` - Bad API key
    /// * `Error::Scope` - Missing Permissions
    /// * `Error::Limit` - Too many requests 1 / 1m0s
    /// * `Error::Timeout` - Request timed out
    /// # Example
    /// ```
    /// use trading212::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let client = Client::new("token", trading212::Target::Live).unwrap();
    ///    let exports = client.export_list().await.unwrap();
    ///    println!("{:?}", exports);
    /// }
    /// ```
    pub async fn export_list(&self) -> Result<Vec<models::report_response::ReportResponse>, Error> {
        self.api_get("history/exports").await
    }

    /// Initiate CSV export
    /// # Arguments
    /// * `data` - The data to export
    /// # Errors
    /// * `Error::Token` - Bad API key
    /// * `Error::Scope` - Missing Permissions
    /// * `Error::Limit` - Too many requests 1 / 30s
    /// * `Error::Timeout` - Request timed out
    /// * `Error::Malformed` - Malformed request
    /// # Example
    /// ```
    /// use trading212::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let client = Client::new("token", trading212::Target::Live).unwrap();
    ///    let exports = client.export_csv(trading212::models::public_report_request::PublicReportRequest::new()).await.unwrap();
    ///    println!("{:?}", exports);
    /// }
    /// ```
    pub async fn export_csv(
        &self,
        data: models::public_report_request::PublicReportRequest,
    ) -> Result<models::enqueued_report_response::EnqueuedReportResponse, Error> {
        self.api_post("history/exports", &data).await
    }

    /// Fetch superficial information about movements to and from your account
    /// # Arguments
    /// * `limit` - The number of transactions to fetch
    /// * `cursor` - The cursor to use for pagination
    /// # Errors
    /// * `Error::Token` - Bad API key
    /// * `Error::Scope` - Scope( history:transactions ) missing for API key
    /// * `Error::Limit` - Too many requests 6 / 1m0s
    /// * `Error::Timeout` - Request timed out
    /// * `Error::Malformed` - Malformed request
    /// # Example
    /// ```
    /// use trading212::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///    let client = Client::new("token", trading212::Target::Live).unwrap();
    ///    let transactions = client.transaction_list(None, None).await.unwrap();
    ///    println!("{:?}", transactions);
    /// }
    /// ```
    pub async fn transaction_list(
        &self,
        limit: Option<i32>,
        cursor: Option<i64>,
    ) -> Result<models::paginated_response_history_transaction_item::PaginatedResponseHistoryTransactionItem, Error>{
        let mut path = "history/transactions".to_string();
        if limit.is_some() || cursor.is_some() {
            path.push('?');
        }
        if let Some(limit) = limit {
            path.push_str(format!("limit={limit}").as_str());
        }
        if let Some(cursor) = cursor {
            path.push_str(format!("&cursor={cursor}").as_str());
        }
        self.api_get(&path).await
    }
}
