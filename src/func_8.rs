
fn FUN_0041361b(param_1: *mut i32)

{
    let bVar1: u8;
    let bVar2: u8;
    let mut pcVar3: String;
    let puVar4: *mut u32;
    let mut iVar5: i32;
    let mut iVar6: i32;
    let mut local_300: i32;
    let mut local_2fc: u32;
    let local_2f8: *mut i32;;
    let local_2f4: *mut i32;;
    let mut local_2f0: u32;
    let local_2ec: *mut i32;;
    let local_2e8: *mut i32;;
    let mut local_2e4: u32;
    let mut local_2e0: u32;
    let local_2dc: *mut u32;
    let local_2d8: *mut u32;
    let mut local_2d4: u32;
    let local_2d0: *mut u32;
    let local_2cc: *mut u32;
    let mut local_2c8: i32;
    let mut local_2c4: u32;
    let local_2c0: *mut u32;
    let local_2bc: *mut u32;
    let mut local_2b8: i32;
    let mut local_2b4: u32;
    let local_2b0: *mut u32;
    let local_2ac: *mut u32;
    let mut local_2a8: u32;
    let local_2a4: *mut u32;
    let local_2a0: *mut u32;
    let local_29c: *mut u32;
    let mut local_298: u32;
    let mut local_294: u32;
    let local_290: *mut u32;
    let mut local_28c: u32;
    let mut local_288: u32;
    let local_284: *mut u32;
    let mut local_280: i32;
    let mut local_27c: u32;
    let local_278: *mut i32;;
    let local_274: *mut i32;;
    let mut local_270: u32;
    u32 local_26c [0x7d];
    let mut local_78: i32;
    let local_74: *mut i32;;
    let mut local_70: u32;
    ushort *local_6c;
    ushort *local_68;
    let mut local_64: u32;
    let mut local_60: u32;
    let mut local_5c: u32;
    let mut local_58: u32;
    let mut local_54: i32;
    let mut local_50: u32;
    let mut local_4c: u32;
    let local_48: *mut u32;
    let mut local_44: i32;
    let mut local_40: u32;
    let mut local_3c: u32;
    let local_38: *mut u32;
    let mut local_34: u32;
    let local_30: *mut i32;;
    let local_2c: *mut i32;;
    let mut local_28: u32;
    let mut local_24: u32;
    let local_20: *mut i32;;
    let local_1c: *mut i32;;
    let mut local_18: u32;
    let local_14: u8;

    FUN_00431d0a(&DAT_005967b8);
    DAT_005967bc = FUN_00434de1(param_1);
    local_14 = *(param_1 + 0x12);
    if (local_14 < 0xa) {
        if (local_14 < 0x5) {
            if ((local_14 != 0x0) && (local_14 == 0x1)) {
                bVar1 = *(param_1 + 0x41);
                bVar2 = *(param_1 + 0x42);
                FUN_004248db();
                if (((param_1 + 0x22) == bVar1) && ((param_1 + 0x9) == bVar2)) {
                    if ((param_1 + 0x8) == DAT_004d557c) {
                        FUN_00413546(&DAT_005967b8);
                    }
                    else {
                        iVar5 = FUN_0043ab95((param_1 + 0x8),(param_1 + 0x22),
                                             (param_1 + 0x9),*(param_1 + 0x23) >> 0x18,
                                             *(param_1 + 0x32) >> 0x18,*(param_1 + 0x46) >> 0x18);
                        if (iVar5 == 0x1) {
                            FUN_00420d18(&DAT_004c7510,(param_1 + 0x8),bVar1,bVar2);
                            FUN_00484b4e(param_1);
                        }
                        else {
                            FUN_00413546(&DAT_005967b8);
                        }
                    }
                }
                else {
                    if (((param_1 + 0x41) == -0x1) || ((param_1 + 0x42) == -0x1)) {
                        FUN_00413546(&DAT_005967b8);
                    }
                }
            }
        }
        else {
            if (local_14 < 0x6) {
                FUN_004248db();
                local_18 = *(param_1 + 0x41);
                local_20 = param_1 + 0x8;
                local_24 = local_20 & 0xffff0000 | (param_1 + 0x22);
                if ((param_1 + 0x22) == local_18) {
                    local_28 = *(param_1 + 0x42);
                    local_30 = param_1 + 0x8;
                    local_34 = local_30 & 0xffff0000 | (param_1 + 0x9);
                    if ((param_1 + 0x9) == local_28) {
                        local_2c = local_30;
                        local_1c = local_20;
                        FUN_00413546(&DAT_005967b8);
                    }
                }
            }
            else {
                if (local_14 < 0x7) {
                    FUN_004248db();
                    local_38 = &DAT_005967b8;
                    local_3c = (DAT_005967bc == 0x0);
                    local_40 = local_3c;
                    if (local_3c == 0x0) {
                        FUN_00413546(&DAT_005967b8);
                    }
                }
                else {
                    if (local_14 == 0x7) {
                        local_44 = FUN_00432bd3(&DAT_005967b8);
                        FUN_004248db();
                        local_48 = &DAT_005967b8;
                        local_4c = (DAT_005967bc == 0x0);
                        local_50 = local_4c;
                        if (local_4c == 0x0) {
                            local_54 = FUN_00432bd3(&DAT_005967b8);
                            if ((local_54 == 0x0) || (local_54 == local_44)) {
                                FUN_00413546(&DAT_005967b8);
                            }
                            else {
                                FUN_004135e4(&DAT_005967b8);
                            }
                        }
                    }
                }
            }
        }
    }
    else {
        if (local_14 < 0xb) {
            FUN_00424e53(DAT_005967bc,*(param_1 + 0x41));
            FUN_00413546(&DAT_005967b8);
        }
        else {
            if (local_14 < 0xd) {
                if (local_14 < 0xc) {
                    local_278 = param_1 + 0x8;
                    local_27c = local_278 & 0xffff0000 | local_278;
                    local_280 = local_278;
                    local_274 = local_278;
                    FUN_0041c01d(param_1);
                    local_284 = &DAT_005967b8;
                    local_288 = (DAT_005967bc == 0x0);
                    local_28c = local_288;
                    if ((local_288 == 0x0) && (iVar5 = FUN_00410fb3(&DAT_005967b8,0x0), iVar5 != 0x0)) {
                        while( true ) {
                            local_290 = &DAT_005967b8;
                            local_294 = (DAT_005967bc == 0x0);
                            local_298 = local_294;
                            if (local_294 != 0x0) break;
                            for (local_29c = (&DAT_005b8b44 + local_280 * 0x4); local_29c != 0x0;
                                local_29c = *local_29c) {
                                local_2a4 = local_29c + 0x8;
                                local_2a8 = local_2a4 & 0xffff0000 | local_2a4;
                                local_2a0 = local_2a4;
                                if (local_2a4 != local_280) break;
                                local_2b0 = local_29c + 0x8;
                                local_2b4 = *(local_29c + 0x3a) & 0x1;
                                local_2ac = local_2b0;
                                if ((local_2b4 == 0x0) && ((local_29c + 0x26) == (param_1 + 0x49))) {
                                    local_2b8 = *(param_1 + 0x46) >> 0x18;
                                    local_2c0 = local_29c + 0x8;
                                    local_2c4 = local_2c0 & 0xffff0000 | (local_29c + 0x9);
                                    local_2c8 = (local_29c + 0x9);
                                    local_2d0 = local_29c + 0x8;
                                    local_2d4 = local_2d0 & 0xffff0000 | (local_29c + 0x22);
                                    local_2cc = local_2d0;
                                    local_2bc = local_2c0;
                                    FUN_0040a70e(0x0,(local_29c + 0x22),local_2c8,local_2b8,0x1);
                                    FUN_004864f7();
                                    break;
                                }
                            }
                            if (local_29c == 0x0) {
                                return;
                            }
                            local_2dc = local_29c + 0x8;
                            local_2e0 = local_2dc & 0xffff0000 | local_2dc;
                            if (local_2dc != local_280) {
                                return;
                            }
                            local_2d8 = local_2dc;
                            if (DAT_005967bc != 0x0) {
                                FUN_0041c01d(DAT_005967bc);
                            }
                        }
                    }
                }
                else {
                    local_2e4 = *(param_1 + 0x41);
                    local_2ec = param_1 + 0x8;
                    local_2f0 = local_2ec & 0xffff0000 | local_2ec;
                    local_2e8 = local_2ec;
                    if (local_2ec == local_2e4) {
                        FUN_00413546(&DAT_005967b8);
                    }
                    else {
                        FUN_00424e53(DAT_005967bc,local_2e4);
                        (param_1 + 0x41) = local_2e4;
                        local_2f8 = param_1 + 0x8;
                        local_2fc = local_2f8 & 0xffff0000 | local_2f8;
                        if (local_2f8 == local_2e4) {
                            local_2f4 = local_2f8;
                            FUN_00413546(&DAT_005967b8);
                        }
                    }
                }
            }
            else {
                if (local_14 < 0xe) {
                    puVar4 = FUN_004823c9((param_1 + 0x8),(param_1 + 0x22),
                                          (param_1 + 0x9),&local_300);
                    if (puVar4 == 0x0) {
                        FUN_00413546(&DAT_005967b8);
                    }
                    else {
                        (param_1 + 0x41) = (puVar4 + 0x2);
                        (param_1 + 0x42) = (puVar4 + 0x1);
                        FUN_004248db();
                        if (((param_1 + 0x22) == (puVar4 + 0x2)) &&
                            ((param_1 + 0x9) == (puVar4 + 0x1))) {
                            FUN_00413546(&DAT_005967b8);
                        }
                    }
                }
                else {
                    if (local_14 < 0x10) {
                        if (local_14 == 0xf) {
                            local_64 = *(param_1 + 0x41);
                            local_60 = *(param_1 + 0x42);
                            local_6c = (DAT_005967bc + 0x20);
                            local_70 = local_6c & 0xffff0000 | *local_6c;
                            local_68 = local_6c;
                            local_5c = local_60;
                            local_58 = local_64;
                            local_74 = FUN_0041dd5a(*local_6c,local_64,local_60);
                            if (local_74 == 0x0) {
                                FUN_00413546(&DAT_005967b8);
                            }
                            else {
                                local_78 = *(local_74 + 0xe) >> 0x10;
                                FUN_004248db();
                                if (((local_74 + 0x4) == 0x6) && (DAT_005967bc != 0x0)) {
                                    if (local_78 < 0x5) {
                                        FUN_00499050(DAT_0059679c,local_78 + 0x414);
                                        FUN_00499050(DAT_0059679c,0x713e);
                                        pcVar3 = FUN_00499050(DAT_0059679c,0x73d4);
                                        FUN_0049c2e0(local_26c,pcVar3);
                                        FUN_0045518a(0x1f,0xffffffff,0x73d4,*(local_74 + 0x6) >> 0x10,local_26c,0xffffffff,0x0);
                                    }
                                    FUN_0040c88e(local_74);
                                    FUN_00413546(&DAT_005967b8);
                                }
                            }
                        }
                    }
                    else {
                        if (local_14 < 0x11) {
                            FUN_00423961(param_1);
                            FUN_004239de(param_1);
                            local_270 = *(param_1 + 0x41);
                            FUN_00424e53(DAT_005967bc,local_270);
                            (param_1 + 0x41) = local_270;
                        }
                        else {
                            if ((((local_14 == 0x11) && ((param_1 + 0x12) = 0x0, 0x0 < param_1[0xb] >> 0x18)) &&
                                (iVar5 = FUN_0044ace5((param_1 + 0x8),*(param_1 + 0x41),
                                                      *(param_1 + 0x42),0x1),
                                 *(param_1 + 0x23) >> 0x18 != iVar5)) &&
                                (iVar6 = FUN_00410fb3(&DAT_005967b8,0x2), iVar6 != 0x0)) {
                                FUN_0040a70e(0x2,*(param_1 + 0x41),*(param_1 + 0x42),iVar5,0x1);
                                FUN_004864f7();
                                FUN_0040c15d(*(param_1 + 0x41),*(param_1 + 0x42),iVar5,0x0);
                                if (DAT_005967c4 != 0x0) {
                                    FUN_00431d5a(&DAT_005967b8,&DAT_005967c0);
                                }
                                FUN_004864f7();
                                FUN_00435409(&DAT_005967b8);
                            }
                        }
                    }
                }
            }
        }
    }
    return;
}



fn FUN_00414315(param_1: i32)

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let local_14: *mut u32;

    if ((param_1 + 0xe) == 0x4) {
        *(param_1 + 0x32) = 0x3e8;
    }
    else {
        *(param_1 + 0x32) = 0xffe2;
        iVar3 = *(param_1 + 0x6) >> 0x10;
        iVar1 = *(param_1 + 0x8);
        iVar2 = *(param_1 + 0xa);
        for (local_14 = (&DAT_005b8b44 + iVar3 * 0x4);
            (local_14 != 0x0 && ((local_14 + 0x8) == iVar3)); local_14 = *local_14) {
            if (((*(local_14 + 0x23) >> 0x18 != DAT_004c9754) &&
                (((*(local_14 + 0x23) >> 0x18 < 0x5 || ((local_14 + 0x26) == '\r')) &&
                    ((*(local_14 + 0x3a) & 0x1) != 0x0)))) &&
                ((*(*(&DAT_00582938 +
                    (*(local_14 + 0x25) >> 0x18) * 0x4 + (local_14[0x9] >> 0x18) * 0x18) +
                    0x121) != 0x0 &&
                    (iVar4 = FUN_0044a87f(iVar1 >> 0x10,iVar2 >> 0x10,(local_14 + 0x22),
                                          (local_14 + 0x9)), iVar4 < 0xc)))) {
                if (iVar4 < 0x6) {
                    (param_1 + 0x32) = (param_1 + 0x32) + -0x3;
                }
                else {
                    (param_1 + 0x32) = (param_1 + 0x32) + -0x2;
                }
            }
        }
    }
    return;
}



fn FUN_004144a5(param_1: i32,param_2: i32)

{
    let local_2c: u8;
    let local_28: u8;
    let local_24: u8;
    let local_20: u8;
    let local_1c: u8;
    let local_18: u8;
    let local_14: u8;

    if (param_2 == 0x0) {
        local_2c = 0x0;
    }
    else {
        local_2c = 0xa;
    }
    (param_1 + 0x53) = local_2c;
    if (param_2 == 0x0) {
        local_28 = 0x0;
    }
    else {
        local_28 = 0xa;
    }
    (param_1 + 0x54) = local_28;
    if (param_2 == 0x0) {
        local_24 = 0x0;
    }
    else {
        local_24 = 0xa;
    }
    (param_1 + 0x55) = local_24;
    if (param_2 == 0x0) {
        local_20 = 0x0;
    }
    else {
        local_20 = 0xa;
    }
    (param_1 + 0x56) = local_20;
    if (param_2 == 0x0) {
        local_1c = 0x0;
    }
    else {
        local_1c = 0xa;
    }
    (param_1 + 0x57) = local_1c;
    if (param_2 == 0x0) {
        local_18 = 0x0;
    }
    else {
        local_18 = 0xa;
    }
    (param_1 + 0x38) = local_18;
    if (param_2 == 0x0) {
        local_14 = 0x0;
    }
    else {
        local_14 = 0xa;
    }
    (param_1 + 0x39) = local_14;
    return;
}



fn FUN_00414592(param_1: i32,param_2: i32)

