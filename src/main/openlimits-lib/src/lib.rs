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
      OrderType,
      AskBid,
      OpenLimitOrderRequest,
      OrderStatus,
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
      websocket::{Subscription, OpenLimitsWebsocketMessage}
  }
};
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


fn get_object<'a>(env: &'a JNIEnv, obj: &'a JObject, field: &str, t: &str) -> Result<Option<JObject<'a>>, String> {
  let f = get_field(env, obj, field, t)?;
  match f {
    Some(s) => match s.l() {
      Ok(obj) => {
        if obj.is_null() {
          Ok(None)
        } else {
          Ok(Some(obj))
        }
      },
      _ => Err(format!("Field {} not an object", field))
    },
    None => Ok(None)
  }
}

fn get_object_not_null<'a>(env: &'a JNIEnv, obj: &'a JObject, field: &str, t: &str) -> Result<JObject<'a>, String> {
  match get_object(env, obj, field, t)? {
    Some(o) => Ok(o),
    None => Err(format!("Could not find non-null field {}", field))
  }
}

fn get_string_non_null(env: &JNIEnv, obj: &JObject, field: &str) -> Result<String, String> {
  match get_string(env, obj, field)? {
    Some(s) => Ok(s),
    _ => Err(format!("Could not find non-null field {}", field))
  }
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


fn bidask_to_jobject<'a>(env: &JNIEnv<'a>, resp: AskBid) -> errors::Result<JObject<'a>> {
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

  let asks = vec_to_jobject(env, "Lio/nash/openlimits/AskBid;", resp.asks, bidask_to_jobject)?;
  let bids = vec_to_jobject(env, "Lio/nash/openlimits/AskBid;", resp.bids, bidask_to_jobject)?;

  let ctor_args = vec![
    asks.into(),
    bids.into(),
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

fn string_option_to_null(v: Option<JString>) -> JValue {
  match v {
    None => JValue::Object(JObject::null()),
    Some(s) => JValue::Object(s.into())
  }
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

fn order_type_to_string(typ: OrderType) -> &'static str {
  match typ {
    OrderType::Limit => "Limit",
    OrderType::Market => "Market",
    OrderType::StopLimit => "StopLimit",
    OrderType::StopMarket => "StopMarket",
    OrderType::Unknown => "Unknown",
  }
}
fn order_status_to_string(typ: OrderStatus) -> &'static str {
  match typ {
    OrderStatus::New => "New",
    OrderStatus::PartiallyFilled => "PartiallyFilled",
    OrderStatus::Filled => "Filled",
    OrderStatus::Canceled => "Canceled",
    OrderStatus::PendingCancel => "PendingCancel",
    OrderStatus::Rejected => "Rejected",
    OrderStatus::Expired => "Expired",
    OrderStatus::Open => "Open",
    OrderStatus::Pending => "Pending",
    OrderStatus::Active => "Active",
  }
}

fn order_to_jobject<'a>(env: &JNIEnv<'a>, order: Order) -> errors::Result<JObject<'a>> {
  let cls_resp = env.find_class("Lio/nash/openlimits/Order;").expect("Failed to find Order class");
  let ctor_args = vec![
    env.new_string(order.id)?.into(),
    env.new_string(order.market_pair)?.into(),
    string_option_to_null(order.client_order_id.map(|s| env.new_string(s)).transpose()?),
    JValue::Long(order.created_at.unwrap_or_default() as i64),
    env.new_string(order_type_to_string(order.order_type))?.into(),
    side_to_string(env, order.side)?.into(),
    env.new_string(order_status_to_string(order.status))?.into(),
    env.new_string(order.size.to_string())?.into(),
    string_option_to_null(order.price.map(|p| env.new_string(p.to_string())).transpose()?)
  ];
  env.new_object(cls_resp, "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JLjava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V", &ctor_args)
}

