
fn FUN_0040ba36()

{
    let local_20: *mut u32;

    local_20 = FUN_004a2831(0x49);
    if (local_20 != 0x0) {
        local_20 = FUN_0049a030(local_20,0x0,0x1,0x0,0x0,0x280,0x1e0,0x0,0x0,0x0);
    }
    DAT_004c614c = local_20;
    FUN_0049ae00(local_20,FUN_0040bb0a);
    FUN_0049a8a0(DAT_004c614c);
    return;
}



fn FUN_0040babb()

{
    FUN_0049ab50(DAT_004c614c,0x0);
    if (DAT_004c614c != 0x0) {
        ((*(DAT_004c614c + 0x45) + 0x8))(DAT_004c614c,0x2);
    }
    return;
}



fn FUN_0040bb0a(param_1: i32,param_2: i32,DWORD param_3,DWORD param_4) -> u32

{
    let local_18: u32;

    if (param_2 == 0x405) {
        // LPARAM lParam for SendMessageA
        // WPARAM wParam for SendMessageA
        // UINT Msg for SendMessageA
        // HWND hWnd for SendMessageA
        SendMessageA(hWnd_00596a34,0x405,0x0,0x0);
        local_18 = 0x1;
    }
    else {
        local_18 = ((*(param_1 + 0x45) + 0xc))(param_1,param_1,param_2,param_3,param_4);
    }
    return local_18;
}



fn FUN_0040bb78(param_1: i32,param_2: i32,param_3: i32,param_4: i32) -> u32

{
    let mut iVar1: i32;
    let local_14: *mut u32;

    local_14 = *DAT_005967c8;
    while( true ) {
        if (local_14 == 0x0) {
            return 0x0;
        }
        if ((((local_14 + 0xe) == 0x11) && (*(local_14 + 0x6) >> 0x10 == param_1)) &&
            (*(local_14 + 0xe) >> 0x10 == param_4)) break;
        local_14 = *local_14;
    }
    iVar1 = FUN_0044a87f(param_2,param_3,local_14[0x2] >> 0x10,*(local_14 + 0xa) >> 0x10);
    if (iVar1 <= shield_radius_00599d9c) {
        return 0x1;
    }
    return 0x0;
}



fn FUN_0040bc1e(param_1: i32,param_2: i32) -> u32

{
    let mut local_18: u32;

    if (param_1 < param_2) {
        if (param_1 == 0x0) {
            local_18 = 0x0;
        }
        else {
            switch((param_1 + -0x1 + param_2) / param_1) {
                case 0x2:
                    local_18 = 0x3;
                break;
                case 0x3:
                    local_18 = 0x2;
                break;
                case 0x4:
                    local_18 = 0x1;
                break;
                default:
                    local_18 = 0x0;
            }
        }
    }
    else {
        if (param_2 == 0x0) {
            local_18 = 0xb;
        }
        else {
            switch(param_1 / param_2) {
                case 0x1:
                    local_18 = 0x4;
                break;
                case 0x2:
                    local_18 = 0x5;
                break;
                case 0x3:
                    local_18 = 0x6;
                break;
                case 0x4:
                    local_18 = 0x7;
                break;
                case 0x5:
                    local_18 = 0x8;
                break;
                case 0x6:
                    local_18 = 0x9;
                break;
                case 0x7:
                    local_18 = 0xa;
                break;
                default:
                    local_18 = 0xb;
            }
        }
    }
    return local_18;
}



// WARNING: Removing unreachable block (ram,0x0040be20)

fn FUN_0040bd7e(param_1: i32,param_2: i32,param_3: i32,param_4: i32) -> i32

{
let bVar1: u8;
let mut bVar2: bool;
undefined3 extraout_var;
let mut local_18: i32;

local_18 = *(param_3 * 0x4 +
*(&DAT_00582938 +
(*(param_1 + 0x25) >> 0x18) * 0x4 + (*(param_1 + 0x24) >> 0x18) * 0x18) +
0x81) * (*(param_1 + 0x30) >> 0x18);
bVar1 = *(param_1 + 0x2e);
if (bVar1 == 0x0) {
local_18 = (local_18 * 0x8) / 0xa;
}
else {
if ((0x1 < bVar1) && (bVar1 == 0x2)) {
local_18 = (local_18 * 0xc) / 0xa;
}
}
if (((param_1 + 0x2c) & 0x1c) == 0x8) {
local_18 = (local_18 * 0xc) / 0xa;
}
else {
if (((param_1 + 0x2c) & 0x1c) == 0x10) {
local_18 = (local_18 << 0x3) / 0xa;
}
}
if (param_4 == 0x8) {
if (((&DAT_00569e55)[(*(param_1 + 0x23) >> 0x18) * 0x1e22] & 0x1) != 0x0) {
local_18 = (local_18 * 0xc) / 0xa;
}
}
else {
if ((param_4 == 0x7) && (((&DAT_00569e5e)[(*(param_1 + 0x23) >> 0x18) * 0x1e22] & 0x1) != 0x0)) {
local_18 = (local_18 * 0xc) / 0xa;
}
}
if (((param_2 != 0x0) &&
(bVar2 = FUN_0046292a((&DAT_00568210 + (*(param_1 + 0x23) >> 0x18) * 0x1e22)),
CONCAT31(extraout_var,bVar2) != 0x0)) && (DAT_004d79fc._3_1_ == (param_1 + 0x26))) {
local_18 = ((DAT_004d799a + 0x64) * local_18) / 0x64;
}
return local_18;
}



// WARNING: Removing unreachable block (ram,0x0040bffa)

fn FUN_0040bf3c(param_1: i32,param_2: i32,param_3: i32,param_4: u32) -> i32

{
let bVar1: u8;
let mut bVar2: bool;
undefined3 extraout_var;
let mut local_18: i32;
let mut local_14: i32;

local_14 = *(*(&DAT_00582938 +
(*(param_1 + 0x25) >> 0x18) * 0x4 + (*(param_1 + 0x24) >> 0x18) * 0x18) +
0x55);
if (param_3 != 0x0) {
local_14 = local_14 + (*(&DAT_00569b90 + (*(param_1 + 0x23) >> 0x18) * 0x1e22) >> 0x18);
}
local_18 = local_14 * (*(param_1 + 0x30) >> 0x18);
bVar1 = *(param_1 + 0x2e);
if (bVar1 == 0x0) {
local_18 = (local_18 * 0x8) / 0xa;
}
else {
if ((0x1 < bVar1) && (bVar1 == 0x2)) {
local_18 = (local_18 * 0xc) / 0xa;
}
}
if (((param_1 + 0x2c) & 0x1c) == 0x8) {
local_18 = (local_18 << 0x3) / 0xa;
}
else {
if (((param_1 + 0x2c) & 0x1c) == 0x10) {
local_18 = (local_18 * 0xc) / 0xa;
}
}
if (param_4 < 0x10) {
if (param_4 < 0x1) {
if (-0x2 < param_4) {
if (0x7fffffff < param_4)^ // goto LAB_0040c0f5;^
// goto LAB_0040c057;
}
}
else {
if (param_4 < 0x2) {
// LAB_0040c057:
local_18 = local_18 << 0x1;^
// goto LAB_0040c0f5;
}
if (0xc < param_4) {
if (param_4 < 0xe)^ // goto LAB_0040c057;
if (param_4 == 0xf)^ // goto LAB_0040c0f5;
}
}
}
else {
if (param_4 < 0x11)^ // goto LAB_0040c0f5;
if (param_4 < 0x1c) {
if (0x19 < param_4)^ // goto LAB_0040c0f5;
}
else {
if ((((param_4 < 0x1d) || (param_4 < 0x1e)) || (param_4 < 0x1f)) || (param_4 == 0x1f))^
// goto LAB_0040c0f5;
}
}
local_18 = (local_18 * 0xf) / 0xa;
// LAB_0040c0f5:
if (((param_2 != 0x0) &&
(bVar2 = FUN_0046292a((&DAT_00568210 + (*(param_1 + 0x23) >> 0x18) * 0x1e22)),
CONCAT31(extraout_var,bVar2) != 0x0)) && (DAT_004d7931 == (param_1 + 0x26))) {
local_18 = ((DAT_004d78cc + 0x64) * local_18) / 0x64;
}
return local_18;
}



fn FUN_0040c15d(param_1: i32,param_2: i32,param_3: i32,param_4: u32)

{
    let sVar1: i16;
    let sVar2: i16;
    let sVar3: i16;
    let sVar4: i16;
    let mut iVar5: i32;
    let mut iVar6: i32;
    let mut iVar7: i32;
    let local_3c: *mut u32;

    FUN_00431d5a(&DAT_005967c0,&DAT_005967b8);
    if ((DAT_005967bc != 0x0) && (iVar5 = FUN_00410fb3(&DAT_005967b8,0x3), iVar5 != 0x0)) {
        sVar3 = -0x1;
        sVar4 = -0x1;
        iVar6 = (DAT_005967bc + 0x20);
        sVar1 = (DAT_005967bc + 0x22);
        sVar2 = (DAT_005967bc + 0x24);
        iVar5 = *(DAT_005967bc + 0x23);
        for (local_3c = (&DAT_005b8b44 + iVar6 * 0x4);
            (local_3c != 0x0 && ((local_3c + 0x8) == iVar6)); local_3c = *local_3c) {
            if ((((*(local_3c + 0x3a) & 0x1) != 0x0) &&
                ((((local_3c + 0x22) != sVar3 && ((local_3c + 0x9) != sVar4)) &&
                    (*(local_3c + 0x23) >> 0x18 == param_3)))) &&
                ((((local_3c + 0x2a) == 0x8 &&
                    (iVar7 = FUN_0044a87f((local_3c + 0x22),(local_3c + 0x9),param_1,param_2),
                     iVar7 != 0x0)) && (iVar7 < 0x6)))) {
                sVar3 = (local_3c + 0x22);
                sVar4 = (local_3c + 0x9);
                FUN_00432683(&DAT_005967b8,iVar6,(local_3c + 0x22),(local_3c + 0x9));
                FUN_00459ca5(iVar6,(local_3c + 0x22),(local_3c + 0x9),
                             *(DAT_005967c4 + 0x23) >> 0x18);
                FUN_0040a70e(0x3,sVar1,sVar2,iVar5 >> 0x18,param_4);
            }
        }
    }
    return;
}



fn FUN_0040c567() -> u32

{
    let mut local_118: *mut u32 [0x11];
    let ppuStack211: *mut *mut u8;;
    let mut local_27: String;;
    let mut local_20: u32;
    let mut local_1c: u32;
    let local_18: *mut i32;;

    local_18 = FUN_004990e0(local_118,0x0,s_efs_res_004c087b,s_OrdersDlg_004c0871);
    local_1c = FUN_0049bb50(local_118,FUN_0040c67d);
    if (local_1c != 0x0) {
        local_1c = local_1c - 0xc7;
    }
    local_20 = local_1c;
    ppuStack211 = &PTR_FUN_004c3d34;
    if (local_27 != 0x0) {
        FUN_00499b30(local_118,local_27);
    }
    FUN_0049a1c0(local_118,0x1);
    return local_20;
}



fn FUN_0040c67d(param_1: i32,param_2: i32,param_3: i32) -> u32

{
    if (param_2 == 0x407) {
        if (param_3 == 0x65) {
            FUN_0049c140(param_1,0x0);
        }
        else {
            FUN_0049c140(param_1,param_3);
        }
    }
    return 0x0;
}



i32  sleep_0040c6d8(param_1: &mut String)

{
let mut extraout_EAX: i32;
let mut iVar1: i32;
let mut local_1c: i32;
let mut local_18: String;
let mut local_14: i32;

FUN_0049c2c9(0x3a0);
// DWORD dwMilliseconds for Sleep
Sleep(0x4b);
iVar1 = extraout_EAX;
for (local_14 = 0x0; local_14 < 0x3; local_14 = local_14 + 0x1) {
local_18 = param_1;
for (local_1c = 0x0; local_1c < 0x4; local_1c = local_1c + 0x1) {
FUN_00496ee6(local_18,local_14 * 0x4 + 0x10e,local_14 * 0x4 + 0x177,0x1d,0x20);
local_18 = local_18 + 0x3a0;
// DWORD dwMilliseconds for Sleep
Sleep(0x4b);
}
iVar1 = local_14;
}
return iVar1;
}



fn FUN_0040c771(param_1: &mut String,param_2: i32,param_3: i32,param_4: i32,param_5: i32,param_6: i32,param_7: i32)

{
    let puVar1: *mut u32;
    let mut local_1c: i32;
    let mut local_14: i32;

    puVar1 = FUN_0049c2c9(0x484);
    if (0x64 < param_7) {
        param_7 = 0x64;
    }
    timer_func_0049e710(0x4b);
    FUN_0042feac();
    for (local_14 = 0x0; local_14 < 0x4; local_14 = local_14 + 0x1) {
        FUN_00496ee6(param_1,param_5 + 0x1,param_6 + 0x1,0x1d,0x20);
        param_1 = param_1 + 0x3a0;
        timer_func_0049e710(0x4b);
        local_1c = param_4 - (local_14 + 0x1) * (param_7 >> 0x2);
        if (local_1c < 0x0) {
            local_1c = 0x0;
        }
        FUN_004906c1(puVar1,*(&DAT_004d6058 + param_2 * 0x1c),
                     (char)*(&DAT_004be9e8 + param_3 * 0x4),local_1c,0x22,-0x1);
        FUN_00496ac0(puVar1,param_5,param_6,0x22,0x22);
        FUN_0042feac();
    }
    FUN_0049af50(puVar1);
    return;
}



fn FUN_0040c88e(param_1: *mut i32)

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    undefined2 *puVar3;
    let piVar4: *mut i32;;
    let mut bVar5: bool;
    let mut bVar6: bool;
    let local_1c: u16;
    let local_1a: u16;
    let local_18: u16;
    i32 **local_14;

    local_1c = *(param_1 + 0x2);
    local_1a = *(param_1 + 0xa);
    local_18 = *(param_1 + 0x3);
    local_14 = (i32 **)*DAT_005967b0;
    loop {
    if (local_14 == (i32 **)0x0) {
        FUN_004811e6(param_1);
        return;
    }
    iVar2 = 0x6;
    bVar5 = false;
    iVar1 = 0x0;
    bVar6 = true;
    puVar3 = &local_1c;
    piVar4 = (local_14 + 0x8);
    loop {
        if (iVar2 == 0x0) break;
        iVar2 = iVar2 + -0x1;
        bVar5 = *puVar3 < *piVar4;
        bVar6 = *puVar3 == *piVar4;
        puVar3 = (puVar3 + 0x1);
        piVar4 = (piVar4 + 0x1);
    } while (bVar6);
    if (!bVar6) {
        iVar1 = (0x1 - bVar5) - (bVar5 != 0x0);
    }
    if ((((iVar1 == 0x0) &&
        (*(*(&DAT_00582938 +
            (*(local_14 + 0x25) >> 0x18) * 0x4 + (local_14[0x9] >> 0x18) * 0x18) + 0xad)
            != 0x0)) && ((local_14 + 0x27) != '[')) &&
        (((local_14 + 0x27) != '\x1d' && ((local_14 + 0x27) != '\x1c')))) {
        local_14 = (i32 **)FUN_00484b4e(local_14);
    }
    local_14 = (i32 **)*local_14;
} while( true );
}



fn FUN_0040c966(param_1: i32,param_2: i32,param_3: i32,param_4: i32) -> u32

{
    byte *pbVar1;
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_24: i32;

    local_2c = 0x0;
    switch(*(*(param_2 * 0x5e + param_3) + 0x24) >> 0x18) {
    case 0x2d:
        case 0x30:
        case 0x31:
        case 0x33:
        local_28 = 0x6;
    break;
    default:
        local_28 = 0x0;
}
    while( true ) {
        while( true ) {
            for (; local_28 < 0xa; local_28 = local_28 + 0x1) {
                for (local_24 = 0x0; local_24 < 0x14; local_24 = local_24 + 0x1) {
                    if (((((((local_24 * 0x5e + param_4 + 0x4) & 0x1) != 0x0) &&
                        (((local_24 * 0x5e + param_4 + 0x4) & 0x8000) == 0x0)) &&
                        (((local_24 * 0x5e + param_4 + 0x4) & 0x8) != 0x0)) &&
                        ((((local_24 * 0x5e + param_4 + 0x4) & 0x20) == 0x0 &&
                            (((local_24 * 0x5e + param_4 + 0x4) & 0x80) == 0x0)))) &&
                        (*(local_24 * 0x5e + param_4 + 0x46) == local_28)) {
                        if (((DAT_004c611c == 0x1) || (DAT_004c611c == 0x4)) &&
                            ((*(*(local_24 * 0x5e + param_4) + 0x28) >> 0x10 == 0x5 ||
                                ((*(*(local_24 * 0x5e + param_4) + 0x28) >> 0x10 == 0x7 ||
                                    (*(*(local_24 * 0x5e + param_4) + 0x28) >> 0x10 == 0x9)))))) {
                            if (*(&DAT_00591b44 + param_1 * 0x4) != 0x0) {
                                pbVar1 = (local_24 * 0x5e + param_4 + 0x4);
                                *pbVar1 = *pbVar1 | 0x20;
                                *(param_2 * 0x5e + param_3 + 0x42) = local_24;
                                return 0x1;
                            }
                        }
                        else {
                            if (*(&DAT_00591b44 +
                                (*(*(local_24 * 0x5e + param_4) + 0x28) >> 0x10) * 0x24 + param_1 * 0x4) != 0x0)
                            {
                                pbVar1 = (local_24 * 0x5e + param_4 + 0x4);
                                *pbVar1 = *pbVar1 | 0x20;
                                *(param_2 * 0x5e + param_3 + 0x42) = local_24;
                                return 0x1;
                            }
                        }
                    }
                }
            }
            local_2c = local_2c + 0x1;
            if (local_28 != 0x6) break;
            if (local_2c == 0x2) {
                local_28 = 0x0;
                local_2c = 0x0;
            }
            else {
                for (local_24 = 0x0; local_24 < 0x14; local_24 = local_24 + 0x1) {
                    if (0x5 < *(local_24 * 0x5e + param_4 + 0x46)) {
                        pbVar1 = (local_24 * 0x5e + param_4 + 0x4);
                        *pbVar1 = *pbVar1 & 0xdf;
                    }
                }
                local_28 = 0x6;
            }
        }
        if (local_2c == 0x2) break;
        for (local_24 = 0x0; local_24 < 0x14; local_24 = local_24 + 0x1) {
            pbVar1 = (local_24 * 0x5e + param_4 + 0x4);
            *pbVar1 = *pbVar1 & 0xdf;
        }
        local_28 = 0x0;
    }
    return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0040cd10(param_1: i32) -> u32

{
    let mut iVar1: i32;
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: i32;

    local_18 = 0x0;
    for (local_14 = 0x0; local_14 < 0x14; local_14 = local_14 + 0x1) {
        (&DAT_004c50bc)[local_14 * 0x5e] = (&DAT_004c50bc)[local_14 * 0x5e] & 0xdf;
        *(&DAT_004c50fa + local_14 * 0x5e) = 0xffffffff;
        (&DAT_004c58ec)[local_14 * 0x5e] = (&DAT_004c58ec)[local_14 * 0x5e] & 0xdf;
        *(&DAT_004c592a + local_14 * 0x5e) = 0xffffffff;
    }
    if (((DAT_004c611c == 0x1) || (DAT_004c611c == 0x4)) && ((param_1 == 0x6 || ((param_1 == 0x7 || (param_1 == 0x8))))))
    {
        local_1c = 0x0;
    }
    else {
        for (local_14 = 0x0; local_14 < 0x14; local_14 = local_14 + 0x1) {
            if ((((((((&DAT_004c50bc + local_14 * 0x5e) & 0x1) != 0x0) &&
                (((&DAT_004c50bc + local_14 * 0x5e) & 0x8) != 0x0)) &&
                (((&DAT_004c50bc + local_14 * 0x5e) & 0x80) == 0x0)) &&
                ((((&DAT_004c50bc + local_14 * 0x5e) & 0x8000) == 0x0 &&
                    (*(*(&DAT_00582938 +
                        (*(*(&DAT_004c50b8 + local_14 * 0x5e) + 0x25) >> 0x18) * 0x4 +
                        (*(*(&DAT_004c50b8 + local_14 * 0x5e) + 0x24) >> 0x18) * 0x18) +
                        param_1 * 0x4 + 0x5d) != 0x0)))) &&
                ((DAT_004c611c != 0x3 || (*(*(&DAT_004c50b8 + local_14 * 0x5e) + 0x28) >> 0x10 == 0x8))))
                && (iVar1 = FUN_0040c966(param_1,local_14,&DAT_004c50b8,&DAT_004c58e8), iVar1 != 0x0)) {
                local_18 = 0x1;
            }
        }
        if (_DAT_004c6128 == 0x0) {
            local_1c = local_18;
        }
        else {
            for (local_14 = 0x0; local_14 < 0x14; local_14 = local_14 + 0x1) {
                if ((((((&DAT_004c58ec + local_14 * 0x5e) & 0x1) != 0x0) &&
                    (((&DAT_004c58ec + local_14 * 0x5e) & 0x8) != 0x0)) &&
                    ((((&DAT_004c58ec + local_14 * 0x5e) & 0x80) == 0x0 &&
                        ((((&DAT_004c58ec + local_14 * 0x5e) & 0x8000) == 0x0 &&
                            (*(*(&DAT_00582938 +
                                (*(*(&DAT_004c58e8 + local_14 * 0x5e) + 0x25) >> 0x18) * 0x4 +
                                (*(*(&DAT_004c58e8 + local_14 * 0x5e) + 0x24) >> 0x18) * 0x18) +
                                param_1 * 0x4 + 0x5d) != 0x0)))))) &&
                    (((DAT_004c611c != 0x2 || (*(*(&DAT_004c58e8 + local_14 * 0x5e) + 0x28) >> 0x10 == 0x8))
                        && (iVar1 = FUN_0040c966(param_1,local_14,&DAT_004c58e8,&DAT_004c50b8), iVar1 != 0x0)))) {
                    local_18 = 0x1;
                }
            }
            local_1c = local_18;
        }
    }
    return local_1c;
}



fn FUN_0040d161()

{
    let mut bVar1: bool;
    let mut iVar2: i32;
    undefined3 extraout_var;
    let mut uVar3: u32;
    let mut local_64: i32;
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0x14; local_14 = local_14 + 0x1) {
        if ((((((&DAT_004c50bc + local_14 * 0x5e) & 0x1) != 0x0) &&
            (((&DAT_004c50bc + local_14 * 0x5e) & 0x8) != 0x0)) &&
            (((&DAT_004c50bc + local_14 * 0x5e) & 0x8000) == 0x0)) &&
            (((&DAT_004c50bc + local_14 * 0x5e) & 0x80) == 0x0)) {
            local_64 = *(*(&DAT_00582938 +
                (*(*(&DAT_004c50b8 + local_14 * 0x5e) + 0x25) >> 0x18) * 0x4 +
                (*(*(&DAT_004c50b8 + local_14 * 0x5e) + 0x24) >> 0x18) * 0x18) +
                0x75);
            if ((*((DAT_004c58d0 >> 0x10) * 0x1e22 + 0x569ba1) & 0x1) != 0x0) {
                local_64 = local_64 + (*(&DAT_00569b9f + (DAT_004c58d0 >> 0x10) * 0x1e22) >> 0x18);
            }
            iVar2 = FUN_004515ca(&DAT_004c50b8,0x5);
            if (((iVar2 != 0x0) &&
                (bVar1 = FUN_0046292a((&DAT_00568210 + (DAT_004c58d0 >> 0x10) * 0x1e22)),
                 CONCAT31(extraout_var,bVar1) != 0x0)) && (DAT_004d79fc >> 0x18 == DAT_004c58d0 >> 0x10)) {
                local_64 = local_64 + DAT_004d78cc;
            }
            uVar3 = FUN_004a2edc();
            if (0x9 < uVar3 % 0x14 + local_64) {
                FUN_0041116a(&DAT_004c50b8,local_14,0x6);
            }
        }
    }
    return;
}



fn FUN_0040d38b(param_1: i32)

{
    let mut bVar1: bool;
    let mut iVar2: i32;
    undefined3 extraout_var;
    let mut uVar3: u32;
    undefined3 extraout_var_00;
    let mut local_64: i32;
    let mut local_44: i32;
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0x14; local_14 = local_14 + 0x1) {
        if (*(&DAT_004c50fa + local_14 * 0x5e) != -0x1) {
            local_44 = *(param_1 * 0x4 +
                *(&DAT_00582938 +
                    (*(*(&DAT_004c50b8 + local_14 * 0x5e) + 0x25) >> 0x18) * 0x4 +
                    (*(*(&DAT_004c50b8 + local_14 * 0x5e) + 0x24) >> 0x18) * 0x18) +
                0x5d);
            if ((*((DAT_004c58d0 >> 0x10) * 0x1e22 + 0x569ba1) & 0x1) != 0x0) {
                local_44 = local_44 + (*(&DAT_00569b9f + (DAT_004c58d0 >> 0x10) * 0x1e22) >> 0x18);
            }
            iVar2 = FUN_004515ca(&DAT_004c50b8,0x5);
            if (((iVar2 != 0x0) &&
                (bVar1 = FUN_0046292a((&DAT_00568210 + (DAT_004c58d0 >> 0x10) * 0x1e22)),
                 CONCAT31(extraout_var,bVar1) != 0x0)) && (DAT_004d79fc >> 0x18 == DAT_004c58d0 >> 0x10)) {
                local_44 = local_44 + DAT_004d78cc;
            }
            iVar2 = *(&DAT_004c58f6 + *(&DAT_004c50fa + local_14 * 0x5e) * 0x5e);
            uVar3 = FUN_004a2edc();
            if ((local_44 - iVar2) + uVar3 % 0x14 < 0xa) {
                if ((DAT_004c611c == 0x2) && (uVar3 = FUN_004a2edc(), 0x9 < uVar3 % 0x14 + local_44)) {
                    FUN_0041116a(&DAT_004c50b8,local_14,param_1);
                }
            }
            else {
                FUN_0040d6e5(&DAT_004c50b8,&DAT_004c58e8,local_14,param_1);
            }
        }
        if (*(&DAT_004c592a + local_14 * 0x5e) != -0x1) {
            local_64 = *(param_1 * 0x4 +
                *(&DAT_00582938 +
                    (*(*(&DAT_004c58e8 + local_14 * 0x5e) + 0x25) >> 0x18) * 0x4 +
                    (*(*(&DAT_004c58e8 + local_14 * 0x5e) + 0x24) >> 0x18) * 0x18) +
                0x5d);
            if ((*((DAT_004c6100 >> 0x10) * 0x1e22 + 0x569ba1) & 0x1) != 0x0) {
                local_64 = local_64 + (*(&DAT_00569b9f + (DAT_004c6100 >> 0x10) * 0x1e22) >> 0x18);
            }
            iVar2 = FUN_004515ca(&DAT_004c58e8,0x5);
            if (((iVar2 != 0x0) &&
                (bVar1 = FUN_0046292a((&DAT_00568210 + (DAT_004c6100 >> 0x10) * 0x1e22)),
                 CONCAT31(extraout_var_00,bVar1) != 0x0)) && (DAT_004d79fc >> 0x18 == DAT_004c6100 >> 0x10)) {
                local_64 = local_64 + DAT_004d78cc;
            }
            iVar2 = *(&DAT_004c50c6 + *(&DAT_004c592a + local_14 * 0x5e) * 0x5e);
            uVar3 = FUN_004a2edc();
            if (0x9 < (local_64 - iVar2) + uVar3 % 0x14) {
                FUN_0040d6e5(&DAT_004c58e8,&DAT_004c50b8,local_14,param_1);
            }
        }
    }
    return;
}



fn FUN_0040d6e5(param_1: i32,param_2: i32,param_3: i32,param_4: i32)

{
    let piVar1: *mut i32;;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let mut uVar6: u32;
    let mut local_20: i32;

    iVar2 = *(param_3 * 0x5e + param_1 + 0x42);
    iVar3 = *(param_2 + 0x818) >> 0x10;
    iVar5 = param_4;
    iVar4 = FUN_004515ca(param_1,0x6);
    iVar5 = FUN_0040bd7e(*(param_3 * 0x5e + param_1),iVar4,iVar5,iVar3);
    if (DAT_004c6114 == 0x0) {
        uVar6 = 0xffffffff;
        iVar3 = FUN_0045172c(param_2,0x2d);
        iVar4 = FUN_004515ca(param_2,0x4);
        local_20 = FUN_0040bf3c(*(iVar2 * 0x5e + param_2),iVar4,iVar3,uVar6);
    }
    else {
        uVar6 = *(DAT_004c6114 + 0xc) >> 0x10;
        iVar3 = FUN_0045172c(param_2,0x2d);
        iVar4 = FUN_004515ca(param_2,0x4);
        local_20 = FUN_0040bf3c(*(iVar2 * 0x5e + param_2),iVar4,iVar3,uVar6);
    }
    iVar5 = FUN_0040bc1e(iVar5,local_20);
    uVar6 = FUN_004a2edc();
    iVar5 = *(&DAT_00591964 + iVar5 * 0x4 + (uVar6 % 0xa) * 0x30);
    piVar1 = (iVar2 * 0x5e + param_2 + 0x16);
    *piVar1 = *piVar1 + iVar5;
    piVar1 = (iVar2 * 0x5e + param_2 + 0x12);
    *piVar1 = *piVar1 + iVar5;
    piVar1 = (param_4 * 0x4 + iVar2 * 0x5e + param_2 + 0x1a);
    *piVar1 = *piVar1 + iVar5;
    return;
}



fn FUN_0040d820(param_1: u32)

{
    let mut iVar1: i32;
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0x14; local_14 = local_14 + 0x1) {
        if (*(&DAT_004c50ca + local_14 * 0x5e) != 0x0) {
            iVar1 = *(*(&DAT_004c50b8 + local_14 * 0x5e) + 0x30) >> 0x18;
            if (iVar1 == *(&DAT_004c50ce + local_14 * 0x5e) || iVar1 - *(&DAT_004c50ce + local_14 * 0x5e) < 0x0)
            {
                *(&DAT_004c50f6 + local_14 * 0x5e) = param_1;
                (&DAT_004c50bd)[local_14 * 0x5e] = (&DAT_004c50bd)[local_14 * 0x5e] | 0x80;
                (&DAT_004c50bc)[local_14 * 0x5e] = (&DAT_004c50bc)[local_14 * 0x5e] & 0xbf;
                (&DAT_004c50bc)[local_14 * 0x5e] = (&DAT_004c50bc)[local_14 * 0x5e] & 0x7f;
            }
            *(&DAT_004c50ca + local_14 * 0x5e) = 0x0;
        }
        if (*(&DAT_004c58fa + local_14 * 0x5e) != 0x0) {
            iVar1 = *(*(&DAT_004c58e8 + local_14 * 0x5e) + 0x30) >> 0x18;
            if (iVar1 == *(&DAT_004c58fe + local_14 * 0x5e) || iVar1 - *(&DAT_004c58fe + local_14 * 0x5e) < 0x0)
            {
                *(&DAT_004c5926 + local_14 * 0x5e) = param_1;
                (&DAT_004c58ed)[local_14 * 0x5e] = (&DAT_004c58ed)[local_14 * 0x5e] | 0x80;
                (&DAT_004c58ec)[local_14 * 0x5e] = (&DAT_004c58ec)[local_14 * 0x5e] & 0xbf;
                (&DAT_004c58ec)[local_14 * 0x5e] = (&DAT_004c58ec)[local_14 * 0x5e] & 0x7f;
            }
            *(&DAT_004c58fa + local_14 * 0x5e) = 0x0;
        }
    }
    return;
}



fn FUN_0040d920() -> u32

