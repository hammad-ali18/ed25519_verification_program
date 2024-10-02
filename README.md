## Step 1: Update the Keypair
### Anchor.toml
```bash
[provider]
cluster = "devnet"
wallet = "/home/muhammad/wallet/keypair1.json"  # Replace with your own keypair
```
## Step 2: Generate Build
```bash
anchor build
```

## Step 3: This is will return the existing programId
```bash
solana address -k target/deploy/state_check_program-keypair.json
```

#### copy paste the keypair on lib.rs ,Anchor.toml and state-check-program.ts

## Step4 Generate a new programId (): 
```bash
solana-keygen new --outfile target/deploy/state_check_program-keypair.json

```
#### copy paste the keypair on lib.rs ,Anchor.toml and state-check-program.ts

## Step 5: Deploy the program
```bash
anchor deploy
```

## Step 6: Perform tests
```bash
anchor test --skip-deploy
```