fn vec_to_jobject<'a, T, F>(env: &JNIEnv<'a>, cls: &str, entries: Vec<T>, f: F) -> errors::Result<JObject<'a>>
  where F: Fn(&JNIEnv<'a>,T) -> errors::Result<JObject<'a>> {
  let pairs_maybe: errors::Result<Vec<_>> = entries.into_iter().map(|v| f(env, v)).collect();
  let pairs = pairs_maybe?;
  let pairs_cls = env.find_class(cls)?;

  let out = vec_to_java_arr(&env, pairs_cls, &pairs)?;
  out.l()
}


fn order_cancelled_to_jobject<'a>(env: &JNIEnv<'a>, order: OrderCanceled) -> errors::Result<JObject<'a>> {
  let cls_resp = env.find_class("Lio/nash/openlimits/OrderCanceled;").expect("Failed to find OrderCanceled class");
  let ctor_args = vec![
    env.new_string(order.id)?.into(),
  ];
  env.new_object(cls_resp, "(Ljava/lang/String;)V", &ctor_args)
}

fn balance_to_jobject<'a>(env: &JNIEnv<'a>, balance: Balance) -> errors::Result<JObject<'a>> {
  let cls_resp = env.find_class("Lio/nash/openlimits/Balance;").expect("Failed to find Balance class");
  let ctor_args = vec![
    env.new_string(balance.asset)?.into(),
    env.new_string(balance.total.to_string())?.into(),
    env.new_string(balance.free.to_string())?.into(),
  ];

  env.new_object(cls_resp, "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V", &ctor_args)
}

fn market_pair_to_jobject<'a>(env: &JNIEnv<'a>, pair: MarketPair) -> errors::Result<JObject<'a>> {
  let cls_resp = env.find_class("Lio/nash/openlimits/MarketPair;").expect("Failed to find MarketPair class");

  let ctor_args = vec![
    env.new_string(pair.base)?.into(),
    env.new_string(pair.quote)?.into(),
    env.new_string(pair.symbol)?.into(),
    env.new_string(pair.base_increment.to_string())?.into(),
    env.new_string(pair.quote_increment.to_string())?.into(),
  ];

  env.new_object(cls_resp, "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V", &ctor_args)
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

  env.set_rust_field(cli, "_config", init_params).unwrap();
  env.set_rust_field(cli, "_client", client).unwrap();
  env.set_rust_field(cli, "_runtime", runtime).unwrap();
  1
}

#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_subscribe(env: JNIEnv, _class: JClass,  cli: JObject, subs: JObject, handler: JObject) -> jint {
  let jvm = env.get_java_vm().expect("Failed to get java VM");

  let init_params: MutexGuard<InitAnyExchange> = env.get_rust_field(cli, "_config").unwrap();
  let init_params = init_params.clone();
  let subs = get_subscriptions(&env, &subs).expect("Failed to convert subscriptions");
  let handler = env.new_global_ref(handler).expect("Failed to create global ref");
  std::thread::spawn(move || {
    let subs = subs;
    let init_params = init_params.clone();
    let env = jvm.attach_current_thread().expect("Failed to attach thread");

    let handler = handler.as_obj();
    let mut rt = tokio::runtime::Builder::new()
                .basic_scheduler()
                .enable_all()
                .build().expect("Could not create Tokio runtime");
    let mut client: OpenLimitsWs<AnyWsExchange> = rt.block_on(OpenLimitsWs::instantiate(init_params));


    for sub in subs {
      rt.block_on(client.subscribe(sub));
    }
    loop {
      let msg = match rt.block_on(client.next()) {
        Some(Ok(msg)) => msg,
        Some(Err(e)) => panic!(e),
        None => {
          continue;
        }
      };
      match msg {
        OpenLimitsWebsocketMessage::OrderBook(orderbook) => {
          match orderbook_resp_to_jobject(&env, orderbook) {
            Ok(orderbook) => {
              env.call_method(handler, "onOrderbook", "(Lio/nash/openlimits/OrderbookResponse;)V", &[orderbook.into()]);
            },
            Err(e) => {
              println!("Failed to parse orderbookmessage: {}", e);
            }
          }
        },
        msg => {
          println!("Ignoring {:?}", msg);
        }
      };
    }
  });


  1
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
  let out = vec_to_jobject(&env, "Lio/nash/openlimits/Candle;", resp, candle_to_jobject).expect("Failed to convert result to Java");
  out.into_inner()
}