{
    let local_18: u8;
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0x5c; local_14 = local_14 + 0x1) {
        if (((*(&DAT_00582938 + local_14 * 0x18) != 0x0) &&
            (*(*(&DAT_00582938 + local_14 * 0x18) + 0x121) != 0x0)) && (local_14 != 0x35)) {
            if (param_2 == 0x0) {
                local_18 = 0x0;
            }
            else {
                local_18 = *(*(&DAT_00582938 + local_14 * 0x18) + 0x121);
            }
            (param_1 + local_14) = local_18;
        }
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00414614(param_1: u32,param_2: i32,param_3: i32,param_4: i32,param_5: i32)

{
    let sVar1: i16;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut local_1f0: i32;
    let local_144: *mut u32;
    i32 **local_100;
    let mut local_fc: u32;
    let local_f8: *mut i32;;
    let local_f4: *mut i32;;
    let mut local_f0: u32;
    let local_ec: *mut i32;;
    let local_e8: *mut i32;;
    let mut local_e4: u32;
    let local_e0: *mut i32;;
    let local_dc: *mut i32;;
    let mut local_d8: u32;
    let local_d4: *mut i32;;
    let local_d0: *mut i32;;
    let mut local_cc: u32;
    let local_c8: *mut i32;;
    let local_c4: *mut i32;;
    let mut local_c0: u32;
    let local_bc: *mut i32;;
    let local_b8: *mut i32;;
    let mut local_b4: u32;
    let local_b0: *mut i32;;
    let local_ac: *mut i32;;
    let mut local_a8: u32;
    let local_a4: *mut i32;;
    let local_a0: *mut i32;;
    let mut local_9c: u32;
    let local_98: *mut i32;;
    let local_94: *mut i32;;
    let mut local_90: u32;
    let local_8c: *mut i32;;
    let local_88: *mut i32;;
    let mut local_84: u32;
    let local_80: *mut i32;;
    let local_7c: *mut i32;;
    let mut local_78: u32;
    let local_74: *mut i32;;
    let local_70: *mut i32;;
    let mut local_6c: u32;
    let local_68: *mut i32;;
    let local_64: *mut i32;;
    let mut local_60: u32;
    let local_5c: *mut i32;;
    let local_58: *mut i32;;
    let mut local_54: u32;
    let local_50: *mut i32;;
    let local_4c: *mut i32;;
    let mut local_48: u32;
    let local_44: *mut i32;;
    let local_40: *mut i32;;
    let mut local_3c: u32;
    let local_38: *mut u32;
    i32 **local_34;
    let mut local_30: i32;
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: u32;
    let mut local_1c: u32;
    let mut local_18: u32;

    local_30 = DAT_004c9754;
    if ((param_1 & 0x1) == 0x0) {
        for (local_38 = *DAT_005967c8; local_38 != 0x0; local_38 = *local_38) {
            if ((local_38 + 0xe) == 0x4) {
                *(local_38 + 0x32) = 0x3e8;
            }
            else {
                *(local_38 + 0x32) = 0x0;
            }
        }
    }
    else {
        _DAT_004c71a8 = *DAT_005967b0;
        for (local_38 = *DAT_005967c8; local_38 != 0x0; local_38 = *local_38) {
            if ((*(local_38 + 0xe) >> 0x10 == local_30) && (iVar2 = FUN_00481a44(local_38), iVar2 == 0x0)) {
                FUN_00414315(local_38);
            }
            else {
                if ((local_38 + 0xe) == 0x4) {
                    *(local_38 + 0x32) = 0x3e8;
                }
                else {
                    *(local_38 + 0x32) = 0x0;
                }
            }
        }
    }
    local_2c = -0x1;
    local_28 = -0x1;
    local_24 = -0x1;
    local_20 = 0xffffffff;
    local_1c = 0xffffffff;
    local_18 = 0xffffffff;
    local_3c = 0xffffffff;
    for (local_38 = *DAT_005967c8; local_38 != 0x0; local_38 = *local_38) {
        *(local_38 + 0xd) = *(local_38 + 0x32);
    }
    if (param_3 != 0x0) {
        for (local_34 = *DAT_005967b0; local_34 != 0x0; local_34 = *local_34) {
            if ((*(local_34 + 0x23) >> 0x18 == local_30) &&
                (((local_34[0x9] >> 0x18) + param_3) != '\0')) {
                local_44 = (local_34 + 0x8);
                local_48 = *(local_34 + 0x3a) & 0x1;
                local_40 = local_44;
                if (local_48 != 0x0) {
                    local_50 = (local_34 + 0x8);
                    local_54 = *(local_34 + 0x3a) & 0x40;
                    local_4c = local_50;
                    if ((local_54 == 0x0) && (((local_34 + 0x12) == '\x04' || ((local_34 + 0x12) == '\0')))) {
                        local_5c = (local_34 + 0x8);
                        local_60 = local_5c & 0xffff0000 | (local_34 + 0x22);
                        local_58 = local_5c;
                        if ((local_34 + 0x22) == local_2c) {
                            local_68 = (local_34 + 0x8);
                            local_6c = local_68 & 0xffff0000 | (local_34 + 0x9);
                            local_64 = local_68;
                            if ((local_34 + 0x9) != local_28)^ // goto LAB_00414855;
                            local_74 = (local_34 + 0x8);
                            local_78 = local_74 & 0xffff0000 | local_74;
                            local_70 = local_74;
                            if (local_74 != local_24)^ // goto LAB_00414855;
                        }
                        else {
                            LAB_00414855:
                                local_80 = (local_34 + 0x8);
                            local_84 = local_80 & 0xffff0000 | (local_34 + 0x22);
                            local_2c = (local_34 + 0x22);
                            local_8c = (local_34 + 0x8);
                            local_90 = local_8c & 0xffff0000 | (local_34 + 0x9);
                            local_28 = (local_34 + 0x9);
                            local_98 = (local_34 + 0x8);
                            local_9c = local_98 & 0xffff0000 | local_98;
                            local_24 = local_98;
                            local_94 = local_98;
                            local_88 = local_8c;
                            local_7c = local_80;
                            local_38 = FUN_00481784(local_24,local_2c,local_28);
                        }
                        if (local_38 != 0x0) {
                            (local_38 + 0x32) =
                                (local_38 + 0x32) + (ushort)*((local_34[0x9] >> 0x18) + param_3);
                        }
                    }
                }
            }
        }
    }
    if (param_2 != 0x0) {
        for (local_34 = *DAT_005967b0; local_34 != 0x0; local_34 = *local_34) {
            if ((*(local_34 + 0x23) >> 0x18 == local_30) &&
                (((local_34[0x9] >> 0x18) + param_2) != '\0')) {
                local_a4 = (local_34 + 0x8);
                local_a8 = *(local_34 + 0x3a) & 0x1;
                local_a0 = local_a4;
                if (local_a8 != 0x0) {
                    local_b0 = (local_34 + 0x8);
                    local_b4 = *(local_34 + 0x3a) & 0x40;
                    local_ac = local_b0;
                    if ((local_b4 == 0x0) && ((local_34 + 0x12) == '\0')) {
                        local_bc = (local_34 + 0x8);
                        local_c0 = local_bc & 0xffff0000 | (local_34 + 0x22);
                        local_b8 = local_bc;
                        if ((local_34 + 0x22) == local_2c) {
                            local_c8 = (local_34 + 0x8);
                            local_cc = local_c8 & 0xffff0000 | (local_34 + 0x9);
                            local_c4 = local_c8;
                            if ((local_34 + 0x9) != local_28)^ // goto LAB_00414a78;
                            local_d4 = (local_34 + 0x8);
                            local_d8 = local_d4 & 0xffff0000 | local_d4;
                            local_d0 = local_d4;
                            if (local_d4 != local_24)^ // goto LAB_00414a78;
                        }
                        else {
                            LAB_00414a78:
                                local_e0 = (local_34 + 0x8);
                            local_e4 = local_e0 & 0xffff0000 | (local_34 + 0x22);
                            local_2c = (local_34 + 0x22);
                            local_ec = (local_34 + 0x8);
                            local_f0 = local_ec & 0xffff0000 | (local_34 + 0x9);
                            local_28 = (local_34 + 0x9);
                            local_f8 = (local_34 + 0x8);
                            local_fc = local_f8 & 0xffff0000 | local_f8;
                            local_24 = local_f8;
                            local_f4 = local_f8;
                            local_e8 = local_ec;
                            local_dc = local_e0;
                            local_38 = FUN_00481784(local_24,local_2c,local_28);
                        }
                        if ((local_38 != 0x0) &&
                            (local_38[0xc] >> 0x10 <
                                *(&DAT_004bdf1c + param_4 * 0x4 + (local_38[0x3] >> 0x10) * 0x3c))) {
                            (local_38 + 0x32) =
                                (local_38 + 0x32) + (ushort)*((local_34[0x9] >> 0x18) + param_2);
                            (local_34 + 0x12) = 0x4;
                        }
                    }
                }
            }
        }
        FUN_004840cd(&local_100,&local_34,-0x1);
        while (local_34 != 0x0) {
            if ((((*(local_34 + 0x23) >> 0x18 == local_30) &&
                (((local_34[0x9] >> 0x18) + param_2) != '\0')) &&
                ((*(local_34 + 0x3a) & 0x1) != 0x0)) &&
                (((*(local_34 + 0x3a) & 0x40) == 0x0 && ((local_34 + 0x12) == '\0')))) {
                iVar2 = *(*(&DAT_00582938 +
                    (*(local_34 + 0x25) >> 0x18) * 0x4 + (local_34[0x9] >> 0x18) * 0x18) +
                    0x45);
                FUN_00431d0a(&DAT_005967b8);
                DAT_005967bc = local_34;
                local_144 = 0x0;
                sVar1 = (local_34 + 0x8);
                local_38 = (&DAT_005b89f8 + sVar1 * 0x4);
                while( true ) {
                    if ((local_38 == 0x0) || (*(local_38 + 0x6) >> 0x10 != sVar1))^
                    // goto LAB_004150bf;
                    if ((*(local_38 + 0xe) >> 0x10 == local_30) &&
                        ((((iVar2 < 0x6 || ((local_38 + 0xd) == (local_38 + 0x32))) &&
                            (local_38[0xc] >> 0x10 <
                                *(&DAT_004bd79c + param_4 * 0x4 + (local_38[0x3] >> 0x10) * 0x3c))) &&
                            (iVar3 = FUN_0044a87f(local_38[0x2] >> 0x10,*(local_38 + 0xa) >> 0x10,
                                                  (local_34 + 0x22),(local_34 + 0x9)),
                             iVar3 < param_5)))) break;
                    local_38 = *local_38;
                }
                FUN_0045af67(&DAT_005967b8,(local_34 + 0x22),(local_34 + 0x9),
                             (local_34 + 0x22),(local_34 + 0x9),0x6);
                local_144 = FUN_00418efd(local_34,param_4);
                if (local_144 == 0x0) {
                    FUN_0045af67(&DAT_005967b8,(local_34 + 0x22),(local_34 + 0x9),
                                 (local_34 + 0x22),(local_34 + 0x9),param_5);
                    local_144 = FUN_00418efd(local_34,param_4);
                }
                LAB_004150bf:
                if ((local_144 == 0x0) || (param_5 < local_1f0)) {
                    if ((param_1 & 0x2) == 0x0) {
                        if (*(*(&DAT_00582938 +
                            (*(local_34 + 0x25) >> 0x18) * 0x4 + (local_34[0x9] >> 0x18) * 0x18) +
                            0xa9) != 0x0) {
                            (local_34 + 0x12) = 0x2;
                        }
                    }
                    else {
                        (local_34 + 0x12) = 0x3;
                    }
                }
                else {
                    (local_34 + 0x12) = 0x6;
                    (local_34 + 0x41) = (local_144 + 0xa);
                    (local_34 + 0x42) = (local_144 + 0x3);
                    (local_144 + 0x32) =
                        (local_144[0xc] >> 0x10) +
                            ((*((local_34[0x9] >> 0x18) + param_2) << 0x2) / 0x5);
                    FUN_0041361b(local_34);
                }
            }
            local_34 = local_100;
            if (local_100 != 0x0) {
                local_100 = *local_100;
            }
        }
        FUN_0048418d(&local_100);
    }
    return;
}



fn FUN_0041519f()

{
    let puVar1: *mut u32;
    let local_34: *mut u32;
    let mut local_28: u32;
    let local_24: *mut u32;
    let local_20: *mut u32;
    let local_1c: *mut u32;
    let local_18: *mut u32;

    FUN_00431d31(&local_28);
    local_18 = FUN_004840cd(&local_1c,&local_20,-0x1);
    while (local_20 != 0x0) {
        if (((local_20[0x3] == 0x0) && (local_20[0x2] != 0x0)) && (*(local_20 + 0x23) >> 0x18 == DAT_004c9754))
        {
            local_24 = local_20;
            local_34 = local_20;
            puVar1 = local_34;
            while (local_34 = puVar1, local_34 != 0x0) {
                puVar1 = local_34[0x2];
                if ((*(local_34 + 0x3a) & 0x40) == 0x0) {
                    FUN_00431dec(&local_28,local_34);
                }
            }
        }
        local_20 = local_1c;
        if (local_1c != 0x0) {
            local_1c = *local_1c;
        }
    }
    FUN_0048418d(&local_1c);
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0041529f(param_1: i32) -> *mut u32

{
    let sVar1: i16;
    let sVar2: i16;
    let sVar3: i16;
    let puVar4: *mut u32;
    let mut bVar5: bool;
    let puVar6: *mut u32;
    i32 aiStackY588 [0x58];
    let mut local_98: i32;
    i32 local_4c [0x5];
    let mut local_38: i32;
    let local_34: *mut u32;
    let local_2c: *mut u32;
    let mut local_1c: i32;
    let mut local_18: i32;

    FUN_004a0430(local_4c,0x0,0x10);
    for (local_38 = 0x0; local_38 < _DAT_004c6160; local_38 = local_38 + 0x1) {
        local_4c[*(&DAT_004c6168 + local_38 * 0x4) >> 0x18] =
            local_4c[*(&DAT_004c6168 + local_38 * 0x4) >> 0x18] + 0x1;
    }
    local_34 = 0x0;
    sVar2 = -0x1;
    sVar3 = -0x1;
    local_1c = 0x0;
    sVar1 = (param_1 + 0x20);
    puVar4 = (&DAT_005b8b44 + sVar1 * 0x4);
    LAB_0041533e:
    loop {
    loop {
        local_2c = puVar4;
        puVar6 = local_2c;
        if ((local_2c == 0x0) || ((local_2c + 0x8) != sVar1)) {
            return local_34;
        }
        puVar4 = *local_2c;
    } while ((*(local_2c + 0x23) >> 0x18 != DAT_004c9754) || ((local_2c + 0x12) != '\x02'));
    if (((local_2c + 0x35) == -0x1) ||
        ((local_2c + 0x26) == (local_2c + 0x35))) {
        bVar5 = false;
    }
    else {
        bVar5 = true;
    }
    if (!bVar5)^ // goto LAB_0041544a;
    if (((param_1 + 0x35) == -0x1) || ((param_1 + 0x26) == (param_1 + 0x35))) {
        bVar5 = false;
    }
    else {
        bVar5 = true;
    }
} while ((!bVar5) || ((param_1 + 0x35) != (local_2c + 0x35)));^
    // goto LAB_004154bb;
    LAB_0041544a:
    if (((param_1 + 0x35) == -0x1) || ((param_1 + 0x26) == (param_1 + 0x35))) {
        bVar5 = false;
    }
    else {
        bVar5 = true;
    }
    if (!bVar5) {
        LAB_004154bb:
        if ((local_2c + 0x4a) == -0x1) {
            local_98 = 0x0;
            while ((local_98 < _DAT_004c6160 &&
                (local_2c[0x9] >> 0x18 !=
                    *(&DAT_004bd0ac + (*(&DAT_004c6168 + local_98 * 0x4) >> 0x18) * 0x4)))) {
                local_98 = local_98 + 0x1;
            }
            if (_DAT_004c6160 <= local_98) {
                (local_2c + 0x12) = 0x4;^
                // goto LAB_0041533e;
            }
        }
        if (((local_2c + 0x22) != sVar2) || ((local_2c + 0x9) != sVar3)) {
            sVar2 = (local_2c + 0x22);
            sVar3 = (local_2c + 0x9);
            local_18 = 0x0;
            for (; ((local_2c != 0x0 && ((local_2c + 0x22) == sVar2)) &&
            ((local_2c + 0x9) == sVar3)); local_2c = *local_2c) {
                local_18 = local_18 + (0x64 - (local_2c[0x9] >> 0x18));
                if ((local_2c + 0x27) == '/') {
                    local_18 = local_18 + 0x64;
                }
            }
            if (local_1c < local_18) {
                local_1c = local_18;
                local_34 = puVar6;
            }
        }
    }^
    // goto LAB_0041533e;
}



fn FUN_004156e9(param_1: i32)

{
    let mut bVar1: bool;
    let puVar2: *mut u32;
    let mut uVar3: u32;
    let mut local_38: i32;
    let mut local_34: u32;
    let local_30: *mut u32;
    let local_2c: *mut u32;
    let local_28: *mut u32;
    u32 **local_24;
    let local_20: *mut u32;
    let local_1c: *mut u32;
    let local_18: *mut u32;

    local_18 = FUN_004840cd(&local_1c,&local_20,-0x1);
    loop {
    if (local_20 == 0x0) {
        FUN_0048418d(&local_1c);
        return;
    }
    if ((*(local_20 + 0x23) >> 0x18 == DAT_004c9754) && (*(local_20 + 0x45) >> 0x18 == param_1))
    {
        local_30 = local_20 + 0x8;
        local_34 = *(local_20 + 0x3a) & 0x1;
        local_2c = local_30;
        if ((local_34 != 0x0) &&
            (puVar2 = FUN_004823c9((local_20 + 0x8),(local_20 + 0x22),
                                   (local_20 + 0x9),&local_38), puVar2 != 0x0)) {
            if ((*(puVar2 + 0x25) & 0x1) == 0x0) {
                if (((local_20 + 0x35) == -0x1) ||
                    ((local_20 + 0x26) == (local_20 + 0x35))) {
                    bVar1 = false;
                }
                else {
                    bVar1 = true;
                }
                if (!bVar1)^ // goto LAB_004158bd;
            }
            else {
                if (((local_20 + 0x35) == -0x1) ||
                    ((local_20 + 0x26) == (local_20 + 0x35))) {
                    bVar1 = false;
                }
                else {
                    bVar1 = true;
                }
                if ((bVar1) && (*(local_20 + 0x32) >> 0x18 == puVar2[0x2] >> 0x10)) {
                    LAB_004158bd:
                        FUN_00431d0a(&DAT_005967b8);
                    DAT_005967bc = FUN_00434de1(local_20);
                    if (((local_20 + 0x22) != (puVar2 + 0x2)) &&
                        ((local_20 + 0x9) != (puVar2 + 0x1))) {
                        (local_20 + 0x41) = (puVar2 + 0x2);
                        (local_20 + 0x42) = (puVar2 + 0x1);
                        uVar3 = FUN_0045af67(&DAT_005967b8,(local_20 + 0x22),
                                             (local_20 + 0x9),*(local_20 + 0x41),
                                             *(local_20 + 0x42),0x0);
                        if (uVar3 != 0x0) {
                            FUN_004248db();
                        }
                    }
                }
            }
        }
    }
    local_20 = local_1c;
    local_24 = &local_1c;
    local_28 = local_1c;
    if (local_1c != 0x0) {
        local_1c = *local_1c;
    }
} while( true );
}



fn FUN_00415a2e()

{
    return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00415a42()

{
    let mut iVar1: i32;
    let mut local_1b8: i32;
    let mut local_1b4: i32;
    let local_150: *mut u32;
    let mut local_40: i32;
    let mut local_34: u32;
    let mut local_30: i32;
    let local_2c: *mut u32;
    let local_28: *mut u32;
    let local_24: *mut u32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let local_18: *mut u32;

    FUN_00431d31(&local_34);
    local_18 = FUN_004840cd(&local_2c,&local_24,-0x1);
    loop {
    if (local_24 == 0x0) {
        FUN_0048418d(&local_2c);
        return;
    }
    if ((*(local_24 + 0x23) >> 0x18 == DAT_004c9754) &&
        (((((local_24 + 0x2a) == 0x9 || ((local_24 + 0x2a) == 0x7)) &&
            ((local_24 + 0x2f) != '\0')) &&
            (((local_24 + 0x12) == '\0' &&
                (local_20 = *(*(&DAT_00582938 +
                    (*(local_24 + 0x25) >> 0x18) * 0x4 +
                    (local_24[0x9] >> 0x18) * 0x18) + 0xa5), local_20 != 0x0)))))) {
        local_1c = local_20;
        for (local_40 = 0x0; local_40 < local_20; local_40 = local_40 + 0x1) {
            if (local_24[local_40 + 0x4] != 0x0) {
                local_1c = local_1c + -0x1;
            }
        }
        if (local_1c != 0x0) {
            local_28 = FUN_0041529f(local_24);
            FUN_00431d0a(&local_34);
            local_30 = FUN_00434de1(local_24);
            if (local_28 == 0x0) {
                if ((*(local_30 + 0x3a) & 0x1) != 0x0) {
                    FUN_00419085();
                    FUN_0042feac();
                    FUN_004579ea(&local_34);
                    if ((local_24 + 0x2f) == '\0') {
                        (local_24 + 0x12) = 0x4;
                    }
                }
            }
            else {
                FUN_00419085();
                FUN_0042feac();
                if (((*(local_30 + 0x3a) & 0x1) != 0x0) &&
                    (((local_24 + 0x22) != (local_28 + 0x22) ||
                        ((local_24 + 0x9) != (local_28 + 0x9))))) {
                    FUN_00419085();
                    FUN_0042feac();
                    iVar1 = FUN_004579ea(&local_34);
                    if (iVar1 == 0x0) {
                        FUN_00418d7a((local_30 + 0x20));
                        iVar1 = FUN_004579ea(&local_34);
                        if (iVar1 == 0x0) {
                            FUN_00419085();
                            FUN_0042feac();
                            (local_24 + 0x12) = 0x4;^
                            // goto LAB_00415a71;
                        }
                    }
                }
                if (((*(local_30 + 0x3a) & 0x1) == 0x0) &&
                    ((0x1 < *(local_30 + 0x2c) >> 0x18 ||
                        (iVar1 = FUN_00485ea2((local_28 + 0x8),(local_28 + 0x22),
                                              (local_28 + 0x9),0x1), iVar1 < 0x14)))) {
                    FUN_00419085();
                    FUN_0042feac();
                    FUN_00431d5a(&DAT_005967b8,&local_34);
                    iVar1 = FUN_00457f10(&DAT_005967b8,(local_28 + 0x8),
                                         (local_28 + 0x22),(local_28 + 0x9),0x0);
                    if (iVar1 == 0x0) {
                        FUN_00419085();
                        FUN_0042feac();
                    }
                    else {
                        if (DAT_005967bc != 0x0) {
                            FUN_00431d5a(&local_34,&DAT_005967b8);^
                            // goto LAB_00416029;
                        }
                    }
                }
                else {
                    LAB_00416029:
                    if ((*(local_30 + 0x3a) & 0x1) == 0x0) {
                        FUN_00419085();
                        FUN_0042feac();
                    }
                    else {
                        FUN_00419085();
                        FUN_0042feac();
                        for (local_150 = local_28;
                            (((local_150 != 0x0 && (local_1c != 0x0)) &&
                            (*(local_150 + 0x23) >> 0x18 == DAT_004c9754)) &&
                            ((((local_28 + 0x8) == (local_150 + 0x8) &&
                            ((local_28 + 0x22) == (local_150 + 0x22))) &&
                            ((local_28 + 0x9) == (local_150 + 0x9))))); local_150 = *local_150)
                        {
                            if ((local_150 + 0x12) == '\x02') {
                                if ((*(local_150 + 0x3a) & 0x40) == 0x0) {
                                    iVar1 = FUN_004330dc(&local_34,local_150);
                                    if (iVar1 != 0x0) {
                                        local_1c = local_1c + -0x1;
                                        (local_150 + 0x12) = 0x4;
                                    }
                                }
                                else {
                                    (local_150 + 0x12) = 0x4;
                                }
                            }
                        }
                        if ((local_24[0x4] != 0x0) && ((local_24[0x4] + 0x4a) == -0x1)) {
                            local_1b4 = 0x0;
                            while ((local_1b4 < _DAT_004c6160 &&
                                (*(local_24[0x4] + 0x24) >> 0x18 !=
                                    *(&DAT_004bd0ac + (*(&DAT_004c6168 + local_1b4 * 0x4) >> 0x18) * 0x4)))) {
                                local_1b4 = local_1b4 + 0x1;
                            }
                            (local_24[0x4] + 0x49) = (&DAT_004c616a)[local_1b4 * 0x4];
                            (local_24[0x4] + 0x4a) = (ushort)(byte)local_1b4;
                            if ((&DAT_004c616b)[local_1b4 * 0x4] == '\x02') {
                                (&DAT_004c616b)[local_1b4 * 0x4] = 0x3;
                            }
                            for (local_1b8 = 0x1; local_1b8 < 0x4; local_1b8 = local_1b8 + 0x1) {
                                if ((local_24[local_1b8 + 0x4] != 0x0) && ((local_24[local_1b8 + 0x4] + 0x4a) == -0x1)) {
                                    (local_24[local_1b8 + 0x4] + 0x49) = (&DAT_004c616a)[local_1b4 * 0x4];
                                    (local_24[local_1b8 + 0x4] + 0x4a) = (ushort)(byte)local_1b4;
                                }
                            }
                        }
                        if (0x1 < *(local_30 + 0x2c) >> 0x18) {
                            FUN_004579ea(&local_34);
                        }
                        if ((local_24 + 0x2f) == '\0') {
                            (local_24 + 0x12) = 0x4;
                        }
                    }
                }
            }
        }
    }
    LAB_00415a71:
        local_24 = local_2c;
    if (local_2c != 0x0) {
        local_2c = *local_2c;
    }
} while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0047bf4b(param_1: u32,param_2: u32,param_3: u32) -> i32

{
let bVar1: u8;
let cVar2: u8;
let mut pcVar3: String;
let mut uVar4: u32;
undefined3 extraout_var;
let mut iVar5: i32;
byte *pbVar6;
byte *pbVar7;
let mut local_1fc: u32;
let mut local_1f8: u32;
let mut local_1f4: u32;
let mut local_1f0: u32;
let mut local_1ec: u32;
let mut local_1e8: u32;
let mut local_1e4: u32;
let mut local_1e0: u32;
let mut local_1dc: u32;
let mut local_1d8: u32;
let mut local_1d4: u32;
let mut local_1d0: u32;
let mut local_1c0: u32;
let mut local_1bc: u32;
let mut local_1b8: u32;
let mut local_1b4: u32;
let mut local_1b0: u32;
let mut local_1ac: u32;
let mut local_1a8: u32;
let mut local_1a4: u32;
let mut local_1a0: u32;
let mut local_19c: u32;
let mut local_198: u32;
let mut local_194: u32;
let mut local_190: u32;
let mut local_18c: u32;
let mut local_188: u32;
let mut local_184: u32;
let mut local_180: u32;
let mut local_17c: u32;
let mut local_178: u32;
let mut local_174: u32;
let mut local_170: u32;
let mut local_16c: u32;
let mut local_168: u32;
let mut local_164: u32;
let mut local_160: u32;
let mut local_15c: u32;
let mut local_158: u32;
let mut local_154: u32;
let mut local_150: u32;
let mut local_14c: u32;
let mut local_148: u32;
let mut local_144: u32;
let mut local_140: u32;
let mut local_13c: u32;
let mut local_138: u32;
let mut local_134: u32;
let mut local_130: u32;
let mut local_12c: u32;
let mut local_128: u32;
let mut local_124: u32;
let mut local_120: u32;
let mut local_11c: u32;
let mut local_118: u32;
let mut local_114: u32;
let mut local_110: u32;
let mut local_10c: u32;
let mut local_108: u32;
let mut local_104: u32;
let mut local_100: u32;
let mut local_fc: u32;
let mut local_f8: u32;
let mut local_f4: u32;
let mut local_f0: u32;
let mut local_ec: u32;
let mut local_e8: u32;
let mut local_e4: u32;
let mut local_e0: u32;
let mut local_dc: u32;
let mut local_d8: u32;
let mut local_d4: u32;
let mut local_d0: u32;
let mut local_cc: u32;
let mut local_c8: u32;
let mut local_c4: u32;
let mut local_c0: u32;
let mut local_bc: u32;
let local_b8: u8 [0x80];
let mut local_38: i32;
let local_34: *mut i32;;
byte *local_30;
byte *local_2c;
let mut local_28: i32;
let mut local_24: i32;
let mut local_20: i32;
let local_1c: u32;
let local_18: *mut u32;
let mut local_14: u32;

local_30 = FUN_0049c2c9(0x104);
local_2c = FUN_0049c2c9(0x104);
local_24 = 0x0;
local_20 = 0xecf71;
FUN_0049c2e0(local_2c,s_sav__s_tmp_004c2527);
pcVar3 = FUN_0049c7b5(s_sav__s_sav_004c2532);
FUN_0049c2e0(local_30,pcVar3);
local_34 = FUN_0049c4bd(local_2c,&DAT_004c253d);
if (local_34 == 0x0) {
pcVar3 = FUN_00499050(DAT_0059679c,0x7420);
FUN_0049c2e0(local_b8,pcVar3);
FUN_0049d2e0(0x0,0x1,local_b8);
local_38 = 0x0;
}
else {
local_1c = FUN_0047d5ab(local_34);
if ((param_3 & 0x8) != 0x0) {
local_20 = local_20 + -0xea600;
}
uVar4 = FUN_004a7160(&local_20,0x4,0x1,local_34);
local_bc = (uVar4 == 0x0);
local_24 = local_24 + local_bc;
uVar4 = FUN_004a7160(&DAT_00591cb0,0x4,0x1,local_34);
local_c0 = (uVar4 == 0x0);
local_24 = local_24 + local_c0;
uVar4 = FUN_004a7160(&param_3,0x4,0x1,local_34);
local_c4 = (uVar4 == 0x0);
local_24 = local_24 + local_c4;
uVar4 = FUN_004a7160(&DAT_004c9740,0x4,0x1,local_34);
local_c8 = (uVar4 == 0x0);
local_24 = local_24 + local_c8;
for (local_28 = 0x0; local_28 < 0xb; local_28 = local_28 + 0x1) {
cVar2 = FUN_00460296(&DAT_004d7730 + local_28 * 0x67,local_34);
local_24 = local_24 + CONCAT31(extraout_var,cVar2);
}
for (local_28 = 0x0; local_28 < 0xe; local_28 = local_28 + 0x1) {
iVar5 = FUN_00461b77((&DAT_00568210 + local_28 * 0x1e22),local_34);
local_24 = local_24 + iVar5;
}
uVar4 = FUN_004a7160(&DAT_005827f0,0x134,0x1,local_34);
local_cc = (uVar4 == 0x0);
local_24 = local_24 + local_cc;
iVar5 = FUN_00462a5b(&DAT_004d55a8,local_34);
local_24 = local_24 + iVar5;
uVar4 = FUN_004a7160(&DAT_00591cac,0x4,0x1,local_34);
local_d0 = (uVar4 == 0x0);
local_24 = local_24 + local_d0;
uVar4 = FUN_004a7160(&DAT_00591cb0,0x4,0x1,local_34);
local_d4 = (uVar4 == 0x0);
local_24 = local_24 + local_d4;
uVar4 = FUN_004a7160(&DAT_00591cb4,0x4,0x1,local_34);
local_d8 = (uVar4 == 0x0);
local_24 = local_24 + local_d8;
uVar4 = FUN_004a7160(&DAT_004c9754,0x4,0x1,local_34);
local_dc = (uVar4 == 0x0);
local_24 = local_24 + local_dc;
uVar4 = FUN_004a7160(&DAT_004c9760,0x4,0x1,local_34);
local_e0 = (uVar4 == 0x0);
local_24 = local_24 + local_e0;
uVar4 = FUN_004a7160(&DAT_004c9764,0x4,0x1,local_34);
local_e4 = (uVar4 == 0x0);
local_24 = local_24 + local_e4;
uVar4 = FUN_004a7160(&DAT_004c9768,0x4,0x1,local_34);
local_e8 = (uVar4 == 0x0);
local_24 = local_24 + local_e8;
uVar4 = FUN_004a7160(&DAT_004c9778,0xbde0,0x1,local_34);
local_ec = (uVar4 == 0x0);
local_24 = local_24 + local_ec;
uVar4 = FUN_004a7160(&DAT_004d5560,0x4,0x1,local_34);
local_f0 = (uVar4 == 0x0);
local_24 = local_24 + local_f0;
uVar4 = FUN_004a7160(&DAT_004d5564,0x4,0x1,local_34);
local_f4 = (uVar4 == 0x0);
local_24 = local_24 + local_f4;
uVar4 = FUN_004a7160(&DAT_004d557c,0x4,0x1,local_34);
local_f8 = (uVar4 == 0x0);
local_24 = local_24 + local_f8;
uVar4 = FUN_004a7160(&DAT_004d5580,0x4,0x1,local_34);
local_fc = (uVar4 == 0x0);
local_24 = local_24 + local_fc;
uVar4 = FUN_004a7160(&DAT_004d5584,0x4,0x1,local_34);
local_100 = (uVar4 == 0x0);
local_24 = local_24 + local_100;
uVar4 = FUN_004a7160(&DAT_004d5588,0x4,0x1,local_34);
local_104 = (uVar4 == 0x0);
local_24 = local_24 + local_104;
uVar4 = FUN_004a7160(&DAT_004d558c,0x4,0x1,local_34);
local_108 = (uVar4 == 0x0);
local_24 = local_24 + local_108;
uVar4 = FUN_004a7160(&DAT_004d5590,0x4,0x1,local_34);
local_10c = (uVar4 == 0x0);
local_24 = local_24 + local_10c;
uVar4 = FUN_004a7160(&DAT_004d5594,0x4,0x1,local_34);
local_110 = (uVar4 == 0x0);
local_24 = local_24 + local_110;
uVar4 = FUN_004a7160(&DAT_00595738,0x4,0x1,local_34);
local_114 = (uVar4 == 0x0);
local_24 = local_24 + local_114;
uVar4 = FUN_004a7160(&DAT_004d5a58,0x1f4,0x1,local_34);
local_118 = (uVar4 == 0x0);
local_24 = local_24 + local_118;
uVar4 = FUN_004a7160(&DAT_004d5c50,0x1f4,0x1,local_34);
local_11c = (uVar4 == 0x0);
local_24 = local_24 + local_11c;
uVar4 = FUN_004a7160(&DAT_004d566c,0x3e8,0x1,local_34);
local_120 = (uVar4 == 0x0);
local_24 = local_24 + local_120;
uVar4 = FUN_004a7160(&DAT_004c976c,0x4,0x1,local_34);
local_124 = (uVar4 == 0x0);
local_24 = local_24 + local_124;
uVar4 = FUN_004a7160(&DAT_004c9770,0x4,0x1,local_34);
local_128 = (uVar4 == 0x0);
local_24 = local_24 + local_128;
uVar4 = FUN_004a7160(&DAT_004c9774,0x4,0x1,local_34);
local_12c = (uVar4 == 0x0);
local_24 = local_24 + local_12c;
uVar4 = FUN_004a7160(&DAT_004c5048,0x18,0x1,local_34);
local_130 = (uVar4 == 0x0);
local_24 = local_24 + local_130;
uVar4 = FUN_004a7160(&DAT_004d5568,0x14,0x1,local_34);
local_134 = (uVar4 == 0x0);
local_24 = local_24 + local_134;
uVar4 = FUN_004a7160(&DAT_00596a44,0x4,0x1,local_34);
local_138 = (uVar4 == 0x0);
local_24 = local_24 + local_138;
uVar4 = FUN_004a7160(&DAT_00596a50,0x4,0x1,local_34);
local_13c = (uVar4 == 0x0);
local_24 = local_24 + local_13c;
uVar4 = FUN_004a7160(&DAT_004c6160,0x4,0x1,local_34);
local_140 = (uVar4 == 0x0);
local_24 = local_24 + local_140;
for (local_28 = 0x0; local_28 < _DAT_004c6160; local_28 = local_28 + 0x1) {
uVar4 = FUN_004a7160((&DAT_004c6168 + local_28 * 0x4),0x4,0x1,local_34);
local_144 = (uVar4 == 0x0);
local_24 = local_24 + local_144;
}
uVar4 = FUN_004a7160(&DAT_005b4ac0,0x2580,0x1,local_34);
local_148 = (uVar4 == 0x0);
local_24 = local_24 + local_148;
for (local_28 = 0x0; local_28 < 0x28; local_28 = local_28 + 0x1) {
if (*(&DAT_005b70c2 + local_28 * 0x4e) != 0x0) {
uVar4 = FUN_004a7160((&DAT_005b7078 + local_28 * 0x4e),0x2,0x1,local_34);
local_14c = (uVar4 == 0x0);
local_24 = local_24 + local_14c;
uVar4 = FUN_004a7160((&DAT_005b707a + local_28 * 0x4e),0x2,0x1,local_34);
local_150 = (uVar4 == 0x0);
local_24 = local_24 + local_150;
uVar4 = FUN_004a7160((&DAT_005b707c + local_28 * 0x4e),0x2,0x1,local_34);
local_154 = (uVar4 == 0x0);
local_24 = local_24 + local_154;
uVar4 = FUN_004a7160((&DAT_005b707e + local_28 * 0x4e),0x20,0x1,local_34);
local_158 = (uVar4 == 0x0);
local_24 = local_24 + local_158;
uVar4 = FUN_004a7160((&DAT_005b709e + local_28 * 0x4e),0x20,0x1,local_34);
local_15c = (uVar4 == 0x0);
local_24 = local_24 + local_15c;
uVar4 = FUN_004a7160((local_28 * 0x4e + 0x5b70be),0x2,0x1,local_34);
local_160 = (uVar4 == 0x0);
local_24 = local_24 + local_160;
uVar4 = FUN_004a7160((local_28 * 0x4e + 0x5b70c0),0x2,0x1,local_34);
local_164 = (uVar4 == 0x0);
local_24 = local_24 + local_164;
uVar4 = FUN_004a7160((&DAT_005b70c2 + local_28 * 0x4e),0x4,0x1,local_34);
local_168 = (uVar4 == 0x0);
local_24 = local_24 + local_168;
uVar4 = FUN_004a7160((&DAT_004daab0 + local_28 * 0x3890),0x4,0x1,local_34);
local_16c = (uVar4 == 0x0);
local_24 = local_24 + local_16c;
uVar4 = FUN_004a7160((local_28 * 0x3890 + 0x4d7e00),0x2cb0,0x1,local_34);
local_170 = (uVar4 == 0x0);
local_24 = local_24 + local_170;
}
}
uVar4 = FUN_004a7160(&DAT_004c3fb0,0x2,0x1,local_34);
local_174 = (uVar4 == 0x0);
local_24 = local_24 + local_174;
for (local_28 = 0x0; local_28 < 0x78; local_28 = local_28 + 0x1) {
if (*(&DAT_005b7cb8 + local_28 * 0x14) != 0x0) {
local_178 = local_28 * 0x14 & 0xffff0000U | (&DAT_005b7ca8 + local_28 * 0x14);
uVar4 = FUN_004a7160(&local_178,0x2,0x1,local_34);
local_17c = (uVar4 == 0x0);
local_24 = local_24 + local_17c;
local_178 = local_28 * 0x14 & 0xffff0000U | (&DAT_005b7cac + local_28 * 0x14);
uVar4 = FUN_004a7160(&local_178,0x2,0x1,local_34);
local_180 = (uVar4 == 0x0);
local_24 = local_24 + local_180;
local_178 = local_28 * 0x14 & 0xffff0000U | (&DAT_005b7cb0 + local_28 * 0x14);
uVar4 = FUN_004a7160(&local_178,0x2,0x1,local_34);
local_184 = (uVar4 == 0x0);
local_24 = local_24 + local_184;
local_178 = local_28 * 0x14 & 0xffff0000U | (&DAT_005b7cb4 + local_28 * 0x14);
uVar4 = FUN_004a7160(&local_178,0x2,0x1,local_34);
local_188 = (uVar4 == 0x0);
local_24 = local_24 + local_188;
uVar4 = FUN_004a7160((&DAT_005b7cb8 + local_28 * 0x14),0x4,0x1,local_34);
local_18c = (uVar4 == 0x0);
local_24 = local_24 + local_18c;
}
}
uVar4 = FUN_004a7160(&DAT_004c3fb0,0x2,0x1,local_34);
local_190 = (uVar4 == 0x0);
local_24 = local_24 + local_190;
iVar5 = FUN_00457721(local_34);
local_24 = local_24 + iVar5;
iVar5 = FUN_004875c2(local_34);
local_24 = local_24 + iVar5;
for (local_18 = *DAT_005967c8; local_18 != 0x0; local_18 = *local_18) {
uVar4 = FUN_004a7160(local_18 + 0x2,0x2,0x1,local_34);
local_194 = (uVar4 == 0x0);
local_24 = local_24 + local_194;
uVar4 = FUN_004a7160((local_18 + 0xa),0x2,0x1,local_34);
local_198 = (uVar4 == 0x0);
local_24 = local_24 + local_198;
uVar4 = FUN_004a7160(local_18 + 0x3,0x2,0x1,local_34);
local_19c = (uVar4 == 0x0);
local_24 = local_24 + local_19c;
uVar4 = FUN_004a7160((local_18 + 0xe),0x2,0x1,local_34);
local_1a0 = (uVar4 == 0x0);
local_24 = local_24 + local_1a0;
uVar4 = FUN_004a7160(local_18 + 0x4,0x2,0x1,local_34);
local_1a4 = (uVar4 == 0x0);
local_24 = local_24 + local_1a4;
uVar4 = FUN_004a7160((local_18 + 0x12),0x2,0x1,local_34);
local_1a8 = (uVar4 == 0x0);
local_24 = local_24 + local_1a8;
uVar4 = FUN_004a7160(local_18 + 0x5,0x2,0x1,local_34);
local_1ac = (uVar4 == 0x0);
local_24 = local_24 + local_1ac;
uVar4 = FUN_004a7160((local_18 + 0x16),0x2,0x1,local_34);
local_1b0 = (uVar4 == 0x0);
local_24 = local_24 + local_1b0;
uVar4 = FUN_004a7160(local_18 + 0x6,0x2,0x1,local_34);
local_1b4 = (uVar4 == 0x0);
local_24 = local_24 + local_1b4;
uVar4 = FUN_004a7160((local_18 + 0x1a),0x2,0x1,local_34);
local_1b8 = (uVar4 == 0x0);
local_24 = local_24 + local_1b8;
uVar4 = FUN_004a7160(local_18 + 0x7,0x2,0x1,local_34);
local_1bc = (uVar4 == 0x0);
local_24 = local_24 + local_1bc;
uVar4 = FUN_004a7160((local_18 + 0x1e),0x2,0x1,local_34);
local_1c0 = (uVar4 == 0x0);
local_24 = local_24 + local_1c0;
local_14 = (local_18 + 0x2) & 0xffff0000 | (local_18 + 0x8);
uVar4 = FUN_004a7160(&local_14,0x2,0x1,local_34);
local_1d0 = (uVar4 == 0x0);
local_24 = local_24 + local_1d0;
uVar4 = FUN_004a7160((local_18 + 0x22),0x2,0x1,local_34);
local_1d4 = (uVar4 == 0x0);
local_24 = local_24 + local_1d4;
uVar4 = FUN_004a7160(local_18 + 0x9,0x2,0x1,local_34);
local_1d8 = (uVar4 == 0x0);
local_24 = local_24 + local_1d8;
uVar4 = FUN_004a7160((local_18 + 0x26),0x2,0x1,local_34);
local_1dc = (uVar4 == 0x0);
local_24 = local_24 + local_1dc;
uVar4 = FUN_004a7160(local_18 + 0xa,0x2,0x1,local_34);
local_1e0 = (uVar4 == 0x0);
local_24 = local_24 + local_1e0;
uVar4 = FUN_004a7160((local_18 + 0x2a),0x2,0x1,local_34);
local_1e4 = (uVar4 == 0x0);
local_24 = local_24 + local_1e4;
uVar4 = FUN_004a7160(local_18 + 0xb,0x4,0x1,local_34);
local_1e8 = (uVar4 == 0x0);
local_24 = local_24 + local_1e8;
uVar4 = FUN_004a7160(local_18 + 0xc,0x2,0x1,local_34);
local_1ec = (uVar4 == 0x0);
local_24 = local_24 + local_1ec;
}
uVar4 = FUN_004a7160(&DAT_004c3fb0,0x2,0x1,local_34);
local_1f0 = (uVar4 == 0x0);
local_24 = local_24 + local_1f0;
if (((byte)DAT_004d559c & 0x8) != 0x0) {
uVar4 = FUN_004a7160(&DAT_005b2e26,0x2,0x5,local_34);
local_1f4 = (uVar4 == 0x0);
local_24 = local_24 + local_1f4;
uVar4 = FUN_004a7160(&DAT_005b2e1c,0x2,0x5,local_34);
local_1f8 = (uVar4 == 0x0);
local_24 = local_24 + local_1f8;
uVar4 = FUN_004a7160(&DAT_005b2e30,0x2,0x1,local_34);
local_1fc = (uVar4 == 0x0);
local_24 = local_24 + local_1fc;
}
FUN_0049ca40(local_34);
pbVar6 = FUN_0049c7b5(local_2c);
pbVar7 = local_2c;
loop {
bVar1 = *pbVar6;
*pbVar7 = bVar1;
if (bVar1 == 0x0) break;
bVar1 = pbVar6[0x1];
pbVar6 = pbVar6 + 0x2;
pbVar7[0x1] = bVar1;
pbVar7 = pbVar7 + 0x2;
} while (bVar1 != 0x0);
FUN_00453a3c(local_2c,local_30);
FUN_004a5420(local_2c);
FUN_0049af50(local_2c);
FUN_0049af50(local_30);
local_38 = local_24;
}
return local_38;
}



fn FUN_0047d42c()

{
    byte *pbVar1;
    let local_28: *mut u32;
    let local_24: *mut u32;
    let mut local_20: u32;
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0x28; local_14 = local_14 + 0x1) {
        for (local_18 = 0x0; local_18 < 0x2c; local_18 = local_18 + 0x1) {
            local_1c = ((local_18 & 0x1) == 0x0);
            for (local_20 = local_1c; local_20 < (0x41 - (local_18 & 0x1)); local_20 = local_20 + 0x2) {
                pbVar1 = (*(&DAT_004d7d50 + local_18 * 0x4 + local_14 * 0x3890) + local_20 * 0x4 + 0x6);
                *pbVar1 = *pbVar1 & 0xfd;
                pbVar1 = (*(&DAT_004d7d50 + local_18 * 0x4 + local_14 * 0x3890) + local_20 * 0x4 + 0x6);
                *pbVar1 = *pbVar1 & 0xfb;
                pbVar1 = (*(&DAT_004d7d50 + local_18 * 0x4 + local_14 * 0x3890) + local_20 * 0x4 + 0x6);
                *pbVar1 = *pbVar1 & 0xf7;
                pbVar1 = (*(&DAT_004d7d50 + local_14 * 0x3890 + local_18 * 0x4) + local_20 * 0x4 + 0x6);
                *pbVar1 = *pbVar1 & 0xef;
                pbVar1 = (*(&DAT_004d7d50 + local_18 * 0x4 + local_14 * 0x3890) + local_20 * 0x4 + 0x6);
                *pbVar1 = *pbVar1 & 0xdf;
            }
        }
    }
    for (local_24 = *DAT_005967c8; local_24 != 0x0; local_24 = *local_24) {
        (local_24 + 0x2e) = (local_24 + 0x2e) & 0x1;
    }
    for (local_28 = *DAT_005967b0; local_28 != 0x0; local_28 = *local_28) {
        (local_28 + 0xf) = (local_28 + 0xf) & 0x1;
    }
    return;
}



fn FUN_0047d5ab(param_1: *mut i32) -> u32

{
    let DVar1: u32;
    let DVar2: u32;

    DVar1 = FUN_004aa690(param_1);
    FUN_004aa75c(param_1,0x0,0x2);
    DVar2 = FUN_004aa690(param_1);
    FUN_004aa75c(param_1,DVar1,0x0);
    return DVar2;
}



fn FUN_0047d608(param_1: i32,param_2: i32,param_3: i32)

{
    if (param_2 < 0x0) {
        *(param_1 + 0x49) = 0x0;
    }
    else {
        if (0x32 - *(param_1 + 0x75) < param_2) {
            *(param_1 + 0x49) = 0x32 - *(param_1 + 0x75);
        }
        else {
            *(param_1 + 0x49) = param_2;
        }
    }
    if (param_3 < 0x0) {
        *(param_1 + 0x4d) = 0x0;
    }
    else {
        if (0x30 - *(param_1 + 0x71) < param_3) {
            *(param_1 + 0x4d) = 0x30 - *(param_1 + 0x71);
        }
        else {
            *(param_1 + 0x4d) = param_3;
        }
    }
    return;
}



fn FUN_0047d6a0(param_1: i32,param_2: i32,param_3: i32,param_4: i32) -> u32

{
    let mut iVar1: i32;
    let mut iVar2: i32;

    if ((*(param_1 + 0x81) == 0x2) && ((*(param_1 + 0x2d) & 0x2) != 0x0)) {
        iVar1 = *(param_1 + 0x59);
        iVar2 = *(param_1 + 0x5d);
        FUN_0047e204(param_1,param_2,param_3);
        if ((iVar1 == *(param_1 + 0x59)) && (iVar2 == *(param_1 + 0x5d))) {
            return 0x0;
        }
        if (param_4 == 0x0) {
            return 0x0;
        }
        FUN_00471bcd(param_1);
    }
    else {
        iVar1 = *(param_1 + 0x49);
        iVar2 = *(param_1 + 0x4d);
        FUN_0047d608(param_1,param_2 - (*(param_1 + 0x75) >> 0x1),param_3 - (*(param_1 + 0x71) >> 0x1));
        if ((iVar1 == *(param_1 + 0x49)) && (iVar2 == *(param_1 + 0x4d))) {
            return 0x0;
        }
        if (param_4 == 0x0) {
            return 0x0;
        }
        FUN_00471bcd(param_1);
    }
    return 0x1;
}



fn FUN_0047d7d7(param_1: i32,param_2: i32,param_3: i32)

{
    *(param_1 + 0x8d) = (param_2 - *(param_1 + 0x1d)) / *(param_1 + 0x81) + *(param_1 + 0x49);
    *(param_1 + 0x91) = (param_3 - *(param_1 + 0x21)) / *(param_1 + 0x81) + *(param_1 + 0x4d);
    return;
}



fn FUN_0047d83f(param_1: i32,param_2: u32,param_3: u32)

{
    *(param_1 + 0x8d) = param_2;
    *(param_1 + 0x91) = param_3;
    *(param_1 + 0x85) =
        *(param_1 + 0x81) * (*(param_1 + 0x8d) - *(param_1 + 0x49)) + *(param_1 + 0x1d);
    *(param_1 + 0x89) =
        *(param_1 + 0x81) * (*(param_1 + 0x91) - *(param_1 + 0x4d)) + *(param_1 + 0x21);
    return;
}



fn FUN_0047d8c7(param_1: i32,param_2: u32,param_3: u32)

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut iVar4: i32;

    iVar1 = *(param_1 + 0x81) * 0x2;
    FUN_00498999(*(param_1 + 0x1d),*(param_1 + 0x21),*(param_1 + 0x25),*(param_1 + 0x29));
    FUN_0047d83f(param_1,param_2,param_3);
    iVar3 = *(param_1 + 0x85) - *(param_1 + 0x81);
    iVar4 = *(param_1 + 0x89) - *(param_1 + 0x81);
    iVar2 = *(param_1 + 0x81) * 0x3;
    FUN_00497896(iVar3,iVar4,iVar3,iVar4 + iVar2,0x10);
    FUN_00497896(iVar3 + *(param_1 + 0x81),iVar4,iVar3 + *(param_1 + 0x81),iVar4 + iVar2,0x10);
    FUN_00497896(iVar3 + iVar1,iVar4,iVar3 + iVar1,iVar4 + iVar2,0x10);
    FUN_00497896(iVar3 + iVar2,iVar4,iVar3 + iVar2,iVar4 + iVar2,0x10);
    FUN_00497896(iVar3 + 0x1,iVar4,iVar3 + 0x1,iVar4 + iVar2,0x10);
    FUN_00497896(iVar3 + 0x1 + *(param_1 + 0x81),iVar4,iVar3 + 0x1 + *(param_1 + 0x81),iVar4 + iVar2,0x10);
    FUN_00497896(iVar3 + 0x1 + iVar1,iVar4,iVar3 + 0x1 + iVar1,iVar4 + iVar2,0x10);
    FUN_00497896(iVar3 + 0x1 + iVar2,iVar4,iVar3 + 0x1 + iVar2,iVar4 + iVar2,0x10);
    FUN_00497896(iVar3,iVar4,iVar3 + iVar2,iVar4,0x10);
    FUN_00497896(iVar3,iVar4 + *(param_1 + 0x81),iVar3 + iVar2,iVar4 + *(param_1 + 0x81),0x10);
    FUN_00497896(iVar3,iVar4 + iVar1,iVar3 + iVar2,iVar4 + iVar1,0x10);
    FUN_00497896(iVar3,iVar4 + iVar2,iVar3 + iVar2,iVar4 + iVar2,0x10);
    FUN_00497896(iVar3,iVar4 + 0x1,iVar3 + iVar2,iVar4 + 0x1,0x10);
    FUN_00497896(iVar3,iVar4 + 0x1 + *(param_1 + 0x81),iVar3 + iVar2,iVar4 + 0x1 + *(param_1 + 0x81),0x10);
    FUN_00497896(iVar3,iVar4 + 0x1 + iVar1,iVar3 + iVar2,iVar4 + 0x1 + iVar1,0x10);
    FUN_00497896(iVar3,iVar4 + 0x1 + iVar2,iVar3 + iVar2,iVar4 + 0x1 + iVar2,0x10);
    FUN_00498ae4();
    return;
}



fn FUN_0047dbde(param_1: i32)

{
    let mut local_14: i32;

    FUN_004953d7();
    FUN_00498999(*(param_1 + 0x1d),*(param_1 + 0x21),*(param_1 + 0x25),*(param_1 + 0x29));
    for (local_14 = 0x0; local_14 < 0x78; local_14 = local_14 + 0x1) {
        if (((&DAT_005b7cb8)[local_14 * 0x14] & 0x1) != 0x0) {
            FUN_0047dc83(param_1,*(&DAT_005b7ca8 + local_14 * 0x14),
                         *(&DAT_005b7cac + local_14 * 0x14),*(&DAT_005b7cb0 + local_14 * 0x14),
                         *(&DAT_005b7cb4 + local_14 * 0x14),*(&DAT_005b7cb8 + local_14 * 0x14));
        }
    }
    FUN_00498ae4();
    FUN_0049536f();
    return;
}



void
FUN_0047dc83(param_1: i32,param_2: u32,param_3: u32,param_4: u32,param_5: u32,param_6: u32)

{
let mut local_48: u32;
let mut local_44: u32;
let mut local_40: u32;
let mut local_3c: u32;
let mut local_38: u32;
let mut local_34: u32;
let mut local_30: u32;
let mut local_2c: u32;
let mut local_28: i32;
let mut local_24: u32;
let mut local_20: u32;
let mut local_1c: u32;
let mut local_18: u32;
let mut local_14: *mut u8;

local_28 = *(param_1 + 0x81) >> 0x1;
FUN_0047d83f(param_1,param_2,param_3);
local_48 = *(param_1 + 0x85) + local_28;
local_44 = *(param_1 + 0x89) + local_28;
local_38 = *(param_1 + 0x85);
local_34 = *(param_1 + 0x85) + *(param_1 + 0x81);
local_30 = *(param_1 + 0x89);
local_2c = *(param_1 + 0x89) + *(param_1 + 0x81);
FUN_0047d83f(param_1,param_4,param_5);
local_40 = *(param_1 + 0x85) + local_28;
local_3c = *(param_1 + 0x89) + local_28;
local_24 = local_40;
local_20 = local_3c;
FUN_0047de3c(&local_48,&local_44,&local_24,&local_20,local_38,local_30,local_34,local_2c);
local_38 = *(param_1 + 0x85);
local_34 = *(param_1 + 0x85) + *(param_1 + 0x81);
local_30 = *(param_1 + 0x89);
local_2c = *(param_1 + 0x89) + *(param_1 + 0x81);
local_1c = local_48;
local_18 = local_44;
FUN_0047de3c(&local_40,&local_3c,&local_1c,&local_18,local_38,local_30,local_34,local_2c);
if ((param_6 & 0x10) == 0x0) {
if ((param_6 & 0x20) == 0x0) {
local_14 = 0xa0a0a;
}
else {
local_14 = 0xe7e7e7;
}
}
else {
local_14 = &DAT_00575757;
}
FUN_00497896(local_24,local_20,local_1c,local_18,local_14);
return;
}



u32
FUN_0047de3c(param_1: *mut u32,param_2: *mut u32,param_3: *mut u32,param_4: *mut u32,param_5: u32,param_6: u32,param_7: u32,param_8: u32
)

{
let mut uVar1: u32;
let mut local_18: u32;
let mut local_14: u32;

if (*param_1 < param_5) {
local_18 = 0x1000;
}
else {
if (param_7 < *param_1) {
local_18 = 0x100;
}
else {
local_18 = 0x0;
}
}
if (*param_2 < param_6) {
local_18 = local_18 | 0x10;
}
else {
if (param_8 < *param_2) {
local_18 = local_18 | 0x1;
}
}
if (*param_3 < param_5) {
local_14 = 0x1000;
}
else {
if (param_7 < *param_3) {
local_14 = 0x100;
}
else {
local_14 = 0x0;
}
}
if (*param_4 < param_6) {
local_14 = local_14 | 0x10;
}
else {
if (param_8 < *param_4) {
local_14 = local_14 | 0x1;
}
}
if ((local_14 & local_18) != 0x0) {
return local_18;
}
if ((local_18 | local_14) == 0x0) {
return 0x0;
}
if ((local_18 & 0x1000) == 0x0) {
if ((local_18 & 0x100) != 0x0) {
*param_2 = *param_2 +
((*param_1 - param_7) * (*param_4 - *param_2) * 0xa + 0x5) / ((*param_1 - *param_3) * 0xa);
*param_1 = param_7;^
// goto LAB_0047df8d;
}
}
else {
*param_2 = *param_2 +
((param_5 - *param_1) * (*param_4 - *param_2) * 0xa + 0x5) / ((*param_3 - *param_1) * 0xa);
*param_1 = param_5;
// LAB_0047df8d:
if ((param_8 < *param_2) && (param_8 < *param_4)) {
return *param_4;
}
if ((*param_2 < param_6) && (*param_4 < param_6)) {
return *param_4;
}
}
if (*param_2 < param_6) {
*param_1 = *param_1 +
((param_6 - *param_2) * (*param_3 - *param_1) * 0xa + 0x5) / ((*param_4 - *param_2) * 0xa);
*param_2 = param_6;
// LAB_0047e052:
if ((param_7 < *param_1) && (param_7 < *param_3)) {
return *param_3;
}
if ((*param_1 < param_5) && (*param_3 < param_5)) {
return *param_3;
}
}
else {
if (param_8 < *param_2) {
*param_1 = *param_1 +
((*param_2 - param_8) * (*param_3 - *param_1) * 0xa + 0x5) / ((*param_2 - *param_4) * 0xa);
*param_2 = param_8;^
// goto LAB_0047e052;
}
}
if ((local_14 & 0x1000) == 0x0) {
if ((local_14 & 0x100) == 0x0)^ // goto LAB_0047e144;
*param_4 = *param_4 +
((*param_3 - param_7) * (*param_2 - *param_4) * 0xa + 0x5) / ((*param_3 - *param_1) * 0xa);
*param_3 = param_7;
}
else {
*param_4 = *param_4 +
((param_5 - *param_3) * (*param_2 - *param_4) * 0xa + 0x5) / ((*param_1 - *param_3) * 0xa);
*param_3 = param_5;
}
if ((param_8 < *param_2) && (param_8 < *param_4)) {
return *param_4;
}
if ((*param_2 < param_6) && (*param_4 < param_6)) {
return *param_4;
}
// LAB_0047e144:
if (*param_4 < param_6) {
*param_3 = *param_3 +
((param_6 - *param_4) * (*param_1 - *param_3) * 0xa + 0x5) / ((*param_2 - *param_4) * 0xa);
*param_4 = param_6;
}
else {
if (*param_4 <= param_8) {
return *param_4;
}
*param_3 = *param_3 +
((*param_4 - param_8) * (*param_1 - *param_3) * 0xa + 0x5) / ((*param_4 - *param_2) * 0xa);
*param_4 = param_8;
}
if (((*param_1 <= param_7) || (uVar1 = *param_3, uVar1 <= param_7)) &&
(uVar1 = *param_1, uVar1 < param_5)) {
uVar1 = *param_3;
}
return uVar1;
}



fn FUN_0047e204(param_1: i32,param_2: i32,param_3: i32)

{
    *(param_1 + 0x59) = param_2 - (*(param_1 + 0x75) >> 0x1);
    *(param_1 + 0x5d) = param_3 - (*(param_1 + 0x75) >> 0x1);
    if (*(param_1 + 0x59) < 0x0) {
        *(param_1 + 0x59) = 0x0;
    }
    else {
        if (0x23 < *(param_1 + 0x59)) {
            *(param_1 + 0x59) = 0x23;
        }
    }
    if (*(param_1 + 0x5d) < 0x0) {
        *(param_1 + 0x5d) = 0x0;
    }
    else {
        if (0x23 < *(param_1 + 0x5d)) {
            *(param_1 + 0x5d) = 0x23;
        }
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address
// WARNING: Restarted to delay deadcode elimination for space: stack

void
FUN_0047e2a6(param_1: i32,param_2: u32,param_3: u32,param_4: u32,param_5: u32,param_6: u32,
param_7: u32,ulonglong param_8)

{
let cVar1: u8;
let mut iVar2: i32;
let mut uVar3: u32;
let mut pcVar4: String;
let mut uVar5: u32;
let mut in_stack_00000038: i32;
let mut local_3c: i32;
let mut local_38: i32;
let mut local_18: i32;

if (((*(&DAT_004be9b0 + DAT_004c9758 * 0x4) & param_8._2_4_) != 0x0) || (_DAT_004c975c != 0x0)) {
if ((param_8 & 0x800000) == 0x0) {
param_7._1_1_ = param_3._2_1_;
}
else {
}
local_18 = param_7._1_1_;
cVar1 = (&DAT_004be9e8)[local_18 * 0x4];
FUN_00498999(*(param_1 + 0x1d),*(param_1 + 0x21),*(param_1 + 0x25),*(param_1 + 0x29));
FUN_0047d83f(param_1,param_2._2_2_,param_3);
if (*(param_1 + 0x81) == 0x2) {
FUN_004953d7();
FUN_004968e7(*(param_1 + 0x85),*(param_1 + 0x89),*(param_1 + 0x81),*(param_1 + 0x81),
cVar1);
FUN_0049536f();
}
else {
if (*(param_1 + 0x81) == 0x22) {
local_38 = 0x6;
local_3c = 0x4;
}
else {
local_38 = 0x1;
local_3c = 0x1;
}
pcVar4 = FUN_0049c2c9(0x484);
FUN_004953d7();
FUN_004906c1(pcVar4,*(&DAT_004d6058 + param_3._3_1_ * 0x1c),cVar1,param_6._3_1_,0x22,-0x1);
FUN_00490a17(pcVar4,pcVar4,in_stack_00000038);
iVar2 = *(&DAT_004be878 + local_18 * 0x4);
uVar3 = *(&DAT_005b707e + param_2 * 0x4e + iVar2 * 0x4);
if ((*(&DAT_004be840 + local_18 * 0x4) & uVar3) == 0x0) {
switch(local_18) {
case 0x0:
FUN_00496d7e(*(param_1 + 0x85),*(param_1 + 0x89),*(param_1 + 0x61),
*(param_1 + 0x65),pcVar4,0x22,0x22);
*(&DAT_005b707e + local_18 * 0x4 + param_2 * 0x4e) =
*(&DAT_004be840 + local_18 * 0x4);
break;
case 0x1:
FUN_00496d7e(*(param_1 + 0x85) + 0x1,*(param_1 + 0x89),*(param_1 + 0x61),
*(param_1 + 0x65),pcVar4,0x22,0x22);
*(&DAT_005b707e + local_18 * 0x4 + param_2 * 0x4e) =
*(&DAT_004be840 + local_18 * 0x4);
break;
case 0x2:
FUN_00496d7e(*(param_1 + 0x85) + 0x2,*(param_1 + 0x89),*(param_1 + 0x61),
*(param_1 + 0x65),pcVar4,0x22,0x22);
*(&DAT_005b707e + local_18 * 0x4 + param_2 * 0x4e) =
*(&DAT_004be840 + local_18 * 0x4);
break;
case 0x3:
FUN_00496d7e(*(param_1 + 0x85),*(param_1 + 0x89) + 0x1,*(param_1 + 0x61),
*(param_1 + 0x65),pcVar4,0x22,0x22);
*(&DAT_005b707e + local_18 * 0x4 + param_2 * 0x4e) =
*(&DAT_004be840 + local_18 * 0x4);
break;
case 0x4:
FUN_00496d7e(*(param_1 + 0x85) + 0x2,*(param_1 + 0x89) + 0x1,*(param_1 + 0x61),
*(param_1 + 0x65),pcVar4,0x22,0x22);
*(&DAT_005b707e + local_18 * 0x4 + param_2 * 0x4e) =
*(&DAT_004be840 + local_18 * 0x4);
break;
case 0x5:
case 0x6:
case 0xd:
uVar5 = uVar3 >> 0x1c;
FUN_00496d7e(uVar5 * local_38 + *(param_1 + 0x85) + 0x2,
uVar5 * local_3c + *(param_1 + 0x89) + 0x2,*(param_1 + 0x61),
*(param_1 + 0x65),pcVar4,0x22,0x22);
*(&DAT_005b707e + iVar2 * 0x4 + param_2 * 0x4e) =
(uVar3 << 0x4) >> 0x4 | *(&DAT_004be840 + local_18 * 0x4) | (uVar5 + 0x1) * 0x10000000;
break;
case 0x7:
case 0x8:
uVar5 = uVar3 >> 0x1c;
FUN_00496d7e(*(param_1 + 0x85) - uVar5 * local_38,uVar5 * local_3c + *(param_1 + 0x89) + 0x2,
*(param_1 + 0x61),*(param_1 + 0x65),pcVar4,0x22,0x22);
*(&DAT_005b707e + iVar2 * 0x4 + param_2 * 0x4e) =
(uVar3 << 0x4) >> 0x4 | *(&DAT_004be840 + local_18 * 0x4) | (uVar5 + 0x1) * 0x10000000;
break;
case 0x9:
case 0xa:
case 0xb:
case 0xc:
FUN_00496d7e(*(param_1 + 0x85) + 0x1,(uVar3 >> 0x1c) * local_3c + *(param_1 + 0x89) + 0x2,
*(param_1 + 0x61),*(param_1 + 0x65),pcVar4,0x22,0x22);
*(&DAT_005b707e + iVar2 * 0x4 + param_2 * 0x4e) =
(uVar3 << 0x4) >> 0x4 | *(&DAT_004be840 + local_18 * 0x4) |
((uVar3 >> 0x1c) + 0x1) * 0x10000000;
}
}
FUN_0049536f();
FUN_0049af50(pcVar4);
}
FUN_00498ae4();
}
return;
}



fn FUN_0047e980(param_1: i32,param_2: u32,param_3: u32,param_4: i32)

{
    let mut uVar1: u32;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut local_38: u32;
    let local_24: u8;

    FUN_00498999(*(param_1 + 0x1d),*(param_1 + 0x21),*(param_1 + 0x25),*(param_1 + 0x29));
    FUN_0047d83f(param_1,param_2,param_3);
    iVar2 = FUN_00472441(param_1,*(param_1 + 0x8d),*(param_1 + 0x91));
    if (*(param_1 + 0x81) < 0x7) {
        FUN_00495520(*(param_1 + 0x85),*(param_1 + 0x89),*(param_1 + 0x81),
                     *(param_1 + 0x81),0x0);
        if (DAT_005b7068 == iVar2) {
            if (DAT_005b8808 == 0x0) {
                local_24 = 0xe;
            }
            else {
                local_24 = 0x27;
            }
            FUN_004968e7(*(param_1 + 0x85),*(param_1 + 0x89),*(param_1 + 0x81),*(param_1 + 0x81),
                         local_24);
        }
        else {
            FUN_004968e7(*(param_1 + 0x85),*(param_1 + 0x89),*(param_1 + 0x81),*(param_1 + 0x81),
                         0x27);
        }
        FUN_00495607(0x0);
    }
    else {
        uVar1 = *(&DAT_005b70c2 + iVar2 * 0x4e);
        iVar3 = (uVar1 & 0xf) - 0x1;
        FUN_004953d7();
        if (uVar1 != 0xffffffff) {
            if (param_4 != -0x1) {
                if (*(param_1 + 0x81) == 0x22) {
                    FUN_00496ee6(*(&DAT_005b8608 + (iVar3 * 0x10 + param_4) * 0x4),*(param_1 + 0x85) + 0x2,
                                 *(param_1 + 0x89) + 0x2,0x20,0x20);
                }
                else {
                    FUN_00497045(*(param_1 + 0x85) + 0x1,*(param_1 + 0x89) + 0x1,*(param_1 + 0x61),
                                 *(param_1 + 0x65),*(&DAT_005b8608 + (iVar3 * 0x10 + param_4) * 0x4),0x20,0x20);
                }
                if (DAT_005b7068 == iVar2) {
                    FUN_0047ef5e(*(param_1 + 0x85),*(param_1 + 0x89),*(param_1 + 0x81));
                }
            }
            iVar3 = FUN_004825ee(iVar2);
            if (iVar3 == -0x1) {
                local_38 = 0x272727;
            }
            else {
                local_38 = *(&DAT_004be9e8 + iVar3 * 0x4);
            }
            FUN_00497567((*(param_1 + 0x81) >> 0x1) + *(param_1 + 0x85),
                         *(param_1 + 0x89) - *(param_1 + 0x81),(&DAT_005b709e + iVar2 * 0x4e),
                         *(param_1 + 0x81) * 0x3,local_38,-0x1,0x0,DAT_004d6a6c,0x19);
        }
        FUN_0049536f();
    }
    FUN_00498ae4();
    return;
}



fn FUN_0047ec92(param_1: u32,param_2: i32,param_3: i32,param_4: i32,param_5: i32,param_6: u32)

{
    let mut local_14: i32;

    local_14 = 0x0;
    while( true ) {
        if (0x77 < local_14) {
            return;
        }
        if ((((&DAT_005b7cb8)[local_14 * 0x14] & 0x1) != 0x0) &&
            (((((*(&DAT_005b7ca8 + local_14 * 0x14) == param_2 &&
                (*(&DAT_005b7cac + local_14 * 0x14) == param_3)) &&
                (*(&DAT_005b7cb0 + local_14 * 0x14) == param_4)) &&
                (*(&DAT_005b7cb4 + local_14 * 0x14) == param_5)) ||
                (((*(&DAT_005b7ca8 + local_14 * 0x14) == param_4 &&
                    (*(&DAT_005b7cac + local_14 * 0x14) == param_5)) &&
                    ((*(&DAT_005b7cb0 + local_14 * 0x14) == param_2 &&
                        (*(&DAT_005b7cb4 + local_14 * 0x14) == param_3)))))))) break;
        local_14 = local_14 + 0x1;
    }
    *(&DAT_005b7cb8 + local_14 * 0x14) = *(&DAT_005b7cb8 + local_14 * 0x14) | param_6;
    return;
}



fn FUN_0047ed66()

{
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0x78; local_14 = local_14 + 0x1) {
        if (((&DAT_005b7cb8)[local_14 * 0x14] & 0x1) != 0x0) {
            (&DAT_005b7cb8)[local_14 * 0x14] = (&DAT_005b7cb8)[local_14 * 0x14] & 0xf;
        }
    }
    return;
}



fn FUN_0047eda9(param_1: u32,param_2: u32,param_3: i32,param_4: i32,param_5: i32,param_6: i32)

{
    let mut iVar1: i32;
    let cStack48: u8;
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: i32;
    let local_1c: *mut u32;
    let mut local_18: u32;
    let mut local_14: u32;

    local_1c = &DAT_005967b8;
    local_14 = (DAT_005967bc == 0x0);
    local_18 = local_14;
    if (((local_14 == 0x0) && ((DAT_005967bc + 0x41) != -0x1)) &&
        (*(DAT_005967bc + 0x23) >> 0x18 == DAT_004c9754)) {
        local_24 = 0x1;
        iVar1 = FUN_00432bd3(&DAT_005967b8);
        if (iVar1 == 0x0) {
            local_24 = 0x0;
        }
        if ((*(DAT_005967bc + 0x3a) & 0x80) == 0x0) {
            cStack48 = (DAT_005967bc + 0x26);
        }
        else {
            cStack48 = (DAT_005967bc + 0x35);
        }
        local_20 = cStack48;
        FUN_004729ba(local_20,&param_3,&param_4,0x1);
        param_2 = FUN_004710a6(&DAT_005967b8,param_3,param_4,*(DAT_005967bc + 0x41),
                               *(DAT_005967bc + 0x42));
        while (((param_2 = FUN_004713df(&DAT_005967b8,param_2,param_3,param_4,&local_2c,&local_28), param_2 != 0xffffffff &&
            (local_2c != -0x1)) && (local_28 != -0x1))) {
            if (local_24 == 0x0) {
                FUN_0047ec92(param_1,param_3,param_4,local_2c,local_28,0x20);
            }
            else {
                FUN_0047ec92(param_1,param_3,param_4,local_2c,local_28,0x10);
                local_24 = 0x0;
            }
            if (param_2 == 0x0) {
                return;
            }
            if ((local_2c == param_5) && (local_28 == param_6)) {
                return;
            }
            param_3 = local_2c;
            param_4 = local_28;
        }
    }
    return;
}



fn FUN_0047ef5e(param_1: i32,param_2: i32,param_3: i32)

{
    let mut iVar1: i32;
    let mut iVar2: i32;

    iVar1 = ((param_3 + (param_3 >> 0x1f) * -0x8) - ((param_3 >> 0x1f) << 0x2 < 0x0)) >> 0x3;
    iVar2 = param_3 - iVar1;
    FUN_00497896(param_1,param_2,param_1,param_2 + iVar1,0x27);
    FUN_00497896(param_1,param_2,param_1 + iVar1,param_2,0x27);
    FUN_00497896(param_1 + param_3,param_2,param_1 + iVar2,param_2,0x27);
    FUN_00497896(param_1 + param_3,param_2,param_1 + param_3,param_2 + iVar1,0x27);
    FUN_00497896(param_1,param_2 + param_3,param_1,param_2 + iVar2,0x27);
    FUN_00497896(param_1,param_2 + param_3,param_1 + iVar1,param_2 + param_3,0x27);
    FUN_00497896(param_1 + param_3,param_2 + param_3,param_1 + iVar2,param_2 + param_3,0x27);
    FUN_00497896(param_1 + param_3,param_2 + param_3,param_1 + param_3,param_2 + iVar2,0x27);
    return;
}



// WARNING: Switch with 1 destination removed at 0x0047f3d7 : 5 cases all go to same destination
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0047f0b3()

{
    let mut uVar1: u32;
    let puVar2: *mut u32;
    let mut uVar3: u32;
    let mut uVar4: u32;
    let mut local_1c: u32;
    let mut local_18: i32;

    DAT_004c9758 = 0x0;
    _DAT_004c975c = 0x0;
    FUN_004a0430(&DAT_004d7730,0x0,0x46d);
    FUN_004a0430(&DAT_00568210,0x0,0x1a5dc);
    FUN_004a0430(&DAT_005827f0,0x0,0x134);
    FUN_004a0430(&DAT_004c9778,0x0,0xf8);
    _DAT_004c6160 = 0x0;
    FUN_004a0430(&DAT_004c6168,0x0,0xfa0);
    _DAT_004c9750 = 0x0;
    DAT_00591cb4 = 0x0;
    DAT_00591cb0 = 0x186a0;
    DAT_004c9754 = 0x0;
    DAT_004c9760 = 0x1;
    _DAT_004c9764 = 0x0;
    DAT_004c9770 = 0xffffffff;
    DAT_004c976c = 0xffffffff;
    _DAT_004c9774 = 0x0;
    uVar3 = third_republic_max_00599dc0 - third_republic_min_00599dbc;
    uVar4 = uVar3 >> 0x1f;
    uVar1 = FUN_004a2edc();
    _DAT_004c9768 = uVar1 % ((uVar3 ^ uVar4) - uVar4) + third_republic_min_00599dbc;
    _DAT_004d5560 = turns_till_vote_opt_00599d44;
    _DAT_004d5564 = turns_till_patriarch_dies_00599d48;
    DAT_00595738 = 0xffffffff;
    FUN_004a0430(&DAT_004d566c,0x0,0x3e8);
    for (local_1c = 0x0; local_1c < 0x3; local_1c = local_1c + 0x1) {
        (&DAT_004c5048)[local_1c * 0x2] = 0xffffffff;
        *(local_1c * 0x8 + 0x4c504c) = 0xffffffff;
    }
    FUN_004629cd(&DAT_004d55a8);
    for (local_1c = 0x0; local_1c < 0xe; local_1c = local_1c + 0x1) {
        *(&DAT_00569aa1 + local_1c * 0x1e22) = local_1c;
        if (local_1c < 0x5) {
            if (((&DAT_00569a98)[local_1c * 0x1e22] & 0x2) == 0x0) {
                *(&DAT_00569adc + local_1c * 0x1e22) = dflt_tax_rate_00599d64;
                *(&DAT_00569aec + local_1c * 0x1e22) = dflt_tax_rate_00599d64;
                *(&DAT_00569ae4 + local_1c * 0x1e22) = 0x0;
            }
            else {
                *(&DAT_00569adc + local_1c * 0x1e22) = 0xa;
                *(&DAT_00569aec + local_1c * 0x1e22) = 0xa;
                *(&DAT_00569ae4 + local_1c * 0x1e22) = 0x0;
                FUN_004a0430(&DAT_004d5a58 + local_1c * 0x64,0x0,0x64);
                FUN_004a0430(&DAT_004d5c50 + local_1c * 0x64,0x0,0x64);
            }
            *(&DAT_00569ae0 + local_1c * 0x1e22) = dflt_tithe_rate_00599d68;
            *(&DAT_00569ae8 + local_1c * 0x1e22) = 0x64;
            *(&DAT_00569af0 + local_1c * 0x1e22) = 0x64;
            *(&DAT_00569b04 + local_1c * 0x1e22) = dflt_interest_rate_00599d78;
            puVar2 = FUN_00455143();
            (&DAT_00599c6c + local_1c * 0x4) = puVar2;
        }
        for (local_18 = 0x0; local_18 < 0xd; local_18 = local_18 + 0x1) {
            *(&DAT_00569bc8 + local_18 * 0x8 + local_1c * 0x1e22) =
                *(&DAT_004d6a70 + local_18 * 0xa8);
            *(&DAT_00569bcc + local_1c * 0x1e22 + local_18 * 0x8) =
                *(&DAT_004d6a74 + local_18 * 0xa8);
        }
            *(&DAT_00569ac5 + local_1c * 0x1e22) = 0xffffffff;
        (&DAT_00569ad1)[local_1c * 0x1e22] = 0xff;
        (&DAT_00569ad2)[local_1c * 0x1e22] = 0xff;
        (&DAT_00569ad3)[local_1c * 0x1e22] = 0xff;
        *(&DAT_00569acd + local_1c * 0x1e22) = default_leadership_00599d5c;
        (&DAT_00569a98)[local_1c * 0x1e22] = (&DAT_00569a98)[local_1c * 0x1e22] & 0xef;
        *(&DAT_00569ad4 + local_1c * 0x1e22) = 0x0;
        (&DAT_00569c30)[local_1c * 0x1e22] = (&DAT_00569c30)[local_1c * 0x1e22] | 0x1;
        (&DAT_00569c30)[local_1c * 0x1e22] = (&DAT_00569c30)[local_1c * 0x1e22] & 0xfd;
        (&DAT_00569e94)[local_1c * 0x1e22] = (&DAT_00569e94)[local_1c * 0x1e22] | 0x1;
        (&DAT_00569e94)[local_1c * 0x1e22] = (&DAT_00569e94)[local_1c * 0x1e22] & 0xfd;
        (&DAT_00569c39)[local_1c * 0x1e22] = (&DAT_00569c39)[local_1c * 0x1e22] & 0xfe;
        (&DAT_00569d11)[local_1c * 0x1e22] = (&DAT_00569d11)[local_1c * 0x1e22] & 0xfe;
        (&DAT_00569e1f)[local_1c * 0x1e22] = (&DAT_00569e1f)[local_1c * 0x1e22] & 0xfe;
        (&DAT_00569c39)[local_1c * 0x1e22] = (&DAT_00569c39)[local_1c * 0x1e22] | 0x2;
        (&DAT_00569d11)[local_1c * 0x1e22] = (&DAT_00569d11)[local_1c * 0x1e22] | 0x2;
        (&DAT_00569e1f)[local_1c * 0x1e22] = (&DAT_00569e1f)[local_1c * 0x1e22] | 0x2;
        *(&DAT_00569abd + local_1c * 0x1e22) = starting_credits_opts_00599d60;
        *(&DAT_00569aa5 + local_1c * 0x1e22) = 0x0;
        *(&DAT_00569aa9 + local_1c * 0x1e22) = 0x0;
        *(&DAT_00569ab1 + local_1c * 0x1e22) = 0x0;
        for (local_18 = 0x0; local_18 < 0xe; local_18 = local_18 + 0x1) {
            *(&DAT_00569b14 + local_18 * 0x4 + local_1c * 0x1e22) = 0x32;
        }
    }
    for (local_1c = 0x1; local_1c < 0x72; local_1c = local_1c + 0x1) {
        if (*(&DAT_0058ad6e + local_1c * 0xda) < 0x6) {
            *(&DAT_0058ad72 + local_1c * 0xda) = 0x1;
        }
        else {
            *(&DAT_0058ad72 + local_1c * 0xda) = 0x0;
        }
    }
    FUN_004a0430(&DAT_005b2e1c,0x0,0xa);
    DAT_004d559c._0_1_ = (byte)DAT_004d559c & 0xf3;
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0047f618() -> i32

{
let cVar1: u8;
let mut iVar2: i32;
UINT UVar3;
let mut uVar4: u32;
let mut pcVar5: String;
let mut pcVar6: String;
let mut local_60: i32;
let mut local_5c: i32;
let key_name: u8 [0x40];
let mut local_18: i32;
let mut local_14: i32;

for (local_14 = 0x0; local_14 < 0x28; local_14 = local_14 + 0x1) {
FUN_00472a32(local_14);
for (local_18 = 0x0; local_18 < 0x2c; local_18 = local_18 + 0x1) {
*(&DAT_004d7d50 + local_18 * 0x4 + local_14 * 0x3890) = local_14 * 0x3890 + 0x4d7e00 + local_18 * 0x104;
}
}
FUN_0047f0b3();
FUN_00493573();
_DAT_00591cac = 0x0;
DAT_004d5588 = 0x1;
DAT_004d5584 = 0x1;
DAT_004d558c = 0x1;
DAT_004d5590 = 0x1;
DAT_004d5594 = 0x1;
DAT_004c9754 = 0x0;
_DAT_00596a48 = 0x0;
DAT_00599d3c = 0x0;
FUN_00431d0a(&DAT_005967b8);
FUN_00450d29(&DAT_00595740);
iVar2 = FUN_00450d29(&DAT_00595f70);
for (local_14 = 0x0; local_14 < 0xb; local_14 = local_14 + 0x1) {
(&DAT_004d7796)[local_14 * 0x67] = 0xff;
(&DAT_004d7792 + local_14 * 0x67 + 0x3) = 0xff;
iVar2 = local_14;
}
local_14 = 0x0;
loop {
if (0x4 < local_14) {
for (local_14 = 0x0; local_14 < 0xe; local_14 = local_14 + 0x1) {
for (local_60 = 0x0; local_60 < 0xe; local_60 = local_60 + 0x1) {
FUN_0042d188(local_14,local_60);
}
*(&DAT_00569b4c + local_14 * 0x1e22) = 0x0;
*(&DAT_00569b64 + local_14 * 0x1e22) = 0x0;
switch(local_14) {
case 0x0:
case 0x1:
case 0x2:
case 0x3:
case 0x4:
break;
case 0x5:
(&DAT_00569a98)[local_14 * 0x1e22] = (&DAT_00569a98)[local_14 * 0x1e22] | 0x2;
pcVar6 = FUN_00499050(DAT_0059679c,0x7142);
FUN_0049c2e0(&DAT_00569b50 + local_14 * 0x1e22,pcVar6);
*(&DAT_00569ac9 + local_14 * 0x1e22) = 0x5;
break;
case 0x6:
(&DAT_00569a98)[local_14 * 0x1e22] = (&DAT_00569a98)[local_14 * 0x1e22] | 0x2;
*(&DAT_00569ac9 + local_14 * 0x1e22) = 0x5;
FUN_00499050(DAT_0059679c,*(&DAT_00569ac9 + local_14 * 0x1e22) + 0x40a);
pcVar6 = FUN_00499050(DAT_0059679c,0x7141);
FUN_0049c2e0(&DAT_00569b50 + local_14 * 0x1e22,pcVar6);
break;
case 0x7:
(&DAT_00569a98)[local_14 * 0x1e22] = (&DAT_00569a98)[local_14 * 0x1e22] | 0x2;
pcVar6 = FUN_00499050(DAT_0059679c,0x713f);
FUN_0049c2e0(&DAT_00569b50 + local_14 * 0x1e22,pcVar6);
break;
case 0x8:
(&DAT_00569a98)[local_14 * 0x1e22] = (&DAT_00569a98)[local_14 * 0x1e22] | 0x2;
pcVar6 = FUN_00499050(DAT_0059679c,0x7143);
FUN_0049c2e0(&DAT_00569b50 + local_14 * 0x1e22,pcVar6);
break;
default:
(&DAT_00569a98)[local_14 * 0x1e22] = (&DAT_00569a98)[local_14 * 0x1e22] | 0x2;
pcVar6 = FUN_00499050(DAT_0059679c,local_14 + 0x414);
FUN_0049c2e0(&DAT_00569b50 + local_14 * 0x1e22,pcVar6);
}
iVar2 = local_14;
}
return iVar2;
}
FUN_0049c2e0(key_name,s_Player__d_004c2540);
iVar2 = local_14 * 0x1e22;
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
UVar3 = GetPrivateProfileIntA(s_Computer_004c254a,key_name,0x0,&filename_00599c80);
(&DAT_00569a98)[iVar2] = (&DAT_00569a98)[iVar2] & 0xfd;
(&DAT_00569a98)[iVar2] = (&DAT_00569a98)[iVar2] | ((byte)UVar3 & 0x1) * '\x02';
for (local_5c = 0x0; local_5c < 0x10; local_5c = local_5c + 0x1) {
(&DAT_00569b68)[local_5c * 0x3 + local_14 * 0x1e22] = (&DAT_004d7410)[local_14 * 0xa0 + local_5c];
(&DAT_00569b98)[local_5c * 0x3 + local_14 * 0x1e22] = (&DAT_004d7420)[local_14 * 0xa0 + local_5c];
}
if (((&DAT_00569a98)[local_14 * 0x1e22] & 0x2) == 0x0) {
*(&DAT_00569b60 + local_14 * 0x1e22) = 0x0;
}
else {
uVar4 = FUN_004a2edc();
*(&DAT_00569b60 + local_14 * 0x1e22) = uVar4 % 0x3;
}
pcVar5 = *(&DAT_004d7ba0 + local_14 * 0xc + *(&DAT_00569b60 + local_14 * 0x1e22) * 0x4);
pcVar6 = &DAT_00569b50 + local_14 * 0x1e22;
loop {
cVar1 = *pcVar5;
*pcVar6 = cVar1;
if (cVar1 == '\0') break;
cVar1 = pcVar5[0x1];
pcVar5 = pcVar5 + 0x2;
pcVar6[0x1] = cVar1;
pcVar6 = pcVar6 + 0x2;
} while (cVar1 != '\0');
(&DAT_00569b60)[local_14 * 0x1e22] = 0x0;
*(&DAT_00569ac9 + local_14 * 0x1e22) = 0x5;
iVar2 = local_14;
local_14 = local_14 + 0x1;
} while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0047fb0a()

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    byte *pbVar3;
    PDWORD pDVar4;
    let mut bVar5: bool;
    let mut bVar6: bool;
    PDWORD local_20;
    PDWORD local_1c;

    // LPCSTR lpFileName for GetPrivateProfileIntA
    // INT nDefault for GetPrivateProfileIntA
    // LPCSTR lpKeyName for GetPrivateProfileIntA
    // LPCSTR lpAppName for GetPrivateProfileIntA
    excommunicate_empire_expiration_00582914 =
        GetPrivateProfileIntA(s_Church_004c2568,s_excommunicate_expire_004c2553,0x5,&filename_00599c80);
    // LPCSTR lpFileName for GetPrivateProfileIntA
    // INT nDefault for GetPrivateProfileIntA
    // LPCSTR lpKeyName for GetPrivateProfileIntA
    // LPCSTR lpAppName for GetPrivateProfileIntA
    excommunicate_turns_00582918 =
        GetPrivateProfileIntA(s_Church_004c2583,s_excommunicate_turns_004c256f,0x5,&filename_00599c80);
    // LPCSTR lpFileName for GetPrivateProfileIntA
    // INT nDefault for GetPrivateProfileIntA
    // LPCSTR lpKeyName for GetPrivateProfileIntA
    // LPCSTR lpAppName for GetPrivateProfileIntA
    excommunicate_loyalty_hit_0058291c =
        GetPrivateProfileIntA(s_Church_004c25a4,s_excom_peasant_loyalty_hit_004c258a,0x1e,&filename_00599c80);
    // LPCSTR lpFileName for GetPrivateProfileIntA
    // INT nDefault for GetPrivateProfileIntA
    // LPCSTR lpKeyName for GetPrivateProfileIntA
    // LPCSTR lpAppName for GetPrivateProfileIntA
    sign_holy_writ_00582920 =
        GetPrivateProfileIntA(s_Church_004c25ba,s_sign_holy_writ_004c25ab,0x32,&filename_00599c80);
    _WORD_005827f2 = 0xffffffff;
    local_20 = *DAT_005967c8;
    loop {
    if (local_20 == (PDWORD)0x0) {
        LAB_0047fc18:
            local_1c = *DAT_005967b0;
        loop {
            if (local_1c == (PDWORD)0x0) {
                _DAT_005827f0 = DAT_004d5580;
                DAT_005827f4._2_2_ = 0xffff;
                return;
            }
            iVar2 = 0x6;
            bVar5 = false;
            iVar1 = 0x0;
            bVar6 = true;
            pbVar3 = &DAT_005827f0;
            pDVar4 = local_1c + 0x8;
            loop {
                if (iVar2 == 0x0) break;
                iVar2 = iVar2 + -0x1;
                bVar5 = *pbVar3 < *pDVar4;
                bVar6 = *pbVar3 == *pDVar4;
                pbVar3 = pbVar3 + 0x1;
                pDVar4 = (PDWORD)(pDVar4 + 0x1);
            } while (bVar6);
            if (!bVar6) {
                iVar1 = (0x1 - bVar5) - (bVar5 != 0x0);
            }
            if ((iVar1 == 0x0) && ((local_1c + 0x27) == '4')) {
                _DAT_005827f0 = DAT_004d5580;
                DAT_005827f4._2_2_ = (local_1c + 0xd);
                return;
            }
            local_1c = (PDWORD)*local_1c;
        } while( true );
    }
    if ((*(local_20 + 0x6) >> 0x10 == _DAT_004d5580) && ((local_20 + 0xe) == 0x1)) {
        _WORD_005827f2 = *(local_20 + 0xa);^
        // goto LAB_0047fc18;
    }
    local_20 = (PDWORD)*local_20;
} while( true );
}



fn FUN_0047fc74() -> u32

{
    let mut uVar1: u32;
    let mut local_118: *mut u32 [0x11];
    let ppuStack211: *mut *mut u8;;
    let mut local_27: String;;
    let local_20: *mut i32;;
    let mut local_1c: u32;
    let mut local_18: u32;

    local_20 = FUN_004990e0(local_118,0x0,s_efs_res_004c25ca,s_HouseCon_004c25c1);
    loop {
    local_18 = FUN_0049bb50(local_118,FUN_0047fda8);
    if (local_18 != 0xc5) break;
    uVar1 = FUN_004806c5();
} while (uVar1 == 0xc6);
    local_1c = local_18;
    ppuStack211 = &PTR_FUN_004c3d34;
    if (local_27 != 0x0) {
        FUN_00499b30(local_118,local_27);
    }
    FUN_0049a1c0(local_118,0x1);
    return local_1c;
}



fn FUN_0047fda8(param_1: *mut *mut u32,param_2: u32,param_3: u32,param_4: i32) -> u32

{
    let mut pcVar1: String;
    let mut puVar2: *mut u8;
    let mut iVar3: i32;
    let mut local_28: i32;
    let mut local_1c: i32;

    if (param_2 < 0x401) {
        if (param_2 == 0x100) {
            if ((param_4 != 0x0) && (iVar3 = FUN_004a11c0(param_4), iVar3 == 0x1b)) {
                return 0x1;
            }
            return 0x0;
        }
    }
    else {
        if (param_2 < 0x402) {
            for (local_1c = 0x0; local_1c < 0x5; local_1c = local_1c + 0x1) {
                pcVar1 = FUN_00499050(DAT_0059679c,local_1c + 0x414);
                FUN_0049bf80(param_1,local_1c + 0xc8,0x502,0xc,pcVar1);
                FUN_0049bf80(param_1,local_1c + 0x12c,0x502,((byte)((&DAT_00569a98)[local_1c * 0x1e22] << 0x6) >> 0x7),0x0
                );
            }
        }
        else {
            if (0x406 < param_2) {
                if (param_2 < 0x408) {
                    if (param_3 < 0x12c) {
                        if (param_3 < 0xc6) {
                            if (param_3 == 0xc5)^ // goto LAB_0047ff61;
                        }
                        else {
                            if ((param_3 < 0xc7) || (param_3 == 0xc7)) {
                                LAB_0047ff61:
                                    FUN_0049c140(param_1,param_3);
                                return 0x0;
                            }
                        }
                    }
                    else {
                        if ((((param_3 < 0x12d) || (param_3 < 0x12e)) || (param_3 < 0x12f)) ||
                            ((param_3 < 0x130 || (param_3 == 0x130)))) {
                            iVar3 = (param_3 - 0x12c) * 0x1e22;
                            puVar2 = FUN_0049bf80(param_1,param_3,0x501,0x0,0x0);
                            (&DAT_00569a98)[iVar3] = (&DAT_00569a98)[iVar3] & 0xfd;
                            (&DAT_00569a98)[iVar3] = (&DAT_00569a98)[iVar3] | ((byte)puVar2 & 0x1) * '\x02';
                            local_28 = 0x0;
                            while( true ) {
                                if (0x4 < local_28) {
                                    FUN_0049bf80(param_1,param_3,0x502,0x0,0x0);
                                    (&DAT_00569a98)[(param_3 - 0x12c) * 0x1e22] = (&DAT_00569a98)[(param_3 - 0x12c) * 0x1e22] & 0xfd;
                                    return 0x0;
                                }
                                if (((&DAT_00569a98)[local_28 * 0x1e22] & 0x2) == 0x0) break;
                                local_28 = local_28 + 0x1;
                            }
                            return 0x0;
                        }
                    }
                }
                else {
                    if (param_2 == 0x40c) {
                        FUN_004953d7();
                        FUN_004a08c5(s_pcx_cathed3_pcx_004c25d2,0x0,0x0,0x280,0x1e0,0x0,0x0,0x0,0x1);
                        FUN_0049536f();
                        return 0x1;
                    }
                }
            }
        }
    }
    return 0x0;
}



fn FUN_0048006a() -> u32

{
    let mut uVar1: u32;
    let mut local_118: *mut u32 [0x11];
    let ppuStack211: *mut *mut u8;;
    let mut local_27: String;;
    let local_20: *mut i32;;
    let mut local_1c: u32;
    let mut local_18: u32;

    local_20 = FUN_004990e0(local_118,0x0,s_efs_res_004c25ed,s_MapOptions_004c25e2);
    loop {
    local_18 = FUN_0049bb50(local_118,FUN_0048019e);
    if (local_18 != 0xc5) break;
    uVar1 = FUN_0047fc74();
} while (uVar1 == 0xc6);
    local_1c = local_18;
    ppuStack211 = &PTR_FUN_004c3d34;
    if (local_27 != 0x0) {
        FUN_00499b30(local_118,local_27);
    }
    FUN_0049a1c0(local_118,0x1);
    return local_1c;
}



fn FUN_0048019e(param_1: *mut *mut u32,param_2: u32,param_3: i32,param_4: i32) -> u32

{
    let mut iVar1: i32;

    if (param_2 < 0x401) {
        if (param_2 == 0x100) {
            if ((param_4 != 0x0) && (iVar1 = FUN_004a11c0(param_4), iVar1 == 0x1b)) {
                return 0x1;
            }
            return 0x0;
        }
    }
    else {
        if (param_2 < 0x402) {
            FUN_0049bf80(param_1,0xca,0x414,0x0,0x0);
            FUN_0049bf80(param_1,0xca,0x410,0x0,0x0);
            return 0x0;
        }
        if (0x406 < param_2) {
            if (param_2 < 0x408) {
                switch(param_3) {
                    case 0xc5:
                        case 0xc6:
                        case 0xc7:
                    if (DAT_004d5f50 == '\0') {
                        FUN_0049c2e0(&DAT_004d5f50,s_galaxy_gal_004c261d);
                    }
                    FUN_0049c140(param_1,param_3);
                    break;
                    case 0xc8:
                        FUN_0049c2e0(&DAT_004d5f50,s_galaxy_gal_004c2605);
                    FUN_0049c140(param_1,0xc5);
                    break;
                    case 0xc9:
                        FUN_0049c2e0(&DAT_004d5f50,s__________gal_004c2610);
                    FUN_0049c140(param_1,0xc5);
                }
            }
            else {
                if (param_2 == 0x40c) {
                    FUN_004953d7();
                    FUN_004a08c5(s_pcx_cathed3_pcx_004c25f5,0x0,0x0,0x280,0x1e0,0x0,0x0,0x0,0x1);
                    FUN_0049536f();
                    return 0x1;
                }
            }
        }
    }
    return 0x0;
}



fn FUN_00480382() -> u32

{
    let mut uVar1: u32;
    let mut local_118: *mut u32 [0x11];
    let ppuStack211: *mut *mut u8;;
    let mut local_27: String;;
    let local_20: *mut i32;;
    let mut local_1c: u32;
    let mut local_18: u32;

    local_20 = FUN_004990e0(local_118,0x0,s_efs_res_004c2631,s_StartDlg_004c2628);
    loop {
    local_18 = FUN_0049bb50(local_118,FUN_004804b3);
    if (local_18 != 0x64) break;
    uVar1 = FUN_0048006a();
} while (uVar1 == 0xc6);
    local_1c = local_18;
    ppuStack211 = &PTR_FUN_004c3d34;
    if (local_27 != 0x0) {
        FUN_00499b30(local_118,local_27);
    }
    FUN_0049a1c0(local_118,0x1);
    return local_1c;
}



fn FUN_004804b3(param_1: i32,param_2: u32,param_3: u32,param_4: i32) -> u32

{
    let cVar1: u8;
    let mut iVar2: i32;
    let mut pcVar3: String;
    let mut uVar4: u32;
    let mut pcVar5: String;

    if (param_2 < 0x407) {
        if (param_2 == 0x100) {
            if ((param_4 != 0x0) && (iVar2 = FUN_004a11c0(param_4), iVar2 == 0x1b)) {
                return 0x1;
            }
            return 0x0;
        }
    }
    else {
        if (param_2 < 0x408) {
            if (param_3 < 0x66) {
                if (param_3 < 0x64) {
                    return 0x0;
                }
                if (0x64 < param_3) {
                    if (DAT_00599d04 == 0x0) {
                        DAT_00599d04 = FUN_0049c2c9(0x104);
                    }
                    uVar4 = find_file_fn_0045bde0(s_sav___sav_004c2649,DAT_00599d04,0x0);
                    if (uVar4 == 0x0) {
                        DAT_00599d04 = 0x0;
                        return 0x0;
                    }
                    pcVar5 = FUN_00430bce(DAT_00599d04);
                    pcVar3 = DAT_00599d04;
                    loop {
                        cVar1 = *pcVar5;
                        *pcVar3 = cVar1;
                        if (cVar1 == '\0') break;
                        cVar1 = pcVar5[0x1];
                        pcVar5 = pcVar5 + 0x2;
                        pcVar3[0x1] = cVar1;
                        pcVar3 = pcVar3 + 0x2;
                    } while (cVar1 != '\0');
                    FUN_0047a593(DAT_00596a38);
                    FUN_0049c140(param_1,param_3);
                    return 0x0;
                }
            }
            else {
                if (0x66 < param_3) {
                    if (param_3 < 0x68) {
                        return 0x0;
                    }
                    if (0x68 < param_3) {
                        if (param_3 != 0x29a) {
                            return 0x0;
                        }
                        FUN_00426782();
                        return 0x0;
                    }
                    pcVar3 = FUN_00499050(DAT_0059679c,0x7341);
                    uVar4 = FUN_0049d2e0(param_1,0x3,pcVar3);
                    if (uVar4 == 0x0) {
                        return 0x0;
                    }
                    FUN_0049c140(param_1,param_3);
                    return 0x0;
                }
            }
            FUN_0049c140(param_1,param_3);
        }
        else {
            if (param_2 == 0x40c) {
                FUN_004953d7();
                FUN_004a08c5(s_pcx_cathed3_pcx_004c2639,0x0,0x0,0x280,0x1e0,0x0,0x0,0x0,0x1);
                FUN_0049536f();
                return 0x1;
            }
        }
    }
    return 0x0;
}



fn FUN_004806c5() -> u32

{
    let mut local_11c: *mut u32 [0x11];
    let ppuStack215: *mut *mut u8;;
    let mut local_2b: String;;
    let mut local_24: u32;
    let mut local_20: u32;
    let mut local_1c: i32;
    let local_18: *mut i32;;

    local_18 = FUN_004990e0(local_11c,0x0,s_efs_res_004c265b,s_GameOpt_004c2653);
    local_20 = FUN_0049bb50(local_11c,FUN_00480801);
    for (local_1c = 0x0; local_1c < 0x5; local_1c = local_1c + 0x1) {
        FUN_004a0430(&DAT_004d5c50 + local_1c * 0x64,DAT_004d5594,0x64);
    }
    local_24 = local_20;
    ppuStack215 = &PTR_FUN_004c3d34;
    if (local_2b != 0x0) {
        FUN_00499b30(local_11c,local_2b);
    }
    FUN_0049a1c0(local_11c,0x1);
    return local_24;
}



fn FUN_00480801(param_1: *mut *mut u32,param_2: u32,param_3: u32,param_4: *mut i32) -> u32

{
    let mut pcVar1: String;
    let mut iVar2: i32;
    let mut puVar3: *mut u8;
    let mut uVar4: u32;
    let mut iVar5: i32;
    let mut uVar6: u32;
    let puVar7: *mut u32;
    let bVar8: u8;
    let mut local_30: i32;
    let mut local_24: i32;
    let mut local_20: u32;
    let mut local_1c: u32;

    if (param_2 < 0x407) {
        if (0xff < param_2) {
            if (param_2 < 0x101) {
                if ((param_4 != 0x0) && (iVar2 = FUN_004a11c0(param_4), iVar2 == 0x1b)) {
                    return 0x1;
                }
                return 0x0;
            }
            if (param_2 == 0x401) {
                FUN_0049bf80(param_1,0xc5,0x414,0x0,0x0);
                FUN_0049bf80(param_1,0xc5,0x410,0x0,0x0);
                FUN_0049bf80(param_1,0x19d,0x502,DAT_004d5584,0x0);
                FUN_0049bf80(param_1,0x19a,0x502,DAT_004d5588,0x0);
                FUN_0049bf80(param_1,0x19c,0x502,DAT_004d558c,0x0);
                FUN_0049bf80(param_1,0x19e,0x502,DAT_004d5590,0x0);
                FUN_0049bf80(param_1,0x19f,0x502,DAT_004d5594,0x0);
                local_20 = (((byte)DAT_004d559c & 0x4) != 0x0);
                FUN_0049bf80(param_1,0x1bc8,0x502,local_20,0x0);
                local_1c = (((byte)DAT_004d559c & 0x8) != 0x0);
                FUN_0049bf80(param_1,0x1bcb,0x502,local_1c,0x0);
                FUN_0049bf80(param_1,0x19b,0x502,DAT_004c9760,0x0);
                if (((byte)DAT_004d559c & 0x4) == 0x0) {
                    FUN_0049bf80(param_1,0x1bcb,0x410,0x1,0x0);
                }
                for (local_24 = 0x0; local_24 < 0x5; local_24 = local_24 + 0x1) {
                    pcVar1 = FUN_00499050(DAT_0059679c,local_24 + 0x414);
                    FUN_0049bf80(param_1,local_24 + 0xc8,0x502,0x10,pcVar1);
                    FUN_0049bf80(param_1,local_24 + 0x136,0x503,0x5,0x0);
                    FUN_0049bf80(param_1,local_24 + 0x136,0x502,*(&DAT_00569b4c + local_24 * 0x1e22),0x0);
                }
            }
        }
    }
    else {
        if (param_2 < 0x408) {
            if (param_3 < 0x13a) {
                if (param_3 < 0x136) {
                    if (param_3 < 0xc6) {
                        if (param_3 != 0xc5) {
                            return 0x0;
                        }
                    }
                    else {
                        if ((0xc6 < param_3) && (param_3 != 0xc7)) {
                            return 0x0;
                        }
                    }
                    FUN_0049c140(param_1,param_3);
                    return 0x0;
                }
            }
            else {
                if (0x13a < param_3) {
                    if (param_3 < 0x19d) {
                        if (param_3 < 0x19b) {
                            if (param_3 != 0x19a) {
                                return 0x0;
                            }
                            DAT_004d5588 = FUN_0049bf80(param_1,0x19a,0x501,0x0,0x0);
                            return 0x0;
                        }
                        if (0x19b < param_3) {
                            DAT_004d558c = FUN_0049bf80(param_1,param_3,0x501,0x0,0x0);
                            return 0x0;
                        }
                        DAT_004c9760 = FUN_0049bf80(param_1,param_3,0x501,0x0,0x0);
                        return 0x0;
                    }
                    if (param_3 < 0x19e) {
                        DAT_004d5584 = FUN_0049bf80(param_1,param_3,0x501,0x0,0x0);
                        return 0x0;
                    }
                    if (param_3 < 0x19f) {
                        DAT_004d5590 = FUN_0049bf80(param_1,param_3,0x501,0x0,0x0);
                        return 0x0;
                    }
                    if (param_3 < 0x1a0) {
                        DAT_004d5594 = FUN_0049bf80(param_1,param_3,0x501,0x0,0x0);
                        return 0x0;
                    }
                    if (param_3 < 0x1bc8) {
                        return 0x0;
                    }
                    if (param_3 < 0x1bc9) {
                        if (param_4 != 0x0) {
                            DAT_004d559c._0_1_ = (byte)DAT_004d559c | 0x4;
                            FUN_0049bf80(param_1,0x1bcb,0x40f,0x0,0x0);
                            return 0x0;
                        }
                        DAT_004d559c._0_1_ = (byte)DAT_004d559c & 0xfb;
                        FUN_0049bf80(param_1,0x1bcb,0x502,0x0,0x0);
                        FUN_0049bf80(param_1,0x1bcb,0x410,0x0,0x0);
                        return 0x0;
                    }
                    if (param_3 != 0x1bcb) {
                        return 0x0;
                    }
                    if (param_4 != 0x0) {
                        DAT_004d559c._0_1_ = (byte)DAT_004d559c | 0x8;
                        return 0x0;
                    }
                    DAT_004d559c._0_1_ = (byte)DAT_004d559c & 0xf7;
                    return 0x0;
                }
            }
            iVar2 = param_3 - 0x136;
            puVar3 = FUN_0049bf80(param_1,param_3,0x501,0x0,0x0);
            (&DAT_00569b4c + iVar2 * 0x1e22) = puVar3;
            for (local_30 = 0x0; local_30 < 0x10; local_30 = local_30 + 0x1) {
                (&DAT_00569b68)[local_30 * 0x3 + iVar2 * 0x1e22] =
                    (&DAT_004d7410)[local_30 + iVar2 * 0xa0 + *(&DAT_00569b4c + iVar2 * 0x1e22) * 0x20];
                (&DAT_00569b98)[local_30 * 0x3 + iVar2 * 0x1e22] =
                    (&DAT_004d7420)[local_30 + *(&DAT_00569b4c + iVar2 * 0x1e22) * 0x20 + iVar2 * 0xa0];
            }
        }
        else {
            if (param_2 < 0x411) {
                if (param_2 == 0x40c) {
                    FUN_004953d7();
                    FUN_004a08c5(s_pcx_cathed3_pcx_004c2663,0x0,0x0,0x280,0x1e0,0x0,0x0,0x0,0x1);
                    FUN_0049536f();
                    return 0x1;
                }
            }
            else {
                if (param_2 < 0x412) {
                    return 0x1;
                }
                if (param_2 == 0x412) {
                    if ((param_3 < 0x136) || (0x13a < param_3)) {
                        return 0x0;
                    }
                    FUN_004953d7();
                    FUN_004968e7(*param_4,param_4[0x1],0x64,0x10,0xe);
                    bVar8 = 0x11;
                    uVar6 = 0xcaccce;
                    iVar5 = -0x1;
                    uVar4 = 0xcaccce;
                    iVar2 = param_4[0x2];
                    puVar7 = LPCSTR_005b9218;
                    pcVar1 = FUN_00499050(DAT_005967a0,param_4[0x5] + 0x190);
                    FUN_00497567((param_4[0x2] >> 0x1) + *param_4,param_4[0x1],pcVar1,iVar2,uVar4,iVar5,uVar6,puVar7,
                                 bVar8);
                    FUN_0049536f();
                }
            }
        }
    }
    return 0x0;
}



fn FUN_00480f18()

{
    let mut iVar1: i32;

    FUN_004953d7();
    FUN_00496a10();
    FUN_0049536f();
    iVar1 = FUN_004a9800(&DAT_004d5f50,s__________004c2673,0x8);
    if (iVar1 == 0x0) {
        FUN_00430bce(s_galaxy_gal_004c267c);
        FUN_00478d5b(DAT_00596a38);
        FUN_00469575();
        FUN_0046f15b();
        FUN_0047d42c();
        FUN_00488257();
        DAT_004c9754 = 0x0;
    }
    else {
        FUN_00430bce(&DAT_004d5f50);
        FUN_00478d5b(DAT_00596a38);
    }
    return;
}



fn FUN_00480faf(param_1: i32,param_2: u32,param_3: i32,param_4: i32)

{
    byte *pbVar1;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut uVar5: u32;
    let mut uVar6: u32;
    let mut iVar7: i32;
    let mut local_20: i32;
    let mut local_1c: u32;

    iVar4 = param_2 + shield_radius_00599d9c;
    iVar2 = shield_radius_00599d9c * -0x2;
    iVar3 = shield_radius_00599d9c * 0x2;
    for (local_20 = param_2 - shield_radius_00599d9c; local_20 <= iVar4; local_20 = local_20 + 0x1) {
        uVar5 = FUN_0043a8a2(local_20);
        for (local_1c = (param_3 + iVar2 + (uVar5 & 0x1)) - (param_2 & 0x1); local_1c <= param_3 + iVar3;
            local_1c = local_1c + 0x2) {
            uVar6 = FUN_0043a8d5(uVar5,local_1c);
            iVar7 = FUN_0044a87f(param_2,param_3,uVar5,uVar6);
            if (iVar7 <= shield_radius_00599d9c) {
                if (param_4 == 0x0) {
                    pbVar1 = (*(&DAT_004d7d50 + uVar5 * 0x4 + param_1 * 0x3890) + uVar6 * 0x4 + 0x4);
                    *pbVar1 = *pbVar1 & 0xbf;
                }
                else {
                    pbVar1 = (*(&DAT_004d7d50 + uVar5 * 0x4 + param_1 * 0x3890) + uVar6 * 0x4 + 0x4);
                    *pbVar1 = *pbVar1 | 0x40;
                }
            }
        }
    }
    return;
}



fn FUN_004810d1()

{
    let mut local_18: i32;
    let local_14: *mut u32;

    for (local_18 = 0x0; local_18 < 0x28; local_18 = local_18 + 0x1) {
        *(&DAT_005b89f8 + local_18 * 0x4) = 0x0;
    }
    for (local_14 = *DAT_005967c8; local_14 != 0x0; local_14 = *local_14) {
        if (*(&DAT_005b89f8 + (*(local_14 + 0x6) >> 0x10) * 0x4) == 0x0) {
            (&DAT_005b89f8 + (*(local_14 + 0x6) >> 0x10) * 0x4) = local_14;
        }
    }
    return;
}



fn FUN_00481152(param_1: i32)

{
    i32 **ppiVar1;
    let local_14: *mut i32;;

    local_14 = *(&DAT_005b89f8 + param_1 * 0x4);
    while ((local_14 != 0x0 && (*(local_14 + 0x6) >> 0x10 == param_1))) {
        ppiVar1 = FUN_004811e6(local_14);
        local_14 = *ppiVar1;
    }
    local_14 = *(&DAT_005b89f8 + param_1 * 0x4);
    while ((local_14 != 0x0 && (*(local_14 + 0x6) >> 0x10 == param_1))) {
        ppiVar1 = FUN_004811e6(local_14);
        local_14 = *ppiVar1;
    }
    return;
}



fn FUN_004811e6(param_1: *mut i32) -> i32

{
byte *pbVar1;
let mut iVar2: i32;

iVar2 = param_1[0x1];
if ((param_1 + 0xe) == 0x11) {
FUN_00480faf(*(param_1 + 0x6) >> 0x10,param_1[0x2] >> 0x10,*(param_1 + 0xa) >> 0x10,0x0);
}
if ((param_1 + 0x7) == -0x1) {
pbVar1 = ((*(param_1 + 0xa) >> 0x10) * 0x4 +
*(&DAT_004d7d50 +
(*(param_1 + 0x6) >> 0x10) * 0x3890 + (param_1[0x2] >> 0x10) * 0x4) + 0x4);
*pbVar1 = *pbVar1 & 0x7f;
*param_1[0x1] = *param_1;
if (*param_1 != 0x0) {
*(*param_1 + 0x4) = param_1[0x1];
}
FUN_0049af50(param_1);
FUN_004810d1();
}
else {
*(param_1 + 0xe) = *(param_1 + 0x7);
*(param_1 + 0x7) = 0xffff;
*(param_1 + 0x4) = 0xd;
*(param_1 + 0x12) = 0xd;
*(param_1 + 0x5) = 0xffff;
*(param_1 + 0x16) = 0xffff;
*(param_1 + 0x6) = 0xffff;
*(param_1 + 0xa) = 0x64;
}
return iVar2;
}



fn FUN_0048130e()

{
    i32 **ppiVar1;
    let local_14: *mut i32;;

    local_14 = *DAT_005967c8;
    while (local_14 != 0x0) {
        ppiVar1 = FUN_004811e6(local_14);
        local_14 = *ppiVar1;
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0048134d(undefined2 param_1,undefined2 param_2,undefined2 param_3)

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

    local_1c = param_1;
    local_1a = param_2;
    local_18 = param_3;
    local_14 = *DAT_005967c8;
    loop {
    if (local_14 == 0x0) {
        return;
    }
    iVar2 = 0x6;
    bVar5 = false;
    iVar1 = 0x0;
    bVar6 = true;
    puVar3 = &local_1c;
    piVar4 = (local_14 + 0x2);
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
    if (iVar1 == 0x0) {
        FUN_004811e6(local_14);
        _DAT_00596a3c = 0x0;
        return;
    }
    local_14 = *local_14;
} while( true );
}



fn FUN_004813ca()

{
    let puVar1: *mut u32;
    let mut iVar2: i32;
    let mut iVar3: i32;
    byte *pbVar4;
    i32 **ppiVar5;
    i32 **ppiVar6;
    let mut bVar7: bool;
    let mut bVar8: bool;
    let mut in_stack_00000038: i32;
    i32 **local_18;
    i32 **local_14;

    local_14 = FUN_004a2831(0x3a);
    ppiVar5 = &stack0x00000004;
    ppiVar6 = local_14 + 0x2;
    for (iVar2 = 0xc; iVar2 != 0x0; iVar2 = iVar2 + -0x1) {
        *ppiVar6 = *ppiVar5;
        ppiVar5 = ppiVar5 + 0x1;
        ppiVar6 = ppiVar6 + 0x1;
    }
        *ppiVar6 = *ppiVar5;
    local_18 = DAT_005967c8;
    loop {
    if (*local_18 == 0x0) {
        if (*local_18 == 0x0) {
            *local_14 = 0x0;
            local_14[0x1] = local_18;
            *local_14[0x1] = local_14;
        }^
        // goto LAB_004814a7;
    }
    local_18 = *local_18;
    iVar3 = 0x6;
    bVar7 = false;
    iVar2 = 0x0;
    bVar8 = true;
    pbVar4 = &stack0x00000004;
    ppiVar5 = local_18 + 0x2;
    loop {
        if (iVar3 == 0x0) break;
        iVar3 = iVar3 + -0x1;
        bVar7 = *pbVar4 < *ppiVar5;
        bVar8 = *pbVar4 == *ppiVar5;
        pbVar4 = pbVar4 + 0x1;
        ppiVar5 = (ppiVar5 + 0x1);
    } while (bVar8);
    if (!bVar8) {
        iVar2 = (0x1 - bVar7) - (bVar7 != 0x0);
    }
    if (iVar2 == 0x0) {
        ppiVar5 = &stack0x00000004;
        ppiVar6 = local_18 + 0x2;
        for (iVar2 = 0xc; iVar2 != 0x0; iVar2 = iVar2 + -0x1) {
            *ppiVar6 = *ppiVar5;
            ppiVar5 = ppiVar5 + 0x1;
            ppiVar6 = ppiVar6 + 0x1;
        }
            *ppiVar6 = *ppiVar5;
        local_14 = local_18;^
        // goto LAB_004814a7;
    }
} while (-0x1 < iVar2);
    *local_14 = local_18;
    local_14[0x1] = local_18[0x1];
    *local_18[0x1] = local_14;
    local_18[0x1] = local_14;
    LAB_004814a7:
    if ((local_14 + 0xe) == 0x11) {
        FUN_00480faf(*(local_14 + 0x6) >> 0x10,local_14[0x2] >> 0x10,*(local_14 + 0xa) >> 0x10,
                     0x1);
    }
    if (in_stack_00000038 != 0x0) {
        local_14[0xb] = 0x0;
        if ((local_14 + 0x12) == -0x1) {
            *(local_14 + 0x12) = *(local_14 + 0x4);
        }
        if ((local_14 + 0x4) != (local_14 + 0x12)) {
            *(local_14 + 0x2d) = *(local_14 + 0x2d) | 0x1;
        }
        iVar2 = FUN_00482bd5(*(local_14 + 0xe) >> 0x10);
        FUN_00482992((local_14 + 0x2),iVar2);
    }
    pbVar4 = (*(&DAT_004d7d50 +
        (local_14[0x2] >> 0x10) * 0x4 + (*(local_14 + 0x6) >> 0x10) * 0x3890) +
        (*(local_14 + 0xa) >> 0x10) * 0x4 + 0x4);
    *pbVar4 = *pbVar4 | 0x80;
    puVar1 = ((*(local_14 + 0xa) >> 0x10) * 0x4 +
        *(&DAT_004d7d50 +
            (*(local_14 + 0x6) >> 0x10) * 0x3890 + (local_14[0x2] >> 0x10) * 0x4) + 0x4
    );
    *puVar1 = *puVar1 | *(&DAT_004be9b0 + (*(local_14 + 0xe) >> 0x10) * 0x4);
    local_14[0xb] =
        (local_14[0xb] | *(&DAT_004be9b0 + (*(local_14 + 0xe) >> 0x10) * 0x4));
    *(local_14 + 0xe) = 0x0;
    FUN_004810d1();
    return;
}



fn FUN_004815e3() -> *mut u32

{
    let puVar1: *mut u32;
    let mut local_14: i32;

    puVar1 = FUN_004a2831(0x3a);
    *puVar1 = 0x0;
    puVar1[0x1] = 0x0;
    for (local_14 = 0x0; local_14 < 0x28; local_14 = local_14 + 0x1) {
        *(&DAT_005b89f8 + local_14 * 0x4) = 0x0;
    }
    return puVar1;
}



fn FUN_00481647(param_1: i32) -> i32

{
let local_18: *mut u32;
let mut local_14: i32;

local_14 = 0x0;
for (local_18 = (&DAT_005b89f8 + param_1 * 0x4);
(local_18 != 0x0 && (*(local_18 + 0x6) >> 0x10 == param_1));
local_18 = *local_18) {
local_14 = local_14 + 0x1;
}
return local_14;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_004816a2(param_1: i32,param_2: i32,param_3: i32)

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    undefined2 *puVar3;
    i32 **ppiVar4;
    let mut bVar5: bool;
    let mut bVar6: bool;
    let local_1c: u16;
    let local_1a: u16;
    let local_18: u16;
    i32 **local_14;

    local_1c = param_1;
    local_1a = param_2;
    local_18 = param_3;
    local_14 = *(i32 ***)(&DAT_005b89f8 + param_1 * 0x4);
    loop {
    if (local_14 == 0x0) {
        _DAT_00596a3c = 0x0;
        return;
    }
    if (*(local_14 + 0x6) >> 0x10 != param_1) {
        _DAT_00596a3c = 0x0;
        return;
    }
    iVar2 = 0x6;
    bVar5 = false;
    iVar1 = 0x0;
    bVar6 = true;
    puVar3 = &local_1c;
    ppiVar4 = local_14 + 0x2;
    loop {
        if (iVar2 == 0x0) break;
        iVar2 = iVar2 + -0x1;
        bVar5 = *puVar3 < *ppiVar4;
        bVar6 = *puVar3 == *ppiVar4;
        puVar3 = (puVar3 + 0x1);
        ppiVar4 = (ppiVar4 + 0x1);
    } while (bVar6);
    if (!bVar6) {
        iVar1 = (0x1 - bVar5) - (bVar5 != 0x0);
    }
    if (iVar1 == 0x0) {
        if ((((*local_14 == 0x0) || (*(&DAT_00583208 + ((*local_14)[0x3] >> 0x10) * 0x50) == 0x0)) ||
            ((*local_14)[0x2] >> 0x10 != param_2)) || (*(*local_14 + 0xa) >> 0x10 != param_3)) {
            _DAT_00596a3c = local_14;
        }
        else {
            _DAT_00596a3c = *local_14;
        }
        return;
    }
    local_14 = *local_14;
} while( true );
}



fn FUN_00481784(param_1: i32,param_2: i32,param_3: i32) -> *mut u32

{
    let local_14: *mut u32;

    local_14 = (&DAT_005b89f8 + param_1 * 0x4);
    while( true ) {
        if ((local_14 == 0x0) || (*(local_14 + 0x6) >> 0x10 != param_1)) {
            return 0x0;
        }
        if ((local_14[0x2] >> 0x10 == param_2) && (*(local_14 + 0xa) >> 0x10 == param_3)) break;
        local_14 = *local_14;
    }
    return local_14;
}



fn FUN_004817f9(param_1: i32,param_2: i32,short *param_3)

{
    let puVar1: *mut u32;
    let mut puVar2: *mut u8;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let mut local_24: i32;

    puVar1 = FUN_0049c2c9(0x780);
    FUN_004953d7();
    FUN_004968e7(param_1,param_2,0x30,0x28,0xe);
    FUN_004968e7(param_1 + 0x30,param_2,0x36,0xa,0xe);
    FUN_00401010(puVar1,
                 (*(&DAT_004daab0 + *param_3 * 0x3890) * 0x940 + 0x565c90 +
                     (*(param_3 + 0x2) >> 0x10) * 0x4),0x30,0x28,0x1c);
    FUN_00496ee6(puVar1,param_1,param_2,0x30,0x28);
    local_24 = (*(param_3 + 0xf) >> 0x10) - (*(param_3 + 0x17) >> 0x10);
    if (local_24 < 0x0) {
        local_24 = 0x0;
    }
    iVar5 = 0x0;
    iVar4 = 0x0;
    iVar3 = 0xe0e0e;
    puVar2 = FUN_0048f614(local_24,0x64,0x21,0x42,0x64);
    FUN_0048f678(param_1,param_2 + 0x26,0x30,0x2,local_24,0x64,puVar2,iVar3,iVar4,iVar5);
    FUN_00497567(param_1 + 0x32,param_2,(&DAT_005831e8 + (*(param_3 + 0x2) >> 0x10) * 0x50),0x62,0xcaccce,-0x1
                 ,0x272727,DAT_004d6a6c,0x11);
    if (*(param_3 + 0x3) >> 0x10 == DAT_004c9754) {
        if ((*(param_3 + 0x12) & 0x10) == 0x0) {
            if ((*(param_3 + 0x12) & 0x20) != 0x0) {
                FUN_00497567(param_1 + 0x8,param_2 + 0x2,&DAT_004c2689,0x6,0xdedede,-0x1,0xdedede,DAT_004d6a6c,0x10);
            }
        }
        else {
            FUN_00497567(param_1 + 0x8,param_2 + 0x2,&DAT_004c2687,0x6,0xe7e7e7,-0x1,0xe7e7e7,DAT_004d6a6c,0x10);
        }
        if ((*(param_3 + 0x12) & 0x8) != 0x0) {
            FUN_00497567(param_1 + 0x2,param_2 + 0x2,&DAT_004c268b,0x6,0xe7e7e7,-0x1,0xe7e7e7,DAT_004d6a6c,0x10);
        }
    }
    FUN_0049536f();
    FUN_0049af50(puVar1);
    return;
}



fn FUN_00481a44(param_1: i32) -> u32

{
    let mut local_14: u32;

    if (param_1 == 0x0) {
        local_14 = 0x0;
    }
    else {
        local_14 = *(&DAT_004be930 + (*(param_1 + 0xc) >> 0x10) * 0x4);
    }
    return local_14;
}



fn FUN_00481a7f(param_1: i32,param_2: i32,param_3: i32) -> i32

{
let mut iVar1: i32;
let mut local_18: i32;
let local_14: *mut u32;

local_18 = 0x0;
for (local_14 = (&DAT_005b89f8 + param_1 * 0x4);
(local_14 != 0x0 && (*(local_14 + 0x6) >> 0x10 == param_1));
local_14 = *local_14) {
iVar1 = FUN_00481a44(local_14);
if (((iVar1 == 0x0) && ((*(&DAT_004be9b0 + param_2 * 0x4) & local_14[0xb]) == 0x0)) &&
((*(&DAT_004be9b0 + param_3 * 0x4) & local_14[0xb]) != 0x0)) {
local_18 = local_18 + 0x1;
}
}
return local_18;
}



fn FUN_00481b16(param_1: i32,param_2: i32,param_3: i32) -> i32

{
let mut local_18: i32;
let local_14: *mut u32;

local_18 = 0x0;
for (local_14 = (&DAT_005b89f8 + param_1 * 0x4);
(local_14 != 0x0 && (*(local_14 + 0x6) >> 0x10 == param_1));
local_14 = *local_14) {
if ((((local_14 + 0xe) == 0x17) &&
((*(&DAT_004be9b0 + param_2 * 0x4) & local_14[0xb]) == 0x0)) &&
((*(&DAT_004be9b0 + param_3 * 0x4) & local_14[0xb]) != 0x0)) {
local_18 = local_18 + 0x1;
}
}
return local_18;
}



fn FUN_00481ba7(param_1: i32,param_2: i32,param_3: *mut i32,param_4: *mut i32) -> u32

{
    let local_14: *mut u32;

    local_14 = (&DAT_005b89f8 + param_1 * 0x4);
    loop {
    if ((local_14 == 0x0) || (*(local_14 + 0x6) >> 0x10 != param_1)) {
        return 0x0;
    }
    if ((local_14 + 0xe) == 0x1) {
        if (param_2 == -0x1) {
            *param_3 = local_14[0x2] >> 0x10;
            *param_4 = *(local_14 + 0xa) >> 0x10;
            return 0x1;
        }
        if (*(local_14 + 0xe) >> 0x10 == param_2) {
            *param_3 = local_14[0x2] >> 0x10;
            *param_4 = *(local_14 + 0xa) >> 0x10;
            return 0x1;
        }
    }
    local_14 = *local_14;
} while( true );
}



fn FUN_00481c66(param_1: i32) -> i32

{
let local_1c: *mut u32;
let mut local_18: i32;
let mut local_14: i32;

local_18 = 0x0;
local_1c = (&DAT_005b89f8 + param_1 * 0x4);
while( true ) {
if ((local_1c == 0x0) || (*(local_1c + 0x6) >> 0x10 != param_1))^ // goto LAB_00481d1a;
if ((*(local_1c + 0xe) >> 0x10 == DAT_004c9754) &&
(((local_1c + 0xe) == 0x16 &&
(local_18 = local_18 + ((local_1c + 0x8) * (*(local_1c + 0x26) >> 0x10)) / 0x3e8,
0xa < local_18)))) break;
local_1c = *local_1c;
}
local_18 = 0xa;
// LAB_00481d1a:
local_14 = local_18 * 0x3;
if (((&DAT_00569cd2)[DAT_004c9754 * 0x1e22] & 0x1) != 0x0) {
local_14 = local_18 * 0x5;
}
return local_14;
}



fn FUN_00481d4f(param_1: i32) -> i32

{
let local_18: *mut u32;
let mut local_14: i32;

local_14 = 0x0;
local_18 = (&DAT_005b89f8 + param_1 * 0x4);
while( true ) {
if ((local_18 == 0x0) || (*(local_18 + 0x6) >> 0x10 != param_1)) {
return local_14;
}
if (((*(local_18 + 0xe) >> 0x10 == DAT_004c9754) && ((local_18 + 0xe) == 0x16)) &&
(local_14 = local_14 + ((local_18 + 0x8) * (*(local_18 + 0x26) >> 0x10)) / 0x3e8,
0xa < local_14)) break;
local_18 = *local_18;
}
return 0xa;
}



fn FUN_00481e14(param_1: i32,param_2: i32,param_3: i32) -> u32

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    i32 **local_2c;

    iVar1 = (*(param_1 + 0x4) + 0x20);
    local_2c = *(i32 ***)(&DAT_005b89f8 + iVar1 * 0x4);
    loop {
    if ((local_2c == 0x0) || (*(local_2c + 0x6) >> 0x10 != iVar1)) {
        return 0x1;
    }
    if ((local_2c[0x2] >> 0x10 == param_2) && (*(local_2c + 0xa) >> 0x10 == param_3)) {
        if ((DAT_004c9754 != 0x5) && (((local_2c + 0xe) == 0x4 && ((local_2c + 0x4) == 0x5)))) {
            return 0x4;
        }
        if (((*(local_2c + 0xb) & 0x4) == 0x0) &&
            (((local_2c + 0xe) == 0xf || ((local_2c + 0xe) == 0x10)))) {
            FUN_0046f354(local_2c);
            *(local_2c + 0xb) = *(local_2c + 0xb) | 0x4;
            return 0x29a;
        }
        iVar2 = FUN_00481a44(local_2c);
        if ((iVar2 != 0x0) || ((local_2c + 0xe) == 0x2)) {
            return 0x1;
        }
        if (*(local_2c + 0xe) >> 0x10 != DAT_004c9754) {
            iVar1 = FUN_0044ace5(iVar1,param_2,param_3,0x1);
            if (iVar1 == -0x1) {
                return 0x3;
            }
            return 0x0;
        }
        if (((*(*(param_1 + 0x4) + 0x32) >> 0x18 != local_2c[0x4] >> 0x10) &&
            (0x9 < local_2c[0x4] >> 0x10)) && (local_2c[0x4] >> 0x10 < 0xd)) {
            return 0x6;
        }
    }
    local_2c = *local_2c;
} while( true );
}



fn FUN_00481fde(param_1: i32,param_2: i32,param_3: i32)

{
    let mut iVar1: i32;
    let mut pcVar2: String;
    u32 local_e0 [0x20];
    let mut local_60: i32;
    let mut local_5c: i32;
    let mut local_58: i32;
    let mut local_54: u32;
    let mut local_50: i32;
    let mut local_4c: i32;
    let mut local_48: *mut u8;
    let mut local_44: i32;
    let mut local_40: u32;
    let mut local_3c: i32;
    let mut local_38: i32;
    let mut local_34: *mut u8;
    let mut local_30: i32;
    i32 **local_2c;
    let mut local_28: i32;
    let mut local_24: i32;
    ushort *local_20;
    ushort *local_1c;
    let mut local_18: u32;
    let mut local_14: u32;

    local_24 = param_1;
    local_20 = (*(param_1 + 0x4) + 0x20);
    local_18 = local_20 & 0xffff0000 | *local_20;
    local_28 = *local_20;
    local_2c = *(i32 ***)(&DAT_005b89f8 + local_28 * 0x4);
    local_1c = local_20;
    local_14 = local_18;
    loop {
    if (local_2c == 0x0) {
        return;
    }
    if (*(local_2c + 0x6) >> 0x10 != local_28) {
        return;
    }
    if (((local_2c[0x2] >> 0x10 == param_2) && (*(local_2c + 0xa) >> 0x10 == param_3)) &&
        (iVar1 = FUN_00481a44(local_2c), iVar1 == 0x0)) {
        if (((local_2c + 0xe) == 0x4) && (DAT_004c9754 != 0x5)) {
            local_3c = *(local_2c + 0xe) >> 0x10;
            local_34 = &DAT_004d55a8;
            local_38 = DAT_004c9754;
            local_40 = (byte)(&DAT_004d55a8)[DAT_004c9754 * 0xe + local_3c];
            local_30 = local_3c;
            if (local_40 != 0x2) {
                FUN_004112d1(DAT_004c9754,*(local_2c + 0xe) >> 0x10);
            }
            FUN_004811e6(local_2c);
            if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) != 0x0) {
                return;
            }
            FUN_00430418(0x6978,0x5dc,0x0);
            return;
        }
        if (((local_2c + 0xe) == 0x7) || ((local_2c + 0xe) == 0xa)) {
            local_50 = *(local_2c + 0xe) >> 0x10;
            local_48 = &DAT_004d55a8;
            local_4c = DAT_004c9754;
            local_54 = (byte)(&DAT_004d55a8)[DAT_004c9754 * 0xe + local_50];
            local_44 = local_50;
            if (local_54 != 0x2) {
                FUN_004112d1(DAT_004c9754,*(local_2c + 0xe) >> 0x10);
            }
            FUN_004811e6(local_2c);
            if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) != 0x0) {
                return;
            }
            FUN_00430418(0x6978,0x5dc,0x0);
            return;
        }
        if (*(local_2c + 0xe) >> 0x10 != DAT_004c9754) {
            local_58 = *(local_2c + 0xe) >> 0x10;
            *(local_2c + 0x6) = 0xffff;
            *(local_2c + 0x22) = 0xffff;
            *(local_2c + 0x9) = 0xffff;
            *(local_2c + 0x5) = 0xffff;
            *(local_2c + 0x16) = 0xffff;
            *(local_2c + 0x26) = 0xffff;
            *(local_2c + 0x4) = (undefined2)DAT_004c9754;
            local_5c = param_1;
            iVar1 = *(*(param_1 + 0x4) + 0x32);
            local_60 = iVar1 >> 0x18;
            (local_2c + 0x12) = (iVar1 >> 0x18);
            iVar1 = FUN_00482bd5(DAT_004c9754);
            FUN_00482992((local_2c + 0x2),iVar1);
            if ((local_2c + 0x12) == -0x1) {
                *(local_2c + 0x12) = *(local_2c + 0x4);
            }
            if ((local_2c + 0x12) == (local_2c + 0x4)) {
                *(local_2c + 0x2d) = *(local_2c + 0x2d) & 0xfe;
            }
            else {
                *(local_2c + 0x2d) = *(local_2c + 0x2d) | 0x1;
            }
            if (DAT_004c9754 == 0x7) {
                pcVar2 = FUN_00499050(DAT_0059679c,0x73d0);
                FUN_0049c2e0(local_e0,pcVar2);
                FUN_0045518a(0x1f,0xffffffff,0x73d0,*(local_2c + 0x6) >> 0x10,local_e0,0xffffffff,0x0);
            }
            else {
                if ((&DAT_004d55a8)[DAT_004c9754 * 0xe + local_58] != '\x02') {
                    if ((local_2c + 0xe) == 0x17) {
                        if (DAT_004c9754 != 0x6) {
                            FUN_004112d1(DAT_004c9754,local_58);
                        }
                    }
                    else {
                        FUN_004112d1(DAT_004c9754,local_58);
                    }
                }
                if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
                    FUN_00430418(0x6979,0x5dc,0x0);
                }
            }
            return;
        }
    }
    local_2c = *local_2c;
} while( true );
}



