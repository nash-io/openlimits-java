use jni;
use jni::{errors, JNIEnv};
use jni::objects::{JClass, JValue, JObject, JString};
use jni::sys::{jsize, jobject};
use rust_decimal::{Decimal, prelude::ToPrimitive};
use rust_decimal::prelude::*;
use chrono::Duration;
use openlimits::{
  exchange::{OpenLimits, ExchangeAccount, ExchangeMarketData}, 
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
      TimeInForce,
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
      websocket::{Subscription, OpenLimitsWebSocketMessage, WebSocketResponse}
  }
};
use tokio::stream::StreamExt;
use std::sync::MutexGuard;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OpenlimitsJavaError {
  #[error("Invalid argument {0}")]
  InvalidArgument(String),
  #[error("{0}")]
  OpenLimitsError(#[from] openlimits::errors::OpenLimitError),
  #[error("{0}")]
  JNIError(#[from] jni::errors::Error)
}

fn map_openlimits_error_class(err: &openlimits::errors::OpenLimitError) -> &'static str {
  match err {
    openlimits::errors::OpenLimitError::BinanceError(_) => "Lio/nash/openlimits/BinanceError;",
    openlimits::errors::OpenLimitError::CoinbaseError(_) => "Lio/nash/openlimits/CoinbaseError;",
    openlimits::errors::OpenLimitError::NashProtocolError(_) => "Lio/nash/openlimits/NashProtocolError;",
    openlimits::errors::OpenLimitError::MissingImplementation(_) => "Lio/nash/openlimits/MissingImplementation;",
    openlimits::errors::OpenLimitError::AssetNotFound() => "Lio/nash/openlimits/AssetNotFound;",
    openlimits::errors::OpenLimitError::NoApiKeySet() => "Lio/nash/openlimits/NoApiKeySet;",
    openlimits::errors::OpenLimitError::InternalServerError() => "Lio/nash/openlimits/InternalServerError;",
    openlimits::errors::OpenLimitError::ServiceUnavailable() => "Lio/nash/openlimits/ServiceUnavailable;",
    openlimits::errors::OpenLimitError::Unauthorized() => "Lio/nash/openlimits/Unauthorized;",
    openlimits::errors::OpenLimitError::SymbolNotFound() => "Lio/nash/openlimits/SymbolNotFound;",
    openlimits::errors::OpenLimitError::SocketError() => "Lio/nash/openlimits/SocketError;",
    openlimits::errors::OpenLimitError::GetTimestampFailed() => "Lio/nash/openlimits/GetTimestampFailed;",
    openlimits::errors::OpenLimitError::ReqError(_) => "Lio/nash/openlimits/ReqError;",
    openlimits::errors::OpenLimitError::InvalidHeaderError(_) => "Lio/nash/openlimits/InvalidHeaderError;",
    openlimits::errors::OpenLimitError::InvalidPayloadSignature(_) => "Lio/nash/openlimits/InvalidPayloadSignature;",
    openlimits::errors::OpenLimitError::IoError(_) => "Lio/nash/openlimits/IoError;",
    openlimits::errors::OpenLimitError::PoisonError() => "Lio/nash/openlimits/PoisonError;",
    openlimits::errors::OpenLimitError::JsonError(_) => "Lio/nash/openlimits/JsonError;",
    openlimits::errors::OpenLimitError::ParseFloatError(_) => "Lio/nash/openlimits/ParseFloatError;",
    openlimits::errors::OpenLimitError::UrlParserError(_) => "Lio/nash/openlimits/UrlParserError;",
    openlimits::errors::OpenLimitError::Tungstenite(_) => "Lio/nash/openlimits/Tungstenite;",
    openlimits::errors::OpenLimitError::TimestampError(_) => "Lio/nash/openlimits/TimestampError;",
    openlimits::errors::OpenLimitError::UnkownResponse(_) => "Lio/nash/openlimits/UnkownResponse;",
    openlimits::errors::OpenLimitError::NotParsableResponse(_) => "Lio/nash/openlimits/NotParsableResponse;",
    openlimits::errors::OpenLimitError::MissingParameter(_) => "Lio/nash/openlimits/MissingParameter;",
    openlimits::errors::OpenLimitError::WebSocketMessageNotSupported() => "Lio/nash/openlimits/WebSocketMessageNotSupported;",
  }
}

fn map_error_to_error_class(err: &OpenlimitsJavaError) -> &'static str {
  match err {
    OpenlimitsJavaError::InvalidArgument(_) => "Ljava/lang/IllegalArgumentException;",
    OpenlimitsJavaError::OpenLimitsError(e) => map_openlimits_error_class(e),
    OpenlimitsJavaError::JNIError(e) => {
      match e {
        jni::errors::Error::NullPtr(_) => "Ljava/lang/NullPointerException;",
        jni::errors::Error::NullDeref(_) => "Ljava/lang/NullPointerException;",
        _ => "Ljava/lang/Exception;"
      }
    }
  }
}

