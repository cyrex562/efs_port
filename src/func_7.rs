
fn FUN_00411505(param_1: i32)

{
    let mut uVar1: u32;

    uVar1 = FUN_004515ca(param_1,0x3);
    *(param_1 + 0x828) = *(param_1 + 0x828) & 0xfb;
    *(param_1 + 0x828) = *(param_1 + 0x828) | ((byte)uVar1 & 0x1) << 0x2;
    uVar1 = FUN_004515ca(param_1,0xa);
    *(param_1 + 0x828) = *(param_1 + 0x828) & 0xbf;
    *(param_1 + 0x828) = *(param_1 + 0x828) | ((byte)uVar1 & 0x1) << 0x6;
    uVar1 = FUN_004515ca(param_1,0x4);
    *(param_1 + 0x828) = *(param_1 + 0x828) & 0xf7;
    *(param_1 + 0x828) = *(param_1 + 0x828) | ((byte)uVar1 & 0x1) << 0x3;
    uVar1 = FUN_004515ca(param_1,0x5);
    *(param_1 + 0x828) = *(param_1 + 0x828) & 0xef;
    *(param_1 + 0x828) = *(param_1 + 0x828) | ((byte)uVar1 & 0x1) << 0x4;
    uVar1 = FUN_004515ca(param_1,0x6);
    *(param_1 + 0x828) = *(param_1 + 0x828) & 0xdf;
    *(param_1 + 0x828) = *(param_1 + 0x828) | ((byte)uVar1 & 0x1) << 0x5;
    return;
}



fn FUN_004115e1(param_1: i32,param_2: i32,param_3: i32,param_4: i32) -> u32

{
    let sVar1: i16;
    let sVar2: i16;
    let mut iVar3: i32;
    let local_20: *mut u32;

    sVar1 = -0x1;
    sVar2 = -0x1;
    local_20 = (&DAT_005b8b44 + param_1 * 0x4);
    loop {
    if ((local_20 == 0x0) || ((local_20 + 0x8) != param_1)) {
        return 0x1;
    }
    if (((((*(local_20 + 0x3a) & 0x1) != 0x0) && ((local_20 + 0x22) != sVar1)) &&
        ((local_20 + 0x9) != sVar2)) &&
        ((*(local_20 + 0x23) >> 0x18 == param_4 && ((local_20 + 0x2a) == 0x8)))) {
        iVar3 = FUN_0044a87f((local_20 + 0x22),(local_20 + 0x9),param_2,param_3);
        if ((iVar3 != 0x0) && (iVar3 < 0x6)) {
            return 0x0;
        }
        sVar1 = (local_20 + 0x22);
        sVar2 = (local_20 + 0x9);
    }
    local_20 = *local_20;
} while( true );
}



fn FUN_004117a9(param_1: u32) -> u32

{
    let mut iVar1: i32;
    let mut pcVar2: String;
    let mut uVar3: u32;

    if (param_1 < 0x1bbd) {
        if (param_1 == 0x1bbc) {
            if ((DAT_004d559c & 0x8) != 0x0) {
                pcVar2 = FUN_00499050(DAT_0059679c,0x7427);
                FUN_0049d2e0(0x0,0x1,pcVar2);
                return 0x0;
            }
            if (DAT_00599d04 != 0x0) {
                uVar3 = DAT_004d559c & 0xfffffffd;
                iVar1 = FUN_00430bce(DAT_00599d04);
                iVar1 = FUN_0047bf4b(DAT_00596a38,iVar1,uVar3);
                if (iVar1 != 0x0) {
                    pcVar2 = FUN_00499050(DAT_0059679c,0x7395);
                    FUN_0049d2e0(0x0,0x1,pcVar2);
                    DAT_00599d04 = 0x0;
                }
                return 0x0;
            }^
            // goto LAB_0041185d;
        }
    }
    else {
        if (param_1 < 0x1bbe) {
            LAB_0041185d:
            if ((DAT_004d559c & 0x8) != 0x0) {
                pcVar2 = FUN_00499050(DAT_0059679c,0x7427);
                FUN_0049d2e0(0x0,0x1,pcVar2);
                return 0x0;
            }
            if (DAT_00599d04 == 0x0) {
                DAT_00599d04 = FUN_0049c2c9(0x104);
            }
            uVar3 = find_file_fn_0045bde0(s_sav___sav_004c08d9,DAT_00599d04,0x1);
            if (uVar3 == 0x0) {
                DAT_00599d04 = 0x0;
            }
            else {
                uVar3 = DAT_004d559c & 0xfffffffd;
                iVar1 = FUN_00430bce(DAT_00599d04);
                iVar1 = FUN_0047bf4b(DAT_00596a38,iVar1,uVar3);
                if (iVar1 != 0x0) {
                    pcVar2 = FUN_00499050(DAT_0059679c,0x7395);
                    FUN_0049d2e0(0x0,0x1,pcVar2);
                    DAT_00599d04 = 0x0;
                }
            }
            return 0x0;
        }
        if (param_1 < 0x1bbf) {
            if (DAT_00599d04 == 0x0) {
                DAT_00599d04 = FUN_0049c2c9(0x104);
            }
            uVar3 = find_file_fn_0045bde0(s_sav___sav_004c08e3,DAT_00599d04,0x0);
            if (uVar3 != 0x0) {
                FUN_0047f618();
                FUN_00431d0a(&DAT_005967b8);
                FUN_00430bce(DAT_00599d04);
                FUN_0047a593(DAT_00596a38);
                return 0x1;
            }
            return 0x0;
        }
        if (param_1 == 0x1bc9) {
            pcVar2 = FUN_00499050(DAT_0059679c,0x73e8);
            uVar3 = FUN_0049d2e0(0x0,0x3,pcVar2);
            if (uVar3 == 0x0) {
                return 0x0;
            }
            DAT_00599d04 = 0x0;
            return uVar3;
        }
    }
    return 0x0;
}



fn FUN_00411a3d(param_1: u32) -> u32

{
    let mut local_20: i32;
    let mut local_1c: i32;

    if (param_1 < 0x1d4d) {
        if (param_1 == 0x1d4c) {
            FUN_00406851();
            return 0x1;
        }
    }
    else {
        if (param_1 < 0x1d4e) {
            FUN_004553fd(0x1,0x0);
            return 0x1;
        }
        if (param_1 == 0x1d4e) {
            local_20 = 0x0;
            for (local_1c = 0x0; local_1c < 0xe; local_1c = local_1c + 0x1) {
                if (*(&DAT_004c977c + DAT_004c9754 * 0xf8 + local_1c * 0xd90) == 0x0) {
                    local_20 = local_20 + 0x1;
                    FUN_0045578c(DAT_004c9754,local_1c,0x3);
                }
            }
            if (local_20 == 0x0) {
                FUN_0049d2e0(0x0,0x1,s_Sorry__no_unresolved_contracts__004c08ed);
            }
            return 0x1;
        }
    }
    return 0x0;
}



fn FUN_00411b3a(param_1: *mut *mut u32,param_2: u32)

{
    let mut iVar1: i32;

    if (param_2 < 0x516) {
        if (param_2 == 0x515) {
            FUN_004a2d6b();
            FUN_0044b1a8();
        }
    }
    else {
        if (param_2 < 0x517) {
            FUN_004a2d6b();
            FUN_0042732a();
        }
        else {
            if (param_2 == 0x517) {
                FUN_004a2d6b();
                iVar1 = FUN_00403f41();
                if ((iVar1 != 0x0) && (*(DAT_005967bc + 0x23) >> 0x18 != DAT_004c9754)) {
                    if ((*(DAT_005967bc + 0x3a) & 0x1) == 0x0) {
                        FUN_0047830a(param_1);
                    }
                    else {
                        FUN_0044587d(param_1);
                    }
                }
            }
        }
    }
    return;
}



fn FUN_00411c3b(param_1: i32) -> u32

{
    let mut iVar1: i32;
    let mut local_18: u32;

    iVar1 = *(&DAT_00582938 + (*(param_1 + 0x25) >> 0x18) * 0x4 + (*(param_1 + 0x24) >> 0x18) * 0x18)
    ;
    if (((*(iVar1 + 0x99) == 0x0) && (*(iVar1 + 0x9d) == 0x0)) && (*(iVar1 + 0xa1) == 0x0)) {
        local_18 = 0x0;
    }
    else {
        local_18 = 0x1;
    }
    return local_18;
}



fn FUN_00411cab() -> *mut u32

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    i32 aiStackY131436 [0x7ff7];
    let mut local_184: i32;
    let mut local_180: i32;
    let local_174: *mut u32;
    let local_170: *mut u32;
    i32 local_16c [0x28];
    let local_cc: *mut u32;
    let local_c8: *mut u32;
    let local_c4: *mut u32;
    let mut local_c0: u32;
    let mut local_bc: u32;
    let mut local_b8: i32;
    let local_b4: *mut u32;
    let mut local_b0: *mut u8;
    let mut local_ac: i32;
    let local_a8: *mut u32;
    let local_a4: *mut u32;
    let mut local_a0: u32;
    let local_9c: *mut u32;
    let local_98: *mut u32;
    let mut local_94: u32;
    let local_90: *mut u32;
    let local_8c: *mut u32;
    let mut local_88: u32;
    let local_84: *mut u32;
    let local_80: *mut u32;
    let mut local_7c: u32;
    let mut local_78: u32;
    let mut local_74: i32;
    let local_70: *mut u32;
    let mut local_6c: *mut u8;
    let mut local_68: i32;
    let local_64: *mut u32;
    let local_60: *mut u32;
    let mut local_5c: u32;
    let local_58: *mut u32;
    let local_54: *mut u32;
    let mut local_50: u32;
    let local_4c: *mut u32;
    let local_48: *mut u32;
    let mut local_44: u32;
    let mut local_40: i32;
    let mut local_3c: *mut u8;
    let local_38: *mut u32;
    let mut local_34: i32;
    let mut local_30: u32;
    let mut local_2c: i32;
    let mut local_28: *mut u8;
    let local_24: *mut u32;
    let mut local_20: i32;
    let mut local_1c: u32;
    let local_18: *mut u32;
    let local_14: *mut u32;

    FUN_004a0430(&DAT_004c7108,0x0,0xa0);
    for (local_18 = *DAT_005967c8; local_18 != 0x0; local_18 = *local_18) {
        if (((local_18 + 0x4) == 0xd) && (iVar2 = FUN_00481a44(local_18), iVar2 == 0x0)) {
            (&DAT_004c7108)[(*(local_18 + 0x6) >> 0x10) * 0x4] =
                (&DAT_004c7108)[(*(local_18 + 0x6) >> 0x10) * 0x4] | 0x1;
        }
    }
    for (local_18 = *DAT_005967c8; local_18 != 0x0; local_18 = *local_18) {
        if ((*(local_18 + 0xe) >> 0x10) == DAT_004c9754) {
            (&DAT_004c7108)[(*(local_18 + 0x6) >> 0x10) * 0x4] =
                (&DAT_004c7108)[(*(local_18 + 0x6) >> 0x10) * 0x4] & 0xfe;
        }
        else {
            local_2c = *(local_18 + 0xe) >> 0x10;
            local_28 = &DAT_004d55a8;
            local_24 = DAT_004c9754;
            local_1c = (byte)(&DAT_004d55a8)[DAT_004c9754 * 0xe + local_2c];
            local_20 = local_2c;
            if (local_1c != 0x2) {
                (&DAT_004c7108)[(*(local_18 + 0x6) >> 0x10) * 0x4] =
                    (&DAT_004c7108)[(*(local_18 + 0x6) >> 0x10) * 0x4] & 0xfe;
            }
        }
    }
    for (local_18 = *DAT_005967c8; local_18 != 0x0; local_18 = *local_18) {
        iVar2 = FUN_00481a44(local_18);
        if (iVar2 == 0x0) {
            if ((*(local_18 + 0xe) >> 0x10) == DAT_004c9754) {
                (&DAT_004c7109)[(*(local_18 + 0x6) >> 0x10) * 0x4] =
                    (&DAT_004c7109)[(*(local_18 + 0x6) >> 0x10) * 0x4] | 0x4;
            }
            else {
                if ((local_18 + 0x4) == 0xd) {
                    (&DAT_004c7109)[(*(local_18 + 0x6) >> 0x10) * 0x4] =
                        (&DAT_004c7109)[(*(local_18 + 0x6) >> 0x10) * 0x4] | 0x1;
                }
                else {
                    local_40 = *(local_18 + 0xe) >> 0x10;
                    local_3c = &DAT_004d55a8;
                    local_38 = DAT_004c9754;
                    local_30 = (byte)(&DAT_004d55a8)[DAT_004c9754 * 0xe + local_40];
                    local_34 = local_40;
                    if (local_30 == 0x2) {
                        (&DAT_004c7108)[(*(local_18 + 0x6) >> 0x10) * 0x4] =
                            (&DAT_004c7108)[(*(local_18 + 0x6) >> 0x10) * 0x4] | 0x40;
                    }
                    else {
                        if (*(local_18 + 0xe) >> 0x10 < 0x5) {
                            (&DAT_004c7109)[(*(local_18 + 0x6) >> 0x10) * 0x4] =
                                (&DAT_004c7109)[(*(local_18 + 0x6) >> 0x10) * 0x4] | 0x10;
                        }
                    }
                }
            }
        }
    }
    for (local_14 = *DAT_005967b0; local_14 != 0x0; local_14 = *local_14) {
        local_4c = local_14 + 0x8;
        local_44 = *(local_14 + 0x3a) & 0x1;
        local_48 = local_4c;
        if (local_44 == 0x0) {
            local_b8 = *(local_14 + 0x23) >> 0x18;
            local_b0 = &DAT_004d55a8;
            local_b4 = DAT_004c9754;
            local_bc = (byte)(&DAT_004d55a8)[DAT_004c9754 * 0xe + local_b8];
            local_ac = local_b8;
            if ((local_bc == 0x2) && (iVar2 = FUN_00411c3b(local_14), iVar2 != 0x0)) {
                local_c8 = local_14 + 0x8;
                local_c0 = local_c8 & 0xffff0000 | local_c8;
                (&DAT_004c7109)[local_c8 * 0x4] = (&DAT_004c7109)[local_c8 * 0x4] | 0x40;
                local_c4 = local_c8;
            }
        }
        else {
            if ((*(local_14 + 0x23) >> 0x18) == DAT_004c9754) {
                local_58 = local_14 + 0x8;
                local_50 = local_58 & 0xffff0000 | local_58;
                (&DAT_004c7109)[local_58 * 0x4] = (&DAT_004c7109)[local_58 * 0x4] | 0x2;
                local_54 = local_58;
            }
            else {
                if ((local_14 + 0x26) == '\r') {
                    local_64 = local_14 + 0x8;
                    local_5c = local_64 & 0xffff0000 | local_64;
                    (&DAT_004c7108)[local_64 * 0x4] = (&DAT_004c7108)[local_64 * 0x4] | 0x80;
                    local_60 = local_64;
                }
                else {
                    local_74 = *(local_14 + 0x23) >> 0x18;
                    local_6c = &DAT_004d55a8;
                    local_70 = DAT_004c9754;
                    local_78 = (byte)(&DAT_004d55a8)[DAT_004c9754 * 0xe + local_74];
                    local_68 = local_74;
                    if (local_78 == 0x2) {
                        local_84 = local_14 + 0x8;
                        local_7c = local_84 & 0xffff0000 | local_84;
                        (&DAT_004c7108)[local_84 * 0x4] = (&DAT_004c7108)[local_84 * 0x4] | 0x20
                        ;
                        local_80 = local_84;
                        if ((local_14 + 0x2a) == 0x8) {
                            local_90 = local_14 + 0x8;
                            local_88 = local_90 & 0xffff0000 | local_90;
                            (&DAT_004c7109)[local_90 * 0x4] =
                                (&DAT_004c7109)[local_90 * 0x4] | 0x20;
                            local_8c = local_90;
                        }
                        else {
                            if ((((local_14 + 0x2a) == 0x5) || ((local_14 + 0x2a) == 0x7)) &&
                                (iVar2 = FUN_00411c3b(local_14), iVar2 != 0x0)) {
                                local_9c = local_14 + 0x8;
                                local_94 = local_9c & 0xffff0000 | local_9c;
                                (&DAT_004c7109)[local_9c * 0x4] =
                                    (&DAT_004c7109)[local_9c * 0x4] | 0x40;
                                local_98 = local_9c;
                            }
                        }
                    }
                    else {
                        if (*(local_14 + 0x23) >> 0x18 < 0x5) {
                            local_a8 = local_14 + 0x8;
                            local_a0 = local_a8 & 0xffff0000 | local_a8;
                            (&DAT_004c7109)[local_a8 * 0x4] =
                                (&DAT_004c7109)[local_a8 * 0x4] | 0x8;
                            local_a4 = local_a8;
                        }
                    }
                }
            }
        }
    }
    local_cc = DAT_004c9754;
    local_170 = DAT_004c9754;
    if (0x4 < DAT_004c9754) {
        if (DAT_004c9754 < &DAT_00000006) {
            for (local_170 = *DAT_005967c8; local_170 != 0x0; local_170 = *local_170
            ) {
                if (((local_170 + 0xe) == 0x4) && ((local_170 + 0x4) == 0x5)) {
                    (&DAT_004c7108)[(*(local_170 + 0x6) >> 0x10) * 0x4] =
                        (&DAT_004c7108)[(*(local_170 + 0x6) >> 0x10) * 0x4] | 0x10;
                }
            }
        }
        else {
            if (DAT_004c9754 == 0x7) {
                FUN_004a0430(local_16c,0x0,0xa0);
                for (local_170 = *DAT_005967c8; local_170 != 0x0;
                    local_170 = *local_170) {
                    if (((local_170 + 0x4) == 0x7) && ((local_170 + 0xe) == 0x7)) {
                        local_16c[*(local_170 + 0x6) >> 0x10] = local_16c[*(local_170 + 0x6) >> 0x10] + 0x1;
                    }
                }
                for (local_174 = 0x0; local_174 < 0x28; local_174 = (local_174 + 0x1)) {
                    if (local_16c[local_174] < 0x6) {
                        if (local_16c[local_174] < 0x4) {
                            if (0x0 < local_16c[local_174]) {
                                (&DAT_004c7108)[local_174 * 0x4] = (&DAT_004c7108)[local_174 * 0x4] | 0x8;
                            }
                        }
                        else {
                            (&DAT_004c7108)[local_174 * 0x4] = (&DAT_004c7108)[local_174 * 0x4] | 0x4;
                        }
                    }
                    else {
                        (&DAT_004c7108)[local_174 * 0x4] = (&DAT_004c7108)[local_174 * 0x4] | 0x2;
                    }
                    local_170 = local_174;
                }
                for (local_174 = 0x0; local_174 < 0x28; local_174 = (local_174 + 0x1)) {
                    if (((&DAT_004c7108)[local_174 * 0x4] & 0xe) == 0x0) {
                        iVar2 = *(&DAT_005b7076 + local_174 * 0x4e);
                        iVar1 = *(&DAT_005b7078 + local_174 * 0x4e);
                        for (local_180 = 0x0; local_180 < 0x28; local_180 = local_180 + 0x1) {
                            if (((&DAT_004c7108)[local_180 * 0x4] & 0x6) != 0x0) {
                                for (local_184 = 0x0; local_184 < 0x78; local_184 = local_184 + 0x1) {
                                    if (((((*(&DAT_005b7ca8 + local_184 * 0x14) == iVar2 >> 0x10) &&
                                        (*(&DAT_005b7cac + local_184 * 0x14) == iVar1 >> 0x10)) &&
                                        (*(&DAT_005b7076 + local_180 * 0x4e) >> 0x10 == *(&DAT_005b7cb0 + local_184 * 0x14)
                                        )) && (*(&DAT_005b7078 + local_180 * 0x4e) >> 0x10 ==
                                        *(&DAT_005b7cb4 + local_184 * 0x14))) ||
                                        (((*(&DAT_005b7cb0 + local_184 * 0x14) == iVar2 >> 0x10 &&
                                            (*(&DAT_005b7cb4 + local_184 * 0x14) == iVar1 >> 0x10)) &&
                                            ((*(&DAT_005b7076 + local_180 * 0x4e) >> 0x10 == *(&DAT_005b7ca8 + local_184 * 0x14)
                                                && (*(&DAT_005b7078 + local_180 * 0x4e) >> 0x10 ==
                                                *(&DAT_005b7cac + local_184 * 0x14))))))) {
                                        (&DAT_004c7108)[local_174 * 0x4] = (&DAT_004c7108)[local_174 * 0x4] | 0x8;
                                    }
                                }
                                if (((&DAT_004c7108)[local_180 * 0x4] & 0x8) != 0x0) break;
                            }
                        }
                    }
                    local_170 = local_174;
                }
            }
        }
    }
    return local_170;
}



fn FUN_004124b5()

{
    let local_14: *mut u32;

    for (local_14 = *DAT_005967b0; local_14 != 0x0; local_14 = *local_14) {
        if ((((*(local_14 + 0x3b) & 0x10) != 0x0) && (*(local_14 + 0x23) >> 0x18 == DAT_004c9754))
            && ((local_14 + 0x12) == '\0')) {
            (local_14 + 0x12) = 0xd;
        }
    }
    return;
}



fn FUN_00412513()

{
    i32 **local_20;
    i32 **local_1c;
    let local_18: *mut u32;

    FUN_00419085();
    FUN_0042feac();
    local_18 = FUN_004840cd(&local_1c,&local_20,-0x1);
    while (local_20 != 0x0) {
    if (((local_20 + 0x12) == '\r') && (*(local_20 + 0x23) >> 0x18 == DAT_004c9754)) {
        FUN_0041361b(local_20);
    }
    local_20 = local_1c;
    if (local_1c != 0x0) {
        local_1c = *local_1c;
    }
}
    FUN_0048418d(&local_1c);
    return;
}



fn FUN_004125af()

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let local_f4: *mut u32;
    let mut local_e4: i32;
    let local_e0: *mut u32;
    let local_d0: *mut u32;
    let local_cc: u8 [0x26];
    let local_a6: u8;
    let local_a4: u8;
    let local_a3: u8;
    let local_a2: u8;
    let local_9f: u8;
    let local_9d: u8;
    let local_98: u8;
    let local_94: u16;
    let local_79: u8;
    let local_78: u8;
    let local_77: u8;
    let local_76: u8;
    let local_75: u8;
    let local_70: u8 [0x53];
    let local_1d: u8;
    let mut local_14: u32;

    FUN_004a0430(local_cc,0x0,0x5c);
    FUN_004a0430(local_70,0x0,0x5c);
    local_14 = DAT_004c9754;
    if (0x4 < DAT_004c9754) {
        if (DAT_004c9754 < 0x6) {
            for (local_e0 = *DAT_005967c8; local_e0 != 0x0; local_e0 = *local_e0) {
                if ((local_e0 + 0xe) == 0x4) {
                    local_e4 = 0x0;
                    iVar1 = local_e0[0x2];
                    iVar2 = *(local_e0 + 0xa);
                    iVar3 = *(local_e0 + 0x6) >> 0x10;
                    local_f4 = (&DAT_005b8b44 + iVar3 * 0x4);
                    while (((local_f4 != 0x0 && ((local_f4 + 0x8) == iVar3)) && (local_e4 < 0x7))) {
                        if ((((local_f4 + 0x22) == iVar1 >> 0x10) &&
                            ((local_f4 + 0x9) == iVar2 >> 0x10)) && ((local_f4 + 0x27) != '[')) {
                            local_e4 = local_e4 + 0x1;
                            (local_f4 + 0x12) = 0x4;
                        }
                        local_f4 = *local_f4;
                    }
                }
            }
        }
        else {
            if (DAT_004c9754 == 0x7) {
                FUN_00419085();
                FUN_0042feac();
                local_a6 = 0xa;
                FUN_00414614(0x2,local_cc,0x0,0xb,0x2d);
                local_a6 = 0x0;
                FUN_00419085();
                FUN_0042feac();
                local_a4 = 0xa;
                FUN_00414614(0x2,local_cc,0x0,0xc,0x20);
                local_a4 = 0x0;
                FUN_00419085();
                FUN_0042feac();
                local_a3 = 0xa;
                FUN_00414614(0x2,local_cc,0x0,0xd,0x20);
                local_a3 = 0x0;
                FUN_00419085();
                FUN_0042feac();
                local_a2 = 0xa;
                FUN_00414614(0x2,local_cc,0x0,0xe,0x20);
                for (local_d0 = *DAT_005967b0; local_d0 != 0x0; local_d0 = *local_d0)
                {
                    if ((((local_d0 + 0x27) == ',') && ((local_d0 + 0x26) == '\a')) &&
                        ((((&DAT_004c7108)[(local_d0 + 0x8) * 0x4] & 0x2) != 0x0 &&
                            (*(*(&DAT_00582938 +
                                (*(local_d0 + 0x25) >> 0x18) * 0x4 + (local_d0[0x9] >> 0x18) * 0x18) +
                                0xa9) != 0x0)))) {
                        (local_d0 + 0x12) = 0x2;
                    }
                }
                return;
            }
        }
    }
    FUN_00419085();
    FUN_0042feac();
    local_98 = 0xa;
    FUN_00414614(0x0,local_cc,0x0,0x3,0x28);
    local_98 = 0x0;
    FUN_00419085();
    FUN_0042feac();
    local_77 = 0xa;
    FUN_00414614(0x2,local_cc,0x0,0x5,0x14);
    local_77 = 0x0;
    FUN_00419085();
    FUN_0042feac();
    local_76 = 0xa;
    FUN_00414614(0x2,local_cc,0x0,0x6,0x14);
    local_76 = 0x0;
    FUN_00419085();
    FUN_0042feac();
    local_75 = 0xa;
    FUN_00414614(0x2,local_cc,0x0,0x7,0x14);
    local_75 = 0x0;
    FUN_00419085();
    FUN_0042feac();
    FUN_00419085();
    FUN_0042feac();
    local_94 = 0xa0a;
    FUN_00414614(0x0,local_cc,0x0,0x8,0x10);
    local_94 = 0x0;
    FUN_00419085();
    FUN_0042feac();
    local_79 = 0xa;
    FUN_00414614(0x2,local_cc,0x0,0x9,0x14);
    local_79 = 0x0;
    FUN_00419085();
    FUN_0042feac();
    local_1d = 0xa;
    local_78 = 0xa;
    FUN_00414614(0x2,local_cc,local_70,0x4,0x14);
    local_1d = 0x0;
    local_78 = 0x0;
    FUN_00412513();
    FUN_004144a5(local_70,0x1);
    FUN_00414592(local_cc,0x1);
    FUN_004144a5(local_cc,0x0);
    local_9f = 0x0;
    local_9d = 0x0;
    FUN_00419085();
    FUN_0042feac();
    FUN_00414614(0x3,local_cc,local_70,0xa,0xc);
    FUN_004a0430(local_cc,0x0,0x5c);
    FUN_004a0430(local_70,0x0,0x5c);
    return;
}



// WARNING: Switch with 1 destination removed at 0x00412ccb : 14 cases all go to same destination
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00412bbf()