fn FUN_004823c9(param_1: i32,param_2: i32,param_3: i32,param_4: *mut i32) -> *mut u32

{
    let mut iVar1: i32;
    let local_1c: *mut u32;
    let mut local_18: i32;
    let local_14: *mut u32;

    local_18 = 0xffff;
    local_14 = 0x0;
    for (local_1c = (&DAT_005b89f8 + param_1 * 0x4);
        (local_1c != 0x0 && (*(local_1c + 0x6) >> 0x10 == param_1));
        local_1c = *local_1c) {
        iVar1 = FUN_00481a44(local_1c);
        if ((iVar1 == 0x0) &&
            ((*(local_1c + 0xe) >> 0x10 == DAT_004c9754 &&
                (iVar1 = FUN_0044a87f(param_2,param_3,local_1c[0x2] >> 0x10,*(local_1c + 0xa) >> 0x10),
                 iVar1 < local_18)))) {
            local_14 = local_1c + 0x2;
            local_18 = iVar1;
        }
    }
        *param_4 = local_18;
    return local_14;
}



fn FUN_00482491()

{
    let mut iVar1: i32;
    let local_18: *mut u32;
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0x28; local_14 = local_14 + 0x1) {
        FUN_004a0430(&DAT_004daab4 + local_14 * 0x3890,0xff,0xb2c);
    }
    for (local_18 = *DAT_005967c8; local_18 != 0x0; local_18 = *local_18) {
        iVar1 = FUN_00481a44(local_18);
        if (iVar1 == 0x0) {
            if ((local_18 + 0x7) != -0x1) {
                (&DAT_004daab4)
                    [(*(local_18 + 0xa) >> 0x10) +
                    (*(local_18 + 0x6) >> 0x10) * 0x3890 + (local_18[0x2] >> 0x10) * 0x41] =
                    (local_18 + 0x7);
            }
        }
        else {
            (&DAT_004daab4)
                [(*(local_18 + 0xa) >> 0x10) +
                (*(local_18 + 0x6) >> 0x10) * 0x3890 + (local_18[0x2] >> 0x10) * 0x41] =
                (local_18 + 0xe);
        }
    }
    return;
}