type OpenLimitsJavaResult<T> = Result<T, OpenlimitsJavaError>;

static EVENT_HANDLER_CLS_NAME: &str = "Lio/nash/openlimits/ExchangeClient;";
static ASK_BID_CLS_NAME: &str = "Lio/nash/openlimits/AskBid;";
static BALANCE_CLS_NAME: &str = "Lio/nash/openlimits/Balance;";
static BINANCE_CONFIG_CLS_NAME: &str = "Lio/nash/openlimits/BinanceConfig;";
static BINANCE_CREDENTIALS_CLS_NAME: &str = "Lio/nash/openlimits/BinanceCredentials;";
static CANDLE_CLS_NAME: &str = "Lio/nash/openlimits/Candle;";
static MARKET_PAIR_CLS_NAME: &str = "Lio/nash/openlimits/MarketPair;";
static NASH_CONFIG_CLS_NAME: &str = "Lio/nash/openlimits/NashConfig;";
static NASH_CREDENTIALS_CLS_NAME: &str = "Lio/nash/openlimits/NashCredentials;";
static ORDER_CLS_NAME: &str = "Lio/nash/openlimits/Order;";
static ORDERBOOK_RESPONSE_CLS_NAME: &str = "Lio/nash/openlimits/OrderbookResponse;";
static ORDER_CANCELED_CLS_NAME: &str = "Lio/nash/openlimits/OrderCanceled;";
static PAGINATOR_CLS_NAME: &str = "Lio/nash/openlimits/Paginator;";
static TICKER_CLS_NAME: &str = "Lio/nash/openlimits/Ticker;";
static TRADE_CLS_NAME: &str = "Lio/nash/openlimits/Trade;";

static STRING_CLS_NAME: &str = "Ljava/lang/String;";

fn decimal_to_jvalue<'a>(_env: &JNIEnv<'a>, s: Decimal) -> errors::Result<JValue<'a>> {
  Ok(JValue::Float(s.to_f32().unwrap_or_default()))
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
  let value = get_field(env, obj, field, STRING_CLS_NAME)?;
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
  let cls_bidask = env.find_class(ASK_BID_CLS_NAME)?;

  let ctor_args = &[
    decimal_to_jvalue(env, resp.price)?,
    decimal_to_jvalue(env, resp.qty)?,
  ];

  env.new_object(cls_bidask, "(FF)V", ctor_args)
}

fn vec_to_java_arr<'a>(env: &JNIEnv<'a>, cls: JClass, v: &Vec<JObject<'a>>) -> errors::Result<JValue<'a>> {
  let arr = env.new_object_array(v.len() as i32, cls, JObject::null())?;
  for i in 0..v.len() {
    env.set_object_array_element(arr, i as jsize, v[i])?;
  }
  Ok(JValue::from(arr))
}

fn orderbook_resp_to_jobject<'a>(env: &JNIEnv<'a>, resp: OrderBookResponse, market_pair: JValue) -> errors::Result<JObject<'a>> {
  let cls_resp = env.find_class(ORDERBOOK_RESPONSE_CLS_NAME)?;

  let asks = vec_to_jobject(env, ASK_BID_CLS_NAME, resp.asks, bidask_to_jobject)?;
  let bids = vec_to_jobject(env, ASK_BID_CLS_NAME, resp.bids, bidask_to_jobject)?;

  let ctor_args = &[
    market_pair,
    asks.into(),
    bids.into(),
    JValue::Long(resp.last_update_id.unwrap_or_default() as i64)
  ];
  env.new_object(cls_resp, "(Ljava/lang/String;[Lio/nash/openlimits/AskBid;[Lio/nash/openlimits/AskBid;J)V", ctor_args)
}


fn candle_to_jobject<'a>(env: &JNIEnv<'a>, candle: Candle) -> errors::Result<JObject<'a>> {
  let cls_candle = env.find_class(CANDLE_CLS_NAME)?;
  
  let ctor_args = &[
    JValue::Long(candle.time as i64),
    decimal_to_jvalue(env, candle.low)?, 
    decimal_to_jvalue(env, candle.high)?, 
    decimal_to_jvalue(env, candle.open)?, 
    decimal_to_jvalue(env, candle.close)?, 
    decimal_to_jvalue(env, candle.volume)?
  ];

  env.new_object(cls_candle, "(JFFFFF)V", ctor_args)
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
fn liquidity_to_string<'a>(env: &JNIEnv<'a>, s: Liquidity) -> errors::Result<JObject<'a>> {
  let s = match s {
    Liquidity::Maker => "Maker",
    Liquidity::Taker => "Taker",
  };
  Ok(string_to_jstring(env, String::from(s))?.into())
}

