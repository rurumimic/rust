# Arc

- youtube: [send is not about ownership](https://www.youtube.com/watch?v=eRxqX5_UxaY) by Alice Ryhl

## Test

```bash
cargo test
```

```bash
running 5 tests
test weak::tests::test_option ... ok
test basic::tests::test_mut ... ok
test weak::tests::test_weak ... ok
test basic::tests::test ... ok
test optimized::tests::test ... ok

test result: ok. 5 passed;
```

### Optimized

```bash
cargo test optimized -- --nocapture
```

```bash
=== Test Start ===
Num drops: 0
new arc x
[src/optimized.rs:238:9] &x = Arc {
    data_ref (arc): 1,
    alloc_ref (weak + (1)): 1,
}
new weak x->y
[src/optimized.rs:242:9] &y = Weak {
    data_ref (arc): 1,
    alloc_ref (weak + (1)): 2,
}
new weak x->z
[src/optimized.rs:246:9] &z = Weak {
    data_ref (arc): 1,
    alloc_ref (weak + (1)): 3,
}
main thread
[src/optimized.rs:260:9] &z = Weak {
    data_ref (arc): 1,
    alloc_ref (weak + (1)): 3,
}
weak y to new thread
[src/optimized.rs:250:13] &y = Weak {
    data_ref (arc): 1,
    alloc_ref (weak + (1)): 3,
}
new arc w<-y in new thread
[src/optimized.rs:254:13] &y = Weak {
    data_ref (arc): 2,
    alloc_ref (weak + (1)): 3,
}
Arc drop
[src/optimized.rs:180:9] &self.ptr = 0x000075d0d4000c40
Weak drop
[src/optimized.rs:165:9] &self = Weak {
    data_ref (arc): 1,
    alloc_ref (weak + (1)): 3,
}
thread end
[src/optimized.rs:263:9] &x = Arc {
    data_ref (arc): 1,
    alloc_ref (weak + (1)): 2,
}
[src/optimized.rs:264:9] &z = Weak {
    data_ref (arc): 1,
    alloc_ref (weak + (1)): 2,
}
new arc <- z
Arc drop
[src/optimized.rs:180:9] &self.ptr = 0x000075d0d4000c40
[src/optimized.rs:270:9] &z = Weak {
    data_ref (arc): 1,
    alloc_ref (weak + (1)): 2,
}
drop x
Arc drop
[src/optimized.rs:180:9] &self.ptr = 0x000075d0d4000c40
Drop: DetectDrop 1
Weak drop
[src/optimized.rs:165:9] &self = Weak {
    data_ref (arc): 0,
    alloc_ref (weak + (1)): 2,
}
---
new arc <- z
[src/optimized.rs:280:13] &z = Weak {
    data_ref (arc): 0,
    alloc_ref (weak + (1)): 1,
}
new arc <- z
[src/optimized.rs:285:9] &z = Weak {
    data_ref (arc): 0,
    alloc_ref (weak + (1)): 1,
}
=== Test End ===
Weak drop
[src/optimized.rs:165:9] &self = Weak {
    data_ref (arc): 0,
    alloc_ref (weak + (1)): 1,
}
Weak remove: 0x75d0d4000c40
```

### Weak

```bash
cargo test weak::tests::test_weak -- --nocapture
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