{
    let mut uVar1: u32;
    let puVar2: *mut u32;
    let mut pcVar3: String;
    let mut iVar4: i32;
    let uVar5: u16;
    u32 local_250 [0x80];
    let mut local_50: u32;
    let mut local_4c: u32;
    let local_48: *mut u32;
    let local_44: *mut u32;
    let mut local_40: u32;
    let mut local_3c: i32;
    let mut local_38: i32;
    let mut local_34: i32;
    let local_30: *mut u32;
    let mut local_2c: u32;
    let mut local_28: u32;
    let mut local_24: *mut u8;
    let mut local_20: u32;
    let mut local_1c: u32;
    let mut local_18: u32;
    let local_14: *mut u32;

    _DAT_005b8be4 = 0x1;
    FUN_004864f7();
    FUN_00420c87(&DAT_004c7510);
    if (((DAT_004c9754 != 0x9) && (DAT_004c9754 != 0xa)) && (DAT_004c9754 != 0xc)) {
        if (DAT_004c9754 == 0x8) {
            local_24 = &DAT_004d55a8;
            local_20 = 0x8;
            local_1c = 0x0;
            local_18 = DAT_004d5618;
            if (local_18 != 0x2) {
                return;
            }
        }
        FUN_00423df5();
        FUN_00411cab();
        if (DAT_004c9754 < 0x5) {
            FUN_0045e179();
        }
        FUN_004257de();
        local_28 = DAT_004c9754;
        if (DAT_004c9754 < 0xe) {
            _DAT_004c975c = 0x0;
        }
        local_2c = DAT_004c9754;
        switch(DAT_004c9754) {
            case 0x0:
                case 0x1:
                case 0x2:
                case 0x3:
                case 0x4:
                FUN_00425463();
            break;
            case 0x5:
                uVar1 = FUN_004a2edc();
            DAT_00573167 = DAT_00573167 + uVar1 % 0x800 + 0xbb8;
            if ((_DAT_004c9764 == 0x0) && (_DAT_004c9768 <= DAT_00573167)) {
                for (local_38 = 0x0; local_38 < 0x5; local_38 = local_38 + 0x1) {
                    FUN_00462a28(&DAT_004d55a8,local_38,0x5,0x2);
                }
                _DAT_004c9764 = 0x1;
                uVar5 = 0x0;
                uVar1 = 0xffffffff;
                puVar2 = FUN_00499050(DAT_0059679c,0x73d2);
                FUN_0045518a(0x1f,0x5,0x73d2,0xffffffff,puVar2,uVar1,uVar5);
            }
            break;
            case 0x6:
            for (local_34 = 0x0; local_34 < 0x3e8; local_34 = local_34 + 0x1) {
                if (((&DAT_004c616b)[local_34 * 0x4] & 0x1) == 0x0) {
                    (&DAT_004c616b)[local_34 * 0x4] = 0x0;
                }
            }
            _DAT_004c74f4 = 0x0;
            for (local_30 = *DAT_005967b0; local_30 != 0x0; local_30 = *local_30) {
                if (((local_30 + 0x27) == '/') && ((local_30 + 0x26) == '\x06')) {
                    _DAT_004c74f4 = _DAT_004c74f4 + 0x1;
                }
            }
            FUN_0041d87d();
        }
        if ((_DAT_004c9728 == 0x0) || (((byte)DAT_004d559c & 0x2) != 0x0)) {
        if ((_DAT_00596a50 != 0x0) && ((DAT_004c9754 < 0x7 && (iVar4 = FUN_00406cc8(DAT_004c9754), iVar4 != 0x0)))) {
        local_3c = FUN_004255c0(DAT_004c9754);
        *(&DAT_004d5568 + local_3c * 0x4) =
        *(&DAT_004d5568 + local_3c * 0x4) + *(&DAT_00569ab5 + DAT_004c9754 * 0x1e22);
        FUN_00419085();
        }
        FUN_0045e963();
        FUN_0045e9de();
        FUN_0042feac();
        if (*(&DAT_004be70c + DAT_004c9754 * 0x4) != 0x0) {
        FUN_00419085();
        FUN_00462d47();
        FUN_0042feac();
        }
        if ((DAT_004d5584 != 0x0) && (*(&DAT_004be6d4 + DAT_004c9754 * 0x4) != 0x0)) {
        local_40 = DAT_004c9760;
        DAT_004c9760 = 0x1;
        FUN_00419085();
        FUN_00464c97();
        DAT_004c9760 = local_40;
        FUN_0042feac();
        }
        if (*(&DAT_004be70c + DAT_004c9754 * 0x4) != 0x0) {
        FUN_00419085();
        FUN_0047051a();
        FUN_0042feac();
        }
        if (*(&DAT_004be744 + DAT_004c9754 * 0x4) != 0x0) {
        FUN_00419085();
        FUN_0046cb74();
        FUN_0042feac();
        }
        if (*(&DAT_004be77c + DAT_004c9754 * 0x4) != 0x0) {
        FUN_00419085();
        FUN_0045d45a(0x1);
        FUN_0042feac();
        }
        FUN_004228d7();
        FUN_0042feac();
        }
        else {
        _DAT_004c9728 = 0x0;
        }
        if (DAT_004c9754 < 0x5) {
            FUN_0042145e();
        }
        FUN_0041519f();
        for (local_14 = *DAT_005967b0; local_14 != 0x0; local_14 = *local_14) {
            local_48 = local_14 + 0x8;
            local_4c = *(local_14 + 0x3a) & 0x40;
            if (local_4c == 0x0) {
                if ((*(local_14 + 0x23) >> 0x18 == DAT_004c9754) &&
                    (*(&DAT_004bd0bc + (*(local_14 + 0x45) >> 0x18) * 0x4) != 0x0)) {
                    (local_14 + 0x12) = 0x0;
                }
            }
            else {
                (local_14 + 0x12) = 0x4;
            }
            local_44 = local_48;
        }
        if (*(&DAT_004be70c + DAT_004c9754 * 0x4) != 0x0) {
            FUN_00419085();
            FUN_0042feac();
            FUN_0041cad6();
        }
        FUN_004124b5();
        FUN_00412513();
        FUN_00419085();
        FUN_0042feac();
        FUN_00416cb8();
        FUN_00419085();
        FUN_0042feac();
        if (*(&DAT_004be7b4 + DAT_004c9754 * 0x4) != 0x0) {
            FUN_0041de42();
        }
        local_50 = DAT_004c9754;
        if (DAT_004c9754 == 0x6) {
            FUN_0041d3ba();
        }
        FUN_004125af();
        FUN_00412513();
        FUN_00419085();
        FUN_0042feac();
        FUN_00419099();
        FUN_00419085();
        FUN_0042feac();
        FUN_004156e9(0x2);
        FUN_00412513();
        FUN_00419085();
        FUN_0042feac();
        FUN_004156e9(0x9);
        FUN_00419085();
        FUN_0042feac();
        FUN_00418819();
        FUN_00419085();
        FUN_0042feac();
        FUN_00418529();
        if (DAT_004c9754 == 0x5) {
            FUN_00423ac1();
        }
        FUN_00419085();
        FUN_0042feac();
        FUN_00417035();
        FUN_00419085();
        FUN_0042feac();
        FUN_00415a42();
        FUN_00419085();
        FUN_0042feac();
        FUN_00417354();
        FUN_00419085();
        FUN_0042feac();
        FUN_00417a10();
        FUN_00419085();
        FUN_0042feac();
        FUN_00417257();
        FUN_00412513();
        FUN_00419085();
        FUN_0042feac();
        if (_DAT_004c975c != 0x0) {
            FUN_0042433b();
        }
        if ((((_DAT_004c975c != 0x0) || (DAT_004c9754 < 0x5)) || (DAT_004c9754 == 0x6)) || (DAT_004c9754 == 0x7)) {
            if ((((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x10) != 0x0) &&
                (iVar4 = *(&DAT_00569ad4 + DAT_004c9754 * 0x1e22),
                 *(&DAT_00569ad4 + DAT_004c9754 * 0x1e22) = *(&DAT_00569ad4 + DAT_004c9754 * 0x1e22) + -0x1,
                 iVar4 == 0x0)) {
                (&DAT_00569a98)[DAT_004c9754 * 0x1e22] = (&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0xef;
                FUN_00408371(DAT_004c9754);
            }
            if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x4) != 0x0) {
                (&DAT_00569a98)[DAT_004c9754 * 0x1e22] = (&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0xfb;
            }
            FUN_00455648();
            if (*(&DAT_00569b0c + DAT_004c9754 * 0x1e22) != 0x0) {
                *(&DAT_00569b0c + DAT_004c9754 * 0x1e22) = 0x0;
                pcVar3 = FUN_00499050(DAT_0059679c,0x737e);
                FUN_0049c2e0(local_250,pcVar3);
                FUN_0045518a(0x1 << ((byte)DAT_004c9754 & 0x1f),0x5,0x737e,0xffffffff,local_250,0xffffffff,0x0);
            }
        }
        FUN_0045e87b();
        FUN_00482c8a(DAT_004c9754);
        FUN_0045d45a(0x0);
        if (DAT_004c9754 < 0x5) {
            iVar4 = FUN_0046031c(DAT_004c9754,0x0);
            (&DAT_004d566c + DAT_00591cb4 * 0x2 + DAT_004c9754 * 0xc8) = iVar4;
        }
        _DAT_004c975c = 0x0;
    }
    return;
}



fn FUN_004133bd(param_1: *mut u32,param_2: *mut i32) -> u32

{
    let mut iVar1: i32;
    let mut pcVar2: String;
    let mut uVar3: u32;
    let mut uVar4: u32;

    iVar1 = FUN_0049c2c9(0xb8);
    *param_1 = iVar1 + 0x4;
    iVar1 = FUN_0049c2c9(0x1842);
    *param_2 = iVar1;
    if ((*param_1 == 0x0) || (*param_2 == 0x0)) {
        pcVar2 = FUN_00499050(DAT_005b9bd8,0x7d01);
        pop_err_msg_box_and_exit_004a02f5(pcVar2);
    }
    FUN_004a0430(*param_2,0xff,0x1842);
    for (iVar1 = 0x0; iVar1 < 0x2d; iVar1 = iVar1 + 0x1) {
        *(*param_1 + iVar1 * 0x4) = *param_2 + iVar1 * 0x8a + 0x4;
    }
        *(*param_1 - 0x4) = *(*param_1 + 0xac);
    uVar3 = *param_1;
    *(uVar3 + 0xb0) = **param_1;
    for (uVar4 = 0x0; uVar4 < 0x2c; uVar4 = uVar4 + 0x1) {
        *(*(*param_1 + uVar4 * 0x4) + (-0x1 - (uVar4 & 0x1)) * 0x2) = 0x0;
        *(*(*param_1 + uVar4 * 0x4) + (0x41 - (uVar4 & 0x1)) * 0x2) = 0x0;
        uVar3 = uVar4;
    }
    return uVar3;
}



fn FUN_00413500(param_1: i32,param_2: u8)

{
    let mut local_14: i32;

    for (local_14 = *(param_1 + 0x4); local_14 != 0x0; local_14 = *(local_14 + 0x8)) {
        (local_14 + 0x48) = param_2;
        if (param_2 == '\x04') {
            *(local_14 + 0x3a) = *(local_14 + 0x3a) | 0x20;
        }
    }
    return;
}



fn FUN_00413546(param_1: i32)

{
    let mut local_14: i32;

    for (local_14 = *(param_1 + 0x4); local_14 != 0x0; local_14 = *(local_14 + 0x8)) {
        (local_14 + 0x48) = 0x4;
        *(local_14 + 0x3a) = *(local_14 + 0x3a) | 0x20;
    }
    return;
}



fn FUN_00413584(param_1: i32)

{
    let mut local_14: i32;

    for (local_14 = *(param_1 + 0x4); local_14 != 0x0; local_14 = *(local_14 + 0x8)) {
        if (*(*(&DAT_00582938 +
            (*(local_14 + 0x25) >> 0x18) * 0x4 + (*(local_14 + 0x24) >> 0x18) * 0x18) + 0xa9)
            != 0x0) {
            (local_14 + 0x48) = 0x2;
        }
    }
    return;
}



fn FUN_004135e4(param_1: i32)

{
    let mut local_14: i32;

    for (local_14 = *(param_1 + 0x4); local_14 != 0x0; local_14 = *(local_14 + 0x8)) {
        (local_14 + 0x48) = 0x3;
    }
    return;
}


fn FUN_0046b2ed(param_1: u32,param_2: *mut i32) -> i32

{
let bVar1: u8;
let mut iVar2: i32;
let mut pcVar3: String;
byte *pbVar4;
let mut local_c4: i32;
let mut local_c0: i32;
let mut local_bc: i32;
let mut local_b8: i32;
let mut local_b4: u32;
let mut local_b0: i32;
byte *local_ac;
let local_a8: u8 [0x84];
let mut local_24: i32;
let mut local_20: i32;
let mut local_1c: i32;
byte **local_18;
let mut local_14: u32;

local_24 = 0x0;
local_20 = 0x0;
local_1c = 0x0;
local_14 = param_1;
if (0x6 < param_1) {
if (param_1 < 0x8) {
pcVar3 = s_RAND_SYMBIOT_TXT_004c22e5;
pbVar4 = local_a8;
loop {
bVar1 = *pcVar3;
*pbVar4 = bVar1;
if (bVar1 == 0x0) break;
bVar1 = (pcVar3)[0x1];
pcVar3 = (pcVar3 + 0x2);
pbVar4[0x1] = bVar1;
pbVar4 = pbVar4 + 0x2;
} while (bVar1 != 0x0);^
// goto LAB_0046b3aa;
}
if (param_1 == 0x8) {
pcVar3 = s_RAND_VAU_TXT_004c22d8;
pbVar4 = local_a8;
loop {
bVar1 = *pcVar3;
*pbVar4 = bVar1;
if (bVar1 == 0x0) break;
bVar1 = (pcVar3)[0x1];
pcVar3 = (pcVar3 + 0x2);
pbVar4[0x1] = bVar1;
pbVar4 = pbVar4 + 0x2;
} while (bVar1 != 0x0);^
// goto LAB_0046b3aa;
}
}
pcVar3 = s_RAND_NORMAL_TXT_004c22c8;
pbVar4 = local_a8;
loop {
bVar1 = *pcVar3;
*pbVar4 = bVar1;
if (bVar1 == 0x0) break;
bVar1 = (pcVar3)[0x1];
pcVar3 = (pcVar3 + 0x2);
pbVar4[0x1] = bVar1;
pbVar4 = pbVar4 + 0x2;
} while (bVar1 != 0x0);
// LAB_0046b3aa:
local_18 = (byte **)FUN_004a96cc(local_a8,&DAT_004c22f6);
if (local_18 != (byte **)0x0) {
local_1c = FUN_0049c2c9(0xaf0);
if (local_1c == 0x0) {
FUN_0049ca40(local_18);
return 0x0;
}
loop {
loop {
while( true ) {
pcVar3 = FUN_004a2f60(local_a8,0x7f,local_18);
if (pcVar3 == 0x0)^ // goto LAB_0046b5a0;
local_ac = FUN_00467684(local_a8);
if (local_24 != 0x0) break;
iVar2 = FUN_004a9800(local_a8,s_BEGIN_STRUCTS_004c22f9,0xd);
if (iVar2 == 0x0) {
local_24 = local_24 + 0x1;
}
}
iVar2 = FUN_004a9800(local_a8,&DAT_004c2307,0x3);
if (iVar2 == 0x0)^ // goto LAB_0046b5a0;
local_ac = FUN_00467940(local_ac,&local_c4);
local_ac = FUN_00467940(local_ac,&local_c0);
local_ac = FUN_00467940(local_ac,&local_bc);
local_ac = FUN_00467940(local_ac,&local_b8);
local_ac = FUN_004679f5(local_ac,&local_b4);
FUN_00467940(local_ac,&local_b0);
} while ((local_bc == 0x0) || (local_ac == 0x0));
*(local_20 * 0x1c + local_1c) = local_c4;
*(local_20 * 0x1c + local_1c + 0x4) = local_c0;
*(local_20 * 0x1c + local_1c + 0x8) = local_bc;
*(local_20 * 0x1c + local_1c + 0xc) = local_b8;
*(local_20 * 0x1c + local_1c + 0x10) = local_b4;
*(local_20 * 0x1c + local_1c + 0x14) = local_b0;
local_20 = local_20 + 0x1;
} while (local_20 < 0x64);
// LAB_0046b5a0:
FUN_0049ca40(local_18);
}
*param_2 = local_20;
return local_1c;
}



fn FUN_0046b5c5(param_1: i32,param_2: u32)

{
    let mut uVar1: u32;
    let mut iVar2: i32;
    let mut local_40: u32;
    let mut local_3c: u32;
    let mut local_38: i32;
    let mut local_34: i32;
    let mut local_30: i32;
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_24: i32;
    let local_20: *mut i32;;
    let mut local_1c: i32;
    let local_18: *mut i32;;
    let local_14: *mut i32;;

    DAT_005b4a78 = 0xffffffff;
    DAT_005b4a74 = 0xffffffff;
    local_20 = FUN_0046b2ed(param_2,&local_1c);
    if (local_20 != 0x0) {
        local_14 = local_20 + local_1c * 0x7;
        for (local_18 = local_20; local_18 < local_14; local_18 = local_18 + 0x7) {
            local_18[0x6] = 0x0;
            local_24 = (local_18[0x2] - local_18[0x1]) + 0x1;
            if (local_24 != 0x0) {
                if (local_24 < 0x2) {
                    local_2c = local_18[0x1];
                }
                else {
                    local_34 = local_24;
                    uVar1 = FUN_004a2edc();
                    local_30 = uVar1 % local_34;
                    local_2c = local_18[0x1] + local_30;
                }
                for (local_28 = 0x0; local_28 < local_2c; local_28 = local_28 + 0x1) {
                    if ((((*local_18 == 0x12) || (*local_18 == 0x13)) || (*local_18 == 0x18)) || (*local_18 == 0x19)) {
                        local_38 = 0x1;
                    }
                    else {
                        local_38 = 0x0;
                    }
                    iVar2 = FUN_0046b837(param_1,local_18[0x5],&local_40,&local_3c,local_38);
                    if (iVar2 != 0x0) {
                        if (((*local_18 == 0xf) || (*local_18 == 0x2)) || (*local_18 == 0x10)) {
                            iVar2 = FUN_0043ab95(param_1,local_40,local_3c,0xd,0xd,*local_18);
                            if (iVar2 == 0x1) {
                                FUN_00439559((&DAT_004d7d50 + param_1 * 0x3890),local_40,local_3c,0x1,0x0);
                                local_18[0x6] = local_18[0x6] + 0x1;
                            }
                        }
                        else {
                            iVar2 = FUN_0043ab95(param_1,local_40,local_3c,param_2,param_2,*local_18);
                            if (iVar2 == 0x1) {
                                FUN_0046c8b5(*local_18,param_1,local_40,local_3c,param_2);
                                if ((DAT_005b4a74 == 0xffffffff) && (((*(local_18 + 0x5) & 0x10) != 0x0 || (*local_18 == 0x11)))
                                ) {
                                    DAT_005b4a74 = local_40;
                                    DAT_005b4a78 = local_3c;
                                }
                                FUN_00439559((&DAT_004d7d50 + param_1 * 0x3890),local_40,local_3c,0x1,0x0);
                                local_18[0x6] = local_18[0x6] + 0x1;
                            }
                        }
                    }
                }
            }
        }
        FUN_0046be67();
        FUN_0046c65e(param_1,local_20,local_1c);
        FUN_0049af50(local_20);
    }
    return;
}



// WARNING: Could not reconcile some variable overlaps

fn FUN_0046b837(param_1: i32,param_2: u32,param_3: *mut u32,param_4: *mut u32,param_5: i32) -> u32

{
    let mut iVar1: i32;
    let mut uVar2: u32;
    let mut local_ac: i32;
    let local_a8: *mut u32;
    let mut local_a4: u32;
    let mut local_a0: u32;
    let mut local_9c: u32;
    let mut local_98: u32;
    let mut local_94: i32;
    let mut local_90: i32;
    let local_8c: *mut i32;;
    let mut local_88: i32;
    let mut local_84: u32;
    let mut local_80: i32;
    let mut local_7c: i32;
    let mut local_78: i32;
    let mut local_74: u32;
    let mut local_70: i32;
    let mut local_6c: i32;
    let mut local_68: i32;
    let mut local_64: i32;
    let mut local_60: u32;
    let mut local_5c: i32;
    let mut local_58: i32;
    let mut local_54: i32;
    let mut local_50: u32;
    let mut local_4c: i32;
    let mut local_48: i32;
    let mut local_44: i32;
    let mut local_40: u32;
    let mut local_3c: u32;
    let mut local_34: i32;
    let mut local_30: i32;
    let mut local_2c: u32;
    let mut local_28: u32;
    let mut local_24: i32;
    let local_20: *mut i32;;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: u32;

    local_34 = 0x0;
    local_30 = 0x0;
    local_24 = param_1;
    local_20 = 0x0;
    loop {
    if (local_34 != 0x0) {
        return 0x0;
    }
    local_44 = 0x64;
    if ((param_2 & 0xe0) == 0x0) {
        if (((param_2 & 0x10) == 0x0) || (DAT_005b4a74 == -0x1)) {
            local_88 = 0x2c;
            local_84 = FUN_004a2edc();
            local_84 = local_84 % local_88;
            local_80 = 0x1c;
            local_2c = local_84;
            uVar2 = FUN_004a2edc();
            local_7c = uVar2 % local_80;
            local_78 = local_7c * 0x2 + 0x4;
            local_74 = ((local_2c & 0x1) == 0x0);
            local_28 = local_78 + local_74;
        }
        else {
            local_6c = 0x6;
            uVar2 = FUN_004a2edc();
            local_70 = uVar2 % local_6c;
            local_68 = local_70;
            local_2c = FUN_0043a8a2(DAT_005b4a74 + (&DAT_004bea60)[local_70]);
            local_28 = FUN_0043a8d5(local_2c,DAT_005b4a78 + (&DAT_004bea7c)[local_70]);
            local_44 = local_44 + 0x64;
        }
    }
    else {
        if (local_20 == 0x0) {
            LAB_0046b882:
                local_20 = FUN_0046c0f5(local_24,param_2);
            if (local_20 != 0x0) {
                local_4c = 0x6;
                uVar2 = FUN_004a2edc();
                local_48 = uVar2 % local_4c;
                local_14 = 0x0;
                local_1c = local_48;
                local_18 = local_48;
            }
        }
        else {
            local_1c = local_1c + 0x1;
            if (0x5 < local_1c) {
                local_1c = 0x0;
            }
            if (local_1c == local_18) {
                if (local_14 == 0x0) {
                    FUN_0046c20d(param_2,local_20);
                }^
                // goto LAB_0046b882;
            }
        }
        if (local_20 == 0x0) {
            local_64 = 0x2c;
            local_60 = FUN_004a2edc();
            local_60 = local_60 % local_64;
            local_5c = 0x1c;
            local_2c = local_60;
            uVar2 = FUN_004a2edc();
            local_58 = uVar2 % local_5c;
            local_54 = local_58 * 0x2 + 0x4;
            local_50 = ((local_2c & 0x1) == 0x0);
            local_28 = local_54 + local_50;
        }
        else {
            local_2c = FUN_0043a8a2((*local_20 >> 0x10) + (&DAT_004bea60)[local_1c]);
            local_28 = FUN_0043a8d5(local_2c,(*(local_20 + 0x2) >> 0x10) + (&DAT_004bea7c)[local_1c]);
        }
    }
    local_40 = *(*(local_2c * 0x4 + DAT_005b4a48) + local_28 * 0x4);
    local_3c = local_40 & 0xf;
    if (((local_3c != 0x9) && (local_3c != 0xa)) &&
        ((local_3c != 0x0 || (((param_2 & 0x80) != 0x0 && ((param_2 & 0x8) != 0x0)))))) {
        local_8c = FUN_0046bdab(local_2c,local_28,local_24,&local_90,0x0);
        if ((local_3c == 0x6) && ((param_2 & 0x40) == 0x0)) {
            local_44 = local_44 + -0x50;
        }
        if ((0x2 < local_90) || ((param_5 == 0x0 && (local_90 != 0x0)))) {
            if ((param_2 & 0x1) != 0x0) {
                local_44 = local_44 + -0x64 + (local_90 + -0x2) * 0xf;
            }
            if ((param_2 & 0xe) != 0x0) {
                if ((param_2 & 0xa) != 0x0) {
                    local_44 = local_44 + -0x64;
                }
                for (local_94 = 0x0; local_94 < 0x6; local_94 = local_94 + 0x1) {
                    local_9c = FUN_0043a8a2(local_2c + (&DAT_004bea60)[local_94]);
                    local_98 = FUN_0043a8d5(local_9c,local_28 + (&DAT_004bea7c)[local_94]);
                    if ((local_9c != local_2c) || (local_98 != local_28)) {
                        local_a4 = *(*(local_9c * 0x4 + DAT_005b4a48) + local_98 * 0x4);
                        local_a0 = local_a4 & 0xf;
                        if ((param_2 & 0x8) != 0x0) {
                            if (local_3c == 0x0) {
                                if (local_a0 != 0x0) {
                                    local_44 = local_44 + 0x1e;
                                }
                            }
                            else {
                                if (local_a0 == 0x0) {
                                    local_44 = local_44 + 0x1e;
                                }
                            }
                        }
                        if (((param_2 & 0x2) != 0x0) && ((local_a0 == 0xa || (local_a0 == 0x9)))) {
                            local_44 = local_44 + 0x2d;
                        }
                        if (((param_2 & 0x4) != 0x0) && (local_a0 == 0x0)) {
                            local_44 = local_44 + -0x6e;
                        }
                    }
                }
            }
            iVar1 = local_44;
            if ((param_2 & 0x100) != 0x0) {
                if (local_90 < 0x3) {
                    local_44 = local_44 + -0x28;
                    local_a8 = FUN_0046bdab(*local_8c >> 0x10,*(local_8c + 0x2) >> 0x10,local_24,&local_ac,local_8c)
                    ;
                    iVar1 = local_44;
                    if (local_ac < 0x3) {
                        iVar1 = local_44 + 0x32;
                    }
                }
                else {
                    iVar1 = local_44 + -0x64;
                    if (local_90 < 0x5) {
                        iVar1 = local_44 + -0x46;
                    }
                }
            }
            local_44 = iVar1;
            if (local_44 != 0x0) {
                uVar2 = FUN_004a2edc();
                if (uVar2 % 0x64 < local_44) {
                    *param_3 = local_2c;
                    *param_4 = local_28;
                    if (local_20 != 0x0) {
                        FUN_0046c20d(param_2,local_20);
                    }
                    return 0x1;
                }
                if (local_20 != 0x0) {
                    local_14 = local_14 | 0x1 << ((byte)local_1c & 0x1f);
                }
            }
        }
    }
    local_30 = local_30 + 0x1;
    if (0x1388 < local_30) {
        return 0x0;
    }
} while( true );
}



fn FUN_0046bdab(param_1: i32,param_2: i32,param_3: i32,param_4: *mut i32,param_5: *mut u32) -> *mut u32

{
    let mut iVar1: i32;
    let local_1c: *mut u32;
    let mut local_18: i32;
    let local_14: *mut u32;

    local_18 = 0xffff;
    local_14 = 0x0;
    for (local_1c = *DAT_005967c8; local_1c != 0x0; local_1c = *local_1c) {
        if ((((*(local_1c + 0x6) >> 0x10 == param_3) && (local_1c[0x3] >> 0x10 < 0x1a)) &&
            (local_1c + 0x2 != param_5)) &&
            (iVar1 = FUN_0044a87f(param_1,param_2,local_1c[0x2] >> 0x10,*(local_1c + 0xa) >> 0x10),
             iVar1 < local_18)) {
            local_14 = local_1c + 0x2;
            local_18 = iVar1;
        }
    }
        *param_4 = local_18;
    return local_14;
}



fn FUN_0046be67()

{
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0x3; local_14 = local_14 + 0x1) {
        if ((&DAT_005b4a88)[local_14] != 0x0) {
            FUN_0049af50((&DAT_005b4a88)[local_14]);
            (&DAT_005b4a88)[local_14] = 0x0;
        }
    }
    DAT_004bf750 = 0xffffffff;
    return;
}



fn FUN_0046becf() -> u32

{
    let mut uVar1: u32;
    let local_18: *mut u32;
    let mut local_14: i32;

    DAT_005b4a84 = 0x0;
    DAT_005b4a80 = 0x0;
    DAT_005b4a7c = 0x0;
    for (local_18 = *DAT_005967c8; local_18 != 0x0; local_18 = *local_18) {
        if ((*(local_18 + 0x6) >> 0x10 == DAT_004bf750) && (0x19 < local_18[0x3] >> 0x10)) {
            if (((local_18 + 0xe) == 0x1c) || ((local_18 + 0xe) == 0x1d)) {
                DAT_005b4a7c = DAT_005b4a7c + 0x1;
            }
            if ((((local_18 + 0xe) == 0x1a) || ((local_18 + 0xe) == 0x1b)) ||
                ((local_18 + 0xe) == 0x1e)) {
                DAT_005b4a80 = DAT_005b4a80 + 0x1;
            }
            if ((local_18 + 0xe) == 0x1f) {
                DAT_005b4a84 = DAT_005b4a84 + 0x1;
            }
        }
    }
    local_14 = 0x0;
    while( true ) {
        if (0x2 < local_14) {
            DAT_005b4a84 = 0x0;
            DAT_005b4a80 = 0x0;
            DAT_005b4a7c = 0x0;
            for (local_18 = *DAT_005967c8; local_18 != 0x0; local_18 = *local_18) {
                if ((*(local_18 + 0x6) >> 0x10 == DAT_004bf750) && (0x19 < local_18[0x3] >> 0x10)) {
                    if (((local_18 + 0xe) == 0x1c) || ((local_18 + 0xe) == 0x1d)) {
                        (DAT_005b4a7c * 0x4 + DAT_005b4a88) = local_18 + 0x2;
                        DAT_005b4a7c = DAT_005b4a7c + 0x1;
                    }
                    if ((((local_18 + 0xe) == 0x1a) || ((local_18 + 0xe) == 0x1b)) ||
                        ((local_18 + 0xe) == 0x1e)) {
                        (DAT_005b4a80 * 0x4 + DAT_005b4a8c) = local_18 + 0x2;
                        DAT_005b4a80 = DAT_005b4a80 + 0x1;
                    }
                    if ((local_18 + 0xe) == 0x1f) {
                        (DAT_005b4a84 * 0x4 + DAT_005b4a90) = local_18 + 0x2;
                        DAT_005b4a84 = DAT_005b4a84 + 0x1;
                    }
                }
            }
            return 0x1;
        }
        uVar1 = FUN_0049c2c9(((&DAT_005b4a7c)[local_14] + 0x1) * 0x4);
        (&DAT_005b4a88)[local_14] = uVar1;
        if ((&DAT_005b4a88)[local_14] == 0x0) break;
        local_14 = local_14 + 0x1;
    }
    return 0x0;
}



fn FUN_0046c0f5(param_1: i32,param_2: u32) -> u32

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut uVar4: u32;
    let mut local_14: u32;

    if (DAT_004bf750 != param_1) {
        DAT_004bf750 = param_1;
        iVar3 = FUN_0046becf();
        if (iVar3 == 0x0) {
            DAT_004bf750 = 0xffffffff;
            return 0x0;
        }
    }
    iVar2 = DAT_005b4a84;
    iVar1 = DAT_005b4a80;
    iVar3 = DAT_005b4a7c;
    if (((param_2 & 0x20) == 0x0) || (DAT_005b4a7c == 0x0)) {
        if (((param_2 & 0x40) == 0x0) || (DAT_005b4a80 == 0x0)) {
            if (((param_2 & 0x80) == 0x0) || (DAT_005b4a84 == 0x0)) {
                local_14 = 0x0;
            }
            else {
                uVar4 = FUN_004a2edc();
                local_14 = *(DAT_005b4a90 + (uVar4 % iVar2) * 0x4);
            }
        }
        else {
            uVar4 = FUN_004a2edc();
            local_14 = *(DAT_005b4a8c + (uVar4 % iVar1) * 0x4);
        }
    }
    else {
        uVar4 = FUN_004a2edc();
        local_14 = *(DAT_005b4a88 + (uVar4 % iVar3) * 0x4);
    }
    return local_14;
}



fn FUN_0046c20d(param_1: u32,param_2: i32)

{
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let mut local_18: i32;
    let mut local_14: i32;

    local_18 = -0x1;
    if (((param_1 & 0x20) != 0x0) && (DAT_005b4a7c != 0x0)) {
        local_18 = 0x0;
    }
    if (((local_18 == -0x1) && ((param_1 & 0x40) != 0x0)) && (DAT_005b4a80 != 0x0)) {
        local_18 = 0x1;
    }
    if (((local_18 == -0x1) && ((param_1 & 0x80) != 0x0)) && (DAT_005b4a84 != 0x0)) {
        local_18 = 0x2;
    }
    if (local_18 != -0x1) {
        for (local_14 = 0x0;
            (local_14 < (&DAT_005b4a7c)[local_18] && (*((&DAT_005b4a88)[local_18] + local_14 * 0x4) != param_2));
            local_14 = local_14 + 0x1) {
        }
            (&DAT_005b4a7c)[local_18] = (&DAT_005b4a7c)[local_18] + -0x1;
        if (local_14 < (&DAT_005b4a7c)[local_18]) {
            puVar1 = (local_14 * 0x4 + 0x4 + (&DAT_005b4a88)[local_18]);
            puVar2 = (local_14 * 0x4 + (&DAT_005b4a88)[local_18]);
            *puVar2 = *puVar1;
            (puVar2 + 0x1) = (puVar1 + 0x1);
        }
    }
    return;
}



fn FUN_0046c33d(param_1: i32,param_2: u32,param_3: u32,param_4: u32,param_5: u32) -> u32

{
    let mut uVar1: u32;
    let local_7c: u8 [0x8];
    let mut local_74: u32;
    let mut local_70: u32;
    let local_5c: u16;
    let local_5a: i16;
    let sStack88: i16;
    let local_56: u16;
    let local_52: u16;
    let local_4d: u8;
    let local_3b: u8;
    let local_3a: u8;
    let mut local_2c: u32;
    let mut local_28: *mut u8;
    let mut local_24: u32;
    let mut local_20: u32;
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: u32;

    local_18 = 0x0;
    if ((param_2 == param_4) && (param_3 == param_5)) {
        local_24 = 0x0;
    }
    else {
        FUN_00486065(&local_5c);
        local_5a = param_2;
        sStack88 = param_3;
        local_56 = 0x540d;
        local_52 = *(DAT_00583118 + 0x41);
        local_4d = 0xff;
        local_3b = param_4;
        local_3a = param_5;
        local_5c = param_1;
        FUN_00431d31(&local_2c);
        FUN_00431d0a(&local_2c);
        local_28 = local_7c;
        local_70 = 0x0;
        local_74 = 0x0;
        local_14 = FUN_0045af67(&local_2c,param_2,param_3,param_4,param_5,0x0);
        if (local_14 != 0x0) {
            while (((local_5a != param_4 || (sStack88 != param_5)) &&
                (local_14 = FUN_0045b45b(&local_2c,local_14,local_5a,sStack88,&local_20,&local_1c),
                 local_14 != 0xffffffff))) {
                uVar1 = *(*(local_20 * 0x4 + DAT_005b4a48) + local_1c * 0x4);
                if ((uVar1 & 0xf) != 0xb) {
                    *(*(local_20 * 0x4 + DAT_005b4a48) + local_1c * 0x4) =
                        *(DAT_005b4a4c + 0x68c) << 0x4 | uVar1 << 0x9 | 0xb;
                    FUN_00439559((&DAT_004d7d50 + param_1 * 0x3890),local_20,local_1c,0x1,0x0);
                }
                local_5a = local_20;
                sStack88 = local_1c;
                local_5c = param_1;
            }
        }
        local_24 = local_18;
    }
    return local_24;
}



fn FUN_0046c5b9(param_1: i32,param_2: i32) -> i32

{
let mut uVar1: u32;
let mut uVar2: u32;
let mut local_18: i32;
let mut local_14: i32;

local_18 = 0x0;
for (local_14 = 0x0; local_14 < 0x6; local_14 = local_14 + 0x1) {
uVar1 = FUN_0043a8a2(param_1 + (&DAT_004bea60)[local_14]);
uVar2 = FUN_0043a8d5(uVar1,param_2 + (&DAT_004bea7c)[local_14]);
if ((*(*(uVar1 * 0x4 + DAT_005b4a48) + uVar2 * 0x4) & 0xf) == 0xb) {
local_18 = local_18 + 0x1;
}
}
return local_18;
}



fn FUN_0046c65e(param_1: i32,param_2: *mut i32,param_3: i32)

{
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut uVar4: u32;
    i32 aiStackY131268 [0x7ff5];
    let local_c8: *mut u32;
    i32 local_c4 [0x28];
    let local_24: *mut u32;
    let mut local_20: i32;
    let mut local_1c: u32;
    let local_18: *mut i32;;
    let local_14: *mut i32;;

    local_1c = DAT_004c9754;
    DAT_004c9754 = 0xd;
    FUN_004a0430(local_c4,0x0,0xa0);
    local_14 = param_2 + param_3 * 0x7;
    for (local_18 = param_2; local_18 < local_14; local_18 = local_18 + 0x7) {
        local_c4[*local_18] = local_18[0x4];
    }
    for (local_24 = *DAT_005967c8; local_24 != 0x0; local_24 = *local_24) {
        if ((*(local_24 + 0x6) >> 0x10 == param_1) && (local_24[0x3] >> 0x10 < 0x1a)) {
            uVar1 = local_24[0x2] >> 0x10;
            uVar2 = *(local_24 + 0xa) >> 0x10;
            for (local_c8 = *local_24; local_c8 != 0x0; local_c8 = *local_c8) {
                if ((*(local_c8 + 0x6) >> 0x10 == param_1) && (local_c8[0x3] >> 0x10 < 0x1a)) {
                    iVar3 = FUN_0044a87f(uVar1,uVar2,local_c8[0x2] >> 0x10,*(local_c8 + 0xa) >> 0x10);
                    local_20 = iVar3;
                    if (((iVar3 < 0x8) &&
                        (((local_c4[local_24[0x3] >> 0x10] != 0x0 && (local_c4[local_c8[0x3] >> 0x10] != 0x0)) &&
                            (uVar4 = FUN_004a2edc(), uVar4 % (0xc - iVar3) != 0x0)))) &&
                        ((iVar3 = FUN_0046c5b9(uVar1,uVar2), iVar3 < 0x2 &&
                            (iVar3 = FUN_0046c5b9(local_c8[0x2] >> 0x10,*(local_c8 + 0xa) >> 0x10), iVar3 < 0x2)))) {
                        FUN_0046c33d(param_1,uVar1,uVar2,local_c8[0x2] >> 0x10,*(local_c8 + 0xa) >> 0x10);
                    }
                }
            }
        }
    }
    DAT_004c9754 = local_1c;
    return;
}



// WARNING: Could not reconcile some variable overlaps

fn FUN_0046c8b5(param_1: i32,param_2: u32,param_3: u32,param_4: u32,param_5: i32)