fn string_option_to_null(v: Option<JString>) -> JValue {
  match v {
    None => JValue::Object(JObject::null()),
    Some(s) => JValue::Object(s.into())
  }
}


fn trade_to_jobject<'a>(env: &JNIEnv<'a>, trade: Trade) -> errors::Result<JObject<'a>> {
  let cls_trade = env.find_class(TRADE_CLS_NAME)?;
  let ctor_args = &[
    env.new_string(trade.id)?.into(),
    env.new_string(trade.order_id)?.into(),
    env.new_string(trade.market_pair)?.into(),
    decimal_to_jvalue(env, trade.price)?,
    decimal_to_jvalue(env, trade.qty)?,
    trade.fees.map_or(Ok(JValue::Float(0.0)), |f| decimal_to_jvalue(env, f))?,
    side_to_string(env, trade.side)?.into(),
    trade.liquidity.map_or(Ok(JObject::null()), |l| liquidity_to_string(env, l))?.into(),
    JValue::Long(trade.created_at as i64)
  ];

  env.new_object(cls_trade, "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;FFFLjava/lang/String;Ljava/lang/String;J)V", ctor_args)
}

fn ticker_to_jobject<'a>(env: &JNIEnv<'a>, resp: Ticker) -> errors::Result<JObject<'a>> {
  let cls_resp = env.find_class(TICKER_CLS_NAME)?;
  let ctor_args = &[
    decimal_to_jvalue(env, resp.price.unwrap_or_default())?
  ];
  env.new_object(cls_resp, "(F)V", ctor_args)
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
  let cls_resp = env.find_class(ORDER_CLS_NAME)?;
  let ctor_args = &[
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
  env.new_object(cls_resp, "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;JLjava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V", ctor_args)
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
  let cls_resp = env.find_class(ORDER_CANCELED_CLS_NAME)?;
  let ctor_args = &[
    env.new_string(order.id)?.into(),
  ];
  env.new_object(cls_resp, "(Ljava/lang/String;)V", ctor_args)
}

fn balance_to_jobject<'a>(env: &JNIEnv<'a>, balance: Balance) -> errors::Result<JObject<'a>> {
  let cls_resp = env.find_class(BALANCE_CLS_NAME)?;
  let ctor_args = &[
    env.new_string(balance.asset)?.into(),
    env.new_string(balance.total.to_string())?.into(),
    env.new_string(balance.free.to_string())?.into(),
  ];

  env.new_object(cls_resp, "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V", ctor_args)
}

fn market_pair_to_jobject<'a>(env: &JNIEnv<'a>, pair: MarketPair) -> errors::Result<JObject<'a>> {
  let cls_resp = env.find_class(MARKET_PAIR_CLS_NAME)?;

  let ctor_args = &[
    env.new_string(pair.base)?.into(),
    env.new_string(pair.quote)?.into(),
    env.new_string(pair.symbol)?.into(),
    env.new_string(pair.base_increment.to_string())?.into(),
    env.new_string(pair.quote_increment.to_string())?.into(),
  ];

  env.new_object(cls_resp, "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V", ctor_args)
}

enum SubthreadCmd {
  Sub(Subscription),
  Disconnect
}

enum JavaReportBackMsg {
  Disconnect,
  Message(OpenLimitsWebSocketMessage, String),
  Error(openlimits::errors::OpenLimitError)
}

