### Environment Setup
1. Install Rust from https://rustup.rs/
2. Install Solana from https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool

### Build and test for program compiled natively
```
$ cargo build
$ cargo test
```

### Build and test the program compiled for BPF
```
$ cargo build-bpf
$ cargo test-bpf
```

### Program Objective
Step 1: Alice sends asset X to trusted third-party (this program)
Step 2: Bob sends asset Y to trusted third-party
Step 3: Trusted third-party sends asset X to Bob and sends asset Y to Alice.


### Program Flow
1. Someone calls the entrypoint.
2. The entrypoint forwards the arguments to the processor.
3. The processor asks `instruction.rs` to decode the `instruction_data` argument from the entrypoint function.
4. Using the decoded data, the processor will now decide which processing function to use to process the request.
5. The processor may use `state.rs` to encode state into or decode the state of an account which has been passed into the entrypoint.

![alt text](https://paulx.dev/assets/img/escrow-sketch-1.6df070a8.png)


