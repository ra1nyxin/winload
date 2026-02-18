# Windows Loopback Traffic Monitoring: Why Standard APIs Fail and Npcap Works

> **[📖 English](win_loopback.md)**
> **[📖 简体中文(大陆)](win_loopback.zh-cn.md)**
> **[📖 繁體中文(台灣)](win_loopback.zh-tw.md)**

## TL;DR

Windows loopback (`127.0.0.1`) traffic is short-circuited inside `tcpip.sys`, **completely bypassing the NDIS network driver layer**. As a result, `GetIfEntry` / `GetIfTable` counters are always 0.

This is a **functional deficiency in Windows' network stack** — the loopback interface was never given a proper NDIS miniport driver, so the standard monitoring APIs simply don't work for it.

Npcap works around this by using a WFP (Windows Filtering Platform) callout driver to intercept packets before the short-circuit happens.

---

## Windows Network Stack Architecture

```
App  →  Winsock  →  AFD.sys  →  tcpip.sys  →  NDIS  →  NIC driver  →  Hardware
```

### Normal NIC Traffic Path

```
tcpip.sys  →  NDIS (counter +1)  →  miniport driver  →  physical NIC
```

The NDIS driver updates `MIB_IFROW.dwInOctets` / `dwOutOctets` here, so `GetIfEntry()` returns correct values.

### Loopback Traffic Path

```
tcpip.sys  →  direct loopback (short-circuit)  →  receive path
                     ↑
              NDIS layer entirely skipped
              counters never updated
```

When data is sent to `127.0.0.1`, `tcpip.sys` **short-circuits at the transport layer** — packets are copied directly from the send path to the receive path, **never touching the NDIS layer**.

Windows' "Loopback Pseudo-Interface 1" is essentially a **placeholder entry**. It appears in the `GetIfTable` interface list (`dwType = IF_TYPE_SOFTWARE_LOOPBACK = 24`), but has no real NDIS miniport driver behind it doing any counting. The counters are always 0.

**This is not a "performance optimization" — it's an incomplete implementation.** Linux and macOS handle loopback through the full device path with negligible overhead. Microsoft simply never invested in making the loopback interface a proper NDIS device, and since no standard API can report loopback traffic, third-party tools (Npcap) had to fill the gap.

---

## Why Can Npcap Capture Loopback Traffic?

Npcap registers a **WFP (Windows Filtering Platform) callout driver** that intercepts packets **before** `tcpip.sys` performs the short-circuit, and copies them to userspace:

```
tcpip.sys send path
    ↓
  WFP callout (Npcap intercepts and copies here)
    ↓
  short-circuit loopback to receive path
```

This bypasses the NDIS layer entirely — WFP operates at hook points inside `tcpip.sys` itself.

That's why Npcap's installer has the **"Support loopback traffic capture"** checkbox — it enables this WFP callout driver and creates the `NPF_Loopback` virtual adapter for the `pcap` library to use.

---

## Comparison with Linux and macOS

### Linux

Linux's `lo` interface is a **real network device** with a full driver implementation (`drivers/net/loopback.c`):

```
App → socket → TCP/IP stack → dev_queue_xmit() → lo driver → netif_rx() → receive path
                                    ↑
                              lo driver updates stats normally here
```

The `loopback_xmit()` function properly updates `dev->stats`, so `/proc/net/dev` shows accurate `RX bytes` / `TX bytes` for `lo`.

```c
// Simplified Linux kernel loopback_xmit()
static netdev_tx_t loopback_xmit(struct sk_buff *skb, struct net_device *dev) {
    len = skb->len;
    dev->stats.tx_bytes += len;   // ← actually updated!
    dev->stats.tx_packets++;
    dev->stats.rx_bytes += len;
    dev->stats.rx_packets++;
    netif_rx(skb);                // deliver to receive path
    return NETDEV_TX_OK;
}
```

### macOS

macOS's `lo0` interface is similar to Linux — a **real BPF-capable network interface**. Loopback traffic goes through the `if_loop.c` driver, BPF can tap into it, and `netstat -I lo0` shows real counters.

macOS inherits BSD's design philosophy: loopback is an ordinary network interface, just without physical hardware.

---

## Design Philosophy Summary

|                          | Windows                              | Linux                            | macOS                        |
| ------------------------ | ------------------------------------ | -------------------------------- | ---------------------------- |
| Loopback implementation  | Short-circuit inside `tcpip.sys`     | `lo` device driver               | `lo0` device driver          |
| Goes through device layer? | ❌ Bypasses NDIS                   | ✅ Full `dev_queue_xmit` path    | ✅ Full `if_output` path     |
| Counters accurate?       | ❌ Always 0                         | ✅ Accurate                      | ✅ Accurate                  |
| Native BPF/pcap capture? | ❌ Requires Npcap WFP workaround    | ✅ Natively supported            | ✅ Natively supported        |
| Design philosophy        | Incomplete implementation            | Everything is a device           | BSD tradition, everything is an interface |

**The root cause is Windows' incomplete loopback implementation**, not a deliberate design tradeoff. Linux's `lo` driver adds only a handful of function calls to the loopback path — negligible overhead — yet provides full compatibility with all standard monitoring and capture tools. Microsoft simply never finished the job: the loopback pseudo-interface exists in the interface table but lacks a proper driver to back it.

---

## winload's Solution

winload uses **Npcap** as its Windows loopback capture backend:

- **`--npcap` (recommended)**: Captures real loopback packets via Npcap's WFP callout. Accurate data.
  Requires [Npcap](https://npcap.com/#download) installed with "Support loopback traffic capture" enabled.

> I previously experimented with polling `GetIfEntry` / `GetIfTable` counters directly, hoping to avoid the Npcap dependency. The result? Counters stubbornly stayed at 0 on every Windows version I tested. As detailed above, this is because the loopback pseudo-interface has no NDIS driver behind it — so there is simply nothing to count. I ended up removing that code path entirely. Thanks, Microsoft, for the *delightful* consistency.

So yes — to monitor loopback traffic on Windows, you need to install a third-party driver. On Linux and macOS it just works out of the box, because those operating systems treated loopback as a real network device from the start. On Windows, the [Npcap](https://npcap.com/#download) project graciously fills the gap that the OS left behind.

On Linux / macOS, loopback traffic is obtained directly via the [`sysinfo`](https://crates.io/crates/sysinfo) crate — no extra flags needed.