{
    let local_138: *mut i32;;
    let local_134: *mut i32;;
    let mut local_12c: *mut u32 [0x11];
    let ppuStack231: *mut *mut u8;;
    let mut local_3b: String;;
    let local_34: *mut i32;;
    let mut local_30: u32;
    let local_2c: *mut i32;;
    let local_28: *mut i32;;
    let local_24: *mut i32;;
    i32 **local_20;
    i32 **local_1c;
    let mut local_18: u32;

    DAT_004c50b4 = FUN_0049c2c9(0xe80);
    if (DAT_004c50b4 == 0x0) {
        local_30 = 0x0;
    }
    else {
        FUN_0049c57b(s_bin_fire_bin_004c0883,DAT_004c50b4,0xe80);
        FUN_004990e0(local_12c,0x0,s_efs_res_004c089a,s_CombatDlg_004c0890);
        local_134 = FUN_004a2831(0xb9);
        local_2c = local_134;
        local_28 = local_134;
        if (local_134 != 0x0) {
            local_134 = FUN_00438792(local_134,local_12c,0x1f4,0x114,0x118,0x58,0x3f,0x0,0x2,0x2,DAT_004c58ca >> 0x10);
        }
        DAT_005b8c38 = local_134;
        local_138 = FUN_004a2831(0x95);
        local_34 = local_138;
        local_24 = local_138;
        if (local_138 != 0x0) {
            local_138 = FUN_0047157e(local_138,local_12c,0x1f5,0x10e,0xa0,0x64,0x60,0x40,0x2);
        }
        DAT_005b8c3c = local_138;
        FUN_0049bf40(local_12c,DAT_005b8c38);
        FUN_0049bf40(local_12c,DAT_005b8c3c);
        local_20 = (i32 **)FUN_004a2831(0x10);
        local_1c = local_20;
        if (local_20 != (i32 **)0x0) {
            FUN_004a2874(local_20,local_12c,0x96);
        }
        local_18 = FUN_0049bb50(local_12c,FUN_0040dc25);
        FUN_004a2965(local_12c);
        if (DAT_005b8c38 != 0x0) {
            ((*(DAT_005b8c38 + 0x45) + 0x8))(DAT_005b8c38,0x2);
        }
        if (DAT_005b8c3c != 0x0) {
            ((*(DAT_005b8c3c + 0x45) + 0x8))(DAT_005b8c3c,0x2);
        }
        FUN_0049af50(DAT_004c50b4);
        local_30 = local_18;
        ppuStack231 = &PTR_FUN_004c3d34;
        if (local_3b != 0x0) {
            FUN_00499b30(local_12c,local_3b);
        }
        FUN_0049a1c0(local_12c,0x1);
    }
    return local_30;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0040dc25(param_1: *mut *mut u32,param_2: u32,param_3: i32,param_4: i32) -> u32

{
    i32 **ppiVar1;
    let local_60: *mut i32; [0x6];
    let local_48: *mut u32;
    let mut local_44: i32;
    let mut local_40: i32;
    let mut local_3c: i32;
    let mut local_38: i32;
    let mut local_34: u32;
    let mut local_30: i32;
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: u32;
    let mut local_18: u32;

    local_18 = param_2;
    if (param_2 < 0x407) {
        if (0x112 < param_2) {
            if (param_2 < 0x114) {
                if (param_4 == 0x2710) {
                    FUN_0049c140(param_1,0x0);
                    return 0x0;
                }
                if (param_4 != 0x3e8) {
                    if (param_4 == 0x96) {
                        _DAT_004c50b0 = _DAT_004c50b0 ^ 0x1;
                        if (((DAT_004c611c == 0x1) || (DAT_004c611c == 0x4)) || (DAT_004c611c == 0x2)) {
                            FUN_00447607(DAT_005b8c38,DAT_004c60fc._2_2_,DAT_004c6100);
                            if (_DAT_004c50b0 == 0x0) {
                                local_20 = 0xe0e0e;
                            }
                            else {
                                local_20 = 0x272727;
                            }
                            local_24 = DAT_005b8c38;
                            local_28 = *(DAT_005b8c38 + 0x89);
                            local_2c = DAT_005b8c38;
                            local_30 = *(DAT_005b8c38 + 0x85);
                            FUN_004968e7(local_30,local_28,0x2,0x2,(char)local_20);
                        }
                        else {
                            FUN_0047d83f(DAT_005b8c3c,DAT_004c60fc._2_2_,DAT_004c6100);
                            if (_DAT_004c50b0 == 0x0) {
                                local_34 = 0xe0e0e;
                            }
                            else {
                                local_34 = 0x272727;
                            }
                            local_38 = DAT_005b8c3c;
                            local_3c = *(DAT_005b8c3c + 0x89);
                            local_40 = DAT_005b8c3c;
                            local_44 = *(DAT_005b8c3c + 0x85);
                            FUN_004968e7(local_44,local_3c,0x2,0x2,(char)local_34);
                        }
                        return 0x0;
                    }
                    return 0x0;
                }
                FUN_004a15ad();
                return 0x0;
            }
            if (param_2 == 0x401) {
                _DAT_004c50b0 = 0x0;
                FUN_0049bf80(param_1,0x64,0x414,0x0,0x0);
                FUN_0049bf80(param_1,0x64,0x410,0x0,0x0);
                FUN_0049bf80(param_1,0x6f,0x414,0x0,0x0);
                FUN_0049bf80(param_1,0x6f,0x410,0x0,0x0);
                FUN_0049f99b(DAT_005b9bd4,param_1,0x407,0x3039,0x0);
            }
        }
    }
    else {
        if (param_2 < 0x408) {
            if (param_3 == 0x3039) {
                FUN_004953d7();
                FUN_0040ec2e();
                FUN_0049536f();
                ppiVar1 = (i32 **)FUN_004a2831(0x10);
                if (ppiVar1 != (i32 **)0x0) {
                    FUN_004a2874(ppiVar1,param_1,0x3e8);
                }
                FUN_0049bf80(param_1,0x64,0x40f,0x0,0x0);
                FUN_0049bf80(param_1,0x64,0x414,0x1,0x0);
                if ((DAT_004c611c == 0x2) && (_DAT_004c6130 != 0x0)) {
                    FUN_0049bf80(param_1,0x6f,0x40f,0x0,0x0);
                    FUN_0049bf80(param_1,0x6f,0x414,0x1,0x0);
                }
                if ((((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) != 0x0) &&
                    (ppiVar1 = (i32 **)FUN_004a2831(0x10), ppiVar1 != (i32 **)0x0)) {
                FUN_004a2874(ppiVar1,param_1,0x2710);
                }
            }
            else {
                if ((_DAT_004c6130 == 0x0) || (param_3 != 0x6f)) {
                    if (param_3 == 0x64) {
                        FUN_0049c140(param_1,0x0);
                    }
                }
                else {
                    FUN_0049c140(param_1,0x1);
                }
            }
        }
        else {
            if (0x40b < param_2) {
                if (param_2 < 0x40d) {
                    FUN_004953d7();
                    local_48 = FUN_0049c2c9(0x4b000);
                    FUN_00498ba4(local_60,local_48,0x1e0,0x280);
                    FUN_0040e76e();
                    FUN_0040eba9();
                    FUN_0049e640(*(DAT_005b8c38 + 0x1d),*(DAT_005b8c38 + 0x21),*(DAT_005b8c38 + 0x25),
                                 *(DAT_005b8c38 + 0x29),0xce,0xca,0xcc,0x1);
                    FUN_0049e640(*(DAT_005b8c3c + 0x1d),*(DAT_005b8c3c + 0x21),*(DAT_005b8c3c + 0x25),
                                 *(DAT_005b8c3c + 0x29),0xce,0xca,0xcc,0x1);
                    FUN_00497567(0x140,0x10b,(&DAT_005b709e + (DAT_004c58ca >> 0x10) * 0x4e),0x78,0xcaccce,-0x1,0xcaccce,
                                 DAT_004d6a6c,0x11);
                    FUN_00452328(&DAT_004c50b8);
                    FUN_00452328(&DAT_004c58e8);
                    FUN_00498cf4(local_60);
                    FUN_00496ac0(local_48,0x0,0x0,0x280,0x1e0);
                    FUN_0049af50(local_48);
                    FUN_0049536f();
                    return 0x1;
                }
                if (param_2 == 0x415) {
                    FUN_004a15ad();
                    return 0x0;
                }
            }
        }
    }
    return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0040e175(param_1: i32,param_2: i32,param_3: i32,param_4: i32)

{
    let mut bVar1: bool;
    undefined3 extraout_var;
    let mut pcVar2: String;
    let mut iVar3: i32;
    let mut puVar4: *mut u8;
    let mut iVar5: i32;
    let mut puVar6: *mut u8;
    let puVar7: *mut u32;
    let bVar8: u8;
    let local_a4: u8 [0x80];
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    local_24 = 0xa;
    bVar1 = FUN_0046292a((&DAT_00568210 + (*(param_1 + 0x818) >> 0x10) * 0x1e22));
    local_20 = CONCAT31(extraout_var,bVar1);
    local_1c = 0x0;
    local_18 = 0x0;
    for (local_14 = 0x0; local_14 < 0x14; local_14 = local_14 + 0x1) {
        if ((((local_14 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) &&
            (((local_14 * 0x5e + param_1 + 0x4) & 0x2) != 0x0)) {
            if (*(local_14 * 0x5e + param_1 + 0x52) == 0x2d) {
                local_1c = 0x1;
            }
            else {
                if (*(local_14 * 0x5e + param_1 + 0x52) == 0x33) {
                    local_18 = 0x1;
                }
            }
        }
    }
    if (local_1c == 0x0) {
        if (local_18 != 0x0) {
            bVar8 = 0x10;
            puVar6 = &DAT_00535557;
            iVar5 = -0x1;
            puVar4 = &DAT_00535557;
            iVar3 = 0xc8;
            puVar7 = DAT_004d6a6c;
            pcVar2 = FUN_00499050(DAT_0059679c,0x7416);
            FUN_00497567(param_3,param_4,pcVar2,iVar3,puVar4,iVar5,puVar6,puVar7,bVar8);
            param_4 = param_4 + local_24;
        }
    }
    else {
        bVar8 = 0x10;
        puVar6 = &DAT_00535557;
        iVar5 = -0x1;
        puVar4 = &DAT_00535557;
        iVar3 = 0xc8;
        puVar7 = DAT_004d6a6c;
        pcVar2 = FUN_00499050(DAT_0059679c,0x7415);
        FUN_00497567(param_3,param_4,pcVar2,iVar3,puVar4,iVar5,puVar6,puVar7,bVar8);
        param_4 = param_4 + local_24;
        if ((*((*(param_1 + 0x818) >> 0x10) * 0x1e22 + 0x569b92) & 0x1) != 0x0) {
            bVar8 = 0x10;
            puVar6 = &DAT_00535557;
            iVar5 = -0x1;
            puVar4 = &DAT_00535557;
            iVar3 = 0xc8;
            puVar7 = DAT_004d6a6c;
            pcVar2 = FUN_00499050(DAT_0059679c,0x7417);
            FUN_00497567(param_3,param_4,pcVar2,iVar3,puVar4,iVar5,puVar6,puVar7,bVar8);
            param_4 = param_4 + local_24;
        }
    }
    if ((param_2 + 0x81a) == 0x8) {
        if (((&DAT_00569e55)[(*(param_1 + 0x818) >> 0x10) * 0x1e22] & 0x1) != 0x0) {
            bVar8 = 0x10;
            puVar6 = &DAT_00535557;
            iVar5 = -0x1;
            puVar4 = &DAT_00535557;
            iVar3 = 0xc8;
            puVar7 = DAT_004d6a6c;
            pcVar2 = FUN_00499050(DAT_0059679c,0x7421);
            FUN_00497567(param_3,param_4,pcVar2,iVar3,puVar4,iVar5,puVar6,puVar7,bVar8);
            param_4 = param_4 + local_24;
        }
    }
    else {
        if (((param_2 + 0x81a) == 0x7) &&
            (((&DAT_00569e5e)[(*(param_1 + 0x818) >> 0x10) * 0x1e22] & 0x1) != 0x0)) {
            bVar8 = 0x10;
            puVar6 = &DAT_00535557;
            iVar5 = -0x1;
            puVar4 = &DAT_00535557;
            iVar3 = 0xc8;
            puVar7 = DAT_004d6a6c;
            pcVar2 = FUN_00499050(DAT_0059679c,0x7422);
            FUN_00497567(param_3,param_4,pcVar2,iVar3,puVar4,iVar5,puVar6,puVar7,bVar8);
            param_4 = param_4 + local_24;
        }
    }
    if ((*((*(param_1 + 0x818) >> 0x10) * 0x1e22 + 0x569ba1) & 0x1) != 0x0) {
        bVar8 = 0x10;
        puVar6 = &DAT_00535557;
        iVar5 = -0x1;
        puVar4 = &DAT_00535557;
        iVar3 = 0xc8;
        puVar7 = DAT_004d6a6c;
        pcVar2 = FUN_00499050(DAT_0059679c,0x7418);
        FUN_00497567(param_3,param_4,pcVar2,iVar3,puVar4,iVar5,puVar6,puVar7,bVar8);
        param_4 = param_4 + local_24;
    }
    if (((_DAT_004d792e >> 0x18 == *(param_1 + 0x818) >> 0x10) && ((*(param_1 + 0x828) & 0x8) != 0x0)) &&
        (local_20 != 0x0)) {
        pcVar2 = FUN_00499050(DAT_0059679c,0x7419);
        FUN_0049c2e0(local_a4,pcVar2);
        FUN_00497567(param_3,param_4,local_a4,0xc8,&DAT_00535557,-0x1,&DAT_00535557,DAT_004d6a6c,0x10);
        param_4 = param_4 + local_24;
    }
    if (((DAT_004d7995 >> 0x18 == *(param_1 + 0x818) >> 0x10) && ((*(param_1 + 0x828) & 0x10) != 0x0)) &&
        (local_20 != 0x0)) {
        pcVar2 = FUN_00499050(DAT_0059679c,0x7419);
        FUN_0049c2e0(local_a4,pcVar2);
        FUN_00497567(param_3,param_4,local_a4,0xc8,&DAT_00535557,-0x1,&DAT_00535557,DAT_004d6a6c,0x10);
        param_4 = param_4 + local_24;
    }
    if (((DAT_004d79fc >> 0x18 == *(param_1 + 0x818) >> 0x10) && ((*(param_1 + 0x828) & 0x20) != 0x0)) &&
        (local_20 != 0x0)) {
        pcVar2 = FUN_00499050(DAT_0059679c,0x7419);
        FUN_0049c2e0(local_a4,pcVar2);
        FUN_00497567(param_3,param_4,local_a4,0xc8,&DAT_00535557,-0x1,&DAT_00535557,DAT_004d6a6c,0x10);
    }
    return;
}


fn FUN_00452646(param_1: i32,param_2: i32,param_3: i32) -> u32

{
    byte *pbVar1;
    let mut iVar2: i32;
    u32 local_34 [0x2];
    let mut local_2c: u32;
    let mut local_28: i32;
    let mut local_24: u32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    if ((*(*(&DAT_004d7d50 + param_2 * 0x4 + (*(param_1 + 0x812) >> 0x10) * 0x3890) + param_3 * 0x4)
        & 0xf) == 0x0) {
        local_24 = 0x0;
    }
    else {
        local_20 = FUN_0044ace5(*(param_1 + 0x812) >> 0x10,param_2,param_3,0x1);
        if ((local_20 == -0x1) || (local_20 == DAT_004c9754)) {
            local_1c = FUN_00451c91(param_1);
            local_18 = FUN_00485ea2(*(param_1 + 0x812) >> 0x10,param_2,param_3,0x1);
            if (local_1c + local_18 < 0x15) {
                FUN_00431d31(local_34);
                FUN_00431d31(&local_2c);
                FUN_00431d0a(local_34);
                FUN_00431d0a(&local_2c);
                for (local_14 = 0x0; local_14 < 0x14; local_14 = local_14 + 0x1) {
                    if ((((((local_14 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) &&
                        (((local_14 * 0x5e + param_1 + 0x4) & 0x2) != 0x0)) &&
                        ((*(*(local_14 * 0x5e + param_1) + 0x3a) & 0x40) != 0x0)) &&
                        (iVar2 = FUN_00409fc6(param_1,local_14), iVar2 != -0x1)) {
                        *((local_14 - iVar2) * 0x4 + *(iVar2 * 0x5e + param_1) + 0xc) = 0x0;
                        pbVar1 = (*(local_14 * 0x5e + param_1) + 0x3a);
                        *pbVar1 = *pbVar1 & 0xbf;
                        for (local_28 = *(local_14 * 0x5e + param_1); (local_28 != 0x0 && (*(local_28 + 0xc) != 0x0));
                            local_28 = *(local_28 + 0xc)) {
                        }
                        if (local_28 != 0x0) {
                            FUN_00431dec(&local_2c,*(local_14 * 0x5e + param_1));
                        }
                        FUN_00431efd(local_34,*(local_14 * 0x5e + param_1));
                        *(local_14 * 0x5e + param_1) = 0x0;
                        pbVar1 = (local_14 * 0x5e + param_1 + 0x4);
                        *pbVar1 = *pbVar1 & 0xfe;
                    }
                }
                FUN_00434129(local_34,param_2,param_3);
                local_24 = 0x1;
            }
            else {
                local_24 = 0x0;
            }
        }
        else {
            local_24 = 0x0;
        }
    }
    return local_24;
}



fn FUN_0045294f(param_1: i32,param_2: i32,param_3: i32) -> i32

{
let mut local_1c: i32;
let mut local_18: i32;
let mut local_14: i32;

if ((*(param_1 + 0x828) & 0x1) == 0x0) {
local_18 = 0x0;
for (local_14 = 0x0; local_14 < 0x14; local_14 = local_14 + 0x1) {
if (((((local_14 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) &&
(((param_3 == 0x0 || (((local_14 * 0x5e + param_1 + 0x4) & 0x2) != 0x0)) &&
((*(local_14 * 0x5e + param_1) + 0x27) == '[')))) &&
(*(*(local_14 * 0x5e + param_1) + 0x2d) >> 0x18 == param_2)) {
local_18 = local_18 + (*(*(local_14 * 0x5e + param_1) + 0x2f) >> 0x10);
}
}
local_1c = local_18;
}
else {
local_1c = 0x0;
}
return local_1c;
}



fn FUN_00452a43(param_1: i32) -> u32

{
    let mut local_14: i32;

    local_14 = 0x0;
    while( true ) {
        if (0x13 < local_14) {
            return 0x0;
        }
        if (((local_14 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) break;
        local_14 = local_14 + 0x1;
    }
    return *(local_14 * 0x5e + param_1);
}



fn FUN_00452ac3(param_1: i32)

{
    byte *pbVar1;
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0x14; local_14 = local_14 + 0x1) {
        if (((local_14 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) {
            pbVar1 = (*(local_14 * 0x5e + param_1) + 0x3a);
            *pbVar1 = *pbVar1 & 0xfb;
        }
    }
    return;
}



fn FUN_00452b26(param_1: i32,param_2: i32) -> i32

{
let mut local_18: i32;
let mut local_14: i32;

local_18 = 0x0;
for (local_14 = 0x0; local_14 < 0x14; local_14 = local_14 + 0x1) {
if (((((local_14 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) &&
(((local_14 * 0x5e + param_1 + 0x4) & 0x2) != 0x0)) &&
((param_2 == 0x0 || ((*(*(local_14 * 0x5e + param_1) + 0x3a) & 0x40) == 0x0)))) {
local_18 = local_18 + 0x1;
}
}
return local_18;
}



fn FUN_00452bf8(param_1: i32,param_2: i32,param_3: i32) -> u32

{
    let mut local_1c: i32;
    let mut local_18: u32;

    local_18 = 0x1;
    if (*(*(&DAT_00582938 + param_3 * 0x4 + param_2 * 0x18) + 0xed) != -0x1) {
        if ((*(param_1 + 0x828) & 0x1) != 0x0) {
            return 0x0;
        }
        local_18 = 0x0;
        for (local_1c = 0x0; local_1c < 0x14; local_1c = local_1c + 0x1) {
            if (((((local_1c * 0x5e + param_1 + 0x4) & 0x1) != 0x0) &&
                (*(*(local_1c * 0x5e + param_1) + 0x24) >> 0x18 ==
                    *(*(&DAT_00582938 + param_3 * 0x4 + param_2 * 0x18) + 0xed))) &&
                (*(*(local_1c * 0x5e + param_1) + 0x25) >> 0x18 == 0x0)) {
                return 0x1;
            }
        }
    }
    return local_18;
}



fn FUN_00452d01(param_1: i32) -> *mut u32

{
    let unaff_ESI: *mut u32;
    let mut local_1c: u32;
    let mut uStack24: u32;
    let mut local_10: i32;

    FUN_00431d31(&local_1c);
    FUN_00431d0a(&local_1c);
    for (local_10 = 0x0; local_10 < 0x14; local_10 = local_10 + 0x1) {
        if ((((*(local_10 * 0x5e + param_1) == 0x0) ||
            ((*(*(local_10 * 0x5e + param_1) + 0x3a) & 0x80000000) == 0x0)) &&
            (((local_10 * 0x5e + param_1 + 0x4) & 0x1) != 0x0)) &&
            ((((local_10 * 0x5e + param_1 + 0x4) & 0x2) != 0x0 &&
                ((*(*(local_10 * 0x5e + param_1) + 0x3a) & 0x4) == 0x0)))) {
            FUN_00431efd(&local_1c,*(local_10 * 0x5e + param_1));
        }
    }
        *unaff_ESI = local_1c;
    unaff_ESI[0x1] = uStack24;
    return unaff_ESI;
}



fn FUN_00452e41(param_1: i32) -> u32

{
    let mut local_14: i32;

    local_14 = 0x0;
    while( true ) {
        if (0x13 < local_14) {
            return 0x1;
        }
        if ((((local_14 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) &&
            ((*(&DAT_004be9b0 + DAT_004c9754 * 0x4) & *(*(local_14 * 0x5e + param_1) + 0x3a)) != 0x0))
        break;
        local_14 = local_14 + 0x1;
    }
    return 0x0;
}



bool  FUN_00452ec6(param_1: i32)

{
let mut bVar1: bool;
let mut local_14: i32;

bVar1 = true;
local_14 = 0x0;
while( true ) {
if (0x13 < local_14) {
return !bVar1;
}
if ((((local_14 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) &&
(bVar1 = false, (*(*(local_14 * 0x5e + param_1) + 0x3a) & 0x4) == 0x0)) break;
local_14 = local_14 + 0x1;
}
return false;
}



fn FUN_00452f5b(param_1: i32)

{
    byte *pbVar1;
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0x14; local_14 = local_14 + 0x1) {
        if (((local_14 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) {
            pbVar1 = (*(local_14 * 0x5e + param_1) + 0x3a);
            *pbVar1 = *pbVar1 & 0xfb;
            pbVar1 = (local_14 * 0x5e + param_1 + 0x4);
            *pbVar1 = *pbVar1 | 0x2;
        }
    }
    return;
}



fn FUN_00452fc9(param_1: i32)

{
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0x2002; local_14 = local_14 + 0x1) {
        *(&DAT_0059aaac + local_14 * 0xc) = 0x0;
        *(&DAT_0059aab0 + local_14 * 0xc) = *(&DAT_0059aaac + local_14 * 0xc);
        *(&DAT_0059aaa8 + local_14 * 0xc) = *(&DAT_0059aab0 + local_14 * 0xc);
    }
    DAT_005b2abc = param_1;
    *(&DAT_0059aaa8 + param_1 * 0xc) = 0x2001;
    *(&DAT_0059aab0 + param_1 * 0xc) = 0x0;
    *(&DAT_0059aaac + param_1 * 0xc) = 0x0;
    return;
}



fn FUN_00453057(param_1: i32,param_2: i32)

{
    *(&DAT_0059aaa8 + param_2 * 0xc) = *(&DAT_0059aaa8 + param_1 * 0xc);
    if (*(&DAT_0059aab0 + *(&DAT_0059aaa8 + param_1 * 0xc) * 0xc) == param_1) {
        *(&DAT_0059aab0 + *(&DAT_0059aaa8 + param_1 * 0xc) * 0xc) = param_2;
    }
    else {
        *(&DAT_0059aaac + *(&DAT_0059aaa8 + param_1 * 0xc) * 0xc) = param_2;
    }
    *(&DAT_0059aaa8 + param_1 * 0xc) = 0x0;
    return;
}



fn FUN_004530cd(param_1: i32,param_2: i32)

{
    let mut iVar1: i32;
    let mut iVar2: i32;

    iVar1 = *(&DAT_0059aaa8 + param_1 * 0xc);
    if (*(&DAT_0059aaac + iVar1 * 0xc) == param_1) {
        *(&DAT_0059aaac + iVar1 * 0xc) = param_2;
    }
    else {
        *(&DAT_0059aab0 + iVar1 * 0xc) = param_2;
    }
    iVar1 = param_1 * 0xc;
    iVar2 = param_2 * 0xc;
    *(&DAT_0059aaa8 + iVar2) = *(&DAT_0059aaa8 + iVar1);
    *(&DAT_0059aaac + iVar2) = *(&DAT_0059aaac + iVar1);
    *(&DAT_0059aab0 + iVar2) = *(&DAT_0059aab0 + iVar1);
    *(&DAT_0059aaa8 + *(&DAT_0059aaac + param_2 * 0xc) * 0xc) = param_2;
    *(&DAT_0059aaa8 + *(&DAT_0059aab0 + param_2 * 0xc) * 0xc) = param_2;
    *(&DAT_0059aaa8 + param_1 * 0xc) = 0x0;
    return;
}



fn FUN_00453166(param_1: i32) -> i32

{
let mut local_14: i32;

for (local_14 = *(&DAT_0059aaac + param_1 * 0xc); *(&DAT_0059aab0 + local_14 * 0xc) != 0x0;
local_14 = *(&DAT_0059aab0 + local_14 * 0xc)) {
}
return local_14;
}



fn FUN_004531ac(param_1: i32)

{
    let mut iVar1: i32;

    if (*(&DAT_0059aaa8 + param_1 * 0xc) != 0x0) {
        if (*(&DAT_0059aab0 + param_1 * 0xc) == 0x0) {
            FUN_00453057(param_1,*(&DAT_0059aaac + param_1 * 0xc));
        }
        else {
            if (*(&DAT_0059aaac + param_1 * 0xc) == 0x0) {
                FUN_00453057(param_1,*(&DAT_0059aab0 + param_1 * 0xc));
            }
            else {
                iVar1 = FUN_00453166(param_1);
                FUN_004531ac(iVar1);
                FUN_004530cd(param_1,iVar1);
            }
        }
    }
    return;
}



fn FUN_00453247(param_1: i32,param_2: *mut i32) -> i32

{
let mut local_28: i32;
let mut local_24: i32;
let mut local_20: i32;
let mut local_1c: i32;
let mut local_18: i32;
let local_14: *mut i32;;

if (param_1 == 0x0) {
local_28 = 0x0;
}
else {
local_20 = DAT_005b2abc;
local_18 = 0x0;
while( true ) {
local_24 = 0x0;
while ((local_24 < 0x9 &&
(local_1c = *((param_1 + local_24 & 0x1fffU) + DAT_0059aaa0) -
*((local_20 + local_24 & 0x1fffU) + DAT_0059aaa0), local_1c == 0x0))) {
local_24 = local_24 + 0x1;
}
if (local_18 <= local_24) {
local_18 = local_24;
*param_2 = local_20;
if (0x8 < local_24) {
FUN_004530cd(local_20,param_1);
return local_24;
}
}
if (local_1c < 0x0) {
local_14 = (&DAT_0059aaac + local_20 * 0xc);
}
else {
local_14 = (&DAT_0059aab0 + local_20 * 0xc);
}
if (*local_14 == 0x0) break;
local_20 = *local_14;
}
*local_14 = param_1;
*(&DAT_0059aaa8 + param_1 * 0xc) = local_20;
*(&DAT_0059aab0 + param_1 * 0xc) = 0x0;
*(&DAT_0059aaac + param_1 * 0xc) = 0x0;
local_28 = local_18;
}
return local_28;
}



fn FUN_00453383(param_1: *mut i32)

{
    let mut local_14: i32;

    FUN_004a9080(DAT_005b2ac0,param_1);
    for (local_14 = 0x0; local_14 < DAT_005b2cbc; local_14 = local_14 + 0x1) {
        FUN_004a9080(*(&DAT_005b2ac1 + local_14 * 0x4) >> 0x18,param_1);
        if ((&DAT_005b2c58)[local_14] == '\x02') {
            FUN_004a9080(*(&DAT_005b2ac2 + local_14 * 0x4) >> 0x18,param_1);
        }
    }
    DAT_005b2ac0 = 0x0;
    DAT_005b2ac1 = 0x1;
    DAT_005b2cbc = 0x0;
    return;
}



fn FUN_00453423(param_1: *mut i32,param_2: i32)

{
    if (DAT_005b2ac1 == '\0') {
        FUN_00453383(param_1);
    }
    if (param_2 != 0x0) {
        DAT_005b2ac0 = DAT_005b2ac0 + DAT_005b2ac1;
    }
    DAT_005b2ac1 = DAT_005b2ac1 << 0x1;
    return;
}



fn FUN_00453463(param_1: u32,undefined param_2)

{
    let mut iVar1: i32;

    *(&DAT_005b2ac4 + DAT_005b2cbc * 0x4) = param_1;
    iVar1 = DAT_005b2cbc;
    DAT_005b2cbc = DAT_005b2cbc + 0x1;
    (&DAT_005b2c58)[iVar1] = param_2;
    return;
}



fn FUN_0045349e(byte **param_1,param_2: *mut i32,param_3: i32)

{
    byte *pbVar1;
    let mut in_stack_ffffffa0: i32;
    let mut uVar2: u32;
    let mut local_5c: u32;
    let mut local_58: u32;
    let mut local_54: u32;
    let mut local_50: i32;
    let local_4c: u8;
    let mut local_48: i32;
    let mut local_44: u32;
    let mut local_40: i32;
    let mut local_3c: i32;
    let mut local_38: i32;
    let mut local_34: i32;
    let mut local_30: i32;
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: u32;
    let mut local_1c: i32;
    let local_18: u32;
    let mut local_14: u32;

    local_44 = 0x1;
    local_3c = 0x0;
    local_38 = 0x0;
    local_34 = 0x0;
    local_30 = 0x0;
    local_2c = 0x0;
    local_28 = 0x0;
    local_24 = 0x0;
    local_1c = 0x0;
    local_18 = FUN_004a91d0(param_1[0x4]);
    DAT_005b2cbc = 0x0;
    DAT_005b2ac0 = 0x0;
    DAT_005b2ac1 = 0x1;
    FUN_004a0430(&DAT_005b2ac4,0x0,0x64);
    FUN_004a0430(&DAT_005b2c58,0x0,0x64);
    FUN_004a7160(&local_18,0x4,0x1,param_2);
    for (local_50 = 0x0; local_50 < 0x9; local_50 = local_50 + 0x1) {
        if ((param_1[0x1] < 0x1) || (**param_1 - 0xd < 0xfe)) {
            local_54 = FUN_004a9250(param_1);
        }
        else {
            param_1[0x1] = param_1[0x1] + -0x1;
            pbVar1 = *param_1;
            *param_1 = *param_1 + 0x1;
            local_54 = *pbVar1;
        }
        if (local_54 == 0xffffffff) break;
        local_1c = local_1c + 0x1;
        local_4c = local_54;
        (DAT_0059aaa0 + local_50 + 0x1) = local_4c;
    }
    local_48 = local_50;
    FUN_004a2ff0(0x0,s_Saving_game____004c1d59,0x64,in_stack_ffffffa0);
    FUN_00452fc9(0x1);
    while (-0x2 < local_48) {
        if (param_3 != 0x0) {
            local_20 = (local_1c * 0x64) / local_18;
            FUN_004a36b0(local_20);
        }
        if ((local_3c < 0x2) || ((local_3c < local_34 && (local_2c <= local_34)))) {
            local_40 = 0x1;
            FUN_00453423(param_2,0x1);
            FUN_00453463(*((local_44 - local_24 & 0x1fff) + DAT_0059aaa0),0x1);
        }
        else {
            FUN_00453423(param_2,0x0);
            local_14 = local_14 & 0xffff0000 | ((local_44 - local_38) - 0x2 & 0x1fff) << 0x3 | local_3c - 0x2U & 0x7;
            FUN_00453463(local_14,0x2);
            local_40 = local_3c;
        }
        for (local_50 = local_24; local_50 < local_40 + 0x2; local_50 = local_50 + 0x1) {
            local_24 = 0x2;
            local_3c = local_34;
            local_38 = local_30;
            local_34 = local_2c;
            local_30 = local_28;
            local_58 = local_44 + 0x9 & 0x1fff;
            if (local_58 == 0x0) {
                local_58 = 0x2000;
            }
            FUN_004531ac(local_58);
            if ((param_1[0x1] < 0x1) || (**param_1 - 0xd < 0xfe)) {
                local_5c = FUN_004a9250(param_1);
            }
            else {
                param_1[0x1] = param_1[0x1] + -0x1;
                pbVar1 = *param_1;
                *param_1 = *param_1 + 0x1;
                local_5c = *pbVar1;
            }
            if (local_5c == 0xffffffff) {
                local_48 = local_48 + -0x1;
            }
            else {
                local_1c = local_1c + 0x1;
                local_4c = local_5c;
                ((local_44 + 0x9 & 0x1fff) + DAT_0059aaa0) = local_4c;
            }
            local_44 = local_44 + 0x1 & 0x1fff;
            if (0x0 < local_48) {
                uVar2 = local_44;
                if (local_44 == 0x0) {
                    uVar2 = 0x2000;
                }
                local_2c = FUN_00453247(uVar2,&local_28);
                if (local_48 < local_2c) {
                    local_2c = local_48;
                }
            }
        }
    }
    FUN_00453423(param_2,0x0);
    local_14 = local_14 & 0xffff0007;
    FUN_00453463(local_14,0x2);
    FUN_00453383(param_2);
    FUN_004a3800();
    return;
}



fn FUN_0045383c(char **param_1,param_2: *mut i32)

{
    let mut uVar1: u32;
    let puVar2: *mut u32;
    let puVar3: *mut u32;
    let mut iVar4: i32;
    let local_34: u8;
    let mut local_30: i32;
    let mut local_2c: i32;
    let mut local_1c: u32;
    let mut local_14: u32;

    puVar2 = FUN_0049c2c9(0x800);
    local_1c = 0x4;
    puVar3 = FUN_0049c2c9(0x28000);
    local_14 = 0x0;
    if ((puVar2 == 0x0) || (puVar3 == 0x0)) {
        pop_err_msg_box_and_exit_004a02f5(s_Memory_Error__004c1d68);
    }
    puVar3 = 0x0;
    FUN_004a7970(puVar2,0x800,0x1,param_1);
    loop {
    local_34 = *(local_1c + puVar2);
    local_1c = local_1c + 0x1;
    for (local_30 = 0x0; local_30 < 0x8; local_30 = local_30 + 0x1) {
        if ((local_34 & 0x1) == 0x0) {
            uVar1 = *(puVar2 + local_1c);
            local_1c = local_1c + 0x2;
            if ((uVar1 << 0x10) >> 0x13 == 0x0) {
                FUN_004a7160(puVar3,local_14,0x1,param_2);
                FUN_0049af50(puVar2);
                FUN_0049af50(puVar3);
                return;
            }
            iVar4 = (uVar1 & 0x7) + 0x2;
            for (local_2c = 0x0; local_2c < iVar4; local_2c = local_2c + 0x1) {
                (local_14 + local_2c + puVar3) =
                    ((local_14 - ((uVar1 << 0x10) >> 0x13)) + local_2c + puVar3);
            }
            local_14 = local_14 + iVar4;
        }
        else {
            (local_14 + puVar3) = (local_1c + puVar2);
            local_1c = local_1c + 0x1;
            local_14 = local_14 + 0x1;
        }
        local_34 = local_34 >> 0x1;
    }
    if (0x738 < local_1c) {
        FUN_004a1dc0(puVar2,(puVar2 + local_1c),0x800 - local_1c);
        FUN_004a7970(((0x800 - local_1c) + puVar2),local_1c,0x1,param_1);
        local_1c = 0x0;
    }
    if (0x27ef0 < local_14) {
        FUN_004a7160(puVar3,local_14 - 0x2000,0x1,param_2);
        FUN_004a1dc0(puVar3,((local_14 - 0x2000) + puVar3),0x2000);
        local_14 = 0x2000;
    }
} while( true );
}



fn FUN_00453a3c(byte *param_1,byte *param_2) -> u32

{
    byte **ppbVar1;
    let piVar2: *mut i32;;

    DAT_0059aaa0 = FUN_0049c2c9(0x2000);
    FUN_004a0430(DAT_0059aaa0,0x0,0x2000);
    if (DAT_0059aaa0 == 0x0) {
        pop_err_msg_box_and_exit_004a02f5(s_Memory_Error__004c1d76);
    }
    ppbVar1 = (byte **)FUN_004a96cc(param_1,&DAT_004c1d84);
    if (ppbVar1 == (byte **)0x0) {
    pop_err_msg_box_and_exit_004a02f5(s_Error_opening__s_for_input_004c1d87);
}
    piVar2 = FUN_004a96cc(param_2,&DAT_004c1da3);
    if (piVar2 == 0x0) {
        pop_err_msg_box_and_exit_004a02f5(s_Error_opening__s_for_output_004c1da6);
    }
    FUN_0045349e(ppbVar1,piVar2,0x1);
    FUN_0049ca40(piVar2);
    FUN_0049ca40(ppbVar1);
    FUN_0049af50(DAT_0059aaa0);
    return 0x0;
}



fn FUN_00453b2a(byte *param_1,byte *param_2) -> u32

{
    let ppcVar1: *mut *mut char;
    let piVar2: *mut i32;;

    DAT_0059aaa0 = FUN_0049c2c9(0x2000);
    FUN_004a0430(DAT_0059aaa0,0x0,0x2000);
    if (DAT_0059aaa0 == 0x0) {
        pop_err_msg_box_and_exit_004a02f5(s_Memory_Error__004c1dc3);
    }
    ppcVar1 = FUN_004a96cc(param_1,&DAT_004c1dd1);
    if (ppcVar1 == 0x0) {
        pop_err_msg_box_and_exit_004a02f5(s_Error_opening__s_for_input_004c1dd4);
    }
    piVar2 = FUN_004a96cc(param_2,&DAT_004c1df0);
    if (piVar2 == 0x0) {
        pop_err_msg_box_and_exit_004a02f5(s_Error_opening__s_for_output_004c1df3);
    }
    FUN_0045383c(ppcVar1,piVar2);
    FUN_0049ca40(ppcVar1);
    FUN_0049ca40(piVar2);
    FUN_0049af50(DAT_0059aaa0);
    return 0x0;
}



fn FUN_00453c16(param_1: u32,param_2: u32)

{
    let mut pCVar1: String;;
    let mut pcVar2: String;
    let puVar3: *mut u32;
    let mut iVar4: i32;
    let mut local_3f0: i32;
    let local_3ec: *mut u32;
    let mut local_39c: i32;
    let mut local_398: i32;
    let local_390: u8 [0x100];
    let mut local_290: *mut u32 [0x11];
    let ppuStack587: *mut *mut u8;;
    let mut local_19f: String;;
    i32 aiStack408 [0x30];
    let mut apCStack216: String; [0x30];
    byte **local_18;

    DAT_005b2cd4 = param_1;
    DAT_005b2cd8 = param_2;
    DAT_005b2cdc = 0x0;
    DAT_005b2cc0 = LPCSTR_005b9218;
    LPCSTR_005b9218 = FUN_004971ad(s_efsfnt4_004c1e10);
    if (LPCSTR_005b9218 == 0x0) {
        LPCSTR_005b9218 = DAT_005b2cc0;
    }
    else {
        DAT_005b2cd0 = FUN_0049c2c9(0x900);
        DAT_005b2cc4 = FUN_0049c2c9(0x400);
        FUN_0049c57b(s_mouse3_msk_004c1e18,DAT_005b2cc4,0x400);
        FUN_004990e0(local_290,0x0,s_diplo_res_004c1e2c,s_Manowitz_004c1e23);
        DAT_005b2ce0 = 0x0;
        FUN_0049c2e0(local_390,s_manowitz_volume_d_txt_004c1e36);
        local_18 = (byte **)FUN_0049c4bd(local_390,&DAT_004c1e4c);
        if (local_18 == (byte **)0x0) {
            FUN_0049af50(LPCSTR_005b9218);
            LPCSTR_005b9218 = DAT_005b2cc0;
        }
        else {
            local_398 = 0x4c;
            local_39c = 0x39;
            while (pcVar2 = FUN_004a2f60(local_390,0xff,local_18), pcVar2 != 0x0) {
                puVar3 = FUN_00467684(local_390);
                if (puVar3 != '\0') {
                    puVar3 = FUN_0049e750(puVar3);
                    apCStack216[DAT_005b2ce0] = puVar3;
                    local_3ec = FUN_004a2831(0x5d);
                    if (local_3ec != 0x0) {
                        pCVar1 = apCStack216[DAT_005b2ce0];
                        local_3ec = FUN_0049a030(local_3ec,local_290,DAT_005b2ce0 + 0x3e8,local_398,local_39c,0xc3,0xd,0x4000,
                                                 0x272b2f,0x212121);
                        $1: &mut String(local_3ec + 0x45) = &PTR_FUN_004c3d94;
                        *(local_3ec + 0x51) = pCVar1;
                        *(local_3ec + 0x55) = 0x0;
                        *(local_3ec + 0x4d) = 0x0;
                        *(local_3ec + 0x49) = 0x2;
                    }
                    aiStack408[DAT_005b2ce0] = local_3ec;
                    FUN_0049bf40(local_290,aiStack408[DAT_005b2ce0]);
                    DAT_005b2ce0 = DAT_005b2ce0 + 0x1;
                    if (0x2f < DAT_005b2ce0) break;
                    iVar4 = local_39c + 0x1a;
                    local_39c = local_39c + 0xd;
                    if (0x174 < iVar4) {
                        local_39c = 0x39;
                        local_398 = 0x171;
                    }
                }
            }
            FUN_0049ca40(local_18);
            FUN_0049bb50(local_290,FUN_004541bf);
            for (local_3f0 = 0x0; local_3f0 < DAT_005b2ce0; local_3f0 = local_3f0 + 0x1) {
                if ((aiStack408[local_3f0] != 0x0) && (iVar4 = aiStack408[local_3f0], iVar4 != 0x0)) {
                    ((*(iVar4 + 0x45) + 0x8))(iVar4,0x2);
                }
                if (apCStack216[local_3f0] != 0x0) {
                    FUN_0049af50(apCStack216[local_3f0]);
                }
            }
        }
        if (DAT_005b2cc8 != 0x0) {
            FUN_0049af50(DAT_005b2cc8);
            DAT_005b2cc8 = 0x0;
        }
        if (DAT_005b2ccc != 0x0) {
            FUN_0049af50(DAT_005b2ccc);
            DAT_005b2ccc = 0x0;
        }
        FUN_0049af50(DAT_005b2cc4);
        FUN_0049af50(DAT_005b2cd0);
        FUN_00495491();
        ppuStack587 = &PTR_FUN_004c3d34;
        if (local_19f != 0x0) {
            FUN_00499b30(local_290,local_19f);
        }
        FUN_0049a1c0(local_290,0x1);
    }
    return;
}



fn FUN_004541bf(param_1: *mut *mut u32,param_2: u32,param_3: i32) -> u32

{
    let mut iVar1: i32;
    let mut uVar2: u32;
    let mut local_1b0: *mut u32 [0x11];
    let ppuStack363: *mut *mut u8;;
    let mut local_bf: String;;
    let local_b8: u8 [0x80];
    let mut local_38: i32;
    let mut local_34: i32;
    let mut local_30: i32;
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_18: u32;

    local_18 = param_2;
    if (param_2 < 0x407) {
        if (0x400 < param_2) {
            if (param_2 < 0x402) {
                local_24 = FUN_0049c160(param_1,0x3e8);
                if (local_24 == 0x0) {
                    if ((DAT_005b2cd8 == 0x1) && (DAT_005b2cdc == 0x0)) {
                        FUN_0049bf80(param_1,0x1f8,0x410,0x0,0x0);
                    }
                    else {
                        if ((DAT_005b2cd8 == DAT_005b2ce0) && (DAT_005b2cdc + 0x1 == DAT_005b2ce4)) {
                            FUN_0049bf80(param_1,0x1f5,0x410,0x0,0x0);
                        }
                    }
                }
                else {
                    FUN_0049bf80(param_1,0x1f5,0x410,0x0,0x0);
                    FUN_0049bf80(param_1,0x1f8,0x410,0x0,0x0);
                    FUN_0049bf80(param_1,0x1f7,0x410,0x0,0x0);
                    FUN_004953d7();
                    FUN_00496a10();
                    FUN_0049637b(s_manowitz_book_pal_004c1e4f,DAT_005b2cd0);
                    FUN_00496404(DAT_005b2cd0);
                    FUN_0049536f();
                    if (DAT_005b2cd8 != 0x0) {
                        FUN_0049f99b(DAT_005b9bd4,param_1,0x407,DAT_005b2cd8 + 0x3e7,0x0);
                    }
                }
                return 0x1;
            }
            if (((param_2 == 0x405) && (local_28 = FUN_0049c160(param_1,0x3e8), local_28 == 0x0)) && (DAT_005b2cd8 != 0x0)) {
                if (DAT_005b2ce4 == 0x0) {
                    FUN_00497330(0x4c,0x39,s__null__004c1e75,0x6,0x272b2f,-0x1,0x272b2f);
                    return 0x0;
                }
                local_2c = DAT_005b2cdc * 0x30 - DAT_005b2cec;
                local_30 = local_2c + 0x30;
                if (DAT_005b2ce8 < local_30) {
                    local_30 = DAT_005b2ce8;
                }
                local_34 = 0x4c;
                local_38 = 0x39;
                FUN_004953d7();
                if ((DAT_005b2cdc == 0x0) && (DAT_005b2cec != 0x0)) {
                    FUN_0049c2e0(local_b8,s_MANOWITZ__s_004c1e7c);
                    FUN_004a08c5(local_b8,0x4c,0x37,0x2710,0x2710,0x0,0x0,0x0,0x1);
                    local_38 = local_38 + DAT_005b2cec * 0xd;
                    local_2c = 0x1;
                }
                for (; local_2c < local_30; local_2c = local_2c + 0x1) {
                    FUN_00497330(local_34,local_38,*(local_2c * 0x6 + DAT_005b2ccc),
                                 *(local_2c * 0x6 + DAT_005b2ccc + 0x2) >> 0x10,0x272b2f,-0x1,0x272b2f);
                    iVar1 = local_38 + 0x1a;
                    local_38 = local_38 + 0xd;
                    if (0x173 < iVar1) {
                        local_38 = 0x39;
                        local_34 = 0x171;
                    }
                }
                FUN_0049536f();
                return 0x1;
            }
        }
    }
    else {
        if (param_2 < 0x408) {
            if (param_3 < 0x3e8) {
                switch(param_3) {
                    case 0x1f5:
                        FUN_0049bf80(param_1,0x1f8,0x40f,0x0,0x0);
                    if (DAT_005b2cdc + 0x1 < DAT_005b2ce4) {
                        DAT_005b2cdc = DAT_005b2cdc + 0x1;
                        FUN_0049a770(param_1,0x405,0x0,0x0);
                        if ((DAT_005b2cd8 == DAT_005b2ce0) && (DAT_005b2cdc + 0x1 == DAT_005b2ce4)) {
                            FUN_0049bf80(param_1,0x1f5,0x410,0x0,0x0);
                        }
                    }
                    else {
                        if (DAT_005b2cd8 < DAT_005b2ce0) {
                            DAT_005b2cd8 = DAT_005b2cd8 + 0x1;
                            FUN_00454908();
                            FUN_0049a770(param_1,0x405,0x0,0x0);
                            if ((DAT_005b2cd8 == DAT_005b2ce0) && (DAT_005b2cdc + 0x1 == DAT_005b2ce4)) {
                                FUN_0049bf80(param_1,0x1f5,0x410,0x0,0x0);
                            }
                        }
                    }
                    break;
                    case 0x1f6:
                        FUN_0049c140(param_1,0x1);
                    break;
                    case 0x1f7:
                        FUN_0049c140(param_1,0x0);
                    break;
                    case 0x1f8:
                        FUN_0049bf80(param_1,0x1f5,0x40f,0x0,0x0);
                    if (DAT_005b2cdc == 0x0) {
                        if (0x1 < DAT_005b2cd8) {
                            DAT_005b2cd8 = DAT_005b2cd8 + -0x1;
                            FUN_00454908();
                            if (0x1 < DAT_005b2ce4) {
                                DAT_005b2cdc = DAT_005b2ce4 + -0x1;
                            }
                            FUN_0049a770(param_1,0x405,0x0,0x0);
                            if ((DAT_005b2cd8 == 0x1) && (DAT_005b2cdc == 0x0)) {
                                FUN_0049bf80(param_1,0x1f8,0x410,0x0,0x0);
                            }
                        }
                    }
                    else {
                        DAT_005b2cdc = DAT_005b2cdc + -0x1;
                        FUN_0049a770(param_1,0x405,0x0,0x0);
                        if ((DAT_005b2cd8 == 0x1) && (DAT_005b2cdc == 0x0)) {
                            FUN_0049bf80(param_1,0x1f8,0x410,0x0,0x0);
                        }
                    }
                }
            }
            else {
                DAT_005b2cd8 = param_3 + -0x3e7;
                FUN_00454908();
                FUN_004990e0(local_1b0,0x0,s_diplo_res_004c1e91,s_Manowitz_004c1e88);
                uVar2 = FUN_0049bb50(local_1b0,FUN_004541bf);
                if (uVar2 == 0x0) {
                    DAT_005b2cd8 = 0x0;
                }
                else {
                    FUN_0049c140(param_1,0x0);
                }
                ppuStack363 = &PTR_FUN_004c3d34;
                if (local_bf != 0x0) {
                    FUN_00499b30(local_1b0,local_bf);
                }
                FUN_0049a1c0(local_1b0,0x1);
            }
        }
        else {
            if (param_2 < 0x411) {
                if (param_2 == 0x40c) {
                    FUN_004953d7();
                    FUN_004a08c5(s_manowitz_book5h_pcx_004c1e61,0x0,0x0,0x280,0x1e0,0x0,0x0,0x0,0x0);
                    FUN_00495438(DAT_005b2cc4,0x20,0x20);
                    FUN_004954f3(0x3,0x3);
                    FUN_0049536f();
                    return 0x1;
                }
            }
            else {
                if (param_2 < 0x412) {
                    return 0x1;
                }
                if (param_2 == 0x415) {
                    local_20 = FUN_0049c160(param_1,0x3e8);
                    if (local_20 != 0x0) {
                        FUN_004953d7();
                        FUN_00496a10();
                        FUN_00496404(DAT_005b96c8);
                        FUN_0049536f();
                        FUN_0049af50(LPCSTR_005b9218);
                        LPCSTR_005b9218 = DAT_005b2cc0;
                    }
                    return 0x1;
                }
            }
        }
    }
    return 0x0;
}



fn FUN_00454908() -> i32

{
let cVar1: u8;
let mut iVar2: i32;
let mut uVar3: u32;
let puVar4: *mut u32;
byte *pbVar5;
let puVar6: *mut u32;
let mut pcVar7: String;
let mut local_23c: u32;
ushort local_236;
ushort local_232;
let local_1bc: *mut *mut char;
byte *local_1b8;
byte *local_1b4;
let mut local_1b0: u32;
let local_b0: u8 [0x80];
let mut local_30: i32;
let local_2c: u32;
let local_28: *mut *mut char;
let mut local_24: i32;
let mut local_20: i32;
let mut local_1c: i32;
let mut local_18: u32;
let mut local_14: i32;
let puVar8: *mut u32;

DAT_005b2ce4 = 0x0;
DAT_005b2ce8 = 0x0;
FUN_0049c2e0(local_b0,s_manowitz_v__1dChp__3d_txt_004c1e9b);
local_28 = FUN_0049c4bd(local_b0,&DAT_004c1eb5);
if (local_28 == 0x0) {
local_30 = 0x0;
}
else {
local_2c = FUN_004a91d0(local_28[0x4]);
if (DAT_005b2cc8 != 0x0) {
FUN_0049af50(DAT_005b2cc8);
}
local_20 = -0x1;
for (local_24 = 0x1; local_24 < 0x72; local_24 = local_24 + 0x1) {
if ((*(&DAT_0058ad76 + local_24 * 0xda) == DAT_005b2cd4) &&
(*(&DAT_0058ad7a + local_24 * 0xda) == DAT_005b2cd8)) {
local_20 = local_24;
}
}
puVar4 = &DAT_004bf540;
puVar6 = &local_1b0;
for (iVar2 = 0x40; iVar2 != 0x0; iVar2 = iVar2 + -0x1) {
*puVar6 = *puVar4;
puVar4 = puVar4 + 0x1;
puVar6 = puVar6 + 0x1;
}
local_1c = 0x0;
if (local_20 != -0x1) {
FUN_0049c2e0(&local_1b0,&DAT_004c1eb8);
FUN_00454cc4((&local_1b0 + 0x1),local_20);
uVar3 = 0xffffffff;
puVar4 = &local_1b0;
loop {
if (uVar3 == 0x0) break;
uVar3 = uVar3 - 0x1;
cVar1 = puVar4;
puVar4 = (puVar4 + 0x1);
} while (cVar1 != '\0');
local_1c = ~uVar3 - 0x1;
}
DAT_005b2cc8 = FUN_0049c2c9(local_2c + local_1c + 0x1);
if (DAT_005b2cc8 == 0x0) {
FUN_0049ca40(local_28);
local_30 = 0x0;
}
else {
local_18 = FUN_004a7970(DAT_005b2cc8,local_2c,0x1,local_28);
FUN_0049ca40(local_28);
if (local_18 == 0x0) {
local_30 = 0x0;
}
else {
(DAT_005b2cc8 + local_2c) = '\0';
if (local_20 != -0x1) {
puVar4 = &local_1b0;
iVar2 = -0x1;
puVar6 = DAT_005b2cc8;
loop {
puVar8 = puVar6;
if (iVar2 == 0x0) break;
iVar2 = iVar2 + -0x1;
puVar8 = (puVar6 + 0x1);
cVar1 = puVar6;
puVar6 = puVar8;
} while (cVar1 != '\0');
pcVar7 = (puVar8 + -0x1);
loop {
cVar1 = puVar4;
*pcVar7 = cVar1;
if (cVar1 == '\0') break;
cVar1 = (puVar4 + 0x1);
puVar4 = (puVar4 + 0x2);
pcVar7[0x1] = cVar1;
pcVar7 = pcVar7 + 0x2;
} while (cVar1 != '\0');
}
if (DAT_005b2ccc == (byte **)0x0) {
DAT_005b2ccc = (byte **)FUN_0049c2c9(0x1194);
}
if (DAT_005b2ccc == (byte **)0x0) {
local_30 = 0x0;
}
else {
DAT_005b2ce8 = FUN_004a3840(DAT_005b2cc8,DAT_005b2ccc,0xc3,0x2ee,&local_14,
LPCSTR_005b9218,0x0);
*(DAT_005b2ce8 * 0x6 + DAT_005b2ccc) = 0x0;
DAT_005b2cdc = 0x0;
DAT_005b2cec = 0x0;
iVar2 = FUN_004a9800(*DAT_005b2ccc,&DAT_004c1eba,0x2);
if (iVar2 == 0x0) {
local_1b4 = *DAT_005b2ccc + 0x2;
pbVar5 = local_1b4;
loop {
local_1b8 = pbVar5;
if (*pbVar5 == 0x3e)^ // goto LAB_00454bcd;
if (*pbVar5 == 0x0) break;
local_1b8 = pbVar5 + 0x1;
if (*local_1b8 == 0x3e)^ // goto LAB_00454bcd;
pbVar5 = pbVar5 + 0x2;
} while (*local_1b8 != 0x0);
local_1b8 = 0x0;
// LAB_00454bcd:
if (local_1b8 != 0x0) {
*local_1b8 = 0x0;
}
FUN_0049c2e0(local_b0,s_MANOWITZ__s_004c1ebd);
local_1bc = FUN_0049c4bd(local_b0,&DAT_004c1ec9);
if (local_1bc != 0x0) {
FUN_004a0430(&local_23c,0x0,0x80);
FUN_004a7970(&local_23c,0x1,0x80,local_1bc);
FUN_0049ca40(local_1bc);
DAT_005b2cec = ((local_232 - local_236) + 0xd) / 0xd;
}
}
DAT_005b2ce4 = (DAT_005b2ce8 + 0x2f + DAT_005b2cec) / 0x30;
local_30 = DAT_005b2ce4;
}
}
}
}
return local_30;
}



fn FUN_00454cc4(param_1: &mut String,param_2: i32)

{
    let cVar1: u8;
    let mut pcVar2: String;
    let mut pcVar3: String;
    let mut uVar4: u32;
    let mut uVar5: u32;
    let mut iVar6: i32;
    let mut pcVar7: String;
    let mut local_1f0: i32;
    i32 local_1ec [0x72];
    let mut local_24: i32;
    let mut local_20: String;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    if (((&DAT_00569c30)[param_2 * 0x9 + DAT_004c9754 * 0x1e22] & 0x1) == 0x0) {
        if (((&DAT_00569c30)[param_2 * 0x9 + DAT_004c9754 * 0x1e22] & 0x4) == 0x0) {
            if (((&DAT_00569c30)[param_2 * 0x9 + DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
                FUN_004a0430(local_1ec,0x0,0x1c8);
                local_24 = FUN_00455070(param_2,local_1ec,0x0);
                pcVar3 = FUN_00499050(DAT_0059679c,0x7406);
                pcVar2 = param_1;
                loop {
                    cVar1 = *pcVar3;
                    *pcVar2 = cVar1;
                    if (cVar1 == '\0') break;
                    cVar1 = pcVar3[0x1];
                    pcVar3 = pcVar3 + 0x2;
                    pcVar2[0x1] = cVar1;
                    pcVar2 = pcVar2 + 0x2;
                } while (cVar1 != '\0');
                local_20 = FUN_00499050(DAT_0059679c,0x7407);
                local_1c = 0x0;
                uVar4 = 0xffffffff;
                pcVar2 = param_1;
                loop {
                    if (uVar4 == 0x0) break;
                    uVar4 = uVar4 - 0x1;
                    cVar1 = *pcVar2;
                    pcVar2 = pcVar2 + 0x1;
                } while (cVar1 != '\0');
                uVar5 = 0xffffffff;
                pcVar2 = local_20;
                loop {
                    if (uVar5 == 0x0) break;
                    uVar5 = uVar5 - 0x1;
                    cVar1 = *pcVar2;
                    pcVar2 = pcVar2 + 0x1;
                } while (cVar1 != '\0');
                local_18 = ~uVar4 + (~uVar5 - 0x2);
                loop {
                    for (local_1f0 = 0x0; local_1f0 < 0x72; local_1f0 = local_1f0 + 0x1) {
                        if (local_1ec[local_1f0] != 0x0) {
                            uVar4 = 0xffffffff;
                            pcVar2 = &DAT_0058aca8 + local_1f0 * 0xda;
                            loop {
                                if (uVar4 == 0x0) break;
                                uVar4 = uVar4 - 0x1;
                                cVar1 = *pcVar2;
                                pcVar2 = pcVar2 + 0x1;
                            } while (cVar1 != '\0');
                            local_18 = local_18 + (~uVar4 - 0x1);
                            local_1c = local_1c + 0x1;
                            if (local_24 < 0x3) {
                                local_18 = local_18 + 0x1;
                            }
                            else {
                                local_18 = local_18 + 0x2;
                            }
                            if (local_24 - local_1c == 0x1) {
                                local_18 = local_18 + 0x4;
                            }
                        }
                    }
                    if (local_18 < 0xff)^ // goto LAB_00454ef6;
                    local_24 = local_24 + -0x1;
                } while( true );
            }
            pcVar2 = FUN_00499050(DAT_0059679c,0x7405);
            loop {
                cVar1 = *pcVar2;
                *param_1 = cVar1;
                if (cVar1 == '\0') {
                    return;
                }
                cVar1 = pcVar2[0x1];
                pcVar2 = pcVar2 + 0x2;
                param_1[0x1] = cVar1;
                param_1 = param_1 + 0x2;
            } while (cVar1 != '\0');
        }
        else {
            pcVar2 = FUN_00499050(DAT_0059679c,0x7404);
            loop {
                cVar1 = *pcVar2;
                *param_1 = cVar1;
                if (cVar1 == '\0') {
                    return;
                }
                cVar1 = pcVar2[0x1];
                pcVar2 = pcVar2 + 0x2;
                param_1[0x1] = cVar1;
                param_1 = param_1 + 0x2;
            } while (cVar1 != '\0');
        }
    }
    else {
        pcVar2 = FUN_00499050(DAT_0059679c,0x7403);
        loop {
            cVar1 = *pcVar2;
            *param_1 = cVar1;
            if (cVar1 == '\0') {
                return;
            }
            cVar1 = pcVar2[0x1];
            pcVar2 = pcVar2 + 0x2;
            param_1[0x1] = cVar1;
            param_1 = param_1 + 0x2;
        } while (cVar1 != '\0');
    }
    return;
    LAB_00454ef6:
        local_1c = 0x0;
    for (local_14 = 0x0; local_14 < 0x72; local_14 = local_14 + 0x1) {
        if (local_1ec[local_14] != 0x0) {
            pcVar3 = &DAT_0058aca8 + local_14 * 0xda;
            iVar6 = -0x1;
            pcVar2 = param_1;
            loop {
                pcVar7 = pcVar2;
                if (iVar6 == 0x0) break;
                iVar6 = iVar6 + -0x1;
                pcVar7 = pcVar2 + 0x1;
                cVar1 = *pcVar2;
                pcVar2 = pcVar7;
            } while (cVar1 != '\0');
            pcVar7 = pcVar7 + -0x1;
            loop {
                cVar1 = *pcVar3;
                *pcVar7 = cVar1;
                if (cVar1 == '\0') break;
                cVar1 = pcVar3[0x1];
                pcVar3 = pcVar3 + 0x2;
                pcVar7[0x1] = cVar1;
                pcVar7 = pcVar7 + 0x2;
            } while (cVar1 != '\0');
            local_1c = local_1c + 0x1;
            if (local_24 <= local_1c) break;
            if (local_24 < 0x3) {
                pcVar3 = &DAT_004c1ecf;
                iVar6 = -0x1;
                pcVar2 = param_1;
                loop {
                    pcVar7 = pcVar2;
                    if (iVar6 == 0x0) break;
                    iVar6 = iVar6 + -0x1;
                    pcVar7 = pcVar2 + 0x1;
                    cVar1 = *pcVar2;
                    pcVar2 = pcVar7;
                } while (cVar1 != '\0');
                pcVar7 = pcVar7 + -0x1;
                loop {
                    cVar1 = *pcVar3;
                    *pcVar7 = cVar1;
                    if (cVar1 == '\0') break;
                    cVar1 = pcVar3[0x1];
                    pcVar3 = pcVar3 + 0x2;
                    pcVar7[0x1] = cVar1;
                    pcVar7 = pcVar7 + 0x2;
                } while (cVar1 != '\0');
            }
            else {
                pcVar3 = &DAT_004c1ecc;
                iVar6 = -0x1;
                pcVar2 = param_1;
                loop {
                    pcVar7 = pcVar2;
                    if (iVar6 == 0x0) break;
                    iVar6 = iVar6 + -0x1;
                    pcVar7 = pcVar2 + 0x1;
                    cVar1 = *pcVar2;
                    pcVar2 = pcVar7;
                } while (cVar1 != '\0');
                pcVar7 = pcVar7 + -0x1;
                loop {
                    cVar1 = *pcVar3;
                    *pcVar7 = cVar1;
                    if (cVar1 == '\0') break;
                    cVar1 = pcVar3[0x1];
                    pcVar3 = pcVar3 + 0x2;
                    pcVar7[0x1] = cVar1;
                    pcVar7 = pcVar7 + 0x2;
                } while (cVar1 != '\0');
            }
            if (local_24 - local_1c == 0x1) {
                pcVar3 = FUN_00499050(DAT_0059679c,0x7135);
                iVar6 = -0x1;
                pcVar2 = param_1;
                loop {
                    pcVar7 = pcVar2;
                    if (iVar6 == 0x0) break;
                    iVar6 = iVar6 + -0x1;
                    pcVar7 = pcVar2 + 0x1;
                    cVar1 = *pcVar2;
                    pcVar2 = pcVar7;
                } while (cVar1 != '\0');
                pcVar7 = pcVar7 + -0x1;
                loop {
                    cVar1 = *pcVar3;
                    *pcVar7 = cVar1;
                    if (cVar1 == '\0') break;
                    cVar1 = pcVar3[0x1];
                    pcVar3 = pcVar3 + 0x2;
                    pcVar7[0x1] = cVar1;
                    pcVar7 = pcVar7 + 0x2;
                } while (cVar1 != '\0');
                pcVar3 = &DAT_004c1ed1;
                iVar6 = -0x1;
                pcVar2 = param_1;
                loop {
                    pcVar7 = pcVar2;
                    if (iVar6 == 0x0) break;
                    iVar6 = iVar6 + -0x1;
                    pcVar7 = pcVar2 + 0x1;
                    cVar1 = *pcVar2;
                    pcVar2 = pcVar7;
                } while (cVar1 != '\0');
                pcVar7 = pcVar7 + -0x1;
                loop {
                    cVar1 = *pcVar3;
                    *pcVar7 = cVar1;
                    if (cVar1 == '\0') break;
                    cVar1 = pcVar3[0x1];
                    pcVar3 = pcVar3 + 0x2;
                    pcVar7[0x1] = cVar1;
                    pcVar7 = pcVar7 + 0x2;
                } while (cVar1 != '\0');
            }
        }
    }
    iVar6 = -0x1;
    loop {
    pcVar2 = param_1;
    if (iVar6 == 0x0) break;
    iVar6 = iVar6 + -0x1;
    pcVar2 = param_1 + 0x1;
    cVar1 = *param_1;
    param_1 = pcVar2;
} while (cVar1 != '\0');
    pcVar2 = pcVar2 + -0x1;
    loop {
    cVar1 = *local_20;
    *pcVar2 = cVar1;
    if (cVar1 == '\0') {
        return;
    }
    cVar1 = local_20[0x1];
    local_20 = local_20 + 0x2;
    pcVar2[0x1] = cVar1;
    pcVar2 = pcVar2 + 0x2;
} while (cVar1 != '\0');
    return;
}



fn FUN_00455070(param_1: i32,param_2: i32,param_3: i32) -> i32

{
let mut iVar1: i32;
let mut local_14: i32;

for (local_14 = 0x0; local_14 < 0x3; local_14 = local_14 + 0x1) {
iVar1 = FUN_0046295d((&DAT_0058aca8 + param_1 * 0xda),local_14);
if (((iVar1 != 0x320) && (iVar1 != 0x0)) && (((&DAT_00569c30)[iVar1 * 0x9 + DAT_004c9754 * 0x1e22] & 0x1) == 0x0)) {
if (((&DAT_00569c30)[iVar1 * 0x9 + DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
param_3 = FUN_00455070(iVar1,param_2,param_3);
}
else {
*(iVar1 * 0x4 + param_2) = 0x1;
param_3 = param_3 + 0x1;
}
}
}
return param_3;
}



fn FUN_00455143() -> *mut u32

{
    let puVar1: *mut u32;

    puVar1 = FUN_004a2831(0x1d);
    puVar1[0x6] = 0x0;
    puVar1[0x5] = 0x0;
    *puVar1 = 0xffffffff;
    return puVar1;
}



void
FUN_0045518a(param_1: u32,param_2: u32,param_3: u32,param_4: u32,param_5: *mut u32,param_6: u32,
let param_7: u16)

{
let cVar1: u8;
let mut uVar2: u32;
let puVar3: *mut u32;
let mut uVar4: u32;
let mut local_20: i32;
let mut local_18: i32;

local_18 = 0x0;
loop {
if (0x4 < local_18) {
return;
}
if (((((&DAT_00569a98)[local_18 * 0x1e22] & 0x1) == 0x0) && (((&DAT_00569a98)[local_18 * 0x1e22] & 0x2) == 0x0)) &&
(uVar2 = 0x1 << ((byte)local_18 & 0x1f), (uVar2 & param_1) != 0x0)) {
local_20 = *(&DAT_00599c6c + local_18 * 0x4);
if ((param_5 != 0x0) && (param_6 == -0x1)) {
uVar4 = 0xffffffff;
puVar3 = param_5;
loop {
if (uVar4 == 0x0) break;
uVar4 = uVar4 - 0x1;
cVar1 = puVar3;
puVar3 = (puVar3 + 0x1);
} while (cVar1 != '\0');
param_6 = ~uVar4;
}
if (param_5 == 0x0) {
param_6 = 0x0;
}
puVar3 = FUN_0049c2c9(param_6 + 0x1d);
if (puVar3 == 0x0) {
return;
}
*puVar3 = param_3;
puVar3[0x1] = param_2;
puVar3[0x2] = uVar2;
puVar3[0x3] = param_4;
(puVar3 + 0x4) = param_6;
*(puVar3 + 0x12) = param_7;
if (local_18 != DAT_004c9754) {
*(puVar3 + 0x12) = *(puVar3 + 0x12) & 0xfe;
}
if ((param_5 != 0x0) && (0x0 < param_6)) {
puVar3[0x7] = *param_5;
(puVar3 + 0x8) = (param_5 + 0x1);
}
if (*(local_20 + 0x18) == 0x0) {
puVar3[0x6] = 0x0;
puVar3[0x5] = local_20;
(puVar3[0x5] + 0x18) = puVar3;
}
else {
for (; *(local_20 + 0x18) != 0x0; local_20 = *(local_20 + 0x18)) {
}
puVar3[0x6] = 0x0;
puVar3[0x5] = local_20;
(local_20 + 0x18) = puVar3;
}
}
local_18 = local_18 + 0x1;
} while( true );
}



fn FUN_0045532a() -> u32

{
    let mut local_20: i32;
    let mut local_1c: i32;

    for (local_20 = 0x0; local_20 < 0xe; local_20 = local_20 + 0x1) {
        if (*(&DAT_004c977c + DAT_004c9754 * 0xf8 + local_20 * 0xd90) == 0x0) {
            return 0x1;
        }
    }
    local_1c = *(*(&DAT_00599c6c + DAT_004c9754 * 0x4) + 0x18);
    while( true ) {
        if (local_1c == 0x0) {
            return 0x0;
        }
        if ((((*(local_1c + 0x12) & 0x1) == 0x0) &&
            ((*(local_1c + 0x8) & 0x1 << ((byte)DAT_004c9754 & 0x1f)) != 0x0)) &&
            ((*(local_1c + 0x8) & 0x1 << ((byte)DAT_004c9754 + 0x5 & 0x1f)) == 0x0)) break;
        local_1c = *(local_1c + 0x18);
    }
    return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

i32 *  FUN_004553fd(param_1: i32,param_2: i32)

{
let piVar1: *mut i32;;
let mut bVar2: bool;
let piVar3: *mut i32;;
let mut uVar4: u32;
let mut pcVar5: String;
let local_30: *mut i32;;
let local_2c: *mut i32;;
let mut local_1c: i32;
let local_14: *mut i32;;

local_2c = 0x0;
bVar2 = false;
local_1c = -0x1;
DAT_005b2d3c = 0x0;
piVar3 = 0x0;
_DAT_005b2d38 = 0x0;
local_30 = *(i32 **)(*(&DAT_00599c6c + DAT_004c9754 * 0x4) + 0x18);
while (local_30 != 0x0) {
if ((local_30[0x2] & 0x1 << ((byte)DAT_004c9754 & 0x1f)) == 0x0) {
piVar3 = local_30[0x6];
local_30 = piVar3;
}
else {
piVar1 = (local_2c + 0x1);
piVar3 = local_2c;
local_2c = piVar1;
if (bVar2) break;
DAT_005b2d3c = (local_30[0x6] != 0x0);
if (local_30[0x5] == 0x0) {
_DAT_005b2d38 = 0x0;
}
else {
_DAT_005b2d38 = (*local_30[0x5] != -0x1);
}
if ((param_2 != 0x0) && (local_1c != *local_30)) {
local_1c = *local_30;
FUN_004953d7();
FUN_00496a10();
FUN_0049536f();
}
uVar4 = FUN_004566ac(local_30);
if (uVar4 == 0x65) {
bVar2 = true;
}
else {
if ((uVar4 == 0x66) && (*local_30 == 0x737e)) {
FUN_0042f801();
}
}
if (((*local_30 != 0x737e) || (uVar4 == 0x66)) || (uVar4 == 0x67)) {
local_30[0x2] = local_30[0x2] | 0x1 << ((char)DAT_004c9754 + 0x5U & 0x1f);
}
if (uVar4 == 0x6c) {
piVar3 = local_30[0x5];
local_30 = piVar3;
}
else {
piVar3 = local_30[0x6];
local_30 = piVar3;
}
}
}
bVar2 = true;
for (local_14 = 0x0; local_14 < 0xe; local_14 = (local_14 + 0x1)) {
if (*(&DAT_004c977c + DAT_004c9754 * 0xf8 + local_14 * 0xd90) == 0x0) {
if (bVar2) {
bVar2 = false;
FUN_004953d7();
FUN_00496a10();
FUN_0049536f();
}
local_2c = (local_2c + 0x1);
FUN_0045578c(DAT_004c9754,local_14,0x3);
}
piVar3 = local_14;
}
if (!bVar2) {
piVar3 = FUN_0049ad10(0x0);
}
if ((local_2c == 0x0) && (param_1 != 0x0)) {
pcVar5 = FUN_00499050(DAT_0059679c,0x715a);
piVar3 = FUN_0049d2e0(0x0,0x1,pcVar5);
}
return piVar3;
}



fn FUN_00455648()

{
    let mut pCVar1: String;;
    let mut local_1c: String;;
    let mut local_14: i32;

    if (DAT_004c9754 < 0x5) {
        for (local_14 = 0x0; local_14 < 0xe; local_14 = local_14 + 0x1) {
            if (*(&DAT_004c977c + DAT_004c9754 * 0xf8 + local_14 * 0xd90) == 0x0) {
                FUN_0042d03d(DAT_004c9754,local_14);
            }
        }
        local_1c = *(*(&DAT_00599c6c + DAT_004c9754 * 0x4) + 0x18);
        while (local_1c != 0x0) {
            pCVar1 = *(local_1c + 0x18);
            if ((local_1c[0x12] & 0x1U) == 0x0) {
                *(*(local_1c + 0x14) + 0x18) = *(local_1c + 0x18);
                if (*(local_1c + 0x18) != 0x0) {
                    *(*(local_1c + 0x18) + 0x14) = *(local_1c + 0x14);
                }
                FUN_0049af50(local_1c);
                local_1c = pCVar1;
            }
            else {
                local_1c[0x12] = local_1c[0x12] & 0xfe;
                local_1c = pCVar1;
            }
        }
    }
    return;
}



fn FUN_0045571f()

{
    let mut pCVar1: String;;
    let mut local_1c: String;;
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0x5; local_14 = local_14 + 0x1) {
        local_1c = *(&DAT_00599c6c + local_14 * 0x4);
        while (local_1c != 0x0) {
            pCVar1 = *(local_1c + 0x18);
            FUN_0049af50(local_1c);
            local_1c = pCVar1;
        }
        *(&DAT_00599c6c + local_14 * 0x4) = 0x0;
    }
    return;
}



fn FUN_0045578c(param_1: u32,param_2: i32,param_3: u32) -> u32

{
    let mut local_118: *mut u32 [0x11];
    let ppuStack211: *mut *mut u8;;
    let mut local_27: String;;
    let mut local_20: u32;
    let mut local_1c: u32;
    let mut local_18: i32;

    DAT_005b2d30 = param_1;
    DAT_005b2d34 = param_2;
    DAT_004c93c8 = FUN_0049c2c9(0xc350);
    local_18 = param_2;
    switch(param_2) {
    case 0x0:
        case 0x1:
        case 0x2:
        case 0x3:
        case 0x4:
        FUN_0049c60b(s_bin_portrait_bin_004c1ed3,DAT_004c93c8,0xc350,
    *(&DAT_00569b60 + param_2 * 0x1e22) * 0x3d090 + param_2 * 0xc350);
    break;
    case 0x5:
        FUN_0049c57b(s_bin_ptratlea_bin_004c1ef5,DAT_004c93c8,0xc350);
    break;
    case 0x6:
        FUN_0049c60b(s_bin_ptratchu_bin_004c1ee4,DAT_004c93c8,0xc350,((DAT_005827f4 >> 0x10) + -0x5) * 0xc350);
    break;
    default:
        FUN_004a0430(DAT_004c93c8,0xe,0xc350);
    break;
    case 0x8:
        FUN_0049c57b(s_bin_ptratvau_bin_004c1f06,DAT_004c93c8,0xc350);
}
    FUN_004990e0(local_118,0x0,s_diplo_res_004c1f22,s_DispConDlg_004c1f17);
    if ((param_3 & 0x1) == 0x0) {
        FUN_0049bf80(local_118,0x66,0x414,0x0,0x0);
        FUN_0049bf80(local_118,0x66,0x410,0x0,0x0);
    }
    if ((param_3 & 0x2) == 0x0) {
        FUN_0049bf80(local_118,0x67,0x414,0x0,0x0);
        FUN_0049bf80(local_118,0x67,0x410,0x0,0x0);
    }
    local_1c = FUN_0049bb50(local_118,FUN_00455a3d);
    FUN_0049af50(DAT_004c93c8);
    local_20 = local_1c;
    ppuStack211 = &PTR_FUN_004c3d34;
    if (local_27 != 0x0) {
        FUN_00499b30(local_118,local_27);
    }
    FUN_0049a1c0(local_118,0x1);
    return local_20;
}



fn FUN_00455a3d(param_1: *mut *mut u32,param_2: u32,param_3: u32,param_4: i32) -> u32

{
    let mut pcVar1: String;
    let mut iVar2: i32;
    let mut uVar3: u32;
    let mut iVar4: i32;
    let mut uVar5: u32;
    let puVar6: *mut u32;
    let bVar7: u8;
    let local_98: u8 [0x84];
    let mut local_14: u32;

    local_14 = param_2;
    if (param_2 < 0x405) {
        if (0xff < param_2) {
            if (param_2 < 0x101) {
                if ((param_4 != 0x0) && (iVar2 = FUN_004a11c0(param_4), iVar2 == 0x1b)) {
                    return 0x1;
                }
                return 0x0;
            }
            if ((param_2 == 0x401) && (((&DAT_00569a98)[DAT_005b2d30 * 0x1e22] & 0x2) != 0x0)) {
                FUN_0049bf80(param_1,0x66,0x410,0x0,0x0);
                FUN_0049bf80(param_1,0x67,0x410,0x0,0x0);
            }
        }
    }
    else {
        if (param_2 < 0x406) {
            FUN_004953d7();
            FUN_004968e7(0xb,0x13e,0xf8,0x12,0xe);
            FUN_00496ac0(DAT_004c93c8,0xa,0x6a,0xfa,0xc8);
            if (DAT_005b2d34 < 0x5) {
                FUN_00499050(DAT_0059679c,DAT_005b2d34 + 0x414);
                pcVar1 = FUN_00499050(DAT_0059679c,0x73cc);
                FUN_0049c2e0(local_98,pcVar1);
            }
            else {
                FUN_0049c2e0(local_98,&DAT_00569b50 + DAT_005b2d34 * 0x1e22);
            }
            FUN_00497567(0x88,0x140,local_98,0xf0,0xcaccce,-0x1,0xcaccce,LPCSTR_005b9218,0x11);
            FUN_00428288(DAT_005b2d30,DAT_005b2d34,0x1,0x5a);
            FUN_00428288(DAT_005b2d30,DAT_005b2d34,0x0,0x5a);
            bVar7 = 0x11;
            uVar5 = 0xcaccce;
            iVar4 = -0x1;
            uVar3 = 0xcaccce;
            iVar2 = 0xa0;
            puVar6 = LPCSTR_005b9218;
            pcVar1 = FUN_00499050(DAT_005967ac,0xc8);
            FUN_00497567(0x1c7,0x82,pcVar1,iVar2,uVar3,iVar4,uVar5,puVar6,bVar7);
            if (*(&DAT_004c9778 + DAT_005b2d34 * 0xd90 + DAT_005b2d30 * 0xf8) == 0x0) {
                bVar7 = 0x11;
                uVar5 = 0xcaccce;
                iVar4 = -0x1;
                uVar3 = 0xcaccce;
                iVar2 = 0xa0;
                puVar6 = LPCSTR_005b9218;
                pcVar1 = FUN_00499050(DAT_005967ac,0xc9);
                FUN_00497567(0x1c7,0xe1,pcVar1,iVar2,uVar3,iVar4,uVar5,puVar6,bVar7);
            }
            else {
                bVar7 = 0x11;
                uVar5 = 0xcaccce;
                iVar4 = -0x1;
                uVar3 = 0xcaccce;
                iVar2 = 0xa0;
                puVar6 = LPCSTR_005b9218;
                pcVar1 = FUN_00499050(DAT_005967ac,0xca);
                FUN_00497567(0x1c7,0xe1,pcVar1,iVar2,uVar3,iVar4,uVar5,puVar6,bVar7);
            }
            FUN_0049536f();
            return 0x1;
        }
        if (param_2 < 0x40c) {
            if (param_2 == 0x407) {
                if (0x65 < param_3) {
                    if (param_3 < 0x67) {
                        FUN_0042ce75(DAT_005b2d30,DAT_005b2d34);
                    }
                    else {
                        if (param_3 == 0x67) {
                            FUN_0042d03d(DAT_005b2d30,DAT_005b2d34);
                        }
                    }
                }
                FUN_0049c140(param_1,param_3);
            }
        }
        else {
            if (param_2 < 0x40d) {
                FUN_004953d7();
                FUN_004a08c5(s_pcx_contract_pcx_004c1f2c,*(param_1 + 0x1d),*(param_1 + 0x21),
                             *(param_1 + 0x25),*(param_1 + 0x29),0x0,0x0,0x0,0x1);
                FUN_0049536f();
                return 0x1;
            }
            if (param_2 == 0x411) {
                return 0x1;
            }
        }
    }
    return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00455e22(param_1: &mut String,param_2: i32,param_3: u32) -> u32

{
    let local_138: *mut i32;;
    let local_12c: *mut i32;;
    let mut local_120: *mut u32 [0x11];
    let ppuStack219: *mut *mut u8;;
    let mut local_2f: String;;
    let mut local_28: u32;
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_1c: u32;
    let local_18: *mut i32;;

    DAT_005b2d44 = param_3;
    DAT_005b2d40 = param_2;
    local_18 = FUN_004990e0(local_120,0x0,s_diplo_res_004c1f45,s_TextPCX_004c1f3d);
    if (param_2 != -0x1) {
        local_12c = FUN_004a2831(0xb9);
        if (local_12c != 0x0) {
            local_12c = FUN_00438792(local_12c,local_120,0x1f4,0x88,0x17c,0x58,0x3f,0x40,0x2,0x2,param_2);
        }
        DAT_005b8c38 = local_12c;
        local_138 = FUN_004a2831(0x95);
        if (local_138 != 0x0) {
            local_138 = FUN_0047157e(local_138,local_120,0x1f5,0x16,0x17c,0x64,0x60,0x40,0x2);
        }
        DAT_005b8c3c = local_138;
        FUN_0049bf40(local_120,DAT_005b8c38);
        FUN_0049bf40(local_120,DAT_005b8c3c);
        local_24 = DAT_005b7068;
        DAT_005b7068 = param_2;
        DAT_005b8808 = 0x0;
    }
    _DAT_005b2d2c = FUN_004a3840(param_1,&DAT_005b2cf0,0x168,0x4,&local_20,LPCSTR_005b9218,0x0);
    local_1c = FUN_0049bb50(local_120,FUN_0045614c);
    if (param_2 != -0x1) {
        FUN_0047ed66();
        DAT_005b7068 = local_24;
        DAT_005b8808 = 0x0;
        if (DAT_005b8c3c != 0x0) {
            ((*(DAT_005b8c3c + 0x45) + 0x8))(DAT_005b8c3c,0x2);
        }
        if (DAT_005b8c38 != 0x0) {
            ((*(DAT_005b8c38 + 0x45) + 0x8))(DAT_005b8c38,0x2);
        }
    }
    local_28 = local_1c;
    ppuStack219 = &PTR_FUN_004c3d34;
    if (local_2f != 0x0) {
        FUN_00499b30(local_120,local_2f);
    }
    FUN_0049a1c0(local_120,0x1);
    return local_28;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0045614c(param_1: *mut *mut u32,param_2: u32,param_3: u32,param_4: i32) -> u32

{
    let mut iVar1: i32;
    let mut local_60: i32;
    let local_5c: u8 [0x40];
    let mut local_1c: u32;
    let mut local_14: u32;

    local_14 = param_2;
    if (param_2 < 0x405) {
        if (0xff < param_2) {
            if (param_2 < 0x101) {
                if ((param_4 != 0x0) && (iVar1 = FUN_004a11c0(param_4), iVar1 == 0x1b)) {
                    return 0x1;
                }
                return 0x0;
            }
            if (param_2 == 0x401) {
                if (DAT_005b2d44 == 0x73f0) {
                    FUN_0049bf80(param_1,0x6d,0x414,0x0,0x0);
                    FUN_0049bf80(param_1,0x6d,0x410,0x0,0x0);
                    FUN_0049bf80(param_1,0x6c,0x414,0x0,0x0);
                    FUN_0049bf80(param_1,0x6c,0x410,0x0,0x0);
                }
                else {
                    if (DAT_005b2d3c == 0x0) {
                        FUN_0049bf80(param_1,0x6d,0x410,0x0,0x0);
                    }
                    if (_DAT_005b2d38 == 0x0) {
                        FUN_0049bf80(param_1,0x6c,0x410,0x0,0x0);
                    }
                }
            }
        }
        return 0x0;
    }
    if (param_2 < 0x406) {
        FUN_004953d7();
        iVar1 = 0x4 - _DAT_005b2d2c;
        for (local_60 = 0x0; local_60 < _DAT_005b2d2c; local_60 = local_60 + 0x1) {
            FUN_00497567(0x140,local_60 * 0x14 + (iVar1 / 0x2 + 0x1) * 0x14 + 0x12c,*(&DAT_005b2cf0 + local_60 * 0x6),
                         *(&DAT_005b2cf2 + local_60 * 0x6) >> 0x10,0xcaccce,-0x1,0xcaccce,LPCSTR_005b9218
                         ,0x1);
        }
        if (DAT_005b2d40 != -0x1) {
            FUN_00497567(0xb4,0x1cc,(&DAT_005b709e + DAT_005b2d40 * 0x4e),0x91,0xcaccce,-0x1,0xcaccce,DAT_004d6a6c,0x19);
            FUN_0049e640(*(DAT_005b8c38 + 0x1d),*(DAT_005b8c38 + 0x21),*(DAT_005b8c38 + 0x25),
                         *(DAT_005b8c38 + 0x29),0xce,0xca,0xcc,0x1);
            FUN_0049e640(*(DAT_005b8c3c + 0x1d),*(DAT_005b8c3c + 0x21),*(DAT_005b8c3c + 0x25),
                         *(DAT_005b8c3c + 0x29),0xce,0xca,0xcc,0x1);
        }
        FUN_0049536f();
        return 0x0;
    }
    if (param_2 < 0x40c) {
        if (param_2 != 0x407) {
            return 0x0;
        }
        if (param_3 < 0x6c) {
            if (param_3 != 0x65) {
                return 0x0;
            }
        }
        else {
            if ((0x6c < param_3) && (param_3 != 0x6d)) {
                return 0x0;
            }
        }
        FUN_0049c140(param_1,param_3);
        return 0x0;
    }
    if (0x40c < param_2) {
        if (param_2 != 0x411) {
            return 0x0;
        }
        return 0x1;
    }
    FUN_004953d7();
    local_1c = DAT_005b2d44;
    if (DAT_005b2d44 < 0x73d2) {
        if (DAT_005b2d44 < 0x738d) {
            if (0x7373 < DAT_005b2d44) {
                if (DAT_005b2d44 < 0x7375) {
                    FUN_0049c2e0(local_5c,s_pcx_evwrit1_pcx_004c1f6d);^
                    // goto LAB_00456428;
                }
                if (DAT_005b2d44 == 0x7376) {
                    FUN_0049c2e0(local_5c,s_pcx_evfamn_pcx_004c1f4f);^
                    // goto LAB_00456428;
                }
            }
        }
        else {
            if (DAT_005b2d44 < 0x738e) {
                FUN_0049c2e0(local_5c,s_pcx_evplag_pcx_004c1f5e);^
                // goto LAB_00456428;
            }
            if (0x73cf < DAT_005b2d44) {
                if (DAT_005b2d44 < 0x73d1) {
                    FUN_0049c2e0(local_5c,s_pcx_evsymat_pcx_004c1f7d);
                }
                else {
                    FUN_0049c2e0(local_5c,s_pcx_evinqui_pcx_004c1f8d);
                }^
                // goto LAB_00456428;
            }
        }
    }
    else {
        if (DAT_005b2d44 < 0x73d3) {
            FUN_0049c2e0(local_5c,s_pcx_evleag_pcx_004c1f9d);^
            // goto LAB_00456428;
        }
        if (DAT_005b2d44 < 0x73ec) {
            if (DAT_005b2d44 < 0x73d4) {
                FUN_0049c2e0(local_5c,s_pcx_evsym3_pcx_004c1fac);^
                // goto LAB_00456428;
            }
            if (DAT_005b2d44 == 0x73d4) {
                FUN_0049c2e0(local_5c,s_pcx_evlab2_pcx_004c1fbb);^
                // goto LAB_00456428;
            }
        }
        else {
            if (DAT_005b2d44 < 0x73ed) {
                FUN_0049c2e0(local_5c,s_pcx_evassa_pcx_004c1fca);^
                // goto LAB_00456428;
            }
            if (DAT_005b2d44 < 0x73ee) {
                FUN_0049c2e0(local_5c,s_pcx_evassb_pcx_004c1fd9);^
                // goto LAB_00456428;
            }
            if (DAT_005b2d44 < 0x73ef) {
                FUN_0049c2e0(local_5c,s_pcx_evassc_pcx_004c1fe8);^
                // goto LAB_00456428;
            }
            if (DAT_005b2d44 == 0x73f0) {
                FUN_0049c2e0(local_5c,s_pcx_lose_bck_pcx_004c1ff7);^
                // goto LAB_00456428;
            }
        }
    }
    FUN_0049c2e0(local_5c,s_pcx_bg0_pcx_004c2008);
    LAB_00456428:
        FUN_004a08c5(local_5c,0x0,0x0,0x280,0x1e0,0x0,0x0,0x0,0x0);
    FUN_0049536f();
    return 0x1;
}



// WARNING: Could not reconcile some variable overlaps

fn FUN_004566ac(param_1: *mut i32) -> u32

{
    let sVar1: i16;
    let mut uVar2: u32;
    let mut iVar3: i32;
    short *psVar4;
    let mut local_24c: u32;
    let mut local_23c: u32;
    let local_22c: u8 [0x204];
    let mut local_28: i32;
    let local_24: *mut i32;;
    let mut local_20: u32;
    let mut local_1c: u32;
    let mut local_18: u32;

    DAT_005b2d48 = 0xffffffff;
    local_28 = 0x0;
    while ((local_28 < 0x5 && ((param_1[0x2] & 0x1 << ((byte)local_28 & 0x1f)) == 0x0))) {
local_28 = local_28 + 0x1;
}
    local_24 = local_22c;
    local_20 = 0x0;
    local_1c = 0x0;
    local_18 = 0x0;
    if ((*param_1 < 0x1068) || (0x1077 < *param_1)) {
        if ((*param_1 < 0x109a) || (0x10a9 < *param_1)) {
            if (0x171 < *param_1 - 0x735dU) {
                DAT_005b2d48 = 0xffffffff;
                return 0x0;
            }
            iVar3 = 0x17;
            psVar4 = &DAT_00456a63;
            loop {
                if (iVar3 == 0x0) break;
                iVar3 = iVar3 + -0x1;
                sVar1 = *psVar4;
                psVar4 = psVar4 + 0x1;
            } while ((*param_1 - 0x735dU) != sVar1);
            // WARNING: Could not recover jumptable at 0x00456b1f. Too many branches
            // WARNING: Treating indirect jump as call
            uVar2 = ((&UNK_00456a8f + iVar3 * 0x4))();
            return uVar2;
        }
        local_24c = (((&DAT_00569b98)[(*param_1 + -0x109a) * 0x3 + local_28 * 0x1e22] & 0x10) != 0x0);
        DAT_005b2d48 = local_24c;
    }
    else {
        local_23c = (((&DAT_00569b68)[(*param_1 + -0x1068) * 0x3 + local_28 * 0x1e22] & 0x10) != 0x0);
        DAT_005b2d48 = local_23c;
    }
    local_24 = param_1 + 0x7;
    local_18 = 0x4;
    uVar2 = FUN_00456c19(param_1[0x1],local_24,0x4);
    if (DAT_005b2d48 != 0xffffffff) {
        if ((*param_1 < 0x1068) || (0x1077 < *param_1)) {
            if ((0x1099 < *param_1) && (*param_1 < 0x10aa)) {
                if (DAT_005b2d48 == 0x0) {
                    (&DAT_00569b98)[(*param_1 + -0x109a) * 0x3 + local_28 * 0x1e22] =
                        (&DAT_00569b98)[(*param_1 + -0x109a) * 0x3 + local_28 * 0x1e22] & 0xef;
                }
                else {
                    (&DAT_00569b98)[(*param_1 + -0x109a) * 0x3 + local_28 * 0x1e22] =
                        (&DAT_00569b98)[(*param_1 + -0x109a) * 0x3 + local_28 * 0x1e22] | 0x10;
                }
            }
        }
        else {
            if (DAT_005b2d48 == 0x0) {
                (&DAT_00569b68)[(*param_1 + -0x1068) * 0x3 + local_28 * 0x1e22] =
                    (&DAT_00569b68)[(*param_1 + -0x1068) * 0x3 + local_28 * 0x1e22] & 0xef;
            }
            else {
                (&DAT_00569b68)[(*param_1 + -0x1068) * 0x3 + local_28 * 0x1e22] =
                    (&DAT_00569b68)[(*param_1 + -0x1068) * 0x3 + local_28 * 0x1e22] | 0x10;
            }
        }
    }
    return uVar2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00456c19(param_1: i32,param_2: &mut String,param_3: u32) -> u32

{
    let mut uVar1: u32;
    let mut pcVar2: String;
    let mut local_120: *mut u32 [0x11];
    let ppuStack219: *mut *mut u8;;
    let mut local_2f: String;;
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_20: i32;
    let local_1c: *mut i32;;
    let mut local_18: i32;

    local_1c = FUN_004990e0(local_120,0x0,s_diplo_res_004c202e,s_MessTextDlg_004c2022);
    _DAT_005b2d2c = FUN_004a3840(param_2,&DAT_005b2cf0,0x168,0x8,&local_20,LPCSTR_005b9218,0x0);
    DAT_005b2d34 = param_1;
    DAT_004c93c8 = FUN_0049c2c9(0xc350);
    local_18 = param_1;
    switch(param_1) {
    case 0x0:
        case 0x1:
        case 0x2:
        case 0x3:
        case 0x4:
        FUN_0049c60b(s_bin_portrait_bin_004c2049,DAT_004c93c8,0xc350,
    *(&DAT_00569b60 + param_1 * 0x1e22) * 0x3d090 + param_1 * 0xc350);
    break;
    case 0x5:
        FUN_0049c57b(s_bin_ptratlea_bin_004c206b,DAT_004c93c8,0xc350);
    break;
    case 0x6:
        FUN_0049c60b(s_bin_ptratchu_bin_004c205a,DAT_004c93c8,0xc350,((DAT_005827f4 >> 0x10) + -0x5) * 0xc350);
    break;
    case 0x8:
        FUN_0049c57b(s_bin_ptratvau_bin_004c207c,DAT_004c93c8,0xc350);
    break;
    case -0x1:
        FUN_0049c57b(s_bin_ptrattut_bin_004c2038,DAT_004c93c8,0xc350);
}
    if ((param_3 & 0x1) == 0x0) {
        FUN_0049bf80(local_120,0x66,0x414,0x0,0x0);
        FUN_0049bf80(local_120,0x66,0x410,0x0,0x0);
    }
    else {
        uVar1 = param_3 >> 0x4 & 0xf;
        if (uVar1 != 0x0) {
            pcVar2 = FUN_00499050(DAT_005967a0,uVar1);
            FUN_0049bf80(local_120,0x66,0x40f,0x0,pcVar2);
        }
    }
    if ((param_3 & 0x2) == 0x0) {
        FUN_0049bf80(local_120,0x67,0x414,0x0,0x0);
        FUN_0049bf80(local_120,0x67,0x410,0x0,0x0);
    }
    if ((param_3 & 0x4) == 0x0) {
        FUN_0049bf80(local_120,0x6e,0x414,0x0,0x0);
        FUN_0049bf80(local_120,0x6e,0x410,0x0,0x0);
        FUN_0049bf80(local_120,0x3e8,0x414,0x0,0x0);
        FUN_0049bf80(local_120,0x3e8,0x410,0x0,0x0);
        FUN_0049bf80(local_120,0x7d0,0x414,0x0,0x0);
        FUN_0049bf80(local_120,0x7d0,0x410,0x0,0x0);
    }
    else {
        if (DAT_005b2d48 != -0x1) {
            FUN_0049bf80(local_120,0x6e,0x502,DAT_005b2d48,0x0);
        }
    }
    local_24 = FUN_0049bb50(local_120,FUN_00457028);
    FUN_0049af50(DAT_004c93c8);
    local_28 = local_24;
    ppuStack219 = &PTR_FUN_004c3d34;
    if (local_2f != 0x0) {
        FUN_00499b30(local_120,local_2f);
    }
    FUN_0049a1c0(local_120,0x1);
    return local_28;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00457028(param_1: *mut *mut u32,param_2: u32,param_3: u32,param_4: i32) -> u32

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
        if (0xff < param_2) {
            if (param_2 < 0x101) {
                if ((param_4 != 0x0) && (iVar2 = FUN_004a11c0(param_4), iVar2 == 0x1b)) {
                    return 0x1;
                }
                return 0x0;
            }
            if (param_2 == 0x401) {
                FUN_0049bf80(param_1,0x6e,0x502,0x1,0x0);
                if (DAT_005b2d3c == 0x0) {
                    FUN_0049bf80(param_1,0x6d,0x410,0x0,0x0);
                }
                if (_DAT_005b2d38 == 0x0) {
                    FUN_0049bf80(param_1,0x6c,0x410,0x0,0x0);
                }
            }
        }
        return 0x0;
    }
    if (param_2 < 0x406) {
        FUN_004953d7();
        iVar2 = 0x8 - _DAT_005b2d2c;
        for (local_1c = 0x0; local_1c < _DAT_005b2d2c; local_1c = local_1c + 0x1) {
            FUN_00497567(0x1c2,local_1c * 0x14 + (iVar2 / 0x2 + 0x1) * 0x14 + 0x64,*(&DAT_005b2cf0 + local_1c * 0x6),
                         *(&DAT_005b2cf2 + local_1c * 0x6) >> 0x10,0xcaccce,-0x1,0xcaccce,LPCSTR_005b9218
                         ,0x1);
        }
        switch(DAT_005b2d34) {
            default:
                FUN_00496ac0((&DAT_00595700 + DAT_005b2d34 * 0x4),0x55,0xa5,0x64,0x64);
            FUN_0049e640(0x55,0xa5,0x64,0x64,0xce,0xca,0xcc,0x1);
            FUN_004968e7(0xa,0x13b,0xfa,0x14,0xe);
            FUN_0049e640(0xa,0x13b,0xfa,0x14,0xce,0xca,0xcc,0x1);
            FUN_00497567(0x87,0x13e,(&DAT_00569b50 + DAT_005b2d34 * 0x1e22),0xf0,0xcaccce,0xe0e0e,0xcaccce,
                         LPCSTR_005b9218,0x11);
            break;
            case -0x1:
                case 0x0:
                case 0x1:
                case 0x2:
                case 0x3:
                case 0x4:
                case 0x5:
                case 0x6:
                case 0x8:
                FUN_00496ac0(DAT_004c93c8,0xa,0x73,0xfa,0xc8);
            FUN_0049e640(0xa,0x73,0xfa,0xdc,0xce,0xca,0xcc,0x1);
            FUN_004968e7(0xa,0x13b,0xfa,0x14,0xe);
            if (DAT_005b2d34 == -0x1) {
                bVar7 = 0x11;
                uVar5 = 0xcaccce;
                iVar4 = 0xe0e0e;
                uVar3 = 0xcaccce;
                iVar2 = 0xf0;
                puVar6 = LPCSTR_005b9218;
                pcVar1 = FUN_00499050(DAT_005967ac,0x2710);
                FUN_00497567(0x87,0x13e,pcVar1,iVar2,uVar3,iVar4,uVar5,puVar6,bVar7);
            }
            else {
                FUN_00497567(0x87,0x13e,(&DAT_00569b50 + DAT_005b2d34 * 0x1e22),0xf0,0xcaccce,0xe0e0e,0xcaccce,
                             LPCSTR_005b9218,0x11);
            }
        }
        FUN_0049536f();
        return 0x1;
    }
    if (0x406 < param_2) {
        if (0x407 < param_2) {
            if (param_2 != 0x411) {
                return 0x0;
            }
            return 0x1;
        }
        if (param_3 < 0x6c) {
            if (param_3 < 0x66) {
                if (param_3 != 0x65) {
                    return 0x1;
                }
                FUN_0049c140(param_1,0x65);
                return 0x1;
            }
            if ((0x66 < param_3) && (param_3 != 0x67)) {
                return 0x1;
            }
        }
        else {
            if ((0x6c < param_3) && (0x6d < param_3)) {
                if ((0x6e < param_3) && (param_3 != 0x7d0)) {
                    return 0x1;
                }
                DAT_005b2d48 = DAT_005b2d48 ^ 0x1;
                if (param_3 != 0x7d0) {
                    return 0x1;
                }
                FUN_0049bf80(param_1,0x6e,0x502,DAT_005b2d48,0x0);
                return 0x1;
            }
        }
        *(param_1 + 0x2d) = *(param_1 + 0x2d) | 0x1;
        FUN_0049c140(param_1,param_3);
        return 0x1;
    }
    return 0x0;
}



fn FUN_004574d6(char **param_1) -> i32

{
let puVar1: *mut u32;
let mut uVar2: u32;
let mut pcVar3: String;
let mut local_58: u32;
let mut local_54: u32;
let mut local_50: u32;
let mut local_4c: u32;
let mut local_48: u32;
let mut local_44: u32;
let mut local_40: u32;
let mut local_3c: u32;
let mut local_34: i32;
let mut local_30: u32;
let mut local_2c: u32;
let mut local_28: u32;
let mut local_24: i32;
let mut local_20: i32;
let local_1c: *mut u32;
let local_18: u16;
let local_14: i16;

local_20 = 0x0;
for (local_24 = 0x0; local_24 < 0x5; local_24 = local_24 + 0x1) {
puVar1 = FUN_00455143();
(&DAT_00599c6c + local_24 * 0x4) = puVar1;
}
uVar2 = FUN_004a7970(&local_34,0x4,0x1,param_1);
local_3c = (uVar2 == 0x0);
local_20 = local_20 + local_3c;
while (local_34 != 0xff) {
uVar2 = FUN_004a7970(&local_30,0x4,0x1,param_1);
local_40 = (uVar2 == 0x0);
local_20 = local_20 + local_40;
uVar2 = FUN_004a7970(&local_2c,0x4,0x1,param_1);
local_44 = (uVar2 == 0x0);
local_20 = local_20 + local_44;
uVar2 = FUN_004a7970(&local_28,0x4,0x1,param_1);
local_48 = (uVar2 == 0x0);
local_20 = local_20 + local_48;
uVar2 = FUN_004a7970(&local_14,0x2,0x1,param_1);
local_4c = (uVar2 == 0x0);
local_20 = local_20 + local_4c;
uVar2 = FUN_004a7970(&local_18,0x2,0x1,param_1);
local_50 = (uVar2 == 0x0);
local_20 = local_20 + local_50;
if (local_14 == 0x0) {
local_1c = 0x0;
}
else {
local_1c = FUN_0049c2c9(local_14);
if (local_1c == 0x0) {
pcVar3 = FUN_00499050(DAT_005b9bd8,0x7d01);
pop_err_msg_box_and_exit_004a02f5(pcVar3);
}
uVar2 = FUN_004a7970(local_1c,0x1,local_14,param_1);
local_54 = (uVar2 == 0x0);
local_20 = local_20 + local_54;
}
FUN_0045518a(local_2c,local_30,local_34,local_28,local_1c,local_14,local_18);
if (local_1c != 0x0) {
FUN_0049af50(local_1c);
}
uVar2 = FUN_004a7970(&local_34,0x4,0x1,param_1);
local_58 = (uVar2 == 0x0);
local_20 = local_20 + local_58;
}
return local_20;
}



fn FUN_00457721(param_1: *mut i32) -> i32

{
let bVar1: u8;
let mut uVar2: u32;
let mut local_5c: u32;
let mut local_58: u32;
let mut local_54: u32;
let mut local_50: u32;
let mut local_4c: u32;
let mut local_48: u32;
let mut local_44: u32;
let mut local_40: u32;
let local_20: *mut i32;;
let mut local_1c: u32;
let mut local_18: i32;
let mut local_14: i32;

local_1c = 0xff;
local_18 = 0x0;
local_14 = 0x0;
loop {
if (0x4 < local_14) {
uVar2 = FUN_004a7160(&local_1c,0x4,0x1,param_1);
local_5c = (uVar2 == 0x0);
return local_18 + local_5c;
}
for (local_20 = *(i32 **)(*(&DAT_00599c6c + local_14 * 0x4) + 0x18); local_20 != 0x0;
local_20 = local_20[0x6]) {
if ((*local_20 < 0x1068) || (0x1077 < *local_20)) {
if ((0x1099 < *local_20) && (*local_20 < 0x10aa)) {
bVar1 = (&DAT_00569b98)[(*local_20 + -0x109a) * 0x3 + local_14 * 0x1e22];^
// goto joined_r0x00457834;
}
// LAB_0045783b:
uVar2 = FUN_004a7160(local_20,0x4,0x1,param_1);
local_40 = (uVar2 == 0x0);
local_18 = local_18 + local_40;
uVar2 = FUN_004a7160(local_20 + 0x1,0x4,0x1,param_1);
local_44 = (uVar2 == 0x0);
local_18 = local_18 + local_44;
uVar2 = FUN_004a7160(local_20 + 0x2,0x4,0x1,param_1);
local_48 = (uVar2 == 0x0);
local_18 = local_18 + local_48;
uVar2 = FUN_004a7160(local_20 + 0x3,0x4,0x1,param_1);
local_4c = (uVar2 == 0x0);
local_18 = local_18 + local_4c;
uVar2 = FUN_004a7160(local_20 + 0x4,0x2,0x1,param_1);
local_50 = (uVar2 == 0x0);
local_18 = local_18 + local_50;
uVar2 = FUN_004a7160((local_20 + 0x12),0x2,0x1,param_1);
local_54 = (uVar2 == 0x0);
local_18 = local_18 + local_54;
if ((local_20 + 0x4) != 0x0) {
uVar2 = FUN_004a7160(local_20 + 0x7,0x1,*(local_20 + 0xe) >> 0x10,param_1);
local_58 = (uVar2 == 0x0);
local_18 = local_18 + local_58;
}
}
else {
bVar1 = (&DAT_00569b68)[(*local_20 + -0x1068) * 0x3 + local_14 * 0x1e22];
joined_r0x00457834:
if ((bVar1 & 0x10) != 0x0)^ // goto LAB_0045783b;
}
}
local_14 = local_14 + 0x1;
} while( true );
}



// WARNING: Could not reconcile some variable overlaps

fn FUN_004579ea(param_1: i32) -> u32

{
    let mut bVar1: bool;
    undefined3 extraout_var;
    let mut iVar2: i32;
    let mut pcVar3: String;
    let local_54: u8;
    let mut local_50: u32;
    let mut local_48: i32;
    let mut local_44: i32;
    let mut local_40: i32;
    let mut local_3c: i32;
    let mut local_38: i32;
    let mut local_34: i32;
    let local_30: *mut u32;
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_24: i32;
    i32 **local_20;
    let mut local_1c: i32;
    let mut local_18: u32;
    let mut local_14: u32;

    local_1c = param_1;
    local_14 = (*(param_1 + 0x4) == 0x0);
    local_18 = local_14;
    if (((local_14 == 0x0) && (bVar1 = FUN_00432fca(param_1,0x1), CONCAT31(extraout_var,bVar1) != 0x0)) &&
        (iVar2 = FUN_00432bd3(param_1), iVar2 != 0x0)) {
        iVar2 = (*(param_1 + 0x4) + 0x20);
        local_48 = *(&DAT_005b7076 + iVar2 * 0x4e) >> 0x10;
        local_44 = *(&DAT_005b7078 + iVar2 * 0x4e) >> 0x10;
        if ((*(*(param_1 + 0x4) + 0x3a) & 0x80) == 0x0) {
            local_54 = (*(param_1 + 0x4) + 0x26);
        }
        else {
            local_54 = (*(param_1 + 0x4) + 0x35);
        }
        local_40 = local_54;
        local_3c = FUN_004729ba(local_40,&local_48,&local_44,0x0);
        local_38 = FUN_00485ea2(iVar2,local_48,local_44,0x0);
        local_34 = FUN_00432c94(param_1);
        if (local_38 + local_34 < 0x15) {
            if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
                FUN_004a1651();
            }
            *(&DAT_005b707e + iVar2 * 0x4e + local_3c * 0x4) = 0x0;
            local_30 = 0x0;
            if ((((*((*(*(param_1 + 0x4) + 0x23) >> 0x18) * 0x1e22 + 0x569b77) & 0x1) != 0x0) &&
                (iVar2 = FUN_00432aca(param_1,0x2d), iVar2 != 0x0)) &&
                (local_30 = FUN_00481784((*(param_1 + 0x4) + 0x20),
                                         (*(param_1 + 0x4) + 0x22),
                                         (*(param_1 + 0x4) + 0x24)), local_30 != 0x0)) {
                local_2c = (*(param_1 + 0x4) + 0x20);
                local_28 = (*(param_1 + 0x4) + 0x22);
                local_24 = (*(param_1 + 0x4) + 0x24);
            }
            for (local_20 = *(i32 ***)(param_1 + 0x4); local_20 != (i32 **)0x0; local_20 = (i32 **)local_20[0x2]) {
                FUN_004841ea(local_20,(local_20 + 0x8),local_48,local_44);
                if ((*(local_20 + 0x3a) & 0x40) == 0x0) {
                    (local_20 + 0x2f) = (local_20 + 0x2f) + -0x1;
                }
                *(local_20 + 0x3a) = *(local_20 + 0x3a) & 0xfe;
                *(local_20 + 0x3a) = *(local_20 + 0x3a) | 0x20;
                FUN_00459e8f(local_20);
            }
            if (((local_30 != 0x0) && ((local_30 + 0xe) == 0x0)) &&
                ((iVar2 = FUN_00486432(0x2d,local_2c,local_28,local_24), iVar2 == 0x0 &&
                    ((FUN_004883c0((*(param_1 + 0x4) + 0x20),
                                   *(*(param_1 + 0x4) + 0x23) >> 0x18,-0x14),
                      ((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0 &&
                          ((*((*(*(param_1 + 0x4) + 0x23) >> 0x18) * 0x1e22 + 0x569b77) & 0x10) != 0x0)))))) {
                pcVar3 = FUN_00499050(DAT_0059679c,0x73f8);
                FUN_0049d2e0(0x0,0x1,pcVar3);
            }
            local_50 = 0x1;
        }
        else {
            if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
                pcVar3 = FUN_00499050(DAT_0059679c,0x714d);
                FUN_0049d2e0(0x0,0x1,pcVar3);
            }
            local_50 = 0x0;
        }
    }
    else {
        local_50 = 0x0;
    }
    return local_50;
}



fn FUN_00457f10(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: i32) -> u32

{
    i32 **ppiVar1;
    let mut pcVar2: String;
    let mut iVar3: i32;
    let mut uVar4: u32;
    let local_194: u8 [0x80];
    let mut local_114: String;
    let mut local_110: u32;
    let mut local_10c: i32;
    let mut local_108: i32;
    let mut local_104: *mut u8;
    let mut local_100: u32;
    let local_fc: *mut u32;
    let local_f8: *mut u32;
    let mut local_f4: u32;
    let local_f0: *mut u32;
    let local_ec: *mut u32;
    let mut local_e8: u32;
    let local_e4: *mut u32;
    let local_e0: *mut u32;
    let local_dc: *mut u32;
    let local_d8: *mut u32;
    let mut local_d4: u32;
    let mut local_d0: i32;
    let local_cc: *mut u32;
    let mut local_c8: i32;
    let mut local_c4: i32;
    let local_c0: u8 [0x80];
    let mut local_40: u32;
    let mut local_3c: u32;
    let mut local_38: u32;
    let local_34: *mut u32;
    let local_30: *mut u32;
    let mut local_2c: i32;
    let local_28: *mut u32;
    i32 **local_24;
    let mut local_20: i32;
    let mut local_1c: i32;
    i32 **local_18;
    i32 **local_14;

    local_3c = *(*(&DAT_004d7d50 + param_3 * 0x4 + param_2 * 0x3890) + param_4 * 0x4);
    local_38 = local_3c & 0xf;
    if ((local_38 == 0x0) || (local_38 == 0xa)) {
        if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
            pcVar2 = FUN_00499050(DAT_0059679c,0x714e);
            FUN_0049d2e0(0x0,0x1,pcVar2);
        }
        local_40 = 0x0;
    }
    else {
        local_34 = FUN_00481784(param_2,param_3,param_4);
        if ((local_34 == 0x0) ||
            ((((local_34 + 0xe) != 0xf && ((local_34 + 0xe) != 0x10)) &&
                ((local_34 + 0xe) != 0x2)))) {
            local_30 = 0x0;
            if ((local_34 == 0x0) || (iVar3 = FUN_00481a44(local_34), iVar3 != 0x0)) {
                local_30 = FUN_0045ad4e(param_2,param_3,param_4);
                if (local_30 == 0x0) {
                    local_2c = -0x1;
                }
                else {
                    local_2c = *(local_30 + 0x23) >> 0x18;
                }
            }
            else {
                local_2c = *(local_34 + 0xe) >> 0x10;
            }
            if (local_2c != -0x1) {
                if (local_2c == DAT_004c9754) {
                    if (local_30 != 0x0) {
                        if ((*(*(param_1 + 0x4) + 0x3a) & 0x80) == 0x0) {
                            if ((*(local_30 + 0x3a) & 0x80) != 0x0) {
                                if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
                                    pcVar2 = FUN_00499050(DAT_0059679c,0x715e);
                                    FUN_0049d2e0(0x0,0x1,pcVar2);
                                }
                                return 0x0;
                            }
                        }
                        else {
                            if (((*(local_30 + 0x3a) & 0x80) == 0x0) ||
                                ((*(param_1 + 0x4) + 0x35) != (local_30 + 0x35))) {
                                if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
                                    pcVar2 = FUN_00499050(DAT_0059679c,0x715e);
                                    FUN_0049d2e0(0x0,0x1,pcVar2);
                                }
                                return 0x0;
                            }
                        }
                    }
                }
                else {
                    local_d0 = 0x0;
                    for (local_cc = (&DAT_005b8b44 + param_2 * 0x4); local_cc != 0x0;
                        local_cc = *local_cc) {
                        local_dc = local_cc + 0x8;
                        local_d4 = local_dc & 0xffff0000 | local_dc;
                        local_d8 = local_dc;
                        if (local_dc != param_2) break;
                        local_e4 = local_cc + 0x8;
                        local_e8 = *(local_cc + 0x3a) & 0x1;
                        local_e0 = local_e4;
                        if (local_e8 != 0x0) {
                            local_f0 = local_cc + 0x8;
                            local_f4 = local_f0 & 0xffff0000 | (local_cc + 0x22);
                            local_ec = local_f0;
                            if ((local_cc + 0x22) == param_3) {
                                local_fc = local_cc + 0x8;
                                local_100 = local_fc & 0xffff0000 | (local_cc + 0x9);
                                local_f8 = local_fc;
                                if ((((local_cc + 0x9) == param_4) &&
                                    (*(*(&DAT_00582938 +
                                        (*(local_cc + 0x25) >> 0x18) * 0x4 +
                                        (local_cc[0x9] >> 0x18) * 0x18) + 0xad) == 0x0)) &&
                                    ((*(local_cc + 0x3a) & 0x2) == 0x0)) {
                                    local_d0 = 0x1;
                                    break;
                                }
                            }
                        }
                    }
                    if (local_d0 == 0x0) {
                        if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
                            iVar3 = FUN_00410fb3(param_1,0x0);
                            if (iVar3 == 0x0) {
                                return 0x0;
                            }
                            local_104 = &DAT_004d55a8;
                            local_108 = DAT_004c9754;
                            local_10c = local_2c;
                            local_110 = (byte)(&DAT_004d55a8)[DAT_004c9754 * 0xe + local_2c];
                            if (local_110 == 0x0) {
                                FUN_00499050(DAT_0059679c,local_2c + 0x414);
                                if (local_2c < 0x5) {
                                    local_114 = FUN_00499050(DAT_0059679c,0x713e);
                                }
                                else {
                                    local_114 = FUN_00499050(DAT_0059679c,0x713a);
                                }
                                pcVar2 = FUN_00499050(DAT_0059679c,0x734b);
                                FUN_0049c2e0(local_194,pcVar2);
                                uVar4 = FUN_0049d2e0(0x0,0x3,local_194);
                                if (uVar4 == 0x0) {
                                    return 0x0;
                                }
                            }
                            else {
                                if ((&DAT_004d55a8)[DAT_004c9754 * 0xe + local_2c] == '\x01') {
                                    FUN_00499050(DAT_0059679c,local_2c + 0x414);
                                    pcVar2 = FUN_00499050(DAT_0059679c,0x7365);
                                    FUN_0049c2e0(local_194,pcVar2);
                                    uVar4 = FUN_0049d2e0(0x0,0x3,local_194);
                                    if (uVar4 == 0x0) {
                                        return 0x0;
                                    }
                                }
                            }
                        }
                    }
                    else {
                        if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
                            pcVar2 = FUN_00499050(DAT_0059679c,0x7391);
                            FUN_0049d2e0(0x0,0x1,pcVar2);
                            return 0x0;
                        }
                    }
                }
                if (param_5 != 0x0) {
                    local_c8 = FUN_00485ea2(param_2,param_3,param_4,0x1);
                    local_c4 = FUN_00432c94(param_1);
                    if (0x14 < local_c8 + local_c4) {
                        if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
                            pcVar2 = FUN_00499050(DAT_0059679c,0x714d);
                            FUN_0049d2e0(0x0,0x1,pcVar2);
                        }
                        return 0x0;
                    }
                }
                if ((local_34 != 0x0) && (iVar3 = FUN_00481a44(local_34), iVar3 == 0x0)) {
                    if ((*(*(param_1 + 0x4) + 0x3a) & 0x80) == 0x0) {
                        if ((*(local_34 + 0x2d) & 0x1) != 0x0) {
                            if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
                                pcVar2 = FUN_00499050(DAT_0059679c,0x715e);
                                FUN_0049d2e0(0x0,0x1,pcVar2);
                            }
                            return 0x0;
                        }
                    }
                    else {
                        if (((*(local_34 + 0x2d) & 0x1) == 0x0) ||
                            (local_34[0x4] >> 0x10 != *(*(param_1 + 0x4) + 0x32) >> 0x18)) {
                            if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
                                pcVar2 = FUN_00499050(DAT_0059679c,0x715e);
                                FUN_0049d2e0(0x0,0x1,pcVar2);
                            }
                            return 0x0;
                        }
                    }
                }
            }
            local_28 = 0x0;
            if ((((*((*(*(param_1 + 0x4) + 0x23) >> 0x18) * 0x1e22 + 0x569b77) & 0x1) != 0x0) &&
                (iVar3 = FUN_00432aca(param_1,0x2d), iVar3 != 0x0)) &&
                (iVar3 = FUN_00486432(0x2d,param_2,param_3,param_4), iVar3 == 0x0)) {
                local_28 = FUN_00481784(param_2,param_3,param_4);
            }
            if (((local_34 == 0x0) || (iVar3 = FUN_00481a44(local_34), iVar3 != 0x0)) &&
                (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0)) {
                pcVar2 = FUN_00499050(DAT_0059679c,0x73ad);
                uVar4 = FUN_0049d2e0(0x0,0x3,pcVar2);
                if (uVar4 == 0x0) {
                    return 0x0;
                }
            }
            if (((local_2c != -0x1) && (local_2c != DAT_004c9754)) && (local_2c == 0x8)) {
                FUN_00423530(DAT_004c9754);
            }
            for (local_24 = *(i32 ***)(param_1 + 0x4); local_24 != (i32 **)0x0; local_24 = *(i32 ***)(local_24 + 0x8)) {
                if ((*(local_24 + 0x3a) & 0x40) == 0x0) {
                    (local_24 + 0x2f) = (local_24 + 0x2f) + -0x1;
                }
            }
            for (local_20 = 0x0; local_20 < 0xe; local_20 = local_20 + 0x1) {
                if ((local_20 != DAT_004c9754) && ((&DAT_004d55a8)[local_20 * 0xe + DAT_004c9754] == '\x02')) {
                    FUN_0040c15d(param_3,param_4,local_20,0x1);
                    FUN_00431d5a(&DAT_005967b8,&DAT_005967c0);
                }
            }
            DAT_00599d3c = 0x1;
            local_1c = 0x0;
            local_24 = DAT_005967bc;
            ppiVar1 = local_24;
            while (local_24 = ppiVar1, local_24 != (i32 **)0x0) {
                for (local_18 = (i32 **)local_24[0x2];
                    (local_18 != (i32 **)0x0 && ((*(local_18 + 0x3a) & 0x40) != 0x0));
                    local_18 = (i32 **)local_18[0x2]) {
                }
                ppiVar1 = local_18;
                if (((*(local_24 + 0x3a) & 0x40) == 0x0) &&
                    (((*(local_24 + 0x3a) & 0x80000000) != 0x0 ||
                        (ppiVar1 = local_18, (*(local_24 + 0x3b) & 0x20) != 0x0)))) {
                    *(local_24 + 0x3b) = *(local_24 + 0x3b) & 0xdf;
                    FUN_00431dec(&DAT_005967b8,local_24);
                    ppiVar1 = local_18;
                }
            }
            local_24 = DAT_005967bc;
            ppiVar1 = local_24;
            while (local_24 = ppiVar1, local_24 != (i32 **)0x0) {
                local_18 = (i32 **)local_24[0x2];
                ppiVar1 = local_18;
                if ((*(local_24 + 0x3a) & 0x80000000) == 0x0) {
                    if ((((local_34 == 0x0) || (iVar3 = FUN_00481a44(local_34), iVar3 != 0x0)) &&
                        ((local_24 + 0x2a) != 0x9)) && ((*(local_24 + 0x3a) & 0x40) == 0x0)) {
                        if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
                            uVar4 = FUN_004a2edc();
                            FUN_00488efd(local_24,uVar4 % 0x28 + 0x1);
                        }
                        else {
                            uVar4 = FUN_004a2edc();
                            FUN_00488efd(local_24,uVar4 % 0x5 + 0x1);
                        }
                    }
                    ppiVar1 = local_18;
                    if ((*(local_24 + 0x3a) & 0x80000000) == 0x0) {
                        local_14 = local_24;
                        if ((*(local_24 + 0x3a) & 0x40) == 0x0) {
                            local_1c = 0x1;
                        }
                        FUN_004841ea(local_24,(local_24 + 0x8),param_3,param_4);
                        *(local_24 + 0x3a) = *(local_24 + 0x3a) | 0x21;
                        if (((&DAT_00569a98)[(*(local_24 + 0x23) >> 0x18) * 0x1e22] & 0x2) == 0x0) {
                            (local_24 + 0x42) = 0xff;
                            (local_24 + 0x41) = (local_24 + 0x42);
                        }
                        FUN_00459e8f(local_24);
                        FUN_0044add9(local_24);
                        ppiVar1 = local_18;
                    }
                }
            }
            if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
                FUN_004a1651();
            }
            FUN_004864f7();
            if (local_1c == 0x0) {
                FUN_00431d0a(&DAT_005967b8);
            }
            else {
                if ((local_2c != -0x1) && (local_2c != DAT_004c9754)) {
                    FUN_004112d1(DAT_004c9754,local_2c);
                    FUN_0048616e(param_2,param_3,param_4,DAT_004c9754,*(DAT_005967bc + 0x32) >> 0x18);
                    if ((local_34 != 0x0) && (iVar3 = FUN_00481a44(local_34), iVar3 == 0x0)) {
                        FUN_00481fde(&DAT_005967b8,param_3,param_4);
                    }
                }
                DAT_005967bc = (i32 **)FUN_00434de1(local_14);
                FUN_00450dbf(&DAT_00595740,param_2,param_3,param_4,DAT_005967bc,0x1,-0x1,0x0);
            }
            if (((local_28 != 0x0) && ((local_28 + 0xe) == 0x0)) &&
                ((FUN_004883c0((*(param_1 + 0x4) + 0x20),*(*(param_1 + 0x4) + 0x23) >> 0x18
                               ,0x14), ((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0 &&
                      ((*((*(*(param_1 + 0x4) + 0x23) >> 0x18) * 0x1e22 + 0x569b77) & 0x10) != 0x0)))) {
                pcVar2 = FUN_00499050(DAT_0059679c,0x73f9);
                FUN_0049d2e0(0x0,0x1,pcVar2);
            }
            local_40 = 0x1;
        }
        else {
            if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
                iVar3 = FUN_00430d15(&DAT_005831e8 + (local_34[0x3] >> 0x10) * 0x50);
                if (iVar3 == 0x0) {
                    FUN_00499050(DAT_0059679c,0x7132);
                    pcVar2 = FUN_00499050(DAT_0059679c,0x73ca);
                    FUN_0049c2e0(local_c0,pcVar2);
                }
                else {
                    FUN_00499050(DAT_0059679c,0x7133);
                    pcVar2 = FUN_00499050(DAT_0059679c,0x73ca);
                    FUN_0049c2e0(local_c0,pcVar2);
                }
                FUN_0049d2e0(0x0,0x1,local_c0);
            }
            local_40 = 0x0;
        }
    }
    return local_40;
}



fn FUN_00458e3a(param_1: i32,param_2: *mut *mut u32,param_3: i32)

{
    let mut uVar1: u32;

    if (DAT_005967bc == 0x0) {
        FUN_0049c140(param_2,0x68);
    }
    else {
        if ((*(param_1 + 0xa9) == (DAT_005967bc + 0x20)) &&
            ((*(DAT_005967bc + 0x3a) & 0x1) != 0x0)) {
            if (((((*(DAT_005967bc + 0x3a) & 0x1) != 0x0) && ((DAT_005967bc + 0x41) != -0x1)) &&
                (*(DAT_005967bc + 0x23) >> 0x18 == DAT_004c9754)) &&
                (uVar1 = FUN_0045af67(&DAT_005967b8,(DAT_005967bc + 0x22),
                                      (DAT_005967bc + 0x24),*(DAT_005967bc + 0x41),
                                      *(DAT_005967bc + 0x42),0x0), uVar1 != 0x0)) {
                FUN_00449654(param_1,uVar1,(DAT_005967bc + 0x22),(DAT_005967bc + 0x24),
                             *(DAT_005967bc + 0x41),*(DAT_005967bc + 0x42));
            }
            FUN_00450dbf(&DAT_00595740,(DAT_005967bc + 0x20),(DAT_005967bc + 0x22),
                         (DAT_005967bc + 0x24),DAT_005967bc,0x1,-0x1,0x0);
            FUN_0042e871(param_2);
            FUN_0048fe33(&DAT_00595740,0x0);
            FUN_004a756b();
            FUN_00483978();
            FUN_004a75a6();
        }
        else {
            if ((param_3 == 0x0) || (FUN_00431f8d(&DAT_005967b8,DAT_004c9754), DAT_005967bc != 0x0)) {
                FUN_0049c140(param_2,0x67);
            }
            else {
                FUN_0049c140(param_2,0x68);
            }
        }
    }
    return;
}



fn FUN_004591f7(param_1: i32,param_2: *mut *mut u32) -> u32

{
    FUN_00431f8d(&DAT_005967b8,DAT_004c9754);
    FUN_0043a597(param_1);
    FUN_00458e3a(param_1,param_2,0x0);
    return 0x1;
}



fn FUN_00459247(param_1: u32,param_2: *mut *mut u32)

{
    let mut local_3c: i32;
    let mut local_38: i32;
    let mut local_34: i32;
    let mut local_30: u32;
    let mut local_2c: u32;
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_20: u32;
    let mut local_1c: u32;
    let mut local_18: u32;
    let local_14: u8;

    local_2c = DAT_005967bc + 0x20;
    local_1c = local_2c & 0xffff0000 | (DAT_005967bc + 0x22);
    local_3c = (DAT_005967bc + 0x22);
    local_24 = DAT_005967bc + 0x20;
    local_18 = local_24 & 0xffff0000 | (DAT_005967bc + 0x24);
    local_38 = (DAT_005967bc + 0x24);
    if ((*(DAT_005967bc + 0x3a) & 0x80) == 0x0) {
        local_14 = (DAT_005967bc + 0x26);
    }
    else {
        local_14 = (DAT_005967bc + 0x35);
    }
    local_34 = local_14;
    local_28 = local_2c;
    local_20 = local_24;
    FUN_004729ba(local_34,&local_3c,&local_38,0x1);
    FUN_0047ed66();
    local_30 = FUN_004710a6(&DAT_005967b8,local_3c,local_38,*(DAT_005967bc + 0x41),
                            *(DAT_005967bc + 0x42));
    if (local_30 != 0x0) {
        FUN_0047eda9(param_1,local_30,(DAT_005967bc + 0x22),(DAT_005967bc + 0x24),
                     *(DAT_005967bc + 0x41),*(DAT_005967bc + 0x42));
        FUN_00471bcd(DAT_005b8850);
        FUN_00471bcd(DAT_005b8854);
    }
    FUN_00450dbf(&DAT_00595740,(DAT_005967bc + 0x20),(DAT_005967bc + 0x22),
                 (DAT_005967bc + 0x24),DAT_005967bc,0x0,-0x1,0x0);
    FUN_0042e871(param_2);
    FUN_0048fe33(&DAT_00595740,0x0);
    FUN_004a756b();
    FUN_00483978();
    FUN_004a75a6();
    return;
}



// WARNING: Removing unreachable block (ram,0x004598b8)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00459479(param_1: i32,param_2: *mut *mut u32,param_3: i32,param_4: i32) -> u32

{
    let mut uVar1: u32;
    let mut bVar2: bool;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut pcVar5: String;
    let mut uVar6: u32;
    let mut local_44: u32;

    bVar2 = false;
    if ((param_4 != 0x0) && (param_3 != -0x1)) {
        FUN_0043234c(&DAT_005967b8,param_3);
    }
    loop {
    if ((param_3 == -0x1) && (!bVar2)) {
        if (DAT_005967bc == 0x0) {
            if (_DAT_00596a48 == 0x0) {
                pcVar5 = FUN_00499050(DAT_0059679c,0x73a6);
                uVar6 = FUN_0049d2e0(param_2,0x3,pcVar5);
                if (uVar6 != 0x0) {
                    _DAT_00596a48 = 0x1;
                    FUN_0049a770(param_2,0x407,0x51b,0x0);
                    return 0x65;
                }
            }
            _DAT_00596a48 = 0x1;
            FUN_00431f8d(&DAT_005967b8,DAT_004c9754);
        }
        if (DAT_005967bc == 0x0) {
            return 0x65;
        }
    }
    if (((!bVar2) && (param_3 == -0x1)) && ((*(DAT_005967bc + 0x3a) & 0x1) == 0x0)) {
        FUN_00459247(param_1,param_2);
        return 0x64;
    }
    if (bVar2) {
        param_3 = *(DAT_004c50a8 + 0x6) >> 0x10;
    }
    else {
        if (param_3 == -0x1) {
            param_3 = (DAT_005967bc + 0x20);
        }
    }
    DAT_005b7068 = param_3;
    DAT_005b8808 = 0x0;
    iVar4 = FUN_0047d6a0(DAT_005b8850,*(&DAT_005b7076 + param_3 * 0x4e) >> 0x10,
                         *(&DAT_005b7078 + param_3 * 0x4e) >> 0x10,0x1);
    iVar3 = DAT_005b8854;
    if (iVar4 != 0x0) {
        uVar1 = *(param_1 + 0x4d);
        *(DAT_005b8854 + 0x59) = *(param_1 + 0x49);
        *(iVar3 + 0x5d) = uVar1;
        FUN_00471bcd(DAT_005b8854);
    }
    if (bVar2) {
        bVar2 = false;
        local_44 = FUN_0043bf7a(param_3,*(DAT_004c50a8 + 0x8) >> 0x10,*(DAT_004c50a8 + 0xa) >> 0x10);
    }
    else {
        local_44 = FUN_0043bf7a(param_3,-0x1,0xffffffff);
    }
    _DAT_004d555c = 0x0;
    _DAT_004d5558 = 0x0;
    if (local_44 < 0x69) {
        if (local_44 < 0x66) {
            if (local_44 == 0x65)^ // goto LAB_004597cf;
        }
        else {
            if (local_44 < 0x67)^ // goto LAB_004597f3;
            if (0x67 < local_44) {
                _DAT_00596a48 = 0x1;
                pcVar5 = FUN_00499050(DAT_0059679c,0x73a6);
                uVar6 = FUN_0049d2e0(param_2,0x3,pcVar5);
                if (uVar6 != 0x0) {
                    LAB_004597cf:
                        FUN_0049a770(param_2,0x407,0x51b,0x0);
                    return 0x65;
                }
                FUN_00431f8d(&DAT_005967b8,DAT_004c9754);
            }
        }
    }
    else {
        if (local_44 < 0x6a) {
            _DAT_004d5558 = 0x0;
            _DAT_004d555c = 0x0;
            return 0x69;
        }
        if (local_44 < 0x6b) {
            LAB_004597f3:
                FUN_0049c140(param_2,local_44);
            return local_44;
        }
        if (local_44 < 0x6c) {
            if (DAT_005967bc == 0x0) {
                bVar2 = true;
            }
        }
        else {
            if ((0x3e7 < local_44) && ((local_44 < 0x3e9 || (local_44 == 0x3e9))))^ // goto LAB_004597f3;
        }
    }
    param_3 = -0x1;
} while( true );
}



fn FUN_004598c9(param_1: i32,param_2: *mut *mut u32) -> u32

{
    FUN_0047ed66();
    FUN_00431f8d(&DAT_005967b8,DAT_004c9754);
    FUN_00459479(param_1,param_2,-0x1,0x0);
    return 0x0;
}



fn FUN_00459921(param_1: i32)

{
    let mut local_14: i32;

    for (local_14 = *(param_1 + 0x4); local_14 != 0x0; local_14 = *(local_14 + 0x8)) {
        *(local_14 + 0x3a) = *(local_14 + 0x3a) | 0x20;
        (local_14 + 0x4c) = DAT_00599e10 + '\x01';
    }
    return;
}



fn FUN_0045996c(param_1: i32)

{
    let mut local_14: i32;

    for (local_14 = *(param_1 + 0x4); local_14 != 0x0; local_14 = *(local_14 + 0x8)) {
        (local_14 + 0x4c) = DAT_00599e10 + '\x01';
    }
    return;
}



// WARNING: Restarted to delay deadcode elimination for space: stack

fn FUN_004599b0(param_1: u32,param_2: u32,param_3: u32) -> i32

{
let mut uVar1: u32;
let mut iVar2: i32;
let mut in_stack_0000001e: u32;
let local_90: u8;
let mut local_48: u32;
let mut local_44: u32;
let local_40: *mut u32;
let local_3c: *mut i32;;
let mut local_38: u32;
let local_34: *mut u32;
let mut local_30: *mut u8;
let mut local_2c: u32;
let local_28: *mut u32;
let local_20: *mut u32;
let mut local_1c: i32;
let local_18: *mut u32;
let mut local_14: u32;

local_1c = *(*(&DAT_00582938 + param_2._3_1_ * 0x18 + (char)param_3 * 0x4) + 0x4b) >> 0x10;
uVar1 = FUN_004a2edc();
local_1c = local_1c + uVar1 % 0x5 + -0x2;
local_18 = &param_1;
local_14 = in_stack_0000001e & 0x1;
if (local_14 != 0x0) {
local_28 = &param_1;
local_2c = local_28 & 0xffff0000 | param_1 & 0xffff;
local_30 = &DAT_004d7d50 + param_1 * 0x3890;
local_34 = &param_1;
local_38 = local_34 & 0xffff0000 | param_1._2_2_;
local_3c = (local_30 + param_1._2_2_ * 0x4);
local_40 = &param_1;
local_44 = local_40 & 0xffff0000 | param_2 & 0xffff;
local_48 = *(*local_3c + param_2 * 0x4);
local_20 = FUN_00481784(param_1,param_1._2_2_,param_2);
local_90 = 0x4;
local_1c = local_1c *
(*(&DAT_0059126e +
param_3._2_2_ * 0x2 +
*(&DAT_004daab0 + param_1 * 0x3890) * 0x14 + (local_48 & 0xf) * 0x64) >> 0x10);
while( true ) {
iVar2 = FUN_0043936e(param_1,&local_48);
if (iVar2 == 0x0) break;
local_1c = local_1c *
(*(&DAT_0059126e +
param_3._2_2_ * 0x2 +
*(&DAT_004daab0 + param_1 * 0x3890) * 0x14 + (local_48 & 0xf) * 0x64) >> 0x10);
local_90 = local_90 + 0x4;
}
if (local_20 != 0x0) {
local_1c = local_1c *
(*(&DAT_0059171e +
param_3._2_2_ * 0x2 + *(&DAT_004daab0 + param_1 * 0x3890) * 0x14) >> 0x10);
local_90 = local_90 + 0x4;
}
local_1c = (0x1 << (local_90 & 0x1f)) + -0x1 + local_1c >> (local_90 & 0x1f);
}
return local_1c;
}



fn FUN_00459ca5(param_1: i32,param_2: i32,param_3: i32,param_4: i32)

{
    let puVar1: *mut u32;
    byte *pbVar2;
    let puVar3: *mut u32;

    puVar1 = (*(&DAT_004d7d50 + param_1 * 0x3890 + param_2 * 0x4) + param_3 * 0x4 + 0x4);
    *puVar1 = *puVar1 | *(&DAT_004be9b0 + param_4 * 0x4);
    pbVar2 = (*(&DAT_004d7d50 + param_1 * 0x3890 + param_2 * 0x4) + param_3 * 0x4 + 0x4);
    *pbVar2 = *pbVar2 | 0x10;
    puVar3 = FUN_00481784(param_1,param_2,param_3);
    if (puVar3 != 0x0) {
        puVar3[0xb] = puVar3[0xb] | *(&DAT_004be9b0 + param_4 * 0x4);
    }
    return;
}



fn FUN_00459d35(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: i32)

{
    let puVar1: *mut u32;
    byte *pbVar2;
    let mut iVar3: i32;
    let mut uVar4: u32;
    let mut uVar5: u32;
    let mut local_20: i32;
    let local_1c: *mut u32;

    for (local_1c = (&DAT_005b89f8 + param_1 * 0x4);
        (local_1c != 0x0 && (*(local_1c + 0x6) >> 0x10 == param_1));
        local_1c = *local_1c) {
        if ((*(&DAT_004be9b0 + param_4 * 0x4) & local_1c[0xb]) == 0x0) {
            iVar3 = FUN_0044a87f(param_2,param_3,local_1c[0x2] >> 0x10,*(local_1c + 0xa) >> 0x10);
            if (iVar3 <= param_5) {
                local_1c[0xb] = local_1c[0xb] | *(&DAT_004be9b0 + param_4 * 0x4);
                puVar1 = (*(&DAT_004d7d50 + param_1 * 0x3890 + (local_1c[0x2] >> 0x10) * 0x4) +
                    (*(local_1c + 0xa) >> 0x10) * 0x4 + 0x4);
                *puVar1 = *puVar1 | *(&DAT_004be9b0 + param_4 * 0x4);
                for (local_20 = 0x0; local_20 < 0x6; local_20 = local_20 + 0x1) {
                    uVar4 = FUN_0043a8a2((local_1c[0x2] >> 0x10) + (&DAT_004bea60)[local_20]);
                    uVar5 = FUN_0043a8d5(uVar4,(*(local_1c + 0xa) >> 0x10) + (&DAT_004bea7c)[local_20]);
                    pbVar2 = (*(&DAT_004d7d50 + uVar4 * 0x4 + param_1 * 0x3890) + uVar5 * 0x4 + 0x4);
                    *pbVar2 = *pbVar2 | 0x10;
                }
            }
        }
    }
    return;
}



fn FUN_00459e8f(param_1: i32) -> u32

{
    let sVar1: i16;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut bVar5: bool;
    let mut iVar6: i32;
    let mut local_34: u32;
    let local_30: *mut u32;
    let mut local_28: i32;
    let mut local_20: i32;
    let mut local_1c: u32;

    uVar2 = *(&DAT_004be9b0 + (*(param_1 + 0x23) >> 0x18) * 0x4);
    iVar3 = *(*(&DAT_00582938 +
        (*(param_1 + 0x25) >> 0x18) * 0x4 + (*(param_1 + 0x24) >> 0x18) * 0x18) + 0x49)
    ;
    (param_1 + 0x3c) = (param_1 + 0x3c) & 0x1;
    *(param_1 + 0x3a) = *(param_1 + 0x3a) | uVar2;
    if (iVar3 == 0x0) {
        local_34 = 0x0;
    }
    else {
        local_1c = 0x0;
        bVar5 = false;
        if ((*(param_1 + 0x3a) & 0x1) == 0x0) {
            sVar1 = (param_1 + 0x20);
            for (local_30 = (&DAT_005b8b44 + sVar1 * 0x4);
                (local_30 != 0x0 && ((local_30 + 0x8) == sVar1));
                local_30 = *local_30) {
                if (((*(local_30 + 0x3a) & 0x1) == 0x0) &&
                    (((local_30 + 0x26) != (param_1 + 0x26) &&
                        ((*(local_30 + 0x3a) & 0x40) == 0x0)))) {
                    if ((*(local_30 + 0x3d) >> 0x18 <= iVar3) && ((uVar2 & *(local_30 + 0x3a)) == 0x0)) {
                        *(local_30 + 0x3a) = *(local_30 + 0x3a) | uVar2;
                        local_1c = 0x1;
                    }
                    if ((!bVar5) &&
                        (*(param_1 + 0x3d) >> 0x18 <=
                            *(*(&DAT_00582938 +
                                (*(local_30 + 0x25) >> 0x18) * 0x4 + (local_30[0x9] >> 0x18) * 0x18) +
                                0x49))) {
                        *(param_1 + 0x3a) =
                            *(param_1 + 0x3a) | *(&DAT_004be9b0 + (*(local_30 + 0x23) >> 0x18) * 0x4);
                        bVar5 = true;
                    }
                }
            }
        }
        else {
            if (iVar3 < 0x4) {
                local_28 = 0x1;
            }
            else {
                local_28 = (iVar3 >> 0x1) + -0x1;
            }
            FUN_00459d35((param_1 + 0x20),(param_1 + 0x22),(param_1 + 0x24),
                         *(param_1 + 0x23) >> 0x18,local_28);
            sVar1 = (param_1 + 0x20);
            for (local_30 = (&DAT_005b8b44 + sVar1 * 0x4);
                (local_30 != 0x0 && ((local_30 + 0x8) == sVar1));
                local_30 = *local_30) {
                if (((local_30 + 0x26) != (param_1 + 0x26)) &&
                    ((((!bVar5 || ((uVar2 & *(local_30 + 0x3a)) == 0x0)) &&
                        ((*(local_30 + 0x3a) & 0x1) != 0x0)) && ((*(local_30 + 0x3a) & 0x40) == 0x0)))) {
                    iVar6 = FUN_0044a87f((param_1 + 0x22),(param_1 + 0x24),
                                         (local_30 + 0x22),(local_30 + 0x9));
                    if (((iVar6 <= local_28) && ((uVar2 & *(local_30 + 0x3a)) == 0x0)) &&
                        (*(local_30 + 0x3d) >> 0x18 <= iVar3 + iVar6 * -0x2)) {
                        *(local_30 + 0x3a) = *(local_30 + 0x3a) | uVar2;
                        local_1c = 0x1;
                    }
                    if (!bVar5) {
                        iVar4 = *(*(&DAT_00582938 +
                            (*(local_30 + 0x25) >> 0x18) * 0x4 +
                            (local_30[0x9] >> 0x18) * 0x18) + 0x49);
                        if (iVar4 < 0x4) {
                            local_20 = 0x1;
                        }
                        else {
                            local_20 = (iVar4 >> 0x1) + -0x1;
                        }
                        if ((iVar6 <= local_20) && (*(param_1 + 0x3d) >> 0x18 <= iVar4 + iVar6 * -0x2)) {
                            *(param_1 + 0x3a) =
                                *(param_1 + 0x3a) | *(&DAT_004be9b0 + (*(local_30 + 0x23) >> 0x18) * 0x4)
                            ;
                            bVar5 = true;
                        }
                    }
                }
            }
        }
        local_34 = local_1c;
    }
    return local_34;
}



bool  FUN_0045a451(param_1: i32,param_2: u32,param_3: u32)

{
let mut bVar1: bool;
let mut iVar2: i32;
undefined3 extraout_var;
let mut uVar3: u32;
let mut uVar4: u32;
let mut iVar5: i32;
let puVar6: *mut u32;
let mut local_5c: i32;
let mut local_24: i32;
let mut local_20: i32;

iVar2 = (*(param_1 + 0x4) + 0x20);
bVar1 = false;
uVar3 = *(*(&DAT_004d7d50 + param_2 * 0x4 + iVar2 * 0x3890) + param_3 * 0x4);
local_24 = 0x0;
for (local_20 = *(param_1 + 0x4); local_20 != 0x0; local_20 = *(local_20 + 0x8)) {
if (local_24 <
*(*(&DAT_00582938 +
(*(local_20 + 0x25) >> 0x18) * 0x4 + (*(local_20 + 0x24) >> 0x18) * 0x18) + 0x49)
) {
local_24 = *(*(&DAT_00582938 +
(*(local_20 + 0x25) >> 0x18) * 0x4 + (*(local_20 + 0x24) >> 0x18) * 0x18
) + 0x49);
}
if (((*(local_20 + 0x3a) & 0x40) == 0x0) &&
(bVar1 = FUN_0045a870(local_20,uVar3), CONCAT31(extraout_var,bVar1) == 0x0)) {
bVar1 = false;
break;
}
bVar1 = true;
}
if ((local_24 == 0x0) &&
((*(&DAT_004be9b0 + (*(*(param_1 + 0x4) + 0x23) >> 0x18) * 0x4) &
*(*(&DAT_004d7d50 + iVar2 * 0x3890 + param_2 * 0x4) + param_3 * 0x4 + 0x4)) == 0x0)) {
return false;
}
if (!bVar1) {
if ((*(*(&DAT_004d7d50 + param_2 * 0x4 + iVar2 * 0x3890) + param_3 * 0x4 + 0x4) & 0x4) == 0x0) {
if ((*(*(param_1 + 0x4) + 0x41) != param_2) ||
(*(*(param_1 + 0x4) + 0x42) != param_3)) {
return false;
}
if ((uVar3 & 0xf) != 0x0) {
return false;
}
for (local_5c = 0x0; local_5c < 0x6; local_5c = local_5c + 0x1) {
uVar3 = FUN_0043a8a2(param_2 + (&DAT_004bea60)[local_5c]);
uVar4 = FUN_0043a8d5(uVar3,param_3 + (&DAT_004bea7c)[local_5c]);
if (((uVar3 != param_2) || (uVar4 != param_3)) &&
((*(*(&DAT_004d7d50 + uVar3 * 0x4 + iVar2 * 0x3890) + uVar4 * 0x4) & 0xf) != 0x0)) break;
}
if (local_5c == 0x6) {
return false;
}
iVar5 = FUN_0044aa04(iVar2,param_2,param_3);
bVar1 = 0x0 < iVar5;
if ((!bVar1) && (puVar6 = FUN_00481784(iVar2,param_2,param_3), puVar6 != 0x0)) {
iVar5 = FUN_00481a44(puVar6);
if (iVar5 != 0x0) {
return false;
}
bVar1 = true;
}
}
else {
if ((uVar3 & 0xf) == 0x0) {
bVar1 = true;
}
else {
if ((*(&DAT_00590dbe +
*(&DAT_004daab0 + iVar2 * 0x3890) * 0x14 +
(*(*(param_1 + 0x4) + 0x28) >> 0x10) * 0x2) >> 0x10 != 0x0) &&
((*(*(&DAT_004d7d50 + param_2 * 0x4 + iVar2 * 0x3890) + param_3 * 0x4 + 0x7) & 0x10) != 0x0))
{
bVar1 = true;
}
}
}
}
if (((*(*(&DAT_004d7d50 + param_2 * 0x4 + iVar2 * 0x3890) + param_3 * 0x4 + 0x4) & 0x2) == 0x0) ||
((*(*(param_1 + 0x4) + 0x41) == param_2 && (*(*(param_1 + 0x4) + 0x42) == param_3))))
{
if (((*(*(&DAT_004d7d50 + param_2 * 0x4 + iVar2 * 0x3890) + param_3 * 0x4 + 0x4) & 0x8) != 0x0) &&
((*(*(param_1 + 0x4) + 0x41) != param_2 || (*(*(param_1 + 0x4) + 0x42) != param_3))
)) {
bVar1 = false;
}
}
else {
bVar1 = false;
}
return bVar1;
}



bool  FUN_0045a870(param_1: i32,param_2: u32)

{
return (&DAT_00590dc0 +
*(*(&DAT_00582938 +
(*(param_1 + 0x25) >> 0x18) * 0x4 + (*(param_1 + 0x24) >> 0x18) * 0x18
) + 0x41) * 0x2 +
*(&DAT_004daab0 + (param_1 + 0x20) * 0x3890) * 0x14 + (param_2 & 0xf) * 0x64) != 0x0
;
}



fn FUN_0045a902(param_1: u32,param_2: i32,param_3: u32) -> i32

{
let sVar1: i16;
let mut iVar2: i32;
let bVar3: u8;
let mut iVar4: i32;
let mut local_24: i32;

sVar1 = (param_2 + 0x20);
iVar2 = *(*(&DAT_00582938 +
(*(param_2 + 0x25) >> 0x18) * 0x4 + (*(param_2 + 0x24) >> 0x18) * 0x18) + 0x41)
;
local_24 = *(&DAT_00590dbe +
iVar2 * 0x2 + (param_3 & 0xf) * 0x64 + *(&DAT_004daab0 + sVar1 * 0x3890) * 0x14) >> 0x10;
bVar3 = 0x4;
while( true ) {
iVar4 = FUN_0043936e((param_2 + 0x20),&param_3);
if (iVar4 == 0x0) break;
local_24 = local_24 *
(*(&DAT_00590dbe +
iVar2 * 0x2 + *(&DAT_004daab0 + sVar1 * 0x3890) * 0x14 + (param_3 & 0xf) * 0x64) >> 0x10)
;
bVar3 = bVar3 + 0x4;
}
return (0x1 << (bVar3 & 0x1f)) + -0x1 + local_24 >> (bVar3 & 0x1f);
}



// WARNING: Could not reconcile some variable overlaps

fn FUN_0045aa12(param_1: i32,param_2: i32,param_3: i32,param_4: i32) -> i32

{
let mut uVar1: u32;
let mut iVar2: i32;
let mut bVar3: bool;
let mut iVar4: i32;
let mut local_30: i32;
let mut local_28: i32;
let mut local_24: i32;
let mut local_18: i32;
let mut local_14: i32;

if ((param_2 == 0xff) || (param_3 == 0xff)) {
local_30 = 0xff;
}
else {
iVar4 = (*(param_1 + 0x4) + 0x20);
if ((*(*(&DAT_00582938 +
(*(*(param_1 + 0x4) + 0x25) >> 0x18) * 0x4 +
(*(*(param_1 + 0x4) + 0x24) >> 0x18) * 0x18) + 0x49) == 0x0) &&
((*(&DAT_004be9b0 + (*(*(param_1 + 0x4) + 0x23) >> 0x18) * 0x4) &
*(*(&DAT_004d7d50 + iVar4 * 0x3890 + param_2 * 0x4) + param_3 * 0x4 + 0x4)) == 0x0)) {
local_30 = 0x0;
}
else {
local_24 = -0x1;
uVar1 = *(*(&DAT_004d7d50 + param_2 * 0x4 + iVar4 * 0x3890) + param_3 * 0x4);
local_14 = 0xff;
for (local_18 = *(param_1 + 0x4); local_18 != 0x0; local_18 = *(local_18 + 0x8)) {
iVar2 = *(*(&DAT_00582938 +
(*(local_18 + 0x25) >> 0x18) * 0x4 + (*(local_18 + 0x24) >> 0x18) * 0x18)
+ 0x41);
if ((local_24 != iVar2) && ((*(local_18 + 0x3a) & 0x40) == 0x0)) {
local_28 = FUN_0045a902(param_1,local_18,uVar1);
if (local_28 == 0x0) {
if (((&DAT_00590dc0 + *(&DAT_004daab0 + iVar4 * 0x3890) * 0x14 + iVar2 * 0x2) == 0x0) &&
((&DAT_0059120c + iVar2 * 0x2 + *(&DAT_004daab0 + iVar4 * 0x3890) * 0x14) == 0x0)) {
bVar3 = false;
}
else {
bVar3 = true;
}
if (!bVar3) {
return 0x0;
}
}
if ((local_28 == 0x0) &&
((*(*(&DAT_004d7d50 + param_2 * 0x4 + iVar4 * 0x3890) + param_3 * 0x4 + 0x4) & 0x4) != 0x0))
{
if ((uVar1 & 0xf) == 0x0) {
local_28 = 0x1;
}
else {
if ((*(&DAT_00590dbe + *(&DAT_004daab0 + iVar4 * 0x3890) * 0x14 + iVar2 * 0x2) >> 0x10 !=
0x0) && ((*(*(&DAT_004d7d50 + param_2 * 0x4 + iVar4 * 0x3890) + param_3 * 0x4 + 0x7) &
0x10) != 0x0)) {
local_28 = 0x1;
}
}
}
if (((param_4 == 0x0) && (*(local_18 + 0x2c) >> 0x18 < local_28)) &&
(*(local_18 + 0x2c) >> 0x18 ==
*(*(&DAT_00582938 +
(*(local_18 + 0x25) >> 0x18) * 0x4 + (*(local_18 + 0x24) >> 0x18) * 0x18) +
0x45))) {
local_28 = *(local_18 + 0x2c) >> 0x18;
}
if (local_28 == 0x0) {
local_14 = 0x0;
break;
}
local_24 = iVar2;
if ((local_14 == 0xff) || (local_14 < local_28)) {
local_14 = local_28;
}
}
}
local_30 = local_14;
}
}
return local_30;
}



fn FUN_0045ad4e(param_1: i32,param_2: i32,param_3: i32) -> *mut u32

{
    let local_14: *mut u32;

    local_14 = (&DAT_005b8b44 + param_1 * 0x4);
    while( true ) {
        if ((local_14 == 0x0) || ((local_14 + 0x8) != param_1)) {
            return 0x0;
        }
        if ((((local_14 + 0x22) == param_2) && ((local_14 + 0x9) == param_3)) &&
            ((*(local_14 + 0x3a) & 0x1) != 0x0)) break;
        local_14 = *local_14;
    }
    return local_14;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0045ae28() -> u32

{
    let mut iVar1: i32;
    let mut local_18: u32;
    let mut local_14: u32;

    iVar1 = FUN_0049c2c9(0xb8);
    DAT_005b2d50 = (iVar1 + 0x4);
    DAT_005b2d54 = FUN_0049c2c9(0xc21);
    if ((DAT_005b2d50 == 0x0) || (DAT_005b2d54 == 0x0)) {
        local_18 = 0x0;
    }
    else {
        FUN_004a0430(DAT_005b2d54,0xff,0xc21);
        for (local_14 = 0x0; local_14 < 0x2d; local_14 = local_14 + 0x1) {
            DAT_005b2d50[local_14] = local_14 * 0x45 + DAT_005b2d54 + 0x2;
        }
        DAT_005b2d50[-0x1] = DAT_005b2d50[0x2b];
        DAT_005b2d50[0x2c] = *DAT_005b2d50;
        for (local_14 = 0x0; local_14 < 0x2c; local_14 = local_14 + 0x1) {
            (DAT_005b2d50[local_14] + (-0x1 - (local_14 & 0x1))) = 0x0;
            (DAT_005b2d50[local_14] + (0x41 - (local_14 & 0x1))) = 0x0;
        }
        _DAT_005b2d58 = 0x1;
        local_18 = 0x1;
    }
    return local_18;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0045af67(param_1: i32,param_2: i32,param_3: i32,param_4: u32,param_5: u32,param_6: i32) -> u32

{
    let mut uVar1: u32;
    let mut bVar2: bool;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut uVar5: u32;
    let mut uVar6: u32;
    undefined3 extraout_var;
    let mut uVar7: u32;
    let mut uVar8: u32;
    let mut uVar9: u32;
    undefined3 extraout_var_00;
    byte *pbVar10;
    let mut local_94: u32;
    let mut local_90: u32;
    let local_8c: u8;
    let local_70: u8;
    let local_6d: u8;
    let mut local_60: u32;
    let cStack92: u8;
    let mut local_44: i32;
    let mut local_34: u32;
    let mut local_2c: u32;
    let mut local_24: u32;
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: u32;

    if ((param_4 == 0xff) || (param_5 == 0xff)) {
        local_2c = 0x0;
    }
    else {
        if (param_6 == 0x0) {
            local_44 = 0x1869f;
        }
        else {
            local_44 = param_6;
        }
        iVar3 = (*(param_1 + 0x4) + 0x20);
        if ((_DAT_005b2d58 == 0x0) && (iVar4 = FUN_0045ae28(), iVar4 == 0x0)) {
            local_2c = 0xffffffff;
        }
        else {
            _DAT_005b2d5c = 0x0;
            _DAT_005b2d60 = 0x0;
            for (local_24 = 0x0; local_24 < 0x2c; local_24 = local_24 + 0x1) {
                FUN_004a0430(*(DAT_005b2d50 + local_24 * 0x4),0xff,0x40 - (local_24 & 0x1));
            }
            FUN_0045bc82(iVar3);
            FUN_0045b7ac(param_1,iVar3,param_2,param_3);
            iVar3 = FUN_00432c94(param_1);
            FUN_0045bd09(iVar3);
            (*(DAT_005b2d50 + param_4 * 0x4) + param_5) = 0x0;
            uVar5 = FUN_0044a87f(param_2,param_3,param_4,param_5);
            local_34 = param_4 << 0x18 | (uVar5 & 0xff) << 0x10;
            local_34 = local_34 | uVar5 & 0xff;
            FUN_00462b69(&DAT_005967d0);
            FUN_00462b8d(&DAT_005967d0,local_34,(char)param_5);
            local_18 = 0x0;
            while ((param_6 == 0x0 || (_DAT_005967d0 != 0x0))) {
                FUN_00462c31(&DAT_005967d0);
                local_6d = (char)(local_60 >> 0x18);
                if (param_6 == 0x0) {
                    local_70 = (byte)local_60;
                    if (local_70 == 0x0) {
                        local_1c = local_18;^
                        // goto LAB_0045b396;
                    }
                    pbVar10 = (*(DAT_005b2d50 + param_2 * 0x4) + param_3);
                    if (*pbVar10 <= local_70 && local_70 != *pbVar10) {
                        local_1c = *(*(DAT_005b2d50 + param_2 * 0x4) + param_3);^
                        // goto LAB_0045b396;
                    }
                    if (((local_70 != *(*(DAT_005b2d50 + param_2 * 0x4) + param_3)) || (local_6d != param_2)) ||
                        (cStack92 != param_3))^ // goto LAB_0045b1f5;
                    local_18 = *(*(DAT_005b2d50 + param_2 * 0x4) + param_3);
                }
                else {
                    LAB_0045b1f5:
                        uVar5 = SEXT14(local_6d);
                    uVar6 = SEXT14(cStack92);
                    bVar2 = FUN_0045a451(param_1,uVar5,uVar6);
                    if (CONCAT31(extraout_var,bVar2) != 0x0) {
                        iVar3 = FUN_0045aa12(param_1,uVar5,uVar6,0x0);
                        uVar7 = iVar3 + (local_60 >> 0x8 & 0xff);
                        for (local_24 = 0x0; local_24 < 0x6; local_24 = local_24 + 0x1) {
                            if ((uVar7 <
                                *(uVar6 + (&DAT_004bea7c)[local_24] +
                                    *(DAT_005b2d50 + (uVar5 + (&DAT_004bea60)[local_24]) * 0x4))) &&
                                (uVar7 <= local_44)) {
                                uVar8 = FUN_0043a8a2(uVar5 + (&DAT_004bea60)[local_24]);
                                uVar9 = FUN_0043a8d5(uVar8,uVar6 + (&DAT_004bea7c)[local_24]);
                                bVar2 = FUN_0045a451(param_1,uVar8,uVar9);
                                if (CONCAT31(extraout_var_00,bVar2) != 0x0) {
                                    local_8c = (char)uVar9;
                                    local_34._3_1_ = (char)uVar8;
                                    iVar3 = FUN_0044a87f(param_2,param_3,local_34._3_1_,local_8c);
                                    local_34._0_3_ = CONCAT12((char)iVar3,((uVar7 & 0xff) << 0x8));
                                    uVar1 = (uint3)local_34;
                                    local_34 = uVar8 << 0x18 | uVar1;
                                    local_34._1_1_ = (char)(uVar1 >> 0x8);
                                    local_34._2_1_ = (char)(uVar1 >> 0x10);
                                    local_34 = local_34 | (byte)(local_34._1_1_ + local_34._2_1_);
                                    FUN_00462b8d(&DAT_005967d0,local_34,local_8c);
                                    (*(uVar8 * 0x4 + DAT_005b2d50) + uVar9) = local_34._1_1_;
                                }
                            }
                        }
                    }
                }
            }
            local_1c = 0x0;
            LAB_0045b396:
            for (local_14 = 0x0; local_14 < 0x2c; local_14 = local_14 + 0x1) {
                local_90 = ((local_14 & 0x1) == 0x0);
                for (local_94 = local_90; local_94 < (0x41 - (local_14 & 0x1)); local_94 = local_94 + 0x2) {
                    if (((*(DAT_005b2d50 + local_14 * 0x4) + local_94) == '\0') &&
                        ((local_14 != param_4 || (local_94 != param_5)))) {
                        (*(DAT_005b2d50 + local_14 * 0x4) + local_94) = 0xff;
                    }
                }
            }
            local_2c = local_1c;
        }
    }
    return local_2c;
}



fn FUN_0045b45b(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: *mut u32,param_6: *mut u32) -> u32

{
    let mut bVar1: bool;
    undefined3 extraout_var;
    let mut uVar2: u32;
    u32 auStack172 [0x1d];
    let mut local_38: i32;
    let mut local_34: u32;
    let mut local_30: u32;
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: i32;
    ushort *local_1c;
    ushort *local_18;
    let mut local_14: u32;

    local_1c = (*(param_1 + 0x4) + 0x20);
    local_14 = local_1c & 0xffff0000 | *local_1c;
    local_20 = *local_1c;
    *param_5 = 0xffffffff;
    *param_6 = 0xffffffff;
    local_2c = 0x6;
    local_28 = 0x0;
    auStack172[24] = 0xffff;
    auStack172[25] = 0xffff;
    local_18 = local_1c;
    for (local_38 = 0x0; local_38 < 0x6; local_38 = local_38 + 0x1) {
        local_34 = FUN_0043a8a2(param_3 + (&DAT_004bea60)[local_38]);
        local_30 = FUN_0043a8d5(local_34,param_4 + (&DAT_004bea7c)[local_38]);
        bVar1 = FUN_0045a451(param_1,local_34,local_30);
        if (CONCAT31(extraout_var,bVar1) == 0x0) {
            auStack172[local_38 * 0x4] = 0xffffffff;
        }
        else {
            local_24 = FUN_0045aa12(param_1,local_34,local_30,0x0);
            if (local_24 == 0xff) {
                auStack172[local_38 * 0x4] = 0xffffffff;
            }
            else {
                if (*(*(local_34 * 0x4 + DAT_005b2d50) + local_30) + local_24 == param_2) {
                    local_2c = local_38;
                    local_28 = local_28 + 0x1;
                    uVar2 = FUN_0044a87f(local_34,local_30,param_3,param_4);
                    auStack172[local_38 * 0x4] = uVar2;
                    auStack172[local_38 * 0x4 + 0x2] = local_34;
                    auStack172[local_38 * 0x4 + 0x3] = local_30;
                    uVar2 = *(*(&DAT_004d7d50 + local_20 * 0x3890 + local_34 * 0x4) + local_30 * 0x4);
                    if ((uVar2 & 0xf) == 0xb) {
                        auStack172[local_38 * 0x4 + 0x1] = 0xffffffff;
                    }
                    else {
                        auStack172[local_38 * 0x4 + 0x1] = uVar2 & 0xf;
                    }
                }
                else {
                    auStack172[local_38 * 0x4] = 0xffffffff;
                }
            }
        }
    }
    if (0x1 < local_28) {
        local_2c = 0x6;
        for (local_38 = 0x0; local_38 < 0x7; local_38 = local_38 + 0x1) {
            if (auStack172[local_38 * 0x4] != 0xffffffff) {
                if (auStack172[local_38 * 0x4] < auStack172[local_2c * 0x4]) {
                    local_2c = local_38;
                }
                else {
                    if ((auStack172[local_38 * 0x4] == auStack172[local_2c * 0x4]) &&
                        (auStack172[local_38 * 0x4 + 0x1] <= auStack172[local_2c * 0x4 + 0x1])) {
                        local_2c = local_38;
                    }
                }
            }
        }
    }
    if (local_2c == 0x6) {
        auStack172[28] = 0xffffffff;
    }
    else {
        *param_5 = auStack172[local_2c * 0x4 + 0x2];
        *param_6 = auStack172[local_2c * 0x4 + 0x3];
        auStack172[28] = *(*(DAT_005b2d50 + *param_5 * 0x4) + *param_6);
    }
    return auStack172[28];
}



fn FUN_0045b75f(param_1: i32,param_2: i32,param_3: i32,param_4: i32) -> i32

{
let mut uVar1: u32;
let mut local_18: i32;

uVar1 = param_1 - param_3 >> 0x1f;
local_18 = (param_1 - param_3 ^ uVar1) - uVar1;
uVar1 = param_2 - param_4 >> 0x1f;
if (0x16 < local_18) {
local_18 = 0x2c - local_18;
}
return local_18 + ((param_2 - param_4 ^ uVar1) - uVar1);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0045b7ac(param_1: i32,param_2: i32,param_3: i32,param_4: i32)

{
    byte *pbVar1;
    let mut pcVar2: String;
    let mut iVar3: i32;
    let local_18: *mut u32;
    let local_14: *mut u32;

    for (local_18 = (&DAT_005b8b44 + param_2 * 0x4);
        (local_18 != 0x0 && ((local_18 + 0x8) == param_2)); local_18 = *local_18) {
        if ((*(local_18 + 0x3a) & 0x1) != 0x0) {
            if (*(local_18 + 0x23) >> 0x18 == DAT_004c9754) {
                if (((local_18 + 0x22) != param_3) || ((local_18 + 0x9) != param_4)) {
                    if ((*(*(param_1 + 0x4) + 0x32) >> 0x18 == *(local_18 + 0x32) >> 0x18) ||
                        ((*(local_18 + 0x32) >> 0x18 < 0xa || (0xc < *(local_18 + 0x32) >> 0x18)))) {
                        pcVar2 = (*(DAT_005b2d50 + (local_18 + 0x22) * 0x4) +
                            (local_18 + 0x9));
                        *pcVar2 = *pcVar2 + '\x01';
                    }
                    else {

                        (*(DAT_005b2d50 + (local_18 + 0x22) * 0x4) + (local_18 + 0x9)) = 0x19;
                    }
                }
            }
            else {
                if (((*(&DAT_004be9b0 + DAT_004c9754 * 0x4) & *(local_18 + 0x3a)) != 0x0) &&
                    ((_DAT_005b2d64 == 0x0 ||
                        ((&DAT_004d55a8)[DAT_004c9754 * 0xe + (*(local_18 + 0x23) >> 0x18)] != '\x02')))) {
                    pbVar1 = (*(&DAT_004d7d50 + param_2 * 0x3890 + (local_18 + 0x22) * 0x4) +
                        (local_18 + 0x9) * 0x4 + 0x4);
                    *pbVar1 = *pbVar1 | 0x2;
                }
            }
        }
    }
    for (local_14 = (&DAT_005b89f8 + param_2 * 0x4);
        (local_14 != 0x0 && (*(local_14 + 0x6) >> 0x10 == param_2));
        local_14 = *local_14) {
        iVar3 = FUN_00481a44(local_14);
        if (((((iVar3 == 0x0) || ((local_14 + 0xe) == 0x10)) || ((local_14 + 0xe) == 0xf)) ||
            ((local_14 + 0xe) == 0x2)) &&
            ((*((*(local_14 + 0xa) >> 0x10) * 0x4 +
                *(&DAT_004d7d50 + param_2 * 0x3890 + (local_14[0x2] >> 0x10) * 0x4) + 0x4) & 0x4) == 0x0)
        ) {
            pbVar1 = ((*(local_14 + 0xa) >> 0x10) * 0x4 +
                *(&DAT_004d7d50 + (local_14[0x2] >> 0x10) * 0x4 + param_2 * 0x3890) + 0x4);
            *pbVar1 = *pbVar1 | 0x4;
            iVar3 = FUN_0044ab7b(param_2,local_14[0x2] >> 0x10,*(local_14 + 0xa) >> 0x10);
            if (iVar3 != 0x0) {
                pbVar1 = ((*(local_14 + 0xa) >> 0x10) * 0x4 +
                    *(&DAT_004d7d50 + param_2 * 0x3890 + (local_14[0x2] >> 0x10) * 0x4) + 0x7);
                *pbVar1 = *pbVar1 | 0x10;
            }
            if (*(local_14 + 0xe) >> 0x10 == DAT_004c9754) {
                if (((*(*(param_1 + 0x4) + 0x32) >> 0x18 != local_14[0x4] >> 0x10) &&
                    (0x9 < local_14[0x4] >> 0x10)) && (local_14[0x4] >> 0x10 < 0xd)) {

                    ((*(local_14 + 0xa) >> 0x10) + *((local_14[0x2] >> 0x10) * 0x4 + DAT_005b2d50)) =
                        0x19;
                }
            }
            else {
                if (_DAT_005b2d64 == 0x0) {
                    pbVar1 = ((*(local_14 + 0xa) >> 0x10) * 0x4 +
                        *(&DAT_004d7d50 + param_2 * 0x3890 + (local_14[0x2] >> 0x10) * 0x4) + 0x4);
                    *pbVar1 = *pbVar1 | 0x8;
                }
            }
        }
    }
    return;
}



fn FUN_0045bc82(param_1: i32)

{
    byte *pbVar1;
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: u32;

    for (local_14 = 0x0; local_14 < 0x2c; local_14 = local_14 + 0x1) {
        local_18 = ((local_14 & 0x1) == 0x0);
        for (local_1c = local_18; local_1c < (0x41 - (local_14 & 0x1)); local_1c = local_1c + 0x2) {
            pbVar1 = (*(&DAT_004d7d50 + local_14 * 0x4 + param_1 * 0x3890) + local_1c * 0x4 + 0x4);
            *pbVar1 = *pbVar1 & 0xf0;
        }
    }
    return;
}



fn FUN_0045bd09(param_1: i32)

{
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: u32;

    for (local_14 = 0x0; local_14 < 0x2c; local_14 = local_14 + 0x1) {
        local_18 = ((local_14 & 0x1) == 0x0);
        for (local_1c = local_18; local_1c < (0x41 - (local_14 & 0x1)); local_1c = local_1c + 0x2) {
            if ((*(DAT_005b2d50 + local_14 * 0x4) + local_1c) != -0x1) {
                if ((*(*(local_14 * 0x4 + DAT_005b2d50) + local_1c) + param_1) < 0x14) {
                    (*(DAT_005b2d50 + local_14 * 0x4) + local_1c) = 0xff;
                }
                else {
                    (*(DAT_005b2d50 + local_14 * 0x4) + local_1c) = 0x0;
                }
            }
        }
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32  find_file_fn_0045bde0(byte *param_1,param_2: &mut String,param_3: i32)

{
let mut pcVar1: String;
let local_38c: u8 [0x100];
HANDLE search_handle;
_WIN32_FIND_DATAA find_file_data;
let mut local_148: *mut u32 [0x11];
let ppuStack259: *mut *mut u8;;
let mut local_57: String;;
let mut local_50: u32;
let mut local_4c: String;;
byte *local_48;
byte *local_44;
byte *local_40;
byte *local_3c;
let mut local_38: String;;
byte *local_34;
byte *local_30;
byte *local_2c;
byte *local_28;
byte *local_24;
byte *local_20;
let mut local_1c: u32;
let local_18: *mut i32;;

_DAT_005b2d70 = param_3;
_DAT_005b2d68 = param_1;
lpFileName_005b2d6c = FUN_0049c2c9(0x104);
local_4c = FUN_0049c2c9(0x104);
local_20 = FUN_0049c2c9(0x107);
local_38 = FUN_0049c2c9(0x104);
local_34 = FUN_0049c2c9(0x107);
FUN_004a98c0(param_1,local_20,&local_48,&local_44,&local_40,&local_3c);
local_18 = FUN_004990e0(local_148,0x0,s_efs_res_004c20b4,s_MyFileOpen_004c20a9);
local_1c = FUN_0049bb50(local_148,FUN_0045c087);
if (local_1c != 0x0) {
FUN_004a98c0(lpFileName_005b2d6c,local_34,&local_30,&local_2c,&local_28,&local_24);
FUN_004a9a00(param_2,local_28,0x103);
if (param_3 == 0x1) {
FUN_0049c2e0(lpFileName_005b2d6c,s__s_s_sav_004c20bc);
// LPWIN32_FIND_DATAA lpFindFileData for FindFirstFileA
// LPCSTR lpFileName for FindFirstFileA
search_handle = FindFirstFileA(lpFileName_005b2d6c,(LPWIN32_FIND_DATAA)&find_file_data);
if (search_handle != (HANDLE)0xffffffff) {
// HANDLE hFindFile for FindClose
FindClose(search_handle);
FUN_00430bce(param_2);
pcVar1 = FUN_00499050(DAT_005b9bd8,0x7531);
FUN_0049c2e0(local_38c,pcVar1);
local_1c = FUN_0049d2e0(0x0,0x2,local_38c);
}
}
}
FUN_0049af50(lpFileName_005b2d6c);
FUN_0049af50(local_38);
FUN_0049af50(local_34);
FUN_0049af50(local_4c);
FUN_0049af50(local_20);
local_50 = local_1c;
ppuStack259 = &PTR_FUN_004c3d34;
if (local_57 != 0x0) {
FUN_00499b30(local_148,local_57);
}
FUN_0049a1c0(local_148,0x1);
return local_50;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0045c087(param_1: *mut *mut u32,param_2: u32,param_3: u32,param_4: *mut i32) -> u32

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut uVar3: u32;
    let puVar4: *mut u32;
    let bVar5: u8;
    let mut local_12c: u32;
    let mut local_128: i32;
    let local_124: u8 [0x104];
    let mut local_20: *mut u8;
    u32 **local_1c;
    let mut local_14: u32;

    local_14 = param_2;
    if (param_2 < 0x406) {
        if (param_2 == 0x401) {
            if (_DAT_005b2d70 == 0x0) {
                FUN_0049bf80(param_1,0x3e8,0x414,0x0,0x0);
                FUN_0049bf80(param_1,0x3e8,0x410,0x0,0x0);
                FUN_0049bf80(param_1,0x1f5,0x414,0x0,0x0);
                FUN_0049bf80(param_1,0x1f5,0x410,0x0,0x0);
            }
            else {
                FUN_0049bf80(param_1,0x1f4,0x414,0x0,0x0);
                FUN_0049bf80(param_1,0x1f4,0x410,0x0,0x0);
            }
            local_1c = FUN_0045c3e4((u32 **)0x0,s_sav___sav_004c20c5,(HANDLE)0x21);
            FUN_0049bf80(param_1,0xc8,0x503,0x0,local_1c);
            FUN_0049bf80(param_1,0xc8,0x502,0x0,0x0);
        }
    }
    else {
        if (param_2 < 0x407) {
            if ((param_3 == 0xc8) &&
                (local_20 = FUN_0049bf80(param_1,0xc8,0x501,0x0,0x0), local_20 != 0xffffffff)) {
                FUN_0049bf80(param_1,0xc8,0x504,local_20,local_124);
                FUN_0049bf80(param_1,0x3e8,0x502,0xd,local_124);
            }
        }
        else {
            if (param_2 < 0x408) {
                if (param_3 < 0x1f4) {
                    if (param_3 < 0x65) {
                        return 0x0;
                    }
                    if (param_3 < 0x66) {
                        FUN_0049c140(param_1,0x0);
                        return 0x0;
                    }
                    if (param_3 != 0xc8) {
                        return 0x0;
                    }
                }
                else {
                    if (((0x1f4 < param_3) && (0x1f5 < param_3)) && (param_3 != 0x3e8)) {
                        return 0x0;
                    }
                }
                FUN_0049bf80(param_1,0x3e8,0x501,0xc8,lpFileName_005b2d6c);
                FUN_0049c140(param_1,0x1);
            }
            else {
                if (param_2 == 0x412) {
                    iVar1 = param_4[0x5];
                    if (param_4[0x4] == 0x1) {
                        local_12c = 0xe0e0e;
                        local_128 = 0xcaccce;
                    }
                    else {
                        local_12c = 0xcaccce;
                        local_128 = 0xe0e0e;
                    }
                    FUN_004953d7();
                    bVar5 = 0x10;
                    uVar3 = 0xcaccce;
                    iVar2 = param_4[0x2];
                    puVar4 = LPCSTR_005b9218;
                    iVar1 = FUN_00430bce(iVar1);
                    FUN_00497567(*param_4,param_4[0x1],iVar1,iVar2,local_12c,local_128,uVar3,puVar4,bVar5);
                    FUN_0049536f();
                }
            }
        }
    }
    return 0x0;
}



u32 **  FUN_0045c3e4(param_1: *mut *mut u32,LPCSTR param_2,HANDLE param_3)

{
let cVar1: u8;
let DVar2: u32;
let mut extraout_ECX: u32;
let mut extraout_EDX: u32;
let puVar3: *mut u32;
ulonglong uVar4;
HANDLE local_13c [0x5];
let local_127: u8;
u32 local_11e [0x40];
u32 **local_1c;
let mut local_18: i32;
let mut local_14: u32;

if ((param_1 == (u32 **)0x0) && (param_1 = (u32 **)FUN_0049ea90(), param_1 == (u32 **)0x0)) {
local_1c = (u32 **)0x0;
}
else {
DVar2 = find_file_fn_004a3a00(param_2,param_3,local_13c);
if (DVar2 == 0x0) {
if ((((param_3 & local_127) != 0x0) || ((param_3 & 0x20) != 0x0)) &&
(uVar4 = FUN_004a2ae0(extraout_ECX,extraout_EDX,local_11e,&DAT_004c20cf), uVar4 != 0x0)) {
local_14 = 0xffffffff;
puVar3 = local_11e;
loop {
if (local_14 == 0x0) break;
local_14 = local_14 - 0x1;
cVar1 = puVar3;
puVar3 = (puVar3 + 0x1);
} while (cVar1 != '\0');
local_14 = ~local_14;
FUN_004a9a40(local_11e);
local_18 = FUN_0049eae0(param_1,local_11e,local_14);
if (local_18 == 0x0) {
return param_1;
}
}
loop {
loop {
DVar2 = find_file_fn_004a3a94(local_13c);
if (DVar2 != 0x0) {
FUN_0049ebc0(param_1,0x1);
return param_1;
}
} while (((param_3 & local_127) == 0x0) && ((param_3 & 0x20) == 0x0));
local_14 = 0xffffffff;
puVar3 = local_11e;
loop {
if (local_14 == 0x0) break;
local_14 = local_14 - 0x1;
cVar1 = puVar3;
puVar3 = (puVar3 + 0x1);
} while (cVar1 != '\0');
local_14 = ~local_14;
FUN_004a9a40(local_11e);
local_18 = FUN_0049eae0(param_1,local_11e,local_14);
} while (local_18 != 0x0);
local_1c = param_1;
}
else {
local_1c = (u32 **)0x0;
}
}
return local_1c;
}



fn FUN_0045c549() -> u32

{
    let mut local_120: *mut u32 [0x11];
    let ppuStack219: *mut *mut u8;;
    let mut local_2f: String;;
    let mut local_28: u32;
    let local_24: *mut i32;;
    u32 **local_20;
    let mut local_1c: u32;

    FUN_0049c7b5(s_dat_efs_ini_004c20d1);
    FUN_0049c2e0(&filename_00599c80,&DAT_004c20dd);
    local_24 = FUN_004990e0(local_120,0x0,s_efs_res_004c20eb,s_OptionsDlg_004c20e0);
    local_28 = FUN_0049bb50(local_120,FUN_0045c663);
    local_20 = local_120;
    local_1c = 0x0;
    ppuStack219 = &PTR_FUN_004c3d34;
    if (local_2f != 0x0) {
        FUN_00499b30(local_120,local_2f);
    }
    FUN_0049a1c0(local_120,0x1);
    return local_28;
}



fn FUN_0045c663(param_1: *mut *mut u32,param_2: u32,param_3: u32,DWORD param_4) -> u32

{
    let puVar1: *mut u32;
    let mut pcVar2: String;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut uVar5: u32;
    let mut uVar6: u32;
    let mut uVar7: u32;
    let bVar8: u8;
    let uVar9: u16;
    let mut local_dc: i32;
    let mut local_d8: i32;
    let mut local_d4: i32;
    let mut local_d0: i32;
    let mut local_c0: i32;
    let local_b8: u8 [0x80];
    let mut local_38: i32;
    let mut local_34: i32;
    let mut local_30: i32;
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_24: *mut u8;
    let mut local_20: u32;
    let mut local_1c: u32;
    let mut local_14: u32;

    local_14 = param_2;
    if (param_2 < 0x407) {
        if (0x400 < param_2) {
            if (param_2 < 0x402) {
                FUN_0049bf80(param_1,0x1bc5,0x502,grid_opt_00599de0,0x0);
                FUN_0049bf80(param_1,0x1bc2,0x502,sound_opt_00599de8,0x0);
                FUN_0049bf80(param_1,0x1bc1,0x502,autosave_opt_00599df0,0x0);
                local_1c = (((byte)DAT_004d559c & 0x4) != 0x0);
                FUN_0049bf80(param_1,0x1bc8,0x502,local_1c,0x0);
                FUN_0049bf80(param_1,0xbb8,0x502,(byte)(&DAT_004d5a58)[DAT_004c9754 * 0x64],0x0);
                FUN_0049bf80(param_1,0xbb9,0x502,(byte)(&DAT_004d5a59)[DAT_004c9754 * 0x64],0x0);
                FUN_0049bf80(param_1,0xbba,0x502,(byte)(&DAT_004d5a5a)[DAT_004c9754 * 0x64],0x0);
                FUN_0049ae60(param_1,0xc1c,0xc1e,0xc1e);
                FUN_0049ae60(param_1,0xc80,0xc82,0xc82);
                if (((byte)DAT_004d559c & 0x4) == 0x0) {
                    FUN_0049bf80(param_1,0x1bcb,0x410,0x1,0x0);
                }
                else {
                    local_20 = (((byte)DAT_004d559c & 0x8) != 0x0);
                    FUN_0049bf80(param_1,0x1bcb,0x502,local_20,0x0);
                }
                if (((byte)DAT_004d559c & 0x8) != 0x0) {
                    FUN_0049bf80(param_1,0x1bcb,0x410,0x1,0x0);
                    FUN_0049bf80(param_1,0x1bc8,0x410,0x1,0x0);
                }
                return 0x0;
            }
            if (param_2 == 0x405) {
                FUN_004953d7();
                FUN_00499050(DAT_0059679c,0x742d);
                pcVar2 = FUN_00499050(DAT_0059679c,0x7423);
                FUN_0049c2e0(local_b8,pcVar2);
                FUN_00497567(0x5,0x5,local_b8,0xc8,0xcaccce,-0x1,0xcaccce,DAT_004d6a6c,0x10);
                bVar8 = 0x10;
                uVar6 = 0xcaccce;
                iVar4 = -0x1;
                uVar5 = 0xcaccce;
                iVar3 = 0x3c;
                if (DAT_004d5588 == 0x0) {
                    local_38 = 0x7517;
                }
                else {
                    local_38 = 0x7516;
                }
                puVar1 = LPCSTR_005b9218;
                pcVar2 = FUN_00499050(DAT_0059679c,local_38);
                FUN_00497567(0xcd,0x113,pcVar2,iVar3,uVar5,iVar4,uVar6,puVar1,bVar8);
                bVar8 = 0x10;
                uVar6 = 0xcaccce;
                iVar4 = -0x1;
                uVar5 = 0xcaccce;
                iVar3 = 0x3c;
                if (DAT_004d558c == 0x0) {
                    local_34 = 0x7517;
                }
                else {
                    local_34 = 0x7516;
                }
                puVar1 = LPCSTR_005b9218;
                pcVar2 = FUN_00499050(DAT_0059679c,local_34);
                FUN_00497567(0xcd,0x127,pcVar2,iVar3,uVar5,iVar4,uVar6,puVar1,bVar8);
                bVar8 = 0x10;
                uVar6 = 0xcaccce;
                iVar4 = -0x1;
                uVar5 = 0xcaccce;
                iVar3 = 0x3c;
                if (DAT_004c9760 == 0x0) {
                    local_30 = 0x7517;
                }
                else {
                    local_30 = 0x7516;
                }
                puVar1 = LPCSTR_005b9218;
                pcVar2 = FUN_00499050(DAT_0059679c,local_30);
                FUN_00497567(0xcd,0x13b,pcVar2,iVar3,uVar5,iVar4,uVar6,puVar1,bVar8);
                bVar8 = 0x10;
                uVar6 = 0xcaccce;
                iVar4 = -0x1;
                uVar5 = 0xcaccce;
                iVar3 = 0x3c;
                if (DAT_004d5584 == 0x0) {
                    local_2c = 0x7517;
                }
                else {
                    local_2c = 0x7516;
                }
                puVar1 = LPCSTR_005b9218;
                pcVar2 = FUN_00499050(DAT_0059679c,local_2c);
                FUN_00497567(0xcd,0x14f,pcVar2,iVar3,uVar5,iVar4,uVar6,puVar1,bVar8);
                bVar8 = 0x10;
                uVar6 = 0xcaccce;
                iVar4 = -0x1;
                uVar5 = 0xcaccce;
                iVar3 = 0x3c;
                if (((byte)DAT_004d559c & 0x4) == 0x0) {
                    local_28 = 0x7517;
                }
                else {
                    local_28 = 0x7516;
                }
                puVar1 = LPCSTR_005b9218;
                pcVar2 = FUN_00499050(DAT_0059679c,local_28);
                FUN_00497567(0xcd,0x163,pcVar2,iVar3,uVar5,iVar4,uVar6,puVar1,bVar8);
                FUN_0049536f();
                return 0x0;
            }
        }
    }
    else {
        if (param_2 < 0x408) {
            if ((param_3 < 0xbb8) || (0xc1b < param_3)) {
                if ((param_3 < 0xc1c) || (0xc1e < param_3)) {
                    if ((param_3 < 0xc80) || (0xc82 < param_3)) {
                        if (param_3 < 0x1bc2) {
                            if (param_3 < 0x1bbf) {
                                if (param_3 == 0xc1d) {
                                    for (local_c0 = 0x0; local_c0 < 0x10; local_c0 = local_c0 + 0x1) {
                                        if (param_4 == 0x0) {
                                            (&DAT_00569b68)[local_c0 * 0x3 + DAT_004c9754 * 0x1e22] =
                                                (&DAT_00569b68)[local_c0 * 0x3 + DAT_004c9754 * 0x1e22] & 0xef;
                                            (&DAT_00569b98)[DAT_004c9754 * 0x1e22 + local_c0 * 0x3] =
                                                (&DAT_00569b98)[DAT_004c9754 * 0x1e22 + local_c0 * 0x3] & 0xef;
                                        }
                                        else {
                                            (&DAT_00569b68)[local_c0 * 0x3 + DAT_004c9754 * 0x1e22] =
                                                (&DAT_00569b68)[local_c0 * 0x3 + DAT_004c9754 * 0x1e22] | 0x10;
                                            (&DAT_00569b98)[DAT_004c9754 * 0x1e22 + local_c0 * 0x3] =
                                                (&DAT_00569b98)[DAT_004c9754 * 0x1e22 + local_c0 * 0x3] | 0x10;
                                        }
                                    }
                                    return 0x0;
                                }
                            }
                            else {
                                if (param_3 < 0x1bc0) {
                                    iVar3 = FUN_0049aea0(param_1,0xc1c,0xc1e);
                                    iVar4 = FUN_0049aea0(param_1,0xc80,0xc82);
                                    if (iVar3 == 0xc1c) {
                                        for (local_d0 = 0x0; local_d0 < 0x64; local_d0 = local_d0 + 0x1) {
                                            (&DAT_004d5c50)[DAT_004c9754 * 0x64 + local_d0] = 0x1;
                                        }
                                    }
                                    else {
                                        if (iVar3 == 0xc1d) {
                                            for (local_d4 = 0x0; local_d4 < 0x64; local_d4 = local_d4 + 0x1) {
                                                (&DAT_004d5c50)[DAT_004c9754 * 0x64 + local_d4] = 0x0;
                                            }
                                        }
                                    }
                                    if (iVar4 == 0xc80) {
                                        for (local_d8 = 0x0; local_d8 < 0x10; local_d8 = local_d8 + 0x1) {
                                            (&DAT_00569b68)[local_d8 * 0x3 + DAT_004c9754 * 0x1e22] =
                                                (&DAT_00569b68)[local_d8 * 0x3 + DAT_004c9754 * 0x1e22] | 0x10;
                                            (&DAT_00569b98)[DAT_004c9754 * 0x1e22 + local_d8 * 0x3] =
                                                (&DAT_00569b98)[DAT_004c9754 * 0x1e22 + local_d8 * 0x3] | 0x10;
                                        }
                                    }
                                    else {
                                        if (iVar4 == 0xc81) {
                                            for (local_dc = 0x0; local_dc < 0x10; local_dc = local_dc + 0x1) {
                                                (&DAT_00569b68)[local_dc * 0x3 + DAT_004c9754 * 0x1e22] =
                                                    (&DAT_00569b68)[local_dc * 0x3 + DAT_004c9754 * 0x1e22] & 0xef;
                                                (&DAT_00569b98)[DAT_004c9754 * 0x1e22 + local_dc * 0x3] =
                                                    (&DAT_00569b98)[DAT_004c9754 * 0x1e22 + local_dc * 0x3] & 0xef;
                                            }
                                        }
                                    }
                                    FUN_0049c140(param_1,0x0);
                                }
                                else {
                                    if (param_3 == 0x1bc1) {
                                        autosave_opt_00599df0 = param_4;
                                        FUN_00430d8a(s_Options_004c20fc,s_autosave_004c20f3,param_4,&filename_00599c80);
                                    }
                                }
                            }
                        }
                        else {
                            if (param_3 < 0x1bc3) {
                                sound_opt_00599de8 = param_4;
                                FUN_00430d8a(s_Options_004c2117,s_sound_004c2111,param_4,&filename_00599c80);
                                if (sound_opt_00599de8 == 0x0) {
                                    FUN_004a12c7();
                                }
                                else {
                                    FUN_004a1243();
                                }
                            }
                            else {
                                if (param_3 < 0x1bc7) {
                                    if (param_3 == 0x1bc5) {
                                        grid_opt_00599de0 = param_4;
                                        FUN_00430d8a(s_Options_004c2109,&DAT_004c2104,param_4,&filename_00599c80);
                                    }
                                }
                                else {
                                    if (param_3 < 0x1bc8) {
                                        if (DAT_004d6a68 != 0x0) {
                                            FUN_004a4c20(DAT_004d6a68,param_1);
                                        }
                                    }
                                    else {
                                        if (param_3 < 0x1bc9) {
                                            if (param_4 == 0x0) {
                                                DAT_004d559c._0_1_ = (byte)DAT_004d559c & 0xfb;
                                                FUN_0049bf80(param_1,0x1bcb,0x502,0x0,0x0);
                                                FUN_0049bf80(param_1,0x1bcb,0x410,0x0,0x0);
                                            }
                                            else {
                                                DAT_004d559c._0_1_ = (byte)DAT_004d559c | 0x4;
                                                FUN_0049bf80(param_1,0x1bcb,0x40f,0x0,0x0);
                                            }
                                        }
                                        else {
                                            if (param_3 == 0x1bca) {
                                                FUN_0045f499(0x1);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    else {
                        FUN_0049ae60(param_1,0xc80,0xc82,param_3);
                    }
                }
                else {
                    FUN_0049ae60(param_1,0xc1c,0xc1e,param_3);
                }
            }
            else {
                (&DAT_004d4ea0)[DAT_004c9754 * 0x64 + param_3] = param_4;
            }
        }
        else {
            if (0x410 < param_2) {
                if (param_2 < 0x412) {
                    return 0x1;
                }
                if (param_2 == 0x415) {
                    if (((byte)DAT_004d559c & 0x4) != 0x0) {
                        local_24 = FUN_0049bf80(param_1,0x1bcb,0x501,0x0,0x0);
                        if ((local_24 != 0x0) && (((byte)DAT_004d559c & 0x8) == 0x0)) {
                        uVar9 = 0x1;
                        uVar7 = 0xffffffff;
                        puVar1 = FUN_00499050(DAT_0059679c,0x7429);
                        FUN_0045518a(0x1f,0xffffffff,0x74cc,0xffffffff,puVar1,uVar7,uVar9);
                        DAT_004d559c._0_1_ = (byte)DAT_004d559c | 0x8;
                        }
                        if ((local_24 == 0x0) && (((byte)DAT_004d559c & 0x8) != 0x0)) {
                        uVar9 = 0x1;
                        uVar7 = 0xffffffff;
                        puVar1 = FUN_00499050(DAT_0059679c,0x7428);
                        FUN_0045518a(0x1f,0xffffffff,0x74cc,0xffffffff,puVar1,uVar7,uVar9);
                        DAT_004d559c._0_1_ = (byte)DAT_004d559c & 0xf7;
                        }
                    }
                    return 0x1;
                }
            }
        }
    }
    return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0045d0c7(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: *mut i32) -> u32

{
    let mut bVar1: bool;
    undefined3 extraout_var;
    let mut pcVar2: String;
    u32 local_98 [0x21];
    let mut local_14: i32;

    local_14 = param_1 >> 0x1;
    if (((&DAT_00569cd2)[DAT_004c9754 * 0x1e22] & 0x1) != 0x0) {
        local_14 = local_14 + immune_plague_bonus_00599dac;
    }
    local_14 = local_14 + DAT_005b2d74;
    if (param_4 <= local_14) {
        return 0x1;
    }
    bVar1 = FUN_0046292a((&DAT_00568210 + DAT_004c9754 * 0x1e22));
    if (CONCAT31(extraout_var,bVar1) != 0x0) {
        if ((DAT_004d78c7 >> 0x18 == DAT_004c9754) && (cRam004d78cb == param_2)) {
            if (local_14 + _DAT_004d7865 < param_4) {
                return 0x0;
            }
            if (*param_5 == 0x0) {
                pcVar2 = FUN_00499050(DAT_0059679c,0x7457);
                FUN_0049c2e0(local_98,pcVar2);
                FUN_0045518a(0x1 << ((byte)DAT_004c9754 & 0x1f),0xffffffff,0x74cc,param_2,local_98,0xffffffff,0x0);
            }
            return 0x1;
        }
        if ((param_3 != 0x0) && (DAT_004d7b98 >> 0x18 == DAT_004c9754)) {
            if (local_14 + _DAT_004d7b36 < param_4) {
                return 0x0;
            }
            if (*param_5 == 0x0) {
                pcVar2 = FUN_00499050(DAT_0059679c,0x745e);
                FUN_0049c2e0(local_98,pcVar2);
                FUN_0045518a(0x1 << ((byte)DAT_004c9754 & 0x1f),0xffffffff,0x74cc,param_2,local_98,0xffffffff,0x0);
            }
            return 0x1;
        }
    }
    return 0x0;
}



fn FUN_0045d2af(param_1: i32,param_2: i32,param_3: i32)

{
    let mut iVar1: i32;
    let local_1c: *mut u32;
    let local_18: *mut u32;

    for (local_1c = (&DAT_005b8b44 + param_1 * 0x4);
        (local_1c != 0x0 && ((local_1c + 0x8) == param_1)); local_1c = *local_1c) {
        if (((*(local_1c + 0x3a) & 0x1) != 0x0) &&
            ((((*(*(&DAT_00582938 +
                (*(local_1c + 0x25) >> 0x18) * 0x4 + (local_1c[0x9] >> 0x18) * 0x18) +
                0x119) != 0x0 && ((*(local_1c + 0x3b) & 0x4) == 0x0)) &&
                ((*(local_1c + 0x3b) & 0x8) == 0x0)) &&
                (iVar1 = FUN_0044a87f((local_1c + 0x22),(local_1c + 0x9),param_2,param_3),
                 iVar1 < 0x6)))) {
            *(local_1c + 0x3b) = *(local_1c + 0x3b) | 0x8;
        }
    }
    for (local_18 = (&DAT_005b89f8 + param_1 * 0x4);
        (local_18 != 0x0 && (*(local_18 + 0x6) >> 0x10 == param_1));
        local_18 = *local_18) {
        if (((*(local_18 + 0xb) & 0x10) == 0x0) &&
            (((*(local_18 + 0xb) & 0x20) == 0x0 &&
                (iVar1 = FUN_0044a87f(local_18[0x2] >> 0x10,*(local_18 + 0xa) >> 0x10,param_2,param_3),
                 iVar1 < 0x6)))) {
            *(local_18 + 0xb) = *(local_18 + 0xb) | 0x20;
        }
    }
    return;
}



fn FUN_0045d45a(param_1: i32)

{
    i32 **ppiVar1;
    let mut pcVar2: String;
    u32 local_b0 [0x20];
    let local_30: *mut u32;
    let mut local_2c: u32;
    let mut local_28: i32;
    i32 **local_24;
    i32 **local_20;
    let mut local_1c: i32;
    let mut local_18: u32;
    let mut local_14: i32;

    if (param_1 != 0x0) {
        FUN_004a0430(&DAT_005b2d78,0x0,0xa0);
    }
    if (DAT_004d5588 != 0x0) {
        local_2c = 0xffffffff;
        local_18 = 0xffffffff;
        local_28 = -0x1;
        for (local_30 = *DAT_005967b0; local_30 != 0x0; local_30 = *local_30) {
            if (((*(local_30 + 0x23) >> 0x18 == DAT_004c9754) && ((*(local_30 + 0x3a) & 0x1) != 0x0))
                && (*(*(&DAT_00582938 +
                (*(local_30 + 0x25) >> 0x18) * 0x4 + (local_30[0x9] >> 0x18) * 0x18) +
                0x119) != 0x0)) {
                if ((local_30 + 0x8) != local_28) {
                    local_28 = (local_30 + 0x8);
                    DAT_005b2d74 = FUN_00481c66(local_28);
                }
                FUN_0045d872(local_30,param_1);
            }
        }
        local_24 = (i32 **)*DAT_005967c8;
        local_28 = -0x1;
        while (local_24 != (i32 **)0x0) {
            local_20 = (i32 **)*local_24;
            if (*(local_24 + 0xe) >> 0x10 == DAT_004c9754) {
                if (*(local_24 + 0x6) >> 0x10 != local_28) {
                    local_28 = *(local_24 + 0x6) >> 0x10;
                    DAT_005b2d74 = FUN_00481c66(local_28);
                }
                FUN_0045dc08(local_24,param_1);
            }
            local_24 = local_20;
        }
        local_1c = -0x1;
        for (local_30 = *DAT_005967b0; local_30 != 0x0; local_30 = *local_30) {
            if ((((local_30 + 0x8) != local_1c) && (*(local_30 + 0x23) >> 0x18 == DAT_004c9754)) &&
                ((*(local_30 + 0x3b) & 0x4) != 0x0)) {
                local_1c = (local_30 + 0x8);
                if ((*(&DAT_005b2d78 + local_1c * 0x4) == 0x0) || (param_1 != 0x0)) {
                    *(&DAT_005b2d78 + local_1c * 0x4) = 0x1;
                }
                else {
                    *(&DAT_005b2d78 + local_1c * 0x4) = 0x0;
                }
            }
        }
        local_1c = -0x1;
        ppiVar1 = (i32 **)*DAT_005967c8;
        while (local_24 = ppiVar1, local_24 != (i32 **)0x0) {
            local_20 = (i32 **)*local_24;
            ppiVar1 = local_20;
            if ((((*(local_24 + 0x6) >> 0x10 != local_1c) &&
                (*(&DAT_005b2d78 + (*(local_24 + 0x6) >> 0x10) * 0x4) == 0x0)) &&
                (*(local_24 + 0xe) >> 0x10 == DAT_004c9754)) && ((*(local_24 + 0xb) & 0x10) != 0x0)) {
                local_1c = *(local_24 + 0x6) >> 0x10;
                if ((*(&DAT_005b2d78 + local_1c * 0x4) == 0x0) || (param_1 != 0x0)) {
                    *(&DAT_005b2d78 + local_1c * 0x4) = 0x1;
                }
                else {
                    *(&DAT_005b2d78 + local_1c * 0x4) = 0x0;
                }
            }
        }
        for (local_14 = 0x0; local_14 < 0x28; local_14 = local_14 + 0x1) {
            if (*(&DAT_005b2d78 + local_14 * 0x4) != 0x0) {
                pcVar2 = FUN_00499050(DAT_0059679c,0x738d);
                FUN_0049c2e0(local_b0,pcVar2);
                FUN_0045518a(0x1 << ((byte)DAT_004c9754 & 0x1f),0xffffffff,0x738d,local_14,local_b0,0xffffffff,0x0);
            }
        }
        FUN_004864f7();
    }
    return;
}



fn FUN_0045d872(param_1: *mut u32,param_2: i32)

{
    let mut uVar1: u32;
    let mut iVar2: i32;
    let piVar3: *mut i32;;
    let local_6c: *mut u32;
    let mut local_68: i32;
    let mut local_64: i32;
    let mut local_60: i32;
    let mut local_5c: i32;
    let mut local_58: i32;
    let local_54: *mut u32;
    let local_50: *mut u32;
    let local_4c: *mut u32;
    let local_48: *mut u32;
    let mut local_44: i32;
    let local_40: *mut u32;
    let local_3c: *mut u32;
    let local_38: *mut u32;
    let local_34: *mut u32;
    let local_30: *mut u32;
    let local_2c: *mut u32;
    let mut local_28: i32;
    let mut local_24: u32;
    let mut local_20: u32;
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: u32;

    local_68 = 0x0;
    local_64 = 0x0;
    local_54 = param_1 + 0x8;
    local_24 = local_54 & 0xffff0000 | (param_1 + 0x22);
    local_60 = (param_1 + 0x22);
    local_4c = param_1 + 0x8;
    local_14 = local_4c & 0xffff0000 | (param_1 + 0x9);
    local_5c = (param_1 + 0x9);
    local_3c = param_1 + 0x8;
    local_20 = local_3c & 0xffff0000 | (param_1 + 0x9);
    local_28 = (param_1 + 0x9);
    local_34 = param_1 + 0x8;
    local_1c = local_34 & 0xffff0000 | (param_1 + 0x22);
    local_44 = (param_1 + 0x22);
    local_40 = param_1 + 0x8;
    local_18 = local_40 & 0xffff0000 | local_40;
    local_50 = local_54;
    local_48 = local_4c;
    local_38 = local_3c;
    local_30 = local_40;
    local_2c = local_34;
    local_58 = FUN_00489176(local_40,local_44,local_28,0xa);
    local_6c = param_1;
    while( true ) {
        if (local_6c == 0x0) {
            if (local_64 != 0x0) {
                FUN_0045d2af((param_1 + 0x8),(param_1 + 0x22),(param_1 + 0x9))
                ;
            }
            return;
        }
        if ((local_6c + 0x22) != local_60) break;
        if ((local_6c + 0x9) != local_5c) {
            return;
        }
        if (((*(local_6c + 0x3b) & 0x4) == 0x0) && ((*(local_6c + 0x3b) & 0x8) == 0x0)) {
            if (((param_2 != 0x0) && (local_6c[0xc] >> 0x18 < 0x19)) && ((*(local_6c + 0x3b) & 0x1) != 0x0))
            {
                piVar3 = &local_68;
                uVar1 = FUN_004a2edc();
                iVar2 = FUN_0045d0c7(local_6c[0xc] >> 0x18,(local_6c + 0x8),local_58,uVar1 % 0x64,
                                     piVar3);
                if (iVar2 == 0x0) {
                    *(local_6c + 0x3b) = *(local_6c + 0x3b) | 0x4;
                    *(local_6c + 0x3b) = *(local_6c + 0x3b) & 0xf7;
                    local_64 = 0x1;
                    uVar1 = FUN_004a2edc();
                    FUN_00488efd(local_6c,uVar1 % 0xf + 0x6);
                }
            }
        }
        else {
            piVar3 = &local_68;
            uVar1 = FUN_004a2edc();
            iVar2 = FUN_0045d0c7(local_6c[0xc] >> 0x18,(local_6c + 0x8),local_58,uVar1 % 0x64,piVar3)
            ;
            if (iVar2 == 0x0) {
                *(local_6c + 0x3b) = *(local_6c + 0x3b) | 0x4;
                if (param_2 != 0x0) {
                    local_64 = 0x1;
                    uVar1 = FUN_004a2edc();
                    FUN_00488efd(local_6c,uVar1 % 0xf + 0x6);
                }
            }
            else {
                *(local_6c + 0x3b) = *(local_6c + 0x3b) & 0xf7;
            }
        }
        local_6c = *local_6c;
    }
    return;
}



fn FUN_0045dc08(param_1: *mut i32,param_2: i32)

{
    let mut uVar1: u32;
    let mut iVar2: i32;
    let piVar3: *mut i32;;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    local_1c = 0x0;
    local_18 = 0x0;
    local_14 = FUN_00489176(*(param_1 + 0x6) >> 0x10,param_1[0x2] >> 0x10,*(param_1 + 0xa) >> 0x10
                            ,0xa);
    if (((*(param_1 + 0xb) & 0x10) == 0x0) && ((*(param_1 + 0xb) & 0x20) == 0x0)) {
        if (((param_2 != 0x0) && (*(param_1 + 0x26) >> 0x10 < 0x19)) &&
            ((*(param_1 + 0xb) & 0x8) != 0x0)) {
            piVar3 = &local_1c;
            uVar1 = FUN_004a2edc();
            iVar2 = FUN_0045d0c7(*(param_1 + 0x26) >> 0x10,*(param_1 + 0x6) >> 0x10,local_14,
                                 uVar1 % 0x64,piVar3);
            if (iVar2 == 0x0) {
                *(param_1 + 0xb) = *(param_1 + 0xb) | 0x10;
                *(param_1 + 0xb) = *(param_1 + 0xb) & 0xdf;
                local_18 = 0x1;
                uVar1 = FUN_004a2edc();
                (param_1 + 0xa) =
                    (*(param_1 + 0x26) >> 0x10) - (((longlong)uVar1 % 0xf) + 0x6);
                if (*(param_1 + 0x26) >> 0x10 < 0x0) {
                    FUN_004811e6(param_1);
                }
            }
        }
    }
    else {
        piVar3 = &local_1c;
        uVar1 = FUN_004a2edc();
        iVar2 = FUN_0045d0c7(*(param_1 + 0x26) >> 0x10,*(param_1 + 0x6) >> 0x10,local_14,
                             uVar1 % 0x64,piVar3);
        if (iVar2 == 0x0) {
            *(param_1 + 0xb) = *(param_1 + 0xb) | 0x10;
            if (param_2 != 0x0) {
                local_18 = 0x1;
                uVar1 = FUN_004a2edc();
                (param_1 + 0xa) =
                    (*(param_1 + 0x26) >> 0x10) - (((longlong)uVar1 % 0xf) + 0x6);
                if (*(param_1 + 0x26) >> 0x10 < 0x0) {
                    FUN_004811e6(param_1);
                }
            }
        }
        else {
            *(param_1 + 0xb) = *(param_1 + 0xb) & 0xdf;
        }
    }
    if (local_18 != 0x0) {
        FUN_0045d2af(*(param_1 + 0x6) >> 0x10,param_1[0x2] >> 0x10,*(param_1 + 0xa) >> 0x10);
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0045ddf0() -> u32

{
    let mut pcVar1: String;
    let mut iVar2: i32;
    u32 local_218 [0x80];
    let mut local_18: u32;
    let mut local_14: u32;

    _DAT_005b885c = 0x0;
    FUN_004953d7();
    FUN_00496a10();
    FUN_0049536f();
    FUN_0045e179();
    FUN_004953d7();
    FUN_00496a10();
    FUN_0049536f();
    if ((_DAT_004c9728 == 0x0) || (((byte)DAT_004d559c & 0x2) != 0x0)) {
if ((DAT_004c9754 < 0x5) && (_DAT_00596a50 != 0x0)) {
FUN_00405178();
FUN_004953d7();
FUN_00496a10();
FUN_0049536f();
}
FUN_00462543((&DAT_00568210 + DAT_004c9754 * 0x1e22));
FUN_00489246(DAT_004c9754,0x0);
FUN_0045e963();
FUN_0045e9de();
FUN_00462d47();
FUN_00464c97();
FUN_0047051a();
FUN_0046cb74();
FUN_0045d45a(0x1);
}
    else {
_DAT_004c9728 = 0x0;
}
    if (DAT_004c9754 < 0x5) {
        FUN_004553fd(0x0,0x1);
    }
    if (DAT_00595738 != -0x1) {
        FUN_004a9a64(0x1);
    }
    _DAT_00596a44 = 0x0;
    DAT_005b7068 = 0x0;
    DAT_005b8808 = 0x0;
    local_18 = FUN_00473acd();
    if (((local_18 != 0x66) && (local_18 != 0x3e8)) && (local_18 != 0x3e9)) {
        local_14 = local_18;
        if ((((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x10) != 0x0) &&
            (iVar2 = *(&DAT_00569ad4 + DAT_004c9754 * 0x1e22),
             *(&DAT_00569ad4 + DAT_004c9754 * 0x1e22) = *(&DAT_00569ad4 + DAT_004c9754 * 0x1e22) + -0x1,
             iVar2 == 0x0)) {
            (&DAT_00569a98)[DAT_004c9754 * 0x1e22] = (&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0xef;
            FUN_00408371(DAT_004c9754);
        }
        if ((((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x40) != 0x0) &&
            (iVar2 = *(&DAT_00569ad8 + DAT_004c9754 * 0x1e22),
             *(&DAT_00569ad8 + DAT_004c9754 * 0x1e22) = *(&DAT_00569ad8 + DAT_004c9754 * 0x1e22) + -0x1,
             iVar2 == 0x0)) {
            (&DAT_00569a98)[DAT_004c9754 * 0x1e22] = (&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0xbf;
        }
        FUN_00455648();
        if (*(&DAT_00569b0c + DAT_004c9754 * 0x1e22) != 0x0) {
            *(&DAT_00569b0c + DAT_004c9754 * 0x1e22) = 0x0;
            pcVar1 = FUN_00499050(DAT_0059679c,0x737e);
            FUN_0049c2e0(local_218,pcVar1);
            FUN_0045518a(0x1 << ((byte)DAT_004c9754 & 0x1f),0x5,0x737e,0xffffffff,local_218,0xffffffff,0x0);
        }
        iVar2 = FUN_0042f963(DAT_004c9754,0x1,0x0);
        *(&DAT_00569abd + DAT_004c9754 * 0x1e22) = iVar2;
        _DAT_00574f89 =
            _DAT_00574f89 +
                ((0x64 - *(&DAT_00569ae4 + DAT_004c9754 * 0x1e22)) * *(&DAT_00569af8 + DAT_004c9754 * 0x1e22)) /
                    0x64;
        if (0xf423f < _DAT_00574f89) {
            _DAT_00574f89 = 0xf423f;
        }
        if ((*(&DAT_00569b08 + DAT_004c9754 * 0x1e22) == 0x0) &&
            (*(&DAT_00569b00 + DAT_004c9754 * 0x1e22) != 0x0)) {
            *(&DAT_00569b08 + DAT_004c9754 * 0x1e22) = dflt_loan_turns_00599d7c;
        }
        FUN_0045e87b();
        FUN_00482c8a(DAT_004c9754);
        FUN_0045d45a(0x0);
        iVar2 = FUN_0046031c(DAT_004c9754,0x0);
        (&DAT_004d566c + DAT_00591cb4 * 0x2 + DAT_004c9754 * 0xc8) = iVar2;
        local_18 = 0x1;
    }
    return local_18;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0045e179()

{
    let mut local_114: *mut u32 [0x11];
    let ppuStack207: *mut *mut u8;;
    let mut local_23: String;;
    PCHAR local_1c;
    let local_18: *mut i32;;

    local_1c = FUN_0049c2c9(0x104);
    local_18 = FUN_004990e0(local_114,0x0,s_efs_res_004c212a,s_PlrTurnDlg_004c211f);
    if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
        FUN_0049c2e0(local_1c,s_s_house_d_wav_004c2132);
        FUN_004a1651();
        FUN_0049bb50(local_114,FUN_0045e370);
    }
    if (((_DAT_004c9728 == 0x0) || (((byte)DAT_004d559c & 0x2) != 0x0)) && (DAT_00591cb4 == 0x0)) {
    if ((*(&DAT_00569b4c + DAT_004c9754 * 0x1e22) < 0x2) && (DAT_004d5594 != 0x0)) {
        FUN_004a0430(&DAT_004d5a58 + DAT_004c9754 * 0x64,0x1,0x64);
        FUN_004a0430(&DAT_004d5c50 + DAT_004c9754 * 0x64,0x1,0x64);
    }
    else {
        FUN_004a0430(&DAT_004d5a58 + DAT_004c9754 * 0x64,0x0,0x64);
        FUN_004a0430(&DAT_004d5c50 + DAT_004c9754 * 0x64,0x0,0x64);
    }
    FUN_00483355(0x5);
    if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
        FUN_0045ee03();
        FUN_0045f1a0();
        FUN_0045f499(0x0);
    }
    FUN_00482e50(DAT_004c9754);
    FUN_00482c8a(DAT_004c9754);
}
    FUN_0049af50(local_1c);
    ppuStack207 = &PTR_FUN_004c3d34;
    if (local_23 != 0x0) {
        FUN_00499b30(local_114,local_23);
    }
    FUN_0049a1c0(local_114,0x1);
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0045e370(param_1: *mut *mut u32,param_2: u32,param_3: u32,param_4: i32) -> u32

{
    let mut iVar1: i32;
    let mut pcVar2: String;
    let mut uVar3: u32;
    let local_70: u8 [0x14];
    let mut local_5c: u32;
    let local_58: u8 [0x44];
    let mut local_14: u32;

    local_14 = param_2;
    if (param_2 < 0x405) {
        if (0xff < param_2) {
            if (param_2 < 0x101) {
                if (param_4 != 0x1b) {
                    return 0x0;
                }
                FUN_0045e370(param_1,0x407,0x64,0x0);
                return 0x1;
            }
            if (param_2 == 0x401) {
                _DAT_005b2e38 = 0x0;
                if (((byte)DAT_004d559c & 0x8) == 0x0) {
                    FUN_0049bf80(param_1,0x72,0x414,0x0,0xfedc);
                    FUN_0049bf80(param_1,0x70,0x414,0x0,0xfedc);
                    FUN_0049bf80(param_1,0x70,0x410,0x0,0x0);
                }
                if (((&DAT_005b2e1c + DAT_004c9754 * 0x2) != 0x0) || (((byte)DAT_004d559c & 0x8) == 0x0)) {
                FUN_0049bf80(param_1,0x73,0x414,0x0,0xfedc);
                FUN_0049bf80(param_1,0x71,0x414,0x0,0xfedc);
                FUN_0049bf80(param_1,0x71,0x410,0x0,0x0);
                }
                return 0x1;
            }
        }
    }
    else {
        if (param_2 < 0x406) {
            FUN_004953d7();
            FUN_00496ac0((&DAT_00595700 + DAT_004c9754 * 0x4),0x32,0xbe,0x64,0x64);
            FUN_0049e640(0x32,0xbe,0x64,0x64,0xce,0xca,0xcc,0x1);
            FUN_00496ac0((&DAT_00595700 + DAT_004c9754 * 0x4),0x1ea,0xbe,0x64,0x64);
            FUN_0049e640(0x1ea,0xbe,0x64,0x64,0xce,0xca,0xcc,0x1);
            pcVar2 = FUN_00499050(DAT_0059679c,0x7348);
            FUN_0049c2e0(local_58,pcVar2);
            FUN_00497567(0x140,0xd2,local_58,0xc8,0xcaccce,-0x1,0xcaccce,LPCSTR_005b9218,0x11);
            FUN_00499050(DAT_0059679c,DAT_004c9754 + 0x414);
            pcVar2 = FUN_00499050(DAT_0059679c,0x73cc);
            FUN_0049c2e0(local_58,pcVar2);
            pcVar2 = FUN_00499050(DAT_0059679c,0x7347);
            FUN_0049c2e0(local_58,pcVar2);
            FUN_00497567(0x140,0xfa,local_58,0x12c,0xcaccce,-0x1,0xcaccce,LPCSTR_005b9218,0x11);
            FUN_0049536f();
            return 0x1;
        }
        if (0x406 < param_2) {
            if (param_2 < 0x408) {
                local_5c = param_3;
                if (param_3 < 0x70) {
                    if (param_3 != 0x64) {
                        return 0x0;
                    }
                }
                else {
                    if ((0x70 < param_3) && (param_3 != 0x71)) {
                        return 0x0;
                    }
                }
                if (((byte)DAT_004d559c & 0x8) != 0x0) {
                    FUN_0049bf80(param_1,0x70,0x501,0xd,local_70);
                    if ((&DAT_005b2e1c + DAT_004c9754 * 0x2) == 0x0) {
                        FUN_0049bf80(param_1,0x71,0x501,0xd,&stack0xffffff7c);
                        iVar1 = FUN_004a2f10(local_70,&stack0xffffff7c);
                        if (iVar1 == 0x0) {
                            *(&DAT_005b2e1c + DAT_004c9754 * 0x2) = 0x1;
                            uVar3 = FUN_00461135(local_70);
                            (&DAT_005b2e26 + DAT_004c9754 * 0x2) = uVar3;
                            FUN_0049c140(param_1,0x1);
                            return 0x1;
                        }
                        pcVar2 = FUN_00499050(DAT_0059679c,0x742a);
                        FUN_0049d2e0(param_1,0x1,pcVar2);
                        return 0x1;
                    }
                    iVar1 = DAT_004c9754 * 0x2;
                    uVar3 = FUN_00461135(local_70);
                    if (uVar3 != (&DAT_005b2e26 + iVar1)) {
                        _DAT_005b2e38 = _DAT_005b2e38 + 0x1;
                        if (0x2 < _DAT_005b2e38) {
                            pcVar2 = FUN_00499050(DAT_0059679c,0x742b);
                            pop_err_msg_box_and_exit_004a02f5(pcVar2);
                        }
                        pcVar2 = FUN_00499050(DAT_0059679c,0x742b);
                        FUN_0049d2e0(param_1,0x1,pcVar2);
                        return 0x1;
                    }
                }
                FUN_0049c140(param_1,0x1);
            }
            else {
                if (param_2 == 0x40c) {
                    FUN_004953d7();
                    FUN_004a08c5(s_pcx_cathed3_pcx_004c2140,0x0,0x0,0x280,0x1e0,0x0,0x0,0x0,0x1);
                    FUN_0049536f();
                    return 0x1;
                }
            }
        }
    }
    return 0x0;
}



fn FUN_0045e87b()

{
    let mut iVar1: i32;
    let local_14: *mut u32;

    if (DAT_004c9754 < 0x5) {
        iVar1 = *(&DAT_00569ae8 + DAT_004c9754 * 0x1e22) - *(&DAT_00569af0 + DAT_004c9754 * 0x1e22);
        if (iVar1 != 0x0) {
            for (local_14 = *DAT_005967b0; local_14 != 0x0; local_14 = *local_14) {
                if (*(local_14 + 0x23) >> 0x18 == DAT_004c9754) {
                    if (iVar1 < 0x0) {
                        FUN_00486b30((local_14 + 0x8),-iVar1);
                    }
                    else {
                        FUN_00486af8((local_14 + 0x8),iVar1);
                    }
                }
            }
        }
        *(&DAT_00569aec + DAT_004c9754 * 0x1e22) = *(&DAT_00569adc + DAT_004c9754 * 0x1e22);
        *(&DAT_00569af0 + DAT_004c9754 * 0x1e22) = *(&DAT_00569ae8 + DAT_004c9754 * 0x1e22);
    }
    return;
}



fn FUN_0045e963()

{
    let local_14: *mut u32;

    for (local_14 = *DAT_005967c8; local_14 != 0x0; local_14 = *local_14) {
        if (*(local_14 + 0xe) >> 0x10 == DAT_004c9754) {
            if ((*(local_14 + 0x26) >> 0x10 < 0x64) &&
                ((local_14 + 0xa) = (local_14 + 0xa) + city_heal_rate_00599db0,
                 0x64 < *(local_14 + 0x26) >> 0x10)) {
                *(local_14 + 0xa) = 0x64;
            }
            *(local_14 + 0xb) = *(local_14 + 0xb) & 0xf7;
        }
    }
    return;
}