fn init_ws(env: JNIEnv, _class: JClass, cli: JObject, init_params: InitAnyExchange) {
  let client = env.new_global_ref(cli).expect("Failed to create global ref");
  
  let (sub_request_tx, mut sub_rx) = tokio::sync::mpsc::unbounded_channel::<SubthreadCmd>();
  env.set_rust_field(cli, "_sub_tx", sub_request_tx).unwrap();
  let (msg_request_tx, msg_rx) = std::sync::mpsc::sync_channel::<JavaReportBackMsg>(100);


  let jvm = env.get_java_vm().expect("Failed to get java VM");
  std::thread::spawn(move|| {
    let env = jvm.attach_current_thread().expect("Failed to attach thread");
    let event_handler_cls = env.find_class(EVENT_HANDLER_CLS_NAME).unwrap();
    let on_trades = env.get_method_id(event_handler_cls, "onTrades", "(Lio/nash/openlimits/TradesResponse;)V").unwrap();
    let on_orderbook = env.get_method_id(event_handler_cls, "onOrderbook", "(Lio/nash/openlimits/OrderbookResponse;)V").unwrap();
    let on_error = env.get_method_id(event_handler_cls, "onError", "(Lio/nash/openlimits/OpenLimitsException;)V").unwrap();
    let on_disconnect = env.get_method_id(event_handler_cls, "onDisconnect", "()V").unwrap();
    let on_ping = env.get_method_id(event_handler_cls, "onPing", "()V").unwrap();
    
    loop {
      let msg = msg_rx.recv();
      let (msg, market_str) = match msg {
        Ok(JavaReportBackMsg::Disconnect) => {
          let res = env.call_method_unchecked(
            client.as_obj(),
            on_disconnect,
            jni::signature::JavaType::Primitive(jni::signature::Primitive::Void),
            &[]
          );
          if res.is_err() {
            panic!("Failed to do callback: {}", res.err().unwrap());
          }
          break;
        },
        Ok(JavaReportBackMsg::Error(err)) => {
          let s = map_openlimits_error_class(&err);
          let msg = format!("{:?}", err);
          let msg = env.new_string(msg).unwrap();
          let cls = env.find_class(s).unwrap();
          let inst = env.new_object(cls, "(Ljava/land/String)V", &[msg.into()]).unwrap();
          let res = env.call_method_unchecked(
            client.as_obj(),
            on_error,
            jni::signature::JavaType::Primitive(jni::signature::Primitive::Void),
            &[inst.into()]
          );
          if res.is_err() {
            panic!("Failed to do callback: {}", res.err().unwrap());
          }
          continue;
        },
        Ok(JavaReportBackMsg::Message(msg, market)) => (msg, market),
        Err(_) => {
          let res = env.call_method_unchecked(
            client.as_obj(),
            on_disconnect,
            jni::signature::JavaType::Primitive(jni::signature::Primitive::Void),
            &[]
          );
          if res.is_err() {
            panic!("Failed to do callback: {}", res.err().unwrap());
          }
          break;
        },
      };

      match msg {
        OpenLimitsWebSocketMessage::Trades(trades) => {
          match vec_to_jobject(&env, TRADE_CLS_NAME, trades.clone(), trade_to_jobject) {
            Ok(trades) => {
              let res = env.call_method_unchecked(
                client.as_obj(),
                on_trades,
                jni::signature::JavaType::Primitive(jni::signature::Primitive::Void),
                &[trades.into()]
              );

              if res.is_err() {
                panic!("Failed to do callback: {}", res.err().unwrap());
              }
            },
            Err(e) => {
              panic!("failed to conert object: {}", e);
            }
          };
        },
        OpenLimitsWebSocketMessage::OrderBook(orderbook) => {
          let s = env.new_string(market_str).expect("failed to create a market strings");

          match orderbook_resp_to_jobject(&env, orderbook.clone(),s.into()) {
            Ok(orderbook) => {
              let res =env.call_method_unchecked(
                client.as_obj(),
                on_orderbook,
                jni::signature::JavaType::Primitive(jni::signature::Primitive::Void),
                &[orderbook.into()]
              );

              if res.is_err() {
                panic!("Failed to do callback: {}", res.err().unwrap());
              }
            },
            Err(e) => {
              panic!("failed to conert object: {}", e);
            }
          };
        },
        OpenLimitsWebSocketMessage::Ping => {
          let res = env.call_method_unchecked(
            client.as_obj(),
            on_ping,
            jni::signature::JavaType::Primitive(jni::signature::Primitive::Void),
            &[]
          );

          if res.is_err() {
            panic!("Failed to do callback: {}", res.err().unwrap());
          }
        },
        OpenLimitsWebSocketMessage::OrderBookDiff(orderbook) => {
          let s = env.new_string(market_str).expect("failed to create a market strings");

          match orderbook_resp_to_jobject(&env, orderbook.clone(),s.into()) {
            Ok(orderbook) => {
              let res =env.call_method_unchecked(
                client.as_obj(),
                on_orderbook,
                jni::signature::JavaType::Primitive(jni::signature::Primitive::Void),
                &[orderbook.into()]
              );

              if res.is_err() {
                panic!("Failed to do callback: {}", res.err().unwrap());
              }
            },
            Err(e) => {
              panic!("failed to conert object: {}", e);
            }
          };
        },
      };
    }
  });

  let jvm = env.get_java_vm().expect("Failed to get java VM");
  std::thread::spawn(move || {
    jvm.attach_current_thread().expect("Failed to attach thread");

    let mut rt = tokio::runtime::Builder::new()
                .basic_scheduler()
                .enable_all()
                .build().expect("Could not create Tokio runtime");

    let client: OpenLimitsWs<AnyWsExchange> = rt.block_on(OpenLimitsWs::instantiate(init_params.clone()));

    loop {
      let subcmd = sub_rx.next();
      let next_msg = rt.block_on(subcmd);

      match next_msg {
        Some(thread_cmd) => {
          match thread_cmd {
            SubthreadCmd::Disconnect => {
              msg_request_tx.clone().send(JavaReportBackMsg::Disconnect).unwrap();
              break;
            },
            SubthreadCmd::Sub(sub) => {
              let sub_reporter_tx = msg_request_tx.clone();
              let result = rt.block_on(client.subscribe(sub.clone(), move |resp| {
                let resp = match resp {
                  Ok(e) => e,
                  Err(err) => {
                    let err = match err {
                        openlimits::errors::OpenLimitError::UnkownResponse(e) => openlimits::errors::OpenLimitError::UnkownResponse(e.clone()),
                        openlimits::errors::OpenLimitError::NotParsableResponse(e) => openlimits::errors::OpenLimitError::NotParsableResponse(e.clone()),
                        openlimits::errors::OpenLimitError::MissingParameter(e) => openlimits::errors::OpenLimitError::MissingParameter(e.clone()),
                        openlimits::errors::OpenLimitError::AssetNotFound() => openlimits::errors::OpenLimitError::AssetNotFound(),
                        openlimits::errors::OpenLimitError::NoApiKeySet() => openlimits::errors::OpenLimitError::NoApiKeySet(),
                        openlimits::errors::OpenLimitError::InternalServerError() => openlimits::errors::OpenLimitError::InternalServerError(),
                        openlimits::errors::OpenLimitError::ServiceUnavailable() => openlimits::errors::OpenLimitError::ServiceUnavailable(),
                        openlimits::errors::OpenLimitError::Unauthorized() => openlimits::errors::OpenLimitError::Unauthorized(),
                        openlimits::errors::OpenLimitError::SymbolNotFound() => openlimits::errors::OpenLimitError::SymbolNotFound(),
                        openlimits::errors::OpenLimitError::SocketError() => openlimits::errors::OpenLimitError::SocketError(),
                        openlimits::errors::OpenLimitError::WebSocketMessageNotSupported() => openlimits::errors::OpenLimitError::WebSocketMessageNotSupported(),
                        openlimits::errors::OpenLimitError::GetTimestampFailed() => openlimits::errors::OpenLimitError::GetTimestampFailed(),
                        openlimits::errors::OpenLimitError::PoisonError() => openlimits::errors::OpenLimitError::PoisonError(),
                        _ => openlimits::errors::OpenLimitError::SocketError(),
                    };

                    sub_reporter_tx.send(JavaReportBackMsg::Error(err)).unwrap();
                    return;
                  }
                };
                let resp = match resp {
                  WebSocketResponse::Generic(msg) => msg,
                  _ => {
                    return;
                  }
                };
                let market = match sub.clone() {
                  Subscription::Ticker(e) => e.clone(),
                  Subscription::OrderBookUpdates(e) => e.clone(),
                  Subscription::Trades(e) => e.clone(),
                  _ => String::from("Unknown")
                };
                sub_reporter_tx.send(JavaReportBackMsg::Message(resp.clone(), market)).unwrap();
              }));

              match result {
                Err(err) => {
                  let err_reporter_tx = msg_request_tx.clone();
                  err_reporter_tx.send(JavaReportBackMsg::Error(err)).unwrap();
                  msg_request_tx.clone().send(JavaReportBackMsg::Disconnect).unwrap();
                  break;
                },
                _ => {}
              };
            },
          }
        },
        None => {}
      }
    }
  });
}