{
    i32 **ppiVar1;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let puVar4: *mut u32;
    let puVar5: *mut u32;
    let in_stack_ffffff0c: i16;
    let mut in_stack_ffffff10: u32;
    u32 local_bc [0xc];
    let mut local_8c: u32;
    let local_88: u16;
    let local_86: u8;
    let local_85: u8;
    let local_84: u8;
    let local_82: u16;
    let local_7e: u16;
    let local_7c: u8;
    let local_7b: u16;
    let local_79: u8;
    let local_77: u8;
    let mut local_76: u32;
    let mut local_72: u32;
    i32 **local_5c;
    undefined2 *local_58;
    let mut local_54: u32;
    let mut local_50: u32;
    let mut local_4c: u32;
    let mut local_48: u32;
    i32 **local_44;
    let mut local_40: i32;
    let mut local_3c: i32;
    let mut local_38: i32;
    let mut local_34: i32;
    let mut local_30: i32;
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    local_1c = 0x0;
    local_18 = 0x0;
    if (param_5 == 0x8) {
        param_5 = 0x1;
        param_1 = 0xd;
    }
    else {
        if (param_5 == 0x7) {
            param_5 = 0x2;
            param_1 = 0xd;
        }
        else {
            param_5 = 0x0;
        }
    }
    local_14 = 0x9;
    loop {
    if (local_14 < 0x0) {
        return;
    }
    local_24 = *(local_14 * 0x4 + param_5 * 0x28 + 0x4bf754);
    if (local_24 != -0x1) {
        local_34 = param_1 * 0x3c + local_14 * 0x4;
        local_30 = (*(&DAT_004bdf1c + local_14 * 0x4 + param_1 * 0x3c) + 0x1) -
            *(&DAT_004bd79c + local_14 * 0x4 + param_1 * 0x3c);
        local_2c = local_30;
        uVar2 = FUN_004a2edc();
        local_28 = uVar2 % local_2c;
        local_38 = *(&DAT_004bd79c + local_34) + local_28;
        if (local_38 != 0x0) {
            local_38 = local_38 / 0xa;
            if (local_24 == 0x53) {
                local_40 = 0x2;
                uVar2 = FUN_004a2edc();
                local_3c = uVar2 % local_40;
                if (local_3c != 0x0) {
                    local_18 = 0x1;
                    LAB_0046ca03:
                        FUN_00431d31(&local_48);
                    FUN_00431d0a(&local_48);
                    for (local_20 = 0x0; local_20 < local_38; local_20 = local_20 + 0x1) {
                        FUN_00486065(&local_8c);
                        FUN_00486b6b(&local_8c,0x4b);
                        local_58 = &local_8c;
                        local_54 = param_2;
                        local_50 = param_3;
                        local_4c = param_4;
                        local_8c._0_2_ = param_2;
                        local_8c._2_2_ = param_3;
                        local_88 = param_4;
                        local_85 = local_24;
                        local_84 = 0x0;
                        local_82 = *(*(&DAT_00582938 + local_24 * 0x18) + 0x41);
                        local_7e = 0x0;
                        local_7c = 0x0;
                        local_7b = 0x0;
                        local_79 = 0x64;
                        if (param_5 == 0x1) {
                            local_86 = 0x8;
                        }
                        else {
                            if (param_5 == 0x2) {
                                local_86 = 0x7;
                            }
                            else {
                                local_86 = 0xd;
                            }
                        }
                        local_77 = local_86;
                        local_76 = 0x0;
                        local_72 = local_72 | 0x1;
                        puVar4 = &local_8c;
                        puVar5 = local_bc;
                        for (iVar3 = 0xb; iVar3 != 0x0; iVar3 = iVar3 + -0x1) {
                            *puVar5 = *puVar4;
                            puVar4 = puVar4 + 0x1;
                            puVar5 = puVar5 + 0x1;
                        }
                        puVar5 = puVar4;
                        puVar4 = local_bc;
                        puVar5 = &stack0xffffff0c;
                        for (iVar3 = 0xb; iVar3 != 0x0; iVar3 = iVar3 + -0x1) {
                            *puVar5 = *puVar4;
                            puVar4 = puVar4 + 0x1;
                            puVar5 = puVar5 + 0x1;
                        }
                        puVar5 = puVar4;
                        local_5c = FUN_00485463(in_stack_ffffff0c,in_stack_ffffff10);
                        ppiVar1 = local_5c;
                        if (local_20 != 0x0) {
                            FUN_00431efd(&local_48,local_5c);
                            ppiVar1 = local_44;
                        }
                        local_44 = ppiVar1;
                        local_1c = local_1c + 0x1;
                        if (0x13 < local_1c) break;
                    }
                    if (0x13 < local_1c) {
                        return;
                    }
                }
            }
            else {
                if ((local_24 != 0x54) || (local_18 == 0x0))^ // goto LAB_0046ca03;
            }
        }
    }
    local_14 = local_14 + -0x1;
} while( true );
}



fn FUN_0046cb74()

{
    let mut iVar1: i32;
    i32 **ppiVar2;
    let mut iVar3: i32;
    let mut uVar4: u32;
    let mut pcVar5: String;
    let piVar6: *mut i32;;
    let mut iVar7: i32;
    let puVar8: *mut u32;
    let puVar9: *mut u32;
    float10 fVar10;
    let in_stack_fffffd1c: i16;
    let mut in_stack_fffffd20: u32;
    u32 local_154 [0xc];
    i32 **local_124;
    i32 **local_120;
    let local_11c: *mut u32;
    ushort local_118;
    ushort local_116;
    ushort local_114;
    let local_112: u8;
    let local_111: u16;
    let local_10e: u16;
    let local_10a: u8;
    let local_109: u8;
    let local_105: u8;
    let mut local_e8: u32;
    let mut local_e4: u32;
    let mut local_e0: u32;
    let mut local_dc: u32;
    let mut local_d8: u32;
    let mut local_d4: i32;
    let mut local_d0: i32;
    let mut local_cc: i32;
    let mut local_c4: i32;
    let local_c0: *mut i32;;
    let local_bc: *mut i32;;
    let mut local_b8: u32;
    u32 local_b4 [0x20];
    let mut local_34: u32;
    let mut local_30: i32;
    i32 **local_2c;
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: i32;
    i32 **local_1c;
    i32 **local_18;
    let local_14: *mut i32;;

    if ((DAT_004d558c != 0x0) && (DAT_004c9754 < 0x5)) {
        FUN_00431d31(&local_34);
        for (local_1c = *DAT_005967c8; local_1c != 0x0; local_1c = *local_1c) {
            if ((*(local_1c + 0xe) >> 0x10 == DAT_004c9754) && ((*(local_1c + 0x2d) & 0x1) == 0x0)) {
                local_c0 = (local_1c + 0x2);
                local_b8 = local_c0 & 0xffff0000 | (local_1c + 0x8);
                local_bc = local_c0;
                if ((local_1c + 0x8) < 0x19) {
                    iVar1 = (*(local_1c + 0x26) >> 0x10) / 0xa;
                    local_c4 = iVar1;
                    FUN_004a9ae0((double)iVar1);
                    local_cc = iVar1;
                    fVar10 = FUN_004a9b12();
                    local_d0 = ROUND(fVar10);
                    for (local_d4 = 0x0; local_d4 < local_d0; local_d4 = local_d4 + 0x1) {
                        FUN_00486065(&local_118);
                        local_11c = &local_118;
                        local_118 = (local_1c + 0x2);
                        local_e0 = local_1c & 0xffff0000 | local_118;
                        local_116 = (local_1c + 0xa);
                        local_dc = local_1c & 0xffff0000 | local_116;
                        local_114 = (local_1c + 0x3);
                        local_d8 = local_1c & 0xffff0000 | local_114;
                        local_112 = (byte)DAT_004c9754;
                        local_111 = 0x5a;
                        local_10e = *(DAT_005831a8 + 0x41);
                        local_109 = (DAT_005831a8 + 0x45);
                        local_124 = local_1c + 0x2;
                        local_e8 = local_124 & 0xffff0000 | (local_1c + 0x8);
                        local_120 = local_124;
                        FUN_00486b6b(&local_118,(local_1c + 0x8));
                        local_10a = 0x0;
                        local_105 = (local_1c + 0xa);
                        puVar8 = &local_118;
                        puVar9 = local_154;
                        for (iVar1 = 0xb; iVar1 != 0x0; iVar1 = iVar1 + -0x1) {
                            *puVar9 = *puVar8;
                            puVar8 = puVar8 + 0x1;
                            puVar9 = puVar9 + 0x1;
                        }
                        puVar9 = puVar8;
                        puVar8 = local_154;
                        puVar9 = &stack0xfffffd1c;
                        for (iVar1 = 0xb; iVar1 != 0x0; iVar1 = iVar1 + -0x1) {
                            *puVar9 = *puVar8;
                            puVar8 = puVar8 + 0x1;
                            puVar9 = puVar9 + 0x1;
                        }
                        puVar9 = puVar8;
                        ppiVar2 = FUN_00485463(in_stack_fffffd1c,in_stack_fffffd20);
                        *(ppiVar2 + 0x3a) = *(ppiVar2 + 0x3a) | 0x1;
                        *(ppiVar2 + 0x47) = *(ppiVar2 + 0x47) | 0x1;
                        local_e4 = (local_1c + 0x2) & 0xffff0000 | (local_1c + 0x8);
                        FUN_00486b6b((ppiVar2 + 0x8),(local_1c + 0x8));
                    }
                        *(local_1c + 0xa) = 0x0;
                }
            }
        }
        local_28 = -0x1;
        local_24 = -0x1;
        local_20 = -0x1;
        DAT_00599d3c = 0x1;
        for (local_2c = *DAT_005967b0; local_2c != 0x0; local_2c = *local_2c) {
            if (((((*(local_2c + 0x47) & 0x4) == 0x0) && ((*(local_2c + 0x3a) & 0x80000000) == 0x0))
                && ((local_2c + 0x27) != '-')) &&
                ((((*(local_2c + 0x3a) & 0x80) == 0x0 && (*(local_2c + 0x23) >> 0x18 == DAT_004c9754))
                    && ((*(local_2c + 0x3a) & 0x1) != 0x0)))) {
                if ((((local_2c + 0x8) != local_28) || ((local_2c + 0x22) != local_24)) ||
                    ((local_2c + 0x9) != local_20)) {
                    local_28 = (local_2c + 0x8);
                    local_24 = (local_2c + 0x22);
                    local_20 = (local_2c + 0x9);
                    if (local_30 != 0x0) {
                        FUN_0046d88e(&local_34);
                        FUN_00431d0a(&local_34);
                    }
                }
                if ((*(*(&DAT_00582938 +
                    (*(local_2c + 0x25) >> 0x18) * 0x4 + (local_2c[0x9] >> 0x18) * 0x18) +
                    0x119) != 0x0) && ((local_2c + 0x29) < '\x19')) {
                    iVar1 = *(&DAT_00569b8d + DAT_004c9754 * 0x1e22) >> 0x18;
                    iVar3 = *(&DAT_00569b99 + DAT_004c9754 * 0x1e22) >> 0x18;
                    uVar4 = FUN_004a2edc();
                    iVar7 = uVar4 % 0x32;
                    if ((local_2c + 0x29) < iVar7) {
                        if (iVar1 == iVar3) {
                            *(local_2c + 0x47) = *(local_2c + 0x47) & 0xfe;
                            *(local_2c + 0x3b) = *(local_2c + 0x3b) | 0x2;
                            FUN_00431efd(&local_34,local_2c);
                        }
                        else {
                            if ((local_2c + 0x29) < iVar7 - iVar1) {
                                *(local_2c + 0x47) = *(local_2c + 0x47) & 0xfe;
                                *(local_2c + 0x3b) = *(local_2c + 0x3b) | 0x2;
                                FUN_00431efd(&local_34,local_2c);
                            }
                            else {
                                if ((*(DAT_004c9754 * 0x1e22 + 0x569b8f) & 0x10) != 0x0) {
                                    FUN_00499050(DAT_0059679c,0x1075);
                                    pcVar5 = FUN_00499050(DAT_0059679c,0x6921);
                                    FUN_0049c2e0(local_b4,pcVar5);
                                    FUN_0045518a(0x1 << ((byte)DAT_004c9754 & 0x1f),0xffffffff,0x1075,0xffffffff,local_b4,0xffffffff,0x0);
                                }
                            }
                        }
                    }
                    else {
                        if ((iVar1 != iVar3) && ((local_2c + 0x29) < iVar7 - iVar3)) {
                            *(local_2c + 0x47) = *(local_2c + 0x47) & 0xfe;
                            *(local_2c + 0x3b) = *(local_2c + 0x3b) | 0x2;
                            FUN_00431efd(&local_34,local_2c);
                            if ((*(DAT_004c9754 * 0x1e22 + 0x569b6b) & 0x10) != 0x0) {
                                FUN_00499050(DAT_0059679c,0x109b);
                                pcVar5 = FUN_00499050(DAT_0059679c,0x6947);
                                FUN_0049c2e0(local_b4,pcVar5);
                                FUN_0045518a(0x1 << ((byte)DAT_004c9754 & 0x1f),0xffffffff,0x109b,0xffffffff,local_b4,0xffffffff,0x0);
                            }
                        }
                    }
                }
            }
        }
        if (local_30 != 0x0) {
            FUN_0046d88e(&local_34);
        }
        local_28 = 0xffffffff;
        local_24 = 0xffffffff;
        local_20 = 0xffffffff;
        local_1c = 0x0;
        for (local_2c = *DAT_005967b0; local_2c != 0x0; local_2c = *local_2c) {
            if ((*(local_2c + 0x3a) & 0x80000000) == 0x0) {
                if ((*(local_2c + 0x47) & 0x4) != 0x0) {
                    *(local_2c + 0x47) = *(local_2c + 0x47) & 0xfb;
                }
                if ((*(local_2c + 0x47) & 0x1) != 0x0) {
                    if ((((local_1c == 0x0) || ((local_2c + 0x8) != (local_1c + 0x2))) ||
                        ((local_2c + 0x22) != (local_1c + 0xa))) ||
                        ((local_2c + 0x9) != (local_1c + 0x3))) {
                        local_1c = FUN_00481784((local_2c + 0x8),(local_2c + 0x22),
                                                        (local_2c + 0x9));
                        *(local_1c + 0xa) = 0x0;
                    }
                    (local_1c + 0xa) = (local_1c + 0xa) + 0xa;
                    local_2c = FUN_00484b4e(local_2c);
                }
            }
        }
        local_18 = *DAT_005967c8;
        while (local_18 != 0x0) {
            local_14 = *local_18;
            if ((local_18 + 0xa) == 0x0) {
                if ((*((*(local_18 + 0xa) >> 0x10) * 0x4 +
                    *(&DAT_004d7d50 +
                        (*(local_18 + 0x6) >> 0x10) * 0x3890 + (local_18[0x2] >> 0x10) * 0x4)) &
                    0xf) == 0xb) {
                    piVar6 = ((*(local_18 + 0xa) >> 0x10) * 0x4 +
                        *(&DAT_004d7d50 +
                            (*(local_18 + 0x6) >> 0x10) * 0x3890 + (local_18[0x2] >> 0x10) * 0x4)
                    );
                    *piVar6 = *piVar6 >> 0x9;
                }
                FUN_004811e6(local_18);
            }
            local_18 = local_14;
        }
        DAT_00599d3c = 0x0;
        FUN_004864f7();
    }
    return;
}



bool  FUN_0046d7b4(param_1: i32,param_2: i32,param_3: i32)

{
let mut bVar1: bool;
let local_18: *mut u32;

bVar1 = true;
local_18 = (&DAT_005b8b44 + param_1 * 0x4);
while( true ) {
if ((local_18 == 0x0) || ((local_18 + 0x8) != param_1)) {
return !bVar1;
}
if ((((local_18 + 0x22) == param_2) && ((local_18 + 0x9) == param_3)) &&
(bVar1 = false, (local_18 + 0x26) != '\r')) break;
local_18 = *local_18;
}
return false;
}



fn FUN_0046d88e(param_1: i32)

{
    ushort uVar1;
    let mut bVar2: bool;
    undefined3 extraout_var;
    let mut iVar3: i32;
    let mut uVar4: u32;
    i32 local_8c [0x6];
    let mut local_74: i32;
    let mut local_70: u32;
    let mut local_6c: u32;
    i32 **local_68;
    let local_64: *mut u32;
    let mut local_60: i32;
    let mut local_5c: i32;
    let mut local_58: u32;
    let mut local_54: u32;
    let mut local_50: u32;
    ushort *local_4c;
    let mut local_48: u32;
    let mut local_44: u32;
    let mut local_40: i32;
    let mut local_3c: u32;
    let mut local_38: u32;
    let mut local_34: u32;
    let mut local_30: u32;
    ushort *local_2c;
    let mut local_28: i32;
    let mut local_24: u32;
    let mut local_20: u32;
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: u32;

    local_54 = *(param_1 + 0x4) + 0x20;
    uVar1 = (*(param_1 + 0x4) + 0x22);
    local_1c = local_54 & 0xffff0000 | uVar1;
    local_6c = SEXT24(uVar1);
    local_48 = *(param_1 + 0x4) + 0x20;
    uVar1 = (*(param_1 + 0x4) + 0x24);
    local_14 = local_48 & 0xffff0000 | uVar1;
    local_58 = SEXT24(uVar1);
    local_50 = local_54;
    local_44 = local_48;
    local_60 = FUN_00432c94(param_1);
    local_3c = *(param_1 + 0x4) + 0x20;
    uVar1 = (*(param_1 + 0x4) + 0x24);
    local_24 = local_3c & 0xffff0000 | uVar1;
    local_28 = uVar1;
    local_34 = *(param_1 + 0x4) + 0x20;
    uVar1 = (*(param_1 + 0x4) + 0x22);
    local_20 = local_34 & 0xffff0000 | uVar1;
    local_40 = uVar1;
    local_4c = (*(param_1 + 0x4) + 0x20);
    local_18 = local_4c & 0xffff0000 | *local_4c;
    local_38 = local_3c;
    local_30 = local_34;
    local_2c = local_4c;
    bVar2 = FUN_0046d7b4(*local_4c,local_40,local_28);
    if ((CONCAT31(extraout_var,bVar2) == 0x0) ||
        ((local_64 = FUN_00481784((*(param_1 + 0x4) + 0x20),
                                  (*(param_1 + 0x4) + 0x22),
                                  (*(param_1 + 0x4) + 0x24)), local_64 != 0x0 &&
              (iVar3 = FUN_00481a44(local_64), iVar3 == 0x0)))) {
        FUN_004a0430(local_8c,0x0,0x18);
        for (local_74 = 0x0; local_74 < 0x6; local_74 = local_74 + 0x1) {
            local_6c = FUN_0043a8a2((*(param_1 + 0x4) + 0x22) + (&DAT_004bea60)[local_74]);
            local_58 = FUN_0043a8d5(local_6c,(*(param_1 + 0x4) + 0x24) + (&DAT_004bea7c)[local_74]);
            local_70 = *(*(&DAT_004d7d50 + (*(param_1 + 0x4) + 0x20) * 0x3890 + local_6c * 0x4
            ) + local_58 * 0x4);
            local_64 = FUN_00481784((*(param_1 + 0x4) + 0x20),local_6c,local_58);
            local_5c = FUN_0044ace5((*(param_1 + 0x4) + 0x20),local_6c,local_58,0x1);
            if (local_5c == 0xd) {
                local_8c[local_74] = 0x0;
                iVar3 = FUN_00485ea2((*(param_1 + 0x4) + 0x20),local_6c,local_58,0x1);
                if (iVar3 + local_60 < 0x15) {
                    if ((*(param_1 + 0x4) + 0x2a) == 0x4) {
                        if ((local_70 & 0xf) == 0x0)^ // goto LAB_0046dfc7;
                    }
                    else {
                        if (((local_70 & 0xf) != 0x0) && ((local_70 & 0xf) != 0xa))^ // goto LAB_0046dfc7;
                    }
                }
            }
            else {
                if ((local_5c == -0x1) &&
                    ((local_64 == 0x0 || (iVar3 = FUN_00481a44(local_64), iVar3 != 0x0)))) {
                    local_8c[local_74] = 0x1;
                }
            }
        }
        for (local_74 = 0x0; local_74 < 0x6; local_74 = local_74 + 0x1) {
            local_6c = FUN_0043a8a2((*(param_1 + 0x4) + 0x22) + (&DAT_004bea60)[local_74]);
            local_58 = FUN_0043a8d5(local_6c,(*(param_1 + 0x4) + 0x24) + (&DAT_004bea7c)[local_74]);
            local_70 = *(*(&DAT_004d7d50 + local_6c * 0x4 + (*(param_1 + 0x4) + 0x20) * 0x3890
            ) + local_58 * 0x4);
            if ((*(param_1 + 0x4) + 0x2a) == 0x4) {
                if ((local_70 & 0xf) == 0x0) {
                    LAB_0046de62:
                    if (local_8c[local_74] != 0x0)^ // goto LAB_0046dfc7;
                }
            }
            else {
                if (((local_70 & 0xf) != 0x0) && ((local_70 & 0xf) != 0xa))^ // goto LAB_0046de62;
            }
        }
        if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
            FUN_00450dbf(&DAT_00595740,(*(param_1 + 0x4) + 0x20),
                         (*(param_1 + 0x4) + 0x22),(*(param_1 + 0x4) + 0x24),
                         *(param_1 + 0x4),0x1,-0x1,0x0);
            FUN_0046e218(0x73b4,(*(param_1 + 0x4) + 0x20));
        }
        for (local_68 = *(i32 ***)(param_1 + 0x4); local_68 != 0x0; local_68 = local_68[0x2]) {
            FUN_00484b4e(local_68);
        }
    }
    else {
        LAB_0046dfc7:
        if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
            FUN_00450dbf(&DAT_00595740,(*(param_1 + 0x4) + 0x20),
                         (*(param_1 + 0x4) + 0x22),(*(param_1 + 0x4) + 0x24),
                         *(param_1 + 0x4),0x1,-0x1,0x0);
            FUN_0046e218(0x73b3,(*(param_1 + 0x4) + 0x20));
        }
        for (local_68 = *(i32 ***)(param_1 + 0x4); local_68 != 0x0; local_68 = local_68[0x2]) {
            FUN_004841ea(local_68,(local_68 + 0x8),local_6c,local_58);
            if ((local_68 + 0x27) == 'Z') {
                uVar4 = FUN_004a2edc();
                if (uVar4 % 0x64 < 0x32) {
                    (local_68 + 0x27) = 0x37;
                }
                else {
                    if (uVar4 % 0x64 < 0x4b) {
                        (local_68 + 0x27) = 0x57;
                    }
                    else {
                        (local_68 + 0x27) = 0x56;
                    }
                }
                *(local_68 + 0x2a) =
                    *(*(&DAT_00582938 + (local_68[0x9] >> 0x18) * 0x18) + 0x41);
                (local_68 + 0x2f) =
                    (*(&DAT_00582938 + (local_68[0x9] >> 0x18) * 0x18) + 0x45);
            }
            FUN_00486b6b((local_68 + 0x8),0x4b);
            (local_68 + 0x26) = 0xd;
            *(local_68 + 0x3b) = *(local_68 + 0x3b) & 0xfd;
            *(local_68 + 0x47) = *(local_68 + 0x47) | 0x4;
        }
    }
    return;
}



fn FUN_0046e218(param_1: u32,param_2: u32)

{
    let piVar1: *mut i32;;
    let mut local_160: *mut u32 [0x11];
    let ppuStack283: *mut *mut u8;;
    let mut local_6f: String;;
    let mut local_68: i32;
    let mut local_64: u32;
    let local_60: *mut i32;;
    let local_5c: *mut i32;;
    let local_58: *mut i32;;
    let local_54: *mut i32;;
    let local_50: *mut i32;;
    let mut local_4c: u32;
    let local_48: *mut i32;;
    let local_44: *mut i32;;
    let mut local_40: u32;
    let mut local_3c: i32;
    let mut local_38: i32;
    let local_34: *mut i32;;
    let mut local_30: i32;
    let mut local_2c: u32;
    let mut local_28: u32;
    let mut local_24: u32;
    u32 **local_20;
    let mut local_1c: u32;

    FUN_004a1651();
    DAT_005b4a98 = param_1;
    DAT_005b4a9c = param_2;
    local_50 = FUN_004990e0(local_160,0x0,s_efs_res_004c2320,s_RebelPic_004c2317);
    local_4c = DAT_005b7068;
    local_64 = DAT_005b7068;
    local_60 = FUN_004a2831(0xb9);
    local_5c = local_60;
    local_48 = local_60;
    if (local_60 != 0x0) {
        local_48 = FUN_00438792(local_60,local_160,0x1f4,0x88,0x17c,0x58,0x3f,0x40,0x2,0x2,param_2);
    }
    DAT_005b8c38 = local_48;
    local_58 = FUN_004a2831(0x95);
    local_54 = local_58;
    local_44 = local_58;
    if (local_58 != 0x0) {
        local_44 = FUN_0047157e(local_58,local_160,0x1f5,0x16,0x17c,0x64,0x60,0x40,0x2);
    }
    DAT_005b8c3c = local_44;
    FUN_0049bf40(local_160,DAT_005b8c38);
    FUN_0049bf40(local_160,DAT_005b8c3c);
    piVar1 = DAT_005b8c38;
    local_40 = param_2;
    DAT_005b7068 = param_2;
    DAT_005b8808 = 0x0;
    local_38 = DAT_00595f56._2_2_;
    local_68 = DAT_00595f56;
    local_34 = DAT_005b8c38;
    *(DAT_005b8c38 + 0x95) = local_68;
    *(piVar1 + 0x99) = local_38;
    local_3c = local_68;
    local_30 = local_38;
    FUN_00431d0a(&DAT_005967b8);
    DAT_005967bc = DAT_00595740;
    FUN_0049bb50(local_160,FUN_0046e49d);
    local_2c = local_64;
    DAT_005b7068 = local_64;
    DAT_005b8808 = 0x0;
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
    local_20 = local_160;
    local_1c = 0x0;
    ppuStack283 = &PTR_FUN_004c3d34;
    if (local_6f != 0x0) {
        FUN_00499b30(local_160,local_6f);
    }
    FUN_0049a1c0(local_160,0x1);
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0046e49d(param_1: *mut *mut u32,param_2: u32,param_3: i32) -> u32

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
        if (param_2 == 0x401) {
            FUN_0049bf80(param_1,0x1f4,0x414,0x0,0x0);
            FUN_0049bf80(param_1,0x1f4,0x410,0x0,0x0);
            FUN_0049bf80(param_1,0x1f5,0x414,0x0,0x0);
            FUN_0049bf80(param_1,0x1f5,0x410,0x0,0x0);
            _DAT_005b4a94 = 0x0;
            return 0x0;
        }
    }
    else {
        if (param_2 < 0x406) {
            FUN_004953d7();
            if (_DAT_005b4a94 == 0x0) {
                pcVar1 = FUN_00499050(DAT_0059679c,0x73b3);
                FUN_0049c2e0(local_98,pcVar1);
                FUN_00497567(0x140,0x15e,local_98,0x140,0xe3e5e7,-0x1,0xe3e5e7,LPCSTR_005b9218,0x11);
            }
            else {
                FUN_0048f724(&DAT_00595740,0x1);
                if (DAT_005b4a98 == 0x73b4) {
                    bVar7 = 0x11;
                    uVar5 = 0xe3e5e7;
                    iVar4 = -0x1;
                    uVar3 = 0xe3e5e7;
                    iVar2 = 0x258;
                    puVar6 = LPCSTR_005b9218;
                    pcVar1 = FUN_00499050(DAT_0059679c,0x73b4);
                    FUN_00497567(0x140,0x64,pcVar1,iVar2,uVar3,iVar4,uVar5,puVar6,bVar7);
                }
                FUN_00497567(0xb4,0x1cc,(&DAT_005b709e + DAT_005b4a9c * 0x4e),0x91,0xcaccce,-0x1,0xcaccce,DAT_004d6a6c,0x19
                );
                FUN_0049e640(*(DAT_005b8c38 + 0x1d),*(DAT_005b8c38 + 0x21),*(DAT_005b8c38 + 0x25),
                             *(DAT_005b8c38 + 0x29),0xce,0xca,0xcc,0x1);
                FUN_0049e640(*(DAT_005b8c3c + 0x1d),*(DAT_005b8c3c + 0x21),*(DAT_005b8c3c + 0x25),
                             *(DAT_005b8c3c + 0x29),0xce,0xca,0xcc,0x1);
            }
            FUN_0049536f();
            return 0x1;
        }
        if (0x406 < param_2) {
            if (param_2 < 0x408) {
                if (param_3 == 0x64) {
                    if (_DAT_005b4a94 == 0x0) {
                        _DAT_005b4a94 = 0x1;
                        FUN_0049bf80(param_1,0x1f4,0x40f,0x0,0x0);
                        FUN_0049bf80(param_1,0x1f4,0x414,0x1,0x0);
                        FUN_0049bf80(param_1,0x1f5,0x40f,0x0,0x0);
                        FUN_0049bf80(param_1,0x1f5,0x414,0x1,0x0);
                        FUN_0049a770(param_1,0x405,0x0,0x0);
                    }
                    else {
                        FUN_0049c140(param_1,0x1);
                    }
                }
                return 0x0;
            }
            if (param_2 == 0x40c) {
                FUN_004953d7();
                FUN_004a08c5(s_pcx_evrebel_pcx_004c2328,0x0,0x0,0x280,0x1e0,0x0,0x0,0x0,0x1);
                FUN_0049536f();
                return 0x1;
            }
        }
    }
    return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0046e816(param_1: i32,param_2: i32,param_3: i32) -> u32

{
    let cVar1: u8;
    let mut iVar2: i32;
    let mut uVar3: u32;
    let mut pcVar4: String;
    let piVar5: *mut i32;;
    let mut pcVar6: String;
    let mut local_12c: i32;
    let mut local_128: String;
    let local_11c: u8 [0x104];
    i32 **local_18;
    let mut local_14: u32;

    _DAT_0059a9f8 = 0x0;
    if (0x4 < DAT_004c9754) {
        _DAT_0059a9f8 = 0x0;
        return 0x1;
    }
    local_14 = 0x1;
    local_18 = *(i32 ***)(&DAT_005b89f8 + param_1 * 0x4);
    while( true ) {
        if ((local_18 == 0x0) || (*(local_18 + 0x6) >> 0x10 != param_1)) {
            _DAT_0059a9f8 = 0x0;
            return 0x1;
        }
        if (((local_18[0x2] >> 0x10 == param_2) && (*(local_18 + 0xa) >> 0x10 == param_3)) &&
            ((((local_18 + 0xe) == 0xf || ((local_18 + 0xe) == 0x10)) ||
                ((local_18 + 0xe) == 0x2)))) break;
        local_18 = *local_18;
    }
    switch(*(local_18 + 0x5)) {
    case 0x1:
        iVar2 = FUN_0046ef1d();
    if (iVar2 != 0x0) {
        (&DAT_00569c30)[DAT_004c9754 * 0x1e22 + iVar2 * 0x9] = (&DAT_00569c30)[DAT_004c9754 * 0x1e22 + iVar2 * 0x9] & 0xfb
        ;
        (&DAT_00569c30)[iVar2 * 0x9 + DAT_004c9754 * 0x1e22] = (&DAT_00569c30)[iVar2 * 0x9 + DAT_004c9754 * 0x1e22] | 0x1;
        *(&DAT_00569c31 + DAT_004c9754 * 0x1e22 + iVar2 * 0x9) = 0x0;
        *(&DAT_00569c35 + iVar2 * 0x9 + DAT_004c9754 * 0x1e22) = 0x0;
        FUN_00463f7b();
        if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
            pcVar4 = FUN_00499050(DAT_0059679c,0x738c);
            FUN_0049c2e0(local_11c,pcVar4);
            FUN_0049d2e0(0x0,0x1,local_11c);
        }
        local_14 = 0x1;
        break;
    }
    *(local_18 + 0x5) = 0x3;
    uVar3 = FUN_004a2edc();
    (local_18 + 0x16) = (uVar3 % 0x3) + 0xa;
    *(local_18 + 0x1a) = 0x5;
    case 0x3:
    if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
        if ((local_18 + 0x1a) == 0x1) {
            pcVar4 = FUN_00499050(DAT_0059679c,0x7390);
            FUN_0049c2e0(local_11c,pcVar4);
        }
        else {
            pcVar4 = FUN_00499050(DAT_0059679c,0x7390);
            FUN_0049c2e0(local_11c,pcVar4);
        }
        FUN_0049d2e0(0x0,0x1,local_11c);
    }
    FUN_00465f06(param_1,param_2,param_3,local_18[0x5] >> 0x10,local_18[0x6] >> 0x10,DAT_004c9754,0x0);
    _DAT_0059a9f8 = 0x1;
    iVar2 = FUN_00432c94(&DAT_005967b8);
    local_14 = (iVar2 != 0x14);
    break;
    case 0x2:
    if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
        local_128 = FUN_0049c2c9(0x40);
        pcVar4 = *(&DAT_00582938 + (local_18[0x5] >> 0x10) * 0x18);
        pcVar6 = local_128;
        loop {
            cVar1 = *pcVar4;
            *pcVar6 = cVar1;
            if (cVar1 == '\0') break;
            cVar1 = pcVar4[0x1];
            pcVar4 = pcVar4 + 0x2;
            pcVar6[0x1] = cVar1;
            pcVar6 = pcVar6 + 0x2;
        } while (cVar1 != '\0');
        if (local_18[0x6] >> 0x10 < 0x2) {
            pcVar4 = FUN_00499050(DAT_0059679c,0x7393);
            FUN_0049c2e0(local_11c,pcVar4);
        }
        else {
            local_128 = FUN_00430c19(local_128,
                                     *(*(&DAT_00582938 + (local_18[0x5] >> 0x10) * 0x18) + 0x3d));
            pcVar4 = FUN_00499050(DAT_0059679c,0x738f);
            FUN_0049c2e0(local_11c,pcVar4);
        }
        FUN_0049d2e0(0x0,0x1,local_11c);
        FUN_0049af50(local_128);
    }
    FUN_0046fa64(*(local_18 + 0x6) >> 0x10,local_18[0x2] >> 0x10,*(local_18 + 0xa) >> 0x10,
                 (local_18[0x5] >> 0x10),'\0',local_18[0x6] >> 0x10,0x0,0x0,DAT_004c9754);
    case 0x4:
        iVar2 = FUN_00432c94(&DAT_005967b8);
    local_14 = (iVar2 != 0x14);
    for (local_12c = 0x0; local_12c < 0xb; local_12c = local_12c + 0x1) {
        if ((&DAT_004d7792 + local_12c * 0x67 + 0x3) == -0x1)^ // goto LAB_0046ecdb;
    }
    LAB_0046ed4a:
    if (local_12c == 0xb) {
        FUN_0046fa64(*(local_18 + 0x6) >> 0x10,local_18[0x2] >> 0x10,
                     *(local_18 + 0xa) >> 0x10,'[','\0',0x1,0xc,0xa,DAT_004c9754);
        if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
            pcVar4 = FUN_00499050(DAT_0059679c,0x7390);
            FUN_0049c2e0(local_11c,pcVar4);
            FUN_0049d2e0(0x0,0x1,local_11c);
        }
    }
    else {
        if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
            FUN_00483355(0x1c);
            pcVar4 = FUN_00499050(DAT_0059679c,0x7392);
            FUN_0049c2e0(local_11c,pcVar4);
            FUN_0049d2e0(0x0,0x1,local_11c);
        }
    }
    break;
    default:
    break;
}
    if ((*((*(local_18 + 0xa) >> 0x10) * 0x4 +
        *(&DAT_004d7d50 +
            (*(local_18 + 0x6) >> 0x10) * 0x3890 + (local_18[0x2] >> 0x10) * 0x4)) & 0xf)
        == 0xb) {
        piVar5 = ((*(local_18 + 0xa) >> 0x10) * 0x4 +
            *(&DAT_004d7d50 +
                (*(local_18 + 0x6) >> 0x10) * 0x3890 + (local_18[0x2] >> 0x10) * 0x4));
        *piVar5 = *piVar5 >> 0x9;
    }
    FUN_004811e6(local_18);
    return local_14;
    LAB_0046ecdb:
    loop {
    uVar3 = FUN_004a2edc();
} while ((&DAT_004d7792 + (uVar3 % 0xb) * 0x67 + 0x3) != -0x1);
    FUN_0046fa64(*(local_18 + 0x6) >> 0x10,local_18[0x2] >> 0x10,*(local_18 + 0xa) >> 0x10,
                 '\x1d','\0',0x1,(uVar3 % 0xb),0x0,DAT_004c9754);^
    // goto LAB_0046ed4a;
}



