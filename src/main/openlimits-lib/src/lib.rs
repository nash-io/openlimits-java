use jni;
use jni::{errors, JNIEnv, JavaVM};
use jni::objects::{JClass, JValue, JObject, JString};
use jni::sys::{jstring, jsize, jobjectArray, jobject, jint};
use rust_decimal::{Decimal, prelude::ToPrimitive};
use rust_decimal::prelude::*;

use openlimits::{
  exchange::{OpenLimits, ExchangeAccount, ExchangeMarketData, Exchange}, 
  exchange_ws::OpenLimitsWs, 
  exchange_info::{MarketPair, ExchangeInfoRetrieval},
  any_exchange::{AnyExchange, InitAnyExchange, AnyWsExchange},
  nash::{
    NashCredentials,
    NashParameters,
    Environment
  },
  binance::{
    BinanceCredentials,
    BinanceParameters,
  },
  model::{
      OrderBookRequest, 
      OrderBookResponse,
      Liquidity,
      Side,
      CancelAllOrdersRequest, 
      CancelOrderRequest,
      AskBid,
      OpenLimitOrderRequest,
      OpenMarketOrderRequest,
      GetOrderHistoryRequest,
      GetOrderRequest,
      TradeHistoryRequest,
      GetHistoricTradesRequest,
      GetHistoricRatesRequest,
      GetPriceTickerRequest,
      Paginator,
      Balance,
      OrderCanceled,
      Order,
      Trade,
      Interval,
      Candle,
      Ticker,
      websocket::Subscription
  }
};
use std::collections::HashMap;
use tokio::stream::StreamExt;
use std::sync::MutexGuard;
use tokio::sync::mpsc::UnboundedSender;
use std::sync::Arc;
use futures_util::future::{select, Either, Future};


fn decimal_to_jvalue<'a>(_env: &JNIEnv<'a>, s: Decimal) -> errors::Result<JValue<'a>> {
  Ok(JValue::Float(s.to_f32().unwrap()))
}

fn get_field<'a>(
  env: &'a JNIEnv,
  obj: &'a JObject,
  field: &str,
  type_: &str,
) -> Result<Option<JValue<'a>>, String> {
  if env.get_field_id(*obj, field, type_).is_err() {
      return Err(format!("Can't find `{}` field", &field));
  }
  
  env.get_field(*obj, field, type_)
      .map(|value| Some(value))
      .or_else(|e| match e {
          errors::Error::NullPtr(_) => Ok(None),
          _ => Err(format!(
              "Can't find `{}` field",
              &field,
          )),
      })
}

fn get_string(env: &JNIEnv, obj: &JObject, field: &str) -> Result<Option<String>, String> {
  let value = get_field(env, obj, field, "Ljava/lang/String;")?;
  match value {
      Some(value) => {
          let string: JString = value
              .l()
              .map_err(|_| format!("field `{}` is not an Object", field))?
              .into();
          if string.is_null() {
            Ok(None)
          } else {
            Ok(env.get_string(string).map(|s| s.into()).ok())
          }
      },
      None => Ok(None),
  }
}


fn bidask_to_jobject<'a>(env: &JNIEnv<'a>, resp: &AskBid) -> errors::Result<JObject<'a>> {
  let cls_bidask = env.find_class("Lio/nash/openlimits/AskBid;")?;

  let ctor_args = vec![
    decimal_to_jvalue(env, resp.price)?,
    decimal_to_jvalue(env, resp.qty)?,
  ];

  let obj = env.new_object(cls_bidask, "(FF)V", ctor_args.as_ref());
  match obj {
    Ok(ok) => Ok(ok),
    Err(e) => panic!("Failed to construct AskBid: {}", e)
  }
}

fn vec_to_java_arr<'a>(env: &JNIEnv<'a>, cls: JClass, v: &Vec<JObject<'a>>) -> errors::Result<JValue<'a>> {
  let arr = env.new_object_array(v.len() as i32, cls, JObject::null())?;
  for i in 0..v.len() {
    env.set_object_array_element(arr, i as jsize, v[i])?;
  }
  Ok(JValue::from(arr))
}