#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_init(env: JNIEnv, _class: JClass, cli: JObject, conf: JObject) {
  let call = move || -> OpenLimitsJavaResult<()> {
    let init_params = get_options(&env, &conf).map_err(OpenlimitsJavaError::InvalidArgument)?;
    let ws_params = init_params.clone();
    let mut runtime = tokio::runtime::Builder::new().basic_scheduler().enable_all().build().expect("Failed to set up Tokio runtime");
    
    let client_future = OpenLimits::instantiate(init_params.clone());
    let client: AnyExchange = runtime.block_on(client_future);

    env.set_rust_field(cli, "_config", init_params)?;
    env.set_rust_field(cli, "_client", client)?;
    env.set_rust_field(cli, "_runtime", runtime)?;
    init_ws(env, _class, cli, ws_params);
    Ok(())
  };

  handle_void_result(env, call());
}

#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_subscribe(env: JNIEnv, _class: JClass,  cli: JObject, sub: JObject) {
  let call = move || -> OpenLimitsJavaResult<()> {
    let sub_request_tx: MutexGuard<tokio::sync::mpsc::UnboundedSender<SubthreadCmd>> = env.get_rust_field(cli, "_sub_tx")?;
    let sub = get_subscription(&env, &sub).map_err(OpenlimitsJavaError::InvalidArgument)?;
    match sub_request_tx.send(SubthreadCmd::Sub(sub)) {
      Err(_) => panic!("Failed to send subscribe cmd"),
      _ => Ok(())
    }
  };

  handle_void_result(env, call());
}

