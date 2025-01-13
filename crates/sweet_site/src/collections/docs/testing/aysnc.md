---
title: Async
description: Running async tests
draft: true
sidebar:
  order: 3
---

## `#[tokio::test]` (native)

Sweet supports `#[tokio::test]` and any other macro that runs with the default test runner, and they will run in the same fashion.

## `#[wasm_bindgen_test` (wasm)

These tests are run in the `wasm_bindgen_test` runner and cannot be accessed by `sweet`. 

## `#[sweet::test]` (native,wasm)

The sweet test macro works differently from tokio in that it employs a shared async runtime which results in faster startup times. For 99% of cases `#[sweet::test]` is the way to go, but if you do have two tests mutating the same static resources.