fn FUN_00482588(param_1: i32,param_2: i32) -> u32

{
    let local_14: *mut u32;

    for (local_14 = (&DAT_005b89f8 + param_1 * 0x4);
        (local_14 != 0x0 && (*(local_14 + 0x6) >> 0x10 == param_1));
        local_14 = *local_14) {
        if (local_14[0x3] >> 0x10 == param_2) {
            return 0x1;
        }
    }
    return 0x0;
}



fn FUN_004825ee(param_1: i32) -> i32

{
let local_14: *mut u32;

for (local_14 = (&DAT_005b89f8 + param_1 * 0x4);
(local_14 != 0x0 && (*(local_14 + 0x6) >> 0x10 == param_1));
local_14 = *local_14) {
if ((local_14 + 0xe) == 0x0) {
return *(local_14 + 0xe) >> 0x10;
}
}
return -0x1;
}



fn FUN_00482655(param_1: i32) -> i32

{
let local_14: *mut u32;

for (local_14 = (&DAT_005b89f8 + param_1 * 0x4);
(local_14 != 0x0 && (*(local_14 + 0x6) >> 0x10 == param_1));
local_14 = *local_14) {
if ((local_14 + 0xe) == 0x1) {
return *(local_14 + 0xe) >> 0x10;
}
}
return -0x1;
}