fn FUN_0046ef1d() -> i32

{
let piVar1: *mut i32;;
let mut uVar2: u32;
i32 aiStack484 [0x72];
let mut local_1c: i32;
let mut local_18: i32;
let mut local_14: i32;

local_1c = 0x0;
for (local_14 = 0x0; local_14 < 0x72; local_14 = local_14 + 0x1) {
if ((((&DAT_00569c30)[local_14 * 0x9 + DAT_004c9754 * 0x1e22] & 0x2) != 0x0) &&
(*(&DAT_0058ad72 + local_14 * 0xda) == 0x0)) {
piVar1 = aiStack484 + local_1c;
local_1c = local_1c + 0x1;
*piVar1 = local_14;
}
}
if (local_1c == 0x0) {
local_18 = local_1c;
}
else {
uVar2 = FUN_004a2edc();
local_18 = aiStack484[uVar2 % local_1c];
}
return local_18;
}



fn FUN_0046efbe(param_1: i32) -> i32

{
let mut uVar1: u32;
let mut iVar2: i32;

switch(*(param_1 + 0x14)) {
case 0x1:
*(param_1 + 0x16) = 0xffff;
*(param_1 + 0x1a) = 0xffff;
break;
case 0x2:
loop {
uVar1 = FUN_004a2edc();
iVar2 = uVar1 % 0x5c;
} while (*(*(&DAT_00582938 + iVar2 * 0x18) + 0x111) == 0x0);
(param_1 + 0x16) = iVar2;
iVar2 = *(&DAT_00582938 + iVar2 * 0x18);
uVar1 = FUN_004a2edc();
iVar2 = *(iVar2 + 0x111);
(param_1 + 0x1a) = (uVar1 % iVar2) + 0x1;
param_1 = uVar1 / iVar2;
break;
case 0x3:
uVar1 = FUN_004a2edc();
(param_1 + 0x16) = ((longlong)uVar1 % 0xd);
switch(*(param_1 + 0x16)) {
case 0x0:
case 0x1:
case 0x2:
case 0x3:
case 0x4:
*(param_1 + 0x1a) = 0x64;
break;
case 0x5:
case 0x6:
case 0x7:
case 0x8:
case 0x9:
*(param_1 + 0x1a) = 0x32;
break;
case 0xa:
case 0xb:
case 0xc:
*(param_1 + 0x1a) = 0x5;
break;
default:
*(param_1 + 0x1a) = 0x1;
}
break;
case 0x4:
*(param_1 + 0x16) = 0xffff;
*(param_1 + 0x1a) = 0xffff;
break;
default:
*(param_1 + 0x16) = 0xffff;
*(param_1 + 0x1a) = 0xffff;
}
return param_1;
}



fn FUN_0046f15b()

{
    let mut uVar1: u32;
    let mut iVar2: i32;
    let local_14: *mut u32;

    for (local_14 = *DAT_005967c8; local_14 != 0x0; local_14 = *local_14) {
        if ((local_14 + 0xe) == 0x2) {
            *(local_14 + 0x5) = 0x1;
        }
        else {
            if ((local_14 + 0xe) == 0xf) {
                uVar1 = FUN_004a2edc();
                (local_14 + 0x5) = (uVar1 % 0x4) + 0x1;
                FUN_0046efbe(local_14);
                uVar1 = FUN_004a2edc();
                iVar2 = uVar1 % 0x64;
                if (iVar2 < 0xf) {
                    *(local_14 + 0x22) = 0x0;
                }
                else {
                    if (iVar2 < 0x28) {
                        *(local_14 + 0x22) = 0x1;
                    }
                    else {
                        if (iVar2 < 0x32) {
                            *(local_14 + 0x22) = 0x2;
                        }
                        else {
                            if (iVar2 < 0x3c) {
                                *(local_14 + 0x22) = 0x3;
                            }
                            else {
                                if (iVar2 < 0x50) {
                                    *(local_14 + 0x22) = 0x4;
                                }
                                else {
                                    *(local_14 + 0x22) = 0x5;
                                }
                            }
                        }
                    }
                }
            }
            else {
                if ((local_14 + 0xe) == 0x10) {
                    uVar1 = FUN_004a2edc();
                    iVar2 = uVar1 % 0x64;
                    if (iVar2 < 0xb) {
                        *(local_14 + 0x5) = 0x3;
                        *(local_14 + 0x16) = 0xc;
                        *(local_14 + 0x1a) = 0xa;
                    }
                    else {
                        if (iVar2 < 0x32) {
                            *(local_14 + 0x5) = 0x3;
                            *(local_14 + 0x16) = 0xb;
                            *(local_14 + 0x1a) = 0x14;
                        }
                        else {
                            if (iVar2 < 0x5b) {
                                *(local_14 + 0x5) = 0x3;
                                *(local_14 + 0x16) = 0xa;
                                *(local_14 + 0x1a) = 0x1e;
                            }
                            else {
                                *(local_14 + 0x5) = 0x4;
                                *(local_14 + 0x16) = 0xffff;
                                *(local_14 + 0x1a) = 0xffff;
                            }
                        }
                    }
                    uVar1 = FUN_004a2edc();
                    iVar2 = uVar1 % 0x64;
                    if (iVar2 < 0x1e) {
                        *(local_14 + 0x22) = 0x0;
                    }
                    else {
                        if (iVar2 < 0x3c) {
                            *(local_14 + 0x22) = 0x1;
                        }
                        else {
                            if (iVar2 < 0x63) {
                                *(local_14 + 0x22) = 0x2;
                            }
                            else {
                                *(local_14 + 0x22) = 0x3;
                            }
                        }
                    }
                }
            }
        }
    }
    return;
}



fn FUN_0046f354(param_1: *mut i32) -> u32

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut uVar5: u32;
    let mut pcVar6: String;
    let mut local_34: i32;
    let mut local_30: i32;
    let mut local_2c: i32;

    iVar1 = *(param_1 + 0x6) >> 0x10;
    iVar2 = param_1[0x2] >> 0x10;
    iVar3 = *(param_1 + 0xa) >> 0x10;
    if ((param_1 + 0xe) == 0xf) {
        switch(*(param_1 + 0x22)) {
            case 0x0:
                FUN_0046fa64(iVar1,iVar2,iVar3,'\'','\0',0x1,0x0,0x0,0x7);
            FUN_0046fa64(iVar1,iVar2,iVar3,'&','\0',0x3,0x0,0x0,0x7);
            FUN_0046fa64(iVar1,iVar2,iVar3,'*','\0',0x2,0x0,0x0,0x7);
            FUN_0046fa64(iVar1,iVar2,iVar3,'+','\0',0x3,0x0,0x0,0x7);
            break;
            case 0x1:
                FUN_0046fa64(iVar1,iVar2,iVar3,'7','\0',0x3,0x0,0x0,0xd);
            FUN_0046fa64(iVar1,iVar2,iVar3,'W','\0',0x1,0x0,0x0,0xd);
            FUN_0046fa64(iVar1,iVar2,iVar3,'V','\0',0x1,0x0,0x0,0xd);
            FUN_0046fa64(iVar1,iVar2,iVar3,'U','\0',0x1,0x0,0x0,0xd);
            FUN_0046fa64(iVar1,iVar2,iVar3,'J','\0',0x3,0x0,0x0,0xd);
            break;
            case 0x2:
                FUN_0046fa64(iVar1,iVar2,iVar3,'T','\0',0x3,0x0,0x0,0xd);
            FUN_0046fa64(iVar1,iVar2,iVar3,'W','\0',0x2,0x0,0x0,0xd);
            FUN_0046fa64(iVar1,iVar2,iVar3,'V','\0',0x2,0x0,0x0,0xd);
            FUN_0046fa64(iVar1,iVar2,iVar3,'\x0f','\0',0x1,0x0,0x0,0xd);
            FUN_0046fa64(iVar1,iVar2,iVar3,'\r','\0',0x1,0x0,0x0,0xd);
            FUN_0046fa64(iVar1,iVar2,iVar3,'\x13','\0',0x2,0x0,0x0,0xd);
            break;
            case 0x3:
                FUN_0046fa64(iVar1,iVar2,iVar3,'3','\0',0x1,0x0,0x0,0xd);
            FUN_0046fa64(iVar1,iVar2,iVar3,'S','\0',0x5,0x0,0x0,0xd);
            FUN_0046fa64(iVar1,iVar2,iVar3,'W','\0',0x1,0x0,0x0,0xd);
            FUN_0046fa64(iVar1,iVar2,iVar3,'V','\0',0x1,0x0,0x0,0xd);
            FUN_0046fa64(iVar1,iVar2,iVar3,'U','\0',0x1,0x0,0x0,0xd);
            break;
            case 0x4:
                FUN_0046fa64(iVar1,iVar2,iVar3,';','\0',0x3,0x0,0x0,0xd);
            FUN_0046fa64(iVar1,iVar2,iVar3,'A','\0',0x1,0x0,0x0,0xd);
            FUN_0046fa64(iVar1,iVar2,iVar3,'C','\0',0x1,0x0,0x0,0xd);
            FUN_0046fa64(iVar1,iVar2,iVar3,'T','\0',0x3,0x0,0x0,0xd);
            FUN_0046fa64(iVar1,iVar2,iVar3,'W','\0',0x1,0x0,0x0,0xd);
            break;
            case 0x5:
                FUN_0046fa64(iVar1,iVar2,iVar3,'-','\0',0x1,0x0,0x0,0xd);
            FUN_0046fa64(iVar1,iVar2,iVar3,'W','\0',0x3,0x0,0x0,0xd);
            FUN_0046fa64(iVar1,iVar2,iVar3,'V','\0',0x2,0x0,0x0,0xd);
            FUN_0046fa64(iVar1,iVar2,iVar3,'U','\0',0x2,0x0,0x0,0xd);
            FUN_0046fa64(iVar1,iVar2,iVar3,'R','\0',0x4,0x0,0x0,0xd);
        }
    }
    else {
        if ((param_1 + 0xe) == 0x10) {
            for (local_2c = 0x0; local_2c < 0x5; local_2c = local_2c + 0x1) {
                iVar4 = FUN_0046031c(local_2c,0x0);
                (&DAT_004d566c + DAT_00591cb4 * 0x2 + local_2c * 0xc8) = iVar4;
            }
            local_30 = 0x8000;
            local_34 = -0x1;
            for (local_2c = 0x0; local_2c < 0x5; local_2c = local_2c + 0x1) {
                if (*(&DAT_004d566a + local_2c * 0xc8 + DAT_00591cb4 * 0x2) >> 0x10 < local_30) {
                    local_30 = *(&DAT_004d566a + DAT_00591cb4 * 0x2 + local_2c * 0xc8) >> 0x10;
                    local_34 = local_2c;
                }
            }
            if (local_34 == DAT_004c9754) {
                uVar5 = FUN_004a2edc();
                FUN_00482d56(DAT_004c9754,uVar5 % 0x3 + 0x3de);
                if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
                    pcVar6 = FUN_00499050(DAT_0059679c,0x73bf);
                    FUN_0049d2e0(0x0,0x1,pcVar6);
                }
                FUN_004811e6(param_1);
            }
            else {
                switch(*(param_1 + 0x22)) {
                    case 0x0:
                        FUN_0046fa64(iVar1,iVar2,iVar3,'.','\0',0x1,0x0,0x0,0x7);
                    FUN_0046fa64(iVar1,iVar2,iVar3,'+','\0',0x2,0x0,0x0,0x7);
                    FUN_0046fa64(iVar1,iVar2,iVar3,'O','\0',0x3,0x0,0x0,0x7);
                    break;
                    case 0x1:
                        FUN_0046fa64(iVar1,iVar2,iVar3,'.','\0',0x6,0x0,0x0,0x7);
                    FUN_0046fa64(iVar1,iVar2,iVar3,'O','\0',0x6,0x0,0x0,0x7);
                    break;
                    case 0x2:
                        FUN_0046fa64(iVar1,iVar2,iVar3,'X','\0',0x1,0x0,0x0,0xd);
                    FUN_0046fa64(iVar1,iVar2,iVar3,'O','\0',0x2,0x0,0x0,0xd);
                    FUN_0046fa64(iVar1,iVar2,iVar3,'P','\x01',0x2,0x0,0x0,0xd);
                    break;
                    case 0x3:
                        FUN_0046fa64(iVar1,iVar2,iVar3,'.','\0',0xd,0x0,0x0,0x7);
                }
            }
        }
    }
    return 0x1;
}



void
FUN_0046fa64(param_1: u32,param_2: u32,param_3: u32,param_4: u8,param_5: u8,param_6: i32,
let param_7: u8,undefined2 param_8,param_9: i32)

{
let mut iVar1: i32;
let puVar2: *mut u32;
let puVar3: *mut u32;
let in_stack_ffffff44: i16;
let mut in_stack_ffffff48: u32;
u32 local_84 [0xc];
let mut local_54: u32;
let local_50: u16;
let local_4e: u8;
let local_4d: u8;
let local_4c: u8;
let local_4a: u16;
let local_46: u8;
let local_45: u8;
let local_44: u8;
let local_43: u16;
let local_41: u8;
let local_40: u8;
let local_3f: u8;
let mut local_3e: u32;
let mut local_3a: u32;
let local_36: u8;
let local_35: u8;
let mut local_24: i32;
undefined2 *local_20;
let mut local_1c: u32;
let mut local_18: u32;
let mut local_14: u32;

FUN_00486065(&local_54);
local_20 = &local_54;
local_1c = param_1;
local_18 = param_2;
local_14 = param_3;
local_54._0_2_ = (undefined2)param_1;
local_54._2_2_ = (undefined2)param_2;
local_50 = (undefined2)param_3;
local_4d = param_4;
local_4c = param_5;
FUN_00486b6b(&local_54,0x4b);
local_4a = *(*(&DAT_00582938 + local_4d * 0x18 + local_4c * 0x4) + 0x41);
local_45 = (*(&DAT_00582938 + local_4d * 0x18 + local_4c * 0x4) + 0x45);
local_46 = 0x0;
local_44 = param_7;
local_43 = param_8;
local_41 = 0x64;
local_40 = (&DAT_00569ac9)[param_9 * 0x1e22];
local_4e = param_9;
local_3f = param_9;
local_3e = 0x0;
local_3a = local_3a | 0x1;
local_36 = (*(&DAT_00582938 + local_4c * 0x4 + local_4d * 0x18) + 0xed);
local_35 = 0x0;
for (local_24 = 0x0; local_24 < param_6; local_24 = local_24 + 0x1) {
puVar2 = &local_54;
puVar3 = local_84;
for (iVar1 = 0xb; iVar1 != 0x0; iVar1 = iVar1 + -0x1) {
*puVar3 = *puVar2;
puVar2 = puVar2 + 0x1;
puVar3 = puVar3 + 0x1;
}
puVar3 = puVar2;
puVar2 = local_84;
puVar3 = &stack0xffffff44;
for (iVar1 = 0xb; iVar1 != 0x0; iVar1 = iVar1 + -0x1) {
*puVar3 = *puVar2;
puVar2 = puVar2 + 0x1;
puVar3 = puVar3 + 0x1;
}
puVar3 = puVar2;
FUN_00485463(in_stack_ffffff44,in_stack_ffffff48);
}
return;
}



fn FUN_0046fba7(param_1: i32)

{
    let mut pcVar1: String;
    let local_30: u8 [0x20];

    FUN_00497567(0x3c,0xac,(&DAT_005b709e + param_1 * 0x4e),0x5a,0xcaccce,0xe0e0e,0xcaccce,DAT_004d6a6c,0x11);
    pcVar1 = FUN_00499050(DAT_0059679c,0x7348);
    FUN_0049c2e0(local_30,pcVar1);
    FUN_00497567(0x3c,0xb8,local_30,0x5a,0xcaccce,0xe0e0e,0xcaccce,DAT_004d6a6c,0x11);
    return;
}



fn FUN_0046fc49()

{
    let sVar1: i16;
    let sVar2: i16;
    let sVar3: i16;
    let mut bVar4: bool;
    let mut pcVar5: String;
    let mut iVar6: i32;
    let mut iVar7: i32;
    let local_58: *mut i32;;
    let local_4c: *mut i32;;
    let local_40: *mut i32;;
    let local_34: *mut u32;
    i32 **local_24;

    if ((DAT_004c9754 < 0x5) && (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0)) {
        DAT_004c50b4 = FUN_0049c2c9(0xe80);
        FUN_0049c57b(s_bin_fire_bin_004c2338,DAT_004c50b4,0xe80);
        local_34 = FUN_004a2831(0x49);
        if (local_34 != 0x0) {
            local_34 = FUN_0049a030(local_34,0x0,0x1,0x0,0x0,0x280,0x1e0,0x4000,0x0,0x0);
        }
        DAT_005b4aac = local_34;
        FUN_0049ae00(local_34,FUN_00470ee2);
        FUN_0049a8a0(DAT_005b4aac);
        local_40 = FUN_004a2831(0x95);
        if (local_40 != 0x0) {
            local_40 = FUN_0047157e(local_40,&DAT_005b4aac,0x1f5,0xa,0x3,0x64,0x60,0x40,0x2);
        }
        DAT_005b4aa8 = local_40;
        local_4c = FUN_004a2831(0xb9);
        if (local_4c != 0x0) {
            local_4c = FUN_00438792(local_4c,&DAT_005b4aac,0x4b0,0x7c,0xc,0x1f8,0x1a4,0x4009,0x1c,0x28,0x0);
        }
        DAT_005b4aa0 = local_4c;
        local_58 = FUN_004a2831(0xb9);
        if (local_58 != 0x0) {
            local_58 = FUN_00438792(local_58,&DAT_005b4aac,0x4b1,0x10,0x67,0x58,0x3f,0x4042,0x2,0x2,0x0);
        }
        DAT_005b4aa4 = local_58;
    }
    DAT_00599d3c = 0x1;
    bVar4 = true;
    sVar3 = -0x1;
    sVar1 = -0x1;
    sVar2 = -0x1;
    for (local_24 = *DAT_005967b0; local_24 != 0x0; local_24 = *local_24) {
        if (((((*(local_24 + 0x3a) & 0x1) != 0x0) && ((*(local_24 + 0x3a) & 0x80000000) != 0x0))
            && (*(local_24 + 0x23) >> 0x18 == DAT_004c9754)) && ((local_24 + 0x2a) == 0x3)) {
            if (((DAT_004c9754 < 0x5) && (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0)) &&
                (((local_24 + 0x22) != sVar1 || ((local_24 + 0x9) != sVar2)))) {
                sVar1 = (local_24 + 0x22);
                sVar2 = (local_24 + 0x9);
                FUN_00450dbf(&DAT_00595740,(local_24 + 0x8),(local_24 + 0x22),
                             (local_24 + 0x9),0x0,0x1,-0x1,0x0);
                if ((local_24 + 0x8) != sVar3) {
                    FUN_004390ae(DAT_005b4aa0,(local_24 + 0x8));
                    FUN_004390ae(DAT_005b4aa4,(local_24 + 0x8));
                    FUN_00439d27(DAT_005b4aa4,0x0);
                    FUN_00471bcd(DAT_005b4aa8);
                    FUN_0046fba7((local_24 + 0x8));
                    sVar3 = (local_24 + 0x8);
                }
                if (bVar4) {
                    bVar4 = false;
                    FUN_0044783a(DAT_005b4aa0,(local_24 + 0x22),(local_24 + 0x9),0x0);
                    ((*(DAT_005b4aac + 0x45) + 0xc))(DAT_005b4aac,DAT_005b4aac,0x414,0x1,0x0);
                }
                else {
                    FUN_0044783a(DAT_005b4aa0,(local_24 + 0x22),(local_24 + 0x9),0x1);
                }
                iVar7 = 0xa;
                iVar6 = -0x1;
                pcVar5 = FUN_00499050(DAT_0059679c,0x7380);
                FUN_0042fefc(-0x1,0x5dc,pcVar5,iVar6,iVar7);
                FUN_00447607(DAT_005b4aa0,(local_24 + 0x22),(local_24 + 0x9));
                FUN_0040c771(DAT_004c50b4,local_24[0x9] >> 0x18,*(local_24 + 0x23) >> 0x18,
                             local_24[0xc] >> 0x18,*(DAT_005b4aa0 + 0x85) + -0x4,
                             *(DAT_005b4aa0 + 0x89) + 0x3,0x64);
                timer_func_0049e710(0x3e8);
            }
            local_24 = FUN_00484b4e(local_24);
        }
    }
    FUN_004864f7();
    if ((DAT_004c9754 < 0x5) && (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0)) {
        FUN_0049ab50(DAT_005b4aac,0x0);
        FUN_004953d7();
        FUN_00496a10();
        FUN_0049536f();
        if (DAT_005b4aa8 != 0x0) {
            ((*(DAT_005b4aa8 + 0x45) + 0x8))(DAT_005b4aa8,0x2);
        }
        if (DAT_005b4aa0 != 0x0) {
            ((*(DAT_005b4aa0 + 0x45) + 0x8))(DAT_005b4aa0,0x2);
        }
        if (DAT_005b4aa4 != 0x0) {
            ((*(DAT_005b4aa4 + 0x45) + 0x8))(DAT_005b4aa4,0x2);
        }
        FUN_0049af50(DAT_004c50b4);
        if (DAT_005b4aac != 0x0) {
            ((*(DAT_005b4aac + 0x45) + 0x8))(DAT_005b4aac,0x2);
        }
    }
    return;
}



fn FUN_0047051a()

{
    let puVar1: *mut u32;
    let cVar2: u8;
    let mut iVar3: i32;
    let mut pcVar4: String;
    let mut pcVar5: String;
    let puVar6: *mut u32;
    let puVar7: *mut u32;
    let mut pcVar8: String;
    let in_stack_fffffd98: i16;
    let mut in_stack_fffffd9c: u32;
    let mut local_210: i32;
    let mut local_20c: u32;
    let mut local_208: u32;
    let mut local_204: u32;
    let mut local_200: u32;
    let mut local_1fc: i32;
    let mut local_1f8: i32;
    let local_1f4: *mut i32;;
    u32 local_1f0 [0xc];
    let local_1c0: u8 [0x100];
    let mut local_c0: String;
    let mut local_bc: String;
    let mut local_b8: String;
    let mut local_b4: i32;
    let mut local_b0: u32;
    let mut local_ac: u32;
    let mut local_a8: u32;
    let mut local_a4: u32;
    let local_a0: *mut u32;
    ushort local_9c;
    ushort local_9a;
    ushort local_98;
    let local_96: u8;
    let local_95: u8;
    let local_94: u8;
    let local_92: u16;
    let local_8e: u8;
    let local_8d: u8;
    let local_8c: u8;
    let local_8b: u16;
    let local_89: u8;
    let local_88: u8;
    let local_87: u8;
    let mut local_86: u32;
    let mut local_82: u32;
    let local_7e: u8;
    let local_7d: u8;
    i32 **local_6c;
    let mut local_68: u32;
    let mut local_64: i32;
    let mut local_60: i32;
    let local_5c: *mut i32;;
    let local_58: *mut i32;;
    let local_54: *mut i32;;
    let local_50: *mut i32;;
    let local_4c: *mut i32;;
    let local_48: *mut i32;;
    let local_44: *mut i32;;
    let local_40: *mut i32;;
    let local_3c: *mut i32;;
    let local_38: *mut i32;;
    let local_34: *mut u32;
    let local_30: *mut u32;
    let local_2c: *mut u32;
    let mut local_28: i32;
    let local_24: *mut u32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: i32;

    if ((DAT_004c9754 < 0x5) && (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0)) {
        local_34 = FUN_004a2831(0x49);
        local_30 = local_34;
        local_2c = local_34;
        if (local_34 != 0x0) {
            local_34 = FUN_0049a030(local_34,0x0,0x1,0x0,0x0,0x280,0x1e0,0x4000,0x0,0x0);
        }
        DAT_005b4aac = local_34;
        FUN_0049ae00(local_34,FUN_00470ee2);
        FUN_0049a8a0(DAT_005b4aac);
        local_40 = FUN_004a2831(0x95);
        local_3c = local_40;
        local_38 = local_40;
        if (local_40 != 0x0) {
            local_40 = FUN_0047157e(local_40,&DAT_005b4aac,0x1f5,0xa,0x3,0x64,0x60,0x40,0x2);
        }
        DAT_005b4aa8 = local_40;
        local_4c = FUN_004a2831(0xb9);
        local_48 = local_4c;
        local_44 = local_4c;
        if (local_4c != 0x0) {
            local_4c = FUN_00438792(local_4c,&DAT_005b4aac,0x4b0,0x7c,0xc,0x1f8,0x1a4,0x4009,0x1c,0x28,0x0);
        }
        DAT_005b4aa0 = local_4c;
        local_58 = FUN_004a2831(0xb9);
        local_54 = local_58;
        local_50 = local_58;
        if (local_58 != 0x0) {
            local_58 = FUN_00438792(local_58,&DAT_005b4aac,0x4b1,0x10,0x67,0x58,0x3f,0x4042,0x2,0x2,0x0);
        }
        DAT_005b4aa4 = local_58;
    }
    local_20 = 0x1;
    local_28 = -0x1;
    local_1c = -0x1;
    local_18 = -0x1;
    local_24 = *DAT_005967c8;
    loop {
    if (local_24 == 0x0) {
        if (local_1c != -0x1) {
            puVar1 = (*(*(DAT_005b4aa0 + 0xa1) + local_1c * 0x4) + local_18 * 0x4 + 0x4);
            *puVar1 = *puVar1 & 0xffffefff;
        }
        if ((DAT_004c9754 < 0x5) && (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0)) {
            FUN_0049ab50(DAT_005b4aac,0x0);
            FUN_004953d7();
            FUN_00496a10();
            FUN_0049536f();
            if (DAT_005b4aa8 != 0x0) {
                ((*(DAT_005b4aa8 + 0x45) + 0x8))();
            }
            if (DAT_005b4aa0 != 0x0) {
                ((*(DAT_005b4aa0 + 0x45) + 0x8))();
            }
            if (DAT_005b4aa4 != 0x0) {
                ((*(DAT_005b4aa4 + 0x45) + 0x8))();
            }
            if (DAT_005b4aac != 0x0) {
                ((*(DAT_005b4aac + 0x45) + 0x8))();
            }
        }
        return;
    }
    if (((*(local_24 + 0xe) >> 0x10 == DAT_004c9754) && ((local_24 + 0x5) != -0x1)) &&
        ((local_24 + 0x6) = (local_24 + 0x6) + -0x1, (local_24 + 0x6) == 0x0)) {
        if (local_1c != -0x1) {
            local_5c = DAT_005b4aa0;
            local_60 = local_1c;
            local_64 = local_18;
            local_68 = 0x1000;
            puVar1 = (*(*(DAT_005b4aa0 + 0xa1) + local_1c * 0x4) + local_18 * 0x4 + 0x4);
            *puVar1 = *puVar1 & 0xffffefff;
        }
        local_6c = 0x0;
        FUN_00486065(&local_9c);
        local_a0 = &local_9c;
        local_9c = (local_24 + 0x2);
        local_a4 = local_24 & 0xffff0000 | local_9c;
        local_9a = (local_24 + 0xa);
        local_a8 = local_24 & 0xffff0000 | local_9a;
        local_98 = (local_24 + 0x3);
        local_ac = local_24 & 0xffff0000 | local_98;
        local_95 = (local_24 + 0x5);
        local_94 = (local_24 + 0x16);
        FUN_00486b6b(&local_9c,(local_24[0x7] >> 0x10) + (local_24[0x7] >> 0x11));
        local_92 = *(*(&DAT_00582938 + local_95 * 0x18 + local_94 * 0x4) + 0x41);
        local_8d = (*(&DAT_00582938 + local_94 * 0x4 + local_95 * 0x18) + 0x45);
        local_8e = (&DAT_00569b75)[DAT_004c9754 * 0x1e22] != '\0';
        local_b0 = (byte)local_8e;
        local_8c = 0x0;
        local_8b = 0x0;
        local_89 = 0x64;
        local_88 = (&DAT_00569ac9)[DAT_004c9754 * 0x1e22];
        local_96 = DAT_004c9754;
        if ((*(local_24 + 0x2d) & 0x1) == 0x0) {
            iVar3 = FUN_004888c0(*(local_24 + 0x6) >> 0x10,local_24[0x2] >> 0x10,
                                 *(local_24 + 0xa) >> 0x10,0x0);
            local_87 = iVar3;
        }
        else {
            local_87 = (local_24 + 0x12);
            local_82 = local_82 | 0x80;
        }
        if (local_87 == -0x1) {
            local_87 = DAT_004c9754;
            local_82 = local_82 & 0xffffff7f;
        }
        local_86 = 0x0;
        local_82 = local_82 | 0x8001;
        local_7e = (local_24 + 0x22);
        local_7d = (local_24 + 0x9);
        local_b4 = 0x1;
        local_bc = *(&DAT_00582938 + local_94 * 0x4 + local_95 * 0x18);
        local_b8 = local_bc;
        iVar3 = FUN_00430d15(local_bc);
        if (iVar3 == 0x0) {
            local_c0 = FUN_00499050(DAT_0059679c,0x7132);
        }
        else {
            local_c0 = FUN_00499050(DAT_0059679c,0x7133);
        }
        pcVar4 = FUN_00499050(DAT_0059679c,0x7343);
        FUN_0049c2e0(local_1c0,pcVar4);
        iVar3 = FUN_004889db(&local_9c,0x0);
        if (iVar3 == 0x0) {
            local_b4 = 0x0;
            *(local_24 + 0x6) = 0x1;
            pcVar5 = FUN_00499050(DAT_0059679c,0x7402);
            iVar3 = -0x1;
            pcVar4 = local_1c0;
            loop {
                pcVar8 = pcVar4;
                if (iVar3 == 0x0) break;
                iVar3 = iVar3 + -0x1;
                pcVar8 = pcVar4 + 0x1;
                cVar2 = *pcVar4;
                pcVar4 = pcVar8;
            } while (cVar2 != '\0');
            pcVar8 = pcVar8 + -0x1;
            loop {
                cVar2 = *pcVar5;
                *pcVar8 = cVar2;
                if (cVar2 == '\0') break;
                cVar2 = pcVar5[0x1];
                pcVar5 = pcVar5 + 0x2;
                pcVar8[0x1] = cVar2;
                pcVar8 = pcVar8 + 0x2;
            } while (cVar2 != '\0');
        }
        else {
            puVar6 = &local_9c;
            puVar7 = local_1f0;
            for (iVar3 = 0xb; iVar3 != 0x0; iVar3 = iVar3 + -0x1) {
                *puVar7 = *puVar6;
                puVar6 = puVar6 + 0x1;
                puVar7 = puVar7 + 0x1;
            }
            puVar7 = puVar6;
            puVar6 = local_1f0;
            puVar7 = &stack0xfffffd98;
            for (iVar3 = 0xb; iVar3 != 0x0; iVar3 = iVar3 + -0x1) {
                *puVar7 = *puVar6;
                puVar6 = puVar6 + 0x1;
                puVar7 = puVar7 + 0x1;
            }
            puVar7 = puVar6;
            local_6c = FUN_00485463(in_stack_fffffd98,in_stack_fffffd9c);
            *(local_24 + 0x6) = 0xffff;
            *(local_24 + 0x5) = 0xffff;
            *(local_24 + 0x16) = 0xffff;
        }
        if ((DAT_004c9754 < 0x5) && (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0)) {
            if (*(local_24 + 0x6) >> 0x10 != local_28) {
                FUN_004390ae(DAT_005b4aa0,*(local_24 + 0x6) >> 0x10);
                FUN_004390ae(DAT_005b4aa4,*(local_24 + 0x6) >> 0x10);
                FUN_00471bcd(DAT_005b4aa8);
                FUN_00439d27(DAT_005b4aa4,0x0);
                FUN_0046fba7(*(local_24 + 0x6) >> 0x10);
            }
            local_1f8 = local_24[0x2] >> 0x10;
            local_1fc = *(local_24 + 0xa) >> 0x10;
            local_1f4 = DAT_005b4aa0;
            local_200 = 0x1000;
            puVar1 = (*(*(DAT_005b4aa0 + 0xa1) + local_1f8 * 0x4) + local_1fc * 0x4 + 0x4);
            *puVar1 = *puVar1 | 0x1000;
            local_1c = local_1f8;
            local_18 = local_1fc;
            if (local_20 == 0x0) {
                FUN_0044783a(DAT_005b4aa0,local_24[0x2] >> 0x10,*(local_24 + 0xa) >> 0x10,0x1);
            }
            else {
                local_20 = 0x0;
                FUN_0044783a(DAT_005b4aa0,local_24[0x2] >> 0x10,*(local_24 + 0xa) >> 0x10,0x0);
                ((*(DAT_005b4aac + 0x45) + 0xc))();
            }
            FUN_00450dbf(&DAT_00595740,*(local_24 + 0x6) >> 0x10,local_24[0x2] >> 0x10,
                         *(local_24 + 0xa) >> 0x10,0x0,0x1,-0x1,0x0);
            FUN_0048fe33(&DAT_00595740,0x1);
            FUN_0049d2e0(0x0,0x1,local_1c0);
            if (local_b4 != 0x0) {
                if (*(local_24 + 0xa) >> 0x10 < 0xf) {
                    FUN_00401796(local_24,0x0,0x1,0x7c,0xe6);
                }
                else {
                    FUN_00401796(local_24,0x0,0x1,0x7c,0x0);
                }
                if (local_6c != 0x0) {
                    *(local_6c + 0x3b) = *(local_6c + 0x3b) & 0x7f;
                }
                local_210 = 0x75;
                local_20c = 0x1b7;
                local_208 = 0x208;
                local_204 = 0x28;
                FUN_00498a5b(&local_210);
                FUN_0049a770(DAT_005b4aac,0x405,0x1,&local_210);
                FUN_00498ae4();
            }
        }
    }
    local_24 = *local_24;
} while( true );
}



