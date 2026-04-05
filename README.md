# Binabook


snapshot:  https://fapi.binance.com/fapi/v1/depth?symbol=BTCUSDT&limit=1000
depth deltas: wss://fstream.binance.com/public/ws/btcusdt@depth@100ms

# TODO

- [x] establish ws to bina
- [x] parse depth update
- [x] get and parse snapshot
- [x] internal orderbook datastructure
- [x] print top n level
- [x] map snapshot into internal orderbook
- [x] apply a single depth update to internal orderbook
- [x] assemble main program:
    1. sub to depth stream
    1. buffer depth updates
    1. get snapshot
    1. build orderbook from snapshot
    1. apply buffered depth updates, apply subsequent updates to orderbook
- [ ] fix parse error
- [ ] auto reconnect

# Extra todos

- visualize orderbook with ratatui
- get funding rates and funding rate countdown