#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_disconnect(env: JNIEnv, _class: JClass,  cli: JObject) {
  let call = move || -> OpenLimitsJavaResult<()> {
    let sub_request_tx: MutexGuard<tokio::sync::mpsc::UnboundedSender<SubthreadCmd>> = env.get_rust_field(cli, "_sub_tx")?;
    match sub_request_tx.send(SubthreadCmd::Disconnect) {
      Err(_) => panic!("Failed to send subscribe cmd"),
      _ => Ok(())
    }
  };
  handle_void_result(env, call());
}

fn handle_jobject_result(env: JNIEnv, result: OpenLimitsJavaResult<JObject>) -> jobject {
  match result {
    Ok(obj) => obj.into_inner(),
    Err(err) => {
      let s = map_error_to_error_class(&err);
      let msg = format!("{:?}", err);
      env.throw_new(env.find_class(s).expect("Failed to find class"), msg).expect("Failed to raise exception");
      JObject::null().into_inner()
    }
  }
}

fn handle_void_result(env: JNIEnv, result: OpenLimitsJavaResult<()>) {
  match result {
    Ok(_) => {},
    Err(err) => {
      let s = map_error_to_error_class(&err);
      let msg = format!("{:?}", err);
      env.throw_new(env.find_class(s).expect("Failed to find class"), msg).expect("Failed to raise exception");
    }
  }
}

#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_orderBook(env: JNIEnv, _class: JClass,  cli: JObject, market: JString) -> jobject {
  let call = move || -> OpenLimitsJavaResult<JObject> {
    let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client")?;

    let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime")?;
    let market_pair_jstring = env.get_string(market)?;
    let market_pair_str = market_pair_jstring.to_str().map_err(|_|OpenlimitsJavaError::InvalidArgument(String::from("Failed to decode market string")))?;
    let req = OrderBookRequest {
      market_pair: market_pair_str.into()
    };
  
    let resp = runtime.block_on(client.order_book(&req))?;
    let out = orderbook_resp_to_jobject(&env, resp, market.into())?;
    Ok(out)
  };

  handle_jobject_result(env, call())
}

#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_getPriceTicker(env: JNIEnv, _class: JClass,  cli: JObject, market: JString) -> jobject {
  let call = move || -> OpenLimitsJavaResult<JObject> {
    let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client")?;
    let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime")?;

    let req = GetPriceTickerRequest {
      market_pair: env.get_string(market)?.into()
    };

    let resp = runtime.block_on(client.get_price_ticker(&req))?;
    let out = ticker_to_jobject(&env, resp)?;
    Ok(out)
  };

  handle_jobject_result(env, call())
}


#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_getHistoricRates(env: JNIEnv, _class: JClass,  cli: JObject, hist_req: JObject) -> jobject {
  let call = move || -> OpenLimitsJavaResult<JObject> {
    let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client")?;
    let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime")?;

    let req = get_historic_rates_request(&env, &hist_req).map_err(OpenlimitsJavaError::InvalidArgument)?;

    let resp = runtime.block_on(client.get_historic_rates(&req))?;
    let out = vec_to_jobject(&env, CANDLE_CLS_NAME, resp, candle_to_jobject)?;
    Ok(out)
  };

  handle_jobject_result(env, call())
}


#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_getHistoricTrades(env: JNIEnv, _class: JClass,  cli: JObject, trades_req: JObject) -> jobject {
  let call = move || -> OpenLimitsJavaResult<JObject> {
    let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client")?;
    let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime")?;
    let req = get_historic_trades_request(&env, &trades_req).map_err(OpenlimitsJavaError::InvalidArgument)?;

    let resp = runtime.block_on(client.get_historic_trades(&req))?;
    let out = vec_to_jobject(&env, TRADE_CLS_NAME, resp, trade_to_jobject)?;
    Ok(out)
  };

  handle_jobject_result(env, call())
}

#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_limitBuy(env: JNIEnv, _class: JClass,  cli: JObject, req: JObject) -> jobject {
  let call = move || -> OpenLimitsJavaResult<JObject> {
    let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client")?;
    let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime")?;

    let req = get_limit_request(&env, &req).map_err(OpenlimitsJavaError::InvalidArgument)?;
    let resp = runtime.block_on(client.limit_buy(&req))?;
    
    Ok(order_to_jobject(&env, resp)?)
  };
  handle_jobject_result(env, call())
}

#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_limitSell(env: JNIEnv, _class: JClass,  cli: JObject, req: JObject) -> jobject {
  let call = move || -> OpenLimitsJavaResult<JObject> {
    let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client")?;
    let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime")?;

    let req = get_limit_request(&env, &req).map_err(OpenlimitsJavaError::InvalidArgument)?;

    let resp = runtime.block_on(client.limit_sell(&req))?;
    Ok(order_to_jobject(&env, resp)?)
  };
  handle_jobject_result(env, call())
}


