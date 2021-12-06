# FAQ

## General questions

### What are the hardware requirements?

The philosophy of Massa is to be as decentralized as possible. To
fulfill this goal, we aim to have low hardware requirements so that many
people can run nodes. Right now 4 cores and 8 GB of RAM should be enough
to run a node. As the transaction rate increases, it might not be
sufficient anymore. Ultimately, we plan that the mainnet fits on a
desktop computer with 8 cores, 16 GB RAM, and 1TB disk.

### Can it run on a VPS?

You can use a VPS to run a node. The pros of VPS are that they have high
availability and are easy to configure. Cons are that nodes running on a
VPS can lead to centralization if a lot of nodes running on the same
provider (e.g. AWS).

### How to keep the node running when I close the terminal?

You can run the following command in the terminal:

    nohup cargo run --release &

the output will go to the `nohup.out` file. You will be able to close
the terminal safely then. To kill the app you'll have to use
`pkill -f massa-node`. You can also use
[screen](https://help.ubuntu.com/community/Screen) or
[tmux](http://manpages.ubuntu.com/manpages/cosmic/man1/tmux.1.html) for
example.

### Will Massa support smart contracts?

We will try to support both the EVM for retro compatibility, and a
specific smart contract engine that fully leverages the Massa protocol
allows developing in more usual languages and introduces several
innovations.

We are currently finishing the specification of the smart contract
engine.

We are planning some exciting features, such as self-wakeup, a bit like
what is introduced here: https://arxiv.org/pdf/2102.10784.pdf

### What ports does Massa use?

By default, Massa uses TCP port 31244 for protocol communication with
other nodes, and 31245 to bootstrap other nodes. Massa also uses TCP
port 33034 for the new private
API, and 33035 for the new public API (API v2).

### How to restart the Node?

-   Ubuntu : ctrl + c for killing the process and
    `RUST_BACKTRACE=full cargo run --release |& tee logs.txt`
-   Windows : ctrl + c for killing the process and `cargo run --release`
-   Mac Os : ctrl + c for killing the process and
    `RUST_BACKTRACE=full cargo run --release > logs.txt 2>&1`

## Balance and wallet

### How to migrate from one server to another without losing staked amounts and tokens?

You need to back up the file wallet.dat and migrate it to the
massa-client folder on your new server. You also need to backup and
migrate the node_privkey.key file in massa-node/config to keep your
connectivity stats.

If you have rolls, you also need to register the key used to buy rolls
to start staking again (see [Staking](staking.md)).

### Why are the balances in the client and the explorer different ?

It may mean that your node is desynchronized.
Check that your node is running, that the computer meets hardware requirements, and try restarting your node.

### Does the command `cargo run -- --wallet wallet.dat` override my existing wallet?

No, it loads the wallet if it exists, otherwise, it creates it.

### Where is the wallet.dat located?

By default, in the massa-client directory.

## Rolls and staking

### My rolls disappeared/were sold automatically.

The most likely reason is that you did not produce some blocks when
selected to do so. Most frequent reasons:

-   Node not running 100% of the time during which you had
    active_rolls \> 0
-   Node not being properly connected to the network 100% of the time
    during which you had active_rolls \> 0
-   Node being desynchronized (which can be caused by temporary overload
    if the specs are insufficient or if other programs are using
    resources on the computer or because of internet connection
    problems) at some point while you had active_rolls \> 0
-   The node does not having the right registered staking keys (type
    staking_addresses in the client to verify that they match the
    addresses in your wallet_info that have active rolls) 100% of the
    time during which you had active_rolls \> 0

Diagnostic process:
* make sure the node is running on a computer that matches hardware requirements and that no other software is hogging ressources
* type `wallet_info` and make sure that at least one address has active rolls > 0
  * if there are no addresses listed, create a new one by calling `wallet_generate_private_key` and try the diagnostic process again
  * if none of the listed addresses has non-zero active rolls, perform a new roll buy (see tutorials) and try the diagnostic process again
* type `node_get_staking_addresses` in the client:
  * if the list is empty or if none of the addresses listed matches addresses that have active rolls in `wallet_info`:
    * call `node_add_staking_private_keys` with the private key matching an address that has non-zero active rolls in `wallet_info`
* check your address with the online explorer: if there is a mismatch between the number of active rolls displayed in the online interface and what is returned by `wallet_info`, it might be that your node is desynchronized. Try restarting it.

### Why are rolls automatically sold? Is it some kind of penalty/slashing?

It is not slashing because the funds are reimbursed fully. It's more
like an implicit roll sell.

The point is the following: for the network to be healthy, everyone with
active rolls needs to produce blocks whenever they are selected to do
so. If an address misses more than 70% of its block creation
opportunities during cycle C, all its rolls are implicitly sold at the
beginning of cycle C+3.

### Do I need to register the keys after subsequent purchases of ROLLs, or do they get staked automatically?

For now, they don't stake automatically. In the future, we will add a
feature allowing auto compounding. That being said, some people appear
to have done that very early in the project. Feel free to ask on the 
[Discord](https://discord.com/invite/massa) server :).

### I can buy, send, sell ROLLs and coins without fees. When should I increase the fee \>0?

For the moment, there are only a few transactions at the same time and
so most created blocks are empty. This means that your operation will be
added to a block even if the fee is zero. We will communicate if you
need to increase the fee.

### I am staking ROLLs but my wallet info doesn't change. When do I get my first staking rewards?

You need to wait for your rolls to become active (around 1h45), then
depending on the number of rolls you have, you might want to wait for
more to be selected for block/endorsement production.

## Testnet and rewards

### How can I migrate my node from one computer/provider to another and keep my score in the Testnet Staking Reward Program?

If you migrate your node from one computer/provider to another you
should save the private key associated with the staking address that is
registered. This private key is located in the `wallet.dat` file located
in `massa-client` folder. You can also save your node private key
`node_privkey.key` located in the `massa-node/config` folder, if you
don't then don't forget to register your new node private key to the
Discord bot.

If your new node has a new IP address then you should not forget to
register the new IP address to the Discord bot.

If you lost `wallet.dat` and/or `node_privkey.key`, don't panic, just
redo the whole node setup and rewards registration process and the newly
generated keys will be associated with your discord account. Past scores
won't be lost.

### I want to stake more! Can I abuse the faucet bot to get more coins?

You can claim testnet tokens every 24h. The tokens are worthless, you
won't have any advantage over the others by doing that.

### Will the amount of staked Rolls affect Testnet rewards?

No, as long as you have at least 1 roll, further roll purchases won't
change your score.

## Common issues

### Ping too high issue

Check the quality of your internet connection. Try increasing the
"max_ping" setting in your config file:

-   edit file `massa-node/config/config.toml` (create if it is absent) with the following
    content:

```toml
[bootstrap]
    max_ping = 10000 # try 10000 for example
```

### API can't start

-   If your API can't start, e.g. with
    `could not start API controller: ServerError(hyper::Error(Listen, Os { code: 98, kind: AddrInUse, message: "Address already in use" }))`,
    it's probably because the default API ports 33034/33035 are already in use
    on your computer. You should change the port in the config files,
    both in the API and Client:

  * create/edit file `massa-node/config/config.toml` to change the port used by the API:

```toml
[api]
    bind_private = "127.0.0.1:33034"  # change port here from 33034 to something else
    bind_public = "0.0.0.0:33035"  # change port here from 33035 to something else
```

  * create/edit file `massa-client/config/config.toml` and put the same
port:

```toml
[default_node]
    ip = "127.0.0.1"
    private_port = 33034  # change port here from 33034 to the port chosen in node's bind_private
    public_port = 33035  # change port here from 33035 to the port chosen in node's bind_public
```