fn orderbook_resp_to_jobject<'a>(env: &JNIEnv<'a>, resp: OrderBookResponse) -> errors::Result<JObject<'a>> {
  let cls_resp = env.find_class("Lio/nash/openlimits/OrderbookResponse;").expect("Failed to find OrderbookResponse class");
  let cls_bidask = env.find_class("Lio/nash/openlimits/AskBid;").expect("Failed to find AskBid class");

  let mut bids = vec![];
  for bid in resp.bids {
    bids.push(bidask_to_jobject(env, &bid)?);
  }
  let bid_arr = vec_to_java_arr(env, cls_bidask, &bids)?;

  let mut asks: Vec<JObject> = vec![];
  for ask in resp.asks {
    asks.push(bidask_to_jobject(env, &ask)?);
  }

  let ask_arr = vec_to_java_arr(env, cls_bidask, &asks)?;

  let ctor_args = vec![
    ask_arr,
    bid_arr,
    JValue::Long(resp.last_update_id.unwrap_or_default() as i64)
  ];
  env.new_object(cls_resp, "([Lio/nash/openlimits/AskBid;[Lio/nash/openlimits/AskBid;J)V", &ctor_args)
}


fn candle_to_jobject<'a>(env: &JNIEnv<'a>, candle: Candle) -> errors::Result<JObject<'a>> {
  let cls_candle = env.find_class("Lio/nash/openlimits/Candle;").expect("Failed to find Candle class");
  
  let ctor_args = vec![
    JValue::Long(candle.time as i64),
    decimal_to_jvalue(env, candle.low)?, 
    decimal_to_jvalue(env, candle.high)?, 
    decimal_to_jvalue(env, candle.open)?, 
    decimal_to_jvalue(env, candle.close)?, 
    decimal_to_jvalue(env, candle.volume)?
  ];

  env.new_object(cls_candle, "(JFFFFF)V", ctor_args.as_ref())
}

fn string_to_jstring<'a>(env: &JNIEnv<'a>, s: String) -> errors::Result<JString<'a>> {
  env.new_string(s)
}

fn side_to_string<'a>(env: &JNIEnv<'a>, s: Side) -> errors::Result<JString<'a>> {
  let s = match s {
    Side::Buy => "Buy",
    Side::Sell => "Sell",
  };
  string_to_jstring(env, String::from(s))
}
fn liquidity_to_string<'a>(env: &JNIEnv<'a>, s: Liquidity) -> errors::Result<JString<'a>> {
  let s = match s {
    Liquidity::Maker => "Maker",
    Liquidity::Taker => "Taker",
  };
  string_to_jstring(env, String::from(s))
}

fn trade_to_jobject<'a>(env: &JNIEnv<'a>, trade: Trade) -> errors::Result<JObject<'a>> {
  let cls_trade = env.find_class("Lio/nash/openlimits/Trade;").expect("Failed to find Trade class");
  
  let ctor_args: Vec<JValue> = vec![
    env.new_string(trade.id)?.into(),
    env.new_string(trade.order_id)?.into(),
    env.new_string(trade.market_pair)?.into(),
    decimal_to_jvalue(env, trade.price)?,
    decimal_to_jvalue(env, trade.qty)?,
    trade.fees.map_or(JValue::Float(0.0), |f| decimal_to_jvalue(env, f).expect("Failed to convert fees to float")),
    side_to_string(env, trade.side)?.into(),
    trade.liquidity.map_or(JValue::Object(JObject::null()), |l| liquidity_to_string(env, l).expect("Failed to convert liquidity to string").into()),
    JValue::Long(trade.created_at as i64)
  ];

  env.new_object(cls_trade, "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;FFFLjava/lang/String;Ljava/lang/String;J)V", ctor_args.as_ref())
}

fn ticker_to_jobject<'a>(env: &JNIEnv<'a>, resp: Ticker) -> errors::Result<JObject<'a>> {
  let cls_resp = env.find_class("Lio/nash/openlimits/Ticker;").expect("Failed to find Ticker class");
  let ctor_args = vec![
    decimal_to_jvalue(env, resp.price)?
  ];
  env.new_object(cls_resp, "(F)V", &ctor_args)
}

fn order_to_jobject<'a>(env: &JNIEnv<'a>, resp: Order) -> errors::Result<JObject<'a>> {
  let cls_resp = env.find_class("Lio/nash/openlimits/Order;").expect("Failed to find Order class");
  let ctor_args = vec![
  ];
  env.new_object(cls_resp, "()V", &ctor_args)
}

// This keeps Rust from "mangling" the name and making it unique for this crate.
#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_init(env: JNIEnv, _class: JClass, cli: JObject, conf: JObject) -> jint {
  let init_params = match get_options(&env, &conf) {
    Ok(conf) => conf,
    Err(e) => panic!("invalid config: {}", e)
  };
  
  let mut runtime = tokio::runtime::Builder::new().basic_scheduler().enable_all().build().expect("Failed to create tokio runtime");
  
  let client_future = OpenLimits::instantiate(init_params.clone());
  let client: AnyExchange = runtime.block_on(client_future);

  env.set_rust_field(cli, "_client", client).unwrap();
  env.set_rust_field(cli, "_runtime", runtime).unwrap();
  return 1
}