#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_marketBuy(env: JNIEnv, _class: JClass,  cli: JObject, req: JObject) -> jobject {
  let call = move || -> OpenLimitsJavaResult<JObject> {
    let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client")?;
    let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime")?;

    let req = get_market_request(&env, &req).map_err(OpenlimitsJavaError::InvalidArgument)?;

    let resp = runtime.block_on(client.market_buy(&req))?;
    Ok(order_to_jobject(&env, resp)?)
  };
  handle_jobject_result(env, call())
}

#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_marketSell(env: JNIEnv, _class: JClass,  cli: JObject, req: JObject) -> jobject {
  let call = move || -> OpenLimitsJavaResult<JObject> {
    let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client")?;
    let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime")?;

    let req = get_market_request(&env, &req).map_err(OpenlimitsJavaError::InvalidArgument)?;

    let resp = runtime.block_on(client.market_sell(&req))?;
    Ok(order_to_jobject(&env, resp)?)
  };
  handle_jobject_result(env, call())
}

#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_getAllOpenOrders(env: JNIEnv, _class: JClass,  cli: JObject) -> jobject {
  let call = move || -> OpenLimitsJavaResult<JObject> {
    let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client")?;
    let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime")?;

    let resp = runtime.block_on(client.get_all_open_orders())?;

    let out = vec_to_jobject(&env, ORDER_CLS_NAME, resp, order_to_jobject)?;
    Ok(out)
  };
  handle_jobject_result(env, call())
}

#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_getOrderHistory(env: JNIEnv, _class: JClass,  cli: JObject, req: JObject) -> jobject {
  let call = move || -> OpenLimitsJavaResult<JObject> {
    let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client")?;
    let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime")?;
    let req = get_order_history_request(&env, &req).map_err(OpenlimitsJavaError::InvalidArgument)?;

    let resp = runtime.block_on(client.get_order_history(&req))?;
    let out = vec_to_jobject(&env, ORDER_CLS_NAME, resp, order_to_jobject)?;
    Ok(out)
  };
  handle_jobject_result(env, call())
}

#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_getOrder(env: JNIEnv, _class: JClass,  cli: JObject, req: JObject) -> jobject {
  let call = move || -> OpenLimitsJavaResult<JObject> {
    let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client")?;
    let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime").expect("Failed to get runtime");
    let req = get_order_request(&env, &req).map_err(OpenlimitsJavaError::InvalidArgument)?;

    let resp = runtime.block_on(client.get_order(&req))?;

    Ok(order_to_jobject(&env, resp)?)
  };
  handle_jobject_result(env, call())
}

#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_getTradeHistory(env: JNIEnv, _class: JClass,  cli: JObject, req: JObject) -> jobject {
  let call = move || -> OpenLimitsJavaResult<JObject> {
    let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client")?;
    let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime")?;
    let req = get_trade_history_request(&env, &req).map_err(OpenlimitsJavaError::InvalidArgument)?;

    let resp = runtime.block_on(client.get_trade_history(&req))?;
    let out = vec_to_jobject(&env, TRADE_CLS_NAME, resp, trade_to_jobject)?;
    Ok(out)
  };
  handle_jobject_result(env, call())
}

#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_getAccountBalances(env: JNIEnv, _class: JClass,  cli: JObject, req: JObject) -> jobject {
  let call = move || -> OpenLimitsJavaResult<JObject> {
    let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client")?;
    let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime")?;
    let req = match req.is_null() {
      true => None,
      false => Some(get_paginator(&env, &req))
    };
    let paginator = req.transpose().map_err(OpenlimitsJavaError::InvalidArgument)?;

    let resp = runtime.block_on(client.get_account_balances(paginator))?;
    let out = vec_to_jobject(&env, BALANCE_CLS_NAME, resp, balance_to_jobject)?;
    Ok(out)
  };
  handle_jobject_result(env, call())
}


#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_cancelOrder(env: JNIEnv, _class: JClass,  cli: JObject, req: JObject) -> jobject {
  let call = move || -> OpenLimitsJavaResult<JObject> {
    let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client")?;
    let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime")?;
    let req = get_cancel_order_request(&env, &req).map_err(OpenlimitsJavaError::InvalidArgument)?;

    let resp = runtime.block_on(client.cancel_order(&req))?;

    let out = order_cancelled_to_jobject(&env, resp)?;
    Ok(out)
  };
  handle_jobject_result(env, call())
}


#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_cancelAllOrders(env: JNIEnv, _class: JClass,  cli: JObject, req: JObject) -> jobject {
  let call = move || -> OpenLimitsJavaResult<JObject> {
    let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client")?;
    let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime")?;
    let req = get_cancel_all_orders_request(&env, &req).map_err(OpenlimitsJavaError::InvalidArgument)?;

    let resp = runtime.block_on(client.cancel_all_orders(&req))?;

    let out = vec_to_jobject(&env, ORDER_CANCELED_CLS_NAME, resp, order_cancelled_to_jobject)?;
    Ok(out)
  };
  handle_jobject_result(env, call())
}