#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_getHistoricTrades(env: JNIEnv, _class: JClass,  cli: JObject, trades_req: JObject) -> jobject {
  let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client").expect("Failed to get client");
  let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime").expect("Failed to get runtime");

  let req = get_historic_trades_request(&env, &trades_req).expect("Failed to parse params");

  let resp = runtime.block_on(client.get_historic_trades(&req)).expect("Failed to get response");
  let out = vec_to_jobject(&env, "Lio/nash/openlimits/Trade;", resp, trade_to_jobject).expect("Failed to convert result to Java");
  out.into_inner()
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

#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_getAllOpenOrders(env: JNIEnv, _class: JClass,  cli: JObject) -> jobject {
  let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client").expect("Failed to get client");
  let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime").expect("Failed to get runtime");

  let resp = runtime.block_on(client.get_all_open_orders()).expect("Failed to get response");

  let out = vec_to_jobject(&env, "Lio/nash/openlimits/Order;", resp, order_to_jobject).expect("Failed to convert result to Java");
  out.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_getOrderHistory(env: JNIEnv, _class: JClass,  cli: JObject, req: JObject) -> jobject {
  let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client").expect("Failed to get client");
  let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime").expect("Failed to get runtime");
  let req = get_order_history_request(&env, &req).expect("Failed to parse params");

  let resp = runtime.block_on(client.get_order_history(&req)).expect("Failed to get response");
  let out = vec_to_jobject(&env, "Lio/nash/openlimits/Order;", resp, order_to_jobject).expect("Failed to convert result to Java");
  out.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_getOrder(env: JNIEnv, _class: JClass,  cli: JObject, req: JObject) -> jobject {
  let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client").expect("Failed to get client");
  let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime").expect("Failed to get runtime");
  let req = get_order_request(&env, &req).expect("Failed to parse params");

  let resp = runtime.block_on(client.get_order(&req)).expect("Failed to get response");

  order_to_jobject(&env, resp).expect("Failed to convert response to order").into_inner()
}

#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_getTradeHistory(env: JNIEnv, _class: JClass,  cli: JObject, req: JObject) -> jobject {
  let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client").expect("Failed to get client");
  let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime").expect("Failed to get runtime");
  let req = get_trade_history_request(&env, &req).expect("Failed to parse params");

  let resp = runtime.block_on(client.get_trade_history(&req)).expect("Failed to get response");
  let out = vec_to_jobject(&env, "Lio/nash/openlimits/Trade;", resp, trade_to_jobject).expect("Failed to convert result to Java");
  out.into_inner()
}

#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_getAccountBalances(env: JNIEnv, _class: JClass,  cli: JObject, req: JObject) -> jobject {
  let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client").expect("Failed to get client");
  let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime").expect("Failed to get runtime");
  let req = match req.is_null() {
    true => None,
    false => Some(get_paginator(&env, &req))
  };
  let paginator = req.transpose().expect("Invalid paginator object");

  let resp = runtime.block_on(client.get_account_balances(paginator)).expect("Failed to get response");
  let out = vec_to_jobject(&env, "Lio/nash/openlimits/Balance;", resp, balance_to_jobject).expect("Failed to convert result to Java");
  out.into_inner()
}


#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_cancelOrder(env: JNIEnv, _class: JClass,  cli: JObject, req: JObject) -> jobject {
  let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client").expect("Failed to get client");
  let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime").expect("Failed to get runtime");
  let req = get_cancel_order_request(&env, &req).expect("Failed to parse params");

  let resp = runtime.block_on(client.cancel_order(&req)).expect("Failed to get response");

  order_cancelled_to_jobject(&env, resp).expect("Failed to convert response to order").into_inner()
}


#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_cancelAllOrders(env: JNIEnv, _class: JClass,  cli: JObject, req: JObject) -> jobject {
  let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client").expect("Failed to get client");
  let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime").expect("Failed to get runtime");
  let req = get_cancel_all_orders_request(&env, &req).expect("Failed to parse params");

  let resp = runtime.block_on(client.cancel_all_orders(&req)).expect("Failed to get response");

  let out = vec_to_jobject(&env, "Lio/nash/openlimits/OrderCanceled;", resp, order_cancelled_to_jobject).expect("Failed to convert result to Java");
  out.into_inner()
}