#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_orderBook(env: JNIEnv, _class: JClass,  cli: JObject, market: JString) -> jobject {
  let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client").expect("Failed to get client");
  let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime").expect("Failed to get runtime");

  let req = OrderBookRequest {
    market_pair: env.get_string(market).expect("unvalid string").into()
  };

  let resp = runtime.block_on(client.order_book(&req)).expect("Failed to get response");
  let out = orderbook_resp_to_jobject(&env, resp).expect("Failed to convert responce to Java object");

  out.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_getPriceTicker(env: JNIEnv, _class: JClass,  cli: JObject, market: JString) -> jobject {
  let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client").expect("Failed to get client");
  let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime").expect("Failed to get runtime");

  let req = GetPriceTickerRequest {
    market_pair: env.get_string(market).expect("unvalid string").into()
  };

  let resp = runtime.block_on(client.get_price_ticker(&req)).expect("Failed to get response");
  let out = ticker_to_jobject(&env, resp).expect("Failed to convert responce to Java object");

  out.into_inner()
}


#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_getHistoricRates(env: JNIEnv, _class: JClass,  cli: JObject, hist_req: JObject) -> jobject {
  let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client").expect("Failed to get client");
  let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime").expect("Failed to get runtime");

  let req = get_historic_rates_request(&env, &hist_req).expect("Failed to parse params");

  let resp = runtime.block_on(client.get_historic_rates(&req)).expect("Failed to get response");
  let mut candles: Vec<JObject> = Vec::new();
  for candle in resp {
    candles.push(candle_to_jobject(&env, candle).unwrap());
  }
  
  let candle_cls = env.find_class("Lio/nash/openlimits/Candle;").expect("Can't find Candle Class");
  let out = vec_to_java_arr(&env, candle_cls, &candles).expect("Failed to convert vec to array");

  out.l().expect("failed to convert out array to object").into_inner()
}


#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_getHistoricTrades(env: JNIEnv, _class: JClass,  cli: JObject, trades_req: JObject) -> jobject {
  let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client").expect("Failed to get client");
  let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime").expect("Failed to get runtime");

  let req = get_historic_trades_request(&env, &trades_req).expect("Failed to parse params");

  let resp = runtime.block_on(client.get_historic_trades(&req)).expect("Failed to get response");
  let mut trades: Vec<JObject> = Vec::new();
  for trade in resp {
    trades.push(match trade_to_jobject(&env, trade) {
      Ok(r) => r,
      Err(e) => panic!("Failed to create Trade: {}", e)
    });
  }
  
  let trade_cls = env.find_class("Lio/nash/openlimits/Trade;").expect("Can't find Trade Class");
  let out = vec_to_java_arr(&env, trade_cls, &trades).expect("Failed to convert vec to array");

  out.l().expect("failed to convert out array to object").into_inner()
}

#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_limitBuy(env: JNIEnv, _class: JClass,  cli: JObject, req: JObject) -> jobject {
  let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client").expect("Failed to get client");
  let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime").expect("Failed to get runtime");

  let req = get_limit_request(&env, &req).expect("Failed to parse params");

  let resp = runtime.block_on(client.limit_buy(&req)).expect("Failed to get response");
  order_to_jobject(&env, resp).expect("Failed to convert response to order").into_inner()
}

#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_limitSell(env: JNIEnv, _class: JClass,  cli: JObject, req: JObject) -> jobject {
  let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client").expect("Failed to get client");
  let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime").expect("Failed to get runtime");

  let req = get_limit_request(&env, &req).expect("Failed to parse params");

  let resp = runtime.block_on(client.limit_sell(&req)).expect("Failed to get response");
  order_to_jobject(&env, resp).expect("Failed to convert response to order").into_inner()
}



#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_marketBuy(env: JNIEnv, _class: JClass,  cli: JObject, req: JObject) -> jobject {
  let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client").expect("Failed to get client");
  let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime").expect("Failed to get runtime");

  let req = get_market_request(&env, &req).expect("Failed to parse params");

  let resp = runtime.block_on(client.market_buy(&req)).expect("Failed to get response");
  order_to_jobject(&env, resp).expect("Failed to convert response to order").into_inner()
}

