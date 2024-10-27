use steel::*;

use super::RentExampleAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Counter {
    pub value: u64 
}

account!(RentExampleAccount, Counter);
