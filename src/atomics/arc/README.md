# Arc

- youtube: [send is not about ownership](https://www.youtube.com/watch?v=eRxqX5_UxaY) by Alice Ryhl

## Test

### Weak

```bash
cargo test weak::tests -- --nocapture
```

```bash
Conunts: (1, 2)
Conunts: (1, 3)
Conunts: (1, 3)
Conunts: (1, 3)
Conunts: (2, 4)
Arc drop
Weak drop
Weak drop
Arc drop
Weak drop
Conunts: (1, 2)
drop x
Arc drop
DetectDrop drop
Weak drop
---
Conunts: (0, 1)
Conunts: (0, 1)
=== end ===
Weak drop
Weak remove: 0x7c928c000c40
```