#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_marketSell(env: JNIEnv, _class: JClass,  cli: JObject, req: JObject) -> jobject {
  let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client").expect("Failed to get client");
  let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime").expect("Failed to get runtime");

  let req = get_market_request(&env, &req).expect("Failed to parse params");

  let resp = runtime.block_on(client.market_sell(&req)).expect("Failed to get response");
  order_to_jobject(&env, resp).expect("Failed to convert response to order").into_inner()
}




fn get_long_nullable(
  env: &JNIEnv,
  obj: &JObject,
  field: &str,
) -> Result<Option<u64>, String> {
  let f = get_field(env, obj, field,  "J")?;
  match f {
    None => Ok(None),
    Some(f) => Ok(Some(f.j().expect(format!("{} not long", field).as_str()) as u64))
  }
}

fn get_paginator(
  env: &JNIEnv,
  paginator: &JObject
) -> Result<Paginator, String> {
  let start_time = get_long_nullable(env, paginator, "startTime")?;
  let end_time = get_long_nullable(env, paginator, "endTime")?;
  let limit = get_long_nullable(env, paginator, "limit")?;
  let before = get_string(env, paginator, "before").expect("No before field");
  let after = get_string(env, paginator, "after").expect("No after field");

  Ok(Paginator {
    start_time: start_time.map(|v| v as u64 ),
    end_time: end_time.map(|v| v as u64 ),
    limit: limit.map(|v| v as u64 ),
    before,
    after
  })
}

fn interval_from_string(
  str: String
) -> Result<Interval, String> {
  match str.as_str() {
    "OneMinute" => Ok(Interval::OneMinute),
    "ThreeMinutes" => Ok(Interval::ThreeMinutes),
    "FiveMinutes" => Ok(Interval::FiveMinutes),
    "FifteenMinutes" => Ok(Interval::FifteenMinutes),
    "ThirtyMinutes" => Ok(Interval::ThirtyMinutes),
    "OneHour" => Ok(Interval::OneHour),
    "TwoHours" => Ok(Interval::TwoHours),
    "FourHours" => Ok(Interval::FourHours),
    "SixHours" => Ok(Interval::SixHours),
    "EightHours" => Ok(Interval::EightHours),
    "TwelveHours" => Ok(Interval::TwelveHours),
    "OneDay" => Ok(Interval::OneDay),
    "ThreeDays" => Ok(Interval::ThreeDays),
    "OneWeek" => Ok(Interval::OneWeek),
    "OneMonth" => Ok(Interval::OneMonth),
    _ => Err(format!("Invalid interval string {}", str))
  }
}

fn get_historic_trades_request(
  env: &JNIEnv,
  trades_req: &JObject
) -> errors::Result<GetHistoricTradesRequest> {
  let market_pair = get_string(env, trades_req, "market").expect("Can't find market field").expect("market must be non-null");
  let paginator = get_field(env, trades_req, "paginator", "Lio/nash/openlimits/Paginator;").expect("paginator field not found").map(|i|i.l().expect("Paginator not object"));
  let paginator = match paginator {
    None => None,
    Some(paginator) => {
      if paginator.is_null() {
        None
      } else {
        Some(get_paginator(env, &paginator).expect("Failed to parse paginator"))
      }
    }
  };
  
  Ok(GetHistoricTradesRequest {
    market_pair,
    paginator
  })
}

fn get_historic_rates_request(
  env: &JNIEnv,
  hist_req: &JObject
) -> errors::Result<GetHistoricRatesRequest> {
  let market_pair = get_string(env, hist_req, "market").expect("Can't find market field").expect("market must be non-null");
  let interval = get_string(env, hist_req, "interval").expect("Can't find interval field").expect("interval must be non-null");
  let paginator = get_field(env, hist_req, "paginator", "Lio/nash/openlimits/Paginator;").expect("paginator field not found").map(|i|i.l().expect("Paginator not object"));
  let paginator = match paginator {
    None => None,
    Some(paginator) => {
      if paginator.is_null() {
        None
      } else {
        Some(get_paginator(env, &paginator).expect("Failed to parse paginator"))
      }
    }
  };

  Ok(GetHistoricRatesRequest {
    market_pair,
    interval: interval_from_string(interval).unwrap(),
    paginator
  })

}

fn get_options_nash_credentials(
  env: &JNIEnv,
  nash: &JObject,
) -> Result<Option<NashCredentials>, String> {
  let credentials_opt = get_field(&env, nash, "credentials",  "Lio/nash/openlimits/NashCredentials;")?;
  
  let credentials =match credentials_opt {
    Some(c) => c.l().expect("Credentials object not of NashCredentials"),
    None => return Ok(None)
  };

  let secret = get_string(&env, &credentials, "secret")?.expect("Missing field secret");
  let session = get_string(&env, &credentials, "session")?.expect("Missing field session");

  Ok(Some(NashCredentials {
    secret,
    session
  }))
}

