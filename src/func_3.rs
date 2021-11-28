
fn FUN_00405d5a(param_1: i32)

{
    let cVar1: u8;
    let mut bVar2: bool;
    let puVar3: *mut u32;
    let mut pcVar4: String;
    let mut puVar5: *mut u8;
    let mut iVar6: i32;
    let mut pcVar7: String;
    let local_a8: *mut u32;
    u32 local_98 [0x20];
    let local_18: *mut u32;
    let local_14: *mut u32;
    let puVar8: *mut u32;

    for (local_14 = *DAT_005967c8; local_14 != 0x0; local_14 = *local_14) {
        if ((local_14[0x3] >> 0x10 < 0x1a) || (0x1f < local_14[0x3] >> 0x10)) {
            if (local_14[0x4] >> 0x10 == *(&DAT_004bd038 + param_1 * 0x4)) {
                if (*(param_1 * 0x8 + 0x4c504c) != -0x1) {
                    for (local_a8 = (&DAT_005b8b44 + (*(local_14 + 0x6) >> 0x10) * 0x4);
                        (local_a8 != 0x0 && ((local_a8 + 0x8) == (local_14 + 0x2)));
                        local_a8 = *local_a8) {
                        if (((*(local_a8 + 0x3a) & 0x1) != 0x0) &&
                            (((local_a8 + 0x22) == (local_14 + 0xa) &&
                                ((local_a8 + 0x9) == (local_14 + 0x3))))) {
                            if (((local_a8 + 0x35) == -0x1) ||
                                ((local_a8 + 0x26) == (local_a8 + 0x35))) {
                                bVar2 = false;
                            }
                            else {
                                bVar2 = true;
                            }
                            if (!bVar2) {
                                *(local_a8 + 0x3a) =
                                    *(local_a8 + 0x3a) &
                                ~*(&DAT_004be9b0 + (*(local_a8 + 0x23) >> 0x18) * 0x4);
                                (local_a8 + 0x26) = (local_14 + 0x12);
                                (local_a8 + 0x35) = 0xff;
                            }
                        }
                    }
                        *(local_14 + 0x4) = *(&DAT_004bd038 + param_1 * 0x4);
                    *(local_14 + 0x2d) = *(local_14 + 0x2d) & 0xfe;
                    *(local_14 + 0x6) = 0xffff;
                    *(local_14 + 0x22) = 0xffff;
                    *(local_14 + 0x9) = 0xffff;
                    *(local_14 + 0x5) = 0xffff;
                    *(local_14 + 0x16) = 0xffff;
                    *(local_14 + 0x26) = 0xffff;
                }
            }
            else {
                if (((local_14[0xb] & 0x100) == 0x0) &&
                    (puVar3 = FUN_0045ad4e(*(local_14 + 0x6) >> 0x10,local_14[0x2] >> 0x10,
                                           *(local_14 + 0xa) >> 0x10), puVar3 != 0x0)) {
                    if (((puVar3 + 0x35) == -0x1) ||
                        ((puVar3 + 0x26) == (puVar3 + 0x35))) {
                        bVar2 = false;
                    }
                    else {
                        bVar2 = true;
                    }
                    if ((bVar2) && (*(puVar3 + 0x32) >> 0x18 == *(&DAT_004bd038 + param_1 * 0x4))) {
                        (local_14 + 0x4) = (puVar3 + 0x35);
                        (local_14 + 0x12) = (puVar3 + 0x35);
                        *(local_14 + 0x2d) = *(local_14 + 0x2d) | 0x1;
                        *(local_14 + 0x6) = 0xffff;
                        *(local_14 + 0x22) = 0xffff;
                        *(local_14 + 0x9) = 0xffff;
                        *(local_14 + 0x5) = 0xffff;
                        *(local_14 + 0x16) = 0xffff;
                        *(local_14 + 0x26) = 0xffff;
                    }
                }
            }
        }
    }
    for (local_18 = *DAT_005967b0; local_18 != 0x0; local_18 = *local_18) {
        if ((*(local_18 + 0x32) >> 0x18 == *(&DAT_004bd038 + param_1 * 0x4)) &&
            (*(param_1 * 0x8 + 0x4c504c) != -0x1)) {
            *(local_18 + 0x3a) =
                *(local_18 + 0x3a) & ~*(&DAT_004be9b0 + *(param_1 * 0x8 + 0x4c504c) * 0x4);
            (local_18 + 0x26) = (&DAT_004bd038)[param_1 * 0x4];
            (local_18 + 0x35) = (&DAT_004bd038)[param_1 * 0x4];
            *(local_18 + 0x3a) = *(local_18 + 0x3a) & 0x7f;
        }
    }
    if ((&DAT_004c5048)[param_1 * 0x2] != -0x1) {
        FUN_00499050(DAT_0059679c,(&DAT_004c5048)[param_1 * 0x2] + 0x414);
        pcVar4 = FUN_00499050(DAT_0059679c,0x73cc);
        FUN_0049c2e0(local_98,pcVar4);
        pcVar4 = FUN_00499050(DAT_0059679c,param_1 + 0x735e);
        iVar6 = -0x1;
        puVar3 = local_98;
        loop {
            puVar8 = puVar3;
            if (iVar6 == 0x0) break;
            iVar6 = iVar6 + -0x1;
            puVar8 = (puVar3 + 0x1);
            cVar1 = puVar3;
            puVar3 = puVar8;
        } while (cVar1 != '\0');
        pcVar7 = (puVar8 + -0x1);
        loop {
            cVar1 = *pcVar4;
            *pcVar7 = cVar1;
            if (cVar1 == '\0') break;
            cVar1 = pcVar4[0x1];
            pcVar4 = pcVar4 + 0x2;
            pcVar7[0x1] = cVar1;
            pcVar7 = pcVar7 + 0x2;
        } while (cVar1 != '\0');
        FUN_0045518a(0x1f,0xffffffff,0x74cc,0xffffffff,local_98,0xffffffff,0x1);
        for (local_18 = *DAT_005967b0; local_18 != 0x0; local_18 = *local_18) {
            if (*(local_18 + 0x23) >> 0x18 == *(&DAT_004bd038 + param_1 * 0x4)) {
                (local_18 + 0x26) = (&DAT_004c5048)[param_1 * 0x2];
                (local_18 + 0x35) = (&DAT_004bd038)[param_1 * 0x4];
                *(local_18 + 0x3a) = *(local_18 + 0x3a) | 0x80;
                FUN_00459e8f(local_18);
                FUN_0044add9(local_18);
                puVar5 = &DAT_00568210 + (local_18 + 0x8) * 0x9d + (*(local_18 + 0x23) >> 0x18) * 0x1e22;
                puVar5[0x9c] = puVar5[0x9c] & 0xfe;
                puVar5[0x9c] = puVar5[0x9c] | 0x1;
            }
        }
        for (local_14 = *DAT_005967c8; local_14 != 0x0; local_14 = *local_14) {
            if (*(local_14 + 0xe) >> 0x10 == *(&DAT_004bd038 + param_1 * 0x4)) {
                (local_14 + 0x4) = (&DAT_004c5048)[param_1 * 0x2];
                *(local_14 + 0x12) = *(&DAT_004bd038 + param_1 * 0x4);
                *(local_14 + 0x2d) = *(local_14 + 0x2d) | 0x1;
                local_14[0xb] = local_14[0xb] | *(&DAT_004be9b0 + (&DAT_004c5048)[param_1 * 0x2] * 0x4);
                puVar5 = &DAT_00568210 +
                    (*(local_14 + 0x6) >> 0x10) * 0x9d + (*(local_14 + 0xe) >> 0x10) * 0x1e22;
                puVar5[0x9c] = puVar5[0x9c] & 0xfe;
                puVar5[0x9c] = puVar5[0x9c] | 0x1;
            }
        }
    }
    for (local_14 = *DAT_005967c8; local_14 != 0x0; local_14 = *local_14) {
        if ((((local_14[0x3] >> 0x10 < 0x1a) || (0x1f < local_14[0x3] >> 0x10)) &&
            ((local_14[0xb] & 0x100) == 0x0)) &&
            (puVar3 = FUN_0045ad4e(*(local_14 + 0x6) >> 0x10,local_14[0x2] >> 0x10,
                                   *(local_14 + 0xa) >> 0x10), puVar3 != 0x0)) {
            if (((puVar3 + 0x35) == -0x1) || ((puVar3 + 0x26) == (puVar3 + 0x35))) {
                bVar2 = false;
            }
            else {
                bVar2 = true;
            }
            if (bVar2) {
                (local_14 + 0x4) = (puVar3 + 0x26);
                (local_14 + 0x12) = (puVar3 + 0x35);
                *(local_14 + 0x2d) = *(local_14 + 0x2d) | 0x1;
                *(local_14 + 0x6) = 0xffff;
                *(local_14 + 0x22) = 0xffff;
                *(local_14 + 0x9) = 0xffff;
                *(local_14 + 0x5) = 0xffff;
                *(local_14 + 0x16) = 0xffff;
                *(local_14 + 0x26) = 0xffff;
            }
        }
    }
        *(param_1 * 0x8 + 0x4c504c) = (&DAT_004c5048)[param_1 * 0x2];
    return;
}



fn FUN_00406851()

{
    let mut uVar1: u32;
    let mut local_110: *mut u32 [0x11];
    let ppuStack203: *mut *mut u8;;
    let mut local_1f: String;;
    let local_18: *mut i32;;

    local_18 = FUN_004990e0(local_110,0x0,s_diplo_res_004c06db,s_SendMessDlg_004c06cf);
    uVar1 = FUN_0049bb50(local_110,FUN_0040690b);
    if (uVar1 != 0x0) {
        FUN_004a1651();
    }
    ppuStack203 = &PTR_FUN_004c3d34;
    if (local_1f != 0x0) {
        FUN_00499b30(local_110,local_1f);
    }
    FUN_0049a1c0(local_110,0x1);
    return;
}



// WARNING: Could not reconcile some variable overlaps

fn FUN_0040690b(param_1: *mut *mut u32,param_2: u32,param_3: u32) -> u32

{
    u32 local_22c [0x80];
    let mut local_2c: u32;
    let mut local_28: i32;
    let mut local_24: u32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_14: u32;

    local_14 = param_2;
    if (param_2 < 0x405) {
        if (param_2 == 0x401) {
            for (local_1c = 0x0; local_1c < 0x5; local_1c = local_1c + 0x1) {
                *(&DAT_004c5060 + local_1c * 0x4) = 0x1;
                FUN_0049bf80(param_1,local_1c + 0x1f8,0x410,0x0,0x0);
            }
        }
    }
    else {
        if (param_2 < 0x406) {
            FUN_004953d7();
            for (local_20 = 0x0; local_20 < 0x5; local_20 = local_20 + 0x1) {
                FUN_0049e640(local_20 * 0x78 + 0x1f,0x144,0x64,0x64,0xce,0xca,0xcc,0x1);
                FUN_0049e640(local_20 * 0x78 + 0x1f,0x1ac,0x64,0x10,0xce,0xca,0xcc,0x1);
                FUN_004968e7(local_20 * 0x78 + 0x1f,0x1ac,0x64,0x10,0xe);
                FUN_00497567(local_20 * 0x78 + 0x51,0x1ad,(&DAT_00569b50 + local_20 * 0x1e22),0x64,0xcaccce,0xe0e0e,
                             0xcaccce,LPCSTR_005b9218,0x11);
            }
            FUN_0049536f();
            return 0x1;
        }
        if (0x406 < param_2) {
            if (param_2 < 0x408) {
                local_24 = param_3;
                if (param_3 < 0x1f9) {
                    if (param_3 < 0x65) {
                        if (param_3 != 0x64) {
                            return 0x0;
                        }
                        local_2c = 0x0;
                        FUN_0049bf80(param_1,0xca,0x501,0x1ff,local_22c);
                        for (local_28 = 0x0; local_28 < 0x5; local_28 = local_28 + 0x1) {
                            if (*(&DAT_004c5060 + local_28 * 0x4) == 0x0) {
                                local_2c = local_2c | 0x1 << ((byte)local_28 & 0x1f);
                            }
                        }
                        FUN_0045518a(local_2c,DAT_004c9754,0x74cc,0xffffffff,local_22c,0xffffffff,0x0);
                        FUN_0049c140(param_1,0x1);
                        return 0x0;
                    }
                    if (param_3 < 0x66) {
                        FUN_0049c140(param_1,0x0);
                        return 0x0;
                    }
                    if (param_3 != 0x1f8) {
                        return 0x0;
                    }
                }
                else {
                    if ((((0x1f9 < param_3) && (0x1fa < param_3)) && (0x1fb < param_3)) && (param_3 != 0x1fc)) {
                        return 0x0;
                    }
                }
                if (*(param_3 * 0x4 + 0x4c4880) == 0x0) {
                    *(param_3 * 0x4 + 0x4c4880) = 0x1;
                    FUN_0049bf80(param_1,param_3,0x410,0x0,0x0);
                }
                else {
                    *(param_3 * 0x4 + 0x4c4880) = 0x0;
                    FUN_0049bf80(param_1,param_3,0x40f,0x0,0x0);
                }
            }
            else {
                if (param_2 == 0x411) {
                    if ((*(param_3 * 0x4 + 0x4c4880) == 0x0) ||
                        (((&DAT_00569a98)[(param_3 - 0x1f8) * 0x1e22] & 0x1) != 0x0)) {
                        *(param_3 * 0x4 + 0x4c4880) = 0x1;
                        FUN_0049bf80(param_1,param_3,0x410,0x0,0x0);
                    }
                    else {
                        *(param_3 * 0x4 + 0x4c4880) = 0x0;
                        FUN_0049bf80(param_1,param_3,0x40f,0x0,0x0);
                    }
                    return 0x1;
                }
            }
        }
    }
    return 0x0;
}



fn FUN_00406cc8(param_1: i32) -> u32

{
    let local_14: *mut u32;

    local_14 = (&DAT_005b8b44 + DAT_004d557c * 0x4);
    while( true ) {
        if ((local_14 == 0x0) || ((local_14 + 0x8) != DAT_004d557c)) {
            return 0x0;
        }
        if ((*(local_14 + 0x23) >> 0x18 == param_1) && ((local_14 + 0x27) == '-')) break;
        local_14 = *local_14;
    }
    return 0x1;
}



fn FUN_00406d51(param_1: i32) -> i32

{
let cVar1: u8;
let mut iVar2: i32;
let mut pcVar3: String;
let mut uVar4: u32;
u32 local_a0 [0x20];
let mut local_20: i32;
let mut local_18: i32;
let mut local_14: i32;

local_18 = 0x0;
if (0x13 < DAT_004c5074) {
DAT_004c5074 = DAT_004c5074 + -0x1;
FUN_004a1dc0(&DAT_004c5078,&DAT_004c5079,0x13);
}
iVar2 = FUN_004a11c0(param_1);
(&DAT_004c5078)[DAT_004c5074] = iVar2;
(&DAT_004c5079)[DAT_004c5074] = 0x0;
DAT_004c5074 = DAT_004c5074 + 0x1;
local_14 = 0x0;
loop {
if (DAT_004c5074 + -0x3 <= local_14) {
if (local_18 != 0x0) {
DAT_004c5074 = 0x0;
pcVar3 = FUN_00499050(DAT_0059679c,0x7414);
FUN_0049c2e0(local_a0,pcVar3);
FUN_0045518a(0x1f,0xffffffff,0x74cc,0xffffffff,local_a0,0xffffffff,0x1);
}
return local_18;
}
for (local_20 = 0x0; local_20 < 0x5; local_20 = local_20 + 0x1) {
uVar4 = 0xffffffff;
pcVar3 = (&PTR_s_HDIKASH_004bd050)[local_20];
loop {
if (uVar4 == 0x0) break;
uVar4 = uVar4 - 0x1;
cVar1 = *pcVar3;
pcVar3 = pcVar3 + 0x1;
} while (cVar1 != '\0');
iVar2 = FUN_004a1e20(&DAT_004c5078 + local_14,(&PTR_s_HDIKASH_004bd050)[local_20],~uVar4 - 0x1);
if (iVar2 == 0x0) {
local_18 = local_20 + 0x1;
break;
}
}
local_14 = local_14 + 0x1;
} while( true );
}



fn FUN_00406eab() -> u32

{
    let mut local_120: *mut u32 [0x11];
    let ppuStack219: *mut *mut u8;;
    let mut local_2f: String;;
    let mut local_28: u32;
    u32 **local_24;
    let local_20: *mut i32;;
    let mut local_1c: u32;

    local_20 = FUN_004990e0(local_120,0x0,s_efs_res_004c06fa,s_Cheat_004c06f4);
    local_28 = FUN_0049bb50(local_120,FUN_00406fa2);
    local_24 = local_120;
    local_1c = 0x0;
    ppuStack219 = &PTR_FUN_004c3d34;
    if (local_2f != 0x0) {
        FUN_00499b30(local_120,local_2f);
    }
    FUN_0049a1c0(local_120,0x1);
    return local_28;
}



fn FUN_00406fa2(param_1: *mut *mut u32,param_2: u32,param_3: i32) -> u32

{
    let mut iVar1: i32;
    let local_28: u8 [0x14];
    let mut local_14: u32;

    local_14 = param_2;
    if (0x400 < param_2) {
        if (param_2 < 0x402) {
            FUN_0049bf80(param_1,0xc8,0x502,0xa,&DAT_004c0702);
        }
        else {
            if ((param_2 == 0x407) && (param_3 == 0xc8)) {
                FUN_0049bf80(param_1,0xc8,0x501,0xa,local_28);
                iVar1 = FUN_004a1e60(local_28);
                FUN_0049c140(param_1,iVar1);
            }
        }
    }
    return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0040705e(param_1: i32) -> i32

{
let cVar1: u8;
let mut pcVar2: String;
let mut iVar3: i32;
let puVar4: *mut u32;
let mut pcVar5: String;
u32 local_114 [0x40];
let mut local_14: i32;
let puVar6: *mut u32;

DAT_004c9770 = param_1;
_DAT_004c9774 = 0x0;
_DAT_004d5560 = 0x1;
FUN_00499050(DAT_0059679c,param_1 + 0x414);
pcVar2 = FUN_00499050(DAT_0059679c,0x73cc);
FUN_0049c2e0(local_114,pcVar2);
pcVar2 = FUN_00499050(DAT_0059679c,0x73e3);
iVar3 = -0x1;
puVar4 = local_114;
loop {
puVar6 = puVar4;
if (iVar3 == 0x0) break;
iVar3 = iVar3 + -0x1;
puVar6 = (puVar4 + 0x1);
cVar1 = puVar4;
puVar4 = puVar6;
} while (cVar1 != '\0');
pcVar5 = (puVar6 + -0x1);
loop {
cVar1 = *pcVar2;
*pcVar5 = cVar1;
if (cVar1 == '\0') break;
cVar1 = pcVar2[0x1];
pcVar2 = pcVar2 + 0x2;
pcVar5[0x1] = cVar1;
pcVar5 = pcVar5 + 0x2;
} while (cVar1 != '\0');
iVar3 = FUN_0045518a(0x1f,0xffffffff,0x74cc,0xffffffff,local_114,0xffffffff,0x1);
for (local_14 = 0x0; local_14 < 0xe; local_14 = local_14 + 0x1) {
if ((local_14 != param_1) && ((&DAT_004d55a8)[param_1 * 0xe + local_14] != '\x01')) {
*(&DAT_00569b14 + param_1 * 0x4 + local_14 * 0x1e22) =
*(&DAT_00569b14 + param_1 * 0x4 + local_14 * 0x1e22) + -0x14;
}
iVar3 = local_14;
}
return iVar3;
}



fn FUN_004071ba(param_1: i32) -> i32

{
let local_14: *mut u32;

local_14 = *DAT_005967c8;
while( true ) {
if (local_14 == 0x0) {
return -0x1;
}
if ((*(local_14 + 0x6) >> 0x10 == param_1) && ((local_14 + 0xe) == 0x1)) break;
local_14 = *local_14;
}
return local_14[0xa] >> 0x10;
}



fn FUN_0040721c(param_1: i32) -> i32

{
let mut iVar1: i32;
let local_1c: *mut u32;
let local_18: *mut u32;
let mut local_14: i32;

*(&DAT_004d73e8 + param_1 * 0x4) = 0x0;
iVar1 = param_1 * 0x4;
for (local_14 = 0x0; local_14 < 0x28; local_14 = local_14 + 0x1) {
if ((*(&DAT_005b70c2 + local_14 * 0x4e) != 0x0) && (iVar1 = FUN_004071ba(local_14), iVar1 == param_1)) {
for (local_1c = *DAT_005967b0; local_1c != 0x0; local_1c = *local_1c) {
if (((local_1c + 0x8) == local_14) &&
(*(*(&DAT_00582938 +
(*(local_1c + 0x25) >> 0x18) * 0x4 + (local_1c[0x9] >> 0x18) * 0x18) +
0x10d) != 0x0)) {
*(&DAT_004d73e8 + param_1 * 0x4) = *(&DAT_004d73e8 + param_1 * 0x4) + 0x1;
}
}
for (local_18 = *DAT_005967c8; local_18 != 0x0; local_18 = *local_18) {
if (*(local_18 + 0x6) >> 0x10 == local_14) {
*(&DAT_004d73e8 + param_1 * 0x4) =
*(&DAT_004d73e8 + param_1 * 0x4) + (*(local_18 + 0x26) >> 0x10) / 0xa;
}
}
}
iVar1 = local_14;
}
return iVar1;
}



fn FUN_00407343()

{
    let mut local_110: *mut u32 [0x11];
    let ppuStack203: *mut *mut u8;;
    let mut local_1f: String;;
    let mut local_18: i32;

    FUN_00489246(0x6,0x0);
    DAT_004c93cc = 0x6;
    DAT_004c93d4 = 0x0;
    DAT_004c93c8 = FUN_0049c2c9(0xc350);
    for (local_18 = 0x5; local_18 < 0x9; local_18 = local_18 + 0x1) {
        FUN_0040721c(local_18);
    }
    FUN_0049c60b(s_bin_ptratchu_bin_004c0755,DAT_004c93c8,0xc350,((DAT_005827f4 >> 0x10) + -0x5) * 0xc350);
    FUN_004990e0(local_110,0x0,s_diplo_res_004c0770,s_ChuDipDlg_004c0766);
    FUN_0049bb50(local_110,FUN_004074d1);
    FUN_0049af50(DAT_004c93c8);
    ppuStack203 = &PTR_FUN_004c3d34;
    if (local_1f != 0x0) {
        FUN_00499b30(local_110,local_1f);
    }
    FUN_0049a1c0(local_110,0x1);
    return;
}



fn FUN_004074d1(param_1: *mut *mut u32,param_2: u32,param_3: u32) -> u32

{
    let mut bVar1: bool;
    let mut pcVar2: String;
    let mut uVar3: u32;
    let mut puVar4: *mut u8;
    let mut iVar5: i32;
    let mut uVar6: u32;
    let mut iVar7: i32;
    let mut uVar8: u32;
    let puVar9: *mut u32;
    let bVar10: u8;
    let mut local_48: i32;
    let mut local_38: i32;
    let mut local_34: i32;
    let local_30: u8 [0x10];
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_14: u32;

    local_14 = param_2;
    if (param_2 < 0x407) {
        if (0x400 < param_2) {
            if (param_2 < 0x402) {
                for (local_1c = 0x5; local_1c < 0xa; local_1c = local_1c + 0x1) {
                    FUN_0040721c(local_1c);
                }
                FUN_0049f99b(DAT_005b9bd4,param_1,0x407,0x3039,0x0);
                return 0x0;
            }
            if (param_2 == 0x405) {
                FUN_004953d7();
                FUN_00496ac0(DAT_004c93c8,0xa,0x10,0xfa,0xc8);
                FUN_00497567(0x88,0xe4,&DAT_0057501c,0xf0,0xcaccce,0xe0e0e,0xcaccce,LPCSTR_005b9218,0x11);
                FUN_0049c2e0(local_30,&DAT_004c078b);
                FUN_00497567(0x235,0x122,local_30,0x28,0xcaccce,-0x1,0xcaccce,LPCSTR_005b9218,0x11);
                for (local_20 = 0x0; local_20 < 0x5; local_20 = local_20 + 0x1) {
                    bVar10 = 0x11;
                    uVar8 = 0xcaccce;
                    iVar7 = -0x1;
                    uVar6 = 0xcaccce;
                    iVar5 = 0x64;
                    puVar9 = LPCSTR_005b9218;
                    pcVar2 = FUN_00499050(DAT_0059679c,local_20 + 0x40f);
                    FUN_00497567(0x4b,local_20 * 0x1c + 0x12d,pcVar2,iVar5,uVar6,iVar7,uVar8,puVar9,bVar10);
                    FUN_0049c2e0(local_30,&DAT_004c078e);
                    FUN_00497567(0x154,local_20 * 0x1c + 0x12d,local_30,0x64,0xcaccce,-0x1,0xcaccce,
                                 LPCSTR_005b9218,0x11);
                    for (local_34 = 0x0; local_34 < 0x5; local_34 = local_34 + 0x1) {
                        local_38 = (CONCAT44((&DAT_005827f8)[local_20 + local_34 * 0xe] >> 0x1f,
                                             (&DAT_005827f8)[local_20 + local_34 * 0xe]) / 0x14);
                        if (local_38 == 0x5) {
                            local_38 = 0x4;
                        }
                        FUN_00496ee6(&DAT_00593140 + local_38 * 0xc3,local_34 * 0x1c + 0x90,local_20 * 0x1c + 0x12a,0xf,0xd);
                    }
                    if (((&DAT_00569a98)[local_20 * 0x1e22] & 0x20) == 0x0) {
                        FUN_0049c2e0(local_30,&DAT_004c0793);
                    }
                    else {
                        FUN_0049c2e0(local_30,&DAT_004c0791);
                    }
                    FUN_00497567(local_20 * 0x1c + 0x94,0x1bd,local_30,0x1b,0x636567,-0x1,0x636567,
                                 LPCSTR_005b9218,0x10);
                }
                FUN_00428288(DAT_004c93cc,DAT_004c9754,0x1,0x0);
                FUN_00428288(DAT_004c93cc,DAT_004c9754,0x0,0x0);
                FUN_0049536f();
            }
        }
    }
    else {
        if (param_2 < 0x408) {
            if (param_3 < 0x1a6) {
                if (param_3 < 0x135) {
                    if (param_3 != 0x64) {
                        return 0x0;
                    }
                    bVar1 = true;
                    local_48 = 0x0;
                    loop {
                        if (0x2 < local_48) {
                            LAB_00407a1e:
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
                        if (0x0 < *(&DAT_004c9780 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8 + local_48 * 0x28)) {
                            bVar1 = false;^
                            // goto LAB_00407a1e;
                        }
                        if (0x0 < *(&DAT_004c97f8 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8 + local_48 * 0x28)) {
                            bVar1 = false;^
                            // goto LAB_00407a1e;
                        }
                        local_48 = local_48 + 0x1;
                    } while( true );
                }
                if (param_3 < 0x136) {
                    FUN_0042d188(DAT_004c93cc,DAT_004c9754);
                    FUN_00428288(DAT_004c93cc,DAT_004c9754,0x1,0x0);
                    FUN_00428288(DAT_004c93cc,DAT_004c9754,0x0,0x0);
                    return 0x0;
                }
                if (param_3 < 0x137) {
                    puVar4 = FUN_0049bf80(param_1,0x136,0x501,0x0,0x0);
                    (&DAT_004c9778 + DAT_004c93cc * 0xf8 + DAT_004c9754 * 0xd90) = puVar4;
                    FUN_0042d418();
                    FUN_00428288(DAT_004c93cc,DAT_004c9754,0x1,0x0);
                    FUN_00428288(DAT_004c93cc,DAT_004c9754,0x0,0x0);
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
            uVar3 = FUN_00429b8c(0x2);
            if ((0x63 < uVar3) && (uVar3 < 0x65)) {
                if (DAT_004c93d0 == 0x0) {
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
                    FUN_004a08c5(s_pcx_diplochu_pcx_004c077a,0x0,0x0,0x280,0x1e0,0x0,0x0,0x0,0x1);
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



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00407be4() -> u32

{
    let mut local_124: *mut u32 [0x11];
    let ppuStack223: *mut *mut u8;;
    let mut local_33: String;;
    let mut local_2c: u32;
    let local_28: *mut u32;
    let local_24: *mut i32;;
    u32 **local_20;
    let mut local_1c: u32;

    _DAT_0059aa0c = 0x3de;
    DAT_0059aa18 = 0x3;
    local_24 = FUN_004990e0(local_124,0x0,s_diplo_res_004c079d,s_TechDlg_004c0795);
    local_28 = FUN_0049a250(local_124,0x4);
    *local_28 = DAT_0059aa18;
    local_2c = FUN_0049bb50(local_124,FUN_0044cd2e);
    local_20 = local_124;
    local_1c = 0x0;
    ppuStack223 = &PTR_FUN_004c3d34;
    if (local_33 != 0x0) {
        FUN_00499b30(local_124,local_33);
    }
    FUN_0049a1c0(local_124,0x1);
    return local_2c;
}



fn FUN_00407d0e() -> u32

{
    let mut uVar1: u32;
    let local_194: *mut u32;
    let mut local_140: *mut u32 [0x8];
    let mut local_11f: i32;
    let ppuStack251: *mut *mut u8;;
    let mut local_4f: String;;
    u32 auStack72 [0x5];
    let local_34: u8 [0x10];
    let mut local_24: u32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let local_18: *mut i32;;

    local_18 = FUN_004990e0(local_140,0x0,s_diplo_res_004c07b3,s_AskExcomDlg_004c07a7);
    local_1c = 0x0;
    for (local_20 = 0x0; local_20 < 0x5; local_20 = local_20 + 0x1) {
        if (local_20 != DAT_004c9754) {
            uVar1 = FUN_0049c2c9(0x2710);
            *(&DAT_004c93e0 + local_1c * 0x4) = uVar1;
            FUN_0049c2e0(local_34,s_bin_house_d_bin_004c07bd);
            FUN_0049c57b(local_34,(&DAT_004c93e0 + local_1c * 0x4),0x2710);
            local_194 = FUN_004a2831(0x5d);
            if (local_194 != 0x0) {
                uVar1 = *(&DAT_004c93e0 + local_1c * 0x4);
                local_194 = FUN_0049a030(local_194,local_140,local_20 + 0x258,local_1c * 0x8c + 0x3c,local_11f + 0x28,0x64,
                                         0x64,0x2,0xcaccce,0xe0e0e);
                $1: &mut String(local_194 + 0x45) = &PTR_FUN_004c3d94;
                *(local_194 + 0x51) = uVar1;
                *(local_194 + 0x55) = 0x0;
                *(local_194 + 0x4d) = 0x0;
                *(local_194 + 0x49) = 0x2;
            }
            auStack72[local_20] = local_194;
            FUN_0049bf40(local_140,auStack72[local_20]);
            local_1c = local_1c + 0x1;
        }
    }
    for (local_20 = 0x0; local_20 < 0x4; local_20 = local_20 + 0x1) {
        if (*(&DAT_004c93e0 + local_20 * 0x4) != 0x0) {
            FUN_0049af50(&DAT_004c93e0);
        }
    }
    local_24 = FUN_0049bb50(local_140,FUN_004080a3);
    ppuStack251 = &PTR_FUN_004c3d34;
    if (local_4f != 0x0) {
        FUN_00499b30(local_140,local_4f);
    }
    FUN_0049a1c0(local_140,0x1);
    return local_24;
}



fn FUN_004080a3(param_1: i32,param_2: u32,param_3: u32) -> u32

{
    let mut local_20: i32;
    let mut local_1c: i32;

    if (param_2 < 0x407) {
        if (param_2 == 0x405) {
            FUN_004953d7();
            local_20 = 0x0;
            for (local_1c = 0x0; local_1c < 0x5; local_1c = local_1c + 0x1) {
                if (local_1c != DAT_004c9754) {
                    FUN_00497567(local_20 * 0x8c + 0x6e,*(param_1 + 0x21) + 0x91,(&DAT_00569b50 + local_1c * 0x1e22),
                                 0x64,0xcaccce,-0x1,0xcaccce,LPCSTR_005b9218,0x11);
                    local_20 = local_20 + 0x1;
                }
            }
            FUN_0049536f();
            return 0x1;
        }
    }
    else {
        if (param_2 < 0x408) {
            if (param_3 < 0x259) {
                if (param_3 < 0x65) {
                    return 0x0;
                }
                if (param_3 < 0x66) {
                    FUN_0049c140(param_1,0x0);
                    return 0x0;
                }
                if (param_3 != 0x258) {
                    return 0x0;
                }
            }
            else {
                if ((((0x259 < param_3) && (0x25a < param_3)) && (0x25b < param_3)) && (param_3 != 0x25c)) {
                    return 0x0;
                }
            }
            if (DAT_004c93d0 == 0x0) {
                *(&DAT_004c97f8 + DAT_004c93d4 * 0x28 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8) = 0x172;
                *(&DAT_004c981c + DAT_004c93d4 * 0x28 + DAT_004c93cc * 0xf8 + DAT_004c9754 * 0xd90) = param_3 - 0x258;
            }
            else {
                *(&DAT_004c9780 + DAT_004c93d4 * 0x28 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8) = 0x172;
                *(&DAT_004c97a4 + DAT_004c93d4 * 0x28 + DAT_004c93cc * 0xf8 + DAT_004c9754 * 0xd90) = param_3 - 0x258;
            }
            FUN_0049c140(param_1,0x172);
        }
        else {
            if (param_2 == 0x411) {
                return 0x1;
            }
        }
    }
    return 0x0;
}



fn FUN_004082e7(param_1: i32)

{
    let puVar1: *mut u32;
    let mut uVar2: u32;
    let uVar3: u16;

    (&DAT_00569a98)[param_1 * 0x1e22] = (&DAT_00569a98)[param_1 * 0x1e22] | 0x10;
    *(&DAT_00569ad4 + param_1 * 0x1e22) = excommunicate_turns_00582918;
    FUN_00482c8a(param_1);
    if (((&DAT_00569a98)[param_1 * 0x1e22] & 0x2) == 0x0) {
        uVar3 = 0x0;
        uVar2 = 0xffffffff;
        puVar1 = FUN_00499050(DAT_0059679c,0x7385);
        FUN_0045518a(0x1 << ((byte)param_1 & 0x1f),0x6,0x74cc,0x0,puVar1,uVar2,uVar3);
    }
    return;
}



fn FUN_00408371(param_1: i32)

{
    (&DAT_00569a98)[param_1 * 0x1e22] = (&DAT_00569a98)[param_1 * 0x1e22] & 0xef;
    *(&DAT_00569ad4 + param_1 * 0x1e22) = excommunicate_turns_00582918;
    FUN_00482c8a(param_1);
    return;
}



fn FUN_004083b2(param_1: i32)

{
    let mut pcVar1: String;
    u32 local_90 [0x20];

    pcVar1 = FUN_00499050(DAT_0059679c,0x7383);
    FUN_0049c2e0(local_90,pcVar1);
    FUN_0045518a(0x1f,0x6,0x7383,0x0,local_90,0xffffffff,0x0);
    *(&DAT_0058ad72 + param_1 * 0xda) = 0x1;
    return;
}



fn FUN_0040842c(param_1: i32)

{
    (param_1 + 0x1c) = 0x0;
    return;
}



fn FUN_00408447() -> String

{
    return PTR_s_BUTTON_004bf954;
}



fn FUN_00408466(param_1: &mut String,byte param_2) -> String

{
    let piVar1: *mut i32;;

    if ((param_2 & 0x4) == 0x0) {
        param_1 = FUN_0049a1c0(param_1,0x1);
        if ((param_2 & 0x2) != 0x0) {
            FUN_0049af50(param_1);
        }
    }
    else {
        piVar1 = FUN_00498dce(param_1,&DAT_004c3db0);
        FUN_00498df5(piVar1);
    }
    return param_1;
}



fn FUN_004084c9() -> i32

{
u32 local_144 [0x2];
let mut local_13c: *mut u32 [0x11];
let ppuStack247: *mut *mut u8;;
let mut local_4b: String;;
let mut local_44: i32;
let local_40: *mut i32;;
let local_3c: *mut i32;;
let local_38: *mut i32;;
let local_34: *mut i32;;
i32 **local_30;
i32 **local_2c;
let mut local_28: u32;
let local_24: *mut i32;;
let local_20: *mut i32;;
let local_1c: *mut i32;;
i32 **local_18;

DAT_004c50a8 = 0x0;
local_24 = FUN_004990e0(local_13c,0x0,s_efs_res_004c07d6,s_CityInfo_004c07cd);
local_40 = FUN_004a2831(0xb9);
local_3c = local_40;
local_20 = local_40;
if (local_40 != 0x0) {
local_20 = FUN_00438792(local_40,local_13c,0x1f4,0x24,0xa6,0x58,0x3f,0x40,0x2,0x2,0x0);
}
DAT_004c509c = local_20;
local_38 = FUN_004a2831(0x95);
local_34 = local_38;
local_1c = local_38;
if (local_38 != 0x0) {
local_1c = FUN_0047157e(local_38,local_13c,0x1f5,0x1e,0x2e,0x64,0x60,0x40,0x2);
}
DAT_004c50a0 = local_1c;
FUN_0049bf40(local_13c,DAT_004c509c);
FUN_0049bf40(local_13c,DAT_004c50a0);
local_30 = FUN_004a2831(0x10);
local_2c = local_30;
local_18 = local_30;
if (local_30 != 0x0) {
local_18 = FUN_004a2874(local_30,local_13c,0x96);
}
local_28 = FUN_0049bb50(local_13c,FUN_0040885a);
if (local_28 != 0x0) {
FUN_00431d31(local_144);
FUN_00432515(local_144,*(DAT_004c50a8 + 0x6) >> 0x10,*(DAT_004c50a8 + 0x8) >> 0x10,
*(DAT_004c50a8 + 0xa) >> 0x10,0x1,-0x1);
FUN_00431d5a(&DAT_005967b8,local_144);
}
FUN_004a2965(local_13c);
if (DAT_004c509c != 0x0) {
((*(DAT_004c509c + 0x45) + 0x8))(DAT_004c509c,0x2);
}
if (DAT_004c50a0 != 0x0) {
((*(DAT_004c50a0 + 0x45) + 0x8))(DAT_004c50a0,0x2);
}
local_44 = DAT_004c50a8;
ppuStack247 = &PTR_FUN_004c3d34;
if (local_4b != 0x0) {
FUN_00499b30(local_13c,local_4b);
}
FUN_0049a1c0(local_13c,0x1);
return local_44;
}



fn FUN_00432de2(param_1: i32,param_2: i32,param_3: i32)

{
    let mut local_14: i32;

    for (local_14 = *(param_1 + 0x4); local_14 != 0x0; local_14 = *(local_14 + 0x8)) {
        if (param_3 == -0x1) {
            *(local_14 + 0x3a) = *(local_14 + 0x3a) | *(&DAT_004be9b0 + param_2 * 0x4);
        }
        else {
            if (*(local_14 + 0x24) >> 0x18 == param_3) {
                *(local_14 + 0x3a) = *(local_14 + 0x3a) | *(&DAT_004be9b0 + param_2 * 0x4);
            }
        }
    }
    return;
}



fn FUN_00432e4c(param_1: i32) -> u32

{
    let mut local_14: i32;

    local_14 = *(param_1 + 0x4);
    while( true ) {
        if (local_14 == 0x0) {
            return 0x1;
        }
        if ((*(&DAT_004be9b0 + DAT_004c9754 * 0x4) & *(local_14 + 0x3a)) != 0x0) break;
        local_14 = *(local_14 + 0x8);
    }
    return 0x0;
}



fn FUN_00432ea6(param_1: i32) -> u32

{
    let mut local_14: i32;

    local_14 = *(param_1 + 0x4);
    while( true ) {
        if (local_14 == 0x0) {
            return 0x1;
        }
        if (*(*(&DAT_00582938 +
            (*(local_14 + 0x25) >> 0x18) * 0x4 + (*(local_14 + 0x24) >> 0x18) * 0x18) + 0xad)
            == 0x0) break;
        local_14 = *(local_14 + 0x8);
    }
    return 0x0;
}



fn FUN_00432f12(param_1: i32) -> u32

{
    let mut local_14: i32;

    local_14 = *(param_1 + 0x4);
    while( true ) {
        if (local_14 == 0x0) {
            return 0x1;
        }
        if ((local_14 + 0x27) != '[') break;
        local_14 = *(local_14 + 0x8);
    }
    return 0x0;
}



fn FUN_00432f5e(param_1: i32) -> u32

{
    let mut local_14: i32;

    local_14 = *(param_1 + 0x4);
    while( true ) {
        if (local_14 == 0x0) {
            return 0x1;
        }
        if (*(*(&DAT_00582938 +
            (*(local_14 + 0x25) >> 0x18) * 0x4 + (*(local_14 + 0x24) >> 0x18) * 0x18) + 0xa9)
            == 0x0) break;
        local_14 = *(local_14 + 0x8);
    }
    return 0x0;
}



bool  FUN_00432fca(param_1: i32,param_2: i32)

{
let mut bVar1: bool;
let mut local_18: i32;

bVar1 = true;
local_18 = *(param_1 + 0x4);
loop {
if (local_18 == 0x0) {
return !bVar1;
}
if ((*(local_18 + 0x3a) & 0x40) == 0x0) {
bVar1 = false;
if ((local_18 + 0x2a) == 0x5) {
if (param_2 == 0x0) {
return false;
}
}
else {
if (((local_18 + 0x2a) != 0x7) && ((local_18 + 0x2a) != 0x9)) {
return false;
}
}
}
local_18 = *(local_18 + 0x8);
} while( true );
}



fn FUN_00433083(param_1: i32,param_2: i32) -> u32

{
    let mut local_14: u32;

    if (param_2 == 0x0) {
        if (param_1 == 0x4) {
            local_14 = 0x1;
        }
        else {
            local_14 = 0x0;
        }
    }
    else {
        if (((param_1 == 0x5) || (param_1 == 0x7)) || (param_1 == 0x9)) {
            local_14 = 0x1;
        }
        else {
            local_14 = 0x0;
        }
    }
    return local_14;
}



fn FUN_004330dc(param_1: i32,param_2: i32) -> u32

{
    let mut local_1c: i32;
    let mut local_14: i32;

    if (*(*(&DAT_00582938 +
        (*(param_2 + 0x25) >> 0x18) * 0x4 + (*(param_2 + 0x24) >> 0x18) * 0x18) + 0xa9) !=
        0x0) {
        for (local_14 = *(param_1 + 0x4); local_14 != 0x0; local_14 = *(local_14 + 0x8)) {
            if (*(*(&DAT_00582938 +
                (*(local_14 + 0x25) >> 0x18) * 0x4 + (*(local_14 + 0x24) >> 0x18) * 0x18) +
                0xa5) != 0x0) {
                for (local_1c = 0x0;
                    local_1c <
                    *(*(&DAT_00582938 +
                        (*(local_14 + 0x25) >> 0x18) * 0x4 + (*(local_14 + 0x24) >> 0x18) * 0x18) +
                        0xa5); local_1c = local_1c + 0x1) {
                    if (*(local_1c * 0x4 + local_14 + 0x10) == 0x0) {
                        *(param_2 + 0x3a) = *(param_2 + 0x3a) & 0xfb;
                        (param_2 + 0x3c) = (param_2 + 0x3c) & 0x1;
                        *(param_2 + 0x3a) = *(param_2 + 0x3a) | 0x40;
                        *(param_2 + 0x3a) =
                            *(param_2 + 0x3a) | *(&DAT_004be9b0 + (*(param_2 + 0x23) >> 0x18) * 0x4);
                        (param_2 + 0x2f) = 0x0;
                        *(local_1c * 0x4 + local_14 + 0x10) = param_2;
                        FUN_00431efd(param_1,param_2);
                        return 0x1;
                    }
                }
            }
        }
    }
    return 0x0;
}



// WARNING: Could not reconcile some variable overlaps

fn FUN_0043322a(param_1: i32,param_2: i32,param_3: i32,param_4: i32)

{
    let mut iVar1: i32;
    let mut local_74: i32;
    let mut local_70: i32;
    let mut local_30: u32;
    let local_2c: *mut u32;
    let local_28: *mut u32;
    i32 **local_24;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: u32;
    let mut local_14: u32;

    local_1c = param_1;
    local_14 = (*(param_1 + 0x4) == 0x0);
    local_18 = local_14;
    if (local_14 == 0x0) {
        FUN_00431d31(&local_30);
        local_20 = (DAT_005967bc + 0x20);
        LAB_00433294:
        if (*(param_1 + 0x4) != 0x0) {
            local_24 = *(i32 ***)(param_1 + 0x4);
            FUN_00431dec(param_1,*(param_1 + 0x4));
            for (local_28 = (&DAT_005b8b44 + local_20 * 0x4);
                (local_28 != 0x0 && ((local_28 + 0x8) == local_20));
                local_28 = *local_28) {
                if ((((local_28 + 0x22) == param_2) &&
                    (((((local_28 + 0x9) == param_3 && (*(local_28 + 0x23) >> 0x18 == DAT_004c9754)) &&
                        ((*(local_28 + 0x3a) & 0x1) != 0x0)) &&
                        ((*(*(&DAT_00582938 +
                            (*(local_28 + 0x25) >> 0x18) * 0x4 + (local_28[0x9] >> 0x18) * 0x18) +
                            0xa5) != 0x0 && (iVar1 = FUN_00433083(local_28[0xa] >> 0x10,param_4), iVar1 != 0x0)))))) &&
                    ((*(*(&DAT_00582938 +
                        (*(local_24 + 0x25) >> 0x18) * 0x4 + (local_24[0x9] >> 0x18) * 0x18) +
                        0x41) != 0x3 || ((local_28 + 0x27) == '\x1a')))) {
                    local_70 = 0x0;
                    LAB_0043340e:
                    if (local_70 < 0x4) {
                        if (local_28[local_70 + 0x4] != 0x0)^ // goto LAB_00433408;
                        FUN_004841ea(local_24,local_20,param_2,param_3);
                        *(local_24 + 0x3a) = *(local_24 + 0x3a) & 0xfb;
                        (local_24 + 0xf) = (local_24 + 0xf) & 0x1;
                        *(local_24 + 0x3a) = *(local_24 + 0x3a) | 0x40;
                        *(local_24 + 0x3a) =
                            *(local_24 + 0x3a) |
                                *(&DAT_004be9b0 + (*(local_24 + 0x23) >> 0x18) * 0x4);
                        (local_24 + 0x2f) = 0x0;
                        *(local_28 + 0x3a) = *(local_28 + 0x3a) & 0xfb;
                        for (local_74 = 0x0; local_74 < 0x4; local_74 = local_74 + 0x1) {
                            if (local_28[local_74 + 0x4] != 0x0) {
                                *(local_28[local_74 + 0x4] + 0x3a) = *(local_28[local_74 + 0x4] + 0x3a) & 0xfb;
                            }
                        }
                        local_28[local_70 + 0x4] = local_24;
                        for (local_2c = local_28; (local_2c != 0x0 && (local_2c[0x3] != 0x0));
                            local_2c = local_2c[0x3]) {
                        }
                        if (local_2c != 0x0) {
                            FUN_00431efd(&local_30,local_24);
                        }
                    }
                    if (local_70 == 0x4)^ // goto LAB_004332cd;
                    break;
                }
                LAB_004332cd:
            }^
            // goto LAB_00433294;
        }
    }
    return;
    LAB_00433408:
        local_70 = local_70 + 0x1;^
    // goto LAB_0043340e;
}



fn FUN_00433545(param_1: i32,param_2: i32,param_3: i32) -> u32

{
    let mut iVar1: i32;
    let puVar2: *mut u32;
    let mut pcVar3: String;
    let mut iVar4: i32;
    let mut local_94: i32;
    let mut local_30: u32;
    let local_2c: *mut u32;
    let mut local_28: i32;
    let mut local_24: i32;

    local_28 = 0x0;
    local_24 = 0x0;
    iVar1 = (*(param_1 + 0x4) + 0x20);
    for (local_2c = (&DAT_005b8b44 + iVar1 * 0x4);
        (local_2c != 0x0 && ((local_2c + 0x8) == iVar1)); local_2c = *local_2c) {
        if (((*(local_2c + 0x3a) & 0x1) != 0x0) &&
            (((local_2c + 0x22) == param_2 && ((local_2c + 0x9) == param_3)))) {
            if ((local_2c + 0x26) != (*(param_1 + 0x4) + 0x26)) {
                if ((((local_2c + 0x26) == '\x05') &&
                    (puVar2 = FUN_00481784((local_2c + 0x8),(local_2c + 0x22),
                                           (local_2c + 0x9)), puVar2 != 0x0)) &&
                    ((puVar2 + 0xe) == 0x4)) {
                    return 0x1;
                }
                return 0x29a;
            }
            local_28 = local_28 + 0x1;
            if ((*(*(&DAT_00582938 +
                (*(local_2c + 0x25) >> 0x18) * 0x4 + (local_2c[0x9] >> 0x18) * 0x18) + 0xa5
            ) != 0x0) &&
                (*(*(&DAT_00582938 +
                    (*(local_2c + 0x25) >> 0x18) * 0x4 + (local_2c[0x9] >> 0x18) * 0x18) + 0x41)
                    == 0x4)) {
                for (local_94 = 0x0; local_94 < 0x4; local_94 = local_94 + 0x1) {
                    if (local_2c[local_94 + 0x4] == 0x0) {
                        local_24 = local_24 + 0x1;
                    }
                }
            }
        }
    }
    if (local_28 == 0x0) {
        local_30 = 0x1;
    }
    else {
        iVar1 = FUN_00432c94(param_1);
        if (local_28 + iVar1 < 0x15) {
            iVar4 = FUN_00434c46(param_1,0x3);
            if (((iVar4 == 0x0) || (local_2c == 0x0)) || ((local_2c + 0x27) == '\x1a')) {
                if (local_24 != 0x0) {
                    iVar4 = FUN_00432f5e(param_1);
                    if (iVar4 != 0x0) {
                        if (iVar1 <= local_24) {
                            return 0x2;
                        }
                        if (((&DAT_00569a98)[(*(*(param_1 + 0x4) + 0x23) >> 0x18) * 0x1e22] & 0x2) == 0x0) {
                            pcVar3 = FUN_00499050(DAT_0059679c,0x7366);
                            FUN_0049d2e0(0x0,0x1,pcVar3);
                        }
                        return 0x0;
                    }
                    if ((*(param_1 + 0x4) + 0x2a) != 0x4) {
                        if (((&DAT_00569a98)[(*(*(param_1 + 0x4) + 0x23) >> 0x18) * 0x1e22] & 0x2) == 0x0) {
                            pcVar3 = FUN_00499050(DAT_0059679c,0x7157);
                            FUN_0049d2e0(0x0,0x1,pcVar3);
                        }
                        return 0x0;
                    }
                }
                local_30 = 0x1;
            }
            else {
                local_30 = 0x1;
            }
        }
        else {
            if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
                pcVar3 = FUN_00499050(DAT_0059679c,0x714d);
                FUN_0049d2e0(0x0,0x1,pcVar3);
            }
            local_30 = 0x0;
        }
    }
    return local_30;
}



fn FUN_00433916(param_1: i32,param_2: i32,param_3: i32) -> u32

{
    let mut iVar1: i32;
    let mut pcVar2: String;
    let mut uVar3: u32;
    let local_a34: u8 [0x80];
    let mut local_9b4: String;
    let local_9b0: *mut u32;
    let local_9ac: *mut u32;
    let mut local_9a8: i32;
    let local_9a4: *mut u32;
    let local_9a0: *mut u32;
    let mut local_99c: i32;
    let mut local_998: u32;
    let local_994: *mut u32;
    let local_990: *mut u32;
    let mut local_98c: u32;
    let local_988: *mut u32;
    let local_984: *mut u32;
    let mut local_980: i32;
    let mut local_97c: u32;
    let mut local_978: u32;
    let mut local_974: u32;
    let mut local_970: i32;
    let mut local_96c: i32;
    let mut local_968: *mut u8;
    let mut local_964: i32;
    let mut local_960: String;
    let local_95c: u8 [0x80];
    let mut local_8dc: String;
    let local_8d8: *mut u32;
    let local_8d4: *mut u32;
    let mut local_8d0: i32;
    let local_8cc: *mut u32;
    let local_8c8: *mut u32;
    let mut local_8c4: i32;
    let mut local_8c0: u32;
    let local_8bc: *mut u32;
    let local_8b8: *mut u32;
    let mut local_8b4: u32;
    let local_8b0: *mut u32;
    let local_8ac: *mut u32;
    let mut local_8a8: i32;
    let mut local_8a4: u32;
    let mut local_8a0: u32;
    let mut local_89c: u32;
    let mut local_898: i32;
    let mut local_894: i32;
    let mut local_890: *mut u8;
    let mut local_88c: i32;
    let mut local_888: u32;
    let local_884: *mut u32;
    let local_880: *mut u32;
    let mut local_87c: u32;
    let local_878: *mut u32;
    let local_874: *mut u32;
    let mut local_870: u32;
    let local_86c: *mut u32;
    let local_868: *mut u32;
    let local_864: *mut u32;
    let local_860: *mut u32;
    let mut local_85c: u32;
    let local_858: u8 [0x830];
    let local_28: *mut u32;
    let mut local_24: i32;
    let mut local_20: i32;
    ushort *local_1c;
    ushort *local_18;
    let mut local_14: u32;

    local_1c = (*(param_1 + 0x4) + 0x20);
    local_14 = local_1c & 0xffff0000 | *local_1c;
    local_24 = *local_1c;
    local_20 = 0x0;
    local_28 = (&DAT_005b8b44 + local_24 * 0x4);
    local_18 = local_1c;
    loop {
    if (local_28 == 0x0) {
        return 0x29a;
    }
    local_864 = local_28 + 0x8;
    local_85c = local_864 & 0xffff0000 | local_864;
    if (local_864 != local_24) {
        return 0x29a;
    }
    local_86c = local_28 + 0x8;
    local_870 = local_86c & 0xffff0000 | (local_28 + 0x22);
    if ((local_28 + 0x22) == param_2) {
        local_878 = local_28 + 0x8;
        local_87c = local_878 & 0xffff0000 | (local_28 + 0x9);
        if ((local_28 + 0x9) == param_3) {
            local_884 = local_28 + 0x8;
            local_888 = *(local_28 + 0x3a) & 0x1;
            if (local_888 != 0x0) {
                if ((local_28 + 0x26) == (*(param_1 + 0x4) + 0x26)) {
                    return 0x0;
                }
                if ((DAT_004c9754 < 0x5) && (local_20 == 0x0)) {
                    local_20 = 0x1;
                    local_880 = local_884;
                    local_874 = local_878;
                    local_868 = local_86c;
                    local_860 = local_864;
                    if ((local_28 + 0x26) == '\b') {
                        FUN_00423530(DAT_004c9754);
                    }
                    else {
                        if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
                            local_898 = *(local_28 + 0x23) >> 0x18;
                            local_890 = &DAT_004d55a8;
                            local_894 = DAT_004c9754;
                            local_89c = (byte)(&DAT_004d55a8)[DAT_004c9754 * 0xe + local_898];
                            local_88c = local_898;
                            if (local_89c == 0x0) {
                                iVar1 = FUN_00410fb3(param_1,0x1);
                                if (iVar1 == 0x0) {
                                    return 0x0;
                                }
                                local_8a8 = *(local_28 + 0x23) >> 0x18;
                                local_8b0 = local_28 + 0x8;
                                local_8b4 = *(local_28 + 0x3a) & 0x1;
                                local_8bc = local_28 + 0x8;
                                local_8c0 = local_8bc & 0xffff0000 | (local_28 + 0x9);
                                local_8c4 = (local_28 + 0x9);
                                local_8cc = local_28 + 0x8;
                                local_8a0 = local_8cc & 0xffff0000 | (local_28 + 0x22);
                                local_8d0 = (local_28 + 0x22);
                                local_8d8 = local_28 + 0x8;
                                local_8a4 = local_8d8 & 0xffff0000 | local_8d8;
                                local_8d4 = local_8d8;
                                local_8c8 = local_8cc;
                                local_8b8 = local_8bc;
                                local_8ac = local_8b0;
                                FUN_00450dbf(local_858,local_8d8,local_8d0,local_8c4,0x0,local_8b4,
                                             local_8a8,0x1);
                                iVar1 = FUN_00410fb3(param_1,0x1);
                                if (iVar1 == 0x0) {
                                    return 0x0;
                                }
                                iVar1 = FUN_00452e41(local_858);
                                if (iVar1 == 0x0) {
                                    FUN_00499050(DAT_0059679c,(*(local_28 + 0x23) >> 0x18) + 0x414);
                                    if (*(local_28 + 0x23) >> 0x18 < 0x5) {
                                        local_960 = FUN_00499050(DAT_0059679c,0x713e);
                                    }
                                    else {
                                        local_960 = FUN_00499050(DAT_0059679c,0x713a);
                                    }
                                    pcVar2 = FUN_00499050(DAT_0059679c,0x734b);
                                    FUN_0049c2e0(local_95c,pcVar2);
                                }
                                else {
                                    FUN_00499050(DAT_0059679c,(*(local_28 + 0x23) >> 0x18) + 0x414);
                                    if (*(local_28 + 0x23) >> 0x18 < 0x5) {
                                        local_8dc = FUN_00499050(DAT_0059679c,0x713e);
                                    }
                                    else {
                                        local_8dc = FUN_00499050(DAT_0059679c,0x713a);
                                    }
                                    pcVar2 = FUN_00499050(DAT_0059679c,0x737c);
                                    FUN_0049c2e0(local_95c,pcVar2);
                                }
                                uVar3 = FUN_0049d2e0(0x0,0x3,local_95c);
                                if (uVar3 == 0x0) {
                                    return 0x0;
                                }
                            }
                            else {
                                local_970 = *(local_28 + 0x23) >> 0x18;
                                local_968 = &DAT_004d55a8;
                                local_96c = DAT_004c9754;
                                local_974 = (byte)(&DAT_004d55a8)[DAT_004c9754 * 0xe + local_970];
                                local_964 = local_970;
                                if (local_974 == 0x1) {
                                    iVar1 = FUN_00410fb3(param_1,0x1);
                                    if (iVar1 == 0x0) {
                                        return 0x0;
                                    }
                                    local_980 = *(local_28 + 0x23) >> 0x18;
                                    local_988 = local_28 + 0x8;
                                    local_98c = *(local_28 + 0x3a) & 0x1;
                                    local_994 = local_28 + 0x8;
                                    local_998 = local_994 & 0xffff0000 | (local_28 + 0x9);
                                    local_99c = (local_28 + 0x9);
                                    local_9a4 = local_28 + 0x8;
                                    local_978 = local_9a4 & 0xffff0000 | (local_28 + 0x22);
                                    local_9a8 = (local_28 + 0x22);
                                    local_9b0 = local_28 + 0x8;
                                    local_97c = local_9b0 & 0xffff0000 | local_9b0;
                                    local_9ac = local_9b0;
                                    local_9a0 = local_9a4;
                                    local_990 = local_994;
                                    local_984 = local_988;
                                    FUN_00450dbf(local_858,local_9b0,local_9a8,local_99c,0x0,local_98c,
                                                 local_980,0x1);
                                    iVar1 = FUN_00452e41(local_858);
                                    if (iVar1 == 0x0) {
                                        FUN_00499050(DAT_0059679c,(*(local_28 + 0x23) >> 0x18) + 0x414);
                                        pcVar2 = FUN_00499050(DAT_0059679c,0x7365);
                                        FUN_0049c2e0(local_a34,pcVar2);
                                    }
                                    else {
                                        FUN_00499050(DAT_0059679c,(*(local_28 + 0x23) >> 0x18) + 0x414);
                                        if (*(local_28 + 0x23) >> 0x18 < 0x5) {
                                            local_9b4 = FUN_00499050(DAT_0059679c,0x713e);
                                        }
                                        else {
                                            local_9b4 = FUN_00499050(DAT_0059679c,0x713a);
                                        }
                                        pcVar2 = FUN_00499050(DAT_0059679c,0x737d);
                                        FUN_0049c2e0(local_a34,pcVar2);
                                    }
                                    uVar3 = FUN_0049d2e0(0x0,0x3,local_a34);
                                    if (uVar3 == 0x0) {
                                        return 0x0;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    local_28 = *local_28;
} while( true );
}



// WARNING: Could not reconcile some variable overlaps

fn FUN_0043402e(param_1: i32,param_2: i32,param_3: i32)

{
    let mut iVar1: i32;
    let mut local_2c: i32;
    let mut local_24: i32;

    local_24 = FUN_0045aa12(param_1,param_2,param_3,0x0);
    iVar1 = FUN_00432aca(param_1,0x1a);
    if (local_24 == 0x0) {
        local_24 = 0x1;
    }
    for (local_2c = *(param_1 + 0x4); local_2c != 0x0; local_2c = *(local_2c + 0x8)) {
        if (((*(local_2c + 0x3a) & 0x40) == 0x0) && (((local_2c + 0x2a) != 0x3 || (iVar1 == 0x0)))) {
            (local_2c + 0x2f) = (local_2c + 0x2f) - local_24;
            if (*(local_2c + 0x2c) >> 0x18 < 0x0) {
                (local_2c + 0x2f) = 0x0;
            }
        }
    }
    return;
}



fn FUN_00434129(param_1: *mut u32,param_2: i32,param_3: i32) -> u32

{
    let sVar1: i16;
    let sVar2: i16;
    let mut uVar3: u32;
    let mut bVar4: bool;
    let mut iVar5: i32;
    let mut pcVar6: String;
    let mut local_6c: u32;
    i32 **local_68;
    let local_64: *mut u32;
    let local_60: *mut u32;
    let mut local_44: i32;

    local_64 = 0x0;
    local_60 = 0x0;
    bVar4 = false;
    sVar1 = (param_1[0x1] + 0x22);
    sVar2 = (param_1[0x1] + 0x24);
    local_44 = 0x0;
    if (((*((*(param_1[0x1] + 0x23) >> 0x18) * 0x1e22 + 0x569b77) & 0x1) != 0x0) &&
        (iVar5 = FUN_00432aca(param_1,0x2d), iVar5 != 0x0)) {
        uVar3 = *(*(&DAT_004d7d50 + param_2 * 0x4 + (param_1[0x1] + 0x20) * 0x3890) + param_3 * 0x4
            + 0x4);
        if ((*(*(&DAT_004d7d50 +
            (param_1[0x1] + 0x22) * 0x4 + (param_1[0x1] + 0x20) * 0x3890) +
            (param_1[0x1] + 0x24) * 0x4 + 0x4) & 0x80) != 0x0) {
            local_64 = FUN_00481784((param_1[0x1] + 0x20),(param_1[0x1] + 0x22),
                                    (param_1[0x1] + 0x24));
        }
        if ((uVar3 & 0x80) != 0x0) {
            local_60 = FUN_00481784((param_1[0x1] + 0x20),param_2,param_3);
        }
        local_44 = FUN_00486432(0x2d,(param_1[0x1] + 0x20),param_2,param_3);
    }
    for (local_68 = param_1[0x1]; local_68 != 0x0; local_68 = local_68[0x2]) {
        FUN_004841ea(local_68,(local_68 + 0x8),param_2,param_3);
        iVar5 = FUN_00459e8f(local_68);
        if ((!bVar4) && (iVar5 != 0x0)) {
            bVar4 = true;
        }
        FUN_0044add9(local_68);
    }
    if ((local_64 == 0x0) || ((local_64 + 0xe) != 0x0)) {
        if ((local_60 != 0x0) &&
            ((((local_60 + 0xe) == 0x0 && (local_44 == 0x0)) &&
                (FUN_004883c0((param_1[0x1] + 0x20),*(param_1[0x1] + 0x23) >> 0x18,0x14),
                 (*((*(param_1[0x1] + 0x23) >> 0x18) * 0x1e22 + 0x569b77) & 0x10) != 0x0)))) {
            pcVar6 = FUN_00499050(DAT_0059679c,0x73f9);
            FUN_0049d2e0(0x0,0x1,pcVar6);
        }
    }
    else {
        iVar5 = FUN_00486432(0x2d,(param_1[0x1] + 0x20),sVar1,sVar2);
        if ((iVar5 == 0x0) &&
            (FUN_004883c0((param_1[0x1] + 0x20),*(param_1[0x1] + 0x23) >> 0x18,-0x14),
             (*((*(param_1[0x1] + 0x23) >> 0x18) * 0x1e22 + 0x569b77) & 0x10) != 0x0)) {
            pcVar6 = FUN_00499050(DAT_0059679c,0x73f8);
            FUN_0049d2e0(0x0,0x1,pcVar6);
        }
    }
    iVar5 = FUN_00432aca(param_1,0x35);
    if ((iVar5 != 0x0) && (((&DAT_00569a98)[(*(param_1[0x1] + 0x23) >> 0x18) * 0x1e22] & 0x2) != 0x0)) {
        FUN_0043b7fd((param_1[0x1] + 0x20),(param_1[0x1] + 0x22),
                     (param_1[0x1] + 0x24));
    }
    DAT_0059a9f4 = 0x0;
    if ((bVar4) && (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0)) {
        FUN_004a2d6b();
        *param_1 = *param_1 & 0xfffffffe;
        local_6c = 0x0;
    }
    else {
        local_6c = 0x1;
    }
    return local_6c;
}



fn FUN_004348e8(param_1: i32,param_2: i32,param_3: i32)

{
    let sVar1: i16;
    let sVar2: i16;
    let mut uVar3: u32;
    let mut iVar4: i32;
    let mut local_7c: i32;
    let local_54: *mut u32;

    iVar4 = (*(param_1 + 0x4) + 0x20);
    sVar1 = (*(param_1 + 0x4) + 0x22);
    sVar2 = (*(param_1 + 0x4) + 0x24);
    uVar3 = *(*(param_1 + 0x4) + 0x3a);
    *(param_1 + 0x4) = 0x0;
    local_54 = (&DAT_005b8b44 + iVar4 * 0x4);
    loop {
    if ((local_54 == 0x0) || ((local_54 + 0x8) != iVar4)) {
        return;
    }
    if (((local_54 + 0x22) == sVar1) &&
        (((local_54 + 0x9) == sVar2 && ((*(local_54 + 0x3a) & 0x40) == 0x0)))) {
        if ((*(local_54 + 0x47) & 0x2) == 0x0) {
            if ((param_2 == 0x0) || ((*(local_54 + 0x3a) & 0x4) == 0x0)) {
                if ((param_3 == 0x0) || ((local_54 + 0x2f) != '\0')) {
                    LAB_00434b1e:
                    if ((uVar3 & 0x1) == 0x0) {
                        if ((*(local_54 + 0x3a) & 0x1) == 0x0) {
                            LAB_00434b92:
                                FUN_00431efd(param_1,local_54);
                        }
                    }
                    else {
                        if ((*(local_54 + 0x3a) & 0x1) != 0x0)^ // goto LAB_00434b92;
                    }
                }
                else {
                    if (*(*(&DAT_00582938 +
                        (*(local_54 + 0x25) >> 0x18) * 0x4 + (local_54[0x9] >> 0x18) * 0x18) +
                        0xa5) == 0x0) {
                        if ((*(local_54 + 0x3a) & 0x40) != 0x0)^ // goto LAB_00434b1e;
                    }
                    else {
                        for (local_7c = 0x0;
                            local_7c <
                            *(*(&DAT_00582938 +
                                (*(local_54 + 0x25) >> 0x18) * 0x4 + (local_54[0x9] >> 0x18) * 0x18)
                                + 0xa5); local_7c = local_7c + 0x1) {
                            if (local_54[local_7c + 0x4] != 0x0) {
                                *(local_54[local_7c + 0x4] + 0x47) = *(local_54[local_7c + 0x4] + 0x47) | 0x2;
                            }
                        }
                    }
                }
            }
        }
        else {
            *(local_54 + 0x47) = *(local_54 + 0x47) & 0xfd;
        }
    }
    local_54 = *local_54;
} while( true );
}



fn FUN_00434baf(param_1: i32) -> i32

{
let mut local_14: i32;

local_14 = *(param_1 + 0x4);
while( true ) {
if (local_14 == 0x0) {
return 0x0;
}
if ((local_14 + 0x27) == '5') break;
local_14 = *(local_14 + 0x8);
}
return local_14;
}



fn FUN_00434bfa(param_1: i32) -> u32

{
    let mut local_14: i32;

    local_14 = *(param_1 + 0x4);
    while( true ) {
        if (local_14 == 0x0) {
            return 0x1;
        }
        if ((*(local_14 + 0x3a) & 0x4) == 0x0) break;
        local_14 = *(local_14 + 0x8);
    }
    return 0x0;
}



fn FUN_00434c46(param_1: i32,param_2: i32) -> u32

{
    let mut local_14: i32;

    local_14 = *(param_1 + 0x4);
    while( true ) {
        if (local_14 == 0x0) {
            return 0x1;
        }
        if (*(*(&DAT_00582938 +
            (*(local_14 + 0x25) >> 0x18) * 0x4 + (*(local_14 + 0x24) >> 0x18) * 0x18) + 0x41)
            != param_2) break;
        local_14 = *(local_14 + 0x8);
    }
    return 0x0;
}



fn FUN_00434cb1(param_1: i32) -> u32

{
    let mut local_18: i32;
    let mut local_14: u32;

    local_14 = 0x0;
    for (local_18 = *(param_1 + 0x4); local_18 != 0x0; local_18 = *(local_18 + 0x8)) {
        if ((((local_18 + 0x2a) == 0x3) && ((*(local_18 + 0x3b) & 0x10) != 0x0)) &&
            ((*(local_18 + 0x3b) & 0x40) == 0x0)) {
            *(local_18 + 0x3b) = *(local_18 + 0x3b) | 0x40;
            local_14 = 0x1;
        }
    }
    return local_14;
}



fn FUN_00434d1f(param_1: i32) -> u32

{
    let mut local_14: i32;

    local_14 = *(param_1 + 0x4);
    loop {
    if (local_14 == 0x0) {
        return 0x1;
    }
    if ((*(local_14 + 0x3a) & 0x40) == 0x0) {
        if (*(*(&DAT_00582938 +
            (*(local_14 + 0x25) >> 0x18) * 0x4 + (*(local_14 + 0x24) >> 0x18) * 0x18) +
            0x99) == 0x0) {
            return 0x0;
        }
        if ((((local_14 + 0x2a) != 0x7) && ((local_14 + 0x2a) != 0x5)) &&
            ((local_14 + 0x2a) != 0x9)) {
            return 0x0;
        }
    }
    local_14 = *(local_14 + 0x8);
} while( true );
}



fn FUN_00434de1(param_1: i32) -> i32

{
let mut local_14: i32;

for (local_14 = param_1; *(local_14 + 0xc) != 0x0; local_14 = *(local_14 + 0xc)) {
}
return local_14;
}



fn FUN_00434e1a(param_1: i32,param_2: i32,param_3: i32,param_4: i32) -> u32

{
    byte *pbVar1;
    let mut local_24: i32;
    let mut local_20: u32;
    let mut local_1c: u32;
    let mut local_14: i32;

    FUN_00431d31(&local_20);
    for (local_14 = *(param_1 + 0x4); local_14 != 0x0; local_14 = *(local_14 + 0x8)) {
        if (*(*(&DAT_00582938 +
            (*(local_14 + 0x25) >> 0x18) * 0x4 + (*(local_14 + 0x24) >> 0x18) * 0x18) + 0xa5)
            != 0x0) {
            for (local_24 = 0x0;
                local_24 <
                *(*(&DAT_00582938 +
                    (*(local_14 + 0x25) >> 0x18) * 0x4 + (*(local_14 + 0x24) >> 0x18) * 0x18) +
                    0xa5); local_24 = local_24 + 0x1) {
                if (*(local_24 * 0x4 + local_14 + 0x10) != 0x0) {
                    pbVar1 = (*(local_24 * 0x4 + local_14 + 0x10) + 0x3a);
                    *pbVar1 = *pbVar1 & 0xbf;
                    FUN_004841ea(*(i32 ***)(local_24 * 0x4 + local_14 + 0x10),(local_14 + 0x20),param_2,param_3);
                    if (param_4 == 0x0) {
                        FUN_00431dec(param_1,*(local_24 * 0x4 + local_14 + 0x10));
                    }
                    else {
                        FUN_00431efd(&local_20,*(local_24 * 0x4 + local_14 + 0x10));
                    }
                    *(local_24 * 0x4 + local_14 + 0x10) = 0x0;
                }
            }
        }
    }
    return local_1c;
}



fn FUN_00434f84(param_1: i32) -> i32

{
let mut local_20: i32;
let mut local_18: i32;
let mut local_14: i32;

local_14 = 0x0;
for (local_18 = *(param_1 + 0x4); local_18 != 0x0; local_18 = *(local_18 + 0x8)) {
if (*(*(&DAT_00582938 +
(*(local_18 + 0x25) >> 0x18) * 0x4 + (*(local_18 + 0x24) >> 0x18) * 0x18) + 0xa5)
!= 0x0) {
for (local_20 = 0x0;
local_20 <
*(*(&DAT_00582938 +
(*(local_18 + 0x25) >> 0x18) * 0x4 + (*(local_18 + 0x24) >> 0x18) * 0x18) +
0xa5); local_20 = local_20 + 0x1) {
if (*(local_20 * 0x4 + local_18 + 0x10) != 0x0) {
local_14 = local_14 + 0x1;
}
}
}
}
return local_14;
}



fn FUN_00435045(param_1: i32)

{
    let mut local_14: i32;

    for (local_14 = *(param_1 + 0x4); local_14 != 0x0; local_14 = *(local_14 + 0x8)) {
        *(local_14 + 0x3a) = *(local_14 + 0x3a) | 0x8;
    }
    return;
}



fn FUN_0043507c(param_1: i32)

{
    let mut local_14: i32;

    for (local_14 = *(param_1 + 0x4); local_14 != 0x0; local_14 = *(local_14 + 0x8)) {
        *(local_14 + 0x3a) = *(local_14 + 0x3a) & 0xf7;
    }
    return;
}



fn FUN_004350b3(param_1: i32,param_2: i32,param_3: i32) -> u32

{
    let puVar1: *mut u32;
    let mut iVar2: i32;
    let mut local_40: i32;
    let mut local_14: i32;

    local_14 = *(param_1 + 0x4);
    loop {
    if (local_14 == 0x0) {
        return 0x0;
    }
    if (((*(*(&DAT_00582938 +
        (*(local_14 + 0x25) >> 0x18) * 0x4 + (*(local_14 + 0x24) >> 0x18) * 0x18) +
        0xa5) != 0x0) && ((local_14 + 0x2a) == 0x4)) &&
        ((*(*(&DAT_004d7d50 + param_2 * 0x4 + (local_14 + 0x20) * 0x3890) + param_3 * 0x4) & 0xf
        ) != 0x0)) {
        puVar1 = FUN_00481784((local_14 + 0x20),param_2,param_3);
        if (puVar1 != 0x0) {
            iVar2 = FUN_00481a44(puVar1);
            if (iVar2 == 0x0) {
                if ((puVar1 + 0xe) == 0x2) {
                    return 0x0;
                }
                if (*(puVar1 + 0xe) >> 0x10 != DAT_004c9754) {
                    return 0x0;
                }
            }
            else {
                if (((puVar1 + 0xe) == 0xf) || ((puVar1 + 0xe) == 0x10)) {
                    return 0x0;
                }
            }
        }
        for (local_40 = 0x0;
            local_40 <
            *(*(&DAT_00582938 +
                (*(local_14 + 0x25) >> 0x18) * 0x4 + (*(local_14 + 0x24) >> 0x18) * 0x18) +
                0xa5); local_40 = local_40 + 0x1) {
            if (*(local_40 * 0x4 + local_14 + 0x10) != 0x0) {
                return 0x1;
            }
        }
    }
    local_14 = *(local_14 + 0x8);
} while( true );
}



fn FUN_00435263(param_1: i32,param_2: i32) -> u32

{
    let local_4c: *mut u32;
    let local_44: *mut u32;

    if ((*(param_1 + 0x4) + 0x4) != DAT_005967b0) {
        if (param_2 == 0x0) {
            local_4c = (*(param_1 + 0x4) + 0x4);
        }
        else {
            local_4c = *(param_1 + 0x4);
        }
        local_44 = local_4c;
        while (local_44 != 0x0) {
            if (((((local_44 + 0x8) != (*(param_1 + 0x4) + 0x20)) ||
                ((local_44 + 0x22) != (*(param_1 + 0x4) + 0x22))) ||
                ((local_44 + 0x9) != (*(param_1 + 0x4) + 0x24))) &&
                (((local_44[0x3] == 0x0 && (*(local_44 + 0x23) >> 0x18 == DAT_004c9754)) &&
                    (local_44 != DAT_005967b0)))) {
                (param_1 + 0x4) = local_44;
                return 0x1;
            }
            if (param_2 == 0x0) {
                local_44 = local_44[0x1];
            }
            else {
                local_44 = *local_44;
            }
        }
    }
    return 0x0;
}



fn FUN_00435409(param_1: i32)

{
    let mut local_14: i32;

    for (local_14 = *(param_1 + 0x4); local_14 != 0x0; local_14 = *(local_14 + 0x8)) {
        *(local_14 + 0x3b) = *(local_14 + 0x3b) & 0xdf;
    }
    return;
}



fn FUN_00435440(param_1: i32) -> u32

{
    let mut local_14: i32;

    local_14 = *(param_1 + 0x4);
    while( true ) {
        if (local_14 == 0x0) {
            return 0x0;
        }
        if ((*(local_14 + 0x3c) & 0x1) != 0x0) break;
        local_14 = *(local_14 + 0x8);
    }
    return 0x1;
}



fn FUN_0043548c(param_1: i32) -> u32

{
    let mut local_14: i32;

    local_14 = *(param_1 + 0x4);
    while( true ) {
        if (local_14 == 0x0) {
            return 0x0;
        }
        if ((*(local_14 + 0x3b) & 0x1) != 0x0) break;
        local_14 = *(local_14 + 0x8);
    }
    return 0x1;
}



fn FUN_004354d8(param_1: i32) -> u32

{
    let mut local_14: i32;

    local_14 = *(param_1 + 0x4);
    while( true ) {
        if (local_14 == 0x0) {
            return 0x0;
        }
        if ((*(local_14 + 0x3b) & 0x4) != 0x0) break;
        local_14 = *(local_14 + 0x8);
    }
    return 0x1;
}



fn FUN_00435524(param_1: u32,param_2: u32)

{
    let mut local_11c: *mut u32 [0x11];
    let ppuStack215: *mut *mut u8;;
    let mut local_2b: String;;
    let mut local_24: u32;
    let local_20: *mut i32;;
    u32 **local_1c;

    DAT_00599e14 = param_2;
    DAT_00599e18 = param_1;
    local_20 = FUN_004990e0(local_11c,0x0,s_efs_res_004c1866,s_ResDlg_004c185f);
    FUN_0049bb50(local_11c,FUN_004355ca);
    local_1c = local_11c;
    local_24 = 0x0;
    ppuStack215 = &PTR_FUN_004c3d34;
    if (local_2b != 0x0) {
        FUN_00499b30(local_11c,local_2b);
    }
    FUN_0049a1c0(local_11c,0x1);
    return;
}



fn FUN_004355ca(param_1: i32,param_2: u32,param_3: i32) -> u32

{
    let mut pcVar1: String;
    let local_98: u8 [0x84];
    let mut local_14: u32;

    local_14 = param_2;
    if (0x404 < param_2) {
        if (param_2 < 0x406) {
            FUN_004953d7();
            FUN_00496ee6(&DAT_00596a58 + DAT_00599e14 * 0x3da,*(param_1 + 0x1d) + 0x5,*(param_1 + 0x21) + 0x5,
                         0x22,0x1d);
            if (DAT_00599e14 == 0x1) {
                FUN_00497567(*(param_1 + 0x1d) + 0x2c,*(param_1 + 0x21) + 0x5,0x4d6b30,0x96,0xcaccce,-0x1,0xcaccce
                             ,LPCSTR_005b9218,0x10);
            }
            else {
                FUN_00497567(*(param_1 + 0x1d) + 0x2c,*(param_1 + 0x21) + 0x5,DAT_00599e14 * 0xa8 + 0x4d6a78,0x96,
                             0xcaccce,-0x1,0xcaccce,LPCSTR_005b9218,0x10);
            }
            FUN_00462571((&DAT_00568210 + DAT_004c9754 * 0x1e22),DAT_00599e18,DAT_00599e14);
            FUN_0049c2e0(local_98,&DAT_004c186e);
            FUN_00497567(*(param_1 + 0x1d) + 0x2c,*(param_1 + 0x21) + 0x17,local_98,0x96,0xcaccce,-0x1,
                         0xcaccce,LPCSTR_005b9218,0x10);
            FUN_00497567(*(param_1 + 0x1d) + 0x5,*(param_1 + 0x21) + 0xa0,DAT_00599e14 * 0xa8 + 0x4d6a98,0xfa,
                         0xcaccce,-0x1,0xcaccce,LPCSTR_005b9218,0x10);
            pcVar1 = FUN_00499050(DAT_0059679c,0x73af);
            FUN_0049c2e0(local_98,pcVar1);
            FUN_00497567(*(param_1 + 0x1d) + 0x5,*(param_1 + 0x21) + 0xb4,local_98,0xfa,0xcaccce,-0x1,
                         0xcaccce,LPCSTR_005b9218,0x10);
            FUN_00435cfd(param_1,DAT_00599e14);
            FUN_00438010(param_1,DAT_00599e14);
            FUN_0049536f();
        }
        else {
            if ((param_2 == 0x407) && (param_3 == 0x64)) {
                FUN_0049c140(param_1,0x1);
            }
        }
    }
    return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0043586e(param_1: u32)

{
    let mut local_11c: *mut u32 [0x11];
    let ppuStack215: *mut *mut u8;;
    let mut local_2b: String;;
    let local_24: *mut i32;;
    u32 **local_20;
    let mut local_1c: u32;

    _DAT_00599e1c = param_1;
    local_24 = FUN_004990e0(local_11c,0x0,s_efs_res_004c187a,s_SideHelp_004c1871);
    FUN_0049bb50(local_11c,FUN_0043590c);
    local_20 = local_11c;
    local_1c = 0x0;
    ppuStack215 = &PTR_FUN_004c3d34;
    if (local_2b != 0x0) {
        FUN_00499b30(local_11c,local_2b);
    }
    FUN_0049a1c0(local_11c,0x1);
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0043590c(param_1: *mut *mut u32,param_2: u32,param_3: i32) -> u32

{
    let mut pcVar1: String;
    let mut iVar2: i32;
    let mut uVar3: u32;
    let mut iVar4: i32;
    let mut uVar5: u32;
    let puVar6: *mut u32;
    let bVar7: u8;

    if (param_2 < 0x405) {
        if ((param_2 == 0x401) && (_DAT_00599e1c == 0x0)) {
            FUN_0049bf80(param_1,0x3ee,0x414,0x0,0x0);
            FUN_0049bf80(param_1,0x3ee,0x410,0x0,0x0);
            FUN_0049bf80(param_1,0x3ef,0x414,0x0,0x0);
            FUN_0049bf80(param_1,0x3ef,0x410,0x0,0x0);
            FUN_0049a770(param_1,0x416,*(param_1 + 0x25),*(param_1 + 0x29) + -0x55
            );
            FUN_0049bf80(param_1,0x64,0x415,0x226,0xd7);
        }
    }
    else {
        if (param_2 < 0x406) {
            FUN_004953d7();
            bVar7 = 0x10;
            uVar5 = 0xcaccce;
            iVar4 = -0x1;
            uVar3 = 0xcaccce;
            iVar2 = 0x190;
            puVar6 = LPCSTR_005b9218;
            pcVar1 = FUN_00499050(DAT_0059679c,0x7468);
            FUN_00497567(*(param_1 + 0x1d) + 0x28,*(param_1 + 0x21) + 0xa,pcVar1,iVar2,uVar3,
                         iVar4,uVar5,puVar6,bVar7);
            bVar7 = 0x10;
            uVar5 = 0xcaccce;
            iVar4 = -0x1;
            uVar3 = 0xcaccce;
            iVar2 = 0x190;
            puVar6 = LPCSTR_005b9218;
            pcVar1 = FUN_00499050(DAT_0059679c,0x7469);
            FUN_00497567(*(param_1 + 0x1d) + 0x28,*(param_1 + 0x21) + 0x28,pcVar1,iVar2,uVar3,
                         iVar4,uVar5,puVar6,bVar7);
            bVar7 = 0x10;
            uVar5 = 0xcaccce;
            iVar4 = -0x1;
            uVar3 = 0xcaccce;
            iVar2 = 0x190;
            puVar6 = LPCSTR_005b9218;
            pcVar1 = FUN_00499050(DAT_0059679c,0x746a);
            FUN_00497567(*(param_1 + 0x1d) + 0x28,*(param_1 + 0x21) + 0x46,pcVar1,iVar2,uVar3,
                         iVar4,uVar5,puVar6,bVar7);
            bVar7 = 0x10;
            uVar5 = 0xcaccce;
            iVar4 = -0x1;
            uVar3 = 0xcaccce;
            iVar2 = 0x190;
            puVar6 = LPCSTR_005b9218;
            pcVar1 = FUN_00499050(DAT_0059679c,0x746b);
            FUN_00497567(*(param_1 + 0x1d) + 0x28,*(param_1 + 0x21) + 0x64,pcVar1,iVar2,uVar3,
                         iVar4,uVar5,puVar6,bVar7);
            bVar7 = 0x10;
            uVar5 = 0xcaccce;
            iVar4 = -0x1;
            uVar3 = 0xcaccce;
            iVar2 = 0x190;
            puVar6 = LPCSTR_005b9218;
            pcVar1 = FUN_00499050(DAT_0059679c,0x746c);
            FUN_00497567(*(param_1 + 0x1d) + 0x28,*(param_1 + 0x21) + 0x82,pcVar1,iVar2,uVar3,
                         iVar4,uVar5,puVar6,bVar7);
            bVar7 = 0x10;
            uVar5 = 0xcaccce;
            iVar4 = -0x1;
            uVar3 = 0xcaccce;
            iVar2 = 0x190;
            puVar6 = LPCSTR_005b9218;
            pcVar1 = FUN_00499050(DAT_0059679c,0x746d);
            FUN_00497567(*(param_1 + 0x1d) + 0x28,*(param_1 + 0x21) + 0xa0,pcVar1,iVar2,uVar3,
                         iVar4,uVar5,puVar6,bVar7);
            if (_DAT_00599e1c != 0x0) {
                bVar7 = 0x10;
                uVar5 = 0xcaccce;
                iVar4 = -0x1;
                uVar3 = 0xcaccce;
                iVar2 = 0x190;
                puVar6 = LPCSTR_005b9218;
                pcVar1 = FUN_00499050(DAT_0059679c,0x746e);
                FUN_00497567(*(param_1 + 0x1d) + 0x28,*(param_1 + 0x21) + 0xbe,pcVar1,iVar2,uVar3,
                             iVar4,uVar5,puVar6,bVar7);
                bVar7 = 0x10;
                uVar5 = 0xcaccce;
                iVar4 = -0x1;
                uVar3 = 0xcaccce;
                iVar2 = 0x190;
                puVar6 = LPCSTR_005b9218;
                pcVar1 = FUN_00499050(DAT_0059679c,0x746f);
                FUN_00497567(*(param_1 + 0x1d) + 0x28,*(param_1 + 0x21) + 0xdc,pcVar1,iVar2,uVar3,
                             iVar4,uVar5,puVar6,bVar7);
            }
            FUN_0049536f();
        }
        else {
            if (0x406 < param_2) {
                if (param_2 < 0x408) {
                    if (param_3 == 0x64) {
                        FUN_0049c140(param_1,0x1);
                    }
                }
                else {
                    if (param_2 == 0x40c) {
                        FUN_004953d7();
                        FUN_004a08c5(s_pcx_bg0_pcx_004c1882,*(param_1 + 0x1d),*(param_1 + 0x21),
                                     *(param_1 + 0x25),*(param_1 + 0x29),0x0,0x0,0x0,0x1);
                        FUN_0049536f();
                        return 0x1;
                    }
                }
            }
        }
    }
    return 0x0;
}



// WARNING: Removing unreachable block (ram,0x00435f13)

fn FUN_00435cfd(param_1: i32,param_2: u32)

{
    let mut pcVar1: String;
    let local_ec: u8 [0x80];
    u32 auStack108 [0xd];
    let local_38: u8 [0x10];
    let mut local_28: i32;
    let mut local_24: i32;
    let local_20: *mut u32;
    let mut local_1c: i32;
    let mut local_18: u32;
    let mut local_14: u32;

    local_20 = FUN_0049c2c9(0x780);
    for (local_28 = 0x0; local_28 < 0xd; local_28 = local_28 + 0x1) {
        auStack108[local_28] = 0x0;
    }
    switch(param_2) {
    case 0x5:
        local_24 = 0xb;
    break;
    case 0x6:
        local_24 = 0x9;
    break;
    case 0x7:
        local_24 = 0x6;
    break;
    case 0x8:
        local_24 = 0x8;
    break;
    case 0x9:
        local_24 = 0x5;
    break;
    case 0xa:
        local_24 = 0x14;
    break;
    default:
    if (param_2 < 0x3) {
        if (param_2 == 0x0) {
            pcVar1 = FUN_00499050(DAT_0059679c,0x736c);
            FUN_0049c2e0(local_ec,pcVar1);^
            // goto LAB_00435f15;
        }
        if (param_2 < 0x2) {
            pcVar1 = FUN_00499050(DAT_0059679c,0x736d);
            FUN_0049c2e0(local_ec,pcVar1);^
            // goto LAB_00435f15;
        }
    }
    else {
        if (0x3 < param_2) {
            if (param_2 < 0xa) {
                if (param_2 == 0x4) {
                    pcVar1 = FUN_00499050(DAT_0059679c,0x736c);
                    FUN_0049c2e0(local_ec,pcVar1);
                }^
                // goto LAB_00435f15;
            }
            if (param_2 < 0xb) {
                pcVar1 = FUN_00499050(DAT_0059679c,0x736d);
                FUN_0049c2e0(local_ec,pcVar1);^
                // goto LAB_00435f15;
            }
            if (param_2 != 0xb)^ // goto LAB_00435f15;
        }
    }
    pcVar1 = FUN_00499050(DAT_0059679c,0x736d);
    FUN_0049c2e0(local_ec,pcVar1);
    LAB_00435f15:
        FUN_00497567(*(param_1 + 0x1d) + 0x5,*(param_1 + 0x21) + 0x8c,local_ec,0xfa,0xcaccce,-0x1,
    0xcaccce,LPCSTR_005b9218,0x10);
    return;
    case 0xc:
        local_24 = 0xc;
}
    local_1c = *(param_1 + 0x21);
    local_28 = 0x0;
    while ((local_28 < 0x3 && (*(&DAT_00583c2c + local_28 * 0x4 + local_24 * 0x384) != -0x1))) {
        local_14 = (&DAT_00583c2c + local_24 * 0x384 + local_28 * 0x4);
        local_18 = *(&DAT_00583c2c + local_24 * 0x384 + local_28 * 0x4) >> 0x10;
        FUN_00496ee6(&DAT_00596a58 + local_18 * 0x3da,local_28 * 0x66 + *(param_1 + 0x1d) + 0x86,local_1c + 0x32,0x22
                     ,0x1d);
        if (local_14 == 0x1) {
            FUN_00497567(local_28 * 0x66 + *(param_1 + 0x1d) + 0x97,local_1c + 0x59,local_18 * 0xa8 + 0x4d6a88,0x66,
                         0xcaccce,-0x1,0xcaccce,LPCSTR_005b9218,0x11);
        }
        else {
            FUN_00497567(local_28 * 0x66 + *(param_1 + 0x1d) + 0x97,local_1c + 0x59,local_18 * 0xa8 + 0x4d6a78,0x66,
                         0xcaccce,-0x1,0xcaccce,LPCSTR_005b9218,0x11);
        }
        FUN_0049c2e0(local_38,&DAT_004c188e);
        FUN_00497567(local_28 * 0x66 + *(param_1 + 0x1d) + 0x97,local_1c + 0x50,local_38,0x1e,0xcaccce,-0x1,
                     0xcaccce,LPCSTR_005b9218,0x11);
        if ((local_28 == 0x2) || (*(&DAT_00583c30 + local_28 * 0x4 + local_24 * 0x384) == -0x1)) {
            FUN_00497567(local_28 * 0x66 + *(param_1 + 0x1d) + 0xca,local_1c + 0x3c,&DAT_004c1891,0xa,0xcaccce,
                         -0x1,0xcaccce,LPCSTR_005b9218,0x10);
        }
        else {
            FUN_00497567(local_28 * 0x66 + *(param_1 + 0x1d) + 0xca,local_1c + 0x3c,&DAT_004c1893,0xa,0xcaccce,
                         -0x1,0xcaccce,LPCSTR_005b9218,0x10);
        }
        local_28 = local_28 + 0x1;
    }
    local_14 = (&DAT_00583c28 + local_24 * 0x384);
    FUN_00496ee6(&DAT_00596a58 + param_2 * 0x3da,local_28 * 0x66 + *(param_1 + 0x1d) + 0x86,local_1c + 0x32,0x22,
                 0x1d);
    if (local_14 == 0x1) {
        FUN_00497567(local_28 * 0x66 + *(param_1 + 0x1d) + 0x97,local_1c + 0x59,param_2 * 0xa8 + 0x4d6a88,0x66,
                     0xcaccce,-0x1,0xcaccce,LPCSTR_005b9218,0x11);
    }
    else {
        FUN_00497567(local_28 * 0x66 + *(param_1 + 0x1d) + 0x97,local_1c + 0x59,param_2 * 0xa8 + 0x4d6a78,0x66,
                     0xcaccce,-0x1,0xcaccce,LPCSTR_005b9218,0x11);
    }
    FUN_0049c2e0(local_38,&DAT_004c1895);
    FUN_00497567(local_28 * 0x66 + *(param_1 + 0x1d) + 0x97,local_1c + 0x50,local_38,0x1e,0xcaccce,-0x1,
                 0xcaccce,LPCSTR_005b9218,0x11);
    FUN_00401010(local_20,
                 (*(&DAT_004daab0 + DAT_00599e18 * 0x3890) * 0x940 + 0x565c90 + local_24 * 0x4),0x30,0x28
                 ,0x1c);
    FUN_00496ee6(local_20,*(param_1 + 0x1d) + 0x14,local_1c + 0x32,0x30,0x28);
    FUN_00497567(*(param_1 + 0x1d) + 0x2c,local_1c + 0x59,(&DAT_005831e8 + local_24 * 0x50),0x96,0xcaccce,-0x1
                 ,0xcaccce,LPCSTR_005b9218,0x11);
    FUN_0049af50(local_20);
    return;
}



fn FUN_004363f8(param_1: i32,param_2: i32,param_3: i32)

{
    let local_50: u8 [0x20];
    let mut local_30: i32;
    let mut local_2c: u32;
    let mut local_28: u32;
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    if ((((*(&DAT_00583c28 + param_1 * 0x384) != -0x1) && (param_1 != 0x18)) && (param_1 != 0x19)) &&
        ((param_1 != 0x12 && (param_1 != 0x13)))) {
        local_30 = 0x44;
        local_24 = param_2 + 0x33;
        local_20 = param_3 + 0xa;
        local_1c = param_3 + 0x14;
        local_18 = param_2 + 0x11;
        local_14 = 0x0;
        while ((local_14 < 0x3 && (*(&DAT_00583c2c + local_14 * 0x4 + param_1 * 0x384) != -0x1))) {
            local_28 = (&DAT_00583c2c + param_1 * 0x384 + local_14 * 0x4);
            local_2c = *(&DAT_00583c2c + param_1 * 0x384 + local_14 * 0x4) >> 0x10;
            FUN_00496ee6(&DAT_00596a58 + local_2c * 0x3da,local_14 * local_30 + param_2,param_3,0x22,0x1d);
            FUN_0049c2e0(local_50,&DAT_004c1898);
            FUN_00497567(local_14 * local_30 + local_18,local_1c,local_50,0x1e,0xcaccce,-0x1,0xcaccce,
                         LPCSTR_005b9218,0x11);
            if (*(&DAT_00583c30 + local_14 * 0x4 + param_1 * 0x384) == -0x1) {
                FUN_00497567(local_14 * local_30 + local_24,local_20,&DAT_004c189b,0xa,0xcaccce,-0x1,0xcaccce,
                             LPCSTR_005b9218,0x10);
            }
            else {
                FUN_00497567(local_14 * local_30 + local_24,local_20,&DAT_004c189d,0xa,0xcaccce,-0x1,0xcaccce,
                             LPCSTR_005b9218,0x10);
            }
            local_14 = local_14 + 0x1;
        }
        local_28 = (&DAT_00583c28 + param_1 * 0x384);
        local_2c = *(&DAT_00583c28 + param_1 * 0x384) >> 0x10;
        FUN_00496ee6(&DAT_00596a58 + local_2c * 0x3da,local_14 * local_30 + param_2,param_3,0x22,0x1d);
        FUN_0049c2e0(local_50,&DAT_004c189f);
        FUN_00497567(local_14 * local_30 + local_18,local_1c,local_50,0x1e,0xcaccce,-0x1,0xcaccce,
                     LPCSTR_005b9218,0x11);
    }
    return;
}



void
FUN_0043667f(param_1: u32,param_2: i32,param_3: i32,param_4: i32,param_5: u32,param_6: i32,param_7: i32,param_8: i32)

{
let cVar1: u8;
let mut pcVar2: String;
let mut pcVar3: String;
let mut pcVar4: String;
let mut iVar5: i32;
let mut uVar6: u32;
let mut iVar7: i32;
let mut uVar8: u32;
let puVar9: *mut u32;
let bVar10: u8;
let mut local_18c: i32;
let mut local_188: i32;
let local_184: u8 [0x14];
let mut local_170: i32;
let mut local_16c: i32;
let mut local_168: u32;
let mut local_164: u32;
let mut local_160: i32;
let mut local_15c: i32;
let mut local_158: i32;
let local_154: u8 [0x80];
let local_d4: u8;
let local_d3: u8 [0x7f];
i32 local_54 [0xd];
let mut local_20: i32;
let mut local_1c: i32;
let mut local_18: u32;
let mut local_14: u32;

local_18 = 0x0;
FUN_004a0430(local_54,0x0,0x34);
local_14 = param_1;
if (param_1 < 0xc) {
if (param_1 < 0x8) {
if ((0x4 < param_1) && ((param_1 < 0x6 || (param_1 == 0x6))))^ // goto LAB_00436ab0;
}
else {
if ((param_1 < 0x9) || ((param_1 < 0xa || (param_1 == 0xb)))) {
// LAB_00436ab0:
if (param_7 == 0x2) {
bVar10 = 0x10;
uVar8 = 0xe3e5e7;
iVar7 = -0x1;
uVar6 = 0xe3e5e7;
iVar5 = 0x78;
puVar9 = DAT_004d6a6c;
pcVar2 = FUN_00499050(DAT_0059679c,0x73cd);
FUN_00497567(param_2 + 0x5a,param_3 + -0xb,pcVar2,iVar5,uVar6,iVar7,uVar8,puVar9,bVar10);
}
local_d4 = '\0';
local_16c = 0x0;
for (local_20 = 0x0; local_20 < 0x3; local_20 = local_20 + 0x1) {
if (*(&DAT_00583c2c + local_20 * 0x4 + param_1 * 0x384) != -0x1) {
local_16c = local_16c + 0x1;
}
}
local_160 = local_16c;
if (local_16c == 0x0) {
return;
}
local_164 = (&DAT_00583c28 + param_1 * 0x384);
local_168 = *(&DAT_00583c28 + param_1 * 0x384) >> 0x10;
if (local_164 == 0x1) {
pcVar2 = FUN_00499050(DAT_0059679c,0x736e);
FUN_0049c2e0(&local_d4,pcVar2);
}
else {
pcVar2 = FUN_00499050(DAT_0059679c,0x736e);
FUN_0049c2e0(&local_d4,pcVar2);
}
FUN_00497567(param_2,param_3,&local_d4,0x78,&DAT_00535557,-0x1,&DAT_00535557,DAT_004d6a6c,0x10);
FUN_00499050(DAT_0059679c,0x7134);
FUN_0049c2e0(&local_d4,&DAT_004c18d2);
local_20 = 0x0;
while (local_16c != 0x0) {
if (*(&DAT_00583c2c + local_20 * 0x4 + param_1 * 0x384) != -0x1) {
local_164 = (&DAT_00583c2c + param_1 * 0x384 + local_20 * 0x4);
local_168 = *(&DAT_00583c2c + param_1 * 0x384 + local_20 * 0x4) >> 0x10;
local_16c = local_16c + -0x1;
if ((local_16c == 0x0) && (0x1 < local_160)) {
FUN_00499050(DAT_0059679c,0x7135);
FUN_0049c2e0(&local_d4,s__s__s__d__s_004c18d5);
}
else {
FUN_0049c2e0(&local_d4,s__s__d__s_004c18e1);
}
if (0x1 < local_16c) {
pcVar3 = &DAT_004c18ea;
iVar5 = -0x1;
pcVar2 = &local_d4;
loop {
pcVar4 = pcVar2;
if (iVar5 == 0x0) break;
iVar5 = iVar5 + -0x1;
pcVar4 = pcVar2 + 0x1;
cVar1 = *pcVar2;
pcVar2 = pcVar4;
} while (cVar1 != '\0');
pcVar4 = pcVar4 + -0x1;
loop {
cVar1 = *pcVar3;
*pcVar4 = cVar1;
if (cVar1 == '\0') break;
cVar1 = pcVar3[0x1];
pcVar3 = pcVar3 + 0x2;
pcVar4[0x1] = cVar1;
pcVar4 = pcVar4 + 0x2;
} while (cVar1 != '\0');
}
}
local_20 = local_20 + 0x1;
}
pcVar3 = &DAT_004c18ec;
iVar5 = -0x1;
pcVar2 = &local_d4;
loop {
pcVar4 = pcVar2;
if (iVar5 == 0x0) break;
iVar5 = iVar5 + -0x1;
pcVar4 = pcVar2 + 0x1;
cVar1 = *pcVar2;
pcVar2 = pcVar4;
} while (cVar1 != '\0');
pcVar4 = pcVar4 + -0x1;
loop {
cVar1 = *pcVar3;
*pcVar4 = cVar1;
if (cVar1 == '\0') break;
cVar1 = pcVar3[0x1];
pcVar3 = pcVar3 + 0x2;
pcVar4[0x1] = cVar1;
pcVar4 = pcVar4 + 0x2;
} while (cVar1 != '\0');
if (param_8 == 0x0) {
local_188 = FUN_004a3840(&local_d4,local_184,0xfa,0x2,&local_170,DAT_004d6a6c,0x0);
}
else {
local_188 = FUN_004a3840(&local_d4,local_184,0x15e,0x2,&local_170,DAT_004d6a6c,0x0);
}
for (local_18c = 0x0; local_18c < local_188; local_18c = local_18c + 0x1) {
FUN_00497567(param_2,local_18c * 0xb + param_3 + 0xb,*(local_184 + local_18c * 0x6),
*(local_184 + local_18c * 0x6 + 0x2) >> 0x10,&DAT_00535557,-0x1,&DAT_00535557,DAT_004d6a6c
,0x0);
}
return;
}
}
}
else {
if (param_1 < 0xd)^ // goto LAB_00436ab0;
if (param_1 < 0x14) {
if (0x11 < param_1) {
// LAB_004366ad:
if (param_7 == 0x2) {
bVar10 = 0x10;
uVar8 = 0xe3e5e7;
iVar7 = -0x1;
uVar6 = 0xe3e5e7;
iVar5 = 0x78;
puVar9 = DAT_004d6a6c;
pcVar2 = FUN_00499050(DAT_0059679c,0x73cd);
FUN_00497567(param_2 + 0x5a,param_3 + -0xb,pcVar2,iVar5,uVar6,iVar7,uVar8,puVar9,bVar10);
}
FUN_00466271(param_4,param_5,param_6,param_1,local_54);
if (param_8 != 0x0) {
local_1c = FUN_0046645d(param_4,param_5,param_6);
}
local_15c = 0x0;
for (local_20 = 0x0; local_20 < 0xd; local_20 = local_20 + 0x1) {
if (local_54[local_20] != 0x0) {
local_15c = local_15c + 0x1;
}
}
local_158 = local_15c;
if (local_15c == 0x0) {
return;
}
if (param_8 == 0x0) {
pcVar2 = FUN_00499050(DAT_0059679c,0x7136);
FUN_0049c2e0(&local_d4,pcVar2);
}
else {
pcVar2 = FUN_00499050(DAT_0059679c,0x7136);
FUN_0049c2e0(local_154,pcVar2);
pcVar2 = FUN_00499050(DAT_0059679c,0x73ce);
FUN_0049c2e0(&local_d4,pcVar2);
}
local_20 = 0x0;
while (local_15c != 0x0) {
if (local_54[local_20] != 0x0) {
local_15c = local_15c + -0x1;
if ((local_15c == 0x0) && (0x1 < local_158)) {
if (param_8 != 0x0) {
FUN_00499050(DAT_0059679c,0x7135);
FUN_0049c2e0(local_154,s__s__s__d__s_004c18a2);
}
FUN_00499050(DAT_0059679c,0x7135);
FUN_0049c2e0(&local_d4,s__s__s__d__s_004c18ae);
}
else {
if (param_8 != 0x0) {
FUN_0049c2e0(local_154,s__s__d__s_004c18ba);
}
FUN_0049c2e0(&local_d4,s__s__d__s_004c18c3);
}
if (0x1 < local_15c) {
pcVar3 = &DAT_004c18cc;
iVar5 = -0x1;
pcVar2 = &local_d4;
loop {
pcVar4 = pcVar2;
if (iVar5 == 0x0) break;
iVar5 = iVar5 + -0x1;
pcVar4 = pcVar2 + 0x1;
cVar1 = *pcVar2;
pcVar2 = pcVar4;
} while (cVar1 != '\0');
pcVar4 = pcVar4 + -0x1;
loop {
cVar1 = *pcVar3;
*pcVar4 = cVar1;
if (cVar1 == '\0') break;
cVar1 = pcVar3[0x1];
pcVar3 = pcVar3 + 0x2;
pcVar4[0x1] = cVar1;
pcVar4 = pcVar4 + 0x2;
} while (cVar1 != '\0');
if (param_8 != 0x0) {
pcVar3 = &DAT_004c18ce;
iVar5 = -0x1;
pcVar2 = local_154;
loop {
pcVar4 = pcVar2;
if (iVar5 == 0x0) break;
iVar5 = iVar5 + -0x1;
pcVar4 = pcVar2 + 0x1;
cVar1 = *pcVar2;
pcVar2 = pcVar4;
} while (cVar1 != '\0');
pcVar4 = pcVar4 + -0x1;
loop {
cVar1 = *pcVar3;
*pcVar4 = cVar1;
if (cVar1 == '\0') break;
cVar1 = pcVar3[0x1];
pcVar3 = pcVar3 + 0x2;
pcVar4[0x1] = cVar1;
pcVar4 = pcVar4 + 0x2;
} while (cVar1 != '\0');
}
}
}
local_20 = local_20 + 0x1;
}
pcVar3 = &DAT_004c18d0;
iVar5 = -0x1;
pcVar2 = &local_d4;
loop {
pcVar4 = pcVar2;
if (iVar5 == 0x0) break;
iVar5 = iVar5 + -0x1;
pcVar4 = pcVar2 + 0x1;
cVar1 = *pcVar2;
pcVar2 = pcVar4;
} while (cVar1 != '\0');
pcVar4 = pcVar4 + -0x1;
loop {
cVar1 = *pcVar3;
*pcVar4 = cVar1;
if (cVar1 == '\0') break;
cVar1 = pcVar3[0x1];
pcVar3 = pcVar3 + 0x2;
pcVar4[0x1] = cVar1;
pcVar4 = pcVar4 + 0x2;
} while (cVar1 != '\0');
if (param_8 == 0x0) {
FUN_00497567(param_2,param_3,&local_d4,0xfa,&DAT_00535557,-0x1,&DAT_00535557,DAT_004d6a6c,0x10);
return;
}
pcVar2 = &local_d4;
iVar5 = -0x1;
pcVar3 = local_154;
loop {
pcVar4 = pcVar3;
if (iVar5 == 0x0) break;
iVar5 = iVar5 + -0x1;
pcVar4 = pcVar3 + 0x1;
cVar1 = *pcVar3;
pcVar3 = pcVar4;
} while (cVar1 != '\0');
pcVar4 = pcVar4 + -0x1;
loop {
cVar1 = *pcVar2;
*pcVar4 = cVar1;
if (cVar1 == '\0') break;
cVar1 = pcVar2[0x1];
pcVar2 = pcVar2 + 0x2;
pcVar4[0x1] = cVar1;
pcVar4 = pcVar4 + 0x2;
} while (cVar1 != '\0');
FUN_00497567(param_2,param_3,local_154,0xfa,&DAT_00535557,-0x1,&DAT_00535557,DAT_004d6a6c,0x10);
return;
}
}
else {
if (param_1 < 0x15)^ // goto LAB_00436ab0;
if ((0x17 < param_1) && ((param_1 < 0x19 || (param_1 == 0x19))))^ // goto LAB_004366ad;
}
}
if (param_7 == 0x2) {
bVar10 = 0x10;
uVar8 = 0xe3e5e7;
iVar7 = -0x1;
uVar6 = 0xe3e5e7;
iVar5 = 0x78;
puVar9 = DAT_004d6a6c;
pcVar2 = FUN_00499050(DAT_0059679c,0x73cd);
FUN_00497567(param_2 + 0x5a,param_3 + -0xb,pcVar2,iVar5,uVar6,iVar7,uVar8,puVar9,bVar10);
}
return;
}



fn FUN_00436fe6(param_1: i32) -> u32

{
    let mut iVar1: i32;
    let mut local_110: *mut u32 [0x11];
    let ppuStack203: *mut *mut u8;;
    let mut local_1f: String;;
    let mut local_18: u32;

    iVar1 = FUN_00481a44(param_1);
    if ((iVar1 == 0x0) && ((param_1 + 0xe) != 0x2)) {
        DAT_00599e20 = param_1;
        FUN_004990e0(local_110,0x0,s_efs_res_004c18f8,s_EmptyCity_004c18ee);
        local_18 = FUN_0049bb50(local_110,FUN_0043711a);
        ppuStack203 = &PTR_FUN_004c3d34;
        if (local_1f != 0x0) {
            FUN_00499b30(local_110,local_1f);
        }
        FUN_0049a1c0(local_110,0x1);
    }
    else {
        local_18 = 0x0;
    }
    return local_18;
}



fn FUN_0043711a(param_1: *mut *mut u32,param_2: u32,param_3: u32) -> u32

{
    ushort uVar1;
    let mut puVar2: *mut u8;
    let mut pcVar3: String;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let mut iVar6: i32;
    let local_bc: u8 [0x80];
    let mut local_3c: u32;
    let mut local_38: u32;
    let mut local_34: u32;
    let local_30: *mut u32;
    let local_2c: *mut u32;
    let local_28: *mut u32;
    let local_24: *mut u32;
    let mut local_20: u32;
    let mut local_1c: u32;
    let mut local_14: u32;

    local_14 = param_2;
    if (0x404 < param_2) {
        if (param_2 < 0x406) {
            FUN_004953d7();
            FUN_004968e7(*(param_1 + 0x1d) + 0xa,*(param_1 + 0x21) + 0xa,0x30,0x28,0xe);
            FUN_0049e640(*(param_1 + 0x1d) + 0xa,*(param_1 + 0x21) + 0xa,0x30,0x28,0xce,0xca,0xcc,0x1)
            ;
            local_24 = FUN_0049c2c9(0x780);
            local_2c =
                (*(&DAT_004daab0 + (*(DAT_00599e20 + 0x6) >> 0x10) * 0x3890) * 0x940 + 0x565c90 +
                    (*(DAT_00599e20 + 0xc) >> 0x10) * 0x4);
            local_30 = *local_2c;
            local_28 = local_2c;
            FUN_00401010(local_24,local_30,0x30,0x28,0x1c);
            FUN_00496ee6(local_24,*(param_1 + 0x1d) + 0xa,*(param_1 + 0x21) + 0xa,0x30,0x28);
            iVar6 = 0x0;
            iVar5 = 0x0;
            iVar4 = 0xe0e0e;
            puVar2 = FUN_0048f614(*(DAT_00599e20 + 0x26) >> 0x10,0x64,0x21,0x42,0x64);
            FUN_0048f678(*(param_1 + 0x1d) + 0xa,*(param_1 + 0x21) + 0x30,0x30,0x2,
                         *(DAT_00599e20 + 0x26) >> 0x10,0x64,puVar2,iVar4,iVar5,iVar6);
            FUN_00497567(*(param_1 + 0x1d) + 0x41,*(param_1 + 0x21) + 0xa,
                         (&DAT_005831e8 + (*(DAT_00599e20 + 0xc) >> 0x10) * 0x50),0x64,0xcaccce,-0x1,0x272727,
                         LPCSTR_005b9218,0x10);
            local_38 = DAT_00599e20 + 0x8;
            local_3c = local_38 & 0xffff0000 | (DAT_00599e20 + 0x20);
            local_34 = local_38;
            pcVar3 = FUN_00499050(DAT_0059679c,0x73cf);
            FUN_0049c2e0(local_bc,pcVar3);
            FUN_00497567(*(param_1 + 0x1d) + 0x41,*(param_1 + 0x21) + 0x2b,local_bc,0x64,0xcaccce
                         ,-0x1,0x272727,DAT_004d6a6c,0x10);
            FUN_0043667f(*(DAT_00599e20 + 0xc) >> 0x10,*(param_1 + 0x1d) + 0x41,
                         *(param_1 + 0x21) + 0x16,*(DAT_00599e20 + 0x6) >> 0x10,
                         *(DAT_00599e20 + 0x8) >> 0x10,*(DAT_00599e20 + 0xa) >> 0x10,0x0,0x1);
            if ((DAT_00599e20 + 0x14) != -0x1) {
                iVar4 = FUN_00430d15(*(&DAT_00582938 +
                    (*(DAT_00599e20 + 0x14) >> 0x10) * 0x4 +
                    (*(DAT_00599e20 + 0x12) >> 0x10) * 0x18));
                if (iVar4 == 0x0) {
                    FUN_00499050(DAT_0059679c,0x7132);
                }
                else {
                    FUN_00499050(DAT_0059679c,0x7133);
                }
                pcVar3 = FUN_00499050(DAT_0059679c,0x73c0);
                FUN_0049c2e0(local_bc,pcVar3);
                FUN_00497567(*(param_1 + 0x1d) + 0x64,*(param_1 + 0x21) + 0x41,local_bc,0x12c,
                             0xcaccce,-0x1,0x272727,LPCSTR_005b9218,0x10);
            }
            FUN_0049af50(local_24);
            FUN_0049536f();
            return 0x1;
        }
        if (param_2 != 0x407) {
            return 0x0;
        }
        if (param_3 < 0x52b) {
            if (param_3 == 0x64) {
                FUN_0049c140(param_1,0x1);
                return 0x0;
            }
        }
        else {
            if ((param_3 < 0x52c) || (param_3 == 0x52c)) {
                FUN_0049c140(param_1,param_3);
                return 0x0;
            }
        }
        return 0x0;
    }
    if (param_2 != 0x401) {
        return 0x0;
    }
    uVar1 = (DAT_00599e20 + 0xe);
    local_20 = DAT_00599e20 & 0xffff0000 | uVar1;
    local_1c = local_20;
    if (uVar1 < 0x4) {
        if (uVar1 == 0x1) {
            pcVar3 = FUN_00499050(DAT_005967a0,0x577);
            FUN_0049bf80(param_1,0x52c,0x40f,0x0,pcVar3);
            FUN_0049bf80(param_1,0x52c,0x414,0x1,0x0);
            return 0x0;
        }
    }
    else {
        if (uVar1 < 0x5) {
            if ((DAT_00599e20 + 0x10) != 0x5) {
                return 0x0;
            }
            pcVar3 = FUN_00499050(DAT_005967a0,0x575);
            FUN_0049bf80(param_1,0x52c,0x40f,0x0,pcVar3);
            FUN_0049bf80(param_1,0x52c,0x414,0x1,0x0);
            return 0x0;
        }
        if (uVar1 == 0x17) {
            pcVar3 = FUN_00499050(DAT_005967a0,0x576);
            FUN_0049bf80(param_1,0x52c,0x40f,0x0,pcVar3);
            FUN_0049bf80(param_1,0x52c,0x414,0x1,0x0);
            return 0x0;
        }
    }
    FUN_0049bf80(param_1,0x52c,0x414,0x0,0x0);
    FUN_0049bf80(param_1,0x52c,0x410,0x0,0x0);
    return 0x0;
}



fn FUN_004376d8(param_1: u32) -> i32

{
let mut iVar1: i32;
let mut local_18: i32;
let mut local_14: i32;

local_18 = 0x0;
for (local_14 = 0x0; local_14 < 0x28; local_14 = local_14 + 0x1) {
iVar1 = FUN_00437726(local_14,param_1);
local_18 = local_18 + iVar1;
}
return local_18;
}



fn FUN_00437726(param_1: i32,param_2: u32) -> i32

{
ushort uVar1;
let mut local_24: i32;
let mut local_18: i32;
let local_14: *mut u32;

local_18 = 0x0;
local_14 = (&DAT_005b89f8 + param_1 * 0x4);
loop {
if ((local_14 == 0x0) || (*(local_14 + 0x6) >> 0x10 != param_1)) {
return local_18;
}
if (*(local_14 + 0xe) >> 0x10 != DAT_004c9754)^ // goto LAB_0043774a;
uVar1 = (local_14 + 0xe);
if (uVar1 < 0x9) {
if (uVar1 < 0x6) {
if (uVar1 == 0x5)^ // goto LAB_0043778f;
}
else {
if ((uVar1 < 0x7) || (uVar1 == 0x8)) {
// LAB_0043778f:
local_24 = 0x0;
while ((local_24 < 0x4 &&
(*(&DAT_00583c2c + (local_14[0x3] >> 0x10) * 0x384 + local_24 * 0x4) != -0x1))) {
if (*(&DAT_00583c2c + local_24 * 0x4 + (local_14[0x3] >> 0x10) * 0x384) >> 0x10 == param_2) {
local_18 = local_18 +
(&DAT_00583c2c + local_24 * 0x4 + (local_14[0x3] >> 0x10) * 0x384);
}
local_24 = local_24 + 0x1;
}
}
}
}
else {
if (uVar1 < 0xa)^ // goto LAB_0043778f;
if (uVar1 < 0xc) {
if (uVar1 == 0xb)^ // goto LAB_0043778f;
}
else {
if ((uVar1 < 0xd) || (uVar1 == 0x14))^ // goto LAB_0043778f;
}
}
// LAB_0043774a:
local_14 = *local_14;
} while( true );
}



fn FUN_004378b4(param_1: u32) -> i32

{
let mut iVar1: i32;
let mut local_18: i32;
let mut local_14: i32;

local_18 = 0x0;
for (local_14 = 0x0; local_14 < 0x28; local_14 = local_14 + 0x1) {
iVar1 = FUN_00437902(local_14,param_1);
local_18 = local_18 + iVar1;
}
return local_18;
}



fn FUN_00437902(param_1: i32,param_2: u32) -> i32

{
ushort uVar1;
let mut bVar2: bool;
let mut iVar3: i32;
undefined3 extraout_var;
let mut local_30: i32;
let mut local_24: i32;
let mut local_18: i32;
let local_14: *mut u32;

local_18 = 0x0;
local_14 = (&DAT_005b89f8 + param_1 * 0x4);
loop {
if ((local_14 == 0x0) || (*(local_14 + 0x6) >> 0x10 != param_1)) {
return local_18;
}
if (*(local_14 + 0xe) >> 0x10 != DAT_004c9754)^ // goto LAB_00437926;
uVar1 = (local_14 + 0xe);
if (uVar1 < 0x9) {
if (uVar1 < 0x6) {
if (uVar1 == 0x5)^ // goto LAB_0043796b;
}
else {
if ((uVar1 < 0x7) || (uVar1 == 0x8)) {
// LAB_0043796b:
if (*(&DAT_00583c28 + (local_14[0x3] >> 0x10) * 0x384) >> 0x10 == param_2) {
local_24 = 0x0;
uVar1 = (&DAT_00583c28 + (local_14[0x3] >> 0x10) * 0x384);
iVar3 = FUN_0046645d(*(local_14 + 0x6) >> 0x10,local_14[0x2] >> 0x10,
*(local_14 + 0xa) >> 0x10);
local_30 = (uVar1 * iVar3) / 0x64;
if (local_30 == 0x0) {
if (iVar3 == 0x0)^ // goto LAB_00437926;
local_30 = 0x1;
}
if (((DAT_004d7aca >> 0x18 == DAT_004c9754) &&
(bVar2 = FUN_0046292a((&DAT_00568210 + DAT_004c9754 * 0x1e22)), CONCAT31(extraout_var,bVar2) != 0x0
)) && (iVar3 = FUN_00489176(*(local_14 + 0x6) >> 0x10,local_14[0x2] >> 0x10,
*(local_14 + 0xa) >> 0x10,0x8), iVar3 != 0x0)) {
local_24 = (local_30 * DAT_004d7a68) / 0x64;
}
local_18 = local_18 +
local_30 +
local_24 + ((*(&DAT_00569b9c + DAT_004c9754 * 0x1e22) >> 0x18) * local_30) / 0x64;
}
}
}
}
else {
if (uVar1 < 0xa)^ // goto LAB_0043796b;
if (uVar1 < 0xc) {
if (uVar1 == 0xb)^ // goto LAB_0043796b;
}
else {
if ((uVar1 < 0xd) || (uVar1 == 0x14))^ // goto LAB_0043796b;
}
}
// LAB_00437926:
local_14 = *local_14;
} while( true );
}



fn FUN_00437b48(param_1: i32) -> i32

{
let mut iVar1: i32;
let mut local_18: i32;
let mut local_14: i32;

local_18 = 0x0;
for (local_14 = 0x0; local_14 < 0x28; local_14 = local_14 + 0x1) {
iVar1 = FUN_00437b96(local_14,param_1);
local_18 = local_18 + iVar1;
}
return local_18;
}



fn FUN_00437b96(param_1: i32,param_2: i32) -> i32

{
ushort uVar1;
let mut bVar2: bool;
let mut iVar3: i32;
let mut iVar4: i32;
undefined3 extraout_var;
undefined3 extraout_var_00;
let mut local_5c: i32;
i32 local_50 [0xe];
let mut local_18: i32;
let local_14: *mut u32;

local_18 = 0x0;
local_14 = (&DAT_005b89f8 + param_1 * 0x4);
loop {
if ((local_14 == 0x0) || (*(local_14 + 0x6) >> 0x10 != param_1)) {
return local_18;
}
if (*(local_14 + 0xe) >> 0x10 == DAT_004c9754) {
uVar1 = (local_14 + 0xe);
if (uVar1 < 0x13) {
if (uVar1 == 0x12)^ // goto LAB_00437bff;
}
else {
if ((uVar1 < 0x14) || ((0x17 < uVar1 && ((uVar1 < 0x19 || (uVar1 == 0x19)))))) {
// LAB_00437bff:
FUN_00466271(*(local_14 + 0x6) >> 0x10,local_14[0x2] >> 0x10,
*(local_14 + 0xa) >> 0x10,local_14[0x3] >> 0x10,local_50);
if (local_50[param_2] != 0x0) {
iVar4 = local_50[param_2];
local_5c = 0x0;
iVar3 = FUN_0046645d(*(local_14 + 0x6) >> 0x10,local_14[0x2] >> 0x10,
*(local_14 + 0xa) >> 0x10);
iVar4 = (iVar4 * iVar3) / 0x64;
if (iVar4 != 0x0) {
if ((((param_2 == 0x0) && (DAT_004d7860 >> 0x18 == DAT_004c9754)) &&
(bVar2 = FUN_0046292a((&DAT_00568210 + DAT_004c9754 * 0x1e22)),
CONCAT31(extraout_var,bVar2) != 0x0)) &&
(iVar3 = FUN_00488757(*(local_14 + 0x6) >> 0x10,0x2), iVar3 != 0x0)) {
local_5c = (iVar4 * DAT_004d77fe) / 0x64;
}
if (((DAT_004d7aca >> 0x18 == DAT_004c9754) &&
(bVar2 = FUN_0046292a((&DAT_00568210 + DAT_004c9754 * 0x1e22)),
CONCAT31(extraout_var_00,bVar2) != 0x0)) &&
(iVar3 = FUN_00489176(*(local_14 + 0x6) >> 0x10,local_14[0x2] >> 0x10,
*(local_14 + 0xa) >> 0x10,0x8), iVar3 != 0x0)) {
local_5c = local_5c + (iVar4 * DAT_004d7a68) / 0x64;
}
local_18 = local_18 +
iVar4 + local_5c +
(((*(&DAT_00569b96 + DAT_004c9754 * 0x1e22) >> 0x18) +
(*(&DAT_00569b69 + DAT_004c9754 * 0x1e22) >> 0x18) +
(*(&DAT_00569b9c + DAT_004c9754 * 0x1e22) >> 0x18)) * iVar4) / 0x64;
}
}
}
}
}
local_14 = *local_14;
} while( true );
}



fn FUN_00437e3e() -> i32

{
let mut iVar1: i32;
let mut local_1c: i32;
let mut local_14: i32;

local_14 = 0x0;
if (DAT_004d5584 != 0x0) {
for (local_1c = 0x0; local_1c < 0x28; local_1c = local_1c + 0x1) {
iVar1 = FUN_00437e91(local_1c);
local_14 = local_14 + iVar1;
}
}
return local_14;
}



fn FUN_00437e91(param_1: i32) -> i32

{
let local_20: *mut u32;
let local_1c: *mut u32;
let mut local_14: i32;

local_14 = 0x0;
if (DAT_004d5584 != 0x0) {
for (local_20 = (&DAT_005b89f8 + param_1 * 0x4);
(local_20 != 0x0 && (*(local_20 + 0x6) >> 0x10 == param_1));
local_20 = *local_20) {
if ((*(local_20 + 0xe) >> 0x10 == DAT_004c9754) && ((*(local_20 + 0x2d) & 0x1) == 0x0)) {
local_14 = local_14 + (*(local_20 + 0x26) >> 0x10) / 0xa;
}
}
for (local_1c = (&DAT_005b8b44 + param_1 * 0x4);
(local_1c != 0x0 && ((local_1c + 0x8) == param_1)); local_1c = *local_1c)
{
if (((*(local_1c + 0x23) >> 0x18 == DAT_004c9754) &&
((((*(local_1c + 0x3a) & 0x80) == 0x0 && ((*(local_1c + 0x3a) & 0x1) != 0x0)) &&
((*(local_1c + 0x3a) & 0x80000000) == 0x0)))) &&
(*(*(&DAT_00582938 +
(*(local_1c + 0x25) >> 0x18) * 0x4 + (local_1c[0x9] >> 0x18) * 0x18) + 0x115
) != 0x0)) {
local_14 = local_14 + 0x1;
}
}
}
return local_14;
}



fn FUN_00438010(param_1: i32,param_2: u32)

{
    let mut pcVar1: String;
    let local_94: u8 [0x80];
    let mut local_14: u32;

    local_14 = param_2;
    switch(param_2) {
    case 0x0:
        FUN_00437e91(DAT_00599e18);
    pcVar1 = FUN_00499050(DAT_0059679c,0x74a0);
    FUN_0049c2e0(local_94,pcVar1);
    FUN_00497567(*(param_1 + 0x1d) + 0x5,*(param_1 + 0x21) + 0xc8,local_94,0xfa,0xcaccce,-0x1,
                 0xcaccce,LPCSTR_005b9218,0x10);
    FUN_00437e3e();
    pcVar1 = FUN_00499050(DAT_0059679c,0x74a1);
    FUN_0049c2e0(local_94,pcVar1);
    FUN_00497567(*(param_1 + 0x1d) + 0x5,*(param_1 + 0x21) + 0xdc,local_94,0xfa,0xcaccce,-0x1,
                 0xcaccce,LPCSTR_005b9218,0x10);
    FUN_00437b96(DAT_00599e18,param_2);
    pcVar1 = FUN_00499050(DAT_0059679c,0x749c);
    FUN_0049c2e0(local_94,pcVar1);
    FUN_00497567(*(param_1 + 0x1d) + 0x5,*(param_1 + 0x21) + 0xf0,local_94,0xfa,0xcaccce,-0x1,
                 0xcaccce,LPCSTR_005b9218,0x10);
    FUN_00437b48(param_2);
    pcVar1 = FUN_00499050(DAT_0059679c,0x749d);
    FUN_0049c2e0(local_94,pcVar1);
    FUN_00497567(*(param_1 + 0x1d) + 0x5,*(param_1 + 0x21) + 0x104,local_94,0xfa,0xcaccce,-0x1,
                 0xcaccce,LPCSTR_005b9218,0x10);
    break;
    case 0x1:
        case 0x3:
        case 0x4:
        case 0xb:
        FUN_00437726(DAT_00599e18,param_2);
    pcVar1 = FUN_00499050(DAT_0059679c,0x749a);
    FUN_0049c2e0(local_94,pcVar1);
    FUN_00497567(*(param_1 + 0x1d) + 0x5,*(param_1 + 0x21) + 0xc8,local_94,0xfa,0xcaccce,-0x1,
                 0xcaccce,LPCSTR_005b9218,0x10);
    FUN_004376d8(param_2);
    pcVar1 = FUN_00499050(DAT_0059679c,0x749b);
    FUN_0049c2e0(local_94,pcVar1);
    FUN_00497567(*(param_1 + 0x1d) + 0x5,*(param_1 + 0x21) + 0xdc,local_94,0xfa,0xcaccce,-0x1,
                 0xcaccce,LPCSTR_005b9218,0x10);
    FUN_00437b96(DAT_00599e18,param_2);
    pcVar1 = FUN_00499050(DAT_0059679c,0x749c);
    FUN_0049c2e0(local_94,pcVar1);
    FUN_00497567(*(param_1 + 0x1d) + 0x5,*(param_1 + 0x21) + 0xf0,local_94,0xfa,0xcaccce,-0x1,
                 0xcaccce,LPCSTR_005b9218,0x10);
    FUN_00437b48(param_2);
    pcVar1 = FUN_00499050(DAT_0059679c,0x749d);
    FUN_0049c2e0(local_94,pcVar1);
    FUN_00497567(*(param_1 + 0x1d) + 0x5,*(param_1 + 0x21) + 0x104,local_94,0xfa,0xcaccce,-0x1,
                 0xcaccce,LPCSTR_005b9218,0x10);
    break;
    case 0x2:
        FUN_00437b96(DAT_00599e18,param_2);
    pcVar1 = FUN_00499050(DAT_0059679c,0x749c);
    FUN_0049c2e0(local_94,pcVar1);
    FUN_00497567(*(param_1 + 0x1d) + 0x5,*(param_1 + 0x21) + 0xc8,local_94,0xfa,0xcaccce,-0x1,
                 0xcaccce,LPCSTR_005b9218,0x10);
    FUN_00437b48(param_2);
    pcVar1 = FUN_00499050(DAT_0059679c,0x749d);
    FUN_0049c2e0(local_94,pcVar1);
    FUN_00497567(*(param_1 + 0x1d) + 0x5,*(param_1 + 0x21) + 0xdc,local_94,0xfa,0xcaccce,-0x1,
                 0xcaccce,LPCSTR_005b9218,0x10);
    break;
    case 0x5:
        case 0x6:
        case 0x7:
        case 0x8:
        case 0xa:
        FUN_00437726(DAT_00599e18,param_2);
    pcVar1 = FUN_00499050(DAT_0059679c,0x749a);
    FUN_0049c2e0(local_94,pcVar1);
    FUN_00497567(*(param_1 + 0x1d) + 0x5,*(param_1 + 0x21) + 0xc8,local_94,0xfa,0xcaccce,-0x1,
                 0xcaccce,LPCSTR_005b9218,0x10);
    FUN_004376d8(param_2);
    pcVar1 = FUN_00499050(DAT_0059679c,0x749b);
    FUN_0049c2e0(local_94,pcVar1);
    FUN_00497567(*(param_1 + 0x1d) + 0x5,*(param_1 + 0x21) + 0xdc,local_94,0xfa,0xcaccce,-0x1,
                 0xcaccce,LPCSTR_005b9218,0x10);
    FUN_00437902(DAT_00599e18,param_2);
    pcVar1 = FUN_00499050(DAT_0059679c,0x749e);
    FUN_0049c2e0(local_94,pcVar1);
    FUN_00497567(*(param_1 + 0x1d) + 0x5,*(param_1 + 0x21) + 0xf0,local_94,0xfa,0xcaccce,-0x1,
                 0xcaccce,LPCSTR_005b9218,0x10);
    FUN_004378b4(param_2);
    pcVar1 = FUN_00499050(DAT_0059679c,0x749f);
    FUN_0049c2e0(local_94,pcVar1);
    FUN_00497567(*(param_1 + 0x1d) + 0x5,*(param_1 + 0x21) + 0x104,local_94,0xfa,0xcaccce,-0x1,
                 0xcaccce,LPCSTR_005b9218,0x10);
    break;
    case 0x9:
        case 0xc:
        FUN_00437902(DAT_00599e18,param_2);
    pcVar1 = FUN_00499050(DAT_0059679c,0x749e);
    FUN_0049c2e0(local_94,pcVar1);
    FUN_00497567(*(param_1 + 0x1d) + 0x5,*(param_1 + 0x21) + 0xc8,local_94,0xfa,0xcaccce,-0x1,
                 0xcaccce,LPCSTR_005b9218,0x10);
    FUN_004378b4(param_2);
    pcVar1 = FUN_00499050(DAT_0059679c,0x749f);
    FUN_0049c2e0(local_94,pcVar1);
    FUN_00497567(*(param_1 + 0x1d) + 0x5,*(param_1 + 0x21) + 0xdc,local_94,0xfa,0xcaccce,-0x1,
                 0xcaccce,LPCSTR_005b9218,0x10);
}
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

i32 *
FUN_00438792(param_1: *mut u32,param_2: i32,param_3: u32,param_4: i32,param_5: i32,param_6: i32,param_7: i32,
param_8: u32,param_9: i32,param_10: i32,param_11: u32)

{
let piVar1: *mut i32;;
i32 **ppiVar2;

piVar1 = FUN_0049a030(param_1,param_2,param_3,param_4,param_5,param_6,param_7,param_8,0xf,0x0);
*(piVar1 + 0x49) = 0x0;
*(piVar1 + 0x4d) = 0x0;
$1: &mut String(piVar1 + 0x45) = &PTR_FUN_004c3ec4;
*(piVar1 + 0x2e) = *(piVar1 + 0x2e) | 0x10;
if (grid_opt_00599de0 != 0x0) {
*(piVar1 + 0x2d) = *(piVar1 + 0x2d) | 0x80;
}
if (DWORD_00599dcc == 0x0) {
*(piVar1 + 0x2d) = *(piVar1 + 0x2d) | 0x10;
}
if ((*(piVar1 + 0x2d) & 0x2) != 0x0) {
*(piVar1 + 0x51) = 0xe;
*(piVar1 + 0x55) = 0x15;
*(piVar1 + 0x5d) = 0x0;
*(piVar1 + 0x59) = *(piVar1 + 0x5d);
}
*(piVar1 + 0xa9) = param_11;
*(piVar1 + 0x7d) = param_10;
*(piVar1 + 0x81) = param_10 >> 0x1;
*(piVar1 + 0x75) = param_9;
if ((*(piVar1 + 0x75) & 0x1) != 0x0) {
*(piVar1 + 0x75) = *(piVar1 + 0x75) + 0x1;
}
if ((*(piVar1 + 0x7d) & 0x1) != 0x0) {
*(piVar1 + 0x7d) = *(piVar1 + 0x7d) + 0x1;
}
if (*(piVar1 + 0x75) < 0x7) {
*(piVar1 + 0x79) = param_9;
}
else {
*(piVar1 + 0x79) = param_9 + *(piVar1 + 0x81);
}
*(piVar1 + 0x71) = *(piVar1 + 0x79);
*(piVar1 + 0x65) = *(piVar1 + 0x7d);
*(piVar1 + 0x61) = *(piVar1 + 0x79);
*(piVar1 + 0x69) = 0x17;
*(piVar1 + 0x6d) = 0xf;
if ((0x2 < *(piVar1 + 0x75)) && (ppiVar2 = FUN_004a2831(0x10), ppiVar2 != 0x0)) {
FUN_004a2874(ppiVar2,piVar1,0x12c);
}
if (_DAT_0059a1a0 == 0x0) {
FUN_004a3c48(DAT_005b96c8,&DAT_00599e60,0x30);
FUN_004a3c48(DAT_005b96c8,&DAT_00599f60,0xffffffc0);
FUN_004a0430(&DAT_0059a060,0xff,0xa0);
FUN_004a0430(&DAT_0059a100,0xff,0xa0);
_DAT_0059a1a0 = 0x1;
}
(piVar1 + 0xa5) = &DAT_004d7d50 + *(piVar1 + 0xa9) * 0x3890;
*(piVar1 + 0xa1) = *(piVar1 + 0xa5);
*(piVar1 + 0xad) = *(&DAT_004daab0 + *(piVar1 + 0xa9) * 0x3890);
(piVar1 + 0xb1) = &DAT_005653d0 + *(piVar1 + 0xad) * 0x940;
*(piVar1 + 0xb5) = *(piVar1 + 0xad) * 0x940 + 0x565c90;
*(piVar1 + 0x9d) = 0x1;
ppiVar2 = FUN_004a2831(0x10);
if (ppiVar2 != 0x0) {
FUN_004a2874(ppiVar2,piVar1,0xc8);
}
*(&DAT_00599e30 + DAT_00599e58 * 0x4) = piVar1;
DAT_00599e58 = DAT_00599e58 + 0x1;
return piVar1;
}



fn FUN_00438ad0(param_1: &mut String,byte param_2) -> String

{
    let piVar1: *mut i32;;
    let mut local_18: i32;

    if ((param_2 & 0x4) == 0x0) {
        $1: &mut String(param_1 + 0x45) = &PTR_FUN_004c3ec4;
        FUN_004a2965(param_1);
        for (local_18 = 0x0; local_18 < DAT_00599e58; local_18 = local_18 + 0x1) {
            if (param_1 == *(LPCSTR **)(&DAT_00599e30 + local_18 * 0x4)) {
                DAT_00599e58 = DAT_00599e58 + -0x1;
                *(&DAT_00599e30 + local_18 * 0x4) = *(&DAT_00599e30 + DAT_00599e58 * 0x4);
                break;
            }
        }
        param_1 = FUN_0049a1c0(param_1,0x1);
        if ((param_2 & 0x2) != 0x0) {
            FUN_0049af50(param_1);
        }
    }
    else {
        piVar1 = FUN_00498dce(param_1,&DAT_004c3ee0);
        FUN_00498df5(piVar1);
    }
    return param_1;
}



fn FUN_00438b9b(param_1: i32,param_2: *mut u8,param_3: u32,param_4: i32,param_5: i32) -> u32

{
    let mut bVar1: bool;
    let mut iVar2: i32;
    let mut uVar3: u32;
    let mut uVar4: u32;
    let mut iVar5: i32;
    let puVar6: *mut u32;
    let puVar7: *mut u32;
    let mut in_stack_fffffedc: u32;
    let mut in_stack_fffffee0: u32;
    let mut in_stack_fffffee4: u32;
    let mut in_stack_fffffee8: u32;
    let mut in_stack_fffffeec: u32;
    let mut in_stack_fffffef0: u32;
    let mut in_stack_fffffef4: u32;
    let mut in_stack_fffffef8: u32;

    if (param_3 < 0x401) {
        if (0xff < param_3) {
            if (param_3 < 0x101) {
                return 0x1;
            }
            if (param_3 == 0x113) {
                if (((param_1 + 0x1c) == '\0') || ((*(param_1 + 0x2e) & 0x4) != 0x0)) {
                    bVar1 = false;
                }
                else {
                    bVar1 = true;
                }
                if (bVar1) {
                    iVar5 = *(param_2 + 0x4);
                    iVar2 = FUN_0049ab40();
                    if (iVar5 == iVar2) {
                        if ((*(param_1 + 0x2d) & 0x40) == 0x0) {
                            FUN_00496263((&DAT_004bf300 + DAT_0059a1a4 * 0xc),0xa,0x4);
                            DAT_0059a1a4 = DAT_0059a1a4 + 0x1;
                            if (0x8 < DAT_0059a1a4) {
                                DAT_0059a1a4 = 0x0;
                            }
                        }
                        else {
                            if ((((DAT_005967bc != 0x0) && ((*(DAT_005967bc + 0x3a) & 0x1) != 0x0)) &&
                                (*(param_1 + 0xa9) == (DAT_005967bc + 0x20))) && ((DAT_005967b8 & 0x1) == 0x0))
                            {
                                DAT_00599e5c._0_1_ = (byte)DAT_00599e5c ^ 0x1;
                                puVar6 = (DAT_005967bc + 0x20);
                                puVar7 = &stack0xfffffedc;
                                for (iVar5 = 0xb; iVar5 != 0x0; iVar5 = iVar5 + -0x1) {
                                    *puVar7 = *puVar6;
                                    puVar6 = puVar6 + 0x1;
                                    puVar7 = puVar7 + 0x1;
                                }
                                puVar7 = puVar6;
                                FUN_00449f24(param_1,in_stack_fffffedc,in_stack_fffffee0,in_stack_fffffee4,in_stack_fffffee8,
                                             in_stack_fffffeec,in_stack_fffffef0,CONCAT44(in_stack_fffffef8,in_stack_fffffef4));
                            }
                        }
                    }
                    return 0x1;
                }
                return 0x1;
            }
        }
    }
    else {
        if (param_3 < 0x402) {
            return 0x1;
        }
        if (0x403 < param_3) {
            if (param_3 < 0x405) {
                return 0x0;
            }
            if (param_3 == 0x405) {
                if (((param_1 + 0x1c) == '\0') || ((*(param_1 + 0x2e) & 0x4) != 0x0)) {
                    bVar1 = false;
                }
                else {
                    bVar1 = true;
                }
                if (bVar1) {
                    if (((DAT_005967bc != 0x0) && ((*(DAT_005967bc + 0x3a) & 0x1) != 0x0)) &&
                        ((*(param_1 + 0xa9) == (DAT_005967bc + 0x20) &&
                            (((DAT_005967bc + 0x41) != -0x1 &&
                                (uVar3 = FUN_0045af67(&DAT_005967b8,(DAT_005967bc + 0x22),
                                                      (DAT_005967bc + 0x24),*(DAT_005967bc + 0x41),
                                                      *(DAT_005967bc + 0x42),0x0), uVar3 != 0x0)))))) {
                        FUN_00449654(param_1,uVar3,(DAT_005967bc + 0x22),(DAT_005967bc + 0x24),
                                     *(DAT_005967bc + 0x41),*(DAT_005967bc + 0x42));
                    }
                    if ((*(param_1 + 0x2d) & 0x4) == 0x0) {
                        FUN_00439d27(param_1,param_5);
                    }
                    else {
                        FUN_00439f46(param_1,0x0);
                    }
                    return 0x1;
                }
                return 0x1;
            }
        }
    }
    uVar4 = FUN_0049a270(param_1,param_2,param_3,param_4,param_5);
    return uVar4;
}



fn FUN_004390ae(param_1: i32,param_2: i32)

{
    *(param_1 + 0xa9) = param_2;
    (param_1 + 0xa5) = &DAT_004d7d50 + param_2 * 0x3890;
    (param_1 + 0xa1) = &DAT_004d7d50 + param_2 * 0x3890;
    *(param_1 + 0xad) = *(&DAT_004daab0 + param_2 * 0x3890);
    (param_1 + 0xb1) = &DAT_005653d0 + *(param_1 + 0xad) * 0x940;
    *(param_1 + 0xb5) = *(param_1 + 0xad) * 0x940 + 0x565c90;
    return;
}



fn FUN_0043915e(param_1: i32,param_2: u32,param_3: i32) -> u32

{
    let mut local_18: u32;

    if (*(&DAT_00565454 + (param_2 & 0xf) * 0x8c + *(param_1 + 0x2d60) * 0x940) == param_3) {
        local_18 = param_2 & 0xf;
    }
    else {
        switch(*(&DAT_00565454 + (param_2 & 0xf) * 0x8c + *(param_1 + 0x2d60) * 0x940)) {
            case 0x2:
                case 0x3:
                case 0x4:
                case 0x6:
                local_18 = FUN_0043915e(param_1,param_2 >> 0x9,param_3);
            break;
            default:
                local_18 = 0xffffffff;
        }
    }
    return local_18;
}



fn FUN_0043920b(param_1: i32,param_2: i32,param_3: u32) -> u32

{
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut uVar3: u32;

    uVar1 = FUN_0043a8a2(param_2);
    uVar2 = FUN_0043a8d5(uVar1,param_3);
    uVar3 = FUN_00439289(param_1,*(*(*(param_1 + 0xa1) + uVar1 * 0x4) + uVar2 * 0x4));
    return uVar3;
}



fn FUN_00439289(param_1: i32,param_2: u32) -> u32

{
    let mut local_20: u32;

    switch(*(*(param_1 + 0xb1) + (param_2 & 0xf) * 0x8c + 0x84)) {
    case 0x0:
        local_20 = 0x0;
    break;
    case 0x1:
        local_20 = *(*(param_1 + 0xb1) + (param_2 & 0xf) * 0x8c);
    break;
    case 0x2:
        case 0x3:
        case 0x4:
        case 0x6:
        local_20 = FUN_00439289(param_1,param_2 >> 0x9);
    break;
    case 0x5:
        local_20 = FUN_00439289(param_1,param_2 >> 0xb);
    break;
    default:
        local_20 = 0x0;
}
    return local_20;
}



fn FUN_0043936e(param_1: i32,param_2: *mut u32) -> u32

{
    switch(*(&DAT_00565454 + (*param_2 & 0xf) * 0x8c + *(&DAT_004daab0 + param_1 * 0x3890) * 0x940))
    {
        case 0x2:
        case 0x3:
        case 0x4:
        case 0x6:
        *param_2 = *param_2 >> 0x9;
        break;
        case 0x5:
        *param_2 = *param_2 >> 0xb;
        break;
        default:
        return 0x0;
    }
    return 0x1;
}



fn FUN_00439407(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: i32,param_6: i32) -> u32

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut local_28: i32;
    let mut local_1c: i32;
    let mut local_18: i32;

    FUN_00439559(*(param_1 + 0xa5),param_2,param_3,param_4,param_5);
    if (param_6 != 0x0) {
        for (local_18 = 0x0; local_18 < DAT_00599e58; local_18 = local_18 + 0x1) {
            FUN_00447bc6(*(&DAT_00599e30 + local_18 * 0x4),param_2,param_3);
        }
        if (param_4 != 0x0) {
            for (local_1c = 0x0; local_1c < 0x6; local_1c = local_1c + 0x1) {
                iVar1 = param_2 + (&DAT_004bea60)[local_1c];
                iVar2 = param_3 + (&DAT_004bea7c)[local_1c];
                if ((((-0x1 < iVar2) && (iVar2 < 0x41)) && (-0x1 < iVar1)) && (iVar1 < 0x2c)) {
                    for (local_28 = 0x0; local_28 < DAT_00599e58; local_28 = local_28 + 0x1) {
                        FUN_00447bc6(*(&DAT_00599e30 + local_28 * 0x4),iVar1,iVar2);
                    }
                }
            }
        }
    }
    return *(*(*(param_1 + 0xa1) + param_2 * 0x4) + param_3 * 0x4);
}



fn FUN_00439559(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: i32) -> u32

{
    let mut uVar1: u32;
    let mut iVar2: i32;
    let mut uVar3: u32;
    let mut uVar4: u32;
    let mut local_7c: u32;
    let mut local_78: i32;
    let mut local_74: i32;
    let mut local_70: i32;
    let mut local_68: i32;
    let mut local_64: i32;
    let mut local_60: i32;
    let mut local_58: i32;
    let mut local_50: i32;
    let mut local_4c: i32;
    let mut local_48: i32;
    let mut local_3c: u32;
    let mut local_38: i32;
    let mut local_30: i32;
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_20: u32;
    let mut local_1c: u32;

    if ((param_3 < 0x0) || (0x40 < param_3)) {
        local_20 = 0xffffffff;
    }
    else {
        if ((param_2 < 0x0) || (0x2b < param_2)) {
            local_20 = 0xffffffff;
        }
        else {
            local_1c = *(*(param_2 * 0x4 + param_1) + param_3 * 0x4);
            uVar1 = local_1c & 0xf;
            if (*(&DAT_00565454 + uVar1 * 0x8c + *(param_1 + 0x2d60) * 0x940) < 0x7) {
                iVar2 = local_1c >> 0x9;
                switch(*(&DAT_00565454 + uVar1 * 0x8c + *(param_1 + 0x2d60) * 0x940)) {
                    case 0x0:
                        local_50 = 0x0;
                    local_4c = 0x1;
                    for (local_48 = 0x0; local_48 < 0x6; local_48 = local_48 + 0x1) {
                        uVar3 = FUN_00439c67(param_1,param_2,param_3,local_48);
                        if ((uVar3 != 0xffffffff) && (uVar3 = FUN_0043915e(param_1,uVar3,0x1), uVar3 != 0xffffffff)) {
                            local_50 = local_50 + local_4c;
                        }
                        local_4c = local_4c << 0x1;
                    }
                        *(*(param_2 * 0x4 + param_1) + param_3 * 0x4) =
                        uVar1 | *(&DAT_004bf200 + local_50 * 0x4) << 0x4;
                    break;
                    case 0x1:
                    break;
                    case 0x2:
                    if (param_5 != 0x0) {
                        *(*(param_2 * 0x4 + param_1) + param_3 * 0x4) = iVar2;
                        iVar2 = FUN_00439559(param_1,param_2,param_3,0x0,0x1);
                        local_1c = iVar2 << 0x9;
                    }
                    local_3c = 0x0;
                    for (local_38 = 0x0; local_38 < 0x6; local_38 = local_38 + 0x1) {
                        uVar3 = FUN_00439c67(param_1,param_2,param_3,local_38);
                        if ((uVar3 != 0xffffffff) &&
                            (*(&DAT_00565454 + (uVar3 & 0xf) * 0x8c + *(param_1 + 0x2d60) * 0x940) == 0x2)) {
                            local_3c = uVar3 >> 0x4 & 0x1f;
                            switch(local_38) {
                                case 0x0:
                                    case 0x2:
                                    case 0x4:
                                    local_3c = local_3c - 0x1;
                                if (local_3c == 0xffffffff) {
                                    local_3c = 0x2;
                                }
                                break;
                                case 0x1:
                                    case 0x3:
                                    case 0x5:
                                    local_3c = local_3c + 0x1;
                                if (local_3c == 0x3) {
                                    local_3c = 0x0;
                                }
                            }
                            break;
                        }
                    }
                        *(*(param_2 * 0x4 + param_1) + param_3 * 0x4) =
                        uVar1 | (local_1c >> 0x9) << 0x9 | local_3c << 0x4;
                    param_4 = 0x0;
                    break;
                    case 0x3:
                    if (param_5 != 0x0) {
                        *(*(param_2 * 0x4 + param_1) + param_3 * 0x4) = iVar2;
                        iVar2 = FUN_00439559(param_1,param_2,param_3,0x0,0x1);
                        local_1c = iVar2 << 0x9;
                    }
                    local_30 = 0x0;
                    local_2c = 0x1;
                    for (local_28 = 0x0; local_28 < 0x6; local_28 = local_28 + 0x1) {
                        uVar3 = FUN_00439c67(param_1,param_2,param_3,local_28);
                        if ((uVar3 != 0xffffffff) && (uVar3 = FUN_0043915e(param_1,uVar3,0x3), uVar3 != 0xffffffff)) {
                            local_30 = local_30 + local_2c;
                        }
                        local_2c = local_2c << 0x1;
                    }
                        *(*(param_2 * 0x4 + param_1) + param_3 * 0x4) =
                        uVar1 | (local_1c >> 0x9) << 0x9 | *(&DAT_004bf000 + local_30 * 0x4) << 0x4;
                    break;
                    case 0x4:
                    if (param_5 != 0x0) {
                        *(*(param_2 * 0x4 + param_1) + param_3 * 0x4) = iVar2;
                        iVar2 = FUN_00439559(param_1,param_2,param_3,0x0,0x1);
                        local_1c = iVar2 << 0x9;
                    }
                    local_64 = 0x0;
                    local_60 = 0x1;
                    for (local_58 = 0x0; local_58 < 0x6; local_58 = local_58 + 0x1) {
                        uVar3 = FUN_00439c67(param_1,param_2,param_3,local_58);
                        if ((uVar3 != 0xffffffff) &&
                            ((uVar4 = FUN_0043915e(param_1,uVar3,0x4), uVar4 != 0xffffffff ||
                                (uVar3 = FUN_0043915e(param_1,uVar3,0x5), uVar3 != 0xffffffff)))) {
                            local_64 = local_64 + local_60;
                        }
                        local_60 = local_60 << 0x1;
                    }
                    if (*(&DAT_004bf100 + local_64 * 0x4) == -0x1) {
                        *(*(param_2 * 0x4 + param_1) + param_3 * 0x4) = local_1c >> 0x9;
                        param_4 = 0x0;
                    }
                    else {
                        *(*(param_2 * 0x4 + param_1) + param_3 * 0x4) =
                            uVar1 | (local_1c >> 0x9) << 0x9 | *(&DAT_004bf100 + local_64 * 0x4) << 0x4;
                    }
                    break;
                    case 0x5:
                        local_7c = *(&DAT_00565450 + *(param_1 + 0x2d60) * 0x940 + uVar1 * 0x8c) - 0x1;
                    local_78 = 0x0;
                    local_74 = 0x0;
                    local_70 = 0x1;
                    for (local_68 = 0x0; local_68 < 0x6; local_68 = local_68 + 0x1) {
                        uVar3 = FUN_00439c67(param_1,param_2,param_3,local_68);
                        if (uVar3 != 0xffffffff) {
                            uVar4 = FUN_0043915e(param_1,uVar3,0x0);
                            if (uVar4 == 0xffffffff) {
                                uVar3 = FUN_0043915e(param_1,uVar3,0x4);
                                if (uVar3 != 0xffffffff) {
                                    local_74 = local_74 + local_70;
                                }
                            }
                            else {
                                local_78 = local_78 + local_70;
                            }
                        }
                        local_70 = local_70 << 0x1;
                    }
                    for (local_68 = 0x0; local_68 < 0x4e; local_68 = local_68 + 0x1) {
                        if ((local_78 == *(&DAT_004beb24 + local_68 * 0x10)) &&
                            (local_74 == *(&DAT_004beb28 + local_68 * 0x10))) {
                            local_7c = *(&DAT_004beb20 + local_68 * 0x10) | *(&DAT_004beb2c + local_68 * 0x10);
                            break;
                        }
                    }
                        *(*(param_2 * 0x4 + param_1) + param_3 * 0x4) =
                        uVar1 | (local_1c >> 0xb) << 0xb | local_7c << 0x4;
                    break;
                    case 0x6:
                    if (param_5 != 0x0) {
                        *(*(param_2 * 0x4 + param_1) + param_3 * 0x4) = iVar2;
                        iVar2 = FUN_00439559(param_1,param_2,param_3,0x0,0x1);
                        local_1c = iVar2 << 0x9;
                    }
                    *(*(param_2 * 0x4 + param_1) + param_3 * 0x4) = uVar1 | (local_1c >> 0x9) << 0x9;
                }
            }
            if (param_4 != 0x0) {
                FUN_00439559(param_1,param_2,param_3 + -0x2,0x0,0x1);
                FUN_00439559(param_1,param_2 + 0x1,param_3 + -0x1,0x0,0x1);
                FUN_00439559(param_1,param_2 + 0x1,param_3 + 0x1,0x0,0x1);
                FUN_00439559(param_1,param_2,param_3 + 0x2,0x0,0x1);
                FUN_00439559(param_1,param_2 + -0x1,param_3 + 0x1,0x0,0x1);
                FUN_00439559(param_1,param_2 + -0x1,param_3 + -0x1,0x0,0x1);
            }
            local_20 = *(*(param_2 * 0x4 + param_1) + param_3 * 0x4);
        }
    }
    return local_20;
}



fn FUN_00439c67(param_1: i32,param_2: i32,param_3: i32,param_4: i32) -> u32

{
    let mut iVar1: i32;
    let mut local_14: u32;

    param_2 = param_2 + (&DAT_004bea60)[param_4];
    iVar1 = param_3 + (&DAT_004bea7c)[param_4];
    if ((iVar1 < 0x0) || (0x40 < iVar1)) {
        local_14 = 0xffffffff;
    }
    else {
        if (param_2 < 0x0) {
            param_2 = param_2 + 0x2c;
        }
        else {
            if (0x2b < param_2) {
                param_2 = param_2 + -0x2c;
            }
        }
        local_14 = *(*(param_2 * 0x4 + param_1) + iVar1 * 0x4);
    }
    return local_14;
}



fn FUN_00439ce1()

{
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < DAT_00599e58; local_14 = local_14 + 0x1) {
        FUN_00439d27(*(&DAT_00599e30 + local_14 * 0x4),0x0);
    }
    return;
}



fn FUN_00439d27(param_1: i32,param_2: i32)

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut local_68: u32;
    let local_60: *mut i32; [0x6];
    let mut local_48: u32;
    let mut local_44: i32;
    let mut local_40: i32;
    let mut local_3c: u32;
    let mut local_38: i32;
    let mut local_34: i32;
    let mut local_30: i32;
    let mut local_2c: u32;
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_20: u32;
    let mut local_1c: u32;
    let local_18: *mut u32;

    local_44 = *(param_1 + 0x79) - *(param_1 + 0x75) >> 0x1;
    local_40 = *(param_1 + 0x79) - local_44;
    local_2c = *(param_1 + 0x1d);
    local_28 = *(param_1 + 0x21);
    local_48 = *(param_1 + 0x25);
    local_24 = *(param_1 + 0x29);
    if (param_2 != 0x0) {
        *(param_1 + 0x1d) = *(param_2 + 0x4);
        *(param_1 + 0x21) = *(param_2 + 0x4);
        *(param_1 + 0x25) = *(param_2 + 0x8);
        *(param_1 + 0x29) = *(param_2 + 0xc);
    }
    local_20 = *(param_1 + 0x1d);
    local_1c = *(param_1 + 0x21);
    local_18 = FUN_0049c2c9(*(param_1 + 0x25) * *(param_1 + 0x29));
    local_34 = (*(param_1 + 0x25) - local_44) / local_40 + 0x2;
    local_30 = *(param_1 + 0x4d) + 0x1 + *(param_1 + 0x29) / *(param_1 + 0x81);
    *(param_1 + 0x1d) = 0x0;
    *(param_1 + 0x21) = 0x0;
    FUN_004953d7();
    FUN_00498ba4(local_60,local_18,*(param_1 + 0x29),*(param_1 + 0x25));
    for (local_3c = *(param_1 + 0x49); local_3c < *(param_1 + 0x49) + local_34;
        local_3c = local_3c + 0x1) {
        local_68 = ((local_3c & 0x1) == 0x0);
        for (local_38 = *(param_1 + 0x4d) + local_68; local_38 <= local_30; local_38 = local_38 + 0x2) {
            iVar2 = local_38;
            iVar1 = FUN_0043a8a2(local_3c);
            FUN_00447c1d(param_1,iVar1,iVar2);
        }
    }
    FUN_00449bde(param_1);
    *(param_1 + 0x1d) = local_20;
    *(param_1 + 0x21) = local_1c;
    FUN_00498cf4(local_60);
    FUN_00496ac0(local_18,*(param_1 + 0x1d),*(param_1 + 0x21),*(param_1 + 0x25),
                 *(param_1 + 0x29));
    FUN_0049536f();
    FUN_0049af50(local_18);
    *(param_1 + 0x1d) = local_2c;
    *(param_1 + 0x21) = local_28;
    *(param_1 + 0x25) = local_48;
    *(param_1 + 0x29) = local_24;
    return;
}



fn FUN_00439f46(param_1: i32,param_2: i32)

{
    let mut pcVar1: String;
    let mut in_stack_fffffe5c: i32;
    let local_19c: u8 [0x100];
    let local_9c: u8 [0x40];
    let mut local_5c: u32;
    let mut local_58: u32;
    let mut local_54: i32;
    let mut local_50: u32;
    let local_4c: *mut i32; [0x6];
    let mut local_34: u32;
    let mut local_30: u32;
    let mut local_2c: u32;
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_20: String;;
    let mut local_1c: u32;
    let local_18: *mut u32;

    local_1c = *(param_1 + 0x1d);
    local_2c = *(param_1 + 0x21);
    local_28 = *(param_1 + 0x25);
    local_24 = *(param_1 + 0x29);
    local_20 = FUN_0049c2c9(0x1ff428);
    *(param_1 + 0x1d) = 0x0;
    *(param_1 + 0x21) = 0x0;
    *(param_1 + 0x25) = 0x67e;
    *(param_1 + 0x29) = 0x4ec;
    FUN_004953d7();
    local_18 = FUN_00498ba4(local_4c,local_20,0x4ec,0x67e);
    for (local_34 = 0x0; local_34 < 0x2c; local_34 = local_34 + 0x1) {
        local_50 = ((local_34 & 0x1) == 0x0);
        for (local_30 = local_50; local_30 < 0x41; local_30 = local_30 + 0x2) {
            FUN_00447c1d(param_1,local_34,local_30);
        }
    }
    if ((*(param_1 + 0x2d) & 0x20) == 0x0) {
        FUN_00449bde(param_1);
    }
    *(param_1 + 0x1d) = local_1c;
    *(param_1 + 0x21) = local_2c;
    *(param_1 + 0x25) = local_28;
    *(param_1 + 0x29) = local_24;
    FUN_00498cf4(local_4c);
    if (param_2 == 0x0) {
        FUN_00496d7e(*(param_1 + 0x1d),*(param_1 + 0x21),*(param_1 + 0x25),*(param_1 + 0x29),
                     local_20,0x67e,0x4ec);
    }
    else {
        local_54 = param_1;
        local_5c = *(param_1 + 0xa9);
        local_58 = local_5c;
        FUN_0049c2e0(local_9c,s_map_d_pcx_004c1900);
        pcVar1 = FUN_00499050(DAT_0059679c,0x741a);
        FUN_0049c2e0(local_19c,pcVar1);
        FUN_004a2ff0(0x0,local_19c,0x64,in_stack_fffffe5c);
        FUN_0043baeb(local_9c,DAT_005b96c8,local_20,0x67e,0x4ec);
        FUN_004a3800();
    }
    FUN_0049536f();
    FUN_0049af50(local_20);
    return;
}



fn FUN_0043a17b(param_1: i32,param_2: i32,param_3: u32,param_4: u32)

{
    byte *pbVar1;
    let puVar2: *mut u32;

    pbVar1 = (*(&DAT_004d7d50 + *(param_1 + 0xa9) * 0x3890 + param_2 * 0x4) + param_3 * 0x4 + 0x5);
    *pbVar1 = *pbVar1 & 0xf8;
    puVar2 = (param_3 * 0x4 + *(&DAT_004d7d50 + *(param_1 + 0xa9) * 0x3890 + param_2 * 0x4) + 0x4);
    *puVar2 = *puVar2 | param_4;
    FUN_0043a32d(param_1,param_2,param_3,0x0,0x0);
    return;
}



fn FUN_0043a213(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: u32)

{
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut local_14: i32;

    for (local_14 = 0x4; local_14 < 0x7; local_14 = local_14 + 0x1) {
        uVar1 = FUN_0043a8a2(param_2 + (&DAT_004bea60)[local_14]);
        uVar2 = FUN_0043a8d5(uVar1,param_3 + (&DAT_004bea7c)[local_14]);
        FUN_0043a32d(param_1,uVar1,uVar2,0x0,0x0);
    }
    FUN_0043a32d(param_1,param_4,param_5,0x0,0x0);
    return;
}



fn FUN_0043a2ac(param_1: i32,param_2: i32,param_3: i32)

{
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0x7; local_14 = local_14 + 0x1) {
        uVar1 = FUN_0043a8a2(param_2 + (&DAT_004bea60)[local_14]);
        uVar2 = FUN_0043a8d5(uVar1,param_3 + (&DAT_004bea7c)[local_14]);
        FUN_0043a32d(param_1,uVar1,uVar2,0x0,0x0);
    }
    return;
}



fn FUN_0043a32d(param_1: i32,param_2: i32,param_3: u32,param_4: i32,param_5: i32)

{
    byte *pbVar1;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let puVar4: *mut u32;
    let puVar5: *mut u32;
    let mut in_stack_ffffffb4: u32;
    let mut in_stack_ffffffb8: u32;
    let mut in_stack_ffffffbc: u32;
    let mut in_stack_ffffffc0: u32;
    let mut in_stack_ffffffc4: u32;
    let mut in_stack_ffffffc8: u32;
    ulonglong in_stack_ffffffcc;

    if ((param_2 != -0x1) && (param_3 != 0xffffffff)) {
        uVar2 = FUN_0043a8a2(param_2);
        FUN_0043a8d5(uVar2,param_3);
        if (param_5 == 0x0) {
            pbVar1 = (*(*(param_1 + 0xa1) + param_2 * 0x4) + param_3 * 0x4 + 0x4);
            *pbVar1 = *pbVar1 | 0x10;
        }
        else {
            FUN_00447bc6(param_1,param_2,param_3);
            if (param_4 != 0x0) {
                puVar4 = (param_4 + 0x20);
                puVar5 = &stack0xffffffb4;
                for (iVar3 = 0xb; iVar3 != 0x0; iVar3 = iVar3 + -0x1) {
                    *puVar5 = *puVar4;
                    puVar4 = puVar4 + 0x1;
                    puVar5 = puVar5 + 0x1;
                }
                puVar5 = puVar4;
                FUN_00449f24(param_1,in_stack_ffffffb4,in_stack_ffffffb8,in_stack_ffffffbc,in_stack_ffffffc0,in_stack_ffffffc4,
                             in_stack_ffffffc8,in_stack_ffffffcc);
            }
        }
    }
    return;
}



fn FUN_0043a3d6(param_1: i32,param_2: i32)

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
    FUN_004953d7();
    FUN_00498999(*(param_1 + 0x1d),*(param_1 + 0x21),*(param_1 + 0x25),*(param_1 + 0x29));
    for (local_20 = *(param_1 + 0x49);
        local_20 < *(param_1 + 0x49) + (iVar3 - iVar7) / (iVar2 - iVar7) + 0x2; local_20 = local_20 + 0x1) {
        iVar8 = FUN_0043a8a2(local_20);
        local_2c = ((local_20 & 0x1) == 0x0);
        for (local_1c = *(param_1 + 0x4d) + local_2c; local_1c <= iVar6 + 0x1 + iVar4 / iVar5;
            local_1c = local_1c + 0x2) {
            if (((*(*(*(param_1 + 0xa1) + iVar8 * 0x4) + local_1c * 0x4 + 0x4) & 0x10) != 0x0) &&
                (FUN_00447bc6(param_1,iVar8,local_1c), param_2 != 0x0)) {
                pbVar1 = (*(*(param_1 + 0xa1) + iVar8 * 0x4) + local_1c * 0x4 + 0x4);
                *pbVar1 = *pbVar1 & 0xef;
            }
        }
    }
    FUN_00449bde(param_1);
    FUN_00498ae4();
    FUN_0049536f();
    return;
}



fn FUN_0043a541(param_1: u32,param_2: i32,param_3: i32,param_4: i32)

{
    byte *pbVar1;

    pbVar1 = (*(&DAT_004d7d50 + param_3 * 0x4 + param_2 * 0x3890) + param_4 * 0x4 + 0x5);
    *pbVar1 = *pbVar1 & 0xf8;
    pbVar1 = (*(&DAT_004d7d50 + param_3 * 0x4 + param_2 * 0x3890) + param_4 * 0x4 + 0x4);
    *pbVar1 = *pbVar1 | 0x10;
    return;
}



fn FUN_0043a597(param_1: i32) -> u32

{
    byte *pbVar1;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_1c: u32;

    uVar2 = *(param_1 + 0xa9);
    uVar3 = uVar2;
    for (local_1c = 0x0; local_1c < 0x2c; local_1c = local_1c + 0x1) {
        local_24 = ((local_1c & 0x1) == 0x0);
        for (local_28 = local_24; local_28 < (0x41 - (local_1c & 0x1)); local_28 = local_28 + 0x2) {
            if ((*(*(&DAT_004d7d50 + local_1c * 0x4 + uVar2 * 0x3890) + local_28 * 0x4 + 0x5) & 0x7) != 0x0) {
                pbVar1 = (*(&DAT_004d7d50 + local_1c * 0x4 + uVar2 * 0x3890) + local_28 * 0x4 + 0x5);
                *pbVar1 = *pbVar1 & 0xf8;
                pbVar1 = (*(&DAT_004d7d50 + local_1c * 0x4 + uVar2 * 0x3890) + local_28 * 0x4 + 0x4);
                *pbVar1 = *pbVar1 | 0x10;
            }
        }
        uVar3 = local_1c;
    }
    return uVar3;
}



fn FUN_0043a681(param_1: i32,param_2: i32,param_3: i32)

{
    let puVar1: *mut u32;
    let mut puVar2: *mut u8;
    let mut local_20: u32;
    let mut local_1c: u32;
    let mut local_18: u32;
    let local_14: *mut u32;

    for (local_1c = 0x0; local_1c < 0x2c; local_1c = local_1c + 0x1) {
        local_20 = ((local_1c & 0x1) == 0x0);
        for (local_18 = local_20; local_18 < (0x41 - (local_1c & 0x1)); local_18 = local_18 + 0x2) {
            if ((*(&DAT_004be9b0 + param_3 * 0x4) &
                *(*(&DAT_004d7d50 + local_1c * 0x4 + param_1 * 0x3890) + local_18 * 0x4 + 0x4)) != 0x0) {
                puVar1 = (*(&DAT_004d7d50 + local_1c * 0x4 + param_1 * 0x3890) + local_18 * 0x4 + 0x4);
                *puVar1 = *puVar1 | *(&DAT_004be9b0 + param_2 * 0x4);
            }
        }
    }
    puVar2 = &DAT_00568210 + param_1 * 0x9d + param_2 * 0x1e22;
    puVar2[0x9c] = puVar2[0x9c] & 0xfe;
    puVar2[0x9c] = puVar2[0x9c] | 0x1;
    for (local_14 = *DAT_005967c8; local_14 != 0x0; local_14 = *local_14) {
        if ((*(local_14 + 0x6) >> 0x10 == param_1) &&
            ((*(&DAT_004be9b0 + param_3 * 0x4) & local_14[0xb]) != 0x0)) {
            local_14[0xb] = local_14[0xb] | *(&DAT_004be9b0 + param_2 * 0x4);
            puVar1 = (*(&DAT_004d7d50 + param_1 * 0x3890 + (local_14[0x2] >> 0x10) * 0x4) +
                (*(local_14 + 0xa) >> 0x10) * 0x4 + 0x4);
            *puVar1 = *puVar1 | *(&DAT_004be9b0 + param_2 * 0x4);
        }
    }
    return;
}



fn FUN_0043a810(param_1: i32,param_2: i32)

{
    let puVar1: *mut u32;
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: u32;

    for (local_18 = 0x0; local_18 < 0x2c; local_18 = local_18 + 0x1) {
        local_1c = ((local_18 & 0x1) == 0x0);
        for (local_14 = local_1c; local_14 < (0x41 - (local_18 & 0x1)); local_14 = local_14 + 0x2) {
            puVar1 = (*(&DAT_004d7d50 + local_18 * 0x4 + param_1 * 0x3890) + local_14 * 0x4 + 0x4);
            *puVar1 = *puVar1 | *(&DAT_004be9b0 + param_2 * 0x4);
        }
    }
    return;
}



fn FUN_0043a8a2(param_1: i32) -> i32

{
if (param_1 < 0x0) {
param_1 = param_1 + 0x2c;
}
else {
if (0x2b < param_1) {
param_1 = param_1 + -0x2c;
}
}
return param_1;
}



fn FUN_0043a8d5(param_1: u32,param_2: u32) -> u32

{
    let mut local_20: u32;
    let mut local_1c: u32;
    let mut local_14: u32;

    local_14 = ((param_1 & 0x1) == 0x0);
    if (param_2 < local_14) {
        local_1c = ((param_1 & 0x1) == 0x0);
        param_2 = local_1c;
    }
    else {
        if ((0x3f - (param_1 & 0x1)) < param_2) {
            local_20 = ((param_1 & 0x1) == 0x0);
            param_2 = local_20 + 0x3e;
        }
    }
    return param_2;
}



fn FUN_0043a96a(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: i32) -> u32

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let local_20: *mut u32;
    let mut local_18: u32;

    iVar1 = *(&DAT_004daab0 + param_1 * 0x3890);
    if ((param_5 == 0x0) && (iVar2 = FUN_00482588(param_1,0x0), iVar2 != 0x0)) {
        local_18 = 0x0;
    }
    else {
        if ((param_5 == 0x1) && (iVar2 = FUN_00482588(param_1,0x1), iVar2 != 0x0)) {
            local_18 = 0x0;
        }
        else {
            if ((param_5 == 0x11) && (iVar2 = FUN_00482588(param_1,0x11), iVar2 != 0x0)) {
                local_18 = 0x0;
            }
            else {
                if ((*(&DAT_00565454 + param_4 * 0x8c + iVar1 * 0x940) == 0x0) ||
                    (*(&DAT_00565454 + param_4 * 0x8c + iVar1 * 0x940) == 0x5)) {
                    if (*(&DAT_005831f8 + param_5 * 0x50) == 0x0) {
                        return 0x0;
                    }
                    if ((iVar1 == 0x4) && (*(&DAT_00583204 + param_5 * 0x50) == 0x0)) {
                        return 0x0;
                    }
                }
                else {
                    if (*(&DAT_005831fc + param_5 * 0x50) == 0x0) {
                        return 0x0;
                    }
                    if ((iVar1 == 0x4) && (*(&DAT_00583204 + param_5 * 0x50) == 0x0)) {
                        return 0x0;
                    }
                }
                if (*(&DAT_00583210 + param_5 * 0x50) != 0x0) {
                    for (local_20 = *DAT_005967c8; local_20 != 0x0; local_20 = *local_20
                    ) {
                        if (((*(local_20 + 0x6) >> 0x10 == param_1) &&
                            (*(&DAT_00583210 + (local_20[0x3] >> 0x10) * 0x50) != 0x0)) &&
                            (iVar1 = *(&DAT_00583210 + (local_20[0x3] >> 0x10) * 0x50),
                             iVar2 = FUN_0044a87f(param_2,param_3,local_20[0x2] >> 0x10,*(local_20 + 0xa) >> 0x10),
                             iVar2 < iVar1 * 0x2 + 0x1)) {
                            return 0x0;
                        }
                    }
                }
                local_18 = 0x1;
            }
        }
    }
    return local_18;
}



fn FUN_0043ab53(param_1: i32,param_2: i32,param_3: i32,param_4: i32) -> u32

{
    let mut uVar1: u32;

    uVar1 = FUN_0043ab95(param_1,param_2,param_3,DAT_004c9754,DAT_004c9754,param_4);
    return uVar1;
}



fn FUN_0043ab95(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: i32,param_6: i32) -> u32

{
    ushort uVar1;
    let mut iVar2: i32;
    let puVar3: *mut u32;
    let puVar4: *mut u32;
    u32 auStackY208 [0x8];
    let mut uStackY176: u32;
    u32 local_98 [0xd];
    let mut local_64: u32;
    let mut local_60: u32;
    let mut local_5c: u32;
    let local_58: u16;
    let local_56: u16;
    let local_54: u16;
    let local_52: u16;
    let mut local_50: u32;
    let local_4c: u16;
    let local_4a: u16;
    let local_48: u16;
    let local_46: u16;
    let mut local_42: u32;
    let local_3e: u16;
    let local_3c: u16;
    let local_3a: u16;
    let mut local_38: u32;
    let mut local_24: u32;
    let mut local_20: u32;
    let local_1c: *mut u32;
    let mut local_18: i32;
    let mut local_14: i32;

    local_24 = *(*(&DAT_004d7d50 + param_2 * 0x4 + param_1 * 0x3890) + param_3 * 0x4);
    local_20 = local_24 & 0xf;
    if (param_5 == -0x1) {
        param_5 = param_4;
    }
    uStackY176 = 0x43abf1;
    iVar2 = FUN_0043a96a(param_1,param_2,param_3,local_20,param_6);
    if (iVar2 != 0x0) {
        local_1c = FUN_00481784(param_1,param_2,param_3);
        local_18 = -0x1;
        if (local_1c != 0x0) {
            uVar1 = (local_1c + 0xe);
            local_64 = local_1c & 0xffff0000 | uVar1;
            if (uVar1 < 0x1b) {
                if (uVar1 < 0x10) {
                    if (uVar1 != 0xf) {
                        return 0x714b;
                    }
                }
                else {
                    if ((0x10 < uVar1) && (uVar1 != 0x1a)) {
                        return 0x714b;
                    }
                }
            }
            else {
                if ((((0x1b < uVar1) && (0x1c < uVar1)) && (0x1d < uVar1)) && ((0x1e < uVar1 && (uVar1 != 0x1f)))) {
                    return 0x714b;
                }
            }
            local_18 = local_1c[0x3] >> 0x10;
            local_60 = local_64;
        }
        local_14 = *(&DAT_004daab0 + param_1 * 0x3890);
        if (((*(&DAT_00565454 + (local_24 & 0xf) * 0x8c + local_14 * 0x940) != 0x6) &&
            (*(&DAT_00565454 + (local_24 & 0xf) * 0x8c + local_14 * 0x940) != 0x0)) &&
            (*(&DAT_00583200 + param_6 * 0x50) != 0x0)) {
            *(*(&DAT_004d7d50 + param_2 * 0x4 + param_1 * 0x3890) + param_3 * 0x4) =
                *(&DAT_00565a5c + local_14 * 0x940) << 0x4 | local_24 << 0x9 | 0xb;
            uStackY176 = 0x43ad61;
            FUN_00439559((&DAT_004d7d50 + param_1 * 0x3890),param_2,param_3,0x1,0x0);
        }
        local_5c._0_2_ = (undefined2)param_1;
        local_5c._2_2_ = (undefined2)param_2;
        local_58 = (undefined2)param_3;
        local_56 = (undefined2)param_6;
        local_54 = (undefined2)param_4;
        local_52 = (undefined2)param_5;
        local_4c = 0xffff;
        local_42 = 0xffffffff;
        local_50 = 0xffffffff;
        local_3e = 0xffff;
        local_4a = 0x0;
        local_48 = (undefined2)local_18;
        local_46 = 0x0;
        FUN_00482992(&local_5c,0x4b);
        local_3c = 0x32;
        local_3a = *(&DAT_00569ac9 + param_4 * 0x1e22);
        local_38 = 0x0;
        if (param_4 != param_5) {
            local_38 = 0x100;
        }
        puVar3 = &local_5c;
        puVar4 = local_98;
        for (iVar2 = 0xc; iVar2 != 0x0; iVar2 = iVar2 + -0x1) {
            *puVar4 = *puVar3;
            puVar3 = puVar3 + 0x1;
            puVar4 = puVar4 + 0x1;
        }
            *puVar4 = *puVar3;
        puVar3 = local_98;
        puVar4 = auStackY208;
        for (iVar2 = 0xc; iVar2 != 0x0; iVar2 = iVar2 + -0x1) {
            *puVar4 = *puVar3;
            puVar3 = puVar3 + 0x1;
            puVar4 = puVar4 + 0x1;
        }
            *puVar4 = *puVar3;
        FUN_004813ca();
        return 0x1;
    }
    return 0x7148;
}



fn FUN_0043ae39(param_1: u32) -> i32

{
let mut uVar1: u32;
let mut local_18: i32;

DAT_0059a1a8 = param_1;
uVar1 = FUN_0043ae7c();
if (uVar1 == 0x0) {
local_18 = -0x1;
}
else {
local_18 = uVar1 - 0x414;
}
return local_18;
}



fn FUN_0043ae7c() -> u32

{
    let local_1e8: *mut u32;
    let local_19c: *mut u32;
    let mut local_150: *mut u32 [0x11];
    let ppuStack267: *mut *mut u8;;
    let mut local_5f: String;;
    u32 auStack88 [0xe];
    let mut local_20: u32;
    let local_1c: *mut i32;;
    let mut local_18: i32;

    local_1c = FUN_004990e0(local_150,0x0,s_efs_res_004c1915,s_EditStrDlg_004c190a);
    for (local_18 = 0x0; local_18 < 0x9; local_18 = local_18 + 0x1) {
        local_19c = FUN_004a2831(0x5d);
        if (local_19c != 0x0) {
            local_19c = FUN_0049a030(local_19c,local_150,local_18 + 0x414,0x10e,local_18 * 0x14 + 0x6e,0xc,0x10,0x1,
                                     0xcaccce,0xe0e0e);
            $1: &mut String(local_19c + 0x45) = &PTR_FUN_004c3d94;
            (local_19c + 0x51) = &DAT_004c191f;
            (local_19c + 0x55) = &DAT_004c191d;
            *(local_19c + 0x4d) = 0x0;
            *(local_19c + 0x49) = 0x2;
        }
        auStack88[local_18] = local_19c;
        FUN_0049bf40(local_150,auStack88[local_18]);
    }
    for (; local_18 < 0xe; local_18 = local_18 + 0x1) {
        local_1e8 = FUN_004a2831(0x5d);
        if (local_1e8 != 0x0) {
            local_1e8 = FUN_0049a030(local_1e8,local_150,local_18 + 0x414,0x1ef,(local_18 + -0x9) * 0x14 + 0x6e,0xc,0x10,
                                     0x1,0xcaccce,0xe0e0e);
            $1: &mut String(local_1e8 + 0x45) = &PTR_FUN_004c3d94;
            (local_1e8 + 0x51) = &DAT_004c1923;
            (local_1e8 + 0x55) = &DAT_004c1921;
            *(local_1e8 + 0x4d) = 0x0;
            *(local_1e8 + 0x49) = 0x2;
        }
        auStack88[local_18] = local_1e8;
        FUN_0049bf40(local_150,auStack88[local_18]);
    }
    local_20 = FUN_0049bb50(local_150,FUN_0043b319);
    ppuStack267 = &PTR_FUN_004c3d34;
    if (local_5f != 0x0) {
        FUN_00499b30(local_150,local_5f);
    }
    FUN_0049a1c0(local_150,0x1);
    return local_20;
}



fn FUN_0043b319(param_1: *mut *mut u32,param_2: u32,param_3: u32) -> u32

{
    let mut pcVar1: String;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut uVar4: u32;
    let mut iVar5: i32;
    let mut uVar6: u32;
    let puVar7: *mut u32;
    let bVar8: u8;
    let mut local_1c: i32;

    if (param_2 < 0x405) {
        if (param_2 == 0x401) {
            FUN_0049ae60(param_1,0x414,0x421,DAT_0059a1a8 + 0x414);
            FUN_0049bf80(param_1,0x66,0x414,0x0,0x0);
            FUN_0049bf80(param_1,0x66,0x410,0x0,0x0);
        }
    }
    else {
        if (param_2 < 0x406) {
            FUN_004953d7();
            for (local_1c = 0x0; local_1c < 0x9; local_1c = local_1c + 0x1) {
                bVar8 = 0x14;
                uVar6 = 0xcaccce;
                iVar5 = -0x1;
                uVar4 = 0xcaccce;
                iVar3 = 0x96;
                puVar7 = LPCSTR_005b9218;
                pcVar1 = FUN_00499050(DAT_0059679c,local_1c + 0x414);
                FUN_00497567(0x109,local_1c * 0x14 + 0x6e,pcVar1,iVar3,uVar4,iVar5,uVar6,puVar7,bVar8);
            }
            for (; local_1c < 0xe; local_1c = local_1c + 0x1) {
                bVar8 = 0x14;
                uVar6 = 0xcaccce;
                iVar5 = -0x1;
                uVar4 = 0xcaccce;
                iVar3 = 0x96;
                puVar7 = LPCSTR_005b9218;
                pcVar1 = FUN_00499050(DAT_0059679c,local_1c + 0x414);
                FUN_00497567(0x1ea,(local_1c + -0x9) * 0x14 + 0x6e,pcVar1,iVar3,uVar4,iVar5,uVar6,puVar7,bVar8);
            }
            FUN_0049536f();
        }
        else {
            if (param_2 == 0x407) {
                if ((0x413 < param_3) && (param_3 < 0x422)) {
                    FUN_0049ae60(param_1,0x414,0x421,param_3);
                    return 0x1;
                }
                if (0x63 < param_3) {
                    if (param_3 < 0x65) {
                        pcVar1 = FUN_00499050(DAT_0059679c,0x7162);
                        uVar2 = FUN_0049d2e0(param_1,0x3,pcVar1);
                        if (uVar2 != 0x0) {
                            iVar3 = FUN_0049aea0(param_1,0x414,0x421);
                            FUN_0049c140(param_1,iVar3);
                        }
                    }
                    else {
                        if (param_3 == 0x65) {
                            FUN_0049c140(param_1,0x0);
                        }
                    }
                }
            }
        }
    }
    return 0x0;
}



fn FUN_0043b552(param_1: u32) -> u32

{
    let mut local_120: *mut u32 [0x11];
    let ppuStack219: *mut *mut u8;;
    let mut local_2f: String;;
    let mut local_28: u32;
    let local_24: *mut i32;;
    u32 **local_20;
    let mut local_1c: u32;

    DAT_0059a1ac = param_1;
    local_24 = FUN_004990e0(local_120,0x0,s_efs_res_004c192e,s_EditSect_004c1925);
    local_28 = FUN_0049bb50(local_120,FUN_0043b651);
    local_20 = local_120;
    local_1c = 0x0;
    ppuStack219 = &PTR_FUN_004c3d34;
    if (local_2f != 0x0) {
        FUN_00499b30(local_120,local_2f);
    }
    FUN_0049a1c0(local_120,0x1);
    return local_28;
}



fn FUN_0043b651(param_1: *mut *mut u32,param_2: u32,param_3: u32) -> u32

{
    let mut pcVar1: String;
    let mut iVar2: i32;
    let mut uVar3: u32;
    let mut iVar4: i32;
    let mut uVar5: u32;
    let puVar6: *mut u32;
    let bVar7: u8;
    let mut local_1c: i32;

    if (param_2 < 0x405) {
        if (param_2 == 0x401) {
            FUN_0049ae60(param_1,0x40f,0x413,DAT_0059a1ac + 0x40a);
        }
    }
    else {
        if (param_2 < 0x406) {
            FUN_004953d7();
            for (local_1c = 0x0; local_1c < 0x5; local_1c = local_1c + 0x1) {
                bVar7 = 0x14;
                uVar5 = 0xcaccce;
                iVar4 = -0x1;
                uVar3 = 0xcaccce;
                iVar2 = 0x96;
                puVar6 = LPCSTR_005b9218;
                pcVar1 = FUN_00499050(DAT_0059679c,local_1c + 0x40f);
                FUN_00497567(0x16d,local_1c * 0x14 + 0xb4,pcVar1,iVar2,uVar3,iVar4,uVar5,puVar6,bVar7);
            }
            FUN_0049536f();
        }
        else {
            if (param_2 == 0x407) {
                if ((0x40e < param_3) && (param_3 < 0x414)) {
                    FUN_0049ae60(param_1,0x40f,0x413,param_3);
                    return 0x1;
                }
                if (0x63 < param_3) {
                    if (param_3 < 0x65) {
                        iVar2 = FUN_0049aea0(param_1,0x40f,0x413);
                        iVar2 = (iVar2 + -0x40a);
                        if (iVar2 == DAT_0059a1ac) {
                            FUN_0049c140(param_1,0x0);
                        }
                        else {
                            FUN_0049c140(param_1,iVar2);
                        }
                    }
                    else {
                        if (param_3 == 0x65) {
                            FUN_0049c140(param_1,0x0);
                        }
                    }
                }
            }
        }
    }
    return 0x0;
}



fn FUN_0043b7fd(param_1: i32,param_2: i32,param_3: i32) -> u32

{
    byte *pbVar1;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let mut local_14: u32;

    uVar3 = *(*(&DAT_004d7d50 + param_2 * 0x4 + param_1 * 0x3890) + param_3 * 0x4);
    uVar2 = uVar3 & 0xf;
    if (uVar2 != 0xb) {
        *(*(&DAT_004d7d50 + param_2 * 0x4 + param_1 * 0x3890) + param_3 * 0x4) =
            *(&DAT_00565a5c + *(&DAT_004daab0 + param_1 * 0x3890) * 0x940) << 0x4 | uVar3 << 0x9 | 0xb;
        uVar2 = FUN_00439559((&DAT_004d7d50 + param_1 * 0x3890),param_2,param_3,0x1,0x0);
        for (local_14 = 0x0; local_14 < 0x7; local_14 = local_14 + 0x1) {
            uVar3 = FUN_0043a8a2(param_2 + (&DAT_004bea60)[local_14]);
            uVar2 = FUN_0043a8d5(uVar3,param_3 + (&DAT_004bea7c)[local_14]);
            pbVar1 = (*(&DAT_004d7d50 + uVar3 * 0x4 + param_1 * 0x3890) + uVar2 * 0x4 + 0x4);
            *pbVar1 = *pbVar1 | 0x10;
            uVar2 = local_14;
        }
    }
    return uVar2;
}



fn FUN_0043b921(param_1: i32,param_2: u32) -> u32

{
    let mut uVar1: u32;

    uVar1 = *(&DAT_00565454 + (param_2 & 0xf) * 0x8c + *(param_1 + 0x2d60) * 0x940);
    if (uVar1 != 0x0) {
        if (uVar1 < 0x2) {
            return param_2;
        }
        if (uVar1 == 0x5) {
            uVar1 = FUN_0043b921(param_1,param_2 >> 0xb);
            return uVar1;
        }
    }
    uVar1 = FUN_0043b921(param_1,param_2 >> 0x9);
    return uVar1;
}



fn FUN_0043b9ac(param_1: i32,param_2: i32,param_3: i32) -> u32

{
    let mut local_14: u32;

    if ((param_2 < *(param_1 + 0x49)) || (*(param_1 + 0x49) + *(param_1 + 0x6d) < param_2)) {
        local_14 = 0x0;
    }
    else {
        if ((param_3 < *(param_1 + 0x4d)) || (*(param_1 + 0x4d) + *(param_1 + 0x69) < param_3)) {
            local_14 = 0x0;
        }
        else {
            local_14 = 0x1;
        }
    }
    return local_14;
}



fn FUN_0043ba14(param_1: *mut u32,param_2: *mut i32,param_3: i32)

{
    let local_18: u8;
    let local_14: *mut u32;

    local_14 = (param_1 + param_3);
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



fn FUN_0043baeb(param_1: &mut String,param_2: i32,param_3: i32,param_4: i32,param_5: i32)

{
    let mut local_a0: i32;
    let local_9c: u8;
    let mut local_98: u32;
    let mut local_94: u32;
    let local_90: i16;
    let local_8e: i16;
    let local_8c: i16;
    let local_8a: i16;
    let local_58: u16;
    let local_56: i16;
    let mut local_54: u32;
    let local_18: *mut i32;;
    let local_14: *mut u32;

    local_14 = FUN_0049c2c9(0x300);
    if ((param_3 != 0x0) && (local_14 != 0x0)) {
        local_98 = 0x801050a;
        local_94 = 0x0;
        local_8c = param_4;
        local_90 = local_8c + -0x1;
        local_8a = param_5;
        local_8e = local_8a + -0x1;
        local_58 = 0x100;
        local_54 = 0x0;
        local_56 = local_8c;
        local_18 = FUN_0049c4bd(param_1,&DAT_004c1936);
        if (local_18 != 0x0) {
            FUN_004a7160(&local_98,0x80,0x1,local_18);
            for (local_a0 = 0x0; local_a0 < param_5; local_a0 = local_a0 + 0x1) {
                FUN_0043ba14((local_a0 * param_4 + param_3),local_18,param_4);
                FUN_004a36b0((local_a0 * 0x64) / param_5);
            }
            for (local_a0 = 0x0; local_a0 < 0x100; local_a0 = local_a0 + 0x1) {
                (local_a0 * 0x3 + local_14) = (local_a0 * 0x3 + param_2) << 0x2;
                (local_14 + local_a0 * 0x3 + 0x1) = (local_a0 * 0x3 + param_2 + 0x1) << 0x2;
                (local_14 + local_a0 * 0x3 + 0x2) = (local_a0 * 0x3 + param_2 + 0x2) << 0x2;
            }
            local_9c = 0xc;
            FUN_004a7160(&local_9c,0x1,0x1,local_18);
            FUN_004a7160(local_14,0x300,0x1,local_18);
            FUN_004a36b0(0x64);
            FUN_0049ca40(local_18);
        }
    }
    if (local_14 != 0x0) {
        FUN_0049af50(local_14);
    }
    return;
}



fn FUN_0043bcea() -> String

{
    return PTR_s_NORMAL_004bf91c;
}



fn FUN_0043bd09(param_1: i32,param_2: i32)

{
    let mut pcVar1: String;
    let local_38: u8 [0x20];
    let mut local_18: i32;
    let mut local_14: i32;

    if (param_2 == 0x0) {
        local_14 = 0x63;
    }
    else {
        local_14 = 0x80;
    }
    local_18 = local_14;
    FUN_00497567(0x3c,local_14,(&DAT_005b709e + param_1 * 0x4e),0x5a,0xcaccce,-0x1,0xcaccce,DAT_004d6a6c,0x11);
    pcVar1 = FUN_00499050(DAT_0059679c,0x7348);
    FUN_0049c2e0(local_38,pcVar1);
    FUN_00497567(0x3c,local_18 + 0xc,local_38,0x5a,0xcaccce,-0x1,0xcaccce,DAT_004d6a6c,0x11);
    pcVar1 = FUN_00499050(DAT_0059679c,0x7139);
    FUN_0049c2e0(local_38,pcVar1);
    FUN_00497567(0x3c,local_18 + 0x18,local_38,0x5a,0xcaccce,0xe0e0e,0xcaccce,DAT_004d6a6c,0x11);
    return;
}



fn FUN_0043be20(param_1: i32)

{
    let local_38: u8 [0x20];
    let mut local_18: *mut u8;
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0xd; local_14 = local_14 + 0x1) {
        FUN_00496ee6(&DAT_00596a58 + local_14 * 0x3da,local_14 * 0x26 + 0x80,0x1ba,0x22,0x1d);
        FUN_00462571((&DAT_00568210 + DAT_004c9754 * 0x1e22),param_1,local_14);
        FUN_0049c2e0(local_38,&DAT_004c1939);
        if (*(param_1 * 0x9d + DAT_004c9754 * 0x1e22 + local_14 * 0x4 + 0x568278) <
            *(param_1 * 0x9d + DAT_004c9754 * 0x1e22 + local_14 * 0x4 + 0x568244)) {
            local_18 = 0xe7e7e7;
        }
        else {
            local_18 = &DAT_00575757;
        }
        FUN_00497567(local_14 * 0x26 + 0x91,0x1db,local_38,0x1e,local_18,-0x1,local_18,DAT_004d6a6c,0x19);
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0043bf7a(param_1: u32,param_2: i32,param_3: u32) -> u32

{
    let mut iVar1: i32;
    let local_264: *mut u32;
    let local_218: *mut u32;
    let local_1c8: *mut i32;;
    let local_1c4: *mut i32;;
    let mut local_1bc: *mut u32 [0x11];
    let ppuStack375: *mut *mut u8;;
    let mut local_cb: String;;
    i32 aiStack196 [0x14];
    i32 aiStack116 [0xd];
    i32 **local_40;
    let mut local_3c: u32;
    let local_38: *mut i32;;
    let local_34: *mut i32;;
    let local_30: *mut i32;;
    let local_2c: *mut i32;;
    let mut local_28: i32;
    i32 **local_24;
    let mut local_20: u32;
    i32 **local_1c;
    i32 **local_18;

    FUN_0045ae28();
    DAT_0059aa00 = param_2;
    DAT_0059aa04 = param_3;
    _DAT_0059a9fc = (param_2 != -0x1);
    DAT_0059a9f4 = 0x0;
    _DAT_00596a4c = 0x0;
    _DAT_004c9750 = 0x1;
    FUN_004990e0(local_1bc,0x0,s_efs_res_004c1943,s_HexDlg_004c193c);
    local_1c4 = FUN_004a2831(0xb9);
    local_38 = local_1c4;
    local_34 = local_1c4;
    if (local_1c4 != 0x0) {
        local_1c4 = FUN_00438792(local_1c4,local_1bc,0x4b0,0x7c,0x20,0x1f8,0x190,0x4009,0x1c,0x28,param_1);
    }
    DAT_0059a1c0 = local_1c4;
    local_1c8 = FUN_004a2831(0xb9);
    local_30 = local_1c8;
    local_2c = local_1c8;
    if (local_1c8 != 0x0) {
        local_1c8 = FUN_00438792(local_1c8,local_1bc,0x4b1,0x3,0x1c,0x58,0x3f,0x4042,0x2,0x2,param_1);
    }
    DAT_0059a1c4 = local_1c8;
    for (local_28 = 0x0; local_28 < 0x14; local_28 = local_28 + 0x1) {
        local_218 = FUN_004a2831(0x5d);
        if (local_218 != 0x0) {
            local_218 = FUN_0049a030(local_218,local_1bc,local_28 + 0x2328,(local_28 % 0x3) * 0x22 + 0x9,
                                     (local_28 / 0x3) * 0x22 + 0xb5,0x22,0x22,0x4002,0xcaccce,0xe0e0e);
            $1: &mut String(local_218 + 0x45) = &PTR_FUN_004c3d94;
            *(local_218 + 0x51) = 0x0;
            *(local_218 + 0x55) = 0x0;
            *(local_218 + 0x4d) = 0x0;
            *(local_218 + 0x49) = 0x2;
        }
        aiStack196[local_28] = local_218;
        FUN_0049bf40(local_1bc,aiStack196[local_28]);
    }
    for (local_28 = 0x0; local_28 < 0xd; local_28 = local_28 + 0x1) {
        local_264 = FUN_004a2831(0x5d);
        if (local_264 != 0x0) {
            local_264 = FUN_0049a030(local_264,local_1bc,local_28 + 0x238c,local_28 * 0x26 + 0x80,0x1ba,0x22,0x1d,0x4002,
                                     0xcaccce,0xe0e0e);
            $1: &mut String(local_264 + 0x45) = &PTR_FUN_004c3d94;
            *(local_264 + 0x51) = 0x0;
            *(local_264 + 0x55) = 0x0;
            *(local_264 + 0x4d) = 0x0;
            *(local_264 + 0x49) = 0x2;
        }
        aiStack116[local_28] = local_264;
        FUN_0049bf40(local_1bc,aiStack116[local_28]);
    }
    local_40 = FUN_004a2831(0x10);
    local_24 = local_40;
    if (local_40 != 0x0) {
    FUN_004a2874(local_40,local_1bc,0xfa);
}
    local_1c = FUN_004a2831(0x10);
    local_18 = local_1c;
    if (local_1c != 0x0) {
    FUN_004a2874(local_1c,local_1bc,0x14d);
}
    FUN_0049bf40(local_1bc,DAT_0059a1c0);
    FUN_0049bf40(local_1bc,DAT_0059a1c4);
    local_20 = FUN_0049bb50(local_1bc,FUN_0043c7a2);
    FUN_0043a597(DAT_0059a1c0);
    for (local_28 = 0x0; local_28 < 0x14; local_28 = local_28 + 0x1) {
        if ((aiStack196[local_28] != 0x0) && (iVar1 = aiStack196[local_28], iVar1 != 0x0)) {
            ((*(iVar1 + 0x45) + 0x8))(iVar1,0x2);
        }
    }
    for (local_28 = 0x0; local_28 < 0xd; local_28 = local_28 + 0x1) {
        if ((aiStack116[local_28] != 0x0) && (iVar1 = aiStack116[local_28], iVar1 != 0x0)) {
            ((*(iVar1 + 0x45) + 0x8))(iVar1,0x2);
        }
    }
    FUN_004a2965(local_1bc);
    if (DAT_0059a1c4 != 0x0) {
        ((*(DAT_0059a1c4 + 0x45) + 0x8))(DAT_0059a1c4,0x2);
    }
    if (DAT_0059a1c0 != 0x0) {
        ((*(DAT_0059a1c0 + 0x45) + 0x8))(DAT_0059a1c0,0x2);
    }
    _DAT_004c9750 = 0x0;
    local_3c = local_20;
    ppuStack375 = &PTR_FUN_004c3d34;
    if (local_cb != 0x0) {
        FUN_00499b30(local_1bc,local_cb);
    }
    FUN_0049a1c0(local_1bc,0x1);
    return local_3c;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0043c7a2(param_1: *mut *mut u32,param_2: u32,param_3: u32,param_4: i32) -> u32

{
    let sVar1: i16;
    let mut bVar2: bool;
    u32 **ppuVar3;
    undefined3 extraout_var;
    let mut uVar4: u32;
    let mut pcVar5: String;
    let mut uVar6: u32;
    undefined3 extraout_var_00;
    let mut iVar7: i32;
    short *psVar8;
    let local_1fc4: u8 [0x80];
    let mut local_1f44: String;
    let local_1f40: u8 [0x82c];
    let mut local_1714: u32;
    let mut local_1710: i32;
    let mut local_170c: i32;
    let mut local_1708: *mut u8;
    let mut local_1704: String;
    let local_1700: u8 [0x80];
    let mut local_1680: String;
    let local_167c: u8 [0x82c];
    let mut local_e50: u32;
    let mut local_e4c: i32;
    let mut local_e48: i32;
    let mut local_e44: *mut u8;
    let local_e40: *mut u32;
    let mut local_e3c: i32;
    let mut local_e38: u32;
    let mut local_e34: u32;
    let mut local_e30: u32;
    let mut local_e2c: i32;
    let mut local_e28: u32;
    let mut local_e24: u32;
    let mut local_e20: u32;
    let mut local_e1c: u32;
    let mut local_e18: u32;
    let local_e14: *mut u32;
    let mut local_e10: u32;
    let mut local_e0c: i32;
    let mut local_e08: u32;
    let mut local_e04: i32;
    let mut local_e00: u32;
    let mut local_dfc: u32;
    let mut local_df8: u32;
    let mut local_df4: i32;
    let mut local_df0: u32;
    let mut local_dec: u32;
    let mut local_de8: u32;
    let mut local_de4: u32;
    let local_de0: *mut u32;
    let mut local_ddc: u32;
    let local_dd8: *mut u32;
    let mut local_dd4: u32;
    let mut local_dd0: i32;
    let mut local_dcc: i32;
    let mut local_dc8: u32;
    ushort *local_dc4;
    ushort *local_dc0;
    let mut local_dbc: u32;
    let mut local_db8: u32;
    let local_db4: *mut u32;
    let mut local_db0: u32;
    let mut local_dac: i32;
    let mut local_da8: i32;
    let mut local_da4: u32;
    let mut local_da0: u32;
    let local_d9c: *mut u32;
    let mut local_d98: u32;
    let mut local_d94: u32;
    let mut local_d90: u32;
    let mut local_d8c: u32;
    let mut local_d88: i32;
    let mut local_d84: i32;
    let mut local_d80: i32;
    let mut local_d7c: i32;
    let mut local_d78: u32;
    let mut local_d74: u32;
    let mut local_d70: i32;
    let mut local_d6c: u32;
    let mut local_d68: u32;
    let mut local_d64: i32;
    let mut local_d60: u32;
    let mut local_d5c: u32;
    let mut local_d58: u32;
    let mut local_d54: u32;
    let mut local_d50: u32;
    let mut local_d4c: u32;
    let mut local_d48: i32;
    let mut local_d44: i32;
    let mut local_d40: i32;
    let mut local_d3c: u32;
    ushort *local_d38;
    ushort *local_d34;
    let mut local_d30: u32;
    let mut local_d2c: u32;
    let local_d28: *mut u32;
    let mut local_d24: i32;
    let mut local_d20: i32;
    let mut local_d1c: i32;
    let mut local_d18: i32;
    let mut local_d14: i32;
    let mut local_d10: i32;
    let mut local_d0c: i32;
    let mut local_d08: i32;
    let mut local_d04: u32;
    ushort *local_d00;
    ushort *local_cfc;
    let mut local_cf8: i32;
    let mut local_cf4: u32;
    let mut local_cf0: u32;
    let mut local_cec: u32;
    let mut local_ce8: i32;
    let mut local_ce4: u32;
    let mut local_ce0: u32;
    let mut local_cdc: u32;
    let mut local_cd8: u32;
    let mut local_cd4: u32;
    let mut local_cd0: u32;
    let local_ccc: *mut u32;
    let mut local_cc8: u32;
    let mut local_cc4: u32;
    let mut local_cc0: u32;
    let local_cbc: *mut u32;
    let mut local_cb8: u32;
    let mut local_cb4: u32;
    let local_cb0: *mut u32;
    let mut local_cac: u32;
    ushort *local_ca8;
    ushort *local_ca4;
    let mut local_ca0: i32;
    let mut local_c9c: u32;
    let mut local_c98: u32;
    let mut local_c94: u32;
    let mut local_c90: i32;
    let mut local_c8c: u32;
    let mut local_c88: u32;
    let mut local_c84: u32;
    let mut local_c80: u32;
    let mut local_c7c: u32;
    let mut local_c78: u32;
    let local_c74: *mut u32;
    let mut local_c70: u32;
    let mut local_c6c: u32;
    let mut local_c68: u32;
    let local_c64: *mut u32;
    let mut local_c60: u32;
    ushort *local_c5c;
    ushort *local_c58;
    let mut local_c54: i32;
    let mut local_c50: u32;
    let mut local_c4c: u32;
    let mut local_c48: u32;
    let mut local_c44: i32;
    let mut local_c40: u32;
    let mut local_c3c: u32;
    let mut local_c38: u32;
    let mut local_c34: u32;
    let mut local_c30: u32;
    let mut local_c2c: u32;
    let local_c28: *mut u32;
    let mut local_c24: u32;
    let mut local_c20: u32;
    let mut local_c1c: u32;
    let local_c18: *mut u32;
    let mut local_c14: u32;
    let mut local_c10: u32;
    let mut local_c0c: u32;
    let mut local_c08: u32;
    let mut local_c04: i32;
    let mut local_c00: u32;
    let mut local_bfc: u32;
    let mut local_bf8: u32;
    let mut local_bf4: u32;
    let mut local_bf0: u32;
    let local_bec: *mut u32;
    let mut local_be8: u32;
    let local_be4: *mut u32;
    let mut local_be0: u32;
    let mut local_bdc: u32;
    let mut local_bd8: u32;
    let mut local_bd4: u32;
    let mut local_bd0: u32;
    let mut local_bcc: u32;
    let mut local_bc8: u32;
    let mut local_bc4: u32;
    let mut local_bc0: u32;
    let mut local_bbc: u32;
    let mut local_bb8: u32;
    let mut local_bb4: u32;
    let mut local_bb0: u32;
    let mut local_bac: i32;
    let mut local_ba8: u32;
    let mut local_ba4: u32;
    let mut local_ba0: u32;
    let mut local_b9c: u32;
    let mut local_b98: u32;
    let mut local_b94: i32;
    let local_b90: *mut u32;
    let mut local_b8c: u32;
    let mut local_b88: u32;
    let local_b84: *mut u32;
    let mut local_b80: u32;
    ushort *local_b7c;
    ushort *local_b78;
    let mut local_b74: i32;
    let mut local_b70: u32;
    let mut local_b6c: u32;
    let mut local_b68: u32;
    let mut local_b64: i32;
    let mut local_b60: u32;
    let mut local_b5c: u32;
    let mut local_b58: u32;
    let mut local_b54: u32;
    let mut local_b50: u32;
    let local_b4c: *mut u32;
    let mut local_b48: u32;
    let mut local_b44: i32;
    let mut local_b40: u32;
    let mut local_b3c: u32;
    let mut local_b38: i32;
    let mut local_b34: u32;
    let mut local_b30: u32;
    let local_b2c: *mut u32;
    let mut local_b28: i32;
    let mut local_b24: u32;
    let mut local_b20: u32;
    let mut local_b1c: u32;
    let mut local_b18: i32;
    let mut local_b14: u32;
    let mut local_b10: u32;
    let mut local_b0c: u32;
    let mut local_b08: i32;
    let mut local_b04: u32;
    ushort *local_b00;
    ushort *local_afc;
    let mut local_af8: u32;
    let mut local_af4: u32;
    let mut local_af0: u32;
    let mut local_aec: u32;
    let mut local_ae8: u32;
    let mut local_ae4: u32;
    let mut local_ae0: u32;
    ushort *local_adc;
    ushort *local_ad8;
    let mut local_ad4: u32;
    let mut local_ad0: u32;
    let local_acc: *mut u32;
    let mut local_ac8: u32;
    let mut local_ac4: u32;
    let mut local_ac0: u32;
    let mut local_abc: i32;
    let mut local_ab8: u32;
    let mut local_ab4: u32;
    let mut local_ab0: u32;
    let mut local_aac: u32;
    let mut local_aa8: u32;
    let mut local_aa4: i32;
    let mut local_aa0: u32;
    let mut local_a9c: u32;
    let mut local_a98: i32;
    let mut local_a94: u32;
    let mut local_a90: u32;
    let mut local_a8c: i32;
    let mut local_a88: i32;
    let mut local_a84: i32;
    let mut local_a80: i32;
    let mut local_a7c: u32;
    let mut local_a78: u32;
    let mut local_a74: i32;
    let mut local_a70: u32;
    let mut local_a6c: u32;
    let mut local_a68: i32;
    let mut local_a64: u32;
    let mut local_a60: u32;
    let mut local_a5c: u32;
    let mut local_a58: i32;
    let mut local_a54: u32;
    let mut local_a50: u32;
    let mut local_a4c: u32;
    let mut local_a48: u32;
    let mut local_a44: i32;
    let mut local_a40: i32;
    let mut local_a3c: i32;
    let mut local_a38: i32;
    let mut local_a34: i32;
    let mut local_a30: u32;
    ushort *local_a2c;
    ushort *local_a28;
    let mut local_a24: u32;
    let mut local_a20: u32;
    let local_a1c: *mut u32;
    let mut local_a18: u32;
    let mut local_a14: u32;
    let mut local_a08: u32;
    ushort *local_a04;
    ushort *local_a00;
    let mut local_9fc: u32;
    let mut local_9f8: u32;
    let mut local_9f4: u32;
    let mut local_9f0: u32;
    let mut local_9ec: u32;
    let mut local_9e8: u32;
    let mut local_9e4: i32;
    let mut local_9e0: u32;
    let mut local_9dc: u32;
    let mut local_9d8: u32;
    let mut local_9d4: u32;
    let mut local_9d0: u32;
    let mut local_9cc: u32;
    let mut local_9c8: u32;
    let mut local_9c4: u32;
    let mut local_9c0: u32;
    let mut local_9bc: u32;
    let mut local_9b8: u32;
    let mut local_9b4: u32;
    let mut local_9b0: u32;
    let mut local_9ac: u32;
    let mut local_9a8: u32;
    let mut local_9a4: u32;
    let mut local_9a0: u32;
    let mut local_99c: u32;
    let mut local_998: u32;
    let mut local_994: u32;
    let mut local_990: u32;
    let mut local_98c: u32;
    let mut local_988: u32;
    let mut local_984: u32;
    let mut local_980: u32;
    let mut local_97c: u32;
    let mut local_978: u32;
    let mut local_974: u32;
    let mut local_970: u32;
    let mut local_96c: u32;
    let mut local_968: u32;
    let mut local_964: u32;
    let mut local_960: u32;
    let mut local_95c: u32;
    let mut local_958: u32;
    let mut local_954: u32;
    let mut local_950: u32;
    let mut local_94c: u32;
    let mut local_948: u32;
    let mut local_944: u32;
    let mut local_940: u32;
    let mut local_93c: u32;
    let mut local_938: u32;
    let mut local_934: u32;
    let mut local_930: u32;
    let mut local_92c: u32;
    let mut local_928: u32;
    let mut local_924: u32;
    let mut local_920: i32;
    let mut local_91c: i32;
    let mut local_918: i32;
    let mut local_914: i32;
    let mut local_910: i32;
    let mut local_90c: u32;
    let mut local_908: u32;
    ushort *local_904;
    ushort *local_900;
    let local_8fc: *mut u32;
    let mut local_8f8: u32;
    let mut local_8f4: u32;
    let local_8f0: *mut u32;
    let mut local_8ec: u32;
    let mut local_8e8: u32;
    let mut local_37c: u32;
    let mut local_378: u32;
    let mut local_374: u32;
    let mut local_370: u32;
    let mut local_36c: i32;
    let mut local_368: u32;
    let mut local_364: u32;
    let mut local_360: u32;
    let mut local_35c: u32;
    let mut local_358: i32;
    let mut local_354: i32;
    let mut local_350: i32;
    let mut local_34c: i32;
    let mut local_348: i32;
    let mut local_344: u32;
    ushort *local_340;
    ushort *local_33c;
    let mut local_338: u32;
    let mut local_334: u32;
    let local_330: *mut u32;
    let mut local_32c: u32;
    let mut local_328: u32;
    let mut local_324: u32;
    let mut local_320: u32;
    let mut local_31c: u32;
    let mut local_318: u32;
    let mut local_314: u32;
    let mut local_310: u32;
    let mut local_30c: u32;
    let mut local_308: u32;
    let mut local_304: u32;
    let mut local_300: u32;
    let mut local_2fc: u32;
    let mut local_2f8: i32;
    let mut local_2f4: u32;
    let mut local_2f0: u32;
    let mut local_2ec: u32;
    let mut local_2e8: u32;
    let mut local_2e4: u32;
    let mut local_2e0: u32;
    let mut uStack732: u32;
    let mut local_2d8: u32;
    let mut uStack724: u32;
    let mut local_2d0: u32;
    let mut local_2cc: i32;
    let mut local_2c8: i32;
    let mut local_2c4: i32;
    let local_2c0: u32;
    let mut local_2bc: u32;
    let mut local_2b8: i32;
    let local_2b4: *mut u32;
    let mut local_2b0: u32;
    let mut local_2ac: i32;
    let local_2a8: *mut u32;
    let mut local_2a4: i32;
    let mut local_2a0: u32;
    let mut local_29c: u32;
    let mut local_298: u32;
    let mut local_294: i32;
    let mut local_290: u32;
    let mut local_28c: u32;
    let local_288: *mut u32;
    let mut local_284: u32;
    let mut local_280: i32;
    let mut local_27c: i32;
    let mut local_278: u32;
    let mut local_274: i32;
    let local_270: *mut u32;
    let mut local_26c: i32;
    let mut local_268: u32;
    let mut local_264: i32;
    let mut local_260: i32;
    let mut local_25c: i32;
    let mut local_258: i32;
    let mut local_254: i32;
    let mut local_250: u32;
    ushort *local_24c;
    ushort *local_248;
    let mut local_244: u32;
    let mut local_240: u32;
    let local_23c: *mut u32;
    let mut local_238: i32;
    let mut local_234: i32;
    let mut local_230: i32;
    let mut local_22c: i32;
    let mut local_228: u32;
    let mut local_224: u32;
    let mut local_220: i32;
    let mut local_21c: u32;
    let mut local_218: u32;
    let mut local_214: u32;
    let mut local_210: u32;
    let mut local_20c: u32;
    let mut local_208: u32;
    let local_204: *mut u32;
    let mut local_200: u32;
    let mut local_1fc: u32;
    let local_1f8: *mut u32;
    let mut local_1f4: u32;
    let local_1f0: *mut u32;
    let mut local_1ec: u32;
    let mut local_1e8: u32;
    let local_1e4: *mut u32;
    let mut local_1e0: u32;
    ushort *local_1dc;
    ushort *local_1d8;
    let mut local_1d4: i32;
    let mut local_1d0: u32;
    let mut local_1cc: u32;
    let mut local_1c8: u32;
    let mut local_1c4: i32;
    let mut local_1c0: u32;
    let mut local_1bc: u32;
    let mut local_1b8: u32;
    let mut local_1b4: u32;
    let local_1b0: *mut u32;
    let mut local_1ac: u32;
    let mut local_1a8: u32;
    let local_1a4: *mut u32;
    let mut local_1a0: i32;
    let mut local_19c: u32;
    let local_198: *mut u32;
    let mut local_194: u32;
    let local_190: *mut u32;
    ushort *local_18c;
    ushort *local_188;
    let local_184: *mut u32;
    let mut local_180: i32;
    let mut local_17c: u32;
    let mut local_178: u32;
    let mut local_174: u32;
    let mut local_170: u32;
    let mut local_16c: u32;
    let mut local_168: i32;
    let mut local_164: u32;
    let mut local_160: u32;
    let mut local_15c: u32;
    let mut local_158: u32;
    let mut local_154: u32;
    let mut local_150: u32;
    let mut local_14c: u32;
    let mut local_148: u32;
    let mut local_144: u32;
    let mut local_140: i32;
    let mut local_13c: u32;
    let mut local_138: u32;
    let mut local_134: u32;
    let mut local_130: u32;
    let mut local_12c: u32;
    let mut local_128: i32;
    let mut local_124: u32;
    let mut local_120: u32;
    let mut local_11c: u32;
    let mut local_118: i32;
    let mut local_114: u32;
    let mut local_110: u32;
    let mut local_10c: u32;
    let mut local_108: i32;
    let mut local_104: u32;
    ushort *local_100;
    ushort *local_fc;
    let mut local_f8: u32;
    let local_f4: *mut u32;
    let mut local_f0: u32;
    let local_ec: *mut u32;
    let mut local_e8: u32;
    let mut local_e4: u32;
    let local_e0: *mut u32;
    let mut local_dc: i32;
    let mut local_d8: u32;
    let mut local_d4: u32;
    let mut local_d0: i32;
    let mut local_cc: u32;
    let mut local_c8: u32;
    let mut local_c4: u32;
    let mut local_c0: u32;
    code *local_bc;
    let mut local_b8: u32;
    let mut local_b4: u32;
    let mut local_b0: i32;
    let mut local_ac: u32;
    let mut local_a8: u32;
    let mut local_a4: i32;
    let mut local_a0: u32;
    let mut local_9c: u32;
    let mut local_98: i32;
    let mut local_94: u32;
    let mut local_90: u32;
    let mut local_8c: u32;
    let mut local_88: i32;
    let mut local_84: u32;
    let mut local_80: u32;
    let mut local_7c: u32;
    ushort *local_78;
    ushort *local_74;
    let mut local_70: i32;
    let mut local_6c: i32;
    let mut local_68: i32;
    let mut local_64: u32;
    let mut local_60: i32;
    let mut local_5c: i32;
    let mut local_58: u32;
    let local_54: *mut u32;
    let mut local_50: u32;
    let mut local_4c: u32;
    let mut local_48: i32;
    let mut local_44: u32;
    let mut local_40: u32;
    let mut local_3c: i32;
    let mut local_38: u32;
    let mut local_34: u32;
    let mut local_30: i32;
    let mut local_2c: u32;
    let mut local_28: u32;
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_14: u32;

    local_14 = param_2;
    if (0x401 < param_2) {
        if (param_2 < 0x403) {
            if (DAT_005967bc != 0x0) {
                local_cc = DAT_005967bc + 0x20;
                local_c4 = local_cc & 0xffff0000 | (DAT_005967bc + 0x24);
                local_d0 = (DAT_005967bc + 0x24);
                local_d8 = DAT_005967bc + 0x20;
                local_c0 = local_d8 & 0xffff0000 | (DAT_005967bc + 0x22);
                local_d4 = local_d8;
                local_c8 = local_cc;
                FUN_00445021((DAT_005967bc + 0x22),local_d0,0x0);
            }
            return 0x1;
        }
        if (param_2 < 0x407) {
            if (param_2 < 0x404) {
                if (_DAT_0059a1bc != 0x0) {
                    _DAT_0059a1bc = 0x0;
                    FUN_004a756b();
                }
                return 0x0;
            }
            if (param_2 != 0x405) {
                return 0x0;
            }
            FUN_004953d7();
            FUN_004a756b();
            local_22c = DAT_0059a1c0;
            local_230 = *(DAT_0059a1c0 + 0xa9);
            FUN_0043bd09(local_230,0x0);
            local_234 = DAT_0059a1c0;
            local_238 = *(DAT_0059a1c0 + 0xa9);
            FUN_0043be20(local_238);
            local_23c = &DAT_005967b8;
            local_240 = (DAT_005967bc == 0x0);
            local_244 = local_240;
            if (local_240 == 0x0) {
                local_24c = (DAT_005967bc + 0x20);
                local_250 = local_24c & 0xffff0000 | *local_24c;
                local_254 = *local_24c;
                local_258 = DAT_0059a1c0;
                local_25c = *(DAT_0059a1c0 + 0xa9);
                local_248 = local_24c;
                if (local_25c == local_254) {
                    local_264 = DAT_005967bc + 0x20;
                    local_268 = *(DAT_005967bc + 0x3a) & 0x1;
                    local_260 = local_264;
                    if (local_268 != 0x0) {
                        FUN_0042e871(param_1);
                        FUN_0048fe33(&DAT_00595740,0x0);
                    }
                }
            }
            FUN_004a75a6();
            FUN_0049536f();
            return 0x1;
        }
        if (0x407 < param_2) {
            if (param_2 < 0x411) {
                if (param_2 != 0x40c) {
                    return 0x0;
                }
                FUN_004953d7();
                FUN_004a756b();
                FUN_004a08c5(s_pcx_plnplat3_pcx_004c194b,0x0,0x0,0x280,0x1e0,0x0,0x0,0x0,0x1);
                FUN_004a75a6();
                FUN_0049536f();
                return 0x1;
            }
            if (param_2 < 0x412) {
                local_2a0 = param_3;
                if (param_3 < 0x521) {
                    if (param_3 < 0x51e) {
                        if (param_3 != 0x97) {
                            return 0x1;
                        }
                    }
                    else {
                        if ((0x51e < param_3) && (param_3 != 0x51f)) {
                            return 0x1;
                        }
                    }
                }
                else {
                    if ((((0x521 < param_3) && (0x522 < param_3)) && (0x523 < param_3)) &&
                        ((0x524 < param_3 && (param_3 != 0x525)))) {
                        return 0x1;
                    }
                }
                FUN_0043586e(0x1);
                return 0x1;
            }
            if (param_2 != 0x413) {
                return 0x0;
            }
            if ((param_3 < 0x2328) || (0x233b < param_3)) {
                if ((0x238b < param_3) && (param_3 < 0x2399)) {
                    FUN_004a756b();
                    FUN_00483355(0x1a);
                    FUN_004a75a6();
                    local_294 = DAT_0059a1c0;
                    local_298 = *(DAT_0059a1c0 + 0xa9);
                    FUN_00435524(local_298,param_3 - 0x238c);
                    return 0x0;
                }
                local_29c = param_3;
                if (param_3 < 0x521) {
                    if (param_3 < 0x51e) {
                        if (param_3 != 0x97) {
                            return 0x0;
                        }
                    }
                    else {
                        if ((0x51e < param_3) && (param_3 != 0x51f)) {
                            return 0x0;
                        }
                    }
                }
                else {
                    if ((((0x521 < param_3) && (0x522 < param_3)) && (0x523 < param_3)) &&
                        ((0x524 < param_3 && (param_3 != 0x525)))) {
                        return 0x0;
                    }
                }
                FUN_0043586e(0x1);
                return 0x0;
            }
            local_274 = param_3 - 0x2328;
            local_270 = &DAT_00595740;
            local_278 = (&DAT_00595744 + local_274 * 0x5e) & 0x1;
            if (((&DAT_00595744 + local_274 * 0x5e) & 0x1) == 0x0) {
                return 0x0;
            }
            iVar7 = *(&DAT_00595740 + local_274 * 0x5e);
            local_280 = iVar7 + 0x20;
            local_284 = *(iVar7 + 0x3a) & 0x1;
            if (local_284 == 0x0) {
                return 0x0;
            }
            local_27c = local_280;
            local_26c = local_274;
            FUN_00489d55(0x0);
            FUN_00445cc3(param_1);
            local_288 = &DAT_005967b8;
            local_28c = (DAT_005967bc == 0x0);
            local_290 = local_28c;
            if (local_28c == 0x0) {
                FUN_00458e3a(DAT_0059a1c0,param_1,0x1);
            }
            else {
                FUN_0044587d(param_1);
            }
            return 0x1;
        }
        if ((param_3 < 0x2328) || (0x233b < param_3)) {
            if ((0x1c84 < param_3) && (param_3 < 0x1c8a)) {
                FUN_00453c16(param_3 - 0x1c84,0x0);
                return 0x1;
            }
            if (param_3 == 0x1c8a) {
                FUN_00489d55(0x1c8a);
                FUN_00445cc3(param_1);
                FUN_00458e3a(DAT_0059a1c0,param_1,0x0);
                local_330 = &DAT_005967b8;
                local_334 = (DAT_005967bc == 0x0);
                local_338 = local_334;
                if (local_334 == 0x0) {
                    local_340 = (DAT_005967bc + 0x20);
                    local_344 = local_340 & 0xffff0000 | *local_340;
                    local_348 = *local_340;
                    local_34c = DAT_0059a1c0;
                    local_350 = *(DAT_0059a1c0 + 0xa9);
                    if (local_350 == local_348) {
                        local_358 = DAT_005967bc + 0x20;
                        local_35c = *(DAT_005967bc + 0x3a) & 0x1;
                        if (local_35c != 0x0) {
                            local_354 = local_358;
                            local_33c = local_340;
                            FUN_00445ba1(0x0);
                            local_364 = DAT_005967bc + 0x20;
                            local_368 = local_364 & 0xffff0000 | (DAT_005967bc + 0x24);
                            local_36c = (DAT_005967bc + 0x24);
                            local_374 = DAT_005967bc + 0x20;
                            local_378 = local_374 & 0xffff0000 | (DAT_005967bc + 0x22);
                            local_370 = local_374;
                            local_360 = local_364;
                            FUN_00445021((DAT_005967bc + 0x22),local_36c,0x1);
                        }
                    }
                }
                return 0x0;
            }
            local_37c = param_3;
            local_8e8 = param_3 - 0x97;
            if (0x2fa2 < local_8e8) {
                return 0x0;
            }
            iVar7 = 0x1e;
            psVar8 = &DAT_004403ad;
            loop {
                if (iVar7 == 0x0) break;
                iVar7 = iVar7 + -0x1;
                sVar1 = *psVar8;
                psVar8 = psVar8 + 0x1;
            } while (local_8e8 != sVar1);
            // WARNING: Could not recover jumptable at 0x00440493. Too many branches
            // WARNING: Treating indirect jump as call
            uVar4 = ((&UNK_004403e7 + iVar7 * 0x4))();
            return uVar4;
        }
        if ((DAT_00595f68 & 0x1) != 0x0) {
            return 0x0;
        }
        if (sRam00595f5a != DAT_004c9754) {
            return 0x0;
        }
        local_2b8 = param_3 - 0x2328;
        local_2a8 = &DAT_00595740;
        local_2b0 = (&DAT_00595744 + local_2b8 * 0x5e) & 0x1;
        if (((&DAT_00595744 + local_2b8 * 0x5e) & 0x1) == 0x0) {
            return 0x1;
        }
        local_2b4 = &DAT_00595740;
        local_2bc = (&DAT_00595744 + local_2b8 * 0x5e) & 0x2;
        local_2ac = local_2b8;
        local_2a4 = local_2b8;
        if ((((&DAT_00595744 + local_2b8 * 0x5e) & 0x2) != 0x0) &&
            (iVar7 = FUN_00452b26(&DAT_00595740,0x1), iVar7 < 0x2)) {
            return 0x0;
        }
        local_2c0 = timeGetTime();
        local_2c4 = *(&DAT_00595740 + local_2a4 * 0x5e);
        if ((local_2c0 - _DAT_0059a1b8 < 0x1f4) && (DAT_004bf380 == local_2a4)) {
            local_2cc = local_2c4 + 0x20;
            local_2d0 = *(local_2c4 + 0x3a) & 0x40;
            local_2c8 = local_2cc;
            if (local_2d0 == 0x0) {
                FUN_00431dec(&DAT_005967b8,local_2c4);
                DAT_005967bc = local_2c4;
                FUN_00450dbf(&DAT_00595740,DAT_00595f52._2_2_,DAT_00595f56,DAT_00595f56._2_2_,
                             local_2c4,0x1,-0x1,0x0);
                DAT_004bf380 = -0x1;^
                // goto LAB_0043dc6d;
            }
        }
        DAT_004bf380 = local_2a4;
        _DAT_0059a1b8 = local_2c0;
        iVar7 = FUN_004512db(&DAT_00595740,local_2a4);
        if (iVar7 == 0x0) {
            return 0x0;
        }
        FUN_00452d01(&DAT_00595740);
        local_2e0 = local_2d8;
        uStack732 = uStack724;
        FUN_00431d5a(&DAT_005967b8,&local_2e0);
        LAB_0043dc6d:
            bVar2 = FUN_00432fca(&DAT_005967b8,0x1);
        if (CONCAT31(extraout_var,bVar2) == 0x0) {
            FUN_0049bf80(param_1,0x521,0x410,0x0,0x0);
        }
        else {
            FUN_0049bf80(param_1,0x521,0x40f,0x0,0x0);
        }
        FUN_0042e871(param_1);
        FUN_0048fe33(&DAT_00595740,0x0);
        if ((DAT_005967bc + 0x41) != -0x1) {
            FUN_0043a597(DAT_0059a1c0);
            local_2e4 = *(DAT_005967bc + 0x42);
            local_2e8 = *(DAT_005967bc + 0x41);
            local_2f0 = DAT_005967bc + 0x20;
            local_2f4 = local_2f0 & 0xffff0000 | (DAT_005967bc + 0x24);
            local_2f8 = (DAT_005967bc + 0x24);
            local_300 = DAT_005967bc + 0x20;
            local_304 = local_300 & 0xffff0000 | (DAT_005967bc + 0x22);
            local_2fc = local_300;
            local_2ec = local_2f0;
            local_308 = FUN_0045af67(&DAT_005967b8,(DAT_005967bc + 0x22),local_2f8,local_2e8,
                                     local_2e4,0x0);
            if (local_308 != 0x0) {
                local_30c = *(DAT_005967bc + 0x42);
                local_310 = *(DAT_005967bc + 0x41);
                local_318 = DAT_005967bc + 0x20;
                local_31c = local_318 & 0xffff0000 | (DAT_005967bc + 0x24);
                local_320 = SEXT24((DAT_005967bc + 0x24));
                local_328 = DAT_005967bc + 0x20;
                local_32c = local_328 & 0xffff0000 | (DAT_005967bc + 0x22);
                local_324 = local_328;
                local_314 = local_318;
                FUN_00449654(DAT_0059a1c0,local_308,(DAT_005967bc + 0x22),local_320,local_310,local_30c);
            }
            FUN_0043a3d6(DAT_0059a1c0,0x1);
        }
        return 0x1;
    }
    if (0x200 < param_2) {
        if (0x201 < param_2) {
            if (0x203 < param_2) {
                if (0x204 < param_2) {
                    if (param_2 != 0x401) {
                        return 0x0;
                    }
                    local_20 = DAT_0059a1c0;
                    local_24 = *(DAT_0059a1c0 + 0xa9);
                    DAT_0059a1b4 = 0xffffffff;
                    _DAT_0059a1b0 = 0xffffffff;
                    local_1c = local_24;
                    if (_DAT_004d555c == 0x0) {
                        if (_DAT_004d5558 != 0x0) {
                            if (*(&DAT_0059a060 + local_24 * 0x4) != -0x1) {
                                FUN_004477de(DAT_0059a1c0,*(&DAT_0059a060 + local_24 * 0x4),
                                             *(&DAT_0059a100 + local_24 * 0x4));
                            }
                            FUN_00495438(&DAT_00594f00,0x20,0x20);
                            FUN_004954f3(0x3,0x3);
                        }
                    }
                    else {
                        if (*(&DAT_0059a060 + local_24 * 0x4) != -0x1) {
                            FUN_004477de(DAT_0059a1c0,*(&DAT_0059a060 + local_24 * 0x4),
                                         *(&DAT_0059a100 + local_24 * 0x4));
                            FUN_00447997(DAT_0059a1c4);
                            iVar7 = DAT_0059a1c4;
                            local_48 = DAT_0059a1c0;
                            local_44 = *(DAT_0059a1c0 + 0x4d);
                            local_3c = DAT_0059a1c0;
                            local_38 = *(DAT_0059a1c0 + 0x49);
                            local_30 = DAT_0059a1c4;
                            *(DAT_0059a1c4 + 0x59) = local_38;
                            *(iVar7 + 0x5d) = local_44;
                            local_40 = local_44;
                            local_34 = local_38;
                            local_2c = local_38;
                            local_28 = local_44;
                            FUN_00447997(DAT_0059a1c4);
                        }
                        FUN_00495438(&DAT_00595300,0x20,0x20);
                        FUN_004954f3(0x10,0x10);
                    }
                    FUN_0049bf80(param_1,0x52b,0x414,0x0,0x0);
                    FUN_0049bf80(param_1,0x52b,0x410,0x0,0x0);
                    FUN_0049bf80(param_1,0x52c,0x414,0x0,0x0);
                    FUN_0049bf80(param_1,0x52c,0x410,0x0,0x0);
                    _DAT_0059aa08 = 0x0;
                    FUN_0043a597(DAT_0059a1c0);
                    local_54 = &DAT_005967b8;
                    local_4c = (DAT_005967bc == 0x0);
                    local_50 = local_4c;
                    if ((local_4c == 0x0) && ((*(DAT_005967bc + 0x3a) & 0x1) != 0x0)) {
                        local_60 = DAT_005967bc + 0x20;
                        local_58 = *(DAT_005967bc + 0x3a) & 0x1;
                        local_5c = local_60;
                        if (local_58 != 0x0) {
                            local_78 = (DAT_005967bc + 0x20);
                            local_64 = local_78 & 0xffff0000 | *local_78;
                            local_6c = *local_78;
                            local_68 = DAT_0059a1c0;
                            local_70 = *(DAT_0059a1c0 + 0xa9);
                            local_74 = local_78;
                            if (local_70 == local_6c) {
                                local_84 = DAT_005967bc + 0x20;
                                local_7c = local_84 & 0xffff0000 | (DAT_005967bc + 0x24);
                                local_88 = (DAT_005967bc + 0x24);
                                local_90 = DAT_005967bc + 0x20;
                                local_94 = local_90 & 0xffff0000 | (DAT_005967bc + 0x22);
                                local_8c = local_90;
                                local_80 = local_84;
                                FUN_0044783a(DAT_0059a1c0,(DAT_005967bc + 0x22),local_88,0x0);
                                iVar7 = DAT_0059a1c4;
                                local_98 = DAT_0059a1c0;
                                local_b8 = *(DAT_0059a1c0 + 0x4d);
                                local_a4 = DAT_0059a1c0;
                                local_b4 = *(DAT_0059a1c0 + 0x49);
                                local_b0 = DAT_0059a1c4;
                                *(DAT_0059a1c4 + 0x59) = local_b4;
                                *(iVar7 + 0x5d) = local_b8;
                                local_ac = local_b4;
                                local_a8 = local_b4;
                                local_a0 = local_b8;
                                local_9c = local_b8;
                                FUN_0042e871(param_1);
                            }
                            else {
                                FUN_0042e642(param_1);
                            }
                        }
                    }
                    else {
                        FUN_0042e642(param_1);
                    }
                    if ((_DAT_004d5558 != 0x0) || (_DAT_004d555c != 0x0)) {
                        FUN_0049bf80(param_1,0x51e,0x410,0x0,0x0);
                        FUN_0049bf80(param_1,0x51f,0x410,0x0,0x0);
                        FUN_0049bf80(param_1,0x521,0x410,0x0,0x0);
                        FUN_0049bf80(param_1,0x523,0x410,0x0,0x0);
                        FUN_0049bf80(param_1,0x522,0x410,0x0,0x0);
                    }
                    FUN_0049f99b(DAT_005b9bd4,param_1,0x407,0x3039,0x0);
                    FUN_004a756b();
                    FUN_004a75c2(0x22,0x22);
                    local_bc = FUN_00444faf;
                    DAT_005b9cec = FUN_00444faf;
                    FUN_004a75a6();
                    return 0x1;
                }
                if ((((param_3 < *(DAT_0059a1c0 + 0x1d)) ||
                    (*(DAT_0059a1c0 + 0x1d) + *(DAT_0059a1c0 + 0x25) <= param_3)) ||
                    (param_4 < *(DAT_0059a1c0 + 0x21))) ||
                    (*(DAT_0059a1c0 + 0x21) + *(DAT_0059a1c0 + 0x29) <= param_4)) {
                    if (param_3 < *(DAT_0059a1c4 + 0x1d)) {
                        return 0x0;
                    }
                    if (*(DAT_0059a1c4 + 0x1d) + *(DAT_0059a1c4 + 0x25) <= param_3) {
                        return 0x0;
                    }
                    if (param_4 < *(DAT_0059a1c4 + 0x21)) {
                        return 0x0;
                    }
                    if (*(DAT_0059a1c4 + 0x21) + *(DAT_0059a1c4 + 0x29) <= param_4) {
                        return 0x0;
                    }
                    local_d08 = DAT_0059a1c4;
                    local_d0c = *(DAT_0059a1c4 + 0xa9);
                    FUN_0042629a(local_d0c);
                    return 0x0;
                }
                DAT_0059a9f4 = 0x0;
                FUN_0044738c(DAT_0059a1c0,param_3,param_4);
                local_a68 = DAT_0059a1c0;
                local_a70 = *(DAT_0059a1c0 + 0x8d);
                local_a74 = DAT_0059a1c0;
                local_a7c = *(DAT_0059a1c0 + 0x91);
                local_a80 = DAT_0059a1c0;
                local_a88 = *(DAT_0059a1c0 + 0xa9);
                local_a84 = local_a88;
                local_a78 = local_a7c;
                local_a6c = local_a70;
                if (_DAT_0059a1bc != 0x0) {
                    _DAT_0059a1bc = 0x0;
                    FUN_004a756b();
                }
                iVar7 = FUN_0044783a(DAT_0059a1c0,local_a70,local_a7c,0x1);
                if (iVar7 != 0x0) {
                    FUN_00447997(DAT_0059a1c4);
                    iVar7 = DAT_0059a1c4;
                    local_a8c = DAT_0059a1c0;
                    local_aac = *(DAT_0059a1c0 + 0x4d);
                    local_a98 = DAT_0059a1c0;
                    local_aa8 = *(DAT_0059a1c0 + 0x49);
                    local_aa4 = DAT_0059a1c4;
                    *(DAT_0059a1c4 + 0x59) = local_aa8;
                    *(iVar7 + 0x5d) = local_aac;
                    local_aa0 = local_aa8;
                    local_a9c = local_aa8;
                    local_a94 = local_aac;
                    local_a90 = local_aac;
                    FUN_00447997(DAT_0059a1c4);
                    FUN_0043a3d6(DAT_0059a1c4,0x1);
                }
                if (DAT_005967bc != 0x0) {
                    local_ab4 = DAT_005967bc + 0x20;
                    local_ab8 = local_ab4 & 0xffff0000 | (DAT_005967bc + 0x24);
                    local_abc = (DAT_005967bc + 0x24);
                    local_ac4 = DAT_005967bc + 0x20;
                    local_ac8 = local_ac4 & 0xffff0000 | (DAT_005967bc + 0x22);
                    local_ac0 = local_ac4;
                    local_ab0 = local_ab4;
                    FUN_00445021((DAT_005967bc + 0x22),local_abc,0x0);
                }
                if (_DAT_004d5558 != 0x0) {
                    return 0x0;
                }
                if (_DAT_004d555c != 0x0) {
                    return 0x0;
                }
                DAT_0059a9f4 = FUN_00481784(local_a88,local_a70,local_a7c);
                FUN_004a756b();
                FUN_00483b22(DAT_0059a9f4);
                FUN_004a75a6();
                local_acc = &DAT_005967b8;
                local_ad0 = (DAT_005967bc == 0x0);
                local_ad4 = local_ad0;
                if (local_ad0 == 0x0) {
                    local_adc = (DAT_005967bc + 0x20);
                    local_ae0 = local_adc & 0xffff0000 | *local_adc;
                    local_ad8 = local_adc;
                    if (*local_adc == local_a88) {
                        local_ae8 = DAT_005967bc + 0x20;
                        local_aec = local_ae8 & 0xffff0000 | (DAT_005967bc + 0x22);
                        local_ae4 = local_ae8;
                        if ((DAT_005967bc + 0x22) == local_a70) {
                            local_af4 = DAT_005967bc + 0x20;
                            local_af8 = local_af4 & 0xffff0000 | (DAT_005967bc + 0x24);
                            local_af0 = local_af4;
                            if ((DAT_005967bc + 0x24) == local_a7c) {
                                local_b00 = (DAT_005967bc + 0x20);
                                local_b04 = local_b00 & 0xffff0000 | *local_b00;
                                local_b08 = *local_b00;
                                local_b10 = DAT_005967bc + 0x20;
                                local_b14 = local_b10 & 0xffff0000 | (DAT_005967bc + 0x22);
                                local_b18 = (DAT_005967bc + 0x22);
                                local_b20 = DAT_005967bc + 0x20;
                                local_b24 = local_b20 & 0xffff0000 | (DAT_005967bc + 0x24);
                                local_b28 = (DAT_005967bc + 0x24);
                                local_b1c = local_b20;
                                local_b0c = local_b10;
                                local_afc = local_b00;
                                FUN_00450dbf(&DAT_00595740,local_b08,local_b18,local_b28,DAT_005967bc,0x1,-0x1,0x0);
                                FUN_00489d55(0x0);
                                FUN_00445cc3(param_1);
                                local_b2c = &DAT_005967b8;
                                local_b30 = (DAT_005967bc == 0x0);
                                local_b34 = local_b30;
                                if (local_b30 != 0x0) {
                                    for (local_b38 = 0x4; local_b38 < 0x7; local_b38 = local_b38 + 0x1) {
                                        local_b3c = FUN_0043a8a2(local_b18 + (&DAT_004bea60)[local_b38]);
                                        local_b40 = FUN_0043a8d5(local_b3c,local_b28 + (&DAT_004bea7c)[local_b38]);
                                        FUN_0043a32d(DAT_0059a1c0,local_b3c,local_b40,0x0,0x1);
                                        FUN_0043a3d6(DAT_0059a1c0,0x0);
                                        FUN_0043a3d6(DAT_0059a1c4,0x1);
                                    }
                                    FUN_0044587d(param_1);
                                    return 0x0;
                                }
                                FUN_00458e3a(DAT_0059a1c0,param_1,0x1);
                                return 0x0;
                            }
                        }
                    }
                }
                FUN_00431d31(&local_b48);
                FUN_00432515(&local_b48,local_a88,local_a70,local_a7c,0x1,-0x1);
                local_b4c = &local_b48;
                local_b50 = (local_b44 == 0x0);
                local_b54 = local_b50;
                if (local_b50 == 0x0) {
                    if (_DAT_0059a1bc != 0x0) {
                        _DAT_0059a1bc = 0x0;
                        FUN_004a756b();
                    }
                    FUN_00431d5a(&DAT_005967b8,&local_b48);
                    FUN_004a756b();
                    FUN_00483978();
                    FUN_004a75a6();
                    local_b5c = DAT_005967bc + 0x20;
                    local_b60 = local_b5c & 0xffff0000 | (DAT_005967bc + 0x24);
                    local_b64 = (DAT_005967bc + 0x24);
                    local_b6c = DAT_005967bc + 0x20;
                    local_b70 = local_b6c & 0xffff0000 | (DAT_005967bc + 0x22);
                    local_b74 = (DAT_005967bc + 0x22);
                    local_b7c = (DAT_005967bc + 0x20);
                    local_b80 = local_b7c & 0xffff0000 | *local_b7c;
                    local_b78 = local_b7c;
                    local_b68 = local_b6c;
                    local_b58 = local_b5c;
                    FUN_00450dbf(&DAT_00595740,*local_b7c,local_b74,local_b64,DAT_005967bc,0x1,-0x1,0x0);
                    FUN_0043a597(DAT_0059a1c0);
                    local_b84 = &DAT_005967b8;
                    local_b88 = (DAT_005967bc == 0x0);
                    local_b8c = local_b88;
                    if ((local_b88 == 0x0) && ((DAT_005967bc + 0x41) != -0x1)) {
                        local_b90 = &DAT_005967b8;
                        local_b94 = *(DAT_005967bc + 0x23) >> 0x18;
                        if (local_b94 == DAT_004c9754) {
                            local_b98 = *(DAT_005967bc + 0x42);
                            local_b9c = *(DAT_005967bc + 0x41);
                            local_ba4 = DAT_005967bc + 0x20;
                            local_ba8 = local_ba4 & 0xffff0000 | (DAT_005967bc + 0x24);
                            local_bac = (DAT_005967bc + 0x24);
                            local_bb4 = DAT_005967bc + 0x20;
                            local_bb8 = local_bb4 & 0xffff0000 | (DAT_005967bc + 0x22);
                            local_bb0 = local_bb4;
                            local_ba0 = local_ba4;
                            local_bbc = FUN_0045af67(&DAT_005967b8,(DAT_005967bc + 0x22),local_bac,
                                                     local_b9c,local_b98,0x0);
                            if (local_bbc != 0x0) {
                                local_bc0 = local_a7c;
                                local_bc4 = local_a70;
                                local_bcc = DAT_005967bc + 0x20;
                                local_bd0 = local_bcc & 0xffff0000 | (DAT_005967bc + 0x24);
                                local_bd4 = SEXT24((DAT_005967bc + 0x24));
                                local_bdc = DAT_005967bc + 0x20;
                                local_be0 = local_bdc & 0xffff0000 | (DAT_005967bc + 0x22);
                                local_bd8 = local_bdc;
                                local_bc8 = local_bcc;
                                FUN_00449654(DAT_0059a1c0,local_bbc,(DAT_005967bc + 0x22),local_bd4,local_a70,
                                             local_a7c);
                            }
                        }
                    }
                    FUN_00445ba1(0x0);
                    FUN_0043a3d6(DAT_0059a1c0,0x1);
                    FUN_0042e871(param_1);
                    FUN_0048fe33(&DAT_00595740,0x0);
                    local_be4 = &DAT_005967b8;
                    local_be8 = 0x2;
                    DAT_005967b8 = DAT_005967b8 | 0x2;
                    if ((*(DAT_005967bc + 0x23) >> 0x18 == DAT_004c9754) &&
                        (iVar7 = FUN_00434cb1(&DAT_005967b8), iVar7 != 0x0)) {
                        FUN_004a756b();
                        FUN_00483355(0x3);
                        FUN_004a75a6();
                    }
                    local_bec = &DAT_005967b8;
                    local_bf0 = (DAT_005967bc == 0x0);
                    local_bf4 = local_bf0;
                    if (local_bf0 != 0x0) {
                        return 0x0;
                    }
                    local_bfc = DAT_005967bc + 0x20;
                    local_c00 = local_bfc & 0xffff0000 | (DAT_005967bc + 0x24);
                    local_c04 = (DAT_005967bc + 0x24);
                    local_c0c = DAT_005967bc + 0x20;
                    local_c10 = local_c0c & 0xffff0000 | (DAT_005967bc + 0x22);
                    local_c08 = local_c0c;
                    local_bf8 = local_bfc;
                    FUN_00445021((DAT_005967bc + 0x22),local_c04,0x0);
                    return 0x0;
                }
                if (DAT_0059a9f4 == 0x0) {
                    return 0x0;
                }
                if ((*(&DAT_004be9b0 + DAT_004c9754 * 0x4) & DAT_0059a9f4[0xb]) != 0x0) {
                    if ((*(DAT_0059a9f4 + 0xe) >> 0x10 == DAT_004c9754) ||
                        (((DAT_0059a9f4 + 0x4) == 0x5 && ((DAT_0059a9f4 + 0xe) == 0x4)))) {
                        local_c14 = FUN_00443baa(DAT_0059a9f4);
                        if ((local_c14 == 0x52c) || (local_c14 == 0x52b)) {
                            FUN_0043c7a2(param_1,0x407,local_c14,0x0);
                        }
                        else {
                            if (DAT_005967bc != 0x0) {
                                local_c18 = DAT_0059a9f4;
                                local_c20 = DAT_005967bc + 0x20;
                                local_c24 = local_c20 & 0xffff0000 | (DAT_005967bc + 0x22);
                                local_c1c = local_c20;
                                if ((DAT_005967bc + 0x22) != (DAT_0059a9f4 + 0xa)) {
                                    local_c28 = DAT_0059a9f4;
                                    local_c30 = DAT_005967bc + 0x20;
                                    local_c34 = local_c30 & 0xffff0000 | (DAT_005967bc + 0x24);
                                    local_c2c = local_c30;
                                    if ((DAT_005967bc + 0x24) != (DAT_0059a9f4 + 0x3)) {
                                        local_c3c = DAT_005967bc + 0x20;
                                        local_c40 = local_c3c & 0xffff0000 | (DAT_005967bc + 0x24);
                                        local_c44 = (DAT_005967bc + 0x24);
                                        local_c4c = DAT_005967bc + 0x20;
                                        local_c50 = local_c4c & 0xffff0000 | (DAT_005967bc + 0x22);
                                        local_c54 = (DAT_005967bc + 0x22);
                                        local_c5c = (DAT_005967bc + 0x20);
                                        local_c60 = local_c5c & 0xffff0000 | *local_c5c;
                                        local_c58 = local_c5c;
                                        local_c48 = local_c4c;
                                        local_c38 = local_c3c;
                                        DAT_0059a9f4 = FUN_00481784(*local_c5c,local_c54,local_c44);
                                    }
                                }
                            }
                        }
                    }
                    else {
                        if (DAT_005967bc != 0x0) {
                            local_c64 = DAT_0059a9f4;
                            local_c6c = DAT_005967bc + 0x20;
                            local_c70 = local_c6c & 0xffff0000 | (DAT_005967bc + 0x22);
                            local_c68 = local_c6c;
                            if ((DAT_005967bc + 0x22) != (DAT_0059a9f4 + 0xa)) {
                                local_c74 = DAT_0059a9f4;
                                local_c7c = DAT_005967bc + 0x20;
                                local_c80 = local_c7c & 0xffff0000 | (DAT_005967bc + 0x24);
                                local_c78 = local_c7c;
                                if ((DAT_005967bc + 0x24) != (DAT_0059a9f4 + 0x3)) {
                                    local_c88 = DAT_005967bc + 0x20;
                                    local_c8c = local_c88 & 0xffff0000 | (DAT_005967bc + 0x24);
                                    local_c90 = (DAT_005967bc + 0x24);
                                    local_c98 = DAT_005967bc + 0x20;
                                    local_c9c = local_c98 & 0xffff0000 | (DAT_005967bc + 0x22);
                                    local_ca0 = (DAT_005967bc + 0x22);
                                    local_ca8 = (DAT_005967bc + 0x20);
                                    local_cac = local_ca8 & 0xffff0000 | *local_ca8;
                                    local_ca4 = local_ca8;
                                    local_c94 = local_c98;
                                    local_c84 = local_c88;
                                    DAT_0059a9f4 = FUN_00481784(*local_ca8,local_ca0,local_c90);
                                }
                            }
                        }
                    }
                    local_cb0 = &DAT_005967b8;
                    local_cb4 = (DAT_005967bc == 0x0);
                    local_cb8 = local_cb4;
                    if (local_cb4 != 0x0) {
                        FUN_0044587d(param_1);
                        return 0x0;
                    }
                    FUN_00458e3a(DAT_0059a1c0,param_1,0x1);
                    return 0x0;
                }
                if (DAT_005967bc == 0x0) {
                    return 0x0;
                }
                local_cbc = DAT_0059a9f4;
                local_cc4 = DAT_005967bc + 0x20;
                local_cc8 = local_cc4 & 0xffff0000 | (DAT_005967bc + 0x22);
                if ((DAT_005967bc + 0x22) == (DAT_0059a9f4 + 0xa)) {
                    return 0x0;
                }
                local_ccc = DAT_0059a9f4;
                local_cd4 = DAT_005967bc + 0x20;
                local_cd8 = local_cd4 & 0xffff0000 | (DAT_005967bc + 0x24);
                if ((DAT_005967bc + 0x24) == (DAT_0059a9f4 + 0x3)) {
                    return 0x0;
                }
                local_ce0 = DAT_005967bc + 0x20;
                local_ce4 = local_ce0 & 0xffff0000 | (DAT_005967bc + 0x24);
                local_ce8 = (DAT_005967bc + 0x24);
                local_cf0 = DAT_005967bc + 0x20;
                local_cf4 = local_cf0 & 0xffff0000 | (DAT_005967bc + 0x22);
                local_cf8 = (DAT_005967bc + 0x22);
                local_d00 = (DAT_005967bc + 0x20);
                local_d04 = local_d00 & 0xffff0000 | *local_d00;
                local_cfc = local_d00;
                local_cec = local_cf0;
                local_cdc = local_ce0;
                local_cd0 = local_cd4;
                local_cc0 = local_cc4;
                DAT_0059a9f4 = FUN_00481784(*local_d00,local_cf8,local_ce8);
                return 0x0;
            }
            if (param_2 != 0x203) {
                return 0x0;
            }
            if ((((*(DAT_0059a1c0 + 0x1d) <= param_3) &&
                (param_3 < *(DAT_0059a1c0 + 0x1d) + *(DAT_0059a1c0 + 0x25))) &&
                (*(DAT_0059a1c0 + 0x21) <= param_4)) &&
                (param_4 < *(DAT_0059a1c0 + 0x21) + *(DAT_0059a1c0 + 0x29))) {
                FUN_0044738c(DAT_0059a1c0,param_3,param_4);
                local_d10 = DAT_0059a1c0;
                local_d18 = *(DAT_0059a1c0 + 0x8d);
                local_d1c = DAT_0059a1c0;
                local_d24 = *(DAT_0059a1c0 + 0x91);
                local_d28 = &DAT_005967b8;
                local_d2c = (DAT_005967bc == 0x0);
                local_d30 = local_d2c;
                local_d20 = local_d24;
                local_d14 = local_d18;
                if ((local_d2c == 0x0) && (iVar7 = FUN_00434bfa(&DAT_005967b8), iVar7 == 0x0)) {
                    local_d38 = (DAT_005967bc + 0x20);
                    local_d3c = local_d38 & 0xffff0000 | *local_d38;
                    local_d40 = *local_d38;
                    local_d44 = DAT_0059a1c0;
                    local_d48 = *(DAT_0059a1c0 + 0xa9);
                    local_d34 = local_d38;
                    if ((local_d48 == local_d40) && (*(DAT_005967bc + 0x23) >> 0x18 == DAT_004c9754)) {
                        local_d50 = DAT_005967bc + 0x20;
                        local_d54 = local_d50 & 0xffff0000 | (DAT_005967bc + 0x22);
                        local_d4c = local_d50;
                        if ((DAT_005967bc + 0x22) == local_d18) {
                            local_d5c = DAT_005967bc + 0x20;
                            local_d60 = local_d5c & 0xffff0000 | (DAT_005967bc + 0x24);
                            local_d58 = local_d5c;
                            if ((DAT_005967bc + 0x24) == local_d24) {
                                FUN_004348e8(&DAT_005967b8,0x1,0x1);
                                FUN_00451169(&DAT_00595740,DAT_005967bc,0x0);
                                FUN_0042e871(param_1);
                                FUN_0048fe33(&DAT_00595740,0x0);
                                return 0x0;
                            }
                        }
                    }
                }
            }
        }
        if (((param_3 < *(DAT_0059a1c0 + 0x1d)) ||
            (*(DAT_0059a1c0 + 0x1d) + *(DAT_0059a1c0 + 0x25) <= param_3)) ||
            ((param_4 < *(DAT_0059a1c0 + 0x21) ||
                (*(DAT_0059a1c0 + 0x21) + *(DAT_0059a1c0 + 0x29) <= param_4)))) {
            if (param_3 < *(DAT_0059a1c4 + 0x1d)) {
                return 0x0;
            }
            if (*(DAT_0059a1c4 + 0x1d) + *(DAT_0059a1c4 + 0x25) <= param_3) {
                return 0x0;
            }
            if (param_4 < *(DAT_0059a1c4 + 0x21)) {
                return 0x0;
            }
            if (*(DAT_0059a1c4 + 0x21) + *(DAT_0059a1c4 + 0x29) <= param_4) {
                return 0x0;
            }
            FUN_0044738c(DAT_0059a1c4,param_3,param_4);
            if (_DAT_0059a1bc != 0x0) {
                _DAT_0059a1bc = 0x0;
                FUN_004a756b();
            }
            iVar7 = FUN_0044783a(DAT_0059a1c4,*(DAT_0059a1c4 + 0x8d),*(DAT_0059a1c4 + 0x91),0x1);
            if (iVar7 != 0x0) {
                FUN_004477de(DAT_0059a1c0,*(DAT_0059a1c4 + 0x59),*(DAT_0059a1c4 + 0x5d));
                FUN_00439d27(DAT_0059a1c0,0x0);
            }
            if (DAT_005967bc == 0x0) {
                return 0x0;
            }
            FUN_00445021((DAT_005967bc + 0x22),(DAT_005967bc + 0x24),0x0);
            return 0x0;
        }
        FUN_0044738c(DAT_0059a1c0,param_3,param_4);
        local_d64 = DAT_0059a1c0;
        local_d8c = *(DAT_0059a1c0 + 0x8d);
        local_d70 = DAT_0059a1c0;
        local_d90 = *(DAT_0059a1c0 + 0x91);
        local_d7c = DAT_0059a1c0;
        local_d84 = *(DAT_0059a1c0 + 0xa9);
        local_d88 = DAT_0059a1c0;
        local_d98 = *(*(*(DAT_0059a1c0 + 0xa1) + local_d8c * 0x4) + local_d90 * 0x4);
        local_d9c = &DAT_005967b8;
        local_da0 = (DAT_005967bc == 0x0);
        local_da4 = local_da0;
        if (local_da0 == 0x0) {
            local_dac = DAT_005967bc + 0x20;
            local_db0 = *(DAT_005967bc + 0x3a) & 0x1;
            local_da8 = local_dac;
            if (((local_db0 == 0x0) && (_DAT_004d5558 == 0x0)) && (_DAT_004d555c == 0x0)) {
                return 0x0;
            }
        }
        local_db4 = &DAT_005967b8;
        local_db8 = (DAT_005967bc == 0x0);
        local_dbc = local_db8;
        local_d94 = local_d98;
        local_d80 = local_d84;
        local_d78 = local_d90;
        local_d74 = local_d90;
        local_d6c = local_d8c;
        local_d68 = local_d8c;
        if (local_db8 == 0x0) {
            local_dc4 = (DAT_005967bc + 0x20);
            local_dc8 = local_dc4 & 0xffff0000 | *local_dc4;
            local_dc0 = local_dc4;
            if (*local_dc4 == local_d84) {
                local_dd0 = DAT_005967bc + 0x20;
                local_dd4 = *(DAT_005967bc + 0x3a) & 0x1;
                local_dcc = local_dd0;
                if (local_dd4 != 0x0) {
                    local_dd8 = &DAT_005967b8;
                    local_ddc = DAT_005967b8 & 0x1;
                    if (local_ddc != 0x0) {
                        local_de0 = &DAT_005967b8;
                        local_de4 = 0x1;
                        DAT_005967b8 = DAT_005967b8 & 0xfffffffe;
                        local_dec = DAT_005967bc + 0x20;
                        local_df0 = local_dec & 0xffff0000 | (DAT_005967bc + 0x24);
                        local_df4 = (DAT_005967bc + 0x24);
                        local_dfc = DAT_005967bc + 0x20;
                        local_e00 = local_dfc & 0xffff0000 | (DAT_005967bc + 0x22);
                        local_df8 = local_dfc;
                        local_de8 = local_dec;
                        FUN_00445021((DAT_005967bc + 0x22),local_df4,0x0);
                        return 0x0;
                    }
                }
            }
        }
        if (_DAT_004d5558 != 0x0) {
            iVar7 = FUN_00457f10(&DAT_005967b8,local_d84,local_d8c,local_d90,0x1);
            if (iVar7 == 0x0) {
                return 0x0;
            }
            local_e04 = DAT_0059a1c0;
            local_e08 = *(DAT_0059a1c0 + 0x49);
            *(&DAT_0059a060 + local_d84 * 0x4) = local_e08;
            local_e0c = DAT_0059a1c0;
            local_e10 = *(DAT_0059a1c0 + 0x4d);
            *(&DAT_0059a100 + local_d84 * 0x4) = local_e10;
            _DAT_004d5558 = 0x0;
            FUN_00495491();
            FUN_0049bf80(param_1,0x51e,0x40f,0x0,0x0);
            FUN_0049bf80(param_1,0x51f,0x40f,0x0,0x0);
            FUN_0049bf80(param_1,0x523,0x40f,0x0,0x0);
            FUN_0049bf80(param_1,0x522,0x40f,0x0,0x0);
            FUN_0043a3d6(DAT_0059a1c0,0x0);
            FUN_0043a3d6(DAT_0059a1c4,0x1);
            local_e14 = &DAT_005967b8;
            local_e18 = (DAT_005967bc == 0x0);
            local_e1c = local_e18;
            if ((local_e18 == 0x0) && (iVar7 = FUN_00432bd3(&DAT_005967b8), iVar7 != 0x0)) {
                if (_DAT_0059a1bc != 0x0) {
                    FUN_004a756b();
                    _DAT_0059a1bc = 0x0;
                }
                local_e24 = DAT_005967bc + 0x20;
                local_e28 = local_e24 & 0xffff0000 | (DAT_005967bc + 0x24);
                local_e2c = (DAT_005967bc + 0x24);
                local_e34 = DAT_005967bc + 0x20;
                local_e38 = local_e34 & 0xffff0000 | (DAT_005967bc + 0x22);
                local_e30 = local_e34;
                local_e20 = local_e24;
                FUN_00445021((DAT_005967bc + 0x22),local_e2c,0x1);
                FUN_00458e3a(DAT_0059a1c0,param_1,0x1);
                return 0x0;
            }
            FUN_0044587d(param_1);
            return 0x0;
        }
        if (_DAT_004d555c == 0x0) {
            if (DAT_005967bc == 0x0) {
                FUN_0044587d(param_1);
                return 0x0;
            }
            if (((DAT_005967bc + 0x20) != local_d84) || (*(DAT_005967bc + 0x23) >> 0x18 != DAT_004c9754)) {
                FUN_004a2d6b();
                return 0x0;
            }
            iVar7 = FUN_00434bfa(&DAT_005967b8);
            if (iVar7 == 0x0) {
                iVar7 = FUN_00432bd3(&DAT_005967b8);
                if (iVar7 == 0x0) {
                    FUN_0043c7a2(param_1,0x407,0x51e,0x0);
                    return 0x0;
                }
                if (((DAT_005967bc + 0x22) == local_d6c) && ((DAT_005967bc + 0x24) == local_d78))
                {
                    FUN_00432a46(&DAT_005967b8);
                    FUN_0043a597(DAT_0059a1c0);
                }
                else {
                    if ((*(DAT_005967bc + 0x41) == local_d6c) && (*(DAT_005967bc + 0x42) == local_d78)) {
                        FUN_0043a597(DAT_0059a1c0);
                        uVar6 = FUN_0045af67(&DAT_005967b8,(DAT_005967bc + 0x22),
                                             (DAT_005967bc + 0x24),local_d6c,local_d78,0x0);
                        if (uVar6 != 0x0) {
                            FUN_00449654(DAT_0059a1c0,uVar6,(DAT_005967bc + 0x22),(DAT_005967bc + 0x24),
                                         local_d6c,local_d78);
                        }
                        DAT_005967b8 = DAT_005967b8 | 0x1;
                    }
                    else {
                        iVar7 = FUN_0044a87f((DAT_005967bc + 0x22),(DAT_005967bc + 0x24),local_d6c,
                                             local_d78);
                        if (iVar7 == 0x1) {
                            FUN_0045bc82(local_d84);
                            iVar7 = FUN_004350b3(&DAT_005967b8,local_d6c,local_d78);
                            if (iVar7 == 0x0) {
                                FUN_00432a04(&DAT_005967b8,local_d6c,local_d78);
                                FUN_0045bc82((DAT_005967bc + 0x20));
                                FUN_0045b7ac(&DAT_005967b8,(DAT_005967bc + 0x20),
                                             (DAT_005967bc + 0x22),(DAT_005967bc + 0x24));
                                bVar2 = FUN_0045a451(&DAT_005967b8,local_d6c,local_d78);
                                if (CONCAT31(extraout_var_00,bVar2) == 0x0) {
                                    FUN_004a2d6b();
                                    FUN_00432a46(&DAT_005967b8);
                                }
                                else {
                                    DAT_005967b8 = DAT_005967b8 | 0x1;
                                }
                            }
                            else {
                                pcVar5 = FUN_00499050(DAT_0059679c,0x73b2);
                                uVar6 = FUN_0049d2e0(param_1,0x2,pcVar5);
                                if (uVar6 != 0x0) {
                                    FUN_00452646(&DAT_00595740,local_d6c,local_d78);
                                    FUN_0048fe33(&DAT_00595740,0x0);
                                }
                            }
                        }
                        else {
                            FUN_0043a597(DAT_0059a1c0);
                            FUN_00432a04(&DAT_005967b8,local_d6c,local_d78);
                            uVar6 = FUN_0045af67(&DAT_005967b8,(DAT_005967bc + 0x22),
                                                 (DAT_005967bc + 0x24),local_d6c,local_d78,0x0);
                            if (uVar6 == 0x0) {
                                FUN_004a2d6b();
                                FUN_00432a46(&DAT_005967b8);
                            }
                            else {
                                iVar7 = FUN_00449654(DAT_0059a1c0,uVar6,(DAT_005967bc + 0x22),
                                                     (DAT_005967bc + 0x24),local_d6c,local_d78);
                                if (iVar7 == 0x0) {
                                    iVar7 = FUN_0044aa04((DAT_005967bc + 0x20),local_d6c,local_d78);
                                    if (iVar7 != -0x1) {
                                        pcVar5 = FUN_00499050(DAT_0059679c,0x739e);
                                        FUN_0049d2e0(param_1,0x1,pcVar5);
                                    }
                                    FUN_00432a46(&DAT_005967b8);
                                }
                            }
                        }
                    }
                }
                FUN_004a756b();
                FUN_0042e871(param_1);
                FUN_0043a3d6(DAT_0059a1c0,0x1);
                FUN_0043a3d6(DAT_0059a1c4,0x1);
                FUN_004a75a6();
                return 0x0;
            }
            if (((DAT_005967bc + 0x22) == local_d6c) && ((DAT_005967bc + 0x24) == local_d78)) {
                return 0x0;
            }
            pcVar5 = FUN_00499050(DAT_0059679c,0x73a8);
            FUN_0049d2e0(param_1,0x1,pcVar5);
            return 0x0;
        }
        local_e3c = FUN_0044ace5(local_d84,local_d8c,local_d90,0x1);
        if (local_e3c == -0x1) {
            local_e40 = FUN_00481784(local_d84,local_d6c,local_d78);
            if (((local_e40 == 0x0) || (iVar7 = FUN_00481a44(local_e40), iVar7 != 0x0)) ||
                ((local_e40 + 0xe) == 0x2)) {
                pcVar5 = FUN_00499050(DAT_0059679c,0x7159);
                FUN_0049d2e0(param_1,0x1,pcVar5);
                return 0x0;
            }
        }
        else {
            if (local_e3c == DAT_004c9754) {
                pcVar5 = FUN_00499050(DAT_0059679c,0x73a0);
                FUN_0049d2e0(param_1,0x1,pcVar5);
                return 0x0;
            }
            if (local_e3c == 0x8) {
                FUN_00423530(DAT_004c9754);
            }
            else {
                local_e44 = &DAT_004d55a8;
                local_e48 = DAT_004c9754;
                local_e50 = (byte)(&DAT_004d55a8)[DAT_004c9754 * 0xe + local_e3c];
                local_e4c = local_e3c;
                if (local_e50 == 0x0) {
                    iVar7 = FUN_00410fb3(&DAT_005967b8,0x2);
                    if (iVar7 == 0x0) {
                        return 0x0;
                    }
                    FUN_00450dbf(local_167c,local_d84,local_d6c,local_d78,0x0,0x1,local_e3c,0x1);
                    iVar7 = FUN_00452e41(local_167c);
                    if (iVar7 == 0x0) {
                        FUN_00499050(DAT_0059679c,local_e3c + 0x414);
                        if (local_e3c < 0x5) {
                            local_1704 = FUN_00499050(DAT_0059679c,0x713e);
                        }
                        else {
                            local_1704 = FUN_00499050(DAT_0059679c,0x713a);
                        }
                        pcVar5 = FUN_00499050(DAT_0059679c,0x734b);
                        FUN_0049c2e0(local_1700,pcVar5);
                    }
                    else {
                        FUN_00499050(DAT_0059679c,local_e3c + 0x414);
                        if (local_e3c < 0x5) {
                            local_1680 = FUN_00499050(DAT_0059679c,0x713e);
                        }
                        else {
                            local_1680 = FUN_00499050(DAT_0059679c,0x713a);
                        }
                        pcVar5 = FUN_00499050(DAT_0059679c,0x737c);
                        FUN_0049c2e0(local_1700,pcVar5);
                    }
                    uVar6 = FUN_0049d2e0(param_1,0x3,local_1700);
                }
                else {
                    local_1708 = &DAT_004d55a8;
                    local_170c = DAT_004c9754;
                    local_1714 = (byte)(&DAT_004d55a8)[DAT_004c9754 * 0xe + local_e3c];
                    local_1710 = local_e3c;
                    if (local_1714 != 0x1)^ // goto LAB_00442cb4;
                    iVar7 = FUN_00410fb3(&DAT_005967b8,0x2);
                    if (iVar7 == 0x0) {
                        return 0x0;
                    }
                    FUN_00450dbf(local_1f40,local_d84,local_d6c,local_d78,0x0,0x1,local_e3c,0x1);
                    iVar7 = FUN_00452e41(local_1f40);
                    if (iVar7 == 0x0) {
                        FUN_00499050(DAT_0059679c,local_e3c + 0x414);
                        pcVar5 = FUN_00499050(DAT_0059679c,0x7365);
                        FUN_0049c2e0(local_1fc4,pcVar5);
                    }
                    else {
                        FUN_00499050(DAT_0059679c,local_e3c + 0x414);
                        if (local_e3c < 0x5) {
                            local_1f44 = FUN_00499050(DAT_0059679c,0x713e);
                        }
                        else {
                            local_1f44 = FUN_00499050(DAT_0059679c,0x713a);
                        }
                        pcVar5 = FUN_00499050(DAT_0059679c,0x737d);
                        FUN_0049c2e0(local_1fc4,pcVar5);
                    }
                    uVar6 = FUN_0049d2e0(param_1,0x3,local_1fc4);
                }
                if (uVar6 == 0x0) {
                    return 0x0;
                }
            }
        }
        LAB_00442cb4:
            _DAT_004d555c = 0x0;
        FUN_00495491();
        FUN_0049bf80(param_1,0x51e,0x40f,0x0,0x0);
        FUN_0049bf80(param_1,0x51f,0x40f,0x0,0x0);
        FUN_0049bf80(param_1,0x523,0x40f,0x0,0x0);
        FUN_0049bf80(param_1,0x522,0x40f,0x0,0x0);
        if (_DAT_0059a1bc != 0x0) {
            FUN_004a756b();
            _DAT_0059a1bc = 0x0;
        }
        iVar7 = FUN_00410fb3(&DAT_005967b8,0x2);
        if (iVar7 == 0x0) {
            return 0x0;
        }
        FUN_0040a70e(0x2,local_d6c,local_d78,local_e3c,0x1);
        FUN_004864f7();
        FUN_0040c15d(local_d6c,local_d78,local_e3c,0x0);
        FUN_004864f7();
        if (DAT_005967c4 != 0x0) {
            FUN_00431d5a(&DAT_005967b8,&DAT_005967c0);
        }
        FUN_00435409(&DAT_005967b8);
        *(&DAT_0059a060 + local_d84 * 0x4) = *(DAT_0059a1c0 + 0x49);
        *(&DAT_0059a100 + local_d84 * 0x4) = *(DAT_0059a1c0 + 0x4d);
        if ((DAT_005967bc != 0x0) && (iVar7 = FUN_00432bd3(&DAT_005967b8), iVar7 != 0x0)) {
            FUN_0049c140(param_1,0x67);
            return 0x1;
        }
        FUN_0044587d(param_1);
        return 0x0;
    }
    if (param_2 < 0x100) {
        return 0x0;
    }
    if (param_2 < 0x101) {
        local_8ec = param_3 & 0x7f;
        if (local_8ec == 0x39) {
            FUN_0043c7a2(param_1,0x407,0x51e,0x0);
            return 0x1;
        }
        if ((local_8ec < 0x47) || (0x51 < local_8ec)) {
            if (param_4 != 0x0) {
                local_a18 = FUN_004a11c0(param_4);
                local_a14 = local_a18;
                if (local_a18 < 0x4e) {
                    if (local_a18 < 0x42) {
                        if (local_a18 == 0x1b) {
                            return 0x1;
                        }
                    }
                    else {
                        if (local_a18 < 0x43) {
                            FUN_0043c7a2(param_1,0x407,0x527,0x0);
                            return 0x0;
                        }
                        if (local_a18 < 0x44) {
                            local_a1c = &DAT_005967b8;
                            local_a20 = (DAT_005967bc == 0x0);
                            local_a24 = local_a20;
                            if (local_a20 != 0x0) {
                                FUN_0044587d(param_1);
                                return 0x1;
                            }
                            local_a2c = (DAT_005967bc + 0x20);
                            local_a30 = local_a2c & 0xffff0000 | *local_a2c;
                            local_a34 = *local_a2c;
                            local_a38 = DAT_0059a1c0;
                            local_a3c = *(DAT_0059a1c0 + 0xa9);
                            local_a28 = local_a2c;
                            if (local_a3c == local_a34) {
                                local_a44 = DAT_005967bc + 0x20;
                                local_a48 = *(DAT_005967bc + 0x3a) & 0x1;
                                local_a40 = local_a44;
                                if (local_a48 != 0x0) {
                                    if (_DAT_0059a1bc != 0x0) {
                                        FUN_004a756b();
                                        _DAT_0059a1bc = 0x0;
                                    }
                                    local_a50 = DAT_005967bc + 0x20;
                                    local_a54 = local_a50 & 0xffff0000 | (DAT_005967bc + 0x24);
                                    local_a58 = (DAT_005967bc + 0x24);
                                    local_a60 = DAT_005967bc + 0x20;
                                    local_a64 = local_a60 & 0xffff0000 | (DAT_005967bc + 0x22);
                                    local_a5c = local_a60;
                                    local_a4c = local_a50;
                                    FUN_00445021((DAT_005967bc + 0x22),local_a58,0x1);
                                    return 0x0;
                                }
                            }
                            FUN_0049c140(param_1,0x67);
                            return 0x1;
                        }
                        if (local_a18 == 0x47) {
                            if ((param_3 & 0x1200) == 0x0) {
                                FUN_0043c7a2(param_1,0x407,0x522,0x0);
                            }
                            else {
                                grid_opt_00599de0 = grid_opt_00599de0 ^ 0x1;
                                FUN_00430d8a(s_Options_004c198d,&DAT_004c1988,grid_opt_00599de0,&filename_00599c80);
                                FUN_004a756b();
                                *(DAT_0059a1c0 + 0x2d) = *(DAT_0059a1c0 + 0x2d) ^ 0x80;
                                FUN_00439d27(DAT_0059a1c0,0x0);
                                FUN_004a75a6();
                            }
                            return 0x1;
                        }
                    }
                }
                else {
                    if (local_a18 < 0x4f) {
                        if ((param_3 & 0x2400) != 0x0) {
                            FUN_0049c140(param_1,0x65);
                        }
                        return 0x1;
                    }
                    if (local_a18 < 0x54) {
                        if (0x51 < local_a18) {
                            if (local_a18 < 0x53) {
                                FUN_0043c7a2(param_1,0x407,0x526,0x0);
                                return 0x0;
                            }
                            FUN_0043c7a2(param_1,0x407,0x523,0x0);
                        }
                    }
                    else {
                        if (local_a18 < 0x55) {
                            if ((param_3 & 0x1200) == 0x0) {
                                return 0x0;
                            }
                            FUN_004a756b();
                            *(DAT_0059a1c0 + 0x2d) = *(DAT_0059a1c0 + 0x2d) ^ 0x10;
                            FUN_00439d27(DAT_0059a1c0,0x0);
                            FUN_004a75a6();
                            return 0x1;
                        }
                        if (0x56 < local_a18) {
                            if (local_a18 < 0x58) {
                                FUN_0043c7a2(param_1,0x407,0x51f,0x0);
                            }
                            else {
                                if (local_a18 == 0x5a) {
                                    if ((param_3 & 0x1200) != 0x0) {
                                        FUN_00496404(DAT_005b96c8);
                                    }
                                    return 0x1;
                                }
                            }
                        }
                    }
                }
            }
            return 0x0;
        }
        local_8f0 = &DAT_005967b8;
        local_8f4 = (DAT_005967bc == 0x0);
        local_8f8 = local_8f4;
        if (local_8f4 != 0x0) {
            return 0x0;
        }
        local_8fc = &DAT_005967b8;
        local_904 = (DAT_005967bc + 0x20);
        local_90c = local_904 & 0xffff0000 | *local_904;
        local_910 = *local_904;
        local_914 = DAT_0059a1c0;
        local_918 = *(DAT_0059a1c0 + 0xa9);
        if ((local_918 == local_910) && (*(DAT_005967bc + 0x23) >> 0x18 == DAT_004c9754)) {
            local_920 = DAT_005967bc + 0x20;
            local_924 = *(DAT_005967bc + 0x3a) & 0x1;
            if (local_924 == 0x0) {
                return 0x0;
            }
            local_91c = local_920;
            local_908 = local_90c;
            local_900 = local_904;
            iVar7 = FUN_00434bfa(&DAT_005967b8);
            if (iVar7 != 0x0) {
                local_928 = *(DAT_005967bc + 0x41);
                local_930 = DAT_005967bc + 0x20;
                local_934 = local_930 & 0xffff0000 | (DAT_005967bc + 0x22);
                if ((DAT_005967bc + 0x22) == local_928) {
                    local_938 = *(DAT_005967bc + 0x42);
                    local_940 = DAT_005967bc + 0x20;
                    local_944 = local_940 & 0xffff0000 | (DAT_005967bc + 0x24);
                    local_93c = local_940;
                    if ((DAT_005967bc + 0x24) == local_938) {
                        return 0x0;
                    }
                }
                local_92c = local_930;
                pcVar5 = FUN_00499050(DAT_0059679c,0x73a8);
                FUN_0049d2e0(param_1,0x1,pcVar5);
                return 0x0;
            }
            iVar7 = FUN_00432bd3(&DAT_005967b8);
            if (iVar7 == 0x0) {
                return 0x0;
            }
            local_948 = local_8ec;
            local_9e4 = local_8ec - 0x47;
            switch(local_9e4) {
                case 0x0:
                    local_950 = DAT_005967bc + 0x20;
                local_954 = local_950 & 0xffff0000 | (DAT_005967bc + 0x22);
                local_958 = DAT_004bea74 + (DAT_005967bc + 0x22);
                local_960 = DAT_005967bc + 0x20;
                local_964 = local_960 & 0xffff0000 | (DAT_005967bc + 0x24);
                local_968 = DAT_004bea90 + (DAT_005967bc + 0x24);
                local_95c = local_960;
                local_94c = local_950;
                break;
                case 0x1:
                    local_970 = DAT_005967bc + 0x20;
                local_974 = local_970 & 0xffff0000 | (DAT_005967bc + 0x22);
                local_958 = DAT_004bea60 + (DAT_005967bc + 0x22);
                local_97c = DAT_005967bc + 0x20;
                local_980 = local_97c & 0xffff0000 | (DAT_005967bc + 0x24);
                local_968 = DAT_004bea7c + (DAT_005967bc + 0x24);
                local_978 = local_97c;
                local_96c = local_970;
                break;
                case 0x2:
                    local_988 = DAT_005967bc + 0x20;
                local_98c = local_988 & 0xffff0000 | (DAT_005967bc + 0x22);
                local_958 = DAT_004bea64 + (DAT_005967bc + 0x22);
                local_994 = DAT_005967bc + 0x20;
                local_998 = local_994 & 0xffff0000 | (DAT_005967bc + 0x24);
                local_968 = DAT_004bea80 + (DAT_005967bc + 0x24);
                local_990 = local_994;
                local_984 = local_988;
                break;
                default:
                return 0x0;
                case 0x4:
                    case 0x8:
                    local_9a0 = DAT_005967bc + 0x20;
                local_9a4 = local_9a0 & 0xffff0000 | (DAT_005967bc + 0x22);
                local_958 = DAT_004bea70 + (DAT_005967bc + 0x22);
                local_9ac = DAT_005967bc + 0x20;
                local_9b0 = local_9ac & 0xffff0000 | (DAT_005967bc + 0x24);
                local_968 = DAT_004bea8c + (DAT_005967bc + 0x24);
                local_9a8 = local_9ac;
                local_99c = local_9a0;
                break;
                case 0x5:
                    case 0x9:
                    local_9b8 = DAT_005967bc + 0x20;
                local_9bc = local_9b8 & 0xffff0000 | (DAT_005967bc + 0x22);
                local_958 = DAT_004bea6c + (DAT_005967bc + 0x22);
                local_9c4 = DAT_005967bc + 0x20;
                local_9c8 = local_9c4 & 0xffff0000 | (DAT_005967bc + 0x24);
                local_968 = DAT_004bea88 + (DAT_005967bc + 0x24);
                local_9c0 = local_9c4;
                local_9b4 = local_9b8;
                break;
                case 0x6:
                    case 0xa:
                    local_9d0 = DAT_005967bc + 0x20;
                local_9d4 = local_9d0 & 0xffff0000 | (DAT_005967bc + 0x22);
                local_958 = DAT_004bea68 + (DAT_005967bc + 0x22);
                local_9dc = DAT_005967bc + 0x20;
                local_9e0 = local_9dc & 0xffff0000 | (DAT_005967bc + 0x24);
                local_968 = DAT_004bea84 + (DAT_005967bc + 0x24);
                local_9d8 = local_9dc;
                local_9cc = local_9d0;
            }
            local_958 = FUN_0043a8a2(local_958);
            local_968 = FUN_0043a8d5(local_958,local_968);
            local_9ec = DAT_005967bc + 0x20;
            local_9f0 = local_9ec & 0xffff0000 | (DAT_005967bc + 0x22);
            if ((DAT_005967bc + 0x22) == local_958) {
                local_9f8 = DAT_005967bc + 0x20;
                local_9fc = local_9f8 & 0xffff0000 | (DAT_005967bc + 0x24);
                local_9f4 = local_9f8;
                if ((DAT_005967bc + 0x24) == local_968) {
                    return 0x0;
                }
            }
            local_a04 = (DAT_005967bc + 0x20);
            local_a08 = local_a04 & 0xffff0000 | *local_a04;
            local_a00 = local_a04;
            local_9e8 = local_9ec;
            FUN_0045bc82(*local_a04);
            iVar7 = FUN_004350b3(&DAT_005967b8,local_958,local_968);
            if (iVar7 == 0x0) {
                FUN_00432a04(&DAT_005967b8,local_958,local_968);
                DAT_005967b8 = DAT_005967b8 | 0x1;
            }
            else {
                pcVar5 = FUN_00499050(DAT_0059679c,0x73b2);
                uVar6 = FUN_0049d2e0(param_1,0x2,pcVar5);
                if (uVar6 != 0x0) {
                    FUN_00452646(&DAT_00595740,local_958,local_968);
                    FUN_0048fe33(&DAT_00595740,0x0);
                }
            }
            return 0x0;
        }
        return 0x0;
    }
    if (param_2 != 0x113) {
        return 0x0;
    }
    ppuVar3 = (u32 **)FUN_0049ab40();
    if (ppuVar3 != param_1) {
        return 0x1;
    }
    if ((_DAT_00596a44 != 0x0) && (param_4 == 0xfa)) {
        return 0x0;
    }
    if (param_4 != 0x14d) {
        return 0x0;
    }
    local_dc = 0x1;
    local_e0 = &DAT_005967b8;
    local_e4 = (DAT_005967bc == 0x0);
    local_e8 = local_e4;
    if (local_e4 != 0x0)^ // goto LAB_0043d44a;
    local_ec = &DAT_005967b8;
    local_f0 = DAT_005967b8 & 0x1;
    if (local_f0 == 0x0)^ // goto LAB_0043d44a;
    local_dc = 0x0;
    if (((DAT_005967bc + 0x41) == -0x1) || ((DAT_005967bc + 0x42) == -0x1)) {
        FUN_0043a597(DAT_0059a1c0);
        local_f4 = &DAT_005967b8;
        local_f8 = 0x1;
        DAT_005967b8 = DAT_005967b8 & 0xfffffffe;
        local_dc = 0x1;
    }
    else {
        local_100 = (DAT_005967bc + 0x20);
        local_104 = local_100 & 0xffff0000 | *local_100;
        local_108 = *local_100;
        local_110 = DAT_005967bc + 0x20;
        local_114 = local_110 & 0xffff0000 | (DAT_005967bc + 0x22);
        local_118 = (DAT_005967bc + 0x22);
        local_120 = DAT_005967bc + 0x20;
        local_124 = local_120 & 0xffff0000 | (DAT_005967bc + 0x24);
        local_128 = (DAT_005967bc + 0x24);
        local_12c = *(DAT_005967bc + 0x42);
        local_130 = *(DAT_005967bc + 0x41);
        local_138 = DAT_005967bc + 0x20;
        local_13c = local_138 & 0xffff0000 | (DAT_005967bc + 0x24);
        local_140 = (DAT_005967bc + 0x24);
        local_148 = DAT_005967bc + 0x20;
        local_14c = local_148 & 0xffff0000 | (DAT_005967bc + 0x22);
        local_144 = local_148;
        local_134 = local_138;
        local_11c = local_120;
        local_10c = local_110;
        local_fc = local_100;
        local_150 = FUN_0045af67(&DAT_005967b8,(DAT_005967bc + 0x22),local_140,local_130,
                                 local_12c,0x0);
        if (local_150 == 0x0) {
            local_180 = *(DAT_005967bc + 0x41) * 0x4;
            local_184 = &DAT_005967b8;
            local_18c = (DAT_005967bc + 0x20);
            local_17c = local_18c & 0xffff0000 | *local_18c;
            local_188 = local_18c;
            local_178 = local_17c;
            if ((*(*(&DAT_004d7d50 + *local_18c * 0x3890 + local_180) +
                *(DAT_005967bc + 0x42) * 0x4) & 0xf) == 0x0) {
                local_15c = *(DAT_005967bc + 0x41);
                local_158 = *(DAT_005967bc + 0x42);
            }
            else {
                if ((*(*(&DAT_004d7d50 + *(DAT_005967bc + 0x41) * 0x4 + local_108 * 0x3890) +
                    *(DAT_005967bc + 0x42) * 0x4 + 0x4) & 0x2) == 0x0) {
                    local_190 = &DAT_005967b8;
                    local_194 = 0x1;
                    DAT_005967b8 = DAT_005967b8 & 0xfffffffe;
                    local_dc = 0x1;
                }
                else {
                    local_15c = *(DAT_005967bc + 0x41);
                    local_158 = *(DAT_005967bc + 0x42);
                }
            }
        }
        else {
            local_164 = DAT_005967bc + 0x20;
            local_154 = local_164 & 0xffff0000 | (DAT_005967bc + 0x24);
            local_168 = (DAT_005967bc + 0x24);
            local_170 = DAT_005967bc + 0x20;
            local_174 = local_170 & 0xffff0000 | (DAT_005967bc + 0x22);
            local_16c = local_170;
            local_160 = local_164;
            FUN_0045b45b(&DAT_005967b8,local_150,(DAT_005967bc + 0x22),local_168,&local_15c,
                         &local_158);
        }
        if ((local_15c == 0xffffffff) || (local_158 == 0xffffffff)) {
            local_198 = &DAT_005967b8;
            local_19c = 0x1;
            DAT_005967b8 = DAT_005967b8 & 0xfffffffe;
            local_dc = 0x1;
        }
    }
    if (local_dc != 0x0)^ // goto LAB_0043d44a;
    FUN_004953d7();
    if (_DAT_0059a1bc != 0x0) {
        FUN_004a756b();
        _DAT_0059a1bc = 0x0;
    }
    local_1a0 = FUN_00444385(local_15c,local_158,DAT_0059a1c0,DAT_0059a1c4);
    if (_DAT_0059a9f8 != 0x0) {
        _DAT_0059a9f8 = 0x0;
        FUN_00445cc3(param_1);
    }
    if (local_1a0 == 0x0) {
        LAB_0043d23f:
            FUN_0043a597(DAT_0059a1c0);
        local_1e4 = &DAT_005967b8;
        local_1e8 = (DAT_005967bc == 0x0);
        local_1ec = local_1e8;
        if (local_1e8 == 0x0) {
            local_1f0 = &DAT_005967b8;
            local_1f4 = 0x1;
            DAT_005967b8 = DAT_005967b8 & 0xfffffffe;
            FUN_00459921(&DAT_005967b8);
        }
        FUN_0044587d(param_1);
    }
    else {
        local_1a4 = &DAT_005967b8;
        local_1a8 = (DAT_005967bc == 0x0);
        local_1ac = local_1a8;
        if (local_1a8 != 0x0)^ // goto LAB_0043d23f;
        if (local_1a0 == 0x1) {
            if ((*(*(&DAT_004d7d50 + local_15c * 0x4 + local_108 * 0x3890) + local_158 * 0x4 + 0x5) & 0x7) !=
                0x0) {
                FUN_0043a541(DAT_0059a1c0,local_108,local_15c,local_158);
            }
        }
        else {
            if (local_1a0 == -0x1) {
                local_1b0 = &DAT_005967b8;
                local_1b4 = 0x1;
                DAT_005967b8 = DAT_005967b8 & 0xfffffffe;
                local_dc = 0x1;
            }
        }
        local_1bc = DAT_005967bc + 0x20;
        local_1c0 = local_1bc & 0xffff0000 | (DAT_005967bc + 0x24);
        local_1c4 = (DAT_005967bc + 0x24);
        local_1cc = DAT_005967bc + 0x20;
        local_1d0 = local_1cc & 0xffff0000 | (DAT_005967bc + 0x22);
        local_1d4 = (DAT_005967bc + 0x22);
        local_1dc = (DAT_005967bc + 0x20);
        local_1e0 = local_1dc & 0xffff0000 | *local_1dc;
        local_1d8 = local_1dc;
        local_1c8 = local_1cc;
        local_1b8 = local_1bc;
        FUN_00450dbf(&DAT_00595740,*local_1dc,local_1d4,local_1c4,DAT_005967bc,0x1,-0x1,0x0);
        FUN_0042e871(param_1);
        FUN_0048fe33(&DAT_00595740,0x0);
    }
    FUN_0049536f();
    LAB_0043d44a:
        local_1f8 = &DAT_005967b8;
    local_1fc = (DAT_005967bc == 0x0);
    local_200 = local_1fc;
    if (local_1fc != 0x0) {
        return 0x0;
    }
    if (local_dc == 0x0) {
        return 0x0;
    }
    if (_DAT_0059a1bc != 0x0) {
        return 0x0;
    }
    local_204 = &DAT_005967b8;
    local_208 = (DAT_005967bc == 0x0);
    local_20c = local_208;
    if (local_208 != 0x0) {
        FUN_0044587d(param_1);
        return 0x0;
    }
    FUN_00445ba1(0x0);
    FUN_0043a3d6(DAT_0059a1c0,0x1);
    local_21c = DAT_005967bc + 0x20;
    local_210 = local_21c & 0xffff0000 | (DAT_005967bc + 0x24);
    local_220 = (DAT_005967bc + 0x24);
    local_228 = DAT_005967bc + 0x20;
    local_214 = local_228 & 0xffff0000 | (DAT_005967bc + 0x22);
    local_224 = local_228;
    local_218 = local_21c;
    FUN_00445021((DAT_005967bc + 0x22),local_220,local_dc + -0x1);
    return 0x0;
}



fn FUN_00443baa(param_1: i32) -> u32

{
    let mut local_18: u32;

    DAT_0059a9f4 = param_1;
    if ((param_1 + 0xe) == 0x4) {
        if ((param_1 + 0x10) == 0x5) {
            FUN_00450dbf(&DAT_0059a1c8,*(param_1 + 0x6) >> 0x10,*(param_1 + 0x8) >> 0x10,
                         *(param_1 + 0xa) >> 0x10,0x0,0x1,0x5,0x1);
            FUN_0044e6a8();
        }
        local_18 = 0x0;
    }
    else {
        local_18 = FUN_00436fe6(param_1);
    }
    return local_18;
}


fn FUN_0049ebc0(param_1: *mut *mut u32,param_2: i32)

{
    u32 **ppuVar1;
    let puVar2: *mut u32;
    let mut bVar3: bool;
    let mut uVar4: u32;
    let mut extraout_ECX: u32;
    u32 **ppuVar5;
    u32 **ppuVar6;
    ulonglong uVar7;

    if (param_1 == *(u32 ***)(param_1 + 0x12)) {
        loop {
            while( true ) {
                ppuVar6 = (u32 **)param_1[0x2];
                if (ppuVar6 == param_1) {
                    return;
                }
                ppuVar1 = (u32 **)ppuVar6[0x2];
                if (ppuVar1 == (u32 **)0x0) {
                    return;
                }
                if (ppuVar1 == param_1) {
                    return;
                }
                uVar4 = 0x0;
                bVar3 = false;
                if (ppuVar1 == (u32 **)0x0) break;
                loop {
                    ppuVar5 = ppuVar1;
                    uVar7 = FUN_004a2ae0(uVar4,*ppuVar5,*ppuVar5,*ppuVar6);
                    if (((0x0 < param_2) && (uVar7 < 0x0)) || ((uVar4 = extraout_ECX, param_2 < 0x0 && (0x0 < uVar7))))
                    {
                        uVar4 = 0x1;
                        puVar2 = *ppuVar5;
                        *ppuVar5 = *ppuVar6;
                        bVar3 = true;
                        *ppuVar6 = puVar2;
                    }
                    ppuVar1 = (u32 **)ppuVar5[0x2];
                    if (ppuVar1 == param_1)^ // goto LAB_0049ec14;
                    ppuVar6 = ppuVar5;
                } while (ppuVar1 != (u32 **)0x0);
                if (!bVar3) {
                    return;
                }
            }
            LAB_0049ec14:
        } while (bVar3);
    }
    return;
}



fn FUN_0049ec50(param_1: *mut u32) -> u32

{
    let puVar1: *mut u32;

    if (param_1 != (param_1 + 0x12)) {
        return 0x0;
    }
    puVar1 = ((param_1 + 0x12))[0x2];
    (param_1 + 0xe) = puVar1;
    if (param_1 == puVar1) {
        return param_1 ^ puVar1;
    }
    return *puVar1;
}



fn FUN_0049ec70(param_1: *mut u32) -> u32

{
    let puVar1: *mut u32;

    if (param_1 != (param_1 + 0x12)) {
        return 0x0;
    }
    puVar1 = ((param_1 + 0x12))[0x1];
    (param_1 + 0xe) = puVar1;
    if (param_1 == puVar1) {
        return param_1 ^ puVar1;
    }
    return *puVar1;
}



fn FUN_0049ec90(param_1: *mut u32) -> u32

{
    let puVar1: *mut u32;

    if (param_1 != (param_1 + 0x12)) {
        return 0x0;
    }
    puVar1 = (*((param_1 + 0x12) + 0xe) + 0x8);
    if (param_1 == puVar1) {
        return param_1 ^ puVar1;
    }
    (param_1 + 0xe) = puVar1;
    return *puVar1;
}



fn FUN_0049ecc0(param_1: *mut u32) -> u32

{
    let puVar1: *mut u32;

    if (param_1 != (param_1 + 0x12)) {
        return 0x0;
    }
    puVar1 = (*((param_1 + 0x12) + 0xe) + 0x4);
    if (param_1 == puVar1) {
        return param_1 ^ puVar1;
    }
    (param_1 + 0xe) = puVar1;
    return *puVar1;
}



fn FUN_0049ecf0(param_1: i32,param_2: i32) -> u32

{
    let mut iVar1: i32;

    iVar1 = *(param_1 + 0x12);
    if (param_1 == iVar1) {
        *(iVar1 + 0xe) = iVar1;
        loop {
            if (param_2 < 0x0) {
                return *(param_1 + 0xe);
            }
            iVar1 = *(*(param_1 + 0xe) + 0x8);
            *(param_1 + 0xe) = iVar1;
            param_2 = param_2 + -0x1;
        } while (param_1 != iVar1);
    }
    return 0x0;
}



fn FUN_0049ed30(param_1: &mut String) -> u32

{
    LPCSTR *ppCVar1;
    LPCSTR *ppCVar2;

    ppCVar1 = *(LPCSTR **)(param_1 + 0x12);
    if (param_1 == ppCVar1) {
        ppCVar2 = ppCVar1[0x2];
        if (ppCVar2 != ppCVar1) {
            loop {
                if ((ppCVar2 + 0x3) == 0x0) {
                    FUN_0049af50(*ppCVar2);
                }
                ppCVar2 = ppCVar2[0x2];
                FUN_0049af50(ppCVar2[0x1]);
            } while (ppCVar2 != param_1);
        }
        FUN_0049af50(param_1);
        return 0x1;
    }
    return 0x0;
}



fn FUN_0049ed90(param_1: u32) -> u32

{
    let mut uVar1: u32;
    let mut uVar2: u32;

    uVar2 = 0x0;
    if (param_1 != *(param_1 + 0x12)) {
        return 0x0;
    }
    uVar1 = *(*(param_1 + 0x12) + 0x8);
    *(param_1 + 0xe) = uVar1;
    if (param_1 != uVar1) {
        loop {
            uVar1 = *(*(param_1 + 0xe) + 0x8);
            uVar2 = uVar2 + 0x1;
            *(param_1 + 0xe) = uVar1;
        } while (param_1 != uVar1);
        return uVar2;
    }
    return param_1 ^ uVar1;
}



fn FUN_0049edc0(param_1: *mut u32)

{
    *param_1 = *(param_1 + 0xe);
    return;
}



fn FUN_0049edd0(param_1: *mut i32)

{
    if (*param_1 == 0x0) {
        return;
    }
    *(param_1 + 0xe) = *param_1;
    return;
}



fn FUN_0049ede0(param_1: i32) -> u32

{
    return *(param_1 + 0xe);
}



fn FUN_0049edf0(param_1: i32,param_2: i32) -> u32

{
    let mut iVar1: i32;

    if (param_1 == *(param_1 + 0x12)) {
        iVar1 = *(*(param_1 + 0x12) + 0x8);
        *(param_1 + 0xe) = iVar1;
        while (param_1 != iVar1) {
            if (param_2 == **(param_1 + 0xe)) {
                return 0x1;
            }
            iVar1 = (*(param_1 + 0xe))[0x2];
            *(param_1 + 0xe) = iVar1;
        }
    }
    return 0x0;
}



fn FUN_0049ee30(param_1: &mut String,LPCSTR param_2)

{
    let mut pCVar1: String;;
    LPCSTR *ppCVar2;

    ppCVar2 = *(LPCSTR **)(param_1 + 0x12);
    if (param_1 == ppCVar2) {
        loop {
            ppCVar2 = ppCVar2[0x2];
            if (ppCVar2 == param_1) {
                return;
            }
        } while (param_2 != *ppCVar2);
        if ((*(ppCVar2 + 0x3) & 0x1) == 0x0) {
            FUN_0049af50(*ppCVar2);
        }
        pCVar1 = ppCVar2[0x2];
        *(ppCVar2[0x1] + 0x8) = pCVar1;
        *(pCVar1 + 0x4) = ppCVar2[0x1];
        FUN_0049af50(ppCVar2);
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

dword
create_window_fn_0049eee0
(HINSTANCE hinstance_param_1,DWORD param_2,DWORD param_3,LPSTR param_4,LPCSTR str_class_name_5,LPSTR param_6)

{
ATOM reg_class_atom_1;
HWND win_handle;
let piVar1: *mut i32;;
HDC dc_handle;
let mut dev_cap_val: i32;
let mut caption_string: String;
let mut msg_box_text_string: String;
let DVar2: u32;
DWORD *pDVar3;
UINT uType;
WNDCLASSA win_class_38;

// LPCSTR lpWindowName for FindWindowA
// LPCSTR lpClassName for FindWindowA
win_handle = FindWindowA(str_class_name_5,0x0);
if (win_handle == (HWND)0x0) {
piVar1 = FUN_004a2831(0x14);
if (piVar1 != 0x0) {
piVar1 = FUN_004987d6(piVar1,0x7d0,0x7d0,0x0,0x0);
}
DAT_005b9bd0 = 0x0;
DAT_005b9b94 = param_6;
DAT_005b9b98 = piVar1;
FUN_0049c374(param_4);
// LPCSTR lpCursorName for LoadCursorA
// HINSTANCE hInstance for LoadCursorA
win_class_38.hCursor = LoadCursorA((HINSTANCE)0x0,0x7f00);
// LPCSTR lpIconName for LoadIconA
// HINSTANCE hInstance for LoadIconA
win_class_38.hIcon = LoadIconA(hinstance_param_1,0x7f00);
win_class_38.lpszMenuName = 0x0;
win_class_38.lpszClassName = str_class_name_5;
win_class_38.hbrBackground = (HBRUSH)0x0;
win_class_38.hInstance = hinstance_param_1;
win_class_38.style = 0x8;
win_class_38.cbClsExtra = 0x0;
// WNDCLASSA * lpWndClass for RegisterClassA
win_class_38.cbWndExtra = 0x0;
win_class_38.lpfnWndProc = win_func_0049_f380;
reg_class_atom_1 = RegisterClassA(&win_class_38);
if (reg_class_atom_1 != 0x0) {
// LPVOID lpParam for CreateWindowExA
// HINSTANCE hInstance for CreateWindowExA
// HMENU hMenu for CreateWindowExA
// HWND hWndParent for CreateWindowExA
// i32 nHeight for CreateWindowExA
// i32 nWidth for CreateWindowExA
// i32 Y for CreateWindowExA
// i32 X for CreateWindowExA
// DWORD dwStyle for CreateWindowExA
// LPCSTR lpWindowName for CreateWindowExA
// LPCSTR lpClassName for CreateWindowExA
// DWORD dwExStyle for CreateWindowExA
HWND_005b9b84 =
CreateWindowExA(0x0,str_class_name_5,str_class_name_5,0x90080000,0x0,0x0,0x280,0x1e0,(HWND)0x0,(HMENU)0x0,
hinstance_param_1,(LPVOID)0x0);
piVar1 = FUN_004a2831(0x8);
if (piVar1 != 0x0) {
piVar1 = FUN_00498ee0(piVar1,s_win_res_004c346e,s_WindowMess_004c3463);
}
// HWND hWnd for GetDC
DAT_005b9bd8 = piVar1;
dc_handle = GetDC((HWND)0x0);
// i32 index for GetDeviceCaps
// HDC hdc for GetDeviceCaps
dev_cap_val = GetDeviceCaps(dc_handle,BITSPIXEL);
if (dev_cap_val != 0x8) {
// UINT uType for MessageBoxA
uType = 0x40;
caption_string = FUN_00499050(DAT_005b9bd8,0x7d04);
// LPCSTR lpCaption for MessageBoxA
msg_box_text_string = FUN_00499050(DAT_005b9bd8,0x7d03);
// LPCSTR lpText for MessageBoxA
// HWND hWnd for MessageBoxA
MessageBoxA(HWND_005b9b84,msg_box_text_string,caption_string,uType);
}
// HDC hDC for ReleaseDC
// HWND hWnd for ReleaseDC
ReleaseDC((HWND)0x0,dc_handle);
FUN_004a2cab(s_s_click1_wav_004c3476,0x0,0x0,0x0);
FUN_004a2cab(s_s_click2_wav_004c3483,0x0,0x0,0x0);
FUN_004a2cab(s_s_grate1_wav_004c3490,0x0,0x0,0x0);
FUN_004a2cab(s_s_grate2_wav_004c349d,0x0,0x0,0x0);
FUN_004a2cab(s_s_start1_wav_004c34aa,0x0,0x0,0x0);
// BOOL bShow for ShowCursor
_DAT_005b9b80 = hinstance_param_1;
ShowCursor(0x0);
DVar2 = direct_draw_create_00495a72(HWND_005b9b84,0x0,DAT_005b9b94,s_CHAR1216_004c34b7);
DAT_005b96c8 = DAT_005b9230;
FUN_004a3c48(DAT_005b9230,&DAT_005b96d0,0xffffffc0);
pDVar3 = (DWORD *)FUN_004a2831(0x590);
if (pDVar3 != (DWORD *)0x0) {
pDVar3 = config_periodic_timer_0049f686(pDVar3);
}
DAT_005b9bd4 = pDVar3;
FUN_004a02ae(FUN_00495cd1);
hWnd_005b9be0 = HWND_005b9b84;
FUN_0049536f();
win_handle_func_0049fb83(DAT_005b9bd4);
return DVar2;
}
}
else {
// HWND hWnd for SetForegroundWindow
SetForegroundWindow(win_handle);
}
return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0049f160(param_1: i32,param_2: i32,param_3: i32) -> i32

{
let mut bVar1: bool;
let mut iVar2: i32;
let piVar3: *mut i32;;
undefined3 extraout_var;

if (-0x1 < param_1) {
_DAT_005b9b8c = (param_1 == 0x280);
}
if (-0x1 < param_2) {
_DAT_005b9b88 = param_2;
}
if ((_DAT_005b9b88 == 0x0) || (_DAT_005b9b8c == 0x0)) {
piVar3 = DAT_005b9b98;
if (DAT_005b9b98 == 0x0) {
FUN_0049a770(0x0,0x417,0x0,0x0);
if (DAT_005b963c != 0x0) {
bVar1 = FUN_004a12c7();
_DAT_005b9bcc = CONCAT31(extraout_var,bVar1);
FUN_004a778a(&DAT_005b9ba8);
FUN_004a789b();
DAT_005b9b9c = DAT_005b91bc;
DAT_005b9ba0 = DAT_005b96b8;
FUN_00495a31();
FUN_004960df();
DAT_005b9bd0 = 0x1;
}
piVar3 = FUN_004a2831(0x14);
if (piVar3 != 0x0) {
piVar3 = FUN_004987d6(piVar3,0x7d0,0x7d0,0x0,0x0);
}
}
DAT_005b9b98 = piVar3;
return 0x0;
}
if (DAT_005b9b98 == 0x0) {
return 0x1;
}
if (param_3 == 0x0) {
return 0x0;
}
if (DAT_005b9b98 != 0x0) {
piVar3 = FUN_0049890a(DAT_005b9b98);
FUN_0049af50(piVar3);
}
DAT_005b9b98 = 0x0;
if (DAT_005b9bd0 == 0x0) {
iVar2 = 0x0;
}
else {
if (_DAT_005b9bcc != 0x0) {
FUN_004a1243();
}
iVar2 = 0x1;
DAT_005b9bd0 = 0x0;
}
FUN_0049a770(0x0,0x417,0x1,0x0);
return iVar2 + 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

LRESULT win_func_0049_f380(HWND win_handle_1,u32 msg_2,HWND wparam_3,LPARAM lparam_4)

{
LRESULT msg_proc_result;
let mut region_empty: bool;
let mut iVar1: i32;
tagRECT rect_30;
let mut local_20: i32;
let mut local_1c: i32;
let mut local_18: i32;
let mut local_14: i32;

// 0x9f380  1  W?AppWndProc$n(pnvuiuil)l
if (0x11 < msg_2) {
if (msg_2 < 0x13) {
if (DAT_005b9bd4 == 0x0) {
// UINT uExitCode for ExitProcess
// WARNING: Subroutine does not return
ExitProcess((UINT)wparam_3);
}
pop_err_msg_box_and_exit_004a02f5((LPSTR)0x0);
}
else {
if (msg_2 < 0x20) {
if (0x13 < msg_2) {
if (msg_2 < 0x15) {
return 0x1;
}
if (msg_2 == 0x1c) {
FUN_0049f160(-0x1,wparam_3,0x0);
}
}
}
else {
if (msg_2 < 0x21) {
if (_DAT_005b9b80 != 0x0) {
// HCURSOR hCursor for SetCursor
SetCursor((HCURSOR)0x0);
// BOOL bShow for ShowCursor
ShowCursor(0x0);
return 0x1;
}
return 0x1;
}
if (msg_2 < 0x311) {
if (msg_2 != 0x30f)^ // goto LAB_0049f3b5;
}
else {
if (0x311 < msg_2) {
if (msg_2 == 0x3b9) {
if ((wparam_3 == (HWND)0x1) && (_DAT_005b9bdc != (code *)0x0)) {
(*_DAT_005b9bdc)(lparam_4);
return 0x1;
}
return 0x1;
}^
// goto LAB_0049f3b5;
}
if (wparam_3 == win_handle_1)^ // goto LAB_0049f3b5;
}
if (DAT_005b963c != 0x0) {
FUN_00496863();
}
}
}^
// goto LAB_0049f3b5;
}
if (msg_2 < 0x3) {
if ((msg_2 == 0x0) || (msg_2 < 0x2))^ // goto LAB_0049f3b5;
}
else {
if (msg_2 < 0x4)^ // goto LAB_0049f3b5;
if (msg_2 < 0xf) {
if (msg_2 == 0x5) {
FUN_0049f160(lparam_4 & 0xffff,-0x1,0x0);
}^
// goto LAB_0049f3b5;
}
if (msg_2 < 0x10) {
// BOOL bErase for GetUpdateRect
// LPRECT lpRect for GetUpdateRect
// HWND hWnd for GetUpdateRect
region_empty = GetUpdateRect(win_handle_1,(LPRECT)&rect_30,0x0);
if (region_empty != 0x0) {
iVar1 = FUN_0049f160(-0x1,-0x1,0x1);
if (iVar1 != 0x0) {
local_20 = rect_30.left;
local_1c = rect_30.top;
local_18 = rect_30.right - rect_30.left;
local_14 = rect_30.bottom - rect_30.top;
if (iVar1 == 0x2) {
FUN_0049613c();
DAT_005b91bc = 0x0;
}
FUN_0049ad10(&local_20);
if (iVar1 == 0x2) {
FUN_00496863();
FUN_004a7806(&DAT_005b9ba8);
DAT_005b91bc = DAT_005b9b9c + -0x1;
FUN_0049536f();
FUN_004960ff(DAT_005b9ba0);
}
}
// RECT * lpRect for ValidateRect
// HWND hWnd for ValidateRect
ValidateRect(win_handle_1,(RECT *)&rect_30);
}
return 0x0;
}
if (msg_2 != 0x10)^ // goto LAB_0049f3b5;
}
// i32 nExitCode for PostQuitMessage
PostQuitMessage(0x0);
// LAB_0049f3b5:
// LPARAM lParam for DefWindowProcA
// WPARAM wParam for DefWindowProcA
// UINT Msg for DefWindowProcA
// HWND hWnd for DefWindowProcA
msg_proc_result = DefWindowProcA(win_handle_1,msg_2,(WPARAM)wparam_3,lparam_4);
return msg_proc_result;
}



fn FUN_0049f610()

{
    let DVar1: u32;
    let mut local_20: *mut u8;
    let mut local_1c: u32;
    let mut local_18: i32;
    let local_14: i32;

    if (DAT_005b9bd0 != 0x0) {
        loop {
            loop {
                DVar1 = win_func_0049fe83(DAT_005b9bd4,(DWORD *)&local_20,&local_1c,&local_18,&local_14);
            } while (DVar1 == 0x0);
            FUN_0049a850(local_20,local_1c,local_18,local_14);
        } while (DAT_005b9bd0 != 0x0);
    }
    return;
}



DWORD *  config_periodic_timer_0049f686(DWORD *param_1)

{
MMRESULT MVar1;

param_1[0x1] = 0xffffffff;
param_1[0x2] = 0x0;
check_key_state_004b0e63(param_1);
FUN_00495286(0x1000);
// UINT uPeriod for timeBeginPeriod
MVar1 = timeBeginPeriod(0xa);
if (MVar1 != 0x0) {
pop_err_msg_box_and_exit_004a02f5(s_Error_setting_Timer_resolution_004c34c0);
}
return param_1;
}



fn config_periodic_timer_0049f6ee(DWORD param_1) -> u32

{
    // UINT uPeriod for timeEndPeriod
    timeEndPeriod(0xa);
    return param_1;
}



fn FUN_0049f713(param_1: i32) -> u32

{
    let mut local_18: u32;

    if (*(param_1 + 0x4) == *(param_1 + 0x8)) {
        local_18 = 0xffffffff;
    }
    else {
        local_18 = *(param_1 + 0x8);
        *(param_1 + 0x8) = *(param_1 + 0x8) + 0x1;
        if (0x63 < *(param_1 + 0x8)) {
            *(param_1 + 0x8) = *(param_1 + 0x8) + -0x64;
        }
        if (*(param_1 + 0x4) < 0x0) {
            *(param_1 + 0x4) = local_18;
        }
    }
    return local_18;
}



fn FUN_0049f777(param_1: i32) -> u32

{
    let mut local_18: u32;

    if (*(param_1 + 0x4) < 0x0) {
        local_18 = 0xffffffff;
    }
    else {
        local_18 = *(param_1 + 0x4);
        *(param_1 + 0x4) = *(param_1 + 0x4) + 0x1;
        if (0x63 < *(param_1 + 0x4)) {
            *(param_1 + 0x4) = *(param_1 + 0x4) + -0x64;
        }
        if (*(param_1 + 0x4) == *(param_1 + 0x8)) {
            *(param_1 + 0x4) = 0xffffffff;
            *(param_1 + 0x8) = 0x0;
        }
    }
    return local_18;
}



fn FUN_0049f7e6(param_1: i32) -> i32

{
ushort uVar1;
let mut iVar2: i32;
let mut local_20: u32;
let mut local_1c: i32;
let mut local_18: u32;
let mut local_14: i32;

local_18 = 0x65;
local_14 = *(param_1 + 0x4);
loop {
if (*(local_14 * 0xe + param_1 + 0x18) == 0x0) {
uVar1 = (local_14 * 0xe + param_1 + 0x1c);
if (uVar1 < 0x202) {
if (uVar1 < 0x104) {
if (0xff < uVar1) {
if (uVar1 < 0x101)^ // goto LAB_0049f837;
if (uVar1 == 0x101)^ // goto LAB_0049f82b;
}
// LAB_0049f873:
local_20 = 0x64;
}
else {
if (uVar1 < 0x105) {
// LAB_0049f837:
local_20 = 0x55;
}
else {
if (uVar1 < 0x200) {
if (uVar1 != 0x105)^ // goto LAB_0049f873;
// LAB_0049f82b:
local_20 = 0x23;
}
else {
if (0x200 < uVar1)^ // goto LAB_0049f867;
local_20 = 0x14;
}
}
}
}
else {
if (uVar1 < 0x203) {
// LAB_0049f84f:
local_20 = 0x32;
}
else {
if (uVar1 < 0x206) {
if (0x203 < uVar1) {
if (uVar1 < 0x205)^ // goto LAB_0049f867;^
// goto LAB_0049f84f;
}
}
else {
if (0x206 < uVar1) {
if (uVar1 < 0x208) {
// LAB_0049f867:
local_20 = 0x4b;^
// goto LAB_0049f940;
}
if (uVar1 < 0x209)^ // goto LAB_0049f84f;
if (uVar1 != 0x209)^ // goto LAB_0049f873;
}
}
local_20 = 0x46;
}
}
}
else {
local_20 = 0x63;
}
// LAB_0049f940:
if (local_20 < local_18) {
local_1c = local_14;
local_18 = local_20;
}
iVar2 = local_14 + 0x1;
if (0x63 < local_14 + 0x1) {
iVar2 = local_14 + -0x63;
}
local_14 = iVar2;
if (local_14 == *(param_1 + 0x8)) {
return local_1c;
}
} while( true );
}



fn FUN_0049f97a(param_1: i32)

{
    let mut iVar1: i32;

    loop {
    iVar1 = FUN_0049f777(param_1);
} while (iVar1 != -0x1);
    return;
}



fn FUN_0049f99b(param_1: i32,param_2: u32,undefined2 param_3,param_4: u32,param_5: u32)

{
    ushort uVar1;
    let mut iVar2: i32;
    let puVar3: *mut u32;
    let mut local_30: i32;
    let mut local_2c: u32;
    let mut local_28: i32;
    let mut local_24: u32;
    let mut local_18: i32;

    local_18 = FUN_0049f713(param_1);
    if (local_18 < 0x0) {
        local_2c = 0x65;
        local_28 = *(param_1 + 0x4);
        loop {
            if (*(local_28 * 0xe + param_1 + 0x18) == 0x0) {
                uVar1 = (local_28 * 0xe + param_1 + 0x1c);
                if (uVar1 < 0x202) {
                    if (uVar1 < 0x104) {
                        if (0xff < uVar1) {
                            if (uVar1 < 0x101)^ // goto LAB_0049fa0a;
                            if (uVar1 == 0x101)^ // goto LAB_0049f9fe;
                        }
                        LAB_0049fa46:
                            local_24 = 0x64;
                    }
                    else {
                        if (uVar1 < 0x105) {
                            LAB_0049fa0a:
                                local_24 = 0x55;
                        }
                        else {
                            if (uVar1 < 0x200) {
                                if (uVar1 != 0x105)^ // goto LAB_0049fa46;
                                LAB_0049f9fe:
                                    local_24 = 0x23;
                            }
                            else {
                                if (0x200 < uVar1)^ // goto LAB_0049fa3a;
                                local_24 = 0x14;
                            }
                        }
                    }
                }
                else {
                    if (uVar1 < 0x203) {
                        LAB_0049fa22:
                            local_24 = 0x32;
                    }
                    else {
                        if (uVar1 < 0x206) {
                            if (0x203 < uVar1) {
                                if (uVar1 < 0x205)^ // goto LAB_0049fa3a;^
                                // goto LAB_0049fa22;
                            }
                        }
                        else {
                            if (0x206 < uVar1) {
                                if (uVar1 < 0x208) {
                                    LAB_0049fa3a:
                                        local_24 = 0x4b;^
                                    // goto LAB_0049fb13;
                                }
                                if (uVar1 < 0x209)^ // goto LAB_0049fa22;
                                if (uVar1 != 0x209)^ // goto LAB_0049fa46;
                            }
                        }
                        local_24 = 0x46;
                    }
                }
            }
            else {
                local_24 = 0x63;
            }
            LAB_0049fb13:
            if (local_24 < local_2c) {
                local_30 = local_28;
                local_2c = local_24;
            }
            iVar2 = local_28 + 0x1;
            if (0x63 < local_28 + 0x1) {
                iVar2 = local_28 + -0x63;
            }
            local_28 = iVar2;
        } while (local_28 != *(param_1 + 0x8));
        local_18 = local_30;
    }
    puVar3 = (param_1 + 0x18 + local_18 * 0xe);
    *puVar3 = param_2;
    *(puVar3 + 0x1) = param_3;
    *(puVar3 + 0x6) = param_4;
    *(puVar3 + 0xa) = param_5;
    return;
}



fn win_handle_func_0049fb83(param_1: i32) -> u32

{
    let mut BVar1: bool;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let mut iVar4: i32;
    let mut local_38: i32;
    MSG msg_30;

    loop {
    // UINT wRemoveMsg for PeekMessageA
    // UINT wMsgFilterMax for PeekMessageA
    // UINT wMsgFilterMin for PeekMessageA
    // HWND hWnd for PeekMessageA
    // LPMSG lpMsg for PeekMessageA
    BVar1 = PeekMessageA((LPMSG)&msg_30,(HWND)0x0,0x0,0x0,0x1);
    if (BVar1 == 0x0) {
        return 0x0;
    }
    if (msg_30.message < 0x202) {
        if (msg_30.message < 0x104) {
            if ((msg_30.message < 0x100) || ((0x100 < msg_30.message && (msg_30.message != 0x101)))) {
                LAB_0049fd28:
                    local_38 = 0x0;^
                // goto LAB_0049fe1a;
            }
        }
        else {
            if (0x104 < msg_30.message) {
                if (0x1ff < msg_30.message) {
                    if (0x200 < msg_30.message)^ // goto LAB_0049fbf8;^
                    // goto LAB_0049fca4;
                }
                if (msg_30.message != 0x105)^ // goto LAB_0049fd28;
            }
        }
        local_38 = FUN_004b097e(param_1,msg_30.lParam,msg_30.message);
        if (local_38 == -0x2) {
            DAT_msg_ptr_005b9be8 = msg_30.hwnd;
            DAT_005b9bec = msg_30.message;
            local_38 = 0x1;
        }
    }
    else {
        if (msg_30.message < 0x203) {
            *(param_1 + 0x14) = *(param_1 + 0x14) & 0xfe;
            if (*(param_1 + 0x14) == 0x0) {
                ReleaseCapture();
            }
        }
        else {
            if (msg_30.message < 0x206) {
                if (msg_30.message < 0x204) {
                    LAB_0049fbf8:
                    if (*(param_1 + 0x14) == 0x0) {
                        // HWND hWnd for SetCapture
                        SetCapture(hWnd_005b9be0);
                    }
                    *(param_1 + 0x14) = *(param_1 + 0x14) | 0x1;
                }
                else {
                    if (msg_30.message < 0x205)^ // goto LAB_0049fc1a;
                    *(param_1 + 0x14) = *(param_1 + 0x14) & 0xfd;
                    if (*(param_1 + 0x14) == 0x0) {
                        ReleaseCapture();
                    }
                }
            }
            else {
                if (msg_30.message < 0x207) {
                    LAB_0049fc1a:
                    if (*(param_1 + 0x14) == 0x0) {
                        // HWND hWnd for SetCapture
                        SetCapture(hWnd_005b9be0);
                    }
                    *(param_1 + 0x14) = *(param_1 + 0x14) | 0x2;
                }
                else {
                    if (0x207 < msg_30.message) {
                        if (msg_30.message < 0x209) {
                            *(param_1 + 0x14) = *(param_1 + 0x14) & 0xfb;
                            if (*(param_1 + 0x14) == 0x0) {
                                ReleaseCapture();
                            }^
                            // goto LAB_0049fca4;
                        }
                        if (msg_30.message != 0x209)^ // goto LAB_0049fd28;
                    }
                    if (*(param_1 + 0x14) == 0x0) {
                        // HWND hWnd for SetCapture
                        SetCapture(hWnd_005b9be0);
                    }
                    *(param_1 + 0x14) = *(param_1 + 0x14) | 0x4;
                }
            }
        }
        LAB_0049fca4:
            local_38 = 0x1;
        uVar2 = msg_30.lParam & 0xffff;
        uVar3 = msg_30.lParam >> 0x10;
        iVar4 = FUN_0049f713(param_1);
        if (-0x1 < iVar4) {
            *(iVar4 * 0xe + param_1 + 0x18) = 0x0;
            (iVar4 * 0xe + param_1 + 0x1c) = msg_30.message;
            *(iVar4 * 0xe + param_1 + 0x1e) = uVar2;
            *(iVar4 * 0xe + param_1 + 0x22) = uVar3;
            FUN_00495145(uVar2,uVar3);
        }
    }
    LAB_0049fe1a:
    if (local_38 == -0x1) {
        // MSG * lpMsg for TranslateMessage
        TranslateMessage((MSG *)&DAT_msg_ptr_005b9be8);
        // MSG * lpMsg for DispatchMessageA
        DispatchMessageA((MSG *)&DAT_msg_ptr_005b9be8);
        local_38 = 0x0;
    }
    if (local_38 == 0x0) {
        // MSG * lpMsg for TranslateMessage
        TranslateMessage(&msg_30);
        // MSG * lpMsg for DispatchMessageA
        DispatchMessageA(&msg_30);
    }
    if (msg_30.message == 0x12) {
        pop_err_msg_box_and_exit_004a02f5((LPSTR)0x0);
    }
} while( true );
}



DWORD
win_func_0049fe83(DWORD *param_1,DWORD *param_2,param_3: *mut u32,param_client_x_coord_4: *mut i32,LONG *param_client_y_coord_5)

{
let mut message_avail: bool;
let mut iVar1: i32;
let sys_time_millis: u32;
tagPOINT point_cursor_pos_58;
let mut local_50: u32;
let mut local_4c: u32;
let msg_ptr_48: u8 [0x4];
let mut local_44: u32;
let mut local_3c: u32;
let mut local_2c: u32;
let mut local_28: i32;
let mut local_24: u32;
DWORD *local_20;
let mut local_1c: i32;
let local_18: u32;
DWORD *local_14;

FUN_004a73f7();
if (DAT_005b9db8 != 0x0) {
FUN_004ae8a8();
}
local_20 = param_1;
loop {
// UINT wRemoveMsg for PeekMessageA
// UINT wMsgFilterMax for PeekMessageA
// UINT wMsgFilterMin for PeekMessageA
// HWND hWnd for PeekMessageA
// LPMSG lpMsg for PeekMessageA
message_avail = PeekMessageA((LPMSG)msg_ptr_48,(HWND)0x0,0x0,0x0,0x1);
if (message_avail == 0x0) {
local_2c = 0x0;
if (param_1[0x1] < 0x0) {
iVar1 = FUN_004a2a3b(param_2,param_client_x_coord_4,param_client_y_coord_5);
if (iVar1 == 0x0) {
sys_time_millis = timeGetTime();
if ((sys_time_millis - param_1[0x4] & 0x7fffffff) < 0xb) {
local_18 = 0x0;
}
else {
*param_2 = 0x0;
sys_time_millis = timeGetTime();
param_1[0x4] = sys_time_millis;
*param_3 = 0x200;
// LPPOINT lpPoint for GetCursorPos
GetCursorPos((LPPOINT)&point_cursor_pos_58);
// LPPOINT lpPoint for ScreenToClient
// HWND hWnd for ScreenToClient
ScreenToClient(hWnd_005b9be0,(LPPOINT)&point_cursor_pos_58);
*param_client_x_coord_4 = point_cursor_pos_58.x;
*param_client_y_coord_5 = point_cursor_pos_58.y;
local_18 = 0x1;
}
}
else {
*param_3 = 0x113;
local_18 = 0x1;
}
}
else {
local_14 = (DWORD *)(param_1 + param_1[0x1] * 0xe + 0x18);
*param_2 = *local_14;
*param_3 = (local_14 + 0x1);
*param_client_x_coord_4 = *(local_14 + 0x6);
*param_client_y_coord_5 = *(LONG *)(local_14 + 0xa);
FUN_0049f777(param_1);
sys_time_millis = timeGetTime();
param_1[0x4] = sys_time_millis;
local_18 = 0x1;
}
return local_18;
}
local_24 = local_44;
if (local_44 < 0x202) {
if (local_44 < 0x104) {
if ((local_44 < 0x100) || ((0x100 < local_44 && (local_44 != 0x101)))) {
// LAB_004a0041:
local_28 = 0x0;^
// goto LAB_004a0133;
}
}
else {
if (0x104 < local_44) {
if (0x1ff < local_44) {
if (0x200 < local_44)^ // goto LAB_0049ff11;^
// goto LAB_0049ffbd;
}
if (local_44 != 0x105)^ // goto LAB_004a0041;
}
}
local_28 = FUN_004b097e(local_20,local_3c,local_44);
if (local_28 == -0x2) {
DAT_msg_ptr_005b9be8 = msg_ptr_48;
DAT_005b9bec = local_44;
local_28 = 0x1;
}
}
else {
if (local_44 < 0x203) {
*(local_20 + 0x5) = *(local_20 + 0x5) & 0xfe;
if (local_20[0x5] == 0x0) {
ReleaseCapture();
}
}
else {
if (local_44 < 0x206) {
if (local_44 < 0x204) {
// LAB_0049ff11:
if (local_20[0x5] == 0x0) {
// HWND hWnd for SetCapture
SetCapture(hWnd_005b9be0);
}
*(local_20 + 0x5) = *(local_20 + 0x5) | 0x1;
}
else {
if (local_44 < 0x205)^ // goto LAB_0049ff33;
*(local_20 + 0x5) = *(local_20 + 0x5) & 0xfd;
if (local_20[0x5] == 0x0) {
ReleaseCapture();
}
}
}
else {
if (local_44 < 0x207) {
// LAB_0049ff33:
if (local_20[0x5] == 0x0) {
// HWND hWnd for SetCapture
SetCapture(hWnd_005b9be0);
}
*(local_20 + 0x5) = *(local_20 + 0x5) | 0x2;
}
else {
if (0x207 < local_44) {
if (local_44 < 0x209) {
*(local_20 + 0x5) = *(local_20 + 0x5) & 0xfb;
if (local_20[0x5] == 0x0) {
ReleaseCapture();
}^
// goto LAB_0049ffbd;
}
if (local_44 != 0x209)^ // goto LAB_004a0041;
}
if (local_20[0x5] == 0x0) {
// HWND hWnd for SetCapture
SetCapture(hWnd_005b9be0);
}
*(local_20 + 0x5) = *(local_20 + 0x5) | 0x4;
}
}
}
// LAB_0049ffbd:
local_28 = 0x1;
local_50 = local_3c & 0xffff;
local_4c = local_3c >> 0x10;
local_1c = FUN_0049f713(local_20);
if (-0x1 < local_1c) {
*(local_20 + local_1c * 0xe + 0x18) = 0x0;
(local_20 + local_1c * 0xe + 0x1c) = local_44;
*(local_20 + local_1c * 0xe + 0x1e) = local_50;
*(local_20 + local_1c * 0xe + 0x22) = local_4c;
FUN_00495145(local_50,local_4c);
}
}
// LAB_004a0133:
if (local_28 == -0x1) {
// MSG * lpMsg for TranslateMessage
TranslateMessage((MSG *)&DAT_msg_ptr_005b9be8);
// MSG * lpMsg for DispatchMessageA
DispatchMessageA((MSG *)&DAT_msg_ptr_005b9be8);
local_28 = 0x0;
}
if (local_28 == 0x0) {
// MSG * lpMsg for TranslateMessage
TranslateMessage((MSG *)msg_ptr_48);
// MSG * lpMsg for DispatchMessageA
DispatchMessageA((MSG *)msg_ptr_48);
}
if (local_44 == 0x12) {
pop_err_msg_box_and_exit_004a02f5((LPSTR)0x0);
}
} while( true );
}



bool  FUN_004a02ae(param_1: u32)

{
let mut bVar1: bool;

bVar1 = DAT_005b9c04 < 0xa;
if (bVar1) {
*(&DAT_005b9c08 + DAT_005b9c04 * 0x4) = param_1;
DAT_005b9c04 = DAT_005b9c04 + 0x1;
}
return bVar1;
}



void pop_err_msg_box_and_exit_004a02f5(LPSTR param_1)

{
let char_array_94: u8 [0x7c];
let local_18: *mut u32;
let mut local_14: u32;

if (param_1 != (LPSTR)0x0) {
local_18 = &stack0x00000008;
FUN_004ae978(char_array_94,param_1,&local_18);
local_18 = 0x0;
}
FUN_00495a31();
FUN_004960df();
for (local_14 = 0x0; local_14 < DAT_005b9c04; local_14 = local_14 + 0x1) {
((&DAT_005b9c08 + local_14 * 0x4))();
}
if ((hWnd_005b9be0 != (HWND)0x0) && (param_1 != (LPSTR)0x0)) {
// UINT uType for MessageBoxA
// LPCSTR lpCaption for MessageBoxA
// LPCSTR lpText for MessageBoxA
// HWND hWnd for MessageBoxA
MessageBoxA(hWnd_005b9be0,char_array_94,s_Exiting_004c34df,0x0);
}
// UINT uExitCode for ExitProcess
// WARNING: Subroutine does not return
ExitProcess(0x1);
}



void  check_win_handle_004a0396(LPSTR param_1)

{
let local_94: u8 [0x7c];
let local_18: *mut u32;
let mut local_14: u32;

if (param_1 != (LPSTR)0x0) {
local_18 = &stack0x00000008;
FUN_004ae978(local_94,param_1,&local_18);
local_18 = 0x0;
}
if ((hWnd_005b9be0 != (HWND)0x0) && (param_1 != (LPSTR)0x0)) {
// UINT uType for MessageBoxA
// LPCSTR lpCaption for MessageBoxA
// LPCSTR lpText for MessageBoxA
// HWND hWnd for MessageBoxA
MessageBoxA(hWnd_005b9be0,local_94,s_Exiting_004c34e7,0x0);
}
for (local_14 = 0x0; local_14 < DAT_005b9c04; local_14 = local_14 + 0x1) {
((&DAT_005b9c08 + local_14 * 0x4))();
}
FUN_004b0ef0();
return;
}



fn FUN_004a0430(param_1: u32,undefined param_2,param_3: u32) -> u32

{
    let mut uVar1: u32;

    uVar1 = CONCAT11(param_2,param_2) & 0xff;
    FUN_004b0f10(param_3,uVar1 | (uVar1 | CONCAT11(param_2,param_2) << 0x8) << 0x8);
    return param_1;
}



char
FUN_004a0450(param_1: u32,param_2: i32,param_3: i32,param_4: u32,param_5: *mut u32,param_6: u32,param_7: i32,
param_8: u32,param_9: i32)

{
let mut iVar1: i32;
let bVar2: u8;
let bVar3: u8;
ushort uVar4;
let bVar9: u8;
let mut uVar5: u32;
let mut iVar6: i32;
let mut iVar7: i32;
let mut puVar8: *mut u8;
let cVar10: u8;
let mut iVar11: i32;
let mut uVar12: u32;
let mut iVar13: i32;
let mut uVar14: u32;
let mut uVar15: u32;
let mut iVar16: i32;
let mut uVar17: u32;
ushort *puVar18;
ushort *puVar19;
let mut puVar20: *mut u8;
let mut bVar21: bool;

uVar5 = *param_5;
bVar2 = (byte)uVar5;
bVar9 = (byte)(uVar5 >> 0x8);
if (bVar9 < bVar2) {
uVar5 = CONCAT31(CONCAT21((uVar5 >> 0x10),bVar2),bVar9);
}
bVar3 = (byte)param_1;
if (((byte)(uVar5 >> 0x8) < bVar3) || (bVar3 < (byte)uVar5)) {
cVar10 = '\0';
}
else {
puVar18 =
((param_5 + 0x1) +
((param_5 + 0x1) + (param_1 & 0xffffff00 | (byte)(bVar3 - (byte)uVar5)) * 0x2)
);
puVar19 = puVar18 + 0x1;
uVar4 = *puVar18;
uVar17 = (byte)(uVar4 >> 0x8) & 0xffffff0f;
uVar15 = uVar4 & 0xfffff0ff;
bVar3 = (byte)uVar15;
uVar15 = CONCAT11((byte)(uVar15 >> 0x8) >> 0x4,bVar3);
uVar14 = (byte)((uVar5 >> 0x18) - (uVar15 >> 0x8));
uVar15 = (uVar15 & 0xff) >> 0x3;
cVar10 = uVar15 + (bVar3 & 0x7);
if ((param_8 < uVar14) && (param_6 < uVar15)) {
if (param_9 < uVar14) {
iVar6 = uVar14 - param_9;
}
else {
iVar6 = 0x0;
}
bVar21 = param_8 < uVar17;
param_8 = param_8 - uVar17;
if (bVar21) {
param_8 = 0x0;
}
uVar14 = (uVar14 - iVar6) - param_8;
iVar6 = uVar14 - uVar17;
if (uVar17 <= uVar14) {
if (param_7 < uVar15) {
iVar7 = uVar15 - param_7;
uVar15 = uVar15 - iVar7;
}
else {
iVar7 = 0x0;
}
iVar16 = uVar15 - param_6;
puVar8 = ((param_3 + uVar17 + param_8) * DAT_005b9224 + DAT_005b9220 + param_2 + param_6);
iVar11 = DAT_005b9224 - iVar16;
if (bVar9 < bVar2) {
uVar15 = 0x0;
if (param_8 != 0x0) {
uVar17 = param_6 + iVar16 + iVar7;
uVar14 = uVar17;
while (uVar12 = uVar17, uVar14 != 0x0) {
loop {
uVar4 = (ushort)uVar15;
puVar18 = puVar19;
if ((uVar15 >> 0x8) == '\0') {
puVar18 = (puVar19 + 0x1);
uVar4 = CONCAT11(0x80,puVar19);
}
uVar15 = (ushort)(uVar4 & 0xff | (uVar4 >> 0xa) << 0x8);
uVar12 = uVar12 - 0x1;
puVar19 = puVar18;
} while (uVar12 != 0x0);
param_8 = param_8 - 0x1;
uVar14 = param_8;
}
}
for (; iVar6 != 0x0; iVar6 = iVar6 + -0x1) {
iVar13 = iVar16;
uVar14 = param_6;
puVar18 = puVar19;
if (param_6 != 0x0) {
loop {
uVar4 = (ushort)uVar15;
puVar19 = puVar18;
if ((uVar15 >> 0x8) == '\0') {
puVar19 = (puVar18 + 0x1);
uVar4 = CONCAT11(0x80,puVar18);
}
uVar15 = (ushort)(uVar4 & 0xff | (uVar4 >> 0xa) << 0x8);
uVar14 = uVar14 - 0x1;
puVar18 = puVar19;
} while (uVar14 != 0x0);
}
loop {
while( true ) {
puVar20 = puVar8;
uVar4 = (ushort)uVar15;
puVar18 = puVar19;
if ((uVar15 >> 0x8) == '\0') {
puVar18 = (puVar19 + 0x1);
uVar4 = CONCAT11(0x80,puVar19);
}
bVar2 = (byte)(uVar4 >> 0x8);
puVar19 = puVar18;
iVar1 = iVar7;
if (((byte)uVar4 & bVar2) != 0x0) break;
uVar4 = uVar4 & 0xff;
if ((byte)((byte)uVar4 & bVar2 >> 0x1) == 0x0) {
uVar4 = uVar4 | (ushort)(bVar2 >> 0x2) << 0x8;
}
else {
*puVar20 = param_4;
uVar4 = uVar4 | (ushort)(bVar2 >> 0x2) << 0x8;
}
joined_r0x004a063c:
uVar15 = uVar4;
iVar13 = iVar13 + -0x1;
puVar8 = puVar20 + 0x1;
if (iVar13 == 0x0)^ // goto joined_r0x004a0659;
}
uVar4 = uVar4 & 0xff;
if ((byte)((byte)uVar4 & bVar2 >> 0x1) != 0x0) {
*puVar20 = (param_4 >> 0x10);
uVar4 = uVar4 | (ushort)(bVar2 >> 0x2) << 0x8;^
// goto joined_r0x004a063c;
}
*puVar20 = (param_4 >> 0x8);
uVar15 = (ushort)(uVar4 | (ushort)(bVar2 >> 0x2) << 0x8);
iVar13 = iVar13 + -0x1;
puVar8 = puVar20 + 0x1;
} while (iVar13 != 0x0);
joined_r0x004a0659:
for (; iVar1 != 0x0; iVar1 = iVar1 + -0x1) {
uVar4 = (ushort)uVar15;
puVar18 = puVar19;
if ((uVar15 >> 0x8) == '\0') {
puVar18 = (puVar19 + 0x1);
uVar4 = CONCAT11(0x80,puVar19);
}
uVar15 = (ushort)(uVar4 & 0xff | (uVar4 >> 0xa) << 0x8);
puVar19 = puVar18;
}
puVar8 = puVar20 + iVar11 + 0x1;
}
}
else {
uVar15 = 0x0;
if (param_8 != 0x0) {
uVar17 = param_6 + iVar16 + iVar7;
uVar14 = uVar17;
while (uVar12 = uVar17, uVar14 != 0x0) {
loop {
uVar4 = (ushort)uVar15;
puVar18 = puVar19;
if ((uVar15 >> 0x8) == '\0') {
puVar18 = (puVar19 + 0x1);
uVar4 = CONCAT11(0x80,puVar19);
}
uVar15 = (ushort)(uVar4 & 0xff | (uVar4 >> 0x9) << 0x8);
uVar12 = uVar12 - 0x1;
puVar19 = puVar18;
} while (uVar12 != 0x0);
param_8 = param_8 - 0x1;
uVar14 = param_8;
}
}
for (; iVar6 != 0x0; iVar6 = iVar6 + -0x1) {
iVar13 = iVar16;
uVar14 = param_6;
puVar18 = puVar19;
if (param_6 != 0x0) {
loop {
uVar4 = (ushort)uVar15;
puVar19 = puVar18;
if ((uVar15 >> 0x8) == '\0') {
puVar19 = (puVar18 + 0x1);
uVar4 = CONCAT11(0x80,puVar18);
}
uVar15 = (ushort)(uVar4 & 0xff | (uVar4 >> 0x9) << 0x8);
uVar14 = uVar14 - 0x1;
puVar18 = puVar19;
} while (uVar14 != 0x0);
}
loop {
while( true ) {
puVar20 = puVar8;
uVar4 = (ushort)uVar15;
puVar18 = puVar19;
if ((uVar15 >> 0x8) == '\0') {
puVar18 = (puVar19 + 0x1);
uVar4 = CONCAT11(0x80,puVar19);
}
bVar2 = (byte)(uVar4 >> 0x8);
puVar19 = puVar18;
iVar1 = iVar7;
if (((byte)uVar4 & bVar2) != 0x0) break;
uVar15 = (ushort)(uVar4 & 0xff | (ushort)(bVar2 >> 0x1) << 0x8);
iVar13 = iVar13 + -0x1;
puVar8 = puVar20 + 0x1;
if (iVar13 == 0x0)^ // goto joined_r0x004a05af;
}
*puVar20 = param_4;
uVar15 = (ushort)(uVar4 & 0xff | (ushort)(bVar2 >> 0x1) << 0x8);
iVar13 = iVar13 + -0x1;
puVar8 = puVar20 + 0x1;
} while (iVar13 != 0x0);
joined_r0x004a05af:
for (; iVar1 != 0x0; iVar1 = iVar1 + -0x1) {
uVar4 = (ushort)uVar15;
puVar19 = puVar18;
if ((uVar15 >> 0x8) == '\0') {
puVar19 = (puVar18 + 0x1);
uVar4 = CONCAT11(0x80,puVar18);
}
uVar15 = (ushort)(uVar4 & 0xff | (uVar4 >> 0x9) << 0x8);
puVar18 = puVar19;
}
puVar8 = puVar20 + iVar11 + 0x1;
puVar19 = puVar18;
}
}
}
}
}
return cVar10;
}



char  FUN_004a06b1(param_1: u32,ushort *param_2)

{
ushort uVar1;
let cVar2: u8;
let bVar3: u8;

uVar1 = *param_2;
bVar3 = (byte)(uVar1 >> 0x8);
if (bVar3 < (byte)uVar1) {
uVar1 = CONCAT11((byte)uVar1,bVar3);
}
bVar3 = (byte)param_1;
if (((byte)(uVar1 >> 0x8) < bVar3) || (bVar3 < (byte)uVar1)) {
cVar2 = '\0';
}
else {
bVar3 = (byte)*
((param_2 + 0x2) +
(param_2 + 0x2)[param_1 & 0xffffff00 | (byte)(bVar3 - (byte)uVar1)]);
cVar2 = (bVar3 & 0x7) + (bVar3 >> 0x3);
}
return cVar2;
}



fn FUN_004a06f6(param_1: i32) -> String

{
    return (param_1 + 0x3);
}



fn FUN_004a070a(param_1: i32,param_2: i32,param_3: i32,param_4: *mut u32,param_5: *mut u32) -> i32

{
let mut iVar1: i32;
let mut local_18: i32;
let mut local_14: i32;

local_18 = 0x0;
local_14 = 0x0;
while ((*param_4 != 0x0 && (local_14 < param_3))) {
(local_14 + param_2) = param_5;
*param_4 = *param_4 - 0x1;
local_14 = local_14 + 0x1;
}
while (local_14 < param_3) {
if ((*(param_1 + local_18) & 0xc0) == 0xc0) {
iVar1 = local_18 + 0x1;
*param_4 = (*(local_18 + param_1) & 0x3f);
local_18 = local_18 + 0x2;
*param_5 = *(iVar1 + param_1);
while ((*param_4 != 0x0 && (local_14 < param_3))) {
(local_14 + param_2) = param_5;
*param_4 = *param_4 - 0x1;
local_14 = local_14 + 0x1;
}
}
else {
(local_14 + param_2) = (local_18 + param_1);
local_18 = local_18 + 0x1;
local_14 = local_14 + 0x1;
}
}
return local_18;
}



fn FUN_004a07e0(param_1: &mut String,param_2: *mut u32) -> u32

{
    let ppcVar1: *mut *mut char;
    let mut local_24: i32;
    let mut local_1c: u32;
    let local_14: u8;

    ppcVar1 = FUN_0049c4bd(param_1,&DAT_004c34f0);
    if (ppcVar1 == 0x0) {
        return 0x1;
    }
    FUN_004aa75c(ppcVar1,-0x301,0x2);
    FUN_004a7970(&local_14,0x1,0x1,ppcVar1);
    FUN_004a7970(param_2,0x300,0x1,ppcVar1);
    FUN_0049ca40(ppcVar1);
    if (local_14 < 0xa) {
        LAB_004a08b4:
            local_1c = 0x1;
    }
    else {
        if (0xa < local_14) {
            if (local_14 != 0xc)^ // goto LAB_004a08b4;
            for (local_24 = 0x0; local_24 < 0x300; local_24 = local_24 + 0x1) {
                param_2 = (*param_2 >> 0x2);
                param_2 = (param_2 + 0x1);
            }
        }
        local_1c = 0x0;
    }
    return local_1c;
}



// WARNING: Removing unreachable block (ram,0x004a0949)
// WARNING: Removing unreachable block (ram,0x004a091c)
// WARNING: Restarted to delay deadcode elimination for space: stack

u32
FUN_004a08c5(param_1: &mut String,param_2: i32,param_3: i32,param_4: i32,param_5: i32,param_6: i32,param_7: i32,param_8: i32,
param_9: i32)

{
let CVar1: u8;
let mut iVar2: i32;
let ppcVar3: *mut *mut char;
let puVar4: *mut u32;
let mut uVar5: u32;
let mut pCVar6: String;;
let mut iVar7: i32;
let mut iVar8: i32;
let mut iVar9: i32;
let mut iVar10: i32;
let mut uVar11: u32;
let mut local_8c: i32;
code *local_88;
let mut local_84: i32;
code *local_80;
let mut local_60: i32;
let mut local_5c: i32;
let mut local_58: u32;
let mut local_38: i32;
let mut local_34: i32;
let mut local_2c: i32;
let mut local_28: u32;
let mut local_20: u32;

local_2c = 0x80;
local_28 = 0x0;
CVar1 = '\0';
ppcVar3 = FUN_0049c4bd(param_1,&DAT_004c34f3);
if (ppcVar3 == 0x0) {
local_58 = 0x1;
}
else {
puVar4 = FUN_0049c2c9(0x2490);
FUN_004a7970(puVar4,0x1,0x2490,ppcVar3);
iVar10 = ((puVar4 + 0x2) - (puVar4 + 0x1)) + 0x1;
uVar11 = ((puVar4 + 0xa) - (puVar4 + 0x6)) + 0x1;
uVar5 = (puVar4 + 0x42);
pCVar6 = FUN_0049c2c9(((puVar4 + 0x2) - (puVar4 + 0x1)) + 0x2);
iVar7 = param_2 + param_4;
iVar8 = param_3 + param_5;
if (param_2 < 0x0) {
param_6 = param_6 - param_2;
param_2 = 0x0;
}
if (param_3 < 0x0) {
param_7 = param_7 - param_3;
param_3 = 0x0;
}
for (; param_6 < 0x0; param_6 = param_6 + iVar10) {
}
for (; param_7 < 0x0; param_7 = param_7 + uVar11) {
}
for (local_20 = 0x0; local_20 < uVar11; local_20 = local_20 + 0x1) {
iVar9 = puVar4 + local_2c;
local_60 = 0x0;
local_5c = 0x0;
while ((local_28 != 0x0 && (local_5c < uVar5))) {
pCVar6[local_5c] = CVar1;
local_28 = local_28 - 0x1;
local_5c = local_5c + 0x1;
}
while (local_5c < uVar5) {
if ((*(iVar9 + local_60) & 0xc0) == 0xc0) {
iVar2 = local_60 + 0x1;
local_28 = (*(local_60 + iVar9) & 0x3f);
local_60 = local_60 + 0x2;
CVar1 = (iVar2 + iVar9);
while ((local_28 != 0x0 && (local_5c < uVar5))) {
pCVar6[local_5c] = CVar1;
local_28 = local_28 - 0x1;
local_5c = local_5c + 0x1;
}
}
else {
pCVar6[local_5c] = (local_60 + iVar9);
local_60 = local_60 + 0x1;
local_5c = local_5c + 0x1;
}
}
local_2c = local_2c + local_60;
if (0x2490 < (local_2c + iVar10 * 0x2)) {
FUN_004a1dc0(puVar4,(puVar4 + local_2c),0x2490 - local_2c);
FUN_004a7970((puVar4 + (0x2490 - local_2c)),0x1,local_2c,ppcVar3);
local_2c = 0x0;
}
for (local_34 = (local_20 + param_3) - param_7; local_34 < iVar8; local_34 = local_34 + uVar11) {
if (param_3 <= local_34) {
for (local_38 = param_2 - param_6; local_38 < iVar7; local_38 = local_38 + iVar10) {
iVar9 = local_38 + iVar10;
if (param_2 < local_38) {
if (param_9 == 0x0) {
local_80 = FUN_00496ac0;
}
else {
local_80 = FUN_00496ee6;
}
local_84 = iVar10;
if (iVar7 <= iVar9) {
local_84 = iVar7 - local_38;
}
(*local_80)(pCVar6,local_38,local_34,local_84,0x1);
}
else {
if (param_2 < iVar9) {
if (param_9 == 0x0) {
local_88 = FUN_00496ac0;
}
else {
local_88 = FUN_00496ee6;
}
if (iVar9 < iVar7) {
local_8c = local_38 + (iVar10 - param_2);
}
else {
local_8c = iVar7 - param_2;
}
(*local_88)(pCVar6 + (param_2 - local_38),param_2,local_34,local_8c,0x1);
}
}
if (param_8 == 0x0) break;
}
}
if (param_8 == 0x0) break;
}
}
FUN_0049ca40(ppcVar3);
FUN_0049af50(puVar4);
FUN_0049af50(pCVar6);
local_58 = 0x0;
}
return local_58;
}



// WARNING: Removing unreachable block (ram,0x004a109e)
// WARNING: Removing unreachable block (ram,0x004a0db3)
// WARNING: Removing unreachable block (ram,0x004a10ee)
// WARNING: Removing unreachable block (ram,0x004a0d86)
// WARNING: Removing unreachable block (ram,0x004a114f)
// WARNING: Removing unreachable block (ram,0x004a1163)

fn FUN_004a0cd8(param_1: &mut String,param_2: i32,param_3: i32) -> u32

{
    ushort uVar1;
    let puVar2: *mut u32;
    let puVar3: *mut u32;
    let mut iVar4: i32;
    byte *pbVar5;
    let mut iVar6: i32;
    let mut iVar7: i32;
    let mut iVar8: i32;
    let mut uVar9: u32;
    let mut local_ac: i32;
    let mut local_a8: i32;
    let mut local_9c: i32;
    let mut local_98: i32;
    let mut local_8c: u32;
    let mut local_60: i32;
    let mut local_5c: u32;
    let mut local_58: u32;
    let mut local_54: u32;
    let mut local_50: u32;
    let mut local_4c: u32;
    let local_48: *mut *mut char;
    let mut local_44: i32;
    let mut local_40: i32;
    let local_3c: *mut u32;
    let mut local_38: u32;
    let local_34: *mut u32;
    let local_30: *mut u32;
    let mut local_2c: i32;
    let mut local_28: i32;
    code *local_24;
    code *local_20;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    local_ac = param_2;
    local_a8 = param_3;
    local_9c = 0x0;
    local_98 = 0x0;
    local_60 = 0x80;
    local_5c = 0x0;
    local_58 = 0x0;
    local_48 = FUN_0049c4bd(param_1,&DAT_004c34f3);
    if (local_48 == 0x0) {
        local_8c = 0x1;
    }
    else {
        local_4c = 0x2710;
        local_50 = 0x2490;
        puVar2 = FUN_0049c2c9(0x2490);
        FUN_004a7970(puVar2,0x1,local_50,local_48);
        iVar8 = ((puVar2 + 0x2) - (puVar2 + 0x1)) + 0x1;
        uVar9 = ((puVar2 + 0xa) - (puVar2 + 0x6)) + 0x1;
        uVar1 = (puVar2 + 0x42);
        puVar3 = FUN_0049c2c9(((puVar2 + 0x2) - (puVar2 + 0x1)) + 0x2);
        iVar4 = param_2 + 0x2710;
        if (param_2 < 0x0) {
            local_9c = -param_2;
            local_ac = 0x0;
        }
        if (param_3 < 0x0) {
            local_98 = -param_3;
            local_a8 = 0x0;
        }
        for (; local_9c < 0x0; local_9c = local_9c + iVar8) {
        }
        for (; local_98 < 0x0; local_98 = local_98 + uVar9) {
        }
        for (local_54 = 0x0; local_54 < uVar9; local_54 = local_54 + 0x1) {
            local_44 = puVar2 + local_60;
            local_34 = &local_5c;
            local_30 = &local_58;
            local_2c = 0x0;
            local_28 = 0x0;
            local_40 = local_44;
            local_3c = puVar3;
            local_38 = uVar1;
            while ((*local_34 != 0x0 && (local_28 < local_38))) {
                (local_28 + local_3c) = local_30;
                local_28 = local_28 + 0x1;
                *local_34 = *local_34 - 0x1;
            }
            while (local_28 < local_38) {
                if ((*(local_40 + local_2c) & 0xc0) == 0xc0) {
                    pbVar5 = (local_2c + local_40);
                    local_2c = local_2c + 0x1;
                    *local_34 = (*pbVar5 & 0x3f);
                    pbVar5 = (local_2c + local_40);
                    local_2c = local_2c + 0x1;
                    *local_30 = *pbVar5;
                    while ((*local_34 != 0x0 && (local_28 < local_38))) {
                        (local_28 + local_3c) = local_30;
                        local_28 = local_28 + 0x1;
                        *local_34 = *local_34 - 0x1;
                    }
                }
                else {
                    (local_28 + local_3c) = (local_2c + local_40);
                    local_2c = local_2c + 0x1;
                    local_28 = local_28 + 0x1;
                }
            }
            local_60 = local_60 + local_2c;
            if (local_50 < (local_60 + iVar8 * 0x2)) {
                FUN_004a1dc0(puVar2,(puVar2 + local_60),local_50 - local_60);
                FUN_004a7970((puVar2 + (local_50 - local_60)),0x1,local_60,local_48);
                local_60 = 0x0;
            }
            iVar6 = (local_54 + local_a8) - local_98;
            if (((iVar6 < param_3 + 0x2710) && (local_a8 <= iVar6)) && (iVar7 = local_ac - local_9c, iVar7 < iVar4)) {
                local_18 = iVar7 + iVar8;
                if (local_ac < iVar7) {
                    local_24 = FUN_00496ac0;
                    local_1c = iVar8;
                    if (iVar4 <= local_18) {
                        local_1c = iVar4 - iVar7;
                    }
                    FUN_00496ac0(puVar3,iVar7,iVar6,local_1c,0x1);
                }
                else {
                    if (local_ac < local_18) {
                        local_20 = FUN_00496ac0;
                        if (local_18 < iVar4) {
                            local_14 = iVar7 + (iVar8 - local_ac);
                        }
                        else {
                            local_14 = iVar4 - local_ac;
                        }
                        FUN_00496ac0((puVar3 + (local_ac - iVar7)),local_ac,iVar6,local_14,0x1);
                    }
                }
            }
        }
        FUN_0049ca40(local_48);
        FUN_0049af50(puVar2);
        FUN_0049af50(puVar3);
        local_8c = 0x0;
    }
    return local_8c;
}



fn FUN_004a11c0(param_1: i32) -> i32

{
if ((0x60 < param_1) && (param_1 < 0x7b)) {
param_1 = param_1 + -0x20;
}
return param_1;
}



fn FUN_004a11e2()

{
    // WARNING: Subroutine does not return
    FUN_004b0ff3(0x20);
}



fn FUN_004a1243()

{
    // WARNING: Subroutine does not return
    FUN_004b0ff3(0x20);
}



bool FUN_004a12c7()

{
// WARNING: Subroutine does not return
FUN_004b0ff3(0x24);
}



fn FUN_004a15ad()

{
    // WARNING: Subroutine does not return
    FUN_004b0ff3(0x24);
}



fn FUN_004a1651() -> u32

{
    // WARNING: Subroutine does not return
    FUN_004b0ff3(0xa4);
}



fn FUN_004a1c7f()

{
    // WARNING: Subroutine does not return
    FUN_004b0ff3(0x24);
}



fn FUN_004a1dc0(param_1: *mut u32,param_2: *mut u32,param_3: u32)

{
    let mut uVar1: u32;
    let puVar2: *mut u32;
    undefined2 *puVar3;

    if (param_2 != param_1) {
        if ((param_2 < param_1) && (param_1 < (param_2 + param_3))) {
            uVar1 = param_3 >> 0x1;
            puVar2 = (param_2 + param_3);
            puVar3 = (param_1 + param_3);
            while( true ) {
                if (uVar1 == 0x0) break;
                uVar1 = uVar1 - 0x1;
                puVar3[-0x1] = *(puVar2 - 0x2U);
                puVar2 = (puVar2 - 0x2U);
                puVar3 = puVar3 + -0x1;
            }
            uVar1 = ((param_3 & 0x1) != 0x0);
            while( true ) {
                puVar3 = (puVar3 + -0x1);
                puVar2 = (puVar2 - 0x1);
                if (uVar1 == 0x0) break;
                uVar1 = uVar1 - 0x1;
                puVar3 = puVar2;
            }
            return;
        }
        for (uVar1 = param_3 >> 0x2; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
            *param_1 = *param_2;
            param_2 = param_2 + 0x1;
            param_1 = param_1 + 0x1;
        }
        for (uVar1 = param_3 & 0x3; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
            param_1 = param_2;
            param_2 = (param_2 + 0x1);
            param_1 = (param_1 + 0x1);
        }
    }
    return;
}



fn FUN_004a1e20(byte *param_1,byte *param_2,param_3: i32) -> i32

{
if (param_3 != 0x0) {
loop {
if (*param_1 != *param_2) {
return *param_1 - *param_2;
}
if (*param_1 == 0x0) {
return 0x0;
}
param_1 = param_1 + 0x1;
param_2 = param_2 + 0x1;
param_3 = param_3 + -0x1;
} while (param_3 != 0x0);
}
return 0x0;
}



fn FUN_004a1e60(byte *param_1) -> i32

{
let bVar1: u8;
let bVar2: u8;
let mut iVar3: i32;

while (((&DAT_004bf9c4)[(byte)(*param_1 + 0x1)] & 0x2) != 0x0) {
param_1 = param_1 + 0x1;
}
bVar1 = *param_1;
if ((bVar1 == 0x2b) || (bVar1 == 0x2d)) {
param_1 = param_1 + 0x1;
}
iVar3 = 0x0;
while (((&DAT_004bf9c4)[(byte)(*param_1 + 0x1)] & 0x20) != 0x0) {
bVar2 = *param_1;
param_1 = param_1 + 0x1;
iVar3 = iVar3 * 0xa + bVar2 + -0x30;
}
if (bVar1 == 0x2d) {
iVar3 = -iVar3;
}
return iVar3;
}



fn FUN_004a27a0(param_1: i32)

{
    let mut uVar1: u32;
    let uVar2: u8;
    let mut uVar3: u32;
    let uVar4: u8;
    let uVar5: u8;

    FUN_004953d7();
    uVar1 = DAT_004bf924;
    uVar3 = DAT_005b981c;
    if ((*(param_1 + 0x4e) & 0x10) != 0x0) {
        uVar1 = DAT_005b981c;
        uVar3 = DAT_004bf924;
    }
    uVar4 = uVar1;
    uVar5 = uVar3;
    FUN_0049e640(*(param_1 + 0x1d),*(param_1 + 0x21),*(param_1 + 0x25),*(param_1 + 0x29),uVar4
                 ,uVar5,DAT_004bf928,0x1);
    if ((*(param_1 + 0x2d) & 0x4) == 0x0) {
        uVar2 = uVar5;
        if ((*(param_1 + 0x2e) & 0x80) != 0x0) {
            uVar2 = uVar4;
            uVar4 = uVar5;
        }
        FUN_0049e640(*(param_1 + 0x1d),*(param_1 + 0x21),*(param_1 + 0x25),*(param_1 + 0x29),
                     uVar2,uVar4,DAT_004bf928,0x2);
    }
    FUN_0049536f();
    return;
}



fn FUN_004a2831(param_1: u32) -> String

{
    let mut pCVar1: String;;
    let mut iVar2: i32;

    if (param_1 == 0x0) {
        param_1 = 0x1;
    }
    while( true ) {
        loop {
            pCVar1 = FUN_004aac00(param_1);
            if (pCVar1 != 0x0) {
                return pCVar1;
            }
        } while ((DAT_005ba46c != (code *)0x0) && (iVar2 = (*DAT_005ba46c)(param_1), iVar2 != 0x0));
        if (DAT_005ba468 == (code *)0x0) break;
        (*DAT_005ba468)();
    }
    return 0x0;
}



i32 **  FUN_004a2874(i32 **param_1,param_2: *mut i32,param_3: *mut i32)

{
let piVar1: *mut i32;;
i32 **local_14;

*param_1 = 0x0;
param_1[0x1] = param_3;
piVar1 = timeGetTime();
param_1[0x2] = piVar1;
param_1[0x3] = param_2;
if (DAT_005b9c50 == 0x0) {
DAT_005b9c50 = param_1;
}
else {
for (local_14 = DAT_005b9c50; *local_14 != 0x0; local_14 = *local_14) {
}
*local_14 = param_1;
}
return param_1;
}



fn FUN_004a28ef(param_1: *mut u32) -> *mut u32

{
    i32 **local_18;

    if (param_1 == DAT_005b9c54) {
        DAT_005b9c54 = 0x0;
    }
    if (param_1 == DAT_005b9c50) {
    DAT_005b9c50 = *param_1;
}
    else {
    for (local_18 = DAT_005b9c50; local_18 != 0x0; local_18 = *local_18) {
        if (*local_18 == param_1) {
            *local_18 = *param_1;
            return param_1;
        }
    }
}
    return param_1;
}



fn FUN_004a2965(param_1: i32)

{
    let puVar1: *mut u32;
    let local_1c: *mut u32;
    let local_18: *mut u32;

    local_18 = DAT_005b9c50;
    puVar1 = local_18;
    loop {
    loop {
        local_18 = puVar1;
        if (local_18 == 0x0) {
            return;
        }
        puVar1 = *local_18;
    } while ((local_18[0x3] != param_1) || (local_18 == 0x0));
    if (local_18 == DAT_005b9c54) {
        DAT_005b9c54 = 0x0;
    }
    if (local_18 == DAT_005b9c50) {
        DAT_005b9c50 = *local_18;
    }
    else {
        for (local_1c = DAT_005b9c50; local_1c != 0x0; local_1c = *local_1c) {
            if (*local_1c == local_18) {
                *local_1c = *local_18;
                break;
            }
        }
    }
    FUN_0049af50(local_18);
} while( true );
}



fn FUN_004a2a3b(param_1: *mut u32,param_2: *mut i32,param_3: *mut u32) -> u32

{
    let DVar1: u32;
    let mut iVar2: i32;
    let local_1c: *mut u32;

    DVar1 = timeGetTime();
    if (DAT_005b9c54 == 0x0) {
        local_1c = DAT_005b9c50;
    }
    else {
        local_1c = *DAT_005b9c54;
    }
    while( true ) {
        if (local_1c == 0x0) {
            return 0x0;
        }
        iVar2 = DVar1 - local_1c[0x2];
        if ((local_1c[0x1] <= iVar2) || (iVar2 < 0x0)) break;
        local_1c = *local_1c;
    }
    *param_1 = local_1c[0x3];
    *param_2 = iVar2;
    *param_3 = local_1c[0x1];
    local_1c[0x2] = DVar1;
    return 0x1;
}



ulonglong  FUN_004a2ae0(param_1: u32,param_2: u32,param_3: *mut u32,param_4: *mut u32)

{
let bVar1: u8;
let bVar3: u8;
let mut uVar4: u32;
let mut bVar5: bool;
let mut uVar2: u32;

if (param_3 != param_4) {
loop {
uVar2 = *param_3;
uVar4 = *param_4;
if (uVar4 != uVar2) {
// LAB_004a2b63:
bVar1 = (byte)uVar2;
bVar5 = bVar1 < (byte)uVar4;
if (bVar1 == (byte)uVar4) {
if (bVar1 == 0x0) break;
bVar1 = (byte)(uVar2 >> 0x8);
bVar3 = (byte)(uVar4 >> 0x8);
bVar5 = bVar1 < bVar3;
if (bVar1 == bVar3) {
if (bVar1 == 0x0) break;
bVar1 = (byte)(uVar2 >> 0x10);
bVar3 = (byte)(uVar4 >> 0x10);
bVar5 = bVar1 < bVar3;
if (bVar1 == bVar3) {
if (bVar1 == 0x0) break;
bVar5 = (byte)(uVar2 >> 0x18) < (byte)(uVar4 >> 0x18);
}
}
}
return CONCAT44(param_2,-bVar5) | 0x1;
}
if ((uVar2 + 0xfefefeff & ~uVar4 & 0x80808080) != 0x0) break;
uVar2 = param_3[0x1];
uVar4 = param_4[0x1];
if (uVar4 != uVar2)^ // goto LAB_004a2b63;
if ((uVar2 + 0xfefefeff & ~uVar4 & 0x80808080) != 0x0) break;
uVar2 = param_3[0x2];
uVar4 = param_4[0x2];
if (uVar4 != uVar2)^ // goto LAB_004a2b63;
if ((uVar2 + 0xfefefeff & ~uVar4 & 0x80808080) != 0x0) break;
uVar2 = param_3[0x3];
uVar4 = param_4[0x3];
if (uVar4 != uVar2)^ // goto LAB_004a2b63;
param_3 = param_3 + 0x4;
param_4 = param_4 + 0x4;
} while ((uVar2 + 0xfefefeff & ~uVar4 & 0x80808080) == 0x0);
}
return (ulonglong)param_2 << 0x20;
}



fn FUN_004a2cab(LPSTR param_1,dword param_2,DWORD param_3,DWORD param_4) -> u32

{
    // WARNING: Subroutine does not return
    FUN_004b0ff3(0x2c);
}



fn FUN_004a2d6b() -> u32

{
    // WARNING: Subroutine does not return
    FUN_004b0ff3(0x2c);
}



fn FUN_004a2ed0() -> i32

{
let mut iVar1: i32;

iVar1 = (PTR_FUN_004bfb74)();
return iVar1 + 0xc;
}



fn FUN_004a2edc() -> u32

{
    let puVar1: *mut u32;
    let mut uVar2: u32;

    puVar1 = FUN_004a2ed0();
    if (puVar1 == 0x0) {
        return 0x0;
    }
    uVar2 = *puVar1 * 0x41c64e6d + 0x3039;
    *puVar1 = uVar2;
    return uVar2 >> 0x10 & 0x7fff;
}



fn FUN_004a2f00(param_1: u32)

{
    let puVar1: *mut u32;

    puVar1 = FUN_004a2ed0();
    if (puVar1 != 0x0) {
        *puVar1 = param_1;
    }
    return;
}



fn FUN_004a2f10(byte *param_1,byte *param_2) -> i32

{
let bVar1: u8;
let bVar2: u8;

while( true ) {
bVar1 = *param_1;
bVar2 = *param_2;
if ((0x40 < bVar1) && (bVar1 < 0x5b)) {
bVar1 = bVar1 + 0x20;
}
if ((0x40 < bVar2) && (bVar2 < 0x5b)) {
bVar2 = bVar2 + 0x20;
}
if ((bVar1 != bVar2) || (bVar2 == 0x0)) break;
param_1 = param_1 + 0x1;
param_2 = param_2 + 0x1;
}
return bVar1 - bVar2;
}



// WARNING: Could not reconcile some variable overlaps

char *  FUN_004a2f60(param_1: &mut String,param_2: i32,byte **param_3)

{
byte *pbVar1;
let mut pcVar2: String;
let mut pcVar3: String;
let mut uStack24: u32;

(PTR_FUN_004bfb78)(param_3[0x4]);
pbVar1 = param_3[0x3];
*(param_3 + 0x3) = *(param_3 + 0x3) & 0xcf;
pcVar2 = param_1;
loop {
param_2 = param_2 + -0x1;
pcVar3 = pcVar2;
if (param_2 < 0x1) break;
uStack24 = FUN_004a9250(param_3);
if (uStack24 == 0xffffffff) break;
pcVar3 = pcVar2 + 0x1;
*pcVar2 = uStack24;
pcVar2 = pcVar3;
} while (uStack24 != '\n');
if ((uStack24 == 0xffffffff) && ((pcVar3 == param_1 || ((*(param_3 + 0x3) & 0x20) != 0x0)))) {
param_1 = 0x0;
}
else {
*pcVar3 = '\0';
}
param_3[0x3] = (param_3[0x3] | pbVar1 & 0x30);
(PTR_FUN_004bfb7c)(param_3[0x4]);
return param_1;
}



// WARNING: Removing unreachable block (ram,0x004a3671)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_004a2ff0(param_1: u32,param_2: &mut String,param_3: i32,param_4: i32) -> u32

{
    let piVar1: *mut i32;;
    let mut bVar2: bool;
    let mut bVar3: bool;
    let mut iVar4: i32;
    let mut pcVar5: String;
    let cVar6: u8;
    undefined3 extraout_var;
    let mut iVar7: i32;
    let mut pcVar8: String;
    let mut uVar9: u32;
    let mut iVar10: i32;
    let ppcVar11: *mut *mut char;
    let mut iVar12: i32;
    let mut iVar13: i32;
    let local_ac: u8 [0x38];
    let mut local_74: String;
    let aCStack112: u8 [0x2c];
    let aCStack68: u8 [0xc];
    let mut local_38: String;
    let mut local_34: *mut u8;
    let mut local_30: i32;
    let local_2c: *mut *mut char;
    ushort *local_28;
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: String;

    FUN_004a2d6b();
    local_34 = &stack0xffffff50;
    ppcVar11 = &stack0xffffff50;
    DAT_005b9c68 = -0x1;
    DAT_005b9c64 = param_3;
    local_24 = (DAT_005b9228 * 0x55) / 0x64;
    iVar13 = 0x0;
    iVar12 = 0x0;
    local_14 = param_2;
    local_38 = 0x0;
    local_20 = 0x0;
    local_30 = 0x0;
    local_18 = 0x0;
    local_28 = LPCSTR_005b9218;
    local_1c = 0x0;
    pcVar8 = param_2;
    if (*param_2 != '\0') {
        local_2c = &local_74;
        iVar10 = iVar12;
        local_34 = &stack0xffffff50;
        loop {
            cVar6 = FUN_004a06b1(*pcVar8,local_28);
            pcVar5 = local_14;
            iVar12 = CONCAT31(extraout_var,cVar6) + iVar10;
            if (((*pcVar8 == '\r') || (*pcVar8 == '\n')) || (local_24 < iVar12)) {
                cVar6 = *pcVar8;
                if (((cVar6 != '\r') && (cVar6 != '\n')) && ((cVar6 != ' ' && (local_38 != 0x0)))) {
                    pcVar8 = local_38;
                    iVar10 = local_30;
                    iVar13 = local_18;
                }
                *ppcVar11 = local_14;
                iVar12 = local_1c;
                (ppcVar11 + 0x1) = (pcVar8 - pcVar5) - iVar13;
                if (iVar12 < iVar10) {
                    local_1c = iVar10;
                }
                bVar3 = false;
                bVar2 = false;
                while( true ) {
                    while( true ) {
                        for (; cVar6 = *pcVar8, cVar6 == '\r'; pcVar8 = pcVar8 + 0x1) {
                            if (bVar2)^ // goto LAB_004a313c;
                            bVar2 = true;
                        }
                        if (cVar6 == '\n') break;
                        if (cVar6 != ' ')^ // goto LAB_004a313c;
                        pcVar8 = pcVar8 + 0x1;
                    }
                    if (bVar3) break;
                    bVar3 = true;
                    pcVar8 = pcVar8 + 0x1;
                }
                LAB_004a313c:
                    local_20 = local_20 + 0x1;
                ppcVar11 = (ppcVar11 + 0x6);
                if (ppcVar11 == local_2c)^ // goto LAB_004a315d;
                iVar12 = 0x0;
                iVar13 = 0x0;
                local_38 = 0x0;
                local_30 = 0x0;
                local_18 = 0x0;
                local_14 = pcVar8;
            }
            else {
                if (*pcVar8 == '~') {
                    iVar13 = iVar13 + 0x1;
                    iVar12 = iVar10;
                }
                else {
                    if (*pcVar8 == ' ') {
                        local_38 = pcVar8 + 0x1;
                        local_30 = iVar12;
                        local_18 = iVar13;
                    }
                }
                pcVar8 = pcVar8 + 0x1;
            }
            iVar10 = iVar12;
        } while (*pcVar8 != '\0');
    }
    if (pcVar8 != local_14) {
        *(local_34 + local_20 * 0x6) = local_14;
        ((local_34 + local_20 * 0x6) + 0x4) = (pcVar8 - local_14) - iVar13;
        if (local_1c < iVar12) {
            local_1c = iVar12;
        }
        local_20 = local_20 + 0x1;
    }
    LAB_004a315d:
        iVar12 = local_20;
    if (local_20 == 0x0) {
        pop_err_msg_box_and_exit_004a02f5((LPSTR)0x0);
    }
    if (param_4 != 0x0) {
        iVar12 = iVar12 + 0x2;
        if (local_18 < DAT_005b9228 / 0x2) {
            local_18 = DAT_005b9228 / 0x2;
        }
        DAT_005b9c80 = local_18;
    }
    DAT_005b9c74 = local_18 + 0x40;
    DAT_005b9c6c = (DAT_005b9228 - DAT_005b9c74) / 0x2;
    DAT_005b9c78 = (iVar12 + 0x2) * DAT_005b9210;
    iVar13 = (DAT_005b922c - DAT_005b9c78) / 0x2;
    DAT_005b9c70 = iVar13;
    FUN_004953d7();
    if (_DAT_005b96c4 != (code *)0x0) {
    (*_DAT_005b96c4)(&DAT_005b9c6c,&DAT_005b9c70,&DAT_005b9c74);
}
    if (param_2 == 0x0) {
        iVar10 = 0x1;
    }
    else {
        iVar10 = (*(param_2 + 0x41) + 0x1) % 0x2;
    }
    iVar7 = FUN_0049a9c0();
    if (iVar7 == 0x0) {
        iVar10 = 0x0;
    }
    if (DAT_005b9824 == 0x0) {
        FUN_004968e7(DAT_005b9c6c,DAT_005b9c70,DAT_005b9c74,DAT_005b9c78,DAT_004bf948);
    }
    else {
        FUN_0049c2e0(aCStack68,s__s_d_pcx_004c356d);
        FUN_004a08c5(aCStack68,DAT_005b9c6c,DAT_005b9c70,DAT_005b9c74,DAT_005b9c78,0x0,0x0,0x1,0x0);
    }
    if (_DAT_005b96c0 == (code *)0x0) {
    FUN_0049e640(DAT_005b9c6c + 0x1,DAT_005b9c70 + 0x1,DAT_005b9c74 + -0x2,DAT_005b9c78 + -0x2,DAT_004bf924,
                 DAT_005b981c,DAT_004bf928,0x1);
}
    else {
    (*_DAT_005b96c0)(DAT_005b9c6c,DAT_005b9c70,DAT_005b9c6c + DAT_005b9c74,DAT_005b9c70 + DAT_005b9c78);
}
    iVar13 = iVar13 / DAT_005b9210;
    if (param_4 != 0x0) {
        DAT_005b9c7c = iVar13 + iVar12;
        iVar12 = iVar12 + -0x2;
    }
    if (0x0 < iVar12) {
        iVar7 = 0x0;
        loop {
            iVar13 = iVar13 + 0x1;
            iVar4 = iVar7 + 0x2;
            piVar1 = (local_ac + iVar7);
            iVar7 = iVar7 + 0x6;
            FUN_00497567(DAT_005b9228 / 0x2,DAT_005b9210 * iVar13,*piVar1,*(local_ac + iVar4) >> 0x10,
                         (&DAT_005b9b74)[iVar10],-0x1,(&DAT_005b9b74)[iVar10],LPCSTR_005b9218,0x1);
        } while (SBORROW4(iVar7,iVar12 * 0x6) != iVar7 + iVar12 * -0x6 < 0x0);
    }
    if (param_4 != 0x0) {
        FUN_0049e640(DAT_005b9228 - DAT_005b9c80 >> 0x1,DAT_005b9c7c + 0x4e20,DAT_005b9c80,0x4e21,DAT_005b981c,
                     DAT_004bf924,DAT_004bf928,0x1);
        win_handle_func_0049fb83(DAT_005b9bd4);
        FUN_004ae8ec();
        if ((DAT_005b9c64 != 0x0) && (DAT_005b9c68 != 0x0)) {
            DAT_005b9c68 = 0x0;
            pcVar8 = FUN_00499050(DAT_005b9bd8,0x791e);
            FUN_0049c2e0(aCStack112,pcVar8);
            FUN_004953d7();
            if (DAT_005b9c80 != 0x0) {
                FUN_004968e7(DAT_005b9228 - DAT_005b9c80 >> 0x1,DAT_005b9c7c * DAT_005b9210,DAT_005b9c80,DAT_005b9210,
                             DAT_004bf95c);
            }
            uVar9 = 0xffffffff;
            pcVar8 = aCStack112;
            loop {
                if (uVar9 == 0x0) break;
                uVar9 = uVar9 - 0x1;
                cVar6 = *pcVar8;
                pcVar8 = pcVar8 + 0x1;
            } while (cVar6 != '\0');
            FUN_00497567(DAT_005b9228 >> 0x1,DAT_005b9c7c * DAT_005b9210,aCStack112,~uVar9 - 0x1,DAT_005b9c84,-0x1,
                         DAT_005b9c84,LPCSTR_005b9218,0x1);
            FUN_0049536f();
        }
    }
    FUN_0049536f();
    return 0x1;
}



fn FUN_004a36b0(param_1: i32)

{
    let cVar1: u8;
    let mut uVar2: u32;
    let mut pcVar3: String;
    let mut iVar4: i32;
    let local_3c: u8 [0x2c];

    win_handle_func_0049fb83(DAT_005b9bd4);
    FUN_004ae8ec();
    if ((DAT_005b9c64 == 0x0) || (uVar2 = (param_1 * 0x64) / DAT_005b9c64, uVar2 == DAT_005b9c68)) {
        return;
    }
    DAT_005b9c68 = uVar2;
    pcVar3 = FUN_00499050(DAT_005b9bd8,0x791e);
    FUN_0049c2e0(local_3c,pcVar3);
    uVar2 = (DAT_005b9c80 * uVar2) / 0x64;
    FUN_004953d7();
    iVar4 = DAT_005b9228 - DAT_005b9c80 >> 0x1;
    if (uVar2 != 0x0) {
        FUN_004968e7(iVar4,DAT_005b9c7c * DAT_005b9210,uVar2,DAT_005b9210,DAT_004bf960);
    }
    if (DAT_005b9c80 - uVar2 != 0x0) {
        FUN_004968e7(uVar2 + iVar4,DAT_005b9c7c * DAT_005b9210,DAT_005b9c80 - uVar2,DAT_005b9210,DAT_004bf95c);
    }
    uVar2 = 0xffffffff;
    pcVar3 = local_3c;
    loop {
    if (uVar2 == 0x0) break;
    uVar2 = uVar2 - 0x1;
    cVar1 = *pcVar3;
    pcVar3 = pcVar3 + 0x1;
} while (cVar1 != '\0');
    FUN_00497567(DAT_005b9228 >> 0x1,DAT_005b9c7c * DAT_005b9210,local_3c,~uVar2 - 0x1,DAT_005b9c84,-0x1,DAT_005b9c84
                 ,LPCSTR_005b9218,0x1);
    FUN_0049536f();
    return;
}



fn FUN_004a3800()

{
    let mut local_10: i32;
    let mut local_c: u32;
    let mut local_8: u32;
    let mut local_4: u32;

    local_10 = DAT_005b9c6c;
    local_c = DAT_005b9c70;
    local_8 = DAT_005b9c74;
    local_4 = DAT_005b9c78;
    FUN_0049ad10(&local_10);
    return;
}



fn FUN_004a3840(param_1: &mut String,char **param_2,param_3: i32,param_4: i32,param_5: *mut i32,ushort *param_6,param_7: i32) -> i32

{
let mut bVar1: bool;
let mut bVar2: bool;
let sVar3: i16;
let cVar4: u8;
undefined3 extraout_var;
let ppcVar5: *mut *mut char;
let mut pcVar6: String;
let mut pcVar7: String;
let sVar8: i16;
let mut iVar9: i32;
let mut iVar10: i32;
let mut local_24: i32;
let local_20: *mut *mut char;
let mut local_14: i32;

pcVar7 = 0x0;
sVar8 = 0x0;
iVar9 = 0x0;
local_24 = 0x0;
*param_5 = 0x0;
local_14 = 0x0;
sVar3 = 0x0;
pcVar6 = param_1;
if (*param_1 == '\0') {
// LAB_004a3941:
if (pcVar6 != param_1) {
ppcVar5 = (local_24 * 0x6 + param_2);
*ppcVar5 = param_1;
(ppcVar5 + 0x1) = (pcVar6 - param_1) - sVar8;
if (*param_5 < iVar9) {
*param_5 = iVar9;
}
local_24 = local_24 + 0x1;
}
return local_24;
}
local_20 = param_2;
iVar10 = iVar9;
sVar8 = 0x0;
// LAB_004a388d:
cVar4 = FUN_004a06b1(*pcVar6,param_6);
iVar9 = CONCAT31(extraout_var,cVar4) + iVar10;
if (((*pcVar6 == '\r') || (*pcVar6 == '\n')) || (param_3 < iVar9)) {
cVar4 = *pcVar6;
if ((((cVar4 != '\r') && (cVar4 != '\n')) && ((cVar4 != ' ' || (param_7 == 0x1)))) && (pcVar7 != 0x0)) {
pcVar6 = pcVar7;
iVar10 = local_14;
sVar8 = sVar3;
}
*local_20 = param_1;
(local_20 + 0x1) = (pcVar6 - param_1) - sVar8;
if (*param_5 < iVar10) {
*param_5 = iVar10;
}
bVar2 = false;
bVar1 = false;
loop {
cVar4 = *pcVar6;
if (cVar4 == '\r') {
if (bVar1)^ // goto LAB_004a390c;
bVar2 = false;
bVar1 = true;
}
else {
if (cVar4 == '\n') {
if (bVar2)^ // goto LAB_004a390c;
bVar2 = true;
}
else {
if ((cVar4 != ' ') || (param_7 == 0x1))^ // goto LAB_004a390c;
}
}
pcVar6 = pcVar6 + 0x1;
} while( true );
}
if (*pcVar6 == '~') {
sVar8 = sVar8 + 0x1;
iVar9 = iVar10;
// LAB_004a39a0:
pcVar6 = pcVar6 + 0x1;
}
else {
if (*pcVar6 != ' ')^ // goto LAB_004a39a0;
pcVar7 = pcVar6 + 0x1;
pcVar6 = pcVar6 + 0x1;
local_14 = iVar9;
sVar3 = sVar8;
}^
// goto LAB_004a3938;
// LAB_004a390c:
local_20 = (local_20 + 0x6);
local_24 = local_24 + 0x1;
if (local_20 == (param_2 + param_4 * 0x6)) {
return local_24;
}
pcVar7 = 0x0;
iVar9 = 0x0;
sVar8 = 0x0;
local_14 = 0x0;
param_1 = pcVar6;
sVar3 = 0x0;
// LAB_004a3938:
iVar10 = iVar9;
if (*pcVar6 == '\0')^ // goto LAB_004a3941;^
// goto LAB_004a388d;
}



fn find_file_fn_004a3a00(LPCSTR file_name,HANDLE param_2,HANDLE *param_3) -> u32

{
    HANDLE find_file_handle;
    let uVar1: u32;
    let DVar1: u32;
    _WIN32_FIND_DATAA find_file_data;

    // LPWIN32_FIND_DATAA lpFindFileData for FindFirstFileA
    // LPCSTR lpFileName for FindFirstFileA
    find_file_handle = FindFirstFileA(file_name,(LPWIN32_FIND_DATAA)&find_file_data);
    if (find_file_handle == (HANDLE)0xffffffff) {
    *param_3 = (HANDLE)0xffffffff;
    uVar1 = handle_err_fn_004b12fc();
}
    else {
    if (param_2 == (HANDLE)0x0) {
        param_2 = (HANDLE)0xffffffe7;
    }
    DVar1 = find_file_fn_004b13bc(find_file_handle,param_2,(LPWIN32_FIND_DATAA)&find_file_data);
    if (DVar1 == 0x0) {
        // HANDLE hFindFile for FindClose
        *param_3 = (HANDLE)0xffffffff;
        FindClose(find_file_handle);
        DVar1 = FUN_004b1290(0x2);
        return DVar1;
    }
    *param_3 = find_file_handle;
    param_3[0x1] = param_2;
    FUN_004b1374(param_3,&find_file_data);
    uVar1 = 0x0;
}
    return uVar1;
}



fn find_file_fn_004a3a94(HANDLE *find_file_handle) -> u32

{
    let mut success: bool;
    let DVar1: u32;
    _WIN32_FIND_DATAA find_file_data;

    // LPWIN32_FIND_DATAA lpFindFileData for FindNextFileA
    // HANDLE hFindFile for FindNextFileA
    success = FindNextFileA(*find_file_handle,(LPWIN32_FIND_DATAA)&find_file_data);
    if (success == 0x0) {
        DVar1 = handle_err_fn_004b12fc();
    }
    else {
        DVar1 = find_file_fn_004b13bc(*find_file_handle,find_file_handle[0x1],(LPWIN32_FIND_DATAA)&find_file_data);
        if (DVar1 == 0x0) {
            DVar1 = handle_err_fn_004b12fc();
            return DVar1;
        }
        FUN_004b1374(find_file_handle,&find_file_data);
        DVar1 = 0x0;
    }
    return DVar1;
}



void  create_proc_004a3b16(LPCSTR current_dir,LPCSTR app_name,LPSTR command_line)

{
let mut bVar1: bool;
undefined3 extraout_var;
let mut BVar2: bool;
let exit_code: u32;
STARTUPINFOA local_68;
_PROCESS_INFORMATION local_24;
let mut local_14: i32;

local_68.cb = 0x44;
local_68.lpReserved = (LPSTR)0x0;
local_68.lpDesktop = (LPSTR)0x0;
local_68.lpTitle = (LPSTR)0x0;
local_68.dwFlags = 0x81;
local_68._48_4_ = 0x3;
local_68.lpReserved2 = (LPBYTE)0x0;
FUN_00495cd1();
bVar1 = FUN_004a12c7();
local_14 = CONCAT31(extraout_var,bVar1);
// LPPROCESS_INFORMATION lpProcessInformation for CreateProcessA
// LPSTARTUPINFOA lpStartupInfo for CreateProcessA
// LPCSTR lpCurrentDirectory for CreateProcessA
// LPVOID lpEnvironment for CreateProcessA
// DWORD dwCreationFlags for CreateProcessA
// BOOL bInheritHandles for CreateProcessA
// LPSECURITY_ATTRIBUTES lpThreadAttributes for CreateProcessA
// LPSECURITY_ATTRIBUTES lpProcessAttributes for CreateProcessA
// LPSTR lpCommandLine for CreateProcessA
// LPCSTR lpApplicationName for CreateProcessA
BVar2 = CreateProcessA(app_name,command_line,(LPSECURITY_ATTRIBUTES)0x0,(LPSECURITY_ATTRIBUTES)0x0,0x0,0x0,(LPVOID)0x0
,current_dir,(LPSTARTUPINFOA)&local_68,(LPPROCESS_INFORMATION)&local_24);
if (BVar2 != 0x0) {
loop {
// DWORD dwMilliseconds for Sleep
Sleep(0xa);
// LPDWORD lpExitCode for GetExitCodeProcess
// HANDLE hProcess for GetExitCodeProcess
BVar2 = GetExitCodeProcess(local_24.hProcess,&exit_code);
if (BVar2 == 0x0) break;
} while (exit_code == 0x103);
}
// HWND hWnd for SetForegroundWindow
SetForegroundWindow(hWnd_005b9be0);
if (local_14 != 0x0) {
FUN_004a1243();
}
direct_draw_create_00495a72(hWnd_005b9be0,0x0,0x0,0x0);
FUN_0049f610();
// DWORD dwMilliseconds for Sleep
Sleep(0xa);
return;
}



fn FUN_004a3c10(param_1: *mut i32) -> i32

{
WORD WVar1;
let extraout_var: u16;
let mut iVar2: i32;
u32 local_28 [0x9];

WVar1 = get_time_info_004b1400(local_28);
if (0x1f3 < CONCAT22(extraout_var,WVar1)) {
local_28[0] = local_28[0] + 0x1;
}
iVar2 = FUN_004b1470(local_28);
if (param_1 != 0x0) {
*param_1 = iVar2;
}
return iVar2;
}



fn FUN_004a3c48(param_1: &mut String,param_2: &mut String,param_3: u32)

{
    let mut pcVar1: String;
    let bVar2: u8;
    let bVar3: u8;
    let bVar4: u8;
    let bVar5: u8;
    let cVar6: u8;
    ushort uVar7;
    let cVar8: u8;
    let mut uVar9: u32;
    let mut local_10: String;
    let mut local_c: u32;
    let local_8: u8;
    let local_7: u8;
    let local_6: u8;
    let local_5: u8;

    local_10 = param_1 + 0x3;
    *param_2 = '\0';
    cVar6 = -0x1;
    loop {
    param_2 = param_2 + 0x1;
    local_c = 0xffffffff;
    cVar8 = param_3;
    if ((param_3 & 0x80) == 0x0) {
        local_5 = *local_10 + ((('?' - *local_10) * cVar8) >> 0x7);
        local_6 = local_10[0x1] + ((('?' - local_10[0x1]) * cVar8) >> 0x7);
        local_7 = local_10[0x2] + ((('?' - local_10[0x2]) * cVar8) >> 0x7);
        uVar9 = param_3;
    }
    else {
        cVar8 = cVar8 + -0x80;
        uVar9 = param_3 & 0xffffff00;
        local_5 = ((ushort)(*local_10 * cVar8) >> 0x7);
        local_6 = ((ushort)(local_10[0x1] * cVar8) >> 0x7);
        local_7 = ((ushort)(local_10[0x2] * cVar8) >> 0x7);
    }
    local_10 = local_10 + 0x3;
    uVar7 = CONCAT11(0x1,cVar6);
    pcVar1 = param_1;
    loop {
        bVar2 = pcVar1[0x3] - local_5;
        bVar3 = (byte)((ushort)bVar2 >> 0x8);
        bVar4 = (byte)((ushort)(pcVar1[0x4] - local_6) >> 0x8);
        bVar5 = (byte)((ushort)(pcVar1[0x5] - local_7) >> 0x8);
        uVar9 = (uVar9 & 0xffff0000 | (byte)(((bVar2 ^ bVar3) - bVar3) + ((pcVar1[0x4] - local_6 ^ bVar4) - bVar4)))
            + (byte)((pcVar1[0x5] - local_7 ^ bVar5) - bVar5);
        cVar8 = (uVar7 >> 0x8);
        if (uVar9 < local_c) {
            local_c = uVar9;
            local_8 = cVar8;
        }
        cVar6 = uVar7;
        bVar2 = cVar8 + 0x1;
        uVar7 = uVar7 & 0xff | (ushort)bVar2 << 0x8;
        pcVar1 = pcVar1 + 0x3;
    } while (bVar2 != 0x0);
    *param_2 = local_8;
    cVar6 = cVar6 + -0x1;
} while (cVar6 != '\0');
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

LPCSTR *  mci_dev_fn_004a3db0(param_1: &mut String,char *str_param_2)

{
let puVar1: *mut u32;
let mut uVar2: u32;
MCIERROR MVar3;
byte **ppbVar4;
undefined2 *puVar5;
let mut pcVar6: String;
byte *pbVar7;
let local_cc: u8 [0x10];
let local_bc: u8;
let local_4c: u8 [0x18];
let local_34: u8 [0x4];
let local_30: u32;
let local_2c: u32;
BYTE *local_28;
let local_20: u8 [0x4];
let local_1c: u32;
DWORD_PTR local_14;

DAT_005b9ca0 = param_1;
_DAT_005b9bdc = &LAB_004a53d0;
FUN_004a02ae(&LAB_004a5400);
(param_1 + 0x19) = 0x0;
param_1[0x1] = 0x0;
pbVar7 = local_cc;
*(param_1 + 0x36) = 0x0;
puVar1 = FUN_0049ea90();
*param_1 = puVar1;
if ((puVar1 != 0x0) &&
(ppbVar4 = (byte **)FUN_0049c4bd(str_param_2,&DAT_004c357d), ppbVar4 != (byte **)0x0)) {
pcVar6 = FUN_004a2f60(local_cc,0x7f,ppbVar4);
if (pcVar6 != 0x0) {
loop {
pcVar6 = pbVar7;
if (*pbVar7 == '\n')^ // goto LAB_004a3fa2;
if (*pbVar7 == '\0') break;
pcVar6 = pbVar7 + 0x1;
if (*pcVar6 == '\n')^ // goto LAB_004a3fa2;
pbVar7 = (pbVar7 + 0x2);
} while (*pcVar6 != '\0');
pcVar6 = 0x0;
// LAB_004a3fa2:
if (pcVar6 != 0x0) {
*pcVar6 = '\0';
}
local_bc = '\0';
FUN_004a9a00((param_1 + 0x2),local_cc,0x10);
}
while (pcVar6 = FUN_004a2f60(local_cc,0x7f,ppbVar4), pcVar6 != 0x0) {
puVar5 = FUN_004a4580(param_1,local_cc);
if (puVar5 != 0x0) {
FUN_0049eb40(*param_1,puVar5);
}
}
FUN_0049ca40(ppbVar4);
}
if ((*param_1 != 0x0) && (uVar2 = FUN_0049ed90(*param_1), uVar2 == 0x0)) {
FUN_0049af50(*param_1);
*param_1 = 0x0;
}
local_14 = 0x3100;
local_2c = 0x204;
if (_DAT_005b9955 >> 0x18 != 0x0) {
FUN_0049c2e0(local_4c,&DAT_004c3580);
local_28 = local_4c;
local_14 = 0x3300;
}
// DWORD_PTR dwParam2 for mciSendCommandA
// DWORD_PTR dwParam1 for mciSendCommandA
// UINT uMsg for mciSendCommandA
// MCIDEVICEID mciId for mciSendCommandA
MVar3 = mciSendCommandA(0x0,0x803,local_14,(DWORD_PTR)local_34);
*(MCIERROR *)(param_1 + 0x3a) = MVar3;
if (MVar3 != 0x0) {
local_14 = local_14 & 0xfffffdff;
// DWORD_PTR dwParam2 for mciSendCommandA
// DWORD_PTR dwParam1 for mciSendCommandA
// UINT uMsg for mciSendCommandA
// MCIDEVICEID mciId for mciSendCommandA
local_28 = &DAT_004c3584;
MVar3 = mciSendCommandA(0x0,0x803,local_14,(DWORD_PTR)local_34);
*(MCIERROR *)(param_1 + 0x3a) = MVar3;
if (MVar3 != 0x0) {
pcVar6 = FUN_00499050(DAT_005b9bd8,0x7595);
FUN_0049d2e0(0x0,0x0,pcVar6);
return param_1;
}
}
*(DWORD *)(param_1 + 0x3e) = local_30;
// DWORD_PTR dwParam2 for mciSendCommandA
// DWORD_PTR dwParam1 for mciSendCommandA
local_1c = 0xa;
// UINT uMsg for mciSendCommandA
// MCIDEVICEID mciId for mciSendCommandA
MVar3 = mciSendCommandA(*(MCIDEVICEID *)(param_1 + 0x3e),0x80d,0x400,(DWORD_PTR)local_20);
*(MCIERROR *)(param_1 + 0x3a) = MVar3;
if (MVar3 != 0x0) {
pcVar6 = FUN_00499050(DAT_005b9bd8,0x7595);
FUN_0049d2e0(0x0,0x0,pcVar6);
return param_1;
}
return param_1;
}



void  mci_dev_fn_004a4050(param_1: &mut String)

{
let bVar1: u8;
let cVar2: u8;
MCIERROR MVar3;
let piVar4: *mut i32;;
let mut iVar5: i32;
let str_len: *mut u32;
byte **ppbVar6;
undefined2 *puVar7;
undefined2 *puVar8;
let mut uVar9: u32;
byte *pbVar10;
let mut str_ptr_4: String;
byte *pbVar11;
byte *pbVar12;
byte *pbVar13;
let mut iVar14: i32;
LPCSTR *ppCVar15;
let mut pcVar16: String;
let str_win_dir_1e4: u8 [0x114];
let local_d0: u8 [0x80];
let local_50: u8 [0x10];
let local_40: u8;
let mut local_3c: u32;
let mut local_38: i32;
let mut local_34: u32;
let mut local_30: u32;
let mut local_2c: u32;
byte *local_28;
let mut local_24: u32;
let mut local_20: i32;
let mut local_1c: i32;
byte *local_18;
let mut local_14: i32;

// DWORD_PTR dwParam2 for mciSendCommandA
// DWORD_PTR dwParam1 for mciSendCommandA
local_3c = 0x0;
local_34 = 0x3;
// UINT uMsg for mciSendCommandA
// MCIDEVICEID mciId for mciSendCommandA
MVar3 = mciSendCommandA(*(MCIDEVICEID *)(param_1 + 0x3e),0x814,0x100,(DWORD_PTR)&local_3c);
*(MCIERROR *)(param_1 + 0x3a) = MVar3;
if (MVar3 != 0x0) {
return;
}
*(param_1 + 0x2a) = 0x1;
*(param_1 + 0x2e) = local_38;
while( true ) {
local_3c = 0x0;
local_34 = 0x4001;
local_30 = *(param_1 + 0x2a);
// DWORD_PTR dwParam2 for mciSendCommandA
// DWORD_PTR dwParam1 for mciSendCommandA
// UINT uMsg for mciSendCommandA
// MCIDEVICEID mciId for mciSendCommandA
MVar3 = mciSendCommandA(*(MCIDEVICEID *)(param_1 + 0x3e),0x814,0x110,(DWORD_PTR)&local_3c);
*(MCIERROR *)(param_1 + 0x3a) = MVar3;
if (MVar3 != 0x0) {
return;
}
if (local_38 == 0x440) break;
iVar14 = *(param_1 + 0x2a) + 0x1;
*(param_1 + 0x2a) = iVar14;
if (*(param_1 + 0x2e) < iVar14) {
str_ptr_4 = FUN_00499050(DAT_005b9bd8,0x759a);
FUN_0049d2e0(0x0,0x0,str_ptr_4);
}
}
local_40 = 0x0;
local_28 = local_50;
// DWORD_PTR dwParam2 for mciSendCommandA
// DWORD_PTR dwParam1 for mciSendCommandA
local_2c = 0x0;
local_24 = 0x10;
// UINT uMsg for mciSendCommandA
// MCIDEVICEID mciId for mciSendCommandA
MVar3 = mciSendCommandA(*(MCIDEVICEID *)(param_1 + 0x3e),0x80a,0x800,(DWORD_PTR)&local_2c);
*(MCIERROR *)(param_1 + 0x3a) = MVar3;
if (MVar3 != 0x0) {
return;
}
FUN_004a1e60(local_50);
FUN_0049c2e0(local_50,&DAT_004c3585);
uVar9 = 0xffffffff;
pbVar10 = local_28;
loop {
if (uVar9 == 0x0) break;
uVar9 = uVar9 - 0x1;
bVar1 = *pbVar10;
pbVar10 = pbVar10 + 0x1;
} while (bVar1 != 0x0);
local_20 = ~uVar9 - 0x1;
iVar14 = FUN_004a9800((param_1 + 0x19),local_28,local_20);
if (iVar14 != 0x0) {
ppCVar15 = param_1[0x1];
if ((ppCVar15 != *param_1) && (ppCVar15 != 0x0)) {
FUN_004a4830(param_1,ppCVar15);
param_1[0x1] = 0x0;
}
piVar4 = FUN_0049c52b(s____HDILOGO1_SMK_004c358b,&DAT_004c3588);
if (piVar4 != 0x0) {
ppCVar15 = param_1 + 0x2;
pbVar10 = local_28;
loop {
bVar1 = *pbVar10;
*ppCVar15 = bVar1;
if (bVar1 == 0x0) break;
bVar1 = pbVar10[0x1];
pbVar10 = pbVar10 + 0x2;
*(ppCVar15 + 0x1) = bVar1;
ppCVar15 = (ppCVar15 + 0x2);
} while (bVar1 != 0x0);
FUN_0049ca40(piVar4);
}
iVar14 = local_20;
ppCVar15 = param_1 + 0x2;
iVar5 = FUN_004a9800(ppCVar15,local_28,local_20);
if (iVar5 == 0x0) {
pbVar10 = (param_1 + 0x19);
param_1[0x1] = *param_1;
loop {
bVar1 = *ppCVar15;
*pbVar10 = bVar1;
if (bVar1 == 0x0) break;
bVar1 = *(ppCVar15 + 0x1);
ppCVar15 = (ppCVar15 + 0x2);
pbVar10[0x1] = bVar1;
pbVar10 = pbVar10 + 0x2;
} while (bVar1 != 0x0);
}
else {
// UINT uSize for GetWindowsDirectoryA
local_18 = local_28;
// LPSTR lpBuffer for GetWindowsDirectoryA
local_1c = iVar14;
str_len = GetWindowsDirectoryA((LPSTR)str_win_dir_1e4,0x104);
if (str_len != 0x0) {
str_ptr_4 = s__CDPLAYER_INI_004c359b;
iVar14 = -0x1;
pbVar10 = str_win_dir_1e4;
loop {
pcVar16 = pbVar10;
if (iVar14 == 0x0) break;
iVar14 = iVar14 + -0x1;
pcVar16 = pbVar10 + 0x1;
cVar2 = *pbVar10;
pbVar10 = pcVar16;
} while (cVar2 != '\0');
pcVar16 = pcVar16 + -0x1;
loop {
cVar2 = *str_ptr_4;
*pcVar16 = cVar2;
if (cVar2 == '\0') break;
cVar2 = str_ptr_4[0x1];
str_ptr_4 = str_ptr_4 + 0x2;
pcVar16[0x1] = cVar2;
pcVar16 = pcVar16 + 0x2;
} while (cVar2 != '\0');
ppbVar6 = (byte **)FUN_004a96cc(str_win_dir_1e4,&DAT_004c35a9);
if (ppbVar6 == (byte **)0x0) {
// LAB_004a44a3:
str_len = 0x0;
}
else {
loop {
loop {
loop {
str_ptr_4 = FUN_004a2f60(local_d0,0x7f,ppbVar6);
pbVar10 = local_18;
iVar14 = local_1c;
if (str_ptr_4 == 0x0) {
FUN_0049ca40(ppbVar6);^
// goto LAB_004a44a3;
}
pbVar12 = local_d0;
loop {
pbVar11 = pbVar12;
if (*pbVar12 == 0x5b)^ // goto LAB_004a4316;
if (*pbVar12 == 0x0) break;
pbVar11 = pbVar12 + 0x1;
if (*pbVar11 == 0x5b)^ // goto LAB_004a4316;
pbVar12 = pbVar12 + 0x2;
} while (*pbVar11 != 0x0);
pbVar11 = 0x0;
// LAB_004a4316:
} while (pbVar11 == 0x0);
pbVar12 = local_d0;
loop {
pbVar13 = pbVar12;
if (*pbVar12 == 0x5d)^ // goto LAB_004a433d;
if (*pbVar12 == 0x0) break;
pbVar13 = pbVar12 + 0x1;
if (*pbVar13 == 0x5d)^ // goto LAB_004a433d;
pbVar12 = pbVar12 + 0x2;
} while (*pbVar13 != 0x0);
pbVar13 = 0x0;
// LAB_004a433d:
} while (pbVar13 == 0x0);
*pbVar13 = 0x0;
iVar14 = FUN_004a9800(pbVar11 + 0x1,pbVar10,iVar14);
} while (iVar14 != 0x0);
str_len = FUN_0049ea90();
if (str_len == 0x0) {
FUN_0049ca40(ppbVar6);
str_len = 0x0;
}
else {
while ((str_ptr_4 = FUN_004a2f60(local_d0,0x7f,ppbVar6), str_ptr_4 != 0x0 &&
(local_d0[0] != 0x5b))) {
puVar7 = FUN_004a4580(param_1,local_d0);
if (puVar7 != 0x0) {
puVar7[0x1] = puVar7[0x1] + 0x1;
FUN_0049eb40(str_len,puVar7);
}
}
if ((*param_1 != 0x0) && (uVar9 = FUN_0049ed90(*param_1), uVar9 == 0x0)) {
FUN_0049af50(*param_1);
*param_1 = 0x0;
}
FUN_0049ca40(ppbVar6);
}
}
}
param_1[0x1] = str_len;
if (str_len == 0x0) {
str_len = FUN_0049ea90();
param_1[0x1] = str_len;
if ((str_len != 0x0) &&
(local_14 = *(param_1 + 0x2a), local_14 <= *(param_1 + 0x2e))) {
loop {
puVar8 = FUN_0049c2c9(0xa);
str_ptr_4 = s_TRACK_004c35ac;
puVar7 = puVar8 + 0x2;
loop {
cVar2 = *str_ptr_4;
puVar7 = cVar2;
if (cVar2 == '\0') break;
cVar2 = str_ptr_4[0x1];
str_ptr_4 = str_ptr_4 + 0x2;
(puVar7 + 0x1) = cVar2;
puVar7 = puVar7 + 0x1;
} while (cVar2 != '\0');
*puVar8 = 0x1;
puVar8[0x1] = local_14;
FUN_0049eb40(param_1[0x1],puVar8);
local_14 = local_14 + 0x1;
} while (local_14 <= *(param_1 + 0x2e));
}
}
}
}
if ((param_1[0x1] != 0x0) && (uVar9 = FUN_0049ed90(param_1[0x1]), uVar9 != 0x0)) {
uVar9 = FUN_0049ec50(param_1[0x1]);
*(param_1 + 0x36) = uVar9;
FUN_0049edc0(param_1[0x1]);
*(param_1 + 0x32) = **(param_1 + 0x36) >> 0x10;
return;
}
if (param_1[0x1] != 0x0) {
FUN_0049ed30(param_1[0x1]);
param_1[0x1] = 0x0;
}
str_ptr_4 = FUN_00499050(DAT_005b9bd8,0x7594);
FUN_0049d2e0(0x0,0x0,str_ptr_4);
*(param_1 + 0x32) = 0x0;
return;
}



undefined2 *  FUN_004a4580(param_1: u32,byte *param_2)

{
let bVar1: u8;
let mut iVar2: i32;
undefined2 *puVar3;
let mut uVar4: u32;
let mut iVar5: i32;
byte *pbVar6;
byte *pbVar7;
byte *pbVar8;
undefined2 *puVar9;

bVar1 = *param_2;
pbVar6 = param_2;
while (bVar1 == 0x20) {
pbVar8 = pbVar6 + 0x1;
pbVar6 = pbVar6 + 0x1;
bVar1 = *pbVar8;
}
pbVar8 = param_2;
if (((&DAT_004bf9c4)[(byte)(*pbVar6 + 0x1)] & 0x20) != 0x0) {
loop {
pbVar6 = pbVar8;
if (*pbVar8 == 0x3d)^ // goto LAB_004a45d5;
if (*pbVar8 == 0x0) break;
pbVar6 = pbVar8 + 0x1;
if (*pbVar6 == 0x3d)^ // goto LAB_004a45d5;
pbVar8 = pbVar8 + 0x2;
} while (*pbVar6 != 0x0);
pbVar6 = 0x0;
// LAB_004a45d5:
if (pbVar6 != 0x0) {
*pbVar6 = 0x0;
iVar2 = FUN_004a1e60(param_2);
loop {
pbVar8 = pbVar6 + 0x1;
pbVar6 = pbVar6 + 0x1;
pbVar7 = pbVar6;
} while (*pbVar8 == 0x20);
loop {
pbVar8 = pbVar7;
if (*pbVar7 == 0xa)^ // goto LAB_004a460f;
if (*pbVar7 == 0x0) break;
pbVar8 = pbVar7 + 0x1;
if (*pbVar8 == 0xa)^ // goto LAB_004a460f;
pbVar7 = pbVar7 + 0x2;
} while (*pbVar8 != 0x0);
pbVar8 = 0x0;
// LAB_004a460f:
if (pbVar8 != 0x0) {
*pbVar8 = 0x0;
}
uVar4 = 0xffffffff;
pbVar8 = pbVar6;
loop {
if (uVar4 == 0x0) break;
uVar4 = uVar4 - 0x1;
bVar1 = *pbVar8;
pbVar8 = pbVar8 + 0x1;
} while (bVar1 != 0x0);
uVar4 = ~uVar4;
if (uVar4 != 0x1) {
if ((uVar4 - 0x1) < 0x4) {
iVar5 = 0x0;
}
else {
iVar5 = uVar4 - 0x4;
}
puVar3 = FUN_0049c2c9(iVar5 + 0x8);
puVar9 = puVar3 + 0x2;
loop {
bVar1 = *pbVar6;
*puVar9 = bVar1;
if (bVar1 == 0x0) break;
bVar1 = pbVar6[0x1];
pbVar6 = pbVar6 + 0x2;
*(puVar9 + 0x1) = bVar1;
puVar9 = puVar9 + 0x1;
} while (bVar1 != 0x0);
*puVar3 = 0x1;
puVar3[0x1] = iVar2;
return puVar3;
}
}
}
return 0x0;
}



fn FUN_004a4830(param_1: u32,param_2: &mut String)

{
    let mut pCVar1: String;;

    pCVar1 = FUN_0049ec50(param_2);
    while (pCVar1 != 0x0) {
        FUN_0049af50(pCVar1);
        pCVar1 = FUN_0049ec90(param_2);
    }
    FUN_0049ed30(param_2);
    return;
}



fn FUN_004a4c20(param_1: &mut String,param_2: i32) -> u32

{
    LPCSTR *ppCVar1;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let mut iVar6: i32;
    let mut local_2f4: *mut u32 [0x11];
    let local_2af: *mut *mut u8;;
    let mut local_2ab: u32;
    let mut local_2a7: u32;
    let mut local_2a3: u32;
    let mut local_29f: u32;
    let mut local_29b: u32;
    let mut local_297: u32;
    let mut local_203: String;;
    let mut local_1fc: *mut u32 [0x1a];
    let mut local_194: String; [0x11];
    let local_14f: *mut *mut u8;;
    let mut local_14b: u32;
    let mut local_147: u32;
    let mut local_143: String;
    let mut local_13f: u32;
    let mut local_134: String; [0x11];
    let local_ef: *mut *mut u8;;
    let mut local_eb: u32;
    let mut local_e7: u32;
    let mut local_e3: String;
    let mut local_df: u32;
    let mut local_d4: String; [0x11];
    let local_8f: *mut *mut u8;;
    let mut local_8b: u32;
    let mut local_87: u32;
    let mut local_83: String;
    let mut local_7f: u32;
    let mut local_74: String; [0xb];
    let local_47: u8;
    let local_46: u8;
    let local_2f: *mut *mut u8;;
    let mut local_2b: String;
    let mut local_27: u32;
    let mut local_20: String;
    let mut local_1c: String;
    let mut local_18: String;
    let mut local_14: String;

    mci_dev_fn_004a4050(param_1);
    iVar5 = DAT_005b9210;
    if (param_1[0x1] == 0x0) {
        return 0x0;
    }
    iVar3 = DAT_004bf948;
    if (DAT_004bf948 == -0x1) {
        iVar3 = 0x22;
    }
    iVar4 = DAT_005b922c + DAT_005b9210 * -0x10 >> 0x1;
    iVar6 = DAT_005b9228 + DAT_005b9214 * -0x1a >> 0x1;
    FUN_0049a030(local_2f4,param_2,0x1,iVar6,iVar4,0x4e3a,0x4e30,0x0,DAT_005b9b70,iVar3);
    local_2af = &PTR_FUN_004c3d34;
    local_297 = 0x0;
    local_2ab = 0x0;
    local_2a7 = 0x0;
    local_2a3 = 0x0;
    local_203 = 0x0;
    local_29b = 0x0;
    local_29f = 0x0;
    ppCVar1 = FUN_0049a250(local_2f4,0x4);
    *ppCVar1 = param_1;
    local_20 = FUN_00499050(DAT_005b9bd8,0x7596);
    FUN_0049a030(local_74,local_2f4,0x3e9,DAT_005b9214 + iVar6,DAT_005b9210 + iVar4,0x4e38,0x4e21,0x0,DAT_005b9b74,
                 0xffffffff);
    iVar5 = iVar5 + 0x4;
    local_2b = local_20;
    local_27 = DAT_004bf958;
    local_2f = &PTR_FUN_004c3f14;
    local_46 = local_46 | 0x78;
    if ((local_47 & 0xc) != 0x0) {
        local_46 = local_46 & 0xbf;
    }
    FUN_0049bf40(local_2f4,local_74);
    FUN_004ad360(local_1fc,local_2f4,0x3ea,DAT_005b9214 + iVar6,DAT_005b9210 * 0x3 + iVar4,0x4e38,0x4e2a,0x1,
                 DAT_005b9b70,iVar3,param_1[0x1]);
    FUN_0049bf40(local_2f4,local_1fc);
    local_14 = FUN_00499050(DAT_005b9bd8,0x7597);
    FUN_0049a030(local_d4,local_2f4,0x3eb,DAT_005b9214 + iVar6,iVar4 + DAT_005b9210 * 0xe,0x4e28,iVar5,0x0,
                 DAT_005b9b70,iVar3);
    local_83 = local_14;
    local_7f = 0x0;
    local_87 = 0x0;
    local_8f = &PTR_FUN_004c3d94;
    local_8b = 0x2;
    FUN_0049bf40(local_2f4,local_d4);
    local_1c = FUN_00499050(DAT_005b9bd8,0x7598);
    FUN_0049a030(local_194,local_2f4,0x3ec,iVar6 + DAT_005b9214 * 0xa,iVar4 + DAT_005b9210 * 0xe,0x4e28,iVar5,0x0,
                 DAT_005b9b70,iVar3);
    local_143 = local_1c;
    local_13f = 0x0;
    local_147 = 0x0;
    local_14f = &PTR_FUN_004c3d94;
    local_14b = 0x2;
    FUN_0049bf40(local_2f4,local_194);
    local_18 = FUN_00499050(DAT_005b9bd8,0x7599);
    FUN_0049a030(local_134,local_2f4,0x3ed,iVar6 + DAT_005b9214 * 0x13,iVar4 + DAT_005b9210 * 0xe,0x4e26,iVar5,0x0,
                 DAT_005b9b70,iVar3);
    local_e3 = local_18;
    local_ef = &PTR_FUN_004c3d94;
    local_df = 0x0;
    local_e7 = 0x0;
    local_eb = 0x2;
    FUN_0049bf40(local_2f4,local_134);
    uVar2 = FUN_0049bb50(local_2f4,&LAB_004a48e0);
    FUN_004ad550(local_1fc,local_1fc,0x503,0xffff,0x0);
    FUN_0049a1c0(local_134,0x1);
    FUN_0049a1c0(local_194,0x1);
    FUN_0049a1c0(local_d4,0x1);
    FUN_004ad4b0(local_1fc,0x0);
    FUN_0049a1c0(local_74,0x1);
    local_2af = &PTR_FUN_004c3d34;
    if (local_203 != 0x0) {
        FUN_00499b30(local_2f4,local_203);
    }
    FUN_0049a1c0(local_2f4,0x1);
    return uVar2;
}



MCIERROR  FUN_004a5130(param_1: &mut String,param_2: u32)

{
let mut iVar1: i32;
MCIERROR MVar2;
let mut pcVar3: String;
DWORD_PTR dwParam1;
HWND local_1c;
let mut local_18: u32;
let mut local_14: u32;

mci_dev_fn_004a4050(param_1);
if ((param_1[0x1] == 0x0) || (*(param_1 + 0x3a) != 0x0)) {
return 0x0;
}
local_18 = param_2 & 0xff;
if (param_2 == *(param_1 + 0x2e)) {
dwParam1 = 0x5;
}
else {
dwParam1 = 0xd;
local_14 = (byte)(param_2 + 0x1);
}
local_1c = hWnd_005b9be0;
// DWORD_PTR dwParam2 for mciSendCommandA
// DWORD_PTR dwParam1 for mciSendCommandA
// UINT uMsg for mciSendCommandA
// MCIDEVICEID mciId for mciSendCommandA
MVar2 = mciSendCommandA(*(MCIDEVICEID *)(param_1 + 0x3e),0x806,dwParam1,(DWORD_PTR)&local_1c);
if (MVar2 != 0x0) {
pcVar3 = FUN_00499050(DAT_005b9bd8,0x7595);
FUN_0049d2e0(0x0,0x0,pcVar3);
}
iVar1 = *(param_1 + 0x32);
if (iVar1 != -0x1) {
while (*(param_1 + 0x32) <= param_2) {
FUN_004a5210(param_1);
if (iVar1 == *(param_1 + 0x32)) {
return MVar2;
}
if (*(param_1 + 0x32) == -0x1) {
return MVar2;
}
}
}
return MVar2;
}



fn FUN_004a5210(param_1: i32) -> i32

{
let piVar1: *mut i32;;
let piVar2: *mut i32;;
let mut uVar3: u32;
let mut iVar4: i32;

piVar1 = *(param_1 + 0x36);
if (*(param_1 + 0x3a) != 0x0) {
return -0x1;
}
if (*(param_1 + 0x4) == 0x0) {
iVar4 = *(param_1 + 0x32) + 0x1;
*(param_1 + 0x32) = iVar4;
if (*(param_1 + 0x2e) < iVar4) {
*(param_1 + 0x32) = *(param_1 + 0x2a);
}
return *(param_1 + 0x32);
}
FUN_0049edd0(*(param_1 + 0x4));
while( true ) {
uVar3 = FUN_0049ec90((param_1 + 0x4));
*(param_1 + 0x36) = uVar3;
if (uVar3 == 0x0) {
uVar3 = FUN_0049ec50((param_1 + 0x4));
*(param_1 + 0x36) = uVar3;
}
piVar2 = *(param_1 + 0x36);
if (piVar1 == piVar2) break;
if (((piVar2 != 0x0) || (*piVar2 >> 0x10 < *(param_1 + 0x2a))) ||
(*(param_1 + 0x2e) < *piVar2 >> 0x10)) {
FUN_0049edc0((param_1 + 0x4));
iVar4 = **(param_1 + 0x36) >> 0x10;
*(param_1 + 0x32) = iVar4;
return iVar4;
}
}
*(param_1 + 0x32) = 0xffffffff;
return *(param_1 + 0x32);
}



MCIERROR  FUN_004a52d0(param_1: i32)

{
MCIERROR MVar1;
let mut local_8: i32;

local_8 = *(param_1 + 0x3a);
if (local_8 != 0x0) {
return 0x0;
}
// DWORD_PTR dwParam2 for mciSendCommandA
// DWORD_PTR dwParam1 for mciSendCommandA
// UINT uMsg for mciSendCommandA
// MCIDEVICEID mciId for mciSendCommandA
MVar1 = mciSendCommandA(*(MCIDEVICEID *)(param_1 + 0x3e),0x808,0x0,(DWORD_PTR)&local_8);
return MVar1;
}



fn FUN_004a5310(param_1: i32) -> i32

{
MCIERROR MVar1;
let mut pcVar2: String;
let mut iVar3: i32;
DWORD_PTR dwParam1;
HWND local_10;
let mut local_c: u32;
let mut local_8: u32;

if (*(param_1 + 0x3a) == 0x0) {
if (((*(short **)(param_1 + 0x36) != (short *)0x0) && (**(short **)(param_1 + 0x36) == 0x0)) ||
(*(param_1 + 0x32) == -0x1)) {
FUN_004a5210(param_1);
}
if (*(param_1 + 0x32) != -0x1) {
local_c = *(param_1 + 0x32);
if (*(param_1 + 0x32) == *(param_1 + 0x2e)) {
dwParam1 = 0x5;
}
else {
dwParam1 = 0xd;
local_8 = (byte)((param_1 + 0x32) + 0x1);
}
local_10 = hWnd_005b9be0;
// DWORD_PTR dwParam2 for mciSendCommandA
// DWORD_PTR dwParam1 for mciSendCommandA
// UINT uMsg for mciSendCommandA
// MCIDEVICEID mciId for mciSendCommandA
MVar1 = mciSendCommandA(*(MCIDEVICEID *)(param_1 + 0x3e),0x806,dwParam1,(DWORD_PTR)&local_10);
*(MCIERROR *)(param_1 + 0x3a) = MVar1;
if (MVar1 != 0x0) {
pcVar2 = FUN_00499050(DAT_005b9bd8,0x7595);
FUN_0049d2e0(0x0,0x0,pcVar2);
}
iVar3 = FUN_004a5210(param_1);
*(param_1 + 0x32) = iVar3;
return iVar3;
}
}
return 0x0;
}



fn FUN_004a5420(LPCSTR param_1)

{
    delete_file_fn_004b1630(param_1);
    return;
}



fn move_file_004a5430(LPCSTR existing_file_name,LPCSTR new_file_name) -> u32

{
    let mut moved: bool;
    let err_code: u32;

    // LPCSTR lpNewFileName for MoveFileA
    // LPCSTR lpExistingFileName for MoveFileA
    moved = MoveFileA(existing_file_name,new_file_name);
    if (moved == 0x0) {
        err_code = handle_err_fn_004b12fc();
        return err_code;
    }
    return 0x0;
}



fn FUN_004a5dd0(param_1: *mut u8,param_2: i32,param_3: i32)

{
    let mut unaff_ESI: u32;

    if (param_3 != 0x0) {
        FUN_004a6a00();
    }
    if (param_2 != 0x0) {
        unaff_ESI = FUN_004a5e40(param_1);
    }
    if (*(param_1 + 0xc) != 0x0) {
        FUN_004a7806((param_1 + 0x65));
        FUN_0049ae40(param_1);
        FUN_0049ab50(param_1,0x0);
    }
    if (param_2 == 0x0) {
        return;
    }
    FUN_004a5ec0(param_1,unaff_ESI);
    return;
}



fn FUN_004a5e40(param_1: i32) -> u32

{
    let mut iVar1: i32;

    if (((param_1 + 0x61) != -0x1) && (*(param_1 + 0x4) != 0x0)) {
        if ((param_1 + 0x63) == -0x1) {
            iVar1 = (*(param_1 + 0x5f) >> 0x10) * 0x12 + *(param_1 + 0x49);
            if ((*(iVar1 + 0xe) == 0x0) && ((*(iVar1 + 0x5) & 0x2) == 0x0)) {
                return (iVar1 + 0xc);
            }
        }
        else {
            iVar1 = *(*(param_1 + 0x49) + 0xe + (*(param_1 + 0x5f) >> 0x10) * 0x12) +
                (*(param_1 + 0x61) >> 0x10) * 0x8;
            if ((*(iVar1 + 0x5) & 0x6) == 0x0) {
                return (iVar1 + 0x6);
            }
        }
    }
    return 0xffffffff;
}



fn FUN_004a5ec0(param_1: i32,param_2: i32)

{
    byte *pbVar1;
    let mut iVar2: i32;
    let mut pcVar3: String;

    if ((param_1 + 0x4) != 0x0) {
        if (param_2 != -0x1) {
            FUN_0049a770((param_1 + 0x4),0x407,param_2,0x0);
        }
        pcVar3 = s_DIALOG_004c35cc;
        pbVar1 = ((*(*(param_1 + 0x4) + 0x45) + 0x14))(*(param_1 + 0x4));
        iVar2 = FUN_004a2f10(pbVar1,pcVar3);
        if (iVar2 == 0x0) {
            FUN_0049bf80(*(u32 ***)(param_1 + 0x4),0x0,0x40d,0x0,0x0);
            return;
        }
    }
    return;
}



fn FUN_004a5f20(param_1: i32)

{
    let mut uVar1: u32;
    let mut uVar2: u32;

    FUN_004953d7();
    uVar1 = DAT_005b981c;
    uVar2 = DAT_004bf924;
    if ((*(param_1 + 0x2e) & 0x80) != 0x0) {
        uVar1 = DAT_004bf924;
        uVar2 = DAT_005b981c;
    }
    FUN_0049e640(*(param_1 + 0x1d),*(param_1 + 0x21),*(param_1 + 0x25),*(param_1 + 0x29),
                 uVar1,uVar2,DAT_004bf928,0x1);
    FUN_0049536f();
    return;
}



i32
FUN_004a5f80(param_1: *mut u32,param_2: i32,param_3: u32,param_4: i32,param_5: i32,param_6: u32,
param_7: u32,param_8: u32,param_9: u32,param_10: u32,param_11: &mut String,byte *param_12)

{
ushort *puVar1;
let cVar2: u8;
ushort uVar3;
let puVar4: *mut u32;
let ppcVar5: *mut *mut char;
let piVar6: *mut i32;;
let mut uVar7: u32;
let mut iVar8: i32;
let mut uVar9: u32;
let mut iVar10: i32;
let ppcVar11: *mut *mut char;
let mut unaff_EBP: u32;
let mut iVar12: i32;
let mut pcVar13: String;
let mut pcVar14: String;
let mut pcVar15: String;
let mut iStack40: i32;
let mut iStack36: i32;
let mut iStack32: i32;
let local_18: i16;
ushort uStack20;
let sStack16: i16;
let uStack14: u16;

sStack16 = unaff_EBP;
uStack14 = (undefined2)(unaff_EBP >> 0x10);
pcVar14 = &stack0xffffffc0;
puVar4 = FUN_0049a030(param_1,param_2,param_3,param_4,param_5,*(param_2 + 0x25),DAT_005b9210,0x800,param_6,
param_7);
$1: &mut String(puVar4 + 0x45) = &PTR_FUN_004c4094;
pcVar13 = param_11;
loop {
cVar2 = *pcVar13;
*pcVar14 = cVar2;
if (cVar2 == '\0') break;
cVar2 = pcVar13[0x1];
pcVar13 = pcVar13 + 0x2;
pcVar14[0x1] = cVar2;
pcVar14 = pcVar14 + 0x2;
} while (cVar2 != '\0');
pcVar13 = &DAT_004c35d3;
iVar8 = -0x1;
pcVar14 = &stack0xffffffc0;
loop {
pcVar15 = pcVar14;
if (iVar8 == 0x0) break;
iVar8 = iVar8 + -0x1;
pcVar15 = pcVar14 + 0x1;
cVar2 = *pcVar14;
pcVar14 = pcVar15;
} while (cVar2 != '\0');
pcVar15 = pcVar15 + -0x1;
loop {
cVar2 = *pcVar13;
*pcVar15 = cVar2;
if (cVar2 == '\0') break;
cVar2 = pcVar13[0x1];
pcVar13 = pcVar13 + 0x2;
pcVar15[0x1] = cVar2;
pcVar15 = pcVar15 + 0x2;
} while (cVar2 != '\0');
ppcVar5 = FUN_00498e10(&stack0xffffffc0,param_12);
if (ppcVar5 == 0x0) {
pcVar14 = FUN_00499050(DAT_005b9bd8,0x7d02);
pop_err_msg_box_and_exit_004a02f5(pcVar14);
}
FUN_004a7970(&uStack20,0x2,0x1,ppcVar5);
iVar8 = FUN_0049c2c9(uStack20 * 0x12);
*(param_2 + 0x49) = iVar8;
if (iVar8 == 0x0) {
pcVar14 = FUN_00499050(DAT_005b9bd8,0x7d01);
pop_err_msg_box_and_exit_004a02f5(pcVar14);
}
iStack40 = DAT_005b9214;
iStack36 = 0x0;
iStack32 = 0x0;
loop {
if (uStack20 <= iStack36) {
FUN_0049ca40(ppcVar5);
*(param_2 + 0x4d) = uStack20;
iVar8 = (uStack20 - 0x1) * 0x12 + *(param_2 + 0x49);
*(param_2 + 0x51) = (iVar8 + 0x8) + (iVar8 + 0x6);
*(param_2 + 0x55) = param_9;
*(param_2 + 0x59) = param_10;
*(param_2 + 0x5d) = param_11;
*(param_2 + 0x61) = 0xffffffff;
*(param_2 + 0x25) = *(param_2 + 0x51);
return param_2;
}
FUN_004a7970((*(param_2 + 0x49) + iStack32),0x12,0x1,ppcVar5);
FUN_004a7970(&sStack16,0x2,0x1,ppcVar5);
uVar7 = FUN_0049c2c9(sStack16);
*(*(param_2 + 0x49) + iStack32) = uVar7;
if (*(*(param_2 + 0x49) + iStack32) == 0x0) {
pcVar14 = FUN_00499050(DAT_005b9bd8,0x7d01);
pop_err_msg_box_and_exit_004a02f5(pcVar14);
}
FUN_004a7970((*(param_2 + 0x49) + iStack32),0x1,sStack16,ppcVar5);
iVar8 = *(param_2 + 0x49) + iStack32;
if (*(iVar8 + 0xe) != 0x0) {
uVar7 = FUN_0049c2c9((iVar8 + 0xa) << 0x3);
*(*(param_2 + 0x49) + iStack32 + 0xe) = uVar7;
if (*(*(param_2 + 0x49) + iStack32 + 0xe) == 0x0) {
pcVar14 = FUN_00499050(DAT_005b9bd8,0x7d01);
pop_err_msg_box_and_exit_004a02f5(pcVar14);
}
iVar12 = 0x0;
iVar8 = *(param_2 + 0x49) + iStack32;
ppcVar11 = *(char ***)(iVar8 + 0xe);
*(iVar8 + 0xc) = 0x0;
while (iVar12 < (*(param_2 + 0x49) + iStack32 + 0xa)) {
FUN_004a7970(ppcVar11,0x8,0x1,ppcVar5);
if ((*(ppcVar11 + 0x5) & 0x4) == 0x0) {
FUN_004a7970(&local_18,0x2,0x1,ppcVar5);
pcVar14 = FUN_0049c2c9(local_18);
*ppcVar11 = pcVar14;
if (pcVar14 == 0x0) {
pcVar14 = FUN_00499050(DAT_005b9bd8,0x7d01);
pop_err_msg_box_and_exit_004a02f5(pcVar14);
}
FUN_004a7970(*ppcVar11,0x1,local_18,ppcVar5);
uVar9 = 0xffffffff;
pcVar14 = *ppcVar11;
loop {
if (uVar9 == 0x0) break;
uVar9 = uVar9 - 0x1;
cVar2 = *pcVar14;
pcVar14 = pcVar14 + 0x1;
} while (cVar2 != '\0');
iVar8 = FUN_00497282(*ppcVar11,LPCSTR_005b9218,~uVar9 - 0x1);
iVar10 = *(param_2 + 0x49) + iStack32;
puVar1 = (iVar10 + 0xc);
uVar3 = (ushort)iVar8;
if (uVar3 < *puVar1 || uVar3 == *puVar1)^ // goto LAB_004a6305;
(iVar10 + 0xc) = uVar3;
ppcVar11 = ppcVar11 + 0x2;
iVar12 = iVar12 + 0x1;
}
else {
*ppcVar11 = 0x0;
// LAB_004a6305:
ppcVar11 = ppcVar11 + 0x2;
iVar12 = iVar12 + 0x1;
}
}
}
piVar6 = (*(param_2 + 0x49) + iStack32);
iVar8 = FUN_00497282(*piVar6,LPCSTR_005b9218,(piVar6 + 0x6));
(*(param_2 + 0x49) + iStack32 + 0x6) = DAT_005b9214 + iVar8;
(*(param_2 + 0x49) + iStack32 + 0x8) = iStack40;
iVar8 = *(param_2 + 0x49) + iStack32;
iStack32 = iStack32 + 0x12;
iStack40 = iStack40 + (iVar8 + 0x6);
iStack36 = iStack36 + 0x1;
} while( true );
}



fn FUN_004a63d0(param_1: i32,param_2: i32)

{
    let piVar1: *mut i32;;
    let mut iVar2: i32;
    let mut uVar3: u32;

    FUN_004953d7();
    piVar1 = (param_2 * 0x12 + *(param_1 + 0x49));
    if (*(param_1 + 0x5f) >> 0x10 == param_2) {
        uVar3 = *(param_1 + 0x59);
        iVar2 = *(param_1 + 0x31);
    }
    else {
        uVar3 = *(param_1 + 0x31);
        iVar2 = *(param_1 + 0x35);
    }
    FUN_00497567((piVar1 + 0x2) + *(param_1 + 0x1d),*(param_1 + 0x21),*piVar1,
                 (piVar1 + 0x6) - DAT_005b9214,uVar3,iVar2,*(param_1 + 0x55),
                 LPCSTR_005b9218,0x10);
    FUN_0049536f();
    return;
}



fn FUN_004a6510(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: i32)

{
    let mut iVar1: i32;
    let mut uVar2: u32;
    let piVar3: *mut i32;;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let mut local_1c: u32;
    let mut local_18: i32;
    let local_14: u8;
    let local_13: u8;

    iVar1 = *(param_1 + 0x49) + param_2 * 0x12;
    piVar3 = (*(iVar1 + 0xe) + param_3 * 0x8);
    if ((*(piVar3 + 0x5) & 0x6) == 0x0) {
        if (*(param_1 + 0x61) >> 0x10 == param_3) {
            local_1c = *(param_1 + 0x59);
            local_18 = *(param_1 + 0x31);^
            // goto LAB_004a6567;
        }
        local_1c = *(param_1 + 0x31);
        if (param_4 == 0x0) {
            local_18 = *(param_1 + 0x35);^
            // goto LAB_004a6567;
        }
    }
    else {
        local_1c = *(param_1 + 0x5d);
        if (param_4 == 0x0) {
            local_18 = *(param_1 + 0x35);^
            // goto LAB_004a6567;
        }
    }
    local_18 = -0x1;
    LAB_004a6567:
        FUN_004953d7();
    if (param_5 == 0x0) {
        iVar5 = (iVar1 + 0x8) + *(param_1 + 0x1d) + 0x1;
        iVar4 = param_3 * DAT_005b9210 + *(param_1 + 0x29) + 0x2 + *(param_1 + 0x21);
    }
    else {
        iVar5 = 0x1;
        iVar4 = param_3 * DAT_005b9210 + 0x1;
    }
    if ((*(piVar3 + 0x5) & 0x4) == 0x0) {
        if ((*(piVar3 + 0x5) & 0x1) == 0x0) {
            local_14 = 0x20;
        }
        else {
            local_14 = 0x2b;
        }
        local_13 = 0x0;
        FUN_00497567(iVar5,iVar4,&local_14,DAT_005b9214,local_1c,local_18,*(param_1 + 0x55),
                     LPCSTR_005b9218,0x10);
        uVar2 = local_1c;
        if ((*(piVar3 + 0x5) & 0x2) == 0x0) {
            uVar2 = *(param_1 + 0x55);
        }
        FUN_00497567(iVar5 + DAT_005b9214,iVar4,*piVar3,(iVar1 + 0xc) + DAT_005b9214,local_1c,local_18,
                     uVar2,LPCSTR_005b9218,0x10);
        FUN_0049536f();
        return;
    }
    if (param_4 == 0x0) {
        FUN_00497567(iVar5,iVar4,&DAT_004c35d8,DAT_005b9214 * 0x2 + (iVar1 + 0xc),local_1c,local_18,
                     *(param_1 + 0x55),LPCSTR_005b9218,0x10);
    }
    iVar4 = iVar4 + DAT_005b9210 / 0x2;
    FUN_00497896(iVar5 + 0x1,iVar4,DAT_005b9214 * 0x2 + iVar5 + (iVar1 + 0xc) + -0x2,iVar4,
                 *(param_1 + 0x31));
    FUN_0049536f();
    return;
}



fn FUN_004a6720(param_1: i32,param_2: i32)

{
    let puVar1: *mut u32;
    let mut iVar2: i32;
    i32 **ppiVar3;
    let mut iVar4: i32;
    let mut iVar5: i32;

    iVar5 = *(param_1 + 0x49) + param_2 * 0x12;
    FUN_004a6a00();
    DAT_005b9cb4._0_2_ = (param_1 + 0x1d) + (iVar5 + 0x8);
    DAT_005b9cb4._2_2_ = (param_1 + 0x21) + DAT_005b9210 + 0x1;
    DAT_005b9cb8._0_2_ = DAT_005b9214 * 0x2 + (iVar5 + 0xc) + 0x2;
    DAT_005b9cb8._2_2_ = DAT_005b9210 * (iVar5 + 0xa) + 0x2;
    puVar1 = FUN_0049c2c9(DAT_005b9cb8._2_2_ * DAT_005b9cb8);
    DAT_005b9cb0._0_2_ = SUB42(puVar1,0x0);
    DAT_005b9cb0._2_2_ = (undefined2)(puVar1 >> 0x10);
    if (puVar1 != 0x0) {
        FUN_00496c1f(DAT_005b9cb4,DAT_005b9cb4._2_2_,puVar1,DAT_005b9cb8,
                     DAT_005b9cb8._2_2_);
    }
    puVar1 = FUN_0049c2c9(DAT_005b9cb8._2_2_ * DAT_005b9cb8);
    FUN_004953d7();
    if (puVar1 == 0x0) {
        iVar2 = *(param_1 + 0x21) + DAT_005b9210;
        for (; puVar1 < (iVar5 + 0xa); puVar1 = (puVar1 + 0x1)) {
            FUN_004a6510(param_1,param_2,puVar1,0x0,0x1);
        }
        FUN_0049e640(puVar1,iVar2 + 0x2,DAT_005b9cb8 + -0x1,DAT_005b9cb8._2_2_ + -0x1,DAT_004bf924,
                     DAT_005b981c,DAT_004bf928,0x1);
        FUN_0049536f();
        return;
    }
    ppiVar3 = FUN_004a2831(0x18);
    if (ppiVar3 != 0x0) {
    ppiVar3 = FUN_00498ba4(ppiVar3,puVar1,DAT_005b9cb8._2_2_,DAT_005b9cb8);
}
    for (iVar2 = 0x0; iVar2 < (iVar5 + 0xa); iVar2 = iVar2 + 0x1) {
        FUN_004a6510(param_1,param_2,iVar2,0x0,0x1);
    }
    FUN_0049e640(0x1,0x1,DAT_005b9cb8 + -0x2,DAT_005b9cb8._2_2_ + -0x2,DAT_004bf924,DAT_005b981c,
                 DAT_004bf928,0x1);
    if (ppiVar3 != 0x0) {
    ppiVar3 = FUN_00498cf4(ppiVar3);
    FUN_0049af50(ppiVar3);
}
    if (DAT_005b922c < 0xfa) {
        iVar5 = 0x2;
    }
    else {
        iVar5 = 0x4;
    }
    iVar2 = 0x0;
    iVar4 = DAT_005b9cb8._2_2_;
    FUN_004a2d6b();
    while (iVar5 < iVar4) {
        iVar2 = iVar2 + iVar5;
        iVar4 = iVar4 - iVar5;
        FUN_00496ac0((DAT_005b9cb8 * iVar4 + puVar1),DAT_005b9cb4,
                     DAT_005b9cb4._2_2_,DAT_005b9cb8,iVar2);
        timer_func_0049e710(0x1);
    }
    FUN_00496ac0(puVar1,DAT_005b9cb4,DAT_005b9cb4._2_2_,DAT_005b9cb8,DAT_005b9cb8._2_2_)
    ;
    FUN_0049af50(puVar1);
    FUN_0049536f();
    return;
}



fn FUN_004a6a00()

{
    let puVar1: *mut u32;
    let mut iVar2: i32;
    let mut local_14: i32;

    FUN_004953d7();
    if (CONCAT22(DAT_005b9cb0._2_2_,(undefined2)DAT_005b9cb0) != 0x0) {
        if (DAT_005b922c < 0xfa) {
            local_14 = 0x2;
        }
        else {
            local_14 = 0x4;
        }
        iVar2 = DAT_005b9cb8._2_2_;
        puVar1 = FUN_0049c2c9(DAT_005b9cb8 * iVar2);
        if (puVar1 == 0x0) {
            local_14 = DAT_005b9cb8._2_2_;
            puVar1 = CONCAT22(DAT_005b9cb0._2_2_,(undefined2)DAT_005b9cb0);
        }
        else {
            FUN_004a2d6b();
            FUN_00496c1f(DAT_005b9cb4,DAT_005b9cb4._2_2_,puVar1,DAT_005b9cb8,
                         DAT_005b9cb8._2_2_);
            while (local_14 < iVar2) {
                iVar2 = iVar2 - local_14;
                FUN_00496ac0((DAT_005b9cb8 * (DAT_005b9cb8._2_2_ - iVar2) + puVar1),
                             DAT_005b9cb4,DAT_005b9cb4._2_2_,DAT_005b9cb8,iVar2);
                FUN_00496ac0((CONCAT22(DAT_005b9cb0._2_2_,(undefined2)DAT_005b9cb0) + iVar2 * DAT_005b9cb8)
                             ,DAT_005b9cb4,DAT_005b9cb4._2_2_ + iVar2,DAT_005b9cb8,local_14);
                timer_func_0049e710(0x1);
            }
            FUN_0049af50(puVar1);
            puVar1 = CONCAT22(DAT_005b9cb0._2_2_,(undefined2)DAT_005b9cb0);
        }
        FUN_00496ac0(puVar1,DAT_005b9cb4,DAT_005b9cb4._2_2_,DAT_005b9cb8,local_14);
        FUN_0049af50(CONCAT22(DAT_005b9cb0._2_2_,(undefined2)DAT_005b9cb0));
        DAT_005b9cb0._0_2_ = 0x0;
        DAT_005b9cb0._2_2_ = 0x0;
    }
    FUN_0049536f();
    return;
}



fn FUN_004a6bc0(param_1: i32,param_2: u32,param_3: i32,param_4: i32)

{
    let mut bVar1: bool;
    let mut iVar2: i32;
    let mut uVar3: u32;
    let mut local_1c: i32;

    iVar2 = *(param_1 + 0x5f) >> 0x10;
    local_1c = *(param_1 + 0x61) >> 0x10;
    bVar1 = false;
    if (((param_2 < *(param_1 + 0x4d)) && (-0x1 < param_2)) && (-0x2 < param_3)) {
        loop {
            if ((param_4 == 0x0) || ((*(*(param_1 + 0x49) + 0x5 + param_2 * 0x12) & 0x2) == 0x0)) {
                FUN_004953d7();
                if (*(param_1 + 0x5f) >> 0x10 == param_2) {
                    LAB_004a6d25:
                    if ((param_3 != -0x1) || ((param_1 + 0x63) != 0x0))^ // goto LAB_004a6c80;
                    LAB_004a6c8a:
                    if (!bVar1)^ // goto LAB_004a6d3e;
                }
                else {
                    (param_1 + 0x61) = param_2;
                    if (iVar2 != -0x1) {
                        FUN_004a63d0(param_1,iVar2);
                    }
                    FUN_004a63d0(param_1,param_2);
                    FUN_004a6a00();
                    local_1c = -0x1;
                    bVar1 = true;
                    param_4 = 0x1;
                    if (param_3 == -0x1)^ // goto LAB_004a6d25;
                    param_3 = 0x0;
                    LAB_004a6c80:
                    if (param_3 == *(param_1 + 0x61) >> 0x10)^ // goto LAB_004a6c8a;
                }
                uVar3 = (*(param_1 + 0x49) + 0xa + param_2 * 0x12);
                if (uVar3 <= param_3) {
                    if (!bVar1) {
                        LAB_004a6d3e:
                            FUN_0049536f();
                        return;
                    }
                    param_3 = uVar3 - 0x1;
                }
                if (((param_1 + 0x63) == -0x1) || (param_3 != -0x1)) {
                    if (param_3 != -0x1) {
                        while ((param_4 != 0x0 &&
                            ((*(*((*(param_1 + 0x5f) >> 0x10) * 0x12 + 0xe + *(param_1 + 0x49)) +
                                param_3 * 0x8 + 0x5) & 0x6) != 0x0))) {
                            param_3 = param_3 + param_4;
                            if ((*(param_1 + 0x49) + param_2 * 0x12 + 0xa) <= param_3) {
                                return;
                            }
                        }
                        (param_1 + 0x63) = param_3;
                        if (local_1c == -0x1) {
                            FUN_004a6720(param_1,param_2);
                        }
                        else {
                            FUN_004a6510(param_1,param_2,local_1c,0x0,0x0);
                        }
                        FUN_004a6510(param_1,param_2,param_3,0x0,0x0);
                        FUN_0049536f();
                        return;
                    }
                }
                else {
                    FUN_004a6a00();
                    *(param_1 + 0x63) = 0xffff;
                }
                FUN_0049536f();
                return;
            }
            param_2 = param_2 + param_4;
        } while ((param_2 < *(param_1 + 0x4d)) && (-0x1 < param_2));
    }
    return;
}



fn FUN_004a6e10(param_1: i32,param_2: u32,byte param_3,param_4: i32) -> u32

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: i32;

    local_1c = 0x0;
    local_18 = 0x0;
    if (*(param_1 + 0x4d) != 0x0) {
        local_14 = 0x0;
        loop {
            iVar1 = *(param_1 + 0x49) + local_14;
            if ((*(iVar1 + 0xe) == 0x0) && ((iVar1 + 0xc) == param_2)) {
                if (param_4 == 0x0) {
                    *(iVar1 + 0x5) = *(iVar1 + 0x5) & ~param_3;
                }
                else {
                    *(iVar1 + 0x5) = *(iVar1 + 0x5) | param_3;
                }
                local_1c = 0x1;
            }
            iVar1 = *(*(param_1 + 0x49) + local_14 + 0xe);
            if (iVar1 != 0x0) {
                iVar2 = 0x0;
                while (iVar2 < (local_14 + 0xa + *(param_1 + 0x49))) {
                    if ((iVar1 + 0x6) == param_2) {
                        if (param_4 == 0x0) {
                            *(iVar1 + 0x5) = *(iVar1 + 0x5) & ~param_3;
                            iVar1 = iVar1 + 0x8;
                            iVar2 = iVar2 + 0x1;
                        }
                        else {
                            *(iVar1 + 0x5) = *(iVar1 + 0x5) | param_3;
                            iVar1 = iVar1 + 0x8;
                            iVar2 = iVar2 + 0x1;
                        }
                    }
                    else {
                        iVar1 = iVar1 + 0x8;
                        iVar2 = iVar2 + 0x1;
                    }
                }
            }
            local_14 = local_14 + 0x12;
            local_18 = local_18 + 0x1;
        } while (local_18 < *(param_1 + 0x4d));
    }
    return local_1c;
}



// WARNING: Unable to track spacebase fully for stack
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void entry(void)

{
let mut iVar1: i32;
let mut iVar2: i32;
let mut puVar3: *mut u8;
HMODULE pHVar4;
let dVar5: u32;
let mut puVar6: *mut u8;
let mut puVar7: *mut u8;
let puVar8: *mut u32;
let mut uStack32: u32;
let mut uStack28: u32;
let DStack24: u32;
let piStack20: *mut i32;;
let auStack16: u8 [0x8];

piStack20 = 0x4b1661;
FUN_004b1cf0();
iVar1 = -(DAT_004c0338 + 0x3U & 0xfffffffc);
*(auStack16 + iVar1 + -0x4) = DAT_004c0338;
*(&DStack24 + iVar1) = 0x0;
(&uStack28 + iVar1) = auStack16 + iVar1;
*(&uStack32 + iVar1) = 0x4b167e;
FUN_004a0430(*(&uStack28 + iVar1),(&DStack24 + iVar1),
*(auStack16 + iVar1 + -0x4));
(auStack16 + iVar1 + -0x4) = auStack16 + iVar1;
*(&stack0x000000e0 + iVar1) = DAT_004c0338;
(&DStack24 + iVar1) = auStack16;
*(&uStack28 + iVar1) = 0x4b1696;
get_mod_handle_fn_004aeee8(*(DWORD *)(&DStack24 + iVar1),*(auStack16 + iVar1 + -0x4));
puVar7 = auStack16 + iVar1;
puVar6 = (DAT_004c0150 + 0x3U & 0xfffffffc);
*(auStack16 + iVar1 + -0x4) = 0x4b16aa;
puVar3 = FUN_004b7330();
if (puVar6 < puVar3) {
(&piStack20 + iVar1) = puVar6;
*(&DStack24 + iVar1) = 0x4b16b4;
FUN_004b102f(*(&piStack20 + iVar1));
iVar2 = -(DAT_004c0150 + 0x3U & 0xfffffffc);
puVar7 = auStack16 + iVar2 + iVar1;
_DAT_004c0154 = auStack16 + iVar2 + iVar1;
}
else {
_DAT_004c0154 = 0x0;
}
_DAT_004c0154 = _DAT_004c0154 + DAT_004c0150;
*(puVar7 + -0x4) = 0x4b16d8;
FUN_004b7340();
*(puVar7 + -0x4) = 0xa;
*(puVar7 + -0x8) = DAT_004c012c;
*(puVar7 + -0xc) = 0x0;
*(puVar7 + -0x10) = 0x0;
puVar8 = (puVar7 + -0x14);
*(puVar7 + -0x14) = 0x4b16ec;
pHVar4 = GetModuleHandleA(*(puVar7 + -0x10));
puVar8[-0x1] = pHVar4;
puVar8[-0x2] = 0x4b16f2;
dVar5 = important_func_0042d653((HINSTANCE)puVar8[-0x1],*puVar8,puVar8[0x1],puVar8[0x2]);
puVar8[0x2] = dVar5;
puVar8[0x1] = 0x4b16f8;
FUN_004a9a64(puVar8[0x2]);
return;
}



fn FUN_004a70a2(param_1: i32,param_2: u32,param_3: i32) -> i32

{
code *pcVar1;
let mut iVar2: i32;
let mut iVar3: i32;
let mut uVar4: u32;

pcVar1 = *(code **)(param_3 + 0x4);
iVar2 = *(param_3 + 0x10);
iVar3 = param_1;
for (uVar4 = 0x0; uVar4 < param_2; uVar4 = uVar4 + 0x1) {
(*pcVar1)(iVar3,uVar4,param_3);
iVar3 = iVar3 + iVar2;
}
return param_1;
}



fn FUN_004a70ea(param_1: *mut u32)

{
    FUN_004b1727(param_1);
    return;
}



fn FUN_004a70f8(param_1: *mut i32) -> i32

{
let mut iVar1: i32;
code *pcVar2;
let mut iVar3: i32;

iVar1 = *(param_1[0x1] + 0x10);
pcVar2 = *(code **)(param_1[0x1] + 0xc);
iVar3 = *param_1 * iVar1 + param_1[0x2];
while (*param_1 != 0x0) {
iVar3 = iVar3 - iVar1;
*param_1 = *param_1 + -0x1;
(*pcVar2)(iVar3,0x0);
}
return param_1[0x2];
}



fn FUN_004a7132(param_1: u32,param_2: i32,param_3: u32) -> u32

{
    let mut local_c: i32;
    let mut local_8: u32;
    let mut local_4: u32;

    local_c = param_2;
    local_8 = param_3;
    local_4 = param_1;
    FUN_004a70f8(&local_c);
    return param_1;
}



fn FUN_004a7160(param_1: *mut u32,param_2: u32,param_3: i32,param_4: *mut i32) -> u32

{
    let mut uVar1: u32;
    let puVar2: *mut u32;
    let mut uVar3: u32;
    let mut uVar4: u32;
    let mut iVar5: i32;
    let bVar6: u8;
    let mut bVar7: bool;
    let mut uStack24: u32;
    let mut uStack20: u32;

    (PTR_FUN_004bfb78)(param_4[0x4]);
    if ((*(param_4 + 0x3) & 0x2) == 0x0) {
        FUN_004b1740(0x4);
        *(param_4 + 0x3) = *(param_4 + 0x3) | 0x20;
        (PTR_FUN_004bfb7c)(param_4[0x4]);
        uStack24 = 0x0;
    }
    else {
        uStack20 = param_3 * param_2;
        if (uStack20 == 0x0) {
            (PTR_FUN_004bfb7c)(param_4[0x4]);
            return 0x0;
        }
        if (*(param_4[0x2] + 0x8) == 0x0) {
            FUN_004b1790(param_4);
        }
        uVar1 = param_4[0x3];
        bVar6 = *(param_4 + 0x3);
        uStack24 = 0x0;
        *(param_4 + 0x3) = bVar6 & 0xcf;
        if ((bVar6 & 0x40) == 0x0) {
            bVar7 = (*(param_4 + 0xd) & 0x4) != 0x0;
            if (bVar7) {
                bVar6 = *(param_4 + 0xd) & 0xfa;
                *(param_4 + 0xd) = bVar6;
                *(param_4 + 0xd) = bVar6 | 0x1;
            }
            uVar3 = *(param_4[0x2] + 0xc);
            *(param_4[0x2] + 0xc) = 0x1;
            loop {
                bVar6 = *param_1;
                param_1 = (param_1 + 0x1);
                FUN_004a9080(bVar6,param_4);
                if ((*(param_4 + 0x3) & 0x30) != 0x0) break;
                uStack24 = uStack24 + 0x1;
            } while (uStack20 - uStack24 != 0x0);
            *(param_4[0x2] + 0xc) = uVar3;
            if (bVar7) {
                bVar6 = *(param_4 + 0xd) & 0xfa;
                *(param_4 + 0xd) = bVar6;
                *(param_4 + 0xd) = bVar6 | 0x4;
                FUN_004af2f0(param_4);
            }
        }
        else {
            loop {
                if ((param_4[0x1] == 0x0) && (param_4[0x5] <= uStack20)) {
                    uVar4 = uStack20 & 0xffff0000 | (uStack20 >> 0x8 & 0xfe) << 0x8;
                    if (uVar4 == 0x0) {
                        uVar4 = uStack20;
                    }
                    uVar4 = write_file_004b1830(param_4[0x4],param_1,uVar4);
                    if (uVar4 != 0xffffffff) {
                        if (uVar4 != 0x0)^ // goto LAB_004a72b0;
                        iVar5 = (PTR_FUN_004bfb74)();
                        *(iVar5 + 0x4) = 0xc;
                    }
                    *(param_4 + 0x3) = *(param_4 + 0x3) | 0x20;
                }
                else {
                    uVar4 = param_4[0x5] - param_4[0x1];
                    if (uStack20 < (param_4[0x5] - param_4[0x1])) {
                        uVar4 = uStack20;
                    }
                    puVar2 = *param_4;
                    *puVar2 = *param_1;
                    (puVar2 + 0x1) = (param_1 + 0x1);
                    *param_4 = *param_4 + uVar4;
                    param_4[0x1] = param_4[0x1] + uVar4;
                    *(param_4 + 0xd) = *(param_4 + 0xd) | 0x10;
                    if ((param_4[0x1] == param_4[0x5]) || ((*(param_4 + 0xd) & 0x4) != 0x0)) {
                        FUN_004af2f0(param_4);
                    }
                }
                LAB_004a72b0:
                    param_1 = (param_1 + uVar4);
                uStack24 = uStack24 + uVar4;
                uStack20 = uStack20 - uVar4;
            } while ((uStack20 != 0x0) && ((*(param_4 + 0x3) & 0x20) == 0x0));
        }
        if ((*(param_4 + 0x3) & 0x20) != 0x0) {
            uStack24 = 0x0;
        }
        param_4[0x3] = param_4[0x3] | uVar1 & 0x30;
        (PTR_FUN_004bfb7c)(param_4[0x4]);
        uStack24 = uStack24 / param_2;
    }
    return uStack24;
}



fn FUN_004a7399()

{
    DAT_005b9cd0 = 0xc8;
    DAT_005b9cdc = 0x10;
    DAT_005b9ce0 = 0x10;
    DAT_005b9ce4 = 0xff;
    FUN_004a70ea(&DAT_004bf988);
    return;
}



fn FUN_004a73f7()

{
    let DVar1: u32;
    let mut uVar2: u32;

    if (((DAT_005b9cd4 == 0x1) &&
        (DVar1 = timeGetTime(), uVar2 = (DVar1 - DAT_005b9ccc) >> 0x1f,
         DAT_005b9cd0 <= ((DVar1 - DAT_005b9ccc ^ uVar2) - uVar2))) &&
        ((DAT_005b9ce8 != 0x0 ||
            (DAT_005b9ce8 = FUN_0049c2c9(DAT_005b9cdc * DAT_005b9ce0), DAT_005b9ce8 != 0x0)))) {
        DAT_005b9cd8 = DAT_005b9cd8 ^ 0x1;
        if ((DAT_005b9ce0 != 0x0) && (DAT_005b9cdc != 0x0)) {
            FUN_00495520(DAT_005b9cc4,DAT_005b9cc8,DAT_005b9ce0,DAT_005b9cdc,0x1);
            if (DAT_005b9cd8 == 0x0) {
                FUN_00496ac0(DAT_005b9ce8,DAT_005b9cc4,DAT_005b9cc8,DAT_005b9ce0,DAT_005b9cdc);
            }
            else {
                FUN_00496c1f(DAT_005b9cc4,DAT_005b9cc8,DAT_005b9ce8,DAT_005b9ce0,DAT_005b9cdc);
                if (DAT_005b9cec == (code *)0x0) {
                    FUN_004968e7(DAT_005b9cc4,DAT_005b9cc8,DAT_005b9ce0,DAT_005b9cdc,DAT_005b9ce4);
                }
            }
        }
        DAT_005b9ccc = timeGetTime();
        if (DAT_005b9cec != (code *)0x0) {
            (*DAT_005b9cec)(DAT_005b9cd8);
        }
        if ((DAT_005b9ce0 != 0x0) && (DAT_005b9cdc != 0x0)) {
            FUN_00495607(0x0);
        }
    }
    return;
}



fn FUN_004a756b()

{
    let DVar1: u32;

    if (DAT_005b9cd8 != 0x0) {
        DVar1 = timeGetTime();
        DAT_005b9ccc = DVar1 + DAT_005b9cd0 * -0x2;
        FUN_004a73f7();
    }
    DAT_005b9cd4 = DAT_005b9cd4 + -0x1;
    return;
}



fn FUN_004a75a6()

{
    DAT_005b9cd4 = DAT_005b9cd4 + 0x1;
    FUN_004a73f7();
    return;
}



fn FUN_004a75c2(param_1: i32,param_2: i32)

{
    FUN_004a756b();
    if ((DAT_005b9ce8 == 0x0) ||
        (param_1 * param_2 - DAT_005b9cdc * DAT_005b9ce0 != 0x0 && DAT_005b9cdc * DAT_005b9ce0 <= param_1 * param_2)) {
        if (DAT_005b9ce8 != 0x0) {
            FUN_0049af50(DAT_005b9ce8);
        }
        DAT_005b9ce8 = FUN_0049c2c9(param_1 * param_2);
    }
    DAT_005b9cdc = param_1;
    DAT_005b9ce0 = param_2;
    DAT_005b9cd4 = DAT_005b9cd4 + 0x1;
    FUN_004a73f7();
    return;
}



fn FUN_004a763f(param_1: u32,param_2: u32)

{
    FUN_004a756b();
    DAT_005b9cc4 = param_1;
    DAT_005b9cc8 = param_2;
    DAT_005b9cd4 = DAT_005b9cd4 + 0x1;
    FUN_004a73f7();
    return;
}



fn FUN_004a7670(param_1: u32)

{
    FUN_004a756b();
    DAT_005b9ce4 = param_1;
    DAT_005b9cd4 = DAT_005b9cd4 + 0x1;
    FUN_004a73f7();
    return;
}



fn FUN_004a7699(param_1: i32,param_2: i32)

{
    let mut iVar1: i32;
    let mut iVar2: i32;

    iVar1 = param_1 * DAT_005b9214;
    iVar2 = (param_2 + 0x1) * DAT_005b9210 - DAT_005b9cdc;
    FUN_004a756b();
    DAT_005b9cd4 = DAT_005b9cd4 + 0x1;
    DAT_005b9cc4 = iVar1;
    DAT_005b9cc8 = iVar2;
    FUN_004a73f7();
    return;
}



fn FUN_004a76f9(param_1: i32)

{
    let mut uVar1: u32;
    let mut iVar2: i32;

    uVar1 = DAT_005b9cc4;
    if (param_1 == 0x0) {
        param_1 = ((DAT_005b9210 + (DAT_005b9210 >> 0x1f) * -0x4) - ((DAT_005b9210 >> 0x1f) << 0x1 < 0x0)) >> 0x2
        ;
    }
    else {
        param_1 = DAT_005b9210;
    }
    iVar2 = DAT_005b9cc8 + DAT_005b9cdc;
    FUN_004a756b();
    DAT_005b9cd4 = DAT_005b9cd4 + 0x1;
    DAT_005b9cc4 = uVar1;
    DAT_005b9cc8 = iVar2 - param_1;
    FUN_004a73f7();
    FUN_004a75c2(param_1,DAT_005b9214);
    return;
}



fn FUN_004a778a(param_1: *mut u32)

{
    *param_1 = DAT_005b9cc4;
    param_1[0x1] = DAT_005b9cc8;
    param_1[0x2] = DAT_005b9ccc;
    param_1[0x3] = DAT_005b9cd0;
    param_1[0x4] = DAT_005b9cd4;
    param_1[0x5] = DAT_005b9cdc;
    param_1[0x6] = DAT_005b9ce0;
    param_1[0x7] = DAT_005b9ce4;
    param_1[0x8] = DAT_005b9cec;
    return;
}



fn FUN_004a7806(param_1: *mut u32)

{
    let DVar1: u32;

    if (DAT_005b9cd8 != 0x0) {
        DVar1 = timeGetTime();
        DAT_005b9ccc = DVar1 + DAT_005b9cd0 * -0x2;
        FUN_004a73f7();
    }
    FUN_004a75c2(param_1[0x5],param_1[0x6]);
    DAT_005b9cc4 = *param_1;
    DAT_005b9cc8 = param_1[0x1];
    DAT_005b9ccc = param_1[0x2];
    DAT_005b9cd0 = param_1[0x3];
    DAT_005b9cd4 = param_1[0x4];
    DAT_005b9ce4 = param_1[0x7];
    DAT_005b9cec = param_1[0x8];
    return;
}



fn FUN_004a789b()

{
    let DVar1: u32;

    if (DAT_005b9cd8 != 0x0) {
        DVar1 = timeGetTime();
        DAT_005b9ccc = DVar1 + DAT_005b9cd0 * -0x2;
        FUN_004a73f7();
    }
    FUN_004a75c2(0x2,0x8);
    DAT_005b9cd0 = 0xc8;
    DAT_005b9cd4 = 0x0;
    DAT_005b9cd8 = 0x0;
    DAT_005b9ce4 = 0xf;
    DAT_005b9cec = 0x0;
    return;
}



fn FUN_004a790e(param_1: u32) -> u32

{
    DAT_005b9cd0 = 0xc8;
    DAT_005b9cdc = 0x10;
    DAT_005b9ce0 = 0x10;
    DAT_005b9ce4 = 0xff;
    return param_1;
}



fn FUN_004a7952(param_1: u32) -> u32

{
    return param_1;
}



// WARNING: Type propagation algorithm not settling

fn FUN_004a7970(param_1: *mut u32,param_2: u32,param_3: i32,char **param_4) -> u32

{
    let cVar1: u8;
    let mut uVar2: u32;
    let mut pcVar3: String;
    let DVar4: u32;
    LPVOID pvVar5;
    let puVar6: *mut u32;
    let puVar7: *mut u32;
    let mut pcVar8: String;
    let mut pcStack20: String;

    (PTR_FUN_004bfb78)(param_4[0x4]);
    if ((*(param_4 + 0x3) & 0x1) == 0x0) {
        FUN_004b1740(0x4);
        *(param_4 + 0x3) = *(param_4 + 0x3) | 0x20;
        (PTR_FUN_004bfb7c)(param_4[0x4]);
        uVar2 = 0x0;
    }
    else {
        pcVar8 = (param_3 * param_2);
        if (pcVar8 == 0x0) {
            (PTR_FUN_004bfb7c)(param_4[0x4]);
            return 0x0;
        }
        if (*(param_4[0x2] + 0x8) == 0x0) {
            FUN_004b1790(param_4);
        }
        pcStack20 = 0x0;
        if ((*(param_4 + 0x3) & 0x40) == 0x0) {
            puVar6 = param_1;
            loop {
                if ((param_4[0x1] == 0x0) && (pvVar5 = FUN_004a9350(param_4), pvVar5 == (LPVOID)0x0)) break;
                pcVar3 = *param_4;
                param_4[0x1] = param_4[0x1] + -0x1;
                *param_4 = pcVar3 + 0x1;
                cVar1 = *pcVar3;
                if (cVar1 == '\r') {
                    if ((param_4[0x1] == 0x0) && (pvVar5 = FUN_004a9350(param_4), pvVar5 == (LPVOID)0x0)) break;
                    param_4[0x1] = param_4[0x1] + -0x1;
                    pcVar3 = *param_4;
                    *param_4 = pcVar3 + 0x1;
                    cVar1 = *pcVar3;
                }
                if (cVar1 == '\x1a') {
                    *(param_4 + 0x3) = *(param_4 + 0x3) | 0x10;
                    break;
                }
                puVar7 = (puVar6 + 0x1);
                pcStack20 = pcStack20 + 0x1;
                puVar6 = cVar1;
                puVar6 = puVar7;
            } while (puVar7 != (pcVar8 + param_1));
        }
        else {
            loop {
                while( true ) {
                    pcVar3 = param_4[0x1];
                    if (pcVar3 != 0x0) {
                        if (pcVar8 < pcVar3) {
                            pcVar3 = pcVar8;
                        }
                        puVar6 = *param_4;
                        *param_1 = *puVar6;
                        (param_1 + 0x1) = (puVar6 + 0x1);
                        pcVar8 = pcVar8 + -pcVar3;
                        param_1 = (param_1 + pcVar3);
                        pcStack20 = pcStack20 + pcVar3;
                        *param_4 = *param_4 + pcVar3;
                        param_4[0x1] = param_4[0x1] + -pcVar3;
                    }
                    if (pcVar8 == 0x0)^ // goto LAB_004a7b49;
                    if ((pcVar8 < param_4[0x5]) && ((*(param_4 + 0xd) & 0x4) == 0x0)) break;
                    *param_4 = *(param_4[0x2] + 0x8);
                    param_4[0x1] = 0x0;
                    pcVar3 = pcVar8;
                    if (((*(param_4 + 0xd) & 0x4) == 0x0) && (0x200 < pcVar8)) {
                        pcVar3 = (pcVar8 & 0xffff0000 | (pcVar8 >> 0x8 & 0xfe) << 0x8);
                    }
                    DVar4 = read_file_004b1940(param_4[0x4],param_1,(DWORD)pcVar3);
                    if (DVar4 == 0xffffffff) {
                        *(param_4 + 0x3) = *(param_4 + 0x3) | 0x20;^
                        // goto LAB_004a7b49;
                    }
                    if (DVar4 == 0x0) {
                        *(param_4 + 0x3) = *(param_4 + 0x3) | 0x10;^
                        // goto LAB_004a7b49;
                    }
                    param_1 = (param_1 + DVar4);
                    pcVar8 = pcVar8 + -DVar4;
                    pcStack20 = pcStack20 + DVar4;
                }
                pvVar5 = FUN_004a9350(param_4);
            } while (pvVar5 != (LPVOID)0x0);
        }
        LAB_004a7b49:
            (PTR_FUN_004bfb7c)(param_4[0x4]);
        uVar2 = pcStack20 / param_2;
    }
    return uVar2;
}



fn FUN_004a7b70() -> i32

{
let mut bVar1: bool;
let mut pcVar2: String;
undefined3 extraout_var;

DAT_005b9cf0 = FUN_0049c2c9(0x500);
if (DAT_005b9cf0 == 0x0) {
pcVar2 = FUN_00499050(DAT_005b9bd8,0x7d01);
pop_err_msg_box_and_exit_004a02f5(pcVar2);
}
bVar1 = FUN_0049c57b(s_scroll_bin_004c35fc,DAT_005b9cf0,0x500);
if (CONCAT31(extraout_var,bVar1) == 0x0) {
return 0x0;
}
DAT_005b9cf4 = DAT_005b9cf0 + 0x80;
DAT_005b9cf8 = DAT_005b9cf0 + 0x40;
DAT_005b9cfc = DAT_005b9cf0 + 0xc0;
DAT_005b9d00 = DAT_005b9cf0 + 0x100;
return CONCAT31(extraout_var,bVar1);
}