#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_receivePairs(env: JNIEnv, _class: JClass,  cli: JObject) -> jobject {
  let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client").expect("Failed to get client");
  let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime").expect("Failed to get runtime");

  
  let resp = runtime.block_on(client.retrieve_pairs()).expect("Failed to get response");
  let pairs_maybe: errors::Result<Vec<_>> = resp.into_iter().map(|v| market_pair_to_jobject(&env, v)).collect();
  let pairs = pairs_maybe.expect("Failed to convert Pairs to Java");
  let pairs_cls = env.find_class("Lio/nash/openlimits/MarketPair;").expect("Can't find MarketPair Class");

  let out = vec_to_java_arr(&env, pairs_cls, &pairs).expect("Failed to convert vec to array");
  out.l().expect("failed to convert out array to object").into_inner()
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

  Ok(
    Paginator {
      start_time: start_time.map(|v| v as u64 ),
      end_time: end_time.map(|v| v as u64 ),
      limit: limit.map(|v| v as u64 ),
      before,
      after
    }
  )
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

fn map_err(e: jni::errors::Error) -> String {
  format!("{}", e)
}

fn get_subscriptions(
  env: &JNIEnv,
  subs: &JObject
) -> Result<Vec<Subscription>, String> {
  let arr = subs.into_inner();
  let len = env.get_array_length(arr).map_err(map_err)?;
  let mut out: Vec<Subscription> = Vec::new();
  for i in 0..len {
    let sub = env.get_object_array_element(arr, i).map_err(map_err)?;
    out.push(get_subscription(env, &sub)?);
  }
  Ok(out)
}

fn get_subscription(
  env: &JNIEnv,
  sub: &JObject
) -> Result<Subscription, String> {
  
  match get_string_non_null(env, sub, "tag")?.as_str() {
    "OrderBook" => {
      let market = get_string_non_null(env, sub, "market")?;
      let depth = env.get_field(*sub, "depth", "J").map_err(map_err)?.j().map_err(map_err)?;
      
      Ok(Subscription::OrderBook(market, depth))
    },
    s => panic!("Invalid subscription type {}", s)
  }
}
fn get_cancel_order_request(
  env: &JNIEnv,
  req: &JObject
) -> Result<CancelOrderRequest, String> {
  let id = get_string_non_null(env, req, "id")?;
  let market_pair = get_string(env, req, "market")?;

  Ok(
    CancelOrderRequest {
      market_pair,
      id
    }
  )
}



fn get_cancel_all_orders_request(
  env: &JNIEnv,
  req: &JObject
) -> Result<CancelAllOrdersRequest, String> {
  let market_pair = get_string(env, req, "market")?;

  Ok(
    CancelAllOrdersRequest {
      market_pair,
    }
  )
}

fn get_historic_trades_request(
  env: &JNIEnv,
  trades_req: &JObject
) -> Result<GetHistoricTradesRequest, String> {
  let market_pair = get_string_non_null(env, trades_req, "market")?;

  let paginator = get_object(env, trades_req, "paginator", "Lio/nash/openlimits/Paginator;")?;
  let paginator = paginator.map(|paginator| get_paginator(env, &paginator)).transpose()?;

  Ok(
    GetHistoricTradesRequest {
      market_pair,
      paginator
    }
  )
}

fn get_order_history_request(
  env: &JNIEnv,
  req: &JObject
) -> Result<GetOrderHistoryRequest, String> {
  let market_pair = get_string(env, req, "market")?;

  let paginator = get_object(env, req, "paginator", "Lio/nash/openlimits/Paginator;")?;
  let paginator = paginator.map(|paginator| get_paginator(env, &paginator)).transpose()?;

  Ok(
    GetOrderHistoryRequest {
      paginator,
      market_pair
    }
  )
}

fn get_order_request(
  env: &JNIEnv,
  req: &JObject
) -> Result<GetOrderRequest, String> {
  let id = get_string_non_null(env, req, "id")?;
  let market_pair = get_string(env, req, "market")?;

  Ok(
    GetOrderRequest {
      id,
      market_pair
    }
  )
}