fn FUN_004826bc(param_1: i32) -> i32

{
let local_14: *mut u32;

for (local_14 = (&DAT_005b89f8 + param_1 * 0x4);
(local_14 != 0x0 && (*(local_14 + 0x6) >> 0x10 == param_1));
local_14 = *local_14) {
if ((local_14 + 0xe) == 0x1) {
if (*(local_14 + 0xe) >> 0x10 == DAT_004c9754) {
return local_14[0xa] >> 0x10;
}
return -0x1;
}
}
return -0x1;
}



fn FUN_0048273d(param_1: i32,undefined2 param_2)

{
    let local_14: *mut u32;

    for (local_14 = (&DAT_005b89f8 + param_1 * 0x4);
        (local_14 != 0x0 && (*(local_14 + 0x6) >> 0x10 == param_1));
        local_14 = *local_14) {
        if ((local_14 + 0xe) == 0x1) {
            *(local_14 + 0x2a) = param_2;
        }
    }
    return;
}



fn FUN_00482796(param_1: i32) -> u32

{
    let mut local_18: u32;
    let local_14: *mut u32;

    if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x40) == 0x0) {
        for (local_14 = (&DAT_005b89f8 + param_1 * 0x4);
            (local_14 != 0x0 && (*(local_14 + 0x6) >> 0x10 == param_1));
            local_14 = *local_14) {
            if (((local_14 + 0x4) == 0x5) && ((local_14 + 0xe) == 0x4)) {
                if ((*(&DAT_004be9b0 + DAT_004c9754 * 0x4) & local_14[0xb]) != 0x0) {
                    return 0x1;
                }
                return 0x0;
            }
        }
        local_18 = 0x0;
    }
    else {
        local_18 = 0xffffffff;
    }
    return local_18;
}



