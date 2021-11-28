use libc;
use winapi::{self, shared::guiddef::GUID, um::{winnt::HRESULT, unknwnbase::IUnknown}};

pub struct IDrectDraw {}

pub fn DirectDrawCreate(
    lp_guid: &GUID,
    lp_lp_dd: &mut IDrectDraw,
    p_unk_outer: &mut IUnknown,
) -> HRESULT {
    let mut HVar1: HRESULT;

    // WARNING: Could not recover jumptable at 0x004bb4ca. Too many branches
    // WARNING: Treating indirect jump as call
    // HVar1 = DirectDrawCreate(lp_guid, lp_lp_dd, p_unk_outer);
    unimplemented!();
    HVar1
}