fn FUN_00470ee2(param_1: i32,param_2: i32,param_3: u32,param_4: u32) -> u32

{
    let mut local_18: u32;

    if (param_2 == 0x405) {
        FUN_004953d7();
        FUN_004a08c5(s_pcx_aiplat_pcx_004c2345,0x0,0x0,0x280,0x1e0,0x0,0x0,0x0,0x1);
        FUN_00439d27(DAT_005b4aa4,0x0);
        FUN_00439d27(DAT_005b4aa0,0x0);
        FUN_00471bcd(DAT_005b4aa8);
        FUN_0046fba7(*(DAT_005b4aa0 + 0xa9));
        FUN_0043be20(*(DAT_005b4aa0 + 0xa9));
        if ((DAT_00595f68 & 0x1) == 0x0) {
            FUN_0048fe33(&DAT_00595740,0x1);
        }
        FUN_0049536f();
        local_18 = 0x1;
    }
    else {
        local_18 = ((*(param_1 + 0x45) + 0xc))(param_1,param_1,param_2,param_3,param_4);
    }
    return local_18;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00470ff6() -> u32

{
    let mut local_18: u32;
    let mut local_14: i32;

    DAT_005b4ab4 = FUN_0049c2c9(0xc8);
    DAT_005b4ab8 = FUN_0049c2c9(0x960);
    if ((DAT_005b4ab4 == 0x0) || (DAT_005b4ab8 == 0x0)) {
        local_18 = 0x0;
    }
    else {
        FUN_004a0430(DAT_005b4ab8,0xff,0x960);
        for (local_14 = 0x0; local_14 < 0x32; local_14 = local_14 + 0x1) {
            *(DAT_005b4ab4 + local_14 * 0x4) = local_14 * 0x30 + DAT_005b4ab8;
        }
        _DAT_005b4abc = 0x1;
        local_18 = 0x1;
    }
    return local_18;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_004710a6(param_1: u32,param_2: i32,param_3: i32,param_4: i32,param_5: i32) -> u32

{
    let mut iVar1: i32;
    let mut local_54: i32;
    let mut local_50: i32;
    let local_40: u8;
    let local_3f: u8;
    let local_3d: u8;
    let mut local_30: u32;
    let cStack44: u8;
    let mut local_20: u32;
    let mut local_18: u32;
    let mut local_14: i32;

    if ((param_4 == 0xff) || (param_5 == 0xff)) {
        local_18 = 0x0;
    }
    else {
        if ((_DAT_005b4abc == 0x0) && (iVar1 = FUN_00470ff6(), iVar1 == 0x0)) {
            local_18 = 0xffffffff;
        }
        else {
            for (local_14 = 0x0; local_14 < 0x32; local_14 = local_14 + 0x1) {
                FUN_004a0430(*(local_14 * 0x4 + DAT_005b4ab4),0xff,0x30);
            }
                (*(param_4 * 0x4 + DAT_005b4ab4) + param_5) = 0x0;
            local_20 = CONCAT13(param_4,0x10000);
            local_20 = local_20 | 0x1;
            FUN_00462b69(&DAT_005967d0);
            FUN_00462b8d(&DAT_005967d0,local_20,param_5);
            while( true ) {
                FUN_00462c31(&DAT_005967d0);
                local_40 = (byte)local_30;
                local_3f = (byte)(local_30 >> 0x8);
                local_3d = (local_30 >> 0x18);
                if (local_40 == 0x0) break;
                if (*(*(DAT_005b4ab4 + param_2 * 0x4) + param_3) <= local_40) {
                    return *(*(DAT_005b4ab4 + param_2 * 0x4) + param_3);
                }
                for (local_14 = 0x0; local_14 < 0x78; local_14 = local_14 + 0x1) {
                    if ((*(&DAT_005b7ca8 + local_14 * 0x14) == local_3d) &&
                        (*(&DAT_005b7cac + local_14 * 0x14) == cStack44)) {
                        local_50 = *(&DAT_005b7cb0 + local_14 * 0x14);
                        local_54 = *(&DAT_005b7cb4 + local_14 * 0x14);
                        LAB_0047134e:
                        if (local_3f + 0x1 < *(*(local_50 * 0x4 + DAT_005b4ab4) + local_54)) {
                            local_20 = local_20 | (byte)(local_3f + 0x2);
                            FUN_00462b8d(&DAT_005967d0,local_20,local_54);
                            *(*(local_50 * 0x4 + DAT_005b4ab4) + local_54) = local_3f + 0x1;
                        }
                    }
                    else {
                        if ((*(&DAT_005b7cb0 + local_14 * 0x14) == local_3d) &&
                            (*(&DAT_005b7cb4 + local_14 * 0x14) == cStack44)) {
                            local_50 = *(&DAT_005b7ca8 + local_14 * 0x14);
                            local_54 = *(&DAT_005b7cac + local_14 * 0x14);^
                            // goto LAB_0047134e;
                        }
                    }
                }
            }
            DAT_005b4ab0 = FUN_0049c4bd(s_misc_debug_txt_004c2357,&DAT_004c2354);
            if (DAT_005b4ab0 != 0x0) {
                FUN_004a9b70(DAT_005b4ab0,s____startx____d__starty____d__dx___004c2366);
                FUN_004a9b70(DAT_005b4ab0,s__Elem____d__d__d__d_004c2395);
                FUN_004a9b70(DAT_005b4ab0,s__Temp____d__d__d__d_004c23ac);
                FUN_0049ca40(DAT_005b4ab0);
            }
            local_18 = 0x0;
        }
    }
    return local_18;
}



fn FUN_004713df(param_1: u32,param_2: i32,param_3: i32,param_4: i32,param_5: *mut i32,param_6: *mut i32) -> String

{
    u32 auStack524 [0x7a];
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_1c: i32;

    *param_5 = -0x1;
    *param_6 = -0x1;
    local_24 = 0x0;
    loop {
    if (0x77 < local_24) {
        return (*(*param_5 * 0x4 + DAT_005b4ab4) + *param_6);
    }
    if ((*(&DAT_005b7ca8 + local_24 * 0x14) == param_3) && (*(&DAT_005b7cac + local_24 * 0x14) == param_4)
    ) {
        local_20 = *(&DAT_005b7cb0 + local_24 * 0x14);
        local_1c = *(&DAT_005b7cb4 + local_24 * 0x14);
        LAB_004714af:
        if (*(*(local_20 * 0x4 + DAT_005b4ab4) + local_1c) + 0x1 == param_2) {
            *param_5 = local_20;
            *param_6 = local_1c;
            auStack524[local_24] = 0x1;
        }
        else {
            auStack524[local_24] = 0xffffffff;
        }
    }
    else {
        if ((*(&DAT_005b7cb0 + local_24 * 0x14) == param_3) &&
            (*(&DAT_005b7cb4 + local_24 * 0x14) == param_4)) {
            local_20 = *(&DAT_005b7ca8 + local_24 * 0x14);
            local_1c = *(&DAT_005b7cac + local_24 * 0x14);^
            // goto LAB_004714af;
        }
        auStack524[local_24] = 0xffffffff;
    }
    local_24 = local_24 + 0x1;
} while( true );
}



fn FUN_00471540()

{
    FUN_004a70a2(&DAT_005b8608,0x80,&DAT_004c3f50);
    FUN_004a70ea(&DAT_004bf7d0);
    return;
}



i32 *
FUN_0047157e(param_1: *mut u32,param_2: i32,param_3: u32,param_4: i32,param_5: i32,param_6: i32,param_7: i32,
param_8: u32,param_9: u32)

{
let piVar1: *mut i32;;
i32 **ppiVar2;

piVar1 = FUN_0049a030(param_1,param_2,param_3,param_4,param_5,param_6,param_7,param_8,0xf,0x0);
*(piVar1 + 0x49) = 0x0;
*(piVar1 + 0x4d) = 0x0;
$1: &mut String(piVar1 + 0x45) = &PTR_FUN_004c3f34;
*(piVar1 + 0x2e) = *(piVar1 + 0x2e) | 0x10;
if ((*(piVar1 + 0x2d) & 0x2) != 0x0) {
*(piVar1 + 0x51) = 0xf;
*(piVar1 + 0x55) = 0xd;
}
*(piVar1 + 0x8d) = 0x0;
*(piVar1 + 0x91) = 0x0;
*(piVar1 + 0x81) = param_9;
*(piVar1 + 0x71) = 0xd;
*(piVar1 + 0x75) = 0xf;
*(piVar1 + 0x61) = *(piVar1 + 0x81);
*(piVar1 + 0x65) = *(piVar1 + 0x81);
ppiVar2 = FUN_004a2831(0x10);
if (ppiVar2 != 0x0) {
FUN_004a2874(ppiVar2,piVar1,0x96);
}
FUN_00472186();
*(&DAT_005b7040 + DAT_005b706c * 0x4) = piVar1;
DAT_005b706c = DAT_005b706c + 0x1;
return piVar1;
}



fn FUN_004716c6(param_1: &mut String,byte param_2) -> String

{
    let piVar1: *mut i32;;
    let mut local_18: i32;

    if ((param_2 & 0x4) == 0x0) {
        $1: &mut String(param_1 + 0x45) = &PTR_FUN_004c3f34;
        FUN_004a2965(param_1);
        for (local_18 = 0x0; local_18 < DAT_005b706c; local_18 = local_18 + 0x1) {
            if (param_1 == *(LPCSTR **)(&DAT_005b7040 + local_18 * 0x4)) {
                DAT_005b706c = DAT_005b706c + -0x1;
                *(&DAT_005b7040 + local_18 * 0x4) = *(&DAT_005b7040 + DAT_005b706c * 0x4);
                break;
            }
        }
        param_1 = FUN_0049a1c0(param_1,0x1);
        if ((param_2 & 0x2) != 0x0) {
            FUN_0049af50(param_1);
        }
    }
    else {
        piVar1 = FUN_00498dce(param_1,&DAT_004c3f70);
        FUN_00498df5(piVar1);
    }
    return param_1;
}



fn FUN_00471791(param_1: i32,param_2: *mut u8,param_3: u32,param_4: i32,param_5: i32) -> u32

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut uVar3: u32;
    let mut local_84: i32;
    let mut local_80: i32;
    let mut local_7c: u32;
    let mut local_78: u32;
    let mut local_74: u32;
    let mut local_70: u32;
    let mut local_6c: u32;
    let mut local_68: u32;
    let mut local_64: u32;
    let local_60: u8;
    let local_5c: *mut u32;
    let mut local_58: u32;
    let mut local_54: u32;
    let mut local_50: i32;
    let mut local_4c: i32;
    let mut local_48: u32;
    let mut local_44: i32;
    let mut local_40: i32;
    let mut local_3c: i32;
    let mut local_38: i32;
    let mut local_34: i32;
    let mut local_30: i32;
    let mut local_2c: *mut u8;
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_14: u32;

    local_14 = param_3;
    if (param_3 < 0x401) {
        if (0xff < param_3) {
            if (param_3 < 0x101) {
                return 0x1;
            }
            if (param_3 == 0x113) {
                local_24 = param_1;
                if (((param_1 + 0x1c) == '\0') || ((*(param_1 + 0x2e) & 0x4) != 0x0)) {
                    local_1c = 0x0;
                }
                else {
                    local_1c = 0x1;
                }
                local_20 = local_1c;
                if (local_1c == 0x0) {
                    return 0x1;
                }
                local_2c = param_2;
                iVar1 = *(param_2 + 0x4);
                local_28 = iVar1;
                iVar2 = FUN_0049ab40();
                if (iVar1 == iVar2) {
                    if ((*(param_1 + 0x2d) & 0x40) == 0x0) {
                        FUN_00496263((&DAT_004bf7e0 + DAT_005b8848 * 0xc),0xa,0x4);
                        DAT_005b8848 = DAT_005b8848 + 0x1;
                        if (0x8 < DAT_005b8848) {
                            DAT_005b8848 = 0x0;
                        }
                        FUN_004722d3(param_1);
                    }
                    else {
                        DAT_005b8808._0_1_ = (byte)DAT_005b8808 ^ 0x1;
                        local_38 = DAT_005b7068;
                        local_30 = *(&DAT_005b7078 + DAT_005b7068 * 0x4e) >> 0x10;
                        local_34 = DAT_005b7068;
                        FUN_0047e980(param_1,*(&DAT_005b7076 + DAT_005b7068 * 0x4e) >> 0x10,local_30,0x0);
                    }
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
                local_44 = param_1;
                if (((param_1 + 0x1c) == '\0') || ((*(param_1 + 0x2e) & 0x4) != 0x0)) {
                    local_3c = 0x0;
                }
                else {
                    local_3c = 0x1;
                }
                local_40 = local_3c;
                if (local_3c != 0x0) {
                    local_5c = &DAT_005967b8;
                    local_54 = (DAT_005967bc == 0x0);
                    local_58 = local_54;
                    if (local_54 == 0x0) {
                        local_50 = DAT_005967bc + 0x20;
                        local_48 = *(DAT_005967bc + 0x3a) & 0x1;
                        local_4c = local_50;
                        if ((local_48 == 0x0) && ((DAT_005967bc + 0x41) != -0x1)) {
                            local_78 = DAT_005967bc + 0x20;
                            local_68 = local_78 & 0xffff0000 | (DAT_005967bc + 0x22);
                            local_84 = (DAT_005967bc + 0x22);
                            local_70 = DAT_005967bc + 0x20;
                            local_64 = local_70 & 0xffff0000 | (DAT_005967bc + 0x24);
                            local_80 = (DAT_005967bc + 0x24);
                            if ((*(DAT_005967bc + 0x3a) & 0x80) == 0x0) {
                                local_60 = (DAT_005967bc + 0x26);
                            }
                            else {
                                local_60 = (DAT_005967bc + 0x35);
                            }
                            local_74 = local_78;
                            local_6c = local_70;
                            FUN_004729ba(local_60,&local_84,&local_80,0x1);
                            local_7c = FUN_004710a6(&DAT_005967b8,local_84,local_80,*(DAT_005967bc + 0x41),
                                                    *(DAT_005967bc + 0x42));
                            if (local_7c != 0x0) {
                                FUN_0047eda9(param_1,local_7c,(DAT_005967bc + 0x22),(DAT_005967bc + 0x24),
                                             *(DAT_005967bc + 0x41),*(DAT_005967bc + 0x42));
                            }
                        }
                    }
                    FUN_00471bcd(param_1);
                    return 0x1;
                }
                return 0x1;
            }
        }
    }
    uVar3 = FUN_0049a270(param_1,param_2,param_3,param_4,param_5);
    return uVar3;
}



fn FUN_00471b89()

{
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < DAT_005b706c; local_14 = local_14 + 0x1) {
        FUN_00471bcd(*(&DAT_005b7040 + local_14 * 0x4));
    }
    return;
}



fn FUN_00471bcd(param_1: i32)

{
    let local_40: *mut i32; [0x6];
    let mut local_28: u32;
    let local_24: *mut u32;
    let mut local_20: u32;
    let mut local_1c: i32;
    let local_18: *mut u32;

    local_20 = *(param_1 + 0x1d);
    local_28 = *(param_1 + 0x21);
    local_24 = FUN_0049c2c9(*(param_1 + 0x25) * *(param_1 + 0x29));
    *(param_1 + 0x1d) = 0x0;
    *(param_1 + 0x21) = 0x0;
    FUN_004953d7();
    local_18 = FUN_00498ba4(local_40,local_24,*(param_1 + 0x29),*(param_1 + 0x25));
    if (*(param_1 + 0x81) < 0x9) {
        FUN_004968e7(*(param_1 + 0x1d),*(param_1 + 0x21),*(param_1 + 0x25),*(param_1 + 0x29),0xe
        );
    }
    else {
        FUN_004a08c5(s_pcx_starfld2_pcx_004c23c3,*(param_1 + 0x1d),*(param_1 + 0x21),*(param_1 + 0x25),
                     *(param_1 + 0x29),0x0,0x0,0x1,0x0);
    }
    for (local_1c = 0x0; (local_1c < 0x28 && (*(&DAT_005b70c2 + local_1c * 0x4e) != 0x0));
        local_1c = local_1c + 0x1) {
        if (0x6 < *(param_1 + 0x81)) {
            FUN_0047d8c7(param_1,*(&DAT_005b7076 + local_1c * 0x4e) >> 0x10,
                         *(&DAT_005b7078 + local_1c * 0x4e) >> 0x10);
        }
        FUN_0047e980(param_1,*(&DAT_005b7076 + local_1c * 0x4e) >> 0x10,
                     *(&DAT_005b7078 + local_1c * 0x4e) >> 0x10,*(&DAT_005b707a + local_1c * 0x4e) >> 0x10);
        FUN_004a0430(&DAT_005b707e + local_1c * 0x4e,0x0,0x20);
    }
    for (local_1c = 0x0; local_1c < 0x78; local_1c = local_1c + 0x1) {
        if (((&DAT_005b7cb8)[local_1c * 0x14] & 0x1) != 0x0) {
            FUN_0047dc83(param_1,*(&DAT_005b7ca8 + local_1c * 0x14),
                         *(&DAT_005b7cac + local_1c * 0x14),*(&DAT_005b7cb0 + local_1c * 0x14),
                         *(&DAT_005b7cb4 + local_1c * 0x14),*(&DAT_005b7cb8 + local_1c * 0x14));
        }
    }
    FUN_00471e90(param_1);
    if ((*(param_1 + 0x81) == 0x2) && ((*(param_1 + 0x2d) & 0x2) != 0x0)) {
        FUN_0047d83f(param_1,*(param_1 + 0x59),*(param_1 + 0x5d));
        FUN_0049e5a0(*(param_1 + 0x85),*(param_1 + 0x89),*(param_1 + 0x51) * *(param_1 + 0x81),
                     *(param_1 + 0x55) * *(param_1 + 0x81),0x27);
    }
    *(param_1 + 0x1d) = local_20;
    *(param_1 + 0x21) = local_28;
    FUN_00498cf4(local_40);
    FUN_00496ac0(local_24,*(param_1 + 0x1d),*(param_1 + 0x21),*(param_1 + 0x25),
                 *(param_1 + 0x29));
    FUN_0049536f();
    FUN_0049af50(local_24);
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00471e90(param_1: i32)

{
    let mut iVar1: i32;
    let puVar2: *mut u32;
    let puVar3: *mut u32;
    let mut in_stack_ffffff28: u32;
    let mut in_stack_ffffff2c: u32;
    let mut in_stack_ffffff30: u32;
    let mut in_stack_ffffff34: u32;
    let mut in_stack_ffffff38: u32;
    let mut in_stack_ffffff3c: u32;
    ulonglong in_stack_ffffff40;
    let cStack144: u8;
    let mut local_1c: i32;
    let local_18: *mut u32;
    let local_14: *mut u32;

    local_1c = 0x0;
    if (((((*(param_1 + 0x2d) & 0x8) != 0x0) && (DAT_005967bc != 0x0)) &&
        ((*(DAT_005967bc + 0x3a) & 0x1) == 0x0)) && ((DAT_005967b8 & 0x1) == 0x0)) {
        if ((*(DAT_005967bc + 0x3a) & 0x80) == 0x0) {
            *
                (&DAT_005b707e +
                    (DAT_005967bc + 0x20) * 0x4e +
                    *(&DAT_004be878 + (*(DAT_005967bc + 0x23) >> 0x18) * 0x4) * 0x4) = 0xff;
        }
        else {
            *
                (&DAT_005b707e +
                    (DAT_005967bc + 0x20) * 0x4e +
                    *(&DAT_004be878 + (*(DAT_005967bc + 0x32) >> 0x18) * 0x4) * 0x4) = 0xff;
        }
    }
    FUN_004953d7();
    for (local_18 = *DAT_005967b0; local_18 != 0x0; local_18 = *local_18) {
        if (((((*(local_18 + 0x3a) & 0x80000000) == 0x0) && ((*(local_18 + 0x3a) & 0x1) == 0x0))
            && (((*(&DAT_004be9b0 + DAT_004c9758 * 0x4) & *(local_18 + 0x3a)) != 0x0 ||
            (_DAT_004c975c != 0x0)))) && ((*(local_18 + 0x3a) & 0x8) == 0x0)) {
            if ((*(local_18 + 0x3a) & 0x80) == 0x0) {
                cStack144 = (local_18 + 0x26);
            }
            else {
                cStack144 = (local_18 + 0x35);
            }
            iVar1 = *(local_18 + 0x23) >> 0x18;
            if ((*(&DAT_004be840 + iVar1 * 0x4) &
                *(&DAT_005b707e + *(&DAT_004be878 + cStack144 * 0x4) * 0x4 + (local_18 + 0x8) * 0x4e)
            ) == 0x0) {
                local_14 = FUN_004499d9(local_18,&local_1c,iVar1,0x0);
                puVar2 = local_14 + 0x8;
                puVar3 = &stack0xffffff28;
                for (iVar1 = 0xb; iVar1 != 0x0; iVar1 = iVar1 + -0x1) {
                    *puVar3 = *puVar2;
                    puVar2 = puVar2 + 0x1;
                    puVar3 = puVar3 + 0x1;
                }
                puVar3 = puVar2;
                FUN_0047e2a6(param_1,in_stack_ffffff28,in_stack_ffffff2c,in_stack_ffffff30,in_stack_ffffff34,in_stack_ffffff38,
                             in_stack_ffffff3c,in_stack_ffffff40);
            }
        }
    }
    FUN_0049536f();
    return;
}



fn FUN_00472186()

{
    let ppcVar1: *mut *mut char;
    let mut pcVar2: String;
    let mut iVar3: i32;
    let mut local_18: i32;

    ppcVar1 = FUN_0049c4bd(s_bin_efsplan_bin_004c23d7,&DAT_004c23d4);
    if (ppcVar1 == 0x0) {
        pcVar2 = FUN_00499050(DAT_0059679c,0x7394);
        pop_err_msg_box_and_exit_004a02f5(pcVar2);
    }
    else {
        for (local_18 = 0x0; local_18 < 0x80; local_18 = local_18 + 0x1) {
            if (*(&DAT_005b8608 + local_18 * 0x4) == 0x0) {
                iVar3 = FUN_0049c2c9(0x400);
                *(&DAT_005b8608 + local_18 * 0x4) = iVar3;
            }
            FUN_004a7970((&DAT_005b8608 + local_18 * 0x4),0x400,0x1,ppcVar1);
        }
    }
    FUN_0049ca40(ppcVar1);
    return;
}



fn FUN_00472275()

{
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0x28; local_14 = local_14 + 0x1) {
        *(&DAT_005b70c2 + local_14 * 0x4e) = 0x0;
    }
    for (local_14 = 0x0; local_14 < 0x78; local_14 = local_14 + 0x1) {
        *(&DAT_005b7cb8 + local_14 * 0x14) = 0x0;
    }
    return;
}



fn FUN_004722d3(param_1: i32)

{
    let mut local_14: i32;

    FUN_00498999(*(param_1 + 0x1d),*(param_1 + 0x21),*(param_1 + 0x25),*(param_1 + 0x29));
    for (local_14 = 0x0; (local_14 < 0x28 && (*(&DAT_005b70c2 + local_14 * 0x4e) != 0x0));
        local_14 = local_14 + 0x1) {
        (&DAT_005b707c + local_14 * 0x4e) = (&DAT_005b707c + local_14 * 0x4e) + 0x1;
        if (0xf < *(&DAT_005b707a + local_14 * 0x4e) >> 0x10) {
            *(&DAT_005b707c + local_14 * 0x4e) = 0x0;
        }
        FUN_0047d83f(param_1,*(&DAT_005b7076 + local_14 * 0x4e) >> 0x10,
                     *(&DAT_005b7078 + local_14 * 0x4e) >> 0x10);
        FUN_00495520(*(param_1 + 0x85),*(param_1 + 0x89),*(param_1 + 0x65) + 0x2,
                     *(param_1 + 0x65) + 0x2,0x1);
        FUN_0047e980(param_1,*(&DAT_005b7076 + local_14 * 0x4e) >> 0x10,
                     *(&DAT_005b7078 + local_14 * 0x4e) >> 0x10,*(&DAT_005b707a + local_14 * 0x4e) >> 0x10);
        if (DAT_005b7068 == local_14) {
            FUN_0047ef5e(*(param_1 + 0x85),*(param_1 + 0x89),*(param_1 + 0x81));
        }
        FUN_00495607(0x0);
    }
    FUN_00498ae4();
    return;
}



fn FUN_00472441(param_1: u32,param_2: i32,param_3: i32) -> i32

{
let mut local_14: i32;

local_14 = 0x0;
while( true ) {
if ((0x27 < local_14) || (*(&DAT_005b70c2 + local_14 * 0x4e) == 0x0)) {
return -0x1;
}
if ((*(&DAT_005b7076 + local_14 * 0x4e) >> 0x10 == param_2) &&
(*(&DAT_005b7078 + local_14 * 0x4e) >> 0x10 == param_3)) break;
local_14 = local_14 + 0x1;
}
return local_14;
}



fn FUN_004724b5(param_1: i32,param_2: i32,param_3: i32)

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut iVar3: i32;

    FUN_0047d7d7(param_1,param_2,param_3);
    iVar1 = *(param_1 + 0x8d);
    iVar2 = *(param_1 + 0x91);
    iVar3 = FUN_00485d4e(DAT_005b7068,iVar1,iVar2);
    if (iVar3 != -0x1) {
        FUN_004853b9(DAT_005b7068,iVar1,iVar2);
    }
    return;
}



fn FUN_0047254a(param_1: u32,param_2: u32) -> u32

{
    DAT_005b7068 = param_2;
    DAT_005b8808 = 0x0;
    return 0x1;
}



fn FUN_00472580(param_1: u32,param_2: i32,param_3: i32) -> u32

{
    let mut local_14: i32;

    local_14 = 0x0;
    while( true ) {
        if (0x77 < local_14) {
            return 0x0;
        }
        if ((*(&DAT_005b70c2 + local_14 * 0x4e) != 0x0) &&
            (((((*(&DAT_005b7076 + param_2 * 0x4e) >> 0x10 == *(&DAT_005b7ca8 + local_14 * 0x14) &&
                (*(&DAT_005b7078 + param_2 * 0x4e) >> 0x10 == *(&DAT_005b7cac + local_14 * 0x14))) &&
                (*(&DAT_005b7076 + param_3 * 0x4e) >> 0x10 == *(&DAT_005b7cb0 + local_14 * 0x14))) &&
                (*(&DAT_005b7078 + param_3 * 0x4e) >> 0x10 == *(&DAT_005b7cb4 + local_14 * 0x14))) ||
                (((*(&DAT_005b7076 + param_3 * 0x4e) >> 0x10 == *(&DAT_005b7ca8 + local_14 * 0x14) &&
                    (*(&DAT_005b7078 + param_3 * 0x4e) >> 0x10 == *(&DAT_005b7cac + local_14 * 0x14))) &&
                    ((*(&DAT_005b7076 + param_2 * 0x4e) >> 0x10 == *(&DAT_005b7cb0 + local_14 * 0x14) &&
                        (*(&DAT_005b7078 + param_2 * 0x4e) >> 0x10 == *(&DAT_005b7cb4 + local_14 * 0x14))))))))) break;
        local_14 = local_14 + 0x1;
    }
    return 0x1;
}



fn FUN_004726ac(param_1: u32,param_2: i32,param_3: i32,param_4: i32) -> u32

{
    let mut pcVar1: String;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut puVar4: *mut u8;
    let cStack72: u8;
    i32 **local_34;

    if (DAT_005967bc == 0x0) {
    return 0x0;
}
    for (local_34 = DAT_005967bc; local_34 != 0x0; local_34 = local_34[0x2]) {
        if ((((local_34 + 0x2a) != 0x7) && ((local_34 + 0x2a) != 0x9)) &&
            ((*(local_34 + 0x3a) & 0x40) == 0x0)) {
            if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
                pcVar1 = FUN_00499050(DAT_0059679c,0x7155);
                FUN_0049d2e0(0x0,0x1,pcVar1);
            }
            return 0x0;
        }
    }
    if ((*(DAT_005967bc + 0x3a) & 0x80) == 0x0) {
        cStack72 = (DAT_005967bc + 0x26);
    }
    else {
        cStack72 = (DAT_005967bc + 0x35);
    }
    FUN_004729ba(cStack72,&param_2,&param_3,0x0);
    iVar2 = FUN_00432c94(&DAT_005967b8);
    iVar3 = FUN_00485ea2(param_4,param_2,param_3,0x0);
    if (0x14 < iVar2 + iVar3) {
        if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
            pcVar1 = FUN_00499050(DAT_0059679c,0x714d);
            FUN_0049d2e0(0x0,0x1,pcVar1);
            return 0x0;
        }
        FUN_00418d7a(param_4);
        iVar3 = FUN_00485ea2(param_4,param_2,param_3,0x0);
        if (0x14 < iVar2 + iVar3) {
            return 0x0;
        }
    }
    puVar4 = &DAT_00568210 + param_4 * 0x9d + DAT_004c9754 * 0x1e22;
    puVar4[0x9c] = puVar4[0x9c] & 0xfe;
    puVar4[0x9c] = puVar4[0x9c] | 0x1;
    for (local_34 = DAT_005967bc; local_34 != 0x0; local_34 = local_34[0x2]) {
        FUN_004841ea(local_34,param_4,param_2,param_3);
        if ((*(local_34 + 0x3a) & 0x40) == 0x0) {
            (local_34 + 0x2f) = 0x0;
            *(local_34 + 0xf) = *(local_34 + 0xf) | 0x1;
        }
        FUN_00459e8f(local_34);
    }
        *(&DAT_005b4ac0 + param_3 * 0x4 + param_2 * 0xc0) =
        *(&DAT_005b4ac0 + param_3 * 0x4 + param_2 * 0xc0) | 0x1;
    return 0x1;
}



