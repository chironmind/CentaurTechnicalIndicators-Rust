# Contributing to Rust TI

First of all I appreciate all contributions no matter how big or small.

## How to contribute

Contributing is easy, all you have to do is:
    * Clone the code
    * Make the changes
    * Raise a PR
    * @ 0100101001010000 on the PR and ask for a review

## What to do?

There are a few things that need to be done in the code, so if you have no ideas, doing some of the following
would be very helpful.

### General

    * remove test_ prefix from all tests
    * in parabolic t/p system remove single functions, and have the bulk function find the trend and determine
      whether to go long or short like it does in the volatility system ... or return s/l in vol system
    * Add the ability for indicators to use something other than normal distributions (for variance, std dev...)

### New indicators

    * Indicator that shows the distribution of prices over a period. Essentially does a count for each unique price, and orders the prices in ascending order
    * Create a version of mode/median absolute deviation that doesn't take the mode/median from the absolute deviation of the mean, but rather the absolute deviation from the mode/mean
    * McGinley dynamic versions of anything using MA (Donchain channels, Keltner channels, Supertrend, 
percentage price oscillator, relative vigor index)

## I have an indicator I want to add

If you have a new indicator you want to add there are a few rules to follow:

    1. Raise an issue with link(s) to the indicator, there needs to be a clear explanation of what
it is used for, and how it is calculated.
    2. Add the indicator to the code, make sure that it is properly documented and has unit tests.
    3. Probably the most important part, in `assets/rust_ti_hand_calcs.ods` add a new tab for the indicator
and do the calculations "by hand" to show that the results that come up in the tests are what is calculated in the spreadsheet.