fn FUN_00482843(param_1: u32,param_2: u32,param_3: i32,param_4: i32,param_5: *mut u8)

{
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut local_20: i32;
    let mut local_1c: u32;

    for (local_20 = param_2 - param_4; local_20 <= (param_2 + param_4); local_20 = local_20 + 0x1) {
        uVar1 = FUN_0043a8a2(local_20);
        for (local_1c = (param_3 + param_4 * -0x2 + (uVar1 & 0x1)) - (param_2 & 0x1);
            local_1c <= param_3 + param_4 * 0x2; local_1c = local_1c + 0x2) {
            uVar2 = FUN_0043a8d5(uVar1,local_1c);
            iVar3 = FUN_0044a87f(param_2,param_3,uVar1,uVar2);
            if (iVar3 <= param_4) {
                (param_5)(param_1,uVar1,uVar2);
            }
        }
    }
    return;
}



fn FUN_00482919(param_1: i32,param_2: i32)

{
    let mut iVar1: i32;

    iVar1 = param_2 + (*(param_1 + 0x16) >> 0x10);
    if (iVar1 < 0x65) {
        (param_1 + 0x18) = iVar1;
    }
    else {
        *(param_1 + 0x18) = 0x64;
    }
    return;
}



fn FUN_00482954(param_1: i32,param_2: i32)

{
    let mut iVar1: i32;

    iVar1 = (*(param_1 + 0x16) >> 0x10) - param_2;
    if (iVar1 < 0x0) {
        *(param_1 + 0x18) = 0x0;
    }
    else {
        (param_1 + 0x18) = iVar1;
    }
    return;
}



fn FUN_00482992(param_1: i32,param_2: i32)

{
    if (param_2 < 0x65) {
        if (param_2 < 0x0) {
            param_2 = 0x0;
        }
    }
    else {
        param_2 = 0x64;
    }
    (param_1 + 0x18) = param_2;
    return;
}



fn FUN_004829cc()

{
    let mut bVar1: bool;
    let piVar2: *mut i32;;
    let local_24: *mut u32;
    let local_20: *mut u32;

    piVar2 = FUN_0049c4bd(s_misc_citydump_txt_004c2690,&DAT_004c268d);
    if (piVar2 != 0x0) {
        for (local_24 = *DAT_005967c8; local_24 != 0x0; local_24 = *local_24) {
            bVar1 = false;
            for (local_20 = (&DAT_005b8b44 + (*(local_24 + 0x6) >> 0x10) * 0x4);
                (local_20 != 0x0 && ((local_20 + 0x8) == (local_24 + 0x2)));
                local_20 = *local_20) {
                if (((local_20 + 0x22) == (local_24 + 0xa)) &&
                    ((local_20 + 0x9) == (local_24 + 0x3))) {
                    bVar1 = true;
                    FUN_00499050(DAT_0059679c,(*(local_24 + 0xe) >> 0x10) + 0x414);
                    FUN_004a9b70(piVar2,s__20s__15s__20s__s_004c26a2);
                }
            }
            if (bVar1) {
                FUN_004a9b70(piVar2,s__________________________________004c26b5);
            }
        }
        FUN_0049ca40(piVar2);
    }
    return;
}



fn FUN_00482b7e(param_1: i32,param_2: i32)

{
    let local_14: *mut u32;

    for (local_14 = (&DAT_005b89f8 + param_1 * 0x4);
        (local_14 != 0x0 && (*(local_14 + 0x6) >> 0x10 == param_1));
        local_14 = *local_14) {
        local_14[0xb] = local_14[0xb] | *(&DAT_004be9b0 + param_2 * 0x4);
    }
    return;
}



fn FUN_00482bd5(param_1: i32) -> i32

{
let mut local_14: i32;

local_14 = (*(&DAT_00569b66 + param_1 * 0x1e22) >> 0x18) + 0x4b +
(*(&DAT_00569b6f + param_1 * 0x1e22) >> 0x18) + (*(&DAT_00569ba2 + param_1 * 0x1e22) >> 0x18)
;
if (((&DAT_00569a98)[param_1 * 0x1e22] & 0x10) != 0x0) {
local_14 = local_14 - excommunicate_loyalty_hit_0058291c;
}
local_14 = local_14 + (dflt_tax_rate_00599d64 - *(&DAT_00569adc + param_1 * 0x1e22)) * 0x3;
if (local_14 < 0x0) {
local_14 = 0x0;
}
else {
if (0x64 < local_14) {
local_14 = 0x64;
}
}
return local_14;
}



fn FUN_00482c8a(param_1: i32)

{
    let mut iVar1: i32;
    let local_18: *mut u32;

    iVar1 = FUN_00482bd5(param_1);
    for (local_18 = *DAT_005967c8; local_18 != 0x0; local_18 = *local_18) {
        if (*(local_18 + 0xe) >> 0x10 == param_1) {
            FUN_00482992((local_18 + 0x2),iVar1);
        }
    }
    return;
}



fn FUN_00482cea(param_1: i32,param_2: i32,param_3: i32)

{
    let local_14: *mut u32;

    for (local_14 = *DAT_005967b0; local_14 != 0x0; local_14 = *local_14) {
        if (*(local_14 + 0x23) >> 0x18 == param_1) {
            if (param_3 == 0x0) {
                FUN_00486b30((local_14 + 0x8),param_2);
            }
            else {
                FUN_00486af8((local_14 + 0x8),param_2);
            }
        }
    }
    return;
}



fn FUN_00482d56(param_1: i32,param_2: i32)

{
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    local_1c = 0x0;
    local_18 = 0x2;
    for (local_14 = 0x1; local_14 < 0x72; local_14 = local_14 + 0x1) {
        if (param_2 == *(&DAT_0058ad5e + local_14 * 0xda)) {
            local_1c = local_14;
            (&DAT_00569c30)[local_14 * 0x9 + param_1 * 0x1e22] = (&DAT_00569c30)[local_14 * 0x9 + param_1 * 0x1e22] | 0x1;
        }
    }
    local_14 = 0x1;
    while ((local_14 < 0x72 && (local_18 != 0x0))) {
        if ((((*(&DAT_0058ad72 + local_14 * 0xda) == 0x0) && (*(&DAT_0058ad5e + local_14 * 0xda) == local_1c))
            && (*(local_14 * 0xda + 0x58ad62) == 0x0)) && (*(local_14 * 0xda + 0x58ad66) == 0x0)) {
            (&DAT_00569c30)[param_1 * 0x1e22 + local_14 * 0x9] = (&DAT_00569c30)[param_1 * 0x1e22 + local_14 * 0x9] | 0x1;
            local_18 = local_18 + -0x1;
        }
        local_14 = local_14 + 0x1;
    }
    FUN_00463f7b();
    return;
}



fn FUN_00482e50(param_1: i32)