fn FUN_004729ba(param_1: i32,param_2: *mut i32,param_3: *mut i32,param_4: i32) -> i32

{
let mut iVar1: i32;

iVar1 = *(&DAT_004be878 + param_1 * 0x4);
if (param_4 == 0x0) {
*param_2 = *param_2 + *(&DAT_004bea20 + iVar1 * 0x4);
*param_3 = *param_3 + *(&DAT_004bea40 + iVar1 * 0x4);
}
else {
*param_2 = *param_2 - *(&DAT_004bea20 + iVar1 * 0x4);
*param_3 = *param_3 - *(&DAT_004bea40 + iVar1 * 0x4);
}
return iVar1;
}



fn FUN_00472a32(param_1: i32)

{
    FUN_00481152(param_1);
    FUN_00484ae4(param_1);
    *(&DAT_004daab0 + param_1 * 0x3890) = 0x0;
    FUN_004a0430(param_1 * 0x3890 + 0x4d7e00,0x0,0x2cb0);
    return;
}



fn FUN_00472a92(param_1: u32,param_2: i32,param_3: i32,param_4: i32) -> i32

{
let local_14: *mut u32;

local_14 = *DAT_005967b0;
while( true ) {
if (local_14 == 0x0) {
return -0x1;
}
if (((((*(local_14 + 0x3a) & 0x1) == 0x0) && ((local_14 + 0x22) == param_3)) &&
((local_14 + 0x9) == param_4)) && ((param_2 == -0x1 || ((local_14 + 0x8) == param_2))))
break;
local_14 = *local_14;
}
DAT_005b7068 = (local_14 + 0x8);
DAT_005b8808 = 0x0;
return *(local_14 + 0x23) >> 0x18;
}



fn FUN_00472bb0(param_1: i32) -> u32

{
    let mut iVar1: i32;
    let mut pcVar2: String;
    let local_274: *mut u32;
    let local_224: *mut u32;
    let local_1d8: *mut u32;
    let local_190: *mut u32;
    let mut local_144: *mut u32 [0x11];
    let ppuStack255: *mut *mut u8;;
    let mut local_53: String;;
    i32 aiStack76 [0x4];
    i32 aiStack60 [0x4];
    let mut local_2c: i32;
    let mut local_28: i32;
    let local_24: *mut u32;
    let mut local_20: i32;
    let mut local_1c: u32;
    let mut local_18: u32;

    local_28 = -0x1;
    DAT_005b880c = 0x0;
    for (local_20 = 0x0; local_20 < 0xe; local_20 = local_20 + 0x1) {
        if (*(&DAT_005b8810 + local_20 * 0x4) != -0x1) {
            local_28 = local_20;
            DAT_005b880c = DAT_005b880c + 0x1;
        }
    }
    if (DAT_005b880c == 0x0) {
        local_1c = 0xffffffff;
    }
    else {
        if (DAT_005b880c == 0x1) {
            local_1c = *(&DAT_005b8810 + local_28 * 0x4);
        }
        else {
            for (local_20 = 0x0; local_20 < 0x4; local_20 = local_20 + 0x1) {
                aiStack76[local_20] = 0x0;
                aiStack60[local_20] = 0x0;
            }
            FUN_004990e0(local_144,0x0,s_efs_res_004c23fb,s_PickDlg_004c23f3);
            if (param_1 == 0x0) {
                local_1d8 = FUN_004a2831(0x51);
                if (local_1d8 != 0x0) {
                    pcVar2 = FUN_00499050(DAT_0059679c,0x734e);
                    local_1d8 = FUN_0049a030(local_1d8,local_144,0x3e8,0x0,0xf,0x280,0x12,0x0,0xcaccce,0xffffffff);
                    $1: &mut String(local_1d8 + 0x45) = &PTR_FUN_004c3f14;
                    *(local_1d8 + 0x49) = pcVar2;
                    *(local_1d8 + 0x4d) = DAT_004bf958;
                    *(local_1d8 + 0x2e) = *(local_1d8 + 0x2e) | 0x78;
                    if ((*(local_1d8 + 0x2d) & 0xc) != 0x0) {
                        *(local_1d8 + 0x2e) = *(local_1d8 + 0x2e) & 0xbf;
                    }
                }
                local_24 = local_1d8;
            }
            else {
                local_190 = FUN_004a2831(0x51);
                if (local_190 != 0x0) {
                    pcVar2 = FUN_00499050(DAT_0059679c,0x734d);
                    local_190 = FUN_0049a030(local_190,local_144,0x3e8,0x0,0xf,0x280,0x12,0x0,0xcaccce,0xffffffff);
                    $1: &mut String(local_190 + 0x45) = &PTR_FUN_004c3f14;
                    *(local_190 + 0x49) = pcVar2;
                    *(local_190 + 0x4d) = DAT_004bf958;
                    *(local_190 + 0x2e) = *(local_190 + 0x2e) | 0x78;
                    if ((*(local_190 + 0x2d) & 0xc) != 0x0) {
                        *(local_190 + 0x2e) = *(local_190 + 0x2e) & 0xbf;
                    }
                }
                local_24 = local_190;
            }
            FUN_0049bf40(local_144,local_24);
            local_2c = 0x0;
            for (local_20 = 0x0; local_20 < 0xe; local_20 = local_20 + 0x1) {
                if (*(&DAT_005b8810 + local_20 * 0x4) != -0x1) {
                    local_224 = FUN_004a2831(0x5d);
                    if (local_224 != 0x0) {
                        local_224 = FUN_0049a030(local_224,local_144,local_20 + 0x414,local_2c * 0x94 + 0x30,0x37,0x64,0x64,0x2
                                                 ,0xcaccce,0xe0e0e);
                        $1: &mut String(local_224 + 0x45) = &PTR_FUN_004c3d94;
                        *(local_224 + 0x51) = 0x0;
                        *(local_224 + 0x55) = 0x0;
                        *(local_224 + 0x4d) = 0x0;
                        *(local_224 + 0x49) = 0x2;
                    }
                    aiStack76[local_2c] = local_224;
                    local_274 = FUN_004a2831(0x51);
                    if (local_274 != 0x0) {
                        pcVar2 = FUN_00499050(DAT_0059679c,local_20 + 0x414);
                        local_274 = FUN_0049a030(local_274,local_144,local_20 + 0x898,local_2c * 0x94 + 0x26,0xa0,0x78,0xc,0x0,
                                                 0xcaccce,0xffffffff);
                        $1: &mut String(local_274 + 0x45) = &PTR_FUN_004c3f14;
                        *(local_274 + 0x49) = pcVar2;
                        *(local_274 + 0x4d) = DAT_004bf958;
                        *(local_274 + 0x2e) = *(local_274 + 0x2e) | 0x78;
                        if ((*(local_274 + 0x2d) & 0xc) != 0x0) {
                            *(local_274 + 0x2e) = *(local_274 + 0x2e) & 0xbf;
                        }
                    }
                    aiStack60[local_2c] = local_274;
                    FUN_0049bf40(local_144,aiStack76[local_2c]);
                    FUN_0049bf40(local_144,aiStack60[local_2c]);
                    local_2c = local_2c + 0x1;
                }
            }
            local_18 = FUN_0049bb50(local_144,FUN_00473573);
            if ((local_24 != 0x0) && (local_24 != 0x0)) {
                ((*(local_24 + 0x45) + 0x8))(local_24,0x2);
            }
            for (local_20 = 0x0; local_20 < 0x4; local_20 = local_20 + 0x1) {
                if ((aiStack76[local_20] != 0x0) && (iVar1 = aiStack76[local_20], iVar1 != 0x0)) {
                    ((*(iVar1 + 0x45) + 0x8))(iVar1,0x2);
                }
                if ((aiStack60[local_20] != 0x0) && (iVar1 = aiStack60[local_20], iVar1 != 0x0)) {
                    ((*(iVar1 + 0x45) + 0x8))(iVar1,0x2);
                }
            }
            local_1c = local_18;
            ppuStack255 = &PTR_FUN_004c3d34;
            if (local_53 != 0x0) {
                FUN_00499b30(local_144,local_53);
            }
            FUN_0049a1c0(local_144,0x1);
        }
    }
    return local_1c;
}



fn FUN_00473573(param_1: i32,param_2: u32,param_3: i32) -> u32

{
    let mut local_20: i32;
    let mut local_1c: i32;

    if (0x404 < param_2) {
        if (param_2 < 0x406) {
            local_20 = 0x0;
            for (local_1c = 0x0; local_1c < 0xe; local_1c = local_1c + 0x1) {
                if (*(&DAT_005b8810 + local_1c * 0x4) != -0x1) {
                    FUN_00496ac0((&DAT_00595700 + local_1c * 0x4),local_20 * 0x94 + 0x30,0x37,0x64,0x64);
                    local_20 = local_20 + 0x1;
                }
            }
            return 0x1;
        }
        if (param_2 == 0x407) {
            if ((0x413 < param_3) && (param_3 < 0x422)) {
                FUN_0049c140(param_1,*(&DAT_005b77c0 + param_3 * 0x4));
            }
            return 0x0;
        }
    }
    return 0x0;
}



fn FUN_00473663(param_1: i32,param_2: i32,param_3: i32)

{
    let local_14: *mut u32;

    FUN_004a0430(&DAT_005b8810,0xff,0x38);
    for (local_14 = (&DAT_005b8b44 + param_1 * 0x4);
        (local_14 != 0x0 && ((local_14 + 0x8) == param_1)); local_14 = *local_14) {
        if (((*(local_14 + 0x3a) & 0x1) == 0x0) &&
            ((((local_14 + 0x22) == param_2 && ((local_14 + 0x9) == param_3)) &&
                ((*(&DAT_004be9b0 + DAT_004c9754 * 0x4) & *(local_14 + 0x3a)) != 0x0)))) {
            if ((*(local_14 + 0x3a) & 0x80) == 0x0) {
                *(&DAT_005b8810 + (*(local_14 + 0x23) >> 0x18) * 0x4) =
                    *(local_14 + 0x23) >> 0x18;
            }
            else {
                *(&DAT_005b8810 + (*(local_14 + 0x32) >> 0x18) * 0x4) =
                    *(local_14 + 0x23) >> 0x18;
            }
        }
    }
    return;
}



fn FUN_00473798(param_1: u32,param_2: i32,param_3: i32,param_4: i32,param_5: i32)

{
    let mut local_14: i32;

    if (DAT_005b7070 >> 0x10 < 0x78) {
        local_14 = 0x0;
        while ((local_14 < 0x78 && (((&DAT_005b7cb8)[local_14 * 0x14] & 0x1) != 0x0))) {
            if ((*(&DAT_005b7ca8 + local_14 * 0x14) == param_2) &&
                (((*(&DAT_005b7cac + local_14 * 0x14) == param_3 &&
                    (*(&DAT_005b7cb0 + local_14 * 0x14) == param_4)) &&
                    (*(&DAT_005b7cb4 + local_14 * 0x14) == param_5)))) {
                return;
            }
            if (((*(&DAT_005b7ca8 + local_14 * 0x14) == param_4) &&
                (*(&DAT_005b7cac + local_14 * 0x14) == param_5)) &&
                ((*(&DAT_005b7cb0 + local_14 * 0x14) == param_2 &&
                    (*(&DAT_005b7cb4 + local_14 * 0x14) == param_3)))) {
                return;
            }
            local_14 = local_14 + 0x1;
        }
        DAT_005b7070._2_2_ = DAT_005b7070._2_2_ + 0x1;
        DAT_005b7070 = DAT_005b7070 & 0xffff | DAT_005b7070._2_2_ << 0x10;
        *(&DAT_005b7ca8 + local_14 * 0x14) = param_2;
        *(&DAT_005b7cac + local_14 * 0x14) = param_3;
        *(&DAT_005b7cb0 + local_14 * 0x14) = param_4;
        *(&DAT_005b7cb4 + local_14 * 0x14) = param_5;
        *(&DAT_005b7cb8 + local_14 * 0x14) = 0x1;
    }
    return;
}



fn FUN_004738c9(param_1: i32,param_2: i32) -> *mut u32

{
    let local_14: *mut u32;

    local_14 = *DAT_005967b0;
    while( true ) {
        if (local_14 == 0x0) {
            return 0x0;
        }
        if ((((local_14 + 0x22) == param_1) && ((local_14 + 0x9) == param_2)) &&
            ((*(local_14 + 0x3a) & 0x1) == 0x0)) break;
        local_14 = *local_14;
    }
    DAT_005b7068 = (local_14 + 0x8);
    DAT_005b8808 = 0x0;
    return local_14;
}



fn FUN_004739b3() -> String

{
    return PTR_s_STATIC_004bfad0;
}



fn FUN_004739d2(param_1: *mut u32) -> *mut u32

{
    *param_1 = 0x0;
    return param_1;
}



fn FUN_004739f8(param_1: &mut String) -> String

{
    if (*param_1 != 0x0) {
        FUN_0049af50(*param_1);
        *param_1 = 0x0;
    }
    return param_1;
}



fn FUN_00473a39(param_1: &mut String,byte param_2) -> String

{
    let piVar1: *mut i32;;

    if ((param_2 & 0x4) == 0x0) {
        param_1 = FUN_0049a1c0(param_1,0x1);
        if ((param_2 & 0x2) != 0x0) {
            FUN_0049af50(param_1);
        }
    }
    else {
        piVar1 = FUN_00498dce(param_1,&DAT_004c3f90);
        FUN_00498df5(piVar1);
    }
    return param_1;
}



fn FUN_00473a9c(param_1: u32) -> u32

{
    let mut uVar1: u32;

    uVar1 = FUN_004a7132(param_1,0x80,&DAT_004c3f50);
    return uVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00473acd() -> u32

{
    let mut iVar1: i32;
    let local_1e4: *mut u32;
    let mut local_194: *mut u32 [0x11];
    let ppuStack335: *mut *mut u8;;
    let mut local_a3: String;;
    i32 aiStack156 [0x14];
    let mut local_4c: u32;
    let mut local_48: u32;
    let local_44: *mut i32;;
    let local_40: *mut i32;;
    let local_3c: *mut i32;;
    let local_38: *mut i32;;
    let mut local_34: i32;
    i32 **local_30;
    i32 **local_2c;
    i32 **local_28;
    i32 **local_24;
    let local_20: *mut i32;;
    let local_1c: *mut i32;;
    let local_18: *mut i32;;

    local_20 = FUN_004990e0(local_194,0x0,s_efs_res_004c240b,s_StarDlg_004c2403);
    local_44 = FUN_004a2831(0x95);
    local_40 = local_44;
    local_1c = local_44;
    if (local_44 != 0x0) {
        local_1c = FUN_0047157e(local_44,local_194,0x4b0,0x7c,0x1d,0x1f8,0x1b3,0x4009,0x22);
    }
    DAT_005b8850 = local_1c;
    local_3c = FUN_004a2831(0x95);
    local_38 = local_3c;
    local_18 = local_3c;
    if (local_3c != 0x0) {
        local_18 = FUN_0047157e(local_3c,local_194,0x4b1,0xa,0x17,0x64,0x60,0x4042,0x2);
    }
    DAT_005b8854 = local_18;
    for (local_34 = 0x0; local_34 < 0x14; local_34 = local_34 + 0x1) {
        local_1e4 = FUN_004a2831(0x5d);
        if (local_1e4 != 0x0) {
            local_1e4 = FUN_0049a030(local_1e4,local_194,local_34 + 0x2328,(local_34 % 0x3) * 0x22 + 0x9,
                                     (local_34 / 0x3) * 0x22 + 0xb5,0x22,0x22,0x4002,0xcaccce,0xe0e0e);
            $1: &mut String(local_1e4 + 0x45) = &PTR_FUN_004c3d94;
            *(local_1e4 + 0x51) = 0x0;
            *(local_1e4 + 0x55) = 0x0;
            *(local_1e4 + 0x4d) = 0x0;
            *(local_1e4 + 0x49) = 0x2;
        }
        aiStack156[local_34] = local_1e4;
        FUN_0049bf40(local_194,aiStack156[local_34]);
    }
    FUN_0049bf40(local_194,DAT_005b8850);
    FUN_0049bf40(local_194,DAT_005b8854);
    local_30 = FUN_004a2831(0x10);
    local_2c = local_30;
    if (local_30 != 0x0) {
    FUN_004a2874(local_30,local_194,0x3e8);
}
    local_28 = FUN_004a2831(0x10);
    local_24 = local_28;
    if (local_28 != 0x0) {
    FUN_004a2874(local_28,local_194,0xfa);
}
    FUN_00431d0a(&DAT_005967b8);
    _DAT_004c9750 = 0x0;
    _DAT_00596a4c = 0x0;
    local_48 = FUN_0049bb50(local_194,&DAT_0047404b);
    FUN_004a2965(local_194);
    for (local_34 = 0x0; local_34 < 0x14; local_34 = local_34 + 0x1) {
        if ((aiStack156[local_34] != 0x0) && (iVar1 = aiStack156[local_34], iVar1 != 0x0)) {
            ((*(iVar1 + 0x45) + 0x8))(iVar1,0x2);
        }
    }
    if (DAT_005b8850 != 0x0) {
        ((*(DAT_005b8850 + 0x45) + 0x8))(DAT_005b8850,0x2);
    }
    if (DAT_005b8854 != 0x0) {
        ((*(DAT_005b8854 + 0x45) + 0x8))(DAT_005b8854,0x2);
    }
    local_4c = local_48;
    ppuStack335 = &PTR_FUN_004c3d34;
    if (local_a3 != 0x0) {
        FUN_00499b30(local_194,local_a3);
    }
    FUN_0049a1c0(local_194,0x1);
    return local_4c;
}



fn FUN_00476d51()

{
    let mut local_12c: *mut u32 [0x11];
    let ppuStack231: *mut *mut u8;;
    let mut local_3b: String;;
    let local_34: *mut i32;;
    let local_30: *mut i32;;
    let local_2c: *mut i32;;
    let local_28: *mut i32;;
    let mut local_24: u32;
    u32 **local_20;
    let mut local_1c: u32;

    local_2c = FUN_004990e0(local_12c,0x0,s_efs_res_004c242e,s_GalaxyDlg_004c2424);
    local_34 = FUN_004a2831(0x95);
    local_30 = local_34;
    local_28 = local_34;
    if (local_34 != 0x0) {
        local_28 = FUN_0047157e(local_34,local_12c,0x63,0x5f,0xa,0x1c2,0x1b0,0x1,0x9);
    }
    DAT_005b8858 = local_28;
    FUN_0049bf40(local_12c,local_28);
    FUN_0049bb50(local_12c,FUN_00476e7c);
    if (DAT_005b8858 == 0x0) {
        local_24 = 0x0;
    }
    else {
        local_24 = ((*(DAT_005b8858 + 0x45) + 0x8))(DAT_005b8858,0x2);
    }
    local_20 = local_12c;
    local_1c = 0x0;
    ppuStack231 = &PTR_FUN_004c3d34;
    if (local_3b != 0x0) {
        FUN_00499b30(local_12c,local_3b);
    }
    FUN_0049a1c0(local_12c,0x1);
    return;
}



fn FUN_00476e7c(param_1: i32,param_2: u32,param_3: i32,param_4: i32) -> u32

{
    let mut uVar1: u32;
    let mut iVar2: i32;

    if (param_2 < 0x405) {
        if ((((param_2 == 0x201) && (*(DAT_005b8858 + 0x1d) <= param_3)) &&
            (param_3 < *(DAT_005b8858 + 0x1d) + *(DAT_005b8858 + 0x25))) &&
            ((*(DAT_005b8858 + 0x21) <= param_4 &&
                (param_4 < *(DAT_005b8858 + 0x21) + *(DAT_005b8858 + 0x29))))) {
            FUN_0047d7d7(DAT_005b8858,param_3,param_4);
            FUN_0047d6a0(DAT_005b8850,*(DAT_005b8858 + 0x8d),*(DAT_005b8858 + 0x91),0x0);
            iVar2 = DAT_005b8854;
            uVar1 = *(DAT_005b8850 + 0x4d);
            *(DAT_005b8854 + 0x59) = *(DAT_005b8850 + 0x49);
            *(iVar2 + 0x5d) = uVar1;
            FUN_0049c140(param_1,0x1);
        }
    }
    else {
        if (param_2 < 0x406) {
            FUN_004953d7();
            FUN_0049e640(0x5f,0xa,0x1c2,0x1b0,0xce,0xca,0xcc,0x1);
            FUN_0049e640(0x5f,0xa,0x1c2,0x1b0,0xce,0xca,0xcc,0x2);
            FUN_0049536f();
            return 0x1;
        }
        if ((param_2 == 0x407) && (param_3 == 0x64)) {
            FUN_0049c140(param_1,0x1);
        }
    }
    return 0x0;
}



fn FUN_0047706a() -> u32

{
    let mut local_120: *mut u32 [0x11];
    let ppuStack219: *mut *mut u8;;
    let mut local_2f: String;;
    let mut local_28: u32;
    u32 **local_24;
    let local_20: *mut i32;;
    let mut local_1c: u32;

    local_20 = FUN_004990e0(local_120,0x0,s_efs_res_004c243f,s_SpAttDlg_004c2436);
    local_28 = FUN_0049bb50(local_120,FUN_00477161);
    local_24 = local_120;
    local_1c = 0x0;
    ppuStack219 = &PTR_FUN_004c3d34;
    if (local_2f != 0x0) {
        FUN_00499b30(local_120,local_2f);
    }
    FUN_0049a1c0(local_120,0x1);
    return local_28;
}



fn FUN_00477161(param_1: *mut *mut u32,param_2: u32,param_3: i32) -> u32

{
    let mut iVar1: i32;
    let mut pcVar2: String;

    if (param_2 < 0x407) {
        if ((param_2 == 0x401) && (iVar1 = FUN_00434d1f(&DAT_005967b8), iVar1 == 0x0)) {
            FUN_0049bf80(param_1,0xc8,0x410,0x0,0x0);
        }
    }
    else {
        if (param_2 < 0x408) {
            FUN_0049c140(param_1,param_3);
        }
        else {
            if (param_2 == 0x411) {
                if (param_3 == 0xc8) {
                    pcVar2 = FUN_00499050(DAT_0059679c,0x7387);
                    FUN_0049d2e0(param_1,0x1,pcVar2);
                }
                return 0x1;
            }
        }
    }
    return 0x0;
}



fn FUN_0047722f(param_1: i32,param_2: i32) -> u32

{
    let mut uVar1: u32;
    let mut iVar2: i32;
    let mut local_14c: i32;
    let local_148: *mut u32;
    let cStack240: u8;
    let mut local_ec: i32;
    let mut local_e8: i32;
    let mut local_e4: u32;
    let mut local_e0: i32;
    let mut local_dc: i32;
    let mut local_d8: u32;
    let mut local_d4: u32;
    let mut local_d0: i32;
    let local_cc: *mut u32;
    let mut local_c8: i32;
    let mut local_c4: i32;
    let mut local_c0: i32;
    let mut local_bc: i32;
    let mut local_b8: i32;
    let mut local_b4: i32;
    let mut local_b0: i32;
    let mut local_ac: i32;
    let mut local_a8: i32;
    let mut local_a4: u32;
    let mut local_a0: u32;
    let mut local_9c: u32;
    let mut local_98: i32;
    let local_94: *mut u32;
    let local_90: u8;
    let mut local_8c: u32;
    let mut local_88: u32;
    let mut local_84: u32;
    let mut local_80: u32;
    let mut local_7c: u32;
    let mut local_78: u32;
    let mut local_6c: u32;
    let local_68: *mut u32;
    let mut local_64: u32;
    let mut local_60: u32;
    let mut local_5c: u32;
    let mut local_58: u32;
    let mut local_54: u32;
    let mut local_50: u32;
    let local_4c: u8;
    let mut local_40: u32;
    let mut local_3c: u32;
    let mut local_38: i32;
    let mut local_34: i32;
    let mut local_30: u32;
    let mut local_2c: u32;
    let mut local_28: i32;
    let mut local_24: u32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    local_24 = *(DAT_005967bc + 0x3a) & *(&DAT_004be9b0 + DAT_004c9758 * 0x4);
    if ((((&DAT_00569a98)[(*(DAT_005967bc + 0x23) >> 0x18) * 0x1e22] & 0x2) != 0x0) &&
        (hide_ai_opt_004d55a4 != 0x0)) {
        local_24 = 0x0;
    }
    if ((local_24 != 0x0) && (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) != 0x0)) {
        FUN_00424d33(DAT_005967bc);
    }
    if (((DAT_005967bc + 0x41) == -0x1) || ((DAT_005967bc + 0x42) == -0x1)) {
        DAT_005967b8 = DAT_005967b8 & 0xfffffffe;
        local_40 = 0x0;
    }
    else {
        local_64 = DAT_005967bc + 0x20;
        local_54 = local_64 & 0xffff0000 | (DAT_005967bc + 0x22);
        local_30 = SEXT24((DAT_005967bc + 0x22));
        local_5c = DAT_005967bc + 0x20;
        local_50 = local_5c & 0xffff0000 | (DAT_005967bc + 0x24);
        local_2c = SEXT24((DAT_005967bc + 0x24));
        if ((*(DAT_005967bc + 0x3a) & 0x80) == 0x0) {
            local_4c = (DAT_005967bc + 0x26);
        }
        else {
            local_4c = (DAT_005967bc + 0x35);
        }
        local_20 = local_4c;
        local_60 = local_64;
        local_58 = local_5c;
        FUN_004729ba(local_20,&local_30,&local_2c,0x1);
        if ((*(DAT_005967bc + 0x41) == local_30) && (*(DAT_005967bc + 0x42) == local_2c)) {
            FUN_00432a46(&DAT_005967b8);
            local_40 = 0x0;
        }
        else {
            local_1c = FUN_00432bd3(&DAT_005967b8);
            if (local_1c == 0x0) {
                local_68 = &DAT_005967b8;
                local_6c = 0x1;
                DAT_005967b8 = DAT_005967b8 & 0xfffffffe;
                FUN_00459921(&DAT_005967b8);
                local_40 = 0x1;
            }
            else {
                local_3c = FUN_004710a6(&DAT_005967b8,local_30,local_2c,*(DAT_005967bc + 0x41),
                                        *(DAT_005967bc + 0x42));
                if (local_3c == 0x0) {
                    FUN_00432a46(&DAT_005967b8);
                    DAT_005967b8 = DAT_005967b8 & 0xfffffffe;
                    local_40 = 0x0;
                }
                else {
                    FUN_004713df(&DAT_005967b8,local_3c,local_30,local_2c,&local_38,&local_34);
                    local_7c = DAT_005967bc + 0x20;
                    local_80 = local_7c & 0xffff0000 | (DAT_005967bc + 0x22);
                    local_18 = (DAT_005967bc + 0x22);
                    local_88 = DAT_005967bc + 0x20;
                    local_8c = local_88 & 0xffff0000 | (DAT_005967bc + 0x24);
                    local_14 = (DAT_005967bc + 0x24);
                    local_84 = local_88;
                    local_78 = local_7c;
                    local_28 = FUN_00472441(DAT_005b8850,local_38,local_34);
                    iVar2 = FUN_004726ac(DAT_005b8850,local_38,local_34,local_28);
                    if (iVar2 == 0x0) {
                        FUN_00432a46(&DAT_005967b8);
                        DAT_005967b8 = DAT_005967b8 & 0xfffffffe;
                        local_40 = 0x0;
                    }
                    else {
                        FUN_004953d7();
                        FUN_00459921(&DAT_005967b8);
                        local_94 = FUN_0049c2c9(0x1f9c);
                        FUN_0049c57b(s_bin_jump_bin_004c2447,local_94,0x1f9c);
                        if ((*(DAT_005967bc + 0x3a) & 0x80) == 0x0) {
                            local_90 = (DAT_005967bc + 0x26);
                        }
                        else {
                            local_90 = (DAT_005967bc + 0x35);
                        }
                        local_98 = local_90;
                        if ((param_1 != 0x0) && (local_24 != 0x0)) {
                            FUN_0047d83f(param_1,local_18,local_14);
                            local_a8 = *(param_1 + 0x1d);
                            local_a4 = *(param_1 + 0x21);
                            local_a0 = *(param_1 + 0x25);
                            local_9c = *(param_1 + 0x29);
                            local_ac = local_98 * 0x4;
                            local_b0 = param_1;
                            local_b4 = *(param_1 + 0x85);
                            local_b8 = *(&DAT_004bf884 + local_ac) + local_b4;
                            local_bc = local_98 * 0x4;
                            local_c0 = param_1;
                            local_c4 = *(param_1 + 0x89);
                            local_c8 = *(&DAT_004bf8bc + local_bc) + local_c4;
                            iVar2 = FUN_004aa04b(&local_a8,local_b8,local_c8);
                            if (iVar2 != 0x0) {
                                FUN_00498a5b(&local_a8);
                                local_cc = local_94;
                                FUN_004a2d6b();
                                for (local_d0 = 0x0; local_d0 < 0x7; local_d0 = local_d0 + 0x1) {
                                    timer_func_0049e710(0x7d);
                                    FUN_00496ee6(local_cc,local_b8,local_c8,0x22,0x22);
                                    local_cc = local_cc + 0x121;
                                }
                                local_e0 = local_b8;
                                local_dc = local_c8;
                                local_d8 = 0x22;
                                local_d4 = 0x22;
                                FUN_00498a5b(&local_e0);
                                FUN_00471bcd(param_1);
                                FUN_00498ae4();
                                FUN_00498ae4();
                            }
                        }
                        local_e4 = *(DAT_005967bc + 0x3a) & *(&DAT_004be9b0 + DAT_004c9758 * 0x4);
                        if ((((&DAT_00569a98)[(*(DAT_005967bc + 0x23) >> 0x18) * 0x1e22] & 0x2) != 0x0) &&
                            (hide_ai_opt_004d55a4 != 0x0)) {
                            local_e4 = 0x0;
                        }
                        if (((local_e4 != 0x0) && (local_24 == 0x0)) && (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) != 0x0)) {
                            FUN_00435045(&DAT_005967b8);
                            FUN_00424d33(DAT_005967bc);
                            FUN_0043507c(&DAT_005967b8);
                        }
                        local_24 = local_e4;
                        if ((param_1 != 0x0) && (local_e4 != 0x0)) {
                            local_e8 = local_38;
                            local_ec = local_34;
                            if ((*(DAT_005967bc + 0x3a) & 0x80) == 0x0) {
                                cStack240 = (DAT_005967bc + 0x26);
                            }
                            else {
                                cStack240 = (DAT_005967bc + 0x35);
                            }
                            local_98 = cStack240;
                            FUN_004729ba(local_98,&local_e8,&local_ec,0x0);
                            FUN_0047d83f(param_1,local_e8,local_ec);
                            local_a8 = *(param_1 + 0x1d);
                            local_a4 = *(param_1 + 0x21);
                            local_a0 = *(param_1 + 0x25);
                            local_9c = *(param_1 + 0x29);
                            local_e8 = *(&DAT_004bf884 + local_98 * 0x4) + *(param_1 + 0x85);
                            local_ec = *(&DAT_004bf8bc + local_98 * 0x4) + *(param_1 + 0x89);
                            iVar2 = FUN_004aa04b(&local_a8,local_e8,local_ec);
                            if (iVar2 == 0x0) {
                                FUN_00435045(&DAT_005967b8);
                                iVar2 = FUN_0047d6a0(DAT_005b8850,local_38,local_34,0x1);
                                if (iVar2 != 0x0) {
                                    uVar1 = *(DAT_005b8850 + 0x4d);
                                    *(param_2 + 0x59) = *(DAT_005b8850 + 0x49);
                                    *(param_2 + 0x5d) = uVar1;
                                    FUN_00471bcd(param_2);
                                }
                                FUN_0043507c(&DAT_005967b8);
                                local_e8 = local_38;
                                local_ec = local_34;
                                FUN_004729ba(local_98,&local_e8,&local_ec,0x0);
                                FUN_0047d83f(param_1,local_e8,local_ec);
                                local_e8 = *(&DAT_004bf884 + local_98 * 0x4) + *(param_1 + 0x85);
                                local_ec = *(&DAT_004bf8bc + local_98 * 0x4) + *(param_1 + 0x89);
                            }
                            local_148 = local_94 + 0x6c6;
                            FUN_004a2d6b();
                            for (local_14c = 0x0; local_14c < 0x7; local_14c = local_14c + 0x1) {
                                timer_func_0049e710(0x7d);
                                FUN_00496ee6(local_148,local_e8,local_ec,0x22,0x22);
                                local_148 = local_148 + -0x121;
                            }
                            FUN_00471bcd(param_1);
                            timer_func_0049e710(0x7d);
                        }
                        FUN_0049af50(local_94);
                        FUN_0049536f();
                        local_40 = 0x1;
                    }
                }
            }
        }
    }
    return local_40;
}



fn FUN_00477c14(param_1: i32)

