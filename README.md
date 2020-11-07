A simple process injection tool supporting injection of shared libraries.

## Usage example

```rust
use process_inject::process::{Process, ProcessEnumerator};
use process_inject::inject::inject_shared_library;

for record in ProcessEnumerator::new().expect("failed to enumerate processes") {
    let record = record.expect("failed to enumerate processes");
    if record.executable() == "target_process.exe" {
        let mut process = Process::open(record.pid())
            .expect("failed to open target process");
        unsafe {
            inject_shared_library(&mut process, Path::new("path/to/shared_library.dll"))
                .expect("failed to inject to target process");
        }
    }
}
```

## OS compatibility table

|      Operation       | Windows |
|:--------------------:|:-------:|
|  Process enumeration |    OK   |
|   Memory allocation  |    OK   |
|  Memory read / write |    OK   |
|    Code execution    |    OK   |
| Shared lib injection |    OK   |

OSes not listed in the table are not currently supported.
