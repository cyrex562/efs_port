// WARNING: Unknown calling convention yet parameter storage is locked

pub fn DirectDrawCreate(GUID *lp_guid,LPDIRECTDRAW *lp_lp_dd,IUnknown *p_unk_outer) -> HRESULT

{
    HRESULT HVar1;

    // WARNING: Could not recover jumptable at 0x004bb4ca. Too many branches
    // WARNING: Treating indirect jump as call
    HVar1 = DirectDrawCreate(lp_guid,lp_lp_dd,p_unk_outer);
    return HVar1;
}
