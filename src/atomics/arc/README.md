# Arc

- youtube: [send is not about ownership](https://www.youtube.com/watch?v=eRxqX5_UxaY) by Alice Ryhl

## Test

### Weak

```bash
cargo test weak -- --nocapture
```

```bash
=== Test Start ===
Num drops: 0
new arc x
[src/weak.rs:200:9] &x = Arc {
    weak: Weak {
        data_ref (arc): 1,
        alloc_ref (arc + weak): 1,
    },
}
new weak x->y
[src/weak.rs:204:9] &y = Weak {
    data_ref (arc): 1,
    alloc_ref (arc + weak): 2,
}
new weak x->z
[src/weak.rs:208:9] &z = Weak {
    data_ref (arc): 1,
    alloc_ref (arc + weak): 3,
}
main thread
[src/weak.rs:222:9] &z = Weak {
    data_ref (arc): 1,
    alloc_ref (arc + weak): 3,
}
weak y to new thread
[src/weak.rs:212:13] &y = Weak {
    data_ref (arc): 1,
    alloc_ref (arc + weak): 3,
}
new arc w<-y in new thread
[src/weak.rs:216:13] &y = Weak {
    data_ref (arc): 2,
    alloc_ref (arc + weak): 4,
}
Arc drop
[src/weak.rs:148:9] &self.weak = Weak {
    data_ref (arc): 2,
    alloc_ref (arc + weak): 4,
}
Weak drop
[src/weak.rs:133:9] &self = Weak {
    data_ref (arc): 1,
    alloc_ref (arc + weak): 4,
}
Weak drop
[src/weak.rs:133:9] &self = Weak {
    data_ref (arc): 1,
    alloc_ref (arc + weak): 3,
}
thread end
[src/weak.rs:225:9] &x = Arc {
    weak: Weak {
        data_ref (arc): 1,
        alloc_ref (arc + weak): 2,
    },
}
[src/weak.rs:226:9] &z = Weak {
    data_ref (arc): 1,
    alloc_ref (arc + weak): 2,
}
new arc <- z
Arc drop
[src/weak.rs:148:9] &self.weak = Weak {
    data_ref (arc): 2,
    alloc_ref (arc + weak): 3,
}
Weak drop
[src/weak.rs:133:9] &self = Weak {
    data_ref (arc): 1,
    alloc_ref (arc + weak): 3,
}
[src/weak.rs:232:9] &z = Weak {
    data_ref (arc): 1,
    alloc_ref (arc + weak): 2,
}
drop x
Arc drop
[src/weak.rs:148:9] &self.weak = Weak {
    data_ref (arc): 1,
    alloc_ref (arc + weak): 2,
}
Drop: DetectDrop 1
Weak drop
[src/weak.rs:133:9] &self = Weak {
    data_ref (arc): 0,
    alloc_ref (arc + weak): 2,
}
---
new arc <- z
[src/weak.rs:242:13] &z = Weak {
    data_ref (arc): 0,
    alloc_ref (arc + weak): 1,
}
new arc <- z
[src/weak.rs:247:9] &z = Weak {
    data_ref (arc): 0,
    alloc_ref (arc + weak): 1,
}
=== Test End ===
Weak drop
[src/weak.rs:133:9] &self = Weak {
    data_ref (arc): 0,
    alloc_ref (arc + weak): 1,
}
Weak remove: 0x73c7ac000c40
```

