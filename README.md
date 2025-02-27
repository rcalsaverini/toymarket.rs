# Purpose
This is an implementation (in the [Rust programming language](https://www.rust-lang.org/)) of an agent-based simulation of simple toy models for financial markets behavior similar to what is described in the paper [Self-referential behaviour, overreaction and conventions in financial markets
](https://www.sciencedirect.com/science/article/abs/pii/S0167268106000576) by Matthieu Wyart and Jean-Philippe Bouchaud.

The premise is that an asset is traded in a market by agents. This asset is represented by it's return series $x_t$, where $t$ is a discrete time parameter. At each tick of the market, each agent make a decision on a volume to buy/sell, with a given decision rule. The total volume is aggregated and the next value of the return is determined by a market response function, which takes the total volume ordered and calculates the next return in the series.

Agents are also given a "learning" oportunity at each tick, by observing the results in the series and re-calibrating their internal decision engine.
