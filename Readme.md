# GO KITE AI BOT

Kite AI is an automation tool, used for interacting with Kite AI agents and do on chain transaction on Kite Ai Chains.

## TABLE OF CONTENTS
- [GO KITE AI BOT](#go-kite-ai-bot)
  - [TABLE OF CONTENTS](#table-of-contents)
  - [PREREQUISITE](#prerequisite)
  - [GO KITE AI](#go-kite-ai)
  - [BOT FEATURE](#bot-feature)
  - [PRE-SETUP](#pre-setup)
  - [SETUP AND CONFIGURE](#setup-and-configure)
  - [NOTE](#note)
  - [CONTRIBUTE](#contribute)
  - [SUPPORT](#support)

## PREREQUISITE
- Git
- Rust : 1.84.1
- Wallet Registered on Kite AI Website

## GO KITE AI
#New 

Kite AI's 

Incentivized Testnet v1 Aero is LIVE!

Kite AI is a Layer 1 blockchain specifically designed for AI applications, utilizing a consensus mechanism called Proof of AI (PoAI).

Register here:
[https://testnet.gokite.ai](https://testnet.gokite.ai/?r=THWzyErS) 
- Connect Wallet
- Connect X & Discord 
- Complete All Tasks 
- Chat With AI [ Daily ]
- Earn Your XP
- Done

Explore Kite AI on Galxe Quest:
https://app.galxe.com/quest/avax/GCj5jtpMyh

Source: https://x.com/GoKiteAI/status/1887561947715149870


## BOT FEATURE

- Auto Interact With AI Agents


## PRE-SETUP
- Installing Rustup
  ```
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- Set Specific Rust Verison
  ```
  rustup default 1.84.1
  ```
- Checking Rust Version
  ```
  rustc --version
  ```
- Install some dependencies
  ```
  sudo apt install curl git build-essential libssl-dev -y
  ```
- [Register To Kite AI](#go-kite-ai)

## SETUP AND CONFIGURE
- Clone Project
  ```
  git clone https://github.com/Widiskel/kite-ai-bot
  ```
- CD to Project dir
  ```
  cd kite-ai-bot
  ```
- Build Project
  ```
  cargo build --release
  ```
- Add Permisison to build binary
  ```
  chmod +x target/release/kite-ai-bot
  ```
- Copy environment and configure `.env`
  ```
  cp .env.example .env
  nano .env
  ```
- Copy accounts and configure accounts
  ```
  cp accounts_tmp.json accounts.json
  nano accounts.json
  ```
- Copy proxy and configure Proxy
  ```
  cp proxy_list_tmp.json proxy_list.json
  nano accounts.json
  ```
  Just Open and leave it [] if you not using proxy
- Execute bot
  ```
  ./target/release/kite-ai-bot
  ```


## NOTE
DWYOR & Always use a new wallet when running the bot, I am not responsible for any loss of assets.


## CONTRIBUTE

Feel free to fork and contribute adding or fixing some feature thanks. 

## SUPPORT

want to support me for creating another bot ?
**star** my repo or buy me a coffee on

EVM : `0x3fe6a02ab20de8bf34fefc106d72d7094c8c4404`

SOLANA : `3tE3Hs7P2wuRyVxyMD7JSf8JTAmEekdNsQWqAnayE1CN`
