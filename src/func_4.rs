
fn FUN_004087c0(param_1: *mut *mut u32,param_2: i32)

{
    let mut pcVar1: String;

    if ((param_2 + 0xe) == 0x17) {
        pcVar1 = FUN_00499050(DAT_005967a0,0x576);
        FUN_0049bf80(param_1,0x52c,0x40f,0x0,pcVar1);
        FUN_0049bf80(param_1,0x52c,0x414,0x1,0x0);
    }
    else {
        FUN_0049bf80(param_1,0x52c,0x414,0x0,0x0);
        FUN_0049bf80(param_1,0x52c,0x410,0x0,0x0);
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0040885a(param_1: *mut *mut u32,param_2: u32,param_3: u32,param_4: *mut i32) -> u32

{
    let mut iVar1: i32;
    u32 **ppuVar2;
    let mut puVar3: *mut u8;
    let local_2c: u8;

    if (param_2 < 0x406) {
        if (param_2 < 0x401) {
            if (param_2 == 0x113) {
                ppuVar2 = (u32 **)FUN_0049ab40();
                if (ppuVar2 != param_1) {
                    return 0x1;
                }
                FUN_00496263((&DAT_004bf7e0 + DAT_005b8848 * 0xc),0xa,0x4);
                iVar1 = DAT_005b8848;
                DAT_005b8848 = DAT_005b8848 + 0x1;
                if (iVar1 == 0x8) {
                    DAT_005b8848 = 0x0;
                }
                _DAT_004c5098 = _DAT_004c5098 ^ 0x1;
                FUN_00447607(DAT_004c509c,*(DAT_004c5090 + 0x8) >> 0x10,*(DAT_004c5090 + 0xa) >> 0x10);
                if (_DAT_004c5098 == 0x0) {
                    local_2c = 0xe;
                }
                else {
                    local_2c = 0xe4;
                }
                FUN_004968e7(*(DAT_004c509c + 0x85),*(DAT_004c509c + 0x89),0x2,0x2,local_2c);
                return 0x1;
            }
        }
        else {
            if (param_2 < 0x402) {
                _DAT_004c5098 = 0x0;
                _DAT_004c5094 = 0x12c;
                FUN_00408d1c(param_1,0xc8,0x12c);
                DAT_004c5090 = FUN_0049bf80(param_1,0xc8,0x510,0x0,0x0);
                FUN_004087c0(param_1,DAT_004c5090);
                return 0x0;
            }
            if (param_2 == 0x405) {
                FUN_004953d7();
                FUN_00496ac0((&DAT_00595700 + DAT_004c9754 * 0x4),0x1e,0xf8,0x64,0x64);
                FUN_0049e640(0x1e,0xf8,0x64,0x64,0xce,0xca,0xcc,0x1);
                FUN_0049e640(*(DAT_004c509c + 0x1d),*(DAT_004c509c + 0x21),*(DAT_004c509c + 0x25),
                             *(DAT_004c509c + 0x29),0xce,0xca,0xcc,0x1);
                FUN_0049e640(*(DAT_004c50a0 + 0x1d),*(DAT_004c50a0 + 0x21),*(DAT_004c50a0 + 0x25),
                             *(DAT_004c50a0 + 0x29),0xce,0xca,0xcc,0x1);
                FUN_004093de(param_1);
                FUN_0049536f();
                return 0x0;
            }
        }
    }
    else {
        if (param_2 < 0x407) {
            if (param_3 == 0xc8) {
                puVar3 = FUN_0049bf80(param_1,0xc8,0x510,param_4,0x0);
                FUN_004093de(param_1);
                if ((DAT_004c5090 != 0x0) && ((puVar3 + 0x8) != (DAT_004c5090 + 0x8))) {
                    FUN_004390ae(DAT_004c509c,*(puVar3 + 0x6) >> 0x10);
                    DAT_005b7068 = *(puVar3 + 0x6) >> 0x10;
                    DAT_005b8808 = 0x0;
                }
                FUN_00439d27(DAT_004c509c,0x0);
                DAT_004c5090 = puVar3;
                FUN_004087c0(param_1,puVar3);
            }
            return 0x0;
        }
        if (param_2 < 0x411) {
            if (param_2 == 0x407) {
                if (param_3 < 0x12c) {
                    if (0x63 < param_3) {
                        if (param_3 < 0x65) {
                            FUN_0049c140(param_1,0x0);
                            return 0x0;
                        }
                        if (param_3 == 0xc8) {
                            if ((DAT_004c5090 + 0xe) == 0x17) {
                                FUN_0044c1ce(DAT_004c5090);
                            }
                            return 0x0;
                        }
                    }
                }
                else {
                    if (((param_3 < 0x12d) || (param_3 < 0x12e)) || (param_3 < 0x12f)) {
                        if (param_3 != _DAT_004c5094) {
                            _DAT_004c5094 = param_3;
                            FUN_00408d1c(param_1,0xc8,param_3);
                        }
                        return 0x0;
                    }
                    if (param_3 == 0x52c) {
                        if ((DAT_004c5090 + 0xe) == 0x17) {
                            FUN_0044c1ce(DAT_004c5090);
                        }
                        return 0x0;
                    }
                }
                return 0x0;
            }
        }
        else {
            if (param_2 < 0x412) {
                return 0x1;
            }
            if (param_2 == 0x412) {
                FUN_00408e35(param_4);
                return 0x1;
            }
        }
    }
    return 0x0;
}



fn FUN_00408d1c(param_1: *mut *mut u32,param_2: i32,param_3: u32)

{
    let mut pcVar1: String;
    let puVar2: *mut u32;

    FUN_0049bf80(param_1,param_2,0x507,0x0,0x0);
    DAT_004c50a4 = FUN_0049ea90();
    if (DAT_004c50a4 == 0x0) {
        pcVar1 = FUN_00499050(DAT_005b9bd8,0x7d01);
        pop_err_msg_box_and_exit_004a02f5(pcVar1);
    }
    for (puVar2 = *DAT_005967c8; puVar2 != 0x0; puVar2 = *puVar2) {
        if ((*(&DAT_004be930 + (puVar2[0x3] >> 0x10) * 0x4) == 0x0) &&
            (*(puVar2 + 0xe) >> 0x10 == DAT_004c9754)) {
            FUN_0049eb40(DAT_004c50a4,puVar2);
        }
    }
    FUN_0040910d(DAT_004c50a4,param_3,FUN_00409207);
    FUN_0049bf80(param_1,param_2,0x503,0x0,DAT_004c50a4);
    FUN_0049bf80(param_1,param_2,0x509,0x0,0x20);
    FUN_0049bf80(param_1,param_2,0x502,0x0,0x0);
    return;
}



fn FUN_00408e35(param_1: *mut i32)

{
    let mut pcVar1: String;
    let mut iVar2: i32;
    let mut uVar3: u32;
    let mut iVar4: i32;
    let mut uVar5: u32;
    let puVar6: *mut u32;
    let bVar7: u8;
    let local_120: u8 [0x100];
    let local_20: *mut i32;;
    let mut local_1c: i32;
    let mut local_18: u32;
    let mut local_14: i32;

    local_20 = param_1;
    local_14 = param_1[0x5];
    if (param_1[0x4] == 0x1) {
        local_18 = 0x272727;
    }
    else {
        local_18 = 0xcaccce;
    }
    local_1c = 0xe0e0e;
    FUN_004953d7();
    if ((local_14 + 0x14) == -0x1) {
        FUN_004968e7(*local_20,local_20[0x1],local_20[0x2],0x20,0xe);
        bVar7 = 0x10;
        uVar5 = 0xcaccce;
        iVar2 = 0xa8;
        uVar3 = local_18;
        iVar4 = local_1c;
        puVar6 = LPCSTR_005b9218;
        pcVar1 = FUN_00499050(DAT_0059679c,0x7145);
        FUN_00497567(*local_20 + 0x20,local_20[0x1],pcVar1,iVar2,uVar3,iVar4,uVar5,puVar6,bVar7);
    }
    else {
        FUN_004968e7(*local_20 + 0x20,local_20[0x1],local_20[0x2] + -0x20,0x20,0xe);
        FUN_00496ac0((&DAT_004d6058 + (*(local_14 + 0x12) >> 0x10) * 0x1c),*local_20,local_20[0x1],
                     0x20,0x20);
        FUN_00497567(*local_20 + 0x20,local_20[0x1],
                     *(&DAT_00582938 +
                         (*(local_14 + 0x14) >> 0x10) * 0x4 + (*(local_14 + 0x12) >> 0x10) * 0x18),0xa8,
                     local_18,local_1c,0xcaccce,LPCSTR_005b9218,0x10);
    }
    FUN_00497567(*local_20 + 0xc8,local_20[0x1],(&DAT_005831e8 + (*(local_14 + 0xc) >> 0x10) * 0x50),0x78,
                 local_18,local_1c,0xcaccce,LPCSTR_005b9218,0x10);
    FUN_00497567(*local_20 + 0x140,local_20[0x1],(&DAT_005b709e + (*(local_14 + 0x6) >> 0x10) * 0x4e),
                 local_20[0x2] + -0x140,local_18,local_1c,0xcaccce,LPCSTR_005b9218,0x10);
    if ((local_14 + 0x18) != -0x1) {
        if ((local_14 + 0x18) == 0x1) {
            pcVar1 = FUN_00499050(DAT_0059679c,0x741b);
            FUN_0049c2e0(local_120,pcVar1);
        }
        else {
            pcVar1 = FUN_00499050(DAT_0059679c,0x741c);
            FUN_0049c2e0(local_120,pcVar1);
        }
        FUN_00497567(*local_20 + 0x20,local_20[0x1] + 0xe,local_120,local_20[0x2] + -0x20,local_18,local_1c,0xcaccce,
                     LPCSTR_005b9218,0x10);
    }
    FUN_0049536f();
    return;
}



fn FUN_0040910d(param_1: i32,param_2: u32,param_3: *mut u8)

{
    let mut bVar1: bool;
    let mut iVar2: i32;
    let local_20: *mut u32;
    let local_18: *mut u32;

    if (*(param_1 + 0x12) == param_1) {
        bVar1 = false;
        while (!bVar1) {
            bVar1 = true;
            local_20 = (param_1 + 0x8);
            while ((local_20[0x2] != 0x0 && (local_20[0x2] != param_1))) {
                local_18 = local_20[0x2];
                iVar2 = (param_3)(*local_20[0x2],*local_20,param_2);
                if (iVar2 < 0x0) {
                    iVar2 = local_20[0x2];
                    local_20[0x2] = *(local_20[0x2] + 0x8);
                    (local_20[0x2] + 0x4) = local_20;
                    *(iVar2 + 0x4) = local_20[0x1];
                    *(*(iVar2 + 0x4) + 0x8) = iVar2;
                    (iVar2 + 0x8) = local_20;
                    local_20[0x1] = iVar2;
                    local_18 = local_20;
                    bVar1 = false;
                }
                local_20 = local_18;
            }
        }
    }
    return;
}



undefined8  FUN_00409207(param_1: u32,param_2: u32,param_3: i32,param_4: i32,param_5: u32)

{
let cVar1: u8;
let mut pcVar2: String;
let puVar3: *mut u32;
ulonglong uVar4;
let mut local_a0: u32;
let mut local_60: u32;
let mut local_20: u32;
let mut local_1c: i32;
let mut local_18: i32;
let mut local_14: u32;

local_1c = param_3;
local_18 = param_4;
local_14 = param_5;
if (param_5 < 0x12d) {
if (param_5 == 0x12c) {
if ((param_3 + 0x14) == -0x1) {
pcVar2 = &DAT_004c07de;
puVar3 = &local_60;
loop {
cVar1 = *pcVar2;
puVar3 = cVar1;
if (cVar1 == '\0') break;
cVar1 = pcVar2[0x1];
pcVar2 = pcVar2 + 0x2;
(puVar3 + 0x1) = cVar1;
puVar3 = (puVar3 + 0x2);
} while (cVar1 != '\0');
}
else {
pcVar2 = *(&DAT_00582938 +
(*(param_3 + 0x12) >> 0x10) * 0x18 + (*(param_3 + 0x14) >> 0x10) * 0x4);
puVar3 = &local_60;
loop {
cVar1 = *pcVar2;
puVar3 = cVar1;
if (cVar1 == '\0') break;
cVar1 = pcVar2[0x1];
pcVar2 = pcVar2 + 0x2;
(puVar3 + 0x1) = cVar1;
puVar3 = (puVar3 + 0x2);
} while (cVar1 != '\0');
}
if ((local_18 + 0x14) == -0x1) {
pcVar2 = &DAT_004c07e2;
puVar3 = &local_a0;
loop {
cVar1 = *pcVar2;
puVar3 = cVar1;
if (cVar1 == '\0') break;
cVar1 = pcVar2[0x1];
pcVar2 = pcVar2 + 0x2;
(puVar3 + 0x1) = cVar1;
puVar3 = (puVar3 + 0x2);
} while (cVar1 != '\0');
}
else {
pcVar2 = *(&DAT_00582938 +
(*(local_18 + 0x12) >> 0x10) * 0x18 + (*(local_18 + 0x14) >> 0x10) * 0x4);
puVar3 = &local_a0;
loop {
cVar1 = *pcVar2;
puVar3 = cVar1;
if (cVar1 == '\0') break;
cVar1 = pcVar2[0x1];
pcVar2 = pcVar2 + 0x2;
(puVar3 + 0x1) = cVar1;
puVar3 = (puVar3 + 0x2);
} while (cVar1 != '\0');
}
uVar4 = FUN_004a2ae0(param_1,param_2,&local_60,&local_a0);
param_2 = (uVar4 >> 0x20);
local_20 = uVar4;^
// goto LAB_004093d3;
}
}
else {
if (param_5 < 0x12e) {
uVar4 = FUN_004a2ae0(param_1,&DAT_005831d8,
(&DAT_005831e8 + (*(param_3 + 0xc) >> 0x10) * 0x50),
(&DAT_005831e8 + (*(param_4 + 0xc) >> 0x10) * 0x50));
param_2 = (uVar4 >> 0x20);
local_20 = uVar4;^
// goto LAB_004093d3;
}
if (param_5 == 0x12e) {
uVar4 = FUN_004a2ae0(param_1,&DAT_005b7078,
(&DAT_005b709e + (*(param_3 + 0x6) >> 0x10) * 0x4e),
(&DAT_005b709e + (*(param_3 + 0x6) >> 0x10) * 0x4e));
param_2 = (uVar4 >> 0x20);
local_20 = uVar4;^
// goto LAB_004093d3;
}
}
local_20 = 0x0;
// LAB_004093d3:
return CONCAT44(param_2,local_20);
}



fn FUN_004093de(param_1: *mut *mut u32)

{
    u32 local_798 [0x1e0];
    let mut local_18: *mut u8;
    let mut local_14: *mut u8;

    local_18 = FUN_0049bf80(param_1,0xc8,0x501,0x0,0x0);
    if (local_18 != 0xffffffff) {
        local_14 = FUN_0049bf80(param_1,0xc8,0x510,local_18,0x0);
        FUN_004968e7(0x1e,0x17c,0x23a,0x28,0xe);
        FUN_0049e640(0x1e,0x17c,0x23a,0x28,0xce,0xca,0xcc,0x1);
        FUN_00401010(local_798,

                     (*(&DAT_004daab0 + (*(local_14 + 0x6) >> 0x10) * 0x3890) * 0x940 + 0x565c90 +
                         (*(local_14 + 0xc) >> 0x10) * 0x4),0x30,0x28,0x1c);
        FUN_00496ee6(local_798,0x1e,0x17c,0x30,0x28);
        FUN_00497567(0x55,0x17c,(&DAT_005831e8 + (*(local_14 + 0xc) >> 0x10) * 0x50),0x64,0xcaccce,-0x1,0x272727
                     ,LPCSTR_005b9218,0x10);
        FUN_0043667f(*(local_14 + 0xc) >> 0x10,0x55,0x188,*(local_14 + 0x6) >> 0x10,
                     *(local_14 + 0x8) >> 0x10,*(local_14 + 0xa) >> 0x10,0x0,0x1);
    }
    return;
}



fn FUN_00409581(param_1: *mut u32,param_2: i32,param_3: i32,param_4: i32) -> u32

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut local_20: u32;

    if ((param_2 < 0xa) || (param_3 < 0xd7)) {
        local_20 = 0x0;
    }
    else {
        *param_1 = 0x0;
        iVar1 = ((param_2 + (param_2 >> 0x1f) * -0x80) - ((param_2 >> 0x1f) << 0x6 < 0x0)) >> 0x7;
        iVar2 = param_2 + -0xa >> 0x1f;
        iVar2 = ((param_2 + -0xa + iVar2 * -0x80) - (iVar2 << 0x6 < 0x0)) >> 0x7;
        if ((iVar1 == iVar2) && (iVar1 < 0x5)) {
            param_1[0x1] = iVar2 + ((param_3 + -0xd7) / 0x29) * 0x5;
            if (param_1[0x1] < 0x14) {
                if (param_4 == 0x0) {
                    *param_1 = 0x2;
                }
                else {
                    *param_1 = 0x1;
                }
                local_20 = 0x1;
            }
            else {
                param_1[0x1] = 0xffffffff;
                local_20 = 0x0;
            }
        }
        else {
            local_20 = 0x0;
        }
    }
    return local_20;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00409663(param_1: i32,param_2: u32,param_3: i32,param_4: u32,param_5: i32,param_6: i32) -> u32

{
    byte *pbVar1;
    short *psVar2;
    ushort uVar3;
    let mut pcVar4: String;
    i32 **ppiVar5;
    let mut iVar6: i32;
    let puVar7: *mut u32;
    let puVar8: *mut u32;
    let in_stack_fffffe98: i16;
    let mut in_stack_fffffe9c: u32;
    u32 local_114 [0xc];
    let mut local_e4: u32;
    let uStack224: u8;
    let local_d3: u16;
    let mut local_ca: u32;
    let mut local_b4: i32;
    let mut local_b0: i32;
    let mut local_ac: i32;
    let mut local_a8: i32;
    let mut local_a4: i32;
    let mut local_a0: i32;
    let mut local_9c: u32;
    let mut local_98: i32;
    let mut local_94: i32;
    let mut local_90: i32;
    let mut local_8c: i32;
    let mut local_88: i32;
    let mut local_84: i32;
    let mut local_80: i32;
    let mut local_7c: i32;
    let mut local_78: i32;
    let mut local_74: i32;
    let mut local_70: i32;
    let mut local_6c: i32;
    let mut local_68: *mut u8;
    let mut local_64: i32;
    let mut local_60: i32;
    let mut local_5c: i32;
    let mut local_58: u32;
    let mut local_54: i32;
    let mut local_50: i32;
    let mut local_4c: i32;
    let mut local_48: i32;
    let mut local_44: i32;
    let mut local_40: *mut u8;
    let mut local_3c: i32;
    let mut local_38: i32;
    let mut local_34: i32;
    let mut local_30: u32;
    let mut local_2c: u32;
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: u32;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: u32;

    local_2c = 0x0;
    local_1c = param_1;
    local_18 = param_3;
    uVar3 = (param_3 * 0x5e + param_1 + 0x4);
    local_14 = uVar3 & 0x1;
    if ((uVar3 & 0x1) == 0x0) {
        local_30 = 0x0;
    }
    else {
        local_34 = param_1;
        local_38 = param_3;
        local_3c = *(*(param_3 * 0x5e + param_1) + 0x24) >> 0x18;
        local_40 = &DAT_00582938 + local_3c * 0x18;
        local_44 = param_1;
        local_48 = param_3;
        local_4c = *(*(param_3 * 0x5e + param_1) + 0x25) >> 0x18;
        local_28 = *(local_40 + local_4c * 0x4);
        if (*(local_28 + 0xa5) == 0x0) {
            local_50 = param_1;
            local_54 = param_5;
            uVar3 = (param_5 * 0x5e + param_1 + 0x4);
            local_58 = uVar3 & 0x1;
            if ((uVar3 & 0x1) != 0x0) {
                local_5c = param_1;
                local_60 = param_5;
                local_64 = *(*(param_5 * 0x5e + param_1) + 0x24) >> 0x18;
                local_68 = &DAT_00582938 + local_64 * 0x18;
                local_6c = param_1;
                local_70 = param_5;
                local_74 = *(*(param_5 * 0x5e + param_1) + 0x25) >> 0x18;
                local_24 = *(local_68 + local_74 * 0x4);
                local_78 = param_1;
                local_7c = param_3;
                local_80 = *(*(param_3 * 0x5e + param_1) + 0x24) >> 0x18;
                if (local_80 == 0x5b) {
                    local_84 = param_1;
                    local_88 = param_5;
                    local_8c = *(*(param_5 * 0x5e + param_1) + 0x24) >> 0x18;
                    if ((local_8c == 0x5b) &&
                        ((*(param_3 * 0x5e + param_1) + 0x30) ==
                            (*(param_5 * 0x5e + param_1) + 0x30))) {
                        if (param_6 == 0x0) {
                            local_20 = *(*(param_3 * 0x5e + param_1) + 0x2f) >> 0x10;
                        }
                        else {
                            local_20 = FUN_0040a2cc(param_3,param_5);
                        }
                        if (local_20 == 0x0) {
                            return 0x0;
                        }
                        psVar2 = (short *)(*(param_3 * 0x5e + param_1) + 0x31);
                        *psVar2 = *psVar2 - local_20;
                        psVar2 = (short *)(*(param_5 * 0x5e + param_1) + 0x31);
                        *psVar2 = *psVar2 + local_20;
                        if ((*(param_3 * 0x5e + param_1) + 0x2f) <
                            (*(param_5 * 0x5e + param_1) + 0x2f)) {
                            (*(param_5 * 0x5e + param_1) + 0x2f) =
                                (*(param_3 * 0x5e + param_1) + 0x2f);
                        }
                        if ((*(param_3 * 0x5e + param_1) + 0x31) == 0x0) {
                            if ((*(param_3 * 0x5e + param_1 + 0x4) & 0x4) != 0x0) {
                                FUN_0040a011(param_1,param_3);
                            }
                            pbVar1 = (param_3 * 0x5e + param_1 + 0x4);
                            *pbVar1 = *pbVar1 & 0xfe;
                            pbVar1 = (*(param_3 * 0x5e + param_1) + 0x3d);
                            *pbVar1 = *pbVar1 | 0x80;
                            _DAT_005b8be4 = 0x1;
                            *(param_3 * 0x5e + param_1) = 0x0;
                            return 0x1;
                        }
                        return 0x0;
                    }
                }
                if ((*(param_5 * 0x5e + param_1 + 0x4) & 0x4) != 0x0) {
                    FUN_004a2d6b();
                    pcVar4 = FUN_00499050(DAT_0059679c,0x73c1);
                    FUN_0049d2e0(0x0,0x1,pcVar4);
                    return 0x0;
                }
                if (*(local_24 + 0xa5) != 0x0) {
                    if (*(local_28 + 0xa9) == 0x0) {
                        FUN_004a2d6b();
                        pcVar4 = FUN_00499050(DAT_0059679c,0x7156);
                        FUN_0049d2e0(0x0,0x1,pcVar4);
                        return 0x0;
                    }
                    if (((*(local_28 + 0x41) == 0x3) && (*(local_24 + 0x41) == 0x4)) &&
                        ((*(param_5 * 0x5e + param_1) + 0x27) != '\x1a')) {
                        FUN_004a2d6b();
                        pcVar4 = FUN_00499050(DAT_0059679c,0x73f3);
                        FUN_0049d2e0(0x0,0x1,pcVar4);
                        return 0x0;
                    }
                    local_90 = 0x0;
                    while ((local_90 < *(local_24 + 0xa5) &&
                        (*(*(param_5 * 0x5e + param_1) + local_90 * 0x4 + 0x10) != 0x0))) {
                        local_90 = local_90 + 0x1;
                    }
                    if (local_90 == *(local_24 + 0xa5)) {
                        FUN_004a2d6b();
                        pcVar4 = FUN_00499050(DAT_0059679c,0x73ac);
                        FUN_0049d2e0(0x0,0x1,pcVar4);
                        return 0x0;
                    }
                    if ((*(param_3 * 0x5e + param_1 + 0x4) & 0x4) != 0x0) {
                        FUN_0040a011(param_1,param_3);
                    }
                    FUN_0040a674(*(param_5 * 0x5e + param_1));
                    FUN_0040a121(param_1,param_5,param_3);
                    return 0x1;
                }
                if ((*(param_3 * 0x5e + param_1 + 0x4) & 0x4) != 0x0) {
                    FUN_004a2d6b();
                    pcVar4 = FUN_00499050(DAT_0059679c,0x73c1);
                    FUN_0049d2e0(0x0,0x1,pcVar4);
                    return 0x0;
                }
            }
            if ((*(param_3 * 0x5e + param_1 + 0x4) & 0x4) != 0x0) {
                iVar6 = *(param_3 * 0x5e + param_1);
                local_98 = iVar6 + 0x20;
                local_9c = *(iVar6 + 0x3a) & 0x1;
                local_94 = local_98;
                if (local_9c == 0x0) {
                    local_a0 = param_1;
                    local_a4 = param_3;
                    local_a8 = *(*(param_3 * 0x5e + param_1) + 0x28) >> 0x10;
                    if (local_a8 != 0x5) {
                        FUN_004a2d6b();
                        pcVar4 = FUN_00499050(DAT_0059679c,0x739a);
                        FUN_0049d2e0(0x0,0x1,pcVar4);
                        return 0x0;
                    }
                }
                else {
                    if (((*((*(param_1 + 0x816) >> 0x10) * 0x4 +
                        *(&DAT_004d7d50 +
                            (*(param_1 + 0x812) >> 0x10) * 0x3890 +
                            (*(param_1 + 0x814) >> 0x10) * 0x4)) & 0xf) == 0x0) &&
                        ((*((*(param_1 + 0x816) >> 0x10) * 0x4 +
                            *(&DAT_004d7d50 +
                                (*(param_1 + 0x812) >> 0x10) * 0x3890 +
                                (*(param_1 + 0x814) >> 0x10) * 0x4) + 0x4) & 0x80) == 0x0)) {
                        FUN_004a2d6b();
                        pcVar4 = FUN_00499050(DAT_0059679c,0x739d);
                        FUN_0049d2e0(0x0,0x1,pcVar4);
                        return 0x0;
                    }
                }
            }
            local_ac = param_1;
            local_b0 = param_3;
            local_b4 = *(*(param_3 * 0x5e + param_1) + 0x24) >> 0x18;
            if (local_b4 == 0x5b) {
                if (param_6 == 0x0) {
                    local_20 = *(*(param_3 * 0x5e + param_1) + 0x2f) >> 0x10;
                }
                else {
                    local_20 = FUN_0040a2cc(param_3,param_5);
                }
                if (local_20 == 0x0) {
                    local_30 = 0x0;
                }
                else {
                    FUN_00486065(&local_e4);
                    iVar6 = *(param_3 * 0x5e + param_1);
                    local_e4 = *(iVar6 + 0x20);
                    uStack224 = (iVar6 + 0x24);
                    local_d3 = local_20;
                    local_ca = local_ca & 0xffffffbf;
                    puVar7 = &local_e4;
                    puVar8 = local_114;
                    for (iVar6 = 0xb; iVar6 != 0x0; iVar6 = iVar6 + -0x1) {
                        *puVar8 = *puVar7;
                        puVar7 = puVar7 + 0x1;
                        puVar8 = puVar8 + 0x1;
                    }
                    puVar8 = puVar7;
                    puVar7 = local_114;
                    puVar8 = &stack0xfffffe98;
                    for (iVar6 = 0xb; iVar6 != 0x0; iVar6 = iVar6 + -0x1) {
                        *puVar8 = *puVar7;
                        puVar7 = puVar7 + 0x1;
                        puVar8 = puVar8 + 0x1;
                    }
                    puVar8 = puVar7;
                    ppiVar5 = FUN_00485463(in_stack_fffffe98,in_stack_fffffe9c);
                    pbVar1 = (param_5 * 0x5e + param_1 + 0x4);
                    *pbVar1 = *pbVar1 | 0x1;
                    if (((param_3 * 0x5e + param_1 + 0x4) & 0x2) != 0x0) {
                        pbVar1 = (param_5 * 0x5e + param_1 + 0x4);
                        *pbVar1 = *pbVar1 | 0x2;
                    }
                    *(i32 ***)(param_5 * 0x5e + param_1) = ppiVar5;
                    psVar2 = (short *)(*(param_3 * 0x5e + param_1) + 0x31);
                    *psVar2 = *psVar2 - local_20;
                    if ((*(param_3 * 0x5e + param_1) + 0x31) == 0x0) {
                        if ((*(param_3 * 0x5e + param_1 + 0x4) & 0x4) != 0x0) {
                            FUN_0040a011(param_1,param_3);
                        }
                        pbVar1 = (param_3 * 0x5e + param_1 + 0x4);
                        *pbVar1 = *pbVar1 & 0xfe;
                        pbVar1 = (*(param_3 * 0x5e + param_1) + 0x3d);
                        *pbVar1 = *pbVar1 | 0x80;
                        _DAT_005b8be4 = 0x1;
                        *(param_3 * 0x5e + param_1) = 0x0;
                        local_30 = 0x1;
                    }
                    else {
                        local_30 = 0x0;
                    }
                }
            }
            else {
                if ((*(*(param_3 * 0x5e + param_1) + 0x3a) & 0x40) == 0x0) {
                    *(param_1 + 0x758) = *(param_5 * 0x5e + param_1);
                    *(param_1 + 0x75c) = *(param_5 * 0x5e + param_1 + 0x4);
                    *(param_5 * 0x5e + param_1) = *(param_3 * 0x5e + param_1);
                    *(param_5 * 0x5e + param_1 + 0x4) = *(param_3 * 0x5e + param_1 + 0x4);
                    *(param_3 * 0x5e + param_1) = *(param_1 + 0x758);
                    *(param_3 * 0x5e + param_1 + 0x4) = *(param_1 + 0x75c);
                    if (*(param_5 * 0x5e + param_1) != 0x0) {
                        pbVar1 = (*(param_5 * 0x5e + param_1) + 0x3a);
                        *pbVar1 = *pbVar1 & 0xfb;
                    }
                    local_30 = 0x0;
                }
                else {
                    FUN_0040a011(param_1,param_3);
                    local_30 = 0x1;
                }
            }
        }
        else {
            local_30 = 0x0;
        }
    }
    return local_30;
}



fn FUN_00409fc6(param_1: i32,param_2: i32) -> i32

{
let mut local_14: i32;

local_14 = param_2 + -0x1;
while( true ) {
if (local_14 < 0x0) {
return -0x1;
}
if ((*(local_14 * 0x5e + param_1 + 0x4) & 0x4) == 0x0) break;
local_14 = local_14 + -0x1;
}
return local_14;
}



fn FUN_0040a011(param_1: i32,param_2: i32)

{
    byte *pbVar1;
    let mut local_24: u32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_14: i32;

    FUN_00431d31(&local_24);
    local_1c = FUN_00409fc6(param_1,param_2);
    for (local_20 = *(local_1c * 0x5e + param_1); *(local_20 + 0xc) != 0x0;
        local_20 = *(local_20 + 0xc)) {
    }
    FUN_00431dec(&local_24,*(param_2 * 0x5e + param_1));
    local_14 = 0x0;
    while( true ) {
        if (*(*(&DAT_00582938 +
            (*(*(local_1c * 0x5e + param_1) + 0x25) >> 0x18) * 0x4 +
            (*(*(local_1c * 0x5e + param_1) + 0x24) >> 0x18) * 0x18) + 0xa5) <= local_14) {
            return;
        }
        if (*(*(local_1c * 0x5e + param_1) + local_14 * 0x4 + 0x10) == *(param_2 * 0x5e + param_1))
        break;
        local_14 = local_14 + 0x1;
    }
    *(local_14 * 0x4 + *(local_1c * 0x5e + param_1) + 0x10) = 0x0;
    pbVar1 = (*(param_2 * 0x5e + param_1) + 0x3a);
    *pbVar1 = *pbVar1 & 0xbf;
    pbVar1 = (*(param_2 * 0x5e + param_1) + 0x3a);
    *pbVar1 = *pbVar1 & 0xdf;
    return;
}



fn FUN_0040a121(param_1: i32,param_2: i32,param_3: i32)

{
    byte *pbVar1;
    ushort *puVar2;
    let puVar3: *mut u32;
    let mut local_1c: u32;
    let mut local_18: i32;
    let mut local_14: i32;

    FUN_00431d31(&local_1c);
    for (local_18 = *(param_2 * 0x5e + param_1); *(local_18 + 0xc) != 0x0;
        local_18 = *(local_18 + 0xc)) {
    }
    if (local_18 != 0x0) {
        FUN_00431efd(&local_1c,*(param_3 * 0x5e + param_1));
    }
    local_14 = 0x0;
    while ((local_14 <
        *(*(&DAT_00582938 +
            (*(*(param_2 * 0x5e + param_1) + 0x25) >> 0x18) * 0x4 +
            (*(*(param_2 * 0x5e + param_1) + 0x24) >> 0x18) * 0x18) + 0xa5) &&
        (*(*(param_2 * 0x5e + param_1) + local_14 * 0x4 + 0x10) != 0x0))) {
        local_14 = local_14 + 0x1;
    }
    *(*(param_2 * 0x5e + param_1) + local_14 * 0x4 + 0x10) =
        *(param_3 * 0x5e + param_1);
    pbVar1 = (*(param_3 * 0x5e + param_1) + 0x3a);
    *pbVar1 = *pbVar1 | 0x40;
    puVar2 = (*(param_3 * 0x5e + param_1) + 0x3c);
    *puVar2 = *puVar2 & 0x1;
    puVar3 = (*(param_3 * 0x5e + param_1) + 0x3a);
    *puVar3 = *puVar3 | *(&DAT_004be9b0 + (*(*(param_3 * 0x5e + param_1) + 0x23) >> 0x18) * 0x4);
    if (((param_2 * 0x5e + param_1 + 0x4) & 0x2) == 0x0) {
        pbVar1 = (param_3 * 0x5e + param_1 + 0x4);
        *pbVar1 = *pbVar1 & 0xfd;
    }
    else {
        pbVar1 = (param_3 * 0x5e + param_1 + 0x4);
        *pbVar1 = *pbVar1 | 0x2;
    }
    return;
}



fn FUN_0040a2cc(param_1: i32,param_2: i32) -> u32

{
    let mut local_124: *mut u32 [0x11];
    let ppuStack223: *mut *mut u8;;
    let mut local_33: String;;
    let local_2c: *mut u32;
    let mut local_28: u32;
    let mut local_24: u32;
    let local_20: *mut i32;;
    let mut local_1c: i32;
    let mut local_18: u32;

    local_20 = FUN_004990e0(local_124,0x0,s_efs_res_004c0851,s_CargoDlg_004c0848);
    DAT_004c50ac = *(*(&DAT_00595740 + param_1 * 0x5e) + 0x2f) >> 0x10;
    local_2c = &DAT_00595740;
    local_1c = param_2;
    local_18 = (&DAT_00595744 + param_2 * 0x5e) & 0x1;
    if ((((&DAT_00595744 + param_2 * 0x5e) & 0x1) != 0x0) &&
        (0x3e7 < ((*(*(&DAT_00595740 + param_2 * 0x5e) + 0x2f) >> 0x10) + DAT_004c50ac))) {
        DAT_004c50ac = 0x3e7 - (*(*(&DAT_00595740 + param_2 * 0x5e) + 0x2f) >> 0x10);
    }
    FUN_0049bf80(local_124,0xc8,0x503,DAT_004c50ac + 0x1,0x5);
    local_24 = FUN_0049bb50(local_124,FUN_0040a484);
    if (local_24 == 0x1869f) {
        local_24 = DAT_004c50ac;
    }
    local_28 = local_24;
    ppuStack223 = &PTR_FUN_004c3d34;
    if (local_33 != 0x0) {
        FUN_00499b30(local_124,local_33);
    }
    FUN_0049a1c0(local_124,0x1);
    return local_28;
}



fn FUN_0040a484(param_1: *mut *mut u32,param_2: u32,param_3: u32,param_4: i32) -> u32

{
    let mut puVar1: *mut u8;
    let local_44: u8 [0x10];
    let mut local_34: i32;
    let local_30: u8 [0x10];
    let mut local_20: i32;
    let mut local_1c: u32;
    let mut local_14: u32;

    local_14 = param_2;
    if (param_2 < 0x406) {
        if (param_2 == 0x405) {
            puVar1 = FUN_0049bf80(param_1,0xc8,0x501,0x0,0x0);
            FUN_0040a484(param_1,0x406,0xc8,puVar1);
            return 0x1;
        }
    }
    else {
        if (param_2 < 0x407) {
            if (param_3 == 0xc8) {
                FUN_0049bf80(param_1,0x7d0,0x504,param_4,0x0);
            }
        }
        else {
            if (param_2 == 0x407) {
                local_1c = param_3;
                if (param_3 < 0x65) {
                    if (param_3 == 0x64) {
                        FUN_0049bf80(param_1,0x7d0,0x501,0xf,local_44);
                        local_34 = FUN_004a1e60(local_44);
                        if (DAT_004c50ac < local_34) {
                            local_34 = DAT_004c50ac;
                        }
                        else {
                            if (local_34 < 0x0) {
                                local_34 = 0x0;
                            }
                        }
                        FUN_0049c140(param_1,local_34);
                    }
                }
                else {
                    if (param_3 < 0x66) {
                        FUN_0049c140(param_1,0x0);
                    }
                    else {
                        if (param_3 < 0x67) {
                            FUN_0049c140(param_1,0x1869f);
                        }
                        else {
                            if (param_3 == 0x7d0) {
                                FUN_0049bf80(param_1,0x7d0,0x501,0x4,local_30);
                                local_20 = FUN_004a1e60(local_30);
                                if ((local_20 <= DAT_004c50ac) && (-0x1 < local_20)) {
                                    FUN_0040a484(param_1,0x407,0x64,0x0);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return 0x0;
}



fn FUN_0040a674(param_1: i32)

{
    byte *pbVar1;
    let mut local_20: i32;

    if ((*(param_1 + 0x3a) & 0x4) != 0x0) {
        *(param_1 + 0x3a) = *(param_1 + 0x3a) & 0xfb;
        for (local_20 = 0x0;
            local_20 <
            *(*(&DAT_00582938 +
                (*(param_1 + 0x25) >> 0x18) * 0x4 + (*(param_1 + 0x24) >> 0x18) * 0x18) + 0xa5);
            local_20 = local_20 + 0x1) {
            if (*(local_20 * 0x4 + param_1 + 0x10) != 0x0) {
                pbVar1 = (*(local_20 * 0x4 + param_1 + 0x10) + 0x3a);
                *pbVar1 = *pbVar1 & 0xfb;
            }
        }
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0040a70e(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: u32) -> u32

{
    let piVar1: *mut i32;;
    let mut bVar2: bool;
    let mut bVar3: bool;
    let mut iVar4: i32;
    let piVar5: *mut i32;;
    undefined3 extraout_var;
    let mut pcVar6: String;
    let mut uVar7: u32;
    let mut uVar8: u32;
    let puVar9: *mut u32;
    let mut uVar10: u32;
    let uVar11: u16;
    let mut local_224: i32;
    let mut local_218: i32;
    u32 local_c4 [0x20];
    u32 local_44 [0x3];
    let mut local_38: u32;
    let mut local_34: i32;
    let mut local_30: i32;
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_1c: u32;
    let mut local_18: i32;
    let mut local_14: i32;

    loop {
    _DAT_004c612c = 0x0;
    DAT_004c6118 = 0xffffffff;
    local_38 = 0x0;
    local_34 = 0x0;
    local_30 = 0x0;
    DAT_004c6138 = 0x0;
    _DAT_004c6134 = 0x0;
    DAT_004c6140 = 0x0;
    _DAT_004c613c = 0x0;
    DAT_004c6148 = 0x0;
    _DAT_004c6144 = 0x0;
    _DAT_004c6128 = param_5;
    DAT_00599d3c = 0x1;
    if (DAT_005967bc == 0x0) {
        DAT_004c6118 = 0xffffffff;
        _DAT_004c612c = 0x0;
        _DAT_004c6134 = 0x0;
        DAT_004c6138 = 0x0;
        _DAT_004c613c = 0x0;
        DAT_004c6140 = 0x0;
        _DAT_004c6144 = 0x0;
        DAT_004c6148 = 0x0;
        DAT_00599d3c = 0x1;
        return 0x0;
    }
    local_2c = 0x1;
    for (local_28 = DAT_005967bc; local_28 != 0x0; local_28 = *(local_28 + 0x8)) {
        if ((*(local_28 + 0x3a) & 0x80000000) == 0x0) {
            local_2c = 0x0;
            break;
        }
    }
    if (local_2c != 0x0) {
        DAT_004c6118 = 0xffffffff;
        _DAT_004c612c = 0x0;
        _DAT_004c6134 = 0x0;
        DAT_004c6138 = 0x0;
        _DAT_004c613c = 0x0;
        DAT_004c6140 = 0x0;
        _DAT_004c6144 = 0x0;
        DAT_004c6148 = 0x0;
        DAT_00599d3c = 0x1;
        return 0x0;
    }
    DAT_004c58e0 = DAT_004c58e0 | 0x1;
    DAT_004c6110 = DAT_004c6110 | 0x1;
    FUN_00450dbf(&DAT_004c50b8,(DAT_005967bc + 0x20),(DAT_005967bc + 0x22),
                 (DAT_005967bc + 0x24),DAT_005967bc,*(DAT_005967bc + 0x3a) & 0x1,
                 *(DAT_005967bc + 0x23) >> 0x18,0x1);
    FUN_00431d31(local_44);
    switch(param_1) {
        case 0x0:
            FUN_00450dbf(&DAT_004c58e8,(DAT_005967bc + 0x20),param_2,param_3,0x0,
        *(DAT_005967bc + 0x3a) & 0x1,param_4,0x1);
        break;
        case 0x1:
            case 0x4:
            FUN_00450dbf(&DAT_004c58e8,(DAT_005967bc + 0x20),param_2,param_3,0x0,
        *(DAT_005967bc + 0x3a) & 0x1,param_4,0x1);
        DAT_004c6114 = FUN_00481784(DAT_004c60fc,param_2,param_3);
        break;
        case 0x2:
            FUN_00450dbf(&DAT_004c58e8,(DAT_005967bc + 0x20),param_2,param_3,0x0,0x1,param_4,0x1);
        DAT_004c6114 = FUN_00481784(DAT_004c60fc,param_2,param_3);
        break;
        case 0x3:
        if (DAT_005967c4 == 0x0) {
            FUN_00450dbf(&DAT_004c58e8,(DAT_005967bc + 0x20),param_2,param_3,0x0,0x0,param_4,0x1);
        }
        else {
            FUN_00450dbf(&DAT_004c58e8,(DAT_005967bc + 0x20),param_2,param_3,DAT_005967c4,0x0,param_4,
                         0x1);
        }
        break;
        default:
        return 0x0;
    }
    if ((DAT_004c6110 & 0x1) != 0x0) {
        return 0x0;
    }
    FUN_00452ac3(&DAT_004c58e8);
    local_24 = FUN_00452a43(&DAT_004c50b8);
    local_20 = FUN_00452a43(&DAT_004c58e8);
    if ((param_1 == 0x1) && (((local_24 + 0x2a) == 0x4 || ((local_20 + 0x2a) == 0x4)))) {
        param_1 = 0x4;
    }
    DAT_004c611c = param_1;
    FUN_00451d6d(&DAT_004c50b8,param_1,0x1);
    FUN_00451d6d(&DAT_004c58e8,param_1,0x0);
    local_1c = 0x1;
    if (param_1 == 0x2) {
        iVar4 = FUN_0040bb78(DAT_004c60fc,DAT_004c60fc._2_2_,DAT_004c6100,
                             DAT_004c6100._2_2_);
        if (iVar4 != 0x0) {
            if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
                FUN_00430418(0x73a1,0x7d0,0x0);
            }
            FUN_00459ca5(DAT_004c60fc,DAT_004c60fc._2_2_,DAT_004c6100,DAT_004c58d0._2_2_);
            return 0x0;
        }
        LAB_0040afca:
            FUN_0040f94b(&DAT_004c50b8,local_1c);
        FUN_0040f94b(&DAT_004c58e8,0x0);
        if (DAT_004c58d0._2_2_ < 0x5) {
            local_34 = FUN_0045172c(&DAT_004c50b8,0x2d);
        }
        if (DAT_004c6100._2_2_ < 0x5) {
            local_30 = FUN_0045172c(&DAT_004c58e8,0x2d);
        }
        _DAT_004c613c = FUN_0045172c(&DAT_004c50b8,0x31);
        DAT_004c6140 = FUN_0045172c(&DAT_004c58e8,0x31);
        iVar4 = FUN_0045172c(&DAT_004c50b8,0x58);
        if (iVar4 != 0x0) {
            _DAT_004c6144 = 0x1;
        }
        iVar4 = FUN_0045172c(&DAT_004c58e8,0x58);
        if (iVar4 != 0x0) {
            DAT_004c6148 = 0x1;
        }
        FUN_0040f7ac(&DAT_004c50b8,0x1);
        FUN_0040f7ac(&DAT_004c58e8,0x0);
        if ((DAT_004c611c == 0x2) && (iVar4 = FUN_00432bd3(&DAT_005967b8), 0x1 < iVar4)) {
            _DAT_004c6130 =
                FUN_004115e1(DAT_004c60fc,DAT_004c60fc._2_2_,DAT_004c6100,
                             DAT_004c6100._2_2_);
        }
        else {
            _DAT_004c6130 = 0x0;
        }
        local_14 = 0x0;
        loop {
            iVar4 = FUN_00410ce7();
            if ((iVar4 != 0x0) || (iVar4 = FUN_00410e4d(), iVar4 != 0x0)) break;
            if ((param_1 == 0x2) || (param_1 == 0x3)) {
                for (local_218 = 0x0; local_218 < _DAT_004bd08c; local_218 = local_218 + 0x1) {
                    iVar4 = FUN_0040cd10(0x6);
                    if (iVar4 == 0x0) {
                        if (param_1 == 0x2) {
                            FUN_0040d161();
                        }
                    }
                    else {
                        FUN_0040d38b(0x6);
                        FUN_0040d820(0x6);
                    }
                }
            }
            else {
                bVar2 = true;
                bVar3 = false;
                for (local_18 = 0x0; local_18 < 0x9; local_18 = local_18 + 0x1) {
                    for (local_224 = 0x0; local_224 < *(&DAT_004bd074 + local_18 * 0x4); local_224 = local_224 + 0x1) {
                        iVar4 = FUN_0040cd10(local_18);
                        if (iVar4 != 0x0) {
                            FUN_0040d38b(local_18);
                            FUN_0040d820(local_18);
                            bVar2 = false;
                        }
                    }
                    if (param_1 != 0x3) {
                        FUN_00452148(&DAT_004c50b8);
                    }
                    FUN_00452148(&DAT_004c58e8);
                    iVar4 = FUN_00410ce7();
                    if (iVar4 != 0x0) {
                        bVar3 = true;
                        break;
                    }
                    iVar4 = FUN_00410e4d();
                    if (iVar4 != 0x0) {
                        bVar3 = true;
                        break;
                    }
                }
                if (bVar3) break;
                if ((bVar2) && (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0)) {
                    pcVar6 = FUN_00499050(DAT_0059679c,0x73ef);
                    FUN_00430418(-0x1,0x7d0,pcVar6);
                    break;
                }
            }
            local_14 = local_14 + 0x1;
            if (local_14 == 0x3e8) {
                if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
                    FUN_00430418(-0x1,0x3e8,s_Infinite_loop_in_combat_004c0859);
                }
                break;
            }
        } while (param_1 == 0x1);
        if (((param_1 != 0x1) && (param_1 != 0x4)) || (iVar4 = FUN_004517dd(&DAT_004c58e8,param_1), iVar4 < 0x1)) {
            FUN_00411505(&DAT_004c50b8);
            FUN_00411505(&DAT_004c58e8);
            if (((DAT_004d5590 != 0x0) && (((&DAT_00569a98)[DAT_004c6100._2_2_ * 0x1e22] & 0x2) == 0x0)) && (param_1 != 0x3)
            ) {
                piVar5 = FUN_004a2831(0x1088);
                *piVar5 = param_1;
                piVar5[0x1] = DAT_004c50b8;
                (piVar5 + 0x2) = DAT_004c50bc;
                *(piVar5 + 0x82d) = DAT_004c58e8;
                (piVar5 + 0x831) = DAT_004c58ec;
                if (DAT_004c6114 == 0x0) {
                    FUN_004a0430(piVar5 + 0x1056,0xff,0x32);
                }
                else {
                    piVar1 = DAT_004c6114 + 0x3;
                    *(piVar5 + 0x1056) = DAT_004c6114[0x2];
                    (piVar5 + 0x105a) = piVar1;
                }
                FUN_0045518a(0x1 << ((byte)DAT_004c6100._2_2_ & 0x1f),0xffffffff,0x74ce,0xffffffff,piVar5,0x1088,0x0);
                FUN_0049af50(piVar5);
            }
            if ((_DAT_004c975c != 0x0) || (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0)) {
                FUN_0040ba36();
                local_38 = FUN_0040d920();
                FUN_0040babb();
            }
            FUN_00410497();
            FUN_004109dc(&DAT_004c50b8,DAT_004c6100._2_2_);
            FUN_004109dc(&DAT_004c58e8,DAT_004c58d0._2_2_);
            if ((param_1 == 0x1) || (param_1 == 0x4)) {
                FUN_00459ca5(DAT_004c60fc,DAT_004c60fc._2_2_,DAT_004c6100,DAT_004c58d0._2_2_
                );
                FUN_00459ca5(DAT_004c58ca._2_2_,_DAT_004c58ce,DAT_004c58d0,DAT_004c6100._2_2_);
            }
            else {
                if (param_1 == 0x2) {
                    FUN_00459ca5(DAT_004c60fc,DAT_004c60fc._2_2_,DAT_004c6100,
                                 DAT_004c58d0._2_2_);
                }
                else {
                    if (param_1 == 0x3) {
                        FUN_00459ca5(DAT_004c58ca._2_2_,_DAT_004c58ce,DAT_004c58d0,DAT_004c6100._2_2_);
                    }
                }
            }
            FUN_004100fe(&DAT_004c50b8,DAT_004c60fc._2_2_,DAT_004c6100,0x1,param_1);
            if (((param_1 == 0x1) || (param_1 == 0x4)) || (param_1 == 0x3)) {
                FUN_004100fe(&DAT_004c58e8,_DAT_004c58ce,DAT_004c58d0,0x0,param_1);
            }
        }
    }
    else {
        if ((param_1 != 0x1) && (param_1 != 0x4))^ // goto LAB_0040afca;
        iVar4 = FUN_004517dd(&DAT_004c58e8,param_1);
        if (iVar4 < 0x1) {
            iVar4 = FUN_00452e41(&DAT_004c58e8);
            if (iVar4 == 0x0) {
                _DAT_004c612c = 0x0;
                if ((((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) && (local_1c = FUN_0040c567(), local_1c == 0x0)) {
                    return 0xffffffff;
                }
            }
            else {
                _DAT_004c612c = 0x1;
                local_1c = 0x3;
            }^
            // goto LAB_0040afca;
        }
    }
    FUN_004112d1(DAT_004c58d0._2_2_,DAT_004c6100._2_2_);
    if (((param_1 == 0x1) && (bVar3 = FUN_00451aa0(&DAT_004c50b8,0x3), CONCAT31(extraout_var,bVar3) == 0x0)) &&
        (iVar4 = FUN_004517dd(&DAT_004c58e8,0x1), iVar4 != 0x0)) {
        if ((0x0 < iVar4) &&
            (FUN_0048616e(DAT_004c60fc,DAT_004c60fc._2_2_,DAT_004c6100,DAT_004c9754,
                          *(DAT_005967bc + 0x32) >> 0x18), ((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0)) {
            FUN_00430418(0x697a,0x5dc,0x0);
        }
        if (((DAT_004c6114 != 0x0) && (iVar4 = FUN_00481a44(DAT_004c6114), iVar4 == 0x0)) &&
            ((DAT_004c6114 + 0xe) != 0x2)) {
            FUN_00481fde(&DAT_005967b8,DAT_004c60fc._2_2_,DAT_004c6100);
        }
    }
    if (param_1 == 0x1) {
        if (_DAT_004c6144 != 0x0) {
            FUN_0045d2af(DAT_004c58ca._2_2_,_DAT_004c58ce,DAT_004c58d0);
        }
        if (DAT_004c6148 != 0x0) {
            FUN_0045d2af(DAT_004c60fc,DAT_004c60fc._2_2_,DAT_004c6100);
        }
    }
    else {
        if ((param_1 == 0x2) && (DAT_004c6114 != 0x0)) {
            if ((*(DAT_004c6114 + 0x2d) & 0x80) == 0x0) {
                (DAT_004c6114 + 0xa) = (DAT_004c6114 + 0xa) - (DAT_004c6114 + 0xe);
                *(DAT_004c6114 + 0xe) = 0x0;
            }
            else {
                FUN_004811e6(DAT_004c6114);
            }
        }
    }
    if (local_34 != 0x0) {
        if (_DAT_004c6134 != 0x0) {
            if (DAT_004c6140 == 0x0) {
                FUN_00499050(DAT_0059679c,DAT_004c58d0._2_2_ + 0x414);
                FUN_00499050(DAT_0059679c,0x713e);
                pcVar6 = FUN_00499050(DAT_0059679c,0x73ec);
                FUN_0049c2e0(local_c4,pcVar6);
            }
            else {
                FUN_00499050(DAT_0059679c,DAT_004c58d0._2_2_ + 0x414);
                FUN_00499050(DAT_0059679c,0x713e);
                pcVar6 = FUN_00499050(DAT_0059679c,0x73d5);
                FUN_0049c2e0(local_c4,pcVar6);
            }
            uVar11 = 0x1;
            uVar10 = 0xffffffff;
            puVar9 = local_c4;
            uVar8 = 0xffffffff;
            uVar7 = FUN_004a2edc();
            FUN_0045518a(0x1f,0xffffffff,uVar7 % 0x3 + 0x73ec,uVar8,puVar9,uVar10,uVar11);
        }
        if ((DAT_004c9754 < 0x5) && (iVar4 = FUN_00485fe3(DAT_004c9754), iVar4 == 0x0)) {
            return 0x29a;
        }
    }
    if ((local_30 != 0x0) && (DAT_004c6138 != 0x0)) {
        if (_DAT_004c613c == 0x0) {
            FUN_00499050(DAT_0059679c,DAT_004c6100._2_2_ + 0x414);
            FUN_00499050(DAT_0059679c,0x713e);
            pcVar6 = FUN_00499050(DAT_0059679c,0x73ec);
            FUN_0049c2e0(local_c4,pcVar6);
        }
        else {
            FUN_00499050(DAT_0059679c,DAT_004c6100._2_2_ + 0x414);
            FUN_00499050(DAT_0059679c,0x713e);
            pcVar6 = FUN_00499050(DAT_0059679c,0x73d5);
            FUN_0049c2e0(local_c4,pcVar6);
        }
        uVar11 = 0x1;
        uVar10 = 0xffffffff;
        puVar9 = local_c4;
        uVar8 = 0xffffffff;
        uVar7 = FUN_004a2edc();
        FUN_0045518a(0x1f,0xffffffff,uVar7 % 0x3 + 0x73ec,uVar8,puVar9,uVar10,uVar11);
    }
    iVar4 = FUN_00410ae6(&DAT_004c58e8);
    if (iVar4 != 0x0) {
        return 0x1;
    }
    if (local_38 == 0x0) {
        return 0x0;
    }
} while( true );
}


fn FUN_00443c4d(param_1: i32,param_2: i32,param_3: i32,param_4: i32)

{
    let mut iVar1: i32;
    let puVar2: *mut u32;
    let puVar3: *mut u32;
    let mut in_stack_fffffec8: u32;
    let mut in_stack_fffffecc: u32;
    let mut in_stack_fffffed0: u32;
    let mut in_stack_fffffed4: u32;
    let mut in_stack_fffffed8: u32;
    let mut in_stack_fffffedc: u32;
    let mut in_stack_fffffee0: u32;
    let mut in_stack_fffffee4: u32;
    let local_100: *mut i32; [0x6];
    let mut local_e8: i32;
    let mut local_e4: i32;
    let mut local_e0: i32;
    let mut local_dc: i32;
    let mut local_d8: i32;
    let mut local_d4: u32;
    let local_d0: *mut u32;
    let local_cc: *mut u32;
    let mut local_c8: u32;
    let local_c4: *mut u32;
    let local_c0: *mut u32;
    let local_bc: *mut i32; [0x6];
    let mut local_a4: i32;
    let mut local_a0: u32;
    let mut local_9c: u32;
    let mut local_98: i32;
    let local_94: *mut u32;
    let local_90: *mut i32; [0x6];
    let mut local_78: u32;
    let mut local_74: u32;
    let mut local_70: i32;
    let mut local_6c: i32;
    let mut local_68: i32;
    let mut local_64: i32;
    let mut local_60: i32;
    let mut local_5c: i32;
    let mut local_58: i32;
    let mut local_54: i32;
    let mut local_50: i32;
    let mut local_4c: i32;
    let mut local_48: i32;
    let local_44: *mut u32;
    let mut local_40: i32;
    let mut local_3c: i32;
    let local_38: *mut u32;
    let local_34: *mut u32;
    let local_30: *mut u32;
    let mut local_2c: u32;
    let local_28: *mut u32;
    let local_24: *mut u32;
    let local_20: *mut u32;
    let mut local_1c: u32;
    let mut local_18: u32;

    local_28 = DAT_005967bc + 0x8;
    local_18 = local_28 & 0xffff0000 | (DAT_005967bc + 0x22);
    local_40 = (DAT_005967bc + 0x22);
    local_44 = DAT_005967bc + 0x8;
    local_1c = local_44 & 0xffff0000 | (DAT_005967bc + 0x9);
    local_3c = (DAT_005967bc + 0x9);
    local_24 = local_28;
    local_20 = local_44;
    if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) != 0x0) {
        FUN_004245ce(DAT_005967bc);
    }
    local_2c = 0x14d0;
    if (param_3 == 0x0) {
        local_38 = 0x0;
        local_34 = 0x0;
        local_30 = 0x0;
    }
    else {
        local_38 = FUN_0049c2c9(0x14d0);
        local_34 = FUN_0049c2c9(local_2c);
        local_30 = FUN_0049c2c9(0x484);
    }
    if (((local_38 != 0x0) && (local_34 != 0x0)) && (local_30 != 0x0)) {
        FUN_00447607(param_3,local_40,local_3c);
        local_48 = param_3;
        local_4c = *(param_3 + 0x85);
        local_50 = local_4c - *(param_3 + 0x1d);
        local_54 = param_3;
        local_58 = *(param_3 + 0x89);
        local_5c = local_58 - *(param_3 + 0x21);
        local_60 = param_1;
        if ((local_40 == 0x0) && (0x16 < param_1)) {
            local_60 = param_1 + -0x2c;
        }
        if ((param_1 == 0x0) && (0x16 < local_40)) {
            local_60 = local_60 + 0x2c;
        }
        if (local_40 < local_60) {
            if (local_3c < param_2) {
                local_68 = 0x0;
                local_70 = 0x1;
            }
            else {
                local_68 = 0x14;
                local_70 = -0x1;
            }
            local_64 = 0x0;
            local_6c = 0x2;
        }
        else {
            if (local_60 < local_40) {
                if (local_3c < param_2) {
                    local_68 = 0x0;
                    local_70 = 0x1;
                }
                else {
                    local_68 = 0x14;
                    local_70 = -0x1;
                }
                local_64 = 0x26;
                local_6c = -0x2;
            }
            else {
                if (local_3c < param_2) {
                    local_64 = 0x0;
                    local_68 = 0x0;
                    local_6c = 0x0;
                    local_70 = 0x2;
                }
                else {
                    local_64 = 0x0;
                    local_68 = 0x28;
                    local_6c = 0x0;
                    local_70 = -0x2;
                }
            }
        }
        local_74 = *(param_3 + 0x1d);
        local_78 = *(param_3 + 0x21);
        FUN_004953d7();
        FUN_00435045(&DAT_005967b8);
        FUN_004a0430(local_34,0x0,0x14d0);
        local_94 = FUN_00498ba4(local_90,local_34,0x4a,0x48);
        *(param_3 + 0x1d) = local_64 - (local_50 + -0x4);
        *(param_3 + 0x21) = local_68 - (local_5c + 0x3);
        for (local_98 = 0x0; local_98 < 0x7; local_98 = local_98 + 0x1) {
            local_9c = FUN_0043a8a2(local_40 + (&DAT_004bea60)[local_98]);
            local_a0 = FUN_0043a8d5(local_9c,local_3c + (&DAT_004bea7c)[local_98]);
            in_stack_fffffee4 = 0x443f80;
            FUN_0043a32d(param_3,local_9c,local_a0,0x0,0x0);
        }
        for (local_98 = 0x0; local_98 < 0x6; local_98 = local_98 + 0x1) {
            local_9c = FUN_0043a8a2(param_1 + (&DAT_004bea60)[local_98]);
            local_a0 = FUN_0043a8d5(local_9c,param_2 + (&DAT_004bea7c)[local_98]);
            in_stack_fffffee4 = 0x444011;
            FUN_0043a32d(param_3,local_9c,local_a0,0x0,0x0);
        }
        FUN_0043a3d6(param_3,0x0);
        FUN_0043507c(&DAT_005967b8);
        FUN_00498cf4(local_90);
        local_a4 = 0x0;
        local_c0 = FUN_00498ba4(local_bc,local_30,0x22,0x22);
        *(param_3 + 0x1d) = local_50 + -0x4;
        *(param_3 + 0x1d) = -*(param_3 + 0x1d);
        *(param_3 + 0x21) = local_5c + 0x3;
        *(param_3 + 0x21) = -*(param_3 + 0x21);
        local_c4 = &DAT_005967b8;
        local_c8 = 0x1;
        DAT_005967b8 = DAT_005967b8 & 0xfffffffe;
        local_cc = FUN_004499d9(DAT_005967bc,&local_a4,-0x1,0x1);
        puVar2 = local_cc + 0x8;
        puVar3 = &stack0xfffffec8;
        for (iVar1 = 0xb; iVar1 != 0x0; iVar1 = iVar1 + -0x1) {
            *puVar3 = *puVar2;
            puVar2 = puVar2 + 0x1;
            puVar3 = puVar3 + 0x1;
        }
        puVar3 = puVar2;
        FUN_00449f24(param_3,in_stack_fffffec8,in_stack_fffffecc,in_stack_fffffed0,in_stack_fffffed4,in_stack_fffffed8,
                     in_stack_fffffedc,CONCAT44(in_stack_fffffee4,in_stack_fffffee0));
        local_d0 = &DAT_005967b8;
        local_d4 = 0x1;
        DAT_005967b8 = DAT_005967b8 | 0x1;
        FUN_00498cf4(local_bc);
        *(param_3 + 0x1d) = local_74;
        *(param_3 + 0x21) = local_78;
        local_d8 = local_64;
        local_dc = local_68;
        if (param_4 == 0x29a) {
            local_e0 = local_64;
            local_e4 = local_68;
        }
        else {
            if (param_4 == -0x1) {
                local_e0 = local_64;
                local_e4 = local_68;
                local_d8 = local_64 + local_6c * 0x9;
                local_dc = local_68 + local_70 * 0x9;
                local_6c = -local_6c;
                local_70 = -local_70;
            }
            else {
                local_e0 = local_64 + local_6c * 0x13;
                local_e4 = local_68 + local_70 * 0x14;
            }
        }
        local_64 = (*(param_3 + 0x1d) + local_50 + -0x4) - local_64;
        local_68 = (*(param_3 + 0x21) + local_5c + 0x3) - local_68;
        local_e8 = 0x0;
        FUN_00498999(*(param_3 + 0x1d),*(param_3 + 0x21),*(param_3 + 0x25),*(param_3 + 0x29));
        while( true ) {
            *local_38 = *local_34;
            (local_38 + 0x1) = (local_34 + 0x1);
            FUN_00498ba4(local_100,local_38,0x4a,0x48);
            FUN_00496ac0(local_30,local_d8,local_dc,0x22,0x22);
            FUN_00498cf4(local_100);
            FUN_00496ee6(local_38,local_64,local_68,0x48,0x4a);
            if (((local_d8 == local_e0) && (local_dc == local_e4)) && (local_e8 != 0x0)) break;
            local_e8 = local_e8 + 0x1;
            if ((local_e8 == 0x8) && (param_4 == 0x29a)) {
                local_6c = -local_6c;
                local_70 = -local_70;
                local_e8 = 0xc;
            }
            local_dc = local_dc + local_70;
            if ((local_6c != 0x0) && (local_e8 % 0xb != 0x0)) {
                local_d8 = local_d8 + local_6c;
            }
            timer_func_0049e710(0xf);
        }
        FUN_00498ae4();
        FUN_0049536f();
    }
    if (local_38 != 0x0) {
        FUN_0049af50(local_38);
    }
    if (local_34 != 0x0) {
        FUN_0049af50(local_34);
    }
    if (local_30 != 0x0) {
        FUN_0049af50(local_30);
    }
    return;
}



fn FUN_00444385(param_1: u32,param_2: u32,param_3: i32,param_4: i32) -> u32

{
    let sVar1: i16;
    let sVar2: i16;
    let sVar3: i16;
    let mut bVar4: bool;
    let mut bVar5: bool;
    undefined3 extraout_var;
    let mut pcVar6: String;
    let mut iVar7: i32;
    let mut uVar8: u32;
    let mut iVar9: i32;
    let mut local_ec: i32;
    let mut local_3c: u32;
    let mut local_2c: u32;
    let mut local_24: u32;

    sVar1 = (DAT_005967bc + 0x20);
    bVar4 = false;
    local_24 = *(DAT_005967bc + 0x3a) & *(&DAT_004be9b0 + DAT_004c9758 * 0x4);
    if ((((&DAT_00569a98)[(*(DAT_005967bc + 0x23) >> 0x18) * 0x1e22] & 0x2) != 0x0) &&
        (hide_ai_opt_004d55a4 != 0x0)) {
        local_24 = 0x0;
    }
    bVar5 = FUN_0045a451(&DAT_005967b8,param_1,param_2);
    if (CONCAT31(extraout_var,bVar5) == 0x0) {
        if ((*(*(&DAT_004d7d50 + param_1 * 0x4 + sVar1 * 0x3890) + param_2 * 0x4 + 0x4) & 0x2) == 0x0) {
            FUN_004a2d6b();
            DAT_005967b8 = DAT_005967b8 & 0xfffffffe;
            return 0xffffffff;
        }
        local_2c = 0x29a;
    }
    else {
        iVar7 = FUN_00432bd3(&DAT_005967b8);
        iVar9 = FUN_0045aa12(&DAT_005967b8,param_1,param_2,0x0);
        if (iVar7 < iVar9) {
            DAT_005967b8 = DAT_005967b8 & 0xfffffffe;
            return 0xffffffff;
        }
        iVar7 = FUN_00489539(DAT_005967bc,param_1,param_2);
        if (iVar7 == 0x0) {
            if (((&DAT_00569a98)[(*(DAT_005967bc + 0x23) >> 0x18) * 0x1e22] & 0x2) == 0x0) {
                pcVar6 = FUN_00499050(DAT_0059679c,0x715e);
                FUN_0049d2e0(0x0,0x1,pcVar6);
            }
            DAT_005967b8 = DAT_005967b8 & 0xfffffffe;
            return 0xffffffff;
        }
        local_2c = FUN_00433545(&DAT_005967b8,param_1,param_2);
        if (local_2c == 0x2) {
            if ((param_3 != 0x0) && (local_24 != 0x0)) {
                FUN_00443c4d(param_1,param_2,param_3,0x29a);
            }
            FUN_0043322a(&DAT_005967b8,param_1,param_2,0x0);
            if (DAT_005967bc != 0x0) {
                DAT_005967b8 = DAT_005967b8 & 0xfffffffe;
            }
            return 0xffffffff;
        }
        if (local_2c == 0x0) {
            DAT_005967b8 = DAT_005967b8 & 0xfffffffe;
            return 0xffffffff;
        }
        if ((local_2c == 0x1) && (local_2c = FUN_00481e14(&DAT_005967b8,param_1,param_2), local_2c == 0x6)) {
            if (((&DAT_00569a98)[(*(DAT_005967bc + 0x23) >> 0x18) * 0x1e22] & 0x2) == 0x0) {
                pcVar6 = FUN_00499050(DAT_0059679c,0x715e);
                FUN_0049d2e0(0x0,0x1,pcVar6);
            }
            DAT_005967b8 = DAT_005967b8 & 0xfffffffe;
            return 0xffffffff;
        }
    }
    sVar2 = (DAT_005967bc + 0x22);
    sVar3 = (DAT_005967bc + 0x24);
    if ((param_3 != 0x0) && (local_24 != 0x0)) {
        FUN_00445021(sVar2,sVar3,-0x1);
    }
    if (local_2c < 0x3) {
        if (local_2c != 0x0)^ // goto LAB_00444a93;^
        // goto LAB_00444885;
    }
    if (local_2c < 0x4) {
        iVar7 = FUN_00432ea6(&DAT_005967b8);
        if (((iVar7 != 0x0) || (iVar7 = FUN_00434c46(&DAT_005967b8,0x3), iVar7 != 0x0)) ||
            (iVar7 = FUN_00434c46(&DAT_005967b8,0x4), iVar7 != 0x0)) {
            DAT_005967b8 = DAT_005967b8 & 0xfffffffe;
            return 0xffffffff;
        }
        FUN_00481fde(&DAT_005967b8,param_1,param_2);
        local_2c = 0x1;^
        // goto LAB_00444a93;
    }
    if (local_2c < 0x5) {
        if ((DAT_005967bc + 0x26) == '\x05')^ // goto LAB_00444a93;
        if ((param_3 != 0x0) && (local_24 != 0x0)) {
            FUN_0043a597(param_3);
            FUN_0043a3d6(param_3,0x1);
        }
        if (((&DAT_00569a98)[(*(DAT_005967bc + 0x23) >> 0x18) * 0x1e22] & 0x2) == 0x0) {
            uVar8 = FUN_00450b06();
            if (uVar8 < 0xc8) {
                LAB_0044484c:
                    local_2c = 0x0;
            }
            else {
                if (uVar8 < 0xc9) {
                    local_2c = 0x29a;
                }
                else {
                    if (uVar8 != 0xc9)^ // goto LAB_0044484c;
                    FUN_00450dbf(&DAT_0059a1c8,(DAT_005967bc + 0x20),param_1,param_2,0x0,0x1,0x5,0x1);
                    FUN_0044f4a5();
                    local_2c = 0x0;
                }
            }
        }
        LAB_00444885:
        if (local_2c != 0x29a) {
            if (DAT_005967bc != 0x0) {
                DAT_005967b8 = DAT_005967b8 & 0xfffffffe;
            }
            return 0xffffffff;
        }
    }
    else {
        if (local_2c != 0x29a)^ // goto LAB_00444a93;
    }
    iVar7 = FUN_00432ea6(&DAT_005967b8);
    if (iVar7 != 0x0) {
        DAT_005967b8 = DAT_005967b8 & 0xfffffffe;
        return 0xffffffff;
    }
    iVar7 = FUN_00433916(&DAT_005967b8,param_1,param_2);
    if ((param_3 != 0x0) && (local_24 != 0x0)) {
        FUN_0043a3d6(param_3,0x0);
        FUN_0043a3d6(param_4,0x1);
    }
    if (iVar7 == 0x0) {
        DAT_005967b8 = DAT_005967b8 & 0xfffffffe;
        return 0xffffffff;
    }
    LAB_00444a93:
        FUN_0043402e(&DAT_005967b8,param_1,param_2);
    if ((param_3 != 0x0) && (local_24 != 0x0)) {
        FUN_00443c4d(param_1,param_2,param_3,local_2c);
        bVar4 = true;
    }
    if (local_2c == 0x29a) {
        local_ec = 0x0;
        iVar7 = FUN_00410fb3(&DAT_005967b8,0x1);
        if (iVar7 != 0x0) {
            local_ec = FUN_0040a70e(0x1,param_1,param_2,-0x1,0x1);
        }
        FUN_004864f7();
        if ((param_3 != 0x0) && (local_24 != 0x0)) {
            FUN_0043a597(param_3);
            FUN_0043a2ac(param_3,param_1,param_2);
            FUN_0043a3d6(param_3,0x0);
            FUN_0043a3d6(param_4,0x1);
        }
        if ((local_ec == 0x0) || (DAT_005967bc == 0x0)) {
            if ((param_3 != 0x0) && (local_24 != 0x0)) {
                FUN_0043a3d6(param_3,0x0);
                FUN_0043a3d6(param_4,0x1);
            }
            local_3c = 0x0;
        }
        else {
            local_3c = 0xffffffff;
        }
    }
    else {
        uVar8 = FUN_0046e816(sVar1,param_1,param_2);
        if (uVar8 == 0x0) {
            DAT_005967b8 = DAT_005967b8 & 0xfffffffe;
            if ((param_3 != 0x0) && (local_24 != 0x0)) {
                FUN_0043a3d6(param_3,0x0);
                FUN_0043a3d6(param_4,0x1);
            }
            local_3c = 0xffffffff;
        }
        else {
            iVar7 = FUN_00434129(&DAT_005967b8,param_1,param_2);
            if (((!bVar4) && ((*(&DAT_004be9b0 + DAT_004c9758 * 0x4) & *(DAT_005967bc + 0x3a)) != 0x0)) &&
                ((hide_ai_opt_004d55a4 == 0x0 ||
                    (((&DAT_00569a98)[(*(DAT_005967bc + 0x23) >> 0x18) * 0x1e22] & 0x2) == 0x0)))) {
                FUN_00443c4d(sVar2,sVar3,param_3,-0x1);
                local_24 = 0x1;
            }
            if ((*(DAT_005967bc + 0x41) == param_1) && (*(DAT_005967bc + 0x42) == param_2)) {
                FUN_00432a46(&DAT_005967b8);
                if ((param_3 != 0x0) && (local_24 != 0x0)) {
                    FUN_0043a597(param_3);
                }
                DAT_005967b8 = DAT_005967b8 & 0xfffffffe;
            }
            iVar9 = FUN_00432bd3(&DAT_005967b8);
            if (iVar9 == 0x0) {
                if ((param_3 != 0x0) && (local_24 != 0x0)) {
                    FUN_0043a3d6(param_3,0x0);
                    FUN_0043a3d6(param_4,0x1);
                }
                local_3c = 0x0;
            }
            else {
                if (iVar7 == 0x0) {
                    if (((param_3 != 0x0) && (local_24 != 0x0)) && ((DAT_005967bc + 0x41) != -0x1)) {
                        FUN_0043a597(param_3);
                        uVar8 = FUN_0045af67(&DAT_005967b8,(DAT_005967bc + 0x22),
                                             (DAT_005967bc + 0x24),*(DAT_005967bc + 0x41),
                                             *(DAT_005967bc + 0x42),0x0);
                        if (uVar8 != 0x0) {
                            FUN_00449654(param_3,uVar8,(DAT_005967bc + 0x22),(DAT_005967bc + 0x24),
                                         *(DAT_005967bc + 0x41),*(DAT_005967bc + 0x42));
                        }
                        FUN_0043a3d6(param_3,0x0);
                        FUN_0043a3d6(param_4,0x1);
                    }
                    local_3c = 0xffffffff;
                }
                else {
                    if ((param_3 != 0x0) && (local_24 != 0x0)) {
                        FUN_0043a3d6(param_3,0x0);
                        FUN_0043a3d6(param_4,0x1);
                    }
                    local_3c = 0x1;
                }
            }
        }
    }
    return local_3c;
}



fn FUN_00444faf(param_1: i32)

{
    let puVar1: *mut u32;
    let mut iVar2: i32;
    let puVar3: *mut u32;
    let mut in_stack_ffffffb4: u32;
    let mut in_stack_ffffffb8: u32;
    let mut in_stack_ffffffbc: u32;
    let mut in_stack_ffffffc0: u32;
    let mut in_stack_ffffffc4: u32;
    let mut in_stack_ffffffc8: u32;
    ulonglong in_stack_ffffffcc;
    let mut local_14: i32;

    if (param_1 != 0x0) {
        FUN_004953d7();
        local_14 = 0x0;
        if (DAT_005967bc != 0x0) {
            puVar1 = FUN_004499d9(DAT_005967bc,&local_14,-0x1,0x1);
            puVar1 = puVar1 + 0x8;
            puVar3 = &stack0xffffffb4;
            for (iVar2 = 0xb; iVar2 != 0x0; iVar2 = iVar2 + -0x1) {
                *puVar3 = *puVar1;
                puVar1 = puVar1 + 0x1;
                puVar3 = puVar3 + 0x1;
            }
            puVar3 = puVar1;
            FUN_00449f24(DAT_0059a1c0,in_stack_ffffffb4,in_stack_ffffffb8,in_stack_ffffffbc,in_stack_ffffffc0,
                         in_stack_ffffffc4,in_stack_ffffffc8,in_stack_ffffffcc);
        }
        FUN_0049536f();
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00445021(param_1: i32,param_2: i32,param_3: i32)

{
    let sVar1: i16;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let puVar4: *mut u32;
    let puVar5: *mut u32;
    let mut in_stack_fffffe58: u32;
    let mut in_stack_fffffe5c: u32;
    let mut in_stack_fffffe60: u32;
    let mut in_stack_fffffe64: u32;
    let mut in_stack_fffffe68: u32;
    let mut in_stack_fffffe6c: u32;
    ulonglong in_stack_fffffe70;
    let mut local_44: i32;
    let mut local_40: i32;
    let mut local_3c: i32;
    let mut local_38: i32;

    iVar3 = DAT_0059a1c4;
    if ((*(DAT_0059a1c0 + 0xa9) == (DAT_005967bc + 0x20)) &&
        ((*(DAT_005967bc + 0x3a) & 0x1) != 0x0)) {
        if ((*(DAT_0059a1c0 + 0x95) != -0x1) &&
            ((*(DAT_0059a1c0 + 0x95) != (DAT_005967bc + 0x22) ||
                (*(DAT_0059a1c0 + 0x99) != (DAT_005967bc + 0x24))))) {
            DAT_00599e5c = 0x0;
            if ((DAT_005967b8 & 0x1) == 0x0) {
                sVar1 = (DAT_005967bc + 0x24);
                *(DAT_0059a1c4 + 0x95) = (DAT_005967bc + 0x22);
                *(iVar3 + 0x99) = sVar1;
                puVar4 = (DAT_005967bc + 0x20);
                puVar5 = &stack0xfffffe58;
                for (iVar3 = 0xb; iVar3 != 0x0; iVar3 = iVar3 + -0x1) {
                    *puVar5 = *puVar4;
                    puVar4 = puVar4 + 0x1;
                    puVar5 = puVar5 + 0x1;
                }
                puVar5 = puVar4;
                FUN_00449f24(DAT_0059a1c4,in_stack_fffffe58,in_stack_fffffe5c,in_stack_fffffe60,in_stack_fffffe64,
                             in_stack_fffffe68,in_stack_fffffe6c,in_stack_fffffe70);
            }
            else {
                *(DAT_0059a1c4 + 0x95) = 0xffffffff;
                *(iVar3 + 0x99) = 0xffffffff;
            }
        }
        FUN_00447607(DAT_0059a1c0,param_1,param_2);
        local_44 = (*(DAT_0059a1c0 + 0x85) + -0x4) - *(DAT_0059a1c0 + 0x1d);
        local_40 = (*(DAT_0059a1c0 + 0x89) + 0x3) - *(DAT_0059a1c0 + 0x21);
        if ((((local_40 < 0x0) || (*(DAT_0059a1c0 + 0x29) <= local_40 + 0x22)) ||
            (*(DAT_0059a1c0 + 0x25) <= local_44 + 0x22)) && (param_3 != 0x0)) {
            iVar3 = FUN_0044783a(DAT_0059a1c0,param_1,param_2,0x1);
            if (iVar3 != 0x0) {
                FUN_00447997(DAT_0059a1c4);
                iVar3 = DAT_0059a1c4;
                uVar2 = *(DAT_0059a1c0 + 0x4d);
                *(DAT_0059a1c4 + 0x59) = *(DAT_0059a1c0 + 0x49);
                *(iVar3 + 0x5d) = uVar2;
                FUN_00447997(DAT_0059a1c4);
                FUN_0043a3d6(DAT_0059a1c4,0x1);
            }
            FUN_00447607(DAT_0059a1c0,param_1,param_2);
            local_44 = (*(DAT_0059a1c0 + 0x85) + -0x4) - *(DAT_0059a1c0 + 0x1d);
            local_40 = (*(DAT_0059a1c0 + 0x89) + 0x3) - *(DAT_0059a1c0 + 0x21);
        }
        else {
            if (param_3 != 0x0) {
                local_38 = 0x0;
                local_3c = 0x0;
                if (local_44 < 0x44) {
                    local_3c = (local_44 + -0x66) / 0x22;
                }
                if (*(DAT_0059a1c0 + 0x25) < local_44 + 0x66) {
                    local_3c = (local_44 - (*(DAT_0059a1c0 + 0x25) + -0x66)) / 0x22;
                }
                if (local_40 < 0x44) {
                    local_38 = (local_40 + -0x66) / 0x22;
                }
                if (*(DAT_0059a1c0 + 0x29) < local_40 + 0x69) {
                    local_38 = (local_40 - (*(DAT_0059a1c0 + 0x29) + -0x69)) / 0x22;
                }
                if ((local_3c != 0x0) || (local_38 != 0x0)) {
                    iVar3 = FUN_0044783a(DAT_0059a1c0,
                                         (*(DAT_0059a1c0 + 0x6d) >> 0x1) + *(DAT_0059a1c0 + 0x49) + local_3c,
                                         *(DAT_0059a1c0 + 0x4d) + (*(DAT_0059a1c0 + 0x69) >> 0x1) + local_38 * 0x2,
                                         0x1);
                    if (iVar3 != 0x0) {
                        FUN_00447997(DAT_0059a1c4);
                        iVar3 = DAT_0059a1c4;
                        uVar2 = *(DAT_0059a1c0 + 0x4d);
                        *(DAT_0059a1c4 + 0x59) = *(DAT_0059a1c0 + 0x49);
                        *(iVar3 + 0x5d) = uVar2;
                        FUN_00447997(DAT_0059a1c4);
                        FUN_0043a3d6(DAT_0059a1c4,0x1);
                    }
                    FUN_00447607(DAT_0059a1c0,param_1,param_2);
                    local_44 = (*(DAT_0059a1c0 + 0x85) + -0x4) - *(DAT_0059a1c0 + 0x1d);
                    local_40 = (*(DAT_0059a1c0 + 0x89) + 0x3) - *(DAT_0059a1c0 + 0x21);
                }
            }
        }
        if (((*(DAT_005967bc + 0x23) >> 0x18 == DAT_004c9754) && ((DAT_005967b8 & 0x1) == 0x0)) &&
            (iVar3 = FUN_00434cb1(&DAT_005967b8), iVar3 != 0x0)) {
            FUN_004a756b();
            FUN_00483355(0x3);
            FUN_004a75a6();
        }
        if (param_3 != -0x1) {
            if (((local_40 == -0x22 || local_40 + 0x22 < 0x0) || (*(DAT_0059a1c0 + 0x29) <= local_40)) ||
                (*(DAT_0059a1c0 + 0x25) <= local_44)) {
                if (_DAT_0059a1bc != 0x0) {
                    FUN_004a756b();
                    _DAT_0059a1bc = 0x0;
                }
            }
            else {
                FUN_004a763f(*(DAT_0059a1c0 + 0x1d) + local_44,*(DAT_0059a1c0 + 0x21) + local_40);
                if ((_DAT_0059a1bc == 0x0) && ((DAT_005967b8 & 0x1) == 0x0)) {
                    FUN_004a75a6();
                    _DAT_0059a1bc = 0x1;
                }
            }
        }
    }
    else {
        if (_DAT_0059a1bc != 0x0) {
            FUN_004a756b();
            _DAT_0059a1bc = 0x0;
        }
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0044587d(param_1: *mut *mut u32)

{
    let mut uVar1: u32;

    if (_DAT_0059a1bc != 0x0) {
        FUN_004a756b();
        _DAT_0059a1bc = 0x0;
    }
    FUN_004591f7(DAT_0059a1c0,param_1);
    if (DAT_005967bc == 0x0) {
        FUN_0049c140(param_1,0x68);
    }
    else {
        DAT_005967b8 = DAT_005967b8 & 0xfffffffe;
        if ((*(DAT_0059a1c0 + 0xa9) == (DAT_005967bc + 0x20)) &&
            ((*(DAT_005967bc + 0x3a) & 0x1) != 0x0)) {
            FUN_00445ba1(0x0);
            FUN_0043a597(DAT_0059a1c0);
            if ((((DAT_005967bc + 0x41) != -0x1) && (*(DAT_005967bc + 0x23) >> 0x18 == DAT_004c9754)) &&
                (uVar1 = FUN_0045af67(&DAT_005967b8,(DAT_005967bc + 0x22),
                                      (DAT_005967bc + 0x24),*(DAT_005967bc + 0x41),
                                      *(DAT_005967bc + 0x42),0x0), uVar1 != 0x0)) {
                FUN_00449654(DAT_0059a1c0,uVar1,(DAT_005967bc + 0x22),(DAT_005967bc + 0x24),
                             *(DAT_005967bc + 0x41),*(DAT_005967bc + 0x42));
            }
            FUN_0043a3d6(DAT_0059a1c0,0x1);
            FUN_00445021((DAT_005967bc + 0x22),(DAT_005967bc + 0x24),0x1);
            DAT_0059a9f4 = 0x0;
        }
        else {
            FUN_0049c140(param_1,0x67);
        }
    }
    return;
}



fn FUN_00445ba1(param_1: i32)

{
    let sVar1: i16;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let mut uVar4: u32;

    if ((DAT_005967bc != 0x0) && ((*(DAT_005967bc + 0x3a) & 0x1) != 0x0)) {
        sVar1 = (DAT_005967bc + 0x22);
        uVar2 = SEXT24((DAT_005967bc + 0x24));
        FUN_0043a32d(DAT_0059a1c0,sVar1,uVar2,DAT_005967bc,param_1);
        uVar3 = FUN_0043a8a2(sVar1 + -0x1);
        uVar4 = FUN_0043a8d5(uVar3,uVar2 - 0x1);
        FUN_0043a32d(DAT_0059a1c0,uVar3,uVar4,DAT_005967bc,param_1);
        uVar2 = FUN_0043a8d5(uVar3,uVar2 + 0x1);
        FUN_0043a32d(DAT_0059a1c0,uVar3,uVar2,DAT_005967bc,param_1);
    }
    return;
}



fn FUN_00445cc3(param_1: *mut u8)

{
    let mut local_20: i32;
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: u32;

    FUN_004953d7();
    FUN_004a756b();
    local_20 = 0x75;
    local_1c = 0x1b7;
    local_18 = 0x208;
    local_14 = 0x28;
    FUN_00498a5b(&local_20);
    FUN_0049a770(param_1,0x405,0x1,&local_20);
    FUN_00498ae4();
    FUN_004a75a6();
    FUN_0049536f();
    return;
}



fn FUN_00445d2f(param_1: i32)

{
    PCHAR str_param_1;
    let ppcVar1: *mut *mut char;
    let piVar2: *mut i32;;
    let mut iVar3: i32;
    let mut local_14: i32;

    str_param_1 = FUN_0049c2c9(0x104);
    FUN_0049c2e0(str_param_1,s_bin_struct_d_bin_004c19cd);
    ppcVar1 = FUN_0049c4bd(str_param_1,&DAT_004c19de);
    if (ppcVar1 == 0x0) {
        pop_err_msg_box_and_exit_004a02f5(s_InitStructSet____Can_t_find_file_004c19e1);
    }
    for (local_14 = 0x0; local_14 < 0x20; local_14 = local_14 + 0x1) {
        piVar2 = (param_1 * 0x940 + 0x565c90 + local_14 * 0x4);
        if (*piVar2 == 0x0) {
            iVar3 = FUN_0049c2c9(0x5f0);
            *piVar2 = iVar3;
        }
        FUN_004a7970((param_1 * 0x940 + 0x565c90 + local_14 * 0x4),0x5f0,0x1,ppcVar1);
    }
    FUN_0049ca40(ppcVar1);
    FUN_0049af50(str_param_1);
    return;
}



fn FUN_00445e54(param_1: i32)

{
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let mut iVar3: i32;
    let piVar4: *mut i32;;
    let mut iVar5: i32;
    i32 local_258 [0x8c];
    let local_28: *mut *mut char;
    let mut local_24: i32;
    PCHAR local_20;
    let mut local_1c: i32;
    let mut local_18: i32;

    local_18 = FUN_004a70a2(local_258,0x8c,&DAT_004c3e60);
    local_20 = FUN_0049c2c9(0x104);
    FUN_0049c2e0(local_20,s_bin_efstile_d_bin_004c1a08);
    local_28 = FUN_0049c4bd(local_20,&DAT_004c1a1a);
    if (local_28 == 0x0) {
        pop_err_msg_box_and_exit_004a02f5(s_InitTileSet____Can_t_find_file___004c1a1d);
    }
    for (local_24 = 0x0; local_24 < 0x8c; local_24 = local_24 + 0x1) {
        piVar4 = local_258 + local_24;
        if (*piVar4 == 0x0) {
            iVar3 = FUN_0049c2c9(0x5f0);
            *piVar4 = iVar3;
        }
        FUN_004a7970(local_258[local_24],0x5f0,0x1,local_28);
    }
    FUN_0049af50(local_20);
    FUN_0049ca40(local_28);
    local_1c = 0x1;
    *(&DAT_005658b0 + param_1 * 0x940) = 0x1d;
    *(&DAT_005658b8 + param_1 * 0x940) = 0x1c;
    *(&DAT_005658b4 + param_1 * 0x940) = 0x3;
    for (local_24 = 0x0; local_24 < 0x1d; local_24 = local_24 + 0x1) {
        piVar4 = (param_1 * 0x940 + 0x565830 + local_24 * 0x4);
        if (*piVar4 == 0x0) {
            iVar3 = FUN_0049c2c9(0x5f0);
            *piVar4 = iVar3;
        }
        puVar1 = local_258[local_1c];
        puVar2 = (param_1 * 0x940 + 0x565830 + local_24 * 0x4);
        *puVar2 = *puVar1;
        (puVar2 + 0x1) = (puVar1 + 0x1);
        local_1c = local_1c + 0x1;
    }
        *(&DAT_00565798 + param_1 * 0x940) = 0x3;
    *(&DAT_005657a0 + param_1 * 0x940) = 0x0;
    *(&DAT_0056579c + param_1 * 0x940) = 0x2;
    for (local_24 = 0x0; local_24 < 0x3; local_24 = local_24 + 0x1) {
        piVar4 = (param_1 * 0x940 + 0x565718 + local_24 * 0x4);
        if (*piVar4 == 0x0) {
            iVar3 = FUN_0049c2c9(0x5f0);
            *piVar4 = iVar3;
        }
        puVar1 = local_258[local_1c];
        puVar2 = (param_1 * 0x940 + 0x565718 + local_24 * 0x4);
        *puVar2 = *puVar1;
        (puVar2 + 0x1) = (puVar1 + 0x1);
        local_1c = local_1c + 0x1;
    }
        *(&DAT_00565450 + param_1 * 0x940) = 0x19;
    *(&DAT_00565458 + param_1 * 0x940) = 0x10;
    *(&DAT_00565454 + param_1 * 0x940) = 0x0;
    for (local_24 = 0x0; local_24 < 0x19; local_24 = local_24 + 0x1) {
        iVar3 = local_24 * 0x4;
        if (*(&DAT_005653d0 + iVar3 + param_1 * 0x940) == 0x0) {
            iVar5 = FUN_0049c2c9(0x5f0);
            *(&DAT_005653d0 + iVar3 + param_1 * 0x940) = iVar5;
        }
        puVar1 = local_258[local_1c];
        puVar2 = (&DAT_005653d0 + local_24 * 0x4 + param_1 * 0x940);
        *puVar2 = *puVar1;
        (puVar2 + 0x1) = (puVar1 + 0x1);
        local_1c = local_1c + 0x1;
    }
        *(&DAT_0056593c + param_1 * 0x940) = 0x8;
    *(&DAT_00565944 + param_1 * 0x940) = 0x0;
    *(&DAT_00565940 + param_1 * 0x940) = 0x4;
    for (local_24 = 0x0; local_24 < 0x8; local_24 = local_24 + 0x1) {
        piVar4 = (param_1 * 0x940 + 0x5658bc + local_24 * 0x4);
        if (*piVar4 == 0x0) {
            iVar3 = FUN_0049c2c9(0x5f0);
            *piVar4 = iVar3;
        }
        puVar1 = local_258[local_1c];
        puVar2 = (param_1 * 0x940 + 0x5658bc + local_24 * 0x4);
        *puVar2 = *puVar1;
        (puVar2 + 0x1) = (puVar1 + 0x1);
        local_1c = local_1c + 0x1;
    }
        *(&DAT_00565a54 + param_1 * 0x940) = 0x7;
    *(&DAT_00565a5c + param_1 * 0x940) = 0x0;
    *(&DAT_00565a58 + param_1 * 0x940) = 0x6;
    for (local_24 = 0x0; local_24 < 0x7; local_24 = local_24 + 0x1) {
        piVar4 = (param_1 * 0x940 + 0x5659d4 + local_24 * 0x4);
        if (*piVar4 == 0x0) {
            iVar3 = FUN_0049c2c9(0x5f0);
            *piVar4 = iVar3;
        }
        puVar1 = local_258[local_1c];
        puVar2 = (param_1 * 0x940 + 0x5659d4 + local_24 * 0x4);
        *puVar2 = *puVar1;
        (puVar2 + 0x1) = (puVar1 + 0x1);
        local_1c = local_1c + 0x1;
    }
        *(&DAT_005654dc + param_1 * 0x940) = 0x7;
    *(&DAT_005654e4 + param_1 * 0x940) = 0x0;
    *(&DAT_005654e0 + param_1 * 0x940) = 0x1;
    for (local_24 = 0x0; local_24 < 0x7; local_24 = local_24 + 0x1) {
        piVar4 = (param_1 * 0x940 + 0x56545c + local_24 * 0x4);
        if (*piVar4 == 0x0) {
            iVar3 = FUN_0049c2c9(0x5f0);
            *piVar4 = iVar3;
        }
        puVar1 = local_258[local_1c];
        puVar2 = (param_1 * 0x940 + 0x56545c + local_24 * 0x4);
        *puVar2 = *puVar1;
        (puVar2 + 0x1) = (puVar1 + 0x1);
        local_1c = local_1c + 0x1;
    }
        *(&DAT_00565568 + param_1 * 0x940) = 0x7;
    *(&DAT_00565570 + param_1 * 0x940) = 0x0;
    *(&DAT_0056556c + param_1 * 0x940) = 0x1;
    for (local_24 = 0x0; local_24 < 0x7; local_24 = local_24 + 0x1) {
        piVar4 = (param_1 * 0x940 + 0x5654e8 + local_24 * 0x4);
        if (*piVar4 == 0x0) {
            iVar3 = FUN_0049c2c9(0x5f0);
            *piVar4 = iVar3;
        }
        puVar1 = local_258[local_1c];
        puVar2 = (param_1 * 0x940 + 0x5654e8 + local_24 * 0x4);
        *puVar2 = *puVar1;
        (puVar2 + 0x1) = (puVar1 + 0x1);
        local_1c = local_1c + 0x1;
    }
        *(&DAT_005655f4 + param_1 * 0x940) = 0x7;
    *(&DAT_005655fc + param_1 * 0x940) = 0x0;
    *(&DAT_005655f8 + param_1 * 0x940) = 0x1;
    for (local_24 = 0x0; local_24 < 0x7; local_24 = local_24 + 0x1) {
        piVar4 = (param_1 * 0x940 + 0x565574 + local_24 * 0x4);
        if (*piVar4 == 0x0) {
            iVar3 = FUN_0049c2c9(0x5f0);
            *piVar4 = iVar3;
        }
        puVar1 = local_258[local_1c];
        puVar2 = (param_1 * 0x940 + 0x565574 + local_24 * 0x4);
        *puVar2 = *puVar1;
        (puVar2 + 0x1) = (puVar1 + 0x1);
        local_1c = local_1c + 0x1;
    }
        *(&DAT_005659c8 + param_1 * 0x940) = 0x17;
    *(&DAT_005659d0 + param_1 * 0x940) = 0x0;
    *(&DAT_005659cc + param_1 * 0x940) = 0x5;
    for (local_24 = 0x0; local_24 < 0x17; local_24 = local_24 + 0x1) {
        piVar4 = (param_1 * 0x940 + 0x565948 + local_24 * 0x4);
        if (*piVar4 == 0x0) {
            iVar3 = FUN_0049c2c9(0x5f0);
            *piVar4 = iVar3;
        }
        puVar1 = local_258[local_1c];
        puVar2 = (param_1 * 0x940 + 0x565948 + local_24 * 0x4);
        *puVar2 = *puVar1;
        (puVar2 + 0x1) = (puVar1 + 0x1);
        local_1c = local_1c + 0x1;
    }
        *(&DAT_0056570c + param_1 * 0x940) = 0x7;
    *(&DAT_00565714 + param_1 * 0x940) = 0x0;
    *(&DAT_00565710 + param_1 * 0x940) = 0x1;
    for (local_24 = 0x0; local_24 < 0x7; local_24 = local_24 + 0x1) {
        piVar4 = (param_1 * 0x940 + 0x56568c + local_24 * 0x4);
        if (*piVar4 == 0x0) {
            iVar3 = FUN_0049c2c9(0x5f0);
            *piVar4 = iVar3;
        }
        puVar1 = local_258[local_1c];
        puVar2 = (param_1 * 0x940 + 0x56568c + local_24 * 0x4);
        *puVar2 = *puVar1;
        (puVar2 + 0x1) = (puVar1 + 0x1);
        local_1c = local_1c + 0x1;
    }
        *(&DAT_00565680 + param_1 * 0x940) = 0x7;
    *(&DAT_00565688 + param_1 * 0x940) = 0x0;
    *(&DAT_00565684 + param_1 * 0x940) = 0x1;
    for (local_24 = 0x0; local_24 < 0x7; local_24 = local_24 + 0x1) {
        piVar4 = (param_1 * 0x940 + 0x565600 + local_24 * 0x4);
        if (*piVar4 == 0x0) {
            iVar3 = FUN_0049c2c9(0x5f0);
            *piVar4 = iVar3;
        }
        puVar1 = local_258[local_1c];
        puVar2 = (param_1 * 0x940 + 0x565600 + local_24 * 0x4);
        *puVar2 = *puVar1;
        (puVar2 + 0x1) = (puVar1 + 0x1);
        local_1c = local_1c + 0x1;
    }
        *(&DAT_00565824 + param_1 * 0x940) = 0x3;
    *(&DAT_0056582c + param_1 * 0x940) = 0x0;
    *(&DAT_00565828 + param_1 * 0x940) = 0x2;
    for (local_24 = 0x0; local_24 < 0x3; local_24 = local_24 + 0x1) {
        piVar4 = (param_1 * 0x940 + 0x5657a4 + local_24 * 0x4);
        if (*piVar4 == 0x0) {
            iVar3 = FUN_0049c2c9(0x5f0);
            *piVar4 = iVar3;
        }
        puVar1 = local_258[local_1c];
        puVar2 = (param_1 * 0x940 + 0x5657a4 + local_24 * 0x4);
        *puVar2 = *puVar1;
        (puVar2 + 0x1) = (puVar1 + 0x1);
        local_1c = local_1c + 0x1;
    }
        *(&DAT_00565ae0 + param_1 * 0x940) = 0x7;
    *(&DAT_00565ae8 + param_1 * 0x940) = 0x0;
    *(&DAT_00565ae4 + param_1 * 0x940) = 0x1;
    for (local_24 = 0x0; local_24 < 0x7; local_24 = local_24 + 0x1) {
        piVar4 = (param_1 * 0x940 + 0x565a60 + local_24 * 0x4);
        if (*piVar4 == 0x0) {
            iVar3 = FUN_0049c2c9(0x5f0);
            *piVar4 = iVar3;
        }
        puVar1 = (&DAT_004d7bf0 + local_24 * 0x4);
        puVar2 = (param_1 * 0x940 + 0x565a60 + local_24 * 0x4);
        *puVar2 = *puVar1;
        (puVar2 + 0x1) = (puVar1 + 0x1);
    }
    FUN_0044735b(local_258);
    return;
}



fn FUN_00447024()

{
    let ppcVar1: *mut *mut char;
    let mut iVar2: i32;
    let mut local_14: i32;

    ppcVar1 = FUN_0049c4bd(s_bin_explore_bin_004c1a45,&DAT_004c1a42);
    if (ppcVar1 == 0x0) {
        pop_err_msg_box_and_exit_004a02f5(s_LoadMisc____Can_t_find_file__exp_004c1a55);
    }
    for (local_14 = 0x0; local_14 < 0x7; local_14 = local_14 + 0x1) {
        if (*(&DAT_004d7bf0 + local_14 * 0x4) == 0x0) {
            iVar2 = FUN_0049c2c9(0x5f0);
            *(&DAT_004d7bf0 + local_14 * 0x4) = iVar2;
        }
        FUN_004a7970((&DAT_004d7bf0 + local_14 * 0x4),0x5f0,0x1,ppcVar1);
    }
    for (local_14 = 0x0; local_14 < 0x4; local_14 = local_14 + 0x1) {
        if ((&DAT_004d7be0)[local_14] == 0x0) {
            iVar2 = FUN_0049c2c9(0x5f0);
            (&DAT_004d7be0)[local_14] = iVar2;
        }
        FUN_004a7970((&DAT_004d7be0)[local_14],0x5f0,0x1,ppcVar1);
    }
    FUN_0049ca40(ppcVar1);
    FUN_0049c60b(s_bin_efsbut7_bin_004c1a80,&DAT_00591f30,0x271,0x0);
    FUN_0049c60b(s_bin_efsbut7_bin_004c1a90,&DAT_00591cb8,0x271,0x271);
    FUN_0049c57b(s_bin_efsbut12_bin_004c1aa0,&DAT_005921a8,0x228);
    FUN_0049c60b(s_bin_efsbut13_bin_004c1ab1,&DAT_005923d0,0xc3,0x0);
    FUN_0049c60b(s_bin_efsbut13_bin_004c1ac2,&DAT_005924d0,0xc3,0xc3);
    FUN_0049c57b(s_bin_efsbut1_bin_004c1ad3,&DAT_005925d0,0x5b2);
    FUN_0049c57b(s_bin_efsbut14_bin_004c1ae3,&DAT_00592b88,0x5b2);
    FUN_0049c57b(s_bin_efsbut15_bin_004c1af4,&DAT_00593140,0x3cf);
    FUN_0049c57b(s_bin_couple_bin_004c1b05,&DAT_005b8ef0,0x26c);
    FUN_0049c57b(s_bin_shield_bin_004c1b14,&DAT_00593510,0x1e);
    FUN_0049c57b(s_bin_glimmer_bin_004c1b23,&DAT_00593530,0x5f0);
    FUN_0049c57b(s_bin_border_bin_004c1b33,&DAT_00593b20,0x285);
    FUN_0049c57b(s_bin_skull_bin_004c1b42,&DAT_00593da8,0x640);
    FUN_0049c57b(s_bin_flag_bin_004c1b50,&DAT_005943e8,0x640);
    FUN_0049c57b(s_mouse_msk_004c1b5d,&DAT_00594b00,0x400);
    FUN_0049c57b(s_mouse1_msk_004c1b67,&DAT_00594f00,0x400);
    FUN_0049c57b(s_mouse2_msk_004c1b72,&DAT_00595300,0x400);
    FUN_0049c57b(s_bin_cargo_bin_004c1b7d,&DAT_00596a58,0x3212);
    FUN_0049c57b(s_bin_pod_bin_004c1b8b,&DAT_00594a28,0xd8);
    return;
}



fn FUN_0044735b(param_1: u32) -> u32

{
    let mut uVar1: u32;

    uVar1 = FUN_004a7132(param_1,0x8c,&DAT_004c3e60);
    return uVar1;
}



fn FUN_0044738c(param_1: i32,param_2: i32,param_3: i32)

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut uVar5: u32;
    let mut iVar6: i32;

    if (*(param_1 + 0x7d) < 0x7) {
        *(param_1 + 0x8d) = (param_2 - *(param_1 + 0x1d)) / *(param_1 + 0x7d);
        *(param_1 + 0x91) = (param_3 - *(param_1 + 0x21)) / (*(param_1 + 0x7d) >> 0x1);
        if ((*(param_1 + 0x8d) & 0x1) == 0x0) {
            *(param_1 + 0x91) = *(param_1 + 0x91) | 0x1;
        }
        else {
            *(param_1 + 0x91) = *(param_1 + 0x91) + 0x1;
            *(param_1 + 0x91) = *(param_1 + 0x91) & 0xfe;
        }
        *(param_1 + 0x91) = *(param_1 + 0x91) & 0xfe;
    }
    else {
        iVar1 = param_2 - *(param_1 + 0x1d);
        iVar2 = param_3 - *(param_1 + 0x21);
        iVar3 = *(param_1 + 0x79) - *(param_1 + 0x75) >> 0x1;
        iVar4 = *(param_1 + 0x79) - iVar3;
        *(param_1 + 0x8d) = iVar1 / iVar4;
        iVar1 = iVar1 - *(param_1 + 0x8d) * iVar4;
        *(param_1 + 0x8d) = *(param_1 + 0x8d) + 0x1;
        *(param_1 + 0x91) = iVar2 / *(param_1 + 0x7d);
        iVar6 = iVar2 - *(param_1 + 0x91) * *(param_1 + 0x7d);
        *(param_1 + 0x91) = iVar2 / *(param_1 + 0x81);
        if (iVar1 <= iVar3) {
            if ((*(param_1 + 0x8d) + *(param_1 + 0x49) & 0x1U) == 0x0) {
                if (iVar1 < *(&DAT_004bf4a0 + iVar6 * 0x4)) {
                    *(param_1 + 0x8d) = *(param_1 + 0x8d) + -0x1;
                }
            }
            else {
                if (iVar1 < *(&DAT_004bf400 + iVar6 * 0x4)) {
                    *(param_1 + 0x8d) = *(param_1 + 0x8d) + -0x1;
                }
            }
        }
        if ((*(param_1 + 0x8d) + *(param_1 + 0x49) & 0x1U) == 0x0) {
            *(param_1 + 0x91) = *(param_1 + 0x91) | 0x1;
        }
        else {
            *(param_1 + 0x91) = *(param_1 + 0x91) + 0x1;
            *(param_1 + 0x91) = *(param_1 + 0x91) & 0xfe;
        }
        *(param_1 + 0x89) = *(param_1 + 0x81) * (*(param_1 + 0x91) + -0x1) + *(param_1 + 0x21);
        *(param_1 + 0x85) = ((*(param_1 + 0x1d) + iVar3) - iVar4) + *(param_1 + 0x8d) * iVar4;
        iVar1 = FUN_0043a8a2(*(param_1 + 0x8d) + *(param_1 + 0x49));
        *(param_1 + 0x8d) = iVar1;
        uVar5 = FUN_0043a8d5(*(param_1 + 0x8d),*(param_1 + 0x91) + *(param_1 + 0x4d));
        *(param_1 + 0x91) = uVar5;
    }
    return;
}



fn FUN_00447607(param_1: i32,param_2: u32,param_3: u32)

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut iVar3: i32;

    *(param_1 + 0x8d) = param_2;
    *(param_1 + 0x91) = param_3;
    if (*(param_1 + 0x7d) < 0x7) {
        *(param_1 + 0x85) = *(param_1 + 0x8d) * *(param_1 + 0x7d) + *(param_1 + 0x1d);
        *(param_1 + 0x89) =
            (*(param_1 + 0x91) + -0x1) * (*(param_1 + 0x7d) >> 0x1) + *(param_1 + 0x21);
    }
    else {
        iVar1 = *(param_1 + 0x79) - *(param_1 + 0x75) >> 0x1;
        iVar2 = *(param_1 + 0x79) - iVar1;
        iVar3 = FUN_0043a8a2(*(param_1 + 0x8d) - *(param_1 + 0x49));
        *(param_1 + 0x85) = (iVar2 * iVar3 + *(param_1 + 0x1d) + iVar1) - iVar2;
        *(param_1 + 0x89) =
            *(param_1 + 0x81) * ((*(param_1 + 0x91) - *(param_1 + 0x4d)) + -0x1) +
                *(param_1 + 0x21);
        if ((*(param_1 + 0x2d) & 0x4) != 0x0) {
            *(param_1 + 0x85) = *(param_1 + 0x85) + 0x1c;
        }
    }
    return;
}



fn FUN_0044771e(param_1: i32,param_2: *mut u32,param_3: u32)

{
    switch(param_3) {
    case 0x0:
        FUN_004013f4(param_1,param_2);
    break;
    case 0x1:
        FUN_0040144c(param_1,param_2);
    break;
    case 0x2:
        FUN_004014b8(param_1,param_2);
    break;
    case 0x3:
        FUN_00401528(param_1,param_2);
    break;
    case 0x4:
        FUN_00401584(param_1,param_2);
    break;
    case 0x5:
        FUN_004015f4(param_1,param_2);
}
    return;
}



fn FUN_004477de(param_1: i32,param_2: i32,param_3: i32)

{
    let mut iVar1: i32;

    iVar1 = FUN_0043a8a2(param_2);
    *(param_1 + 0x49) = iVar1;
    if (param_3 < 0x0) {
        *(param_1 + 0x4d) = 0x0;
    }
    else {
        if (param_3 < 0x2c) {
            *(param_1 + 0x4d) = param_3;
        }
        else {
            *(param_1 + 0x4d) = 0x2b;
        }
    }
    *(param_1 + 0x4d) = *(param_1 + 0x4d) & 0xfe;
    return;
}



fn FUN_0044783a(param_1: i32,param_2: i32,param_3: i32,param_4: i32) -> u32

{
    let mut iVar1: i32;
    let mut iVar2: i32;

    if ((*(param_1 + 0x7d) == 0x2) && ((*(param_1 + 0x2d) & 0x2) != 0x0)) {
        if (param_4 != 0x0) {
            FUN_00447997(param_1);
        }
        iVar1 = *(param_1 + 0x59);
        iVar2 = *(param_1 + 0x5d);
        FUN_00447b38(param_1,param_2,param_3);
        if ((iVar1 == *(param_1 + 0x59)) && (iVar2 == *(param_1 + 0x5d))) {
            return 0x0;
        }
        if (param_4 == 0x0) {
            return 0x0;
        }
        FUN_00447997(param_1);
        FUN_0043a3d6(param_1,0x1);
    }
    else {
        iVar1 = *(param_1 + 0x49);
        iVar2 = *(param_1 + 0x4d);
        FUN_004477de(param_1,param_2 - (*(param_1 + 0x6d) >> 0x1),param_3 - (*(param_1 + 0x69) >> 0x1));
        if ((iVar1 == *(param_1 + 0x49)) && (iVar2 == *(param_1 + 0x4d))) {
            return 0x0;
        }
        if (param_4 == 0x0) {
            return 0x0;
        }
        FUN_00439d27(param_1,0x0);
    }
    return 0x1;
}



fn FUN_00447997(param_1: i32)

{
    byte *pbVar1;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let mut uVar6: u32;
    let mut local_34: u32;
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_14: u32;

    iVar2 = *(param_1 + 0x59) + *(param_1 + 0x51);
    iVar3 = FUN_0043a8a2(iVar2);
    iVar4 = *(param_1 + 0x5d) + *(param_1 + 0x55);
    local_14 = ((*(param_1 + 0x59) & 0x1) == 0x0);
    iVar5 = iVar4 + local_14;
    for (local_20 = *(param_1 + 0x5d) + local_14; local_20 < iVar5; local_20 = local_20 + 0x2) {
        pbVar1 = (*(*(param_1 + 0xa1) + *(param_1 + 0x59) * 0x4) + local_20 * 0x4 + 0x4);
        *pbVar1 = *pbVar1 | 0x10;
    }
    for (local_20 = *(param_1 + 0x5d) + local_14; local_20 < iVar5; local_20 = local_20 + 0x2) {
        pbVar1 = (*(*(param_1 + 0xa1) + iVar3 * 0x4) + local_20 * 0x4 + 0x4);
        *pbVar1 = *pbVar1 | 0x10;
    }
    local_24 = *(param_1 + 0x59);
    while (local_24 = local_24 + 0x1, local_24 < iVar2) {
        uVar6 = FUN_0043a8a2(local_24);
        local_34 = ((uVar6 & 0x1) == 0x0);
        pbVar1 = (*(*(param_1 + 0xa1) + uVar6 * 0x4) + (*(param_1 + 0x5d) + local_34) * 0x4 +
            0x4);
        *pbVar1 = *pbVar1 | 0x10;
    }
    local_24 = *(param_1 + 0x59);
    while (local_24 = local_24 + 0x1, local_24 < iVar2) {
        uVar6 = FUN_0043a8a2(local_24);
        pbVar1 = (*(*(param_1 + 0xa1) + uVar6 * 0x4) + (iVar4 - (uVar6 & 0x1)) * 0x4 + 0x4);
        *pbVar1 = *pbVar1 | 0x10;
    }
    return;
}



fn FUN_00447b38(param_1: i32,param_2: i32,param_3: i32)

{
    let mut iVar1: i32;

    iVar1 = FUN_0043a8a2(param_2 - (*(param_1 + 0x6d) >> 0x1));
    *(param_1 + 0x59) = iVar1;
    *(param_1 + 0x5d) = param_3 - (*(param_1 + 0x69) >> 0x1);
    if (*(param_1 + 0x5d) < 0x0) {
        *(param_1 + 0x5d) = 0x0;
    }
    else {
        if (0x2b < *(param_1 + 0x5d)) {
            *(param_1 + 0x5d) = 0x2b;
        }
    }
    *(param_1 + 0x5d) = *(param_1 + 0x5d) & 0xfe;
    return;
}



fn FUN_00447bc6(param_1: i32,param_2: i32,param_3: i32)

{
    FUN_004953d7();
    FUN_00498999(*(param_1 + 0x1d),*(param_1 + 0x21),*(param_1 + 0x25),*(param_1 + 0x29));
    FUN_00447c1d(param_1,param_2,param_3);
    FUN_0049536f();
    FUN_00498ae4();
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00447c1d(param_1: i32,param_2: i32,param_3: i32)

{
    let cVar1: u8;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let puVar4: *mut u32;
    let mut iVar5: i32;
    let mut pcVar6: String;
    let mut uVar7: u32;
    let mut pcVar8: String;
    let mut local_820: i32;
    let mut local_810: u32;
    let local_804: *mut i32; [0x6];
    let mut local_7ec: u32;
    let mut local_7e8: i32;
    let mut local_7e4: *mut u8;
    u32 local_7e0 [0x2];
    let abStack2006: u8 [0x26];
    let abStack1968: u8 [0x750];
    let mut local_60: i32;
    let mut local_5c: i32;
    let mut local_58: i32;
    let mut local_54: u32;
    let mut local_50: i32;
    let mut local_4c: u32;
    let mut local_48: u32;
    let mut local_44: u32;
    let mut local_40: i32;
    let mut local_3c: i32;
    let mut local_38: i32;
    let mut local_34: i32;
    let mut local_30: u32;
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: u32;
    let mut local_1c: i32;
    let mut local_18: i32;

    FUN_00447607(param_1,param_2,param_3);
    local_3c = param_1;
    local_38 = param_2;
    local_34 = param_3;
    local_4c = *(*(*(param_1 + 0xa1) + param_2 * 0x4) + param_3 * 0x4);
    local_50 = param_3 + 0x1;
    local_2c = param_1;
    local_28 = param_2;
    local_48 = *(*(*(param_1 + 0xa1) + param_2 * 0x4) + local_50 * 0x4);
    local_44 = 0x272727;
    local_1c = param_1;
    local_40 = *(param_1 + 0xa9);
    local_30 = local_4c;
    local_24 = local_50;
    local_20 = local_48;
    local_18 = local_40;
    if (*(param_1 + 0x7d) == 0x2) {
        local_54 = *(&DAT_004d7c0c + *(&DAT_004daab0 + local_40 * 0x3890) * 0x40 + (local_4c & 0xf) * 0x4);
        if (((*(&DAT_004be9b0 + DAT_004c9758 * 0x4) & local_48) == 0x0) && (_DAT_004c975c == 0x0)) {
            local_54 = FUN_00499f60(local_54);
        }
        if ((*(param_1 + 0x2d) & 0x2) == 0x0) {
            FUN_004968e7(*(param_1 + 0x85),*(param_1 + 0x89),*(param_1 + 0x75),*(param_1 + 0x7d),
                         (char)local_54);
        }
        else {
            local_58 = FUN_0043a8a2(*(param_1 + 0x59) + *(param_1 + 0x51));
            local_5c = *(param_1 + 0x5d) + *(param_1 + 0x55);
            FUN_004968e7(*(param_1 + 0x85),*(param_1 + 0x89),*(param_1 + 0x75),*(param_1 + 0x7d),
                         (char)local_54);
            if ((*(param_1 + 0x5d) < param_3) && (param_3 < local_5c)) {
                if (param_2 == *(param_1 + 0x59)) {
                    FUN_004968e7(*(param_1 + 0x85),*(param_1 + 0x89),0x1,*(param_1 + 0x7d),(char)local_44);
                }
                else {
                    if (param_2 == local_58) {
                        FUN_004968e7(*(param_1 + 0x85) + 0x1,*(param_1 + 0x89),0x1,*(param_1 + 0x7d),
                                     (char)local_44);
                    }
                }
            }
            if ((((*(param_1 + 0x59) < local_58) && (*(param_1 + 0x59) <= param_2)) && (param_2 <= local_58)) ||
                ((local_58 < *(param_1 + 0x59) && ((*(param_1 + 0x59) <= param_2 || (param_2 <= local_58)))))) {
                if ((*(param_1 + 0x5d) + 0x1 == param_3) || (param_3 == local_5c)) {
                    FUN_004968e7(*(param_1 + 0x85),*(param_1 + 0x89),*(param_1 + 0x75),0x1,(char)local_44);
                }
                else {
                    if ((param_3 == *(param_1 + 0x5d)) || (local_5c + -0x1 == param_3)) {
                        FUN_004968e7(*(param_1 + 0x85),*(param_1 + 0x89) + 0x1,*(param_1 + 0x75),0x1,
                                     (char)local_44);
                    }
                }
            }
        }
    }
    else {
        local_60 = *(param_1 + 0x79) - *(param_1 + 0x75) >> 0x1;
        FUN_00448824(param_1,param_2,param_3,*(param_1 + 0x85),*(param_1 + 0x89),local_4c,
                     local_7e0);
        if (((*(&DAT_004be9b0 + DAT_004c9758 * 0x4) & local_48) == 0x0) && (_DAT_004c975c == 0x0)) {
            local_7e4 = &DAT_005b96d0;
            FUN_00498799(local_7e0,local_7e0,0x780,&DAT_005b96d0);
        }
        if ((*(param_1 + 0x2d) & 0x80) != 0x0) {
            local_7e8 = 0xa;
            for (local_7ec = 0x0; local_7ec < 0x1c; local_7ec = local_7ec + 0x1) {
                (local_7e0 + local_7e8) = (&DAT_00599e60)[*(local_7e0 + local_7e8)];
                local_7e8 = local_7e8 + 0x1;
            }
            local_7e8 = local_7e8 + -0x1;
            for (local_7ec = 0x1; local_7ec < 0x14; local_7ec = local_7ec + 0x1) {
                local_7e8 = local_7e8 + (local_7ec & 0x1) + 0x30;
                abStack1968[local_7e8 + -0x30] = (&DAT_00599e60)[abStack1968[local_7e8 + -0x30]];
            }
            for (local_7ec = 0x0; local_7ec < 0x14; local_7ec = local_7ec + 0x1) {
                local_7e8 = local_7e8 + (0x30 - (local_7ec & 0x1));
                abStack1968[local_7e8 + -0x30] = (&DAT_00599e60)[abStack1968[local_7e8 + -0x30]];
            }
        }
        if (((local_48 & 0x40) != 0x0) &&
            (((*(&DAT_004be9b0 + DAT_004c9758 * 0x4) & local_48) != 0x0 || (_DAT_004c975c != 0x0)))) {
            FUN_004010c4(local_7e0,&DAT_00593530,0x30,0x28,0x1c);
        }
        if ((*(param_1 + 0x2d) & 0x10) == 0x0) {
            FUN_00498ba4(local_804,local_7e0,0x28,0x30);
            if ((*(*(*(param_1 + 0xa1) + param_2 * 0x4) + param_3 * 0x4 + 0x4) & 0x80) != 0x0) {
                iVar5 = param_3;
                iVar3 = FUN_0043a8a2(param_2);
                puVar4 = FUN_00481784(local_40,iVar3,iVar5);
                if ((puVar4 != 0x0) &&
                    (((*(&DAT_004be9b0 + DAT_004c9758 * 0x4) & puVar4[0xb]) != 0x0 || (_DAT_004c975c != 0x0)))) {
                    local_810 = 0xe0e0e;
                    uVar2 = *(&DAT_004be9e8 + (*(puVar4 + 0xe) >> 0x10) * 0x4);
                    pcVar6 = &DAT_005831d8 + (puVar4[0x3] >> 0x10) * 0x50;
                    uVar7 = 0xffffffff;
                    pcVar8 = pcVar6;
                    loop {
                        if (uVar7 == 0x0) break;
                        uVar7 = uVar7 - 0x1;
                        cVar1 = *pcVar8;
                        pcVar8 = pcVar8 + 0x1;
                    } while (cVar1 != '\0');
                    if ((((puVar4 + 0x4) != (puVar4 + 0x12)) && (0x9 < puVar4[0x4] >> 0x10)) &&
                        (puVar4[0x4] >> 0x10 < 0xd)) {
                        local_810 = 0x272727;
                    }
                    for (local_820 = 0x0; local_820 < 0x4; local_820 = local_820 + 0x1) {
                        FUN_00497567(*(&DAT_004bf3c0 + local_820 * 0x4) + 0x18,
                                     *(&DAT_004bf3e0 + local_820 * 0x4) + 0x27,pcVar6,~uVar7 - 0x1,local_810,-0x1,0x0,
                                     DAT_004d6a6c,0x9);
                    }
                    FUN_00497567(0x18,0x27,pcVar6,~uVar7 - 0x1,uVar2,-0x1,uVar2,DAT_004d6a6c,0x9);
                }
            }
            iVar5 = FUN_0043a8a2(param_2 + 0x1);
            if ((*(*(*(param_1 + 0xa1) + iVar5 * 0x4) + param_3 * 0x4) & 0x80) != 0x0) {
                iVar5 = param_3 + -0x1;
                iVar3 = FUN_0043a8a2(param_2 + 0x1);
                puVar4 = FUN_00481784(local_40,iVar3,iVar5);
                if ((puVar4 != 0x0) &&
                    (((*(&DAT_004be9b0 + DAT_004c9758 * 0x4) & puVar4[0xb]) != 0x0 || (_DAT_004c975c != 0x0)))) {
                    local_810 = 0xe0e0e;
                    uVar2 = *(&DAT_004be9e8 + (*(puVar4 + 0xe) >> 0x10) * 0x4);
                    pcVar6 = &DAT_005831d8 + (puVar4[0x3] >> 0x10) * 0x50;
                    uVar7 = 0xffffffff;
                    pcVar8 = pcVar6;
                    loop {
                        if (uVar7 == 0x0) break;
                        uVar7 = uVar7 - 0x1;
                        cVar1 = *pcVar8;
                        pcVar8 = pcVar8 + 0x1;
                    } while (cVar1 != '\0');
                    if ((((puVar4 + 0x4) != (puVar4 + 0x12)) && (0x9 < puVar4[0x4] >> 0x10)) &&
                        (puVar4[0x4] >> 0x10 < 0xd)) {
                        local_810 = 0x272727;
                    }
                    for (local_820 = 0x0; local_820 < 0x4; local_820 = local_820 + 0x1) {
                        FUN_00497567(*(&DAT_004bf3c0 + local_820 * 0x4) + 0x3e,
                                     *(&DAT_004bf3e0 + local_820 * 0x4) + 0x13,pcVar6,~uVar7 - 0x1,local_810,-0x1,0x0,
                                     DAT_004d6a6c,0x9);
                    }
                    FUN_00497567(0x3e,0x13,pcVar6,~uVar7 - 0x1,uVar2,-0x1,uVar2,DAT_004d6a6c,0x9);
                }
            }
            iVar5 = FUN_0043a8a2(param_2 + -0x1);
            if ((*(*(*(param_1 + 0xa1) + iVar5 * 0x4) + param_3 * 0x4) & 0x80) != 0x0) {
                iVar5 = param_3 + -0x1;
                iVar3 = FUN_0043a8a2(param_2 + -0x1);
                puVar4 = FUN_00481784(local_40,iVar3,iVar5);
                if ((puVar4 != 0x0) &&
                    (((*(&DAT_004be9b0 + DAT_004c9758 * 0x4) & puVar4[0xb]) != 0x0 || (_DAT_004c975c != 0x0)))) {
                    local_810 = 0xe0e0e;
                    uVar2 = *(&DAT_004be9e8 + (*(puVar4 + 0xe) >> 0x10) * 0x4);
                    pcVar6 = &DAT_005831d8 + (puVar4[0x3] >> 0x10) * 0x50;
                    uVar7 = 0xffffffff;
                    pcVar8 = pcVar6;
                    loop {
                        if (uVar7 == 0x0) break;
                        uVar7 = uVar7 - 0x1;
                        cVar1 = *pcVar8;
                        pcVar8 = pcVar8 + 0x1;
                    } while (cVar1 != '\0');
                    if ((((puVar4 + 0x4) != (puVar4 + 0x12)) && (0x9 < puVar4[0x4] >> 0x10)) &&
                        (puVar4[0x4] >> 0x10 < 0xd)) {
                        local_810 = 0x272727;
                    }
                    for (local_820 = 0x0; local_820 < 0x4; local_820 = local_820 + 0x1) {
                        FUN_00497567(*(&DAT_004bf3c0 + local_820 * 0x4) + -0xe,
                                     *(&DAT_004bf3e0 + local_820 * 0x4) + 0x13,pcVar6,~uVar7 - 0x1,local_810,-0x1,0x0,
                                     DAT_004d6a6c,0x9);
                    }
                    FUN_00497567(-0xe,0x13,pcVar6,~uVar7 - 0x1,uVar2,-0x1,uVar2,DAT_004d6a6c,0x9);
                }
            }
            FUN_00498cf4(local_804);
        }
        uVar7 = local_48 & 0x700;
        if (uVar7 < 0x200) {
            if (uVar7 == 0x100) {
                FUN_004010c4(local_7e0,DAT_004d7be0,0x30,0x28,0x1c);
            }
        }
        else {
            if (uVar7 < 0x201) {
                FUN_004010c4(local_7e0,DAT_004d7be4,0x30,0x28,0x1c);
            }
            else {
                if (0x2ff < uVar7) {
                    if (uVar7 < 0x301) {
                        FUN_004010c4(local_7e0,DAT_004d7be8,0x30,0x28,0x1c);
                    }
                    else {
                        if (uVar7 == 0x400) {
                            FUN_004010c4(local_7e0,DAT_004d7bec,0x30,0x28,0x1c);
                        }
                    }
                }
            }
        }
        FUN_00497045(*(param_1 + 0x85) - local_60,*(param_1 + 0x89),*(param_1 + 0x61),
                     *(param_1 + 0x65),local_7e0,0x30,0x28);
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void
FUN_00448824(param_1: i32,param_2: i32,param_3: i32,param_4: u32,param_5: u32,param_6: u32,param_7: *mut u32)

{
let mut uVar1: u32;
let mut bVar2: bool;
let mut uVar3: u32;
let mut uVar4: u32;
let mut uVar5: u32;
let mut uVar6: u32;
let mut uVar7: u32;
let puVar8: *mut u32;
let mut iVar9: i32;
let mut local_74: i32;
let mut local_50: i32;

uVar1 = *(*(*(param_1 + 0xa1) + param_2 * 0x4) + (param_3 + 0x1) * 0x4);
iVar9 = *(param_1 + 0xa9);
uVar3 = param_6 & 0xf;
uVar4 = param_6 >> 0x4;
uVar5 = uVar4 & 0x1f;
uVar6 = *(*(param_1 + 0xb1) + uVar3 * 0x8c + 0x84);
if (uVar6 < 0x7) {
uVar7 = param_6 >> 0x9;
switch(uVar6) {
case 0x0:
for (local_50 = 0x0; local_50 < 0x6; local_50 = local_50 + 0x1) {
puVar8 =
FUN_0043920b(param_1,param_2 + (&DAT_004bea60)[local_50],param_3 + (&DAT_004bea7c)[local_50]);
if (puVar8 != 0x0) {
FUN_0044771e(param_7,puVar8,local_50);
}
}
FUN_004494c2(param_7,*(uVar3 * 0x8c + *(param_1 + 0xb1) + uVar5 * 0x4),uVar4 & 0x60);
break;
case 0x1:
FUN_004010c4(param_7,*(*(param_1 + 0xb1) + uVar3 * 0x8c),0x30,0x28,0x1c);
for (local_74 = 0x0; local_74 < 0x6; local_74 = local_74 + 0x1) {
uVar6 = FUN_00439c67(*(param_1 + 0xa5),*(param_1 + 0x8d),*(param_1 + 0x91),local_74);
FUN_0043a8a2(*(param_1 + 0x8d) + (&DAT_004bea60)[local_74]);
if (((uVar6 != 0xffffffff) && (uVar6 = FUN_0043915e(*(param_1 + 0xa5),uVar6,0x1), uVar6 != 0xffffffff))
&& (uVar3 != uVar6)) {
FUN_004010c4(param_7,*(uVar6 * 0x8c + *(param_1 + 0xb1) + local_74 * 0x4 + 0x4),0x30,0x28,0x1c
);
}
}
break;
default:
FUN_00448824(param_1,param_2,param_3,param_4,param_5,uVar7,param_7);
FUN_004010c4(param_7,*(uVar3 * 0x8c + *(param_1 + 0xb1) + uVar5 * 0x4),0x30,0x28,0x1c);
break;
case 0x4:
FUN_00448824(param_1,param_2,param_3,param_4,param_5,uVar7,param_7);
FUN_004493f9(param_7,*(uVar3 * 0x8c + *(param_1 + 0xb1) + (uVar5 - (uVar4 & 0x18)) * 0x4),
uVar4 & 0x18);
break;
case 0x5:
FUN_00448824(param_1,param_2,param_3,param_4,param_5,param_6 >> 0xb,param_7);
FUN_0044958b(param_7,*(uVar3 * 0x8c + *(param_1 + 0xb1) + uVar5 * 0x4),uVar4 & 0x60);
break;
case 0x6:
FUN_00448824(param_1,param_2,param_3,param_4,param_5,uVar7,param_7);
if ((*(param_1 + 0x2d) & 0x20) == 0x0) {
bVar2 = true;
uVar6 = FUN_00439c67(*(param_1 + 0xa5),*(param_1 + 0x8d),*(param_1 + 0x91),0x5);
if ((uVar6 != 0xffffffff) && (uVar6 = FUN_0043915e(*(param_1 + 0xa5),uVar6,0x6), uVar6 != 0xffffffff)) {
FUN_004010c4(param_7,*(*(param_1 + 0xb1) + uVar3 * 0x8c + 0x4),0x30,0x28,0x1c);
bVar2 = false;
}
uVar6 = FUN_00439c67(*(param_1 + 0xa5),*(param_1 + 0x8d),*(param_1 + 0x91),0x3);
if ((uVar6 != 0xffffffff) && (uVar6 = FUN_0043915e(*(param_1 + 0xa5),uVar6,0x6), uVar6 != 0xffffffff)) {
FUN_004010c4(param_7,*(*(param_1 + 0xb1) + uVar3 * 0x8c + 0x8),0x30,0x28,0x1c);
bVar2 = false;
}
uVar6 = FUN_00439c67(*(param_1 + 0xa5),*(param_1 + 0x8d),*(param_1 + 0x91),0x2);
if ((uVar6 != 0xffffffff) && (uVar6 = FUN_0043915e(*(param_1 + 0xa5),uVar6,0x6), uVar6 != 0xffffffff)) {
FUN_004010c4(param_7,*(*(param_1 + 0xb1) + uVar3 * 0x8c + 0xc),0x30,0x28,0x1c);
bVar2 = false;
}
uVar6 = FUN_00439c67(*(param_1 + 0xa5),*(param_1 + 0x8d),*(param_1 + 0x91),0x0);
if ((uVar6 != 0xffffffff) && (uVar6 = FUN_0043915e(*(param_1 + 0xa5),uVar6,0x6), uVar6 != 0xffffffff)) {
FUN_004010c4(param_7,*(*(param_1 + 0xb1) + uVar3 * 0x8c + 0x10),0x30,0x28,0x1c);
bVar2 = false;
}
uVar6 = FUN_00439c67(*(param_1 + 0xa5),*(param_1 + 0x8d),*(param_1 + 0x91),0x4);
if ((uVar6 != 0xffffffff) && (uVar6 = FUN_0043915e(*(param_1 + 0xa5),uVar6,0x6), uVar6 != 0xffffffff)) {
FUN_004010c4(param_7,*(*(param_1 + 0xb1) + uVar3 * 0x8c + 0x14),0x30,0x28,0x1c);
bVar2 = false;
}
uVar6 = FUN_00439c67(*(param_1 + 0xa5),*(param_1 + 0x8d),*(param_1 + 0x91),0x1);
if ((uVar6 != 0xffffffff) && (uVar6 = FUN_0043915e(*(param_1 + 0xa5),uVar6,0x6), uVar6 != 0xffffffff)) {
FUN_004010c4(param_7,*(*(param_1 + 0xb1) + uVar3 * 0x8c + 0x18),0x30,0x28,0x1c);
bVar2 = false;
}
if (bVar2) {
FUN_004010c4(param_7,*(*(param_1 + 0xb1) + uVar3 * 0x8c),0x30,0x28,0x1c);
}
}
}
}
if ((*(*(*(param_1 + 0xa1) + param_2 * 0x4) + param_3 * 0x4 + 0x4) & 0x80) != 0x0) {
puVar8 = FUN_00481784(iVar9,param_2,param_3);
if (((*(&DAT_004be9b0 + DAT_004c9758 * 0x4) & uVar1) == 0x0) && (_DAT_004c975c == 0x0)) {
iVar9 = FUN_00481a44(puVar8);
if ((iVar9 == 0x0) && ((puVar8 + 0xe) != 0x2)) {
FUN_004010c4(param_7,*(*(param_1 + 0xb5) + 0x54),0x30,0x28,0x1c);
}
}
else {
FUN_004010c4(param_7,*(*(param_1 + 0xb5) + (puVar8[0x3] >> 0x10) * 0x4),0x30,0x28,0x1c);
}
}
if ((*(*(*(param_1 + 0xa1) + param_2 * 0x4) + param_3 * 0x4 + 0x5) & 0x10) != 0x0) {
FUN_00498799(param_7,param_7,0x780,&DAT_005b96d0);
}
return;
}



fn FUN_004493f9(param_1: *mut u32,param_2: &mut String,param_3: u32)

{
    if (param_3 < 0x8) {
        if (param_3 == 0x0) {
            FUN_004010c4(param_1,param_2,0x30,0x28,0x1c);
        }
    }
    else {
        if (param_3 < 0x9) {
            FUN_00401250(param_1,param_2,0x30,0x28,0x1c);
        }
        else {
            if (0xf < param_3) {
                if (param_3 < 0x11) {
                    FUN_00401188(param_1,param_2,0x30,0x28,0x1c,0x5f0);
                }
                else {
                    if (param_3 == 0x18) {
                        FUN_00401320(param_1,param_2,0x30,0x28,0x1c,0x5f0);
                    }
                }
            }
        }
    }
    return;
}



fn FUN_004494c2(param_1: *mut u32,param_2: &mut String,param_3: u32)

{
    if (param_3 < 0x20) {
        if (param_3 == 0x0) {
            FUN_004010c4(param_1,param_2,0x30,0x28,0x1c);
        }
    }
    else {
        if (param_3 < 0x21) {
            FUN_00401250(param_1,param_2,0x30,0x28,0x1c);
        }
        else {
            if (0x3f < param_3) {
                if (param_3 < 0x41) {
                    FUN_00401188(param_1,param_2,0x30,0x28,0x1c,0x5f0);
                }
                else {
                    if (param_3 == 0x60) {
                        FUN_00401320(param_1,param_2,0x30,0x28,0x1c,0x5f0);
                    }
                }
            }
        }
    }
    return;
}



fn FUN_0044958b(param_1: *mut u32,param_2: &mut String,param_3: u32)

{
    if (param_3 < 0x20) {
        if (param_3 == 0x0) {
            FUN_004010c4(param_1,param_2,0x30,0x28,0x1c);
        }
    }
    else {
        if (param_3 < 0x21) {
            FUN_00401250(param_1,param_2,0x30,0x28,0x1c);
        }
        else {
            if (0x3f < param_3) {
                if (param_3 < 0x41) {
                    FUN_00401188(param_1,param_2,0x30,0x28,0x1c,0x5f0);
                }
                else {
                    if (param_3 == 0x60) {
                        FUN_00401320(param_1,param_2,0x30,0x28,0x1c,0x5f0);
                    }
                }
            }
        }
    }
    return;
}



fn FUN_00449654(param_1: i32,param_2: u32,param_3: u32,param_4: u32,param_5: u32,param_6: u32) -> u32

{
    let mut uVar1: u32;
    let mut iVar2: i32;
    let mut local_80: u32;
    let mut local_7c: u32;
    let mut local_5c: u32;
    let mut local_58: u32;
    let mut local_30: u32;
    let mut local_2c: u32;
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: i32;
    let local_1c: *mut u32;
    let mut local_18: u32;
    let mut local_14: u32;

    local_1c = &DAT_005967b8;
    local_14 = (DAT_005967bc == 0x0);
    local_18 = local_14;
    if ((local_14 == 0x0) && ((DAT_005967bc + 0x41) != -0x1)) {
        local_28 = *(param_1 + 0xa9);
        local_20 = 0x1;
        local_24 = FUN_00432bd3(&DAT_005967b8);
        if (local_24 == 0x0) {
            local_24 = FUN_00432b1a(&DAT_005967b8);
            local_20 = 0x0;
        }
        param_2 = FUN_0045af67(&DAT_005967b8,param_3,param_4,*(DAT_005967bc + 0x41),
                               *(DAT_005967bc + 0x42),0x0);
        while (((param_2 = FUN_0045b45b(&DAT_005967b8,param_2,param_3,param_4,&local_30,&local_2c),
                 param_2 != 0xffffffff && (local_30 != 0xffffffff)) && (local_2c != 0xffffffff))) {
            iVar2 = FUN_0045aa12(&DAT_005967b8,local_30,local_2c,0x0);
            local_24 = local_24 - iVar2;
            if (local_24 < 0x0) {
                uVar1 = *(*(&DAT_004d7d50 + param_3 * 0x4 + local_28 * 0x3890) + param_4 * 0x4 + 0x4);
                if ((((uVar1 & 0x300) == 0x0) && ((uVar1 & 0x400) == 0x0)) &&
                    (((DAT_005967bc + 0x22) != param_3 || ((DAT_005967bc + 0x24) != param_4)))) {
                    if (local_20 == 0x0) {
                        local_7c = 0x400;
                    }
                    else {
                        local_7c = 0x300;
                    }
                    FUN_0043a17b(param_1,param_3,param_4,local_7c);
                }
                local_20 = 0x0;
                local_24 = FUN_00432b1a(&DAT_005967b8);
            }
            else {
                if (((*(*(&DAT_004d7d50 + param_3 * 0x4 + local_28 * 0x3890) + param_4 * 0x4 + 0x5) & 0x7) == 0x0
                ) && (((DAT_005967bc + 0x22) != param_3 || ((DAT_005967bc + 0x24) != param_4))
                )) {
                    if (local_20 == 0x0) {
                        local_58 = 0x200;
                    }
                    else {
                        local_58 = 0x100;
                    }
                    FUN_0043a17b(param_1,param_3,param_4,local_58);
                }
                if (local_24 == 0x0) {
                    if (local_20 == 0x0) {
                        local_5c = 0x400;
                    }
                    else {
                        local_5c = 0x300;
                    }
                    FUN_0043a17b(param_1,local_30,local_2c,local_5c);
                    local_20 = 0x0;
                    local_24 = FUN_00432b1a(&DAT_005967b8);
                }
            }
            if ((param_2 == 0x0) || ((local_30 == param_5 && (local_2c == param_6)))) {
                if ((*(*(&DAT_004d7d50 + local_30 * 0x4 + local_28 * 0x3890) + local_2c * 0x4 + 0x5) & 0x7) ==
                    0x0) {
                    if (local_20 == 0x0) {
                        local_80 = 0x400;
                    }
                    else {
                        local_80 = 0x300;
                    }
                    FUN_0043a17b(param_1,local_30,local_2c,local_80);
                }
                return 0x1;
            }
            param_3 = local_30;
            param_4 = local_2c;
        }
    }
    return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_004499d9(param_1: *mut u32,param_2: *mut i32,param_3: i32,param_4: i32) -> *mut u32

{
    let mut uVar1: u32;
    let local_28: *mut u32;
    let local_24: *mut u32;

    local_24 = param_1;
    *param_2 = 0x0;
    uVar1 = *(param_1 + 0x3a) & 0x1;
    local_28 = param_1;
    while ((((local_28 != 0x0 && ((param_1 + 0x8) == (local_28 + 0x8))) &&
        ((param_1 + 0x22) == (local_28 + 0x22))) &&
        ((param_1 + 0x9) == (local_28 + 0x9)))) {
        if ((((((*(&DAT_004be9b0 + DAT_004c9758 * 0x4) & *(local_28 + 0x3a)) != 0x0) ||
            (_DAT_004c975c != 0x0)) &&
            (((*(local_28 + 0x3a) & 0x8) == 0x0 && ((*(local_28 + 0x3a) & 0x1) == uVar1)))) &&
            ((uVar1 != 0x0 || (*(local_28 + 0x23) >> 0x18 == param_3)))) &&
            ((*param_2 = *param_2 + 0x1, (*(local_28 + 0x3a) & 0x40) == 0x0 &&
                ((local_28 + 0x27) <= (local_24 + 0x27))))) {
            local_24 = local_28;
        }
        if (param_4 == 0x0) {
            local_28 = *local_28;
        }
        else {
            local_28 = local_28[0x2];
        }
    }
    return local_24;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00449bde(param_1: i32)

{
    byte *pbVar1;
    let puVar2: *mut u32;
    let mut iVar3: i32;
    let puVar4: *mut u32;
    let mut in_stack_ffffff04: u32;
    let mut in_stack_ffffff08: u32;
    let mut in_stack_ffffff0c: u32;
    let mut in_stack_ffffff10: u32;
    let mut in_stack_ffffff14: u32;
    let mut in_stack_ffffff18: u32;
    ulonglong in_stack_ffffff1c;
    let local_70: *mut u32;
    let mut local_68: i32;
    let mut local_5c: i32;
    let mut local_58: i32;
    let mut local_54: i32;
    let local_50: *mut u32;
    let mut local_4c: u32;
    let mut local_48: u32;
    let local_44: *mut u32;
    let mut local_40: u32;
    let mut local_3c: i32;
    let mut local_38: i32;
    let mut local_34: u32;
    ushort *local_30;
    ushort *local_2c;
    let local_28: *mut u32;
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: u32;
    let mut local_14: u32;

    local_58 = param_1;
    local_68 = *(param_1 + 0xa9);
    local_5c = 0x0;
    local_54 = local_68;
    FUN_0044b0ba(param_1);
    if ((*(param_1 + 0x2d) & 0x4) == 0x0) {
        local_50 = &DAT_005967b8;
        local_48 = (DAT_005967bc == 0x0);
        local_4c = local_48;
        if (local_48 == 0x0) {
            local_44 = &DAT_005967b8;
            local_3c = DAT_005967bc + 0x20;
            local_40 = *(DAT_005967bc + 0x3a) & 0x1;
            local_38 = local_3c;
            local_34 = local_40;
            if (local_40 != 0x0) {
                local_30 = (DAT_005967bc + 0x20);
                local_14 = local_30 & 0xffff0000 | *local_30;
                local_24 = *local_30;
                local_20 = param_1;
                local_1c = *(param_1 + 0xa9);
                local_2c = local_30;
                if (local_1c == local_24) {
                    local_28 = &DAT_005967b8;
                    local_18 = DAT_005967b8 & 0x1;
                    if (local_18 == 0x0) {
                        pbVar1 = (*(*(param_1 + 0xa1) + (DAT_005967bc + 0x22) * 0x4) +
                            (DAT_005967bc + 0x24) * 0x4 + 0x4);
                        *pbVar1 = *pbVar1 | 0x20;
                    }
                }
            }
        }
    }
    FUN_004953d7();
    for (local_70 = (&DAT_005b8b44 + local_68 * 0x4);
        (local_70 != 0x0 && ((local_70 + 0x8) == local_68)); local_70 = *local_70) {
        if ((((*(local_70 + 0x3a) & 0x80000000) == 0x0) && ((*(local_70 + 0x3a) & 0x1) != 0x0)) &&
            (((((*(&DAT_004be9b0 + DAT_004c9758 * 0x4) & *(local_70 + 0x3a)) != 0x0 ||
                (_DAT_004c975c != 0x0)) && ((*(local_70 + 0x3a) & 0x8) == 0x0)) &&
                ((*(*(*(param_1 + 0xa1) + (local_70 + 0x22) * 0x4) +
                    (local_70 + 0x9) * 0x4 + 0x4) & 0x20) == 0x0)))) {
            puVar2 = FUN_004499d9(local_70,&local_5c,-0x1,0x0);
            puVar2 = puVar2 + 0x8;
            puVar4 = &stack0xffffff04;
            for (iVar3 = 0xb; iVar3 != 0x0; iVar3 = iVar3 + -0x1) {
                *puVar4 = *puVar2;
                puVar2 = puVar2 + 0x1;
                puVar4 = puVar4 + 0x1;
            }
            puVar4 = puVar2;
            FUN_00449f24(param_1,in_stack_ffffff04,in_stack_ffffff08,in_stack_ffffff0c,in_stack_ffffff10,in_stack_ffffff14,
                         in_stack_ffffff18,in_stack_ffffff1c);
        }
    }
    FUN_0049536f();
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
// WARNING: Restarted to delay deadcode elimination for space: stack

void
FUN_00449f24(param_1: i32,param_2: u32,param_3: u32,param_4: u32,param_5: u32,param_6: u32,
param_7: u32,ulonglong param_8)

{
byte *pbVar1;
let mut iVar2: i32;
let mut iVar3: i32;
let mut iVar4: i32;
let mut iVar5: i32;
let mut pcVar6: String;
let mut uVar7: u32;
let mut in_stack_00000038: i32;
let mut local_10c: i32;
let mut local_108: i32;
let mut local_fc: i32;
let local_a0: u8;
let local_1c: *mut u32;

if (((*(&DAT_004be9b0 + DAT_004c9758 * 0x4) & param_8._2_4_) == 0x0) && (_DAT_004c975c == 0x0)) {
return;
}
if ((param_8 & 0x400000) != 0x0) {
return;
}
iVar2 = param_2._2_2_;
iVar3 = param_3;
local_1c = 0x0;
FUN_00447607(param_1,iVar2,iVar3);
FUN_00498999(*(param_1 + 0x1d),*(param_1 + 0x21),*(param_1 + 0x25),*(param_1 + 0x29));
if (*(param_1 + 0x7d) == 0x2) {
if ((*(param_1 + 0x2d) & 0x2) != 0x0) {
iVar4 = FUN_0043a8a2(*(param_1 + 0x59) + *(param_1 + 0x51));
iVar5 = *(param_1 + 0x5d) + *(param_1 + 0x55);
if ((((*(param_1 + 0x5d) < iVar3) && (iVar3 < iVar5)) &&
((iVar2 == *(param_1 + 0x59) || (iVar2 == iVar4)))) ||
(((((*(param_1 + 0x59) < iVar4 && (*(param_1 + 0x59) <= iVar2)) && (iVar2 <= iVar4)) ||
((iVar4 < *(param_1 + 0x59) && ((*(param_1 + 0x59) <= iVar2 || (iVar2 <= iVar4)))))) &&
(((*(param_1 + 0x5d) + 0x1 == iVar3 || ((iVar3 == iVar5 || (iVar3 == *(param_1 + 0x5d))))) ||
(iVar5 + -0x1 == iVar3))))))^ // goto LAB_0044a7b1;
}
FUN_00495520(*(param_1 + 0x85),*(param_1 + 0x89),*(param_1 + 0x75),
*(param_1 + 0x7d),0x0);
if ((((DAT_005967bc == 0x0) || ((*(DAT_005967bc + 0x3a) & 0x1) == 0x0)) ||
((DAT_005967bc + 0x22) != iVar2)) || ((DAT_005967bc + 0x24) != iVar3)) {
FUN_004968e7(*(param_1 + 0x85),*(param_1 + 0x89),*(param_1 + 0x75),*(param_1 + 0x7d),
(char)*(&DAT_004be9e8 + param_3._2_1_ * 0x4));
}
else {
if (DAT_00599e5c == 0x0) {
local_a0 = *(&DAT_004be9e8 + param_3._2_1_ * 0x4);
}
else {
local_a0 = 0x27;
}
FUN_004968e7(*(param_1 + 0x85),*(param_1 + 0x89),*(param_1 + 0x75),*(param_1 + 0x7d),
local_a0);
}
FUN_00495607(0x0);
}
else {
if (((*(param_1 + 0x2d) & 0x4) != 0x0) ||
((((*(param_1 + 0x1d) <= *(param_1 + 0x85) + 0x22 &&
(*(param_1 + 0x85) <= *(param_1 + 0x1d) + *(param_1 + 0x25))) &&
(*(param_1 + 0x21) <= *(param_1 + 0x89) + 0x22)) &&
(*(param_1 + 0x89) <= *(param_1 + 0x21) + *(param_1 + 0x29))))) {
if ((((DAT_005967bc == 0x0) || ((DAT_005967bc + 0x22) != iVar2)) ||
(((DAT_005967bc + 0x24) != iVar3 ||
(((*(DAT_005967bc + 0x3a) & 0x1) == 0x0 ||
(*(param_1 + 0xa9) != (DAT_005967bc + 0x20))))))) || ((DAT_005967b8 & 0x1) != 0x0)) {
local_1c = FUN_00481784(param_2,iVar2,iVar3);
}
if ((((local_1c == 0x0) || (*(&DAT_00583208 + (local_1c[0x3] >> 0x10) * 0x50) != 0x0))
|| (*(param_1 + 0x7d) < 0x14)) ||
(((CONCAT44(*(param_1 + 0x7d) >> 0x1f,*(param_1 + 0x7d)) % 0x5) != 0x0 ||
(((*(&DAT_004be9b0 + DAT_004c9758 * 0x4) & local_1c[0xb]) == 0x0 && (_DAT_004c975c == 0x0)))))) {
pcVar6 = FUN_0049c2c9(0x484);
uVar7 = (*(param_1 + 0x7d) << 0x3) / 0xa + 0x2;
FUN_004906c1(pcVar6,*(&DAT_004d6058 + param_3._3_1_ * 0x1c),
(char)*(&DAT_004be9e8 + param_3._2_1_ * 0x4),param_6._3_1_,0x22,-0x1);
if (*(param_1 + 0x7d) == 0x28) {
FUN_00490a17(pcVar6,pcVar6,in_stack_00000038);
}
FUN_004953d7();
FUN_00497045(*(param_1 + 0x85) + -0x4,*(param_1 + 0x89) + 0x3,uVar7,uVar7,pcVar6,0x22,0x22);
FUN_0049af50(pcVar6);
FUN_0049536f();
}
else {
local_fc = 0x0;
uVar7 = (CONCAT44(*(param_1 + 0x7d) >> 0x1f,*(param_1 + 0x7d)) / 0x5);
pcVar6 = FUN_0049c2c9(uVar7 * uVar7);
for (local_108 = 0x0; local_108 < uVar7; local_108 = local_108 + 0x1) {
pcVar6[local_fc] = '\x1f';
local_fc = local_fc + 0x1;
}
for (local_108 = 0x0; local_108 < (uVar7 - 0x2); local_108 = local_108 + 0x1) {
pcVar6[local_fc] = '\x1f';
iVar4 = local_fc;
for (local_10c = 0x0; local_fc = iVar4 + 0x1, local_10c < 0x6; local_10c = local_10c + 0x1) {
pcVar6[local_fc] = (&DAT_004be9e8)[param_3._2_1_ * 0x4];
iVar4 = local_fc;
}
pcVar6[local_fc] = '\x13';
local_fc = iVar4 + 0x2;
}
for (local_108 = 0x0; local_108 < uVar7; local_108 = local_108 + 0x1) {
pcVar6[local_fc] = '\x13';
local_fc = local_fc + 0x1;
}
FUN_004953d7();
FUN_00496ee6(pcVar6,*(param_1 + 0x85),*(param_1 + 0x89) + 0x5,uVar7,uVar7);
FUN_0049536f();
FUN_0049af50(pcVar6);
}
}
}
// LAB_0044a7b1:
pbVar1 = (*(*(param_1 + 0xa1) + iVar2 * 0x4) + iVar3 * 0x4 + 0x4);
*pbVar1 = *pbVar1 | 0x20;
FUN_00498ae4();
return;
}



fn FUN_0044a7dd(param_1: i32,undefined2 param_2,undefined2 param_3)

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    undefined2 *puVar3;
    let puVar4: *mut u32;
    let puVar5: *mut u32;
    let mut bVar6: bool;
    let mut bVar7: bool;
    let mut in_stack_ffffffa8: u32;
    let mut in_stack_ffffffac: u32;
    let mut in_stack_ffffffb0: u32;
    let mut in_stack_ffffffb4: u32;
    let mut in_stack_ffffffb8: u32;
    let mut in_stack_ffffffbc: u32;
    ulonglong in_stack_ffffffc0;
    let local_24: u16;
    let local_22: u16;
    let local_20: u16;
    let local_1c: *mut u32;
    let mut local_18: i32;
    let mut local_14: u32;

    local_18 = param_1;
    local_14 = *(param_1 + 0xa9);
    local_24 = *(param_1 + 0xa9);
    local_22 = param_2;
    local_20 = param_3;
    local_1c = *DAT_005967b0;
    loop {
    if (local_1c == 0x0) {
        return;
    }
    iVar2 = 0x6;
    bVar6 = false;
    iVar1 = 0x0;
    bVar7 = true;
    puVar3 = &local_24;
    puVar4 = local_1c + 0x8;
    loop {
        if (iVar2 == 0x0) break;
        iVar2 = iVar2 + -0x1;
        bVar6 = *puVar3 < *puVar4;
        bVar7 = *puVar3 == *puVar4;
        puVar3 = (puVar3 + 0x1);
        puVar4 = (puVar4 + 0x1);
    } while (bVar7);
    if (!bVar7) {
        iVar1 = (0x1 - bVar6) - (bVar6 != 0x0);
    }
    if ((iVar1 == 0x0) && ((*(local_1c + 0x3a) & 0x1) != 0x0)) {
        puVar4 = local_1c + 0x8;
        puVar5 = &stack0xffffffa8;
        for (iVar1 = 0xb; iVar1 != 0x0; iVar1 = iVar1 + -0x1) {
            *puVar5 = *puVar4;
            puVar4 = puVar4 + 0x1;
            puVar5 = puVar5 + 0x1;
        }
        puVar5 = puVar4;
        FUN_00449f24(param_1,in_stack_ffffffa8,in_stack_ffffffac,in_stack_ffffffb0,in_stack_ffffffb4,in_stack_ffffffb8,
                     in_stack_ffffffbc,in_stack_ffffffc0);
        return;
    }
    local_1c = *local_1c;
} while( true );
}



fn FUN_0044a87f(param_1: i32,param_2: i32,param_3: i32,param_4: i32) -> i32

{
let mut iVar1: i32;
let mut uVar2: u32;
let mut local_1c: i32;
let mut local_18: i32;

uVar2 = param_1 - param_3 >> 0x1f;
local_18 = (param_1 - param_3 ^ uVar2) - uVar2;
uVar2 = param_2 - param_4 >> 0x1f;
iVar1 = (param_2 - param_4 ^ uVar2) - uVar2;
if (0x16 < local_18) {
local_18 = 0x2c - local_18;
}
if (iVar1 < local_18) {
local_1c = local_18;
}
else {
local_1c = local_18 + (iVar1 - local_18 >> 0x1);
}
return local_1c;
}



fn FUN_0044a8e3(param_1: i32,param_2: i32,param_3: i32,param_4: i32) -> u32

{
    let local_14: *mut u32;

    local_14 = (&DAT_005b8b44 + param_1 * 0x4);
    loop {
    if ((local_14 == 0x0) || ((local_14 + 0x8) != param_1)) {
        return 0x0;
    }
    if (param_4 == 0x0) {
        if ((*(local_14 + 0x3a) & 0x1) == 0x0)^ // goto LAB_0044a987;
    }
    else {
        if ((*(local_14 + 0x3a) & 0x1) != 0x0) {
            LAB_0044a987:
            if ((((local_14 + 0x22) == param_2) && ((local_14 + 0x9) == param_3)) &&
                ((*(&DAT_004be9b0 + DAT_004c9758 * 0x4) & *(local_14 + 0x3a)) != 0x0)) {
                return 0x1;
            }
        }
    }
    local_14 = *local_14;
} while( true );
}



fn FUN_0044aa04(param_1: i32,param_2: i32,param_3: i32) -> i32

{
let mut bVar1: bool;
let mut local_48: i32;
let mut local_20: i32;
let local_1c: *mut u32;
let mut local_14: i32;

bVar1 = false;
local_14 = 0x0;
for (local_1c = (&DAT_005b8b44 + param_1 * 0x4);
(local_1c != 0x0 && ((local_1c + 0x8) == param_1)); local_1c = *local_1c) {
if ((((local_1c + 0x22) == param_2) &&
((((local_1c + 0x9) == param_3 && (*(local_1c + 0x23) >> 0x18 == DAT_004c9754)) &&
(*(*(&DAT_00582938 +
(*(local_1c + 0x25) >> 0x18) * 0x4 + (local_1c[0x9] >> 0x18) * 0x18) + 0xa5)
!= 0x0)))) &&
((*(*(&DAT_00582938 +
(*(local_1c + 0x25) >> 0x18) * 0x4 + (local_1c[0x9] >> 0x18) * 0x18) + 0x41)
== 0x4 && (*(local_1c + 0x23) >> 0x18 == DAT_004c9754)))) {
bVar1 = true;
for (local_48 = 0x0; local_48 < 0x4; local_48 = local_48 + 0x1) {
if (local_1c[local_48 + 0x4] == 0x0) {
local_14 = local_14 + 0x1;
}
}
}
}
if (bVar1) {
local_20 = local_14;
}
else {
local_20 = -0x1;
}
return local_20;
}



fn FUN_0044ab7b(param_1: i32,param_2: i32,param_3: i32) -> u32

{
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut local_14: i32;

    local_14 = 0x0;
    while( true ) {
        if (0x5 < local_14) {
            return 0x0;
        }
        uVar1 = FUN_0043a8a2(param_2 + (&DAT_004bea60)[local_14]);
        uVar2 = FUN_0043a8d5(uVar1,param_3 + (&DAT_004bea7c)[local_14]);
        if ((*(*(&DAT_004d7d50 + uVar1 * 0x4 + param_1 * 0x3890) + uVar2 * 0x4) & 0xf) == 0x0) break;
        local_14 = local_14 + 0x1;
    }
    return 0x1;
}



fn FUN_0044ac19(param_1: i32,param_2: *mut u32,param_3: *mut u32) -> u32

{
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut local_14: i32;

    local_14 = 0x0;
    while( true ) {
        if (0x5 < local_14) {
            return 0x0;
        }
        uVar1 = FUN_0043a8a2(*param_2 + (&DAT_004bea60)[local_14]);
        uVar2 = FUN_0043a8d5(uVar1,*param_3 + (&DAT_004bea7c)[local_14]);
        if ((*(uVar2 * 0x4 + *(&DAT_004d7d50 + uVar1 * 0x4 + param_1 * 0x3890)) & 0xf) == 0x0) break;
        local_14 = local_14 + 0x1;
    }
    *param_2 = uVar1;
    uVar1 = FUN_0043a8d5(uVar1,*param_3 + (&DAT_004bea7c)[local_14]);
    *param_3 = uVar1;
    return 0x1;
}



fn FUN_0044ace5(param_1: i32,param_2: i32,param_3: i32,param_4: i32) -> i32

{
let local_14: *mut u32;

local_14 = (&DAT_005b8b44 + param_1 * 0x4);
while( true ) {
if ((local_14 == 0x0) || ((local_14 + 0x8) != param_1)) {
return -0x1;
}
if ((((local_14 + 0x22) == param_2) && ((local_14 + 0x9) == param_3)) &&
((*(local_14 + 0x3a) & 0x1) != 0x0)) break;
local_14 = *local_14;
}
if ((param_4 != 0x0) && ((*(&DAT_004be9b0 + DAT_004c9754 * 0x4) & *(local_14 + 0x3a)) != 0x0)) {
return *(local_14 + 0x23) >> 0x18;
}
return *(local_14 + 0x23) >> 0x18;
}



fn FUN_0044add9(param_1: i32)

{
    let puVar1: *mut u32;
    byte *pbVar2;
    let sVar3: i16;
    let sVar4: i16;
    let sVar5: i16;
    let sVar6: i16;
    let mut uVar7: u32;
    let mut uVar8: u32;
    let mut iVar9: i32;
    let mut local_34: i32;
    let mut local_20: i32;
    let mut local_1c: u32;

    if ((*(param_1 + 0x23) >> 0x18 < 0x5) && ((*(param_1 + 0x3a) & 0x1) != 0x0)) {
        sVar3 = (param_1 + 0x20);
        iVar9 = *(*(&DAT_00582938 +
            (*(param_1 + 0x25) >> 0x18) * 0x4 + (*(param_1 + 0x24) >> 0x18) * 0x18) +
            0x49);
        if (iVar9 != 0x0) {
            if (iVar9 < 0x4) {
                local_34 = 0x1;
            }
            else {
                local_34 = (iVar9 >> 0x1) + -0x1;
            }
            sVar4 = (param_1 + 0x22);
            sVar5 = (param_1 + 0x24);
            sVar6 = (param_1 + 0x24);
            for (local_20 = (param_1 + 0x22) - local_34; local_20 <= local_34 + sVar4; local_20 = local_20 + 0x1) {
                uVar7 = FUN_0043a8a2(local_20);
                for (local_1c = (sVar5 + local_34 * -0x2 + (uVar7 & 0x1)) - ((param_1 + 0x22) & 0x1)
                    ; local_1c <= sVar6 + local_34 * 0x2; local_1c = local_1c + 0x2) {
                    uVar8 = FUN_0043a8d5(uVar7,local_1c);
                    iVar9 = FUN_0044a87f((param_1 + 0x22),(param_1 + 0x24),uVar7,uVar8);
                    if (iVar9 <= local_34) {
                        puVar1 = (*(&DAT_004d7d50 + sVar3 * 0x3890 + uVar7 * 0x4) + uVar8 * 0x4 + 0x4);
                        *puVar1 = *puVar1 | *(&DAT_004be9b0 + (*(param_1 + 0x23) >> 0x18) * 0x4);
                        pbVar2 = (*(&DAT_004d7d50 + uVar7 * 0x4 + sVar3 * 0x3890) + uVar8 * 0x4 + 0x4);
                        *pbVar2 = *pbVar2 | 0x10;
                    }
                }
            }
        }
    }
    return;
}



fn FUN_0044b0ba(param_1: i32)

{
    byte *pbVar1;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let mut iVar6: i32;
    let mut iVar7: i32;
    let mut iVar8: i32;
    let mut local_2c: u32;
    let mut local_20: u32;
    let mut local_1c: i32;

    iVar7 = *(param_1 + 0x79) - *(param_1 + 0x75) >> 0x1;
    iVar2 = *(param_1 + 0x79);
    iVar3 = *(param_1 + 0x25);
    iVar4 = *(param_1 + 0x29);
    iVar5 = *(param_1 + 0x81);
    iVar6 = *(param_1 + 0x4d);
    for (local_20 = *(param_1 + 0x49);
        local_20 < *(param_1 + 0x49) + (iVar3 - iVar7) / (iVar2 - iVar7) + 0x2; local_20 = local_20 + 0x1) {
        iVar8 = FUN_0043a8a2(local_20);
        local_2c = ((local_20 & 0x1) == 0x0);
        for (local_1c = *(param_1 + 0x4d) + local_2c; local_1c <= iVar6 + 0x1 + iVar4 / iVar5;
            local_1c = local_1c + 0x2) {
            pbVar1 = (*(*(param_1 + 0xa1) + iVar8 * 0x4) + local_1c * 0x4 + 0x4);
            *pbVar1 = *pbVar1 & 0xdf;
        }
    }
    return;
}



fn FUN_0044b1a8()

{
    let mut local_118: *mut u32 [0x11];
    let ppuStack211: *mut *mut u8;;
    let mut local_27: String;;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: i32;

    DAT_004c93cc = DAT_004c9754;
    local_18 = *(&DAT_00569adc + DAT_004c9754 * 0x1e22);
    local_1c = *(&DAT_00569ae8 + DAT_004c9754 * 0x1e22);
    for (local_20 = 0x0; local_20 < 0x5; local_20 = local_20 + 0x1) {
        FUN_00489246(local_20,0x0);
    }
    DAT_004c93c8 = FUN_0049c2c9(0xc350);
    FUN_0049c60b(s_bin_portrait_bin_004c1b97,DAT_004c93c8,0xc350,
                 *(&DAT_00569b60 + DAT_004c9754 * 0x1e22) * 0x3d090 + DAT_004c9754 * 0xc350);
    FUN_004990e0(local_118,0x0,s_diplo_res_004c1bb1,s_HouseDlg_004c1ba8);
    FUN_0049bb50(local_118,FUN_0044b5e4);
    if (*(&DAT_00569adc + DAT_004c9754 * 0x1e22) < local_18) {
        FUN_004a1651();
    }
    else {
        if (local_18 < *(&DAT_00569adc + DAT_004c9754 * 0x1e22)) {
            FUN_004a1651();
        }
    }
    if (*(&DAT_00569ae8 + DAT_004c9754 * 0x1e22) < local_1c) {
        FUN_004a1651();
    }
    else {
        if (local_1c < *(&DAT_00569ae8 + DAT_004c9754 * 0x1e22)) {
            FUN_004a1651();
        }
    }
    FUN_0049af50(DAT_004c93c8);
    ppuStack211 = &PTR_FUN_004c3d34;
    if (local_27 != 0x0) {
        FUN_00499b30(local_118,local_27);
    }
    FUN_0049a1c0(local_118,0x1);
    return;
}



fn FUN_0044b3a4(param_1: u32,param_2: i32)

{
    let local_24: u8 [0x10];
    let mut local_14: u32;

    local_14 = param_1;
    if (param_1 < 0x12d) {
        if (param_1 == 0x12c) {
            FUN_0049c2e0(local_24,&DAT_004c1beb);
            FUN_00497567(0x267,0x124,local_24,0x32,0xcaccce,0xe0e0e,0xcaccce,LPCSTR_005b9218,0x14);
            if (dflt_tax_rate_00599d64 < param_2) {
                FUN_00496ee6(&DAT_005923d0,0x1fc,0x124,0xf,0xd);
            }
            else {
                FUN_00496ee6(&DAT_005924d0,0x1fc,0x124,0xf,0xd);
            }
        }
    }
    else {
        if (param_1 < 0x12e) {
            FUN_0049c2e0(local_24,&DAT_004c1bef);
            FUN_00497567(0x267,0x148,local_24,0x32,0xcaccce,0xe0e0e,0xcaccce,LPCSTR_005b9218,0x14);
            if (param_2 < 0x1) {
                FUN_00496ee6(&DAT_005924d0,0x1fc,0x148,0xf,0xd);
            }
            else {
                FUN_00496ee6(&DAT_005923d0,0x1fc,0x148,0xf,0xd);
            }
        }
        else {
            if (param_1 == 0x12e) {
                FUN_0049c2e0(local_24,&DAT_004c1bf3);
                FUN_00497567(0x267,0x16c,local_24,0x32,0xcaccce,0xe0e0e,0xcaccce,LPCSTR_005b9218,0x14);
                if (param_2 < 0x64) {
                    FUN_00496ee6(&DAT_005923d0,0x1fc,0x16c,0xf,0xd);
                }
                else {
                    FUN_00496ee6(&DAT_005924d0,0x1fc,0x16c,0xf,0xd);
                }
            }
        }
    }
    FUN_004281cd();
    return;
}



fn FUN_0044b5e4(param_1: *mut *mut u32,param_2: u32,param_3: u32,param_4: i32) -> u32

{
    let mut puVar1: *mut u8;
    let mut pcVar2: String;
    let mut uVar3: u32;
    let local_1dc: u8 [0x100];
    let mut local_dc: i32;
    let mut local_d8: i32;
    let mut local_d4: i32;
    let mut local_d0: u32;
    let mut local_cc: u32;
    let local_c8: *mut u32;
    let mut local_c4: String;
    let local_c0: *mut u32;
    let mut local_bc: String;
    let local_b8: *mut u32;
    let mut local_b4: String;
    let mut local_b0: i32;
    let local_ac: u8 [0x10];
    let local_9c: u8 [0x80];
    let mut local_1c: u32;
    let mut local_14: u32;

    local_14 = param_2;
    if (param_2 < 0x405) {
        if (param_2 < 0x204) {
            if ((0x200 < param_2) && ((param_2 < 0x202 || (param_2 == 0x203)))) {
                if ((0x11c < param_3) && (((param_3 < 0x26f && (0x7 < param_4)) && (param_4 < 0xfe)))) {
                    FUN_00431980();
                    return 0x1;
                }
                return 0x0;
            }
        }
        else {
            if (param_2 < 0x205) {
                LAB_0044bb9e:
                if ((param_3 < 0x28) || (0xaf < param_3)) {
                    if (((0x199 < param_3) && ((param_3 < 0x1ff && (0x180 < param_4)))) && (param_4 < 0x1b4)) {
                        FUN_00483355(0x2e);
                        return 0x1;
                    }
                }
                else {
                    if ((0x13d < param_4) && (param_4 < 0x1c6)) {
                        FUN_00483355(0x2f);
                        return 0x1;
                    }
                }
                return 0x0;
            }
            if (0x205 < param_2) {
                if (param_2 < 0x207)^ // goto LAB_0044bb9e;
                if (param_2 == 0x401) {
                    FUN_0049bf80(param_1,0x12c,0x502,*(&DAT_00569adc + DAT_004c9754 * 0x1e22),0x0);
                    FUN_0049bf80(param_1,0x12d,0x502,*(&DAT_00569ae4 + DAT_004c9754 * 0x1e22),0x0);
                    FUN_0049bf80(param_1,0x12e,0x502,*(&DAT_00569ae8 + DAT_004c9754 * 0x1e22),0x0);
                }
            }
        }
    }
    else {
        if (param_2 < 0x406) {
            FUN_004953d7();
            FUN_00496ac0(DAT_004c93c8,0xa,0x10,0xfa,0xc8);
            if (DAT_004c9754 < 0x5) {
                FUN_00499050(DAT_0059679c,DAT_004c9754 + 0x414);
                pcVar2 = FUN_00499050(DAT_0059679c,0x73cc);
                FUN_0049c2e0(local_9c,pcVar2);
            }
            else {
                FUN_0049c2e0(local_9c,&DAT_00569b50 + DAT_004c9754 * 0x1e22);
            }
            FUN_00497567(0x88,0xe4,local_9c,0xf0,0xcaccce,0xe0e0e,0xcaccce,LPCSTR_005b9218,0x11);
            FUN_004968e7(0x235,0x181,0x32,0x12,0xe);
            FUN_0049e640(0x235,0x181,0x32,0x12,0xce,0xca,0xcc,0x1);
            FUN_0049c2e0(local_ac,&DAT_004c1c06);
            FUN_00497567(0x267,0x183,local_ac,0x32,0xcaccce,0xe0e0e,0xcaccce,LPCSTR_005b9218,0x14);
            FUN_004968e7(0x235,0x195,0x32,0x12,0xe);
            FUN_0049e640(0x235,0x195,0x32,0x12,0xce,0xca,0xcc,0x1);
            FUN_0049c2e0(local_ac,&DAT_004c1c09);
            FUN_00497567(0x267,0x197,local_ac,0x32,0xcaccce,0xe0e0e,0xcaccce,LPCSTR_005b9218,0x14);
            FUN_004968e7(0x235,0x1a9,0x32,0x12,0xe);
            FUN_0049e640(0x235,0x1a9,0x32,0x12,0xce,0xca,0xcc,0x1);
            for (local_b0 = 0x0; local_b0 < 0x3; local_b0 = local_b0 + 0x1) {
                FUN_004968e7(0x235,local_b0 * 0x24 + 0x121,0x32,0x12,0xe);
                FUN_0049e640(0x235,local_b0 * 0x24 + 0x121,0x32,0x12,0xce,0xca,0xcc,0x1);
                puVar1 = FUN_0049bf80(param_1,local_b0 + 0x12c,0x501,0x0,0x0);
                FUN_0044b3a4(local_b0 + 0x12c,puVar1);
            }
            local_c8 = &DAT_004d6a30;
            local_c4 = DAT_004d6a30;
            FUN_00496ee6(DAT_004d6a30,0x20d,0x116,0x20,0x20);
            local_c0 = &DAT_004d6608;
            local_bc = DAT_004d6608;
            FUN_00496ee6(DAT_004d6608,0x20d,0x139,0x20,0x20);
            local_b8 = &DAT_004d696c;
            local_b4 = DAT_004d696c;
            FUN_00496ee6(DAT_004d696c,0x20d,0x15c,0x20,0x20);
            FUN_0044bf63();
            FUN_004281cd();
            FUN_004312f0(0x0,0x11d,0x8,0x152,0xf6);
            FUN_0049536f();
        }
        else {
            if (param_2 < 0x40c) {
                if (param_2 < 0x407) {
                    FUN_004953d7();
                    local_1c = param_3;
                    if (param_3 < 0x12d) {
                        if ((param_3 == 0x12c) && (*(&DAT_00569adc + DAT_004c9754 * 0x1e22) != param_4)) {
                            *(&DAT_00569adc + DAT_004c9754 * 0x1e22) = param_4;
                            FUN_0048976a(DAT_004c9754,0x0);
                            FUN_0044b3a4(0x12c,param_4);
                        }
                    }
                    else {
                        if (param_3 < 0x12e) {
                            if (*(&DAT_00569ae4 + DAT_004c9754 * 0x1e22) != param_4) {
                                *(&DAT_00569ae4 + DAT_004c9754 * 0x1e22) = param_4;
                                FUN_0044b3a4(param_3,param_4);
                            }
                        }
                        else {
                            if ((param_3 == 0x12e) && (*(&DAT_00569ae8 + DAT_004c9754 * 0x1e22) != param_4)) {
                                *(&DAT_00569ae8 + DAT_004c9754 * 0x1e22) = param_4;
                                FUN_0044b3a4(0x12e,param_4);
                            }
                        }
                    }
                    FUN_0049536f();
                    return 0x1;
                }
                if ((param_2 == 0x407) && (local_d0 = param_3, param_3 == 0x64)) {
                    local_dc = 0x1;
                    local_d8 = *(&DAT_00569ae8 + DAT_004c9754 * 0x1e22) - *(&DAT_00569af0 + DAT_004c9754 * 0x1e22);
                    local_d4 = *(&DAT_00569adc + DAT_004c9754 * 0x1e22) - *(&DAT_00569aec + DAT_004c9754 * 0x1e22);
                    if (local_d8 < 0x0) {
                        pcVar2 = FUN_00499050(DAT_0059679c,0x740f);
                        FUN_0049c2e0(local_1dc,pcVar2);
                        FUN_0049d2b0(0xfa,0xc8);
                        uVar3 = FUN_0049dc40(param_1,0x3,local_1dc,s_bin_ptrattut_bin_004c1c0c);
                        if (uVar3 == 0x0) {
                            local_dc = 0x0;
                        }
                    }
                    if (local_dc == 0x0) {
                        return 0x0;
                    }
                    if (0x0 < local_d4) {
                        pcVar2 = FUN_00499050(DAT_0059679c,0x741f);
                        FUN_0049c2e0(local_1dc,pcVar2);
                        FUN_0049d2b0(0xfa,0xc8);
                        uVar3 = FUN_0049dc40(param_1,0x3,local_1dc,s_bin_ptrattut_bin_004c1c1d);
                        if (uVar3 == 0x0) {
                            local_dc = 0x0;
                        }
                    }
                    if (local_dc != 0x0) {
                        FUN_0049c140(param_1,0x1);
                    }
                    return 0x0;
                }
            }
            else {
                if (param_2 < 0x40d) {
                    FUN_004953d7();
                    FUN_004a08c5(s_pcx_house_pcx_004c1bf8,0x0,0x0,0x280,0x1e0,0x0,0x0,0x0,0x1);
                    FUN_0049536f();
                    return 0x1;
                }
                if (0x410 < param_2) {
                    if (param_2 < 0x412) {
                        return 0x1;
                    }
                    if (param_2 == 0x413) {
                        local_cc = param_3;
                        if (param_3 < 0x12d) {
                            if (param_3 == 0x12c) {
                                FUN_00483355(0x2b);
                            }
                        }
                        else {
                            if (param_3 < 0x12e) {
                                FUN_00483355(0x2c);
                            }
                            else {
                                if (param_3 == 0x12e) {
                                    FUN_00483355(0x2d);
                                }
                            }
                        }
                        return 0x0;
                    }
                }
            }
        }
    }
    return 0x0;
}



fn FUN_0044bf63()

{
    let mut local_18: i32;
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0x5; local_14 = local_14 + 0x1) {
        FUN_0048f678(0x12a,local_14 * 0x5 + 0x123,0x5d,0x4,*(&DAT_00569ab5 + local_14 * 0x1e22),0x23,
                     *(&DAT_004be9e8 + local_14 * 0x4),-0x1,0x0,0x0);
        FUN_0048f678(0x12a,local_14 * 0x5 + 0x14a,0x5d,0x4,*(&DAT_00569ab9 + local_14 * 0x1e22),0xfa0,
                     *(&DAT_004be9e8 + local_14 * 0x4),-0x1,0x0,0x0);
        FUN_0048f678(0x12a,local_14 * 0x5 + 0x171,0x5d,0x4,*(&DAT_00569abd + local_14 * 0x1e22),0xf423f,
                     *(&DAT_004be9e8 + local_14 * 0x4),-0x1,0x0,0x0);
        FUN_0048f678(0x12a,local_14 * 0x5 + 0x198,0x5d,0x4,*(&DAT_00569ac1 + local_14 * 0x1e22),0x1f4,
                     *(&DAT_004be9e8 + local_14 * 0x4),-0x1,0x0,0x0);
        FUN_0048f678(0x12a,local_14 * 0x5 + 0x1bf,0x5d,0x4,*(&DAT_00569a99 + local_14 * 0x1e22),0x72,
                     *(&DAT_004be9e8 + local_14 * 0x4),-0x1,0x0,0x0);
        for (local_18 = local_14; local_18 < 0x5; local_18 = local_18 + 0x1) {
            if ((&DAT_004d55a8)[local_14 * 0xe + local_18] != '\0') {
                FUN_00496ac0((&DAT_005925d0 + ((byte)(&DAT_004d55a8)[local_14 * 0xe + local_18] - 0x1) * 0x2d9),
                             local_14 * 0x1c + 0x28,local_18 * 0x1c + 0x13e,0x1b,0x1b);
                FUN_00496ac0((&DAT_005925d0 + ((byte)(&DAT_004d55a8)[local_14 * 0xe + local_18] - 0x1) * 0x2d9),
                             local_18 * 0x1c + 0x28,local_14 * 0x1c + 0x13e,0x1b,0x1b);
            }
        }
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0044c1ce(param_1: i32)

{
    let mut local_140: *mut u32 [0x11];
    let ppuStack251: *mut *mut u8;;
    let mut local_4f: String;;
    let local_48: *mut i32;;
    let local_44: *mut i32;;
    let local_40: *mut i32;;
    let local_3c: *mut i32;;
    let local_38: *mut u32;
    let local_34: *mut i32;;
    let local_30: *mut i32;;
    let local_2c: *mut i32;;
    let mut local_28: u32;
    let mut local_24: u32;
    u32 **local_20;
    let mut local_1c: u32;

    FUN_0044e131();
    DAT_0059a9f4 = param_1;
    DAT_0059aa1c = *(param_1 + 0x6) >> 0x10;
    local_34 = FUN_004990e0(local_140,0x0,s_efs_res_004c1c35,s_LabDlg_004c1c2e);
    FUN_00463f7b();
    _DAT_0059aa0c = 0x3de;
    DAT_0059aa10 = 0x0;
    DAT_0059aa18 = 0x0;
    local_48 = FUN_004a2831(0xb9);
    local_44 = local_48;
    local_30 = local_48;
    if (local_48 != 0x0) {
        local_30 = FUN_00438792(local_48,local_140,0x1f4,0x1c,0xe0,0x58,0x3f,0x40,0x2,0x2,
                                *(DAT_0059a9f4 + 0x6) >> 0x10);
    }
    DAT_005b8c38 = local_30;
    local_40 = FUN_004a2831(0x95);
    local_3c = local_40;
    local_2c = local_40;
    if (local_40 != 0x0) {
        local_2c = FUN_0047157e(local_40,local_140,0x1f5,0x16,0x69,0x64,0x60,0x40,0x2);
    }
    DAT_005b8c3c = local_2c;
    FUN_0049bf40(local_140,DAT_005b8c38);
    FUN_0049bf40(local_140,DAT_005b8c3c);
    local_38 = FUN_0049a250(local_140,0x4);
    *local_38 = DAT_0059aa18;
    FUN_0049bb50(local_140,FUN_0044cd2e);
    if (DAT_005b8c38 == 0x0) {
        local_28 = 0x0;
    }
    else {
        local_28 = ((*(DAT_005b8c38 + 0x45) + 0x8))(DAT_005b8c38,0x2);
    }
    if (DAT_005b8c3c == 0x0) {
        local_24 = 0x0;
    }
    else {
        local_24 = ((*(DAT_005b8c3c + 0x45) + 0x8))(DAT_005b8c3c,0x2);
    }
    local_20 = local_140;
    local_1c = 0x0;
    ppuStack251 = &PTR_FUN_004c3d34;
    if (local_4f != 0x0) {
        FUN_00499b30(local_140,local_4f);
    }
    FUN_0049a1c0(local_140,0x1);
    return;
}



fn FUN_0044c451() -> u32

{
    let mut local_128: *mut u32 [0x11];
    let ppuStack227: *mut *mut u8;;
    let mut local_37: String;;
    let mut local_30: u32;
    let mut local_2c: u32;
    let local_28: *mut u32;
    let local_24: *mut i32;;
    u32 **local_20;
    let mut local_1c: u32;

    local_2c = 0x0;
    DAT_0059aa18 = 0x1;
    local_24 = FUN_004990e0(local_128,0x0,s_efs_res_004c1c45,s_Archive_004c1c3d);
    local_28 = FUN_0049a250(local_128,0x4);
    *local_28 = DAT_0059aa18;
    local_30 = FUN_0049bb50(local_128,FUN_0044cd2e);
    local_20 = local_128;
    local_1c = 0x0;
    ppuStack227 = &PTR_FUN_004c3d34;
    if (local_37 != 0x0) {
        FUN_00499b30(local_128,local_37);
    }
    FUN_0049a1c0(local_128,0x1);
    return local_30;
}



fn FUN_0044c578(param_1: *mut *mut u32,param_2: i32,param_3: i32,param_4: u32)

{
    let puVar1: *mut u32;
    let mut pcVar2: String;
    let mut iVar3: i32;
    let mut bVar4: bool;
    let mut local_1c: u32;
    let mut local_18: i32;
    let mut local_14: i32;

    local_18 = 0x0;
    local_14 = 0x0;
    FUN_00489246(DAT_004c9754,0x0);
    puVar1 = FUN_0049ea90();
    if (puVar1 == 0x0) {
        pcVar2 = FUN_00499050(DAT_005b9bd8,0x7d01);
        pop_err_msg_box_and_exit_004a02f5(pcVar2);
    }
    if (param_3 == 0x0) {
        local_14 = 0x0;
    }
    DAT_0059aa14 = 0xffffffff;
    for (local_1c = (param_3 != 0x0); local_1c < 0x72; local_1c = local_1c + 0x1) {
        iVar3 = FUN_0046295d((&DAT_0058aca8 + local_1c * 0xda),0x0);
        if (iVar3 != 0x320) {
            bVar4 = true;
            if (param_3 == 0x0) {
                bVar4 = ((&DAT_00569c30)[local_1c * 0x9 + DAT_004c9754 * 0x1e22] & 0x2) != 0x0;
            }
            else {
                if (DAT_0059aa18 == 0x1) {
                    bVar4 = ((&DAT_00569c30)[local_1c * 0x9 + DAT_004c9754 * 0x1e22] & 0x1) != 0x0;
                }
                else {
                    if (DAT_0059aa18 == 0x2) {
                        bVar4 = *(&DAT_0058ad72 + local_1c * 0xda) == 0x0;
                        if ((((&DAT_00569c30)[local_1c * 0x9 + DAT_004c9754 * 0x1e22] & 0x1) == 0x0) &&
                            (((&DAT_00569c30)[local_1c * 0x9 + DAT_004c9754 * 0x1e22] & 0x2) == 0x0)) {
                            bVar4 = false;
                        }
                        if (DAT_004c93d0 == 0x0) {
                            if ((((&DAT_00569c30)[local_1c * 0x9 + DAT_004c9754 * 0x1e22] & 0x1) == 0x0) ||
                                (((&DAT_00569c30)[local_1c * 0x9 + DAT_004c93cc * 0x1e22] & 0x1) != 0x0)) {
                                bVar4 = false;
                            }
                        }
                        else {
                            if ((((&DAT_00569c30)[local_1c * 0x9 + DAT_004c9754 * 0x1e22] & 0x1) != 0x0) ||
                                (((&DAT_00569c30)[local_1c * 0x9 + DAT_004c93cc * 0x1e22] & 0x1) == 0x0)) {
                                bVar4 = false;
                            }
                        }
                    }
                    else {
                        if ((DAT_0059aa18 == 0x3) &&
                            ((*(&DAT_0058ad72 + local_1c * 0xda) != 0x0 || (*(&DAT_0058ad6e + local_1c * 0xda) == 0xa))
                            )) {
                            bVar4 = false;
                        }
                    }
                }
            }
            if (bVar4) {
                if ((param_3 == 0x0) && (param_4 == local_1c)) {
                    DAT_0059aa14 = local_1c;
                    local_14 = local_18;
                }
                FUN_0049eae0(puVar1,&local_1c,0x4);
                local_18 = local_18 + 0x1;
            }
            else {
                if (0x383 < *(&DAT_0058ad5e + local_1c * 0xda)) {
                    FUN_0049eae0(puVar1,&stack0xffffffd8,0x4);
                    local_18 = local_18 + 0x1;
                }
            }
        }
    }
    if (((DAT_0059aa14 == 0xffffffff) && (param_3 == 0x0)) && (((&DAT_00569c30)[DAT_004c9754 * 0x1e22] & 0x4) != 0x0)) {
        DAT_0059aa14 = 0x0;
    }
    FUN_0049bf80(param_1,param_2,0x503,0x0,puVar1);
    FUN_0049bf80(param_1,param_2,0x502,local_14,0x0);
    return;
}



fn FUN_0044c88d(param_1: *mut i32)

{
    let mut bVar1: bool;
    let mut local_78: *mut u8;
    let mut local_74: *mut u8;
    let mut local_70: *mut u8;
    let mut local_6c: *mut u8;
    let local_68: u8 [0x40];
    let local_28: *mut i32;;
    let mut local_24: *mut u8;
    let mut local_20: *mut u8;
    let mut local_1c: i32;
    let mut local_18: u32;
    let mut local_14: i32;

    local_28 = param_1;
    local_1c = *param_1[0x5];
    bVar1 = local_1c < 0x0;
    if (bVar1) {
        local_1c = -local_1c;
    }
    local_18 = bVar1;
    if (param_1[0x4] == 0x1) {
        if ((DAT_0059aa18 == 0x0) && (*(DAT_0059a9f4 + 0x24) >> 0x10 == local_1c)) {
            local_20 = &DAT_00535557;
        }
        else {
            local_20 = 0xe0e0e;
        }
        if (*(&DAT_0058ad72 + local_1c * 0xda) == 0x0) {
            if (local_18 == 0x0) {
                local_70 = 0xcaccce;
            }
            else {
                local_70 = 0x636567;
            }
            local_24 = local_70;
        }
        else {
            if (local_18 == 0x0) {
                local_6c = 0xe2e3e4;
            }
            else {
                local_6c = 0xe3e5e7;
            }
            local_24 = local_6c;
        }
    }
    else {
        if (local_18 == 0x0) {
            local_74 = 0xcaccce;
        }
        else {
            local_74 = 0x636567;
        }
        local_20 = local_74;
        local_24 = 0xe0e0e;
        if ((DAT_0059aa18 == 0x0) && (*(DAT_0059a9f4 + 0x24) >> 0x10 == local_1c)) {
            local_20 = &DAT_00535557;
        }
        if (*(&DAT_0058ad72 + local_1c * 0xda) != 0x0) {
            if (local_18 == 0x0) {
                local_78 = 0xe2e3e4;
            }
            else {
                local_78 = 0xe3e5e7;
            }
            if (local_20 == &DAT_00535557) {
                local_24 = local_78;
            }
            else {
                local_20 = local_78;
            }
        }
    }
    if (*(&DAT_0058ad5e + local_1c * 0xda) < 0x384) {
        FUN_00497567(*param_1,param_1[0x1],&DAT_004c1c4d,0x10,local_20,local_24,0xcaccce,
                     LPCSTR_005b9218,0x10);
        local_14 = 0x10;
    }
    else {
        local_14 = 0x0;
    }
    FUN_004953d7();
    if (DAT_0059aa18 == 0x0) {
        if (local_18 == 0x0) {
            FUN_0049c2e0(local_68,&DAT_004c1c4e);
            FUN_00497567(*local_28 + local_14,local_28[0x1],local_68,0x10e,local_20,local_24,0xcaccce,
                         LPCSTR_005b9218,0x10);
            FUN_0049c2e0(local_68,&DAT_004c1c51);
            FUN_00497567(*local_28 + 0x140,local_28[0x1],local_68,0x32,local_20,local_24,0xcaccce,
                         LPCSTR_005b9218,0x14);
            FUN_0049c2e0(local_68,&DAT_004c1c55);
            FUN_00497567(*local_28 + 0x17c,local_28[0x1],local_68,0x3c,local_20,local_24,0xcaccce,
                         LPCSTR_005b9218,0x14);
            FUN_0049c2e0(local_68,&DAT_004c1c59);
            FUN_00497567(*local_28 + 0x1cb,local_28[0x1],local_68,local_28[0x2] + -0x17c,local_20,local_24,0xcaccce,
                         LPCSTR_005b9218,0x14);
        }
        else {
            FUN_00497567(*local_28,local_28[0x1],(&DAT_0058aca8 + local_1c * 0xda),local_28[0x2],local_20,local_24,
                         0xcaccce,LPCSTR_005b9218,0x10);
        }
    }
    else {
        if (DAT_0059aa18 == 0x1) {
            FUN_0049c2e0(local_68,&DAT_004c1c5d);
            FUN_00497567(*local_28 + local_14,local_28[0x1],local_68,0x10e,local_20,local_24,0xcaccce,
                         LPCSTR_005b9218,0x10);
            FUN_0049c2e0(local_68,&DAT_004c1c60);
            FUN_00497567(*local_28 + 0x1cb,local_28[0x1],local_68,local_28[0x2] + -0x10e,local_20,local_24,0xcaccce,
                         LPCSTR_005b9218,0x14);
        }
        else {
            FUN_0049c2e0(local_68,&DAT_004c1c63);
            FUN_00497567(*local_28 + local_14,local_28[0x1],local_68,local_28[0x2] - local_14,local_20,local_24,
                         0xcaccce,LPCSTR_005b9218,0x10);
        }
    }
    FUN_0049536f();
    return;
}



// WARNING: Type propagation algorithm not settling

fn FUN_0044cd2e(param_1: *mut *mut u32,param_2: u32,param_3: u32,param_4: *mut i32) -> u32

{
    let mut bVar1: bool;
    let mut iVar2: i32;
    undefined3 extraout_var;
    let mut uVar3: u32;
    let mut puVar4: *mut u8;
    let piVar5: *mut i32;;
    let mut pcVar6: String;
    let mut local_318: i32;
    let local_2f8: u8 [0x80];
    let mut local_278: *mut u8;
    let mut local_274: i32;
    let mut local_270: u32;
    let local_26c: u8 [0x40];
    let mut local_22c: i32;
    let local_228: *mut u32;
    u32 **local_224;
    let local_220: *mut u32;
    let local_21c: u8 [0x200];
    let mut local_1c: i32;
    let mut local_14: u32;

    local_14 = param_2;
    if (param_2 < 0x406) {
        if (param_2 < 0x401) {
            if (((param_2 == 0x100) && (param_4 == 0x1b)) && ((DAT_0059aa18 == 0x0 && (DAT_0059aa14 == -0x1)))) {
                return 0x1;
            }
        }
        else {
            if (param_2 < 0x402) {
                if (DAT_0059a9f4 == 0x0) {
                    FUN_0044c578(param_1,0xc8,DAT_0059aa18,0x1);
                }
                else {
                    FUN_0044c578(param_1,0xc8,DAT_0059aa18,*(DAT_0059a9f4 + 0x24) >> 0x10);
                }
                if (((DAT_0059aa18 == 0x0) && (DAT_0059aa14 == -0x1)) && (((&DAT_00569c30)[DAT_004c9754 * 0x1e22] & 0x8) == 0x0)
                ) {
                    FUN_0049bf80(param_1,0x65,0x414,0x0,0x0);
                    FUN_0049bf80(param_1,0x65,0x410,0x0,0x0);
                }
                if (DAT_0059aa18 == 0x2) {
                    FUN_0049bf80(param_1,0x15e,0x414,0x0,0x0);
                    FUN_0049bf80(param_1,0x15e,0x410,0x0,0x0);
                }
                else {
                    if (DAT_0059aa18 == 0x3) {
                        FUN_0049bf80(param_1,0xc5,0x414,0x0,0x0);
                        FUN_0049bf80(param_1,0xc5,0x410,0x0,0x0);
                    }
                }
                FUN_0049f99b(DAT_005b9bd4,param_1,0x407,0x3039,0x0);
                return 0x1;
            }
            if (param_2 == 0x405) {
                local_224 = param_1;
                local_228 = param_1[0x6];
                if (*local_228 == 0x0) {
                    local_220 = local_228;
                    FUN_004953d7();
                    FUN_00497567(0x48,0xd7,(&DAT_005b709e + DAT_0059aa1c * 0x4e),0x91,0xcaccce,-0x1,0xcaccce,DAT_004d6a6c,
                                 0x19);
                    FUN_0049e640(*(DAT_005b8c38 + 0x1d),*(DAT_005b8c38 + 0x21),*(DAT_005b8c38 + 0x25),
                                 *(DAT_005b8c38 + 0x29),0xce,0xca,0xcc,0x1);
                    FUN_0049e640(*(DAT_005b8c3c + 0x1d),*(DAT_005b8c3c + 0x21),*(DAT_005b8c3c + 0x25),
                                 *(DAT_005b8c3c + 0x29),0xce,0xca,0xcc,0x1);
                    FUN_00497045(0x23,0x131,0x4b,0x4b,*(&DAT_00595700 + DAT_004c9754 * 0x4),0x64,0x64);
                    FUN_0049e640(0x23,0x131,0x4b,0x4b,0xce,0xca,0xcc,0x1);
                    iVar2 = FUN_0046645d(*(DAT_0059a9f4 + 0x6) >> 0x10,*(DAT_0059a9f4 + 0x8) >> 0x10,
                                         *(DAT_0059a9f4 + 0xa) >> 0x10);
                    local_22c = (iVar2 * 0x64) / 0x64;
                    if (0x0 < local_22c) {
                        bVar1 = FUN_0046292a((&DAT_00568210 + DAT_004c9754 * 0x1e22));
                        if (((CONCAT31(extraout_var,bVar1) == 0x0) || (DAT_004d7a63 >> 0x18 != DAT_004c9754)) ||
                            (iVar2 = FUN_00489176(*(DAT_0059a9f4 + 0x6) >> 0x10,*(DAT_0059a9f4 + 0x8) >> 0x10,
                                                  *(DAT_0059a9f4 + 0xa) >> 0x10,0x7), iVar2 == 0x0)) {
                            local_22c = local_22c + ((*(&DAT_00569b6c + DAT_004c9754 * 0x1e22) >> 0x18) * local_22c) / 0x64;
                        }
                        else {
                            local_22c = local_22c +
                                (((*(&DAT_00569b6c + DAT_004c9754 * 0x1e22) >> 0x18) + DAT_004d7a01) * local_22c) /
                                    0x64;
                        }
                    }
                    pcVar6 = FUN_00499050(DAT_0059679c,0x741d);
                    FUN_0049c2e0(local_26c,pcVar6);
                    FUN_00497567(0x91,0xeb,local_26c,0x190,0xcaccce,-0x1,0xcaccce,LPCSTR_005b9218,0x10);
                    pcVar6 = FUN_00499050(DAT_0059679c,0x73b9);
                    FUN_0049c2e0(local_26c,pcVar6);
                    FUN_00497567(0x91,0xfa,local_26c,0x190,0xcaccce,-0x1,0xcaccce,LPCSTR_005b9218,0x10);
                    pcVar6 = FUN_00499050(DAT_0059679c,0x7409);
                    FUN_0049c2e0(local_26c,pcVar6);
                    FUN_00497567(0x91,0x109,local_26c,0x190,0xcaccce,-0x1,0xcaccce,LPCSTR_005b9218,0x10);
                    FUN_0049536f();
                }
                return 0x0;
            }
        }
        return 0x0;
    }
    if (param_2 < 0x407) {
        if (param_3 != 0xc8) {
            return 0x0;
        }
        piVar5 = FUN_0049bf80(param_1,0xc8,0x510,param_4,0x0);
        local_1c = *piVar5;
        if (local_1c < 0x0) {
            FUN_0049c2e0(local_21c,&DAT_004c1c66);
        }
        else {
            FUN_0044dd16(local_21c,local_1c);
            FUN_0049c2e0(local_21c,s__s__s_004c1c68);
        }
        FUN_0049bf80(param_1,0xc0,0x502,0x0,local_21c);
        return 0x0;
    }
    if (0x410 < param_2) {
        if (param_2 < 0x412) {
            return 0x1;
        }
        if (param_2 != 0x412) {
            return 0x0;
        }
        FUN_0044c88d(param_4);
        return 0x1;
    }
    if (param_2 != 0x407) {
        return 0x0;
    }
    local_270 = param_3;
    if (param_3 < 0xc4) {
        if (param_3 < 0xbf) {
            if (param_3 != 0x65) {
                return 0x0;
            }
            if (DAT_0059aa18 == 0x1) {
                DAT_0059aa18 = 0x0;
            }
            FUN_0049c140(param_1,0x0);
            return 0x0;
        }
        if (param_3 < 0xc0) {
            puVar4 = FUN_0049bf80(param_1,0xc8,0x501,0x0,0x0);
            if (puVar4 == 0xffffffff) {
                return 0x0;
            }
            piVar5 = FUN_0049bf80(param_1,0xc8,0x510,puVar4,0x0);
            local_318 = *piVar5;
            if (local_318 < 0x0) {
                local_318 = -local_318;
            }
            FUN_00453c16(*(&DAT_0058ad76 + local_318 * 0xda),*(&DAT_0058ad7a + local_318 * 0xda));
            return 0x0;
        }
        if (param_3 < 0xc2) {
            return 0x0;
        }
        if (0xc2 < param_3) {
            uVar3 = FUN_0044c451();
            if (uVar3 == 0x0) {
                return 0x0;
            }
            FUN_00463f7b();
            FUN_0044cd2e(param_1,0x401,0x0,0x0);
            return 0x0;
        }
        puVar4 = FUN_0049bf80(param_1,0xc8,0x501,0x0,0x0);
        if (puVar4 == 0xffffffff) {
            return 0x0;
        }
        piVar5 = FUN_0049bf80(param_1,0xc8,0x510,puVar4,0x0);
        iVar2 = *piVar5;
        if (iVar2 < 0x0) {
            pcVar6 = FUN_00499050(DAT_0059679c,0x73b8);
            FUN_0049d2e0(param_1,0x1,pcVar6);
            return 0x0;
        }
        pcVar6 = FUN_00499050(DAT_0059679c,0x733e);
        uVar3 = FUN_0049d2e0(param_1,0x3,pcVar6);
        if (uVar3 == 0x0) {
            return 0x0;
        }
        (&DAT_00569c30)[iVar2 * 0x9 + DAT_004c9754 * 0x1e22] = (&DAT_00569c30)[iVar2 * 0x9 + DAT_004c9754 * 0x1e22] & 0xfe;
        (&DAT_00569c30)[DAT_004c9754 * 0x1e22 + iVar2 * 0x9] = (&DAT_00569c30)[DAT_004c9754 * 0x1e22 + iVar2 * 0x9] | 0x2;
        (&DAT_00569c30)[DAT_004c9754 * 0x1e22] = (&DAT_00569c30)[DAT_004c9754 * 0x1e22] & 0xf7;
        DAT_0059aa18 = 0x0;
        FUN_0049c140(param_1,0x1);
        return 0x0;
    }
    if (0xc4 < param_3) {
        if (param_3 < 0xc8) {
            if (param_3 != 0xc5) {
                return 0x0;
            }
            puVar4 = FUN_0049bf80(param_1,0xc8,0x501,0x0,0x0);
            if (puVar4 == 0xffffffff) {
                return 0x0;
            }
            piVar5 = FUN_0049bf80(param_1,0xc8,0x510,puVar4,0x0);
            iVar2 = *piVar5;
            if (iVar2 < 0x0) {
                pcVar6 = FUN_00499050(DAT_0059679c,0x73b8);
                FUN_0049d2e0(param_1,0x1,pcVar6);
                return 0x0;
            }
            if (DAT_004c93d0 == 0x0) {
                *(&DAT_004c9800 + DAT_004c93d4 * 0x28 + DAT_004c93cc * 0xf8 + DAT_004c9754 * 0xd90) = iVar2;
                *(&DAT_004c97f8 + DAT_004c93d4 * 0x28 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8) = 0x136;
            }
            else {
                *(&DAT_004c9788 + DAT_004c93d4 * 0x28 + DAT_004c93cc * 0xf8 + DAT_004c9754 * 0xd90) = iVar2;
                *(&DAT_004c9780 + DAT_004c93d4 * 0x28 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8) = 0x136;
            }
            FUN_0049c140(param_1,0x136);
            return 0x0;
        }
        if (param_3 < 0xc9) {
            local_274 = DAT_0059aa18;
            switch(DAT_0059aa18) {
                case 0x0:
                    FUN_0044cd2e(param_1,0x407,0xc4,0x0);
                return 0x0;
                case 0x1:
                    FUN_0044cd2e(param_1,0x407,0xc2,0x0);
                return 0x0;
                case 0x2:
                    FUN_0044cd2e(param_1,0x407,0xc5,0x0);
                return 0x0;
                case 0x3:
                    FUN_0044cd2e(param_1,0x407,0x15e,0x0);
                return 0x0;
                default:
                return 0x0;
            }
        }
        if (param_3 < 0x15e) {
            return 0x0;
        }
        if (0x15e < param_3) {
            if (param_3 != 0x3039) {
                return 0x0;
            }
            FUN_00483355(0x30);
            return 0x0;
        }
        puVar4 = FUN_0049bf80(param_1,0xc8,0x501,0x0,0x0);
        if (puVar4 == 0xffffffff) {
            return 0x0;
        }
        piVar5 = FUN_0049bf80(param_1,0xc8,0x510,puVar4,0x0);
        if (*piVar5 < 0x0) {
            pcVar6 = FUN_00499050(DAT_0059679c,0x73b8);
            FUN_0049d2e0(param_1,0x1,pcVar6);
            return 0x0;
        }
        if (DAT_004c93d0 == 0x0) {
            return 0x0;
        }
        *(&DAT_004c97a0 + DAT_004c93d4 * 0x28 + DAT_004c93cc * 0xf8 + DAT_004c9754 * 0xd90) = *piVar5;
        *(&DAT_004c9780 + DAT_004c93d4 * 0x28 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8) = 0x15e;
        FUN_0049c140(param_1,0x15e);
        return 0x0;
    }
    local_278 = FUN_0049bf80(param_1,0xc8,0x501,0x0,0x0);
    if (local_278 == 0xffffffff) {
        return 0x0;
    }
    if (local_278 == 0x0) {
        pcVar6 = FUN_00499050(DAT_0059679c,0x73b6);
        uVar3 = FUN_0049d2e0(param_1,0x3,pcVar6);
        if (uVar3 == 0x0) {
            return 0x0;
        }
        *(DAT_0059a9f4 + 0x26) = 0x0;
        FUN_0049c140(param_1,0x0);
    }
    piVar5 = FUN_0049bf80(param_1,0xc8,0x510,local_278,0x0);
    DAT_0059aa10 = *piVar5;
    if (DAT_0059aa10 < 0x0) {
        pcVar6 = FUN_00499050(DAT_0059679c,0x73b7);
        FUN_0049d2e0(param_1,0x1,pcVar6);
        return 0x0;
    }
    if (*(&DAT_0058ad72 + DAT_0059aa10 * 0xda) == 0x0) {
        pcVar6 = FUN_00499050(DAT_0059679c,0x73ba);
        FUN_0049c2e0(local_2f8,pcVar6);
        if (local_278 == 0x0)^ // goto LAB_0044d586;
        uVar3 = FUN_0049d2e0(param_1,0x3,local_2f8);
    }
    else {
        pcVar6 = FUN_00499050(DAT_0059679c,0x73b5);
        FUN_0049c2e0(local_2f8,pcVar6);
        uVar3 = FUN_0049d2e0(param_1,0x3,local_2f8);
    }
    if (uVar3 == 0x0) {
        return 0x0;
    }
    LAB_0044d586:
    if (DAT_0059aa14 == -0x1) {
        if (*(&DAT_00569c31 + DAT_0059aa10 * 0x9 + DAT_004c9754 * 0x1e22) == 0x0) {
            *(&DAT_00569c31 + DAT_0059aa10 * 0x9 + DAT_004c9754 * 0x1e22) =
                *(&DAT_0058ad6a + DAT_0059aa10 * 0xda);
        }
        *(&DAT_00569c31 + DAT_0059aa10 * 0x9 + DAT_004c9754 * 0x1e22) =
            *(&DAT_00569c31 + DAT_0059aa10 * 0x9 + DAT_004c9754 * 0x1e22) - (*(DAT_0059a9f4 + 0x18) >> 0x10);
        *(DAT_0059a9f4 + 0x1a) = 0x0;
        if (DAT_0059aa10 == 0x0) {
            *(&DAT_00569c31 + DAT_004c9754 * 0x1e22) = 0x0;
        }
    }
    else {
        if (DAT_0059aa10 == DAT_0059aa14) {
            FUN_0049c140(param_1,0x0);
            return 0x0;
        }
        if (*(&DAT_00569c35 + DAT_0059aa14 * 0x9 + DAT_004c9754 * 0x1e22) < 0x2) {
            pcVar6 = FUN_00499050(DAT_0059679c,0x7150);
            uVar3 = FUN_0049d2e0(param_1,0x2,pcVar6);
            if (uVar3 == 0x0) {
                return 0x0;
            }
            *(&DAT_00569c31 + DAT_004c9754 * 0x1e22 + DAT_0059aa14 * 0x9) =
                *(&DAT_0058ad6a + DAT_0059aa14 * 0xda);
            (&DAT_00569c30)[DAT_0059aa14 * 0x9 + DAT_004c9754 * 0x1e22] =
                (&DAT_00569c30)[DAT_0059aa14 * 0x9 + DAT_004c9754 * 0x1e22] & 0xfb;
        }
        *(&DAT_00569c35 + DAT_0059aa14 * 0x9 + DAT_004c9754 * 0x1e22) =
            *(&DAT_00569c35 + DAT_0059aa14 * 0x9 + DAT_004c9754 * 0x1e22) + -0x1;
        *(&DAT_00569c31 + DAT_004c9754 * 0x1e22 + DAT_0059aa10 * 0x9) =
            *(&DAT_0058ad6a + DAT_0059aa10 * 0xda);
    }
    (&DAT_00569c30)[DAT_0059aa10 * 0x9 + DAT_004c9754 * 0x1e22] =
        (&DAT_00569c30)[DAT_0059aa10 * 0x9 + DAT_004c9754 * 0x1e22] | 0x4;
    *(&DAT_00569c35 + DAT_004c9754 * 0x1e22 + DAT_0059aa10 * 0x9) =
        *(&DAT_00569c35 + DAT_004c9754 * 0x1e22 + DAT_0059aa10 * 0x9) + 0x1;
    *(DAT_0059a9f4 + 0x26) = (undefined2)DAT_0059aa10;
    FUN_0049c140(param_1,0x0);
    return 0x0;
}



fn FUN_0044dd16(byte *param_1,param_2: i32) -> u32

{
    let bVar1: u8;
    let mut in_EAX: u32;
    let mut iVar2: i32;
    let mut pcVar3: String;
    let mut uVar4: u32;
    byte *pbVar5;
    byte *pbVar6;
    byte *pbVar7;
    u32 local_1e4 [0x72];
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: u32;

    pbVar5 = &DAT_004c1c6f;
    pbVar6 = param_1;
    loop {
    bVar1 = *pbVar5;
    uVar4 = in_EAX & 0xffffff00;
    in_EAX = uVar4 | bVar1;
    *pbVar6 = bVar1;
    if (bVar1 == 0x0) break;
    bVar1 = pbVar5[0x1];
    in_EAX = uVar4 | bVar1;
    pbVar5 = pbVar5 + 0x2;
    pbVar6[0x1] = bVar1;
    pbVar6 = pbVar6 + 0x2;
} while (bVar1 != 0x0);
    if (param_2 != 0x0) {
        local_1c = 0x0;
        in_EAX = FUN_004a0430(local_1e4,0x0,0x28);
        for (local_18 = 0x1; local_18 < 0x72; local_18 = local_18 + 0x1) {
            iVar2 = FUN_0046295d((&DAT_0058aca8 + local_18 * 0xda),0x0);
            if (((iVar2 == param_2) || (iVar2 = FUN_0046295d((&DAT_0058aca8 + local_18 * 0xda),0x1), iVar2 == param_2))
                || (iVar2 = FUN_0046295d((&DAT_0058aca8 + local_18 * 0xda),0x2), iVar2 == param_2)) {
                local_1e4[local_1c] = local_18;
                local_1c = local_1c + 0x1;
            }
            in_EAX = local_18;
        }
        if (local_1e4[0] != 0x0) {
            pcVar3 = FUN_00499050(DAT_0059679c,0x740a);
            FUN_0049c2e0(param_1,pcVar3);
            local_14 = local_1c;
            uVar4 = local_1c;
            for (local_18 = 0x0; (local_1c != 0x0 && (uVar4 = local_18 * 0x4, local_1e4[local_18] != 0x0));
                local_18 = local_18 + 0x1) {
                local_1c = local_1c - 0x1;
                if ((local_1c == 0x0) && (0x1 < local_14)) {
                    FUN_00499050(DAT_0059679c,0x7135);
                    FUN_0049c2e0(param_1,s__s__s__s_004c1c71);
                }
                else {
                    FUN_0049c2e0(param_1,s__s__s_004c1c7a);
                }
                if (0x1 < local_1c) {
                    pbVar5 = &DAT_004c1c80;
                    iVar2 = -0x1;
                    pbVar6 = param_1;
                    loop {
                        pbVar7 = pbVar6;
                        if (iVar2 == 0x0) break;
                        iVar2 = iVar2 + -0x1;
                        pbVar7 = pbVar6 + 0x1;
                        bVar1 = *pbVar6;
                        pbVar6 = pbVar7;
                    } while (bVar1 != 0x0);
                    pbVar7 = pbVar7 + -0x1;
                    loop {
                        bVar1 = *pbVar5;
                        *pbVar7 = bVar1;
                        if (bVar1 == 0x0) break;
                        bVar1 = pbVar5[0x1];
                        pbVar5 = pbVar5 + 0x2;
                        pbVar7[0x1] = bVar1;
                        pbVar7 = pbVar7 + 0x2;
                    } while (bVar1 != 0x0);
                }
                uVar4 = local_18;
            }
            pbVar6 = &DAT_004c1c82;
            iVar2 = -0x1;
            in_EAX = uVar4 & 0xffffff00;
            loop {
                pbVar5 = param_1;
                if (iVar2 == 0x0) break;
                iVar2 = iVar2 + -0x1;
                pbVar5 = param_1 + 0x1;
                bVar1 = *param_1;
                param_1 = pbVar5;
            } while (bVar1 != 0x0);
            pbVar5 = pbVar5 + -0x1;
            loop {
                bVar1 = *pbVar6;
                *pbVar5 = bVar1;
                if (bVar1 == 0x0) {
                    return in_EAX & 0xffffff00 | bVar1;
                }
                bVar1 = pbVar6[0x1];
                in_EAX = in_EAX & 0xffffff00 | bVar1;
                pbVar6 = pbVar6 + 0x2;
                pbVar5[0x1] = bVar1;
                pbVar5 = pbVar5 + 0x2;
            } while (bVar1 != 0x0);
        }
    }
    return in_EAX;
}



fn FUN_0044df49(Pparam_1: u8,param_2: i32) -> i32

{
let cVar1: u8;
let mut iVar2: i32;
let mut pcVar3: String;
let mut iVar4: i32;
PCHAR pCVar5;
let mut pcVar6: String;
let mut local_1c: i32;
let mut local_18: i32;

local_1c = 0x0;
for (local_18 = 0x0; iVar2 = local_1c, local_18 < 0x3; local_18 = local_18 + 0x1) {
iVar2 = FUN_0046295d((&DAT_0058aca8 + param_2 * 0xda),local_18);
if ((iVar2 < 0x320) && (iVar2 = FUN_0046295d((&DAT_0058aca8 + param_2 * 0xda),local_18), iVar2 != 0x0)) {
local_1c = local_1c + 0x1;
}
}
if (local_1c == 0x0) {
pcVar3 = FUN_00499050(DAT_0059679c,0x740b);
iVar4 = FUN_0049c2e0(param_1,pcVar3);
}
else {
pcVar3 = FUN_00499050(DAT_0059679c,0x740c);
FUN_0049c2e0(param_1,pcVar3);
local_18 = 0x0;
iVar4 = local_1c;
while ((local_1c != 0x0 && (iVar4 = FUN_0046295d((&DAT_0058aca8 + param_2 * 0xda),local_18), iVar4 != 0x0))) {
local_1c = local_1c + -0x1;
if ((local_1c == 0x0) && (0x1 < iVar2)) {
FUN_0046295d((&DAT_0058aca8 + param_2 * 0xda),local_18);
FUN_00499050(DAT_0059679c,0x7135);
FUN_0049c2e0(param_1,s__s__s__s_004c1c84);
}
else {
FUN_0046295d((&DAT_0058aca8 + param_2 * 0xda),local_18);
FUN_0049c2e0(param_1,s__s__s_004c1c8d);
}
if (0x1 < local_1c) {
pcVar3 = &DAT_004c1c93;
iVar4 = -0x1;
pCVar5 = param_1;
loop {
pcVar6 = pCVar5;
if (iVar4 == 0x0) break;
iVar4 = iVar4 + -0x1;
pcVar6 = pCVar5 + 0x1;
cVar1 = *pCVar5;
pCVar5 = pcVar6;
} while (cVar1 != '\0');
pcVar6 = pcVar6 + -0x1;
loop {
cVar1 = *pcVar3;
*pcVar6 = cVar1;
if (cVar1 == '\0') break;
cVar1 = pcVar3[0x1];
pcVar3 = pcVar3 + 0x2;
pcVar6[0x1] = cVar1;
pcVar6 = pcVar6 + 0x2;
} while (cVar1 != '\0');
}
iVar4 = local_18;
local_18 = local_18 + 0x1;
}
}
return iVar4;
}



fn FUN_0044e131() -> u32

{
    let mut iVar1: i32;
    let mut pcVar2: String;
    let mut local_14: i32;

    local_14 = 0x1;
    while( true ) {
        if (0x71 < local_14) {
            if (((&DAT_00569c30)[DAT_004c9754 * 0x1e22] & 0x8) == 0x0) {
                pcVar2 = FUN_00499050(DAT_0059679c,0x740d);
                FUN_0049d2e0(0x0,0x1,pcVar2);
                (&DAT_00569c30)[DAT_004c9754 * 0x1e22] = (&DAT_00569c30)[DAT_004c9754 * 0x1e22] | 0x8;
            }
            return 0x1;
        }
        iVar1 = FUN_0046295d((&DAT_0058aca8 + local_14 * 0xda),0x0);
        if ((iVar1 != 0x320) && (((&DAT_00569c30)[local_14 * 0x9 + DAT_004c9754 * 0x1e22] & 0x1) == 0x0)) break;
        local_14 = local_14 + 0x1;
    }
    (&DAT_00569c30)[DAT_004c9754 * 0x1e22] = (&DAT_00569c30)[DAT_004c9754 * 0x1e22] & 0xf7;
    return 0x0;
}



fn FUN_0044e201(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: i32) -> u32

{
    let mut uVar1: u32;
    let mut iVar2: i32;
    let mut local_28: i32;
    let mut local_24: u32;
    let mut local_20: u32;

    for (local_28 = 0x0; local_28 < 0x6; local_28 = local_28 + 0x1) {
        local_24 = FUN_0043a8a2(param_2 - (&DAT_004bea60)[local_28]);
        local_20 = FUN_0043a8d5(local_24,param_3 - (&DAT_004bea7c)[local_28]);
        iVar2 = FUN_0044ace5(param_1,local_24,local_20,0x1);
        if ((iVar2 == DAT_004c9754) &&
            (iVar2 = FUN_00485ea2(param_1,local_24,local_20,0x1), iVar2 + param_5 / 0x3e7 + 0x1 < 0x15))^ // goto LAB_0044e382;
    }
    local_28 = 0x0;
    while( true ) {
        if (0x5 < local_28) {
            return 0x0;
        }
        local_24 = FUN_0043a8a2(param_2 + (&DAT_004bea60)[local_28]);
        local_20 = FUN_0043a8d5(local_24,param_3 + (&DAT_004bea7c)[local_28]);
        uVar1 = *(*(&DAT_004d7d50 + local_24 * 0x4 + param_1 * 0x3890) + local_20 * 0x4);
        if ((((uVar1 & 0xf) != 0x0) && ((uVar1 & 0xf) != 0xa)) &&
            (iVar2 = FUN_0044ace5(param_1,local_24,local_20,0x1), iVar2 == -0x1)) break;
        local_28 = local_28 + 0x1;
    }
    LAB_0044e382:
    while (param_5 != 0x0) {
        if (param_5 < 0x3e8) {
            FUN_00465f06(param_1,local_24,local_20,param_4,param_5,DAT_004c9754,0x0);
            FUN_00465dae(param_1,param_2,param_3,param_4,param_5,0x5);
            param_5 = 0x0;
        }
        else {
            param_5 = param_5 + -0x3e7;
            FUN_00465f06(param_1,local_24,local_20,param_4,0x3e7,DAT_004c9754,0x0);
            FUN_00465dae(param_1,param_2,param_3,param_4,0x3e7,0x5);
        }
    }
    return 0x1;
}



fn FUN_0044e442() -> i32

{
return (*(&DAT_00569b7b + DAT_004c9754 * 0x1e22) >> 0x18) +
(*(&DAT_00569ba5 + DAT_004c9754 * 0x1e22) >> 0x18) +
(*(&DAT_00569bae + DAT_004c9754 * 0x1e22) >> 0x18);
}



fn FUN_0044e499() -> i32

{
return (*(&DAT_00569b7c + DAT_004c9754 * 0x1e22) >> 0x18) +
(*(&DAT_00569ba6 + DAT_004c9754 * 0x1e22) >> 0x18) +
(*(&DAT_00569baf + DAT_004c9754 * 0x1e22) >> 0x18);
}



fn FUN_0044e4f0(param_1: *mut *mut u32)

{
    let mut puVar1: *mut u8;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut local_14: i32;

    DAT_0059aa88 = 0x0;
    for (local_14 = 0x0; local_14 < 0xd; local_14 = local_14 + 0x1) {
        puVar1 = FUN_0049bf80(param_1,local_14 + 0x12d,0x501,0x0,0x0);
        iVar3 = DAT_004c9754 * 0x1e22;
        iVar2 = FUN_0044e442();
        DAT_0059aa88 = DAT_0059aa88 +
            (*(&DAT_00569bcc + local_14 * 0x8 + DAT_004c9754 * 0x1e22) +
                (*(&DAT_00569bcc + iVar3 + local_14 * 0x8) * iVar2) / 0x64) * puVar1;
    }
    return;
}



fn FUN_0044e5a4()

{
    let local_20: u8 [0x10];

    FUN_0049c2e0(local_20,&DAT_004c1c95);
    FUN_00497567(0x258,0x198,local_20,0x32,0xcaccce,0xe0e0e,0xcaccce,LPCSTR_005b9218,0x14);
    FUN_0049c2e0(local_20,&DAT_004c1c98);
    FUN_00497567(0x258,0x1ac,local_20,0x32,0xcaccce,0xe0e0e,0xcaccce,LPCSTR_005b9218,0x14);
    FUN_0049c2e0(local_20,&DAT_004c1c9b);
    FUN_00497567(0x258,0x1c0,local_20,0x32,0xcaccce,0xe0e0e,0xcaccce,LPCSTR_005b9218,0x14);
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0044e6a8()

{
    let mut iVar1: i32;
    let mut pcVar2: String;
    let mut in_stack_fffffe78: u32;
    let local_164: *mut u32;
    let mut local_154: *mut u32 [0x11];
    let ppuStack271: *mut *mut u8;;
    let mut local_63: String;;
    i32 aiStack92 [0xd];
    let mut local_28: u32;
    let mut local_24: i32;
    let mut local_20: *mut u8;
    let mut local_1c: i32;
    let mut local_18: u32;

    local_20 = &DAT_004d55a8;
    local_28 = 0x5;
    local_1c = DAT_004c9754;
    local_18 = (byte)(&DAT_004d55ee)[DAT_004c9754];
    if (local_18 == 0x2) {
        pcVar2 = FUN_00499050(DAT_0059679c,0x73e7);
        FUN_0049d2e0(0x0,0x1,pcVar2);
    }
    else {
        if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x40) == 0x0) {
            DAT_00599d3c = 0x1;
            DAT_0059aa88 = 0x0;
            DAT_0059aa90 = 0xffffffff;
            _DAT_0059aa8c = 0xffffffff;
            FUN_004990e0(local_154,0x0,s_diplo_res_004c1ca9,s_BuyLeagDlg_004c1c9e);
            for (local_24 = 0x0; local_24 < 0xd; local_24 = local_24 + 0x1) {
                local_164 = FUN_004a2831(0x69);
                if (local_164 != 0x0) {
                    local_164 = FUN_004a7c00(local_164,local_154,local_24 + 0x12d,0x15e,local_24 * 0x1d + 0x1b,0x64,0x10,0x1,
                                             0x0,0xe0e0e,0x0,0x1,in_stack_fffffe78);
                }
                aiStack92[local_24] = local_164;
                FUN_0049bf40(local_154,aiStack92[local_24]);
            }
            FUN_0049bb50(local_154,FUN_0044ebe0);
            DAT_00599d3c = 0x0;
            FUN_004864f7();
            for (local_24 = 0x0; local_24 < 0xd; local_24 = local_24 + 0x1) {
                if ((aiStack92[local_24] != 0x0) && (iVar1 = aiStack92[local_24], iVar1 != 0x0)) {
                    ((*(iVar1 + 0x45) + 0x8))(iVar1,0x2);
                }
            }
            FUN_00486ba4(DAT_004c9754);
            ppuStack271 = &PTR_FUN_004c3d34;
            if (local_63 != 0x0) {
                FUN_00499b30(local_154,local_63);
            }
            FUN_0049a1c0(local_154,0x1);
        }
        else {
            pcVar2 = FUN_00499050(DAT_0059679c,0x7381);
            FUN_0049d2e0(0x0,0x1,pcVar2);
        }
    }
    return;
}



fn FUN_0044e9a9(param_1: *mut *mut u32,param_2: i32)

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut local_14: i32;

    iVar1 = *(&DAT_00569abd + DAT_004c9754 * 0x1e22) - DAT_0059aa88;
    iVar3 = DAT_004c9754 * 0x1e22;
    iVar2 = FUN_0044e442();
    local_14 = *(&DAT_0059aa54 + param_2 * 0x4) +
        iVar1 / (*(&DAT_00569bcc + param_2 * 0x8 + DAT_004c9754 * 0x1e22) +
            (*(&DAT_00569bcc + iVar3 + param_2 * 0x8) * iVar2) / 0x64);
    if (local_14 == 0x0) {
        FUN_0049bf80(param_1,param_2 + 0x12d,0x410,0x0,0x0);
    }
    else {
        if (*(&DAT_0059aa20 + param_2 * 0x4) < local_14) {
            local_14 = *(&DAT_0059aa20 + param_2 * 0x4);
        }
        FUN_0049bf80(param_1,param_2 + 0x12d,0x40f,0x0,0x0);
        FUN_0049bf80(param_1,param_2 + 0x12d,0x503,local_14 + 0x1,0x5);
        FUN_0049bf80(param_1,param_2 + 0x12d,0x502,*(&DAT_0059aa54 + param_2 * 0x4),0x0);
    }
    return;
}



fn FUN_0044eaf1(param_1: i32)

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let local_24: u8 [0x10];
    let mut local_14: i32;

    FUN_0049c2e0(local_24,&DAT_004c1cb3);
    FUN_00497567(0x20d,param_1 * 0x1d + 0x1d,local_24,0x32,0xcaccce,0xe0e0e,0xcaccce,LPCSTR_005b9218,
                 0x14);
    iVar1 = DAT_004c9754 * 0x1e22;
    iVar2 = FUN_0044e442();
    local_14 = (*(&DAT_00569bcc + param_1 * 0x8 + iVar1) * iVar2) / 0x64;
    FUN_0049c2e0(local_24,&DAT_004c1cb7);
    FUN_00497567(0x258,param_1 * 0x1d + 0x1d,local_24,0x32,0xcaccce,0xe0e0e,0xcaccce,LPCSTR_005b9218,
                 0x14);
    return;
}



fn FUN_0044ebe0(param_1: *mut *mut u32,param_2: u32,param_3: u32,param_4: u32) -> u32

{
    let mut pcVar1: String;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let local_d8: u8 [0x80];
    let mut local_58: *mut u8;
    let mut local_54: i32;
    let mut local_50: i32;
    let mut local_4c: u32;
    let local_48: u8 [0x10];
    let mut local_38: i32;
    let mut local_34: i32;
    let mut local_30: i32;
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_14: u32;

    local_14 = param_2;
    if (param_2 < 0x406) {
        if (0x400 < param_2) {
            if (param_2 < 0x402) {
                for (local_1c = 0x0; local_1c < 0xd; local_1c = local_1c + 0x1) {
                    iVar2 = FUN_0045294f(&DAT_0059a1c8,local_1c,0x0);
                    *(&DAT_0059aa20 + local_1c * 0x4) = iVar2;
                    *(&DAT_0059aa54 + local_1c * 0x4) = 0x0;
                    if (*(&DAT_0059aa20 + local_1c * 0x4) < 0x1) {
                        FUN_0049bf80(param_1,local_1c + 0x12d,0x410,0x0,0x0);
                    }
                    else {
                        FUN_0044e9a9(param_1,local_1c);
                    }
                }
            }
            else {
                if (param_2 == 0x405) {
                    FUN_004953d7();
                    for (local_28 = 0x0; local_28 < 0xd; local_28 = local_28 + 0x1) {
                        FUN_00496ee6(&DAT_00596a58 + local_28 * 0x3da,0x14,local_28 * 0x1d + 0x19,0x22,0x1d);
                        FUN_00497567(0x40,local_28 * 0x1d + 0x1e,local_28 * 0xa8 + 0x4d6a78,0x64,0xcaccce,-0x1,0xcaccce,
                                     LPCSTR_005b9218,0x10);
                        iVar2 = DAT_004c9754 * 0x1e22;
                        iVar4 = local_28 * 0x8;
                        iVar3 = FUN_0044e499();
                        local_38 = (*(&DAT_00569bc8 + iVar4 + iVar2) * iVar3) / 0x64;
                        local_34 = *(&DAT_00569bc8 + local_28 * 0x8 + DAT_004c9754 * 0x1e22) + local_38;
                        FUN_0049c2e0(local_48,&DAT_004c1cbb);
                        FUN_00497567(0xe1,local_28 * 0x1d + 0x1e,local_48,0x32,0xcaccce,-0x1,0xcaccce,
                                     LPCSTR_005b9218,0x14);
                        FUN_0049c2e0(local_48,&DAT_004c1cbe);
                        FUN_00497567(0x113,local_28 * 0x1d + 0x1e,local_48,0x32,0xcaccce,-0x1,0xcaccce,
                                     LPCSTR_005b9218,0x14);
                        iVar4 = DAT_004c9754 * 0x1e22;
                        iVar2 = local_28 * 0x8;
                        iVar3 = FUN_0044e442();
                        local_30 = (*(&DAT_00569bcc + iVar4 + iVar2) * iVar3) / 0x64;
                        local_2c = *(&DAT_00569bcc + DAT_004c9754 * 0x1e22 + local_28 * 0x8) + local_30;
                        FUN_0049c2e0(local_48,&DAT_004c1cc1);
                        FUN_00497567(0x145,local_28 * 0x1d + 0x1e,local_48,0x32,0xcaccce,-0x1,0xcaccce,
                                     LPCSTR_005b9218,0x14);
                        FUN_004968e7(0x1db,local_28 * 0x1d + 0x1a,0x32,0x12,0xe);
                        FUN_0049e640(0x1db,local_28 * 0x1d + 0x1a,0x32,0x12,0xce,0xca,0xcc,0x1);
                        FUN_004968e7(0x226,local_28 * 0x1d + 0x1a,0x32,0x12,0xe);
                        FUN_0049e640(0x226,local_28 * 0x1d + 0x1a,0x32,0x12,0xce,0xca,0xcc,0x1);
                        FUN_0049bf80(param_1,local_28 + 0x12d,0x501,0x0,0x0);
                        FUN_0044eaf1(local_28);
                    }
                    for (local_28 = 0x0; local_28 < 0x3; local_28 = local_28 + 0x1) {
                        FUN_004968e7(0x226,local_28 * 0x14 + 0x195,0x32,0x12,0xe);
                        FUN_0049e640(0x226,local_28 * 0x14 + 0x195,0x32,0x12,0xce,0xca,0xcc,0x1);
                    }
                    FUN_0044e4f0(param_1);
                    FUN_0044e5a4();
                    FUN_0049536f();
                }
            }
        }
    }
    else {
        if (param_2 < 0x407) {
            if ((param_3 < 0x12d) || (0x139 < param_3)) {
                return 0x1;
            }
            FUN_004953d7();
            FUN_0044e4f0(param_1);
            local_24 = param_3 - 0x12d;
            *(&DAT_0059aa54 + local_24 * 0x4) = param_4;
            for (local_20 = 0x0; local_20 < 0xd; local_20 = local_20 + 0x1) {
                if (local_20 != local_24) {
                    if (*(&DAT_0059aa20 + local_20 * 0x4) < 0x1) {
                        FUN_0049bf80(param_1,local_20 + 0x12d,0x410,0x0,0x0);
                    }
                    else {
                        FUN_0044e9a9(param_1,local_20);
                    }
                }
            }
            FUN_0044eaf1(local_24);
            FUN_0044e5a4();
            FUN_0049536f();
        }
        else {
            if (param_2 < 0x408) {
                local_4c = param_3;
                if (0x63 < param_3) {
                    if (param_3 < 0x65) {
                        FUN_0044e4f0(param_1);
                        if (DAT_0059aa88 != 0x0) {
                            if (*(&DAT_00569abd + DAT_004c9754 * 0x1e22) - DAT_0059aa88 < 0x0) {
                                pcVar1 = FUN_00499050(DAT_0059679c,0x715f);
                                FUN_0049d2e0(param_1,0x1,pcVar1);
                            }
                            else {
                                for (local_50 = 0x0; local_50 < 0xd; local_50 = local_50 + 0x1) {
                                    local_58 = FUN_0049bf80(param_1,local_50 + 0x12d,0x501,0x0,0x0);
                                    if (0x0 < local_58) {
                                        iVar2 = FUN_0044e201(DAT_0059a9da._2_2_,DAT_0059a9de,DAT_0059a9de._2_2_,
                                                             local_50,local_58);
                                        if (iVar2 == 0x0) {
                                            pcVar1 = FUN_00499050(DAT_0059679c,0x7160);
                                            FUN_0049c2e0(local_d8,pcVar1);
                                            FUN_0049d2e0(param_1,0x1,local_d8);
                                        }
                                        else {
                                            iVar4 = DAT_004c9754 * 0x1e22;
                                            iVar2 = local_50 * 0x8;
                                            iVar3 = FUN_0044e442();
                                            local_54 = (*(&DAT_00569bcc + iVar4 + iVar2) * iVar3) / 0x64;
                                            *(&DAT_00569abd + DAT_004c9754 * 0x1e22) =
                                                *(&DAT_00569abd + DAT_004c9754 * 0x1e22) -
                                                    (*(&DAT_00569bcc + DAT_004c9754 * 0x1e22 + local_50 * 0x8) + local_54) * local_58
                                            ;
                                            FUN_004626e0();
                                        }
                                    }
                                }
                                FUN_0049c140(param_1,0x1);
                            }
                        }
                    }
                    else {
                        if (param_3 == 0x65) {
                            FUN_0049c140(param_1,0x0);
                        }
                    }
                }
            }
            else {
                if (param_2 == 0x411) {
                    return 0x1;
                }
            }
        }
    }
    return 0x0;
}



fn FUN_0044f2e6(param_1: *mut *mut u32)

{
    let mut puVar1: *mut u8;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut local_14: i32;

    DAT_0059aa88 = 0x0;
    for (local_14 = 0x0; local_14 < 0xd; local_14 = local_14 + 0x1) {
        puVar1 = FUN_0049bf80(param_1,local_14 + 0x12d,0x501,0x0,0x0);
        if (puVar1 != 0x0) {
            iVar2 = DAT_004c9754 * 0x1e22;
            iVar3 = FUN_0044e499();
            DAT_0059aa88 = DAT_0059aa88 +
                (*(&DAT_00569bc8 + DAT_004c9754 * 0x1e22 + local_14 * 0x8) +
                    (*(&DAT_00569bc8 + local_14 * 0x8 + iVar2) * iVar3) / 0x64) *
                    *(&DAT_0059aa20 + local_14 * 0x4);
        }
    }
    return;
}



fn FUN_0044f3a1()

{
    let local_20: u8 [0x10];

    FUN_0049c2e0(local_20,&DAT_004c1cc4);
    FUN_00497567(0x19a,0x197,local_20,0x3c,0xcaccce,0xe0e0e,0xcaccce,LPCSTR_005b9218,0x14);
    FUN_0049c2e0(local_20,&DAT_004c1cc7);
    FUN_00497567(0x19a,0x1ab,local_20,0x3c,0xcaccce,0xe0e0e,0xcaccce,LPCSTR_005b9218,0x14);
    FUN_0049c2e0(local_20,&DAT_004c1cca);
    FUN_00497567(0x19a,0x1bf,local_20,0x3c,0xcaccce,0xe0e0e,0xcaccce,LPCSTR_005b9218,0x14);
    return;
}



fn FUN_0044f4a5()

{
    let mut iVar1: i32;
    let mut pcVar2: String;
    let local_1a4: *mut u32;
    let mut local_154: *mut u32 [0x11];
    let ppuStack271: *mut *mut u8;;
    let mut local_63: String;;
    i32 aiStack92 [0xd];
    let mut local_28: u32;
    let mut local_24: i32;
    let mut local_20: *mut u8;
    let mut local_1c: i32;
    let mut local_18: u32;

    local_20 = &DAT_004d55a8;
    local_28 = 0x5;
    local_1c = DAT_004c9754;
    local_18 = (byte)(&DAT_004d55ee)[DAT_004c9754];
    if (local_18 == 0x2) {
        pcVar2 = FUN_00499050(DAT_0059679c,0x73e7);
        FUN_0049d2e0(0x0,0x1,pcVar2);
    }
    else {
        if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x40) == 0x0) {
            DAT_00599d3c = 0x1;
            FUN_004990e0(local_154,0x0,s_diplo_res_004c1cd9,s_SellLeagDlg_004c1ccd);
            for (local_24 = 0x0; local_24 < 0xd; local_24 = local_24 + 0x1) {
                local_1a4 = FUN_004a2831(0x5d);
                if (local_1a4 != 0x0) {
                    local_1a4 = FUN_0049a030(local_1a4,local_154,local_24 + 0x12d,0x1b3,local_24 * 0x1d + 0x1b,0xc,0x10,0x5,
                                             0xcaccce,0xe0e0e);
                    $1: &mut String(local_1a4 + 0x45) = &PTR_FUN_004c3d94;
                    (local_1a4 + 0x51) = &DAT_004c1ce5;
                    (local_1a4 + 0x55) = &DAT_004c1ce3;
                    *(local_1a4 + 0x4d) = 0x0;
                    *(local_1a4 + 0x49) = 0x2;
                }
                aiStack92[local_24] = local_1a4;
                FUN_0049bf40(local_154,aiStack92[local_24]);
            }
            FUN_0049bb50(local_154,FUN_0044f83f);
            DAT_00599d3c = 0x0;
            FUN_004864f7();
            for (local_24 = 0x0; local_24 < 0xd; local_24 = local_24 + 0x1) {
                if ((aiStack92[local_24] != 0x0) && (iVar1 = aiStack92[local_24], iVar1 != 0x0)) {
                    ((*(iVar1 + 0x45) + 0x8))(iVar1,0x2);
                }
            }
            FUN_00486ba4(DAT_004c9754);
            ppuStack271 = &PTR_FUN_004c3d34;
            if (local_63 != 0x0) {
                FUN_00499b30(local_154,local_63);
            }
            FUN_0049a1c0(local_154,0x1);
        }
        else {
            pcVar2 = FUN_00499050(DAT_0059679c,0x7381);
            FUN_0049d2e0(0x0,0x1,pcVar2);
        }
    }
    return;
}



fn FUN_0044f83f(param_1: *mut *mut u32,param_2: u32,param_3: u32) -> u32

{
    let mut iVar1: i32;
    let mut puVar2: *mut u8;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut local_48: i32;
    let local_40: u8 [0x10];
    let mut local_30: i32;
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_14: u32;

    local_14 = param_2;
    if (param_2 < 0x405) {
        if (param_2 == 0x401) {
            for (local_1c = 0x0; local_1c < 0xd; local_1c = local_1c + 0x1) {
                iVar3 = FUN_0045294f(&DAT_00595740,local_1c,0x1);
                *(&DAT_0059aa20 + local_1c * 0x4) = iVar3;
                if (*(&DAT_0059aa20 + local_1c * 0x4) == 0x0) {
                    FUN_0049bf80(param_1,local_1c + 0x12d,0x414,0x0,0x0);
                    FUN_0049bf80(param_1,local_1c + 0x12d,0x410,0x0,0x0);
                }
            }
        }
    }
    else {
        if (param_2 < 0x406) {
            FUN_004953d7();
            for (local_20 = 0x0; local_20 < 0xd; local_20 = local_20 + 0x1) {
                FUN_00496ee6(&DAT_00596a58 + local_20 * 0x3da,0x14,local_20 * 0x1d + 0x19,0x22,0x1d);
                FUN_00497567(0x40,local_20 * 0x1d + 0x1e,local_20 * 0xa8 + 0x4d6a78,0x64,0xcaccce,-0x1,0xcaccce,
                             LPCSTR_005b9218,0x10);
                iVar3 = DAT_004c9754 * 0x1e22;
                iVar4 = local_20 * 0x8;
                iVar1 = FUN_0044e499();
                local_30 = (*(&DAT_00569bc8 + iVar4 + iVar3) * iVar1) / 0x64;
                local_2c = *(&DAT_00569bc8 + local_20 * 0x8 + DAT_004c9754 * 0x1e22) + local_30;
                FUN_0049c2e0(local_40,&DAT_004c1ce7);
                FUN_00497567(0xe1,local_20 * 0x1d + 0x1e,local_40,0x32,0xcaccce,-0x1,0xcaccce,LPCSTR_005b9218
                             ,0x14);
                FUN_0049c2e0(local_40,&DAT_004c1cea);
                FUN_00497567(0x113,local_20 * 0x1d + 0x1e,local_40,0x32,0xcaccce,-0x1,0xcaccce,
                             LPCSTR_005b9218,0x14);
                iVar3 = DAT_004c9754 * 0x1e22;
                iVar4 = local_20 * 0x8;
                iVar1 = FUN_0044e442();
                local_28 = (*(&DAT_00569bcc + iVar4 + iVar3) * iVar1) / 0x64;
                local_24 = *(&DAT_00569bcc + DAT_004c9754 * 0x1e22 + local_20 * 0x8) + local_28;
                FUN_0049c2e0(local_40,&DAT_004c1ced);
                FUN_00497567(0x145,local_20 * 0x1d + 0x1e,local_40,0x32,0xcaccce,-0x1,0xcaccce,
                             LPCSTR_005b9218,0x14);
                FUN_004968e7(0x15e,local_20 * 0x1d + 0x1a,0x3c,0x12,0xe);
                FUN_0049e640(0x15e,local_20 * 0x1d + 0x1a,0x3c,0x12,0xce,0xca,0xcc,0x1);
                if (0x0 < *(&DAT_0059aa20 + local_20 * 0x4)) {
                    FUN_0049c2e0(local_40,&DAT_004c1cf0);
                    FUN_00497567(0x19a,local_20 * 0x1d + 0x1e,local_40,0x3c,0xcaccce,-0x1,0xcaccce,
                                 LPCSTR_005b9218,0x14);
                }
            }
            for (local_20 = 0x0; local_20 < 0x3; local_20 = local_20 + 0x1) {
                FUN_004968e7(0x15e,local_20 * 0x14 + 0x195,0x3c,0x12,0xe);
                FUN_0049e640(0x15e,local_20 * 0x14 + 0x195,0x3c,0x12,0xce,0xca,0xcc,0x1);
            }
            FUN_0044f2e6(param_1);
            FUN_0044f3a1();
            FUN_0049536f();
        }
        else {
            if (0x406 < param_2) {
                if (param_2 < 0x408) {
                    if ((param_3 < 0x12d) || (0x139 < param_3)) {
                        if (0x63 < param_3) {
                            if (param_3 < 0x65) {
                                for (local_48 = 0x0; local_48 < 0xd; local_48 = local_48 + 0x1) {
                                    puVar2 = FUN_0049bf80(param_1,local_48 + 0x12d,0x501,0x0,0x0);
                                    if (puVar2 != 0x0) {
                                        FUN_004626e0();
                                        FUN_00465dae(DAT_00595f52._2_2_,DAT_00595f56,DAT_00595f56._2_2_,local_48,
                                                     *(&DAT_0059aa20 + local_48 * 0x4),sRam00595f5a);
                                        FUN_00465f06(DAT_0059a9da._2_2_,DAT_0059a9de,DAT_0059a9de._2_2_,local_48,
                                                     *(&DAT_0059aa20 + local_48 * 0x4),0x5,0x0);
                                    }
                                }
                                iVar3 = DAT_004c9754 * 0x1e22;
                                *(&DAT_00569abd + iVar3) = *(&DAT_00569abd + iVar3) + DAT_0059aa88;
                                if (0xf423f < *(&DAT_00569abd + iVar3)) {
                                    *(&DAT_00569abd + DAT_004c9754 * 0x1e22) = 0xf423f;
                                }
                                FUN_0049c140(param_1,0x1);
                            }
                            else {
                                if (param_3 == 0x65) {
                                    FUN_0049c140(param_1,0x0);
                                }
                            }
                        }
                    }
                    else {
                        FUN_0044f2e6(param_1);
                        FUN_0044f3a1();
                    }
                }
                else {
                    if (param_2 == 0x411) {
                        return 0x1;
                    }
                }
            }
        }
    }
    return 0x0;
}



fn FUN_0044fdf2()

{
    let mut local_11c: *mut u32 [0x11];
    let ppuStack215: *mut *mut u8;;
    let mut local_2b: String;;
    let local_24: *mut i32;;
    u32 **local_20;
    let mut local_1c: u32;

    FUN_00489246(0x5,0x0);
    DAT_004c93cc = 0x5;
    DAT_004c93d4 = 0x0;
    DAT_004c93c8 = FUN_0049c2c9(0xc350);
    FUN_0049c57b(s_bin_ptratlea_bin_004c1cf3,DAT_004c93c8,0xc350);
    local_24 = FUN_004990e0(local_11c,0x0,s_diplo_res_004c1d0f,s_LeagDipDlg_004c1d04);
    FUN_0049bb50(local_11c,FUN_0044fee1);
    FUN_0049af50(DAT_004c93c8);
    local_20 = local_11c;
    local_1c = 0x0;
    ppuStack215 = &PTR_FUN_004c3d34;
    if (local_2b != 0x0) {
        FUN_00499b30(local_11c,local_2b);
    }
    FUN_0049a1c0(local_11c,0x1);
    return;
}



fn FUN_0044fee1(param_1: *mut *mut u32,param_2: u32,param_3: u32) -> u32

{
    let mut bVar1: bool;
    let mut pcVar2: String;
    let mut uVar3: u32;
    let mut puVar4: *mut u8;
    let mut local_90: i32;
    let mut local_84: i32;
    let mut local_80: u32;
    let mut local_7c: u32;
    let mut local_78: u32;
    let mut local_74: u32;
    let mut local_70: *mut u8;
    let mut local_6c: u32;
    let mut local_68: i32;
    let mut local_64: u32;
    let mut local_60: u32;
    let mut local_5c: *mut u8;
    let mut local_58: u32;
    let mut local_54: i32;
    let mut local_50: u32;
    let mut local_4c: *mut u8;
    let mut local_48: u32;
    let mut local_44: i32;
    let mut local_40: u32;
    let mut local_3c: *mut u8;
    let mut local_38: u32;
    let mut local_34: i32;
    let mut local_30: u32;
    let local_2c: u8 [0x10];
    let mut local_1c: i32;
    let mut local_14: u32;

    local_14 = param_2;
    if (param_2 < 0x407) {
        if (0x400 < param_2) {
            if (param_2 < 0x402) {
                FUN_0049f99b(DAT_005b9bd4,param_1,0x407,0x3039,0x0);
            }
            else {
                if (param_2 == 0x405) {
                    FUN_004953d7();
                    FUN_00496ac0(DAT_004c93c8,0xa,0x10,0xfa,0xc8);
                    FUN_00497567(0x88,0xe4,&DAT_005731fa,0xf0,0xcaccce,0xe0e0e,0xcaccce,LPCSTR_005b9218,0x11);
                    FUN_0049c2e0(local_2c,&DAT_004c1d2a);
                    FUN_00497567(0x190,0x122,local_2c,0x64,0xcaccce,-0x1,0xcaccce,LPCSTR_005b9218,0x11);
                    FUN_0049c2e0(local_2c,&DAT_004c1d2d);
                    FUN_00497567(0x235,0x122,local_2c,0x28,0xcaccce,-0x1,0xcaccce,LPCSTR_005b9218,0x11);
                    for (local_1c = 0x0; local_1c < 0x5; local_1c = local_1c + 0x1) {
                        local_3c = &DAT_004d55a8;
                        local_38 = 0x5;
                        local_34 = local_1c;
                        local_30 = (byte)(&DAT_004d55ee)[local_1c];
                        if (local_30 != 0x0) {
                            local_4c = &DAT_004d55a8;
                            local_48 = 0x5;
                            local_44 = local_1c;
                            local_40 = (byte)(&DAT_004d55ee)[local_1c];
                            FUN_00496ac0((&DAT_005925d0 + (local_40 - 0x1) * 0x2d9),local_1c * 0x1c + 0x8a,0x123,0x1b,
                                         0x1b);
                        }
                        if (((&DAT_00569a98)[local_1c * 0x1e22] & 0x40) == 0x0) {
                            local_5c = &DAT_004d55a8;
                            local_58 = 0x5;
                            local_54 = local_1c;
                            local_50 = (byte)(&DAT_004d55ee)[local_1c];
                            if (local_50 == 0x2)^ // goto LAB_0045010c;
                            FUN_00496ac0(&DAT_00592b88,local_1c * 0x1c + 0x8a,0x13f,0x1b,0x1b);
                        }
                        else {
                            LAB_0045010c:
                                FUN_00496ac0(&DAT_00592e61,local_1c * 0x1c + 0x8a,0x13f,0x1b,0x1b);
                        }
                        if (*(&DAT_00569b00 + local_1c * 0x1e22) == 0x0) {
                            FUN_0049c2e0(local_2c,&DAT_004c1d32);
                        }
                        else {
                            FUN_0049c2e0(local_2c,&DAT_004c1d30);
                        }
                        FUN_00497567(local_1c * 0x1c + 0x94,0x165,local_2c,0x1b,0x636567,-0x1,0x636567,
                                     LPCSTR_005b9218,0x10);
                    }
                    FUN_00428288(DAT_004c93cc,DAT_004c9754,0x1,0x0);
                    FUN_00428288(DAT_004c93cc,DAT_004c9754,0x0,0x0);
                    FUN_0049536f();
                }
            }
        }
    }
    else {
        if (param_2 < 0x408) {
            local_60 = param_3;
            if (param_3 < 0x1a6) {
                if (param_3 < 0x136) {
                    if (param_3 < 0x64) {
                        return 0x0;
                    }
                    if (0x64 < param_3) {
                        if (param_3 != 0x135) {
                            return 0x0;
                        }
                        FUN_0042d188(DAT_004c93cc,DAT_004c9754);
                        FUN_00428288(DAT_004c93cc,DAT_004c9754,0x1,0x0);
                        FUN_00428288(DAT_004c93cc,DAT_004c9754,0x0,0x0);
                        return 0x0;
                    }
                    bVar1 = true;
                    local_90 = 0x0;
                    loop {
                        if (0x2 < local_90) {
                            LAB_0045060e:
                            if (bVar1) {
                                *(&DAT_004c977c + DAT_004c93cc * 0xf8 + DAT_004c9754 * 0xd90) = 0x1;
                            }
                            else {
                                puVar4 = FUN_0049bf80(param_1,0x136,0x501,0x0,0x0);
                                (&DAT_004c9778 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8) = puVar4;
                                *(&DAT_004c977c + DAT_004c93cc * 0xf8 + DAT_004c9754 * 0xd90) = 0x0;
                            }
                            FUN_0049c140(param_1,0x1);
                            return 0x0;
                        }
                        if (0x0 < *(&DAT_004c9780 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8 + local_90 * 0x28)) {
                            bVar1 = false;^
                            // goto LAB_0045060e;
                        }
                        if (0x0 < *(&DAT_004c97f8 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8 + local_90 * 0x28)) {
                            bVar1 = false;^
                            // goto LAB_0045060e;
                        }
                        local_90 = local_90 + 0x1;
                    } while( true );
                }
                if (param_3 < 0x137) {
                    puVar4 = FUN_0049bf80(param_1,0x136,0x501,0x0,0x0);
                    (&DAT_004c9778 + DAT_004c93cc * 0xf8 + DAT_004c9754 * 0xd90) = puVar4;
                    FUN_0042d418();
                    FUN_00428288(DAT_004c93cc,DAT_004c9754,0x1,0x0);
                    FUN_00428288(DAT_004c93cc,DAT_004c9754,0x0,0x0);
                    return 0x0;
                }
                if (param_3 < 0x154) {
                    return 0x0;
                }
                if (param_3 < 0x155) {
                    local_70 = &DAT_004d55a8;
                    local_6c = 0x5;
                    local_68 = DAT_004c9754;
                    local_64 = (byte)(&DAT_004d55ee)[DAT_004c9754];
                    if (local_64 == 0x2) {
                        pcVar2 = FUN_00499050(DAT_0059679c,0x7410);
                        FUN_0049d2e0(param_1,0x1,pcVar2);
                        return 0x0;
                    }
                    if (*(&DAT_00569b00 + DAT_004c9754 * 0x1e22) != 0x0) {
                        pcVar2 = FUN_00499050(DAT_0059679c,0x7411);
                        FUN_0049d2e0(param_1,0x1,pcVar2);
                        return 0x0;
                    }
                    local_74 = FUN_004507f8();
                    if (local_74 == 0x0) {
                        return 0x0;
                    }
                    *(&DAT_00569abd + DAT_004c9754 * 0x1e22) = *(&DAT_00569abd + DAT_004c9754 * 0x1e22) + local_74;
                    *(&DAT_00569b00 + DAT_004c9754 * 0x1e22) =
                        ((*(&DAT_00569b04 + DAT_004c9754 * 0x1e22) + 0x1) * local_74) /
                            (dflt_loan_turns_00599d7c * 0xa);
                    local_84 = 0x8a;
                    local_80 = 0x15b;
                    local_7c = 0x8b;
                    local_78 = 0x1b;
                    FUN_00498a5b(&local_84);
                    FUN_0049a770(param_1,0x405,0x1,&local_84);
                    FUN_00498ae4();
                    return 0x0;
                }
                if (param_3 != 0x1a5) {
                    return 0x0;
                }
            }
            else {
                if (0x1a6 < param_3) {
                    if (param_3 < 0x1b0) {
                        if ((0x1a7 < param_3) && (param_3 != 0x1af)) {
                            return 0x0;
                        }
                    }
                    else {
                        if ((0x1b0 < param_3) && (0x1b1 < param_3)) {
                            if (param_3 != 0x3039) {
                                return 0x0;
                            }
                            FUN_00483355(0x1d);
                            return 0x0;
                        }
                    }
                }
            }
            if ((param_3 < 0x1a5) || (0x1a7 < param_3)) {
                DAT_004c93d0 = 0x0;
                DAT_004c93d4 = param_3 - 0x1af;
            }
            else {
                DAT_004c93d0 = 0x1;
                DAT_004c93d4 = param_3 - 0x1a5;
            }
            uVar3 = FUN_00429b8c(0x1);
            if ((0x63 < uVar3) && (uVar3 < 0x65)) {
                if (DAT_004c93d0 == 0x0) {
                    if (*(&DAT_004c97f8 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8 + DAT_004c93d4 * 0x28) == 0x12c) {
                        *(&DAT_00569abd + DAT_004c9754 * 0x1e22) =
                            *(&DAT_00569abd + DAT_004c9754 * 0x1e22) +
                                *(&DAT_004c97fc + DAT_004c93cc * 0xf8 + DAT_004c9754 * 0xd90 + DAT_004c93d4 * 0x28);
                    }
                    *(&DAT_004c97f8 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8 + DAT_004c93d4 * 0x28) = 0x0;
                }
                else {
                    *(&DAT_004c9780 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8 + DAT_004c93d4 * 0x28) = 0x0;
                }
            }
            FUN_00428288(DAT_004c93cc,DAT_004c9754,DAT_004c93d0,0x0);
        }
        else {
            if (0x40b < param_2) {
                if (param_2 < 0x40d) {
                    FUN_004953d7();
                    FUN_004a08c5(s_pcx_diploleg_pcx_004c1d19,0x0,0x0,0x280,0x1e0,0x0,0x0,0x0,0x1);
                    FUN_0049536f();
                    return 0x1;
                }
                if (param_2 == 0x411) {
                    return 0x1;
                }
            }
        }
    }
    return 0x0;
}



fn FUN_004507f8() -> u32

{
    let mut local_120: *mut u32 [0x11];
    let ppuStack219: *mut *mut u8;;
    let mut local_2f: String;;
    let mut local_28: u32;
    u32 **local_24;
    let local_20: *mut i32;;
    let mut local_1c: u32;

    local_20 = FUN_004990e0(local_120,0x0,s_diplo_res_004c1d3c,s_LoanDlg_004c1d34);
    local_28 = FUN_0049bb50(local_120,FUN_004508ef);
    local_24 = local_120;
    local_1c = 0x0;
    ppuStack219 = &PTR_FUN_004c3d34;
    if (local_2f != 0x0) {
        FUN_00499b30(local_120,local_2f);
    }
    FUN_0049a1c0(local_120,0x1);
    return local_28;
}



fn FUN_004508ef(param_1: *mut *mut u32,param_2: u32,param_3: u32,param_4: i32) -> u32

{
    let mut iVar1: i32;
    let mut pcVar2: String;
    let local_38: u8 [0x10];
    let mut local_28: i32;
    let mut local_24: u32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_14: u32;

    local_14 = param_2;
    if (param_2 < 0x406) {
        if (param_2 == 0x401) {
            iVar1 = FUN_0042d295(DAT_004c9754);
            local_20 = (pct_asset_is_loan_00599d80 * iVar1) / 0x64;
            if (local_20 < min_loan_amt_00599d84) {
                local_1c = local_20 + 0x1;
            }
            else {
                local_1c = local_20;
                if (0xc350 < local_20) {
                    local_1c = 0xc351;
                }
            }
            FUN_0049bf80(param_1,0x15f,0x502,min_loan_amt_00599d84,0x0);
            FUN_0049bf80(param_1,0x15f,0x503,local_1c,0x64);
            FUN_0049bf80(param_1,0x12e,0x504,min_loan_amt_00599d84,0x0);
        }
    }
    else {
        if (param_2 < 0x407) {
            if (param_3 == 0x15f) {
                if (param_4 < min_loan_amt_00599d84) {
                    return 0x0;
                }
                FUN_0049bf80(param_1,0x12e,0x504,param_4,0x0);
            }
            return 0x1;
        }
        if (param_2 == 0x407) {
            local_24 = param_3;
            if (param_3 < 0x65) {
                if (param_3 == 0x64) {
                    LAB_00450a08:
                        FUN_0049bf80(param_1,0x12e,0x501,0x5,local_38);
                    local_28 = FUN_004a1e60(local_38);
                    if (local_28 < 0x1) {
                        pcVar2 = FUN_00499050(DAT_0059679c,0x740e);
                        FUN_0049d2e0(param_1,0x1,pcVar2);
                        return 0x0;
                    }
                    FUN_0049c140(param_1,local_28);
                    return 0x0;
                }
            }
            else {
                if (param_3 < 0x66) {
                    FUN_0049c140(param_1,0x0);
                }
                else {
                    if (param_3 == 0x12e)^ // goto LAB_00450a08;
                }
            }
        }
    }
    return 0x0;
}



fn FUN_00450b06() -> u32

{
    let mut local_120: *mut u32 [0x11];
    let ppuStack219: *mut *mut u8;;
    let mut local_2f: String;;
    let mut local_28: u32;
    u32 **local_24;
    let local_20: *mut i32;;
    let mut local_1c: u32;

    local_20 = FUN_004990e0(local_120,0x0,s_efs_res_004c1d51,s_MvToLeague_004c1d46);
    local_28 = FUN_0049bb50(local_120,FUN_00450bfd);
    local_24 = local_120;
    local_1c = 0x0;
    ppuStack219 = &PTR_FUN_004c3d34;
    if (local_2f != 0x0) {
        FUN_00499b30(local_120,local_2f);
    }
    FUN_0049a1c0(local_120,0x1);
    return local_28;
}



fn FUN_00450bfd(param_1: *mut *mut u32,param_2: u32,param_3: u32) -> u32

{
    let mut iVar1: i32;

    if (0x400 < param_2) {
        if (param_2 < 0x402) {
            if (DAT_005967bc != 0x0) {
                iVar1 = FUN_00432ea6(&DAT_005967b8);
                if (iVar1 != 0x0) {
                    FUN_0049bf80(param_1,0xc8,0x410,0x0,0x0);
                }
                iVar1 = FUN_00432f12(&DAT_005967b8);
                if (iVar1 == 0x0) {
                    FUN_0049bf80(param_1,0xc9,0x410,0x0,0x0);
                }
            }
        }
        else {
            if (param_2 == 0x407) {
                if (param_3 < 0xc8) {
                    if (param_3 == 0x65) {
                        FUN_0049c140(param_1,0x0);
                    }
                }
                else {
                    if ((param_3 < 0xc9) || (param_3 == 0xc9)) {
                        FUN_0049c140(param_1,param_3);
                    }
                }
            }
        }
    }
    return 0x0;
}



fn FUN_00450d29(param_1: i32) -> i32

{
let mut iVar1: i32;
let mut local_14: i32;

*(param_1 + 0x814) = 0xffff;
*(param_1 + 0x816) = 0xffff;
*(param_1 + 0x818) = 0xffff;
*(param_1 + 0x81a) = 0xffff;
*(param_1 + 0x81c) = 0x0;
*(param_1 + 0x820) = 0x0;
*(param_1 + 0x824) = 0xffffffff;
iVar1 = param_1;
for (local_14 = 0x0; local_14 < 0x16; local_14 = local_14 + 0x1) {
FUN_004a0430(local_14 * 0x5e + param_1,0x0,0x5e);
iVar1 = local_14;
}
return iVar1;
}



byte
FUN_00450dbf(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: i32,param_6: i32,param_7: i32,param_8: i32)

{
byte *pbVar1;
let mut local_7c: i32;
let mut local_78: i32;
let local_1c: *mut u32;
let mut local_18: i32;
let mut local_14: i32;

if (param_7 == -0x1) {
param_7 = DAT_004c9754;
}
local_14 = 0x0;
FUN_00450d29(param_1);
*(param_1 + 0x828) = *(param_1 + 0x828) | 0x1;
local_1c = (&DAT_005b8b44 + param_2 * 0x4);
loop {
if ((local_1c == 0x0) || ((local_1c + 0x8) != param_2))^ // goto LAB_004510d5;
if (((local_1c + 0x22) == param_3) && ((local_1c + 0x9) == param_4)) {
if (0x13 < local_14) {
// LAB_004510d5:
for (local_18 = 0x0; local_18 < 0x14; local_18 = local_18 + 0x1) {
if ((*(local_18 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) {
pbVar1 = (local_18 * 0x5e + param_1 + 0x4);
*pbVar1 = *pbVar1 & 0xfd;
}
}
FUN_00451169(param_1,param_5,param_8);
if ((*(param_1 + 0x828) & 0x1) == 0x0) {
(param_1 + 0x814) = param_2;
(param_1 + 0x816) = param_3;
(param_1 + 0x818) = param_4;
}
return *(param_1 + 0x828) & 0x1;
}
if (param_6 == 0x0) {
if (((*(local_1c + 0x3a) & 0x1) == 0x0) && (*(local_1c + 0x23) >> 0x18 == param_7)) {
// LAB_00450ef3:
(param_1 + 0x81a) = (local_1c + 0x26);
if ((((*(local_1c + 0x3a) & 0x40) == 0x0) &&
(((*(local_1c + 0x3a) & 0x80000000) == 0x0 &&
((*(local_1c + 0x3b) & 0x20) == 0x0)))) &&
((param_8 != 0x0 ||
((*(&DAT_004be9b0 + DAT_004c9754 * 0x4) & *(local_1c + 0x3a)) != 0x0)))) {
*(param_1 + 0x828) = *(param_1 + 0x828) & 0xfe;
(local_14 * 0x5e + param_1) = local_1c;
pbVar1 = (local_14 * 0x5e + param_1 + 0x4);
*pbVar1 = *pbVar1 | 0x1;
*(local_14 * 0x5e + param_1 + 0x52) = local_1c[0x9] >> 0x18;
*(local_14 * 0x5e + param_1 + 0x56) = *(local_1c + 0x25) >> 0x18;
local_78 = 0x1;
for (local_7c = 0x0;
local_7c <
*(*(&DAT_00582938 +
(*(local_1c + 0x25) >> 0x18) * 0x4 + (local_1c[0x9] >> 0x18) * 0x18) +
0xa5); local_7c = local_7c + 0x1) {
if (*(*(local_14 * 0x5e + param_1) + local_7c * 0x4 + 0x10) != 0x0) {
*((local_14 + local_78) * 0x5e + param_1) =
*(*(local_14 * 0x5e + param_1) + local_7c * 0x4 + 0x10);
pbVar1 = ((local_14 + local_78) * 0x5e + param_1 + 0x4);
*pbVar1 = *pbVar1 | 0x5;
*((local_14 + local_78) * 0x5e + param_1 + 0x52) =
*(*(local_7c * 0x4 + *(local_14 * 0x5e + param_1) + 0x10) + 0x24) >> 0x18;
*((local_14 + local_78) * 0x5e + param_1 + 0x56) =
*(*(*(local_14 * 0x5e + param_1) + local_7c * 0x4 + 0x10) + 0x25) >> 0x18;
local_78 = local_78 + 0x1;
}
if (0x13 < local_14 + local_78) break;
}
local_14 = local_14 + local_78;
}
}
}
else {
if ((*(local_1c + 0x3a) & 0x1) != 0x0)^ // goto LAB_00450ef3;
}
}
local_1c = *local_1c;
} while( true );
}



fn FUN_00451169(param_1: i32,param_2: i32,param_3: i32)

{
    byte *pbVar1;
    let mut local_18: i32;
    let mut local_14: i32;

    if (param_2 == 0x0) {
        for (local_18 = 0x0; local_18 < 0x14; local_18 = local_18 + 0x1) {
            if ((((local_18 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) &&
                ((param_3 != 0x0 || ((*(*(local_18 * 0x5e + param_1) + 0x3a) & 0x4) == 0x0)))) {
                pbVar1 = (local_18 * 0x5e + param_1 + 0x4);
                *pbVar1 = *pbVar1 | 0x2;
            }
        }
    }
    else {
        for (local_18 = 0x0; local_18 < 0x14; local_18 = local_18 + 0x1) {
            if (((local_18 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) {
                pbVar1 = (local_18 * 0x5e + param_1 + 0x4);
                *pbVar1 = *pbVar1 & 0xfd;
            }
        }
        for (local_14 = param_2; local_14 != 0x0; local_14 = *(local_14 + 0x8)) {
            for (local_18 = 0x0; local_18 < 0x14; local_18 = local_18 + 0x1) {
                if (((((local_18 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) &&
                    (*(local_18 * 0x5e + param_1) == local_14)) &&
                    ((param_3 != 0x0 || ((*(*(local_18 * 0x5e + param_1) + 0x3a) & 0x4) == 0x0)))) {
                    pbVar1 = (local_18 * 0x5e + param_1 + 0x4);
                    *pbVar1 = *pbVar1 | 0x2;
                    break;
                }
            }
        }
    }
    return;
}



fn FUN_004512db(param_1: i32,param_2: i32) -> u32

{
    byte *pbVar1;
    let mut iVar2: i32;
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_24: u32;

    if (((param_2 * 0x5e + param_1 + 0x4) & 0x1) == 0x0) {
        local_24 = 0x0;
    }
    else {
        if ((*(*(param_2 * 0x5e + param_1) + 0x3a) & 0x40) == 0x0) {
            if ((*(*(param_2 * 0x5e + param_1) + 0x3a) & 0x4) == 0x0) {
                pbVar1 = (param_2 * 0x5e + param_1 + 0x4);
                *pbVar1 = *pbVar1 ^ 0x2;
            }
            else {
                pbVar1 = (*(param_2 * 0x5e + param_1) + 0x3a);
                *pbVar1 = *pbVar1 & 0xfb;
                pbVar1 = (param_2 * 0x5e + param_1 + 0x4);
                *pbVar1 = *pbVar1 | 0x2;
            }
            iVar2 = *(*(&DAT_00582938 +
                (*(*(param_2 * 0x5e + param_1) + 0x25) >> 0x18) * 0x4 +
                (*(*(param_2 * 0x5e + param_1) + 0x24) >> 0x18) * 0x18) + 0xa5);
            if (iVar2 != 0x0) {
                local_28 = param_2;
                for (local_2c = 0x0; local_2c < iVar2; local_2c = local_2c + 0x1) {
                    if (*(*(param_2 * 0x5e + param_1) + local_2c * 0x4 + 0x10) != 0x0) {
                        local_28 = local_28 + 0x1;
                        pbVar1 = (*(local_28 * 0x5e + param_1) + 0x3a);
                        *pbVar1 = *pbVar1 & 0xfb;
                        if (((param_2 * 0x5e + param_1 + 0x4) & 0x2) == 0x0) {
                            pbVar1 = (local_28 * 0x5e + param_1 + 0x4);
                            *pbVar1 = *pbVar1 & 0xfd;
                        }
                        else {
                            pbVar1 = (local_28 * 0x5e + param_1 + 0x4);
                            *pbVar1 = *pbVar1 | 0x2;
                        }
                    }
                }
            }
            local_24 = 0x1;
        }
        else {
            local_24 = 0x0;
        }
    }
    return local_24;
}



fn FUN_0045144a(param_1: i32) -> i32

{
byte *pbVar1;
let mut iVar2: i32;
u32 local_1c [0x2];
let mut local_14: i32;

FUN_00431d31(local_1c);
iVar2 = FUN_00431d0a(local_1c);
for (local_14 = 0x0; local_14 < 0x14; local_14 = local_14 + 0x1) {
if ((((local_14 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) &&
(((local_14 * 0x5e + param_1 + 0x4) & 0x2) != 0x0)) {
pbVar1 = (*(local_14 * 0x5e + param_1) + 0x3a);
*pbVar1 = *pbVar1 | 0x4;
pbVar1 = (local_14 * 0x5e + param_1 + 0x4);
*pbVar1 = *pbVar1 & 0xfd;
FUN_00431efd(local_1c,*(local_14 * 0x5e + param_1));
}
iVar2 = local_14;
}
return iVar2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00451519(param_1: i32)

{
    byte *pbVar1;
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0x14; local_14 = local_14 + 0x1) {
        if (((*(local_14 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) &&
            ((*(local_14 * 0x5e + param_1 + 0x4) & 0x2) != 0x0)) {
            pbVar1 = (*(local_14 * 0x5e + param_1) + 0x3d);
            *pbVar1 = *pbVar1 | 0x80;
            _DAT_005b8be4 = 0x1;
            pbVar1 = (local_14 * 0x5e + param_1 + 0x4);
            *pbVar1 = *pbVar1 & 0xfe;
            *(local_14 * 0x5e + param_1) = 0x0;
        }
    }
    return;
}



fn FUN_0045158f(param_1: i32)

{
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0x14; local_14 = local_14 + 0x1) {
        FUN_004512db(param_1,local_14);
    }
    return;
}



fn FUN_004515ca(param_1: i32,param_2: i32) -> u32

{
    let mut local_14: i32;

    local_14 = 0x0;
    while( true ) {
        if (0x13 < local_14) {
            return 0x0;
        }
        if (((((local_14 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) &&
            ((*(local_14 * 0x5e + param_1) + 0x27) == '\x1d')) &&
            (*(*(local_14 * 0x5e + param_1) + 0x2d) >> 0x18 == param_2)) break;
        local_14 = local_14 + 0x1;
    }
    return 0x1;
}



fn FUN_00451658(param_1: i32) -> i32

{
let mut local_18: i32;
let mut local_14: i32;

local_18 = 0x0;
for (local_14 = 0x0; local_14 < 0x14; local_14 = local_14 + 0x1) {
if ((((local_14 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) &&
(*(*(&DAT_00582938 +
(*(*(local_14 * 0x5e + param_1) + 0x25) >> 0x18) * 0x4 +
(*(*(local_14 * 0x5e + param_1) + 0x24) >> 0x18) * 0x18) + 0x115) != 0x0)) {
local_18 = local_18 + 0x1;
}
}
return local_18;
}



fn FUN_0045172c(param_1: i32,param_2: i32) -> u32

{
    let mut local_14: i32;

    local_14 = 0x0;
    while( true ) {
        if (0x13 < local_14) {
            return 0x0;
        }
        if (((((local_14 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) &&
            (((local_14 * 0x5e + param_1 + 0x4) & 0x2) != 0x0)) &&
            (*(*(local_14 * 0x5e + param_1) + 0x24) >> 0x18 == param_2)) break;
        local_14 = local_14 + 0x1;
    }
    return 0x1;
}



fn FUN_004517dd(param_1: i32,param_2: i32) -> u32

{
    let mut bVar1: bool;
    let mut local_1c: u32;
    let mut local_14: i32;

    bVar1 = true;
    local_14 = 0x0;
    while( true ) {
        if (0x13 < local_14) {
            if (bVar1) {
                local_1c = 0xffffffff;
            }
            else {
                local_1c = 0x1;
            }
            return local_1c;
        }
        if ((((((local_14 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) &&
            (bVar1 = false,
             *(*(&DAT_00582938 +
                 (*(*(local_14 * 0x5e + param_1) + 0x25) >> 0x18) * 0x4 +
                 (*(*(local_14 * 0x5e + param_1) + 0x24) >> 0x18) * 0x18) + 0xad) == 0x0)) &&
            ((*(*(local_14 * 0x5e + param_1) + 0x3a) & 0x2) == 0x0)) &&
            (((param_2 != 0x1 && (param_2 != 0x4)) ||
                ((*(*(local_14 * 0x5e + param_1) + 0x28) >> 0x10 != 0x5 &&
                    (*(*(local_14 * 0x5e + param_1) + 0x28) >> 0x10 != 0x7)))))) break;
        local_14 = local_14 + 0x1;
    }
    return 0x0;
}



bool  FUN_0045193a(param_1: i32)

{
let mut bVar1: bool;
let mut local_14: i32;

bVar1 = true;
local_14 = 0x0;
while( true ) {
if (0x13 < local_14) {
return !bVar1;
}
bVar1 = false;
if ((((local_14 * 0x5e + param_1 + 0x4) & 0x8) != 0x0) &&
(((local_14 * 0x5e + param_1 + 0x4) & 0x8000) == 0x0)) break;
local_14 = local_14 + 0x1;
}
return false;
}



bool  FUN_004519ed(param_1: i32)

{
let mut bVar1: bool;
let mut local_14: i32;

bVar1 = true;
local_14 = 0x0;
while( true ) {
if (0x13 < local_14) {
return !bVar1;
}
bVar1 = false;
if ((((local_14 * 0x5e + param_1 + 0x4) & 0x8) != 0x0) &&
(((local_14 * 0x5e + param_1 + 0x4) & 0x80) == 0x0)) break;
local_14 = local_14 + 0x1;
}
return false;
}



bool  FUN_00451aa0(param_1: i32,param_2: i32)

{
let mut bVar1: bool;
let mut local_14: i32;

bVar1 = true;
local_14 = 0x0;
while( true ) {
if (0x13 < local_14) {
if (bVar1) {
*(param_1 + 0x828) = *(param_1 + 0x828) | 0x1;
}
return !bVar1;
}
if (((((local_14 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) &&
(((local_14 * 0x5e + param_1 + 0x4) & 0x8000) == 0x0)) &&
(bVar1 = false,
*(*(&DAT_00582938 +
(*(*(local_14 * 0x5e + param_1) + 0x25) >> 0x18) * 0x4 +
(*(*(local_14 * 0x5e + param_1) + 0x24) >> 0x18) * 0x18) + 0x41) != param_2))
break;
local_14 = local_14 + 0x1;
}
return false;
}



fn FUN_00451b9e(param_1: i32) -> u32

{
    let mut bVar1: bool;
    let mut local_20: u32;
    let mut local_18: u32;
    let mut local_14: i32;

    bVar1 = true;
    local_18 = 0x1;
    local_14 = 0x0;
    loop {
    if (0x13 < local_14) {
        LAB_00451c0a:
        if (bVar1) {
            *(param_1 + 0x828) = *(param_1 + 0x828) | 0x1;
            local_20 = 0x0;
        }
        else {
            local_20 = local_18;
        }
        return local_20;
    }
    if (((*(local_14 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) &&
        ((bVar1 = false, (*(local_14 * 0x5e + param_1) + 0x27) != '\x1c' ||
            ((*(local_14 * 0x5e + param_1) + 0x27) != '[')))) {
        local_18 = 0x0;^
        // goto LAB_00451c0a;
    }
    local_14 = local_14 + 0x1;
} while( true );
}



fn FUN_00451c34(param_1: i32) -> i32

{
let mut local_18: i32;
let mut local_14: i32;

local_18 = 0x0;
for (local_14 = 0x0; local_14 < 0x14; local_14 = local_14 + 0x1) {
if (((*(local_14 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) &&
((*(local_14 * 0x5e + param_1 + 0x4) & 0x2) != 0x0)) {
local_18 = local_18 + 0x1;
}
}
return local_18;
}



fn FUN_00451c91(param_1: i32) -> i32

{
let mut local_18: i32;
let mut local_14: i32;

local_18 = 0x0;
for (local_14 = 0x0; local_14 < 0x14; local_14 = local_14 + 0x1) {
if (((((local_14 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) &&
(((local_14 * 0x5e + param_1 + 0x4) & 0x2) != 0x0)) &&
((*(*(local_14 * 0x5e + param_1) + 0x3a) & 0x40) != 0x0)) {
local_18 = local_18 + 0x1;
}
}
return local_18;
}



fn FUN_00451d6d(param_1: i32,param_2: i32,param_3: i32)

{
    byte *pbVar1;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut local_14: i32;

    local_14 = 0x0;
    loop {
    if (0x13 < local_14) {
        return;
    }
    if (((((local_14 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) &&
        (((local_14 * 0x5e + param_1 + 0x4) & 0x2) != 0x0)) &&
        (*(*(&DAT_00582938 +
            (*(*(local_14 * 0x5e + param_1) + 0x25) >> 0x18) * 0x4 +
            (*(*(local_14 * 0x5e + param_1) + 0x24) >> 0x18) * 0x18) + 0xad) == 0x0)) {
        if ((param_2 == 0x1) || (param_2 == 0x4)) {
            if ((*(*(local_14 * 0x5e + param_1) + 0x28) >> 0x10 != 0x5) &&
                (*(*(local_14 * 0x5e + param_1) + 0x28) >> 0x10 != 0x7))^ // goto LAB_00451ee9;
        }
        else {
            if (((param_2 != 0x3) || (param_3 == 0x0)) ||
                (*(*(local_14 * 0x5e + param_1) + 0x28) >> 0x10 == 0x8)) {
                LAB_00451ee9:
                if ((*(*(local_14 * 0x5e + param_1) + 0x3a) & 0x40) != 0x0) {
                    iVar2 = *(*(local_14 * 0x5e + param_1) + 0x24) >> 0x18;
                    iVar3 = *(*(local_14 * 0x5e + param_1) + 0x25) >> 0x18;
                    if ((param_2 != 0x0) ||
                        (((((*(*(&DAT_00582938 + iVar3 * 0x4 + iVar2 * 0x18) + 0x95) == 0x0 &&
                            (*(*(&DAT_00582938 + iVar3 * 0x4 + iVar2 * 0x18) + 0x99) == 0x0)) &&
                            (*(*(&DAT_00582938 + iVar3 * 0x4 + iVar2 * 0x18) + 0x9d) == 0x0)) &&
                            (*(*(&DAT_00582938 + iVar3 * 0x4 + iVar2 * 0x18) + 0xa1) == 0x0)) ||
                            (iVar2 = FUN_00409fc6(param_1,local_14),
                             *(*(iVar2 * 0x5e + param_1) + 0x24) >> 0x18 != 0x10))))^ // goto LAB_00451d82;
                }
                pbVar1 = (local_14 * 0x5e + param_1 + 0x4);
                *pbVar1 = *pbVar1 | 0x8;
            }
        }
    }
    LAB_00451d82:
        local_14 = local_14 + 0x1;
} while( true );
}



fn FUN_0045209e(param_1: i32)

{
    byte *pbVar1;
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0x14; local_14 = local_14 + 0x1) {
        if ((((local_14 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) && (*(local_14 * 0x5e + param_1) != 0x0)) {
            pbVar1 = (local_14 * 0x5e + param_1 + 0x4);
            *pbVar1 = *pbVar1 & 0xf7;
            pbVar1 = (local_14 * 0x5e + param_1 + 0x4);
            *pbVar1 = *pbVar1 & 0xdf;
            pbVar1 = (local_14 * 0x5e + param_1 + 0x4);
            *pbVar1 = *pbVar1 & 0x7f;
            pbVar1 = (local_14 * 0x5e + param_1 + 0x4);
            *pbVar1 = *pbVar1 & 0xbf;
        }
    }
    return;
}



fn FUN_00452148(param_1: i32)

{
    byte *pbVar1;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut local_4c: i32;
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0x14; local_14 = local_14 + 0x1) {
        if ((((((local_14 * 0x5e + param_1 + 0x4) & 0x8) != 0x0) &&
            (((local_14 * 0x5e + param_1 + 0x4) & 0x80) == 0x0)) &&
            (((local_14 * 0x5e + param_1 + 0x4) & 0x8000) == 0x0)) &&
            ((*(local_14 * 0x5e + param_1 + 0x16) != 0x0 &&
                ((*(local_14 * 0x5e + param_1) + 0x29) <
                    *(&DAT_00569acd + (*(*(local_14 * 0x5e + param_1) + 0x23) >> 0x18) * 0x1e22))))) {
            uVar2 = FUN_004a2edc();
            local_4c = uVar2 % 0x64;
            iVar3 = *(&DAT_00569acd + (*(*(local_14 * 0x5e + param_1) + 0x23) >> 0x18) * 0x1e22) -
                (*(local_14 * 0x5e + param_1) + 0x29);
            if (local_4c <= iVar3) {
                iVar4 = FUN_0045172c(param_1,0x2d);
                if (iVar4 == 0x0) {
                    iVar4 = FUN_0045172c(param_1,0x33);
                    if (iVar4 != 0x0) {
                        local_4c = local_4c + loyalty_officer_bonus_00599d58;
                    }
                }
                else {
                    local_4c = local_4c + loyalty_noble_bonus_00599d54;
                }
                if (iVar3 < local_4c) {
                    pbVar1 = (local_14 * 0x5e + param_1 + 0x4);
                    *pbVar1 = *pbVar1 | 0x40;
                }
                else {
                    pbVar1 = (local_14 * 0x5e + param_1 + 0x4);
                    *pbVar1 = *pbVar1 | 0x80;
                    pbVar1 = (local_14 * 0x5e + param_1 + 0x4);
                    *pbVar1 = *pbVar1 & 0xbf;
                }
            }
        }
    }
    return;
}



fn FUN_00452328(param_1: i32)

{
    let puVar1: *mut u32;
    let mut local_14: i32;

    puVar1 = FUN_0049c2c9(0x484);
    if (puVar1 != 0x0) {
        for (local_14 = 0x0; local_14 < 0x14; local_14 = local_14 + 0x1) {
            if ((((local_14 * 0x5e + param_1 + 0x4) & 0x200) != 0x0) &&
                (((local_14 * 0x5e + param_1 + 0x4) & 0x8) != 0x0)) {
                if (((local_14 * 0x5e + param_1 + 0x4) & 0x100) == 0x0) {
                    LAB_004525b3:
                        FUN_004906c1(puVar1,*(&DAT_004d6058 + *(local_14 * 0x5e + param_1 + 0x52) * 0x1c),
                    (char)*(&DAT_004be9e8 + (*(param_1 + 0x818) >> 0x10) * 0x4),
                    *(local_14 * 0x5e + param_1 + 0x5a),0x22,-0x1);
                    FUN_00496ac0(puVar1,*(local_14 * 0x5e + param_1 + 0x4a),*(local_14 * 0x5e + param_1 + 0x4e),0x22
                                 ,0x22);
                }
                else {
                    if (((local_14 * 0x5e + param_1 + 0x4) & 0x8000) == 0x0) {
                        if (((local_14 * 0x5e + param_1 + 0x4) & 0x80) == 0x0)^ // goto LAB_004525b3;
                        FUN_004906c1(puVar1,*(&DAT_004d6058 + *(local_14 * 0x5e + param_1 + 0x52) * 0x1c),
                                     (char)*(&DAT_004be9e8 + (*(param_1 + 0x818) >> 0x10) * 0x4),
                                     *(local_14 * 0x5e + param_1 + 0x5a) - *(local_14 * 0x5e + param_1 + 0x16),0x22,
                                     -0x1);
                        FUN_00496ac0(puVar1,*(local_14 * 0x5e + param_1 + 0x4a),*(local_14 * 0x5e + param_1 + 0x4e),
                                     0x22,0x22);
                        FUN_00496ee6(&DAT_005943e8,*(local_14 * 0x5e + param_1 + 0x4a) + -0x3,
                                     *(local_14 * 0x5e + param_1 + 0x4e) + -0x3,0x28,0x28);
                    }
                    else {
                        FUN_004906c1(puVar1,*(&DAT_004d6058 + *(local_14 * 0x5e + param_1 + 0x52) * 0x1c),
                                     (char)*(&DAT_004be9e8 + (*(param_1 + 0x818) >> 0x10) * 0x4),0x0,0x22,-0x1)
                        ;
                        FUN_00496ac0(puVar1,*(local_14 * 0x5e + param_1 + 0x4a),*(local_14 * 0x5e + param_1 + 0x4e),
                                     0x22,0x22);
                        FUN_00496ee6(&DAT_00593da8,*(local_14 * 0x5e + param_1 + 0x4a) + -0x3,
                                     *(local_14 * 0x5e + param_1 + 0x4e) + -0x3,0x28,0x28);
                    }
                }
            }
        }
        FUN_0049af50(puVar1);
    }
    return;
}


u32 *
FUN_004a7c00(param_1: *mut u32,param_2: i32,param_3: u32,param_4: i32,param_5: i32,param_6: i32,param_7: i32,
param_8: u32,param_9: u32,param_10: u32,param_11: u32,param_12: i32,
param_13: u32)

{
let puVar1: *mut u32;
let mut bVar2: bool;
let puVar3: *mut u32;
let puVar4: *mut u32;
let mut iVar5: i32;
undefined3 extraout_var;
undefined3 extraout_var_00;
undefined3 extraout_var_01;
undefined3 extraout_var_02;
let mut pcVar6: String;
undefined3 extraout_var_03;
let mut unaff_ESI: u32;
let aCStack28: u8 [0xc];

puVar3 = FUN_0049a030(param_1,param_2,param_3,param_4,param_5,param_6,param_7,param_8,param_9,param_10);
$1: &mut String(puVar3 + 0x45) = &PTR_LAB_004c4104;
*(puVar3 + 0x59) = 0x10;
if (((*(puVar3 + 0x2d) & 0x1) == 0x0) && (DAT_005b9cf0 == 0x0)) {
iVar5 = FUN_004a7b70();
if (iVar5 == 0x0) {
unaff_ESI = *(puVar3 + 0x59) * *(puVar3 + 0x59);
FUN_0049c2e0(&stack0xffffffe0,s_UpArw16_bin_004c3607);
DAT_005b9cf0 = FUN_0049c2c9(unaff_ESI);
if (DAT_005b9cf0 == 0x0) {
pcVar6 = FUN_00499050(DAT_005b9bd8,0x7d01);
pop_err_msg_box_and_exit_004a02f5(pcVar6);
}
bVar2 = FUN_0049c57b(aCStack28,DAT_005b9cf0,unaff_ESI);
if (CONCAT31(extraout_var,bVar2) == 0x0) {
pcVar6 = FUN_00499050(DAT_005b9bd8,0x7d02);
pop_err_msg_box_and_exit_004a02f5(pcVar6);
}
FUN_0049c2e0(aCStack28,s_DnArw16_bin_004c3613);
DAT_005b9cf4 = FUN_0049c2c9(unaff_ESI);
if (DAT_005b9cf4 == 0x0) {
pcVar6 = FUN_00499050(DAT_005b9bd8,0x7d01);
pop_err_msg_box_and_exit_004a02f5(pcVar6);
}
bVar2 = FUN_0049c57b(aCStack28,DAT_005b9cf4,unaff_ESI);
if (CONCAT31(extraout_var_00,bVar2) == 0x0) {
pcVar6 = FUN_00499050(DAT_005b9bd8,0x7d02);
pop_err_msg_box_and_exit_004a02f5(pcVar6);
}^
// goto LAB_004a7c72;
}
}
if (((*(puVar3 + 0x2d) & 0x1) != 0x0) && (DAT_005b9cf8 == 0x0)) {
iVar5 = FUN_004a7b70();
if (iVar5 == 0x0) {
unaff_ESI = *(puVar3 + 0x59) * *(puVar3 + 0x59);
FUN_0049c2e0(&stack0xffffffe0,s_RtArw16_bin_004c361f);
DAT_005b9cf8 = FUN_0049c2c9(unaff_ESI);
if (DAT_005b9cf8 == 0x0) {
pcVar6 = FUN_00499050(DAT_005b9bd8,0x7d01);
pop_err_msg_box_and_exit_004a02f5(pcVar6);
}
bVar2 = FUN_0049c57b(aCStack28,DAT_005b9cf8,unaff_ESI);
if (CONCAT31(extraout_var_01,bVar2) == 0x0) {
pcVar6 = FUN_00499050(DAT_005b9bd8,0x7d02);
pop_err_msg_box_and_exit_004a02f5(pcVar6);
}
FUN_0049c2e0(aCStack28,s_LtArw16_bin_004c362b);
DAT_005b9cfc = FUN_0049c2c9(unaff_ESI);
if (DAT_005b9cfc == 0x0) {
pcVar6 = FUN_00499050(DAT_005b9bd8,0x7d01);
pop_err_msg_box_and_exit_004a02f5(pcVar6);
}
bVar2 = FUN_0049c57b(aCStack28,DAT_005b9cfc,unaff_ESI);
if (CONCAT31(extraout_var_02,bVar2) == 0x0) {
pcVar6 = FUN_00499050(DAT_005b9bd8,0x7d02);
pop_err_msg_box_and_exit_004a02f5(pcVar6);
}
}
}
// LAB_004a7c72:
if (DAT_005b9d00 == 0x0) {
FUN_0049c2e0(aCStack28,s_YnYng16_bin_004c3637);
DAT_005b9d00 = FUN_0049c2c9(unaff_ESI);
if (DAT_005b9d00 == 0x0) {
pcVar6 = FUN_00499050(DAT_005b9bd8,0x7d01);
pop_err_msg_box_and_exit_004a02f5(pcVar6);
}
bVar2 = FUN_0049c57b(aCStack28,DAT_005b9d00,unaff_ESI);
if (CONCAT31(extraout_var_03,bVar2) == 0x0) {
pcVar6 = FUN_00499050(DAT_005b9bd8,0x7d02);
pop_err_msg_box_and_exit_004a02f5(pcVar6);
}
}
*(puVar3 + 0x49) = param_12;
if (param_12 < 0x1) {
*(puVar3 + 0x49) = 0x1;
}
*(puVar3 + 0x4d) = param_13;
if ((*(puVar3 + 0x2d) & 0x1) == 0x0) {
puVar4 = FUN_004a2831(0x5d);
puVar1 = DAT_005b9cf0;
if (puVar4 != 0x0) {
puVar4 = FUN_0049a030(puVar4,puVar3,0xb,*(puVar3 + 0x1d),*(puVar3 + 0x21),
*(puVar3 + 0x59),*(puVar3 + 0x59),0x6,0x0,0x0);
$1: &mut String(puVar4 + 0x45) = &PTR_FUN_004c3d94;
*(puVar4 + 0x55) = 0x0;
*(puVar4 + 0x4d) = 0x0;
*(puVar4 + 0x49) = 0x2;
(puVar4 + 0x51) = puVar1;
}
(puVar3 + 0x61) = puVar4;
iVar5 = *(puVar3 + 0x59) * 0x4;
if (*(puVar3 + 0x29) < iVar5) {
*(puVar3 + 0x29) = iVar5;
}
if (*(puVar3 + 0x25) < *(puVar3 + 0x59)) {
*(puVar3 + 0x25) = *(puVar3 + 0x59);
}
puVar4 = FUN_004a2831(0x5d);
puVar1 = DAT_005b9cf4;
if (puVar4 != 0x0) {
puVar4 = FUN_0049a030(puVar4,puVar3,0xc,*(puVar3 + 0x1d),
(*(puVar3 + 0x21) + *(puVar3 + 0x29)) - *(puVar3 + 0x59)
,*(puVar3 + 0x59),*(puVar3 + 0x59),0x6,0x0,0x0);
$1: &mut String(puVar4 + 0x45) = &PTR_FUN_004c3d94;
*(puVar4 + 0x55) = 0x0;
*(puVar4 + 0x4d) = 0x0;
*(puVar4 + 0x49) = 0x2;
(puVar4 + 0x51) = puVar1;
}
}
else {
puVar4 = FUN_004a2831(0x5d);
puVar1 = DAT_005b9cfc;
if (puVar4 != 0x0) {
puVar4 = FUN_0049a030(puVar4,puVar3,0xb,*(puVar3 + 0x1d),*(puVar3 + 0x21),
*(puVar3 + 0x59),*(puVar3 + 0x59),0x6,0x0,0x0);
$1: &mut String(puVar4 + 0x45) = &PTR_FUN_004c3d94;
*(puVar4 + 0x55) = 0x0;
*(puVar4 + 0x4d) = 0x0;
*(puVar4 + 0x49) = 0x2;
(puVar4 + 0x51) = puVar1;
}
(puVar3 + 0x61) = puVar4;
iVar5 = *(puVar3 + 0x59) * 0x4;
if (*(puVar3 + 0x25) < iVar5) {
*(puVar3 + 0x25) = iVar5;
}
if (*(puVar3 + 0x29) < *(puVar3 + 0x59)) {
*(puVar3 + 0x29) = *(puVar3 + 0x59);
}
puVar4 = FUN_004a2831(0x5d);
puVar1 = DAT_005b9cf8;
if (puVar4 != 0x0) {
iVar5 = *(puVar3 + 0x59);
puVar4 = FUN_0049a030(puVar4,puVar3,0xc,(*(puVar3 + 0x25) + *(puVar3 + 0x1d)) - iVar5
,*(puVar3 + 0x21),iVar5,iVar5,0x6,0x0,0x0);
$1: &mut String(puVar4 + 0x45) = &PTR_FUN_004c3d94;
*(puVar4 + 0x55) = 0x0;
*(puVar4 + 0x4d) = 0x0;
*(puVar4 + 0x49) = 0x2;
(puVar4 + 0x51) = puVar1;
}
}
(puVar3 + 0x65) = puVar4;
iVar5 = *(puVar3 + 0x61);
((*(iVar5 + 0x45) + 0xc))(iVar5,iVar5,0x401,0x0);
iVar5 = *(puVar3 + 0x65);
((*(iVar5 + 0x45) + 0xc))(iVar5,iVar5,0x401,0x0);
*(puVar3 + 0x5d) = 0xffffffff;
return puVar3;
}



fn FUN_004a8d00(param_1: i32)

{
    let uVar1: u8;
    let mut uVar3: u32;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let mut iVar6: i32;
    let local_1c: u8;
    let mut uVar2: u32;

    if ((param_1 + 0x1c) != '\0') {
        if ((*(param_1 + 0x2e) & 0x20) == 0x0) {
            uVar1 = *(param_1 + 0x35);
        }
        else {
            uVar2 = *(param_1 + 0x35);
            if ((uVar2 & 0xffffff00) == 0x0) {
                uVar1 = (&DAT_005b96d0)[uVar2];
            }
            else {
                uVar2 = FUN_00499f60(uVar2);
                uVar1 = uVar2;
            }
        }
        if (*(param_1 + 0xc) == 0x0) {
            local_1c = DAT_004bf924;
            uVar3 = DAT_005b981c;
        }
        else {
            local_1c = DAT_005b981c;
            uVar3 = DAT_004bf924;
        }
        FUN_004953d7();
        if ((*(param_1 + 0x2d) & 0x1) == 0x0) {
            iVar5 = *(param_1 + 0x1d);
            if (*(param_1 + 0x55) != 0x0) {
                FUN_004968e7(iVar5 + -0x1,*(param_1 + 0x21) + 0x11,*(param_1 + 0x25) + 0x2,
                             *(param_1 + 0x55),uVar1);
            }
            iVar6 = *(param_1 + 0x21) + *(param_1 + 0x55);
            if ((*(param_1 + 0x2e) & 0x20) == 0x0) {
                FUN_00496ac0(DAT_005b9d00,iVar5,iVar6,0x10,0x10);
            }
            else {
                FUN_0049aaa0(iVar5,iVar6,0x10,0x10,DAT_005b9d00);
            }
            iVar4 = *(param_1 + 0x21) + *(param_1 + 0x29);
            if (iVar6 + 0x11 <= iVar4 + -0x12) {
                FUN_004968e7(*(param_1 + 0x1d) + -0x1,iVar6 + 0x11,*(param_1 + 0x25) + 0x2,(iVar4 + -0x22) - iVar6
                             ,uVar1);
            }
        }
        else {
            iVar6 = *(param_1 + 0x21);
            if (*(param_1 + 0x55) != 0x0) {
                FUN_004968e7(*(param_1 + 0x1d) + 0x11,iVar6 + -0x1,*(param_1 + 0x55),
                             *(param_1 + 0x29) + 0x2,uVar1);
            }
            iVar5 = *(param_1 + 0x1d) + *(param_1 + 0x55);
            if ((*(param_1 + 0x2e) & 0x20) == 0x0) {
                FUN_00496ac0(DAT_005b9d00,iVar5,iVar6,0x10,0x10);
            }
            else {
                FUN_0049aaa0(iVar5,iVar6,0x10,0x10,DAT_005b9d00);
            }
            iVar4 = *(param_1 + 0x1d) + *(param_1 + 0x25);
            if (iVar5 + 0x11 <= iVar4 + -0x12) {
                FUN_004968e7(iVar5 + 0x11,*(param_1 + 0x21) + -0x1,(iVar4 + -0x22) - iVar5,*(param_1 + 0x29) + 0x2
                             ,uVar1);
            }
        }
        FUN_0049e640(iVar5,iVar6,0x10,0x10,local_1c,(char)uVar3,(char)DAT_004bf928,0x1);
        FUN_0049536f();
    }
    return;
}



fn FUN_004a8f10(param_1: i32,param_2: i32)

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut iVar3: i32;

    iVar1 = *(param_1 + 0x51);
    iVar3 = param_2 + iVar1;
    if (*(param_1 + 0x49) <= iVar3) {
        iVar3 = *(param_1 + 0x49) + -0x1;
    }
    if (iVar3 < 0x0) {
        iVar3 = 0x0;
    }
    if (iVar3 != *(param_1 + 0x51)) {
        *(param_1 + 0x51) = iVar3;
        if (*(param_1 + 0x49) < 0x2) {
            *(param_1 + 0x55) = *(param_1 + 0x59) + 0x2;
        }
        else {
            if ((*(param_1 + 0x2d) & 0x1) == 0x0) {
                iVar2 = *(param_1 + 0x59);
                iVar3 = *(param_1 + 0x29);
            }
            else {
                iVar2 = *(param_1 + 0x59);
                iVar3 = *(param_1 + 0x25);
            }
            *(param_1 + 0x55) =
                *(param_1 + 0x59) + 0x2 +
                    (*(param_1 + 0x51) * (iVar3 + iVar2 * -0x3 + -0x4)) / (*(param_1 + 0x49) + -0x1);
        }
        FUN_004a8d00(param_1);
        if (((param_1 + 0x4) != 0x0) && (iVar1 != -0x1)) {
            FUN_0049a770((param_1 + 0x4),0x406,*(param_1 + 0x3d),*(param_1 + 0x51))
            ;
            return;
        }
    }
    return;
}



fn FUN_004a9080(param_1: u32,param_2: *mut i32) -> u32

{
    let mut iVar1: i32;
    let DVar2: u32;
    let mut uVar3: u32;

    (PTR_FUN_004bfb78)(param_2[0x4]);
    iVar1 = *(param_2[0x2] + 0xc);
    if (iVar1 != 0x1) {
        if (iVar1 != 0x0) {
            (PTR_FUN_004bfb7c)(param_2[0x4]);
            return 0xffffffff;
        }
        *(param_2[0x2] + 0xc) = 0x1;
    }
    if ((*(param_2 + 0x3) & 0x2) == 0x0) {
        FUN_004b1740(0x4);
        *(param_2 + 0x3) = *(param_2 + 0x3) | 0x20;
        (PTR_FUN_004bfb7c)(param_2[0x4]);
        return 0xffffffff;
    }
    if (*(param_2[0x2] + 0x8) == 0x0) {
        FUN_004b1790(param_2);
    }
    uVar3 = 0x400;
    if ((param_1 == 0xa) && (uVar3 = 0x600, (*(param_2 + 0x3) & 0x40) == 0x0)) {
        *(param_2 + 0xd) = *(param_2 + 0xd) | 0x10;
        *param_2 = 0xd;
        iVar1 = param_2[0x1];
        *param_2 = *param_2 + 0x1;
        param_2[0x1] = iVar1 + 0x1;
        if (iVar1 + 0x1 == param_2[0x5]) {
            DVar2 = FUN_004af2f0(param_2);
            if (DVar2 != 0x0) {
                (PTR_FUN_004bfb7c)(param_2[0x4]);
                return 0xffffffff;
            }
        }
    }
    *(param_2 + 0xd) = *(param_2 + 0xd) | 0x10;
    *param_2 = param_1;
    iVar1 = param_2[0x1];
    *param_2 = *param_2 + 0x1;
    param_2[0x1] = iVar1 + 0x1;
    if (((uVar3 & param_2[0x3]) != 0x0) || (iVar1 + 0x1 == param_2[0x5])) {
        DVar2 = FUN_004af2f0(param_2);
        if (DVar2 != 0x0) {
            (PTR_FUN_004bfb7c)(param_2[0x4]);
            return 0xffffffff;
        }
    }
    (PTR_FUN_004bfb7c)(param_2[0x4]);
    return param_1 & 0xff;
}



fn FUN_004a91d0(param_1: u32) -> u32

{
    let DVar1: u32;
    let DVar2: u32;

    if ((-0x1 < param_1) && (param_1 <= DAT_004bffe8)) {
        (PTR_FUN_004bfb78)(param_1);
        DVar1 = set_file_pointer_004af420(param_1,0x0,0x1);
        if (DVar1 == 0xffffffff) {
            (PTR_FUN_004bfb7c)(param_1);
            return 0xffffffff;
        }
        DVar2 = set_file_pointer_004af420(param_1,0x0,0x2);
        set_file_pointer_004af420(param_1,DVar1,0x0);
        (PTR_FUN_004bfb7c)(param_1);
        return DVar2;
    }
    FUN_004b1740(0x4);
    return 0xffffffff;
}



fn FUN_004a9250(byte **param_1) -> u32

{
    let mut iVar1: i32;
    let mut uVar2: u32;
    byte *pbVar3;

    (PTR_FUN_004bfb78)(param_1[0x4]);
    iVar1 = *(param_1[0x2] + 0xc);
    if (iVar1 != 0x1) {
        if (iVar1 != 0x0) {
            pbVar3 = param_1[0x4];
            uVar2 = 0xffffffff;^
            // goto LAB_004a930e;
        }
        *(param_1[0x2] + 0xc) = 0x1;
    }
    if ((*(param_1 + 0x3) & 0x1) == 0x0) {
        FUN_004b1740(0x4);
        uVar2 = 0xffffffff;
        *(param_1 + 0x3) = *(param_1 + 0x3) | 0x20;
    }
    else {
        pbVar3 = param_1[0x1];
        param_1[0x1] = pbVar3 + -0x1;
        if ((pbVar3 + -0x1) < 0x0) {
            uVar2 = FUN_004a9320(param_1);
        }
        else {
            uVar2 = **param_1;
            *param_1 = *param_1 + 0x1;
        }
    }
    if ((*(param_1 + 0x3) & 0x40) == 0x0) {
        if (uVar2 == 0xd) {
            pbVar3 = param_1[0x1];
            param_1[0x1] = pbVar3 + -0x1;
            if ((pbVar3 + -0x1) < 0x0) {
                uVar2 = FUN_004a9320(param_1);
            }
            else {
                uVar2 = **param_1;
                *param_1 = *param_1 + 0x1;
            }
        }
        if (uVar2 == 0x1a) {
            uVar2 = 0xffffffff;
            *(param_1 + 0x3) = *(param_1 + 0x3) | 0x10;
        }
    }
    pbVar3 = param_1[0x4];
    LAB_004a930e:
        (PTR_FUN_004bfb7c)(pbVar3);
    return uVar2;
}



fn FUN_004a9320(byte **param_1) -> u32

{
    byte *pbVar1;
    LPVOID pvVar2;

    pvVar2 = FUN_004a9350(param_1);
    if (pvVar2 == (LPVOID)0x0) {
    return 0xffffffff;
}
    pbVar1 = *param_1;
    param_1[0x1] = param_1[0x1] + -0x1;
    *param_1 = pbVar1 + 0x1;
    return *pbVar1;
}



LPfn FUN_004a9350(LPVOID *param_1)

{
let mut uVar1: u32;
LPVOID pvVar2;

if (*(param_1[0x2] + 0x8) == 0x0) {
FUN_004b1790(param_1);
}
if (((*(param_1 + 0xd) & 0x20) != 0x0) && ((*(param_1 + 0xd) & 0x6) != 0x0)) {
FUN_004b1acc(0x2000);
}
*(param_1 + 0x3) = *(param_1 + 0x3) & 0xfb;
*param_1 = *(LPVOID *)(param_1[0x2] + 0x8);
if (((param_1[0x3] & 0x2400) == 0x2400) && (param_1[0x4] == (LPVOID)0x0)) {
param_1[0x1] = (LPVOID)0x0;
uVar1 = FUN_004b1b20();
if (uVar1 != 0xffffffff) {
*param_1 = (char)uVar1;
param_1[0x1] = (LPVOID)0x1;^
// goto LAB_004a940f;
}
}
else {
if ((*(param_1 + 0xd) & 0x4) == 0x0) {
pvVar2 = param_1[0x5];
}
else {
pvVar2 = (LPVOID)0x1;
}
pvVar2 = (LPVOID)read_file_004b1940(param_1[0x4],*param_1,(DWORD)pvVar2);
param_1[0x1] = pvVar2;
}
if (param_1[0x1] < 0x1) {
if (param_1[0x1] == (LPVOID)0x0) {
*(param_1 + 0x3) = *(param_1 + 0x3) | 0x10;
return param_1[0x1];
}
param_1[0x1] = (LPVOID)0x0;
*(param_1 + 0x3) = *(param_1 + 0x3) | 0x20;
}
// LAB_004a940f:
return param_1[0x1];
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_004a9420(byte *param_1,param_2: *mut u32) -> u32

{
    let bVar1: u8;
    let mut bVar2: bool;
    let mut bVar3: bool;
    let mut bVar4: bool;
    let mut bVar5: bool;
    let mut uVar6: u32;

    bVar2 = true;
    bVar4 = false;
    bVar5 = false;
    bVar3 = false;
    if (param_2 != 0x0) {
        if (_DAT_004c0040 == 0x1) {
            *param_2 = 0x1;
        }
        else {
            *param_2 = 0x0;
        }
    }
    bVar1 = *param_1;
    if (bVar1 < 0x72) {
        if (bVar1 != 0x61) {
            LAB_004a9472:
                FUN_004b1740(0x9);
            return 0x0;
        }
        uVar6 = 0x82;
    }
    else {
        if (bVar1 < 0x73) {
            uVar6 = 0x1;
        }
        else {
            if (bVar1 != 0x77)^ // goto LAB_004a9472;
            uVar6 = 0x2;
        }
    }
    bVar1 = param_1[0x1];
    while ((bVar1 != 0x0 && (bVar2))) {
        bVar1 = param_1[0x1];
        if (bVar1 < 0x63) {
            if (0x2a < bVar1) {
                if (bVar1 < 0x2c) {
                    if (bVar3) {
                        bVar2 = false;
                    }
                    else {
                        uVar6 = uVar6 | 0x3;
                        bVar3 = true;
                    }
                }
                else {
                    if (bVar1 == 0x62) {
                        if (bVar4) {
                            bVar2 = false;
                        }
                        else {
                            bVar4 = true;
                            uVar6 = uVar6 | 0x40;
                        }
                    }
                }
            }
        }
        else {
            if (bVar1 < 0x64) {
                if (bVar5) {
                    bVar2 = false;
                }
                else {
                    bVar5 = true;
                    *param_2 = *param_2 | 0x1;
                }
            }
            else {
                if (0x6d < bVar1) {
                    if (bVar1 < 0x6f) {
                        if (bVar5) {
                            bVar2 = false;
                        }
                        else {
                            bVar5 = true;
                            *param_2 = *param_2 & 0xfe;
                        }
                    }
                    else {
                        if (bVar1 == 0x74) {
                            if (bVar4) {
                                bVar2 = false;
                            }
                            else {
                                bVar4 = true;
                            }
                        }
                    }
                }
            }
        }
        bVar1 = param_1[0x2];
        param_1 = param_1 + 0x1;
    }
    if ((!bVar4) && (_DAT_004bfdcc == 0x200)) {
        uVar6 = uVar6 | 0x40;
    }
    return uVar6;
}



i32 *  FUN_004a955c(byte *param_1,byte param_2,param_3: u32,param_4: u32,param_5: u32,param_6: *mut i32)

{
let bVar1: u8;
ushort uVar2;
let mut iVar3: i32;
let mut uVar4: u32;

*(param_6 + 0x3) = *(param_6 + 0x3) & 0xfc;
param_6[0x3] = param_6[0x3] | param_3;
iVar3 = FUN_004aa9f0(param_2);
if ((char)iVar3 == 'r') {
uVar4 = 0x0;
if ((param_3 & 0x2) != 0x0) {
uVar4 = 0x2;
}
if ((param_3 & 0x40) == 0x0) {
uVar4 = uVar4 | 0x100;
}
else {
uVar4 = uVar4 | 0x200;
}
}
else {
bVar1 = ((param_3 & 0x1) != 0x0) + 0x21;
if ((param_3 & 0x80) == 0x0) {
bVar1 = bVar1 | 0x40;
}
else {
bVar1 = bVar1 | 0x10;
}
if ((param_3 & 0x40) == 0x0) {
uVar2 = CONCAT11(0x1,bVar1);
}
else {
uVar2 = CONCAT11(0x2,bVar1);
}
uVar4 = uVar2;
}
iVar3 = FUN_004ae9e4(param_1,uVar4,param_5);
param_6[0x4] = iVar3;
if (param_6[0x4] != -0x1) {
param_6[0x1] = 0x0;
param_6[0x5] = 0x0;
*(param_6[0x2] + 0xc) = 0x0;
*(param_6[0x2] + 0x10) = param_4;
*(param_6[0x2] + 0x18) = 0x0;
*(param_6[0x2] + 0x8) = 0x0;
if ((param_3 & 0x80) != 0x0) {
FUN_004aa75c(param_6,0x0,0x2);
}
FUN_004b1b60(param_6);
return param_6;
}
FUN_004af080(param_6);
return 0x0;
}



i32 *  FUN_004a9670(byte *param_1,byte *param_2,param_3: u32)

{
let piVar1: *mut i32;;
let piVar2: *mut i32;;
let mut local_10: u32;

piVar1 = FUN_004a9420(param_2,&local_10);
piVar2 = piVar1;
if ((piVar1 != 0x0) && (piVar2 = FUN_004aefc0(), piVar2 != 0x0)) {
piVar2 = FUN_004a955c(param_1,*param_2,piVar1,local_10,param_3,piVar2);
}
return piVar2;
}



fn FUN_004a96cc(byte *param_1,byte *param_2)

{
    FUN_004a9670(param_1,param_2,0x0);
    return;
}



fn FUN_004a96e4(param_1: *mut u32) -> *mut u32

{
    let puVar1: *mut u32;
    let puVar2: *mut u32;

    (PTR_FUN_004bfb88)();
    for (puVar1 = DAT_005ba410; puVar1 != 0x0; puVar1 = *puVar1) {
        puVar2 = puVar1[0x1];
        if (param_1 == puVar2) {
            if ((*(puVar2 + 0x3) & 0x3) != 0x0) {
                FUN_0049cb70(puVar2,0x1);
            }
            (PTR_FUN_004bfb8c)();
            return param_1;
        }
    }
    puVar1 = &DAT_005ba414;
    loop {
    puVar2 = puVar1;
    puVar1 = *puVar2;
    if (puVar1 == 0x0) {
        FUN_004b1740(0x4);
        (PTR_FUN_004bfb8c)();
        return 0x0;
    }
} while (param_1 != puVar1[0x1]);
    *puVar2 = *puVar1;
    *puVar1 = DAT_005ba410;
    DAT_005ba410 = puVar1;
    (PTR_FUN_004bfb8c)();
    return param_1;
}



i32 *  FUN_004a9764(byte *param_1,byte *param_2,param_3: *mut u32)

{
let mut uVar1: u32;
let piVar2: *mut i32;;
let piVar3: *mut i32;;
let mut uStack20: u32;

piVar2 = FUN_004a9420(param_2,&uStack20);
if (piVar2 != 0x0) {
uVar1 = param_3[0x4];
(PTR_FUN_004bfb78)(uVar1);
if (DAT_004c0048 != (code *)0x0) {
(*DAT_004c0048)(uVar1);
}
piVar3 = FUN_004a96e4(param_3);
if (piVar3 != 0x0) {
piVar3[0x3] = piVar3[0x3] & 0x4000;
piVar3 = FUN_004a955c(param_1,*param_2,piVar2,uStack20,0x0,piVar3);
}
piVar2 = piVar3;
(PTR_FUN_004bfb7c)(uVar1);
}
return piVar2;
}



fn FUN_004a9800(byte *param_1,param_2: *mut u8,param_3: i32) -> i32

{
let bVar2: u8;
let mut uVar1: u32;

if (param_3 != 0x0) {
loop {
bVar2 = *param_1;
uVar1 = CONCAT11(*param_2,bVar2);
if ((0x40 < bVar2) && (bVar2 < 0x5b)) {
uVar1 = uVar1 & 0xffffff00 | (byte)(bVar2 + 0x20);
}
bVar2 = (byte)(uVar1 >> 0x8);
if ((0x40 < bVar2) && (bVar2 < 0x5b)) {
uVar1 = CONCAT11(bVar2 + 0x20,(char)uVar1);
}
bVar2 = (byte)(uVar1 >> 0x8);
if ((byte)uVar1 != bVar2) {
return (uVar1 & 0xff) - bVar2;
}
if (bVar2 == 0x0) {
return 0x0;
}
param_1 = param_1 + 0x1;
param_2 = param_2 + 0x1;
param_3 = param_3 + -0x1;
} while (param_3 != 0x0);
}
return 0x0;
}



byte *  FUN_004a9860(byte **param_1,byte *param_2,byte *param_3,param_4: i32)

{
let mut iVar1: i32;
let mut iVar2: i32;
let mut uVar3: u32;

if (param_1 != (byte **)0x0) {
uVar3 = param_4 - param_3;
*param_1 = param_2;
if (0x106 < uVar3) {
uVar3 = 0x106;
}
iVar1 = FUN_004b1ba0(param_3,uVar3);
FUN_004b1be0(param_2,param_3,iVar1);
iVar2 = FUN_004b1c10(param_2,iVar1);
param_2[iVar2] = 0x0;
iVar1 = FUN_004b1c10(param_2,iVar1);
param_2 = param_2 + iVar1 + 0x1;
}
return param_2;
}



fn FUN_004a98c0(byte *param_1,byte *param_2,byte **param_3,byte **param_4,byte **param_5,byte **param_6)

{
    let bVar1: u8;
    ushort uVar2;
    let extraout_var: u16;
    byte *pbVar4;
    byte *pbVar5;
    byte *pbVar6;
    byte *pbVar7;
    let mut iVar3: i32;

    if (((*param_1 == 0x5c) || (*param_1 == 0x2f)) && ((param_1[0x1] == 0x5c || (param_1[0x1] == 0x2f)))) {
        bVar1 = param_1[0x2];
        pbVar4 = param_1 + 0x2;
        while ((((bVar1 != 0x0 && (bVar1 = *pbVar4, bVar1 != 0x5c)) && (bVar1 != 0x2f)) && (bVar1 != 0x2e))) {
            pbVar4 = FUN_004b1c70(pbVar4);
            bVar1 = *pbVar4;
        }
        param_2 = FUN_004a9860(param_3,param_2,param_1,pbVar4);
        param_1 = pbVar4;
    }
    else {
        if ((*param_1 == 0x0) || (param_1[0x1] != 0x3a)) {
            pbVar4 = param_1;
            if (param_3 != (byte **)0x0) {
                *param_3 = param_2;
                *param_2 = 0x0;
                param_2 = param_2 + 0x1;
            }
        }
        else {
            if (param_3 != (byte **)0x0) {
                *param_3 = param_2;
                bVar1 = *param_1;
                param_2[0x1] = 0x3a;
                param_2[0x2] = 0x0;
                *param_2 = bVar1;
                param_2 = param_2 + 0x3;
            }
            param_1 = param_1 + 0x2;
            pbVar4 = param_1;
        }
    }
    loop {
    pbVar6 = pbVar4;
    pbVar4 = pbVar6;
    pbVar7 = 0x0;
    loop {
        while( true ) {
            pbVar5 = pbVar4;
            uVar2 = FUN_004b1ca0(pbVar5);
            iVar3 = CONCAT22(extraout_var,uVar2);
            if (iVar3 == 0x0) {
                pbVar4 = FUN_004a9860(param_4,param_2,param_1,pbVar6);
                if (pbVar7 == 0x0) {
                    pbVar7 = pbVar5;
                }
                pbVar4 = FUN_004a9860(param_5,pbVar4,pbVar6,pbVar7);
                FUN_004a9860(param_6,pbVar4,pbVar7,pbVar5);
                return;
            }
            if (iVar3 != 0x2e) break;
            pbVar4 = pbVar5 + 0x1;
            pbVar7 = pbVar5;
        }
        pbVar4 = FUN_004b1c70(pbVar5);
    } while ((iVar3 != 0x5c) && (iVar3 != 0x2f));
} while( true );
}



char *  FUN_004a9a00(param_1: &mut String,param_2: &mut String,param_3: i32)

{
let cVar1: u8;
let mut pcVar2: String;
let mut pcVar3: String;

pcVar3 = param_1;
pcVar2 = param_1;
if (param_3 != 0x0) {
loop {
pcVar3 = pcVar2;
if (*param_2 == '\0') break;
pcVar3 = pcVar2 + 0x1;
cVar1 = *param_2;
param_2 = param_2 + 0x1;
*pcVar2 = cVar1;
param_3 = param_3 + -0x1;
pcVar2 = pcVar3;
} while (param_3 != 0x0);
}
for (; param_3 != 0x0; param_3 = param_3 + -0x1) {
*pcVar3 = '\0';
pcVar3 = pcVar3 + 0x1;
}
return param_1;
}



char *  FUN_004a9a40(param_1: &mut String)

{
let cVar1: u8;
let mut pcVar2: String;

for (pcVar2 = param_1; cVar1 = *pcVar2, cVar1 != '\0'; pcVar2 = pcVar2 + 0x1) {
if ((byte)(cVar1 + 0x9fU) < 0x1a) {
*pcVar2 = cVar1 + -0x20;
}
}
return param_1;
}



fn FUN_004a9a60()

{
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_004a9a64(UINT param_1)

{
    let mut extraout_ECX: u32;

    (*_DAT_004bf9bc)();
    if ((char)DAT_005b9de0 == '\0') {
    FUN_004b1d48(extraout_ECX,0xff);
}
    else {
    if (_DAT_004bfbbc != (code *)0x0) {
        (*_DAT_004bfbbc)(0x10,0xff);
    }
}
    exit_func_004a9ab0(param_1);
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void exit_func_004a9ab0(UINT exit_code)

{
let extraout_ECX: u32;

(*_DAT_004bf9bc)();
(*_DAT_004bf9c0)();
if (_DAT_004c0084 != (code *)0x0) {
(*_DAT_004c0084)();
}
if (DAT_005b9de0 == 0x0) {
FUN_004b6084();
FUN_004b1d48(extraout_ECX,0xf);
(PTR_FUN_004bfbb0)();
}
else {
if (_DAT_004bfbbc != (code *)0x0) {
(*_DAT_004bfbbc)(0x0,0xf);
}
}
// UINT uExitCode for ExitProcess
// WARNING: Subroutine does not return
ExitProcess(exit_code);
}



double  FUN_004a9ae0(double param_1)

{
double dVar1;

dVar1 = FUN_004b1dab(SUB84(-param_1,0x0),((ulonglong)-param_1 >> 0x20));
return dVar1;
}



float10 FUN_004a9b12()

{
float10 in_ST0;

return ROUND(in_ST0);
}



fn FUN_004a9b70(param_1: *mut u32,byte *param_2)

{
    let local_8: *mut u32;

    local_8 = &stack0x0000000c;
    FUN_004b2a90(param_1,param_2,&local_8);
    return;
}



fn FUN_004aa04b(param_1: *mut i32,param_2: i32,param_3: i32) -> u32

{
    let mut local_14: u32;

    if ((((param_2 < *param_1) || (*param_1 + param_1[0x2] <= param_2)) || (param_3 < param_1[0x1])) ||
        (param_1[0x1] + param_1[0x3] <= param_3)) {
        local_14 = 0x0;
    }
    else {
        local_14 = 0x1;
    }
    return local_14;
}



fn FUN_004aa0ab(param_1: *mut i32,param_2: i32,param_3: i32,param_4: i32,param_5: i32) -> u32

{
    if (param_2 < *param_1) {
        if (param_2 + param_4 <= *param_1) {
            return 0x0;
        }
    }
    else {
        if (*param_1 + param_1[0x2] <= param_2) {
            return 0x0;
        }
    }
    if (param_3 < param_1[0x1]) {
        if (param_3 + param_5 <= param_1[0x1]) {
            return 0x0;
        }
    }
    else {
        if (param_1[0x1] + param_1[0x3] <= param_3) {
            return 0x0;
        }
    }
    return 0x1;
}



fn FUN_004aa144(param_1: *mut i32,param_2: *mut i32) -> u32

{
    let mut iVar1: i32;
    let mut iVar2: i32;

    iVar1 = *param_2;
    iVar2 = param_2[0x1];
    if (iVar1 < *param_1) {
        if (iVar1 + param_2[0x2] <= *param_1) {
            return 0x0;
        }
    }
    else {
        if (*param_1 + param_1[0x2] <= iVar1) {
            return 0x0;
        }
    }
    if (iVar2 < param_1[0x1]) {
        if (iVar2 + param_2[0x3] <= param_1[0x1]) {
            return 0x0;
        }
    }
    else {
        if (param_1[0x1] + param_1[0x3] <= iVar2) {
            return 0x0;
        }
    }
    return 0x1;
}



fn FUN_004aa20c(param_1: *mut i32,param_2: *mut i32,param_3: *mut i32) -> u32

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut local_20: u32;
    let mut local_1c: u32;
    let mut local_18: i32;
    let mut local_14: i32;

    iVar2 = FUN_004aa144(param_1,param_2);
    if (iVar2 == 0x0) {
        *param_3 = *param_1;
        (param_3 + 0x1) = (param_1 + 0x1);
        local_20 = 0x1;
    }
    else {
        iVar1 = param_2[0x1];
        iVar2 = param_1[0x1];
        if (iVar1 <= iVar2) {
            local_18 = param_1[0x1];
        }
        else {
            *param_3 = *param_1;
            param_3[0x1] = param_1[0x1];
            param_3[0x2] = param_1[0x2];
            param_3[0x3] = param_2[0x1] - param_1[0x1];
            local_18 = param_2[0x1];
        }
        local_1c = (iVar1 > iVar2);
        if (param_2[0x1] + param_2[0x3] < param_1[0x1] + param_1[0x3]) {
            param_3[local_1c * 0x4] = *param_1;
            param_3[local_1c * 0x4 + 0x1] = param_2[0x1] + param_2[0x3];
            param_3[local_1c * 0x4 + 0x2] = param_1[0x2];
            param_3[local_1c * 0x4 + 0x3] = (param_1[0x1] + param_1[0x3]) - (param_2[0x1] + param_2[0x3]);
            local_1c = local_1c + 0x1;
            local_14 = (param_2[0x1] + param_2[0x3]) - local_18;
        }
        else {
            local_14 = (param_1[0x1] + param_1[0x3]) - local_18;
        }
        if (*param_1 < *param_2) {
            param_3[local_1c * 0x4] = *param_1;
            param_3[local_1c * 0x4 + 0x1] = local_18;
            param_3[local_1c * 0x4 + 0x2] = *param_2 - *param_1;
            param_3[local_1c * 0x4 + 0x3] = local_14;
            local_1c = local_1c + 0x1;
        }
        if (*param_2 + param_2[0x2] < *param_1 + param_1[0x2]) {
            param_3[local_1c * 0x4] = *param_2 + param_2[0x2];
            param_3[local_1c * 0x4 + 0x1] = local_18;
            param_3[local_1c * 0x4 + 0x2] = (*param_1 + param_1[0x2]) - (*param_2 + param_2[0x2]);
            param_3[local_1c * 0x4 + 0x3] = local_14;
            local_1c = local_1c + 0x1;
        }
        local_20 = local_1c;
    }
    return local_20;
}



fn FUN_004aa45a(param_1: *mut i32,param_2: *mut i32,param_3: *mut i32) -> i32

{
let mut iVar1: i32;
let mut iVar2: i32;
let mut iVar3: i32;
let mut iVar4: i32;
let mut local_34: i32;
let mut local_30: i32;
let mut local_2c: i32;
let mut local_28: i32;
let mut local_24: i32;

iVar4 = FUN_004aa144(param_1,param_2);
if (iVar4 == 0x0) {
param_3[0x3] = 0x0;
param_3[0x2] = param_3[0x3];
param_3[0x1] = param_3[0x2];
*param_3 = param_3[0x1];
local_24 = *param_3;
}
else {
iVar4 = *param_1;
iVar1 = param_1[0x2];
local_2c = *param_2 + param_2[0x2];
iVar2 = param_1[0x1];
iVar3 = param_1[0x3];
local_28 = param_2[0x1] + param_2[0x3];
if (*param_2 < *param_1) {
local_34 = *param_1;
}
else {
local_34 = *param_2;
}
*param_3 = local_34;
if (param_2[0x1] < param_1[0x1]) {
local_30 = param_1[0x1];
}
else {
local_30 = param_2[0x1];
}
param_3[0x1] = local_30;
if (iVar4 + iVar1 < local_2c) {
local_2c = iVar4 + iVar1;
}
param_3[0x2] = local_2c - *param_3;
if (iVar2 + iVar3 < local_28) {
local_28 = iVar2 + iVar3;
}
param_3[0x3] = local_28 - param_3[0x1];
local_24 = 0x1;
}
return local_24;
}



fn FUN_004aa59b(param_1: *mut i32,param_2: *mut i32,param_3: *mut i32)

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    local_18 = *param_1 + param_1[0x2];
    iVar1 = *param_2;
    iVar2 = param_2[0x2];
    local_14 = param_1[0x1] + param_1[0x3];
    iVar3 = param_2[0x1];
    iVar4 = param_2[0x3];
    if (*param_2 < *param_1) {
        local_20 = *param_2;
    }
    else {
        local_20 = *param_1;
    }
    *param_3 = local_20;
    if (param_2[0x1] < param_1[0x1]) {
        local_1c = param_2[0x1];
    }
    else {
        local_1c = param_1[0x1];
    }
    param_3[0x1] = local_1c;
    if (local_18 < iVar1 + iVar2) {
        local_18 = iVar1 + iVar2;
    }
    param_3[0x2] = local_18 - *param_3;
    if (local_14 < iVar3 + iVar4) {
        local_14 = iVar3 + iVar4;
    }
    param_3[0x3] = local_14 - param_3[0x1];
    return;
}



fn FUN_004aa690(param_1: *mut u32) -> u32

{
    let mut iVar1: i32;
    let DVar2: u32;

    if (((*(param_1 + 0x3) & 0x80) != 0x0) && ((*(param_1 + 0xd) & 0x10) != 0x0)) {
        FUN_004b2b70(param_1);
    }
    DVar2 = set_file_ptr_004b2b90(param_1[0x4]);
    if (DVar2 != 0xffffffff) {
        (PTR_FUN_004bfb78)(param_1[0x4]);
        iVar1 = param_1[0x1];
        if (iVar1 != 0x0) {
            if ((*(param_1 + 0xd) & 0x10) == 0x0) {
                DVar2 = DVar2 - iVar1;
            }
            else {
                DVar2 = DVar2 + iVar1;
            }
        }
        (PTR_FUN_004bfb7c)(param_1[0x4]);
    }
    return DVar2;
}



fn FUN_004aa700(param_1: i32,param_2: *mut i32) -> u32

{
    if ((param_1 <= param_2[0x1]) && (*(param_2[0x2] + 0x8) - *param_2 <= param_1)) {
        *(param_2 + 0x3) = *(param_2 + 0x3) & 0xef;
        *param_2 = *param_2 + param_1;
        param_2[0x1] = param_2[0x1] - param_1;
        return 0x0;
    }
    return 0x1;
}



fn FUN_004aa744(param_1: *mut u32)

{
    let mut uVar1: u32;

    *(param_1 + 0x3) = *(param_1 + 0x3) & 0xef;
    uVar1 = *(param_1[0x2] + 0x8);
    param_1[0x1] = 0x0;
    *param_1 = uVar1;
    return;
}



// WARNING: Removing unreachable block (ram,0x004aa828)

fn FUN_004aa75c(param_1: *mut i32,param_2: i32,param_3: u32) -> u32

{
    let DVar1: u32;
    let mut iVar2: i32;
    let mut iVar3: i32;

    (PTR_FUN_004bfb78)(param_1[0x4]);
    if ((*(param_1 + 0x3) & 0x6) != 0x0) {
        if ((*(param_1 + 0xd) & 0x10) == 0x0) {
            if (param_3 == 0x1) {
                param_2 = param_2 - param_1[0x1];
            }
            iVar3 = *(param_1[0x2] + 0x8);
            param_1[0x1] = 0x0;
            *param_1 = iVar3;
        }
        else {
            DVar1 = FUN_004af2f0(param_1);
            if (DVar1 != 0x0) {
                if ((param_3 == 0x0) && (param_2 < 0x0)) {
                    FUN_004b1740(0x9);
                }
                (PTR_FUN_004bfb7c)(param_1[0x4]);
                return 0xffffffff;
            }
        }
        *(param_1 + 0x3) = *(param_1 + 0x3) & 0xeb;
        DVar1 = set_file_pointer_004af420(param_1[0x4],param_2,param_3);
        if (DVar1 == 0xffffffff) {
            (PTR_FUN_004bfb7c)(param_1[0x4]);
            return 0xffffffff;
        }^
        // goto LAB_004aa92f;
    }
    if (param_3 == 0x0) {
        DVar1 = set_file_ptr_004b2b90(param_1[0x4]);
        iVar3 = FUN_004aa700(param_2 - (DVar1 - param_1[0x1]),param_1);
        if (iVar3 == 0x0)^ // goto LAB_004aa92f;
        DVar1 = set_file_pointer_004af420(param_1[0x4],param_2,0x0);
        if (DVar1 == 0xffffffff) {
            (PTR_FUN_004bfb7c)(param_1[0x4]);
            return 0xffffffff;
        }
    }
    else {
        if (0x1 < param_3) {
            if (param_3 != 0x2) {
                FUN_004b1740(0x9);
                (PTR_FUN_004bfb7c)(param_1[0x4]);
                return 0xffffffff;
            }
            *(param_1 + 0x3) = *(param_1 + 0x3) & 0xef;
            iVar3 = *(param_1[0x2] + 0x8);
            param_1[0x1] = 0x0;
            *param_1 = iVar3;
            DVar1 = set_file_pointer_004af420(param_1[0x4],param_2,0x2);
            if (DVar1 == 0xffffffff) {
                (PTR_FUN_004bfb7c)(param_1[0x4]);
                return 0xffffffff;
            }^
            // goto LAB_004aa92f;
        }
        iVar3 = param_1[0x1];
        iVar2 = FUN_004aa700(param_2,param_1);
        if (iVar2 == 0x0)^ // goto LAB_004aa92f;
        DVar1 = set_file_pointer_004af420(param_1[0x4],param_2 - iVar3,param_3);
        if (DVar1 == 0xffffffff) {
            (PTR_FUN_004bfb7c)(param_1[0x4]);
            return 0xffffffff;
        }
    }
    FUN_004aa744(param_1);
    LAB_004aa92f:
        (PTR_FUN_004bfb7c)(param_1[0x4]);
    return 0x0;
}



fn FUN_004aa980(param_1: u32,byte *param_2,param_3: *mut *mut u32)

{
    let mut local_1c: *mut u8;
    let mut local_18: *mut u8;
    let mut local_14: u32;

    local_14 = param_1;
    local_1c = &LAB_004aa950;
    local_18 = &LAB_004aa978;
    FUN_004b2c10(&local_1c,param_2,param_3);
    return;
}



fn FUN_004aa9bc(param_1: u32,byte *param_2)

{
    let local_8: *mut u32;

    local_8 = &stack0x0000000c;
    FUN_004aa980(param_1,param_2,&local_8);
    return;
}



fn FUN_004aa9f0(param_1: i32) -> i32

{
if ((0x40 < param_1) && (param_1 < 0x5b)) {
param_1 = param_1 + 0x20;
}
return param_1;
}



fn FUN_004aaa04(param_1: *mut u32,param_2: i32,param_3: i32,param_4: i32,param_5: i32,param_6: i32)

{
    let mut iVar1: i32;
    let puVar2: *mut u32;
    let mut uVar3: u32;
    let mut uVar4: u32;
    let mut uVar5: u32;

    iVar1 = DAT_005b9224 - param_4;
    puVar2 = (param_3 * DAT_005b9224 + DAT_005b9220 + param_2);
    uVar4 = -puVar2 & 0x3;
    uVar5 = param_4 - uVar4;
    uVar3 = uVar4;
    if (param_4 < uVar4) {
        uVar4 = uVar4 + uVar5;
        uVar5 = 0x0;
        uVar3 = uVar4;
    }
    loop {
    for (; uVar3 != 0x0; uVar3 = uVar3 - 0x1) {
        puVar2 = param_1;
        param_1 = (param_1 + 0x1);
        puVar2 = (puVar2 + 0x1);
    }
    for (uVar3 = uVar5 >> 0x2; uVar3 != 0x0; uVar3 = uVar3 - 0x1) {
        *puVar2 = *param_1;
        param_1 = param_1 + 0x1;
        puVar2 = puVar2 + 0x1;
    }
    for (uVar3 = uVar5 & 0x3; uVar3 != 0x0; uVar3 = uVar3 - 0x1) {
        puVar2 = param_1;
        param_1 = (param_1 + 0x1);
        puVar2 = (puVar2 + 0x1);
    }
    param_1 = (param_1 + param_6);
    puVar2 = (puVar2 + iVar1);
    param_5 = param_5 + -0x1;
    uVar3 = uVar4;
} while (param_5 != 0x0);
    return;
}



fn FUN_004aaa6c(param_1: i32,param_2: i32,param_3: *mut u32,param_4: i32,param_5: i32,param_6: i32)

{
    let mut iVar1: i32;
    let puVar2: *mut u32;
    let mut uVar3: u32;
    let mut uVar4: u32;
    let mut uVar5: u32;

    iVar1 = DAT_005b9224 - param_4;
    puVar2 = (param_2 * DAT_005b9224 + DAT_005b9220 + param_1);
    uVar4 = -puVar2 & 0x3;
    uVar5 = param_4 - uVar4;
    uVar3 = uVar4;
    if (param_4 < uVar4) {
        uVar4 = uVar4 + uVar5;
        uVar5 = 0x0;
        uVar3 = uVar4;
    }
    loop {
    for (; uVar3 != 0x0; uVar3 = uVar3 - 0x1) {
        param_3 = puVar2;
        puVar2 = (puVar2 + 0x1);
        param_3 = (param_3 + 0x1);
    }
    for (uVar3 = uVar5 >> 0x2; uVar3 != 0x0; uVar3 = uVar3 - 0x1) {
        *param_3 = *puVar2;
        puVar2 = puVar2 + 0x1;
        param_3 = param_3 + 0x1;
    }
    for (uVar3 = uVar5 & 0x3; uVar3 != 0x0; uVar3 = uVar3 - 0x1) {
        param_3 = puVar2;
        puVar2 = (puVar2 + 0x1);
        param_3 = (param_3 + 0x1);
    }
    puVar2 = (puVar2 + iVar1);
    param_3 = (param_3 + param_6);
    param_5 = param_5 + -0x1;
    uVar3 = uVar4;
} while (param_5 != 0x0);
    return;
}



fn FUN_004aaae0(LPCSTR param_1)

{
    FUN_004aaaf0(param_1);
    return;
}



fn FUN_004aaaf0(param_1: *mut i32)

{
    let piVar1: *mut i32;;
    let piVar2: *mut i32;;

    if (param_1 != 0x0) {
        (PTR_FUN_004bfb90)();
        if (((DAT_005b9d10 == 0x0) ||
            ((((param_1 < DAT_005b9d10 || (piVar2 = DAT_005b9d10, (*DAT_005b9d10 + DAT_005b9d10) <= param_1)) &&
                ((piVar2 = DAT_005b9d10[0x1], piVar2 == 0x0 ||
                    ((param_1 < piVar2 || ((*piVar2 + piVar2) <= param_1)))))) &&
                ((piVar2 = DAT_005b9d10[0x2], piVar2 == 0x0 ||
                    ((param_1 < piVar2 || ((*piVar2 + piVar2) <= param_1)))))))) &&
            ((piVar1 = DAT_004bfae8, DAT_004bfaec == 0x0 ||
                ((((param_1 < DAT_004bfaec || (piVar2 = DAT_004bfaec, (*DAT_004bfaec + DAT_004bfaec) <= param_1)) &&
                    ((piVar2 = DAT_004bfaec[0x1], piVar2 == 0x0 ||
                        ((param_1 < piVar2 || ((*piVar2 + piVar2) <= param_1)))))) &&
                    ((piVar2 = DAT_004bfaec[0x2], piVar2 == 0x0 ||
                        ((param_1 < piVar2 || ((*piVar2 + piVar2) <= param_1)))))))))) {
            while (piVar2 = piVar1, piVar2 != 0x0) {
                if ((piVar2 <= param_1) && (param_1 < (*piVar2 + piVar2)))^ // goto LAB_004aabac;
                piVar1 = piVar2[0x2];
            }
        }
        else {
            LAB_004aabac:
                FUN_004b3cc0();
            if ((piVar2 < DAT_004bfaec) && (DAT_004bfaf0 < piVar2[0x5])) {
                DAT_004bfaf0 = piVar2[0x5];
            }
            DAT_005ba480 = 0x0;
            DAT_005b9d10 = piVar2;
        }
        (PTR_FUN_004bfb98)();
    }
    return;
}



fn FUN_004aac00(param_1: u32) -> String

{
    let puVar1: *mut u32;

    puVar1 = FUN_004aac10(param_1);
    return puVar1;
}



fn FUN_004aac10(param_1: u32) -> *mut u32

{
    let mut uVar1: u32;
    let mut bVar2: bool;
    let mut iVar3: i32;
    let mut uVar4: u32;
    let puStack40: *mut u32;
    let mut local_24: u32;

    if ((param_1 == 0x0) || (0xffffffd4 < param_1)) {
        return 0x0;
    }
    local_24 = param_1 + 0xb & 0xfffffff8;
    if (local_24 < 0x10) {
        local_24 = 0x10;
    }
    (PTR_FUN_004bfb90)();
    puStack40 = 0x0;
    bVar2 = false;
    while( true ) {
        while( true ) {
            if (DAT_004bfaf0 < local_24) {
                uVar4 = DAT_004bfaec;
                if (DAT_004bfaec == 0x0) {
                    DAT_004bfaf0 = DAT_004bfaec;
                    uVar4 = DAT_004bfae8;
                }
            }
            else {
                DAT_004bfaf0 = 0x0;
                uVar4 = DAT_004bfae8;
            }
            for (; uVar4 != 0x0; uVar4 = *(uVar4 + 0x8)) {
                uVar1 = *(uVar4 + 0x14);
                DAT_004bfaec = uVar4;
                if ((param_1 <= uVar1) && (puStack40 = FUN_004b3c10(), puStack40 != 0x0))^ // goto LAB_004aad00;
                if (DAT_004bfaf0 < uVar1) {
                    DAT_004bfaf0 = uVar1;
                }
            }
            if ((bVar2) || (iVar3 = FUN_004b3f18(param_1), iVar3 == 0x0)) break;
            bVar2 = true;
        }
        iVar3 = FUN_004b3f80();
        if (iVar3 == 0x0) break;
        bVar2 = false;
    }
    LAB_004aad00:
        DAT_005ba480 = 0x0;
    (PTR_FUN_004bfb98)();
    return puStack40;
}



fn FUN_004aad20(param_1: u32) -> u32

{
    if (DAT_005b9da0 < 0x20) {
        DAT_005b9da0 = DAT_005b9da0 + 0x1;
        *(&DAT_005b9d1c + DAT_005b9da0 * 0x4) = param_1;
        return 0x0;
    }
    return 0xffffffff;
}



fn FUN_004aad90(param_1: i32,param_2: i32,param_3: i32,param_4: i32,undefined param_5)

{
    let puVar1: *mut u32;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut uVar4: u32;
    let mut uVar5: u32;

    puVar1 = (param_2 * DAT_005b9224 + DAT_005b9220 + param_1);
    iVar3 = DAT_005b9224 - param_3;
    uVar4 = -puVar1 & 0x3;
    uVar5 = param_3 - uVar4;
    uVar2 = uVar4;
    if (uVar5 < 0x0) {
        uVar4 = uVar4 + uVar5;
        uVar5 = 0x0;
        uVar2 = uVar4;
    }
    loop {
    for (; uVar2 != 0x0; uVar2 = uVar2 - 0x1) {
        puVar1 = param_5;
        puVar1 = (puVar1 + 0x1);
    }
    for (uVar2 = uVar5 >> 0x2; uVar2 != 0x0; uVar2 = uVar2 - 0x1) {
        *puVar1 = CONCAT22(CONCAT11(param_5,param_5),CONCAT11(param_5,param_5));
        puVar1 = puVar1 + 0x1;
    }
    for (uVar2 = uVar5 & 0x3; uVar2 != 0x0; uVar2 = uVar2 - 0x1) {
        puVar1 = param_5;
        puVar1 = (puVar1 + 0x1);
    }
    puVar1 = (puVar1 + iVar3);
    param_4 = param_4 + -0x1;
    uVar2 = uVar4;
} while (param_4 != 0x0);
    return;
}



fn FUN_004aae04(param_1: i32,param_2: i32,undefined param_3)

{
    (param_2 * DAT_005b9224 + DAT_005b9220 + param_1) = param_3;
    return;
}



void
FUN_004aae28(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: i32,param_6: u32,param_7: u32,param_8: i32,
param_9: i32,param_10: u32,param_11: u32)

{
let mut puVar1: *mut u8;
ulonglong uVar2;
let mut iVar3: i32;
let mut iVar4: i32;
let mut puVar5: *mut u8;
let mut iVar6: i32;
let mut puVar7: *mut u8;
let mut iVar8: i32;
let mut uVar9: u32;
let mut iVar10: i32;
let mut iVar11: i32;
let mut iVar12: i32;
let mut local_18: i32;

iVar3 = (param_7 / param_11) * param_6;
iVar4 = DAT_005b9224 - param_3;
puVar5 = (param_2 * DAT_005b9224 + DAT_005b9220 + param_1);
iVar6 = 0x0;
iVar11 = 0x0;
uVar9 = param_1 - param_8;
if (uVar9 != 0x0) {
uVar2 = (ulonglong)(param_6 % param_10) * (ulonglong)uVar9;
iVar11 = (uVar2 % (ulonglong)param_10);
iVar6 = (uVar2 / param_10) + (param_6 / param_10) * uVar9;
}
iVar12 = 0x0;
uVar9 = param_2 - param_9;
local_18 = param_5;
if (uVar9 != 0x0) {
uVar2 = (ulonglong)(param_7 % param_11) * (ulonglong)uVar9;
iVar12 = (uVar2 % (ulonglong)param_11);
local_18 = (uVar2 / param_11) * param_6 + iVar3 * uVar9 + param_5;
}
while( true ) {
puVar7 = (local_18 + iVar6);
iVar8 = iVar11;
iVar10 = param_3;
while( true ) {
puVar1 = puVar5 + 0x1;
*puVar5 = *puVar7;
puVar7 = puVar7 + param_6 / param_10;
iVar8 = iVar8 + param_6 % param_10;
iVar10 = iVar10 + -0x1;
if (iVar10 == 0x0) break;
puVar5 = puVar1;
if (param_10 <= iVar8) {
iVar8 = iVar8 - param_10;
puVar7 = puVar7 + 0x1;
}
}
puVar5 = puVar1 + iVar4;
param_4 = param_4 + -0x1;
if (param_4 == 0x0) break;
local_18 = local_18 + iVar3;
iVar12 = iVar12 + param_7 % param_11;
if (param_11 <= iVar12) {
iVar12 = iVar12 - param_11;
local_18 = param_6 + local_18;
}
}
return;
}



fn FUN_004aaf15(param_1: *mut u8,param_2: u32,param_3: u32,param_4: *mut u8,param_5: u32,param_6: u32)

{
    let mut iVar1: i32;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let mut iVar4: i32;
    let mut puVar5: *mut u8;
    let mut puVar6: *mut u8;
    let mut local_18: *mut u8;

    local_18 = param_4;
    iVar4 = 0x0;
    uVar3 = param_3;
    while( true ) {
        iVar1 = 0x0;
        uVar2 = param_2;
        puVar5 = local_18;
        puVar6 = param_1;
        while( true ) {
            param_1 = puVar6 + 0x1;
            *puVar6 = *puVar5;
            puVar5 = puVar5 + param_5 / param_2;
            iVar1 = iVar1 + param_5 % param_2;
            uVar2 = uVar2 - 0x1;
            if (uVar2 == 0x0) break;
            puVar6 = param_1;
            if (param_2 <= iVar1) {
                iVar1 = iVar1 - param_2;
                puVar5 = puVar5 + 0x1;
            }
        }
        uVar3 = uVar3 - 0x1;
        if (uVar3 == 0x0) break;
        local_18 = local_18 + (param_6 / param_3) * param_5;
        iVar4 = iVar4 + param_6 % param_3;
        if (param_3 <= iVar4) {
            iVar4 = iVar4 - param_3;
            local_18 = local_18 + param_5;
        }
    }
    return;
}



fn FUN_004aaf98(param_1: &mut String,param_2: i32,param_3: i32,param_4: u32,param_5: i32,param_6: i32)

{
    let mut pcVar1: String;
    let cVar2: u8;
    let mut iVar3: i32;
    let mut pcVar4: String;
    let mut uVar5: u32;
    let mut bVar6: bool;

    iVar3 = DAT_005b9224 - param_4;
    pcVar4 = (param_3 * DAT_005b9224 + DAT_005b9220 + param_2);
    while (uVar5 = param_4 >> 0x2, uVar5 == 0x0) {
        LAB_004ab034:
            uVar5 = param_4 & 0x3;
        if (uVar5 != 0x0)^ // goto LAB_004ab046;
        while( true ) {
            param_1 = param_1 + param_6;
            pcVar4 = pcVar4 + iVar3;
            param_5 = param_5 + -0x1;
            if (param_5 != 0x0) break;
            if (param_5 == 0x0) {
                return;
            }
            LAB_004ab046:
                uVar5 = uVar5 + 0x1;
            pcVar1 = pcVar4;
            while (pcVar4 = pcVar1, uVar5 = uVar5 - 0x1, uVar5 != 0x0) {
                cVar2 = *param_1;
                param_1 = param_1 + 0x1;
                pcVar1 = pcVar4 + 0x1;
                if (cVar2 != '\0') {
                    *pcVar4 = cVar2;
                }
            }
        }
    }
    loop {
    cVar2 = *param_1;
    if (cVar2 != '\0')^ // goto LAB_004ab002;
    LAB_004aafda:
        cVar2 = param_1[0x1];
    if (cVar2 != '\0')^ // goto LAB_004ab00c;
    LAB_004aafe1:
        cVar2 = param_1[0x2];
    if (cVar2 != '\0')^ // goto LAB_004ab017;
    LAB_004aafe8:
        cVar2 = param_1[0x3];
    if (cVar2 != '\0')^ // goto LAB_004ab022;
    while( true ) {
        pcVar4 = pcVar4 + 0x4;
        param_1 = param_1 + 0x4;
        uVar5 = uVar5 - 0x1;
        bVar6 = uVar5 == 0x0;
        if (!bVar6) break;
        while( true ) {
            if (bVar6)^ // goto LAB_004ab034;
            cVar2 = *param_1;
            if (cVar2 == '\0')^ // goto LAB_004aafda;
            LAB_004ab002:
                *pcVar4 = cVar2;
            cVar2 = param_1[0x1];
            if (cVar2 == '\0')^ // goto LAB_004aafe1;
            LAB_004ab00c:
                pcVar4[0x1] = cVar2;
            cVar2 = param_1[0x2];
            if (cVar2 == '\0')^ // goto LAB_004aafe8;
            LAB_004ab017:
                pcVar4[0x2] = cVar2;
            cVar2 = param_1[0x3];
            if (cVar2 == '\0') break;
            LAB_004ab022:
                pcVar4[0x3] = cVar2;
            pcVar4 = pcVar4 + 0x4;
            param_1 = param_1 + 0x4;
            uVar5 = uVar5 - 0x1;
            bVar6 = uVar5 == 0x0;
        }
    }
} while( true );
}



void
FUN_004ab060(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: i32,param_6: i32,param_7: u32,param_8: i32,
param_9: i32,param_10: u32,param_11: u32)

{
let bVar1: u8;
ulonglong uVar2;
let mut puVar3: *mut u8;
let mut uVar4: u32;
let mut iVar5: i32;
let mut iVar6: i32;
let mut puVar7: *mut u8;
let mut iVar8: i32;
byte *pbVar9;
let mut uVar10: u32;
let mut iVar11: i32;
let mut uVar12: u32;
let mut iVar13: i32;
let mut iVar14: i32;
let mut local_18: i32;

uVar4 = (param_6 - 0x1U) / (param_10 - 0x1);
uVar12 = (param_6 - 0x1U) % (param_10 - 0x1);
iVar5 = (param_7 / param_11) * param_6;
iVar6 = DAT_005b9224 - param_3;
puVar7 = (param_2 * DAT_005b9224 + DAT_005b9220 + param_1);
iVar8 = 0x0;
iVar13 = 0x0;
uVar10 = param_1 - param_8;
if (uVar10 != 0x0) {
iVar13 = (((ulonglong)uVar12 * (ulonglong)uVar10) % (ulonglong)param_10);
iVar8 = (((ulonglong)uVar12 * (ulonglong)uVar10) / (ulonglong)param_10) + uVar4 * uVar10;
}
iVar14 = 0x0;
uVar10 = param_2 - param_9;
local_18 = param_5;
if (uVar10 != 0x0) {
uVar2 = (ulonglong)(param_7 % param_11) * (ulonglong)uVar10;
iVar14 = (uVar2 % (ulonglong)param_11);
local_18 = (uVar2 / param_11) * param_6 + iVar5 * uVar10 + param_5;
}
loop {
uVar10 = iVar14 << 0x10;
pbVar9 = (local_18 + iVar8);
iVar14 = iVar13;
iVar11 = param_3;
while( true ) {
bVar1 = *pbVar9;
uVar10 = uVar10 & 0xffffff00 | bVar1;
puVar3 = puVar7;
while (bVar1 != 0x0) {
*puVar3 = (char)uVar10;
pbVar9 = pbVar9 + uVar4;
iVar14 = iVar14 + uVar12;
iVar11 = iVar11 + -0x1;
if (iVar11 == 0x0)^ // goto LAB_004ab147;
if (param_10 <= iVar14) {
iVar14 = iVar14 - param_10;
pbVar9 = pbVar9 + 0x1;
}
bVar1 = *pbVar9;
uVar10 = uVar10 & 0xffffff00 | bVar1;
puVar3 = puVar3 + 0x1;
}
puVar7 = puVar3 + 0x1;
pbVar9 = pbVar9 + uVar4;
iVar14 = iVar14 + uVar12;
iVar11 = iVar11 + -0x1;
if (iVar11 == 0x0) break;
if (param_10 <= iVar14) {
iVar14 = iVar14 - param_10;
pbVar9 = pbVar9 + 0x1;
}
}
// LAB_004ab147:
puVar7 = puVar3 + iVar6 + 0x1;
param_4 = param_4 + -0x1;
if (param_4 == 0x0) {
return;
}
local_18 = local_18 + iVar5;
iVar14 = (uVar10 >> 0x10) + param_7 % param_11;
if (param_11 <= iVar14) {
iVar14 = iVar14 - param_11;
local_18 = param_6 + local_18;
}
} while( true );
}



u32  read_file_func_004ab180(param_1: u32,char *read_buffer,DWORD param_3)

{
let mut pcVar1: String;
let mut uVar2: u32;
let mut iVar3: i32;
let mut success: bool;
let err_code: u32;
let mut pcVar4: String;
let mut uVar5: u32;
let mut num_bytes_read: u32;
let num_bytes_to_read: u32;
let mut local_18: u32;
HANDLE file_handle;

if ((param_1 < 0x0) || (DAT_004bffe8 < param_1)) {
FUN_004b1740(0x4);
uVar2 = 0xffffffff;
}
else {
file_handle = *(HANDLE *)(DAT_004c0190 + param_1 * 0x4);
uVar2 = FUN_004b1a30(param_1);
local_18 = uVar2;
if (uVar2 == 0x0) {
FUN_004b1740(0x4);
return 0xffffffff;
}
(PTR_FUN_004bfb78)(param_1);
if ((uVar2 & 0x1) == 0x0) {
FUN_004b1740(0x6);
(PTR_FUN_004bfb7c)(param_1);
return 0xffffffff;
}
if ((uVar2 & 0x40) == 0x0) {
num_bytes_to_read = param_3;
uVar2 = 0x0;
while( true ) {
if ((DAT_004c006c == (code *)0x0) || (iVar3 = (*DAT_004c0044)(param_1), iVar3 == 0x0)) {
// LPOVERLAPPED lpOverlapped for ReadFile
// LPDWORD lpNumberOfBytesRead for ReadFile
// DWORD nNumberOfBytesToRead for ReadFile
// LPVOID lpBuffer for ReadFile
// HANDLE hFile for ReadFile
success = ReadFile(file_handle,read_buffer,num_bytes_to_read,&num_bytes_read,(LPOVERLAPPED)0x0);
if (success == 0x0) {
(PTR_FUN_004bfb7c)(param_1);
err_code = GetLastError();
if (err_code == 0x6d) {
return uVar2;
}^
// goto LAB_004ab28a;
}
}
else {
num_bytes_read = (*DAT_004c006c)(iVar3,read_buffer,num_bytes_to_read);
}
if (num_bytes_read == 0x0) break;
uVar5 = 0x0;
iVar3 = 0x0;
pcVar4 = read_buffer;
if (num_bytes_read != 0x0) {
loop {
if (*pcVar4 == '\x1a') {
set_file_pointer_004af420(param_1,(uVar5 - num_bytes_read) + 0x1,0x1);^
// goto LAB_004ab376;
}
if (*pcVar4 != '\r') {
uVar2 = uVar2 + 0x1;
pcVar1 = read_buffer + iVar3;
iVar3 = iVar3 + 0x1;
*pcVar1 = *pcVar4;
}
uVar5 = uVar5 + 0x1;
pcVar4 = pcVar4 + 0x1;
} while (uVar5 < num_bytes_read);
}
num_bytes_to_read = num_bytes_to_read - iVar3;
read_buffer = read_buffer + iVar3;
if (((local_18 & 0x2000) != 0x0) || (num_bytes_to_read == 0x0)) break;
}
}
else {
if ((DAT_004c006c == (code *)0x0) || (iVar3 = (*DAT_004c0044)(param_1), iVar3 == 0x0)) {
// LPOVERLAPPED lpOverlapped for ReadFile
// LPDWORD lpNumberOfBytesRead for ReadFile
// DWORD nNumberOfBytesToRead for ReadFile
// LPVOID lpBuffer for ReadFile
// HANDLE hFile for ReadFile
success = ReadFile(file_handle,read_buffer,param_3,&num_bytes_read,(LPOVERLAPPED)0x0);
uVar2 = num_bytes_read;
if (success == 0x0) {
(PTR_FUN_004bfb7c)(param_1);
err_code = GetLastError();
if (err_code == 0x6d) {
return uVar2;
}
// LAB_004ab28a:
uVar2 = handle_err_fn_004b12fc();
return uVar2;
}
}
else {
uVar2 = (*DAT_004c006c)(iVar3,read_buffer,param_3);
}
}
// LAB_004ab376:
(PTR_FUN_004bfb7c)(param_1);
}
return uVar2;
}



fn FUN_004ab390(param_1: u32) -> u32

{
    let DVar1: u32;

    if ((-0x1 < param_1) && (param_1 <= DAT_004bffe8)) {
        (PTR_FUN_004bfb78)(param_1);
        DVar1 = FUN_004af4b0(param_1);
        (PTR_FUN_004bfb7c)(param_1);
        (PTR_FUN_004bfb84)(param_1);
        return DVar1;
    }
    FUN_004b1740(0x4);
    return 0xffffffff;
}



fn FUN_004ab3e4(param_1: i32,param_2: i32,param_3: i32,param_4: i32,undefined param_5)

{
    let mut iVar1: i32;
    let mut puVar2: *mut u8;
    let mut iVar3: i32;
    let mut uVar4: u32;
    let mut uVar5: u32;
    let mut iVar6: i32;
    let mut uVar7: u32;
    let mut uVar8: u32;
    let mut iVar9: i32;

    iVar1 = DAT_005b9224;
    puVar2 = (param_2 * DAT_005b9224 + DAT_005b9220 + param_1);
    uVar4 = param_3 - param_1;
    uVar5 = uVar4;
    if (param_3 < param_1) {
        uVar5 = -uVar4;
    }
    uVar7 = param_4 - param_2;
    uVar8 = uVar7;
    if (param_4 < param_2) {
        uVar8 = -uVar7;
    }
    if (uVar5 < uVar8) {
        iVar3 = uVar8 + 0x1;
        iVar6 = uVar5 * 0x2;
        uVar5 = iVar6 - uVar8;
        iVar9 = uVar5 - uVar8;
        if ((uVar7 & 0x8000) == 0x0) {
            if ((uVar4 & 0x8000) == 0x0) {
                loop {
                    while( true ) {
                        *puVar2 = param_5;
                        puVar2 = puVar2 + iVar1;
                        if ((uVar5 & 0x8000) != 0x0) break;
                        uVar5 = uVar5 + iVar9;
                        puVar2 = puVar2 + 0x1;
                        iVar3 = iVar3 + -0x1;
                        if (iVar3 == 0x0) {
                            return;
                        }
                    }
                    uVar5 = uVar5 + iVar6;
                    iVar3 = iVar3 + -0x1;
                } while (iVar3 != 0x0);
            }
            else {
                loop {
                    while( true ) {
                        *puVar2 = param_5;
                        puVar2 = puVar2 + iVar1;
                        if ((uVar5 & 0x8000) != 0x0) break;
                        uVar5 = uVar5 + iVar9;
                        puVar2 = puVar2 + -0x1;
                        iVar3 = iVar3 + -0x1;
                        if (iVar3 == 0x0) {
                            return;
                        }
                    }
                    uVar5 = uVar5 + iVar6;
                    iVar3 = iVar3 + -0x1;
                } while (iVar3 != 0x0);
            }
        }
        else {
            if ((uVar4 & 0x8000) == 0x0) {
                loop {
                    while( true ) {
                        *puVar2 = param_5;
                        puVar2 = puVar2 + -iVar1;
                        if ((uVar5 & 0x8000) != 0x0) break;
                        uVar5 = uVar5 + iVar9;
                        puVar2 = puVar2 + 0x1;
                        iVar3 = iVar3 + -0x1;
                        if (iVar3 == 0x0) {
                            return;
                        }
                    }
                    uVar5 = uVar5 + iVar6;
                    iVar3 = iVar3 + -0x1;
                } while (iVar3 != 0x0);
            }
            else {
                loop {
                    while( true ) {
                        *puVar2 = param_5;
                        puVar2 = puVar2 + -iVar1;
                        if ((uVar5 & 0x8000) != 0x0) break;
                        uVar5 = uVar5 + iVar9;
                        puVar2 = puVar2 + -0x1;
                        iVar3 = iVar3 + -0x1;
                        if (iVar3 == 0x0) {
                            return;
                        }
                    }
                    uVar5 = uVar5 + iVar6;
                    iVar3 = iVar3 + -0x1;
                } while (iVar3 != 0x0);
            }
        }
    }
    else {
        iVar3 = uVar5 + 0x1;
        iVar9 = uVar8 * 0x2;
        uVar8 = iVar9 - uVar5;
        iVar6 = uVar8 - uVar5;
        if ((uVar7 & 0x8000) == 0x0) {
            if ((uVar4 & 0x8000) == 0x0) {
                loop {
                    while( true ) {
                        *puVar2 = param_5;
                        puVar2 = puVar2 + 0x1;
                        if ((uVar8 & 0x8000) != 0x0) break;
                        uVar8 = uVar8 + iVar6;
                        puVar2 = puVar2 + iVar1;
                        iVar3 = iVar3 + -0x1;
                        if (iVar3 == 0x0) {
                            return;
                        }
                    }
                    uVar8 = uVar8 + iVar9;
                    iVar3 = iVar3 + -0x1;
                } while (iVar3 != 0x0);
            }
            else {
                loop {
                    while( true ) {
                        *puVar2 = param_5;
                        puVar2 = puVar2 + -0x1;
                        if ((uVar8 & 0x8000) != 0x0) break;
                        uVar8 = uVar8 + iVar6;
                        puVar2 = puVar2 + iVar1;
                        iVar3 = iVar3 + -0x1;
                        if (iVar3 == 0x0) {
                            return;
                        }
                    }
                    uVar8 = uVar8 + iVar9;
                    iVar3 = iVar3 + -0x1;
                } while (iVar3 != 0x0);
            }
        }
        else {
            if ((uVar4 & 0x8000) == 0x0) {
                loop {
                    while( true ) {
                        *puVar2 = param_5;
                        puVar2 = puVar2 + 0x1;
                        if ((uVar8 & 0x8000) != 0x0) break;
                        uVar8 = uVar8 + iVar6;
                        puVar2 = puVar2 + -iVar1;
                        iVar3 = iVar3 + -0x1;
                        if (iVar3 == 0x0) {
                            return;
                        }
                    }
                    uVar8 = uVar8 + iVar9;
                    iVar3 = iVar3 + -0x1;
                } while (iVar3 != 0x0);
            }
            else {
                loop {
                    while( true ) {
                        *puVar2 = param_5;
                        puVar2 = puVar2 + -0x1;
                        if ((uVar8 & 0x8000) != 0x0) break;
                        uVar8 = uVar8 + iVar6;
                        puVar2 = puVar2 + -iVar1;
                        iVar3 = iVar3 + -0x1;
                        if (iVar3 == 0x0) {
                            return;
                        }
                    }
                    uVar8 = uVar8 + iVar9;
                    iVar3 = iVar3 + -0x1;
                } while (iVar3 != 0x0);
            }
        }
    }
    return;
}



fn FUN_004ab5b0(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: i32,param_6: i32,param_7: i32) -> i32

{
let mut puVar1: *mut u8;
let mut iVar2: i32;
let mut puVar3: *mut u8;
let mut iVar4: i32;
let mut uVar5: u32;
let mut uVar6: u32;
let mut iVar7: i32;
let mut uVar8: u32;
let mut uVar9: u32;
let mut iVar10: i32;

iVar2 = DAT_005b9224;
puVar3 = (param_2 * DAT_005b9224 + DAT_005b9220 + param_1);
uVar5 = param_3 - param_1;
uVar6 = uVar5;
if (param_3 < param_1) {
uVar6 = -uVar5;
}
uVar8 = param_4 - param_2;
uVar9 = uVar8;
if (param_4 < param_2) {
uVar9 = -uVar8;
}
if (uVar6 < uVar9) {
iVar4 = uVar9 + 0x1;
iVar7 = uVar6 * 0x2;
uVar6 = iVar7 - uVar9;
iVar10 = uVar6 - uVar9;
if ((uVar8 & 0x8000) == 0x0) {
if ((uVar5 & 0x8000) == 0x0) {
loop {
while( true ) {
puVar1 = (param_5 + param_6);
if (param_6 == 0x0) {
param_6 = param_7;
}
param_6 = param_6 + -0x1;
*puVar3 = *puVar1;
puVar3 = puVar3 + iVar2;
if ((uVar6 & 0x8000) != 0x0) break;
uVar6 = uVar6 + iVar10;
puVar3 = puVar3 + 0x1;
iVar4 = iVar4 + -0x1;
if (iVar4 == 0x0) {
return param_6;
}
}
uVar6 = uVar6 + iVar7;
iVar4 = iVar4 + -0x1;
} while (iVar4 != 0x0);
}
else {
loop {
while( true ) {
puVar1 = (param_5 + param_6);
if (param_6 == 0x0) {
param_6 = param_7;
}
param_6 = param_6 + -0x1;
*puVar3 = *puVar1;
puVar3 = puVar3 + iVar2;
if ((uVar6 & 0x8000) != 0x0) break;
uVar6 = uVar6 + iVar10;
puVar3 = puVar3 + -0x1;
iVar4 = iVar4 + -0x1;
if (iVar4 == 0x0) {
return param_6;
}
}
uVar6 = uVar6 + iVar7;
iVar4 = iVar4 + -0x1;
} while (iVar4 != 0x0);
}
}
else {
if ((uVar5 & 0x8000) == 0x0) {
loop {
while( true ) {
puVar1 = (param_5 + param_6);
if (param_6 == 0x0) {
param_6 = param_7;
}
param_6 = param_6 + -0x1;
*puVar3 = *puVar1;
puVar3 = puVar3 + -iVar2;
if ((uVar6 & 0x8000) != 0x0) break;
uVar6 = uVar6 + iVar10;
puVar3 = puVar3 + 0x1;
iVar4 = iVar4 + -0x1;
if (iVar4 == 0x0) {
return param_6;
}
}
uVar6 = uVar6 + iVar7;
iVar4 = iVar4 + -0x1;
} while (iVar4 != 0x0);
}
else {
loop {
while( true ) {
puVar1 = (param_5 + param_6);
if (param_6 == 0x0) {
param_6 = param_7;
}
param_6 = param_6 + -0x1;
*puVar3 = *puVar1;
puVar3 = puVar3 + -iVar2;
if ((uVar6 & 0x8000) != 0x0) break;
uVar6 = uVar6 + iVar10;
puVar3 = puVar3 + -0x1;
iVar4 = iVar4 + -0x1;
if (iVar4 == 0x0) {
return param_6;
}
}
uVar6 = uVar6 + iVar7;
iVar4 = iVar4 + -0x1;
} while (iVar4 != 0x0);
}
}
}
else {
iVar4 = uVar6 + 0x1;
iVar10 = uVar9 * 0x2;
uVar9 = iVar10 - uVar6;
iVar7 = uVar9 - uVar6;
if ((uVar8 & 0x8000) == 0x0) {
if ((uVar5 & 0x8000) == 0x0) {
loop {
while( true ) {
puVar1 = (param_5 + param_6);
if (param_6 == 0x0) {
param_6 = param_7;
}
param_6 = param_6 + -0x1;
*puVar3 = *puVar1;
puVar3 = puVar3 + 0x1;
if ((uVar9 & 0x8000) != 0x0) break;
uVar9 = uVar9 + iVar7;
puVar3 = puVar3 + iVar2;
iVar4 = iVar4 + -0x1;
if (iVar4 == 0x0) {
return param_6;
}
}
uVar9 = uVar9 + iVar10;
iVar4 = iVar4 + -0x1;
} while (iVar4 != 0x0);
}
else {
loop {
while( true ) {
puVar1 = (param_5 + param_6);
if (param_6 == 0x0) {
param_6 = param_7;
}
param_6 = param_6 + -0x1;
*puVar3 = *puVar1;
puVar3 = puVar3 + -0x1;
if ((uVar9 & 0x8000) != 0x0) break;
uVar9 = uVar9 + iVar7;
puVar3 = puVar3 + iVar2;
iVar4 = iVar4 + -0x1;
if (iVar4 == 0x0) {
return param_6;
}
}
uVar9 = uVar9 + iVar10;
iVar4 = iVar4 + -0x1;
} while (iVar4 != 0x0);
}
}
else {
if ((uVar5 & 0x8000) == 0x0) {
loop {
while( true ) {
puVar1 = (param_5 + param_6);
if (param_6 == 0x0) {
param_6 = param_7;
}
param_6 = param_6 + -0x1;
*puVar3 = *puVar1;
puVar3 = puVar3 + 0x1;
if ((uVar9 & 0x8000) != 0x0) break;
uVar9 = uVar9 + iVar7;
puVar3 = puVar3 + -iVar2;
iVar4 = iVar4 + -0x1;
if (iVar4 == 0x0) {
return param_6;
}
}
uVar9 = uVar9 + iVar10;
iVar4 = iVar4 + -0x1;
} while (iVar4 != 0x0);
}
else {
loop {
while( true ) {
puVar1 = (param_5 + param_6);
if (param_6 == 0x0) {
param_6 = param_7;
}
param_6 = param_6 + -0x1;
*puVar3 = *puVar1;
puVar3 = puVar3 + -0x1;
if ((uVar9 & 0x8000) != 0x0) break;
uVar9 = uVar9 + iVar7;
puVar3 = puVar3 + -iVar2;
iVar4 = iVar4 + -0x1;
if (iVar4 == 0x0) {
return param_6;
}
}
uVar9 = uVar9 + iVar10;
iVar4 = iVar4 + -0x1;
} while (iVar4 != 0x0);
}
}
}
return param_6;
}



u32 *
FUN_004abdf0(param_1: *mut u32,param_2: i32,param_3: u32,param_4: i32,param_5: i32,param_6: i32,param_7: i32,
param_8: u32,param_9: u32,param_10: u32,param_11: i32)

{
let cVar1: u8;
let puVar2: *mut u32;
let mut iVar3: i32;
undefined3 extraout_var;
let mut puVar4: *mut u8;
let mut iVar5: i32;
undefined3 extraout_var_00;
let mut pcVar6: String;

puVar2 = FUN_0049a030(param_1,param_2,param_3,param_4,param_5,param_6,param_7,param_8,param_9,param_10);
$1: &mut String(puVar2 + 0x45) = &PTR_FUN_004c4164;
if ((((*(puVar2 + 0x2d) & 0x20) == 0x0) && (DAT_005b9210 <= *(puVar2 + 0x29))) ||
(DAT_005b982c == 0x0)) {
*(puVar2 + 0x51) = LPCSTR_005b9218;
}
else {
*(puVar2 + 0x51) = DAT_005b982c;
}
*(puVar2 + 0x59) = param_11;
if (param_11 < 0x1) {
cVar1 = FUN_004a06b1(0x57,*(ushort **)(puVar2 + 0x51));
*(puVar2 + 0x59) = *(puVar2 + 0x25) / CONCAT31(extraout_var_00,cVar1) + -0x1;
}
iVar3 = FUN_004a06f6(*(puVar2 + 0x51));
iVar3 = *(puVar2 + 0x29) / iVar3;
*(puVar2 + 0x49) = iVar3;
if (iVar3 == 0x1) {
cVar1 = FUN_004a06b1(0x57,*(ushort **)(puVar2 + 0x51));
iVar3 = CONCAT31(extraout_var,cVar1) * *(puVar2 + 0x59);
if (iVar3 - *(puVar2 + 0x25) != 0x0 && *(puVar2 + 0x25) <= iVar3) {
*(puVar2 + 0x2d) = *(puVar2 + 0x2d) | 0x80;
}
}
puVar4 = FUN_0049c2c9(*(puVar2 + 0x59) + 0x2);
(puVar2 + 0x55) = puVar4;
*puVar4 = 0x0;
*(puVar2 + 0x5d) = 0x0;
*(puVar2 + 0x61) = 0xffffffff;
*(puVar2 + 0x65) = 0x0;
iVar3 = FUN_0049c2c9((*(puVar2 + 0x49) + 0x1) * 0x6);
*(puVar2 + 0x4d) = iVar3;
if ((iVar3 == 0x0) || (*(puVar2 + 0x55) == 0x0)) {
pcVar6 = FUN_00499050(DAT_005b9bd8,0x7d01);
pop_err_msg_box_and_exit_004a02f5(pcVar6);
}
iVar3 = 0x0;
if (0x0 < *(puVar2 + 0x49)) {
iVar5 = 0x0;
loop {
*(*(puVar2 + 0x4d) + iVar5) = *(puVar2 + 0x55);
*(*(puVar2 + 0x4d) + 0x4 + iVar5) = 0x0;
iVar3 = iVar3 + 0x1;
iVar5 = iVar5 + 0x6;
} while (iVar3 < *(puVar2 + 0x49));
}
return puVar2;
}



fn FUN_004abfc0(param_1: i32,param_2: &mut String,param_3: u32,param_4: u32,byte *param_5) -> u32

{
    let uVar1: u8;
    let cVar2: u8;
    byte *pbVar3;
    let piVar4: *mut i32;;
    undefined3 extraout_var;
    let mut puVar5: *mut u8;
    let mut iVar6: i32;
    let mut iVar7: i32;
    let DVar8: u32;
    let mut uVar9: u32;
    let mut iVar10: i32;
    let mut iVar11: i32;
    let mut iVar12: i32;
    let mut pcVar13: String;
    let mut puVar14: *mut u8;
    let mut pcVar15: String;
    let mut iVar16: i32;
    let mut bVar17: bool;
    let mut uVar18: u32;
    let mut uVar19: u32;
    let mut local_44: i32;
    let mut local_40: String;
    let mut local_34: i32;
    let local_30: u8;
    let mut local_24: u32;
    let mut local_20: i32;
    let mut local_1c: u32;
    let mut local_14: i32;

    if (0x401 < param_3) {
        if (param_3 < 0x403) {
            uVar9 = 0xffffffff;
            pcVar15 = *(param_1 + 0x55);
            loop {
                if (uVar9 == 0x0) break;
                uVar9 = uVar9 - 0x1;
                cVar2 = *pcVar15;
                pcVar15 = pcVar15 + 0x1;
            } while (cVar2 != '\0');
            *(param_1 + 0x61) = 0x0;
            *(param_1 + 0x65) = 0x0;
            *(param_1 + 0x5d) = ~uVar9 - 0x1;
            FUN_004acfc0(param_1);
            FUN_004acde0(param_1);
            if ((*(param_1 + 0x2e) & 0x80) == 0x0) {
                FUN_004a75a6();
            }
            *(param_1 + 0x2e) = *(param_1 + 0x2e) | 0x80;
            ((*(param_1 + 0x45) + 0xc))(param_1,param_2,0x404,0x0,0x0);
            return 0x1;
        }
        if (param_3 < 0x417) {
            if (param_3 < 0x404) {
                LAB_004ac438:
                if (*(param_1 + 0xc) == 0x0) {
                    if ((*(param_1 + 0x2e) & 0x80) == 0x0) {
                        return 0x0;
                    }
                }
                else {
                    FUN_0049ae40(param_2);
                    if ((*(param_1 + 0x2e) & 0x80) == 0x0) {
                        return 0x0;
                    }
                }
                *(param_1 + 0x61) = 0xffffffff;
                FUN_004a756b();
                FUN_004acfc0(param_1);
                *(param_1 + 0x2e) = *(param_1 + 0x2e) & 0x7f;
                ((*(param_1 + 0x45) + 0xc))(param_1,param_2,0x404,0x0,0x0);
                return 0x1;
            }
            if (param_3 < 0x405) {
                if (((*(param_1 + 0x2e) & 0x40) == 0x0) && ((param_1 + 0x1c) != '\0')) {
                    ((*(param_1 + 0x45) + 0x10))(param_1);
                    return 0x1;
                }
                return 0x0;
            }
            if (param_3 == 0x405) {
                if ((param_1 + 0x1c) == '\0') {
                    return 0x0;
                }
                iVar7 = FUN_004a06f6(*(param_1 + 0x51));
                local_1c = *(param_1 + 0x31);
                local_24 = *(param_1 + 0x35);
                if ((*(param_1 + 0x2e) & 0x20) != 0x0) {
                    local_1c = FUN_00499f60(local_1c);
                    local_24 = FUN_00499f60(local_24);
                }
                FUN_004953d7();
                FUN_004a756b();
                FUN_004968e7(*(param_1 + 0x1d),*(param_1 + 0x21),*(param_1 + 0x25),*(param_1 + 0x29)
                             ,(char)*(param_1 + 0x35));
                local_20 = 0x0;
                local_14 = *(param_1 + 0x21) + (*(param_1 + 0x29) - *(param_1 + 0x49) * iVar7) / 0x2;
                if (0x0 < *(param_1 + 0x49)) {
                    iVar6 = 0x0;
                    loop {
                        if ((*(param_1 + 0x2d) & 0x1) == 0x0) {
                            local_40 = *(iVar6 + *(param_1 + 0x4d));
                        }
                        else {
                            iVar12 = *(iVar6 + 0x2 + *(param_1 + 0x4d)) >> 0x10;
                            if (0x50 < iVar12) {
                                iVar12 = 0x50;
                            }
                            local_40 = s__________________________________004bfb08 + (0x50 - iVar12);
                        }
                        FUN_00497567(*(param_1 + 0x1d),local_14,local_40,
                                     *(iVar6 + 0x2 + *(param_1 + 0x4d)) >> 0x10,local_1c,-0x1,local_1c,
                                     (param_1 + 0x51),0x0);
                        iVar12 = *(param_1 + 0x61);
                        iVar11 = *(iVar6 + *(param_1 + 0x4d)) - *(param_1 + 0x55);
                        if (-0x1 < iVar12) {
                            iVar10 = *(param_1 + 0x5d);
                            iVar16 = iVar10;
                            if (iVar10 < iVar12) {
                                iVar16 = *(param_1 + 0x61);
                                iVar12 = iVar10;
                            }
                            iVar10 = iVar16 + -0x1;
                            if ((iVar10 + *(param_1 + 0x55)) == '\0') {
                                iVar10 = iVar16 + -0x2;
                            }
                            if (iVar12 <= iVar10) {
                                local_44 = *(*(param_1 + 0x4d) + 0x2 + iVar6) >> 0x10;
                                iVar10 = (iVar10 - iVar12) + 0x1;
                                if (iVar12 < local_44 + iVar11) {
                                    if (iVar12 < iVar11) {
                                        if (iVar11 < iVar12 + iVar10) {
                                            iVar16 = 0x0;
                                            iVar10 = iVar10 - (iVar11 - iVar12);^
                                            // goto LAB_004ac358;
                                        }
                                    }
                                    else {
                                        iVar16 = iVar12 - iVar11;
                                        local_44 = local_44 - iVar16;
                                        if (-0x1 < iVar16) {
                                            LAB_004ac358:
                                            if (local_44 < iVar10) {
                                                iVar10 = local_44;
                                            }
                                            iVar12 = FUN_00497282(local_40,*(ushort **)(param_1 + 0x51),iVar16);
                                            FUN_00497567(iVar12 + *(param_1 + 0x1d),local_14,(local_40 + iVar16),iVar10,local_24,
                                                         local_1c,local_24,(param_1 + 0x51),0x0);
                                        }
                                    }
                                }
                            }
                        }
                        iVar6 = iVar6 + 0x6;
                        local_20 = local_20 + 0x1;
                        local_14 = local_14 + iVar7;
                    } while (local_20 < *(param_1 + 0x49));
                }
                if ((*(param_1 + 0x2e) & 0x80) != 0x0) {
                    FUN_004acde0(param_1);
                }
                FUN_004a75a6();
                FUN_0049536f();
                return 0x1;
            }
        }
        else {
            if (param_3 < 0x418) {
                if (param_4 == 0x0)^ // goto LAB_004ac438;
            }
            else {
                if (0x501 < param_3) {
                    if (param_3 < 0x503) {
                        FUN_004a9a00(*(param_1 + 0x55),param_5,*(param_1 + 0x59));
                        (*(param_1 + 0x55) + *(param_1 + 0x59)) = 0x0;
                        uVar9 = 0xffffffff;
                        pcVar15 = *(param_1 + 0x55);
                        loop {
                            if (uVar9 == 0x0) break;
                            uVar9 = uVar9 - 0x1;
                            cVar2 = *pcVar15;
                            pcVar15 = pcVar15 + 0x1;
                        } while (cVar2 != '\0');
                        if ((~uVar9 - 0x1) < *(param_1 + 0x5d)) {
                            *(param_1 + 0x5d) = ~uVar9 - 0x1;
                        }
                    }
                    else {
                        if (param_3 != 0x504)^ // goto LAB_004acccb;
                        pbVar3 = &DAT_004c3692;
                        if (param_5 != 0x0) {
                            pbVar3 = param_5;
                        }
                        FUN_0049c2e0(*(PCHAR *)(param_1 + 0x55),pbVar3);
                        uVar9 = 0xffffffff;
                        pcVar15 = *(param_1 + 0x55);
                        loop {
                            if (uVar9 == 0x0) break;
                            uVar9 = uVar9 - 0x1;
                            cVar2 = *pcVar15;
                            pcVar15 = pcVar15 + 0x1;
                        } while (cVar2 != '\0');
                        if ((~uVar9 - 0x1) < *(param_1 + 0x5d)) {
                            *(param_1 + 0x5d) = ~uVar9 - 0x1;
                        }
                    }^
                    // goto switchD_004ac999_caseD_49;
                }
                if (param_3 == 0x501) {
                    uVar9 = 0xffffffff;
                    pcVar15 = *(param_1 + 0x55);
                    loop {
                        if (uVar9 == 0x0) break;
                        uVar9 = uVar9 - 0x1;
                        cVar2 = *pcVar15;
                        pcVar15 = pcVar15 + 0x1;
                    } while (cVar2 != '\0');
                    if ((~uVar9 - 0x1) < param_4) {
                        param_4 = ~uVar9;
                    }
                    FUN_004a9a00(param_5,*(param_1 + 0x55),param_4);
                    param_5[param_4] = 0x0;
                    return 0x1;
                }
            }
        }
        LAB_004acccb:
            DVar8 = FUN_0049a270(param_1,param_2,param_3,param_4,param_5);
        return DVar8;
    }
    if (0x200 < param_3) {
        if (((param_3 < 0x202) || (param_3 < 0x203)) || (param_3 < 0x204)) {
            LAB_004ac4ad:
                iVar7 = param_4 - *(param_1 + 0x1d);
            if ((param_3 == 0x201) || (param_3 == 0x203)) {
                if (*(param_1 + 0x25) <= iVar7) {
                    return 0x0;
                }
                if (iVar7 < 0x0) {
                    return 0x0;
                }
                FUN_0049ae20(param_2);
            }
            local_34 = FUN_004a06f6(*(param_1 + 0x51));
            local_34 = (param_5 -
                ((*(param_1 + 0x29) - *(param_1 + 0x49) * local_34) / 0x2 + *(param_1 + 0x21))) /
                local_34;
            if (*(param_1 + 0x49) <= local_34) {
                local_34 = *(param_1 + 0x49) + -0x1;
            }
            if (local_34 < 0x0) {
                local_34 = 0x0;
            }
            iVar12 = 0x0;
            iVar6 = 0x0;
            loop {
                iVar11 = iVar6;
                piVar4 = (*(param_1 + 0x4d) + local_34 * 0x6);
                if (*(piVar4 + 0x2) >> 0x10 <= iVar11 + 0x1) break;
                cVar2 = FUN_004a06b1((iVar11 + *piVar4),*(ushort **)(param_1 + 0x51));
                iVar12 = iVar12 + CONCAT31(extraout_var,cVar2);
                iVar6 = iVar11 + 0x1;
            } while (iVar12 <= iVar7);
            iVar6 = *(param_1 + 0x5d);
            if (*(param_1 + 0xc) == 0x0) {
                return 0x0;
            }
            if (iVar7 < 0x0) {
                *(param_1 + 0x5d) = iVar6 + -0x1;
            }
            else {
                if (iVar7 < *(param_1 + 0x25)) {
                    *(param_1 + 0x5d) =
                        (*(local_34 * 0x6 + *(param_1 + 0x4d)) - *(param_1 + 0x55)) + iVar11 +
                            *(param_1 + 0x65);
                }
                else {
                    *(param_1 + 0x5d) = iVar6 + 0x1;
                }
            }
            uVar9 = 0xffffffff;
            pcVar15 = *(param_1 + 0x55);
            loop {
                if (uVar9 == 0x0) break;
                uVar9 = uVar9 - 0x1;
                cVar2 = *pcVar15;
                pcVar15 = pcVar15 + 0x1;
            } while (cVar2 != '\0');
            if ((~uVar9 - 0x1) < *(param_1 + 0x5d)) {
                *(param_1 + 0x5d) = ~uVar9 - 0x1;
            }
            if (*(param_1 + 0x5d) < 0x0) {
                *(param_1 + 0x5d) = 0x0;
            }
            if ((param_3 == 0x201) || (param_3 == 0x203)) {
                if (*(param_1 + 0x61) != *(param_1 + 0x5d)) {
                    iVar6 = -0x1;
                }
                *(param_1 + 0x61) = *(param_1 + 0x5d);
            }
            if (param_3 == 0x202) {
                FUN_0049ae40(param_2);
            }
            if (iVar6 != *(param_1 + 0x5d)) {
                FUN_004acfc0(param_1);
            }
            return 0x1;
        }
        if (param_3 == 0x401) {
            uVar9 = 0xffffffff;
            pcVar15 = *(param_1 + 0x55);
            loop {
                if (uVar9 == 0x0) break;
                uVar9 = uVar9 - 0x1;
                cVar2 = *pcVar15;
                pcVar15 = pcVar15 + 0x1;
            } while (cVar2 != '\0');
            *(param_1 + 0x61) = 0xffffffff;
            *(param_1 + 0x65) = 0x0;
            *(param_1 + 0x5d) = ~uVar9 - 0x1;
            return 0x1;
        }^
        // goto LAB_004acccb;
    }
    if (param_3 < 0x100)^ // goto LAB_004acccb;
    if (0x100 < param_3) {
        if (param_3 == 0x200)^ // goto LAB_004ac4ad;^
        // goto LAB_004acccb;
    }
    if (param_5 != 0x0) {
        if ((param_5 < 0x20) &&
            ((param_5 != 0xd || (*(param_1 + 0x49) <= *(param_1 + 0x69) + 0x1)))) {
            if (param_5 == &DAT_00000008) {
                if (*(param_1 + 0x5d) != 0x0) {
                    pcVar13 = (*(param_1 + 0x55) + *(param_1 + 0x5d));
                    pcVar15 = pcVar13 + -0x1;
                    loop {
                        cVar2 = *pcVar13;
                        *pcVar15 = cVar2;
                        if (cVar2 == '\0') break;
                        cVar2 = pcVar13[0x1];
                        pcVar13 = pcVar13 + 0x2;
                        pcVar15[0x1] = cVar2;
                        pcVar15 = pcVar15 + 0x2;
                    } while (cVar2 != '\0');
                    *(param_1 + 0x5d) = *(param_1 + 0x5d) + -0x1;
                }
                *(param_1 + 0x61) = 0xffffffff;
            }
            else {
                if (param_5 != 0xd) {
                    return 0x0;
                }
                if (((param_1 + 0x4) != 0x0) && (*(param_1 + 0x49) < 0x2)) {
                    FUN_0049a770((param_1 + 0x4),0x407,*(param_1 + 0x3d),
                                 *(param_1 + 0x55));
                    return 0x1;
                }
            }
        }
        else {
            if ((param_5 == 0x7e) ||
                ((((local_30 = (char)param_5, (*(param_1 + 0x2d) & 0x2) != 0x0 &&
                ((*(&DAT_004bf9c1 + (byte)(local_30 + 0x1)) >> 0x18 & 0xc0U) != 0x0)) ||
                (((*(param_1 + 0x2d) & 0x4) != 0x0 && (((&DAT_004bf9c4)[(byte)(local_30 + 0x1)] & 0x20) != 0x0)))) ||
                (((*(param_1 + 0x2d) & 0x8) != 0x0 &&
                    ((*(&DAT_004bf9c1 + (byte)(local_30 + 0x1)) >> 0x18 & 0xe0U) == 0x0)))))) {
            return 0x1;
            }
            if (((*(param_1 + 0x2d) & 0x10) != 0x0) &&
                ((*(&DAT_004bf9c1 + (byte)(local_30 + 0x1)) >> 0x18 & 0xe0U) == 0x0)) {
                pcVar15 = &DAT_004c3695;
                loop {
                    pcVar13 = pcVar15;
                    if (*pcVar15 == local_30)^ // goto LAB_004ac854;
                    if (*pcVar15 == '\0') break;
                    pcVar13 = pcVar15 + 0x1;
                    if (*pcVar13 == local_30)^ // goto LAB_004ac854;
                    pcVar15 = pcVar15 + 0x2;
                } while (*pcVar13 != '\0');
                pcVar13 = 0x0;
                LAB_004ac854:
                if (pcVar13 == 0x0) {
                    return 0x1;
                }
            }
            if (-0x1 < *(param_1 + 0x61)) {
                FUN_004ad210(param_1);
            }
            *(param_1 + 0x61) = 0xffffffff;
            if (*(param_1 + 0x59) <= *(param_1 + 0x5d)) {
                if (*(param_1 + 0x4) == 0x0) {
                    return 0x1;
                }
                if (0x1 < *(param_1 + 0x49)) {
                    return 0x1;
                }
                uVar9 = 0xffffffff;
                pcVar15 = *(param_1 + 0x55);
                loop {
                    if (uVar9 == 0x0) break;
                    uVar9 = uVar9 - 0x1;
                    cVar2 = *pcVar15;
                    pcVar15 = pcVar15 + 0x1;
                } while (cVar2 != '\0');
                puVar14 = (*(param_1 + 0x55) + (~uVar9 - 0x1));
                cVar2 = puVar14[-0x1];
                for (; (cVar2 != ' ' &&
                ((param_1 + 0x55) <= puVar14 &&
                puVar14 != (param_1 + 0x55))); puVar14 = puVar14 + -0x1) {
                    cVar2 = puVar14[-0x2];
                }
                if ((puVar14 == (param_1 + 0x55)) || (param_5 == 0x20)) {
                    uVar9 = 0xffffffff;
                    pcVar15 = *(param_1 + 0x55);
                    loop {
                        if (uVar9 == 0x0) break;
                        uVar9 = uVar9 - 0x1;
                        cVar2 = *pcVar15;
                        pcVar15 = pcVar15 + 0x1;
                    } while (cVar2 != '\0');
                    puVar14 = (*(param_1 + 0x55) + (~uVar9 - 0x1));
                }
                puVar5 = FUN_0049a770((param_1 + 0x4),0x503,*(param_1 + 0x3d),puVar14);
                if (puVar5 == 0x0) {
                    return 0x1;
                }
                *puVar14 = 0x0;
                FUN_004acfc0(param_1);
                if (param_5 == 0x20) {
                    return 0x1;
                }
                // LPARAM lParam for SendMessageA
                // WPARAM wParam for SendMessageA
                // UINT Msg for SendMessageA
                // HWND hWnd for SendMessageA
                SendMessageA(*(HWND *)(param_1 + 0x4),0x100,param_4,(LPARAM)param_5);
                // LPARAM lParam for SendMessageA
                // WPARAM wParam for SendMessageA
                // UINT Msg for SendMessageA
                // HWND hWnd for SendMessageA
                SendMessageA(*(HWND *)(param_1 + 0x4),0x101,param_4,(LPARAM)param_5);
                return 0x1;
            }
            if ((*(param_1 + 0x59) + -0x1 < *(param_1 + 0x5d)) ||
                (pcVar15 = (*(param_1 + 0x55) + *(param_1 + 0x5d)), *pcVar15 != '\0')) {
                if ((DAT_004bfb04 == 0x0) || (iVar7 = *(param_1 + 0x5d), *(param_1 + 0x59) + -0x1 <= iVar7)) {
                    (*(param_1 + 0x55) + *(param_1 + 0x5d)) = local_30;
                }
                else {
                    iVar6 = *(param_1 + 0x59) - iVar7;
                    pcVar13 = (*(param_1 + 0x55) + iVar7);
                    pcVar15 = pcVar13 + iVar6 + -0x1;
                    while (pcVar15 != pcVar13) {
                        *pcVar15 = pcVar15[-0x1];
                        pcVar15 = pcVar15 + -0x1;
                    }
                    *pcVar13 = local_30;
                    pcVar13[iVar6] = '\0';
                }
                *(param_1 + 0x5d) = *(param_1 + 0x5d) + 0x1;
                while (iVar7 = FUN_004ad160(param_1), iVar7 == 0x0) {
                    uVar9 = 0xffffffff;
                    pcVar15 = *(param_1 + 0x55);
                    loop {
                        if (uVar9 == 0x0) break;
                        uVar9 = uVar9 - 0x1;
                        cVar2 = *pcVar15;
                        pcVar15 = pcVar15 + 0x1;
                    } while (cVar2 != '\0');
                    iVar7 = ~uVar9 - 0x2;
                    if (*(param_1 + 0x5d) < iVar7) {
                        (*(param_1 + 0x55) + iVar7) = 0x0;
                    }
                    else {
                        *(param_1 + 0x5d) = *(param_1 + 0x5d) + -0x1;
                        (*(param_1 + 0x55) + iVar7) = 0x0;
                    }
                }
            }
            else {
                *pcVar15 = local_30;
                iVar7 = *(param_1 + 0x5d) + 0x1;
                *(param_1 + 0x5d) = iVar7;
                (*(param_1 + 0x55) + iVar7) = 0x0;
                iVar7 = FUN_004ad160(param_1);
                if (iVar7 == 0x0) {
                    iVar7 = *(param_1 + 0x5d) + -0x1;
                    *(param_1 + 0x5d) = iVar7;
                    (*(param_1 + 0x55) + iVar7) = 0x0;
                }
            }
        }
        puVar14 = (param_1 + 0x4);
        if (puVar14 == 0x0)^ // goto switchD_004ac999_caseD_49;
        uVar19 = *(param_1 + 0x55);
        uVar18 = *(param_1 + 0x3d);
        LAB_004ac070:
            FUN_0049a770(puVar14,0x406,uVar18,uVar19);^
        // goto switchD_004ac999_caseD_49;
    }
    if ((param_4 & 0x900) == 0x0) {
        if ((param_4 & 0x7f) != 0x53) {
            *(param_1 + 0x61) = 0xffffffff;
        }
    }
    else {
        if (*(param_1 + 0x61) < 0x0) {
            *(param_1 + 0x61) = *(param_1 + 0x5d);
        }
    }
    switch(param_4 & 0x7f) {
    case 0x47:
    if ((*(param_1 + 0x49) < 0x2) || ((param_4 & 0x1200) != 0x0)) {
        *(param_1 + 0x5d) = 0x0;
        break;
    }
    pcVar13 = *(*(param_1 + 0x69) * 0x6 + *(param_1 + 0x4d));^
    // goto LAB_004acaf1;
    case 0x48:
    if ((0x1 < *(param_1 + 0x49)) && (iVar7 = *(param_1 + 0x69), iVar7 != 0x0)) {
        iVar6 = *(param_1 + 0x5d) -
            (*(*(param_1 + 0x4d) + iVar7 * 0x6) -
                *(*(param_1 + 0x4d) + (iVar7 + -0x1) * 0x6));
        *(param_1 + 0x5d) = iVar6;
        piVar4 = (*(param_1 + 0x4d) + (*(param_1 + 0x69) + -0x1) * 0x6);
        iVar7 = (*piVar4 - *(param_1 + 0x55)) + (*(piVar4 + 0x2) >> 0x10);
        if (iVar7 < iVar6) {
            *(param_1 + 0x5d) = iVar7;
        }
    }
    break;
    case 0x4b:
        iVar7 = *(param_1 + 0x5d);
    if (iVar7 != 0x0) {
        if ((param_4 & 0x1200) == 0x0) {
            *(param_1 + 0x5d) = iVar7 + -0x1;
        }
        else {
            if (iVar7 != 0x0) {
                iVar6 = *(param_1 + 0x55) + iVar7;
                loop {
                    if ((iVar6 + -0x1) != ' ') break;
                    iVar6 = iVar6 + -0x1;
                    iVar7 = iVar7 + -0x1;
                } while (iVar7 != 0x0);
            }
            uVar1 = (iVar7 + *(param_1 + 0x55));
            (iVar7 + *(param_1 + 0x55)) = 0x0;
            pcVar15 = FUN_004ae9a0(*(param_1 + 0x55),' ');
            if (pcVar15 == 0x0) {
                *(param_1 + 0x5d) = 0x0;
            }
            else {
                *(param_1 + 0x5d) = pcVar15 + (0x1 - *(param_1 + 0x55));
            }
            (iVar7 + *(param_1 + 0x55)) = uVar1;
        }
    }
    break;
    case 0x4d:
        pcVar15 = (*(param_1 + 0x55) + *(param_1 + 0x5d));
    if ((*pcVar15 == '\0') || (*(param_1 + 0x59) < *(param_1 + 0x5d))) break;
    if ((param_4 & 0x1200) == 0x0) {
        *(param_1 + 0x5d) = *(param_1 + 0x5d) + 0x1;
        break;
    }
    loop {
        pcVar13 = pcVar15;
        if (*pcVar15 == ' ')^ // goto LAB_004acb8f;
        if (*pcVar15 == '\0') break;
        pcVar13 = pcVar15 + 0x1;
        if (*pcVar13 == ' ')^ // goto LAB_004acb8f;
        pcVar15 = pcVar15 + 0x2;
    } while (*pcVar13 != '\0');
    pcVar13 = 0x0;
    LAB_004acb8f:
    if (pcVar13 == 0x0)^ // goto LAB_004acb36;
    for (; *pcVar13 == ' '; pcVar13 = pcVar13 + 0x1) {
    }
    LAB_004acaf1:
        *(param_1 + 0x5d) = pcVar13 - *(param_1 + 0x55);
    break;
    case 0x4f:
    if ((0x1 < *(param_1 + 0x49)) && ((param_4 & 0x1200) == 0x0)) {
        piVar4 = (*(param_1 + 0x4d) + *(param_1 + 0x69) * 0x6);
        *(param_1 + 0x5d) = (*piVar4 - *(param_1 + 0x55)) + (*(piVar4 + 0x2) >> 0x10) + -0x1;
        break;
    }
    LAB_004acb36:
        uVar9 = 0xffffffff;
    pcVar15 = *(param_1 + 0x55);
    loop {
        if (uVar9 == 0x0) break;
        uVar9 = uVar9 - 0x1;
        cVar2 = *pcVar15;
        pcVar15 = pcVar15 + 0x1;
    } while (cVar2 != '\0');
    *(param_1 + 0x5d) = ~uVar9 - 0x1;
    break;
    case 0x50:
    if ((0x1 < *(param_1 + 0x49)) && (*(param_1 + 0x69) + 0x1 < *(param_1 + 0x49))) {
        piVar4 = (*(param_1 + 0x4d) + (*(param_1 + 0x69) + 0x1) * 0x6);
        if ((piVar4 + 0x1) != 0x0) {
            iVar6 = *(param_1 + 0x5d) +
                (*piVar4 - *(*(param_1 + 0x4d) + *(param_1 + 0x69) * 0x6));
            *(param_1 + 0x5d) = iVar6;
            piVar4 = (*(param_1 + 0x4d) + (*(param_1 + 0x69) + 0x1) * 0x6);
            iVar7 = (*(piVar4 + 0x2) >> 0x10) + (*piVar4 - *(param_1 + 0x55));
            if (iVar7 < iVar6) {
                *(param_1 + 0x5d) = iVar7;
            }
        }
    }
    break;
    case 0x52:
        bVar17 = DAT_004bfb04 == 0x0;
    DAT_004bfb04 = bVar17;
    FUN_004a76f9(!bVar17);
    break;
    case 0x53:
    if ((*(param_1 + 0x61) < 0x0) || (*(param_1 + 0x61) == *(param_1 + 0x5d))) {
        pcVar15 = (*(param_1 + 0x55) + *(param_1 + 0x5d));
        if (*pcVar15 != '\0') {
            pcVar13 = pcVar15 + 0x1;
            loop {
                cVar2 = *pcVar13;
                *pcVar15 = cVar2;
                if (cVar2 == '\0') break;
                cVar2 = pcVar13[0x1];
                pcVar13 = pcVar13 + 0x2;
                pcVar15[0x1] = cVar2;
                pcVar15 = pcVar15 + 0x2;
            } while (cVar2 != '\0');
        }
    }
    else {
        FUN_004ad210(param_1);
        *(param_1 + 0x61) = 0xffffffff;
    }
    puVar14 = (param_1 + 0x4);
    if (puVar14 == 0x0) break;
    uVar19 = *(param_1 + 0x55);
    uVar18 = *(param_1 + 0x3d);^
    // goto LAB_004ac070;
}
    switchD_004ac999_caseD_49:
        FUN_004acfc0(param_1);
    return 0x1;
}



fn FUN_004acd60(param_1: i32)

{
    let mut uVar1: u32;
    let mut uVar2: u32;

    uVar1 = DAT_005b981c;
    uVar2 = DAT_004bf924;
    FUN_004953d7();
    if ((*(param_1 + 0x2e) & 0x80) != 0x0) {
        uVar1 = DAT_004bf924;
        uVar2 = DAT_005b981c;
    }
    FUN_0049e640(*(param_1 + 0x1d),*(param_1 + 0x21),*(param_1 + 0x25),*(param_1 + 0x29),
                 (char)uVar2,(char)uVar1,(char)DAT_004bf928,0x1);
    FUN_0049e640(*(param_1 + 0x1d),*(param_1 + 0x21),*(param_1 + 0x25),*(param_1 + 0x29),
                 (char)uVar1,(char)uVar2,(char)DAT_004bf928,0x2);
    FUN_0049536f();
    return;
}



fn FUN_004acde0(param_1: i32)

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let cVar3: u8;
    let mut iVar4: i32;
    let mut uVar5: u32;
    undefined3 extraout_var;
    let mut uVar6: u32;
    let piVar7: *mut i32;;
    let mut iVar8: i32;
    let mut iVar9: i32;
    let mut iVar10: i32;
    let mut local_18: i32;

    iVar4 = FUN_004a06f6(*(param_1 + 0x51));
    if ((*(param_1 + 0x2d) & 0x80) == 0x0) {
        iVar9 = 0x0;
        if (0x0 < *(param_1 + 0x49)) {
            piVar7 = *(i32 **)(param_1 + 0x4d);
            loop {
                if (((piVar7 + 0x1) == 0x0) || (*(param_1 + 0x5d) < *piVar7 - *(param_1 + 0x55))) break;
                iVar9 = iVar9 + 0x1;
                piVar7 = (piVar7 + 0x6);
            } while (iVar9 < *(param_1 + 0x49));
        }
        if (iVar9 != 0x0) {
            iVar9 = iVar9 + -0x1;
        }
        iVar10 = *(iVar9 * 0x6 + *(param_1 + 0x4d));
    }
    else {
        iVar10 = *(param_1 + 0x55) + *(param_1 + 0x65);
        iVar9 = 0x0;
    }
    iVar1 = *(param_1 + 0x55);
    iVar2 = *(param_1 + 0x5d);
    iVar8 = DAT_005b9210;
    if (DAT_004bfb04 != 0x0) {
        iVar8 = ((DAT_005b9210 + (DAT_005b9210 >> 0x1f) * -0x4) - ((DAT_005b9210 >> 0x1f) << 0x1 < 0x0)) >> 0x2;
    }
    uVar5 = SEXT14((*(param_1 + 0x55) + *(param_1 + 0x5d)));
    if ((uVar5 == 0x0) || (uVar5 == 0xd)) {
        uVar5 = 0x48;
    }
    cVar3 = FUN_004a06b1(uVar5,*(ushort **)(param_1 + 0x51));
    local_18 = FUN_00497282(iVar10,*(ushort **)(param_1 + 0x51),iVar2 - (iVar10 - iVar1));
    local_18 = *(param_1 + 0x1d) + local_18;
    *(param_1 + 0x69) = iVar9;
    iVar9 = *(param_1 + 0x21) + (*(param_1 + 0x29) - *(param_1 + 0x49) * iVar4) / 0x2 +
        ((iVar4 * iVar9 + iVar4) - iVar8);
    if (local_18 + CONCAT31(extraout_var,cVar3) <= *(param_1 + 0x1d) + *(param_1 + 0x25)) {
        if (*(param_1 + 0x5d) == 0x0)^ // goto LAB_004acedd;
        if (((*(param_1 + 0x5d) + *(param_1 + 0x55)) != '\0') ||
            ((*(param_1 + 0x5d) + -0x1 + *(param_1 + 0x55)) != '\r'))^ // goto LAB_004acedd;
    }
    local_18 = *(param_1 + 0x1d);
    iVar9 = iVar9 + iVar4;
    *(param_1 + 0x69) = *(param_1 + 0x69) + 0x1;
    LAB_004acedd:
    if (iVar9 + iVar8 <= *(param_1 + 0x21) + *(param_1 + 0x29)) {
        FUN_004a75c2(iVar8,CONCAT31(extraout_var,cVar3));
        if (*(param_1 + 0x5d) < *(param_1 + 0x61)) {
            uVar6 = *(param_1 + 0x35);
        }
        else {
            uVar6 = *(param_1 + 0x31);
        }
        FUN_004a7670(uVar6);
        FUN_004a763f(local_18,iVar9);
    }
    return;
}



fn FUN_004acfc0(param_1: i32)

{
    short *psVar1;
    let cVar2: u8;
    undefined3 extraout_var;
    undefined3 extraout_var_00;
    undefined3 extraout_var_01;
    let piVar3: *mut i32;;
    let mut uVar4: u32;
    let mut iVar5: i32;
    let mut pcVar6: String;
    ushort *puVar7;
    let mut iVar8: i32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    if ((*(param_1 + 0x2d) & 0x80) != 0x0) {
        iVar5 = *(param_1 + 0x65);
        if ((iVar5 < *(param_1 + 0x5d)) &&
            (iVar5 = FUN_00497282(*(param_1 + 0x55) + iVar5,*(ushort **)(param_1 + 0x51),
                                  (*(param_1 + 0x5d) - iVar5) + 0x1), *(param_1 + 0x25) < iVar5)) {
            loop {
                cVar2 = FUN_004a06b1((*(param_1 + 0x55) + *(param_1 + 0x65)),
                                     *(ushort **)(param_1 + 0x51));
                iVar5 = iVar5 - CONCAT31(extraout_var,cVar2);
                *(param_1 + 0x65) = *(param_1 + 0x65) + 0x1;
            } while (*(param_1 + 0x25) < iVar5);
        }
        while ((*(param_1 + 0x5d) < *(param_1 + 0x65) + 0x2 && (*(param_1 + 0x65) != 0x0))) {
            *(param_1 + 0x65) = *(param_1 + 0x65) + -0x1;
        }
    }
    piVar3 = &local_20;
    iVar5 = *(param_1 + 0x49);
    if (iVar5 < 0x2) {
        pcVar6 = (*(param_1 + 0x55) + *(param_1 + 0x65));
        **(char ***)(param_1 + 0x4d) = pcVar6;
        *(*(param_1 + 0x4d) + 0x4) = 0x0;
        iVar5 = 0x0;
        if (-0x1 < *(param_1 + 0x25)) {
            loop {
                if (*pcVar6 == '\0') break;
                cVar2 = FUN_004a06b1(*pcVar6,*(ushort **)(param_1 + 0x51));
                iVar5 = iVar5 + CONCAT31(extraout_var_00,cVar2);
                psVar1 = (short *)(*(param_1 + 0x4d) + 0x4);
                *psVar1 = *psVar1 + 0x1;
                pcVar6 = pcVar6 + 0x1;
            } while (iVar5 <= *(param_1 + 0x25));
        }
    }
    else {
        iVar8 = 0x1;
        puVar7 = *(ushort **)(param_1 + 0x51);
        cVar2 = FUN_004a06b1(0x57,puVar7);
        local_1c = FUN_004a3840(*(param_1 + 0x55),*(char ***)(param_1 + 0x4d),
                                *(param_1 + 0x25) - CONCAT31(extraout_var_01,cVar2),iVar5,piVar3,puVar7,iVar8);
        uVar4 = 0xffffffff;
        pcVar6 = *(param_1 + 0x55);
        loop {
            if (uVar4 == 0x0) break;
            uVar4 = uVar4 - 0x1;
            cVar2 = *pcVar6;
            pcVar6 = pcVar6 + 0x1;
        } while (cVar2 != '\0');
        local_18 = *(param_1 + 0x55) + (~uVar4 - 0x1);
        local_14 = 0x0;
        if (0x0 < *(param_1 + 0x49)) {
            iVar5 = 0x0;
            loop {
                if (local_14 < local_1c) {
                    while (piVar3 = (*(param_1 + 0x4d) + iVar5),
                           (*piVar3 + (*(piVar3 + 0x2) >> 0x10)) == '\r') {
                        (piVar3 + 0x1) = (piVar3 + 0x1) + 0x1;
                    }
                }
                else {
                    *(iVar5 + *(param_1 + 0x4d)) = local_18;
                    *(iVar5 + 0x4 + *(param_1 + 0x4d)) = 0x0;
                }
                iVar5 = iVar5 + 0x6;
                local_14 = local_14 + 0x1;
            } while (local_14 < *(param_1 + 0x49));
        }
    }
    ((*(param_1 + 0x45) + 0xc))(param_1,*(param_1 + 0x8),0x405,0x0,0x0);
    return;
}



fn FUN_004ad160(param_1: i32) -> u32

{
    let cVar1: u8;
    let mut iVar2: i32;
    undefined3 extraout_var;
    let mut pcVar3: String;
    let mut pcVar4: String;
    let piVar5: *mut i32;;
    ushort *puVar6;
    let mut iVar7: i32;
    let mut local_14: i32;

    if ((*(param_1 + 0x2d) & 0x80) == 0x0) {
        iVar7 = 0x1;
        puVar6 = *(ushort **)(param_1 + 0x51);
        piVar5 = &local_14;
        iVar2 = *(param_1 + 0x49) + 0x1;
        cVar1 = FUN_004a06b1(0x57,puVar6);
        iVar7 = FUN_004a3840(*(param_1 + 0x55),*(char ***)(param_1 + 0x4d),
                             *(param_1 + 0x25) - CONCAT31(extraout_var,cVar1),iVar2,piVar5,puVar6,iVar7);
        iVar2 = *(param_1 + 0x49);
        if (iVar2 < iVar7) {
            return 0x0;
        }
        if (iVar7 != iVar2) {
            return 0x1;
        }
        pcVar4 = *(*(param_1 + 0x4d) + (iVar2 + -0x1) * 0x6);
        loop {
            pcVar3 = pcVar4;
            if (*pcVar4 == '\r')^ // goto LAB_004ad1fb;
            if (*pcVar4 == '\0') break;
            pcVar3 = pcVar4 + 0x1;
            if (*pcVar3 == '\r')^ // goto LAB_004ad1fb;
            pcVar4 = pcVar4 + 0x2;
        } while (*pcVar3 != '\0');
        pcVar3 = 0x0;
        LAB_004ad1fb:
        if (pcVar3 != 0x0) {
            return 0x0;
        }
    }
    return 0x1;
}



fn FUN_004ad210(param_1: i32)

{
    let cVar1: u8;
    let mut iVar2: i32;
    let mut iVar4: i32;
    let mut pcVar5: String;
    let mut pcVar6: String;
    let mut iVar3: i32;

    iVar2 = *(param_1 + 0x61);
    iVar4 = *(param_1 + 0x5d);
    iVar3 = iVar2;
    if (iVar2 <= iVar4) {
        if (iVar4 <= iVar2) {
            return;
        }
        iVar3 = *(param_1 + 0x5d);
        iVar4 = iVar2;
    }
    iVar2 = iVar3 + -0x1;
    if ((*(param_1 + 0x55) + iVar2) == '\0') {
        iVar2 = iVar3 + -0x2;
    }
    pcVar5 = (iVar2 + *(param_1 + 0x55) + 0x1);
    pcVar6 = (*(param_1 + 0x55) + iVar4);
    loop {
    cVar1 = *pcVar5;
    *pcVar6 = cVar1;
    if (cVar1 == '\0') break;
    cVar1 = pcVar5[0x1];
    pcVar5 = pcVar5 + 0x2;
    pcVar6[0x1] = cVar1;
    pcVar6 = pcVar6 + 0x2;
} while (cVar1 != '\0');
    *(param_1 + 0x61) = 0xffffffff;
    *(param_1 + 0x5d) = iVar4;
    return;
}



u32 *
FUN_004ad360(param_1: *mut u32,param_2: i32,param_3: u32,param_4: i32,param_5: i32,param_6: i32,param_7: i32,
param_8: u32,param_9: u32,param_10: u32,param_11: u32)

{
let puVar1: *mut u32;
let puVar2: *mut u32;
let mut iVar3: i32;
let mut uVar4: u32;
let mut unaff_EBP: u32;

puVar1 = FUN_0049a030(param_1,param_2,param_3,param_4,param_5,param_6,param_7,param_8,param_9,param_10);
$1: &mut String(puVar1 + 0x45) = &PTR_LAB_004c41a4;
if (param_11 == 0x0) {
*(puVar1 + 0x51) = 0x0;
}
else {
*puVar1 = param_11;
uVar4 = FUN_0049ed90(param_11);
*(puVar1 + 0x51) = uVar4;
}
*(puVar1 + 0x49) = 0x0;
*(puVar1 + 0x4d) = 0xffffffff;
if (*(puVar1 + 0x29) < 0x40) {
*(puVar1 + 0x29) = 0x40;
}
*(puVar1 + 0x5d) = DAT_005b9210;
iVar3 = DAT_005b9214;
*(puVar1 + 0x55) = *(puVar1 + 0x29) / *(puVar1 + 0x5d);
*(puVar1 + 0x59) = (*(puVar1 + 0x25) + -0x14) / iVar3;
if ((*(puVar1 + 0x2d) & 0x4) == 0x0) {
*(puVar1 + 0x29) = *(puVar1 + 0x55) * *(puVar1 + 0x5d);
}
puVar2 = FUN_004a2831(0x69);
if (puVar2 != 0x0) {
iVar3 = *(puVar1 + 0x29) / *(puVar1 + 0x5d);
uVar4 = FUN_004ae3d0(puVar1);
puVar2 = FUN_004a7c00(puVar2,puVar1,0x15,*(puVar1 + 0x1d) + *(puVar1 + 0x25) + -0x12,
*(puVar1 + 0x21) + 0x2,0x10,*(puVar1 + 0x29) + -0x4,0x0,
*(puVar1 + 0x31),*(puVar1 + 0x35),uVar4,iVar3,unaff_EBP)
;
}
(puVar1 + 0x61) = puVar2;
((*(puVar2 + 0x45) + 0xc))(puVar2,puVar2,0x401,0x0,0x0);
return puVar1;
}



fn FUN_004ad4b0(param_1: &mut String,byte param_2) -> String

{
    let mut iVar1: i32;
    LPCSTR *ppCVar2;
    let piVar3: *mut i32;;

    if ((param_2 & 0x4) != 0x0) {
        piVar3 = FUN_00498dce(param_1,&DAT_004c4070);
        FUN_00498df5(piVar3);
        return param_1;
    }
    iVar1 = *(param_1 + 0x61);
    $1: &mut String(param_1 + 0x45) = &PTR_LAB_004c41a4;
    if (iVar1 != 0x0) {
        ((*(iVar1 + 0x45) + 0x8))(iVar1,0x2);
    }
    ppCVar2 = FUN_0049a1c0(param_1,0x1);
    if ((param_2 & 0x2) == 0x0) {
        return ppCVar2;
    }
    FUN_0049af50(ppCVar2);
    return ppCVar2;
}



fn FUN_004ad550(param_1: *mut *mut u32,param_2: *mut u8,param_3: u32,param_4: u32,param_5: *mut u32) -> u32

{
    let cVar1: u8;
    let mut bVar2: bool;
    let mut iVar3: i32;
    let puVar4: *mut u32;
    let mut iVar5: i32;
    let mut pcVar6: String;
    byte *pbVar7;
    let mut uVar8: u32;
    let mut uVar9: u32;
    let mut iVar10: i32;
    let uVar11: u8;
    let mut uVar12: u32;
    let mut local_18: i32;
    let mut local_14: u32;

    local_18 = *(param_1 + 0x4d);
    if (local_18 < 0x0) {
        local_18 = 0x0;
    }
    if (0x40e < param_3) {
        if (param_3 < 0x410) {
            *(param_1 + 0x2e) = *(param_1 + 0x2e) & 0xcf;
            FUN_0049a770((param_1 + 0x61),param_3,0x1,param_5);
            if (param_4 != 0x0) {
                return 0x1;
            }
            ((*(param_1 + 0x45) + 0xc))(param_1,param_2,0x405,0x0,0x0);
            return 0x1;
        }
        if (0x502 < param_3) {
            if (param_3 < 0x504) {
                if ((*param_1 != 0x0) && (param_4 != 0xffff)) {
                    FUN_0049ed30(*param_1);
                }
                *param_1 = param_5;
                if (param_5 == 0x0) {
                    *(param_1 + 0x51) = 0x0;
                }
                else {
                    uVar9 = FUN_0049ed90(param_5);
                    *(param_1 + 0x51) = uVar9;
                }
                iVar3 = *(param_1 + 0x61);
                ((*(iVar3 + 0x45) + 0xc))(iVar3,iVar3,0x502,0x0,0x0);
                uVar12 = *(param_1 + 0x55);
                iVar3 = *(param_1 + 0x61);
                iVar10 = *(iVar3 + 0x45);
                uVar9 = FUN_004ae3d0(param_1);
                ((iVar10 + 0xc))(iVar3,*(param_1 + 0x61),0x503,uVar9,uVar12);
                *(param_1 + 0x4d) = 0xffffffff;
                *(param_1 + 0x49) = 0x0;
                ((*(param_1 + 0x45) + 0xc))(param_1,param_2,0x405,0x0,0x0);
                return 0x1;
            }
            if (param_3 < 0x507) {
                if (0x504 < param_3) {
                    if (0x505 < param_3) {
                        if (*param_1 == 0x0) {
                            return 0x0;
                        }
                        uVar9 = FUN_0049ed90(*param_1);
                        return uVar9;
                    }
                    if (*param_1 == 0x0) {
                        puVar4 = FUN_0049ea90();
                        *param_1 = puVar4;
                    }
                    uVar9 = 0xffffffff;
                    puVar4 = param_5;
                    loop {
                        if (uVar9 == 0x0) break;
                        uVar9 = uVar9 - 0x1;
                        cVar1 = puVar4;
                        puVar4 = (puVar4 + 0x1);
                    } while (cVar1 != '\0');
                    uVar9 = FUN_0049eae0(*param_1,param_5,~uVar9);
                    uVar8 = FUN_0049ed90(*param_1);
                    *(param_1 + 0x51) = uVar8;
                    iVar5 = *(param_1 + 0x29) / *(param_1 + 0x5d);
                    iVar3 = *(param_1 + 0x61);
                    iVar10 = *(iVar3 + 0x45);
                    uVar8 = FUN_004ae3d0(param_1);
                    ((iVar10 + 0xc))(iVar3,*(param_1 + 0x61),0x503,uVar8,iVar5);
                    return uVar9;
                }
                if ((param_4 < *(param_1 + 0x51)) &&
                    (pcVar6 = FUN_0049ecf0(*param_1,param_4), pcVar6 != 0x0)) {
                    loop {
                        cVar1 = *pcVar6;
                        param_5 = cVar1;
                        if (cVar1 == '\0') {
                            return 0x1;
                        }
                        cVar1 = pcVar6[0x1];
                        pcVar6 = pcVar6 + 0x2;
                        (param_5 + 0x1) = cVar1;
                        param_5 = (param_5 + 0x2);
                    } while (cVar1 != '\0');
                    return 0x1;
                }
            }
            else {
                if (param_3 < 0x508) {
                    if (*param_1 != 0x0) {
                        FUN_0049ed30(*param_1);
                        *param_1 = 0x0;
                    }
                    iVar3 = *(param_1 + 0x61);
                    *(param_1 + 0x51) = 0x0;
                    ((*(iVar3 + 0x45) + 0xc))(iVar3,iVar3,0x502,0x0,0x0);
                    uVar12 = *(param_1 + 0x55);
                    iVar3 = *(param_1 + 0x61);
                    iVar10 = *(iVar3 + 0x45);
                    uVar9 = FUN_004ae3d0(param_1);
                    ((iVar10 + 0xc))(iVar3,*(param_1 + 0x61),0x503,uVar9,uVar12);
                    *(param_1 + 0x4d) = 0xffffffff;
                    *(param_1 + 0x49) = 0x0;
                    ((*(param_1 + 0x45) + 0xc))(param_1,param_2,0x405,0x0,0x0);
                    return 0x1;
                }
                if (0x508 < param_3) {
                    if (param_3 < 0x50a) {
                        iVar3 = *(param_1 + 0x29) / param_5;
                        (param_1 + 0x5d) = param_5;
                        *(param_1 + 0x55) = iVar3;
                        *(param_1 + 0x29) = iVar3 * *(param_1 + 0x5d);
                        ((*(param_1 + 0x45) + 0xc))(param_1,param_2,0x405,0x0,0x0);
                        return 0x1;
                    }
                    if (param_3 == 0x510) {
                        if ((param_4 < *(param_1 + 0x51)) &&
                            (uVar9 = FUN_0049ecf0(*param_1,param_4), uVar9 != 0x0)) {
                            return uVar9;
                        }
                        return 0x0;
                    }^
                    // goto LAB_004adee1;
                }
                if (param_4 < *(param_1 + 0x51)) {
                    if (param_4 < 0x0) {
                        param_4 = 0x0;
                    }
                    uVar9 = 0xffffffff;
                    puVar4 = param_5;
                    loop {
                        if (uVar9 == 0x0) break;
                        uVar9 = uVar9 - 0x1;
                        cVar1 = puVar4;
                        puVar4 = (puVar4 + 0x1);
                    } while (cVar1 != '\0');
                    pbVar7 = FUN_0049ecf0(*param_1,param_4);
                    loop {
                        iVar3 = FUN_004a9800(pbVar7,param_5,~uVar9 - 0x1);
                        if (iVar3 == 0x0) {
                            FUN_004ae180(param_1,param_4 - local_18,0x1);
                            return 0x1;
                        }
                        param_4 = param_4 + 0x1;
                        pbVar7 = FUN_0049ec90(*param_1);
                    } while (pbVar7 != 0x0);
                    return 0x0;
                }
            }
            return 0x0;
        }
        if (param_3 < 0x416) {
            if (param_3 < 0x411) {
                *(param_1 + 0x2e) = *(param_1 + 0x2e) | 0x30;
                FUN_0049a770((param_1 + 0x61),param_3,0x1,param_5);
                if (param_4 != 0x0) {
                    return 0x1;
                }
                ((*(param_1 + 0x45) + 0xc))(param_1,param_2,0x405,0x0,0x0);
                return 0x1;
            }
            if (param_3 == 0x415) {
                if (((*(param_1 + 0x61) + 0x1c) == '\0') ||
                    ((*(*(param_1 + 0x61) + 0x2e) & 0x4) != 0x0)) {
                    bVar2 = false;
                }
                else {
                    bVar2 = true;
                }
                ((*(*(param_1 + 0x61) + 0x45) + 0x4))(*(param_1 + 0x61));
                FUN_0049a770((param_1 + 0x61),0x415,param_4 + *(param_1 + 0x25) + -0x12,
                             param_5 + 0x2);
                if (bVar2) {
                    (***(code ***)(*(param_1 + 0x61) + 0x45))(*(param_1 + 0x61),0x0);
                }
                uVar9 = FUN_0049a270(param_1,param_2,0x415,param_4,param_5);
                return uVar9;
            }^
            // goto LAB_004adee1;
        }
        if (param_3 < 0x417) {
            return 0x0;
        }
        if (0x500 < param_3) {
            if (param_3 < 0x502) {
                return *(param_1 + 0x4d);
            }
            if (param_4 < *(param_1 + 0x51)) {
                FUN_004ae180(param_1,param_4 - local_18,0x1);
                return 0x1;
            }
            return 0x0;
        }
        if ((param_3 != 0x417) || (param_4 != 0x0))^ // goto LAB_004adee1;
        LAB_004adb9f:
        if (param_1[0x3] != 0x0) {
            FUN_0049ae40(param_2);
        }
        iVar3 = *(param_1 + 0x61);
        ((*(iVar3 + 0x45) + 0xc))(iVar3,iVar3,param_3,0x0,0x0);
        if ((*(param_1 + 0x2e) & 0x80) != 0x0) {
            *(param_1 + 0x2e) = *(param_1 + 0x2e) & 0x7f;
            ((*(param_1 + 0x45) + 0xc))(param_1,param_2,0x404,0x0,0x0);
            return 0x1;
        }
        switchD_004ade71_caseD_4a:
        return 0x0;
    }
    if (0x205 < param_3) {
        if (param_3 < 0x207) {
            LAB_004adcd4:
            if ((*(param_1 + 0x21) <= param_5) &&
                (param_5 < *(param_1 + 0x21) + *(param_1 + 0x29))) {
                if (param_1[0x1] != 0x0) {
                    FUN_0049a770(param_1[0x1],0x413,*(param_1 + 0x3d),
                                 (param_5 - *(param_1 + 0x21)) / *(param_1 + 0x5d) +
                                     *(param_1 + 0x49));
                    return 0x1;
                }
            }
            LAB_004adee1:
                uVar9 = FUN_0049a270(param_1,param_2,param_3,param_4,param_5);
            return uVar9;
        }
        if (param_3 < 0x403) {
            if (0x400 < param_3) {
                if (param_3 < 0x402) {
                    if (*param_1 == 0x0) {
                        *(param_1 + 0x51) = 0x0;
                    }
                    else {
                        uVar9 = FUN_0049ed90(*param_1);
                        *(param_1 + 0x51) = uVar9;
                    }
                    uVar12 = *(param_1 + 0x55);
                    *(param_1 + 0x4d) = 0xffffffff;
                    iVar3 = *(param_1 + 0x61);
                    *(param_1 + 0x49) = 0x0;
                    iVar10 = *(iVar3 + 0x45);
                    uVar9 = FUN_004ae3d0(param_1);
                    ((iVar10 + 0xc))(iVar3,*(param_1 + 0x61),0x503,uVar9,uVar12);
                    iVar3 = *(param_1 + 0x61);
                    ((*(iVar3 + 0x45) + 0xc))(iVar3,iVar3,0x401,0x0,0x0);
                    return 0x1;
                }
                iVar3 = *(param_1 + 0x61);
                ((*(iVar3 + 0x45) + 0xc))(iVar3,iVar3,param_3,0x0,0x0);
                *(param_1 + 0x2e) = *(param_1 + 0x2e) | 0x80;
                ((*(param_1 + 0x45) + 0xc))(param_1,param_2,0x404,0x0,0x0);
                return 0x1;
            }^
            // goto LAB_004adee1;
        }
        if (0x403 < param_3) {
            if (param_3 < 0x405) {
                if (((*(param_1 + 0x2e) & 0x40) == 0x0) && ((param_1 + 0x7) != '\0')) {
                    ((*(param_1 + 0x45) + 0x10))(param_1);
                    return 0x1;
                }
            }
            else {
                if (0x405 < param_3) {
                    if (param_3 == 0x406) {
                        if (param_4 != 0x15) {
                            return 0x1;
                        }
                        if (param_5 == (param_1 + 0x4d)) {
                            return 0x1;
                        }
                        FUN_004ae180(param_1,param_5 - local_18,0x1);
                        return 0x1;
                    }^
                    // goto LAB_004adee1;
                }
                if ((param_1 + 0x7) != '\0') {
                    local_14 = *(param_1 + 0x31);
                    uVar9 = *(param_1 + 0x35);
                    if ((*(param_1 + 0x2e) & 0x20) != 0x0) {
                        local_14 = FUN_00499f60(local_14);
                        uVar9 = FUN_00499f60(uVar9);
                    }
                    FUN_004953d7();
                    iVar10 = *(param_1 + 0x25) + -0x14;
                    uVar11 = uVar9;
                    FUN_004968e7(*(param_1 + 0x1d) + iVar10,*(param_1 + 0x21),0x3,
                                 *(param_1 + 0x29),uVar11);
                    iVar3 = *(param_1 + 0x61);
                    ((*(iVar3 + 0x45) + 0xc))(iVar3,iVar3,0x404,0x0,0x0);
                    iVar3 = *(param_1 + 0x61);
                    ((*(iVar3 + 0x45) + 0xc))(iVar3,iVar3,0x405,0x0,0x0);
                    if (*param_1 == 0x0) {
                        FUN_004968e7(*(param_1 + 0x1d),*(param_1 + 0x21),iVar10,
                                     *(param_1 + 0x29),uVar11);
                        FUN_0049536f();
                        return 0x1;
                    }
                    iVar3 = 0x0;
                    uVar8 = FUN_0049ecf0(*param_1,*(param_1 + 0x49));
                    if (0x0 < *(param_1 + 0x55)) {
                        loop {
                            if (uVar8 == 0x0) {
                                FUN_004968e7(*(param_1 + 0x1d),
                                             *(param_1 + 0x5d) * iVar3 + *(param_1 + 0x21),iVar10,
                                             *(param_1 + 0x5d),uVar11);
                            }
                            else {
                                if ((*(param_1 + 0x2d) & 0x1) == 0x0) {
                                    if ((*(param_1 + 0x49) + iVar3 == *(param_1 + 0x4d)) &&
                                        ((*(param_1 + 0x2d) & 0x2) == 0x0)) {
                                        FUN_00497567(*(param_1 + 0x1d),
                                                     *(param_1 + 0x5d) * iVar3 + *(param_1 + 0x21),uVar8,iVar10,
                                                     uVar9,local_14,0x0,LPCSTR_005b9218,0x10);
                                    }
                                    else {
                                        FUN_00497567(*(param_1 + 0x1d),
                                                     *(param_1 + 0x5d) * iVar3 + *(param_1 + 0x21),uVar8,iVar10,
                                                     local_14,uVar9,0x0,LPCSTR_005b9218,0x10);
                                    }
                                }
                                else {
                                    FUN_004ae340(param_1,iVar3,uVar8);
                                }
                            }
                            iVar3 = iVar3 + 0x1;
                            uVar8 = FUN_0049ec90(*param_1);
                        } while (iVar3 < *(param_1 + 0x55));
                    }
                    FUN_0049536f();
                    return 0x1;
                }
            }
            return 0x0;
        }^
        // goto LAB_004adb9f;
    }
    if (0x200 < param_3) {
        if (0x201 < param_3) {
            if (param_3 < 0x203) {
                if (param_1[0x3] != 0x0) {
                    FUN_0049ae40(param_2);
                }
                iVar3 = *(param_1 + 0x61);
                uVar9 = ((*(iVar3 + 0x45) + 0xc))(iVar3,iVar3,param_3,param_4,param_5);
                return uVar9;
            }
            if (0x203 < param_3) {
                if (param_3 != 0x204)^ // goto LAB_004adee1;^
                // goto LAB_004adcd4;
            }
        }
        if (*(*(param_1 + 0x61) + 0x1d) + -0x3 <= param_4) {
            if (param_1[0x3] != 0x0) {
                FUN_0049ae40(param_2);
            }
            iVar3 = *(param_1 + 0x61);
            uVar9 = ((*(iVar3 + 0x45) + 0xc))(iVar3,iVar3,param_3,param_4,param_5);
            return uVar9;
        }
        if (param_1[0x3] == 0x0) {
            FUN_0049ae20(param_2);
        }
        LAB_004add6f:
        if (param_1[0x3] == 0x0) {
            iVar3 = *(param_1 + 0x61);
            if (*(iVar3 + 0x1d) < param_4) {
                uVar9 = ((*(iVar3 + 0x45) + 0xc))(iVar3,iVar3,param_3,param_4,param_5);
                return uVar9;
            }
        }
        else {
            iVar3 = (param_5 - *(param_1 + 0x21)) / *(param_1 + 0x5d);
            if (param_5 < *(param_1 + 0x21)) {
                iVar3 = -0x1;
            }
            if (*(param_1 + 0x55) <= iVar3) {
                iVar3 = *(param_1 + 0x55);
            }
            iVar3 = iVar3 - (local_18 - *(param_1 + 0x49));
            if ((iVar3 != 0x0) || (*(param_1 + 0x4d) < 0x0)) {
                FUN_004ae180(param_1,iVar3,0x1);
                return 0x1;
            }
            if ((param_3 == 0x203) && (param_1[0x1] != 0x0)) {
                FUN_0049a770(param_1[0x1],0x407,*(param_1 + 0x3d),*(param_1 + 0x4d))
                ;
                return 0x1;
            }
        }
        return 0x1;
    }
    if (param_3 < 0x100)^ // goto LAB_004adee1;
    if (0x100 < param_3) {
        if (param_3 != 0x200)^ // goto LAB_004adee1;^
        // goto LAB_004add6f;
    }
    iVar3 = 0x1;
    if (param_5 != 0x0) {
        if (param_5 != 0xd) {
            return 0x0;
        }
        if (param_1[0x1] == 0x0) {
            return 0x1;
        }
        FUN_0049a770(param_1[0x1],0x407,*(param_1 + 0x3d),
                     *(param_1 + 0x4d));
        return 0x1;
    }
    if (((param_4 & 0x1000) != 0x0) || ((param_4 & 0x200) != 0x0)) {
        iVar3 = *(param_1 + 0x55);
    }
    switch(param_4 & 0x7f) {
    case 0x47:
        iVar3 = *(param_1 + 0x4d);
    break;
    case 0x48:
    break;
    case 0x49:
        iVar3 = *(param_1 + 0x55);
    break;
    default:^
        // goto switchD_004ade71_caseD_4a;
    case 0x4f:
        iVar3 = *(param_1 + 0x51) - *(param_1 + 0x4d);
    case 0x50:^
        // goto switchD_004ade71_caseD_50;
    case 0x51:
        iVar3 = *(param_1 + 0x55);^
    // goto switchD_004ade71_caseD_50;
}
    iVar3 = -iVar3;
    switchD_004ade71_caseD_50:
        FUN_004ae180(param_1,iVar3,0x0);
    return 0x1;
}



fn FUN_004ae0b0(param_1: *mut i32,param_2: i32)

{
    let mut iVar1: i32;
    let mut puVar2: *mut u8;
    let mut iVar3: i32;
    let mut uVar4: u32;
    let mut iVar5: i32;

    iVar3 = param_2 - *(param_1 + 0x49);
    if ((-0x1 < iVar3) && (iVar3 < *(param_1 + 0x55))) {
        FUN_004953d7();
        puVar2 = FUN_0049ecf0(*param_1,param_2);
        if (puVar2 == 0x0) {
            puVar2 = &DAT_004c36a1;
        }
        else {
            if ((*(param_1 + 0x2d) & 0x1) != 0x0) {
                FUN_004ae340(param_1,iVar3,puVar2);
                FUN_0049536f();
                return;
            }
        }
        if ((param_2 == *(param_1 + 0x4d)) && ((*(param_1 + 0x2d) & 0x2) == 0x0)) {
            iVar5 = *(param_1 + 0x31);
            uVar4 = *(param_1 + 0x35);
            iVar1 = *(param_1 + 0x25);
            iVar3 = iVar3 * *(param_1 + 0x5d) + *(param_1 + 0x21);
        }
        else {
            iVar5 = *(param_1 + 0x35);
            uVar4 = *(param_1 + 0x31);
            iVar1 = *(param_1 + 0x25);
            iVar3 = *(param_1 + 0x5d) * iVar3 + *(param_1 + 0x21);
        }
        FUN_00497567(*(param_1 + 0x1d),iVar3,puVar2,iVar1 + -0x14,uVar4,iVar5,0x0,
                     LPCSTR_005b9218,0x10);
        FUN_0049536f();
        return;
    }
    return;
}



fn FUN_004ae180(param_1: *mut i32,param_2: i32,param_3: i32)

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut iVar3: i32;

    iVar2 = *(param_1 + 0x4d);
    if ((*(param_1 + 0x2d) & 0x2) == 0x0) {
        iVar3 = iVar2;
        if (iVar2 < 0x0) {
            iVar3 = 0x0;
        }
        if (param_3 != 0x0) {
            iVar2 = iVar3;
        }
        iVar2 = iVar2 + param_2;
        if (iVar2 < 0x0) {
            iVar2 = 0x0;
        }
        if (*(param_1 + 0x51) <= iVar2) {
            iVar2 = *(param_1 + 0x51) + -0x1;
        }
        if (iVar2 == *(param_1 + 0x4d)) {
            return;
        }
        iVar1 = iVar2 - *(param_1 + 0x49);
        *(param_1 + 0x4d) = iVar2;
        FUN_004953d7();
        if ((-0x1 < iVar1) && (iVar1 < *(param_1 + 0x55))) {
            if (iVar3 == -0x1) {
                iVar3 = 0x0;
            }
            if ((param_1 + 0x7) != '\0') {
                FUN_004ae0b0(param_1,iVar3);
                FUN_004ae0b0(param_1,iVar2);
            }^
            // goto LAB_004ae1e0;
        }
        iVar2 = iVar2 - (iVar3 - *(param_1 + 0x49));
        iVar3 = *(param_1 + 0x51) - *(param_1 + 0x55);
        if (iVar3 < iVar2) {
            iVar2 = iVar3;
        }
        if (iVar2 < 0x0) {
            iVar2 = 0x0;
        }
        iVar3 = param_1[0x2];
        iVar1 = *(param_1 + 0x45);
        *(param_1 + 0x49) = iVar2;
    }
    else {
        iVar2 = *(param_1 + 0x49) + param_2;
        iVar3 = *(param_1 + 0x51) - *(param_1 + 0x55);
        if (iVar3 < iVar2) {
            iVar2 = iVar3;
        }
        if (iVar2 < 0x0) {
            iVar2 = 0x0;
        }
        if (iVar2 == *(param_1 + 0x49)) {
            return;
        }
        *(param_1 + 0x49) = iVar2;
        *(param_1 + 0x4d) = iVar2;
        FUN_004953d7();
        iVar3 = param_1[0x2];
        iVar1 = *(param_1 + 0x45);
    }
    ((iVar1 + 0xc))(param_1,iVar3,0x405,0x0,0x0);
    LAB_004ae1e0:
        iVar2 = *(param_1 + 0x4d);
    if (iVar2 < 0x0) {
        iVar2 = 0x0;
    }
    ((*(*(param_1 + 0x61) + 0x45) + 0xc))
        (*(param_1 + 0x61),*(param_1 + 0x61),0x502,iVar2,0x0);
    if (param_1[0x1] != 0x0) {
        FUN_0049a770(param_1[0x1],0x406,*(param_1 + 0x3d),
                     *(param_1 + 0x4d));
    }
    FUN_0049536f();
    return;
}



fn FUN_004ae340(param_1: i32,param_2: i32,param_3: u32)

{
    let bVar1: u8;
    let mut local_28: u32;
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: u32;
    let mut local_10: i32;

    local_28 = *(param_1 + 0x1d);
    local_10 = param_2 + *(param_1 + 0x49);
    local_24 = *(param_1 + 0x21) + *(param_1 + 0x5d) * param_2;
    local_20 = *(param_1 + 0x25) + -0x14;
    local_1c = *(param_1 + 0x5d);
    bVar1 = local_10 == *(param_1 + 0x4d);
    if ((*(param_1 + 0x2e) & 0x20) != 0x0) {
        bVar1 = bVar1 | 0x2;
    }
    local_18 = bVar1;
    local_14 = param_3;
    if ((param_1 + 0x4) == 0x0) {
        return;
    }
    FUN_0049a770((param_1 + 0x4),0x412,*(param_1 + 0x3d),&local_28);
    return;
}



fn FUN_004ae3d0(param_1: *mut u32) -> u32

{
    let mut uVar1: u32;

    if (*param_1 == 0x0) {
        uVar1 = 0x0;
    }
    else {
        uVar1 = FUN_0049ed90(*param_1);
    }
    if (((*(param_1 + 0x2d) & 0x2) != 0x0) &&
        (uVar1 = uVar1 - (*(param_1 + 0x55) + -0x1), uVar1 < 0x1)) {
        uVar1 = 0x1;
    }
    return uVar1;
}



fn FUN_004ae406(param_1: *mut u32,param_2: *mut i32)

{
    let local_18: u8;
    let local_14: *mut u32;

    local_14 = param_1 + 0xa0;
    while (param_1 < local_14) {
        if (((param_1 + 0x1) == param_1) || ((*param_1 & 0xc0) == 0xc0)) {
            for (local_18 = 0x1;
                ((param_1 == (local_18 + param_1) &&
                ((local_18 + param_1) < local_14)) && (local_18 < 0x3f)); local_18 = local_18 + 0x1)
            {
            }
            local_18 = local_18 | 0xc0;
            FUN_004a7160(&local_18,0x1,0x1,param_2);
            FUN_004a7160(param_1,0x1,0x1,param_2);
            param_1 = (param_1 + (local_18 & 0x3f));
        }
        else {
            FUN_004a7160(param_1,0x1,0x1,param_2);
            param_1 = (param_1 + 0x1);
        }
    }
    return;
}



fn FUN_004ae4d3(param_1: &mut String,param_2: i32)

{
    let piVar1: *mut i32;;
    let puVar2: *mut u32;
    let local_b0: *mut u32;
    let local_a8: u8;
    let mut local_a4: i32;
    let local_a0: u8;
    let mut local_9c: u32;
    let mut local_98: u32;
    let mut local_94: u32;
    let mut local_90: u32;
    let local_5c: u16;
    let mut local_5a: u32;
    let mut local_56: u32;
    let local_1c: *mut i32;;
    let local_18: *mut u32;
    let local_14: *mut u32;

    local_18 = FUN_0049c2c9(0x300);
    local_14 = FUN_0049c2c9(0x4b000);
    if ((local_14 != 0x0) && (local_18 != 0x0)) {
        local_9c = 0x801050a;
        local_98 = 0x0;
        local_94 = 0x1df027f;
        local_90 = 0x1e00280;
        local_5c = 0x100;
        local_5a = 0x280;
        local_56 = 0x0;
        FUN_004953d7();
        FUN_00496c1f(0x0,0x0,local_14,0x280,0x1e0);
        local_1c = FUN_0049c4bd(param_1,&DAT_004c36aa);
        if (local_1c != 0x0) {
            FUN_004a7160(&local_9c,0x80,0x1,local_1c);
            for (local_a4 = 0x0; piVar1 = local_1c, local_a4 < 0x1e0; local_a4 = local_a4 + 0x1) {
                local_b0 = local_14 + local_a4 * 0xa0;
                puVar2 = local_b0 + 0xa0;
                while (local_b0 < puVar2) {
                    if (((local_b0 + 0x1) == local_b0) || ((*local_b0 & 0xc0) == 0xc0)) {
                        for (local_a8 = 0x1;
                            ((local_b0 == (local_a8 + local_b0) &&
                            ((local_a8 + local_b0) < puVar2)) && (local_a8 < 0x3f));
                            local_a8 = local_a8 + 0x1) {
                        }
                        local_a8 = local_a8 | 0xc0;
                        FUN_004a7160(&local_a8,0x1,0x1,piVar1);
                        FUN_004a7160(local_b0,0x1,0x1,piVar1);
                        local_b0 = (local_b0 + (local_a8 & 0x3f));
                    }
                    else {
                        FUN_004a7160(local_b0,0x1,0x1,piVar1);
                        local_b0 = (local_b0 + 0x1);
                    }
                }
            }
            for (local_a4 = 0x0; local_a4 < 0x100; local_a4 = local_a4 + 0x1) {
                (local_a4 * 0x3 + local_18) = (local_a4 * 0x3 + param_2) << 0x2;
                (local_18 + local_a4 * 0x3 + 0x1) = (local_a4 * 0x3 + param_2 + 0x1) << 0x2;
                (local_18 + local_a4 * 0x3 + 0x2) = (local_a4 * 0x3 + param_2 + 0x2) << 0x2;
            }
            local_a0 = 0xc;
            FUN_004a7160(&local_a0,0x1,0x1,local_1c);
            FUN_004a7160(local_18,0x300,0x1,local_1c);
            FUN_0049ca40(local_1c);
        }
        FUN_0049536f();
    }
    if (local_14 != 0x0) {
        FUN_0049af50(local_14);
    }
    if (local_18 != 0x0) {
        FUN_0049af50(local_18);
    }
    return;
}



bool  FUN_004ae7e3(param_1: u32)

{
let mut bVar1: bool;

bVar1 = DAT_005b9db8 < 0x8;
if (bVar1) {
*(&DAT_005b9dbc + DAT_005b9db8 * 0x4) = param_1;
DAT_005b9db8 = DAT_005b9db8 + 0x1;
}
return bVar1;
}



fn FUN_004ae82a(param_1: i32) -> u32

{
    let mut iVar1: i32;
    let mut local_14: u32;

    local_14 = 0x0;
    while( true ) {
        if (DAT_005b9db8 <= local_14) {
            return local_14;
        }
        if (*(&DAT_005b9dbc + local_14 * 0x4) == param_1) break;
        local_14 = local_14 + 0x1;
    }
    DAT_005b9db8 = DAT_005b9db8 - 0x1;
    iVar1 = DAT_005b9db8 - local_14;
    *(&DAT_005b9dbc + local_14 * 0x4) = *(&DAT_005b9dc0 + local_14 * 0x4);
    (&DAT_005b9dc0)[local_14 * 0x4] = (&DAT_005b9dc4)[local_14 * 0x4];
    return iVar1 * 0x4;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_004ae8a8()

{
    let mut local_14: u32;

    _DAT_005b9db0 = timeGetTime();
    for (local_14 = 0x0; local_14 < DAT_005b9db8; local_14 = local_14 + 0x1) {
        ((&DAT_005b9dbc + local_14 * 0x4))();
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_004ae8ec()

{
    let DVar1: u32;
    let mut local_14: u32;

    DVar1 = timeGetTime();
    if (0x23 < DVar1 - _DAT_005b9db0) {
        DAT_005b9db4 = 0x1;
        _DAT_005b9db0 = timeGetTime();
        for (local_14 = 0x0; local_14 < DAT_005b9db8; local_14 = local_14 + 0x1) {
            ((&DAT_005b9dbc + local_14 * 0x4))();
        }
        DAT_005b9db4 = 0x0;
    }
    return;
}



fn FUN_004ae978(LPSTR param_1,LPSTR param_2,param_3: *mut *mut u32)

{
    let mut iVar1: i32;

    iVar1 = FUN_004b42c0(param_1,param_2,param_3,&LAB_004ae960);
    param_1[iVar1] = '\0';
    return;
}



char *  FUN_004ae9a0(param_1: &mut String,param_2: u8)

{
let cVar1: u8;
let mut pcVar2: String;

pcVar2 = 0x0;
loop {
if (param_2 == *param_1) {
pcVar2 = param_1;
}
cVar1 = *param_1;
param_1 = param_1 + 0x1;
} while (cVar1 != '\0');
return pcVar2;
}



fn FUN_004ae9c0(byte *param_1,param_2: u32)

{
    FUN_004ae9e4(param_1,param_2,0x0);
    return;
}



fn FUN_004ae9e4(byte *param_1,param_2: u32,param_3: u32)

{
    let local_8: *mut u32;

    local_8 = &stack0x00000010;
    file_fn_004aea10(param_1,param_2,param_3,&local_8);
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32  file_fn_004aea10(byte *file_name,param_2: u32,param_3: u32,param_4: *mut *mut u32)

{
let puVar1: *mut u32;
let mut iVar2: i32;
let mut uVar3: u32;
let dwCreationDisposition: u32;
HANDLE file_handle;
let creation_disposition: u32;
let mut uVar4: u32;
let DVar5: u32;
let mut uVar6: u32;
_SECURITY_ATTRIBUTES security_attribs;
let share_mode: u32;
let desired_access: u32;
let mut local_14: u32;

iVar2 = FUN_004b3f90();
if (iVar2 != 0x0) {
FUN_004b1740(0xb);
return 0xffffffff;
}
uVar6 = param_2 & 0x7;
FUN_004b5290(uVar6,&desired_access,&local_14);
DVar5 = 0x80;
FUN_004b52d4(param_3 | uVar6,&share_mode);
security_attribs.nLength = 0xc;
security_attribs.lpSecurityDescriptor = (LPVOID)0x0;
security_attribs.bInheritHandle = ZEXT14((param_2 & 0x80) == 0x0);
if ((_DAT_004c004c == (code *)0x0) || (iVar2 = FUN_004a2f10(file_name,&DAT_004c36b0), iVar2 != 0x0)) {
if ((param_2 & 0x20) == 0x0) {
if ((param_2 & 0x40) == 0x0) {
// LAB_004aeb4b:
dwCreationDisposition = 0x3;
}
else {
dwCreationDisposition = 0x5;
}
}
else {
puVar1 = *param_4;
*param_4 = puVar1 + 0x1;
local_14 = *puVar1;
*param_4 = 0x0;
local_14 = local_14 & ~DAT_004c01a4;
if (((local_14 & 0x100) != 0x0) && ((local_14 & 0x80) == 0x0)) {
DVar5 = 0x1;
}
if ((param_2 & 0x400) == 0x0) {
if ((param_2 & 0x40) == 0x0) {
creation_disposition = 0x4;^
// goto LAB_004aeb4b;
}
creation_disposition = 0x2;
dwCreationDisposition = 0x1;
}
else {
creation_disposition = 0x1;
dwCreationDisposition = 0x1;
}
}
// HANDLE hTemplateFile for CreateFileA
// DWORD dwFlagsAndAttributes for CreateFileA
// DWORD dwCreationDisposition for CreateFileA
// LPSECURITY_ATTRIBUTES lpSecurityAttributes for CreateFileA
// DWORD dwShareMode for CreateFileA
// DWORD dwDesiredAccess for CreateFileA
// LPCSTR lpFileName for CreateFileA
file_handle = CreateFileA(file_name,desired_access,share_mode,(LPSECURITY_ATTRIBUTES)&security_attribs,
dwCreationDisposition,DVar5,(HANDLE)0x0);
if (file_handle == (HANDLE)0xffffffff) {
if ((param_2 & 0x20) != 0x0) {
// HANDLE hTemplateFile for CreateFileA
// DWORD dwFlagsAndAttributes for CreateFileA
// DWORD dwCreationDisposition for CreateFileA
// LPSECURITY_ATTRIBUTES lpSecurityAttributes for CreateFileA
// DWORD dwShareMode for CreateFileA
// DWORD dwDesiredAccess for CreateFileA
// LPCSTR lpFileName for CreateFileA
file_handle = CreateFileA(file_name,desired_access,share_mode,(LPSECURITY_ATTRIBUTES)0x0,
creation_disposition,DVar5,(HANDLE)0x0);
}
if (file_handle == (HANDLE)0xffffffff) {
uVar6 = handle_err_fn_004b12fc();
return uVar6;
}
}
uVar3 = (PTR_FUN_004bfb80)(file_handle);
uVar4 = 0x0;
if (DAT_004bffe8 <= uVar3) {
// HANDLE hObject for CloseHandle
CloseHandle(file_handle);
FUN_004b1740(0x5);
return 0xffffffff;
}
DVar5 = get_file_type_fn_004b5340(uVar3);
if (DVar5 != 0x0) {
uVar4 = 0x2000;
}
}
else {
file_handle = create_event_fn_004b41d8();
uVar3 = (PTR_FUN_004bfb80)(file_handle);
uVar4 = 0x2000;
(*_DAT_004c004c)(0x0,uVar3,0xffffffff);
}
if (uVar6 == 0x2) {
uVar4 = uVar4 | 0x3;
}
else {
if (uVar6 == 0x0) {
uVar4 = uVar4 | 0x1;
}
else {
if (uVar6 == 0x1) {
uVar4 = uVar4 | 0x2;
}
}
}
if ((param_2 & 0x10) != 0x0) {
uVar4 = uVar4 | 0x80;
}
if ((param_2 & 0x300) == 0x0) {
if (_DAT_004bfdcc != 0x200)^ // goto LAB_004aec3d;
}
else {
if ((param_2 & 0x200) == 0x0)^ // goto LAB_004aec3d;
}
uVar4 = uVar4 | 0x40;
// LAB_004aec3d:
FUN_004b1a88(uVar3,uVar4);
return uVar3;
}



fn FUN_004aec6c() -> u32

{
    return DAT_005b9dec;
}



fn FUN_004aec74()

{
    return;
}



fn FUN_004aec78(param_1: i32)

{
    FUN_004b3fe0(param_1);
    return;
}



fn FUN_004aec88(param_1: i32) -> u32

{
    let mut uVar1: u32;

    uVar1 = FUN_004b4144(param_1);
    return uVar1;
}



fn FUN_004aec98()

{
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

i32 *  FUN_004aec9c(param_1: i32,param_2: *mut i32,HMODULE module_handle)

{
let cVar1: u8;
let sVar2: i16;
let piVar3: *mut i32;;
let os_version: u32;
LPSTR command_line;
let puVar4: *mut u32;
LPWSTR command_line_w;

DAT_005b9de0 = param_1;
piVar3 = FUN_004b56cc(param_2);
DAT_005b9dec = piVar3;
if (piVar3 == 0x0) {
if (param_1 == 0x0) {
// UINT uExitCode for ExitProcess
// WARNING: Subroutine does not return
ExitProcess(0x1);
}
}
else {
get_std_handles_004b4170();
LPCH_env_strings_004c0169 = GetEnvironmentStrings();
DAT_005bac04 = 0x0;
os_version = GetVersion();
USHORT_maj_vers_004c016f._0_1_ = os_version;
USHORT_min_vers_004c0171 = (ushort)(os_version >> 0x10);
_DAT_004c0173 = USHORT_min_vers_004c0171;
DAT_004c017b = (os_version & 0xffff) >> 0x8;
DAT_004c0177 = os_version & 0xff;
// DWORD nSize for GetModuleFileNameA
USHORT_maj_vers_004c016f._1_1_ = ((os_version & 0xffff) >> 0x8);
// LPSTR lpFilename for GetModuleFileNameA
_DAT_004c017f = DAT_004c0177 << 0x8 | DAT_004c017b;
// HMODULE hModule for GetModuleFileNameA
GetModuleFileNameA((HMODULE)0x0,(LPSTR)&lpFilename_005b9df0,0x104);
DAT_004c0130 = &lpFilename_005b9df0;
get_mod_file_name_fn_004b59f0((HMODULE)0x0,&WCHAR____005b9ef4,0x208);
LPWSTR_004c013c = &WCHAR____005b9ef4;
command_line = GetCommandLineA();
_DAT_005b9de4 = FUN_004b5aa0(command_line);
DAT_004c012c = _DAT_005b9de4;
if (_DAT_005b9de4 == '\"') {
cVar1 = (_DAT_005b9de4 + 0x1);
puVar4 = _DAT_005b9de4;
while ((DAT_004c012c = (puVar4 + 0x1), cVar1 != '\"' && (DAT_004c012c != '\0'))) {
cVar1 = (puVar4 + 0x2);
puVar4 = DAT_004c012c;
}
if (DAT_004c012c != '\0') {
DAT_004c012c = (puVar4 + 0x2);
}
}
else {
for (; (((&DAT_004bf9c4)[(byte)(DAT_004c012c + 0x1)] & 0x2) == 0x0 && (DAT_004c012c != '\0'));
DAT_004c012c = (DAT_004c012c + 0x1)) {
}
}
while (((&DAT_004bf9c4)[(byte)(DAT_004c012c + 0x1)] & 0x2) != 0x0) {
DAT_004c012c = (DAT_004c012c + 0x1);
}
command_line_w = GetCommandLineW();
if (command_line_w == (LPWSTR)0x0) {
_DAT_004c0138 = &DAT_004c36b4;
}
else {
_DAT_005b9de8 = FUN_004b5af0(command_line_w);
_DAT_004c0138 = _DAT_005b9de8;
if (_DAT_005b9de8 == 0x22) {
sVar2 = (_DAT_005b9de8 + 0x2);
puVar4 = _DAT_005b9de8;
while ((_DAT_004c0138 = (puVar4 + 0x2), sVar2 != 0x22 && (_DAT_004c0138 != 0x0))) {
sVar2 = (puVar4 + 0x1);
puVar4 = _DAT_004c0138;
}
if (_DAT_004c0138 != 0x0) {
_DAT_004c0138 = puVar4 + 0x1;
}
}
else {
for (; (((&DAT_004bf9c4)[(byte)(_DAT_004c0138 + 0x1)] & 0x2) == 0x0 && (_DAT_004c0138 != 0x0)
); _DAT_004c0138 = (_DAT_004c0138 + 0x2)) {
}
}
while (((&DAT_004bf9c4)[(byte)(_DAT_004c0138 + 0x1)] & 0x2) != 0x0) {
_DAT_004c0138 = (_DAT_004c0138 + 0x2);
}
}
if (param_1 != 0x0) {
// DWORD nSize for GetModuleFileNameA
// LPSTR lpFilename for GetModuleFileNameA
// HMODULE hModule for GetModuleFileNameA
GetModuleFileNameA(module_handle,(LPSTR)&lpFilename_005ba0fc,0x104);
_DAT_004c0134 = &lpFilename_005ba0fc;
get_mod_file_name_fn_004b59f0(module_handle,&WCHAR_005ba200,0x208);
_DAT_004c0140 = &WCHAR_005ba200;
}
piVar3 = 0x1;
}
return piVar3;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void  get_mod_handle_fn_004aeee8(DWORD param_1,param_2: *mut i32)

{
HMODULE module_handle;
let piVar1: *mut i32;;
let piVar2: *mut i32;;

// LPCSTR lpModuleName for GetModuleHandleA
module_handle = GetModuleHandleA(0x0);
FUN_004aec9c(0x0,param_2,module_handle);
piVar2 = &DAT_004c014c;
piVar1 = (PTR_FUN_004bfb74)();
query_virt_mem_004b5b30(piVar1,piVar2);
set_unhandled_except_filter_004b6038(param_1);
FUN_004b1cf0();
(*_DAT_004bfbb4)();
FUN_004b1cf0();
return;
}



fn FUN_004aefc0() -> *mut u32

{
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let mut uVar3: u32;

    (PTR_FUN_004bfb88)();
    if (DAT_005ba414 == 0x0) {
        puVar2 = &DAT_004bfbc0;
        loop {
            if ((*(puVar2 + 0x3) & 0x3) == 0x0) {
                puVar1 = FUN_004aac00(0x20);
                if (puVar1 == 0x0)^ // goto LAB_004af068;
                uVar3 = 0x3;^
                // goto LAB_004af039;
            }
            puVar2 = (puVar2 + 0x1a);
        } while (puVar2 < &DAT_004bfdc8);
        uVar3 = 0x4003;
        puVar1 = FUN_004aac00(0x3a);
        if (puVar1 == 0x0) {
            LAB_004af068:
                FUN_004b1740(0x5);
            (PTR_FUN_004bfb8c)();
            return 0x0;
        }
        puVar2 = puVar1 + 0x8;
    }
    else {
        puVar2 = DAT_005ba414[0x1];
        uVar3 = ((ushort)puVar2[0x3] & 0x4003 | 0x3);
        puVar1 = DAT_005ba414;
        DAT_005ba414 = *DAT_005ba414;
    }
    LAB_004af039:
        FUN_004a0430(puVar2,0x0,0x1a);
    puVar2[0x3] = uVar3;
    puVar1[0x1] = puVar2;
    puVar2[0x2] = puVar1;
    *puVar1 = DAT_005ba410;
    DAT_005ba410 = puVar1;
    (PTR_FUN_004bfb8c)();
    return puVar2;
}



fn FUN_004af080(param_1: i32)

{
    let puVar1: *mut u32;
    let puVar2: *mut u32;

    puVar1 = &DAT_005ba410;
    loop {
    puVar2 = puVar1;
    puVar1 = *puVar2;
    if (puVar1 == 0x0) {
        return;
    }
} while (param_1 != puVar1[0x1]);
    *(param_1 + 0xc) = *(puVar1[0x1] + 0xc) | 0x3;
    *puVar2 = *puVar1;
    *puVar1 = DAT_005ba414;
    DAT_005ba414 = puVar1;
    return;
}



fn FUN_004af0e0()

{
    GetCurrentProcessId();
    return;
}



undefined1 * FUN_004af21c()

{
let bVar1: u8;
let cVar2: u8;
byte *file_name;
let puVar3: *mut u32;
let mut uVar4: u32;
let ppuVar5: *mut *mut u8;;
byte *pbVar6;
let mut pcVar7: String;

if (DAT_004bfde4 == '\0') {
for (ppuVar5 = &PTR_DAT_004bfdd0; **ppuVar5 != 0x0; ppuVar5 = ppuVar5 + 0x1) {
file_name = FUN_004b6220(*ppuVar5);
if (file_name != 0x0) {
uVar4 = 0xffffffff;
pbVar6 = file_name;
loop {
if (uVar4 == 0x0) break;
uVar4 = uVar4 - 0x1;
bVar1 = *pbVar6;
pbVar6 = pbVar6 + 0x1;
} while (bVar1 != 0x0);
if (~uVar4 - 0x1 < 0x104) {
get_file_path_fn_004b6280(&DAT_004bfde4,file_name,0x103);
break;
}
}
}
if (DAT_004bfde4 == '\0') {
puVar3 = FUN_004b6340(0x0,0x0);
pcVar7 = &DAT_004bfde4;
loop {
cVar2 = puVar3;
*pcVar7 = cVar2;
if (cVar2 == '\0') break;
cVar2 = (puVar3 + 0x1);
puVar3 = (puVar3 + 0x2);
pcVar7[0x1] = cVar2;
pcVar7 = pcVar7 + 0x2;
} while (cVar2 != '\0');
}
uVar4 = 0xffffffff;
pcVar7 = &DAT_004bfde4;
loop {
if (uVar4 == 0x0) break;
uVar4 = uVar4 - 0x1;
cVar2 = *pcVar7;
pcVar7 = pcVar7 + 0x1;
} while (cVar2 != '\0');
uVar4 = ~uVar4;
if (((uVar4 + 0x4bfde2) != '\\') && ((uVar4 + 0x4bfde2) != '/')) {
(&DAT_004bfde3)[uVar4] = 0x5c;
(&DAT_004bfde4)[uVar4] = 0x0;
}
}
return &DAT_004bfde4;
}



fn FUN_004af2f0(param_1: *mut u32) -> u32

{
    let mut uVar1: u32;
    let DVar2: u32;
    LPCVOID write_buf;
    let DVar3: u32;
    let DVar4: u32;

    DVar4 = 0x0;
    (PTR_FUN_004bfb78)(param_1[0x4]);
    if ((*(param_1 + 0xd) & 0x10) == 0x0) {
        if ((*(param_1[0x2] + 0x8) != 0x0) &&
            (*(param_1 + 0x3) = *(param_1 + 0x3) & 0xef, (*(param_1 + 0xd) & 0x20) == 0x0)) {
            DVar3 = param_1[0x1];
            if (DVar3 != 0x0) {
                DVar3 = set_file_pointer_004af420(param_1[0x4],-DVar3,0x1);
            }
            if (DVar3 == 0xffffffff) {
                *(param_1 + 0x3) = *(param_1 + 0x3) | 0x20;
                DVar4 = DVar3;
            }
        }
    }
    else {
        *(param_1 + 0xd) = *(param_1 + 0xd) & 0xef;
        if (((*(param_1 + 0x3) & 0x2) != 0x0) &&
            (write_buf = *(LPCVOID *)(param_1[0x2] + 0x8), write_buf != (LPCVOID)0x0)) {
        DVar3 = param_1[0x1];
        while ((DVar3 != 0x0 && (DVar4 == 0x0))) {
        DVar2 = write_file_004b1830(param_1[0x4],write_buf,DVar3);
        if (DVar2 == 0xffffffff) {
        *(param_1 + 0x3) = *(param_1 + 0x3) | 0x20;
        DVar4 = DVar2;
        }
        else {
        if (DVar2 == 0x0) {
        FUN_004b1740(0xc);
        DVar4 = 0xffffffff;
        *(param_1 + 0x3) = *(param_1 + 0x3) | 0x20;
        }
        }
        write_buf = (LPCVOID)(write_buf + DVar2);
        DVar3 = DVar3 - DVar2;
        }
        }
    }
    uVar1 = *(param_1[0x2] + 0x8);
    param_1[0x1] = 0x0;
    *param_1 = uVar1;
    if (((DVar4 == 0x0) && ((*(param_1[0x2] + 0x10) & 0x1) != 0x0)) &&
        (DVar3 = flush_file_buffers_fn_004b63f0(param_1[0x4]), DVar3 == 0xffffffff)) {
        DVar4 = DVar3;
    }
    (PTR_FUN_004bfb7c)(param_1[0x4]);
    return DVar4;
}



fn set_file_pointer_004af420(param_1: u32,param_2: i32,DWORD param_3) -> u32

{
    let mut uVar1: u32;
    let DVar2: u32;

    if ((-0x1 < param_1) && (param_1 <= DAT_004bffe8)) {
        (PTR_FUN_004bfb78)(param_1);
        uVar1 = FUN_004b1a30(param_1);
        if ((0x0 < param_2) && ((uVar1 & 0x80) == 0x0)) {
            FUN_004b1a88(param_1,uVar1 | 0x8000);
        }
        // DWORD dwMoveMethod for SetFilePointer
        // PLONG lpDistanceToMoveHigh for SetFilePointer
        // LONG lDistanceToMove for SetFilePointer
        // HANDLE hFile for SetFilePointer
        DVar2 = SetFilePointer(*(HANDLE *)(DAT_004c0190 + param_1 * 0x4),param_2,(PLONG)0x0,param_3);
        (PTR_FUN_004bfb7c)(param_1);
        if (DVar2 == 0xffffffff) {
            handle_err_fn_004b12fc();
        }
        return DVar2;
    }
    FUN_004b1740(0x4);
    return 0xffffffff;
}



fn FUN_004af4b0(param_1: u32) -> u32

{
    HANDLE hObject;
    let DVar1: u32;
    let mut iVar2: i32;
    let mut handle_closed: bool;

    if ((param_1 < 0x0) || (DAT_004bffe8 < param_1)) {
        FUN_004b1740(0x4);
        DVar1 = 0xffffffff;
    }
    else {
        hObject = *(HANDLE *)(DAT_004c0190 + param_1 * 0x4);
        DVar1 = 0x0;
        if ((DAT_004c0050 == (code *)0x0) || (iVar2 = (*DAT_004c0044)(param_1), iVar2 == 0x0)) {
            // HANDLE hObject for CloseHandle
            handle_closed = CloseHandle(hObject);
            if (handle_closed == 0x0) {
                FUN_004b1740(0x4);
                return 0xffffffff;
            }
        }
        else {
            (*DAT_004c0048)(param_1);
            (*DAT_004c0050)(iVar2);
        }
        FUN_004b1a88(param_1,0x0);
    }
    return DVar1;
}



u32
FUN_004af560(param_1: *mut u32,param_2: &mut String,param_3: *mut i32,param_4: i32,param_5: i32,param_6: u32)

{
let sVar1: i16;
i32 **ppiVar2;
let puVar3: *mut u32;
let mut iVar4: i32;
let mut uVar5: u32;

FUN_004a0430(param_1,0x0,0xac);
param_1[0x27] = param_6;
ppiVar2 = FUN_004b0530(param_2,param_3,0x186a0,(param_5 != 0x0));
param_1[0x25] = ppiVar2;
if (ppiVar2 != (i32 **)0x0) {
FUN_004b03a0(ppiVar2,param_4);
puVar3 = FUN_004b00f0(param_1[0x25],0x80,0x0);
*param_1 = *puVar3;
(param_1 + 0x1) = (puVar3 + 0x1);
param_1[0x29] = 0x0;
param_1[0x22] = param_2;
param_1[0x28] = param_1[0x29];
if ((*(param_1 + 0x27) & 0x3) == 0x0) {
param_1[0x29] = 0x0;
param_1[0x28] = param_1[0x29];
}
else {
uVar5 = (param_1 + 0xa) * (param_1 + 0x2);
iVar4 = FUN_0049c2c9(uVar5);
param_1[0x28] = iVar4;
if (iVar4 == 0x0) {
if ((i32 **)param_1[0x25] != (i32 **)0x0) {
ppiVar2 = FUN_004b0840((i32 **)param_1[0x25]);
FUN_0049af50(ppiVar2);
}
return 0xfffffffe;
}
if ((*(param_1 + 0x27) & 0x1) == 0x0) {
param_1[0x29] = iVar4;
}
else {
iVar4 = FUN_0049c2c9(uVar5);
param_1[0x29] = iVar4;
if (iVar4 == 0x0) {
FUN_0049af50(param_1[0x28]);
if ((i32 **)param_1[0x25] != (i32 **)0x0) {
ppiVar2 = FUN_004b0840((i32 **)param_1[0x25]);
FUN_0049af50(ppiVar2);
}
return 0xfffffffe;
}
}
}
sVar1 = (param_1 + 0x1);
if ((sVar1 == -0x50ee) || (sVar1 == -0x50eb)) {
FUN_004b00f0(param_1[0x25],param_1[0x14],0x0);
param_1[0x15] = param_1[0x14];
FUN_004b0070(param_1[0x25],param_1[0x14],0x0);
return 0x0;
}
if (sVar1 == -0x50ef) {
param_1[0x14] = 0x80;
param_1[0x15] = param_1[0x14];
FUN_004b0070(param_1[0x25],0x80,0x0);
param_1[0x4] = (param_1[0x4] * 0x3e8) / 0x46;
return 0x0;
}
if ((i32 **)param_1[0x25] != (i32 **)0x0) {
ppiVar2 = FUN_004b0840((i32 **)param_1[0x25]);
FUN_0049af50(ppiVar2);
return 0xfffffffd;
}
}
return 0xfffffffd;
}



fn FUN_004af7d0(param_1: i32)

{
    i32 **ppiVar1;

    if ((*(param_1 + 0x9c) & 0x3) != 0x0) {
        if (*(param_1 + 0xa0) != 0x0) {
            FUN_0049af50(*(param_1 + 0xa0));
        }
        if (((*(param_1 + 0x9c) & 0x1) != 0x0) && (*(param_1 + 0xa4) != 0x0)) {
            FUN_0049af50(*(param_1 + 0xa4));
        }
    }
    if (*(i32 ***)(param_1 + 0x94) == (i32 **)0x0) {
    return;
}
    ppiVar1 = FUN_004b0840(*(i32 ***)(param_1 + 0x94));
    FUN_0049af50(ppiVar1);
    return;
}



fn FUN_004af840(param_1: i32) -> u32

{
    let mut uVar1: u32;
    let puVar2: *mut u32;
    let mut iVar3: i32;
    let mut uVar4: u32;

    puVar2 = FUN_004b00f0(*(param_1 + 0x94),0x10,0x0);
    if ((puVar2 + 0x1) == -0xe06) {
        uVar1 = *puVar2;
        if (uVar1 < 0x11) {
            uVar4 = 0x0;
        }
        else {
            iVar3 = FUN_004b00f0(*(param_1 + 0x94),uVar1,0x0);
            uVar4 = FUN_004b6450(param_1,puVar2,(iVar3 + 0x10));
        }
        FUN_004b0070(*(param_1 + 0x94),uVar1,0x0);
        DAT_005ba420 = uVar1;
        return uVar4;
    }
    return 0xfffffffc;
}