fn get_long_default_with_default(
  env: &JNIEnv,
  obj: &JObject,
  field: &str,
  def: i64
) -> Result<u64, String> {
  let f = get_field(env, obj, field,  "J")?;
  Ok(f.unwrap_or(JValue::Long(def)).j().expect(format!("{} not long", field).as_str()) as u64)
}

fn get_limit_request(
  env: &JNIEnv,
  req: &JObject,
) -> Result<OpenLimitOrderRequest, String> {
  let size = get_string(env, req, "size")?.expect("Missing field size on limit request params");
  let price = get_string(env, req, "price")?.expect("Missing field price on limit request params");
  let market_pair = get_string(env, req, "market")?.expect("Missing field market on limit request params");
  let size = Decimal::from_str(size.as_str());
  let price = Decimal::from_str(price.as_str());

  match (size, price) {
    (Ok(size), Ok(price)) => Ok(
      OpenLimitOrderRequest {
        size,
        price,
        market_pair,
      }
    ),
    _ => Err(String::from("Failed to parse size of price"))
  }
}

fn get_market_request(
  env: &JNIEnv,
  req: &JObject,
) -> Result<OpenMarketOrderRequest, String> {
  let size = get_string(env, req, "size")?.expect("Missing field size on limit request params");
  let market_pair = get_string(env, req, "market")?.expect("Missing field market on limit request params");
  let size = Decimal::from_str(size.as_str());

  match size {
    Ok(size) => Ok(
      OpenMarketOrderRequest {
        size,
        market_pair,
      }
    ),
    _ => Err(String::from("Failed to parse size of price"))
  }
}

fn get_options_nash(
  env: &JNIEnv,
  nash: &JObject,
) -> Result<InitAnyExchange, String> {
  let credentials = get_options_nash_credentials(env, nash)?;
  let client_id = get_long_default_with_default(env, nash, "clientId", 0)?;
  let environment = get_string(env, nash, "environment")?;
  let timeout = get_long_default_with_default(env, nash, "timeout", 1000)?;

  let environment = match environment {
    Some(r) => match r.as_str() {
      "production" => Environment::Production,
      "sandbox" => Environment::Sandbox,
      _ => return Err(format!("Invalid environment {}", r))
    },
    None => return Err(format!("Missing environment"))
  };

  Ok(
    InitAnyExchange::Nash(
      NashParameters {
        credentials,
        client_id,
        environment,
        timeout
      }
    )
  )
}

fn get_options_binance_credentials(
  env: &JNIEnv,
  binance: &JObject,
) -> Result<Option<BinanceCredentials>, String> {
  
  let credentials_opt = get_field(&env, binance, "credentials",  "Lio/nash/openlimits/BinanceCredentials;")?;
  let credentials = match credentials_opt {
    Some(c) => c.l().unwrap(),
    None => return Ok(None)
  };

  let api_key = get_string(&env, &credentials, "apiKey")?.unwrap();
  let api_secret = get_string(&env, &credentials, "apiSecret")?.unwrap();
  Ok(Some(BinanceCredentials {
    api_key,
    api_secret
  }))
}

fn get_options_binance(
  env: &JNIEnv,
  binance: &JObject,
) -> Result<InitAnyExchange, String> {
  let credentials = get_options_binance_credentials(env, binance)?;
  let sandbox = get_field(env, binance, "sandbox",  "Z")?.unwrap().z().unwrap();
  Ok(
    InitAnyExchange::Binance(
      BinanceParameters {
        credentials,
        sandbox
      }
    )
  )
}

fn get_options(
  env: &JNIEnv,
  opts: &JObject,
) -> Result<InitAnyExchange, String> {
  let nash = get_field(&env, opts, "nash",  "Lio/nash/openlimits/NashConfig;")?.unwrap().l().unwrap();
  let binance = get_field(&env, opts, "binance",  "Lio/nash/openlimits/BinanceConfig;")?.unwrap().l().unwrap();
  match (!nash.is_null(), !binance.is_null()) {
    (true, _) => get_options_nash(&env, &nash),
    (_, true) => get_options_binance(&env, &binance),
    // (_, Ok(binance)) => {},
    _ => Err(String::from("Invalid config, nash and binance field both null"))
  }
}