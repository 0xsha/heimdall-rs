#[cfg(test)]
mod benchmark {
    use clap_verbosity_flag::Verbosity;
    use heimdall_common::utils::testing::benchmarks::async_bench;

    use heimdall_core::decode::DecodeArgs;

    #[tokio::test]
    async fn benchmark_decode_transfer() {
        async fn bench() {
            let args = DecodeArgs {
                target: String::from("0xc47f00270000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000b6a6265636b65722e657468000000000000000000000000000000000000000000"),
                verbose: Verbosity::new(0, 0),
                rpc_url: String::from(""),
                openai_api_key: String::from(""),
                explain: false,
                default: true,
                truncate_calldata: false,
                skip_resolving: false,
            };
            let _ = heimdall_core::decode::decode(args).await;
        }

        async_bench("benchmark_decode_transfer", 100, bench).await;
    }

    #[tokio::test]
    async fn benchmark_decode_uniswap_simple() {
        async fn bench() {
            let args = DecodeArgs {
                target: String::from("791ac947000000000000000000000000000000000000000000ac03e3c2829679f93600000000000000000000000000000000000000000000000000000200b952bc426b0c00000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000bffadf2903e6ec32dc0a7b5a2b57de0e728ec0b500000000000000000000000000000000000000000000000000000000645c1321000000000000000000000000000000000000000000000000000000000000000200000000000000000000000039702bb7c6a482bdde6aec96175bc6cdc277b999000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2"),
                verbose: Verbosity::new(0, 0),
                rpc_url: String::from(""),
                openai_api_key: String::from(""),
                explain: false,
                default: true,
                truncate_calldata: false,
                skip_resolving: false,
            };
            let _ = heimdall_core::decode::decode(args).await;
        }

        async_bench("benchmark_decode_uniswap_simple", 100, bench).await;
    }

    #[tokio::test]
    async fn benchmark_decode_seaport_simple() {
        async fn bench() {
            let args = DecodeArgs {
                target: String::from("0xfb0f3ee100000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ec9c58de0a8000000000000000000000000000d2f8a98bde7c701ae961d10d0d1fc3a751be737f000000000000000000000000004c00500000ad104d7dbd00e3ae0a5c00560c000000000000000000000000005008c2a3af41024e9f0bd0432df4f75828602598000000000000000000000000000000000000000000000000000000000000110600000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000006358934b00000000000000000000000000000000000000000000000000000000637e22710000000000000000000000000000000000000000000000000000000000000000360c6ebe000000000000000000000000000000000000000038844ef19f04aecf0000007b02230091a7ed01230072f7006a004d60a8d4e71d599b8104250f000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000024000000000000000000000000000000000000000000000000000000000000002e0000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000066517289880000000000000000000000000000000a26b00c1f0df003000390027140000faa719000000000000000000000000000000000000000000000000000cca2e51310000000000000000000000000000cecf12f47d2896c90f6e19b7376fa3b169fabd920000000000000000000000000000000000000000000000000000000000000041447858c6d8251fb8ffba546bedb410457ff77148fdf59ac8e046993936a134b028f535c5b1f760508b6e0c3c18d44927d82da0502c66688c0dc961a434a9b0071c00000000000000000000000000000000000000000000000000000000000000"),
                verbose: Verbosity::new(0, 0),
                rpc_url: String::from(""),
                openai_api_key: String::from(""),
                explain: false,
                default: true,
                truncate_calldata: false,
                skip_resolving: false,
            };
            let _ = heimdall_core::decode::decode(args).await;
        }

        async_bench("benchmark_decode_seaport_simple", 100, bench).await;
    }

    #[tokio::test]
    async fn benchmark_decode_seaport_complex() {
        async fn bench() {
            let args = DecodeArgs {
                target: String::from("0xc47f00270000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000b6a6265636b65722e657468000000000000000000000000000000000000000000"),
                verbose: Verbosity::new(0, 0),
                rpc_url: String::from(""),
                openai_api_key: String::from(""),
                explain: false,
                default: true,
                truncate_calldata: false,
                skip_resolving: false,
            };
            let _ = heimdall_core::decode::decode(args).await;
        }

        async_bench("benchmark_decode_seaport_complex", 100, bench).await;
    }
}

#[cfg(test)]
mod tests {
    use clap_verbosity_flag::Verbosity;
    use heimdall_core::decode::DecodeArgs;

