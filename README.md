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

## Step 3: Generate a new Program Id
```bash
solana address -k target/deploy/state_check_program-keypair.json
```

#### copy paste the keypair on lib.rs ,Anchor.toml and state-check-program.ts

## Step4: 
```bash
solana-keygen new --outfile target/deploy/state_check_program-keypair.json

```
#### This will generate a keypair file which will resemble the programId newly generated

## Step 5: Deploy the program
```bash
anchor deploy
```

## Step 6: Perform tests
```bash
anchor test --skip-deploy
```