fn get_trade_history_request(
  env: &JNIEnv,
  hist_req: &JObject
) -> Result<TradeHistoryRequest, String> {
  let market_pair = get_string(env, hist_req, "market")?;
  let order_id = get_string(env, hist_req, "orderId")?;
  let paginator = get_object(env, hist_req, "paginator", "Lio/nash/openlimits/Paginator;")?;
  let paginator = paginator.map(|paginator| get_paginator(env, &paginator)).transpose()?;

  Ok(
    TradeHistoryRequest {
      market_pair,
      order_id,
      paginator
    }
  )
}

fn get_historic_rates_request(
  env: &JNIEnv,
  hist_req: &JObject
) -> Result<GetHistoricRatesRequest, String> {
  let market_pair = get_string_non_null(env, hist_req, "market")?;
  let interval = get_string_non_null(env, hist_req, "interval")?;

  let paginator = get_object(env, hist_req, "paginator", "Lio/nash/openlimits/Paginator;")?;
  let paginator = paginator.map(|paginator| get_paginator(env, &paginator)).transpose()?;

  Ok(
    GetHistoricRatesRequest {
      market_pair,
      interval: interval_from_string(interval)?,
      paginator
    }
  )
}

fn get_options_nash_credentials(
  env: &JNIEnv,
  nash: &JObject,
) -> Result<Option<NashCredentials>, String> {
  let credentials = get_object(&env, nash, "credentials",  "Lio/nash/openlimits/NashCredentials;")?;

  credentials.map(|credentials| {
    let secret = get_string_non_null(&env, &credentials, "secret")?;
    let session = get_string_non_null(&env, &credentials, "session")?;
    Ok(
      NashCredentials {
        secret,
        session
      }
    )
  }).transpose()
}


fn get_limit_request(
  env: &JNIEnv,
  req: &JObject,
) -> Result<OpenLimitOrderRequest, String> {
  let size = get_string_non_null(env, req, "size")?;
  let price = get_string_non_null(env, req, "price")?;
  let market_pair = get_string_non_null(env, req, "market")?;
  let size = Decimal::from_str(size.as_str()).map_err(|e|e.to_string())?;
  let price = Decimal::from_str(price.as_str()).map_err(|e|e.to_string())?;

  Ok(
    OpenLimitOrderRequest {
      size,
      price,
      market_pair,
    }
  )
}

fn get_market_request(
  env: &JNIEnv,
  req: &JObject,
) -> Result<OpenMarketOrderRequest, String> {
  let size = get_string_non_null(env, req, "size")?;
  let market_pair = get_string_non_null(env, req, "market")?;
  let size = Decimal::from_str(size.as_str()).map_err(|e|e.to_string())?;

  Ok(
    OpenMarketOrderRequest {
      size,
      market_pair,
    }
  )
}

fn get_options_nash(
  env: &JNIEnv,
  nash: &JObject,
) -> Result<InitAnyExchange, String> {
  let credentials = get_options_nash_credentials(env, nash)?;
  let client_id = get_long_default_with_default(env, nash, "clientId", 0)?;
  let environment = get_string_non_null(env, nash, "environment")?;
  let timeout = get_long_default_with_default(env, nash, "timeout", 1000)?;

  let environment = match environment.as_str() {
    "production" => Environment::Production,
    "sandbox" => Environment::Sandbox,
    r => return Err(format!("Invalid environment {}", r))
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
  
  let credentials_opt = get_object(&env, binance, "credentials",  "Lio/nash/openlimits/BinanceCredentials;")?;

  credentials_opt.map(|credentials| {
    let api_key = get_string_non_null(&env, &credentials, "apiKey")?;
    let api_secret = get_string_non_null(&env, &credentials, "apiSecret")?;
    Ok(
      BinanceCredentials {
        api_key,
        api_secret
      }
    )
  }).transpose()
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
  let nash = get_object(&env, opts, "nash",  "Lio/nash/openlimits/NashConfig;")?;
  let binance = get_object(&env, opts, "binance",  "Lio/nash/openlimits/BinanceConfig;")?;
  match (nash, binance) {
    (Some(nash), _) => get_options_nash(&env, &nash),
    (_, Some(binance)) => get_options_binance(&env, &binance),
    // (_, Ok(binance)) => {},
    _ => Err(String::from("Invalid config, nash and binance field both null"))
  }
}