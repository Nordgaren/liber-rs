# liber-rs
A reimplementation of [liber](https://github.com/Dasaav-dsv/libER) in Rust.

## Use
The user can "inherit" a class by making a new structure that is `#[repr(C)]` and has the base class type as the first
field of the structure. For example, to inherit `CSEzTask`, the user must provide a class that starts with a `CSEzTaskType`.


```rust
#[repr(C)]
#[derive(CSEzTask)]
pub struct MapTaskType {
    task: CSEzTaskType,
}
impl FD4TaskBaseTrait for MapTask {
    extern "C" fn execute(&self, data: &FD4TaskData) {
        self.eztask_execute(data)
    }
}

impl FD4ComponentBaseTrait for MapTask {}

impl CSEzTaskTrait for MapTask {
    extern "C" fn eztask_execute(&self, data: &FD4TaskData) {
        info!("CSEzTask: Hello from Rust! {data:?}");
        if let Some(tx) = MAP_TASK_TX.get() {
            send_inventory_data(tx);
        }
    }
}

fn send_inventory_data(tx: &Sender<AgentUpdate>) {
    if let Some(map_data) = get_map_data() {
        println!("{map_data:?}");
        tx.send(map_data.into_update()).unwrap_or_default()
    }
}


```

## License
Permissive Apache 2.0 with LLVM exception.  
