// automatically generated by the FlatBuffers compiler, do not modify


#ifndef FLATBUFFERS_GENERATED_COMMON_GRAPH_H_
#define FLATBUFFERS_GENERATED_COMMON_GRAPH_H_

#include "flatbuffers/flatbuffers.h"

// Ensure the included flatbuffers.h is the same version as when this file was
// generated, otherwise it may not be compatible.
static_assert(FLATBUFFERS_VERSION_MAJOR == 24 &&
              FLATBUFFERS_VERSION_MINOR == 3 &&
              FLATBUFFERS_VERSION_REVISION == 25,
             "Non-compatible flatbuffers version included");

namespace Graph {

struct Price;

struct Size;

struct Timestamp;

struct InstrumentId;
struct InstrumentIdBuilder;

enum Currency : int16_t {
  Currency_USD = 0,
  Currency_USDC = 1,
  Currency_USDT = 2,
  Currency_BTC = 3,
  Currency_ETH = 4,
  Currency_SOL = 5,
  Currency_XRP = 6,
  Currency_MATIC = 7,
  Currency_EUR = 8,
  Currency_GBP = 9,
  Currency_MIN = Currency_USD,
  Currency_MAX = Currency_GBP
};

inline const Currency (&EnumValuesCurrency())[10] {
  static const Currency values[] = {
    Currency_USD,
    Currency_USDC,
    Currency_USDT,
    Currency_BTC,
    Currency_ETH,
    Currency_SOL,
    Currency_XRP,
    Currency_MATIC,
    Currency_EUR,
    Currency_GBP
  };
  return values;
}

inline const char * const *EnumNamesCurrency() {
  static const char * const names[11] = {
    "USD",
    "USDC",
    "USDT",
    "BTC",
    "ETH",
    "SOL",
    "XRP",
    "MATIC",
    "EUR",
    "GBP",
    nullptr
  };
  return names;
}

inline const char *EnumNameCurrency(Currency e) {
  if (::flatbuffers::IsOutRange(e, Currency_USD, Currency_GBP)) return "";
  const size_t index = static_cast<size_t>(e);
  return EnumNamesCurrency()[index];
}

enum Exchange : int16_t {
  Exchange_Internal = 0,
  Exchange_Binance = 1,
  Exchange_Bitstamp = 2,
  Exchange_Bitfinex = 3,
  Exchange_Bittrex = 4,
  Exchange_Coinbase = 5,
  Exchange_Deribit = 6,
  Exchange_Gateio = 7,
  Exchange_Gemini = 8,
  Exchange_Itbit = 9,
  Exchange_Kraken = 10,
  Exchange_Lmax = 11,
  Exchange_Okcoin = 12,
  Exchange_Okx = 13,
  Exchange_MIN = Exchange_Internal,
  Exchange_MAX = Exchange_Okx
};

inline const Exchange (&EnumValuesExchange())[14] {
  static const Exchange values[] = {
    Exchange_Internal,
    Exchange_Binance,
    Exchange_Bitstamp,
    Exchange_Bitfinex,
    Exchange_Bittrex,
    Exchange_Coinbase,
    Exchange_Deribit,
    Exchange_Gateio,
    Exchange_Gemini,
    Exchange_Itbit,
    Exchange_Kraken,
    Exchange_Lmax,
    Exchange_Okcoin,
    Exchange_Okx
  };
  return values;
}

inline const char * const *EnumNamesExchange() {
  static const char * const names[15] = {
    "Internal",
    "Binance",
    "Bitstamp",
    "Bitfinex",
    "Bittrex",
    "Coinbase",
    "Deribit",
    "Gateio",
    "Gemini",
    "Itbit",
    "Kraken",
    "Lmax",
    "Okcoin",
    "Okx",
    nullptr
  };
  return names;
}

inline const char *EnumNameExchange(Exchange e) {
  if (::flatbuffers::IsOutRange(e, Exchange_Internal, Exchange_Okx)) return "";
  const size_t index = static_cast<size_t>(e);
  return EnumNamesExchange()[index];
}

FLATBUFFERS_MANUALLY_ALIGNED_STRUCT(8) Price FLATBUFFERS_FINAL_CLASS {
 private:
  double value_;

 public:
  Price()
      : value_(0) {
  }
  Price(double _value)
      : value_(::flatbuffers::EndianScalar(_value)) {
  }
  double value() const {
    return ::flatbuffers::EndianScalar(value_);
  }
};
FLATBUFFERS_STRUCT_END(Price, 8);

FLATBUFFERS_MANUALLY_ALIGNED_STRUCT(8) Size FLATBUFFERS_FINAL_CLASS {
 private:
  double value_;

 public:
  Size()
      : value_(0) {
  }
  Size(double _value)
      : value_(::flatbuffers::EndianScalar(_value)) {
  }
  double value() const {
    return ::flatbuffers::EndianScalar(value_);
  }
};
FLATBUFFERS_STRUCT_END(Size, 8);

FLATBUFFERS_MANUALLY_ALIGNED_STRUCT(8) Timestamp FLATBUFFERS_FINAL_CLASS {
 private:
  int64_t value_;

 public:
  Timestamp()
      : value_(0) {
  }
  Timestamp(int64_t _value)
      : value_(::flatbuffers::EndianScalar(_value)) {
  }
  int64_t value() const {
    return ::flatbuffers::EndianScalar(value_);
  }
};
FLATBUFFERS_STRUCT_END(Timestamp, 8);

struct InstrumentId FLATBUFFERS_FINAL_CLASS : private ::flatbuffers::Table {
  typedef InstrumentIdBuilder Builder;
  enum FlatBuffersVTableOffset FLATBUFFERS_VTABLE_UNDERLYING_TYPE {
    VT_EXCHANGE = 4,
    VT_ID = 6
  };
  Graph::Exchange exchange() const {
    return static_cast<Graph::Exchange>(GetField<int16_t>(VT_EXCHANGE, 0));
  }
  const ::flatbuffers::String *id() const {
    return GetPointer<const ::flatbuffers::String *>(VT_ID);
  }
  bool Verify(::flatbuffers::Verifier &verifier) const {
    return VerifyTableStart(verifier) &&
           VerifyField<int16_t>(verifier, VT_EXCHANGE, 2) &&
           VerifyOffset(verifier, VT_ID) &&
           verifier.VerifyString(id()) &&
           verifier.EndTable();
  }
};

struct InstrumentIdBuilder {
  typedef InstrumentId Table;
  ::flatbuffers::FlatBufferBuilder &fbb_;
  ::flatbuffers::uoffset_t start_;
  void add_exchange(Graph::Exchange exchange) {
    fbb_.AddElement<int16_t>(InstrumentId::VT_EXCHANGE, static_cast<int16_t>(exchange), 0);
  }
  void add_id(::flatbuffers::Offset<::flatbuffers::String> id) {
    fbb_.AddOffset(InstrumentId::VT_ID, id);
  }
  explicit InstrumentIdBuilder(::flatbuffers::FlatBufferBuilder &_fbb)
        : fbb_(_fbb) {
    start_ = fbb_.StartTable();
  }
  ::flatbuffers::Offset<InstrumentId> Finish() {
    const auto end = fbb_.EndTable(start_);
    auto o = ::flatbuffers::Offset<InstrumentId>(end);
    return o;
  }
};

inline ::flatbuffers::Offset<InstrumentId> CreateInstrumentId(
    ::flatbuffers::FlatBufferBuilder &_fbb,
    Graph::Exchange exchange = Graph::Exchange_Internal,
    ::flatbuffers::Offset<::flatbuffers::String> id = 0) {
  InstrumentIdBuilder builder_(_fbb);
  builder_.add_id(id);
  builder_.add_exchange(exchange);
  return builder_.Finish();
}

inline ::flatbuffers::Offset<InstrumentId> CreateInstrumentIdDirect(
    ::flatbuffers::FlatBufferBuilder &_fbb,
    Graph::Exchange exchange = Graph::Exchange_Internal,
    const char *id = nullptr) {
  auto id__ = id ? _fbb.CreateString(id) : 0;
  return Graph::CreateInstrumentId(
      _fbb,
      exchange,
      id__);
}

}  // namespace Graph

#endif  // FLATBUFFERS_GENERATED_COMMON_GRAPH_H_