{
    let mut iVar1: i32;
    let puVar2: *mut u32;
    let puVar3: *mut u32;
    let mut in_stack_ffffff84: u32;
    let mut in_stack_ffffff88: u32;
    let mut in_stack_ffffff8c: u32;
    let mut in_stack_ffffff90: u32;
    let mut in_stack_ffffff94: u32;
    let mut in_stack_ffffff98: u32;
    ulonglong in_stack_ffffff9c;
    let mut local_18: i32;
    let local_14: *mut u32;

    if (param_1 != 0x0) {
        FUN_004953d7();
        if (DAT_005967bc != 0x0) {
            if ((*(DAT_005967bc + 0x3a) & 0x80) == 0x0) {
                *
                    (&DAT_005b707e +
                        (DAT_005967bc + 0x8) * 0x4e +
                        *(&DAT_004be878 + (*(DAT_005967bc + 0x23) >> 0x18) * 0x4) * 0x4) = 0x0;
            }
            else {
                *
                    (&DAT_005b707e +
                        (DAT_005967bc + 0x8) * 0x4e +
                        *(&DAT_004be878 + (*(DAT_005967bc + 0x32) >> 0x18) * 0x4) * 0x4) = 0x0;
            }
            local_14 = FUN_004499d9(DAT_005967bc,&local_18,*(DAT_005967bc + 0x23) >> 0x18,0x1);
            puVar2 = local_14 + 0x8;
            puVar3 = &stack0xffffff84;
            for (iVar1 = 0xb; iVar1 != 0x0; iVar1 = iVar1 + -0x1) {
                *puVar3 = *puVar2;
                puVar2 = puVar2 + 0x1;
                puVar3 = puVar3 + 0x1;
            }
            puVar3 = puVar2;
            FUN_0047e2a6(DAT_005b8850,in_stack_ffffff84,in_stack_ffffff88,in_stack_ffffff8c,in_stack_ffffff90,
                         in_stack_ffffff94,in_stack_ffffff98,in_stack_ffffff9c);
        }
        FUN_0049536f();
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00477d4b(param_1: i32,param_2: i32,param_3: i32)

{
    let mut uVar1: u32;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let cStack56: u8;
    let mut local_34: i32;
    let mut local_30: i32;
    let mut local_2c: i32;
    let mut local_28: i32;

    if ((*(DAT_005967bc + 0x3a) & 0x1) == 0x0) {
        if ((*(DAT_005967bc + 0x3a) & 0x80) == 0x0) {
            cStack56 = (DAT_005967bc + 0x26);
        }
        else {
            cStack56 = (DAT_005967bc + 0x35);
        }
        FUN_0047d83f(DAT_005b8850,param_1,param_2);
        local_34 = *(DAT_005b8850 + 0x85) - *(DAT_005b8850 + 0x1d);
        local_30 = *(DAT_005b8850 + 0x89) - *(DAT_005b8850 + 0x21);
        if (((((local_30 < 0x0) || (local_34 < 0x0)) || (*(DAT_005b8850 + 0x29) <= local_30 + 0x22)) ||
            (*(DAT_005b8850 + 0x25) <= local_34 + 0x22)) && (param_3 != 0x0)) {
            iVar3 = FUN_0047d6a0(DAT_005b8850,param_1,param_2,0x1);
            iVar2 = DAT_005b8854;
            if (iVar3 != 0x0) {
                uVar1 = *(DAT_005b8850 + 0x4d);
                *(DAT_005b8854 + 0x59) = *(DAT_005b8850 + 0x49);
                *(iVar2 + 0x5d) = uVar1;
                FUN_00471bcd(DAT_005b8854);
            }
            FUN_0047d83f(DAT_005b8850,param_1,param_2);
            local_34 = *(DAT_005b8850 + 0x85) - *(DAT_005b8850 + 0x1d);
            local_30 = *(DAT_005b8850 + 0x89) - *(DAT_005b8850 + 0x21);
        }
        else {
            if (param_3 != 0x0) {
                local_28 = 0x0;
                local_2c = 0x0;
                if (local_34 < 0x44) {
                    local_2c = (local_34 + -0x66) / 0x22;
                }
                if (*(DAT_005b8850 + 0x25) < local_34 + 0x66) {
                    local_2c = (local_34 - (*(DAT_005b8850 + 0x25) + -0x66)) / 0x22;
                }
                if (local_30 < 0x44) {
                    local_28 = (local_30 + -0x66) / 0x22;
                }
                if (*(DAT_005b8850 + 0x29) < local_30 + 0x69) {
                    local_28 = (local_30 - (*(DAT_005b8850 + 0x29) + -0x69)) / 0x22;
                }
                if ((local_2c != 0x0) || (local_28 != 0x0)) {
                    iVar3 = FUN_0047d6a0(DAT_005b8850,
                                         (*(DAT_005b8850 + 0x75) >> 0x1) + *(DAT_005b8850 + 0x49) + local_2c,
                                         *(DAT_005b8850 + 0x4d) + (*(DAT_005b8850 + 0x71) >> 0x1) + local_28 * 0x2,
                                         0x1);
                    iVar2 = DAT_005b8854;
                    if (iVar3 != 0x0) {
                        uVar1 = *(DAT_005b8850 + 0x4d);
                        *(DAT_005b8854 + 0x59) = *(DAT_005b8850 + 0x49);
                        *(iVar2 + 0x5d) = uVar1;
                        FUN_00471bcd(DAT_005b8854);
                    }
                    FUN_0047d83f(DAT_005b8850,param_1,param_2);
                    local_34 = *(DAT_005b8850 + 0x85) - *(DAT_005b8850 + 0x1d);
                    local_30 = *(DAT_005b8850 + 0x89) - *(DAT_005b8850 + 0x21);
                }
            }
        }
        if (param_3 != -0x1) {
            if (((local_30 == -0x22 || local_30 + 0x22 < 0x0) || (local_34 == -0x22 || local_34 + 0x22 < 0x0)) ||
                ((*(DAT_005b8850 + 0x29) <= local_30 || (*(DAT_005b8850 + 0x25) <= local_34)))) {
                if (_DAT_005b8860 != 0x0) {
                    FUN_004a756b();
                    _DAT_005b8860 = 0x0;
                }
            }
            else {
                FUN_004a763f(*(DAT_005b8850 + 0x1d) + local_34 + *(&DAT_004bf884 + cStack56 * 0x4),
                             *(DAT_005b8850 + 0x21) + local_30 + *(&DAT_004bf8bc + cStack56 * 0x4));
                if ((_DAT_005b8860 == 0x0) && ((DAT_005967b8 & 0x1) == 0x0)) {
                    FUN_004a75a6();
                    _DAT_005b8860 = 0x1;
                }
            }
        }
    }
    else {
        if (_DAT_005b8860 != 0x0) {
            FUN_004a756b();
            _DAT_005b8860 = 0x0;
        }
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0047830a(param_1: *mut *mut u32)

{
    let mut uVar1: u32;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut uVar4: u32;

    if (_DAT_005b8860 != 0x0) {
        FUN_004a756b();
        _DAT_005b8860 = 0x0;
    }
    FUN_004598c9(DAT_005b8850,param_1);
    if (DAT_005967bc != 0x0) {
        DAT_005b7068 = (DAT_005967bc + 0x20);
        DAT_005b8808 = 0x0;
        if ((*(DAT_005967bc + 0x3a) & 0x1) == 0x0) {
            FUN_0047ed66();
            uVar4 = FUN_004710a6(&DAT_005967b8,(DAT_005967bc + 0x22),(DAT_005967bc + 0x24),
                                 *(DAT_005967bc + 0x41),*(DAT_005967bc + 0x42));
            if (uVar4 != 0x0) {
                FUN_0047eda9(DAT_005b8850,uVar4,(DAT_005967bc + 0x22),(DAT_005967bc + 0x24),
                             *(DAT_005967bc + 0x41),*(DAT_005967bc + 0x42));
                FUN_00471bcd(DAT_005b8850);
                FUN_00471bcd(DAT_005b8854);
            }
            FUN_00477d4b((DAT_005967bc + 0x22),(DAT_005967bc + 0x24),0x1);
            return;
        }
    }
    iVar3 = FUN_0047d6a0(DAT_005b8850,*(&DAT_005b7076 + DAT_005b7068 * 0x4e) >> 0x10,
                         *(&DAT_005b7078 + DAT_005b7068 * 0x4e) >> 0x10,0x1);
    iVar2 = DAT_005b8854;
    if (iVar3 != 0x0) {
        uVar1 = *(DAT_005b8850 + 0x4d);
        *(DAT_005b8854 + 0x59) = *(DAT_005b8850 + 0x49);
        *(iVar2 + 0x5d) = uVar1;
        FUN_00471bcd(DAT_005b8854);
    }
    return;
}



fn FUN_004786a2(param_1: i32,param_2: i32,param_3: i32) -> u32

{
    let mut pcVar1: String;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let local_11a4: u8 [0x80];
    let local_1124: u8 [0x82c];
    let mut local_8f8: *mut u8;
    let mut local_8f4: i32;
    let mut local_8f0: u32;
    let mut local_8ec: u32;
    let local_8e8: u8 [0x80];
    let mut local_868: String;
    let local_864: u8 [0x82c];
    let mut local_38: *mut u8;
    let mut local_34: i32;
    let mut local_30: u32;
    let mut local_2c: u32;
    let mut local_28: u32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: u32;
    let mut local_14: u32;

    local_20 = *(&DAT_005b7076 + param_1 * 0x4e) >> 0x10;
    local_1c = *(&DAT_005b7078 + param_1 * 0x4e) >> 0x10;
    local_18 = 0xffffffff;
    local_14 = 0x0;
    loop {
    if (0x7 < local_14) {
        return 0x0;
    }
    if ((local_20 + *(&DAT_004bea20 + local_14 * 0x4) == param_2) &&
        (local_1c + *(&DAT_004bea40 + local_14 * 0x4) == param_3)) {
        if (local_14 < 0x5) {
            local_18 = local_14;
        }
        else {
            FUN_00473663(param_1,param_2,param_3);
            local_18 = FUN_00472bb0(0x1);
        }
        if (local_18 != 0xffffffff) {
            if (local_18 == 0x8) {
                FUN_00423530(DAT_004c9754);
            }
            else {
                local_38 = &DAT_004d55a8;
                local_34 = DAT_004c9754;
                local_30 = local_18;
                local_2c = (byte)(&DAT_004d55a8)[DAT_004c9754 * 0xe + local_18];
                if (local_2c == 0x0) {
                    iVar3 = FUN_00410fb3(&DAT_005967b8,0x0);
                    if (iVar3 == 0x0) {
                        return 0x0;
                    }
                    FUN_00450dbf(local_864,param_1,param_2,param_3,0x0,0x0,local_18,0x1);
                    iVar3 = FUN_00452e41(local_864);
                    if (iVar3 == 0x0) {
                        FUN_00499050(DAT_0059679c,local_18 + 0x414);
                        if (local_18 < 0x5) {
                            local_868 = FUN_00499050(DAT_0059679c,0x713e);
                        }
                        else {
                            local_868 = FUN_00499050(DAT_0059679c,0x713a);
                        }
                        pcVar1 = FUN_00499050(DAT_0059679c,0x734b);
                        FUN_0049c2e0(local_8e8,pcVar1);
                        uVar2 = FUN_0049d2e0(0x0,0x3,local_8e8);
                        if (uVar2 == 0x0) {
                            return 0x0;
                        }
                    }
                }
                else {
                    local_8f8 = &DAT_004d55a8;
                    local_8f4 = DAT_004c9754;
                    local_8f0 = local_18;
                    local_8ec = (byte)(&DAT_004d55a8)[DAT_004c9754 * 0xe + local_18];
                    if (local_8ec == 0x1) {
                        iVar3 = FUN_00410fb3(&DAT_005967b8,0x0);
                        if (iVar3 == 0x0) {
                            return 0x0;
                        }
                        FUN_00450dbf(local_1124,param_1,param_2,param_3,0x0,0x0,local_18,0x1);
                        iVar3 = FUN_00452e41(local_1124);
                        if (iVar3 == 0x0) {
                            FUN_00499050(DAT_0059679c,local_18 + 0x414);
                            pcVar1 = FUN_00499050(DAT_0059679c,0x7365);
                            FUN_0049c2e0(local_11a4,pcVar1);
                            uVar2 = FUN_0049d2e0(0x0,0x3,local_11a4);
                            if (uVar2 == 0x0) {
                                return 0x0;
                            }
                        }
                    }
                }
            }
            iVar3 = FUN_00410fb3(&DAT_005967b8,0x0);
            if (iVar3 == 0x0) {
                return 0x0;
            }
            local_28 = FUN_0040a70e(0x0,param_2,param_3,local_18,0x1);
            FUN_004864f7();
            return 0x1;
        }
    }
    local_14 = local_14 + 0x1;
} while( true );
}



fn FUN_004789fc() -> i32

{
let mut iVar1: i32;
let mut local_18: i32;
let mut local_14: i32;

local_18 = 0x0;
for (local_14 = 0x0; local_14 < 0x5; local_14 = local_14 + 0x1) {
if ((((((&DAT_00569a98)[local_14 * 0x1e22] & 0x1) == 0x0) && (iVar1 = FUN_00406cc8(local_14), iVar1 != 0x0)) &&
(DAT_004c5048 != local_14)) && ((DAT_004c5050 != local_14 && (DAT_004c5058 != local_14)))) {
local_18 = local_18 + 0x1;
}
}
return local_18;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00478ab4()

{
    let piVar1: *mut i32;;
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_1c: i32;
    let mut local_14: i32;

    piVar1 = FUN_0049c4bd(s_misc_planet_txt_004c246d,&DAT_004c246a);
    if (piVar1 != 0x0) {
        for (local_14 = 0x0; local_14 < 0x28; local_14 = local_14 + 0x1) {
            if (*(&DAT_005b70c2 + local_14 * 0x4e) != 0x0) {
                FUN_004a9b70(piVar1,s__2d____2d___2d___s_004c247d);
                for (local_1c = 0x0; local_1c < 0x78; local_1c = local_1c + 0x1) {
                    if (*(&DAT_005b7cb8 + local_1c * 0x14) != 0x0) {
                        if ((*(&DAT_005b7076 + local_14 * 0x4e) >> 0x10 == *(&DAT_005b7ca8 + local_1c * 0x14)) &&
                            (*(&DAT_005b7078 + local_14 * 0x4e) >> 0x10 == *(&DAT_005b7cac + local_1c * 0x14))) {
                            for (local_24 = 0x0; local_24 < ram0x005b706e >> 0x10; local_24 = local_24 + 0x1) {
                            }
                            FUN_004a9b70(piVar1,s____2d___2d_______2d___2d___s_004c2491);
                        }
                        else {
                            if ((*(&DAT_005b7076 + local_14 * 0x4e) >> 0x10 == *(&DAT_005b7cb0 + local_1c * 0x14)) &&
                                (*(&DAT_005b7078 + local_14 * 0x4e) >> 0x10 == *(&DAT_005b7cb4 + local_1c * 0x14))) {
                                for (local_28 = 0x0; local_28 < 0x28; local_28 = local_28 + 0x1) {
                                }
                                FUN_004a9b70(piVar1,s____2d___2d_______2d___2d___s_004c24b2);
                            }
                        }
                    }
                }
            }
        }
        FUN_0049ca40(piVar1);
    }
    return;
}



// WARNING: Could not reconcile some variable overlaps

fn FUN_00478d5b(param_1: u32) -> u32

{
    let mut pcVar1: String;
    let mut uVar2: u32;
    let puVar3: *mut u32;
    let puVar4: *mut u32;
    u32 auStackY672 [0x9];
    let mut uStackY636: u32;
    let mut iVar5: i32;
    let mut in_stack_fffffd98: i32;
    let mut local_234: u32;
    let mut local_230: u32;
    let mut local_22c: u32;
    let mut local_228: u32;
    let mut local_224: u32;
    let mut local_220: u32;
    let mut local_21c: u32;
    let mut local_218: u32;
    let mut local_214: u32;
    let mut local_210: u32;
    let mut local_20c: u32;
    let mut local_208: u32;
    let mut local_204: u32;
    let mut local_200: u32;
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
    let local_1d4: i16;
    let mut local_1d0: u32;
    let mut local_1cc: u32;
    let mut local_1c8: u32;
    let mut local_1c4: i32;
    let mut local_1c0: i32;
    let mut local_1bc: u32;
    let mut local_1b8: i32;
    let mut local_1b4: i32;
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
    let local_170: u8 [0x104];
    let local_6c: u8 [0x4];
    let local_68: u8 [0x4];
    let local_64: u8 [0x4];
    let local_60: u8 [0x4];
    let local_5c: u8 [0x4];
    let local_58: u8 [0x6];
    let local_52: u8 [0x4];
    let local_4e: u16;
    let local_4c: u8 [0x4];
    u32 local_48 [0x5];
    let local_34: *mut *mut char;
    let mut local_30: i32;
    let mut local_2c: i32;
    let mut local_28: i32;
    let local_24: u32;
    let local_20: u32;
    let mut local_1c: i32;
    let local_18: i16;
    let mut local_14: u32;

    local_28 = 0x0;
    uStackY636 = 0x478d84;
    FUN_0049c2e0(local_170,s__s_gal_004c24d3);
    local_34 = FUN_0049c4bd(local_170,&DAT_004c24da);
    if (local_34 == 0x0) {
        uStackY636 = 0x478dbc;
        pcVar1 = FUN_00499050(DAT_0059679c,0x7394);
        pop_err_msg_box_and_exit_004a02f5(pcVar1);
    }
    local_24 = FUN_0047d5ab(local_34);
    FUN_00472275();
    uStackY636 = 0x478df4;
    uVar2 = FUN_004a7970(&local_2c,0x4,0x1,local_34);
    local_174 = (uVar2 == 0x0);
    local_28 = local_28 + local_174;
    uStackY636 = 0x478e2d;
    uVar2 = FUN_004a7970(&DAT_00591cb0,0x4,0x1,local_34);
    local_178 = (uVar2 == 0x0);
    local_28 = local_28 + local_178;
    if ((local_2c != 0xeaa65) && (local_2c < 0xea9ee)) {
        pop_err_msg_box_and_exit_004a02f5(s_LoadGalMap_____d_Wrong_version_n_004c24dd);
    }
    iVar5 = 0x64;
    pcVar1 = FUN_00499050(DAT_0059679c,0x735a);
    FUN_004a2ff0(0x0,pcVar1,iVar5,in_stack_fffffd98);
    uStackY636 = 0x478ead;
    FUN_004a7970(&DAT_005b4ac0,0x2580,0x1,local_34);
    local_30 = 0x0;
    uStackY636 = 0x478ec8;
    uVar2 = FUN_004a7970(&local_14,0x2,0x1,local_34);
    local_17c = (uVar2 == 0x0);
    local_28 = local_28 + local_17c;
    while (local_14 != -0x2) {
        (&DAT_005b7078 + local_30 * 0x4e) = local_14;
        uStackY636 = 0x478f23;
        uVar2 = FUN_004a7970((&DAT_005b707a + local_30 * 0x4e),0x2,0x1,local_34);
        local_180 = (uVar2 == 0x0);
        local_28 = local_28 + local_180;
        uStackY636 = 0x478f65;
        uVar2 = FUN_004a7970((&DAT_005b707c + local_30 * 0x4e),0x2,0x1,local_34);
        local_184 = (uVar2 == 0x0);
        local_28 = local_28 + local_184;
        uStackY636 = 0x478fa7;
        uVar2 = FUN_004a7970((&DAT_005b707e + local_30 * 0x4e),0x20,0x1,local_34);
        local_188 = (uVar2 == 0x0);
        local_28 = local_28 + local_188;
        uStackY636 = 0x478fe9;
        uVar2 = FUN_004a7970((&DAT_005b709e + local_30 * 0x4e),0x20,0x1,local_34);
        local_18c = (uVar2 == 0x0);
        local_28 = local_28 + local_18c;
        uStackY636 = 0x47902b;
        uVar2 = FUN_004a7970((local_30 * 0x4e + 0x5b70be),0x2,0x1,local_34);
        local_190 = (uVar2 == 0x0);
        local_28 = local_28 + local_190;
        uStackY636 = 0x47906d;
        uVar2 = FUN_004a7970((local_30 * 0x4e + 0x5b70c0),0x2,0x1,local_34);
        local_194 = (uVar2 == 0x0);
        local_28 = local_28 + local_194;
        uStackY636 = 0x4790af;
        uVar2 = FUN_004a7970((&DAT_005b70c2 + local_30 * 0x4e),0x4,0x1,local_34);
        local_198 = (uVar2 == 0x0);
        local_28 = local_28 + local_198;
        uStackY636 = 0x4790f6;
        uVar2 = FUN_004a7970((&DAT_004daab0 + local_30 * 0x3890),0x4,0x1,local_34);
        local_19c = (uVar2 == 0x0);
        local_28 = local_28 + local_19c;
        if (local_2c < 0xeaa00) {
            uStackY636 = 0x47913d;
            uVar2 = FUN_004a7970(&local_1a0,0x4,0x1,local_34);
            local_1a4 = (uVar2 == 0x0);
            local_28 = local_28 + local_1a4;
            uStackY636 = 0x479177;
            uVar2 = FUN_004a7970(&local_1a0,0x4,0x1,local_34);
            local_1a8 = (uVar2 == 0x0);
            local_28 = local_28 + local_1a8;
            uStackY636 = 0x4791b1;
            uVar2 = FUN_004a7970(&local_1a0,0x4,0x1,local_34);
            local_1ac = (uVar2 == 0x0);
            local_28 = local_28 + local_1ac;
        }
        uStackY636 = 0x4791fb;
        uVar2 = FUN_004a7970((local_30 * 0x3890 + 0x4d7e00),0x2cb0,0x1,local_34);
        local_1b0 = (uVar2 == 0x0);
        local_28 = local_28 + local_1b0;
        local_1c4 = *(&DAT_005b7078 + local_30 * 0x4e) >> 0x10;
        local_1c0 = *(&DAT_005b7076 + local_30 * 0x4e) >> 0x10;
        local_1bc = param_1;
        local_1c8 = 0x2;
        *(&DAT_005b4ac0 + local_1c4 * 0x4 + local_1c0 * 0xc0) = 0x2;
        local_30 = local_30 + 0x1;
        uStackY636 = 0x4792aa;
        local_1b8 = local_1c0;
        local_1b4 = local_1c4;
        uVar2 = FUN_004a7970(&local_14,0x2,0x1,local_34);
        local_1cc = (uVar2 == 0x0);
        local_28 = local_28 + local_1cc;
        local_20 = FUN_004aa690(local_34);
        local_1c = (local_20 * 0x64) / local_24;
        FUN_004a36b0(local_1c);
    }
    local_30 = 0x0;
    uStackY636 = 0x479317;
    uVar2 = FUN_004a7970(&local_14,0x2,0x1,local_34);
    local_1d0 = (uVar2 == 0x0);
    local_28 = local_28 + local_1d0;
    while (local_14 != -0x2) {
        *(&DAT_005b7ca8 + local_30 * 0x14) = local_14;
        uStackY636 = 0x47936a;
        uVar2 = FUN_004a7970(&local_1d4,0x2,0x1,local_34);
        local_1d8 = (uVar2 == 0x0);
        local_28 = local_28 + local_1d8;
        *(&DAT_005b7cac + local_30 * 0x14) = local_1d4;
        uStackY636 = 0x4793b5;
        uVar2 = FUN_004a7970(&local_1d4,0x2,0x1,local_34);
        local_1dc = (uVar2 == 0x0);
        local_28 = local_28 + local_1dc;
        *(&DAT_005b7cb0 + local_30 * 0x14) = local_1d4;
        uStackY636 = 0x479400;
        uVar2 = FUN_004a7970(&local_1d4,0x2,0x1,local_34);
        local_1e0 = (uVar2 == 0x0);
        local_28 = local_28 + local_1e0;
        *(&DAT_005b7cb4 + local_30 * 0x14) = local_1d4;
        uStackY636 = 0x479453;
        uVar2 = FUN_004a7970((&DAT_005b7cb8 + local_30 * 0x14),0x4,0x1,local_34);
        local_1e4 = (uVar2 == 0x0);
        local_28 = local_28 + local_1e4;
        local_30 = local_30 + 0x1;
        uStackY636 = 0x479490;
        uVar2 = FUN_004a7970(&local_14,0x2,0x1,local_34);
        local_1e8 = (uVar2 == 0x0);
        local_28 = local_28 + local_1e8;
        local_20 = FUN_004aa690(local_34);
        local_1c = (local_20 * 0x64) / local_24;
        FUN_004a36b0(local_1c);
    }
    uStackY636 = 0x4794f6;
    iVar5 = FUN_00487685(local_34,0x0,-0x1,local_2c);
    local_28 = local_28 + iVar5;
    local_30 = 0x0;
    uStackY636 = 0x479514;
    uVar2 = FUN_004a7970(&local_14,0x2,0x1,local_34);
    local_1ec = (uVar2 == 0x0);
    local_28 = local_28 + local_1ec;
    while (local_14 != -0x2) {
        uStackY636 = 0x47955d;
        local_6c._0_2_ = local_14;
        uVar2 = FUN_004a7970((local_6c + 0x2),0x2,0x1,local_34);
        local_1f0 = (uVar2 == 0x0);
        local_28 = local_28 + local_1f0;
        uStackY636 = 0x479594;
        uVar2 = FUN_004a7970(local_68,0x2,0x1,local_34);
        local_1f4 = (uVar2 == 0x0);
        local_28 = local_28 + local_1f4;
        uStackY636 = 0x4795cb;
        uVar2 = FUN_004a7970((local_68 + 0x2),0x2,0x1,local_34);
        local_1f8 = (uVar2 == 0x0);
        local_28 = local_28 + local_1f8;
        uStackY636 = 0x479602;
        uVar2 = FUN_004a7970(local_64,0x2,0x1,local_34);
        local_1fc = (uVar2 == 0x0);
        local_28 = local_28 + local_1fc;
        uStackY636 = 0x479639;
        uVar2 = FUN_004a7970((local_64 + 0x2),0x2,0x1,local_34);
        local_200 = (uVar2 == 0x0);
        local_28 = local_28 + local_200;
        uStackY636 = 0x479670;
        uVar2 = FUN_004a7970(local_60,0x2,0x1,local_34);
        local_204 = (uVar2 == 0x0);
        local_28 = local_28 + local_204;
        uStackY636 = 0x4796a7;
        uVar2 = FUN_004a7970((local_60 + 0x2),0x2,0x1,local_34);
        local_208 = (uVar2 == 0x0);
        local_28 = local_28 + local_208;
        uStackY636 = 0x4796de;
        uVar2 = FUN_004a7970(local_5c,0x2,0x1,local_34);
        local_20c = (uVar2 == 0x0);
        local_28 = local_28 + local_20c;
        uStackY636 = 0x479715;
        uVar2 = FUN_004a7970((local_5c + 0x2),0x2,0x1,local_34);
        local_210 = (uVar2 == 0x0);
        local_28 = local_28 + local_210;
        uStackY636 = 0x47974c;
        uVar2 = FUN_004a7970(local_58,0x2,0x1,local_34);
        local_214 = (uVar2 == 0x0);
        local_28 = local_28 + local_214;
        uStackY636 = 0x479783;
        uVar2 = FUN_004a7970((local_58 + 0x2),0x2,0x1,local_34);
        local_218 = (uVar2 == 0x0);
        local_28 = local_28 + local_218;
        if (local_2c < 0xea9ee) {
            FUN_00482992(local_6c,0x4b);
        }
        else {
            uStackY636 = 0x4797c3;
            uVar2 = FUN_004a7970(&local_18,0x2,0x1,local_34);
            local_21c = (uVar2 == 0x0);
            local_28 = local_28 + local_21c;
            FUN_00482992(local_6c,local_18);
        }
        uStackY636 = 0x47981b;
        uVar2 = FUN_004a7970(local_52,0x2,0x1,local_34);
        local_220 = (uVar2 == 0x0);
        local_28 = local_28 + local_220;
        uStackY636 = 0x479852;
        uVar2 = FUN_004a7970((local_52 + 0x2),0x2,0x1,local_34);
        local_224 = (uVar2 == 0x0);
        local_28 = local_28 + local_224;
        if (local_2c < 0xeaa65) {
            local_4e = 0xffff;
        }
        else {
            uStackY636 = 0x479892;
            uVar2 = FUN_004a7970(&local_4e,0x2,0x1,local_34);
            local_228 = (uVar2 == 0x0);
            local_28 = local_28 + local_228;
        }
        uStackY636 = 0x4798d1;
        uVar2 = FUN_004a7970(local_4c,0x2,0x1,local_34);
        local_22c = (uVar2 == 0x0);
        local_28 = local_28 + local_22c;
        uStackY636 = 0x479908;
        uVar2 = FUN_004a7970((local_4c + 0x2),0x2,0x1,local_34);
        local_230 = (uVar2 == 0x0);
        local_28 = local_28 + local_230;
        uStackY636 = 0x47993f;
        uVar2 = FUN_004a7970(local_48,0x4,0x1,local_34);
        local_234 = (uVar2 == 0x0);
        local_28 = local_28 + local_234;
        if (local_64._0_2_ == 0x9) {
            local_64 = 0x90009;
        }
        else {
            if (local_64._0_2_ == 0xa) {
                local_64 = 0xa000a;
            }
            else {
                if (local_64._0_2_ == 0xb) {
                    local_64 = 0xb000b;
                }
                else {
                    if (local_64._0_2_ == 0xc) {
                        local_64 = 0xc000c;
                    }
                }
            }
        }
        if ((local_64._2_2_ < 0xa) || (0xc < local_64._2_2_)) {
            local_64 = local_64 & 0xffff | local_64 << 0x10;
            local_48[0] = local_48[0] & 0xfffffeff;
        }
        puVar3 = local_6c;
        puVar4 = &stack0xfffffd98;
        for (iVar5 = 0xc; iVar5 != 0x0; iVar5 = iVar5 + -0x1) {
            *puVar4 = *puVar3;
            puVar3 = puVar3 + 0x1;
            puVar4 = puVar4 + 0x1;
        }
            *puVar4 = *puVar3;
        puVar3 = &stack0xfffffd98;
        puVar4 = auStackY672;
        for (iVar5 = 0xc; iVar5 != 0x0; iVar5 = iVar5 + -0x1) {
            *puVar4 = *puVar3;
            puVar3 = puVar3 + 0x1;
            puVar4 = puVar4 + 0x1;
        }
            *puVar4 = *puVar3;
        FUN_004813ca();
        local_30 = local_30 + 0x1;
        uStackY636 = 0x479a06;
        uVar2 = FUN_004a7970(&local_14,0x2,0x1,local_34);
        local_28 = local_28 + (uVar2 == 0x0);
        local_20 = FUN_004aa690(local_34);
        local_1c = (local_20 * 0x64) / local_24;
        FUN_004a36b0(local_1c);
    }
    FUN_004a36b0(0x64);
    FUN_004a3800();
    FUN_00482491();
    FUN_0046f15b();
    FUN_0049ca40(local_34);
    return 0x1;
}



fn FUN_00479a92() -> u32

{
    let mut pcVar1: String;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut local_1dc: u32;
    let mut local_1d8: u32;
    let mut local_1d4: u32;
    let mut local_1d0: u32;
    let mut local_1cc: u32;
    let mut local_1c8: u32;
    let mut local_1c4: u32;
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
    let local_130: u8 [0x108];
    let local_28: *mut i32;;
    let mut local_24: i32;
    let mut local_20: u32;
    let mut local_1c: i32;
    let local_18: *mut u32;
    let mut local_14: u32;

    local_1c = 0x0;
    FUN_0049c2e0(local_130,s__s_gal_004c2504);
    local_28 = FUN_0049c4bd(local_130,&DAT_004c250b);
    if (local_28 == 0x0) {
        pcVar1 = FUN_00499050(DAT_0059679c,0x7394);
        pop_err_msg_box_and_exit_004a02f5(pcVar1);
    }
    local_20 = 0xeaa65;
    uVar2 = FUN_004a7160(&local_20,0x4,0x1,local_28);
    local_134 = (uVar2 == 0x0);
    local_1c = local_1c + local_134;
    uVar2 = FUN_004a7160(&DAT_00591cb0,0x4,0x1,local_28);
    local_138 = (uVar2 == 0x0);
    local_1c = local_1c + local_138;
    uVar2 = FUN_004a7160(&DAT_005b4ac0,0x2580,0x1,local_28);
    local_13c = (uVar2 == 0x0);
    local_1c = local_1c + local_13c;
    for (local_24 = 0x0; local_24 < 0x28; local_24 = local_24 + 0x1) {
        if (*(&DAT_005b70c2 + local_24 * 0x4e) != 0x0) {
            uVar2 = FUN_004a7160((&DAT_005b7078 + local_24 * 0x4e),0x2,0x1,local_28);
            local_140 = (uVar2 == 0x0);
            local_1c = local_1c + local_140;
            uVar2 = FUN_004a7160((&DAT_005b707a + local_24 * 0x4e),0x2,0x1,local_28);
            local_144 = (uVar2 == 0x0);
            local_1c = local_1c + local_144;
            uVar2 = FUN_004a7160((&DAT_005b707c + local_24 * 0x4e),0x2,0x1,local_28);
            local_148 = (uVar2 == 0x0);
            local_1c = local_1c + local_148;
            uVar2 = FUN_004a7160((&DAT_005b707e + local_24 * 0x4e),0x20,0x1,local_28);
            local_14c = (uVar2 == 0x0);
            local_1c = local_1c + local_14c;
            uVar2 = FUN_004a7160((&DAT_005b709e + local_24 * 0x4e),0x20,0x1,local_28);
            local_150 = (uVar2 == 0x0);
            local_1c = local_1c + local_150;
            uVar2 = FUN_004a7160((local_24 * 0x4e + 0x5b70be),0x2,0x1,local_28);
            local_154 = (uVar2 == 0x0);
            local_1c = local_1c + local_154;
            uVar2 = FUN_004a7160((local_24 * 0x4e + 0x5b70c0),0x2,0x1,local_28);
            local_158 = (uVar2 == 0x0);
            local_1c = local_1c + local_158;
            uVar2 = FUN_004a7160((&DAT_005b70c2 + local_24 * 0x4e),0x4,0x1,local_28);
            local_15c = (uVar2 == 0x0);
            local_1c = local_1c + local_15c;
            uVar2 = FUN_004a7160((&DAT_004daab0 + local_24 * 0x3890),0x4,0x1,local_28);
            local_160 = (uVar2 == 0x0);
            local_1c = local_1c + local_160;
            uVar2 = FUN_004a7160((local_24 * 0x3890 + 0x4d7e00),0x2cb0,0x1,local_28);
            local_164 = (uVar2 == 0x0);
            local_1c = local_1c + local_164;
        }
    }
    uVar2 = FUN_004a7160(&DAT_004c3fb0,0x2,0x1,local_28);
    local_168 = (uVar2 == 0x0);
    local_1c = local_1c + local_168;
    for (local_24 = 0x0; local_24 < 0x78; local_24 = local_24 + 0x1) {
        if (*(&DAT_005b7cb8 + local_24 * 0x14) != 0x0) {
            local_16c = local_24 * 0x14 & 0xffff0000U | (&DAT_005b7ca8 + local_24 * 0x14);
            uVar2 = FUN_004a7160(&local_16c,0x2,0x1,local_28);
            local_170 = (uVar2 == 0x0);
            local_1c = local_1c + local_170;
            local_16c = local_24 * 0x14 & 0xffff0000U | (&DAT_005b7cac + local_24 * 0x14);
            uVar2 = FUN_004a7160(&local_16c,0x2,0x1,local_28);
            local_174 = (uVar2 == 0x0);
            local_1c = local_1c + local_174;
            local_16c = local_24 * 0x14 & 0xffff0000U | (&DAT_005b7cb0 + local_24 * 0x14);
            uVar2 = FUN_004a7160(&local_16c,0x2,0x1,local_28);
            local_178 = (uVar2 == 0x0);
            local_1c = local_1c + local_178;
            local_16c = local_24 * 0x14 & 0xffff0000U | (&DAT_005b7cb4 + local_24 * 0x14);
            uVar2 = FUN_004a7160(&local_16c,0x2,0x1,local_28);
            local_17c = (uVar2 == 0x0);
            local_1c = local_1c + local_17c;
            uVar2 = FUN_004a7160((&DAT_005b7cb8 + local_24 * 0x14),0x4,0x1,local_28);
            local_180 = (uVar2 == 0x0);
            local_1c = local_1c + local_180;
        }
    }
    uVar2 = FUN_004a7160(&DAT_004c3fb0,0x2,0x1,local_28);
    local_184 = (uVar2 == 0x0);
    local_1c = local_1c + local_184;
    iVar3 = FUN_004875c2(local_28);
    local_1c = local_1c + iVar3;
    for (local_18 = *DAT_005967c8; local_18 != 0x0; local_18 = *local_18) {
        uVar2 = FUN_004a7160(local_18 + 0x2,0x2,0x1,local_28);
        local_188 = (uVar2 == 0x0);
        local_1c = local_1c + local_188;
        uVar2 = FUN_004a7160((local_18 + 0xa),0x2,0x1,local_28);
        local_18c = (uVar2 == 0x0);
        local_1c = local_1c + local_18c;
        uVar2 = FUN_004a7160(local_18 + 0x3,0x2,0x1,local_28);
        local_190 = (uVar2 == 0x0);
        local_1c = local_1c + local_190;
        uVar2 = FUN_004a7160((local_18 + 0xe),0x2,0x1,local_28);
        local_194 = (uVar2 == 0x0);
        local_1c = local_1c + local_194;
        uVar2 = FUN_004a7160(local_18 + 0x4,0x2,0x1,local_28);
        local_198 = (uVar2 == 0x0);
        local_1c = local_1c + local_198;
        uVar2 = FUN_004a7160((local_18 + 0x12),0x2,0x1,local_28);
        local_19c = (uVar2 == 0x0);
        local_1c = local_1c + local_19c;
        uVar2 = FUN_004a7160(local_18 + 0x5,0x2,0x1,local_28);
        local_1a0 = (uVar2 == 0x0);
        local_1c = local_1c + local_1a0;
        uVar2 = FUN_004a7160((local_18 + 0x16),0x2,0x1,local_28);
        local_1a4 = (uVar2 == 0x0);
        local_1c = local_1c + local_1a4;
        uVar2 = FUN_004a7160(local_18 + 0x6,0x2,0x1,local_28);
        local_1a8 = (uVar2 == 0x0);
        local_1c = local_1c + local_1a8;
        uVar2 = FUN_004a7160((local_18 + 0x1a),0x2,0x1,local_28);
        local_1ac = (uVar2 == 0x0);
        local_1c = local_1c + local_1ac;
        uVar2 = FUN_004a7160(local_18 + 0x7,0x2,0x1,local_28);
        local_1b0 = (uVar2 == 0x0);
        local_1c = local_1c + local_1b0;
        uVar2 = FUN_004a7160((local_18 + 0x1e),0x2,0x1,local_28);
        local_1b4 = (uVar2 == 0x0);
        local_1c = local_1c + local_1b4;
        local_14 = (local_18 + 0x2) & 0xffff0000 | (local_18 + 0x8);
        uVar2 = FUN_004a7160(&local_14,0x2,0x1,local_28);
        local_1c4 = (uVar2 == 0x0);
        local_1c = local_1c + local_1c4;
        uVar2 = FUN_004a7160((local_18 + 0x22),0x2,0x1,local_28);
        local_1c8 = (uVar2 == 0x0);
        local_1c = local_1c + local_1c8;
        uVar2 = FUN_004a7160(local_18 + 0x9,0x2,0x1,local_28);
        local_1cc = (uVar2 == 0x0);
        local_1c = local_1c + local_1cc;
        uVar2 = FUN_004a7160((local_18 + 0x26),0x2,0x1,local_28);
        local_1d0 = (uVar2 == 0x0);
        local_1c = local_1c + local_1d0;
        uVar2 = FUN_004a7160(local_18 + 0xa,0x2,0x1,local_28);
        local_1d4 = (uVar2 == 0x0);
        local_1c = local_1c + local_1d4;
        uVar2 = FUN_004a7160((local_18 + 0x2a),0x2,0x1,local_28);
        local_1d8 = (uVar2 == 0x0);
        local_1c = local_1c + local_1d8;
        uVar2 = FUN_004a7160(local_18 + 0xb,0x4,0x1,local_28);
        local_1dc = (uVar2 == 0x0);
        local_1c = local_1c + local_1dc;
    }
    uVar2 = FUN_004a7160(&DAT_004c3fb0,0x2,0x1,local_28);
    local_1c = local_1c + (uVar2 == 0x0);
    FUN_0049ca40(local_28);
    return 0x1;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0047a593(param_1: u32) -> i32

{
let cVar1: u8;
let mut pcVar2: String;
let mut uVar3: u32;
undefined3 extraout_var;
let puVar4: *mut u32;
let puVar5: *mut u32;
u32 auStackY1696 [0x6];
let mut uStackY1672: u32;
let mut iVar6: i32;
let mut in_stack_fffff998: i32;
let mut local_5ec: i32;
let mut local_5e8: u32;
let mut local_5e4: u32;
let mut local_5e0: u32;
let mut local_5dc: u32;
let mut local_5d8: u32;
u32 local_5d4 [0xd];
let mut local_5a0: u32;
let mut local_59c: u32;
let mut local_598: u32;
let mut local_594: u32;
let mut local_590: u32;
let mut local_58c: u32;
let mut local_588: u32;
let mut local_584: u32;
let mut local_580: u32;
let mut local_57c: u32;
let mut local_578: u32;
let mut local_574: u32;
let mut local_570: u32;
let mut local_56c: u32;
let mut local_568: u32;
let mut local_564: u32;
let mut local_560: u32;
let mut local_55c: u32;
let mut local_558: u32;
let mut local_554: u32;
let mut local_550: u32;
let mut local_54c: u32;
let mut local_548: u32;
let mut local_544: u32;
let mut local_540: u32;
let local_53c: i16;
let mut local_538: u32;
let mut local_534: u32;
let mut local_530: u32;
let mut local_52c: i32;
let mut local_528: i32;
let mut local_524: u32;
let mut local_520: i32;
let mut local_51c: i32;
let mut local_518: u32;
let mut local_514: u32;
let mut local_510: u32;
let mut local_50c: u32;
let mut local_508: u32;
let mut local_504: u32;
let mut local_500: u32;
let mut local_4fc: u32;
let mut local_4f8: u32;
let mut local_4f4: u32;
let mut local_4f0: u32;
let mut local_4ec: u32;
let mut local_4e8: u32;
let mut local_4e4: u32;
let mut local_4e0: u32;
let mut local_4dc: u32;
let mut local_4d8: u32;
let mut local_4d4: u32;
let mut local_4d0: u32;
let mut local_4cc: u32;
let mut local_4c8: u32;
let mut local_4c4: u32;
let mut local_4c0: u32;
let mut local_4bc: u32;
let mut local_4b8: u32;
let mut local_4b4: u32;
let mut local_4b0: u32;
let mut local_4ac: u32;
let mut local_4a8: u32;
let mut local_4a4: u32;
let mut local_4a0: u32;
let mut local_49c: u32;
let mut local_498: u32;
let mut local_494: u32;
let mut local_490: u32;
let mut local_48c: u32;
let mut local_488: u32;
let mut local_484: u32;
let mut local_480: u32;
let mut local_47c: u32;
let mut local_478: u32;
let mut local_474: u32;
let mut local_470: u32;
let mut local_46c: u32;
let mut local_468: u32;
let mut local_464: u32;
let mut local_460: u32;
let mut local_45c: u32;
let uStack1112: u8;
let mut local_1ac: u32;
let uStack424: u8;
let mut local_94: u32;
UINT local_90;
let mut local_8c: u32;
let mut local_88: u32;
let mut local_84: u32;
let mut local_80: u32;
let mut local_7c: u32;
let mut local_78: u32;
let local_74: u8 [0x4];
let local_70: u8 [0x4];
let local_6c: u8 [0x4];
let local_68: u8 [0x4];
let local_64: u8 [0x4];
let local_60: u8 [0x6];
let local_5a: u8 [0x2];
let local_58: u8 [0x4];
let local_54: u8 [0x4];
let mut local_50: u32;
u32 local_4c [0x4];
let local_3c: *mut *mut char;
byte *local_38;
let mut local_34: i32;
let mut local_30: i32;
let mut local_2c: i32;
let local_28: u32;
let local_24: u32;
let mut local_20: i32;
byte *local_1c;
let mut local_18: u32;
let local_14: i16;

local_1c = FUN_0049c2c9(0x104);
local_38 = FUN_0049c2c9(0x104);
local_2c = 0x0;
pcVar2 = FUN_0049c7b5(s_sav__s_sav_004c250e);
FUN_0049c2e0(local_38,pcVar2);
pcVar2 = FUN_0049c7b5(s_sav__s_tmp_004c2519);
FUN_0049c2e0(local_1c,pcVar2);
FUN_00453b2a(local_38,local_1c);
local_3c = FUN_0049c4bd(local_1c,&DAT_004c2524);
if (local_3c == 0x0) {
pcVar2 = FUN_00499050(DAT_0059679c,0x7394);
pop_err_msg_box_and_exit_004a02f5(pcVar2);
}
uVar3 = FUN_004a7970(&local_30,0x4,0x1,local_3c);
local_78 = (uVar3 == 0x0);
local_2c = local_2c + local_78;
uVar3 = FUN_004a7970(&DAT_00591cb0,0x4,0x1,local_3c);
local_7c = (uVar3 == 0x0);
local_2c = local_2c + local_7c;
uVar3 = FUN_004a7970(&DAT_004d559c,0x4,0x1,local_3c);
local_80 = (uVar3 == 0x0);
local_2c = local_2c + local_80;
if (((byte)DAT_004d559c & 0x8) != 0x0) {
local_30 = local_30 + 0xea600;
}
if ((local_30 != 0xecf71) && (local_30 < 0xea995)) {
pcVar2 = FUN_00499050(DAT_0059679c,0x7397);
pop_err_msg_box_and_exit_004a02f5(pcVar2);
}
local_28 = FUN_0047d5ab(local_3c);
FUN_00472275();
iVar6 = 0x64;
pcVar2 = FUN_00499050(DAT_0059679c,0x735b);
FUN_004a2ff0(0x0,pcVar2,iVar6,in_stack_fffff998);
if (0xea9e8 < local_30) {
uVar3 = FUN_004a7970(&DAT_004c9740,0x4,0x1,local_3c);
local_84 = (uVar3 == 0x0);
local_2c = local_2c + local_84;
}
for (local_34 = 0x0; local_34 < 0xb; local_34 = local_34 + 0x1) {
cVar1 = FUN_00460210(&DAT_004d7730 + local_34 * 0x67,local_3c);
local_2c = local_2c + CONCAT31(extraout_var,cVar1);
}
for (local_34 = 0x0; local_34 < 0xe; local_34 = local_34 + 0x1) {
iVar6 = FUN_004611cf((&DAT_00568210 + local_34 * 0x1e22),local_3c,local_30);
local_2c = local_2c + iVar6;
}
if (local_30 < 0xeaa60) {
uVar3 = FUN_004a7970(&local_45c,0x3d8,0x1,local_3c);
local_460 = (uVar3 == 0x0);
local_2c = local_2c + local_460;
_DAT_005827f0 = local_45c;
DAT_005827f4._0_1_ = uStack1112;
DAT_005827f8 = local_1ac;
DAT_005827fc = uStack424;
_DAT_00582910 = local_94;
excommunicate_empire_expiration_00582914 = local_90;
excommunicate_turns_00582918 = local_8c;
excommunicate_loyalty_hit_0058291c = local_88;
sign_holy_writ_00582920 = 0x32;
}
else {
uVar3 = FUN_004a7970(&DAT_005827f0,0x134,0x1,local_3c);
local_464 = (uVar3 == 0x0);
local_2c = local_2c + local_464;
}
if (0xeaa5a < local_30) {
iVar6 = FUN_00462ae2(&DAT_004d55a8,local_3c);
local_2c = local_2c + iVar6;
}
uVar3 = FUN_004a7970(&DAT_00591cac,0x4,0x1,local_3c);
local_468 = (uVar3 == 0x0);
local_2c = local_2c + local_468;
uVar3 = FUN_004a7970(&DAT_00591cb0,0x4,0x1,local_3c);
local_46c = (uVar3 == 0x0);
local_2c = local_2c + local_46c;
uVar3 = FUN_004a7970(&DAT_00591cb4,0x4,0x1,local_3c);
local_470 = (uVar3 == 0x0);
local_2c = local_2c + local_470;
uVar3 = FUN_004a7970(&DAT_004c9754,0x4,0x1,local_3c);
local_474 = (uVar3 == 0x0);
local_2c = local_2c + local_474;
if (0xea9e9 < local_30) {
uVar3 = FUN_004a7970(&DAT_004c9760,0x4,0x1,local_3c);
local_478 = (uVar3 == 0x0);
local_2c = local_2c + local_478;
}
if (local_30 < 0xeaa01) {
_DAT_004c9768 = 0x493e0;
}
else {
uVar3 = FUN_004a7970(&DAT_004c9764,0x4,0x1,local_3c);
local_47c = (uVar3 == 0x0);
local_2c = local_2c + local_47c;
uVar3 = FUN_004a7970(&DAT_004c9768,0x4,0x1,local_3c);
local_480 = (uVar3 == 0x0);
local_2c = local_2c + local_480;
}
uVar3 = FUN_004a7970(&DAT_004c9778,0xbde0,0x1,local_3c);
local_484 = (uVar3 == 0x0);
local_2c = local_2c + local_484;
uVar3 = FUN_004a7970(&DAT_004d5560,0x4,0x1,local_3c);
local_488 = (uVar3 == 0x0);
local_2c = local_2c + local_488;
uVar3 = FUN_004a7970(&DAT_004d5564,0x4,0x1,local_3c);
local_48c = (uVar3 == 0x0);
local_2c = local_2c + local_48c;
uVar3 = FUN_004a7970(&DAT_004d557c,0x4,0x1,local_3c);
local_490 = (uVar3 == 0x0);
local_2c = local_2c + local_490;
uVar3 = FUN_004a7970(&DAT_004d5580,0x4,0x1,local_3c);
local_494 = (uVar3 == 0x0);
local_2c = local_2c + local_494;
uVar3 = FUN_004a7970(&DAT_004d5584,0x4,0x1,local_3c);
local_498 = (uVar3 == 0x0);
local_2c = local_2c + local_498;
uVar3 = FUN_004a7970(&DAT_004d5588,0x4,0x1,local_3c);
local_49c = (uVar3 == 0x0);
local_2c = local_2c + local_49c;
if (0xea9f5 < local_30) {
uVar3 = FUN_004a7970(&DAT_004d558c,0x4,0x1,local_3c);
local_4a0 = (uVar3 == 0x0);
local_2c = local_2c + local_4a0;
}
if (0xeaa5a < local_30) {
uVar3 = FUN_004a7970(&DAT_004d5590,0x4,0x1,local_3c);
local_4a4 = (uVar3 == 0x0);
local_2c = local_2c + local_4a4;
}
if (0xeaa5f < local_30) {
uVar3 = FUN_004a7970(&DAT_004d5594,0x4,0x1,local_3c);
local_4a8 = (uVar3 == 0x0);
local_2c = local_2c + local_4a8;
}
uVar3 = FUN_004a7970(&DAT_00595738,0x4,0x1,local_3c);
local_4ac = (uVar3 == 0x0);
local_2c = local_2c + local_4ac;
uVar3 = FUN_004a7970(&DAT_004d5a58,0x1f4,0x1,local_3c);
local_4b0 = (uVar3 == 0x0);
local_2c = local_2c + local_4b0;
uVar3 = FUN_004a7970(&DAT_004d5c50,0x1f4,0x1,local_3c);
local_4b4 = (uVar3 == 0x0);
local_2c = local_2c + local_4b4;
uVar3 = FUN_004a7970(&DAT_004d566c,0x3e8,0x1,local_3c);
local_4b8 = (uVar3 == 0x0);
local_2c = local_2c + local_4b8;
local_24 = FUN_004aa690(local_3c);
local_20 = (local_24 * 0x64) / local_28;
FUN_004a36b0(local_20);
uVar3 = FUN_004a7970(&DAT_004c976c,0x4,0x1,local_3c);
local_4bc = (uVar3 == 0x0);
local_2c = local_2c + local_4bc;
if (local_30 < 0xeaa02) {
DAT_004c9770 = 0xffffffff;
_DAT_004c9774 = 0x0;
}
else {
uVar3 = FUN_004a7970(&DAT_004c9770,0x4,0x1,local_3c);
local_4c0 = (uVar3 == 0x0);
local_2c = local_2c + local_4c0;
uVar3 = FUN_004a7970(&DAT_004c9774,0x4,0x1,local_3c);
local_4c4 = (uVar3 == 0x0);
local_2c = local_2c + local_4c4;
}
uVar3 = FUN_004a7970(&DAT_004c5048,0x18,0x1,local_3c);
local_4c8 = (uVar3 == 0x0);
local_2c = local_2c + local_4c8;
if (0xeaa04 < local_30) {
uVar3 = FUN_004a7970(&DAT_004d5568,0x14,0x1,local_3c);
local_4cc = (uVar3 == 0x0);
local_2c = local_2c + local_4cc;
}
uVar3 = FUN_004a7970(&DAT_00596a44,0x4,0x1,local_3c);
local_4d0 = (uVar3 == 0x0);
local_2c = local_2c + local_4d0;
uVar3 = FUN_004a7970(&DAT_00596a50,0x4,0x1,local_3c);
local_4d4 = (uVar3 == 0x0);
local_2c = local_2c + local_4d4;
FUN_004a0430(&DAT_004c6168,0x0,0xfa0);
if (0xeaa03 < local_30) {
uVar3 = FUN_004a7970(&DAT_004c6160,0x4,0x1,local_3c);
local_4d8 = (uVar3 == 0x0);
local_2c = local_2c + local_4d8;
for (local_34 = 0x0; local_34 < _DAT_004c6160; local_34 = local_34 + 0x1) {
uVar3 = FUN_004a7970((&DAT_004c6168 + local_34 * 0x4),0x4,0x1,local_3c);
local_4dc = (uVar3 == 0x0);
local_2c = local_2c + local_4dc;
}
}
uVar3 = FUN_004a7970(&DAT_005b4ac0,0x2580,0x1,local_3c);
local_4e0 = (uVar3 == 0x0);
local_2c = local_2c + local_4e0;
local_34 = 0x0;
uVar3 = FUN_004a7970(&local_18,0x2,0x1,local_3c);
local_4e4 = (uVar3 == 0x0);
local_2c = local_2c + local_4e4;
while (local_18 != -0x2) {
(&DAT_005b7078 + local_34 * 0x4e) = local_18;
uVar3 = FUN_004a7970((&DAT_005b707a + local_34 * 0x4e),0x2,0x1,local_3c);
local_4e8 = (uVar3 == 0x0);
local_2c = local_2c + local_4e8;
uVar3 = FUN_004a7970((&DAT_005b707c + local_34 * 0x4e),0x2,0x1,local_3c);
local_4ec = (uVar3 == 0x0);
local_2c = local_2c + local_4ec;
uVar3 = FUN_004a7970((&DAT_005b707e + local_34 * 0x4e),0x20,0x1,local_3c);
local_4f0 = (uVar3 == 0x0);
local_2c = local_2c + local_4f0;
uVar3 = FUN_004a7970((&DAT_005b709e + local_34 * 0x4e),0x20,0x1,local_3c);
local_4f4 = (uVar3 == 0x0);
local_2c = local_2c + local_4f4;
uVar3 = FUN_004a7970((local_34 * 0x4e + 0x5b70be),0x2,0x1,local_3c);
local_4f8 = (uVar3 == 0x0);
local_2c = local_2c + local_4f8;
uVar3 = FUN_004a7970((local_34 * 0x4e + 0x5b70c0),0x2,0x1,local_3c);
local_4fc = (uVar3 == 0x0);
local_2c = local_2c + local_4fc;
uVar3 = FUN_004a7970((&DAT_005b70c2 + local_34 * 0x4e),0x4,0x1,local_3c);
local_500 = (uVar3 == 0x0);
local_2c = local_2c + local_500;
uVar3 = FUN_004a7970((&DAT_004daab0 + local_34 * 0x3890),0x4,0x1,local_3c);
local_504 = (uVar3 == 0x0);
local_2c = local_2c + local_504;
if (local_30 < 0xeaa00) {
uVar3 = FUN_004a7970(&local_508,0x4,0x1,local_3c);
local_50c = (uVar3 == 0x0);
local_2c = local_2c + local_50c;
uVar3 = FUN_004a7970(&local_508,0x4,0x1,local_3c);
local_510 = (uVar3 == 0x0);
local_2c = local_2c + local_510;
uVar3 = FUN_004a7970(&local_508,0x4,0x1,local_3c);
local_514 = (uVar3 == 0x0);
local_2c = local_2c + local_514;
}
uVar3 = FUN_004a7970((local_34 * 0x3890 + 0x4d7e00),0x2cb0,0x1,local_3c);
local_518 = (uVar3 == 0x0);
local_2c = local_2c + local_518;
local_52c = *(&DAT_005b7078 + local_34 * 0x4e) >> 0x10;
local_528 = *(&DAT_005b7076 + local_34 * 0x4e) >> 0x10;
local_524 = param_1;
local_530 = 0x2;
*(&DAT_005b4ac0 + local_52c * 0x4 + local_528 * 0xc0) = 0x2;
local_34 = local_34 + 0x1;
local_520 = local_528;
local_51c = local_52c;
uVar3 = FUN_004a7970(&local_18,0x2,0x1,local_3c);
local_534 = (uVar3 == 0x0);
local_2c = local_2c + local_534;
local_24 = FUN_004aa690(local_3c);
local_20 = (local_24 * 0x64) / local_28;
FUN_004a36b0(local_20);
}
local_34 = 0x0;
uVar3 = FUN_004a7970(&local_18,0x2,0x1,local_3c);
local_538 = (uVar3 == 0x0);
local_2c = local_2c + local_538;
while (local_18 != -0x2) {
*(&DAT_005b7ca8 + local_34 * 0x14) = local_18;
uVar3 = FUN_004a7970(&local_53c,0x2,0x1,local_3c);
local_540 = (uVar3 == 0x0);
local_2c = local_2c + local_540;
*(&DAT_005b7cac + local_34 * 0x14) = local_53c;
uVar3 = FUN_004a7970(&local_53c,0x2,0x1,local_3c);
local_544 = (uVar3 == 0x0);
local_2c = local_2c + local_544;
*(&DAT_005b7cb0 + local_34 * 0x14) = local_53c;
uVar3 = FUN_004a7970(&local_53c,0x2,0x1,local_3c);
local_548 = (uVar3 == 0x0);
local_2c = local_2c + local_548;
*(&DAT_005b7cb4 + local_34 * 0x14) = local_53c;
uVar3 = FUN_004a7970((&DAT_005b7cb8 + local_34 * 0x14),0x4,0x1,local_3c);
local_54c = (uVar3 == 0x0);
local_2c = local_2c + local_54c;
local_34 = local_34 + 0x1;
uVar3 = FUN_004a7970(&local_18,0x2,0x1,local_3c);
local_550 = (uVar3 == 0x0);
local_2c = local_2c + local_550;
local_24 = FUN_004aa690(local_3c);
local_20 = (local_24 * 0x64) / local_28;
FUN_004a36b0(local_20);
}
iVar6 = FUN_004574d6(local_3c);
local_2c = local_2c + iVar6;
local_24 = FUN_004aa690(local_3c);
local_20 = (local_24 * 0x64) / local_28;
FUN_004a36b0(local_20);
iVar6 = FUN_00487685(local_3c,0x1,local_30,-0x1);
local_2c = local_2c + iVar6;
local_34 = 0x0;
uVar3 = FUN_004a7970(&local_18,0x2,0x1,local_3c);
local_554 = (uVar3 == 0x0);
local_2c = local_2c + local_554;
while( true ) {
if (local_18 == -0x2) break;
local_74._0_2_ = local_18;
uVar3 = FUN_004a7970((local_74 + 0x2),0x2,0x1,local_3c);
local_558 = (uVar3 == 0x0);
local_2c = local_2c + local_558;
uVar3 = FUN_004a7970(local_70,0x2,0x1,local_3c);
local_55c = (uVar3 == 0x0);
local_2c = local_2c + local_55c;
uVar3 = FUN_004a7970((local_70 + 0x2),0x2,0x1,local_3c);
local_560 = (uVar3 == 0x0);
local_2c = local_2c + local_560;
uVar3 = FUN_004a7970(local_6c,0x2,0x1,local_3c);
local_564 = (uVar3 == 0x0);
local_2c = local_2c + local_564;
uVar3 = FUN_004a7970((local_6c + 0x2),0x2,0x1,local_3c);
local_568 = (uVar3 == 0x0);
local_2c = local_2c + local_568;
uVar3 = FUN_004a7970(local_68,0x2,0x1,local_3c);
local_56c = (uVar3 == 0x0);
local_2c = local_2c + local_56c;
uVar3 = FUN_004a7970((local_68 + 0x2),0x2,0x1,local_3c);
local_570 = (uVar3 == 0x0);
local_2c = local_2c + local_570;
uVar3 = FUN_004a7970(local_64,0x2,0x1,local_3c);
local_574 = (uVar3 == 0x0);
local_2c = local_2c + local_574;
uVar3 = FUN_004a7970((local_64 + 0x2),0x2,0x1,local_3c);
local_578 = (uVar3 == 0x0);
local_2c = local_2c + local_578;
uVar3 = FUN_004a7970(local_60,0x2,0x1,local_3c);
local_57c = (uVar3 == 0x0);
local_2c = local_2c + local_57c;
uVar3 = FUN_004a7970((local_60 + 0x2),0x2,0x1,local_3c);
local_580 = (uVar3 == 0x0);
local_2c = local_2c + local_580;
if (local_30 < 0xea9ee) {
FUN_00482992(local_74,0x4b);
}
else {
uVar3 = FUN_004a7970(&local_14,0x2,0x1,local_3c);
local_584 = (uVar3 == 0x0);
local_2c = local_2c + local_584;
FUN_00482992(local_74,local_14);
}
uVar3 = FUN_004a7970(local_5a,0x2,0x1,local_3c);
local_588 = (uVar3 == 0x0);
local_2c = local_2c + local_588;
uVar3 = FUN_004a7970(local_58,0x2,0x1,local_3c);
local_58c = (uVar3 == 0x0);
local_2c = local_2c + local_58c;
if (local_30 < 0xeaa65) {
local_58 = local_58 & 0xffff | local_58 << 0x10;
}
else {
uVar3 = FUN_004a7970((local_58 + 0x2),0x2,0x1,local_3c);
local_590 = (uVar3 == 0x0);
local_2c = local_2c + local_590;
}
uVar3 = FUN_004a7970(local_54,0x2,0x1,local_3c);
local_594 = (uVar3 == 0x0);
local_2c = local_2c + local_594;
uVar3 = FUN_004a7970((local_54 + 0x2),0x2,0x1,local_3c);
local_598 = (uVar3 == 0x0);
local_2c = local_2c + local_598;
uVar3 = FUN_004a7970(&local_50,0x4,0x1,local_3c);
local_59c = (uVar3 == 0x0);
local_2c = local_2c + local_59c;
uVar3 = FUN_004a7970(local_4c,0x2,0x1,local_3c);
if (uVar3 != 0x0) {
local_5a0 = 0x0;
}
else {
local_5a0 = 0x1;
}
local_5a0 = (uVar3 == 0x0);
local_2c = local_2c + local_5a0;
if ((local_6c._2_2_ < 0xa) || (0xc < local_6c._2_2_)) {
local_6c = local_6c & 0xffff | local_6c << 0x10;
local_50 = local_50 & 0xfffffeff;
}
puVar4 = local_74;
puVar5 = local_5d4;
for (iVar6 = 0xc; iVar6 != 0x0; iVar6 = iVar6 + -0x1) {
*puVar5 = *puVar4;
puVar4 = puVar4 + 0x1;
puVar5 = puVar5 + 0x1;
}
*puVar5 = *puVar4;
puVar4 = local_5d4;
puVar5 = auStackY1696;
for (iVar6 = 0xc; iVar6 != 0x0; iVar6 = iVar6 + -0x1) {
*puVar5 = *puVar4;
puVar4 = puVar4 + 0x1;
puVar5 = puVar5 + 0x1;
}
*puVar5 = *puVar4;
FUN_004813ca();
local_34 = local_34 + 0x1;
uVar3 = FUN_004a7970(&local_18,0x2,0x1,local_3c);
local_5d8 = (uVar3 == 0x0);
local_2c = local_2c + local_5d8;
local_24 = FUN_004aa690(local_3c);
local_20 = (local_24 * 0x64) / local_28;
FUN_004a36b0(local_20);
}
if ((local_2c == 0x0) && (((byte)DAT_004d559c & 0x8) != 0x0)) {
uVar3 = FUN_004a7970(&DAT_005b2e26,0x2,0x5,local_3c);
local_5dc = (uVar3 == 0x0);
local_2c = local_2c + local_5dc;
uVar3 = FUN_004a7970(&DAT_005b2e1c,0x2,0x5,local_3c);
local_5e0 = (uVar3 == 0x0);
local_2c = local_2c + local_5e0;
uVar3 = FUN_004a7970(&local_5e4,0x2,0x1,local_3c);
local_5e8 = (uVar3 == 0x0);
local_2c = local_2c + local_5e8;
if (local_2c == 0x0) {
if (local_5e4 != DAT_005b2e30) {
pcVar2 = FUN_00499050(DAT_0059679c,0x7424);
FUN_0049c2e0(&stack0xfffff994,pcVar2);
uStackY1672 = 0x47bed3;
FUN_0045518a(0x1f,0xffffffff,0x74cc,0xffffffff,&stack0xfffff994,0xffffffff,0x1);
pcVar2 = FUN_00499050(DAT_0059679c,0x7425);
FUN_0049d2e0(0x0,0x1,pcVar2);
}
}
else {
local_5ec = DAT_004c9754;
loop {
local_5ec = local_5ec + -0x1;
if (local_5ec < 0x0) {
local_5ec = 0x4;
}
} while ((((&DAT_00569a98)[local_5ec * 0x1e22] & 0x1) != 0x0) ||
(((&DAT_00569a98)[local_5ec * 0x1e22] & 0x2) != 0x0));
pcVar2 = FUN_00499050(DAT_0059679c,0x7426);
FUN_0049c2e0(&stack0xfffff994,pcVar2);
uStackY1672 = 0x47be4d;
FUN_0045518a(0x1f,0xffffffff,0x74cc,0xffffffff,&stack0xfffff994,0xffffffff,0x1);
local_2c = 0x0;
}
}
FUN_004a36b0(0x64);
FUN_004a3800();
FUN_00482491();
FUN_0049ca40(local_3c);
FUN_004a5420(local_1c);
FUN_0049af50(local_1c);
FUN_0049af50(local_38);
return local_2c;
}

