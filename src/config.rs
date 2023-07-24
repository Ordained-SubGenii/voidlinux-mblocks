use crate::block::Block;
#[allow(unused_imports)]
use crate::block::BlockType::{Once, Periodic, Signal, PeriodicOrSignal};
#[allow(unused_imports)]
use crate::block::CommandType::{Function, Shell};
use crate::blocks::cpu::cpu_usage;
use crate::blocks::datetime::{current_time, current_date};
//use crate::blocks::memory::memory_usage;

pub const SEPARATOR: &str = " | ";
pub const PREFIX: &str = " ";
pub const SUFFIX: &str = " ";

pub const BLOCKS: &[Block] = &[
    Block {
        kind: Periodic(60),
        command: Shell(&["cat", "/sys/class/net/wlan0/operstate"]),
        prefix: " : ",
        suffix: "",
    },
    Block {
        kind: Periodic(10),
        command: Function(cpu_usage),
        prefix: " : ",
        suffix: "%",
    },
    //Block {
    //    kind: Periodic(40),
    //    command: Function(memory_usage),
    //    prefix: "MEM: ",
    //    suffix: "",
    //},
    Block {
        kind: Periodic(50),
        command: Shell(&["/home/nullifier/bin/battstatbar"]),
        prefix: "",
        suffix: "%",
    },
    Block {
        kind: Signal(5),
        command: Shell(&["/home/nullifier/bin/volumestatbar"]),
        prefix: ": ",
        suffix: "",
    },    
    //Block {
    //    kind: Signal(5),
    //    command: Shell(&["wpctl", "get-volume", "@DEFAULT_SINK@"]),
    //    prefix: "",
    //    suffix: "",
    //},    
    Block {
        kind: Periodic(60),
        command: Function(current_date),
        prefix: "",
        suffix: "",
    },
    Block {
        kind: Periodic(30),
        command: Function(current_time),
        prefix: "",
        suffix: "",
    },
    //Block {
    //    kind: Once,
    //    command: Shell(&["whoami"]),
    //    prefix: "",
    //    suffix: "",
    //},
    //Block {
    //    kind: Once,
    //    command: Shell(&["dwm", "-v"]),
    //    prefix: "",
    //    suffix: "",
    //},
];
