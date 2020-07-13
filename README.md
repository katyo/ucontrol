# Generic control library for low-end hadrware

[![License: MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg)](https://opensource.org/licenses/MIT)
[![Travis-CI Build Status](https://travis-ci.org/katyo/ucontrol.svg?branch=master)](https://travis-ci.org/katyo/ucontrol)
[![Crates.io Package](https://img.shields.io/crates/v/uctl.svg?style=popout)](https://crates.io/crates/uctl)
[![Docs.rs API Documentation](https://docs.rs/uctl/badge.svg)](https://docs.rs/uctl)

This library intended to simplify developing control algorithms for bare-metal low-end hardware such as
microcontrollers.

## Overview

This library consist of several independent components which grouped to filters, regulators and convertors.
Also it includes some useful utilities, such as clampers, scalers and etc.

The filters and regulators can be configured in a human-friendly way without using obscure artifical coefficients.

All components can operate both with floating point and fixed point values.

### Optimization techniques

When you targeted to FPU-less hardware in order to get best possible performance and reduce firmware size you should use only binary fixed point arithmetic because internally it operates with integers.
Also you should avoid exceeding platform word size when it is possible without lossing required precision.

### Safe usage

Fixed point arithmetic has well known problems with overflowing especially on multiplication.
Also it has well known problems with precision loss on division.

## Links

* [ufix crate](https://crates.io/crates/ufix)
* [ufix API](https://docs.rs/ufix)
* [uctl crate](https://crates.io/crates/uctl)
* [uctl API](https://docs.rs/uctl)