#[no_mangle]
pub extern "system" fn Java_io_nash_openlimits_ExchangeClient_receivePairs(env: JNIEnv, _class: JClass,  cli: JObject) -> jobject {
  let call = move || -> OpenLimitsJavaResult<JObject> {
    let client: MutexGuard<AnyExchange> = env.get_rust_field(cli, "_client")?;
    let mut runtime: MutexGuard<tokio::runtime::Runtime> = env.get_rust_field(cli, "_runtime")?;

    
    let resp = runtime.block_on(client.retrieve_pairs())?;
    let pairs_maybe: errors::Result<Vec<_>> = resp.into_iter().map(|v| market_pair_to_jobject(&env, v)).collect();
    let pairs = pairs_maybe?;
    let pairs_cls = env.find_class(MARKET_PAIR_CLS_NAME)?;

    let out = vec_to_java_arr(&env, pairs_cls, &pairs)?;
    Ok(out.l()?)
  };
  handle_jobject_result(env, call())
}


/// jobject to openlimits

fn get_paginator(
  env: &JNIEnv,
  paginator: &JObject
) -> Result<Paginator, String> {
  let start_time = get_long_nullable(env, paginator, "startTime")?;
  let end_time = get_long_nullable(env, paginator, "endTime")?;
  let limit = get_long_nullable(env, paginator, "limit")?;
  let before = get_string(env, paginator, "before")?;
  let after = get_string(env, paginator, "after")?;

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


fn get_subscription(
  env: &JNIEnv,
  sub: &JObject
) -> Result<Subscription, String> {
  match get_string_non_null(env, sub, "tag")?.as_str() {
    "OrderBook" => {
      let market = get_string_non_null(env, sub, "market")?;
      Ok(Subscription::OrderBookUpdates(market))
    },
    "Trade" => {
      let market = get_string_non_null(env, sub, "market")?;
      Ok(Subscription::Trades(market))
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

  let paginator = get_object(env, trades_req, "paginator", PAGINATOR_CLS_NAME)?;
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

  let paginator = get_object(env, req, "paginator", PAGINATOR_CLS_NAME)?;
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
  let paginator = get_object(env, hist_req, "paginator", PAGINATOR_CLS_NAME)?;
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
  let credentials = get_object(&env, nash, "credentials",  NASH_CREDENTIALS_CLS_NAME)?;

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

fn string_to_time_in_force(
  time_in_force_str: &str,
  time_in_force_ms: i64
) -> Result<TimeInForce, String> {
  match time_in_force_str {
    "GTC" => Ok(TimeInForce::GoodTillCancelled),
    "GTT" => Ok(TimeInForce::GoodTillTime(Duration::milliseconds(time_in_force_ms))),
    "IOC" => Ok(TimeInForce::ImmediateOrCancelled),
    "FOK" => Ok(TimeInForce::FillOrKill),
    s => Err(format!("Invalid TimeInForce string {}", s))
  }
}

fn get_limit_request(
  env: &JNIEnv,
  req: &JObject,
) -> Result<OpenLimitOrderRequest, String> {
  let size = get_string_non_null(env, req, "size")?;
  let price = get_string_non_null(env, req, "price")?;
  let time_in_force = get_string_non_null(env, req, "timeInForce")?;
  let time_in_force_time = get_long_default_with_default(env, req, "timeInForceDurationMs", 0)?;
  let time_in_force = string_to_time_in_force(time_in_force.as_str(), time_in_force_time as i64)?;
  let market_pair = get_string_non_null(env, req, "market")?;
  let post_only = get_field(env, req, "post_only",  "Z")?.unwrap_or(JValue::Bool(0)).z().map_err(|_| "Failed to convert boolean to jvalue")?;
  let size = Decimal::from_str(size.as_str()).map_err(|e|e.to_string())?;
  let price = Decimal::from_str(price.as_str()).map_err(|e|e.to_string())?;
  Ok(
    OpenLimitOrderRequest {
      size,
      time_in_force,
      price,
      post_only,
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
  
  let credentials_opt = get_object(&env, binance, "credentials",  BINANCE_CREDENTIALS_CLS_NAME)?;

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
  let nash = get_object(&env, opts, "nash",  NASH_CONFIG_CLS_NAME)?;
  let binance = get_object(&env, opts, "binance",  BINANCE_CONFIG_CLS_NAME)?;
  match (nash, binance) {
    (Some(nash), _) => get_options_nash(&env, &nash),
    (_, Some(binance)) => get_options_binance(&env, &binance),
    // (_, Ok(binance)) => {},
    _ => Err(String::from("Invalid config, nash and binance field both null"))
  }
}