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
- [x] check sequencing (ignore stale updates, error on missed updates)
- [x] fix parse error
- [ ] auto reconnect
- [ ] implement refetching snapshot on missed event
- [x] "lockless" orderbook (funnel depth update and snapshot into same channel)
- [ ] use uint64 for price, quantity and updateid
- [ ] replace btreemap with bitset
    - preallocated array where each item represents a pricelevel
    - array of bitsets to find the best bid/ask price
    - bitset array length is pricelevels / 64, each item is uint64
    - find best price: iterate through array
        - if item is 0, continue
        - use leading_zeros() to find the count of leading/trailing zeros in the number
        - index of number in array + count of leading zeros in numer = index of best pricelevel

# Extra todos

- multiple markets at the same time
- visualize orderbook with ratatui
- get funding rates and funding rate countdown