    #[tokio::test]
    async fn test_decode_transfer() {
        let args = DecodeArgs {
            target: String::from("0xfb0f3ee100000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ec9c58de0a8000000000000000000000000000d2f8a98bde7c701ae961d10d0d1fc3a751be737f000000000000000000000000004c00500000ad104d7dbd00e3ae0a5c00560c000000000000000000000000005008c2a3af41024e9f0bd0432df4f75828602598000000000000000000000000000000000000000000000000000000000000110600000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000006358934b00000000000000000000000000000000000000000000000000000000637e22710000000000000000000000000000000000000000000000000000000000000000360c6ebe000000000000000000000000000000000000000038844ef19f04aecf0000007b02230091a7ed01230072f7006a004d60a8d4e71d599b8104250f000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000024000000000000000000000000000000000000000000000000000000000000002e0000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000066517289880000000000000000000000000000000a26b00c1f0df003000390027140000faa719000000000000000000000000000000000000000000000000000cca2e51310000000000000000000000000000cecf12f47d2896c90f6e19b7376fa3b169fabd920000000000000000000000000000000000000000000000000000000000000041447858c6d8251fb8ffba546bedb410457ff77148fdf59ac8e046993936a134b028f535c5b1f760508b6e0c3c18d44927d82da0502c66688c0dc961a434a9b0071c00000000000000000000000000000000000000000000000000000000000000"),
            verbose: Verbosity::new(0, 0),
            rpc_url: String::from(""),
            openai_api_key: String::from(""),
            explain: false,
            default: true,
            truncate_calldata: false,
            skip_resolving: false,
        };
        let _ = heimdall_core::decode::decode(args).await;
    }

    #[tokio::test]
    async fn test_decode_seaport_simple() {
        let args = DecodeArgs {
            target: String::from("0xfb0f3ee100000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000ec9c58de0a8000000000000000000000000000d2f8a98bde7c701ae961d10d0d1fc3a751be737f000000000000000000000000004c00500000ad104d7dbd00e3ae0a5c00560c000000000000000000000000005008c2a3af41024e9f0bd0432df4f75828602598000000000000000000000000000000000000000000000000000000000000110600000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000006358934b00000000000000000000000000000000000000000000000000000000637e22710000000000000000000000000000000000000000000000000000000000000000360c6ebe000000000000000000000000000000000000000038844ef19f04aecf0000007b02230091a7ed01230072f7006a004d60a8d4e71d599b8104250f000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000024000000000000000000000000000000000000000000000000000000000000002e0000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000066517289880000000000000000000000000000000a26b00c1f0df003000390027140000faa719000000000000000000000000000000000000000000000000000cca2e51310000000000000000000000000000cecf12f47d2896c90f6e19b7376fa3b169fabd920000000000000000000000000000000000000000000000000000000000000041447858c6d8251fb8ffba546bedb410457ff77148fdf59ac8e046993936a134b028f535c5b1f760508b6e0c3c18d44927d82da0502c66688c0dc961a434a9b0071c00000000000000000000000000000000000000000000000000000000000000"),
            verbose: Verbosity::new(0, 0),
            rpc_url: String::from(""),
            openai_api_key: String::from(""),
            explain: false,
            default: true,
            truncate_calldata: false,
            skip_resolving: false,
        };
        let _ = heimdall_core::decode::decode(args).await;
    }
}

mod integration_tests {
    use clap_verbosity_flag::Verbosity;
    use heimdall_common::utils::{sync::blocking_await, threading::task_pool};
    use heimdall_core::decode::DecodeArgsBuilder;
    use serde_json::Value;

    /// Thorough testing for decode across a large number of transactions.
    #[test]
    #[ignore]
    fn heavy_test_decode_thorough() {
        // load ./tests/testdata/txids.json into a vector using serde
        let txids = serde_json::from_str::<Value>(
            &std::fs::read_to_string("./tests/testdata/txids.json").expect("failed to read file"),
        )
        .expect("failed to parse json")
        .get("txids")
        .expect("failed to get txids")
        .as_array()
        .expect("failed to convert txids to array")
        .iter()
        .map(|v| v.as_str().expect("failed to stringify json value").to_string())
        .collect::<Vec<String>>();
        let total = txids.len();

        // task_pool(items, num_threads, f)
        let results = task_pool(txids, 10, |txid: String| {
            let args = DecodeArgsBuilder::new()
                .target(txid.to_string())
                .verbose(Verbosity::new(-1, 0))
                .rpc_url("https://eth.llamarpc.com".to_string())
                .build()
                .expect("failed to build args");

            blocking_await(move || {
                // get new blocking runtime
                let rt = tokio::runtime::Runtime::new().expect("failed to get runtime");

                // get the storage diff for this transaction
                println!("decoding txid: {}", txid);
                match rt.block_on(heimdall_core::decode::decode(args)) {
                    Ok(resolved_functions) => {
                        // check if any resolved_function is named Unresolved_{}
                        if resolved_functions.iter().any(|rf| rf.name.starts_with("Unresolved_")) {
                            println!("decoding txid: {} ... unresolved succeeded", txid);
                        }

                        println!("decoding txid: {} ... succeeded", txid);
                        1
                    }
                    Err(e) => {
                        println!("decoding txid: {} ... failed", txid);
                        println!("  \\- error: {:?}", e);

                        // we dont want to count RPC errors as failures
                        match e {
                            heimdall_core::error::Error::RpcError(_) => 1,
                            _ => 0,
                        }
                    }
                }
            })
        });
        let success_count = results.iter().filter(|r| **r == 1).count();

        // assert 95% of the transactions were successful
        let success_rate = (success_count as f64) / (total as f64);
        println!(
            "heavy_test_decode_thorough:\n * total: {}\n * failed: {}\n * success rate: {}",
            total,
            total - success_count,
            success_rate * 100.0
        );

        assert!(success_rate >= 0.93);
    }
}