{
    let puVar1: *mut u32;
    let local_28: *mut u32;
    let mut local_14: i32;

    local_14 = 0x0;
    loop {
    if (0xf < local_14) {
        return;
    }
    (&DAT_00569b69)[param_1 * 0x1e22 + local_14 * 0x3] = 0x0;
    (&DAT_00569b9a)[local_14 * 0x3 + param_1 * 0x1e22] = 0x0;
    if (((&DAT_00569b68)[local_14 * 0x3 + param_1 * 0x1e22] & 0x1) != 0x0) {
        switch(local_14) {
            case 0x0:
                FUN_00482cea(param_1,0xa,0x1);
            (&DAT_00569b69)[param_1 * 0x1e22 + local_14 * 0x3] = (&DAT_00569b69)[param_1 * 0x1e22 + local_14 * 0x3] + '\n';
            break;
            case 0x1:
                (&DAT_00569b69)[local_14 * 0x3 + param_1 * 0x1e22] = (&DAT_00569b69)[local_14 * 0x3 + param_1 * 0x1e22] + '\n';
            break;
            case 0x2:
                (&DAT_00569b69)[local_14 * 0x3 + param_1 * 0x1e22] = (&DAT_00569b69)[local_14 * 0x3 + param_1 * 0x1e22] + '\n';
            break;
            case 0x3:
                FUN_00482cea(param_1,0xa,0x1);
            (&DAT_00569b69)[param_1 * 0x1e22 + local_14 * 0x3] = (&DAT_00569b69)[param_1 * 0x1e22 + local_14 * 0x3] + '\n';
            break;
            case 0x4:
                (&DAT_00569b69)[local_14 * 0x3 + param_1 * 0x1e22] = 0x1;
            break;
            case 0x5:
            for (local_28 = *DAT_005967c8; local_28 != 0x0; local_28 = *local_28)
            {
                if ((((local_28 + 0xe) == 0x0) && (*(local_28 + 0xe) >> 0x10 == param_1)) &&
                    (puVar1 = FUN_00488613(*(local_28 + 0x6) >> 0x10,local_28[0x2] >> 0x10,
                                           *(local_28 + 0xa) >> 0x10,0x2d), puVar1 != 0x0)) {
                    FUN_004883c0(*(local_28 + 0x6) >> 0x10,param_1,0x14);
                    break;
                }
            }
            break;
            case 0x6:
                (&DAT_00569b69)[local_14 * 0x3 + param_1 * 0x1e22] = (&DAT_00569b69)[local_14 * 0x3 + param_1 * 0x1e22] + '\n';
            break;
            case 0x7:
                (&DAT_00569b69)[local_14 * 0x3 + param_1 * 0x1e22] = (&DAT_00569b69)[local_14 * 0x3 + param_1 * 0x1e22] + -0xa;
            (&DAT_00569b6a)[param_1 * 0x1e22 + local_14 * 0x3] = (&DAT_00569b6a)[param_1 * 0x1e22 + local_14 * 0x3] + '\n';
            break;
            case 0x8:
                FUN_0042fcf9((&DAT_00574fe0 + param_1 * 0x4),0x14);
            break;
            case 0x9:
                FUN_0042fcf9((&DAT_005731be + param_1 * 0x4),0x14);
            break;
            case 0xa:
                FUN_00482d56(param_1,0x3de);
            break;
            case 0xb:
                FUN_00482d56(param_1,0x3df);
            break;
            case 0xc:
                FUN_00482d56(param_1,0x3e0);
            break;
            case 0xd:
                (&DAT_00569b69)[local_14 * 0x3 + param_1 * 0x1e22] = (&DAT_00569b69)[local_14 * 0x3 + param_1 * 0x1e22] + -0xa;
            break;
            case 0xe:
                (&DAT_00569b69)[local_14 * 0x3 + param_1 * 0x1e22] = (&DAT_00569b69)[local_14 * 0x3 + param_1 * 0x1e22] + '\x14'
            ;
        }
    }
    if (((&DAT_00569b98)[local_14 * 0x3 + param_1 * 0x1e22] & 0x1) != 0x0) {
        switch(local_14) {
            case 0x0:
                (&DAT_00569b99)[local_14 * 0x3 + param_1 * 0x1e22] = (&DAT_00569b99)[local_14 * 0x3 + param_1 * 0x1e22] + -0xa;
            break;
            case 0x1:
                (&DAT_00569b99)[local_14 * 0x3 + param_1 * 0x1e22] = (&DAT_00569b99)[local_14 * 0x3 + param_1 * 0x1e22] + '\n';
            break;
            case 0x2:
                (&DAT_00569b99)[local_14 * 0x3 + param_1 * 0x1e22] = (&DAT_00569b99)[local_14 * 0x3 + param_1 * 0x1e22] + -0x5;
            break;
            case 0x3:
                (&DAT_00569b99)[local_14 * 0x3 + param_1 * 0x1e22] = (&DAT_00569b99)[local_14 * 0x3 + param_1 * 0x1e22] + -0x1;
            break;
            case 0x4:
                FUN_00482cea(param_1,0xa,0x0);
            (&DAT_00569b69)[param_1 * 0x1e22 + local_14 * 0x3] = (&DAT_00569b69)[param_1 * 0x1e22 + local_14 * 0x3] + -0xa;
            break;
            case 0x5:
                (&DAT_00569b99)[local_14 * 0x3 + param_1 * 0x1e22] = (&DAT_00569b99)[local_14 * 0x3 + param_1 * 0x1e22] + '\n';
            (&DAT_00569b9a)[param_1 * 0x1e22 + local_14 * 0x3] = (&DAT_00569b9a)[param_1 * 0x1e22 + local_14 * 0x3] + -0xa;
            break;
            case 0x6:
                (&DAT_00569b99)[local_14 * 0x3 + param_1 * 0x1e22] = (&DAT_00569b99)[local_14 * 0x3 + param_1 * 0x1e22] + -0xa;
            break;
            case 0x7:
                (&DAT_00569b99)[local_14 * 0x3 + param_1 * 0x1e22] = (&DAT_00569b99)[local_14 * 0x3 + param_1 * 0x1e22] + -0xa;
            break;
            case 0x8:
                (&DAT_00569b99)[local_14 * 0x3 + param_1 * 0x1e22] = (&DAT_00569b99)[local_14 * 0x3 + param_1 * 0x1e22] + '\n';
            (&DAT_00569b9a)[param_1 * 0x1e22 + local_14 * 0x3] = (&DAT_00569b9a)[param_1 * 0x1e22 + local_14 * 0x3] + -0xa;
            break;
            case 0x9:
                FUN_0042fd26((&DAT_00574fe0 + param_1 * 0x4),0x14);
            break;
            case 0xa:
                FUN_0042fd26((&DAT_005731be + param_1 * 0x4),0x14);
        }
    }
    local_14 = local_14 + 0x1;
} while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00483355(param_1: i32)

{
    let mut local_10c: *mut u32 [0x11];
    let ppuStack199: *mut *mut u8;;
    let mut local_1b: String;;

    if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
        _DAT_005b8b40 = param_1;
        _DAT_005b8b3c = param_1 + -0x1;
        if (((-0x1 < _DAT_005b8b3c) && (_DAT_005b8b3c < 0x65)) &&
            ((&DAT_004d5c50)[DAT_004c9754 * 0x64 + _DAT_005b8b3c] != '\0')) {
            FUN_004990e0(local_10c,0x0,s_diplo_res_004c26f3,s_Tutorial_004c26ea);
            DAT_005b8b38 = FUN_0049c2c9(0xc350);
            FUN_0049c57b(s_bin_ptrattut_bin_004c26fd,DAT_005b8b38,0xc350);
            FUN_0049bb50(local_10c,FUN_0048348e);
            FUN_0049af50(DAT_005b8b38);
            ppuStack199 = &PTR_FUN_004c3d34;
            if (local_1b != 0x0) {
                FUN_00499b30(local_10c,local_1b);
            }
            FUN_0049a1c0(local_10c,0x1);
        }
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0048348e(param_1: *mut *mut u32,param_2: u32,param_3: u32) -> u32

{
    let cVar1: u8;
    let mut pcVar2: String;
    let mut iVar3: i32;
    let mut uVar4: u32;
    let mut uVar5: u32;
    let mut iVar6: i32;
    let mut uVar7: u32;
    let puVar8: *mut u32;
    let bVar9: u8;
    let mut local_38: i32;
    let mut local_34: u32;
    let mut local_30: u32;
    let mut local_2c: u32;
    let mut local_28: u32;
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_14: u32;

    local_14 = param_2;
    if (param_2 < 0x405) {
        if (param_2 == 0x401) {
            DAT_005b8b2c = param_3;
            DAT_005b8b30 = FUN_00499050(DAT_005967a8,param_3 * 0x64 + _DAT_005b8b40);
            uVar4 = 0xffffffff;
            pcVar2 = DAT_005b8b30;
            loop {
                if (uVar4 == 0x0) break;
                uVar4 = uVar4 - 0x1;
                cVar1 = *pcVar2;
                pcVar2 = pcVar2 + 0x1;
            } while (cVar1 != '\0');
            if (~uVar4 - 0x1 < 0x12d) {
                DAT_005b8b28 = 0xa;
                _DAT_005b8b20 =
                    FUN_004a3840(DAT_005b8b30,&DAT_005b8aa0,0x168,0xa,&DAT_005b8b24,LPCSTR_005b9218,
                                 0x0);
            }
            else {
                DAT_005b8b28 = 0x14;
                _DAT_005b8b20 =
                    FUN_004a3840(DAT_005b8b30,&DAT_005b8aa0,0x168,0x14,&DAT_005b8b24,DAT_004d6a6c,0x0
                    );
            }
            DAT_005b8b34 = FUN_00499050(DAT_005967a8,(DAT_005b8b2c + 0x1) * 0x64 + _DAT_005b8b40);
            iVar3 = FUN_004a2f10(s__null__004c270e,DAT_005b8b34);
            if (iVar3 == 0x0) {
                FUN_0049bf80(param_1,0x6d,0x414,0x0,0x0);
                FUN_0049bf80(param_1,0x6d,0x410,0x0,0x0);
            }
            (&DAT_004d5c50)[DAT_004c9754 * 0x64 + _DAT_005b8b3c] = 0x0;
            FUN_0049bf80(param_1,0x6e,0x502,0x0,0x0);
            return 0x0;
        }
    }
    else {
        if (param_2 < 0x406) {
            FUN_004953d7();
            if (DAT_005b8b28 == 0x14) {
                local_1c = ((0x14 - _DAT_005b8b20) / 0x2 + 0x1) * 0xa;
                for (local_20 = 0x0; local_20 < _DAT_005b8b20; local_20 = local_20 + 0x1) {
                    FUN_00497567(0x1c2,local_20 * 0xc + local_1c,*(&DAT_005b8aa0 + local_20 * 0x6),
                                 *(&DAT_005b8aa2 + local_20 * 0x6) >> 0x10,0xcaccce,-0x1,0xcaccce,DAT_004d6a6c,0x1);
                }
            }
            else {
                local_1c = ((0xa - _DAT_005b8b20) / 0x2 + 0x1) * 0x14;
                for (local_24 = 0x0; local_24 < _DAT_005b8b20; local_24 = local_24 + 0x1) {
                    FUN_00497567(0x1c2,local_24 * 0x14 + local_1c,*(&DAT_005b8aa0 + local_24 * 0x6),
                                 *(&DAT_005b8aa2 + local_24 * 0x6) >> 0x10,0xcaccce,-0x1,0xcaccce,
                                 LPCSTR_005b9218,0x1);
                }
            }
            FUN_00496ac0(DAT_005b8b38,0xa,0xa,0xfa,0xc8);
            FUN_0049e640(0xa,0xa,0xfa,0xdc,0xce,0xca,0xcc,0x1);
            FUN_004968e7(0xa,0xd2,0xfa,0x14,0xe);
            bVar9 = 0x11;
            uVar7 = 0xcaccce;
            iVar6 = 0xe0e0e;
            uVar5 = 0xcaccce;
            iVar3 = 0xf0;
            puVar8 = LPCSTR_005b9218;
            pcVar2 = FUN_00499050(DAT_005967ac,0x2710);
            FUN_00497567(0x82,0xd5,pcVar2,iVar3,uVar5,iVar6,uVar7,puVar8,bVar9);
            FUN_0049536f();
            return 0x1;
        }
        if (0x406 < param_2) {
            if (param_2 < 0x408) {
                local_28 = param_3;
                if (param_3 < 0x6d) {
                    if (param_3 == 0x65) {
                        FUN_0049c140(param_1,0x1);
                    }
                }
                else {
                    if (param_3 < 0x6e) {
                        DAT_005b8b34 = FUN_00499050(DAT_005967a8,(DAT_005b8b2c + 0x1) * 0x64 + _DAT_005b8b40);
                        iVar3 = FUN_004a2f10(s__null__004c2715,DAT_005b8b34);
                        if (iVar3 == 0x0) {
                            return 0x0;
                        }
                        DAT_005b8b2c = DAT_005b8b2c + 0x1;
                        FUN_0049a770(param_1,0x401,DAT_005b8b2c,0x0);
                        local_38 = 0x10e;
                        local_34 = 0xa;
                        local_30 = 0x168;
                        local_2c = 0xdc;
                        FUN_00498a5b(&local_38);
                        FUN_0049a770(param_1,0x405,0x1,&local_38);
                        FUN_00498ae4();
                        return 0x0;
                    }
                    if (((param_3 < 0x6f) || (param_3 == 0x7d0)) &&
                        ((&DAT_004d5c50)[DAT_004c9754 * 0x64 + _DAT_005b8b3c] =
                             (&DAT_004d5c50)[DAT_004c9754 * 0x64 + _DAT_005b8b3c] ^ 0x1, param_3 == 0x7d0)) {
                        FUN_0049bf80(param_1,0x6e,0x502,(byte)(&DAT_004d5c50)[DAT_004c9754 * 0x64 + _DAT_005b8b3c],0x0);
                    }
                }
                return 0x1;
            }
            if (param_2 == 0x411) {
                return 0x1;
            }
        }
    }
    return 0x0;
}



fn FUN_00483978()

{
    let mut iVar1: i32;

    if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
        iVar1 = FUN_00432aca(&DAT_005967b8,0x35);
        if (iVar1 != 0x0) {
            FUN_00483355(0x6);
        }
        iVar1 = FUN_00432aca(&DAT_005967b8,0x2d);
        if (iVar1 != 0x0) {
            FUN_00483355(0x7);
        }
        iVar1 = FUN_00432aca(&DAT_005967b8,0x33);
        if (iVar1 != 0x0) {
            FUN_00483355(0x8);
        }
        iVar1 = FUN_00432aca(&DAT_005967b8,0x1a);
        if (iVar1 != 0x0) {
            FUN_00483355(0x9);
        }
        iVar1 = FUN_00432aca(&DAT_005967b8,0x13);
        if (iVar1 != 0x0) {
            FUN_00483355(0xa);
        }
        iVar1 = FUN_00432aca(&DAT_005967b8,0x58);
        if (iVar1 != 0x0) {
            FUN_00483355(0xb);
        }
        iVar1 = FUN_00432aca(&DAT_005967b8,0x31);
        if (iVar1 != 0x0) {
            FUN_00483355(0xc);
        }
        iVar1 = FUN_00432cec(&DAT_005967b8);
        if (iVar1 != 0x0) {
            FUN_00483355(0xd);
        }
        iVar1 = FUN_00434d1f(&DAT_005967b8);
        if (iVar1 != 0x0) {
            FUN_00483355(0xe);
        }
        iVar1 = FUN_00432d43(&DAT_005967b8,0x1);
        if (iVar1 != 0x0) {
            FUN_00483355(0xf);
        }
        iVar1 = FUN_00432d43(&DAT_005967b8,0x0);
        if (iVar1 != 0x0) {
            FUN_00483355(0x10);
        }
        iVar1 = FUN_0043548c(&DAT_005967b8);
        if (iVar1 != 0x0) {
            FUN_00483355(0x11);
        }
        iVar1 = FUN_004354d8(&DAT_005967b8);
        if (iVar1 != 0x0) {
            FUN_00483355(0x12);
        }
    }
    return;
}



fn FUN_00483b22(param_1: i32)

{
    if (((((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) && (param_1 != 0x0)) &&
        ((*(&DAT_004be9b0 + DAT_004c9754 * 0x4) & *(param_1 + 0x2c)) != 0x0)) {
        switch(*(param_1 + 0xe)) {
            case 0x0:
                FUN_00483355(0x33);
            break;
            case 0x1:
                FUN_00483355(0x34);
            break;
            case 0x2:
                case 0xf:
                case 0x10:
                FUN_00483355(0x13);
            break;
            case 0x3:
                FUN_00483355(0x15);
            break;
            case 0x4:
            if ((param_1 + 0x10) == 0x5) {
                FUN_00483355(0x19);
            }
            break;
            case 0x5:
                FUN_00483355(0x32);
            break;
            case 0x6:
                FUN_00483355(0x35);
            break;
            case 0x7:
                FUN_00483355(0x14);
            break;
            case 0x8:
                FUN_00483355(0x36);
            break;
            case 0x9:
                FUN_00483355(0x37);
            break;
            case 0xb:
                FUN_00483355(0x38);
            break;
            case 0xc:
                FUN_00483355(0x39);
            break;
            case 0xd:
                FUN_00483355(0x16);
            break;
            case 0xe:
                FUN_00483355(0x17);
            break;
            case 0x11:
                FUN_00483355(0x3a);
            break;
            case 0x12:
                FUN_00483355(0x3b);
            break;
            case 0x13:
                FUN_00483355(0x3c);
            break;
            case 0x14:
                FUN_00483355(0x3d);
            break;
            case 0x16:
                FUN_00483355(0x22);
            break;
            case 0x18:
                FUN_00483355(0x23);
            break;
            case 0x19:
                FUN_00483355(0x24);
            break;
            case 0x1a:
                case 0x1b:
                case 0x1c:
                case 0x1d:
                case 0x1e:
                case 0x1f:
                FUN_00483355(0x18);
        }
        if ((*(param_1 + 0x2c) & 0x8) != 0x0) {
            FUN_00483355(0x11);
        }
        if ((*(param_1 + 0x2c) & 0x10) != 0x0) {
            FUN_00483355(0x12);
        }
    }
    return;
}



fn FUN_00483da3() -> u32

{
    i32 **local_18;
    let mut local_14: i32;

    for (local_18 = *DAT_005967b0; local_18 != 0x0; local_18 = *local_18) {
        if (*(&DAT_005b8b44 + (local_18 + 0x8) * 0x4) == 0x0) {
            return 0x0;
        }
        if (*local_18 != 0x0) {
            if ((*local_18 + 0x8) < (local_18 + 0x8)) {
                return 0x0;
            }
            if ((*local_18 + 0x8) == (local_18 + 0x8)) {
                if ((*local_18 + 0x22) < (local_18 + 0x22)) {
                    return 0x0;
                }
                if (((*local_18 + 0x22) == (local_18 + 0x22)) &&
                    ((*local_18 + 0x9) < (local_18 + 0x9))) {
                    return 0x0;
                }
            }
        }
    }
    local_14 = 0x0;
    loop {
    if (0x27 < local_14) {
        return 0x1;
    }
    if (*(&DAT_005b8b44 + local_14 * 0x4) != 0x0) {
        if ((*(&DAT_005b8b44 + local_14 * 0x4) + 0x20) != local_14) {
            return 0x0;
        }
        if (((*(*(&DAT_005b8b44 + local_14 * 0x4) + 0x4) != 0x0) &&
            (*(i32 ***)(*(&DAT_005b8b44 + local_14 * 0x4) + 0x4) != DAT_005967b0)) &&
            ((*(*(&DAT_005b8b44 + local_14 * 0x4) + 0x4) + 0x20) == local_14)) {
            return 0x0;
        }
    }
    local_14 = local_14 + 0x1;
} while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_004840cd(param_1: *mut u32,param_2: *mut u32,param_3: i32) -> *mut u32

{
    let puVar1: *mut u32;
    let mut local_14: i32;

    _DAT_005b8c10 = _DAT_005b8c10 + 0x1;
    for (local_14 = 0x0; *(&DAT_005b8be8 + local_14 * 0x4) != 0x0; local_14 = local_14 + 0x1) {
    }
        (&DAT_005b8be8 + local_14 * 0x4) = param_1;
    puVar1 = FUN_00484134(param_1,param_3);
    *param_2 = puVar1;
    return param_1;
}



fn FUN_00484134(param_1: *mut u32,param_2: i32) -> *mut u32

{
    let local_14: *mut u32;

    if (param_2 == -0x1) {
        local_14 = *DAT_005967b0;
    }
    else {
        local_14 = (&DAT_005b8b44 + param_2 * 0x4);
    }
    if (local_14 == 0x0) {
        *param_1 = 0x0;
    }
    else {
        *param_1 = *local_14;
    }
    return local_14;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0048418d(param_1: i32) -> i32

{
let mut local_14: i32;

_DAT_005b8c10 = _DAT_005b8c10 + -0x1;
local_14 = 0x0;
while( true ) {
if (0x9 < local_14) {
return param_1;
}
if (*(&DAT_005b8be8 + local_14 * 0x4) == param_1) break;
local_14 = local_14 + 0x1;
}
*(&DAT_005b8be8 + local_14 * 0x4) = 0x0;
return param_1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_004841ea(i32 **param_1,param_2: i32,param_3: i32,param_4: i32)

{
    let piVar1: *mut i32;;
    i32 **ppiVar2;
    let mut iVar3: i32;
    i32 **local_188;
    i32 **local_184;
    let mut local_11c: i32;
    let mut local_88: i32;

    piVar1 = *param_1;
    iVar3 = (param_1 + 0x8);
    (param_1 + 0x8) = param_2;
    (param_1 + 0x22) = param_3;
    (param_1 + 0x9) = param_4;
    if ((param_1[0x1] == DAT_005967b0) ||
    ((((param_1[0x1] + 0x8) <= param_2 &&
        (((param_1[0x1] + 0x8) != param_2 || ((param_1[0x1] + 0x22) <= param_3)))) &&
        (((param_1[0x1] + 0x8) != param_2 ||
            (((param_1[0x1] + 0x22) != param_3 || ((param_1[0x1] + 0x9) <= param_4)))))))) {
    while ((*param_1 != 0x0 &&
        ((((*param_1 + 0x8) < param_2 ||
            (((*param_1 + 0x8) == param_2 && ((*param_1 + 0x22) < param_3)))) ||
            (((*param_1 + 0x8) == param_2 &&
                (((*param_1 + 0x22) == param_3 && ((*param_1 + 0x9) < param_4))))))))) {
        if (_DAT_005b8c10 != 0x0) {
            for (local_11c = 0x0; local_11c < 0xa; local_11c = local_11c + 0x1) {
                if ((*(&DAT_005b8be8 + local_11c * 0x4) != 0x0) &&
                    (**(i32 ***)(&DAT_005b8be8 + local_11c * 0x4) == param_1)) {
                    **(i32 ***)(&DAT_005b8be8 + local_11c * 0x4) = *param_1;
                }
            }
        }
        ppiVar2 = *param_1;
        *param_1 = *ppiVar2;
        if (*ppiVar2 != 0x0) {
            (*ppiVar2)[0x1] = param_1;
        }
        if (param_1[0x1] != 0x0) {
            *param_1[0x1] = ppiVar2;
        }
        ppiVar2[0x1] = param_1[0x1];
        param_1[0x1] = ppiVar2;
        *ppiVar2 = param_1;
    }
}
    else {
    loop {
        ppiVar2 = param_1[0x1];
        if (_DAT_005b8c10 != 0x0) {
            for (local_88 = 0x0; local_88 < 0xa; local_88 = local_88 + 0x1) {
                if ((*(&DAT_005b8be8 + local_88 * 0x4) != 0x0) &&
                    (**(i32 ***)(&DAT_005b8be8 + local_88 * 0x4) == ppiVar2)) {
                    **(i32 ***)(&DAT_005b8be8 + local_88 * 0x4) = param_1;
                }
            }
        }
        param_1[0x1] = ppiVar2[0x1];
        if (ppiVar2[0x1] != 0x0) {
            *ppiVar2[0x1] = param_1;
        }
        if (*param_1 != 0x0) {
            (*param_1)[0x1] = ppiVar2;
        }
        *ppiVar2 = *param_1;
        *param_1 = ppiVar2;
        ppiVar2[0x1] = param_1;
    } while ((param_1[0x1] != DAT_005967b0) &&
        (((param_2 < (param_1[0x1] + 0x8) ||
            (((param_1[0x1] + 0x8) == param_2 && (param_3 < (param_1[0x1] + 0x22))))) ||
            (((param_1[0x1] + 0x8) == param_2 &&
                (((param_1[0x1] + 0x22) == param_3 && (param_4 < (param_1[0x1] + 0x9)))))))));
}
    if (((param_1 + 0x8) != iVar3) && ((-0x1 < iVar3 && (*(i32 ***)(&DAT_005b8b44 + iVar3 * 0x4) == param_1))))
    {
        if ((piVar1 == 0x0) || ((piVar1 + 0x8) != iVar3)) {
            *(&DAT_005b8b44 + iVar3 * 0x4) = 0x0;
        }
        else {
            *(&DAT_005b8b44 + iVar3 * 0x4) = piVar1;
        }
    }
    if (-0x1 < (param_1 + 0x8)) {
        if ((param_1[0x1] == 0x0) || (param_1[0x1] == DAT_005967b0)) {
        if (param_1 == *DAT_005967b0) {
        *(i32 ***)(&DAT_005b8b44 + (param_1 + 0x8) * 0x4) = param_1;
        }
        }
        else {
        if ((param_1 + 0x8) != (param_1[0x1] + 0x8)) {
        *(i32 ***)(&DAT_005b8b44 + (param_1 + 0x8) * 0x4) = param_1;
        }
        }
    }
    if (*(i32 ***)(&DAT_005b8b44 + (param_1 + 0x8) * 0x4) == param_1) {
        local_184 = param_1[0x1];
        local_188 = param_1;
        for (; ((local_184 != 0x0 && (local_184 != DAT_005967b0)) &&
        ((param_1 + 0x8) == (local_184 + 0x8))); local_184 = local_184[0x1]) {
            local_188 = local_184;
        }
            *(i32 ***)(&DAT_005b8b44 + (param_1 + 0x8) * 0x4) = local_188;
    }
    return;
}



fn FUN_00484ae4(param_1: i32)

{
    i32 **local_14;

    DAT_00599d3c = 0x0;
    for (local_14 = *DAT_005967b0; local_14 != 0x0; local_14 = *local_14) {
        if ((local_14 + 0x8) == param_1) {
            local_14 = FUN_00484b4e(local_14);
        }
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

i32 *  FUN_00484b4e(param_1: *mut i32)

{
let mut iVar1: i32;
let mut pcVar2: String;
let mut iVar3: i32;
let puVar4: *mut u32;
let mut local_10c: i32;
let mut local_108: i32;
let mut local_c4: i32;
let mut local_bc: i32;
let mut local_b8: u32;
let mut local_b4: i32;
let mut local_b0: u32;
let local_ac: *mut i32;;
let local_a8: *mut i32;;
let mut local_a4: i32;
let mut local_a0: u32;
byte *local_9c;
byte *local_98;
let mut local_94: u32;
let local_90: *mut i32;;
let local_8c: *mut i32;;
let mut local_88: i32;
let mut local_84: u32;
let local_80: *mut i32;;
let local_7c: *mut i32;;
let mut local_78: i32;
let mut local_74: u32;
let local_70: *mut i32;;
let local_6c: *mut i32;;
let mut local_68: i32;
let mut local_64: u32;
let local_60: *mut i32;;
let local_5c: *mut i32;;
let mut local_58: u32;
let local_54: *mut i32;;
let local_50: *mut i32;;
let mut local_4c: i32;
let mut local_48: u32;
let local_44: *mut i32;;
let local_40: *mut i32;;
let mut local_3c: i32;
let mut local_38: u32;
let local_34: *mut i32;;
let local_30: *mut i32;;
let local_2c: *mut u32;
byte *local_28;
byte *local_24;
let mut local_20: u32;
let local_1c: *mut i32;;
let local_18: *mut i32;;
let local_14: *mut i32;;

if (DAT_00599d3c == 0x0) {
if (DAT_005967bc == param_1) {
DAT_005967bc = param_1[0x2];
}
if (DAT_005967c4 == param_1) {
DAT_005967c4 = param_1[0x2];
}
if ((param_1 + 0x27) == '-') {
local_28 = ((*(param_1 + 0x23) >> 0x18) * 0x1e22 + 0x569b77);
local_20 = (*local_28 & 0x1);
local_24 = local_28;
if (local_20 != 0x0) {
local_2c = 0x0;
local_34 = param_1 + 0x8;
local_38 = local_34 & 0xffff0000 | (param_1 + 0x9);
local_3c = (param_1 + 0x9);
local_44 = param_1 + 0x8;
local_48 = local_44 & 0xffff0000 | (param_1 + 0x22);
local_4c = (param_1 + 0x22);
local_54 = param_1 + 0x8;
local_58 = local_54 & 0xffff0000 | local_54;
local_50 = local_54;
local_40 = local_44;
local_30 = local_34;
local_2c = FUN_00481784(local_54,local_4c,local_3c);
if ((local_2c != 0x0) && ((local_2c + 0xe) == 0x0)) {
local_60 = param_1 + 0x8;
local_64 = local_60 & 0xffff0000 | (param_1 + 0x9);
local_68 = (param_1 + 0x9);
local_70 = param_1 + 0x8;
local_74 = local_70 & 0xffff0000 | (param_1 + 0x22);
local_78 = (param_1 + 0x22);
local_80 = param_1 + 0x8;
local_84 = local_80 & 0xffff0000 | local_80;
local_7c = local_80;
local_6c = local_70;
local_5c = local_60;
iVar1 = FUN_00486432(0x2d,local_80,local_78,local_68);
if (iVar1 == 0x1) {
local_88 = *(param_1 + 0x23) >> 0x18;
local_90 = param_1 + 0x8;
local_94 = local_90 & 0xffff0000 | local_90;
local_8c = local_90;
FUN_004883c0(local_90,local_88,-0x14);
if ((((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) &&
(*(param_1 + 0x23) >> 0x18 == DAT_004c9754)) {
local_9c = ((*(param_1 + 0x23) >> 0x18) * 0x1e22 + 0x569b77);
local_a0 = (*local_9c & 0x10);
local_98 = local_9c;
if (local_a0 != 0x0) {
pcVar2 = FUN_00499050(DAT_0059679c,0x73f8);
FUN_0049d2e0(0x0,0x1,pcVar2);
}
}
}
}
}
}
if (_DAT_005b8c10 != 0x0) {
for (local_a4 = 0x0; local_a4 < 0xa; local_a4 = local_a4 + 0x1) {
if ((*(&DAT_005b8be8 + local_a4 * 0x4) != 0x0) &&
(**(i32 ***)(&DAT_005b8be8 + local_a4 * 0x4) == param_1)) {
**(&DAT_005b8be8 + local_a4 * 0x4) = *param_1;
}
}
}
local_18 = param_1;
local_14 = param_1[0x1];
local_ac = param_1 + 0x8;
local_b0 = *(param_1 + 0x3a) & 0x40;
local_a8 = local_ac;
if (local_b0 != 0x0) {
FUN_00431d31(&local_b8);
local_b4 = FUN_00434de1(param_1);
for (local_bc = local_b4; local_bc != 0x0; local_bc = *(local_bc + 0x8)) {
if (*(*(&DAT_00582938 +
(*(local_bc + 0x25) >> 0x18) * 0x4 + (*(local_bc + 0x24) >> 0x18) * 0x18) +
0xa5) != 0x0) {
iVar1 = *(*(&DAT_00582938 +
(*(local_bc + 0x25) >> 0x18) * 0x4 +
(*(local_bc + 0x24) >> 0x18) * 0x18) + 0xa5);
for (local_c4 = 0x0; local_c4 < iVar1; local_c4 = local_c4 + 0x1) {
if ((*(local_c4 * 0x4 + local_bc + 0x10) != 0x0) &&
(*(local_c4 * 0x4 + local_bc + 0x10) == param_1)) {
if (local_c4 < iVar1 + -0x1) {
iVar3 = local_c4 * 0x4 + local_bc;
puVar4 = (local_c4 * 0x4 + local_bc + 0x10);
*puVar4 = *(iVar3 + 0x14);
(puVar4 + 0x1) = (iVar3 + 0x18);
*(iVar1 * 0x4 + local_bc + 0xc) = 0x0;
}
else {
*(local_c4 * 0x4 + local_bc + 0x10) = 0x0;
}
}
}
}
}
}
if (*(&DAT_005b8b44 + (param_1 + 0x8) * 0x4) == param_1) {
if ((*param_1 == 0x0) || ((param_1 + 0x8) != (*param_1 + 0x20))) {
*(&DAT_005b8b44 + (param_1 + 0x8) * 0x4) = 0x0;
}
else {
*(&DAT_005b8b44 + (param_1 + 0x8) * 0x4) = *param_1;
}
}
if (param_1[0x2] != 0x0) {
*(param_1[0x2] + 0xc) = param_1[0x3];
}
if (param_1[0x3] != 0x0) {
*(param_1[0x3] + 0x8) = param_1[0x2];
}
*param_1[0x1] = *param_1;
if (*param_1 != 0x0) {
*(*param_1 + 0x4) = param_1[0x1];
}
if (*(*(&DAT_00582938 + (*(param_1 + 0x25) >> 0x18) * 0x4 + (param_1[0x9] >> 0x18) * 0x18)
+ 0xa5) != 0x0) {
for (local_108 = 0x0;
local_108 <
*(*(&DAT_00582938 +
(*(param_1 + 0x25) >> 0x18) * 0x4 + (param_1[0x9] >> 0x18) * 0x18) + 0xa5);
local_108 = local_108 + 0x1) {
if (param_1[local_108 + 0x4] != 0x0) {
FUN_00484b4e(param_1[local_108 + 0x4]);
}
}
}
if (((param_1 + 0x27) == '\x1d') && ((param_1 + 0xc) != -0x1)) {
(&DAT_004d7796)[(*(param_1 + 0x2d) >> 0x18) * 0x67] = 0xff;
(&DAT_004d7792 + (*(param_1 + 0x2d) >> 0x18) * 0x67 + 0x3) = 0xff;
if ((param_1 + 0xc) == '\x01') {
FUN_0042fd26((&DAT_00574fe0 + (*(param_1 + 0x23) >> 0x18) * 0x4),
*(&DAT_004d7730 + (*(param_1 + 0x2d) >> 0x18) * 0x67));
}
else {
if ((param_1 + 0xc) == '\t') {
for (local_10c = 0x0; local_10c < 0x5; local_10c = local_10c + 0x1) {
FUN_0042fd26((&DAT_00569b14 + (*(param_1 + 0x23) >> 0x18) * 0x4 + local_10c * 0x1e22),
*(&DAT_004d7730 + (*(param_1 + 0x2d) >> 0x18) * 0x67));
}
}
}
}
FUN_0049af50(local_18);
local_1c = local_14;
}
else {
*(param_1 + 0x3d) = *(param_1 + 0x3d) | 0x80;
_DAT_005b8be4 = 0x1;
local_1c = param_1;
}
return local_1c;
}



fn FUN_00485347() -> *mut u32

{
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let local_14: *mut u32;

    puVar2 = *DAT_005967b0;
    DAT_005967b0 = puVar2;
    while (puVar1 = DAT_005967b0, DAT_005967b0 != 0x0) {
        DAT_005967b0 = *DAT_005967b0;
        puVar2 = FUN_0049af50(puVar1);
    }
    for (local_14 = 0x0; local_14 < 0x28; local_14 = (local_14 + 0x1)) {
        *(&DAT_005b8b44 + local_14 * 0x4) = 0x0;
        puVar2 = local_14;
    }
    return puVar2;
}



fn FUN_004853b9(param_1: i32,param_2: i32,param_3: i32)

{
    i32 **local_14;

    local_14 = *DAT_005967b0;
    while( true ) {
        if (local_14 == 0x0) {
            return;
        }
        if ((((local_14 + 0x8) == param_1) && ((local_14 + 0x22) == param_2)) &&
            ((local_14 + 0x9) == param_3)) break;
        local_14 = *local_14;
    }
    FUN_00484b4e(local_14);
    return;
}



// WARNING: Restarted to delay deadcode elimination for space: stack

i32 **  FUN_00485463(short param_1,param_2: u32)

{
let mut puVar1: *mut u8;
let puVar2: *mut u32;
let mut pcVar3: String;
let mut iVar4: i32;
let mut iVar5: i32;
i32 **ppiVar6;
short *psVar7;
i32 **ppiVar8;
byte *pbVar9;
let mut bVar10: bool;
let mut bVar11: bool;
let mut in_stack_00000034: i32;
let mut in_stack_00000038: i32;
let in_stack_fffffecc: *mut i32;;
let mut in_stack_fffffed0: u32;
let mut in_stack_fffffed4: u32;
let mut local_48: i32;
i32 **local_18;
let local_14: *mut i32;;

local_18 = FUN_004a2831(0x4d);
ppiVar6 = &param_1;
ppiVar8 = local_18 + 0x8;
for (iVar4 = 0xb; iVar4 != 0x0; iVar4 = iVar4 + -0x1) {
*ppiVar8 = *ppiVar6;
ppiVar6 = ppiVar6 + 0x1;
ppiVar8 = ppiVar8 + 0x1;
}
ppiVar8 = ppiVar6;
local_18[0x2] = 0x0;
local_18[0x3] = 0x0;
local_18[0x4] = 0x0;
local_18[0x5] = 0x0;
local_18[0x6] = 0x0;
local_18[0x7] = 0x0;
iVar4 = DAT_00591cb0;
if (in_stack_00000034 != 0x0) {
DAT_00591cb0 = DAT_00591cb0 + 0x1;
*(local_18 + 0x36) = iVar4;
}
if (in_stack_00000038 == 0x0) {
ppiVar6 = local_18 + 0x8;
ppiVar8 = &stack0xfffffecc;
for (iVar4 = 0xb; iVar4 != 0x0; iVar4 = iVar4 + -0x1) {
*ppiVar8 = *ppiVar6;
ppiVar6 = ppiVar6 + 0x1;
ppiVar8 = ppiVar8 + 0x1;
}
ppiVar8 = ppiVar6;
iVar4 = FUN_004599b0(in_stack_fffffecc,in_stack_fffffed0,in_stack_fffffed4);
(local_18 + 0x10) = iVar4;
}
if (param_2._2_1_ < '\x05') {
puVar1 = &DAT_00568210 + param_1 * 0x9d + param_2._2_1_ * 0x1e22;
puVar1[0x9c] = puVar1[0x9c] & 0xfe;
puVar1[0x9c] = puVar1[0x9c] | 0x1;
}
local_14 = DAT_005967b0;
loop {
if (*local_14 == 0x0) {
if (*local_14 == 0x0) {
*local_18 = 0x0;
local_18[0x1] = local_14;
*local_18[0x1] = local_18;
}^
// goto LAB_004855ee;
}
local_14 = *local_14;
iVar5 = 0xe;
bVar10 = false;
iVar4 = 0x0;
bVar11 = true;
psVar7 = &param_1;
pbVar9 = (local_14 + 0x8);
loop {
if (iVar5 == 0x0) break;
iVar5 = iVar5 + -0x1;
bVar10 = *psVar7 < *pbVar9;
bVar11 = *psVar7 == *pbVar9;
psVar7 = (short *)(psVar7 + 0x1);
pbVar9 = pbVar9 + 0x1;
} while (bVar11);
if (!bVar11) {
iVar4 = (0x1 - bVar10) - (bVar10 != 0x0);
}
} while (0x0 < iVar4);
*local_18 = local_14;
local_18[0x1] = local_14[0x1];
*(i32 ***)local_14[0x1] = local_18;
local_14[0x1] = local_18;
// LAB_004855ee:
if (in_stack_00000038 == 0x0) {
if (((local_18 + 0x27) == '\x1d') && ((local_18 + 0xc) != -0x1)) {
(&DAT_004d7796)[(*(local_18 + 0x2d) >> 0x18) * 0x67] = *(local_18 + 0x8);
(&DAT_004d7792 + (*(local_18 + 0x2d) >> 0x18) * 0x67 + 0x3) =
(local_18 + 0x26);
if ((local_18 + 0xc) == '\x01') {
FUN_0042fcf9((&DAT_00574fe0 + (*(local_18 + 0x23) >> 0x18) * 0x4),
*(&DAT_004d7730 + (*(local_18 + 0x2d) >> 0x18) * 0x67));
}
else {
if ((local_18 + 0xc) == '\t') {
for (local_48 = 0x0; local_48 < 0x5; local_48 = local_48 + 0x1) {
FUN_0042fcf9((&DAT_00569b14 + (*(local_18 + 0x23) >> 0x18) * 0x4 + local_48 * 0x1e22),
*(&DAT_004d7730 + (*(local_18 + 0x2d) >> 0x18) * 0x67));
}
}
}
}
iVar4 = FUN_004884c0(*(local_18 + 0x23) >> 0x18,(local_18 + 0x8));
FUN_00486b6b((local_18 + 0x8),iVar4);
}
if (in_stack_00000034 != 0x0) {
FUN_00459e8f(local_18);
FUN_0044add9(local_18);
}
FUN_004841ea(local_18,(local_18 + 0x8),(local_18 + 0x22),
(local_18 + 0x9));
if ((((((local_18 + 0x27) == '-') &&
((*((*(local_18 + 0x23) >> 0x18) * 0x1e22 + 0x569b77) & 0x1) != 0x0)) &&
(puVar2 = FUN_00481784((local_18 + 0x8),(local_18 + 0x22),
(local_18 + 0x9)), puVar2 != 0x0)) &&
(((puVar2 + 0xe) == 0x0 &&
(iVar4 = FUN_00486432(0x2d,(local_18 + 0x8),(local_18 + 0x22),
(local_18 + 0x9)), iVar4 == 0x1)))) &&
((FUN_004883c0((local_18 + 0x8),*(local_18 + 0x23) >> 0x18,0x14),
((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0 &&
((*(local_18 + 0x23) >> 0x18 == DAT_004c9754 &&
((*((*(local_18 + 0x23) >> 0x18) * 0x1e22 + 0x569b77) & 0x10) != 0x0)))))) {
pcVar3 = FUN_00499050(DAT_0059679c,0x73f9);
FUN_0049d2e0(0x0,0x1,pcVar3);
}
return local_18;
}



// WARNING: Restarted to delay deadcode elimination for space: stack

i32 **  FUN_00485a91(short param_1,param_2: u32)

{
let mut puVar1: *mut u8;
let mut iVar2: i32;
i32 **ppiVar3;
i32 **ppiVar4;
let mut in_stack_00000034: i32;
let mut in_stack_00000038: i32;
i32 **in_stack_0000003c;
let in_stack_ffffff64: *mut i32;;
let mut in_stack_ffffff68: u32;
let mut in_stack_ffffff6c: u32;
i32 **local_14;

local_14 = FUN_004a2831(0x4d);
ppiVar3 = &param_1;
ppiVar4 = local_14 + 0x8;
for (iVar2 = 0xb; iVar2 != 0x0; iVar2 = iVar2 + -0x1) {
*ppiVar4 = *ppiVar3;
ppiVar3 = ppiVar3 + 0x1;
ppiVar4 = ppiVar4 + 0x1;
}
ppiVar4 = ppiVar3;
local_14[0x2] = 0x0;
local_14[0x3] = 0x0;
*local_14 = 0x0;
local_14[0x1] = 0x0;
local_14[0x4] = 0x0;
local_14[0x5] = 0x0;
local_14[0x6] = 0x0;
local_14[0x7] = 0x0;
iVar2 = DAT_00591cb0;
if (in_stack_00000034 != 0x0) {
DAT_00591cb0 = DAT_00591cb0 + 0x1;
*(local_14 + 0x36) = iVar2;
}
if (in_stack_00000038 == 0x0) {
ppiVar3 = local_14 + 0x8;
ppiVar4 = &stack0xffffff64;
for (iVar2 = 0xb; iVar2 != 0x0; iVar2 = iVar2 + -0x1) {
*ppiVar4 = *ppiVar3;
ppiVar3 = ppiVar3 + 0x1;
ppiVar4 = ppiVar4 + 0x1;
}
ppiVar4 = ppiVar3;
iVar2 = FUN_004599b0(in_stack_ffffff64,in_stack_ffffff68,in_stack_ffffff6c);
(local_14 + 0x10) = iVar2;
}
if (param_2._2_1_ < '\x05') {
puVar1 = &DAT_00568210 + param_1 * 0x9d + param_2._2_1_ * 0x1e22;
puVar1[0x9c] = puVar1[0x9c] & 0xfe;
puVar1[0x9c] = puVar1[0x9c] | 0x1;
}
if (in_stack_0000003c == 0x0) {
*DAT_005967b0 = local_14;
local_14[0x1] = DAT_005967b0;
}
else {
*in_stack_0000003c = local_14;
local_14[0x1] = in_stack_0000003c;
}
if (((in_stack_00000038 == 0x0) && ((local_14 + 0x27) == '\x1d')) && ((local_14 + 0xc) != -0x1)
) {
(&DAT_004d7796)[(*(local_14 + 0x2d) >> 0x18) * 0x67] = *(local_14 + 0x8);
(&DAT_004d7792 + (*(local_14 + 0x2d) >> 0x18) * 0x67 + 0x3) =
(local_14 + 0x26);
}
if (in_stack_00000034 != 0x0) {
FUN_00459e8f(local_14);
FUN_0044add9(local_14);
}
FUN_004841ea(local_14,(local_14 + 0x8),(local_14 + 0x22),
(local_14 + 0x9));
return local_14;
}



fn FUN_00485cd6() -> *mut u32

{
    let puVar1: *mut u32;
    let mut local_14: i32;

    puVar1 = FUN_004a2831(0x4d);
    *puVar1 = 0x0;
    puVar1[0x1] = 0x0;
    puVar1[0x2] = 0x0;
    puVar1[0x3] = 0x0;
    for (local_14 = 0x0; local_14 < 0x28; local_14 = local_14 + 0x1) {
        *(&DAT_005b8b44 + local_14 * 0x4) = 0x0;
    }
    return puVar1;
}



fn FUN_00485d4e(param_1: i32,param_2: i32,param_3: i32) -> i32

{
let local_14: *mut u32;

local_14 = *DAT_005967b0;
while( true ) {
if (local_14 == 0x0) {
return -0x1;
}
if (((((local_14 + 0x8) == param_1) && ((local_14 + 0x22) == param_2)) &&
((local_14 + 0x9) == param_3)) && ((*(local_14 + 0x3a) & 0x1) == 0x0)) break;
local_14 = *local_14;
}
return local_14[0x9] >> 0x18;
}



fn FUN_00485e3d(param_1: i32) -> i32

{
let local_18: *mut u32;
let mut local_14: i32;

local_14 = 0x0;
for (local_18 = *DAT_005967b0; local_18 != 0x0; local_18 = *local_18) {
if ((local_18 + 0x8) == param_1) {
local_14 = local_14 + 0x1;
}
}
return local_14;
}



fn FUN_00485ea2(param_1: i32,param_2: i32,param_3: i32,param_4: i32) -> i32

{
let mut local_18: i32;
let local_14: *mut u32;

local_18 = 0x0;
for (local_14 = (&DAT_005b8b44 + param_1 * 0x4);
(local_14 != 0x0 && ((local_14 + 0x8) == param_1)); local_14 = *local_14) {
if (((local_14 + 0x22) == param_2) && ((local_14 + 0x9) == param_3)) {
if (param_4 == 0x0) {
if (((*(local_14 + 0x3a) & 0x1) == 0x0) &&
((*(&DAT_004be9b0 + DAT_004c9754 * 0x4) & *(local_14 + 0x3a)) != 0x0)) {
local_18 = local_18 + 0x1;
}
}
else {
if (((*(local_14 + 0x3a) & 0x1) != 0x0) &&
((*(&DAT_004be9b0 + DAT_004c9754 * 0x4) & *(local_14 + 0x3a)) != 0x0)) {
local_18 = local_18 + 0x1;
}
}
}
}
return local_18;
}



fn FUN_00485fe3(param_1: i32) -> u32

{
    let local_14: *mut u32;

    local_14 = *DAT_005967b0;
    while( true ) {
        if (local_14 == 0x0) {
            return 0x0;
        }
        if ((((*(local_14 + 0x3a) & 0x80000000) == 0x0) && (*(local_14 + 0x23) >> 0x18 == param_1))
            && ((local_14 + 0x27) == '-')) break;
        local_14 = *local_14;
    }
    return 0x1;
}



fn FUN_00486065(undefined2 *param_1)

{
    *param_1 = 0xffff;
    param_1[0x1] = 0xffff;
    param_1[0x2] = 0xffff;
    (param_1 + 0x3) = 0xff;
    (param_1 + 0x7) = 0xff;
    (param_1 + 0x4) = 0x0;
    FUN_00486b6b(param_1,0x4b);
    param_1[0x5] = 0x0;
    (param_1 + 0xf) = 0x0;
    (param_1 + 0x7) = 0x0;
    param_1[0x6] = 0x0;
    (param_1 + 0x8) = 0xff;
    *(param_1 + 0x11) = 0x0;
    (param_1 + 0x13) = 0x64;
    (param_1 + 0xa) = 0x5;
    (param_1 + 0x15) = 0xff;
    *(param_1 + 0xb) = 0xffffffff;
    *(param_1 + 0xd) = 0x0;
    (param_1 + 0xf) = 0xff;
    (param_1 + 0x1f) = 0x0;
    (param_1 + 0x10) = 0x0;
    (param_1 + 0x21) = 0xff;
    (param_1 + 0x11) = 0xff;
    (param_1 + 0x14) = 0x0;
    param_1[0x15] = 0xfffd;
    (param_1 + 0x27) = 0x0;
    (param_1 + 0x16) = 0x0;
    return;
}



fn FUN_0048616e(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: i32)

{
    let mut local_60: i32;
    let local_14: *mut u32;

    for (local_14 = (&DAT_005b8b44 + param_1 * 0x4);
        (local_14 != 0x0 && ((local_14 + 0x8) == param_1)); local_14 = *local_14) {
        if (((*(local_14 + 0x3a) & 0x80000000) == 0x0) &&
            ((((local_14 + 0x22) == param_2 && ((local_14 + 0x9) == param_3)) &&
                ((*(local_14 + 0x3a) & 0x1) != 0x0)))) {
            if (((local_14 + 0x27) == '\x1d') && ((local_14 + 0xc) != -0x1)) {
                (&DAT_004d7796)[(*(local_14 + 0x2d) >> 0x18) * 0x67] = *(local_14 + 0x8);
                (&DAT_004d7792 + (*(local_14 + 0x2d) >> 0x18) * 0x67 + 0x3) =
                    (local_14 + 0x26);
                if ((local_14 + 0xc) == '\x01') {
                    FUN_0042fd26((&DAT_00574fe0 + (*(local_14 + 0x23) >> 0x18) * 0x4),
                                 *(&DAT_004d7730 + (*(local_14 + 0x2d) >> 0x18) * 0x67));
                    FUN_0042fcf9((&DAT_00574fe0 + param_4 * 0x4),
                                 *(&DAT_004d7730 + (*(local_14 + 0x2d) >> 0x18) * 0x67));
                }
                else {
                    if ((local_14 + 0xc) == '\t') {
                        for (local_60 = 0x0; local_60 < 0x5; local_60 = local_60 + 0x1) {
                            FUN_0042fd26((&DAT_00569b14 + (*(local_14 + 0x23) >> 0x18) * 0x4 + local_60 * 0x1e22),
                                         *(&DAT_004d7730 + (*(local_14 + 0x2d) >> 0x18) * 0x67));
                            FUN_0042fcf9((&DAT_00569b14 + param_4 * 0x4 + local_60 * 0x1e22),
                                         *(&DAT_004d7730 + (*(local_14 + 0x2d) >> 0x18) * 0x67));
                        }
                    }
                }
            }
            if ((*(local_14 + 0x3a) & 0x2) != 0x0) {
                (local_14 + 0x2f) = 0x0;
                *(local_14 + 0x3a) = *(local_14 + 0x3a) & 0xfd;
            }
            if (param_5 == -0x1) {
                param_5 = param_4;
            }
            *(local_14 + 0x3a) = *(local_14 + 0x3a) | *(&DAT_004be9b0 + param_4 * 0x4);
            (local_14 + 0x35) = param_5;
            (local_14 + 0x26) = param_4;
            if (param_5 == param_4) {
                *(local_14 + 0x3a) = *(local_14 + 0x3a) & 0x7f;
            }
            else {
                *(local_14 + 0x3a) = *(local_14 + 0x3a) | 0x80;
            }
            (local_14 + 0x12) = 0x0;
            (local_14 + 0x41) = 0xff;
            (local_14 + 0x42) = 0xff;
        }
    }
    return;
}



fn FUN_00486432(param_1: i32,param_2: i32,param_3: i32,param_4: i32) -> i32

{
let local_18: *mut u32;
let mut local_14: i32;

local_14 = 0x0;
for (local_18 = (&DAT_005b8b44 + param_2 * 0x4);
(local_18 != 0x0 && ((local_18 + 0x8) == param_2)); local_18 = *local_18) {
if (((local_18 + 0x22) == param_3) &&
(((local_18 + 0x9) == param_4 && (local_18[0x9] >> 0x18 == param_1)))) {
local_14 = local_14 + 0x1;
}
}
return local_14;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_004864f7()

{
    i32 **local_1c;
    i32 **local_18 [0x2];

    DAT_00599d3c = 0x0;
    if (_DAT_005b8be4 != 0x0) {
        _DAT_005b8be4 = 0x0;
        FUN_004840cd(local_18,&local_1c,-0x1);
        while (local_1c != 0x0) {
            if ((*(local_1c + 0x3a) & 0x80000000) != 0x0) {
                FUN_00484b4e(local_1c);
            }
            local_1c = local_18[0];
            if (local_18[0] != 0x0) {
                local_18[0] = *local_18[0];
            }
        }
        FUN_0048418d(local_18);
    }
    return;
}



fn FUN_004865ba(param_1: *mut u32) -> *mut u32

{
    let sVar1: i16;
    let sVar2: i16;
    let puVar3: *mut u32;
    let local_20: *mut u32;
    let local_14: *mut u32;

    local_20 = (&DAT_005b8b44 + param_1 * 0x4);
    puVar3 = local_20;
    loop {
    if ((local_20 == 0x0) || (puVar3 = (local_20 + 0x8), puVar3 != param_1))
    break;
    if ((*(local_20 + 0x3a) & 0x1) == 0x0) {
        local_20 = *local_20;
        puVar3 = local_20;
    }
    else {
        sVar1 = (local_20 + 0x22);
        sVar2 = (local_20 + 0x9);
        local_20[0x3] = 0x0;
        local_20[0x2] = 0x0;
        for (local_20 = *local_20; puVar3 = local_20, local_20 != 0x0;
            local_20 = *local_20) {
            if ((*(local_20 + 0x3a) & 0x1) != 0x0) {
                puVar3 = (local_20 + 0x8);
                if (((puVar3 != param_1) ||
                    (puVar3 = (local_20 + 0x22), puVar3 != sVar1)) ||
                    (puVar3 = (local_20 + 0x9), puVar3 != sVar2)) break;
                local_20[0x3] = local_20[0x1];
                (local_20[0x3] + 0x8) = local_20;
                local_20[0x2] = 0x0;
            }
        }
    }
} while( true );
    local_14 = 0x0;
    LAB_00486770:
    if (0xd < local_14) {
        return puVar3;
    }
    local_20 = (&DAT_005b8b44 + param_1 * 0x4);
    loop {
    if ((local_20 == 0x0) || ((local_20 + 0x8) != param_1)) break;
    if (((*(local_20 + 0x3a) & 0x1) == 0x0) &&
        ((*(local_20 + 0x23) >> 0x18) == local_14)) {
        sVar1 = (local_20 + 0x22);
        sVar2 = (local_20 + 0x9);
        local_20[0x3] = 0x0;
        local_20[0x2] = 0x0;
        for (local_20 = *local_20; local_20 != 0x0; local_20 = *local_20) {
            if ((*(local_20 + 0x3a) & 0x1) == 0x0) {
                if ((((local_20 + 0x8) != param_1) || ((local_20 + 0x22) != sVar1)
                ) || (((local_20 + 0x9) != sVar2 ||
                    ((*(local_20 + 0x23) >> 0x18) != local_14)))) break;
                local_20[0x3] = local_20[0x1];
                (local_20[0x3] + 0x8) = local_20;
                local_20[0x2] = 0x0;
            }
        }
    }
    else {
        local_20 = *local_20;
    }
} while( true );
    puVar3 = local_14;
    local_14 = (local_14 + 0x1);^
    // goto LAB_00486770;
}



i32 **  FUN_004869cb(param_1: i32,param_2: i32)

{
let mut iVar1: i32;
let puVar2: *mut u32;
let puVar3: *mut u32;
let in_stack_ffffff54: i16;
let mut in_stack_ffffff58: u32;
u32 local_74 [0xc];
u32 local_44 [0x4];
let local_33: i16;
i32 **local_14;

if ((param_1 + 0x27) == '[') {
if (*(param_1 + 0x2f) >> 0x10 < param_2) {
puVar2 = (param_1 + 0x20);
puVar3 = local_44;
for (iVar1 = 0xb; iVar1 != 0x0; iVar1 = iVar1 + -0x1) {
*puVar3 = *puVar2;
puVar2 = puVar2 + 0x1;
puVar3 = puVar3 + 0x1;
}
puVar3 = puVar2;
(param_1 + 0x31) = (param_1 + 0x31) - param_2;
local_33 = param_2;
puVar2 = local_44;
puVar3 = local_74;
for (iVar1 = 0xb; iVar1 != 0x0; iVar1 = iVar1 + -0x1) {
*puVar3 = *puVar2;
puVar2 = puVar2 + 0x1;
puVar3 = puVar3 + 0x1;
}
puVar3 = puVar2;
puVar2 = local_74;
puVar3 = &stack0xffffff54;
for (iVar1 = 0xb; iVar1 != 0x0; iVar1 = iVar1 + -0x1) {
*puVar3 = *puVar2;
puVar2 = puVar2 + 0x1;
puVar3 = puVar3 + 0x1;
}
puVar3 = puVar2;
local_14 = FUN_00485463(in_stack_ffffff54,in_stack_ffffff58);
}
else {
local_14 = 0x0;
}
}
else {
local_14 = 0x0;
}
return local_14;
}



fn FUN_00486a5a(param_1: i32,param_2: i32) -> u32

{
    let mut local_14: u32;

    if (((param_1 + 0x27) == '[') && ((param_2 + 0x27) == '[')) {
        if ((param_1 + 0x30) == (param_2 + 0x30)) {
            (param_2 + 0x31) = (param_2 + 0x31) + (param_1 + 0x31);
            if (*(param_2 + 0x2f) >> 0x10 < 0x3e8) {
                *(param_1 + 0x31) = 0x0;
                local_14 = 0x1;
            }
            else {
                (param_1 + 0x31) = 0x3e7 - (param_2 + 0x31);
                *(param_2 + 0x31) = 0x3e7;
                local_14 = 0xffffffff;
            }
        }
        else {
            local_14 = 0x0;
        }
    }
    else {
        local_14 = 0x0;
    }
    return local_14;
}



fn FUN_00486af8(param_1: i32,param_2: i32)

{
    let mut iVar1: i32;

    iVar1 = param_2 + (*(param_1 + 0x6) >> 0x18);
    if (iVar1 < 0x65) {
        param_2._0_1_ = iVar1;
        (param_1 + 0x9) = param_2;
    }
    else {
        (param_1 + 0x9) = 0x64;
    }
    return;
}



fn FUN_00486b30(param_1: i32,param_2: i32)

{
    let mut iVar1: i32;

    iVar1 = (*(param_1 + 0x6) >> 0x18) - param_2;
    if (iVar1 < 0x0) {
        (param_1 + 0x9) = 0x0;
    }
    else {
        param_2._0_1_ = iVar1;
        (param_1 + 0x9) = param_2;
    }
    return;
}



fn FUN_00486b6b(param_1: i32,param_2: i32)

{
    if (param_2 < 0x65) {
        if (param_2 < 0x0) {
            param_2 = 0x0;
        }
    }
    else {
        param_2 = 0x64;
    }
    (param_1 + 0x9) = param_2;
    return;
}



fn FUN_00486ba4(param_1: i32) -> *mut u32

{
    let puVar1: *mut u32;
    let mut local_854: i32;
    i32 local_838 [0x209];
    let local_14: *mut u32;

    FUN_004a0430(local_838,0x0,0x820);
    for (puVar1 = *DAT_005967b0; puVar1 != 0x0; puVar1 = *puVar1) {
        if ((((*(puVar1 + 0x3a) & 0x80000000) == 0x0) && (*(puVar1 + 0x23) >> 0x18 == param_1)) &&
            ((puVar1 + 0x27) == '[')) {
            local_838[(puVar1 + 0x8) * 0xd + (*(puVar1 + 0x2d) >> 0x18)] =
                local_838[(puVar1 + 0x8) * 0xd + (*(puVar1 + 0x2d) >> 0x18)] +
                    (*(puVar1 + 0x2f) >> 0x10);
        }
    }
    for (local_14 = 0x0; local_14 < 0x28; local_14 = (local_14 + 0x1)) {
        for (local_854 = 0x0; local_854 < 0xd; local_854 = local_854 + 0x1) {
            *(&DAT_00568210 + local_854 * 0x4 + local_14 * 0x9d + param_1 * 0x1e22) =
                local_838[local_854 + local_14 * 0xd];
        }
        puVar1 = local_14;
    }
    return puVar1;
}



fn FUN_00486d59()

{
    let sVar1: i16;
    let sVar2: i16;
    i32 **local_20;
    let mut local_14: i32;

    sVar1 = -0x1;
    sVar2 = -0x1;
    local_14 = -0x1;
    for (local_20 = *DAT_005967b0; (local_20 != 0x0 && ((*(local_20 + 0x3a) & 0x1) != 0x0));
        local_20 = *local_20) {
        if (((local_20 + 0x22) != sVar1) || ((local_20 + 0x9) != sVar2)) {
            sVar1 = (local_20 + 0x22);
            sVar2 = (local_20 + 0x9);
            local_14 = *(local_20 + 0x23) >> 0x18;
        }
        if (*(local_20 + 0x23) >> 0x18 != local_14) {
            local_20 = FUN_00484b4e(local_20);
        }
    }
    return;
}



fn FUN_00486e79(param_1: i32,param_2: *mut i32) -> i32

{
let mut uVar1: u32;
ushort local_bc;
ushort local_ba;
ushort local_b8;
let mut local_b0: i32;
ushort *local_ac;
ushort *local_a8;
let mut local_a4: u32;
let mut local_a0: u32;
let mut local_9c: u32;
let mut local_98: u32;
let mut local_94: u32;
let mut local_90: u32;
let mut local_8c: u32;
let mut local_88: u32;
let mut local_84: u32;
let mut local_80: u32;
let mut local_7c: u32;
let mut local_78: u32;
let mut local_74: i32;
let mut local_70: i32;
let mut local_6c: u32;
let mut local_68: u32;
let mut local_64: u32;
let mut local_60: u32;
let mut local_5c: u32;
let mut local_58: u32;
let mut local_54: u32;
let mut local_50: u32;
let mut local_4c: u32;
let mut local_48: u32;
let mut local_44: u32;
let mut local_40: u32;
let mut local_3c: u32;
let mut local_38: u32;
let mut local_34: u32;
let mut local_30: u32;
let mut local_2c: u32;
let mut local_28: u32;
let mut local_24: u32;
let mut local_20: u32;
let mut local_1c: u32;
let local_18: u8;
let local_14: u8;

local_b0 = 0x0;
local_ac = (param_1 + 0x20);
local_bc = *local_ac;
local_24 = local_ac & 0xffff0000 | local_bc;
local_a4 = param_1 + 0x20;
local_ba = (param_1 + 0x22);
local_20 = local_a4 & 0xffff0000 | local_ba;
local_98 = param_1 + 0x20;
local_b8 = (param_1 + 0x24);
local_1c = local_98 & 0xffff0000 | local_b8;
local_a8 = local_ac;
local_a0 = local_a4;
local_94 = local_98;
uVar1 = FUN_004a7160(&local_bc,0x2,0x1,param_2);
local_8c = (uVar1 == 0x0);
local_b0 = local_b0 + local_8c;
uVar1 = FUN_004a7160(&local_ba,0x2,0x1,param_2);
local_88 = (uVar1 == 0x0);
local_b0 = local_b0 + local_88;
uVar1 = FUN_004a7160(&local_b8,0x2,0x1,param_2);
local_84 = (uVar1 == 0x0);
local_b0 = local_b0 + local_84;
uVar1 = FUN_004a7160((param_1 + 0x26),0x1,0x1,param_2);
local_80 = (uVar1 == 0x0);
local_b0 = local_b0 + local_80;
uVar1 = FUN_004a7160((param_1 + 0x27),0x1,0x1,param_2);
local_7c = (uVar1 == 0x0);
local_b0 = local_b0 + local_7c;
uVar1 = FUN_004a7160((param_1 + 0x28),0x1,0x1,param_2);
local_78 = (uVar1 == 0x0);
local_b0 = local_b0 + local_78;
local_74 = param_1 + 0x20;
local_18 = (param_1 + 0x29);
local_70 = local_74;
local_14 = local_18;
uVar1 = FUN_004a7160(&local_18,0x1,0x1,param_2);
local_68 = (uVar1 == 0x0);
local_b0 = local_b0 + local_68;
uVar1 = FUN_004a7160((param_1 + 0x2c),0x2,0x1,param_2);
local_64 = (uVar1 == 0x0);
local_b0 = local_b0 + local_64;
uVar1 = FUN_004a7160((param_1 + 0x2e),0x1,0x1,param_2);
local_60 = (uVar1 == 0x0);
local_b0 = local_b0 + local_60;
uVar1 = FUN_004a7160((param_1 + 0x2f),0x1,0x1,param_2);
local_5c = (uVar1 == 0x0);
local_b0 = local_b0 + local_5c;
uVar1 = FUN_004a7160((param_1 + 0x30),0x1,0x1,param_2);
local_58 = (uVar1 == 0x0);
local_b0 = local_b0 + local_58;
uVar1 = FUN_004a7160((param_1 + 0x31),0x2,0x1,param_2);
local_54 = (uVar1 == 0x0);
local_b0 = local_b0 + local_54;
uVar1 = FUN_004a7160((param_1 + 0x33),0x1,0x1,param_2);
local_4c = (uVar1 == 0x0);
local_b0 = local_b0 + local_4c;
uVar1 = FUN_004a7160((param_1 + 0x34),0x1,0x1,param_2);
local_9c = (uVar1 == 0x0);
local_b0 = local_b0 + local_9c;
uVar1 = FUN_004a7160((param_1 + 0x35),0x1,0x1,param_2);
local_90 = (uVar1 == 0x0);
local_b0 = local_b0 + local_90;
uVar1 = FUN_004a7160((param_1 + 0x36),0x4,0x1,param_2);
local_40 = (uVar1 == 0x0);
local_b0 = local_b0 + local_40;
uVar1 = FUN_004a7160((param_1 + 0x3a),0x4,0x1,param_2);
local_3c = (uVar1 == 0x0);
local_b0 = local_b0 + local_3c;
uVar1 = FUN_004a7160((param_1 + 0x3e),0x1,0x1,param_2);
local_48 = (uVar1 == 0x0);
local_b0 = local_b0 + local_48;
uVar1 = FUN_004a7160((param_1 + 0x3f),0x1,0x1,param_2);
local_34 = (uVar1 == 0x0);
local_b0 = local_b0 + local_34;
uVar1 = FUN_004a7160((param_1 + 0x40),0x1,0x1,param_2);
local_30 = (uVar1 == 0x0);
local_b0 = local_b0 + local_30;
uVar1 = FUN_004a7160((param_1 + 0x41),0x1,0x1,param_2);
local_50 = (uVar1 == 0x0);
local_b0 = local_b0 + local_50;
uVar1 = FUN_004a7160((param_1 + 0x42),0x1,0x1,param_2);
local_6c = (uVar1 == 0x0);
local_b0 = local_b0 + local_6c;
uVar1 = FUN_004a7160((param_1 + 0x48),0x1,0x1,param_2);
local_44 = (uVar1 == 0x0);
local_b0 = local_b0 + local_44;
uVar1 = FUN_004a7160((param_1 + 0x49),0x1,0x1,param_2);
local_28 = (uVar1 == 0x0);
local_b0 = local_b0 + local_28;
uVar1 = FUN_004a7160((param_1 + 0x4a),0x2,0x1,param_2);
local_2c = (uVar1 == 0x0);
local_b0 = local_b0 + local_2c;
uVar1 = FUN_004a7160((param_1 + 0x4c),0x1,0x1,param_2);
local_38 = (uVar1 == 0x0);
return local_b0 + local_38;
}



fn FUN_004874bc(param_1: *mut i32,param_2: i32) -> i32

{
let mut iVar1: i32;
let mut local_28: i32;
let mut local_14: i32;

local_14 = 0x0;
for (; param_2 != 0x0; param_2 = *(param_2 + 0x8)) {
if ((*(param_2 + 0x3a) & 0x40) == 0x0) {
iVar1 = FUN_00486e79(param_2,param_1);
local_14 = local_14 + iVar1;
if (*(*(&DAT_00582938 +
(*(param_2 + 0x25) >> 0x18) * 0x4 + (*(param_2 + 0x24) >> 0x18) * 0x18) + 0xa5)
!= 0x0) {
for (local_28 = 0x0;
local_28 <
*(*(&DAT_00582938 +
(*(param_2 + 0x25) >> 0x18) * 0x4 + (*(param_2 + 0x24) >> 0x18) * 0x18) +
0xa5); local_28 = local_28 + 0x1) {
if (*(local_28 * 0x4 + param_2 + 0x10) != 0x0) {
iVar1 = FUN_00486e79(*(local_28 * 0x4 + param_2 + 0x10),param_1);
local_14 = local_14 + iVar1;
}
}
}
}
}
return local_14;
}



fn FUN_004875c2(param_1: *mut i32) -> i32

{
let mut uVar1: u32;
let mut local_2c: u32;
let mut local_28: u32;
let mut local_20: i32;
let local_1c: *mut u32;
let mut local_18: u32;
let mut local_14: u32;

local_20 = 0x0;
local_18 = 0xfffffffe;
local_14 = 0xfffffffd;
for (local_1c = *DAT_005967b0; local_1c != 0x0; local_1c = *local_1c) {
if (local_1c[0x3] == 0x0) {
FUN_004874bc(param_1,local_1c);
uVar1 = FUN_004a7160(&local_14,0x2,0x1,param_1);
local_28 = (uVar1 == 0x0);
local_20 = local_20 + local_28;
}
}
uVar1 = FUN_004a7160(&local_18,0x2,0x1,param_1);
local_2c = (uVar1 == 0x0);
return local_20 + local_2c;
}
