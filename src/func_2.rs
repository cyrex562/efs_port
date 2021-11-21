
fn FUN_00402fd5(param_1: i32,param_2: i32,param_3: u8,param_4: u8) -> u32

{
    let mut uVar1: u32;
    let mut iVar2: i32;
    let mut pcVar3: String;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let local_228: u8 [0x100];
    let mut local_128: [u320x7];
    let mut local_10b: u32;
    let mut local_107: i32;
    let mut local_103: u32;
    let mut local_ff: i32;
    let ppuStack227: *mut *mut u8;;
    let mut local_37: String;;
    let mut local_30: i32;
    let mut local_2c: i32;
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_20: String;
    let mut local_1c: i32;
    let mut local_18: i32;

    local_24 = 0x0;
    local_20 = *(&DAT_00582938 + param_4 * 0x4 + param_3 * 0x18);
    _DAT_004c502c = param_2;
    DAT_004c5035._3_1_ = param_3;
    DAT_004c5039 = param_4;
    _DAT_004c5024 = param_1;
    if (param_1 == -0x1) {
        iVar2 = FUN_00430d15(&local_20);
        if (iVar2 == 0x0) {
            FUN_00499050(DAT_0059679c,0x7132);
        }
        else {
            FUN_00499050(DAT_0059679c,0x7133);
        }
        pcVar3 = FUN_00499050(DAT_0059679c,0x73c2);
        FUN_0049c2e0(local_228,pcVar3);
        FUN_0049d2e0(0x0,0x1,local_228);
        local_28 = 0x0;
    }
    else {
        if (param_1 == 0x0) {
            iVar2 = FUN_00430d15(&local_20);
            if (iVar2 == 0x0) {
                FUN_00499050(DAT_0059679c,0x7132);
            }
            else {
                FUN_00499050(DAT_0059679c,0x7133);
            }
            pcVar3 = FUN_00499050(DAT_0059679c,0x73c3);
            FUN_0049c2e0(local_228,pcVar3);
            FUN_0049d2e0(0x0,0x1,local_228);
            local_28 = 0x0;
        }
        else {
            DAT_004c5030 = 0x0;
            _DAT_004c5028 = 0x0;
            local_1c = 0x0;
            // TODO
            // for (local_30 = 0x0; local_30 < 0xd; local_30 = local_30 + 0x1) {
            //   iVar2 = *(&DAT_00582938 + param_4 * 0x4 + param_3 * 0x18);
            //   iVar4 = local_30 * 0x4;
            //   iVar5 = FUN_00462571((&DAT_00568210 + DAT_004c9754 * 0x1e22),param_2,local_30);
            //   *(&DAT_005b2e50 + local_30 * 0x4) = iVar5 - *(iVar2 + iVar4 + 0xb9);
            //   if (*(&DAT_005b2e50 + local_30 * 0x4) < 0x0) {
            //     local_1c = local_1c + 0x1;
            //     *(&DAT_005b2e50 + local_30 * 0x4) = -*(&DAT_005b2e50 + local_30 * 0x4);
            //     uVar1 = DAT_004c9760;
            //     DAT_004c9760 = 0x0;
            //     iVar4 = local_30 * 0x4;
            //     iVar2 = FUN_00462571(&DAT_005718ba,param_2,local_30);
            //     if (iVar2 < *(&DAT_005b2e50 + iVar4)) {
            //       _DAT_004c5028 = 0x1;
            //     }
            //     iVar2 = DAT_004c9754 * 0x1e22;
            //     iVar5 = local_30 * 0x8;
            //     DAT_004c9760 = uVar1;
            //     iVar4 = FUN_0044e442();
            //     local_18 = (*(&DAT_00569bcc + iVar5 + iVar2) * iVar4) / 0x64;
            //     DAT_004c5030 = DAT_004c5030 +
            //                    (*(&DAT_00569bcc + DAT_004c9754 * 0x1e22 + local_30 * 0x8) + local_18) *
            //                    *(&DAT_005b2e50 + local_30 * 0x4);
            //   }
            //   else {
            //     *(&DAT_005b2e50 + local_30 * 0x4) = 0x0;
            //   }
            // }
            FUN_004990e0(local_128,0x0,s_efs_res_004c0644,s_AutoBuy_004c063c);
            local_2c = ((local_1c + 0x1) / 0x2) * 0x20;
            FUN_0049a770(local_128,0x416,local_103,local_ff + local_2c);
            FUN_0049a770(local_128,0x415,local_10b,local_107 - local_2c / 0x2);
            FUN_0049bf80(local_128,0x64,0x415,0x17c,local_107 + local_ff + -0x19);
            FUN_0049bf80(local_128,0x66,0x415,0xa0,local_107 + local_ff + -0x19);
            FUN_0049bf80(local_128,0x67,0x415,0x17c,local_107 + local_ff + -0x19);
            if (_DAT_004c5028 == 0x0) {
                if (*(&DAT_00569abd + DAT_004c9754 * 0x1e22) < DAT_004c5030) {
                    FUN_0049bf80(local_128,0x66,0x414,0x0,0x0);
                    FUN_0049bf80(local_128,0x66,0x410,0x0,0x0);
                    FUN_0049bf80(local_128,0x67,0x414,0x0,0x0);
                    FUN_0049bf80(local_128,0x67,0x410,0x0,0x0);
                }
                else {
                    FUN_0049bf80(local_128,0x64,0x414,0x0,0x0);
                    FUN_0049bf80(local_128,0x64,0x410,0x0,0x0);
                }
            }
            else {
                FUN_0049bf80(local_128,0x66,0x414,0x0,0x0);
                FUN_0049bf80(local_128,0x66,0x410,0x0,0x0);
                FUN_0049bf80(local_128,0x67,0x414,0x0,0x0);
                FUN_0049bf80(local_128,0x67,0x410,0x0,0x0);
            }
            local_24 = FUN_0049bb50(local_128,FUN_004036ec);
            if (local_24 != 0x0) {
                *(&DAT_00569abd + DAT_004c9754 * 0x1e22) = *(&DAT_00569abd + DAT_004c9754 * 0x1e22) - DAT_004c5030
                ;
                // TODO:
                // for (local_30 = 0x0; local_30 < 0xd; local_30 = local_30 + 0x1) {
                //   if (*(&DAT_005b2e50 + local_30 * 0x4) != 0x0) {
                //     FUN_00465f06(*(DAT_0059a9f4 + 0x6) >> 0x10,*(DAT_0059a9f4 + 0x8) >> 0x10,
                //                  *(DAT_0059a9f4 + 0xa) >> 0x10,local_30,*(&DAT_005b2e50 + local_30 * 0x4),
                //                  DAT_004c9754,DAT_0059a9f4);
                //     FUN_00465d24(*(DAT_0059a9f4 + 0x6) >> 0x10,local_30,*(&DAT_005b2e50 + local_30 * 0x4),0x5);
                //     FUN_004626e0();
                //   }
                // }
            }
            local_28 = local_24;
            ppuStack227 = &PTR_FUN_004c3d34;
            if (local_37 != 0x0) {
                FUN_00499b30(local_128,local_37);
            }
            FUN_0049a1c0(local_128,0x1);
        }
    }
    return local_28;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_004036ec(param_1: i32,param_2: u32,param_3: i32) -> u32

{
    let mut iVar1: i32;
    let mut pcVar2: String;
    let mut iVar3: i32;
    let mut uVar4: u32;
    let mut iVar5: i32;
    let mut uVar6: u32;
    let puVar7: *mut u32;
    let bVar8: u8;
    let local_f0: [u8;0x40];
    let local_b0: [u8;0x80];
    let mut local_30: String;
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: String;
    let mut local_1c: String;
    let mut local_14: u32;

    local_14 = param_2;
    if (0x404 < param_2) {
        if (param_2 < 0x406) {
            FUN_004953d7();
            local_30 = *(&DAT_00582938 + DAT_004c5039 * 0x4 + (DAT_004c5035 >> 0x18) * 0x18);
            local_1c = local_30;
            iVar1 = FUN_00430d15(&local_30);
            if (iVar1 == 0x0) {
                local_20 = FUN_00499050(DAT_0059679c,0x7132);
            }
            else {
                local_20 = FUN_00499050(DAT_0059679c,0x7133);
            }
            pcVar2 = FUN_00499050(DAT_0059679c,0x73c5);
            FUN_0049c2e0(local_b0,pcVar2);
            bVar8 = 0x11;
            uVar6 = 0xcaccce;
            iVar5 = -0x1;
            uVar4 = 0xcaccce;
            iVar1 = 0x15e;
            puVar7 = LPCSTR_005b9218;
            pcVar2 = FUN_00499050(DAT_0059679c,0x73c4);
            FUN_00497567(0x140,*(param_1 + 0x21) + 0xf,pcVar2,iVar1,uVar4,iVar5,uVar6,puVar7,bVar8);
            FUN_00497567(0x140,*(param_1 + 0x21) + 0x23,local_b0,0x15e,0xcaccce,-0x1,0xcaccce,
                         LPCSTR_005b9218,0x11);
            pcVar2 = FUN_00499050(DAT_0059679c,0x73c9);
            FUN_0049c2e0(local_b0,pcVar2);
            FUN_00497567(0x140,*(param_1 + 0x21) + 0x41,local_b0,0x15e,0xcaccce,-0x1,0xcaccce,
                         LPCSTR_005b9218,0x11);
            local_2c = 0x0;
            // TODO:
            // for (local_24 = 0x0; local_24 < 0xd; local_24 = local_24 + 0x1) {
            //   if (*(&DAT_005b2e50 + local_24 * 0x4) != 0x0) {
            //     iVar3 = DAT_004c9754 * 0x1e22;
            //     iVar1 = local_24 * 0x8;
            //     iVar5 = FUN_0044e442();
            //     local_28 = (*(&DAT_00569bcc + iVar3 + iVar1) * iVar5) / 0x64;
            //     pcVar2 = FUN_00499050(DAT_0059679c,0x73e9);
            //     FUN_0049c2e0(local_b0,pcVar2);
            //     pcVar2 = FUN_00499050(DAT_0059679c,0x73ea);
            //     FUN_0049c2e0(local_f0,pcVar2);
            //     if ((local_2c + 0x1U & 0x1) == 0x0) {
            //       FUN_00496ee6(&DAT_00596a58 + local_24 * 0x3da,0x14a,
            //                    (local_2c / 0x2) * 0x20 + *(param_1 + 0x21) + 0x64,0x22,0x1d);
            //       FUN_00497567(0x17c,(local_2c / 0x2) * 0x20 + *(param_1 + 0x21) + 0x69,local_b0,0xc8,
            //                    &DAT_00535557,-0x1,&DAT_00535557,LPCSTR_005b9218,0x10);
            //       FUN_00497567(0x17c,(local_2c / 0x2) * 0x20 + *(param_1 + 0x21) + 0x75,local_f0,0xc8,
            //                    &DAT_00535557,-0x1,&DAT_00535557,LPCSTR_005b9218,0x10);
            //     }
            //     else {
            //       FUN_00496ee6(&DAT_00596a58 + local_24 * 0x3da,*(param_1 + 0x1d) + 0xa,
            //                    (local_2c / 0x2) * 0x20 + *(param_1 + 0x21) + 0x64,0x22,0x1d);
            //       FUN_00497567(*(param_1 + 0x1d) + 0x32,(local_2c / 0x2) * 0x20 + *(param_1 + 0x21) + 0x69,
            //                    local_b0,0xc8,&DAT_00535557,-0x1,&DAT_00535557,LPCSTR_005b9218,0x10);
            //       FUN_00497567(*(param_1 + 0x1d) + 0x32,(local_2c / 0x2) * 0x20 + *(param_1 + 0x21) + 0x75,
            //                    local_f0,0xc8,&DAT_00535557,-0x1,&DAT_00535557,LPCSTR_005b9218,0x10);
            //     }
            //     local_2c = local_2c + 0x1;
            //   }
            // }
            pcVar2 = FUN_00499050(DAT_0059679c,0x73eb);
            FUN_0049c2e0(local_b0,pcVar2);
            FUN_00497567(0x140,*(param_1 + 0x21) + *(param_1 + 0x29) + -0x50,local_b0,0x15e,0xcaccce,-0x1,
                         0xcaccce,LPCSTR_005b9218,0x11);
            if (_DAT_004c5028 == 0x0) {
                if (*(&DAT_00569abd + DAT_004c9754 * 0x1e22) < DAT_004c5030) {
                    pcVar2 = FUN_00499050(DAT_0059679c,0x73c7);
                    FUN_0049c2e0(local_b0,pcVar2);
                }
                else {
                    pcVar2 = FUN_00499050(DAT_0059679c,0x73c8);
                    FUN_0049c2e0(local_b0,pcVar2);
                }
            }
            else {
                pcVar2 = FUN_00499050(DAT_0059679c,0x73c6);
                FUN_0049c2e0(local_b0,pcVar2);
            }
            FUN_00497567(0x140,*(param_1 + 0x21) + *(param_1 + 0x29) + -0x37,local_b0,0x15e,0xcaccce,-0x1,
                         0xcaccce,LPCSTR_005b9218,0x11);
            FUN_0049536f();
            return 0x1;
        }
        if (param_2 == 0x407) {
            if ((param_3 == 0x64) || (param_3 == 0x67)) {
                FUN_0049c140(param_1,0x0);
            }
            else {
                if (param_3 == 0x66) {
                    FUN_0049c140(param_1,0x1);
                }
            }
        }
    }
    return 0x0;
}
fn FUN_00403cdc() -> u32

{
    let mut local_14: i32;

    local_14 = 0x0;
    while( true ) {
        if (0x13 < local_14) {
            return 0x0;
        }
        if ((((&DAT_00595744 + local_14 * 0x5e) & 0x1) != 0x0) &&
            ((*(*(&DAT_00595740 + local_14 * 0x5e) + 0x3b) & 0x80) != 0x0)) break;
        local_14 = local_14 + 0x1;
    }
    return *(&DAT_00595740 + local_14 * 0x5e);
}



fn FUN_00403d5d(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: i32,param_6: i32)

{
    let mut iVar1: i32;
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0xd; local_14 = local_14 + 0x1) {
        iVar1 = *(*(&DAT_00582938 + param_5 * 0x4 + param_4 * 0x18) + local_14 * 0x4 + 0xb9);
        if (iVar1 != 0x0) {
            FUN_00465f06(param_1,param_2,param_3,local_14,iVar1,DAT_004c9754,param_6);
        }
    }
    return;
}



fn FUN_00403dd7(param_1: i32,param_2: i32,param_3: i32) -> u32

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut local_14: i32;

    local_14 = 0x0;
    while( true ) {
        if (0xc < local_14) {
            return 0x1;
        }
        if ((*(*(&DAT_00582938 + param_3 * 0x4 + param_2 * 0x18) + local_14 * 0x4 + 0xb9) != 0x0) &&
            (iVar1 = *(&DAT_00582938 + param_3 * 0x4 + param_2 * 0x18),
             iVar2 = FUN_00462571((&DAT_00568210 + DAT_004c9754 * 0x1e22),param_1,local_14),
             iVar2 < *(local_14 * 0x4 + iVar1 + 0xb9))) break;
        local_14 = local_14 + 0x1;
    }
    return 0x0;
}



fn FUN_00403e94(param_1: &mut String,byte param_2) -> String

{
    let piVar1: *mut i32;;

    if ((param_2 & 0x4) == 0x0) {
        $1: &mut String(param_1 + 0x45) = &PTR_FUN_004c3d34;
        if (*(param_1 + 0xf1) != 0x0) {
            FUN_00499b30(param_1,*(param_1 + 0xf1));
        }
        param_1 = FUN_0049a1c0(param_1,0x1);
        if ((param_2 & 0x2) != 0x0) {
            FUN_0049af50(param_1);
        }
    }
    else {
        piVar1 = FUN_00498dce(param_1,&DAT_004c3d50);
        FUN_00498df5(piVar1);
    }
    return param_1;
}



fn FUN_00403f22() -> String

{
    return PTR_s_DIALOG_004bf93c;
}



fn FUN_00403f41() -> u32

{
    let mut iVar1: i32;
    let mut local_118: *mut u32 [0x11];
    let ppuStack211: *mut *mut u8;;
    let mut local_27: String;;
    let mut local_20: u32;
    let mut local_1c: i32;
    let mut local_18: u32;

    local_20 = 0x0;
    DAT_004c5040 = FUN_0049c2c9(0x39a8);
    DAT_004c5044 = FUN_0049c2c9(0x39a8);
    FUN_0049c60b(s_bin_regent_bin_004c064c,DAT_004c5040,0x39a8,(DAT_004c976c + 0x1) * 0x39a8);
    for (local_1c = 0x0; local_1c < 0x5; local_1c = local_1c + 0x1) {
        if (((&DAT_00569a98)[local_1c * 0x1e22] & 0x1) == 0x0) {
            iVar1 = FUN_00406cc8(local_1c);
            if (iVar1 == 0x0) {
                *(&DAT_004c5060 + local_1c * 0x4) = 0x1;
            }
            else {
                *(&DAT_004c5060 + local_1c * 0x4) = 0x0;
            }
        }
        else {
            *(&DAT_004c5060 + local_1c * 0x4) = 0x1;
        }
    }
    if ((((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x4) != 0x0) &&
        (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x80) == 0x0)) {
        for (local_1c = 0x0; local_1c < 0x3; local_1c = local_1c + 0x1) {
            if (((&DAT_00569ad1)[DAT_004c9754 * 0x1e22 + local_1c] != -0x1) &&
                (*(&DAT_004c5060 + (*(&DAT_00569ace + DAT_004c9754 * 0x1e22 + local_1c) >> 0x18) * 0x4) == 0x0))
            {
                (&DAT_004c5048)[local_1c * 0x2] = *(&DAT_00569ace + DAT_004c9754 * 0x1e22 + local_1c) >> 0x18;
            }
        }
    }
    FUN_004990e0(local_118,0x0,s_diplo_res_004c0663,s_Byz2Dlg_004c065b);
    FUN_0049bb50(local_118,FUN_0040425b);
    if ((((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x4) != 0x0) &&
        (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x80) == 0x0)) {
        local_20 = 0x1;
        for (local_1c = 0x0; local_1c < 0x3; local_1c = local_1c + 0x1) {
            FUN_00405d5a(local_1c);
        }
            (&DAT_00569a98)[DAT_004c9754 * 0x1e22] = (&DAT_00569a98)[DAT_004c9754 * 0x1e22] | 0x80;
    }
    FUN_0049af50(DAT_004c5040);
    FUN_0049af50(DAT_004c5044);
    local_18 = local_20;
    ppuStack211 = &PTR_FUN_004c3d34;
    if (local_27 != 0x0) {
        FUN_00499b30(local_118,local_27);
    }
    FUN_0049a1c0(local_118,0x1);
    return local_18;
}



fn FUN_0040425b(param_1: *mut *mut u32,param_2: u32,param_3: u32,param_4: i32) -> u32

{
    let mut iVar1: i32;
    let mut uVar2: u32;
    let mut puVar3: *mut u8;
    let mut pcVar4: String;
    let mut uVar5: u32;
    let mut bVar6: bool;
    let mut pcVar7: String;
    let mut local_158: i32;
    let mut local_154: i32;
    let mut local_12c: i32;
    let mut local_90: u32;
    let mut local_8c: i32;
    let mut local_6c: i32;
    let mut local_38: i32;
    let mut local_1c: i32;

    if (param_2 < 0x407) {
        if (param_2 < 0x401) {
            if (param_2 == 0x100) {
                if ((param_4 != 0x0) && (((byte)DAT_004d559c & 0x8) == 0x0)) {
                iVar1 = FUN_00406d51(param_4);
                switch(iVar1) {
                case 0x1:
                uVar5 = FUN_00406eab();
                if ((uVar5 != 0x0) &&
                (*(&DAT_00569abd + DAT_004c9754 * 0x1e22) =
                *(&DAT_00569abd + DAT_004c9754 * 0x1e22) + uVar5,
                0xf423f < *(&DAT_00569abd + DAT_004c9754 * 0x1e22))) {
                *(&DAT_00569abd + DAT_004c9754 * 0x1e22) = 0xf423f;
                }
                break;
                case 0x2:
                uVar5 = FUN_00406eab();
                if ((uVar5 != 0x0) && (uVar5 < 0x72)) {
                (&DAT_00569c30)[uVar5 * 0x9 + DAT_004c9754 * 0x1e22] =
                (&DAT_00569c30)[uVar5 * 0x9 + DAT_004c9754 * 0x1e22] | 0x1;
                (&DAT_00569c30)[DAT_004c9754 * 0x1e22 + uVar5 * 0x9] =
                (&DAT_00569c30)[DAT_004c9754 * 0x1e22 + uVar5 * 0x9] & 0xfb;
                FUN_00463f7b();
                }
                break;
                case 0x3:
                if (((((DAT_005967bc != 0x0) && (*(DAT_005967bc + 0x23) >> 0x18 == DAT_004c9754)) &&
                ((*(DAT_005967bc + 0x3a) & 0x1) != 0x0)) &&
                ((uVar5 = FUN_00406eab(), uVar5 != 0x0 && (uVar5 < 0xd)))) &&
                ((uVar2 = FUN_00406eab(), uVar2 != 0x0 && (uVar2 < 0x3e8)))) {
                FUN_00465f06((DAT_005967bc + 0x20),(DAT_005967bc + 0x22),
                (DAT_005967bc + 0x24),uVar5,uVar2,DAT_004c9754,0x0);
                FUN_00486ba4(DAT_004c9754);
                }
                break;
                case 0x4:
                if (DAT_005967bc != 0x0) {
                FUN_0043a810((DAT_005967bc + 0x20),DAT_004c9754);
                FUN_00482b7e((DAT_005967bc + 0x20),DAT_004c9754);
                FUN_00488456((DAT_005967bc + 0x20),DAT_004c9754);
                }
                break;
                case 0x5:
                for (local_12c = 0x0; local_12c < 0x28; local_12c = local_12c + 0x1) {
                puVar3 = &DAT_00568210 + local_12c * 0x9d + DAT_004c9754 * 0x1e22;
                puVar3[0x9c] = puVar3[0x9c] & 0xfe;
                puVar3[0x9c] = puVar3[0x9c] | 0x1;
                FUN_0043a810(local_12c,DAT_004c9754);
                FUN_00482b7e(local_12c,DAT_004c9754);
                FUN_00488456(local_12c,DAT_004c9754);
                }
                }
                }
                return 0x0;
            }
        }
        else {
            if (param_2 < 0x402) {
                if (DAT_004c976c != DAT_004c9754) {
                    FUN_0049bf80(param_1,0x1d56,0x414,0x0,0x0);
                    FUN_0049bf80(param_1,0x1d56,0x410,0x0,0x0);
                }
                for (local_1c = 0x0; local_1c < 0x3; local_1c = local_1c + 0x1) {
                    FUN_0049bf80(param_1,local_1c + 0x1f5,0x503,0x6,0x0);
                    FUN_0049bf80(param_1,local_1c + 0x1f5,0x502,(&DAT_004c5048)[local_1c * 0x2] + 0x1,0x0);
                }
                for (local_1c = 0x0; local_1c < 0x5; local_1c = local_1c + 0x1) {
                    if (((&DAT_00569a98)[local_1c * 0x1e22] & 0x1) != 0x0) {
                        FUN_0049bf80(param_1,local_1c + 0x1f8,0x410,0x0,0x0);
                    }
                }
                FUN_0049f99b(DAT_005b9bd4,param_1,0x407,0x3039,0x0);
            }
            else {
                if (param_2 == 0x405) {
                    FUN_004953d7();
                    for (local_8c = 0x0; local_8c < 0x5; local_8c = local_8c + 0x1) {
                        if (DAT_004c9754 == local_8c) {
                            local_90 = 0x272727;
                        }
                        else {
                            local_90 = 0xcaccce;
                        }
                        FUN_004968e7(local_8c * 0x78 + 0x1f,0x1aa,0x64,0x11,0xe);
                        FUN_00497567(local_8c * 0x78 + 0x51,0x1ae,(&DAT_00569b50 + local_8c * 0x1e22),0x64,local_90,0xe0e0e,
                                     0xcaccce,LPCSTR_005b9218,0x11);
                    }
                    FUN_00496ac0(DAT_004c5040,0x105,0x20,0x78,0x7b);
                    FUN_0049536f();
                    return 0x1;
                }
            }
        }
    }
    else {
        if (param_2 < 0x408) {
            if (param_3 < 0x1f9) {
                if (0x1f5 < param_3) {
                    if ((param_3 < 0x1f7) || (param_3 < 0x1f8)) {
                        LAB_00404ef2:
                        if (DAT_004c976c == DAT_004c9754) {
                            if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x4) == 0x0) {
                                pcVar4 = FUN_00499050(DAT_0059679c,0x7165);
                                FUN_0049d2e0(param_1,0x1,pcVar4);
                            }
                            else {
                                if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x80) == 0x0) {
                                    if ((&DAT_005698dc)[DAT_004c9754 * 0x1e22 + param_3] != -0x1) {
                                        pcVar4 = FUN_00499050(DAT_0059679c,0x7375);
                                        FUN_0049d2e0(param_1,0x1,pcVar4);
                                    }
                                }
                                else {
                                    pcVar4 = FUN_00499050(DAT_0059679c,0x73e2);
                                    FUN_0049d2e0(param_1,0x1,pcVar4);
                                }
                            }
                        }
                        else {
                            pcVar4 = FUN_00499050(DAT_0059679c,0x7163);
                            FUN_0049d2e0(param_1,0x1,pcVar4);
                        }
                        return 0x0;
                    }^
                    // goto LAB_00404ebe;
                }
                if (0x63 < param_3) {
                    if (param_3 < 0x65) {
                        if ((DAT_004c976c == DAT_004c9754) && (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x4) != 0x0)) {
                            iVar1 = FUN_004789fc();
                            local_154 = 0x0;
                            for (local_158 = 0x0; local_158 < 0x3; local_158 = local_158 + 0x1) {
                                if ((&DAT_004c5048)[local_158 * 0x2] == -0x1) {
                                    local_154 = local_154 + 0x1;
                                }
                            }
                            if ((local_154 != 0x0) && (iVar1 != 0x0)) {
                                FUN_0049d2b0(0xfa,0xc8);
                                pcVar7 = s_bin_ptrattut_bin_004c067d;
                                pcVar4 = FUN_00499050(DAT_0059679c,0x7412);
                                FUN_0049dc40(param_1,0x1,pcVar4,pcVar7);
                                return 0x0;
                            }
                        }
                        if ((((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x4) == 0x0) ||
                            (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x80) != 0x0)) {
                            FUN_0049c140(param_1,0x1);
                        }
                        else {
                            FUN_0049d2b0(0xfa,0xc8);
                            pcVar7 = s_bin_ptrattut_bin_004c068e;
                            pcVar4 = FUN_00499050(DAT_0059679c,0x73ab);
                            uVar5 = FUN_0049dc40(param_1,0x3,pcVar4,pcVar7);
                            if (uVar5 != 0x0) {
                                FUN_0049c140(param_1,0x1);
                            }
                        }
                        return 0x0;
                    }
                    if (param_3 == 0x1f5)^ // goto LAB_00404ef2;
                }
            }
            else {
                if (((param_3 < 0x1fa) || (param_3 < 0x1fc)) || (param_3 < 0x1fd)) {
                    LAB_00404ebe:
                    if (param_3 - 0x1f8 == DAT_004c9754) {
                        FUN_0044b1a8();
                    }
                    else {
                        FUN_00428bd5(param_3 - 0x1f8);
                    }
                    return 0x0;
                }
                if (0x1d55 < param_3) {
                    if (param_3 < 0x1d57) {
                        if (DAT_004c9770 == -0x1) {
                            FUN_0049d2b0(0xfa,0xc8);
                            pcVar7 = s_bin_ptrattut_bin_004c069f;
                            pcVar4 = FUN_00499050(DAT_0059679c,0x73ff);
                            uVar5 = FUN_0049dc40(param_1,0x3,pcVar4,pcVar7);
                            if (uVar5 == 0x0) {
                                return 0x0;
                            }
                            FUN_0040705e(DAT_004c9754);
                        }
                        return 0x0;
                    }
                    if (param_3 == 0x3039) {
                        FUN_00483355(0x20);
                    }
                }
            }
        }
        else {
            if (param_2 < 0x411) {
                if (param_2 == 0x40c) {
                    FUN_004953d7();
                    FUN_004a08c5(s_pcx_byzsecu_pcx_004c066d,0x0,0x0,0x280,0x1e0,0x0,0x0,0x0,0x0);
                    FUN_0049536f();
                    return 0x1;
                }
            }
            else {
                if (param_2 < 0x412) {
                    return 0x1;
                }
                if (param_2 < 0x413) {
                    FUN_004953d7();
                    if ((0x1f4 < param_3) && (param_3 < 0x1f8)) {
                        local_38 = *(param_4 + 0x14);
                        iVar1 = param_3 - 0x1f5;
                        bVar6 = false;
                        if ((&DAT_00569ad1)[DAT_004c9754 * 0x1e22 + iVar1] == -0x1) {
                            if (DAT_004c976c == DAT_004c9754) {
                                if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x4) == 0x0) {
                                    local_38 = (&DAT_004c5048)[iVar1 * 0x2] + 0x1;
                                }
                                else {
                                    if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x80) == 0x0) {
                                        if (local_38 != 0x0) {
                                            bVar6 = *(&DAT_004c505c + local_38 * 0x4) != 0x0;
                                            if (bVar6) {
                                                local_38 = local_38 + 0x1;
                                            }
                                            for (local_6c = 0x0; local_6c < 0x3; local_6c = local_6c + 0x1) {
                                                if ((local_6c != iVar1) && ((&DAT_004c5048)[local_6c * 0x2] + 0x1 == local_38)) {
                                                    local_38 = local_38 + 0x1;
                                                    bVar6 = true;
                                                }
                                            }
                                        }
                                    }
                                    else {
                                        local_38 = (&DAT_004c5048)[iVar1 * 0x2] + 0x1;
                                    }
                                }
                            }
                            else {
                                local_38 = (&DAT_004c5048)[iVar1 * 0x2] + 0x1;
                            }
                        }
                        else {
                            local_38 = (&DAT_004c5048)[iVar1 * 0x2] + 0x1;
                        }
                        if (bVar6) {
                            if (0x5 < local_38) {
                                local_38 = 0x0;
                            }
                            FUN_0049bf80(param_1,param_3,0x502,local_38,0x0);
                        }
                        else {
                            (&DAT_004c5048)[iVar1 * 0x2] = local_38 + -0x1;
                            FUN_0049c60b((&PTR_s_bin_stigmata_bin_004bd044)[iVar1],DAT_004c5044,0x39a8,local_38 * 0x39a8);
                            FUN_00496ac0(DAT_004c5044,iVar1 * 0xc3 + 0x42,0xb5,0x78,0x7b);
                        }
                    }
                    FUN_0049536f();
                }
                else {
                    if (param_2 == 0x413) {
                        FUN_0049d2b0(0xfa,0xc8);
                        switch(param_3) {
                            case 0x1f5:
                                FUN_00483355(0x27);
                            break;
                            case 0x1f6:
                                FUN_00483355(0x28);
                            break;
                            case 0x1f7:
                                FUN_00483355(0x29);
                            break;
                            case 0x1f8:
                                case 0x1f9:
                                case 0x1fa:
                                case 0x1fb:
                                case 0x1fc:
                                FUN_00483355(0x2a);
                        }
                        return 0x0;
                    }
                }
            }
        }
    }
    return 0x0;
}



fn FUN_00405178()

{
    let mut iVar1: i32;
    let mut pcVar2: String;
    u32 local_18c [0x20];
    let mut local_10c: *mut u32 [0x11];
    let ppuStack199: *mut *mut u8;;
    let mut local_1b: String;;

    iVar1 = FUN_00406cc8(DAT_004c9754);
    if (iVar1 == 0x0) {
        (&DAT_00569a98)[DAT_004c9754 * 0x1e22] = (&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0xf7;
        pcVar2 = FUN_00499050(DAT_0059679c,0x73ed);
        FUN_0049d2e0(0x0,0x1,pcVar2);
    }
    else {
        FUN_00489246(DAT_004c9754,0x0);
        if (*(&DAT_00569ac5 + DAT_004c9754 * 0x1e22) != -0x1) {
            if (((&DAT_00569a98)[*(&DAT_00569ac5 + DAT_004c9754 * 0x1e22) * 0x1e22] & 0x1) == 0x0) {
                (&DAT_00569a98)[DAT_004c9754 * 0x1e22] = (&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0xf7;
                *(&DAT_004d5568 + *(&DAT_00569ac5 + DAT_004c9754 * 0x1e22) * 0x4) =
                    *(&DAT_004d5568 + *(&DAT_00569ac5 + DAT_004c9754 * 0x1e22) * 0x4) +
                        *(&DAT_00569ab5 + DAT_004c9754 * 0x1e22);
                pcVar2 = FUN_00499050(DAT_005967ac,0x3f1);
                FUN_0049c2e0(local_18c,pcVar2);
                FUN_0045518a(0x1 << ((byte)DAT_004c9754 & 0x1f),0xffffffff,0x74cc,0xffffffff,local_18c,0xffffffff,0x0);
                return;
            }
            *(&DAT_00569ac5 + DAT_004c9754 * 0x1e22) = 0xffffffff;
        }
        FUN_004990e0(local_10c,0x0,s_diplo_res_004c06b8,s_VoteDlg_004c06b0);
        FUN_0049bb50(local_10c,FUN_00405374);
        ppuStack199 = &PTR_FUN_004c3d34;
        if (local_1b != 0x0) {
            FUN_00499b30(local_10c,local_1b);
        }
        FUN_0049a1c0(local_10c,0x1);
    }
    return;
}



fn FUN_00405374(param_1: *mut *mut u32,param_2: u32,param_3: u32) -> u32

{
    let mut pcVar1: String;
    let mut uVar2: u32;
    let local_130: u8 [0x80];
    let mut local_b0: i32;
    let mut local_ac: u32;
    let mut local_a8: u32;
    let mut local_a4: u32;
    let local_a0: u8 [0x80];
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_14: u32;

    local_14 = param_2;
    if (param_2 < 0x407) {
        if (0x400 < param_2) {
            if (param_2 < 0x402) {
                for (local_1c = 0x0; local_1c < 0x5; local_1c = local_1c + 0x1) {
                    if (((&DAT_00569a98)[local_1c * 0x1e22] & 0x1) != 0x0) {
                        FUN_0049bf80(param_1,local_1c + 0x1f8,0x410,0x0,0x0);
                    }
                }
                return 0x0;
            }
            if (param_2 == 0x405) {
                FUN_004953d7();
                pcVar1 = FUN_00499050(DAT_0059679c,0x7352);
                FUN_0049c2e0(local_a0,pcVar1);
                FUN_00497567(0x140,*(param_1 + 0x21) + 0xf,local_a0,0x280,0xcaccce,-0x1,0xcaccce,
                             LPCSTR_005b9218,0x11);
                if (*(&DAT_00569ab5 + DAT_004c9754 * 0x1e22) == 0x1) {
                    pcVar1 = FUN_00499050(DAT_0059679c,0x7351);
                    FUN_0049c2e0(local_a0,pcVar1);
                }
                else {
                    pcVar1 = FUN_00499050(DAT_0059679c,0x7350);
                    FUN_0049c2e0(local_a0,pcVar1);
                }
                FUN_00497567(0xc9,*(param_1 + 0x21) + 0xb1,local_a0,0x154,0xcaccce,-0x1,0xcaccce,
                             LPCSTR_005b9218,0x11);
                for (local_20 = 0x0; local_20 < 0x5; local_20 = local_20 + 0x1) {
                    if (DAT_004c9754 == local_20) {
                        local_a4 = 0x272727;
                    }
                    else {
                        local_a4 = 0xcaccce;
                    }
                    local_a8 = local_a4;
                    FUN_00497567(local_20 * 0x78 + 0x51,*(param_1 + 0x21) + 0x94,
                                 (&DAT_00569b50 + local_20 * 0x1e22),0x64,local_a4,0xe0e0e,0xcaccce,
                                 LPCSTR_005b9218,0x11);
                }
                FUN_0049536f();
                return 0x1;
            }
        }
    }
    else {
        if (param_2 < 0x408) {
            local_ac = param_3;
            if (param_3 < 0x1f9) {
                if (param_3 < 0xc9) {
                    if (param_3 == 0xc8) {
                        (&DAT_00569a98)[DAT_004c9754 * 0x1e22] = (&DAT_00569a98)[DAT_004c9754 * 0x1e22] | 0x8;
                        FUN_0049c140(param_1,0x0);
                    }
                }
                else {
                    if (param_3 < 0xca) {
                        (&DAT_00569a98)[DAT_004c9754 * 0x1e22] = (&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0xf7;
                        FUN_0049c140(param_1,0x1);
                    }
                    else {
                        if (param_3 == 0x1f8)^ // goto LAB_00405642;
                    }
                }
            }
            else {
                if ((((param_3 < 0x1fa) || (param_3 < 0x1fb)) || (param_3 < 0x1fc)) || (param_3 == 0x1fc)) {
                    LAB_00405642:
                        local_b0 = param_3 - 0x1f8;
                    FUN_00499050(DAT_0059679c,param_3 + 0x21c);
                    pcVar1 = FUN_00499050(DAT_0059679c,0x73f6);
                    FUN_0049c2e0(local_130,pcVar1);
                    uVar2 = FUN_0049d2e0(param_1,0x3,local_130);
                    if (uVar2 != 0x0) {
                        (&DAT_00569a98)[DAT_004c9754 * 0x1e22] = (&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0xf7;
                        *(&DAT_004d5568 + local_b0 * 0x4) =
                            *(&DAT_004d5568 + local_b0 * 0x4) + *(&DAT_00569ab5 + DAT_004c9754 * 0x1e22);
                        FUN_0049c140(param_1,0x1);
                    }
                    return 0x0;
                }
            }
        }
        else {
            if (0x40b < param_2) {
                if (param_2 < 0x40d) {
                    FUN_004953d7();
                    FUN_004a08c5(s_pcx_vote_pcx_004c06c2,*(param_1 + 0x1d),*(param_1 + 0x21),
                                 *(param_1 + 0x25),*(param_1 + 0x29),0x0,0x0,0x0,0x0);
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

fn FUN_0040581a()

{
    let cVar1: u8;
    let puVar2: *mut u32;
    let mut iVar3: i32;
    let mut pcVar4: String;
    let mut iVar5: i32;
    let mut pcVar6: String;
    let puVar7: *mut u32;
    let mut uVar8: u32;
    let uVar9: u16;
    let local_2a0: u8;
    let local_29f: u8 [0x7f];
    u32 local_220 [0x80];
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    local_1c = 0x0;
    local_20 = -0x1;
    local_14 = -0x1;
    for (local_18 = 0x0; local_18 < 0x5; local_18 = local_18 + 0x1) {
        if (local_20 < *(&DAT_004d5568 + local_18 * 0x4)) {
            local_20 = *(&DAT_004d5568 + local_18 * 0x4);
            local_14 = local_18;
        }
        *(&DAT_00569ac5 + local_18 * 0x1e22) = 0xffffffff;
    }
    for (local_18 = 0x0; local_18 < 0x5; local_18 = local_18 + 0x1) {
        if ((local_14 != local_18) && (*(&DAT_004d5568 + local_18 * 0x4) == local_20)) {
            local_1c = 0x1;
            break;
        }
    }
    if (local_1c == 0x0) {
        iVar3 = FUN_00406cc8(local_14);
        iVar5 = _DAT_004c9774;
        if (iVar3 == 0x0) {
            FUN_00499050(DAT_0059679c,local_14 + 0x414);
            pcVar4 = FUN_00499050(DAT_0059679c,0x73cc);
            FUN_0049c2e0(local_220,pcVar4);
            pcVar4 = FUN_00499050(DAT_0059679c,0x7401);
            FUN_0049c2e0(&local_2a0,pcVar4);
            pcVar4 = &local_2a0;
            iVar5 = -0x1;
            puVar2 = local_220;
            loop {
                puVar7 = puVar2;
                if (iVar5 == 0x0) break;
                iVar5 = iVar5 + -0x1;
                puVar7 = (puVar2 + 0x1);
                cVar1 = puVar2;
                puVar2 = puVar7;
            } while (cVar1 != '\0');
            pcVar6 = (puVar7 + -0x1);
            loop {
                cVar1 = *pcVar4;
                *pcVar6 = cVar1;
                if (cVar1 == '\0') break;
                cVar1 = pcVar4[0x1];
                pcVar4 = pcVar4 + 0x2;
                pcVar6[0x1] = cVar1;
                pcVar6 = pcVar6 + 0x2;
            } while (cVar1 != '\0');
            FUN_0045518a(0x1f,0xffffffff,0x74cc,0xffffffff,local_220,0xffffffff,0x1);
        }
        else {
            DAT_004c976c = local_14;
            if (local_14 == DAT_004c9770) {
                local_220[0]._0_1_ = 0x0;
                _DAT_004c9774 = _DAT_004c9774 + 0x1;
                if (iVar5 == 0x0) {
                    FUN_00499050(DAT_0059679c,local_14 + 0x414);
                    pcVar4 = FUN_00499050(DAT_0059679c,0x73cc);
                    FUN_0049c2e0(local_220,pcVar4);
                    pcVar4 = FUN_00499050(DAT_0059679c,0x73e4);
                    iVar5 = -0x1;
                    puVar2 = local_220;
                    loop {
                        puVar7 = puVar2;
                        if (iVar5 == 0x0) break;
                        iVar5 = iVar5 + -0x1;
                        puVar7 = (puVar2 + 0x1);
                        cVar1 = puVar2;
                        puVar2 = puVar7;
                    } while (cVar1 != '\0');
                    pcVar6 = (puVar7 + -0x1);
                    loop {
                        cVar1 = *pcVar4;
                        *pcVar6 = cVar1;
                        if (cVar1 == '\0') break;
                        cVar1 = pcVar4[0x1];
                        pcVar4 = pcVar4 + 0x2;
                        pcVar6[0x1] = cVar1;
                        pcVar6 = pcVar6 + 0x2;
                    } while (cVar1 != '\0');
                }
                else {
                    if (iVar5 == 0x1) {
                        DAT_00595738 = local_14;
                        FUN_00499050(DAT_0059679c,local_14 + 0x414);
                        pcVar4 = FUN_00499050(DAT_0059679c,0x73cc);
                        FUN_0049c2e0(local_220,pcVar4);
                        pcVar4 = FUN_00499050(DAT_0059679c,0x73e5);
                        iVar5 = -0x1;
                        puVar2 = local_220;
                        loop {
                            puVar7 = puVar2;
                            if (iVar5 == 0x0) break;
                            iVar5 = iVar5 + -0x1;
                            puVar7 = (puVar2 + 0x1);
                            cVar1 = puVar2;
                            puVar2 = puVar7;
                        } while (cVar1 != '\0');
                        pcVar6 = (puVar7 + -0x1);
                        loop {
                            cVar1 = *pcVar4;
                            *pcVar6 = cVar1;
                            if (cVar1 == '\0') break;
                            cVar1 = pcVar4[0x1];
                            pcVar4 = pcVar4 + 0x2;
                            pcVar6[0x1] = cVar1;
                            pcVar6 = pcVar6 + 0x2;
                        } while (cVar1 != '\0');
                    }
                }
                FUN_0045518a(0x1f,0xffffffff,0x74cc,0xffffffff,local_220,0xffffffff,0x1);
            }
            else {
                if (DAT_004c9770 != -0x1) {
                    FUN_00499050(DAT_0059679c,DAT_004c9770 + 0x414);
                    pcVar4 = FUN_00499050(DAT_0059679c,0x73cc);
                    FUN_0049c2e0(local_220,pcVar4);
                    pcVar4 = FUN_00499050(DAT_0059679c,0x73f7);
                    iVar5 = -0x1;
                    puVar2 = local_220;
                    loop {
                        puVar7 = puVar2;
                        if (iVar5 == 0x0) break;
                        iVar5 = iVar5 + -0x1;
                        puVar7 = (puVar2 + 0x1);
                        cVar1 = puVar2;
                        puVar2 = puVar7;
                    } while (cVar1 != '\0');
                    pcVar6 = (puVar7 + -0x1);
                    loop {
                        cVar1 = *pcVar4;
                        *pcVar6 = cVar1;
                        if (cVar1 == '\0') break;
                        cVar1 = pcVar4[0x1];
                        pcVar4 = pcVar4 + 0x2;
                        pcVar6[0x1] = cVar1;
                        pcVar6 = pcVar6 + 0x2;
                    } while (cVar1 != '\0');
                    FUN_0045518a(0x1f,0xffffffff,0x74cc,0xffffffff,local_220,0xffffffff,0x1);
                }
                (&DAT_00569a98)[DAT_004c976c * 0x1e22] = (&DAT_00569a98)[DAT_004c976c * 0x1e22] | 0x4;
                (&DAT_00569a98)[DAT_004c976c * 0x1e22] = (&DAT_00569a98)[DAT_004c976c * 0x1e22] & 0x7f;
                *(&DAT_00569aad + DAT_004c976c * 0x1e22) = *(&DAT_00569aad + DAT_004c976c * 0x1e22) + 0x1;
                DAT_004c9770 = -0x1;
                _DAT_004c9774 = 0x0;
                FUN_00499050(DAT_0059679c,DAT_004c976c + 0x414);
                pcVar4 = FUN_00499050(DAT_0059679c,0x73cc);
                FUN_0049c2e0(local_220,pcVar4);
                pcVar4 = FUN_00499050(DAT_0059679c,0x735d);
                iVar5 = -0x1;
                puVar2 = local_220;
                loop {
                    puVar7 = puVar2;
                    if (iVar5 == 0x0) break;
                    iVar5 = iVar5 + -0x1;
                    puVar7 = (puVar2 + 0x1);
                    cVar1 = puVar2;
                    puVar2 = puVar7;
                } while (cVar1 != '\0');
                pcVar6 = (puVar7 + -0x1);
                loop {
                    cVar1 = *pcVar4;
                    *pcVar6 = cVar1;
                    if (cVar1 == '\0') break;
                    cVar1 = pcVar4[0x1];
                    pcVar4 = pcVar4 + 0x2;
                    pcVar6[0x1] = cVar1;
                    pcVar6 = pcVar6 + 0x2;
                } while (cVar1 != '\0');
                FUN_0045518a(0x1f,0xffffffff,0x74cc,0xffffffff,local_220,0xffffffff,0x1);
            }
        }
    }
    else {
        uVar9 = 0x1;
        uVar8 = 0xffffffff;
        puVar2 = FUN_00499050(DAT_0059679c,0x7400);
        FUN_0045518a(0x1f,0xffffffff,0x74cc,0xffffffff,puVar2,uVar8,uVar9);
    }
    return;
}


fn FUN_0042285f(param_1: i32,param_2: i32) -> u32

{
    let mut local_14: i32;

    local_14 = 0x0;
    while( true ) {
        if (0x2 < local_14) {
            return 0x0;
        }
        if (*(&DAT_004c97f8 + param_1 * 0xd90 + param_2 * 0xf8 + local_14 * 0x28) == 0x12c) break;
        local_14 = local_14 + 0x1;
    }
    return *(&DAT_004c97fc + param_1 * 0xd90 + param_2 * 0xf8 + local_14 * 0x28);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_004228d7()

{
    let mut iVar1: i32;
    let puVar2: *mut u32;
    let mut uVar3: u32;
    let mut iVar4: i32;
    let mut uVar5: u32;
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    local_14 = 0x0;
    loop {
    if (0x4 < local_14) {
        return;
    }
    if (*(&DAT_004c977c + DAT_004c9754 * 0xf8 + local_14 * 0xd90) == 0x0) {
        local_28 = 0x0;
        local_24 = 0x0;
        if (*(&DAT_004c9778 + DAT_004c9754 * 0xf8 + local_14 * 0xd90) == 0x1) {
            FUN_0042d03d(DAT_004c9754,local_14);
            FUN_00419085();
        }
        else {
            for (local_18 = 0x0; local_18 < 0x3; local_18 = local_18 + 0x1) {
                puVar2 = (&DAT_004c9780 + local_18 * 0x28 + DAT_004c9754 * 0xf8 + local_14 * 0xd90);
                uVar5 = *puVar2;
                if (uVar5 < 0x154) {
                    if (uVar5 < 0x140) {
                        if (uVar5 < 0x12c) {
                            if (uVar5 != 0x0)^ // goto LAB_00422de6;
                        }
                        else {
                            if (uVar5 < 0x12d) {
                                if (DAT_004c9754 == 0x6) {
                                    local_24 = -0x1;
                                }
                                else {
                                    if ((*(&DAT_00569abd + DAT_004c9754 * 0x1e22) * 0x2) / 0x3 < puVar2[0x1]) {
                                        local_24 = local_24 + puVar2[0x1];
                                    }
                                    else {
                                        local_24 = -0x1;
                                    }
                                }
                            }
                            else {
                                if (uVar5 != 0x136)^ // goto LAB_00422de6;
                                if (DAT_004c9754 == 0x6) {
                                    local_24 = -0x1;
                                }
                                else {
                                    if (((&DAT_00569c30)[puVar2[0x2] * 0x9 + DAT_004c9754 * 0x1e22] & 0x1) == 0x0) {
                                        local_24 = -0x1;
                                    }
                                    else {
                                        uVar5 = puVar2[0x2];
                                        uVar3 = FUN_004a2edc();
                                        local_24 = local_24 + (uVar3 % 0x65 + -0x19 + *(&DAT_0058ad6a + uVar5 * 0xda)) * 0xa;
                                    }
                                }
                            }
                        }
                    }
                    else {
                        if (uVar5 < 0x141) {
                            if (DAT_004c9754 == 0x6) {
                                local_24 = -0x1;
                            }
                            else {
                                iVar4 = FUN_00481a7f(puVar2[0x3],local_14,DAT_004c9754);
                                local_24 = local_24 + iVar4 * 0x64;
                            }
                        }
                        else {
                            if (uVar5 < 0x14b) {
                                if (uVar5 != 0x14a)^ // goto LAB_00422de6;
                                if (*(&DAT_00569ac5 + DAT_004c9754 * 0x1e22) == -0x1) {
                                    if ((&DAT_004d55a8)[DAT_004c9754 * 0xe + local_14] == '\x02') {
                                        local_24 = -0x1;
                                    }
                                    else {
                                        if ((DAT_004c9754 == 0x6) && (((&DAT_00569a98)[local_14 * 0x1e22] & 0x20) == 0x0)) {
                                            uVar5 = FUN_004a2edc();
                                            local_24 = local_24 + (uVar5 % 0xb) * 0x3e8 + 0x2710;
                                        }
                                        else {
                                            uVar5 = FUN_004a2edc();
                                            local_24 = local_24 + (uVar5 % 0xb) * 0x3e8 + 0x1388;
                                        }
                                    }
                                }
                                else {
                                    local_24 = -0x1;
                                }
                            }
                            else {
                                if ((uVar5 < 0x14c) || (uVar5 < 0x14d)) {
                                    if (puVar2[0x5] == DAT_004c9754) {
                                        iVar4 = FUN_00421079(local_14);
                                        if ((*(&DAT_00569b14 + DAT_004c9754 * 0x1e22 + local_14 * 0x4) < 0x32) && (iVar4 < 0x1f)) {
                                            iVar4 = *(&DAT_00569b14 + DAT_004c9754 * 0x1e22 + local_14 * 0x4);
                                            uVar5 = FUN_004a2edc();
                                            local_24 = local_24 + (uVar5 % 0xb + 0x14) * (0x64 - iVar4);
                                        }
                                    }
                                    else {
                                        local_24 = -0x1;
                                    }
                                }
                                else {
                                    if (uVar5 != 0x14d)^ // goto LAB_00422de6;
                                    if (puVar2[0x5] == DAT_004c9754) {
                                        local_24 = -0x1;
                                    }
                                    else {
                                        iVar4 = FUN_00421079(puVar2[0x5]);
                                        if (0x19 < iVar4) {
                                            local_24 = -0x1;
                                        }
                                        if (*(&DAT_00569b14 + puVar2[0x5] * 0x4 + DAT_004c9754 * 0x1e22) < 0x32) {
                                            uVar5 = FUN_004a2edc();
                                            local_24 = local_24 + (uVar5 % 0x3d + 0x46) * iVar4;
                                        }
                                        else {
                                            local_24 = -0x1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                else {
                    if (uVar5 < 0x155) {
                        if ((DAT_004c9754 != 0x5) || ((&DAT_004d55ee)[local_14] == '\x02')) {
                            local_24 = -0x1;
                        }
                    }
                    else {
                        if (uVar5 < 0x17c) {
                            if (uVar5 < 0x168) {
                                if (uVar5 != 0x15e)^ // goto LAB_00422de6;
                                if (((*(&DAT_0058ad72 + *puVar2 * 0xda) == 0x0) &&
                                    (*(&DAT_0058ad6e + *puVar2 * 0xda) != 0xa)) && (_DAT_00582910 == 0x0)) {
                                    uVar5 = *puVar2;
                                    uVar3 = FUN_004a2edc();
                                    iVar4 = *(&DAT_0058ad6e + uVar5 * 0xda) + uVar3 % 0x4;
                                    local_24 = local_24 + iVar4 * iVar4 * 0x64;
                                }
                                else {
                                    local_24 = -0x1;
                                    FUN_00419085();
                                }
                            }
                            else {
                                if (uVar5 < 0x169) {
                                    local_24 = -0x1;
                                }
                                else {
                                    if (uVar5 != 0x172)^ // goto LAB_00422de6;
                                    if (DAT_004c9754 == 0x6) {
                                        iVar4 = *(&DAT_00574fe0 + local_14 * 0x4);
                                        iVar1 = *(&DAT_00574fe0 + puVar2[0x9] * 0x4);
                                        uVar5 = FUN_004a2edc();
                                        iVar4 = (iVar4 - iVar1) + uVar5 % 0xa;
                                        if ((iVar4 < 0x0) || (0x64 < iVar4)) {
                                            local_24 = -0x1;
                                        }
                                        else {
                                            local_24 = local_24 + (0x64 - iVar4) * 0x64;
                                        }
                                    }
                                    else {
                                        local_24 = -0x1;
                                    }
                                }
                            }
                        }
                        else {
                            if (uVar5 < 0x17d) {
                                LAB_00422de6:
                                    local_24 = -0x1;
                            }
                            else {
                                if (uVar5 < 0x187) {
                                    if (uVar5 != 0x186)^ // goto LAB_00422de6;
                                }
                                else {
                                    if (0x187 < uVar5) {
                                        if (uVar5 < 0x189) {
                                            uVar5 = FUN_004a2edc();
                                            local_24 = local_24 + (uVar5 % 0xb) * 0x1f4 + 0x9c4;
                                        }
                                        else {
                                            if (uVar5 != 0x189)^ // goto LAB_00422de6;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                if (local_24 < 0x0) break;
            }
            if (local_24 < 0x0) {
                FUN_0042d03d(DAT_004c9754,local_14);
            }
            if (*(&DAT_004c977c + DAT_004c9754 * 0xf8 + local_14 * 0xd90) == 0x0) {
                for (local_18 = 0x0; uVar5 = DAT_004c9754, local_18 < 0x3; local_18 = local_18 + 0x1) {
                    puVar2 = (&DAT_004c97f8 + local_18 * 0x28 + DAT_004c9754 * 0xf8 + local_14 * 0xd90);
                    uVar3 = *puVar2;
                    if (uVar3 < 0x154) {
                        if (uVar3 < 0x140) {
                            if (uVar3 < 0x12c) {
                                DAT_004c9754 = uVar5;
                                if (uVar3 != 0x0)^ // goto LAB_00423314;
                            }
                            else {
                                if (uVar3 < 0x12d) {
                                    if (puVar2[0x1] < *(&DAT_00569abd + local_14 * 0x1e22)) {
                                        local_28 = local_28 + puVar2[0x1];
                                        DAT_004c9754 = uVar5;
                                    }
                                    else {
                                        local_28 = -0x1;
                                        DAT_004c9754 = uVar5;
                                    }
                                }
                                else {
                                    if (uVar3 != 0x136)^ // goto LAB_00423314;
                                    DAT_004c9754 = uVar5;
                                    if (((&DAT_00569c30)[puVar2[0x2] * 0x9 + DAT_004c9754 * 0x1e22] & 0x1) == 0x0) {
                                        uVar5 = puVar2[0x2];
                                        uVar3 = FUN_004a2edc();
                                        local_28 = local_28 + (uVar3 % 0x65 + -0x19 + *(&DAT_0058ad6a + uVar5 * 0xda)) * 0xa;
                                    }
                                }
                            }
                        }
                        else {
                            if (uVar3 < 0x141) {
                                if (DAT_004c9754 == 0x6) {
                                    iVar4 = FUN_00481b16(puVar2[0x3],0x6,local_14);
                                }
                                else {
                                    iVar4 = FUN_00481a7f(puVar2[0x3],DAT_004c9754,local_14);
                                }
                                local_28 = local_28 + iVar4 * 0x64;
                            }
                            else {
                                if (uVar3 < 0x14b) {
                                    if (uVar3 != 0x14a)^ // goto LAB_00423314;
                                    uVar5 = FUN_004a2edc();
                                    local_28 = local_28 + uVar5 % 0xfa0 + 0x3e8;
                                }
                                else {
                                    if (uVar3 < 0x14c) {
                                        LAB_004231e6:
                                            DAT_004c9754 = uVar5;
                                        if ((&DAT_004d55a8)[DAT_004c9754 * 0xe + local_14] != '\0') {
                                            if (puVar2[0x5] == DAT_004c9754) {
                                                iVar4 = FUN_00421079(local_14);
                                                if ((*(&DAT_00569b14 + DAT_004c9754 * 0x1e22 + local_14 * 0x4) < 0x32) &&
                                                    (iVar4 < 0x1f)) {
                                                    local_28 = -0x1;
                                                }
                                                else {
                                                    uVar5 = FUN_004a2edc();
                                                    local_28 = local_28 + (uVar5 % 0x7 + 0x7) * iVar4;
                                                }
                                            }
                                            else {
                                                local_28 = -0x1;
                                            }
                                        }
                                    }
                                    else {
                                        if (uVar3 < 0x14d) {
                                            DAT_004c9754 = uVar5;
                                            if (0x1 < (byte)(&DAT_004d55a8)[DAT_004c9754 * 0xe + local_14])^ // goto LAB_004231e6;
                                        }
                                        else {
                                            if (uVar3 != 0x14d)^ // goto LAB_00423314;
                                            if (puVar2[0x5] == DAT_004c9754) {
                                                local_28 = -0x1;
                                                DAT_004c9754 = uVar5;
                                            }
                                            else {
                                                iVar4 = FUN_00421079(puVar2[0x5]);
                                                if (*(&DAT_00569b14 + puVar2[0x5] * 0x4 + DAT_004c9754 * 0x1e22) < 0x32) {
                                                    uVar5 = FUN_004a2edc();
                                                    local_28 = local_28 + (uVar5 % 0xb + 0xf) * iVar4;
                                                }
                                                else {
                                                    local_28 = -0x1;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    else {
                        if (uVar3 < 0x155) {
                            local_28 = -0x1;
                            DAT_004c9754 = uVar5;
                        }
                        else {
                            if (uVar3 < 0x17c) {
                                if (uVar3 < 0x168) {
                                    if (uVar3 != 0x15e)^ // goto LAB_00423314;
                                    LAB_00423148:
                                        local_28 = -0x1;
                                    DAT_004c9754 = uVar5;
                                }
                                else {
                                    if (uVar3 < 0x169) {
                                        uVar5 = FUN_004a2edc();
                                        local_28 = local_28 + (uVar5 % 0x5 + 0xf) * 0x3e8;
                                    }
                                    else {
                                        if (uVar3 == 0x172)^ // goto LAB_00423148;
                                        LAB_00423314:
                                            local_28 = -0x1;
                                        DAT_004c9754 = uVar5;
                                    }
                                }
                            }
                            else {
                                if (uVar3 < 0x17d) {
                                    local_28 = -0x1;
                                    DAT_004c9754 = uVar5;
                                }
                                else {
                                    if (uVar3 < 0x187) {
                                        DAT_004c9754 = uVar5;
                                        if (uVar3 != 0x186)^ // goto LAB_00423314;
                                    }
                                    else {
                                        DAT_004c9754 = uVar5;
                                        if (0x187 < uVar3) {
                                            if (uVar3 < 0x189) {
                                                if (DAT_004c976c == local_14) {
                                                    uVar5 = FUN_004a2edc();
                                                    iVar4 = (uVar5 % 0xb) * 0x1f4 + 0x9c4;
                                                }
                                                else {
                                                    uVar5 = FUN_004a2edc();
                                                    iVar4 = uVar5 % 0xc8 + 0x64;
                                                }
                                                local_28 = local_28 + iVar4;
                                            }
                                            else {
                                                if (uVar3 != 0x18d)^ // goto LAB_00423314;
                                                DAT_004c9754 = local_14;
                                                iVar4 = FUN_004826bc(puVar2[0x3]);
                                                if ((iVar4 == -0x1) || (DAT_005827f4 >> 0x10 == iVar4)) {
                                                    local_28 = -0x1;
                                                    DAT_004c9754 = uVar5;
                                                }
                                                else {
                                                    DAT_004c9754 = uVar5;
                                                    uVar5 = FUN_004a2edc();
                                                    local_28 = local_28 + uVar5 % 0x3e8 + 0x1f4;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if (local_28 < 0x0) break;
                }
                if (local_28 < 0x0) {
                    FUN_0042d03d(DAT_004c9754,local_14);
                }
                if (*(&DAT_004c977c + DAT_004c9754 * 0xf8 + local_14 * 0xd90) == 0x0) {
                    if (((local_28 < local_24) || (local_28 == 0x0)) && ((local_28 != 0x0 || (local_24 != 0x0)))) {
                        FUN_0042d03d(DAT_004c9754,local_14);
                        FUN_00419085();
                    }
                    else {
                        FUN_0042ce75(DAT_004c9754,local_14);
                        FUN_00419085();
                    }
                }
            }
            else {
                FUN_00419085();
            }
        }
    }
    local_14 = local_14 + 0x1;
} while( true );
}



// WARNING: Could not reconcile some variable overlaps

fn FUN_00423530(param_1: i32) -> u32

{
    let mut pcVar1: String;
    let mut uVar2: u32;
    u32 local_a4 [0x20];
    let mut local_24: u32;
    let mut local_20: *mut u8;
    let mut local_1c: u32;
    let mut local_18: i32;
    let mut local_14: u32;

    local_20 = &DAT_004d55a8;
    local_1c = 0x8;
    local_18 = param_1;
    uVar2 = (byte)(&DAT_004d5618)[param_1];
    if (uVar2 != 0x2) {
        local_14 = uVar2;
        FUN_00499050(DAT_0059679c,param_1 + 0x414);
        pcVar1 = FUN_00499050(DAT_005967ac,0x7d5);
        uVar2 = FUN_0049c2e0(local_a4,pcVar1);
        for (local_24 = 0x0; local_24 < 0x5; local_24 = local_24 + 0x1) {
            if (local_24 != DAT_004c9754) {
                FUN_0045518a(0x1 << ((byte)local_24 & 0x1f),0x8,0x74cc,0xffffffff,local_a4,0xffffffff,0x0);
            }
            uVar2 = local_24;
        }
    }
    return uVar2;
}



fn FUN_00423615(param_1: i32,param_2: i32,param_3: i32) -> i32

{
let piVar1: *mut i32;;
let mut local_18: i32;
let local_14: *mut u32;

local_18 = 0x0;
FUN_004a0430(param_1,0x0,0x34);
FUN_004a0430(param_2,0x0,0x34);
for (local_14 = (&DAT_005b8b44 + (*(param_3 + 0x6) >> 0x10) * 0x4);
(local_14 != 0x0 && ((local_14 + 0x8) == (param_3 + 0x8)));
local_14 = *local_14) {
if ((((local_14 + 0x22) == (param_3 + 0xa)) &&
((((local_14 + 0x9) == (param_3 + 0xc) && ((*(local_14 + 0x3a) & 0x1) != 0x0))
&& (local_18 = local_18 + 0x1, (local_14 + 0x27) == '[')))) &&
(*(local_14 + 0x23) >> 0x18 == DAT_004c9754)) {
(param_2 + (*(local_14 + 0x2d) >> 0x18) * 0x4) = local_14;
piVar1 = ((*(local_14 + 0x2d) >> 0x18) * 0x4 + param_1);
*piVar1 = *piVar1 + (*(local_14 + 0x2f) >> 0x10);
}
}
return local_18;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0042376e(param_1: i32,param_2: i32) -> u32

{
    let mut uVar1: u32;
    let mut iVar2: i32;
    let mut iVar3: i32;
    i32 aiStackY592 [0x6a];
    let local_84: u8 [0x34];
    i32 local_50 [0xd];
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: u32;

    local_1c = 0x0;
    uVar1 = *(param_1 + 0x6) >> 0x10;
    if (uVar1 != _DAT_004bd714) {
        FUN_00423615(local_50,local_84,param_1);
        FUN_0042feac();
        uVar1 = FUN_004a2edc();
        uVar1 = uVar1 & 0x3;
        local_18 = uVar1;
        local_14 = 0x0;
        while ((local_14 < 0xd && (param_2 != 0x0))) {
            iVar2 = *(&DAT_004bd715 + local_18 * 0xd + local_14) >> 0x18;
            iVar3 = *(&DAT_004bd6e0 + iVar2 * 0x4) - local_50[iVar2];
            if (iVar3 < 0x1) {
                if (iVar3 < 0x0) {
                    FUN_00419085();
                    FUN_00465dae(*(param_1 + 0x6) >> 0x10,*(param_1 + 0x8) >> 0x10,*(param_1 + 0xa) >> 0x10,
                                 iVar2,-iVar3,0x5);
                    param_2 = param_2 + -0x1;
                }
            }
            else {
                FUN_00419085();
                FUN_00465f06(*(param_1 + 0x6) >> 0x10,*(param_1 + 0x8) >> 0x10,*(param_1 + 0xa) >> 0x10,
                             iVar2,iVar3,0x5,0x0);
                param_2 = param_2 + -0x1;
            }
            uVar1 = local_14;
            local_14 = local_14 + 0x1;
        }
    }
    return uVar1;
}



fn FUN_004238dc(param_1: i32) -> *mut u32

{
    let local_14: *mut u32;

    if (((&DAT_004c7108)[param_1 * 0x4] & 0x10) != 0x0) {
        for (local_14 = (&DAT_005b89f8 + param_1 * 0x4);
            (local_14 != 0x0 && (*(local_14 + 0x6) >> 0x10 == param_1));
            local_14 = *local_14) {
            if (((local_14 + 0xe) == 0x4) && ((local_14 + 0x4) == 0x5)) {
                return local_14;
            }
        }
    }
    return 0x0;
}



fn FUN_00423961(param_1: i32)

{
    let mut iVar1: i32;
    let puVar2: *mut u32;

    iVar1 = *(*(&DAT_00582938 +
        (*(param_1 + 0x25) >> 0x18) * 0x4 + (*(param_1 + 0x24) >> 0x18) * 0x18) + 0xa5)
    ;
    puVar2 = FUN_004238dc((param_1 + 0x20));
    if (puVar2 != 0x0) {
        FUN_0042376e(puVar2,iVar1);
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_004239de(param_1: i32)

{
    let mut uVar1: u32;
    let mut local_30: i32;

    if (((param_1 + 0x41) == -0x1) || ((param_1 + 0x20) == (ushort)*(param_1 + 0x41))) {
        if (((param_1 + 0x20) == _DAT_004bd714) || (uVar1 = FUN_004a2edc(), uVar1 % 0x3 != 0x0)) {
            for (local_30 = 0x0; local_30 < 0x1f4; local_30 = local_30 + 0x1) {
                uVar1 = FUN_004a2edc();
                (param_1 + 0x41) = (char)(uVar1 % 0x28);
                if (((&DAT_004c7108)[*(param_1 + 0x41) * 0x4] & 0x10) != 0x0) {
                    return;
                }
            }
        }
        (param_1 + 0x41) = DAT_004bd714;
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00423ac1() -> *mut u32

{
    let mut iVar1: i32;
    let puVar2: *mut u32;
    let mut local_40: i32;
    let local_30: *mut u32;
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_24: i32;
    let local_20: *mut u32;
    let mut local_1c: i32;
    let local_18: *mut u32;

    if (_DAT_004bd714 == -0x1) {
        for (local_24 = 0x0; local_24 < 0x28; local_24 = local_24 + 0x1) {
            if ((*(&DAT_005b70c2 + local_24 * 0x4e) != 0x0) &&
                (iVar1 = FUN_004a2f10(&DAT_005b709e + local_24 * 0x4e,s_LEAGUEHEIM_004c1224), iVar1 == 0x0)) {
                _DAT_004bd714 = local_24;
            }
        }
    }
    local_1c = 0x0;
    for (local_20 = *DAT_005967b0; local_20 != 0x0; local_20 = *local_20) {
        if ((local_20 + 0x12) == '\x10') {
            local_1c = local_1c + 0x1;
        }
    }
    local_18 = 0x0;
    puVar2 = local_20;
    loop {
    if (0x1 < local_18) {
        return puVar2;
    }
    if (local_18 == 0x0) {
        local_28 = 0x15;
    }
    else {
        local_28 = 0x14;
    }
    local_2c = local_28;
    if (local_1c < 0xa) {
        FUN_004840cd(&local_30,&local_20,-0x1);
        while (local_20 != 0x0) {
            if (((local_20[0x9] >> 0x18 == local_2c) && ((local_20 + 0x26) == '\x05')) &&
                ((local_20 + 0x12) == '\0')) {
                for (local_40 = 0x0;
                    local_40 <
                    *(*(&DAT_00582938 +
                        (*(local_20 + 0x25) >> 0x18) * 0x4 + (local_20[0x9] >> 0x18) * 0x18) +
                        0xa5); local_40 = local_40 + 0x1) {
                    if (local_20[local_40 + 0x4] != 0x0)^ // goto LAB_00423bab;
                }
                FUN_00431d0a(&DAT_005967b8);
                DAT_005967bc = FUN_00434de1(local_20);
                if ((*(local_20 + 0x3a) & 0x1) != 0x0) {
                    FUN_004579ea(&DAT_005967b8);
                }
                if ((*(local_20 + 0x3a) & 0x1) == 0x0) {
                    (local_20 + 0x12) = 0x10;
                    (local_20 + 0x41) = 0xff;
                    local_1c = local_1c + 0x1;
                }
                if (0x9 < local_1c) break;
            }
            LAB_00423bab:
                local_20 = local_30;
            if (local_30 != 0x0) {
                local_30 = *local_30;
            }
        }
        FUN_0048418d(&local_30);
    }
    puVar2 = local_18;
    local_18 = (local_18 + 0x1);
} while( true );
}



fn FUN_00423cfb(param_1: i32)

{
    let mut pcVar1: String;
    let local_30: u8 [0x20];

    FUN_004953d7();
    FUN_00497567(0x3c,0xac,(&DAT_005b709e + param_1 * 0x4e),0x5a,0xcaccce,0xe0e0e,0xcaccce,DAT_004d6a6c,0x11);
    pcVar1 = FUN_00499050(DAT_0059679c,0x7348);
    FUN_0049c2e0(local_30,pcVar1);
    FUN_00497567(0x3c,0xb8,local_30,0x5a,0xcaccce,0xe0e0e,0xcaccce,DAT_004d6a6c,0x11);
    FUN_0049c2e0(local_30,&DAT_004c122f);
    FUN_00497567(0x3c,0xc4,local_30,0x5a,0xcaccce,0xe0e0e,0xcaccce,DAT_004d6a6c,0x11);
    FUN_0049536f();
    return;
}



fn FUN_00423df5()

{
    let mut pcVar1: String;
    let local_90: u8 [0x80];

    FUN_004953d7();
    if (DAT_004c9754 < 0x5) {
        FUN_00499050(DAT_0059679c,DAT_004c9754 + 0x414);
        pcVar1 = FUN_00499050(DAT_0059679c,0x73cc);
        FUN_0049c2e0(local_90,pcVar1);
    }
    else {
        FUN_0049c2e0(local_90,&DAT_00569b50 + DAT_004c9754 * 0x1e22);
    }
    FUN_004968e7(0xc8,0x1c7,0x168,0x12,0xe);
    FUN_0049e640(0xc8,0x1c7,0x168,0x12,0xce,0xca,0xcc,0x1);
    FUN_00497567(0x17c,0x1ca,local_90,0x168,0xa0a0a,0xe0e0e,0xa0a0a,LPCSTR_005b9218,0x11);
    FUN_0049536f();
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00423f15()

{
    let mut uVar1: u32;
    let piVar2: *mut i32;;
    let mut pcVar3: String;
    let local_9c: *mut i32;;
    let local_98: *mut i32;;
    let local_94: *mut i32;;
    let local_90: *mut i32;;
    let local_88: *mut u32;
    let local_84: u8 [0x40];
    let local_44: *mut i32;;
    let local_40: *mut u32;
    let local_3c: *mut u32;
    i32 **local_38;
    i32 **local_34;
    let local_30: *mut i32;;
    let local_2c: *mut i32;;
    let local_28: *mut i32;;
    let local_24: *mut i32;;
    let local_20: *mut i32;;
    let local_1c: *mut i32;;
    let local_18: *mut i32;;

    if (_DAT_004c935c == 0x0) {
        _DAT_004c935c = 0x1;
        _DAT_004c9364 = 0x1;
        _DAT_004c9368 = 0x0;
        local_88 = FUN_004a2831(0x49);
        local_40 = local_88;
        local_3c = local_88;
        if (local_88 != 0x0) {
            local_88 = FUN_0049a030(local_88,0x0,0x1,0x0,0x0,0x280,0x1e0,0x4000,0x0,0x0);
        }
        DAT_004c9360 = local_88;
        FUN_0049ae00(local_88,FUN_0042444e);
        local_38 = (i32 **)FUN_004a2831(0x10);
        local_34 = local_38;
        if (local_38 != (i32 **)0x0) {
            FUN_004a2874(local_38,&DAT_004c9360,0x96);
        }
        FUN_0049a8a0(DAT_004c9360);
        local_90 = FUN_004a2831(0x95);
        local_44 = local_90;
        local_30 = local_90;
        if (local_90 != 0x0) {
            local_90 = FUN_0047157e(local_90,&DAT_004c9360,0x1f5,0xa,0x3,0x64,0x60,0x40,0x2);
        }
        DAT_005b8854 = local_90;
        local_94 = FUN_004a2831(0xb9);
        local_28 = local_94;
        local_24 = local_94;
        if (local_94 != 0x0) {
            local_94 = FUN_00438792(local_94,&DAT_004c9360,0x1f4,0x10,0x67,0x58,0x3f,0x4042,0x2,0x2,DAT_004d557c);
        }
        DAT_0059a1c4 = local_94;
        local_98 = FUN_004a2831(0xb9);
        local_20 = local_98;
        local_1c = local_98;
        if (local_98 != 0x0) {
            local_98 = FUN_00438792(local_98,&DAT_004c9360,0x4b0,0x7c,0xc,0x1f8,0x1a4,0x4009,0x1c,0x28,DAT_004d557c);
        }
        DAT_0059a1c0 = local_98;
        local_9c = FUN_004a2831(0x95);
        local_2c = local_9c;
        local_18 = local_9c;
        if (local_9c != 0x0) {
            local_9c = FUN_0047157e(local_9c,&DAT_004c9360,0x4b1,0x7c,0xc,0x1f8,0x1a4,0x4001,0x22);
        }
        DAT_005b8850 = local_9c;
        FUN_0047d6a0(local_9c,*(&DAT_005b7076 + DAT_004d557c * 0x4e) >> 0x10,
                     *(&DAT_005b7078 + DAT_004d557c * 0x4e) >> 0x10,0x0);
        piVar2 = DAT_005b8854;
        uVar1 = *(DAT_005b8850 + 0x4d);
        *(DAT_005b8854 + 0x59) = *(DAT_005b8850 + 0x49);
        *(piVar2 + 0x5d) = uVar1;
        FUN_00471bcd(DAT_005b8854);
        _DAT_004c936c = 0x0;
        ((*(DAT_004c9360 + 0x45) + 0xc))(DAT_004c9360,DAT_004c9360,0x414,0x1,0x0);
        FUN_004a2965(&DAT_004c9360);
    }
    else {
        if (DAT_004c9754 == 0x0) {
            FUN_004953d7();
            if (hide_ai_opt_004d55a4 == 0x0) {
                pcVar3 = FUN_00499050(DAT_0059679c,0x7348);
                FUN_0049c2e0(local_84,pcVar3);
                FUN_00497567(0x3c,0xb8,local_84,0x5a,0xcaccce,0xe0e0e,0xcaccce,DAT_004d6a6c,0x11);
            }
            else {
                FUN_0042444e(DAT_004c9360,0x405,0x0,0x0);
            }
            FUN_0049536f();
        }
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0042433b()

{
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut uVar3: u32;

    if (_DAT_004c935c != 0x0) {
        _DAT_004c935c = 0x0;
        FUN_0049ab50(DAT_004c9360,0x0);
        if (DAT_005b8854 == 0x0) {
            uVar1 = 0x0;
        }
        else {
            uVar1 = ((*(DAT_005b8854 + 0x45) + 0x8))(DAT_005b8854,0x2);
        }
        if (DAT_005b8850 == 0x0) {
            uVar2 = 0x0;
        }
        else {
            uVar2 = ((*(DAT_005b8850 + 0x45) + 0x8))(DAT_005b8850,0x2,uVar1);
        }
        if (DAT_0059a1c0 == 0x0) {
            uVar3 = 0x0;
        }
        else {
            uVar3 = ((*(DAT_0059a1c0 + 0x45) + 0x8))(DAT_0059a1c0,0x2,uVar1,uVar2);
        }
        if (DAT_0059a1c4 != 0x0) {
            ((*(DAT_0059a1c4 + 0x45) + 0x8))(DAT_0059a1c4,0x2,uVar1,uVar2,uVar3);
        }
        if (DAT_004c9360 != 0x0) {
            ((*(DAT_004c9360 + 0x45) + 0x8))(DAT_004c9360,0x2);
        }
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0042444e(param_1: i32,param_2: u32,param_3: u32,param_4: u32) -> u32

{
    let mut iVar1: i32;
    let mut uVar2: u32;

    if (0x112 < param_2) {
        if (param_2 < 0x114) {
            FUN_00496263((&DAT_004bf7e0 + DAT_005b8848 * 0xc),0xa,0x4);
            iVar1 = DAT_005b8848;
            DAT_005b8848 = DAT_005b8848 + 0x1;
            if (iVar1 == 0x8) {
                DAT_005b8848 = 0x0;
            }
            return 0x1;
        }
        if (param_2 == 0x405) {
            FUN_004953d7();
            FUN_004a08c5(s_pcx_aiplat_pcx_004c1231,0x0,0x0,0x280,0x1e0,0x0,0x0,0x0,0x0);
            if ((_DAT_004c9364 != 0x0) || (_DAT_004c9368 != 0x0)) {
                FUN_00471bcd(DAT_005b8854);
                FUN_00439d27(DAT_0059a1c4,0x0);
                FUN_00423cfb(*(DAT_0059a1c0 + 0xa9));
            }
            if (_DAT_004c9364 == 0x0) {
                if (_DAT_004c9368 != 0x0) {
                    FUN_00439d27(DAT_0059a1c0,0x0);
                    _DAT_004c936c = 0x1;
                }
            }
            else {
                FUN_00471bcd(DAT_005b8850);
                _DAT_004c936c = 0x0;
            }
            FUN_00423df5();
            FUN_0049536f();
            return 0x1;
        }
    }
    uVar2 = ((*(param_1 + 0x45) + 0xc))(param_1,param_1,param_2,param_3,param_4);
    return uVar2;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_004245ce(param_1: i32)

{
    let mut iVar1: i32;

    iVar1 = *(DAT_0059a1c0 + 0xa9);
    FUN_004390ae(DAT_0059a1c0,(param_1 + 0x20));
    FUN_004390ae(DAT_0059a1c4,(param_1 + 0x20));
    DAT_005b7068 = (param_1 + 0x20);
    DAT_005b8808 = 0x0;
    if ((*(DAT_0059a1c0 + 0xa9) != iVar1) || (_DAT_004c936c != 0x1)) {
        FUN_0044783a(DAT_0059a1c0,(param_1 + 0x22),(param_1 + 0x24),0x1);
        FUN_00439d27(DAT_0059a1c4,0x0);
        FUN_00423cfb(*(DAT_0059a1c0 + 0xa9));
    }
    FUN_00447607(DAT_0059a1c0,(param_1 + 0x22),(param_1 + 0x24));
    iVar1 = (*(DAT_0059a1c0 + 0x89) + 0x3) - *(DAT_0059a1c0 + 0x21);
    if ((((iVar1 < 0x0) || (*(DAT_0059a1c0 + 0x29) <= iVar1 + 0x22)) ||
        (*(DAT_0059a1c0 + 0x25) <= (*(DAT_0059a1c0 + 0x85) - *(DAT_0059a1c0 + 0x1d)) + 0x1e)) &&
        (iVar1 = FUN_0044783a(DAT_0059a1c0,(param_1 + 0x22),(param_1 + 0x24),0x1),
         iVar1 != 0x0)) {
        FUN_00439d27(DAT_0059a1c4,0x0);
    }
    _DAT_004c9364 = 0x0;
    _DAT_004c9368 = 0x1;
    _DAT_004c936c = 0x1;
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_004248db()

{
    let sVar1: i16;
    let sVar2: i16;
    let mut iVar3: i32;
    let mut uVar4: u32;
    let mut local_d0: i32;
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: u32;

    if (_DAT_004c975c != 0x0) {
        FUN_004245ce(DAT_005967bc);
    }
    while ((((DAT_005967bc + 0x22) != (ushort)*(DAT_005967bc + 0x41) ||
        ((DAT_005967bc + 0x24) != (ushort)*(DAT_005967bc + 0x42))) &&
        (iVar3 = FUN_00432bd3(&DAT_005967b8), iVar3 != 0x0))) {
        FUN_0042feac();
        sVar1 = (DAT_005967bc + 0x22);
        sVar2 = (DAT_005967bc + 0x24);
        uVar4 = FUN_0045af67(&DAT_005967b8,(DAT_005967bc + 0x22),(DAT_005967bc + 0x24),
                             *(DAT_005967bc + 0x41),*(DAT_005967bc + 0x42),0x0);
        if (uVar4 == 0x0) break;
        if (_DAT_004c975c != 0x0) {
            FUN_0043a597(DAT_0059a1c0);
            FUN_00449654(DAT_0059a1c0,uVar4,(DAT_005967bc + 0x22),(DAT_005967bc + 0x24),
                         *(DAT_005967bc + 0x41),*(DAT_005967bc + 0x42));
        }
        FUN_0045b45b(&DAT_005967b8,uVar4,(DAT_005967bc + 0x22),(DAT_005967bc + 0x24),
                     &local_1c,&local_18);
        if ((local_1c == 0xffffffff) || (local_18 == 0xffffffff)) break;
        DAT_005967b8 = DAT_005967b8 | 0x1;
        if (_DAT_004c975c == 0x0) {
            local_d0 = FUN_00444385(local_1c,local_18,DAT_0059a1c0,DAT_0059a1c4);
        }
        else {
            local_d0 = FUN_00444385(local_1c,local_18,0x0,0x0);
        }
        if ((local_d0 == 0x0) || (local_d0 == -0x1)) break;
        if (_DAT_004c975c != 0x0) {
            FUN_0043a213(DAT_0059a1c0,sVar1,sVar2,(DAT_005967bc + 0x22),
                         (DAT_005967bc + 0x24));
            FUN_0043a3d6(DAT_0059a1c0,0x1);
            FUN_0043a3d6(DAT_0059a1c4,0x1);
        }
    }
    DAT_005967b8 = DAT_005967b8 & 0xfffffffe;
    local_14 = DAT_00599d3c;
    DAT_00599d3c = 0x0;
    FUN_004864f7();
    DAT_00599d3c = local_14;
    FUN_0042feac();
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00424d33(param_1: i32)

{
    let mut iVar1: i32;

    DAT_005b7068 = (param_1 + 0x20);
    DAT_005b8808 = 0x0;
    if (_DAT_004c936c != 0x0) {
        iVar1 = FUN_0047d6a0(DAT_005b8850,(param_1 + 0x22),(param_1 + 0x24),0x1);
        if (iVar1 == 0x0) {
            FUN_00471bcd(DAT_005b8850);
        }
    }
    FUN_00477d4b((param_1 + 0x22),(param_1 + 0x24),-0x1);
    _DAT_004c9364 = 0x1;
    _DAT_004c9368 = 0x0;
    _DAT_004c936c = 0x0;
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00424e53(param_1: i32,param_2: i32)

{
    let mut uVar1: u32;
    let mut iVar2: i32;

    DAT_005967bc = param_1;
    (param_1 + 0x41) = (&DAT_005b7078)[param_2 * 0x4e];
    (param_1 + 0x42) = (&DAT_005b707a)[param_2 * 0x4e];
    if (_DAT_004c975c != 0x0) {
        FUN_00424d33(param_1);
        timer_func_0049e710(0x29a);
    }
    DAT_005967b8 = DAT_005967b8 | 0x1;
    FUN_0047722f(DAT_005b8850,DAT_005b8854);
    if (_DAT_004c975c != 0x0) {
        DAT_005b7068 = (param_1 + 0x20);
        DAT_005b8808 = 0x0;
        iVar2 = FUN_0047d6a0(DAT_005b8850,(param_1 + 0x22),(param_1 + 0x24),0x1);
        if (iVar2 == 0x0) {
            FUN_00471bcd(DAT_005b8850);
        }
        FUN_00471bcd(DAT_005b8854);
        timer_func_0049e710(0x29a);
    }
    uVar1 = DAT_00599d3c;
    DAT_005967b8 = DAT_005967b8 & 0xfffffffe;
    DAT_00599d3c = 0x0;
    FUN_004864f7();
    DAT_00599d3c = uVar1;
    FUN_0042feac();
    return;
}



fn FUN_00424fe4()

{
    let mut iVar1: i32;
    let mut uVar2: u32;
    i32 local_1e4 [0x72];
    let local_1c: *mut u32;
    let mut local_18: i32;
    let mut local_14: i32;

    if ((DAT_004c9754 < 0x5) && (iVar1 = FUN_0044e131(), iVar1 == 0x0)) {
        local_18 = -0x1;
        *(&DAT_00569ab1 + DAT_004c9754 * 0x1e22) = 0x0;
        for (local_1c = *DAT_005967c8; local_1c != 0x0; local_1c = *local_1c) {
            if (((local_1c + 0xe) == 0x17) && (*(local_1c + 0xe) >> 0x10 == DAT_004c9754)) {
                *(&DAT_00569ab1 + DAT_004c9754 * 0x1e22) = *(&DAT_00569ab1 + DAT_004c9754 * 0x1e22) + 0x1;
            }
        }
        FUN_004a0430(local_1e4,0x0,0x1c8);
        if (((&DAT_00569f00)[DAT_004c9754 * 0x1e22] & 0x1) == 0x0) {
            if (((&DAT_00569f00)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
                FUN_00455070(0x50,local_1e4,0x0);
                for (local_14 = 0x1; local_14 < 0x72; local_14 = local_14 + 0x1) {
                    if (((local_1e4[local_14] != 0x0) && (((&DAT_00569c30)[local_14 * 0x9 + DAT_004c9754 * 0x1e22] & 0x2) != 0x0))
                        && ((*(&DAT_0058ad72 + local_14 * 0xda) == 0x0 ||
                        (*(&DAT_00569ab1 + DAT_004c9754 * 0x1e22) != 0x1)))) {
                        local_18 = local_14;
                        break;
                    }
                }
            }
            else {
                local_18 = 0x50;
            }
        }
        if (local_18 == -0x1) {
            uVar2 = FUN_004a2edc();
            iVar1 = uVar2 % 0x64;
            if (iVar1 < *(&DAT_004bd74c + DAT_004c9754 * 0x10)) {
                local_18 = FUN_0042530b(0x1);
            }
            else {
                if (iVar1 < *(&DAT_004bd750 + DAT_004c9754 * 0x10)) {
                    local_18 = FUN_0042530b(0x19);
                }
                else {
                    if (iVar1 < *(&DAT_004bd754 + DAT_004c9754 * 0x10)) {
                        local_18 = FUN_0042530b(0x37);
                    }
                    else {
                        if (iVar1 < *(&DAT_004bd758 + DAT_004c9754 * 0x10)) {
                            local_18 = FUN_0042530b(0x44);
                        }
                    }
                }
            }
            if (local_18 == -0x1) {
                local_18 = FUN_0042530b(0xffffffff);
            }
        }
        if (local_18 != -0x1) {
            for (local_1c = *DAT_005967c8; local_1c != 0x0; local_1c = *local_1c) {
                if (((local_1c + 0xe) == 0x17) && (*(local_1c + 0xe) >> 0x10 == DAT_004c9754)) {
                    (&DAT_00569c30)[local_18 * 0x9 + DAT_004c9754 * 0x1e22] =
                        (&DAT_00569c30)[local_18 * 0x9 + DAT_004c9754 * 0x1e22] | 0x4;
                    *(&DAT_00569c35 + local_18 * 0x9 + DAT_004c9754 * 0x1e22) =
                        *(&DAT_00569c35 + local_18 * 0x9 + DAT_004c9754 * 0x1e22) + 0x1;
                    if (*(&DAT_00569c31 + local_18 * 0x9 + DAT_004c9754 * 0x1e22) == 0x0) {
                        *(&DAT_00569c31 + local_18 * 0x9 + DAT_004c9754 * 0x1e22) =
                            *(&DAT_0058ad6a + local_18 * 0xda);
                    }
                    *(&DAT_00569c31 + DAT_004c9754 * 0x1e22 + local_18 * 0x9) =
                        *(&DAT_00569c31 + DAT_004c9754 * 0x1e22 + local_18 * 0x9) - (local_1c[0x6] >> 0x10);
                    *(local_1c + 0x1a) = 0x0;
                    (local_1c + 0x26) = local_18;
                }
            }
        }
    }
    return;
}



fn FUN_0042530b(param_1: u32) -> i32

{
let mut uVar1: u32;
let mut local_20: i32;
let mut local_1c: i32;
let mut local_18: i32;

local_20 = -0x1;
if (param_1 < 0x19) {
if (param_1 == 0x1) {
local_1c = 0x18;^
// goto LAB_0042537d;
}
}
else {
if (param_1 < 0x1a) {
local_1c = 0x1e;^
// goto LAB_0042537d;
}
if (0x36 < param_1) {
if (param_1 < 0x38) {
local_1c = 0xd;^
// goto LAB_0042537d;
}
if (param_1 == 0x44) {
local_1c = 0x2e;^
// goto LAB_0042537d;
}
}
}
local_1c = 0x71;
// LAB_0042537d:
local_18 = local_1c;
loop {
if (local_18 == 0x0) {
return local_20;
}
uVar1 = FUN_004a2edc();
local_20 = uVar1 % local_1c;
if (param_1 < 0x19) {
if (param_1 != 0x1)^ // goto LAB_004253be;
local_20 = local_20 + 0x1;
}
else {
if (param_1 < 0x1a) {
local_20 = local_20 + 0x19;
}
else {
if (param_1 < 0x37) {
// LAB_004253be:
local_20 = local_20 + 0x1;
}
else {
if (param_1 < 0x38) {
local_20 = local_20 + 0x37;
}
else {
if (param_1 != 0x44)^ // goto LAB_004253be;
local_20 = local_20 + 0x44;
}
}
}
}
if (((&DAT_00569c30)[local_20 * 0x9 + DAT_004c9754 * 0x1e22] & 0x2) != 0x0) {
if (*(&DAT_0058ad72 + local_20 * 0xda) == 0x0) {
return local_20;
}
if (0x1 < *(&DAT_00569ab1 + DAT_004c9754 * 0x1e22)) {
return local_20;
}
}
if (local_1c != 0x71) {
local_18 = local_18 + -0x1;
}
} while( true );
}



fn FUN_00425463()

{
    let mut bVar1: bool;
    let local_20: *mut u32;
    let mut local_1c: i32;
    let mut local_14: i32;

    local_1c = 0x0;
    bVar1 = false;
    local_14 = 0x1;
    loop {
    if (0x71 < local_14) {
        LAB_004254ca:
        if (bVar1) {
            for (local_20 = *DAT_005967c8; local_20 != 0x0; local_20 = *local_20)
            {
                if ((((local_20 + 0xe) == 0x17) && (*(local_20 + 0xe) >> 0x10 == DAT_004c9754)) &&
                    (local_1c = local_1c + 0x1, 0x1 < local_1c)) {
                    return;
                }
            }
            if (local_1c != 0x0) {
                for (local_14 = 0x1; local_14 < 0x72; local_14 = local_14 + 0x1) {
                    if ((((&DAT_00569c30)[local_14 * 0x9 + DAT_004c9754 * 0x1e22] & 0x1) != 0x0) &&
                        (*(&DAT_0058ad72 + local_14 * 0xda) != 0x0)) {
                        (&DAT_00569c30)[DAT_004c9754 * 0x1e22 + local_14 * 0x9] =
                            (&DAT_00569c30)[DAT_004c9754 * 0x1e22 + local_14 * 0x9] & 0xfe;
                        (&DAT_00569c30)[DAT_004c9754 * 0x1e22 + local_14 * 0x9] =
                            (&DAT_00569c30)[DAT_004c9754 * 0x1e22 + local_14 * 0x9] | 0x2;
                        (&DAT_00569c30)[DAT_004c9754 * 0x1e22] = (&DAT_00569c30)[DAT_004c9754 * 0x1e22] & 0xf7;
                    }
                }
                FUN_00463f7b();
            }
        }
        return;
    }
    if ((((&DAT_00569c30)[local_14 * 0x9 + DAT_004c9754 * 0x1e22] & 0x1) != 0x0) &&
        (*(&DAT_0058ad72 + local_14 * 0xda) != 0x0)) {
        bVar1 = true;^
        // goto LAB_004254ca;
    }
    local_14 = local_14 + 0x1;
} while( true );
}



fn FUN_004255c0(param_1: i32) -> i32

{
i32 aiStack60 [0x5];
let mut local_28: i32;
let mut local_24: i32;
let mut local_20: i32;
let mut local_1c: i32;
let mut local_18: i32;
let mut local_14: i32;

if (*(&DAT_00569ac5 + param_1 * 0x1e22) == -0x1) {
if (param_1 < 0x5) {
local_28 = param_1;
}
else {
for (local_24 = 0x0; local_24 < 0x5; local_24 = local_24 + 0x1) {
if (((&DAT_00569a98)[local_24 * 0x1e22] & 0x1) == 0x0) {
if ((&DAT_004d55a8)[param_1 * 0xe + local_24] == '\x02') {
aiStack60[local_24] = 0x1;
}
else {
aiStack60[local_24] = 0x0;
}
}
else {
aiStack60[local_24] = 0x1;
}
}
local_20 = 0x0;
local_1c = -0x1;
if (param_1 == 0x6) {
for (local_24 = 0x0; local_24 < 0x5; local_24 = local_24 + 0x1) {
if ((aiStack60[local_24] == 0x0) && (((&DAT_00569a98)[local_24 * 0x1e22] & 0x20) != 0x0)) {
local_1c = local_24;
local_20 = local_20 + 0x1;
}
}
if (local_20 == 0x1) {
return local_1c;
}
}
for (local_24 = 0x0; local_24 < 0x5; local_24 = local_24 + 0x1) {
if ((aiStack60[local_24] == 0x0) && ((&DAT_004d55a8)[param_1 * 0xe + local_24] == '\x01')) {
local_1c = local_24;
local_20 = local_20 + 0x1;
}
}
if (local_20 == 0x1) {
local_28 = local_1c;
}
else {
local_18 = -0x1;
local_14 = -0x1;
for (local_24 = 0x0; local_24 < 0x5; local_24 = local_24 + 0x1) {
if ((aiStack60[local_24] == 0x0) && (local_14 < *(&DAT_00569b14 + local_24 * 0x4 + param_1 * 0x1e22)))
{
local_18 = local_24;
local_14 = *(&DAT_00569b14 + local_24 * 0x4 + param_1 * 0x1e22);
}
}
local_28 = local_18;
}
}
}
else {
local_28 = *(&DAT_00569ac5 + param_1 * 0x1e22);
}
return local_28;
}



fn FUN_004257de() -> *mut u32

{
    let puVar1: *mut u32;
    let puVar2: *mut u32;
    let puVar3: *mut u32;
    let mut iVar4: i32;
    let mut uVar5: u32;
    i32 aiStackY572 [0x4e];
    let local_d0: *mut u32;
    let mut local_cc: i32;
    i32 aiStack60 [0x5];
    let mut local_28: i32;
    let local_24: *mut u32;
    let local_20: *mut u32;
    let local_1c: *mut u32;
    let local_18: *mut u32;
    let mut local_14: i32;

    puVar3 = DAT_004c9754;
    if ((DAT_004c9754 == DAT_004c976c) &&
        (puVar3 = (DAT_004c9754 * 0x1e22),
         (*((DAT_004c9754 * 0x1e22) + 0x15a6a6) & 0x4) != 0x0)) {
        local_28 = 0x0;
        puVar3 = *DAT_005967b0;
        local_24 = puVar3;
        while (puVar3 != 0x0) {
            if (((puVar3 + 0x27) == '\x1c') &&
                ((*(puVar3 + 0x23) >> 0x18) == DAT_004c9754)) {
                local_28 = local_28 + 0x1;
            }
            puVar3 = *puVar3;
            local_24 = puVar3;
        }
        local_24 = puVar3;
        if (0xe < local_28) {
            puVar3 = FUN_0040705e(DAT_004c9754);
        }
        for (local_20 = 0x0; local_20 < 0x5; local_20 = (local_20 + 0x1)) {
            if (((&DAT_00569a98)[local_20 * 0x1e22] & 0x1) == 0x0) {
                if (DAT_004c9754 == local_20) {
                    aiStack60[local_20] = 0x1;
                }
                else {
                    iVar4 = FUN_00406cc8(local_20);
                    if (iVar4 == 0x0) {
                        aiStack60[local_20] = 0x1;
                    }
                    else {
                        aiStack60[local_20] = 0x0;
                    }
                }
            }
            else {
                aiStack60[local_20] = 0x1;
            }
            puVar3 = local_20;
        }
        for (local_20 = 0x0; puVar1 = local_20, local_20 < 0x3;
            local_20 = (local_20 + 0x1)) {
            if ((&DAT_00569ad1)[DAT_004c9754 * 0x1e22 + local_20] == -0x1) {
                (&DAT_004c5048)[local_20 * 0x2] = 0xffffffff;
            }
            else {
                if (aiStack60[*(&DAT_00569ace + DAT_004c9754 * 0x1e22 + local_20) >> 0x18] == 0x0) {
                    (&DAT_004c5048)[local_20 * 0x2] =
                        (*(&DAT_00569ace + DAT_004c9754 * 0x1e22 + local_20) >> 0x18);
                }
                else {
                    (&DAT_004c5048)[local_20 * 0x2] = 0xffffffff;
                }
            }
            puVar3 = puVar1;
        }
        local_18 = 0x0;
        for (local_20 = 0x0; local_20 < 0x3; local_20 = (local_20 + 0x1)) {
            if ((&DAT_004c5048)[local_20 * 0x2] == 0xffffffff) {
                local_18 = (local_18 + 0x1);
            }
            puVar3 = local_20;
        }
        if (local_18 != 0x0) {
            loop {
                uVar5 = FUN_004a2edc();
                puVar3 = local_18;
                local_14 = uVar5 % 0x3;
            } while ((&DAT_004c5048)[local_14 * 0x2] != 0xffffffff);
            (&DAT_004c5048)[local_14 * 0x2] = DAT_004c9754;
            local_18 = (local_18 + -0x1);
            if (local_18 != 0x0) {
                for (local_20 = 0x0; puVar2 = local_18, puVar1 = local_20, local_20 < 0x5;
                    local_20 = (local_20 + 0x1)) {
                    if (aiStack60[local_20] == 0x0) {
                        puVar3 = (byte)(&DAT_004d55a8)[DAT_004c9754 * 0xe + local_20];
                        if ((byte)(&DAT_004d55a8)[DAT_004c9754 * 0xe + local_20] == 0x1)
                        {
                            puVar3 = puVar2;
                            if (DAT_004c5050 == 0xffffffff) {
                                DAT_004c5050 = local_20;
                                aiStack60[local_20] = 0x1;
                                local_18 = (local_18 + -0x1);
                            }
                            else {
                                if (DAT_004c5058 == 0xffffffff) {
                                    DAT_004c5058 = local_20;
                                    aiStack60[local_20] = 0x1;
                                    local_18 = (local_18 + -0x1);
                                }
                                else {
                                    puVar3 = DAT_004c5048;
                                    if (DAT_004c5048 == 0xffffffff) {
                                        DAT_004c5048 = local_20;
                                        aiStack60[local_20] = 0x1;
                                        local_18 = (local_18 + -0x1);
                                        puVar3 = puVar2;
                                    }
                                }
                            }
                        }
                        if (local_18 == 0x0)^ // goto LAB_00425ded;
                    }
                    puVar3 = puVar1;
                }
                if (local_18 != 0x0) {
                    for (local_1c = 0x0; puVar3 = local_1c, local_1c < local_18;
                        local_1c = (local_1c + 0x1)) {
                        local_d0 = 0xffffffff;
                        local_cc = -0x1;
                        for (local_20 = 0x0; puVar1 = local_20, local_20 < 0x5;
                            local_20 = (local_20 + 0x1)) {
                            if ((aiStack60[local_20] == 0x0) &&
                                (local_cc < *(&DAT_00569b14 + local_20 * 0x4 + DAT_004c9754 * 0x1e22))) {
                                local_d0 = local_20;
                                local_cc = *(&DAT_00569b14 + local_20 * 0x4 + DAT_004c9754 * 0x1e22);
                            }
                            puVar3 = puVar1;
                        }
                        if (local_d0 == 0xffffffff) break;
                        if (DAT_004c5050 == 0xffffffff) {
                            DAT_004c5050 = local_d0;
                            aiStack60[local_d0] = 0x1;
                        }
                        else {
                            if (DAT_004c5058 == 0xffffffff) {
                                DAT_004c5058 = local_d0;
                                aiStack60[local_d0] = 0x1;
                            }
                            else {
                                if (DAT_004c5048 == 0xffffffff) {
                                    DAT_004c5048 = local_d0;
                                    aiStack60[local_d0] = 0x1;
                                }
                            }
                        }
                    }
                }
            }
        }
        LAB_00425ded:
        for (local_20 = 0x0; local_20 < 0x3; local_20 = (local_20 + 0x1)) {
            FUN_00405d5a(local_20);
            puVar3 = local_20;
        }
    }
    return puVar3;
}



fn FUN_00425e18(param_1: i32,param_2: *mut i32,param_3: *mut i32)

{
    let mut local_48: u32;
    let mut local_44: i32;
    let local_40: *mut u32;
    let mut local_3c: i32;
    let mut local_38: i32;
    let mut local_34: i32;
    let mut local_30: i32;
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: i32;
    ushort *local_1c;
    ushort *local_18;
    let mut local_14: u32;

    local_2c = param_1 + 0x20;
    if (((param_1 + 0x35) == -0x1) || ((param_1 + 0x26) == (param_1 + 0x35))) {
        local_20 = 0x0;
    }
    else {
        local_20 = 0x1;
    }
    local_24 = local_20;
    local_34 = local_20;
    local_1c = (param_1 + 0x20);
    local_14 = local_1c & 0xffff0000 | *local_1c;
    local_30 = *local_1c;
    *param_2 = -0x1;
    *param_3 = -0x1;
    local_28 = local_2c;
    local_18 = local_1c;
    FUN_00431d31(&local_48);
    local_44 = FUN_00434de1(param_1);
    local_3c = FUN_00432c94(&local_48);
    local_40 = (&DAT_005b89f8 + local_30 * 0x4);
    loop {
    if (local_40 == 0x0) {
        return;
    }
    if (*(local_40 + 0x6) >> 0x10 != local_30) {
        return;
    }
    if (*(local_40 + 0xe) >> 0x10 == *(param_1 + 0x23) >> 0x18) {
        if ((local_40[0xb] & 0x100) == 0x0) {
            if (local_34 == 0x0)^ // goto LAB_00425f5b;
        }
        else {
            if ((local_34 != 0x0) && (*(param_1 + 0x32) >> 0x18 == local_40[0x4] >> 0x10)) {
                LAB_00425f5b:
                    local_38 = FUN_00485ea2(local_30,local_40[0x2] >> 0x10,*(local_40 + 0xa) >> 0x10,0x1);
                if (local_3c + local_38 < 0x15) {
                    *param_2 = local_40[0x2] >> 0x10;
                    *param_3 = *(local_40 + 0xa) >> 0x10;
                    return;
                }
            }
        }
    }
    local_40 = *local_40;
} while( true );
}



fn FUN_00425fb6(param_1: i32,param_2: *mut u32,param_3: *mut u32) -> u32

{
    let mut bVar1: bool;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut local_78: i32;
    let mut local_58: u32;
    let mut local_54: u32;
    let mut local_50: u32;
    let mut local_4c: i32;
    let local_48: *mut u32;
    let local_44: *mut u32;
    let mut local_40: u32;
    let mut local_3c: i32;
    let mut local_38: i32;
    let mut local_34: i32;
    let mut local_30: u32;
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: i32;
    ushort *local_1c;
    ushort *local_18;
    let mut local_14: u32;

    local_44 = 0x0;
    local_2c = param_1 + 0x20;
    if (((param_1 + 0x35) == -0x1) || ((param_1 + 0x26) == (param_1 + 0x35))) {
        local_20 = 0x0;
    }
    else {
        local_20 = 0x1;
    }
    local_24 = local_20;
    local_38 = local_20;
    local_1c = (param_1 + 0x20);
    local_14 = local_1c & 0xffff0000 | *local_1c;
    local_34 = *local_1c;
    *param_2 = 0xffffffff;
    *param_3 = 0xffffffff;
    local_28 = local_2c;
    local_18 = local_1c;
    FUN_00431d31(&local_50);
    local_4c = FUN_00434de1(param_1);
    local_40 = FUN_00432c94(&local_50);
    local_30 = 0x0;
    uVar2 = local_40;
    loop {
    if (0x2b < local_30) {
        return uVar2;
    }
    local_54 = ((local_30 & 0x1) == 0x0);
    for (local_58 = local_54; local_58 < (0x41 - (local_30 & 0x1)); local_58 = local_58 + 0x2) {
        if ((*(*(&DAT_004d7d50 + local_30 * 0x4 + local_34 * 0x3890) + local_58 * 0x4 + 0x4) & 0x80) == 0x0
        ) {
            LAB_0042612f:
                local_3c = FUN_00485ea2(local_34,local_30,local_58,0x1);
            if ((local_40 + local_3c) < 0x15) {
                if (local_3c == 0x0) {
                    LAB_00426276:
                        *param_2 = local_30;
                    *param_3 = local_58;
                    return local_58;
                }
                local_44 = FUN_0045ad4e(local_34,local_30,local_58);
                if (local_38 == 0x0) {
                    if (((local_44 + 0x35) == -0x1) ||
                        ((local_44 + 0x26) == (local_44 + 0x35))) {
                        bVar1 = false;
                    }
                    else {
                        bVar1 = true;
                    }
                    if (!bVar1)^ // goto LAB_00426276;
                }
                else {
                    if (((local_44 + 0x35) == -0x1) ||
                        ((local_44 + 0x26) == (local_44 + 0x35))) {
                        bVar1 = false;
                    }
                    else {
                        bVar1 = true;
                    }
                    if (bVar1) {
                        if (((local_44 + 0x35) == -0x1) ||
                            ((local_44 + 0x26) == (local_44 + 0x35))) {
                            local_78 = 0x0;
                        }
                        else {
                            local_78 = 0x1;
                        }
                        if (local_78 == *(param_1 + 0x32) >> 0x18)^ // goto LAB_00426276;
                    }
                }
            }
        }
        else {
            local_48 = FUN_00481784(local_34,local_30,local_58);
            iVar3 = FUN_00481a44(local_48);
            if ((((iVar3 != 0x0) && ((local_48 + 0xe) != 0xf)) && ((local_48 + 0xe) != 0x10))
                && ((local_48 + 0xe) != 0x2))^ // goto LAB_0042612f;
        }
    }
    uVar2 = local_30;
    local_30 = local_30 + 0x1;
} while( true );
}



fn FUN_0042629a(param_1: i32)

{
    let cVar1: u8;
    let mut pcVar2: String;
    let mut pcVar3: String;
    let mut local_138: *mut u32 [0x11];
    let ppuStack243: *mut *mut u8;;
    let mut local_47: String;;
    let mut local_40: u32;
    let local_3c: *mut i32;;
    let local_38: *mut i32;;
    let local_34: *mut i32;;
    let local_30: *mut i32;;
    let local_2c: *mut i32;;
    let mut local_28: u32;
    let mut local_24: u32;
    u32 **local_20;
    let mut local_1c: u32;

    pcVar2 = &DAT_005b709e + param_1 * 0x4e;
    pcVar3 = &DAT_004c9378;
    loop {
    cVar1 = *pcVar2;
    *pcVar3 = cVar1;
    if (cVar1 == '\0') break;
    cVar1 = pcVar2[0x1];
    pcVar2 = pcVar2 + 0x2;
    pcVar3[0x1] = cVar1;
    pcVar3 = pcVar3 + 0x2;
} while (cVar1 != '\0');
    local_34 = FUN_004990e0(local_138,0x0,s_efs_res_004c1249,s_WorldDlg_004c1240);
    local_3c = FUN_004a2831(0xb9);
    local_38 = local_3c;
    local_30 = local_3c;
    if (local_3c != 0x0) {
        local_30 = FUN_00438792(local_3c,local_138,0x4b0,0x14,0x5,0x258,0x1c2,0x4015,0x1c,0x28,param_1);
    }
    DAT_004c9370 = local_30;
    local_2c = local_30;
    local_28 = 0x80;
    *(local_30 + 0x2d) = *(local_30 + 0x2d) & 0xffffff7f;
    local_40 = *(local_30 + 0x2d);
    FUN_0049bf40(local_138,DAT_004c9370);
    FUN_0049bb50(local_138,FUN_00426421);
    if (DAT_004c9370 == 0x0) {
        local_24 = 0x0;
    }
    else {
        local_24 = ((*(DAT_004c9370 + 0x45) + 0x8))(DAT_004c9370,0x2);
    }
    local_20 = local_138;
    local_1c = 0x0;
    ppuStack243 = &PTR_FUN_004c3d34;
    if (local_47 != 0x0) {
        FUN_00499b30(local_138,local_47);
    }
    FUN_0049a1c0(local_138,0x1);
    return;
}



fn FUN_00426421(param_1: i32,param_2: u32,param_3: i32,param_4: i32) -> u32

{
    let mut uVar1: u32;
    let mut iVar2: i32;

    if (param_2 < 0x405) {
        if ((((param_2 == 0x201) && (*(DAT_004c9370 + 0x1d) <= param_3)) &&
            (param_3 < *(DAT_004c9370 + 0x1d) + *(DAT_004c9370 + 0x25))) &&
            ((*(DAT_004c9370 + 0x21) <= param_4 &&
                (param_4 < *(DAT_004c9370 + 0x21) + *(DAT_004c9370 + 0x29))))) {
            FUN_0044738c(DAT_004c9370,(param_3 * 0x67e) / *(DAT_004c9370 + 0x25) + -0x1c,
                         (param_4 * 0x4ec) / *(DAT_004c9370 + 0x29));
            FUN_0044783a(DAT_0059a1c0,*(DAT_004c9370 + 0x8d),*(DAT_004c9370 + 0x91),0x0);
            iVar2 = DAT_0059a1c4;
            uVar1 = *(DAT_0059a1c0 + 0x4d);
            *(DAT_0059a1c4 + 0x59) = *(DAT_0059a1c0 + 0x49);
            *(iVar2 + 0x5d) = uVar1;
            FUN_0049c140(param_1,0x1);
        }
    }
    else {
        if (param_2 < 0x406) {
            FUN_004953d7();
            FUN_0049e640(*(DAT_004c9370 + 0x1d),*(DAT_004c9370 + 0x21),*(DAT_004c9370 + 0x25),
                         *(DAT_004c9370 + 0x29),0xce,0xca,0xcc,0x1);
            FUN_00497567(0x14,0x1d1,&DAT_004c9378,0xc8,0xcaccce,-0x1,0xcaccce,LPCSTR_005b9218,0x10);
            FUN_0049536f();
            return 0x1;
        }
        if (0x406 < param_2) {
            if (param_2 < 0x408) {
                if (param_3 == 0x64) {
                    FUN_0049c140(param_1,0x1);
                }
                else {
                    if (param_3 == 0x1bbc) {
                        FUN_00439f46(DAT_004c9370,0x1);
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



fn FUN_0042668f(param_1: i32,param_2: u32,param_3: u32) -> u32

{
    let sVar1: i16;

    param_1 = param_1 << 0x8;
    for (sVar1 = 0x8; sVar1 != 0x0; sVar1 = sVar1 + -0x1) {
        if ((((ushort)param_2 ^ (ushort)param_1) & 0x8000) == 0x0) {
            param_2 = param_2 << 0x1;
        }
        else {
            param_2 = param_3 ^ param_2 * 0x2;
        }
        param_1 = param_1 << 0x1;
    }
    return param_2;
}



fn FUN_004266f4(ushort param_1) -> i32

{
ushort uVar1;
let mut uVar2: u32;
let mut local_1c: i32;

local_1c = FUN_0049c2c9(0x200);
if (local_1c == 0x0) {
local_1c = 0x0;
}
else {
for (uVar1 = 0x0; uVar1 < 0x100; uVar1 = uVar1 + 0x1) {
uVar2 = FUN_0042668f(uVar1,0x0,param_1);
(uVar1 * 0x2 + local_1c) = uVar2;
}
}
return local_1c;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00426782()

{
    let mut local_210: String; [0x3f];
    let mut local_114: *mut u32 [0x7];
    let mut local_f7: i32;
    let mut local_f3: i32;
    let ppuStack207: *mut *mut u8;;
    let mut local_cb: u32;
    let mut local_c7: u32;
    let mut local_c3: u32;
    let mut local_bf: u32;
    let mut local_bb: u32;
    let mut local_b7: u32;
    let mut local_23: String;;
    i32 **local_1c;
    i32 **local_18;

    _DAT_004c93b8 = 0x0;
    FUN_00426a99();
    if (DAT_004c93bc != 0x0) {
        DAT_004c93c4 = FUN_0049c2c9(0x11940);
        FUN_004a0430(DAT_004c93c4,0x0,0x11940);
        FUN_0049a030(local_114,0x0,0x1,0x0,0x0,0x280,0x1e0,0x4000,0x8,0x8);
        ppuStack207 = &PTR_FUN_004c3d34;
        local_b7 = 0x0;
        local_cb = 0x0;
        local_c7 = 0x0;
        local_c3 = 0x0;
        local_23 = 0x0;
        local_bb = 0x0;
        local_bf = 0x0;
        local_1c = (i32 **)FUN_004a2831(0x10);
        local_18 = local_1c;
        if (local_1c != (i32 **)0x0) {
            FUN_004a2874(local_1c,local_114,0x64);
        }
        FUN_0049cc70(local_210,local_114,0xbb7,local_f7 + 0xa,local_f3 + 0x82,0x0,0x0,0x0,s_FLC_CREDITS_FLC_004c1251);
        FUN_0049bf40(local_114,local_210);
        FUN_0049bb50(local_114,FUN_00426d07);
        FUN_004a2965(local_114);
        FUN_00426bcf();
        FUN_0049af50(DAT_004c93c4);
        FUN_0049cef0(local_210,0x0);
        ppuStack207 = &PTR_FUN_004c3d34;
        if (local_23 != 0x0) {
            FUN_00499b30(local_114,local_23);
        }
        FUN_0049a1c0(local_114,0x1);
    }
    return;
}



fn FUN_00426a99()

{
    let mut pcVar1: String;
    let puVar2: *mut u32;
    let puVar3: *mut u32;
    let puVar4: *mut u32;
    let mut local_118: u32;
    byte **local_18;
    let local_14: *mut u32;

    local_14 = 0x0;
    FUN_00426bcf();
    local_18 = (byte **)FUN_0049c4bd(s_FLC_CREDIT_TXT_004c1264,&DAT_004c1261);
    if (local_18 != (byte **)0x0) {
    while (pcVar1 = FUN_004a2f60(&local_118,0xff,local_18), pcVar1 != 0x0) {
        puVar2 = FUN_004a2831(0x8);
        if (puVar2 == 0x0) {
            pcVar1 = FUN_00499050(DAT_005b9bd8,0x7d01);
            pop_err_msg_box_and_exit_004a02f5(pcVar1);
        }
        puVar4 = &local_118;
        loop {
            puVar3 = puVar4;
            if (puVar4 == '\n')^ // goto LAB_00426b4a;
            if (puVar4 == '\0') break;
            puVar3 = (puVar4 + 0x1);
            if (puVar3 == '\n')^ // goto LAB_00426b4a;
            puVar4 = (puVar4 + 0x2);
        } while (puVar3 != '\0');
        puVar3 = 0x0;
        LAB_00426b4a:
        if (puVar3 != 0x0) {
            puVar3 = '\0';
        }
        puVar4 = FUN_0049e750(&local_118);
        *puVar2 = puVar4;
        puVar2[0x1] = 0x0;
        puVar4 = puVar2;
        if (local_14 != 0x0) {
            local_14[0x1] = puVar2;
            puVar4 = DAT_004c93bc;
        }
        DAT_004c93bc = puVar4;
        DAT_004c93c0 = DAT_004c93c0 + 0x1;
        local_14 = puVar2;
    }
    FUN_0049ca40(local_18);
}
    return;
}



fn FUN_00426bcf()

{
    let mut pCVar1: String;;
    LPCSTR *ppCVar2;
    LPCSTR *ppCVar3;

    while (ppCVar2 = DAT_004c93bc, DAT_004c93bc != 0x0) {
        ppCVar3 = DAT_004c93bc[0x1];
        if (*DAT_004c93bc != 0x0) {
            pCVar1 = *DAT_004c93bc;
            DAT_004c93bc = DAT_004c93bc[0x1];
            FUN_0049af50(pCVar1);
            ppCVar3 = DAT_004c93bc;
        }
        DAT_004c93bc = ppCVar3;
        FUN_0049af50(ppCVar2);
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00426c24()

{
    let local_38: *mut i32; [0x6];
    let local_20: *mut i32;;
    let mut local_1c: i32;
    let local_18: *mut u32;

    FUN_004953d7();
    local_18 = FUN_00498ba4(local_38,DAT_004c93c4,0x12c,0xf0);
    local_1c = 0x186 - _DAT_004c93b8;
    for (local_20 = DAT_004c93bc; local_20 != 0x0; local_20 = local_20[0x1]) {
        if ((0x5a - DAT_005b9210 < local_1c) && (local_1c < 0x187)) {
            FUN_00497567(0x78,local_1c + -0x5a,*local_20,0xf0,0xe2e3e4,0x0,0xe2e3e4,LPCSTR_005b9218,0x11);
        }
        local_1c = local_1c + DAT_005b9210;
    }
    FUN_00498cf4(local_38);
    FUN_00496ac0(DAT_004c93c4,0x14a,0x5a,0xf0,0x12c);
    FUN_0049536f();
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00426d07(param_1: i32,param_2: u32,param_3: i32) -> u32

{
    let mut iVar1: i32;

    if (param_2 < 0x401) {
        if (param_2 < 0x113) {
            if (param_2 != 0x100) {
                return 0x0;
            }
        }
        else {
            if (param_2 < 0x114) {
                iVar1 = FUN_0049ab40();
                if (iVar1 != param_1) {
                    return 0x1;
                }
                _DAT_004c93b8 = _DAT_004c93b8 + 0x2;
                if ((DAT_004c93c0 + 0x1) * DAT_005b9210 + 0x186 < _DAT_004c93b8) {
                    FUN_0049c140(param_1,0x1);
                }^
                // goto LAB_00426dbe;
            }
            if (param_2 < 0x201) {
                return 0x0;
            }
            if ((0x201 < param_2) && (param_2 != 0x203)) {
                return 0x0;
            }
        }
        FUN_0049c140(param_1,0x1);
    }
    else {
        if (param_2 < 0x402) {
            FUN_0049f99b(DAT_005b9bd4,param_1,0x407,0x29a,0x0);
            return 0x1;
        }
        if (param_2 < 0x407) {
            if (param_2 == 0x405) {
                LAB_00426dbe:
                    FUN_004953d7();
                FUN_00426c24();
                FUN_0049536f();
                return 0x0;
            }
        }
        else {
            if (param_2 < 0x408) {
                if (param_3 == 0x29a) {
                    FUN_00426edc();
                }
                return 0x0;
            }
            if (0x40b < param_2) {
                if (param_2 < 0x40d) {
                    FUN_004953d7();
                    FUN_004968e7(0x0,0x0,0x280,0x1e0,0xa0);
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



fn FUN_00426edc()

{
    let mut local_1a4: *mut u32 [0x11];
    let ppuStack351: *mut *mut u8;;
    let mut local_15b: u32;
    let mut local_157: u32;
    let mut local_153: u32;
    let mut local_14f: u32;
    let mut local_14b: u32;
    let mut local_147: u32;
    let mut local_b3: String;;
    let local_ac: *mut u32;
    let mut local_a8: u32;
    i32 **local_a4;
    i32 **local_a0;
    let local_9c: *mut u32;
    let local_98: *mut u32;
    u32 **local_94;
    u32 **local_90;
    let mut local_8c: u32;
    let mut local_88: u32;
    let mut local_84: u32;
    let mut local_80: u32;
    let mut local_7c: u32;
    let mut local_78: u32;
    let mut local_74: u32;
    let mut local_70: u32;
    let mut local_6c: u32;
    u32 **local_68;
    let local_64: *mut u32;
    i32 **local_60;
    let local_5c: *mut u32;
    let local_58: *mut u32;
    u32 **local_54;
    let mut local_50: u32;
    let mut local_4c: u32;
    let mut local_48: u32;
    let mut local_44: u32;
    let mut local_40: u32;
    let mut local_3c: u32;
    let mut local_38: u32;
    let mut local_34: u32;
    let mut local_30: u32;
    let local_2c: *mut u32;
    u32 **local_28;
    let mut local_24: u32;
    u32 **local_20;
    let mut local_1c: u32;

    local_90 = local_1a4;
    local_8c = 0x0;
    local_88 = 0x1;
    local_84 = 0x0;
    local_80 = 0x0;
    local_7c = 0x280;
    local_78 = 0x1e0;
    local_74 = 0x4000;
    local_70 = 0x8;
    local_6c = 0x8;
    local_64 = FUN_0049a030(local_1a4,0x0,0x1,0x0,0x0,0x280,0x1e0,0x4000,0x8,0x8);
    $1: &mut String(local_90 + 0x45) = &PTR_FUN_004c3d34;
    local_147 = 0x0;
    local_15b = 0x0;
    local_157 = 0x0;
    local_153 = 0x0;
    local_b3 = 0x0;
    local_14b = 0x0;
    local_14f = 0x0;
    local_94 = local_1a4;
    local_68 = local_94;
    local_a4 = (i32 **)FUN_004a2831(0x10);
    local_a0 = local_a4;
    local_60 = local_a4;
    if (local_a4 != (i32 **)0x0) {
    local_60 = FUN_004a2874(local_a4,local_1a4,0x4e20);
}
    local_9c = FUN_004a2831(0x5d);
    local_98 = local_9c;
    local_5c = local_9c;
    if (local_9c != 0x0) {
        local_54 = local_1a4;
        local_a8 = 0x29a;
        local_50 = 0x0;
        local_4c = 0x0;
        local_48 = 0x1;
        local_44 = 0x1;
        local_40 = 0x0;
        local_3c = 0xcaccce;
        local_38 = 0xe0e0e;
        local_34 = 0x0;
        local_30 = 0x0;
        local_58 = local_9c;
        local_5c = FUN_0049a030(local_9c,local_54,0x29a,0x0,0x0,0x1,0x1,0x0,0xcaccce,0xe0e0e);
        local_28 = &local_58;
        $1: &mut String(local_5c + 0x45) = &PTR_FUN_004c3d94;
        *(local_5c + 0x51) = local_34;
        *(local_5c + 0x55) = local_30;
        *(local_5c + 0x4d) = 0x0;
        *(local_5c + 0x49) = 0x2;
        local_58 = local_5c;
        local_2c = local_5c;
    }
    local_ac = local_5c;
    FUN_0049bf40(local_1a4,local_5c);
    FUN_0049bb50(local_1a4,FUN_004271e8);
    FUN_004a2965(local_1a4);
    if (local_ac == 0x0) {
        local_24 = 0x0;
    }
    else {
        local_24 = ((*(local_ac + 0x45) + 0x8))(local_ac,0x2);
    }
    local_20 = local_1a4;
    local_1c = 0x0;
    ppuStack351 = &PTR_FUN_004c3d34;
    if (local_b3 != 0x0) {
        FUN_00499b30(local_1a4,local_b3);
    }
    FUN_0049a1c0(local_1a4,0x1);
    return;
}



fn FUN_004271e8(param_1: *mut *mut u32,param_2: u32,param_3: u32,param_4: i32) -> u32

{
    if (param_2 < 0x203) {
        if (param_2 < 0x113) {
            if (param_2 != 0x100) {
                return 0x0;
            }
        }
        else {
            if (param_2 < 0x114) {
                if (param_4 == 0x4e20) {
                    FUN_0049c140(param_1,0x1);
                }
                return 0x0;
            }
            if (param_2 != 0x201) {
                return 0x0;
            }
        }
    }
    else {
        if (0x203 < param_2) {
            if (param_2 < 0x40c) {
                if (param_2 != 0x401) {
                    return 0x0;
                }
                FUN_0049bf80(param_1,0x29a,0x414,0x0,0x0);
                return 0x0;
            }
            if (0x40c < param_2) {
                if (param_2 != 0x411) {
                    return 0x0;
                }
                return 0x1;
            }
            FUN_004953d7();
            FUN_004a08c5(s_pcx_fsart0_pcx_004c1273,0x0,0x0,0x280,0x1e0,0x0,0x0,0x0,0x0);
            FUN_0049536f();
            return 0x1;
        }
    }
    FUN_0049c140(param_1,0x1);
    return 0x0;
}



fn FUN_0042732a()

{
    let mut uVar1: u32;
    let local_190: *mut u32;
    let mut local_13c: *mut u32 [0x8];
    let mut local_11b: i32;
    let ppuStack247: *mut *mut u8;;
    let mut local_4b: String;;
    u32 auStack68 [0x5];
    let local_30: u8 [0x10];
    let mut local_20: i32;
    let mut local_1c: i32;
    let local_18: *mut i32;;

    local_1c = 0x0;
    FUN_0049d2b0(0xfa,0xc8);
    local_18 = FUN_004990e0(local_13c,0x0,s_diplo_res_004c128c,s_Diplomacy_004c1282);
    for (local_20 = 0x0; local_20 < 0x4; local_20 = local_20 + 0x1) {
        *(&DAT_004c93e0 + local_20 * 0x4) = 0x0;
    }
    for (local_20 = 0x0; local_20 < 0x5; local_20 = local_20 + 0x1) {
        if (local_20 != DAT_004c9754) {
            uVar1 = FUN_0049c2c9(0x2710);
            *(&DAT_004c93e0 + local_1c * 0x4) = uVar1;
            FUN_0049c2e0(local_30,s_bin_house_d_bin_004c1296);
            FUN_0049c57b(local_30,(&DAT_004c93e0 + local_1c * 0x4),0x2710);
            local_190 = FUN_004a2831(0x5d);
            if (local_190 != 0x0) {
                uVar1 = *(&DAT_004c93e0 + local_1c * 0x4);
                local_190 = FUN_0049a030(local_190,local_13c,local_20 + 0x258,local_1c * 0x8c + 0x3c,local_11b + 0x28,0x64,
                                         0x64,0x2,0xcaccce,0xe0e0e);
                $1: &mut String(local_190 + 0x45) = &PTR_FUN_004c3d94;
                *(local_190 + 0x51) = uVar1;
                *(local_190 + 0x55) = 0x0;
                *(local_190 + 0x4d) = 0x0;
                *(local_190 + 0x49) = 0x2;
            }
            auStack68[local_20] = local_190;
            FUN_0049bf40(local_13c,auStack68[local_20]);
            local_1c = local_1c + 0x1;
        }
    }
    FUN_0049bb50(local_13c,FUN_0042769d);
    for (local_20 = 0x0; local_20 < 0x4; local_20 = local_20 + 0x1) {
        if (*(&DAT_004c93e0 + local_20 * 0x4) != 0x0) {
            FUN_0049af50(*(&DAT_004c93e0 + local_20 * 0x4));
        }
    }
    ppuStack247 = &PTR_FUN_004c3d34;
    if (local_4b != 0x0) {
        FUN_00499b30(local_13c,local_4b);
    }
    FUN_0049a1c0(local_13c,0x1);
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0042769d(param_1: *mut *mut u32,param_2: u32,param_3: u32) -> u32

{
    let mut pcVar1: String;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut uVar4: u32;
    let mut iVar5: i32;
    let mut uVar6: u32;
    let puVar7: *mut u32;
    let bVar8: u8;
    let mut pcVar9: String;
    let local_c8: u8 [0x98];
    let mut local_30: i32;
    let mut local_2c: i32;
    let mut local_28: u32;
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_14: u32;

    local_14 = param_2;
    if (param_2 < 0x405) {
        if (param_2 == 0x401) {
            for (local_1c = 0x0; local_1c < 0x5; local_1c = local_1c + 0x1) {
                if (((&DAT_00569a98)[local_1c * 0x1e22] & 0x1) != 0x0) {
                    FUN_0049bf80(param_1,local_1c + 0x258,0x410,0x0,0x0);
                }
            }
            FUN_0049f99b(DAT_005b9bd4,param_1,0x407,0x3039,0x0);
            return 0x0;
        }
    }
    else {
        if (param_2 < 0x406) {
            FUN_004953d7();
            local_24 = 0x0;
            for (local_20 = 0x0; local_20 < 0x5; local_20 = local_20 + 0x1) {
                if (local_20 != DAT_004c9754) {
                    FUN_00497567(local_24 * 0x8c + 0x6e,*(param_1 + 0x21) + 0x91,
                                 (&DAT_00569b50 + local_20 * 0x1e22),0x64,0xcaccce,-0x1,0xcaccce,
                                 LPCSTR_005b9218,0x11);
                    local_24 = local_24 + 0x1;
                }
            }
            bVar8 = 0x11;
            uVar6 = 0xcaccce;
            iVar5 = -0x1;
            uVar4 = 0xcaccce;
            iVar3 = 0x64;
            puVar7 = LPCSTR_005b9218;
            pcVar1 = FUN_00499050(DAT_0059679c,0x419);
            FUN_00497567(0xb4,*(param_1 + 0x21) + 0x11d,pcVar1,iVar3,uVar4,iVar5,uVar6,puVar7,bVar8);
            bVar8 = 0x11;
            uVar6 = 0xcaccce;
            iVar5 = -0x1;
            uVar4 = 0xcaccce;
            iVar3 = 0x64;
            puVar7 = LPCSTR_005b9218;
            pcVar1 = FUN_00499050(DAT_0059679c,0x41a);
            FUN_00497567(0x140,*(param_1 + 0x21) + 0x11d,pcVar1,iVar3,uVar4,iVar5,uVar6,puVar7,bVar8);
            bVar8 = 0x11;
            uVar6 = 0xcaccce;
            iVar5 = -0x1;
            uVar4 = 0xcaccce;
            iVar3 = 0x64;
            puVar7 = LPCSTR_005b9218;
            pcVar1 = FUN_00499050(DAT_0059679c,0x41c);
            FUN_00497567(0x1cc,*(param_1 + 0x21) + 0x11d,pcVar1,iVar3,uVar4,iVar5,uVar6,puVar7,bVar8);
            FUN_0049536f();
        }
        else {
            if (0x406 < param_2) {
                if (param_2 < 0x408) {
                    local_28 = param_3;
                    if (param_3 < 0x25b) {
                        if (param_3 < 0x258) {
                            if (param_3 != 0x64) {
                                return 0x0;
                            }
                            FUN_0049c140(param_1,0x1);
                            return 0x0;
                        }
                    }
                    else {
                        if (0x25b < param_3) {
                            if (0x25d < param_3) {
                                if (param_3 < 0x25f) {
                                    FUN_00407343();
                                    return 0x0;
                                }
                                if (param_3 < 0x260) {
                                    if (_DAT_004c9744 != 0x0) {
                                        pcVar9 = s_bin_ptratvau_bin_004c12a6;
                                        pcVar1 = FUN_00499050(DAT_005967ac,0x7d0);
                                        FUN_0049dc40(param_1,0x1,pcVar1,pcVar9);
                                        return 0x0;
                                    }
                                    _DAT_004c9744 = 0x1;
                                    _DAT_004c93d8 = 0x1;
                                    pcVar9 = s_bin_ptratvau_bin_004c12b7;
                                    pcVar1 = FUN_00499050(DAT_005967ac,0x7d1);
                                    uVar2 = FUN_0049dc40(param_1,0x3,pcVar1,pcVar9);
                                    if (uVar2 != 0x0) {
                                        pcVar9 = s_bin_ptratvau_bin_004c12c8;
                                        pcVar1 = FUN_00499050(DAT_005967ac,0x7d2);
                                        FUN_0049dc40(param_1,0x1,pcVar1,pcVar9);
                                        uVar2 = FUN_0042b826(0x140);
                                        if (uVar2 != 0x0) {
                                            local_2c = FUN_00481a7f(DAT_004c93dc,0x8,DAT_004c9754);
                                            uVar2 = FUN_004a2edc();
                                            local_30 = local_2c * (uVar2 % 0x28 + 0x3c);
                                            if (local_2c == 0x0) {
                                                pcVar1 = FUN_00499050(DAT_005967ac,0x7d3);
                                                FUN_0049c2e0(local_c8,pcVar1);
                                                FUN_0049dc40(param_1,0x1,local_c8,s_bin_ptratvau_bin_004c12d9);
                                            }
                                            else {
                                                pcVar1 = FUN_00499050(DAT_005967ac,0x7d4);
                                                FUN_0049c2e0(local_c8,pcVar1);
                                                uVar2 = FUN_0049dc40(param_1,0x3,local_c8,s_bin_ptratvau_bin_004c12ea);
                                                if (uVar2 != 0x0) {
                                                    _DAT_004c9744 = 0x0;
                                                    *(&DAT_00569abd + DAT_004c9754 * 0x1e22) =
                                                        *(&DAT_00569abd + DAT_004c9754 * 0x1e22) + local_30;
                                                    FUN_0043a681(DAT_004c93dc,0x8,DAT_004c9754);
                                                }
                                            }
                                        }
                                    }
                                    _DAT_004c93d8 = 0x0;
                                    return 0x0;
                                }
                                if (param_3 != 0x3039) {
                                    return 0x0;
                                }
                                FUN_00483355(0x31);
                                return 0x0;
                            }
                            if (0x25c < param_3) {
                                FUN_0044fdf2();
                                return 0x0;
                            }
                        }
                    }
                    FUN_00428bd5(param_3 - 0x258);
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



fn FUN_00427bc4() -> i32

{
let mut uVar1: u32;
let local_198: *mut u32;
let mut local_144: *mut u32 [0x8];
let mut local_123: i32;
let ppuStack255: *mut *mut u8;;
let mut local_53: String;;
u32 auStack76 [0x5];
let local_38: u8 [0x10];
let mut local_28: i32;
let mut local_24: i32;
let mut local_20: i32;
let mut local_1c: u32;
let local_18: *mut i32;;

local_24 = 0x0;
local_18 = FUN_004990e0(local_144,0x0,s_diplo_res_004c1302,s_Choose_004c12fb);
for (local_20 = 0x0; local_20 < 0x4; local_20 = local_20 + 0x1) {
*(&DAT_004c93f0 + local_20 * 0x4) = 0x0;
}
for (local_20 = 0x0; local_20 < 0x5; local_20 = local_20 + 0x1) {
if (local_20 != DAT_004c9754) {
uVar1 = FUN_0049c2c9(0x2710);
*(&DAT_004c93f0 + local_24 * 0x4) = uVar1;
FUN_0049c2e0(local_38,s_bin_house_d_bin_004c130c);
FUN_0049c57b(local_38,(&DAT_004c93f0 + local_24 * 0x4),0x2710);
local_198 = FUN_004a2831(0x5d);
if (local_198 != 0x0) {
uVar1 = *(&DAT_004c93f0 + local_24 * 0x4);
local_198 = FUN_0049a030(local_198,local_144,local_20 + 0x258,local_24 * 0x8c + 0x3c,local_123 + 0x28,0x64,
0x64,0x2,0xcaccce,0xe0e0e);
$1: &mut String(local_198 + 0x45) = &PTR_FUN_004c3d94;
*(local_198 + 0x51) = uVar1;
*(local_198 + 0x55) = 0x0;
*(local_198 + 0x4d) = 0x0;
*(local_198 + 0x49) = 0x2;
}
auStack76[local_20] = local_198;
FUN_0049bf40(local_144,auStack76[local_20]);
local_24 = local_24 + 0x1;
}
}
local_1c = FUN_0049bb50(local_144,FUN_00427ffc);
for (local_20 = 0x0; local_20 < 0x4; local_20 = local_20 + 0x1) {
if (*(&DAT_004c93f0 + local_20 * 0x4) != 0x0) {
FUN_0049af50(*(&DAT_004c93f0 + local_20 * 0x4));
}
}
if (local_1c == 0x0) {
local_28 = -0x1;
ppuStack255 = &PTR_FUN_004c3d34;
if (local_53 != 0x0) {
FUN_00499b30(local_144,local_53);
}
FUN_0049a1c0(local_144,0x1);
}
else {
local_28 = local_1c - 0x258;
ppuStack255 = &PTR_FUN_004c3d34;
if (local_53 != 0x0) {
FUN_00499b30(local_144,local_53);
}
FUN_0049a1c0(local_144,0x1);
}
return local_28;
}



fn FUN_00427ffc(param_1: *mut *mut u32,param_2: u32,param_3: u32) -> u32

{
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_1c: i32;

    if (param_2 < 0x405) {
        if (param_2 == 0x401) {
            for (local_1c = 0x0; local_1c < 0x5; local_1c = local_1c + 0x1) {
                if (((&DAT_00569a98)[local_1c * 0x1e22] & 0x1) != 0x0) {
                    FUN_0049bf80(param_1,local_1c + 0x258,0x410,0x0,0x0);
                }
            }
        }
    }
    else {
        if (param_2 < 0x406) {
            FUN_004953d7();
            local_24 = 0x0;
            for (local_20 = 0x0; local_20 < 0x5; local_20 = local_20 + 0x1) {
                if (local_20 != DAT_004c9754) {
                    FUN_00497567(local_24 * 0x8c + 0x6e,*(param_1 + 0x21) + 0x91,
                                 (&DAT_00569b50 + local_20 * 0x1e22),0x64,0xcaccce,-0x1,0xcaccce,
                                 LPCSTR_005b9218,0x11);
                    local_24 = local_24 + 0x1;
                }
            }
            FUN_0049536f();
        }
        else {
            if (0x406 < param_2) {
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
                    FUN_0049c140(param_1,param_3);
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



fn FUN_004281cd()

{
    let local_30: u8 [0x10];
    let mut local_20: *mut u8;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: u32;

    local_20 = &DAT_004d55a8;
    local_1c = DAT_004c9754;
    local_18 = DAT_004c93cc;
    local_14 = (byte)(&DAT_004d55a8)[DAT_004c9754 * 0xe + DAT_004c93cc];
    if ((local_14 == 0x1) || (DAT_004c9754 == DAT_004c93cc)) {
        FUN_0042f963(DAT_004c93cc,0x0,0x1);
        FUN_0049c2e0(local_30,&DAT_004c131c);
    }
    else {
        FUN_0049c2e0(local_30,&DAT_004c131f);
    }
    FUN_00497567(0x267,0x1ab,local_30,0x32,0xcaccce,0xe0e0e,0xcaccce,LPCSTR_005b9218,0x14);
    return;
}



fn FUN_00428288(param_1: i32,param_2: i32,param_3: i32,param_4: i32)

{
    let cVar1: u8;
    let mut iVar2: i32;
    let mut pcVar3: String;
    let local_b8: u8;
    let mut local_b4: i32;
    let mut local_b0: i32;
    let local_a0: *mut i32;;
    let mut local_14: i32;

    FUN_004953d7();
    local_14 = 0x0;
    while( true ) {
        if (0x2 < local_14) {
            FUN_0049536f();
            return;
        }
        if (param_3 == 0x0) {
            local_a0 = (&DAT_004c97f8 + local_14 * 0x28 + param_1 * 0xf8 + param_2 * 0xd90);
        }
        else {
            local_a0 = (&DAT_004c9780 + local_14 * 0x28 + param_1 * 0xf8 + param_2 * 0xd90);
        }
        if (*local_a0 - 0x12cU < 0x63) break;
        if (param_3 == 0x0) {
            local_b4 = 0x97;
        }
        else {
            local_b4 = 0x31;
        }
        FUN_004968e7(0x14a,param_4 + local_b4 + 0x3 + local_14 * 0x1b,0x118,0x13,0xe);
        if (param_3 == 0x0) {
            local_b0 = 0x97;
        }
        else {
            local_b0 = 0x31;
        }
        FUN_00496ee6(&DAT_00591f30,0x128,local_14 * 0x1b + param_4 + local_b0,0x19,0x19);
        local_14 = local_14 + 0x1;
    }
    local_b8 = (char)(*local_a0 - 0x12cU);
    iVar2 = 0x16;
    pcVar3 = "ba`_^]\\[ZPF<2\"! \x1f\x1e\x14\n";
    loop {
    if (iVar2 == 0x0) break;
    iVar2 = iVar2 + -0x1;
    cVar1 = *pcVar3;
    pcVar3 = pcVar3 + 0x1;
} while (local_b8 != cVar1);
    // WARNING: Could not recover jumptable at 0x00428a8f. Too many branches
    // WARNING: Treating indirect jump as call
    ((&UNK_00428a07 + iVar2 * 0x4))();
    return;
}



fn FUN_00428bd5(param_1: i32)

{
    let mut local_110: *mut u32 [0x11];
    let ppuStack203: *mut *mut u8;;
    let mut local_1f: String;;
    let mut local_18: i32;

    for (local_18 = 0x0; local_18 < 0x5; local_18 = local_18 + 0x1) {
        FUN_00489246(local_18,0x0);
    }
    DAT_004c93cc = param_1;
    DAT_004c93d4 = 0x0;
    DAT_004c93c8 = FUN_0049c2c9(0xc350);
    FUN_0049c60b(s_bin_portrait_bin_004c1323,DAT_004c93c8,0xc350,
                 *(&DAT_00569b60 + param_1 * 0x1e22) * 0x3d090 + param_1 * 0xc350);
    FUN_004990e0(local_110,0x0,s_diplo_res_004c133d,s_DiploDlg_004c1334);
    FUN_0049bb50(local_110,FUN_00428d05);
    FUN_0049af50(DAT_004c93c8);
    ppuStack203 = &PTR_FUN_004c3d34;
    if (local_1f != 0x0) {
        FUN_00499b30(local_110,local_1f);
    }
    FUN_0049a1c0(local_110,0x1);
    return;
}



fn FUN_00428d05(param_1: *mut *mut u32,param_2: u32,param_3: u32) -> u32

{
    let mut bVar1: bool;
    let mut pcVar2: String;
    let mut uVar3: u32;
    let mut puVar4: *mut u8;
    let mut local_13c: i32;
    let mut local_114: i32;
    let local_100: u8 [0x10];
    let mut local_f0: *mut u8;
    let mut local_ec: i32;
    let mut local_e8: i32;
    let mut local_e4: u32;
    let local_e0: u8 [0x80];
    let mut local_60: *mut u8;
    let mut local_5c: i32;
    let mut local_58: i32;
    let mut local_54: u32;
    let mut local_50: *mut u8;
    let mut local_4c: i32;
    let mut local_48: i32;
    let mut local_44: u32;
    let local_40: u8 [0x10];
    let mut local_30: *mut u8;
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_24: u32;
    let mut local_20: u32;
    let mut local_1c: i32;
    let mut local_14: u32;

    local_14 = param_2;
    if (param_2 < 0x406) {
        if (0x400 < param_2) {
            if (param_2 < 0x402) {
                if (DAT_004c93cc == DAT_004c9754) {
                    FUN_0049bf80(param_1,0x12c,0x502,*(&DAT_00569adc + DAT_004c93cc * 0x1e22),0x0);
                    FUN_0049bf80(param_1,0x12d,0x502,*(&DAT_00569ae4 + DAT_004c93cc * 0x1e22),0x0);
                    FUN_0049bf80(param_1,0x12e,0x502,*(&DAT_00569ae8 + DAT_004c93cc * 0x1e22),0x0);
                    for (local_1c = 0x0; local_1c < 0x3; local_1c = local_1c + 0x1) {
                        FUN_0049bf80(param_1,local_1c + 0x1a5,0x414,0x0,0x0);
                        FUN_0049bf80(param_1,local_1c + 0x1a5,0x410,0x0,0x0);
                        FUN_0049bf80(param_1,local_1c + 0x1af,0x414,0x0,0x0);
                        FUN_0049bf80(param_1,local_1c + 0x1af,0x410,0x0,0x0);
                    }
                }
                else {
                    FUN_0049bf80(param_1,0x12c,0x410,0x0,0x0);
                    FUN_0049bf80(param_1,0x12d,0x410,0x0,0x0);
                    FUN_0049bf80(param_1,0x12e,0x410,0x0,0x0);
                }
                FUN_0049bf80(param_1,0x136,0x502,*(&DAT_004c9778 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8),0x0);
                FUN_0049f99b(DAT_005b9bd4,param_1,0x407,0x3039,0x0);
                return 0x0;
            }
            if (param_2 == 0x405) {
                FUN_004953d7();
                FUN_00496ac0(DAT_004c93c8,0xa,0x10,0xfa,0xc8);
                FUN_004968e7(0xb,0xe4,0xf8,0x12,0xe);
                if (DAT_004c93cc < 0x5) {
                    FUN_00499050(DAT_0059679c,DAT_004c93cc + 0x414);
                    pcVar2 = FUN_00499050(DAT_0059679c,0x73cc);
                    FUN_0049c2e0(local_e0,pcVar2);
                }
                else {
                    FUN_0049c2e0(local_e0,&DAT_00569b50 + DAT_004c93cc * 0x1e22);
                }
                FUN_00497567(0x88,0xe4,local_e0,0xf0,0xcaccce,0xe0e0e,0xcaccce,LPCSTR_005b9218,0x11);
                FUN_004968e7(0x235,0x181,0x32,0x12,0xe);
                FUN_0049e640(0x235,0x181,0x32,0x12,0xce,0xca,0xcc,0x1);
                local_f0 = &DAT_004d55a8;
                local_ec = DAT_004c9754;
                local_e8 = DAT_004c93cc;
                local_e4 = (byte)(&DAT_004d55a8)[DAT_004c9754 * 0xe + DAT_004c93cc];
                if ((local_e4 == 0x1) || (DAT_004c9754 == DAT_004c93cc)) {
                    FUN_0049c2e0(local_100,&DAT_004c1370);
                }
                else {
                    FUN_0049c2e0(local_100,&DAT_004c1373);
                }
                FUN_00497567(0x267,0x183,local_100,0x32,0xcaccce,0xe0e0e,0xcaccce,LPCSTR_005b9218,0x14);
                FUN_004968e7(0x235,0x195,0x32,0x12,0xe);
                FUN_0049e640(0x235,0x195,0x32,0x12,0xce,0xca,0xcc,0x1);
                if (((&DAT_004d55a8)[DAT_004c9754 * 0xe + DAT_004c93cc] == '\x01') || (DAT_004c9754 == DAT_004c93cc)) {
                    FUN_0049c2e0(local_100,&DAT_004c1377);
                }
                else {
                    FUN_0049c2e0(local_100,&DAT_004c137a);
                }
                FUN_00497567(0x267,0x197,local_100,0x32,0xcaccce,0xe0e0e,0xcaccce,LPCSTR_005b9218,0x14);
                FUN_004968e7(0x235,0x1a9,0x32,0x12,0xe);
                FUN_0049e640(0x235,0x1a9,0x32,0x12,0xce,0xca,0xcc,0x1);
                FUN_004281cd();
                for (local_114 = 0x0; local_114 < 0x3; local_114 = local_114 + 0x1) {
                    FUN_004968e7(0x235,local_114 * 0x24 + 0x121,0x32,0x12,0xe);
                    FUN_0049e640(0x235,local_114 * 0x24 + 0x121,0x32,0x12,0xce,0xca,0xcc,0x1);
                    FUN_0049bf80(param_1,local_114 + 0x12c,0x501,0x0,0x0);
                    FUN_00428d05(param_1,0x406,local_114 + 0x12c);
                }
                FUN_00496ee6(DAT_004d6a30,0x20d,0x116,0x20,0x20);
                FUN_00496ee6(DAT_004d6608,0x20d,0x139,0x20,0x20);
                FUN_00496ee6(DAT_004d696c,0x20d,0x15c,0x20,0x20);
                FUN_00428288(DAT_004c93cc,DAT_004c9754,0x1,0x0);
                FUN_00428288(DAT_004c93cc,DAT_004c9754,0x0,0x0);
                FUN_0044bf63();
                FUN_0049536f();
            }
        }
    }
    else {
        if (param_2 < 0x407) {
            FUN_004953d7();
            local_20 = param_3;
            if (param_3 < 0x12d) {
                if (param_3 == 0x12c) {
                    FUN_0048976a(DAT_004c93cc,0x0);
                    local_30 = &DAT_004d55a8;
                    local_2c = DAT_004c9754;
                    local_28 = DAT_004c93cc;
                    local_24 = (byte)(&DAT_004d55a8)[DAT_004c9754 * 0xe + DAT_004c93cc];
                    if ((local_24 == 0x1) || (DAT_004c9754 == DAT_004c93cc)) {
                        FUN_0049c2e0(local_40,&DAT_004c1357);
                    }
                    else {
                        FUN_0049c2e0(local_40,&DAT_004c135b);
                    }
                    FUN_00497567(0x267,0x124,local_40,0x32,0xcaccce,0xe0e0e,0xcaccce,LPCSTR_005b9218,0x14);
                    FUN_004281cd();
                }
            }
            else {
                if (param_3 < 0x12e) {
                    local_50 = &DAT_004d55a8;
                    local_4c = DAT_004c9754;
                    local_48 = DAT_004c93cc;
                    local_44 = (byte)(&DAT_004d55a8)[DAT_004c9754 * 0xe + DAT_004c93cc];
                    if ((local_44 == 0x1) || (DAT_004c9754 == DAT_004c93cc)) {
                        FUN_0049c2e0(local_40,&DAT_004c135f);
                    }
                    else {
                        FUN_0049c2e0(local_40,&DAT_004c1363);
                    }
                    FUN_00497567(0x267,0x148,local_40,0x32,0xcaccce,0xe0e0e,0xcaccce,LPCSTR_005b9218,0x14);
                    FUN_004281cd();
                }
                else {
                    if (param_3 == 0x12e) {
                        local_60 = &DAT_004d55a8;
                        local_5c = DAT_004c9754;
                        local_58 = DAT_004c93cc;
                        local_54 = (byte)(&DAT_004d55a8)[DAT_004c9754 * 0xe + DAT_004c93cc];
                        if ((local_54 == 0x1) || (DAT_004c9754 == DAT_004c93cc)) {
                            FUN_0049c2e0(local_40,&DAT_004c1367);
                        }
                        else {
                            FUN_0049c2e0(local_40,&DAT_004c136c);
                        }
                        FUN_00497567(0x267,0x16c,local_40,0x32,0xcaccce,0xe0e0e,0xcaccce,LPCSTR_005b9218,0x14);
                        FUN_004281cd();
                    }
                }
            }
            FUN_0049536f();
            return 0x1;
        }
        if (param_2 < 0x40c) {
            if (param_2 == 0x407) {
                if (param_3 < 0x1a6) {
                    if (param_3 < 0x135) {
                        if (param_3 != 0x64) {
                            return 0x0;
                        }
                        bVar1 = true;
                        local_13c = 0x0;
                        loop {
                            if (0x2 < local_13c) {
                                LAB_0042996e:
                                if (bVar1) {
                                    *(&DAT_004c977c + DAT_004c93cc * 0xf8 + DAT_004c9754 * 0xd90) = 0x1;
                                }
                                else {
                                    puVar4 = FUN_0049bf80(param_1,0x136,0x501,0x0,0x0);
                                    (&DAT_004c9778 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8) = puVar4;
                                    *(&DAT_004c977c + DAT_004c93cc * 0xf8 + DAT_004c9754 * 0xd90) = 0x0;
                                    FUN_004a1651();
                                }
                                FUN_0049c140(param_1,0x1);
                                return 0x0;
                            }
                            if (0x0 < *(&DAT_004c9780 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8 + local_13c * 0x28)) {
                                bVar1 = false;^
                                // goto LAB_0042996e;
                            }
                            if (0x0 < *(&DAT_004c97f8 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8 + local_13c * 0x28)) {
                                bVar1 = false;^
                                // goto LAB_0042996e;
                            }
                            local_13c = local_13c + 0x1;
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
                uVar3 = FUN_00429b8c(0x0);
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
        }
        else {
            if (param_2 < 0x40d) {
                FUN_004953d7();
                FUN_004a08c5(s_pcx_diploma_pcx_004c1347,0x0,0x0,0x280,0x1e0,0x0,0x0,0x0,0x1);
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

fn FUN_00429b8c(param_1: i32) -> u32

{
    let mut local_120: *mut u32 [0x11];
    let ppuStack219: *mut *mut u8;;
    let mut local_2f: String;;
    let local_28: u8 [0x10];
    let mut local_18: u32;

    _DAT_004c9400 = param_1;
    if (DAT_004c93d0 == 0x0) {
        if (*(&DAT_004c9778 + DAT_004c93cc * 0xf8 + DAT_004c9754 * 0xd90) == 0x0) {
            if (param_1 == 0x1) {
                FUN_0049c2e0(local_28,s_LeagueCon2_004c13ad);
            }
            else {
                if (param_1 == 0x2) {
                    FUN_0049c2e0(local_28,s_ChurchCon2_004c13b8);
                }
                else {
                    FUN_0049c2e0(local_28,s_HouseCon2_004c13c3);
                }
            }
        }
        else {
            if (param_1 == 0x1) {
                FUN_0049c2e0(local_28,s_LeagueCon3_004c13cd);
            }
            else {
                if (param_1 == 0x2) {
                    FUN_0049c2e0(local_28,s_ChurchCon3_004c13d8);
                }
                else {
                    FUN_0049c2e0(local_28,s_HouseCon3_004c13e3);
                }
            }
        }
    }
    else {
        if (param_1 == 0x1) {
            FUN_0049c2e0(local_28,s_LeagueCon1_004c138d);
        }
        else {
            if (param_1 == 0x2) {
                FUN_0049c2e0(local_28,s_ChurchCon1_004c1398);
            }
            else {
                FUN_0049c2e0(local_28,s_HouseCon1_004c13a3);
            }
        }
    }
    FUN_004990e0(local_120,0x0,s_diplo_res_004c13ed,local_28);
    local_18 = FUN_0049bb50(local_120,&LAB_00429dab);
    ppuStack219 = &PTR_FUN_004c3d34;
    if (local_2f != 0x0) {
        FUN_00499b30(local_120,local_2f);
    }
    FUN_0049a1c0(local_120,0x1);
    return local_18;
}



fn FUN_0042ae4b() -> u32

{
    let mut local_120: *mut u32 [0x11];
    let ppuStack219: *mut *mut u8;;
    let mut local_2f: String;;
    let mut local_28: u32;
    let local_24: *mut i32;;
    u32 **local_20;
    let mut local_1c: u32;

    local_24 = FUN_004990e0(local_120,0x0,s_diplo_res_004c1402,s_CreditsDlg_004c13f7);
    FUN_0049bf80(local_120,0x12d,0x503,*(&DAT_00569abd + DAT_004c9754 * 0x1e22) + 0x1,0x64);
    local_28 = FUN_0049bb50(local_120,FUN_0042af6f);
    local_20 = local_120;
    local_1c = 0x0;
    ppuStack219 = &PTR_FUN_004c3d34;
    if (local_2f != 0x0) {
        FUN_00499b30(local_120,local_2f);
    }
    FUN_0049a1c0(local_120,0x1);
    return local_28;
}



fn FUN_0042af6f(param_1: *mut *mut u32,param_2: u32,param_3: u32) -> u32

{
    let mut pcVar1: String;
    let mut iVar2: i32;
    let local_50: u8 [0x10];
    let mut local_40: i32;
    let mut local_3c: u32;
    let local_38: u8 [0x10];
    let local_28: u8 [0x14];
    let mut local_14: u32;

    local_14 = param_2;
    switch(param_2) {
    case 0x401:
    if (DAT_004c93d0 == 0x0) {
        FUN_0049bf80(param_1,0x12d,0x502,
                     *(&DAT_004c97fc + DAT_004c93cc * 0xf8 + DAT_004c9754 * 0xd90 + DAT_004c93d4 * 0x28),0x0);
        FUN_0049bf80(param_1,0x12d,0x503,*(&DAT_00569abd + DAT_004c9754 * 0x1e22) + 0x1,0x64);
        FUN_0049c2e0(local_28,&DAT_004c140f);
    }
    else {
        FUN_0049bf80(param_1,0x12d,0x502,
                     *(&DAT_004c9784 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8 + DAT_004c93d4 * 0x28),0x0);
        FUN_0049c2e0(local_28,&DAT_004c140c);
    }
    FUN_0049bf80(param_1,0x12e,0x502,0x5,local_28);
    break;
    case 0x405:
        FUN_004953d7();
    FUN_0049bf80(param_1,0x12d,0x501,0x0,0x0);
    FUN_0042af6f(param_1,0x406,0x12d);
    FUN_0049536f();
    return 0x1;
    case 0x406:
    if (param_3 == 0x12d) {
        FUN_004953d7();
        FUN_0049c2e0(local_38,&DAT_004c1412);
        FUN_0049bf80(param_1,0x12e,0x502,0x6,local_38);
        FUN_0049536f();
    }
    return 0x1;
    case 0x407:
        local_3c = param_3;
    if (param_3 < 0x65) {
        if (param_3 == 0x64) {
            FUN_0049bf80(param_1,0x12e,0x501,0xf,local_50);
            local_40 = FUN_004a1e60(local_50);
            if ((DAT_004c93d0 == 0x0) && (*(&DAT_00569abd + DAT_004c9754 * 0x1e22) < local_40)) {
                pcVar1 = FUN_00499050(DAT_0059679c,0x715f);
                FUN_0049d2e0(param_1,0x1,pcVar1);
                return 0x0;
            }
            if (local_40 < 0x1) {
                pcVar1 = FUN_00499050(DAT_0059679c,0x740e);
                FUN_0049d2e0(param_1,0x1,pcVar1);
                return 0x0;
            }
            if (DAT_004c93d0 == 0x0) {
                iVar2 = FUN_004a1e60(local_50);
                *(&DAT_004c97fc + DAT_004c93cc * 0xf8 + DAT_004c9754 * 0xd90 + DAT_004c93d4 * 0x28) = iVar2;
                *(&DAT_004c97f8 + DAT_004c93d4 * 0x28 + DAT_004c93cc * 0xf8 + DAT_004c9754 * 0xd90) = 0x12c;
                *(&DAT_00569abd + DAT_004c9754 * 0x1e22) =
                    *(&DAT_00569abd + DAT_004c9754 * 0x1e22) -
                        *(&DAT_004c97fc + DAT_004c93d4 * 0x28 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8);
            }
            else {
                iVar2 = FUN_004a1e60(local_50);
                *(&DAT_004c9784 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8 + DAT_004c93d4 * 0x28) = iVar2;
                *(&DAT_004c9780 + DAT_004c93d4 * 0x28 + DAT_004c93cc * 0xf8 + DAT_004c9754 * 0xd90) = 0x12c;
            }
            FUN_0049c140(param_1,0x12c);
            return 0x0;
        }
    }
    else {
        if (param_3 < 0x66) {
            FUN_0049c140(param_1,0x0);
        }
        else {
            if (param_3 == 0x12e) {
                FUN_0042af6f(param_1,0x407,0x64);
            }
        }
    }
}
    return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0042b3d1() -> u32

{
    let mut local_124: *mut u32 [0x11];
    let ppuStack223: *mut *mut u8;;
    let mut local_33: String;;
    let mut local_2c: u32;
    let local_28: *mut u32;
    let local_24: *mut i32;;
    u32 **local_20;
    let mut local_1c: u32;

    DAT_0059a9f4 = 0x0;
    _DAT_0059aa0c = 0x3de;
    DAT_0059aa18 = 0x2;
    local_24 = FUN_004990e0(local_124,0x0,s_diplo_res_004c141e,s_TechDlg_004c1416);
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



fn FUN_0042b505() -> i32

{
let mut local_118: *mut u32 [0x11];
let ppuStack211: *mut *mut u8;;
let mut local_27: String;;
let mut local_20: i32;
let mut local_1c: u32;
let local_18: *mut i32;;

local_18 = FUN_004990e0(local_118,0x0,s_diplo_res_004c1431,s_Ministry_004c1428);
local_1c = FUN_0049bb50(local_118,FUN_0042b67e);
if (local_1c == 0x0) {
local_20 = -0x1;
ppuStack211 = &PTR_FUN_004c3d34;
if (local_27 != 0x0) {
FUN_00499b30(local_118,local_27);
}
FUN_0049a1c0(local_118,0x1);
}
else {
local_20 = local_1c - 0x1f5;
ppuStack211 = &PTR_FUN_004c3d34;
if (local_27 != 0x0) {
FUN_00499b30(local_118,local_27);
}
FUN_0049a1c0(local_118,0x1);
}
return local_20;
}



fn FUN_0042b67e(param_1: *mut *mut u32,param_2: u32,param_3: u32) -> u32

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut local_1c: i32;

    if (param_2 < 0x407) {
        if (param_2 == 0x401) {
            iVar1 = FUN_0042d379(0x1,0x188);
            iVar2 = FUN_0042d379(0x0,0x188);
            for (local_1c = 0x0; local_1c < 0x3; local_1c = local_1c + 0x1) {
                if ((iVar2 == -0x1) ||
                    (*(&DAT_004c981c + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8 + iVar2 * 0x28) != local_1c)) {
                    if ((iVar1 != -0x1) &&
                        (*(&DAT_004c97a4 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8 + iVar1 * 0x28) == local_1c)) {
                        FUN_0049bf80(param_1,local_1c + 0x1f5,0x410,0x0,0x0);
                    }
                }
                else {
                    FUN_0049bf80(param_1,local_1c + 0x1f5,0x410,0x0,0x0);
                }
            }
        }
    }
    else {
        if (param_2 < 0x408) {
            if (param_3 < 0x1f5) {
                if (param_3 == 0x65) {
                    FUN_0049c140(param_1,0x0);
                }
            }
            else {
                if (((param_3 < 0x1f6) || (param_3 < 0x1f7)) || (param_3 == 0x1f7)) {
                    FUN_0049c140(param_1,param_3);
                }
            }
        }
        else {
            if (param_2 == 0x411) {
                return 0x1;
            }
        }
    }
    return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0042b826(param_1: u32) -> u32

{
    let mut local_134: *mut u32 [0x11];
    let ppuStack239: *mut *mut u8;;
    let mut local_43: String;;
    let mut local_3c: u32;
    let local_38: *mut i32;;
    let local_34: *mut i32;;
    let mut local_30: u32;
    let local_2c: *mut i32;;
    let local_28: *mut i32;;
    let mut local_24: u32;
    u32 **local_20;
    let mut local_1c: u32;

    _DAT_004c9404 = param_1;
    local_2c = FUN_004990e0(local_134,0x0,s_efs_res_004c1445,s_GalaxyDlg_004c143b);
    local_38 = FUN_004a2831(0x95);
    local_34 = local_38;
    local_28 = local_38;
    if (local_38 != 0x0) {
        local_28 = FUN_0047157e(local_38,local_134,0x63,0x5f,0xa,0x1c2,0x1b0,0x1,0x9);
    }
    DAT_005b8858 = local_28;
    FUN_0049bf40(local_134,local_28);
    local_30 = FUN_0049bb50(local_134,FUN_0042b9c0);
    if (DAT_005b8858 == 0x0) {
        local_24 = 0x0;
    }
    else {
        local_24 = ((*(DAT_005b8858 + 0x45) + 0x8))(DAT_005b8858,0x2);
    }
    local_3c = local_30;
    local_20 = local_134;
    local_1c = 0x0;
    ppuStack239 = &PTR_FUN_004c3d34;
    if (local_43 != 0x0) {
        FUN_00499b30(local_134,local_43);
    }
    FUN_0049a1c0(local_134,0x1);
    return local_3c;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0042b9c0(param_1: i32,param_2: u32,param_3: i32,param_4: i32) -> u32

{
    let mut pcVar1: String;
    let local_138: u8 [0x100];
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
    if (param_2 < 0x405) {
        if ((((param_2 == 0x201) && (*(DAT_005b8858 + 0x1d) <= param_3)) &&
            (param_3 < *(DAT_005b8858 + 0x1d) + *(DAT_005b8858 + 0x25))) &&
            ((*(DAT_005b8858 + 0x21) <= param_4 &&
                (param_4 < *(DAT_005b8858 + 0x21) + *(DAT_005b8858 + 0x29))))) {
            FUN_0047d7d7(DAT_005b8858,param_3,param_4);
            local_28 = DAT_005b8858;
            local_34 = *(DAT_005b8858 + 0x8d);
            local_20 = DAT_005b8858;
            local_30 = *(DAT_005b8858 + 0x91);
            local_24 = local_34;
            local_1c = local_30;
            local_2c = FUN_00472441(DAT_005b8858,local_34,local_30);
            if (local_2c != -0x1) {
                if (_DAT_004c93d8 == 0x0) {
                    if (DAT_004c93d0 == 0x0) {
                        if (_DAT_004c9404 == 0x140) {
                            if ((*(local_2c * 0x9d + DAT_004c9754 * 0x1e22 + 0x5682ac) & 0x1) == 0x0) {
                                pcVar1 = FUN_00499050(DAT_005967ac,0x3f9);
                                FUN_0049d2e0(param_1,0x1,pcVar1);
                            }
                            else {
                                *(&DAT_004c9804 + DAT_004c93d4 * 0x28 + DAT_004c93cc * 0xf8 + DAT_004c9754 * 0xd90) = local_2c;
                                *(&DAT_004c97f8 + DAT_004c93d4 * 0x28 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8) =
                                    0x140;
                                FUN_0049c140(param_1,0x140);
                            }
                        }
                        else {
                            if (_DAT_004c9404 == 0x18d) {
                                local_38 = FUN_004826bc(local_2c);
                                if (local_38 == -0x1) {
                                    pcVar1 = FUN_00499050(DAT_005967ac,0x411);
                                    FUN_0049c2e0(local_138,pcVar1);
                                    FUN_0049d2e0(param_1,0x1,local_138);
                                }
                                else {
                                    if (DAT_005827f4 >> 0x10 == local_38) {
                                        pcVar1 = FUN_00499050(DAT_005967ac,0x40b);
                                        FUN_0049c2e0(local_138,pcVar1);
                                        FUN_0049d2e0(param_1,0x1,local_138);
                                    }
                                    else {
                                        *(&DAT_004c9804 + DAT_004c93d4 * 0x28 + DAT_004c93cc * 0xf8 + DAT_004c9754 * 0xd90) =
                                            local_2c;
                                        *(&DAT_004c97f8 + DAT_004c93d4 * 0x28 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8) =
                                            0x18d;
                                        FUN_0049c140(param_1,0x18d);
                                    }
                                }
                            }
                            else {
                                if (_DAT_004c9404 == 0x18c) {
                                    local_38 = FUN_004826bc(local_2c);
                                    if (local_38 == -0x1) {
                                        pcVar1 = FUN_00499050(DAT_005967ac,0x411);
                                        FUN_0049c2e0(local_138,pcVar1);
                                        FUN_0049d2e0(param_1,0x1,local_138);
                                    }
                                    else {
                                        if (DAT_005827f4 >> 0x10 == local_38) {
                                            *(&DAT_004c9804 + DAT_004c93d4 * 0x28 + DAT_004c93cc * 0xf8 + DAT_004c9754 * 0xd90) =
                                                local_2c;
                                            *(&DAT_004c97f8 + DAT_004c93d4 * 0x28 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8)
                                                = 0x18c;
                                            FUN_0049c140(param_1,0x18c);
                                        }
                                        else {
                                            pcVar1 = FUN_00499050(DAT_005967ac,0x40c);
                                            FUN_0049c2e0(local_138,pcVar1);
                                            FUN_0049d2e0(param_1,0x1,local_138);
                                        }
                                    }
                                }
                            }
                        }
                    }
                    else {
                        *(&DAT_004c978c + DAT_004c93d4 * 0x28 + DAT_004c93cc * 0xf8 + DAT_004c9754 * 0xd90) = local_2c;
                        *(&DAT_004c9780 + DAT_004c93d4 * 0x28 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8) = 0x140;
                        FUN_0049c140(param_1,0x140);
                    }
                }
                else {
                    DAT_004c93dc = local_2c;
                    FUN_0049c140(param_1,0x140);
                }
            }
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
            FUN_0049c140(param_1,0x0);
        }
    }
    return 0x0;
}



fn FUN_0042bf4e() -> u32

{
    let mut iVar1: i32;
    let mut uVar2: u32;
    let local_18c: *mut u32;
    let mut local_138: *mut u32 [0x8];
    let mut local_117: i32;
    let ppuStack243: *mut *mut u8;;
    let mut local_47: String;;
    i32 aiStack64 [0x5];
    let mut local_2c: u32;
    PCHAR local_28;
    let mut local_24: u32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let local_18: *mut i32;;

    local_28 = FUN_0049c2c9(0x104);
    local_18 = FUN_004990e0(local_138,0x0,s_diplo_res_004c1457,s_TreatyDlg_004c144d);
    local_1c = 0x0;
    for (local_20 = 0x0; local_20 < 0x5; local_20 = local_20 + 0x1) {
        aiStack64[local_20] = 0x0;
    }
    for (local_20 = 0x0; local_20 < 0x3; local_20 = local_20 + 0x1) {
        *(&DAT_004c9410 + local_20 * 0x4) = 0x0;
    }
    for (local_20 = 0x0; local_20 < 0x5; local_20 = local_20 + 0x1) {
        if (((local_20 != DAT_004c9754) && (local_20 != DAT_004c93cc)) &&
            (((&DAT_00569a98)[local_20 * 0x1e22] & 0x1) == 0x0)) {
            uVar2 = FUN_0049c2c9(0x2710);
            *(&DAT_004c9410 + local_1c * 0x4) = uVar2;
            FUN_0049c2e0(local_28,s_bin_house_d_bin_004c1461);
            FUN_0049c57b(local_28,(&DAT_004c9410 + local_1c * 0x4),0x2710);
            local_18c = FUN_004a2831(0x5d);
            if (local_18c != 0x0) {
                uVar2 = *(&DAT_004c9410 + local_1c * 0x4);
                local_18c = FUN_0049a030(local_18c,local_138,local_20 + 0x258,local_1c * 0x8c + 0x82,local_117 + 0x5a,0x64,
                                         0x64,0x2,0xcaccce,0xe0e0e);
                $1: &mut String(local_18c + 0x45) = &PTR_FUN_004c3d94;
                *(local_18c + 0x51) = uVar2;
                *(local_18c + 0x55) = 0x0;
                *(local_18c + 0x4d) = 0x0;
                *(local_18c + 0x49) = 0x2;
            }
            aiStack64[local_20] = local_18c;
            FUN_0049bf40(local_138,aiStack64[local_20]);
            local_1c = local_1c + 0x1;
        }
    }
    local_24 = FUN_0049bb50(local_138,FUN_0042c3e9);
    FUN_0049af50(local_28);
    for (local_20 = 0x0; local_20 < 0x5; local_20 = local_20 + 0x1) {
        if ((aiStack64[local_20] != 0x0) && (iVar1 = aiStack64[local_20], iVar1 != 0x0)) {
            ((*(iVar1 + 0x45) + 0x8))(iVar1,0x2);
        }
    }
    for (local_20 = 0x0; local_20 < 0x3; local_20 = local_20 + 0x1) {
        if (*(&DAT_004c9410 + local_20 * 0x4) != 0x0) {
            FUN_0049af50(*(&DAT_004c9410 + local_20 * 0x4));
        }
    }
    local_2c = local_24;
    ppuStack243 = &PTR_FUN_004c3d34;
    if (local_47 != 0x0) {
        FUN_00499b30(local_138,local_47);
    }
    FUN_0049a1c0(local_138,0x1);
    return local_2c;
}



fn FUN_0042c3e9(param_1: i32,param_2: u32,param_3: u32) -> u32

{
    let mut local_20: i32;
    let mut local_1c: i32;

    if (0x404 < param_2) {
        if (param_2 < 0x406) {
            FUN_004953d7();
            local_20 = 0x0;
            for (local_1c = 0x0; local_1c < 0x5; local_1c = local_1c + 0x1) {
                if (((local_1c != DAT_004c9754) && (local_1c != DAT_004c93cc)) &&
                    (((&DAT_00569a98)[local_1c * 0x1e22] & 0x1) == 0x0)) {
                    FUN_00497567(local_20 * 0x8c + 0xb4,*(param_1 + 0x21) + 0xc3,(&DAT_00569b50 + local_1c * 0x1e22),
                                 0x64,0xcaccce,-0x1,0xcaccce,LPCSTR_005b9218,0x11);
                    local_20 = local_20 + 0x1;
                }
            }
            FUN_0049536f();
            return 0x1;
        }
        if (0x406 < param_2) {
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
                    if (((0x259 < param_3) && (0x25a < param_3)) && ((0x25b < param_3 && (param_3 != 0x25c)))) {
                        return 0x0;
                    }
                }
                FUN_0049c140(param_1,param_3);
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



fn FUN_0042c5a2() -> u32

{
    let mut local_120: *mut u32 [0x11];
    let ppuStack219: *mut *mut u8;;
    let mut local_2f: String;;
    let mut local_28: u32;
    u32 **local_24;
    let local_20: *mut i32;;
    let mut local_1c: u32;

    local_20 = FUN_004990e0(local_120,0x0,s_diplo_res_004c147a,s_VotesDlg_004c1471);
    local_28 = FUN_0049bb50(local_120,FUN_0042c699);
    local_24 = local_120;
    local_1c = 0x0;
    ppuStack219 = &PTR_FUN_004c3d34;
    if (local_2f != 0x0) {
        FUN_00499b30(local_120,local_2f);
    }
    FUN_0049a1c0(local_120,0x1);
    return local_28;
}



fn FUN_0042c699(param_1: i32,param_2: i32,param_3: u32) -> u32

{
    if ((param_2 == 0x407) && (0x63 < param_3)) {
        if (param_3 < 0x65) {
            if (DAT_004c93d0 == 0x0) {
                *(&DAT_004c97f8 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8 + DAT_004c93d4 * 0x28) = 0x14a;
            }
            else {
                *(&DAT_004c9780 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8 + DAT_004c93d4 * 0x28) = 0x14a;
            }
            FUN_0049c140(param_1,0x14a);
        }
        else {
            if (param_3 == 0x65) {
                FUN_0049c140(param_1,0x0);
            }
        }
    }
    return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0042c77c(param_1: *mut u32,param_2: i32,param_3: i32,param_4: i32)

{
    let mut pcVar1: String;
    u32 local_124 [0x20];
    u32 local_a4 [0x20];
    let mut local_24: String;
    let mut local_20: String;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: u32;

    local_14 = *param_1;
    if (local_14 < 0x15e) {
        if (local_14 < 0x14a) {
            if (local_14 < 0x136) {
                if (local_14 == 0x12c) {
                    *(&DAT_00569abd + param_3 * 0x1e22) = *(&DAT_00569abd + param_3 * 0x1e22) + param_1[0x1];
                    if (0xf423f < *(&DAT_00569abd + param_3 * 0x1e22)) {
                        *(&DAT_00569abd + param_3 * 0x1e22) = 0xf423f;
                    }
                    *(&DAT_00569abd + param_2 * 0x1e22) = *(&DAT_00569abd + param_2 * 0x1e22) - param_1[0x1];
                    if (*(&DAT_00569abd + param_2 * 0x1e22) < 0x0) {
                        *(&DAT_00569abd + param_2 * 0x1e22) = 0x0;
                    }
                    if (((&DAT_00569a98)[param_3 * 0x1e22] & 0x2) != 0x0) {
                        FUN_0042fcf9((&DAT_00569b14 + param_2 * 0x4 + param_3 * 0x1e22),credits_received_00582928);
                    }
                }
            }
            else {
                if (local_14 < 0x137) {
                    (&DAT_00569c30)[param_1[0x2] * 0x9 + param_3 * 0x1e22] =
                        (&DAT_00569c30)[param_1[0x2] * 0x9 + param_3 * 0x1e22] | 0x1;
                    (&DAT_00569c30)[param_1[0x2] * 0x9 + param_3 * 0x1e22] =
                        (&DAT_00569c30)[param_1[0x2] * 0x9 + param_3 * 0x1e22] & 0xfb;
                    *(&DAT_00569c31 + param_1[0x2] * 0x9 + param_3 * 0x1e22) = 0x0;
                    *(&DAT_00569c35 + param_1[0x2] * 0x9 + param_3 * 0x1e22) = 0x0;
                    FUN_00463f7b();
                    if (((&DAT_00569a98)[param_3 * 0x1e22] & 0x2) != 0x0) {
                        FUN_0042fcf9((&DAT_00569b14 + param_2 * 0x4 + param_3 * 0x1e22),per_tech_received_0058292c);
                    }
                }
                else {
                    if (local_14 == 0x140) {
                        if (param_3 == 0x6) {
                            local_18 = FUN_00481b16(param_1[0x3],0x6,param_2);
                            FUN_0042fcf9((&DAT_00574fe0 + param_2 * 0x4),recv_unfound_city_00582930 * local_18);
                        }
                        else {
                            if (((&DAT_00569a98)[param_3 * 0x1e22] & 0x2) != 0x0) {
                                local_1c = FUN_00481a7f(param_1[0x3],param_3,param_2);
                                FUN_0042fcf9((&DAT_00569b14 + param_2 * 0x4 + param_3 * 0x1e22),
                                             recv_unfound_city_00582930 * local_1c);
                            }
                        }
                        FUN_0043a681(param_1[0x3],param_3,param_2);
                    }
                }
            }
        }
        else {
            if (local_14 < 0x14b) {
                *(&DAT_00569ac5 + param_2 * 0x1e22) = param_3;
                if (((&DAT_00569a98)[param_3 * 0x1e22] & 0x2) != 0x0) {
                    FUN_0042fcf9((&DAT_00569b14 + param_2 * 0x4 + param_3 * 0x1e22),promise_votes_00582934);
                }
            }
            else {
                if (local_14 < 0x14c) {
                    FUN_00462a28(&DAT_004d55a8,param_2,param_3,(param_1 + 0x4));
                }
                else {
                    if (local_14 < 0x14d) {
                        FUN_00462a28(&DAT_004d55a8,param_2,param_3,(param_1 + 0x4));
                        FUN_00499050(DAT_0059679c,param_3 + 0x414);
                        if (param_3 < 0x5) {
                            local_20 = FUN_00499050(DAT_0059679c,0x713e);
                        }
                        else {
                            local_20 = FUN_00499050(DAT_0059679c,0x713a);
                        }
                        FUN_00499050(DAT_0059679c,param_2 + 0x414);
                        if (param_2 < 0x5) {
                            local_24 = FUN_00499050(DAT_0059679c,0x713e);
                        }
                        else {
                            local_24 = FUN_00499050(DAT_0059679c,0x713a);
                        }
                        pcVar1 = FUN_00499050(DAT_0059679c,0x7378);
                        FUN_0049c2e0(local_a4,pcVar1);
                        FUN_0045518a(0x1f,0xffffffff,0x74cc,0xffffffff,local_a4,0xffffffff,0x1);
                    }
                    else {
                        if (local_14 < 0x14e) {
                            if (param_4 == 0x0) {
                                FUN_00462a28(&DAT_004d55a8,param_3,param_2,(param_1 + 0x4));
                            }
                            else {
                                FUN_00462a28(&DAT_004d55a8,param_2,param_1[0x5],(param_1 + 0x4));
                                FUN_00462a28(&DAT_004d55a8,param_3,param_1[0x5],(param_1 + 0x4));
                            }
                        }
                        else {
                            if ((local_14 == 0x14e) && (param_4 != 0x0)) {
                                FUN_00462a28(&DAT_004d55a8,param_2,param_1[0x5],(param_1 + 0x4));
                            }
                        }
                    }
                }
            }
        }
    }
    else {
        if (local_14 < 0x15f) {
            FUN_004083b2(param_1[0x8]);
            _DAT_00582910 = 0x1;
        }
        else {
            if (local_14 < 0x188) {
                if (local_14 < 0x172) {
                    if (local_14 == 0x168) {
                        pcVar1 = FUN_00499050(DAT_0059679c,0x7374);
                        FUN_0049c2e0(local_124,pcVar1);
                        (&DAT_00569a98)[param_2 * 0x1e22] = (&DAT_00569a98)[param_2 * 0x1e22] | 0x20;
                        if (((&DAT_00569a98)[param_3 * 0x1e22] & 0x2) != 0x0) {
                            FUN_0042fcf9((&DAT_00569b14 + param_2 * 0x4 + param_3 * 0x1e22),sign_holy_writ_00582920);
                        }
                        FUN_0045518a(0x1f,0x6,0x7374,0xffffffff,local_124,0xffffffff,0x0);
                    }
                }
                else {
                    if (local_14 < 0x173) {
                        FUN_004082e7(param_1[0x9]);
                    }
                }
            }
            else {
                if (local_14 < 0x189) {
                    (&DAT_00569ad1)[param_2 * 0x1e22 + param_1[0x9]] = param_3;
                }
                else {
                    if (0x18b < local_14) {
                        if (local_14 < 0x18d) {
                            FUN_0048273d(param_1[0x3],*(&DAT_00569ac9 + param_2 * 0x1e22));
                        }
                        else {
                            if (local_14 < 0x18e) {
                                FUN_0048273d(param_1[0x3],(DAT_005827f4 >> 0x10));
                            }
                            else {
                                if (local_14 == 0x18e) {
                                    (&DAT_00569a98)[param_1[0x5] * 0x1e22] = (&DAT_00569a98)[param_1[0x5] * 0x1e22] | 0x40;
                                    *(&DAT_00569ad8 + param_1[0x5] * 0x1e22) = 0xa;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return;
}



fn FUN_0042ce75(param_1: i32,param_2: i32) -> i32

{
let mut bVar1: bool;
let puVar2: *mut u32;
let mut iVar3: i32;
let mut uVar4: u32;
let uVar5: u16;
let mut local_20: i32;
let mut local_1c: i32;
let mut local_18: i32;

uVar5 = 0x0;
uVar4 = 0xffffffff;
puVar2 = FUN_00499050(DAT_005967ac,0x3f2);
FUN_0045518a(0x1 << ((byte)param_2 & 0x1f),param_1,0x74cc,0xffffffff,puVar2,uVar4,uVar5);
bVar1 = false;
local_18 = 0x0;
loop {
if (0x2 < local_18) {
// LAB_0042cf05:
for (local_18 = 0x0; local_18 < 0x3; local_18 = local_18 + 0x1) {
FUN_0042c77c((&DAT_004c9780 + local_18 * 0x28 + param_1 * 0xf8 + param_2 * 0xd90),param_1,param_2,0x1);
}
if (*(&DAT_004c9778 + param_1 * 0xf8 + param_2 * 0xd90) != 0x1) {
for (local_18 = 0x0; local_18 < 0x3; local_18 = local_18 + 0x1) {
FUN_0042c77c((&DAT_004c97f8 + local_18 * 0x28 + param_1 * 0xf8 + param_2 * 0xd90),param_2,param_1,0x0)
;
}
}
iVar3 = FUN_0042d188(param_1,param_2);
if (bVar1) {
for (local_1c = 0x0; local_1c < 0xe; local_1c = local_1c + 0x1) {
for (local_20 = 0x0; local_20 < 0x3; local_20 = local_20 + 0x1) {
if (*(&DAT_004c9780 + local_1c * 0xd90 + DAT_004c9754 * 0xf8 + local_20 * 0x28) == 0x14a) {
FUN_0042d188(DAT_004c9754,local_1c);
break;
}
}
iVar3 = local_1c;
}
}
return iVar3;
}
if (*(&DAT_004c9780 + param_2 * 0xd90 + param_1 * 0xf8 + local_18 * 0x28) == 0x14a) {
bVar1 = true;^
// goto LAB_0042cf05;
}
local_18 = local_18 + 0x1;
} while( true );
}



fn FUN_0042d03d(param_1: i32,param_2: i32)

{
    let puVar1: *mut u32;
    let mut uVar2: u32;
    let uVar3: u16;
    let mut local_18: i32;
    let mut local_14: i32;

    uVar3 = 0x0;
    uVar2 = 0xffffffff;
    puVar1 = FUN_00499050(DAT_005967ac,0x3f4);
    FUN_0045518a(0x1 << ((byte)param_2 & 0x1f),param_1,0x74cc,0xffffffff,puVar1,uVar2,uVar3);
    if (*(&DAT_004c9778 + param_1 * 0xf8 + param_2 * 0xd90) == 0x1) {
        for (local_14 = 0x0; local_14 < 0x3; local_14 = local_14 + 0x1) {
            FUN_0042c77c((&DAT_004c97f8 + local_14 * 0x28 + param_1 * 0xf8 + param_2 * 0xd90),param_2,param_1,0x0);
        }
    }
    else {
        for (local_18 = 0x0; local_18 < 0x3; local_18 = local_18 + 0x1) {
            if (*(&DAT_004c97f8 + param_2 * 0xd90 + param_1 * 0xf8 + local_18 * 0x28) == 0x12c) {
                *(&DAT_00569abd + param_2 * 0x1e22) =
                    *(&DAT_00569abd + param_2 * 0x1e22) +
                        *(&DAT_004c97fc + param_1 * 0xf8 + param_2 * 0xd90 + local_18 * 0x28);
            }
            *(&DAT_004c97f8 + param_2 * 0xd90 + param_1 * 0xf8 + local_18 * 0x28) = 0x0;
        }
    }
    FUN_0042d188(param_1,param_2);
    return;
}



fn FUN_0042d188(param_1: i32,param_2: i32) -> i32

{
let mut iVar1: i32;
let mut local_14: i32;

*(&DAT_004c9778 + param_1 * 0xf8 + param_2 * 0xd90) = 0x0;
iVar1 = param_2 * 0xd90 + param_1 * 0xf8;
*(&DAT_004c977c + iVar1) = 0x1;
for (local_14 = 0x0; local_14 < 0x3; local_14 = local_14 + 0x1) {
if (*(&DAT_004c97f8 + param_2 * 0xd90 + param_1 * 0xf8 + local_14 * 0x28) == 0x12c) {
*(&DAT_00569abd + param_2 * 0x1e22) =
*(&DAT_00569abd + param_2 * 0x1e22) +
*(&DAT_004c97fc + param_1 * 0xf8 + param_2 * 0xd90 + local_14 * 0x28);
}
FUN_004a0430(&DAT_004c9780 + local_14 * 0x28 + param_1 * 0xf8 + param_2 * 0xd90,0x0,0x28);
FUN_004a0430(&DAT_004c97f8 + local_14 * 0x28 + param_1 * 0xf8 + param_2 * 0xd90,0x0,0x28);
iVar1 = local_14;
}
return iVar1;
}



fn FUN_0042d295(param_1: i32) -> i32

{
let mut iVar1: i32;
let mut local_24: i32;
let mut local_1c: i32;
let mut local_18: i32;
let local_14: *mut u32;

local_1c = 0x0;
for (local_18 = 0x0; local_18 < 0x28; local_18 = local_18 + 0x1) {
if (*(&DAT_005b70c2 + local_18 * 0x4e) != 0x0) {
for (local_24 = 0x0; local_24 < 0xd; local_24 = local_24 + 0x1) {
iVar1 = FUN_00462571((&DAT_00568210 + param_1 * 0x1e22),local_18,local_24);
local_1c = local_1c + iVar1 * *(&DAT_004d6a74 + local_24 * 0xa8);
}
}
}
for (local_14 = *DAT_005967c8; local_14 != 0x0; local_14 = *local_14) {
if (*(local_14 + 0xe) >> 0x10 == DAT_004c9754) {
local_1c = local_1c + *(&DAT_00583224 + (local_14[0x3] >> 0x10) * 0x50);
}
}
return local_1c / 0x3;
}



fn FUN_0042d379(param_1: i32,param_2: i32) -> i32

{
let mut local_14: i32;

local_14 = 0x0;
loop {
if (0x2 < local_14) {
return -0x1;
}
if (param_1 == 0x0) {
if (*(&DAT_004c97f8 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8 + local_14 * 0x28) == param_2) {
return local_14;
}
}
else {
if (*(&DAT_004c9780 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8 + local_14 * 0x28) == param_2) {
return local_14;
}
}
local_14 = local_14 + 0x1;
} while( true );
}



fn FUN_0042d418()

{
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0x3; local_14 = local_14 + 0x1) {
        if (*(&DAT_004c97f8 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8 + local_14 * 0x28) == 0x12c) {
            *(&DAT_00569abd + DAT_004c9754 * 0x1e22) =
                *(&DAT_00569abd + DAT_004c9754 * 0x1e22) +
                    *(&DAT_004c97fc + DAT_004c93cc * 0xf8 + DAT_004c9754 * 0xd90 + local_14 * 0x28);
        }
        *(&DAT_004c97f8 + DAT_004c9754 * 0xd90 + DAT_004c93cc * 0xf8 + local_14 * 0x28) = 0x0;
    }
    return;
}



fn FUN_0042d4ca(param_1: i32,param_2: i32) -> i32

{
let mut local_14: i32;

local_14 = 0x1;
while( true ) {
if (0x71 < local_14) {
return 0x0;
}
if (((*(&DAT_0058ad72 + local_14 * 0xda) == 0x0) &&
(((&DAT_00569c30)[local_14 * 0x9 + param_2 * 0x1e22] & 0x1) != 0x0)) &&
(((&DAT_00569c30)[local_14 * 0x9 + param_1 * 0x1e22] & 0x1) == 0x0)) break;
local_14 = local_14 + 0x1;
}
return local_14;
}



fn check_reg_val_0042d54b(LPBYTE byte_pointer) -> u32

{
    LSTATUS LVar1;
    let local_24: u32;
    DWORD local_20 [0x2];
    HKEY local_18;
    LSTATUS local_14;

    // PHKEY phkResult for RegOpenKeyExA
    // REGSAM samDesired for RegOpenKeyExA
    // DWORD ulOptions for RegOpenKeyExA
    // LPCSTR lpSubKey for RegOpenKeyExA
    // HKEY hKey for RegOpenKeyExA
    local_14 = RegOpenKeyExA((HKEY)0x80000002,s_Software_HDI_Emperor_of_the_Fadi_004c1484,0x0,0x20019,&local_18);
    if (local_14 == 0x0) {
        local_20[0] = 0x64;
        // LPDWORD lpcbData for RegQueryValueExA
        // LPBYTE lpData for RegQueryValueExA
        // LPDWORD lpType for RegQueryValueExA
        // LPDWORD lpReserved for RegQueryValueExA
        // LPCSTR lpValueName for RegQueryValueExA
        // HKEY hKey for RegQueryValueExA
        LVar1 = RegQueryValueExA(local_18,s_CDROM_004c14b0,(LPDWORD)0x0,&local_24,byte_pointer,local_20);
        if ((LVar1 == 0x0) && (local_24 == 0x1)) {
            return 0x1;
        }
    }
    return 0x0;
}



fn FUN_0042d5cf(LPBYTE param_1) -> u32

{
    LSTATUS LVar1;
    let local_24: u32;
    DWORD local_20 [0x2];
    HKEY local_18;
    LSTATUS local_14;

    // PHKEY phkResult for RegOpenKeyExA
    // REGSAM samDesired for RegOpenKeyExA
    // DWORD ulOptions for RegOpenKeyExA
    // LPCSTR lpSubKey for RegOpenKeyExA
    // HKEY hKey for RegOpenKeyExA
    local_14 = RegOpenKeyExA((HKEY)0x80000002,s_Software_HDI_Emperor_of_the_Fadi_004c14b6,0x0,0x20019,&local_18);
    if (local_14 == 0x0) {
        local_20[0] = 0x64;
        // LPDWORD lpcbData for RegQueryValueExA
        // LPBYTE lpData for RegQueryValueExA
        // LPDWORD lpType for RegQueryValueExA
        // LPDWORD lpReserved for RegQueryValueExA
        // LPCSTR lpValueName for RegQueryValueExA
        // HKEY hKey for RegQueryValueExA
        LVar1 = RegQueryValueExA(local_18,s_Directory_004c14e2,(LPDWORD)0x0,&local_24,param_1,local_20);
        if ((LVar1 == 0x0) && (local_24 == 0x1)) {
            return 0x1;
        }
    }
    return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn important_func_0042d653(HINSTANCE param_1,DWORD param_2,param_3: u32,DWORD param_4) -> u32

{
    let cVar1: u8;
    let BVar2: u8;
    UCHAR UVar3;
    let game_installed: u32;
    let dVar4: u32;
    let mut pcVar5: String;
    let mut uVar6: u32;
    PCHAR str_param_1;
    PCHAR new_file_name;
    let piVar7: *mut i32;;
    let mut iVar8: i32;
    BYTE *pBVar9;
    let mut string_11: String;
    BYTE *pBVar10;
    UCHAR *pBVar11;
    let mut pcVar11: String;
    let local_57c: u8 [0x104];
    let mut local_478: u32;
    let mut local_474: i32;
    let local_470: u8 [0x80];
    let mut local_3f0: i32;
    let mut local_3ec: u32;
    let local_3e8: *mut i32;;
    HWND local_3e4;
    LPCSTR *local_3e0;
    LPCSTR *local_3dc;
    LPCSTR *local_3d8;
    let local_3d4: *mut i32;;
    let local_3d0: *mut i32;;
    let local_3cc: *mut i32;;
    let local_3c8: *mut i32;;
    let local_3c4: *mut i32;;
    let mut local_3c0: String;
    let mut local_3bc: String;
    let mut local_3b8: u32;
    let local_3b4: u8 [0x100];
    HANDLE local_2b4 [0x48];
    let local_194: u8 [0x80];
    let local_114: u8 [0x80];
    let local_94: u8 [0x40];
    let local_54: *mut i32;;
    let local_50: u32;
    UINT save_turns_in_log_opt;
    let local_48: *mut i32;;
    let local_44: *mut i32;;
    let local_40: *mut i32;;
    let local_3c: *mut i32;;
    let local_38: *mut i32;;
    let local_34: *mut i32;;
    let local_30: *mut i32;;
    let local_2c: *mut i32;;
    let local_28: *mut i32;;
    let mut local_24: u32;
    let mut local_20: i32;
    let local_1c: *mut i32;;
    let local_18: *mut i32;;
    UCHAR char_val_2;

    DAT_004c9720 = FUN_0049c2c9(0x40);
    game_installed = check_reg_val_0042d54b(local_114);
    if (game_installed == 0x0) {
        // UINT uType for MessageBoxA
        // LPCSTR lpCaption for MessageBoxA
        // LPCSTR lpText for MessageBoxA
        // HWND hWnd for MessageBoxA
        MessageBoxA((HWND)0x0,s_Game_not_installed__please_run_s_004c14f9,s_Installation_004c14ec,0x10);
        local_50 = 0x0;
    }
    else {
        find_file_fn_004a3a00(*DAT_005b9c8c,(HANDLE)0x0,local_2b4);
        FUN_0049c2e0(DAT_004c9720,s___d__d__d___2d___2d__004c151f);
        FUN_0049c2e0(local_194,s_Emperor_of_the_Fading_Suns__s_004c1534);
        FUN_0049c2e0(local_94,s__sefs_004c1552);
        dVar4 = create_window_fn_0049eee0(param_1,param_2,param_4,local_94,local_194,s_EFS_PAL_004c1558);
        if (dVar4 != 0x0) {
            FUN_0049c7b5(s_dat_efs_ini_004c1560);
            FUN_0049c2e0(&filename_00599c80,&DAT_004c156c);
            // LPCSTR lpFileName for GetPrivateProfileIntA
            // INT nDefault for GetPrivateProfileIntA
            // LPCSTR lpKeyName for GetPrivateProfileIntA
            // LPCSTR lpAppName for GetPrivateProfileIntA
            video_on_opt_00599dd8 =
                GetPrivateProfileIntA(s_Options_004c1578,s_video_on_004c156f,0x1,&filename_00599c80);
            // LPCSTR lpFileName for GetPrivateProfileIntA
            // INT nDefault for GetPrivateProfileIntA
            // LPCSTR lpKeyName for GetPrivateProfileIntA
            // LPCSTR lpAppName for GetPrivateProfileIntA
            hide_ai_opt_004d55a4 = GetPrivateProfileIntA(s_Computer_004c1587,s_HideAI_004c1580,0x0,&filename_00599c80)
            ;
            // LPCSTR lpFileName for GetPrivateProfileIntA
            // INT nDefault for GetPrivateProfileIntA
            // LPCSTR lpKeyName for GetPrivateProfileIntA
            // LPCSTR lpAppName for GetPrivateProfileIntA
            save_turns_in_log_opt =
                GetPrivateProfileIntA(s_Options_004c159f,s_SaveTurnsInLog_004c1590,0x0,&filename_00599c80);
            if (video_on_opt_00599dd8 != 0x0) {
                FUN_004953d7();
                pBVar9 = local_114;
                pBVar10 = local_3b4;
                loop {
                    BVar2 = *pBVar9;
                    *pBVar10 = BVar2;
                    if (BVar2 == '\0') break;
                    BVar2 = pBVar9[0x1];
                    pBVar9 = pBVar9 + 0x2;
                    pBVar10[0x1] = BVar2;
                    pBVar10 = pBVar10 + 0x2;
                } while (BVar2 != '\0');
                string_11 = s_smackplw_exe_sega_smk__g__v2__u1_004c15a7;
                iVar8 = -0x1;
                pBVar9 = local_3b4;
                loop {
                    pBVar10 = pBVar9;
                    if (iVar8 == 0x0) break;
                    iVar8 = iVar8 + -0x1;
                    pBVar10 = pBVar9 + 0x1;
                    BVar2 = *pBVar9;
                    pBVar9 = pBVar10;
                } while (BVar2 != '\0');
                pBVar11 = pBVar10 + -0x1;
                loop {
                    UVar3 = *string_11;
                    *pBVar11 = UVar3;
                    if (UVar3 == '\0') break;
                    char_val_2 = ((UCHAR *)string_11)[0x1];
                    string_11 = ((UCHAR *)string_11 + 0x2);
                    pBVar11[0x1] = char_val_2;
                    pBVar11 = pBVar11 + 0x2;
                } while (char_val_2 != '\0');
                create_proc_004a3b16(local_114,0x0,(LPSTR)local_3b4);
                pBVar9 = local_114;
                pBVar10 = local_3b4;
                loop {
                    BVar2 = *pBVar9;
                    *pBVar10 = BVar2;
                    if (BVar2 == '\0') break;
                    BVar2 = pBVar9[0x1];
                    pBVar9 = pBVar9 + 0x2;
                    pBVar10[0x1] = BVar2;
                    pBVar10 = pBVar10 + 0x2;
                } while (BVar2 != '\0');
                string_11 = s_smackplw_exe_hdilogo1_smk__g__v2_004c15c8;
                iVar8 = -0x1;
                pBVar9 = local_3b4;
                loop {
                    pBVar10 = pBVar9;
                    if (iVar8 == 0x0) break;
                    iVar8 = iVar8 + -0x1;
                    pBVar10 = pBVar9 + 0x1;
                    BVar2 = *pBVar9;
                    pBVar9 = pBVar10;
                } while (BVar2 != '\0');
                pBVar10 = pBVar10 + -0x1;
                loop {
                    BVar2 = *string_11;
                    *pBVar10 = BVar2;
                    if (BVar2 == '\0') break;
                    BVar2 = (string_11)[0x1];
                    string_11 = (string_11 + 0x2);
                    pBVar10[0x1] = BVar2;
                    pBVar10 = pBVar10 + 0x2;
                } while (BVar2 != '\0');
                create_proc_004a3b16(local_114,0x0,(LPSTR)local_3b4);
                pBVar9 = local_114;
                pBVar10 = local_3b4;
                loop {
                    BVar2 = *pBVar9;
                    *pBVar10 = BVar2;
                    if (BVar2 == '\0') break;
                    BVar2 = pBVar9[0x1];
                    pBVar9 = pBVar9 + 0x2;
                    pBVar10[0x1] = BVar2;
                    pBVar10 = pBVar10 + 0x2;
                } while (BVar2 != '\0');
                string_11 = s_smackplw_exe_introsnd_smk__g__v2_004c15ed;
                iVar8 = -0x1;
                pBVar9 = local_3b4;
                loop {
                    pBVar10 = pBVar9;
                    if (iVar8 == 0x0) break;
                    iVar8 = iVar8 + -0x1;
                    pBVar10 = pBVar9 + 0x1;
                    BVar2 = *pBVar9;
                    pBVar9 = pBVar10;
                } while (BVar2 != '\0');
                pBVar10 = pBVar10 + -0x1;
                loop {
                    BVar2 = *string_11;
                    *pBVar10 = BVar2;
                    if (BVar2 == '\0') break;
                    BVar2 = (string_11)[0x1];
                    string_11 = (string_11 + 0x2);
                    pBVar10[0x1] = BVar2;
                    pBVar10 = pBVar10 + 0x2;
                } while (BVar2 != '\0');
                create_proc_004a3b16(local_114,0x0,(LPSTR)local_3b4);
                FUN_00496404(DAT_005b96c8);
                FUN_0049536f();
            }
            // BOOL bShow for ShowCursor
            ShowCursor(0x1);
            // BOOL bShow for ShowCursor
            ShowCursor(0x0);
            iVar8 = FUN_004a3c10(0x0);
            FUN_004a2f00(iVar8);
            FUN_00498999(0x0,0x0,0x280,0x1e0);
            local_3b8 = 0x272727;
            DAT_004bf958 = 0x272727;
            FUN_0049aee0();
            local_3bc = s_pcx_bg_004c1612;
            DAT_005b9824 = s_pcx_bg_004c1612;
            DAT_005b9b70 = 0xcaccce;
            DAT_004bf948 = 0xe0e0e;
            DAT_005b9b74 = 0xcaccce;
            _DAT_005b9b78 = 0xcaccce;
            DAT_004bf924 = 0xce;
            DAT_005b981c = 0xca;
            DAT_004bf928 = 0xcc;
            _DAT_005b96c0 = FUN_00401660;
            _DAT_005b96c4 = 0x0;
            local_3c0 = DAT_005b96c8;
            FUN_004a3c48(DAT_005b96c8,&DAT_005b8df0,0x20);
            DAT_004d6a6c = FUN_004971ad(s_char77_004c1619);
            FUN_004954f3(0x3,0x3);
            FUN_00495a10(0x3,0x3);
            local_3c4 = FUN_004a2831(0x8);
            local_48 = local_3c4;
            local_44 = local_3c4;
            if (local_3c4 != 0x0) {
                local_3c4 = FUN_00498ee0(local_3c4,s_strings_res_004c162a,s_glStrings_004c1620);
            }
            DAT_0059679c = local_3c4;
            local_3c8 = FUN_004a2831(0x8);
            local_40 = local_3c8;
            local_3c = local_3c8;
            if (local_3c8 != 0x0) {
                local_3c8 = FUN_00498ee0(local_3c8,s_strings_res_004c1640,s_glButtons_004c1636);
            }
            DAT_005967a0 = local_3c8;
            local_3cc = FUN_004a2831(0x8);
            local_38 = local_3cc;
            local_34 = local_3cc;
            if (local_3cc != 0x0) {
                local_3cc = FUN_00498ee0(local_3cc,s_tutstr_res_004c1657,s_glTutorial_004c164c);
            }
            DAT_005967a8 = local_3cc;
            local_3d0 = FUN_004a2831(0x8);
            local_30 = local_3d0;
            local_2c = local_3d0;
            if (local_3d0 != 0x0) {
                local_3d0 = FUN_00498ee0(local_3d0,s_compstr_res_004c166c,s_glCompStr_004c1662);
            }
            DAT_005967a4 = local_3d0;
            local_3d4 = FUN_004a2831(0x8);
            local_54 = local_3d4;
            local_28 = local_3d4;
            if (local_3d4 != 0x0) {
                local_3d4 = FUN_00498ee0(local_3d4,s_diplostr_res_004c1683,s_glDiploStr_004c1678);
            }
            DAT_005967ac = local_3d4;
            DAT_005967b0 = FUN_00485cd6();
            DAT_005967c8 = FUN_004815e3();
            // LPCSTR lpFileName for GetPrivateProfileIntA
            // INT nDefault for GetPrivateProfileIntA
            // LPCSTR lpKeyName for GetPrivateProfileIntA
            // LPCSTR lpAppName for GetPrivateProfileIntA
            _DAT_00599ddc = GetPrivateProfileIntA(s_Options_004c169c,s_CD_Disabled_004c1690,0x0,&filename_00599c80);
            if (_DAT_00599ddc == 0x0) {
                local_3e0 = FUN_004a2831(0x42);
                local_3dc = local_3e0;
                local_3d8 = local_3e0;
                if (local_3e0 != 0x0) {
                    local_3e0 = mci_dev_fn_004a3db0(local_3e0,s_cdlist_txt_004c16a4);
                }
                DAT_004d6a68 = local_3e0;
            }
            else {
                DAT_004d6a68 = 0x0;
            }
            FUN_00462b69(&DAT_005967d0);
            FUN_00494639();
            read_set_options_004940cb();
            FUN_00493b56();
            FUN_00447024();
            FUN_00460b74();
            FUN_004a11e2();
            if (sound_opt_00599de8 == 0x0) {
                FUN_004a12c7();
            }
            else {
                local_3e4 = hWnd_005b9be0;
                FUN_004a1243();
            }
            for (local_20 = 0x0; local_20 < 0x5; local_20 = local_20 + 0x1) {
                FUN_00445e54(local_20);
                FUN_00445d2f(local_20);
            }
            local_3e8 = FUN_004a2831(0x95);
            local_1c = local_3e8;
            local_18 = local_3e8;
            if (local_3e8 != 0x0) {
                local_3e8 = FUN_0047157e(local_3e8,0x0,0x270f,0x81,0x1d,0x1ee,0x1b3,0x4000,0x22);
            }
            DAT_00596a38 = local_3e8;
            if (DAT_004d6a68 != 0x0) {
                FUN_004a5130(DAT_004d6a68,0x2);
            }
            LAB_0042dded:
                FUN_004953d7();
            FUN_00496a10();
            FUN_0049536f();
            FUN_0047f618();
            _DAT_004c9728 = 0x0;
            local_3ec = FUN_00480382();
            local_3f0 = local_3ec - 0x64;
            local_24 = local_3ec;
            switch(local_3f0) {
                case 0x0:
                    FUN_00480f18();
                FUN_004953d7();
                FUN_00496a10();
                FUN_0049536f();
                default:
                    _DAT_00596a50 = 0x0;
                DAT_004d557c = FUN_0042f6fe();
                _DAT_004d5580 = FUN_0042f693();
                FUN_0047fb0a();
                break;
                case 0x1:
                    _DAT_004c9728 = 0x1;
                break;
                case 0x3:
                    case 0x4:
                    LAB_0042e3dd:
                if (sound_opt_00599de8 != 0x0) {
                    FUN_004a12c7();
                }
                if (DAT_00599d00 != 0x0) {
                    FUN_0049af50(DAT_00599d00);
                }
                if (DAT_004d6a68 != 0x0) {
                    FUN_004a52d0(DAT_004d6a68);
                }
                return 0x0;
            }
            LAB_0042dea5:
                DAT_004d557c = FUN_0042f6fe();
            _DAT_004d5580 = FUN_0042f693();
            FUN_00430604();
            DAT_004c9758 = FUN_00430815(DAT_004c9754);
            LAB_0042ded1:
            if ((DAT_004c9754 == 0x0) &&
                (((_DAT_004c9728 == 0x0 || ((DAT_004d559c & 0x2) != 0x0)) && (FUN_0042e42d(), DAT_00595738 != -0x1))))^
            // goto LAB_0042dded;
            if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x1) == 0x0) {
                _DAT_00596a48 = 0x0;
                _DAT_00596a44 = 0x0;
                if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
                    DAT_004c9758 = DAT_004c9754;
                    FUN_0042433b();
                    local_478 = FUN_0045ddf0();
                    if (local_478 == 0x66)^ // goto LAB_0042e3dd;
                    if (local_478 == 0x3e9) {
                        FUN_00472275();^
                        // goto LAB_0042dded;
                    }
                    if (local_478 == 0x3e8)^ // goto code_r0x0042e0ee;
                }
                else {
                    FUN_00423f15();
                    FUN_00412bbf();
                }
            }
            else {
                if ((*(&DAT_00569a9d + DAT_004c9754 * 0x1e22) != 0x0) &&
                    (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0)) {
                    *(&DAT_00569a9d + DAT_004c9754 * 0x1e22) = 0x0;
                    FUN_00499050(DAT_0059679c,DAT_004c9754 + 0x414);
                    string_11 = FUN_00499050(DAT_0059679c,0x73cc);
                    FUN_0049c2e0(local_470,string_11);
                    pcVar5 = FUN_00499050(DAT_0059679c,0x73f0);
                    iVar8 = -0x1;
                    string_11 = local_470;
                    loop {
                        pcVar11 = string_11;
                        if (iVar8 == 0x0) break;
                        iVar8 = iVar8 + -0x1;
                        pcVar11 = string_11 + 0x1;
                        cVar1 = *string_11;
                        string_11 = pcVar11;
                    } while (cVar1 != '\0');
                    pcVar11 = pcVar11 + -0x1;
                    loop {
                        cVar1 = *pcVar5;
                        *pcVar11 = cVar1;
                        if (cVar1 == '\0') break;
                        cVar1 = pcVar5[0x1];
                        pcVar5 = pcVar5 + 0x2;
                        pcVar11[0x1] = cVar1;
                        pcVar11 = pcVar11 + 0x2;
                    } while (cVar1 != '\0');
                    FUN_00455e22(local_470,-0x1,0x73f0);
                    local_474 = 0x0;
                    while( true ) {
                        if (0x4 < local_474) {
                            FUN_00472275();^
                            // goto LAB_0042dded;
                        }
                        if ((((&DAT_00569a98)[local_474 * 0x1e22] & 0x1) == 0x0) &&
                            (((&DAT_00569a98)[local_474 * 0x1e22] & 0x2) == 0x0)) break;
                        local_474 = local_474 + 0x1;
                    }
                }
            }
            iVar8 = DAT_004c9754;
            DAT_004c9754 = DAT_004c9754 + 0x1;
            if (iVar8 == 0xd) {
                DAT_004c9754 = 0x0;
                FUN_0042e583();
                if (DAT_00595738 != -0x1) {
                    FUN_0042433b();^
                    // goto LAB_0042dded;
                }
            }
            if (save_turns_in_log_opt != 0x0) {
                FUN_0042f769(save_turns_in_log_opt);
            }
            if ((((DAT_004d559c & 0x4) != 0x0) && (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x1) == 0x0)) &&
                (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0)) {
                uVar6 = find_file_fn_0045bde0(s_sav___sav_004c16af,local_57c,0x1);
                if (uVar6 != 0x0) {
                    uVar6 = DAT_004d559c | 0x2;
                    iVar8 = FUN_00430bce(local_57c);
                    iVar8 = FUN_0047bf4b(DAT_00596a38,iVar8,uVar6);
                    if (iVar8 != 0x0) {
                        string_11 = FUN_00499050(DAT_0059679c,0x7395);
                        FUN_0049d2e0(0x0,0x1,string_11);
                    }
                }
                FUN_00472275();^
                // goto LAB_0042dded;
            }
            if (DAT_004c9754 == 0x0) {
                _DAT_004c9740 = _DAT_004c9740 ^ 0x1;
            }
            else {
                if (((autosave_opt_00599df0 != 0x0) && (DAT_004c9754 + -0x1 < 0x5)) &&
                    (((&DAT_00569a98)[(DAT_004c9754 + -0x1) * 0x1e22] & 0x2) == 0x0)) {
                    if (DAT_00599d00 == 0x0) {
                        DAT_00599d00 = FUN_0049c2c9(0x104);
                        _DAT_004c9740 = 0x1;
                    }
                    str_param_1 = FUN_0049c2c9(0x104);
                    new_file_name = FUN_0049c2c9(0x104);
                    FUN_0049c2e0(DAT_00599d00,s__3_3s_2_2d_2_2d_1_1s_004c16bb);
                    string_11 = FUN_0049c7b5(s_sav__s_sav_004c16d0);
                    FUN_0049c2e0(str_param_1,string_11);
                    string_11 = FUN_0049c7b5(s_sav__3_3s_2_2d_2_2d_1_1s_sav_004c16dd);
                    FUN_0049c2e0(new_file_name,string_11);
                    piVar7 = FUN_0049c4bd(str_param_1,&DAT_004c16fa);
                    if (piVar7 != 0x0) {
                        FUN_0049ca40(piVar7);
                        FUN_004a5420(new_file_name);
                        move_file_004a5430(str_param_1,new_file_name);
                    }
                    FUN_0047bf4b(DAT_00596a38,DAT_00599d00,DAT_004d559c | 0x2);
                    FUN_0049af50(str_param_1);
                    FUN_0049af50(new_file_name);
                }
            }^
            // goto LAB_0042ded1;
        }
        local_50 = 0x1;
    }
    return local_50;
    code_r0x0042e0ee:
        _DAT_004c9728 = 0x1;^
    // goto LAB_0042dea5;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0042e42d()

{
    let puVar1: *mut u32;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let uVar4: u16;
    let mut local_18: i32;
    let mut local_14: i32;

    iVar3 = _DAT_004d5560;
    _DAT_004c9744 = 0x0;
    _DAT_004d5560 = _DAT_004d5560 + -0x1;
    if (_DAT_004d5560 == 0x0) {
        for (local_18 = 0x0; local_18 < 0x5; local_18 = local_18 + 0x1) {
            *(&DAT_004d5568 + local_18 * 0x4) = 0x0;
        }
        _DAT_004d5560 = turns_till_vote_opt_00599d44;
        _DAT_00596a50 = 0x1;
    }
    else {
        if (iVar3 == 0x2) {
            uVar4 = 0x0;
            uVar2 = 0xffffffff;
            puVar1 = FUN_00499050(DAT_0059679c,0x73cb);
            FUN_0045518a(0x1f,0xffffffff,0x74cc,0xffffffff,puVar1,uVar2,uVar4);
        }
    }
    if (0x9 < DAT_00591cb4) {
        _DAT_004d5564 = _DAT_004d5564 + -0x1;
        if (_DAT_004d5564 != 0x0) {
            uVar2 = FUN_004a2edc();
            if (uVar2 % turns_till_patriarch_dies_00599d48 != 0x0)^ // goto LAB_0042e4fb;
        }
        FUN_0042fa19();
    }
    LAB_0042e4fb:
        local_14 = 0x0;
    for (local_18 = 0x0; local_18 < 0x5; local_18 = local_18 + 0x1) {
        if (((&DAT_00569a98)[local_18 * 0x1e22] & 0x1) == 0x0) {
            iVar3 = FUN_00485fe3(local_18);
            if (iVar3 == 0x0) {
                local_14 = local_14 + 0x1;
                FUN_004606b6(local_18);
            }
            else {
                DAT_00595738 = local_18;
            }
        }
        else {
            local_14 = local_14 + 0x1;
        }
    }
    if (local_14 < 0x4) {
        DAT_00595738 = -0x1;
    }
    if (DAT_00595738 != -0x1) {
        FUN_00430868();
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0042e583()

{
    let mut iVar1: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    DAT_00591cb4 = DAT_00591cb4 + 0x1;
    if (_DAT_00596a50 != 0x0) {
        FUN_0040581a();
        _DAT_00596a50 = 0x0;
    }
    if (DAT_00595738 == -0x1) {
        local_18 = 0x0;
        for (local_14 = 0x0; local_14 < 0x5; local_14 = local_14 + 0x1) {
            if (((&DAT_00569a98)[local_14 * 0x1e22] & 0x1) == 0x0) {
                iVar1 = FUN_00485fe3(local_14);
                if (iVar1 == 0x0) {
                    local_18 = local_18 + 0x1;
                    FUN_004606b6(local_14);
                }
                else {
                    DAT_00595738 = local_14;
                }
            }
            else {
                local_18 = local_18 + 0x1;
            }
        }
        if (local_18 < 0x4) {
            DAT_00595738 = -0x1;
        }
    }
    if (DAT_00595738 != -0x1) {
        FUN_00430868();
    }
    return;
}



fn FUN_0042e642(param_1: *mut *mut u32)

{
    FUN_0049bf80(param_1,0x521,0x410,0x0,0x0);
    return;
}



fn FUN_0042e670(param_1: i32,param_2: *mut *mut u32)

{
    ushort uVar1;
    let mut pcVar2: String;

    uVar1 = (param_1 + 0xe);
    if (uVar1 < 0x4) {
        if (uVar1 == 0x1) {
            if (*(param_1 + 0xe) >> 0x10 == DAT_004c9754) {
                pcVar2 = FUN_00499050(DAT_005967a0,0x577);
                FUN_0049bf80(param_2,0x52c,0x40f,0x0,pcVar2);
                FUN_0049bf80(param_2,0x52c,0x414,0x1,0x0);
                return;
            }
            FUN_0049bf80(param_2,0x52c,0x414,0x0,0x0);
            FUN_0049bf80(param_2,0x52c,0x410,0x0,0x0);
            return;
        }
    }
    else {
        if (uVar1 < 0x5) {
            if ((param_1 + 0x10) != 0x5) {
                return;
            }
            pcVar2 = FUN_00499050(DAT_005967a0,0x575);
            FUN_0049bf80(param_2,0x52c,0x40f,0x0,pcVar2);
            FUN_0049bf80(param_2,0x52c,0x414,0x1,0x0);
            return;
        }
        if (uVar1 == 0x17) {
            if (*(param_1 + 0xe) >> 0x10 == DAT_004c9754) {
                pcVar2 = FUN_00499050(DAT_005967a0,0x576);
                FUN_0049bf80(param_2,0x52c,0x40f,0x0,pcVar2);
                FUN_0049bf80(param_2,0x52c,0x414,0x1,0x0);
                return;
            }
            FUN_0049bf80(param_2,0x52c,0x414,0x0,0x0);
            FUN_0049bf80(param_2,0x52c,0x410,0x0,0x0);
            return;
        }
    }
    FUN_0049bf80(param_2,0x52c,0x414,0x0,0x0);
    FUN_0049bf80(param_2,0x52c,0x410,0x0,0x0);
    return;
}



// WARNING: Removing unreachable block (ram,0x0042f599)
// WARNING: Removing unreachable block (ram,0x0042f4dd)
// WARNING: Removing unreachable block (ram,0x0042f254)
// WARNING: Removing unreachable block (ram,0x0042f0d3)
// WARNING: Removing unreachable block (ram,0x0042f013)
// WARNING: Removing unreachable block (ram,0x0042edb5)
// WARNING: Removing unreachable block (ram,0x0042ecf9)
// WARNING: Removing unreachable block (ram,0x0042ebbb)
// WARNING: Removing unreachable block (ram,0x0042eaff)
// WARNING: Removing unreachable block (ram,0x0042e988)
// WARNING: Removing unreachable block (ram,0x0042e908)
// WARNING: Removing unreachable block (ram,0x0042e948)
// WARNING: Removing unreachable block (ram,0x0042e9c8)
// WARNING: Removing unreachable block (ram,0x0042eb5d)
// WARNING: Removing unreachable block (ram,0x0042ec19)
// WARNING: Removing unreachable block (ram,0x0042ed57)
// WARNING: Removing unreachable block (ram,0x0042ee07)
// WARNING: Removing unreachable block (ram,0x0042f067)
// WARNING: Removing unreachable block (ram,0x0042f2b2)
// WARNING: Removing unreachable block (ram,0x0042f412)
// WARNING: Removing unreachable block (ram,0x0042f53b)
// WARNING: Removing unreachable block (ram,0x0042f5f7)

fn FUN_0042e871(param_1: *mut *mut u32)

{
    let mut bVar1: bool;
    let mut iVar2: i32;
    undefined3 extraout_var;
    let mut iVar3: i32;
    let puVar4: *mut u32;
    let mut bVar5: bool;
    let mut local_208: u32;
    let mut local_1a0: u32;

    if (DAT_005967bc == 0x0) {
        FUN_0042e642(param_1);
    }
    else {
        bVar5 = false;
        iVar2 = FUN_0049c160(param_1,0x26ac);
        if (*(DAT_005967bc + 0x23) >> 0x18 == DAT_004c9754) {
            FUN_004a6e10(iVar2,0x51e,0x2,0x0);
            FUN_004a6e10(iVar2,0x523,0x2,0x0);
            FUN_004a6e10(iVar2,0x524,0x2,0x0);
            FUN_004a6e10(iVar2,0x522,0x2,0x0);
            FUN_0049bf80(param_1,0x51e,0x40f,0x0,0x0);
            FUN_0049bf80(param_1,0x523,0x40f,0x0,0x0);
            FUN_0049bf80(param_1,0x524,0x40f,0x0,0x0);
            FUN_0049bf80(param_1,0x522,0x40f,0x0,0x0);
            bVar1 = FUN_00432fca(&DAT_005967b8,0x1);
            if (((CONCAT31(extraout_var,bVar1) == 0x0) || (iVar3 = FUN_00434bfa(&DAT_005967b8), iVar3 != 0x0)) ||
                (iVar3 = FUN_00432bd3(&DAT_005967b8), iVar3 == 0x0)) {
                FUN_0049bf80(param_1,0x521,0x410,0x0,0x0);
            }
            else {
                FUN_0049bf80(param_1,0x521,0x40f,0x0,0x0);
            }
        }
        else {
            FUN_004a6e10(iVar2,0x51e,0x2,0x1);
            FUN_004a6e10(iVar2,0x523,0x2,0x1);
            FUN_004a6e10(iVar2,0x524,0x2,0x1);
            FUN_004a6e10(iVar2,0x522,0x2,0x1);
            FUN_0049bf80(param_1,0x51e,0x410,0x0,0x0);
            FUN_0049bf80(param_1,0x523,0x410,0x0,0x0);
            FUN_0049bf80(param_1,0x524,0x410,0x0,0x0);
            FUN_0049bf80(param_1,0x522,0x410,0x0,0x0);
            FUN_0049bf80(param_1,0x521,0x410,0x0,0x0);
            FUN_004a6e10(iVar2,0x526,0x2,0x1);
            FUN_004a6e10(iVar2,0x527,0x2,0x1);
            FUN_004a6e10(iVar2,0x528,0x2,0x1);
            FUN_004a6e10(iVar2,0x52a,0x2,0x1);
            FUN_0049bf80(param_1,0x52b,0x414,0x0,0x0);
            FUN_0049bf80(param_1,0x52b,0x410,0x0,0x0);
            FUN_0049bf80(param_1,0x52c,0x414,0x0,0x0);
            FUN_0049bf80(param_1,0x52c,0x410,0x0,0x0);
        }
        if ((*(DAT_005967bc + 0x3a) & 0x1) == 0x0) {
            FUN_004a6e10(iVar2,0x526,0x2,0x1);
            FUN_004a6e10(iVar2,0x527,0x2,0x1);
            FUN_004a6e10(iVar2,0x528,0x2,0x1);
            FUN_004a6e10(iVar2,0x52a,0x2,0x1);
            FUN_0049bf80(param_1,0x52b,0x414,0x0,0x0);
            FUN_0049bf80(param_1,0x52b,0x410,0x0,0x0);
            FUN_0049bf80(param_1,0x52c,0x414,0x0,0x0);
            FUN_0049bf80(param_1,0x52c,0x410,0x0,0x0);
        }
        else {
            if (*(DAT_005967bc + 0x23) >> 0x18 == DAT_004c9754) {
                iVar3 = FUN_00434baf(&DAT_005967b8);
                bVar5 = iVar3 != 0x0;
            }
            if (bVar5) {
                if ((*(*(&DAT_004d7d50 +
                    (DAT_005967bc + 0x22) * 0x4 + (DAT_005967bc + 0x20) * 0x3890) +
                    (DAT_005967bc + 0x24) * 0x4) & 0xf) == 0x0) {
                    FUN_004a6e10(iVar2,0x526,0x2,0x1);
                }
                else {
                    FUN_004a6e10(iVar2,0x526,0x2,0x0);
                }
            }
            else {
                FUN_004a6e10(iVar2,0x526,0x2,0x1);
            }
            local_1a0 = !bVar5;
            FUN_004a6e10(iVar2,0x527,0x2,local_1a0);
            puVar4 = FUN_00481784((DAT_005967bc + 0x20),(DAT_005967bc + 0x22),
                                  (DAT_005967bc + 0x24));
            if (puVar4 == 0x0) {
                FUN_004a6e10(iVar2,0x528,0x2,0x1);
                FUN_004a6e10(iVar2,0x52a,0x2,0x1);
                FUN_0049bf80(param_1,0x52b,0x414,0x0,0x0);
                FUN_0049bf80(param_1,0x52b,0x410,0x0,0x0);
                FUN_0049bf80(param_1,0x52c,0x414,0x0,0x0);
                FUN_0049bf80(param_1,0x52c,0x410,0x0,0x0);
                FUN_004968e7(0x9,0x89,0x66,0x28,0xe);
            }
            else {
                if ((*(puVar4 + 0xe) >> 0x10 == DAT_004c9754) && (iVar3 = FUN_00481a44(puVar4), iVar3 == 0x0))
                {
                    local_208 = !bVar5;
                    FUN_004a6e10(iVar2,0x528,0x2,local_208);
                    FUN_004a6e10(iVar2,0x52a,0x2,0x0);
                    FUN_0049bf80(param_1,0x52b,0x40f,0x0,0x0);
                    FUN_0049bf80(param_1,0x52b,0x414,0x1,0x0);
                    FUN_0042e670(puVar4,param_1);
                }
                else {
                    FUN_0042e670(puVar4,param_1);
                }
            }
        }
    }
    return;
}



fn FUN_0042f693() -> i32

{
let mut iVar1: i32;
let mut local_14: i32;

local_14 = 0x0;
while( true ) {
if (0x27 < local_14) {
return -0x1;
}
if ((*(&DAT_005b70c2 + local_14 * 0x4e) != 0x0) &&
(iVar1 = FUN_004a2f10(&DAT_005b709e + local_14 * 0x4e,s_Holy_Terra_004c16fd), iVar1 == 0x0)) break;
local_14 = local_14 + 0x1;
}
return local_14;
}



fn FUN_0042f6fe() -> i32

{
let mut iVar1: i32;
let mut local_14: i32;

local_14 = 0x0;
while( true ) {
if (0x27 < local_14) {
return -0x1;
}
if ((*(&DAT_005b70c2 + local_14 * 0x4e) != 0x0) &&
(iVar1 = FUN_004a2f10(&DAT_005b709e + local_14 * 0x4e,s_Byzantium_II_004c1708), iVar1 == 0x0)) break;
local_14 = local_14 + 0x1;
}
return local_14;
}



fn FUN_0042f769(param_1: i32)

{
    let local_90: u8 [0x80];

    if (DAT_004c9754 == 0x0) {
        if (param_1 <= DAT_00591cb4) {
            FUN_0049c2e0(local_90,s_sav_Log_Save___4d_sav_004c1715);
            FUN_004a5420(local_90);
        }
        FUN_0049c2e0(local_90,s_Log_Save___4d_004c172b);
        FUN_0047bf4b(DAT_00596a38,local_90,DAT_004d559c | 0x2);
    }
    return;
}



fn FUN_0042f801()

{
    *(&DAT_00569b08 + DAT_004c9754 * 0x1e22) = *(&DAT_00569b08 + DAT_004c9754 * 0x1e22) + -0x1;
    *(&DAT_00569abd + DAT_004c9754 * 0x1e22) =
        *(&DAT_00569abd + DAT_004c9754 * 0x1e22) - *(&DAT_00569b00 + DAT_004c9754 * 0x1e22);
    DAT_00573167 = DAT_00573167 + *(&DAT_00569b00 + DAT_004c9754 * 0x1e22);
    if (0xf423f < DAT_00573167) {
        DAT_00573167 = 0xf423f;
    }
    if (*(&DAT_00569b08 + DAT_004c9754 * 0x1e22) == 0x0) {
        *(&DAT_00569b00 + DAT_004c9754 * 0x1e22) = 0x0;
    }
    *(&DAT_00569b0c + DAT_004c9754 * 0x1e22) = 0x0;
    return;
}



fn FUN_0042f8ac() -> u32

{
    let mut pcVar1: String;
    let mut uVar2: u32;
    let local_94: u8 [0x84];

    if (*(&DAT_00569b08 + DAT_004c9754 * 0x1e22) != 0x0) {
        pcVar1 = FUN_00499050(DAT_0059679c,0x7363);
        FUN_0049c2e0(local_94,pcVar1);
        uVar2 = FUN_0049d2e0(0x0,0x3,local_94);
        if (uVar2 != 0x0) {
            FUN_0042f801();
            return 0x1;
        }
        *(&DAT_00569b0c + DAT_004c9754 * 0x1e22) = 0x1;
        *(&DAT_00569b04 + DAT_004c9754 * 0x1e22) =
            *(&DAT_00569b04 + DAT_004c9754 * 0x1e22) + league_int_rate_up_00599d94;
    }
    return 0x0;
}



fn FUN_0042f963(param_1: i32,param_2: i32,param_3: i32) -> i32

{
let mut local_14: i32;

FUN_00489246(param_1,param_2);
local_14 = *(&DAT_00569abd + param_1 * 0x1e22) -
(*(&DAT_00569afc + param_1 * 0x1e22) * *(&DAT_00569ae8 + param_1 * 0x1e22)) / 0x64;
if (param_3 != 0x0) {
local_14 = local_14 - *(&DAT_00569b00 + param_1 * 0x1e22);
}
return local_14 + *(&DAT_00569af4 + param_1 * 0x1e22) +
(*(&DAT_00569af8 + param_1 * 0x1e22) * *(&DAT_00569ae4 + param_1 * 0x1e22)) / 0x64;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0042fa19()

{
    let mut iVar1: i32;
    let puVar2: *mut u32;
    let mut pcVar3: String;
    let mut iVar4: i32;
    byte *pbVar5;
    undefined2 *puVar6;
    let mut bVar7: bool;
    let mut bVar8: bool;
    let mut uVar9: u32;
    let uVar10: u16;
    u32 local_ac [0x20];
    let local_2c: u16;
    let local_2a: u16;
    let local_28: u16;
    let local_24: *mut u32;
    let mut local_20: i32;
    let local_1c: *mut u32;
    let mut local_18: i32;
    let mut local_14: i32;

    for (local_24 = *DAT_005967b0; local_24 != 0x0; local_24 = *local_24) {
        iVar4 = 0x6;
        bVar7 = false;
        iVar1 = 0x0;
        bVar8 = true;
        pbVar5 = &DAT_005827f0;
        puVar2 = local_24 + 0x8;
        loop {
            if (iVar4 == 0x0) break;
            iVar4 = iVar4 + -0x1;
            bVar7 = *pbVar5 < *puVar2;
            bVar8 = *pbVar5 == *puVar2;
            pbVar5 = pbVar5 + 0x1;
            puVar2 = (puVar2 + 0x1);
        } while (bVar8);
        if (!bVar8) {
            iVar1 = (0x1 - bVar7) - (bVar7 != 0x0);
        }
        if ((iVar1 == 0x0) && ((local_24 + 0x27) == '4')) break;
    }
    if (local_24 != 0x0) {
        uVar10 = 0x0;
        uVar9 = 0xffffffff;
        puVar2 = FUN_00499050(DAT_0059679c,0x737f);
        FUN_0045518a(0x1f,0xffffffff,0x737f,0xffffffff,puVar2,uVar9,uVar10);
        _DAT_004d5564 = turns_till_patriarch_dies_00599d48;
        for (local_18 = 0x5; local_18 < 0xa; local_18 = local_18 + 0x1) {
            FUN_0040721c(local_18);
        }
        local_14 = 0x0;
        local_20 = -0x1;
        for (local_18 = 0x0; local_18 < 0x5; local_18 = local_18 + 0x1) {
            if (local_14 < *(&DAT_004d73fc + local_18 * 0x4)) {
                local_14 = *(&DAT_004d73fc + local_18 * 0x4);
                local_20 = local_18 + 0x5;
            }
        }
            (local_24 + 0xd) = local_20;
        local_2c = *(local_24 + 0x8);
        local_2a = *(local_24 + 0x22);
        local_28 = *(local_24 + 0x9);
        for (local_1c = *DAT_005967c8; local_1c != 0x0; local_1c = *local_1c) {
            iVar4 = 0x6;
            bVar7 = false;
            iVar1 = 0x0;
            bVar8 = true;
            puVar6 = &local_2c;
            puVar2 = local_1c + 0x2;
            loop {
                if (iVar4 == 0x0) break;
                iVar4 = iVar4 + -0x1;
                bVar7 = *puVar6 < *puVar2;
                bVar8 = *puVar6 == *puVar2;
                puVar6 = (puVar6 + 0x1);
                puVar2 = (puVar2 + 0x1);
            } while (bVar8);
            if (!bVar8) {
                iVar1 = (0x1 - bVar7) - (bVar7 != 0x0);
            }
            if (iVar1 == 0x0) {
                (local_1c + 0x2a) = (local_24 + 0xd);
            }
        }
        _DAT_00574f95 = *(local_24 + 0x31) >> 0x18;
        FUN_00499050(DAT_0059679c,local_20 + 0x40a);
        pcVar3 = FUN_00499050(DAT_0059679c,0x7141);
        FUN_0049c2e0(&DAT_0057501c,pcVar3);
        FUN_00499050(DAT_0059679c,local_20 + 0x40a);
        pcVar3 = FUN_00499050(DAT_0059679c,0x7382);
        FUN_0049c2e0(local_ac,pcVar3);
        FUN_0045518a(0x1f,0xffffffff,0x7382,(local_24 + 0x8),local_ac,0xffffffff,0x0);
    }
    return;
}



fn FUN_0042fcf9(param_1: *mut i32,param_2: i32)

{
    *param_1 = *param_1 + param_2;
    if (0x64 < *param_1) {
        *param_1 = 0x64;
    }
    return;
}



fn FUN_0042fd26(param_1: *mut i32,param_2: i32)

{
    *param_1 = *param_1 - param_2;
    if (*param_1 < 0x0) {
        *param_1 = 0x0;
    }
    return;
}



fn FUN_0042fd53(param_1: i32) -> i32

{
let mut local_1c: i32;
let mut local_18: i32;
let mut local_14: i32;

local_1c = -0x1;
local_18 = -0x1;
for (local_14 = 0x0; local_14 < 0x5; local_14 = local_14 + 0x1) {
if (local_18 < *(&DAT_00569b14 + local_14 * 0x4 + param_1 * 0x1e22)) {
local_1c = local_14;
local_18 = *(&DAT_00569b14 + local_14 * 0x4 + param_1 * 0x1e22);
}
}
return local_1c;
}



fn FUN_0042fdcd()

{
    let mut local_11c: *mut u32 [0x11];
    let ppuStack215: *mut *mut u8;;
    let mut local_2b: String;;
    u32 **local_24;
    let local_20: *mut i32;;
    let mut local_1c: u32;

    local_20 = FUN_004990e0(local_11c,0x0,s_efs_res_004c1742,s_PauseDlg_004c1739);
    FUN_0049bb50(local_11c,FUN_0042fe63);
    local_24 = local_11c;
    local_1c = 0x0;
    ppuStack215 = &PTR_FUN_004c3d34;
    if (local_2b != 0x0) {
        FUN_00499b30(local_11c,local_2b);
    }
    FUN_0049a1c0(local_11c,0x1);
    return;
}



fn FUN_0042fe63(param_1: i32,param_2: i32,param_3: i32) -> u32

{
    if ((param_2 == 0x407) && (param_3 == 0x64)) {
        FUN_0049c140(param_1,0x1);
    }
    return 0x0;
}



fn FUN_0042feac()

{
    let DVar1: u32;
    let mut local_20: *mut u8;
    let mut local_1c: u32;
    let mut local_18: i32;
    let local_14: i32;

    while( true ) {
        DVar1 = win_func_0049fe83(DAT_005b9bd4,(DWORD *)&local_20,&local_1c,&local_18,&local_14);
        if (DVar1 == 0x0) break;
        FUN_0049a850(local_20,local_1c,local_18,local_14);
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0042fefc(param_1: i32,param_2: *mut i32,param_3: &mut String,param_4: i32,param_5: i32)

{
    let local_200: *mut u32;
    let mut local_188: *mut u32 [0x11];
    let ppuStack323: *mut *mut u8;;
    let mut local_13f: u32;
    let mut local_13b: u32;
    let mut local_137: u32;
    let mut local_133: u32;
    let mut local_12f: u32;
    let mut local_12b: u32;
    let mut local_97: String;;
    let mut local_90: String;
    let uStack140: u8;
    i32 **local_40;
    let mut local_3c: i32;
    let local_38: *mut u32;
    let local_34: *mut *mut char;
    let mut local_30: i32;
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_24: i32;
    let local_20: *mut u32;
    let local_1c: *mut u32;
    i32 **local_18;

    local_3c = param_1 + -0x6978;
    if (((local_3c < 0x0) || (0x63 < local_3c)) || ((&DAT_004d5a58)[DAT_004c9754 * 0x64 + local_3c] != '\0')) {
        if (param_1 != -0x1) {
            param_3 = FUN_00499050(DAT_0059679c,param_1);
        }
        _DAT_004c9724 = param_2;
        local_2c = FUN_004a3840(param_3,&local_90,0x1f4,0x8,&local_30,LPCSTR_005b9218,0x0);
        *(&local_90 + local_2c * 0x6) = 0x0;
        if (param_4 == -0x1) {
            local_28 = (0x280 - (local_30 + 0x28)) / 0x2;
        }
        else {
            local_28 = param_4;
        }
        if (param_5 == -0x1) {
            local_24 = (0x1e0 - (local_2c * 0x14 + 0x28)) / 0x2;
        }
        else {
            local_24 = param_5;
        }
        FUN_0049a030(local_188,0x0,0x1,local_28,local_24,local_30 + 0x28,local_2c * 0x14 + 0x28,0x0,0x8,0x8);
        local_12b = 0x0;
        local_13f = 0x0;
        local_13b = 0x0;
        local_137 = 0x0;
        local_97 = 0x0;
        local_12f = 0x0;
        local_133 = 0x0;
        local_200 = FUN_004a2831(0x5d);
        local_20 = local_200;
        local_1c = local_200;
        if (local_200 != 0x0) {
            local_200 = FUN_0049a030(local_200,local_188,0x3039,0x0,0x0,0x1,0x1,0x0,0xcaccce,0xe0e0e);
            $1: &mut String(local_200 + 0x45) = &PTR_FUN_004c3d94;
            *(local_200 + 0x51) = 0x0;
            *(local_200 + 0x55) = 0x0;
            *(local_200 + 0x4d) = 0x0;
            *(local_200 + 0x49) = 0x2;
        }
        local_38 = local_200;
        FUN_0049bf40(local_188,local_200);
        local_40 = (i32 **)FUN_004a2831(0x10);
        local_18 = local_40;
        if (local_40 != (i32 **)0x0) {
            FUN_004a2874(local_40,local_188,param_2);
        }
        local_34 = FUN_0049a250(local_188,0x4e);
        *local_34 = local_90;
        (local_34 + 0x1) = uStack140;
        FUN_0049bb50(local_188,FUN_00430444);
        FUN_004a2965(local_188);
        if (local_38 != 0x0) {
            ((*(local_38 + 0x45) + 0x8))(local_38,0x2);
        }
        ppuStack323 = &PTR_FUN_004c3d34;
        if (local_97 != 0x0) {
            FUN_00499b30(local_188,local_97);
        }
        FUN_0049a1c0(local_188,0x1);
    }
    return;
}



fn FUN_00430418(param_1: i32,param_2: *mut i32,param_3: &mut String)

{
    FUN_0042fefc(param_1,param_2,param_3,-0x1,-0x1);
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00430444(param_1: *mut *mut u32,param_2: u32,param_3: i32,param_4: i32) -> u32

{
    let puVar1: *mut u32;
    let mut local_38: i32;
    let mut local_30: i32;

    if (param_2 < 0x204) {
        if (param_2 < 0x113) {
            return 0x0;
        }
        if (param_2 < 0x114) {
            if (param_4 != _DAT_004c9724) {
                return 0x0;
            }
            FUN_0049c140(param_1,0x1);
            return 0x0;
        }
        if (param_2 != 0x201) {
            return 0x0;
        }
    }
    else {
        if (0x204 < param_2) {
            if (param_2 < 0x405) {
                if (param_2 != 0x401) {
                    return 0x0;
                }
                FUN_0049bf80(param_1,0x3039,0x414,0x0,0x0);
                return 0x0;
            }
            if (param_2 < 0x406) {
                FUN_004953d7();
                puVar1 = param_1[0x6];
                for (local_38 = 0x0; *(local_38 * 0x6 + puVar1) != 0x0; local_38 = local_38 + 0x1) {
                }
                for (local_30 = 0x0; local_30 < local_38; local_30 = local_30 + 0x1) {
                    FUN_00497567(0x140,local_30 * 0x14 + *(param_1 + 0x21) + 0x14,
                                 *(local_30 * 0x6 + puVar1),*(puVar1 + local_30 * 0x6 + 0x2) >> 0x10,
                                 0xcaccce,-0x1,0xcaccce,LPCSTR_005b9218,0x1);
                }
                FUN_0049536f();
            }
            else {
                if (param_2 != 0x407) {
                    return 0x0;
                }
            }
            if (param_3 != 0x3039) {
                return 0x0;
            }
            FUN_0049c140(param_1,0x1);
            return 0x0;
        }
    }
    FUN_0049c140(param_1,0x1);
    return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00430604()

{
    let puVar1: *mut u32;
    let mut iVar2: i32;
    let mut local_1c: i32;
    let local_14: *mut u32;

    for (local_1c = 0x0; local_1c < 0x5; local_1c = local_1c + 0x1) {
        *(&DAT_004c972c + local_1c * 0x4) = 0xffffffff;
    }
    for (local_1c = 0x0; (local_1c < 0x28 && (*(&DAT_005b70c2 + local_1c * 0x4e) != 0x0));
        local_1c = local_1c + 0x1) {
        iVar2 = FUN_004a2f10(&DAT_005b709e + local_1c * 0x4e,&DAT_004c174a);
        if (iVar2 == 0x0) {
            _DAT_004c972c = local_1c;
        }
        else {
            iVar2 = FUN_004a2f10(&DAT_005b709e + local_1c * 0x4e,s_Aragon_004c174f);
            if (iVar2 == 0x0) {
                _DAT_004c9730 = local_1c;
            }
            else {
                iVar2 = FUN_004a2f10(&DAT_005b709e + local_1c * 0x4e,s_Severus_004c1756);
                if (iVar2 == 0x0) {
                    _DAT_004c9734 = local_1c;
                }
                else {
                    iVar2 = FUN_004a2f10(&DAT_005b709e + local_1c * 0x4e,s_Delphi_004c175e);
                    if (iVar2 == 0x0) {
                        _DAT_004c9738 = local_1c;
                    }
                    else {
                        iVar2 = FUN_004a2f10(&DAT_005b709e + local_1c * 0x4e,s_Istakhr_004c1765);
                        if (iVar2 == 0x0) {
                            _DAT_004c973c = local_1c;
                        }
                    }
                }
            }
        }
    }
    for (local_1c = 0x0; local_1c < 0x5; local_1c = local_1c + 0x1) {
        iVar2 = *(&DAT_004c972c + local_1c * 0x4);
        if (iVar2 != -0x1) {
            for (local_14 = *DAT_005967c8; local_14 != 0x0; local_14 = *local_14) {
                if (((*(local_14 + 0x6) >> 0x10 == iVar2) && ((local_14 + 0x4) == 0x5)) &&
                    ((local_14 + 0xe) == 0x4)) {
                    puVar1 = ((*(local_14 + 0xa) >> 0x10) * 0x4 +
                        *(&DAT_004d7d50 + iVar2 * 0x3890 + (local_14[0x2] >> 0x10) * 0x4) + 0x4);
                    *puVar1 = *puVar1 | *(&DAT_004be9b0 + local_1c * 0x4);
                    local_14[0xb] = local_14[0xb] | *(&DAT_004be9b0 + local_1c * 0x4);
                }
            }
        }
    }
    return;
}



fn FUN_00430815(param_1: i32) -> i32

{
let mut local_14: i32;

local_14 = param_1;
loop {
if (((&DAT_00569a98)[local_14 * 0x1e22] & 0x2) == 0x0) {
return local_14;
}
local_14 = local_14 + -0x1;
if (local_14 < 0x0) {
local_14 = 0xd;
}
} while (local_14 != param_1);
return local_14;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00430868()

{
    let cVar1: u8;
    let BVar2: u8;
    let mut pcVar3: String;
    let mut pcVar4: String;
    let DVar5: u32;
    let mut iVar6: i32;
    BYTE *pBVar7;
    let mut pcVar8: String;
    BYTE *pBVar9;
    let local_210: u8 [0x100];
    let local_110: u8 [0x80];
    let local_90: u8 [0x80];

    FUN_004953d7();
    FUN_004a08c5(s_pcx_lose_bck_pcx_004c176d,0x0,0x0,0x280,0x1e0,0x0,0x0,0x0,0x1);
    FUN_0049536f();
    FUN_00499050(DAT_0059679c,DAT_00595738 + 0x414);
    pcVar3 = FUN_00499050(DAT_0059679c,0x73cc);
    FUN_0049c2e0(local_90,pcVar3);
    pcVar4 = FUN_00499050(DAT_0059679c,0x7384);
    iVar6 = -0x1;
    pcVar3 = local_90;
    loop {
    pcVar8 = pcVar3;
    if (iVar6 == 0x0) break;
    iVar6 = iVar6 + -0x1;
    pcVar8 = pcVar3 + 0x1;
    cVar1 = *pcVar3;
    pcVar3 = pcVar8;
} while (cVar1 != '\0');
    pcVar8 = pcVar8 + -0x1;
    loop {
    cVar1 = *pcVar4;
    *pcVar8 = cVar1;
    if (cVar1 == '\0') break;
    cVar1 = pcVar4[0x1];
    pcVar4 = pcVar4 + 0x2;
    pcVar8[0x1] = cVar1;
    pcVar8 = pcVar8 + 0x2;
} while (cVar1 != '\0');
    FUN_0049d2e0(0x0,0x1,local_90);
    if (video_on_opt_00599dd8 != 0x0) {
        if (DAT_004d6a68 != 0x0) {
            FUN_004a52d0(DAT_004d6a68);
        }
        FUN_004953d7();
        DVar5 = check_reg_val_0042d54b(local_110);
        if (DVar5 != 0x0) {
            pBVar7 = local_110;
            pBVar9 = local_210;
            loop {
                BVar2 = *pBVar7;
                *pBVar9 = BVar2;
                if (BVar2 == '\0') break;
                BVar2 = pBVar7[0x1];
                pBVar7 = pBVar7 + 0x2;
                pBVar9[0x1] = BVar2;
                pBVar9 = pBVar9 + 0x2;
            } while (BVar2 != '\0');
            if ((_DAT_004c9764 == 0x0) || (((&DAT_00569a98)[DAT_00595738 * 0x1e22] & 0x20) == 0x0)) {
                if (_DAT_004c9764 == 0x0) {
                    if (((&DAT_00569a98)[DAT_00595738 * 0x1e22] & 0x20) == 0x0) {
                        pcVar3 = s_smackplw_exe_finale1_smk__g__v2___004c17ea;
                        iVar6 = -0x1;
                        pBVar7 = local_210;
                        loop {
                            pBVar9 = pBVar7;
                            if (iVar6 == 0x0) break;
                            iVar6 = iVar6 + -0x1;
                            pBVar9 = pBVar7 + 0x1;
                            BVar2 = *pBVar7;
                            pBVar7 = pBVar9;
                        } while (BVar2 != '\0');
                        pBVar9 = pBVar9 + -0x1;
                        loop {
                            BVar2 = *pcVar3;
                            *pBVar9 = BVar2;
                            if (BVar2 == '\0') break;
                            BVar2 = (pcVar3)[0x1];
                            pcVar3 = (pcVar3 + 0x2);
                            pBVar9[0x1] = BVar2;
                            pBVar9 = pBVar9 + 0x2;
                        } while (BVar2 != '\0');
                    }
                    else {
                        pcVar3 = s_smackplw_exe_finale2_smk__g__v2___004c17c6;
                        iVar6 = -0x1;
                        pBVar7 = local_210;
                        loop {
                            pBVar9 = pBVar7;
                            if (iVar6 == 0x0) break;
                            iVar6 = iVar6 + -0x1;
                            pBVar9 = pBVar7 + 0x1;
                            BVar2 = *pBVar7;
                            pBVar7 = pBVar9;
                        } while (BVar2 != '\0');
                        pBVar9 = pBVar9 + -0x1;
                        loop {
                            BVar2 = *pcVar3;
                            *pBVar9 = BVar2;
                            if (BVar2 == '\0') break;
                            BVar2 = (pcVar3)[0x1];
                            pcVar3 = (pcVar3 + 0x2);
                            pBVar9[0x1] = BVar2;
                            pBVar9 = pBVar9 + 0x2;
                        } while (BVar2 != '\0');
                    }
                }
                else {
                    pcVar3 = s_smackplw_exe_finale3_smk__g__v2___004c17a2;
                    iVar6 = -0x1;
                    pBVar7 = local_210;
                    loop {
                        pBVar9 = pBVar7;
                        if (iVar6 == 0x0) break;
                        iVar6 = iVar6 + -0x1;
                        pBVar9 = pBVar7 + 0x1;
                        BVar2 = *pBVar7;
                        pBVar7 = pBVar9;
                    } while (BVar2 != '\0');
                    pBVar9 = pBVar9 + -0x1;
                    loop {
                        BVar2 = *pcVar3;
                        *pBVar9 = BVar2;
                        if (BVar2 == '\0') break;
                        BVar2 = (pcVar3)[0x1];
                        pcVar3 = (pcVar3 + 0x2);
                        pBVar9[0x1] = BVar2;
                        pBVar9 = pBVar9 + 0x2;
                    } while (BVar2 != '\0');
                }
            }
            else {
                pcVar3 = s_smackplw_exe_finale4_smk__g__v2___004c177e;
                iVar6 = -0x1;
                pBVar7 = local_210;
                loop {
                    pBVar9 = pBVar7;
                    if (iVar6 == 0x0) break;
                    iVar6 = iVar6 + -0x1;
                    pBVar9 = pBVar7 + 0x1;
                    BVar2 = *pBVar7;
                    pBVar7 = pBVar9;
                } while (BVar2 != '\0');
                pBVar9 = pBVar9 + -0x1;
                loop {
                    BVar2 = *pcVar3;
                    *pBVar9 = BVar2;
                    if (BVar2 == '\0') break;
                    BVar2 = (pcVar3)[0x1];
                    pcVar3 = (pcVar3 + 0x2);
                    pBVar9[0x1] = BVar2;
                    pBVar9 = pBVar9 + 0x2;
                } while (BVar2 != '\0');
            }
            create_proc_004a3b16(local_110,0x0,(LPSTR)local_210);
        }
        FUN_00496404(DAT_005b96c8);
        FUN_0049536f();
    }
    if (DAT_004d6a68 != 0x0) {
        FUN_004a5130(DAT_004d6a68,0x3);
    }
    FUN_00426782();
    return;
}



fn FUN_00430af4()

{
    FUN_004629a4(&DAT_004d55a8);
    FUN_004a70a2(&DAT_004d6058,0x5c,&DAT_004c3e40);
    FUN_004a70ea(&DAT_004be800);
    FUN_004a70a2(&DAT_004d7be0,0x4,&DAT_004c3e60);
    FUN_004a70ea(&DAT_004be810);
    FUN_004a70a2(&DAT_004d7bf0,0x7,&DAT_004c3e60);
    FUN_004a70ea(&DAT_004be820);
    FUN_004a70a2(&DAT_005653d0,0x5,&DAT_004c3e80);
    FUN_004a70ea(&DAT_004be830);
    FUN_00431d31(&DAT_005967b8);
    FUN_00431d31(&DAT_005967c0);
    return;
}



fn FUN_00430bce(param_1: i32) -> i32

{
let mut local_14: i32;

for (local_14 = 0x0; (param_1 + local_14) != '\0'; local_14 = local_14 + 0x1) {
if ((param_1 + local_14) == '.') {
(param_1 + local_14) = 0x0;
}
}
return param_1;
}



char *  FUN_00430c19(param_1: &mut String,param_2: u32)

{
let cVar1: u8;
let mut uVar2: u32;
let mut iVar3: i32;
let mut pcVar4: String;
let mut pcVar5: String;
let mut pcVar6: String;

switch(param_2) {
case 0x1:
pcVar4 = &DAT_004c1822;
iVar3 = -0x1;
pcVar5 = param_1;
loop {
pcVar6 = pcVar5;
if (iVar3 == 0x0) break;
iVar3 = iVar3 + -0x1;
pcVar6 = pcVar5 + 0x1;
cVar1 = *pcVar5;
pcVar5 = pcVar6;
} while (cVar1 != '\0');
pcVar6 = pcVar6 + -0x1;
loop {
cVar1 = *pcVar4;
*pcVar6 = cVar1;
if (cVar1 == '\0') {
return param_1;
}
cVar1 = pcVar4[0x1];
pcVar4 = pcVar4 + 0x2;
pcVar6[0x1] = cVar1;
pcVar6 = pcVar6 + 0x2;
} while (cVar1 != '\0');
break;
case 0x2:
pcVar4 = &DAT_004c1824;
iVar3 = -0x1;
pcVar5 = param_1;
loop {
pcVar6 = pcVar5;
if (iVar3 == 0x0) break;
iVar3 = iVar3 + -0x1;
pcVar6 = pcVar5 + 0x1;
cVar1 = *pcVar5;
pcVar5 = pcVar6;
} while (cVar1 != '\0');
pcVar6 = pcVar6 + -0x1;
loop {
cVar1 = *pcVar4;
*pcVar6 = cVar1;
if (cVar1 == '\0') {
return param_1;
}
cVar1 = pcVar4[0x1];
pcVar4 = pcVar4 + 0x2;
pcVar6[0x1] = cVar1;
pcVar6 = pcVar6 + 0x2;
} while (cVar1 != '\0');
break;
case 0x3:
uVar2 = 0xffffffff;
pcVar5 = param_1;
loop {
if (uVar2 == 0x0) break;
uVar2 = uVar2 - 0x1;
cVar1 = *pcVar5;
pcVar5 = pcVar5 + 0x1;
} while (cVar1 != '\0');
param_1[~uVar2 - 0x2] = '\0';
pcVar4 = &DAT_004c1827;
iVar3 = -0x1;
pcVar5 = param_1;
loop {
pcVar6 = pcVar5;
if (iVar3 == 0x0) break;
iVar3 = iVar3 + -0x1;
pcVar6 = pcVar5 + 0x1;
cVar1 = *pcVar5;
pcVar5 = pcVar6;
} while (cVar1 != '\0');
pcVar6 = pcVar6 + -0x1;
loop {
cVar1 = *pcVar4;
*pcVar6 = cVar1;
if (cVar1 == '\0') {
return param_1;
}
cVar1 = pcVar4[0x1];
pcVar4 = pcVar4 + 0x2;
pcVar6[0x1] = cVar1;
pcVar6 = pcVar6 + 0x2;
} while (cVar1 != '\0');
}
return param_1;
}



fn FUN_00430d15(param_1: &mut String) -> u32

{
    let mut uVar1: u32;

    uVar1 = FUN_004a11c0(*param_1);
    if (uVar1 < 0x49) {
        if (0x40 < uVar1) {
            if (uVar1 < 0x42) {
                return 0x1;
            }
            if (uVar1 == 0x45) {
                return 0x1;
            }
        }
    }
    else {
        if ((uVar1 < 0x4a) || ((0x4e < uVar1 && ((uVar1 < 0x50 || (uVar1 == 0x55)))))) {
            return 0x1;
        }
    }
    return 0x0;
}



BOOL  FUN_00430d8a(LPCSTR app_name,LPCSTR key_name,DWORD param_3,LPCSTR file_name)

{
let mut success: bool;
let string_to_write: u8 [0x14];

FUN_0049c2e0(string_to_write,&DAT_004c182b);
// LPCSTR lpFileName for WritePrivateProfileStringA
// LPCSTR lpString for WritePrivateProfileStringA
// LPCSTR lpKeyName for WritePrivateProfileStringA
// LPCSTR lpAppName for WritePrivateProfileStringA
success = WritePrivateProfileStringA(app_name,key_name,string_to_write,file_name);
return success;
}



fn FUN_00430dd1(param_1: *mut u32) -> *mut u32

{
    *param_1 = 0x0;
    return param_1;
}



fn FUN_00430df7(param_1: &mut String) -> String

{
    if (*param_1 != 0x0) {
        FUN_0049af50(*param_1);
        *param_1 = 0x0;
    }
    return param_1;
}



fn FUN_00430e38(param_1: &mut String)

{
    if (*param_1 != 0x0) {
        FUN_0049af50(*param_1);
        *param_1 = 0x0;
    }
    return;
}



fn FUN_00430e6a(param_1: &mut String)

{
    if (*param_1 != 0x0) {
        FUN_0049af50(*param_1);
        *param_1 = 0x0;
    }
    return;
}



fn FUN_00430e9c(param_1: i32) -> i32

{
let mut iVar1: i32;

iVar1 = FUN_004a70a2(param_1,0x10,&DAT_004c3ea0);
iVar1 = FUN_004a70a2(iVar1 + 0x8c0,0x20,&DAT_004c3e60);
return iVar1 + -0x8c0;
}



fn FUN_00430ef1(param_1: i32) -> u32

{
    let mut iVar1: i32;
    let mut uVar2: u32;

    iVar1 = FUN_00431062(param_1 + 0x8c0);
    uVar2 = FUN_00431034(iVar1 + -0x8c0);
    return uVar2;
}



fn FUN_00430f3a(param_1: *mut u32) -> *mut u32

{
    *param_1 = 0x0;
    return param_1;
}



fn FUN_00430f78(param_1: &mut String) -> String

{
    if (*param_1 != 0x0) {
        FUN_0049af50(*param_1);
        *param_1 = 0x0;
    }
    return param_1;
}



fn FUN_00430fd2(param_1: i32) -> i32

{
let mut iVar1: i32;

iVar1 = FUN_004a70a2(param_1,0x20,&DAT_004c3e60);
return iVar1;
}



fn FUN_00431006(param_1: u32) -> u32

{
    let mut uVar1: u32;

    uVar1 = FUN_00431062(param_1);
    return uVar1;
}



fn FUN_00431034(param_1: u32) -> u32

{
    let mut uVar1: u32;

    uVar1 = FUN_004a7132(param_1,0x10,&DAT_004c3ea0);
    return uVar1;
}



fn FUN_00431062(param_1: u32) -> u32

{
    let mut uVar1: u32;

    uVar1 = FUN_004a7132(param_1,0x20,&DAT_004c3e60);
    return uVar1;
}



fn FUN_00431090(param_1: u32) -> u32

{
    let mut uVar1: u32;

    uVar1 = FUN_004a7132(param_1,0x5,&DAT_004c3e80);
    return uVar1;
}



fn FUN_004310be(param_1: u32) -> u32

{
    let mut uVar1: u32;

    uVar1 = FUN_004a7132(param_1,0x7,&DAT_004c3e60);
    return uVar1;
}



fn FUN_004310ec(param_1: u32) -> u32

{
    let mut uVar1: u32;

    uVar1 = FUN_004a7132(param_1,0x4,&DAT_004c3e60);
    return uVar1;
}



fn FUN_0043111a(param_1: u32) -> u32

{
    let mut uVar1: u32;

    uVar1 = FUN_004a7132(param_1,0x5c,&DAT_004c3e40);
    return uVar1;
}



fn FUN_00431148() -> i32

{
let mut iVar1: i32;
let mut local_18: i32;
let mut local_14: i32;

iVar1 = DAT_00591cb4 / 0x64;
local_14 = iVar1;
while( true ) {
if (local_14 == 0x0) {
return iVar1;
}
if (local_14 == 0x1) break;
local_14 = local_14 >> 0x1;
}
for (local_18 = 0x0; local_18 < 0x32; local_18 = local_18 + 0x1) {
*(&DAT_004d566c + local_18 * 0x2) = *(&DAT_004d566c + local_18 * 0x4);
*(&DAT_004d5734 + local_18 * 0x2) = *(&DAT_004d5734 + local_18 * 0x4);
*(&DAT_004d57fc + local_18 * 0x2) = *(&DAT_004d57fc + local_18 * 0x4);
*(&DAT_004d58c4 + local_18 * 0x2) = *(&DAT_004d58c4 + local_18 * 0x4);
*(&DAT_004d598a + local_18 * 0x2 + 0x2) =
*(&DAT_004d598a + local_18 * 0x4 + 0x2);
iVar1 = local_18;
}
return iVar1;
}



fn FUN_00431230(param_1: i32,param_2: i32,param_3: i32) -> i32

{
let mut bVar1: bool;
let mut iVar2: i32;
let mut local_14: u32;

bVar1 = false;
local_14 = DAT_00591cb4;
if (param_1 < 0x5) {
for (; 0x63 < local_14; local_14 = local_14 >> 0x1) {
if ((local_14 & 0x1) != 0x0) {
if ((param_2 == 0x0) && (param_3 == 0x0)) break;
local_14 = local_14 + 0x1;
bVar1 = true;
}
}
if (((bVar1) || (param_3 == 0x0)) || (DAT_004c9754 <= param_1)) {
iVar2 = FUN_0046031c(param_1,param_2);
(&DAT_004d566c + param_1 * 0xc8 + local_14 * 0x2) = iVar2;
}
}
return *(&DAT_004d566a + local_14 * 0x2 + param_1 * 0xc8) >> 0x10;
}



fn FUN_004312f0(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: i32)

{
    let cVar1: u8;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut pcVar4: String;
    let mut iVar5: i32;
    let mut uVar6: u32;
    let mut iVar7: i32;
    let mut iVar8: i32;
    let piVar9: *mut i32;;
    let piVar10: *mut i32;;
    let uVar11: u8;
    let mut local_c4: i32;
    let local_b8: u8 [0xc];
    let local_ac: u8 [0xc];
    let mut local_a0: i32;
    let mut local_9c: String;
    let local_98: u8 [0xc];
    let mut local_8c: i32;
    let mut local_88: i32;
    let mut local_84: i32;
    i32 local_80 [0x8];
    i32 local_60 [0x5];
    i32 local_4c [0x5];
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

    local_28 = -0x1;
    local_24 = 0x1;
    for (local_38 = 0x0; local_38 < 0x5; local_38 = local_38 + 0x1) {
        local_84 = FUN_00431230(local_38,0x0,0x1);
        if (local_28 < local_84) {
            local_28 = local_84;
        }
    }
    for (local_34 = DAT_00591cb4; 0x63 < local_34; local_34 = local_34 >> 0x1) {
        local_24 = local_24 << 0x1;
    }
    for (local_18 = 0x0; local_18 < local_34; local_18 = local_18 + 0x1) {
        for (local_38 = 0x0; local_38 < 0x5; local_38 = local_38 + 0x1) {
            local_88 = *(&DAT_004d566a + local_18 * 0x2 + local_38 * 0xc8) >> 0x10;
            if (local_28 < local_88) {
                local_28 = local_88;
            }
        }
    }
    local_34 = local_34 + 0x1;
    if (local_34 < 0xb) {
        local_30 = 0xa;
        local_2c = 0x2;
    }
    else {
        if (local_34 < 0x1a) {
            local_30 = 0x19;
            local_2c = 0x5;
        }
        else {
            if (local_34 < 0x33) {
                local_30 = 0x32;
                local_2c = 0xa;
            }
            else {
                local_30 = 0x64;
                local_2c = 0x14;
            }
        }
    }
    piVar9 = &DAT_004beb00;
    piVar10 = local_80;
    for (iVar5 = 0x8; iVar5 != 0x0; iVar5 = iVar5 + -0x1) {
        *piVar10 = *piVar9;
        piVar9 = piVar9 + 0x1;
        piVar10 = piVar10 + 0x1;
    }
    for (local_18 = 0x0; local_18 < 0x8; local_18 = local_18 + 0x1) {
        if (local_28 <= local_80[local_18]) {
            local_20 = local_80[local_18];
            break;
        }
    }
    if (local_18 == 0x8) {
        local_20 = 0x7530;
    }
    local_1c = local_20 / 0xa;
    FUN_004953d7();
    FUN_004a0430(local_4c,0xff,0x14);
    FUN_004a0430(local_60,0xff,0x14);
    if (param_1 == 0x0) {
        local_8c = 0x0;
    }
    else {
        local_8c = 0x10;
    }
    iVar7 = local_8c;
    iVar5 = param_2 + local_8c + 0x28;
    iVar2 = param_4 - (local_8c + 0x28);
    iVar3 = param_5 - (local_8c + 0x10);
    FUN_004968e7(iVar5,param_3,iVar2,iVar3,0xe);
    local_14 = param_3 + iVar3 + -0x1;
    for (local_18 = 0x0; local_18 <= local_20; local_18 = local_18 + local_1c) {
        local_38 = local_14 - (local_18 * iVar3) / local_20;
        if ((local_18 != 0x0) && (local_18 % (local_1c * 0x2) == 0x0)) {
            FUN_0049c2e0(local_98,&DAT_004c1838);
            FUN_00497567(iVar5 + -0x2,local_38,local_98,0x5,0xcaccce,-0x1,0xcaccce,DAT_004d6a6c,0x6);
        }
        FUN_00497896(iVar5,local_38,iVar5 + iVar2 + -0x1,local_38,0x1f);
    }
    if (iVar7 != 0x0) {
        local_9c = FUN_00499050(DAT_0059679c,0x73bc);
        uVar6 = 0xffffffff;
        pcVar4 = local_9c;
        loop {
            if (uVar6 == 0x0) break;
            uVar6 = uVar6 - 0x1;
            cVar1 = *pcVar4;
            pcVar4 = pcVar4 + 0x1;
        } while (cVar1 != '\0');
        local_a0 = ~uVar6 - 0x1;
        for (local_18 = 0x0; local_18 < local_a0; local_18 = local_18 + 0x1) {
            FUN_00497567(iVar5 + -0x38,(local_18 * iVar3) / local_a0 + param_3,(local_9c + local_18),0x1,0xcaccce,-0x1,
                         0xcaccce,DAT_004d6a6c,0x0);
        }
        pcVar4 = FUN_00499050(DAT_0059679c,0x73bd);
        FUN_0049c2e0(local_ac,pcVar4);
        uVar6 = 0xffffffff;
        pcVar4 = local_ac;
        loop {
            if (uVar6 == 0x0) break;
            uVar6 = uVar6 - 0x1;
            cVar1 = *pcVar4;
            pcVar4 = pcVar4 + 0x1;
        } while (cVar1 != '\0');
        local_a0 = ~uVar6 - 0x1;
        FUN_00497567(iVar2 / 0x2 + iVar5,param_3 + iVar3 + 0x10,local_ac,local_a0,0xcaccce,-0x1,0xcaccce,DAT_004d6a6c,
                     0x1);
    }
    local_18 = 0x0;
    loop {
    if (0xa < local_18) {
        for (local_18 = 0x1; local_18 <= local_34; local_18 = local_18 + 0x1) {
            iVar7 = iVar5 + (local_18 * iVar2) / local_30;
            for (local_38 = 0x4; -0x1 < local_38; local_38 = local_38 + -0x1) {
                if (*(&DAT_004beaec + local_38 * 0x4) != 0x0) {
                    local_c4 = *(local_18 * 0x2 + local_38 * 0xc8 + 0x4d5668) >> 0x10;
                    if (local_20 < local_c4) {
                        local_c4 = local_20;
                    }
                    if (local_c4 < 0x0) {
                        local_c4 = 0x0;
                    }
                    iVar8 = local_14 - (local_c4 * iVar3) / local_20;
                    uVar11 = *(local_38 * 0x4 + 0x4bead8);
                    if (local_4c[local_38] != -0x1) {
                        FUN_00497896(local_4c[local_38],local_60[local_38],iVar7 + local_38,iVar8,uVar11);
                    }
                    FUN_004968e7(iVar7 + -0x1 + local_38,iVar8 + -0x1,0x3,0x3,uVar11);
                    local_4c[local_38] = iVar7 + local_38;
                    local_60[local_38] = iVar8;
                }
            }
        }
        if (local_34 == 0x1) {
            for (local_38 = 0x4; -0x1 < local_38; local_38 = local_38 + -0x1) {
                if (local_4c[local_38] != -0x1) {
                    FUN_004968e7(local_4c[local_38] + -0x3,local_60[local_38] + -0x1,0x7,0x7,
                                 (char)*(local_38 * 0x4 + 0x4bead8));
                }
            }
        }
        FUN_0049536f();
        return;
    }
    local_38 = iVar5 + (local_18 * iVar2) / 0xa;
    FUN_00497896(local_38,param_3,local_38,local_14 + -0x1,0x1f);
    if ((local_18 != 0x0) && (local_18 % local_2c == 0x0)) {
        FUN_0049c2e0(local_b8,&DAT_004c183b);
        uVar6 = 0xffffffff;
        pcVar4 = local_b8;
        loop {
            if (uVar6 == 0x0) break;
            uVar6 = uVar6 - 0x1;
            cVar1 = *pcVar4;
            pcVar4 = pcVar4 + 0x1;
        } while (cVar1 != '\0');
        FUN_00497567(local_38,param_3 + iVar3 + 0x2,local_b8,~uVar6 - 0x1,0xcaccce,-0x1,0xcaccce,DAT_004d6a6c,0x1);
    }
    local_18 = local_18 + 0x1;
} while( true );
}



fn FUN_00431980() -> u32

{
    let mut local_120: *mut u32 [0x11];
    let ppuStack219: *mut *mut u8;;
    let mut local_2f: String;;
    let mut local_28: u32;
    u32 **local_24;
    let local_20: *mut i32;;
    let mut local_1c: u32;

    local_20 = FUN_004990e0(local_120,0x0,s_diplo_res_004c1847,s_GraphDlg_004c183e);
    FUN_0049bb50(local_120,FUN_00431a7b);
    local_28 = 0x1;
    local_24 = local_120;
    local_1c = 0x0;
    ppuStack219 = &PTR_FUN_004c3d34;
    if (local_2f != 0x0) {
        FUN_00499b30(local_120,local_2f);
    }
    FUN_0049a1c0(local_120,0x1);
    return local_28;
}



fn FUN_00431a7b(param_1: *mut *mut u32,param_2: u32,param_3: i32) -> u32

{
    let mut iVar1: i32;
    let mut puVar2: *mut u8;
    let mut local_20: u32;
    let mut local_1c: i32;

    switch(param_2) {
    case 0x401:
    for (local_1c = 0x0; local_1c < 0x5; local_1c = local_1c + 0x1) {
        FUN_0049bf80(param_1,local_1c + 0x4b1,0x502,*(&DAT_004beaec + local_1c * 0x4),0x0);
        FUN_0049bf80(param_1,local_1c + 0x3e9,0x502,0x0,&DAT_00569b50 + local_1c * 0x1e22);
        puVar2 = &DAT_004c1851;
        iVar1 = FUN_0046031c(local_1c,0x0);
        FUN_0049bf80(param_1,local_1c + 0x44d,0x503,iVar1,puVar2);
    }
    local_20 = 0x1;
    break;
    default:
        local_20 = 0x0;
    break;
    case 0x405:
        FUN_004312f0(0x1,0x14,0x14,0x1f4,0x190);
    local_20 = 0x1;
    break;
    case 0x406:
    if ((0x4b0 < param_3) && (param_3 < 0x4b6)) {
        puVar2 = FUN_0049bf80(param_1,param_3,0x501,0x0,0x0);
        (&DAT_004beaec + (param_3 + -0x4b1) * 0x4) = puVar2;
        FUN_00431a7b(param_1,0x405,0x0);
    }
    local_20 = 0x1;
    break;
    case 0x407:
    if (param_3 == 0x64) {
        FUN_0049c140(param_1,0x1);
        local_20 = 0x1;
    }
    else {
        local_20 = 0x1;
    }
}
    return local_20;
}



fn FUN_00431c53()

{
    let mut local_20: i32;
    let mut local_1c: u32;
    let mut local_18: i32;
    let local_14: *mut u32;

    FUN_00431d31(&local_1c);
    for (local_14 = *DAT_005967b0; local_14 != 0x0; local_14 = *local_14) {
        if (*(*(&DAT_00582938 +
            (*(local_14 + 0x25) >> 0x18) * 0x4 + (local_14[0x9] >> 0x18) * 0x18) + 0xa5)
            != 0x0) {
            local_18 = FUN_00434de1(local_14);
            for (local_20 = 0x0;
                local_20 <
                *(*(&DAT_00582938 +
                    (*(local_14 + 0x25) >> 0x18) * 0x4 + (local_14[0x9] >> 0x18) * 0x18) + 0xa5)
                ; local_20 = local_20 + 0x1) {
            }
        }
    }
    return;
}



fn FUN_00431d0a(param_1: *mut u32)

{
    *param_1 = 0x0;
    param_1[0x1] = 0x0;
    return;
}



fn FUN_00431d31(param_1: *mut u32) -> *mut u32

{
    FUN_00431d0a(param_1);
    return param_1;
}



fn FUN_00431d5a(param_1: *mut u32,param_2: *mut u32)

{
    *param_1 = *param_2;
    param_1[0x1] = param_2[0x1];
    if ((param_1 == &DAT_005967b8) && (DAT_005967bc != 0x0)) {
        if ((*(DAT_005967bc + 0x3a) & 0x1) == 0x0) {
            DAT_00599e08 = -0x1;
        }
        else {
            DAT_00599e08 = (DAT_005967bc + 0x20);
        }
    }
    return;
}



fn FUN_00431dec(param_1: i32,param_2: i32)

{
    let mut local_1c: u32;
    let mut local_18: i32;
    let mut local_14: i32;

    if (param_2 == *(param_1 + 0x4)) {
        *(param_1 + 0x4) = *(param_2 + 0x8);
    }
    if (*(param_2 + 0xc) != 0x0) {
        *(*(param_2 + 0xc) + 0x8) = *(param_2 + 0x8);
    }
    if (*(param_2 + 0x8) != 0x0) {
        *(*(param_2 + 0x8) + 0xc) = *(param_2 + 0xc);
    }
    *(param_2 + 0x8) = 0x0;
    *(param_2 + 0xc) = 0x0;
    if (*(*(&DAT_00582938 +
        (*(param_2 + 0x25) >> 0x18) * 0x4 + (*(param_2 + 0x24) >> 0x18) * 0x18) + 0xa5) !=
        0x0) {
        FUN_00431d31(&local_1c);
        local_18 = param_2;
        for (local_14 = 0x0;
            local_14 <
            *(*(&DAT_00582938 +
                (*(param_2 + 0x25) >> 0x18) * 0x4 + (*(param_2 + 0x24) >> 0x18) * 0x18) + 0xa5);
            local_14 = local_14 + 0x1) {
            if (*(local_14 * 0x4 + param_2 + 0x10) != 0x0) {
                FUN_00431efd(&local_1c,*(local_14 * 0x4 + param_2 + 0x10));
            }
        }
    }
    return;
}



fn FUN_00431efd(param_1: i32,param_2: i32)

{
    let mut local_14: i32;

    if (param_2 != 0x0) {
        FUN_00431dec(param_1,param_2);
        local_14 = *(param_1 + 0x4);
        if (local_14 == 0x0) {
            *(param_1 + 0x4) = param_2;
        }
        else {
            for (; *(local_14 + 0x8) != 0x0; local_14 = *(local_14 + 0x8)) {
            }
                *(local_14 + 0x8) = param_2;
            *(param_2 + 0xc) = local_14;
        }
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00431f8d(param_1: *mut u32,param_2: i32)

{
    let mut iVar1: i32;
    let mut local_34: i32;
    let mut local_1c: i32;
    let mut local_18: i32;
    let local_14: *mut u32;

    local_1c = 0x0;
    local_18 = 0x0;
    param_1[0x1] = *DAT_005967b0;
    *param_1 = 0x0;
    local_14 = (param_1[0x1] + 0x4);
    if (local_14 == DAT_005967b0) {
        local_14 = 0x0;
    }
    while( true ) {
        if ((((param_1[0x1] != 0x0) && (*(param_1[0x1] + 0xc) == 0x0)) &&
            (*(param_1[0x1] + 0x23) >> 0x18 == param_2)) &&
            ((_DAT_00596a48 != 0x0 ||
                (((*(param_1[0x1] + 0x3a) & 0x24) == 0x0 && ((param_1[0x1] + 0x2f) != '\0')))))) {
            if (DAT_00599e10 == (param_1[0x1] + 0x4c)) {
                if ((*(param_1[0x1] + 0x3a) & 0x1) == 0x0) {
                    local_34 = -0x1;
                }
                else {
                    local_34 = (param_1[0x1] + 0x20);
                }
                if (DAT_00599e08 == local_34)^ // goto LAB_00432172;
                if (local_18 == 0x0) {
                    local_18 = param_1[0x1];
                }
            }
            else {
                if (local_1c == 0x0) {
                    local_1c = param_1[0x1];
                }
            }
        }
        iVar1 = *param_1[0x1];
        if ((param_1[0x1] == local_14) || ((iVar1 == 0x0 && (local_14 == 0x0)))) break;
        if (iVar1 == 0x0) {
            param_1[0x1] = *DAT_005967b0;
        }
        else {
            param_1[0x1] = iVar1;
        }
    }
    if (local_18 == 0x0) {
        if (local_1c == 0x0) {
            DAT_00595f68 = DAT_00595f68 | 0x1;
            param_1[0x1] = 0x0;
            DAT_00599e08 = 0xffffffff;
            return;
        }
        DAT_00599e10 = DAT_00599e10 + '\x01';
        param_1[0x1] = local_1c;
    }
    else {
        param_1[0x1] = local_18;
    }
    LAB_00432172:
    if ((*(param_1[0x1] + 0x3a) & 0x1) == 0x0) {
        DAT_00599e08 = -0x1;
        FUN_00450dbf(&DAT_00595740,(param_1[0x1] + 0x20),(param_1[0x1] + 0x22),
                     (param_1[0x1] + 0x24),param_1[0x1],0x0,*(param_1[0x1] + 0x23) >> 0x18,0x0);
    }
    else {
        DAT_00599e08 = (param_1[0x1] + 0x20);
        FUN_00450dbf(&DAT_00595740,(param_1[0x1] + 0x20),(param_1[0x1] + 0x22),
                     (param_1[0x1] + 0x24),param_1[0x1],0x1,-0x1,0x0);
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0043234c(param_1: i32,param_2: i32)

{
    let local_1c: *mut u32;
    let local_18: *mut u32;
    let local_14: *mut u32;

    local_1c = 0x0;
    local_18 = 0x0;
    local_14 = (&DAT_005b8b44 + param_2 * 0x4);
    loop {
    if (local_14 == 0x0) {
        LAB_00432452:
        if (local_14 == 0x0) {
            if (local_1c == 0x0) {
                if (local_18 == 0x0) {
                    return;
                }
                (param_1 + 0x4) = local_18;
            }
            else {
                (param_1 + 0x4) = local_1c;
            }
        }
        DAT_00599e08 = param_2;
        FUN_00450dbf(&DAT_00595740,(*(param_1 + 0x4) + 0x20),
                     (*(param_1 + 0x4) + 0x22),(*(param_1 + 0x4) + 0x24),
                     *(param_1 + 0x4),0x1,-0x1,0x0);
        return;
    }
    if ((local_14 + 0x8) != param_2) {
        local_14 = 0x0;^
        // goto LAB_00432452;
    }
    if (((local_14[0x3] == 0x0) && (*(local_14 + 0x23) >> 0x18 == DAT_004c9754)) &&
        ((*(local_14 + 0x3a) & 0x1) != 0x0)) {
        if ((_DAT_00596a48 == 0x0) &&
            (((*(local_14 + 0x3a) & 0x24) != 0x0 || ((local_14 + 0x2f) == '\0')))) {
            if (local_18 == 0x0) {
                local_18 = local_14;
            }
        }
        else {
            if (DAT_00599e10 == (local_14 + 0x13)) {
                (param_1 + 0x4) = local_14;^
                // goto LAB_00432452;
            }
            if (local_1c == 0x0) {
                local_1c = local_14;
            }
        }
    }
    local_14 = *local_14;
} while( true );
}



fn FUN_00432515(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: i32,param_6: i32)

{
    *(param_1 + 0x4) = *(&DAT_005b8b44 + param_2 * 0x4);
    loop {
    if ((*(param_1 + 0x4) == 0x0) || ((*(param_1 + 0x4) + 0x20) != param_2)) {
        *(param_1 + 0x4) = 0x0;
        return;
    }
    if ((*(&DAT_004be9b0 + DAT_004c9754 * 0x4) & *(*(param_1 + 0x4) + 0x3a)) != 0x0) {
        if (param_5 == 0x0) {
            if (((*(*(param_1 + 0x4) + 0x3a) & 0x1) == 0x0) &&
                (*(*(param_1 + 0x4) + 0x23) >> 0x18 == param_6)) {
                LAB_00432601:
                if (((*(param_1 + 0x4) + 0x22) == param_3) &&
                    ((*(param_1 + 0x4) + 0x24) == param_4)) {
                    while (*(*(param_1 + 0x4) + 0xc) != 0x0) {
                        *(param_1 + 0x4) = *(*(param_1 + 0x4) + 0xc);
                    }
                    return;
                }
            }
        }
        else {
            if ((*(*(param_1 + 0x4) + 0x3a) & 0x1) != 0x0)^ // goto LAB_00432601;
        }
    }
    *(param_1 + 0x4) = *(param_1 + 0x4);
} while( true );
}



fn FUN_00432683(param_1: *mut u32,param_2: i32,param_3: i32,param_4: i32)

{
    let local_14: *mut u32;

    FUN_00431d0a(param_1);
    local_14 = (&DAT_005b8b44 + param_2 * 0x4);
    while( true ) {
        if (local_14 == 0x0) {
            return;
        }
        if ((local_14 + 0x8) != param_2) break;
        if (((local_14 + 0x22) == param_3) && ((local_14 + 0x9) == param_4)) {
            if ((*(local_14 + 0x3a) & 0x1) != 0x0) {
                param_1[0x1] = local_14;
                local_14[0x3] = 0x0;
                local_14[0x2] = 0x0;
                for (local_14 = *local_14;
                    (((local_14 != 0x0 && ((local_14 + 0x8) == param_2)) &&
                    ((local_14 + 0x22) == param_3)) && ((local_14 + 0x9) == param_4));
                    local_14 = *local_14) {
                    if ((*(local_14 + 0x3a) & 0x1) != 0x0) {
                        local_14[0x3] = local_14[0x1];
                        (local_14[0x3] + 0x8) = local_14;
                        local_14[0x2] = 0x0;
                    }
                }
            }
            return;
        }
        local_14 = *local_14;
    }
    return;
}



fn FUN_0043284d(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: i32) -> i32

{
let mut iVar1: i32;
let local_14: *mut u32;

*(param_1 + 0x4) = 0x0;
local_14 = *DAT_005967b0;
while( true ) {
if (local_14 == 0x0) {
return param_2;
}
if ((((local_14 + 0x22) == param_3) && ((local_14 + 0x9) == param_4)) &&
(*(local_14 + 0x23) >> 0x18 == param_5)) break;
local_14 = *local_14;
}
if ((*(local_14 + 0x3a) & 0x1) != 0x0) {
return param_2;
}
iVar1 = (local_14 + 0x8);
(param_1 + 0x4) = local_14;
local_14[0x3] = 0x0;
local_14[0x2] = 0x0;
local_14 = *local_14;
while( true ) {
if (local_14 == 0x0) {
return iVar1;
}
if ((local_14 + 0x8) != iVar1) {
return iVar1;
}
if ((local_14 + 0x22) != param_3) {
return iVar1;
}
if ((local_14 + 0x9) != param_4) break;
local_14[0x3] = local_14[0x1];
(local_14[0x3] + 0x8) = local_14;
local_14[0x2] = 0x0;
local_14 = *local_14;
}
return iVar1;
}



fn FUN_00432a04(param_1: i32,undefined param_2,undefined param_3)

{
    let mut local_14: i32;

    for (local_14 = *(param_1 + 0x4); local_14 != 0x0; local_14 = *(local_14 + 0x8)) {
        (local_14 + 0x41) = param_2;
        (local_14 + 0x42) = param_3;
    }
    return;
}



fn FUN_00432a46(param_1: i32)

{
    let mut local_14: i32;

    for (local_14 = *(param_1 + 0x4); local_14 != 0x0; local_14 = *(local_14 + 0x8)) {
        (local_14 + 0x41) = 0xff;
        (local_14 + 0x42) = 0xff;
    }
    return;
}



fn FUN_00432a84(param_1: i32,param_2: i32)

{
    let mut local_14: i32;

    for (local_14 = *(param_1 + 0x4); local_14 != 0x0; local_14 = *(local_14 + 0x8)) {
        if (param_2 == 0x0) {
            *(local_14 + 0x3a) = *(local_14 + 0x3a) | 0x4;
        }
        else {
            *(local_14 + 0x3a) = *(local_14 + 0x3a) & 0xfb;
        }
    }
    return;
}



fn FUN_00432aca(param_1: i32,param_2: i32) -> i32

{
let mut local_14: i32;

local_14 = *(param_1 + 0x4);
while( true ) {
if (local_14 == 0x0) {
return 0x0;
}
if (*(local_14 + 0x24) >> 0x18 == param_2) break;
local_14 = *(local_14 + 0x8);
}
return local_14;
}



fn FUN_00432b1a(param_1: i32) -> i32

{
let mut local_18: i32;
let mut local_14: i32;

local_14 = 0xff;
for (local_18 = *(param_1 + 0x4); local_18 != 0x0; local_18 = *(local_18 + 0x8)) {
if (((*(local_18 + 0x3a) & 0x40) == 0x0) &&
(*(*(&DAT_00582938 +
(*(local_18 + 0x25) >> 0x18) * 0x4 + (*(local_18 + 0x24) >> 0x18) * 0x18) + 0x45)
< local_14)) {
local_14 = *(*(&DAT_00582938 +
(*(local_18 + 0x25) >> 0x18) * 0x4 + (*(local_18 + 0x24) >> 0x18) * 0x18
) + 0x45);
}
}
return local_14;
}



fn FUN_00432bd3(param_1: i32) -> i32

{
let mut local_18: i32;
let mut local_14: i32;

local_14 = 0xff;
for (local_18 = *(param_1 + 0x4); local_18 != 0x0; local_18 = *(local_18 + 0x8)) {
if (((((*(local_18 + 0x3a) & 0x80000000) == 0x0) && ((*(local_18 + 0x3b) & 0x20) == 0x0)) &&
((*(local_18 + 0x3a) & 0x4) == 0x0)) &&
(((*(local_18 + 0x3a) & 0x40) == 0x0 && (*(local_18 + 0x2c) >> 0x18 < local_14)))) {
local_14 = *(local_18 + 0x2c) >> 0x18;
}
}
return local_14;
}



fn FUN_00432c94(param_1: i32) -> i32

{
let mut local_1c: i32;
let mut local_18: i32;
let mut local_14: i32;

if (*(param_1 + 0x4) == 0x0) {
local_1c = 0x0;
}
else {
local_14 = 0x0;
for (local_18 = *(param_1 + 0x4); local_18 != 0x0; local_18 = *(local_18 + 0x8)) {
local_14 = local_14 + 0x1;
}
local_1c = local_14;
}
return local_1c;
}



fn FUN_00432cec(param_1: i32) -> u32

{
    let mut local_14: i32;

    local_14 = *(param_1 + 0x4);
    while( true ) {
        if (local_14 == 0x0) {
            return 0x0;
        }
        if (((local_14 + 0x2a) == 0x7) || ((local_14 + 0x2a) == 0x9)) break;
        local_14 = *(local_14 + 0x8);
    }
    return 0x1;
}



fn FUN_00432d43(param_1: i32,param_2: i32) -> u32

{
    let mut local_14: i32;

    local_14 = *(param_1 + 0x4);
    loop {
    if (local_14 == 0x0) {
        return 0x0;
    }
    if (*(*(&DAT_00582938 +
        (*(local_14 + 0x25) >> 0x18) * 0x4 + (*(local_14 + 0x24) >> 0x18) * 0x18) + 0xa5)
        != 0x0) {
        if (param_2 == 0x0) {
            if ((local_14 + 0x2a) == 0x4) {
                return 0x1;
            }
        }
        else {
            if (((local_14 + 0x2a) == 0x7) || ((local_14 + 0x2a) == 0x9)) {
                return 0x1;
            }
        }
    }
    local_14 = *(local_14 + 0x8);
} while( true );
}


// WARNING: Could not reconcile some variable overlaps

fn FUN_00487685(char **param_1,param_2: i32,param_3: i32,param_4: i32) -> i32

{
let sVar1: i16;
let puVar2: *mut u32;
let mut uVar3: u32;
let mut iVar4: i32;
let puVar5: *mut u32;
let puVar6: *mut u32;
let in_stack_fffffe20: i16;
let mut in_stack_fffffe24: u32;
let local_198: *mut u32;
let local_194: *mut u32;
let mut local_190: u32;
let mut local_18c: u32;
let mut local_188: u32;
i32 **local_184;
i32 **local_180;
u32 local_17c [0xc];
let mut local_14c: u32;
let local_148: *mut u32;
let mut local_144: i32;
let mut local_140: i32;
let mut local_13c: u32;
let local_138: *mut u32;
let mut local_134: u32;
let mut local_130: u32;
let local_12c: *mut u32;
let mut local_128: i32;
let mut local_124: i32;
let mut local_120: i32;
let local_11c: *mut u32;
let mut local_118: i32;
let mut local_114: u32;
let local_110: *mut u32;
let mut local_10c: i32;
let mut local_108: u32;
let local_104: *mut u32;
let mut local_100: u32;
let local_fc: *mut u32;
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
let mut local_b8: u32;
let mut local_b4: u32;
let mut local_b0: u32;
let mut local_ac: u32;
let mut local_a8: u32;
let mut local_a4: u32;
let mut local_a0: u32;
let mut local_94: u32;
let local_90: *mut u32;
let mut local_8c: u32;
let mut local_88: u32;
let mut local_84: u32;
let local_80: u16;
let local_7e: u8;
let local_7d: u8;
let local_7c: u8;
let local_7a: u16;
let local_78: u8 [0x3];
let local_75: u8 [0x2];
let local_73: u8 [0x3];
let local_70: u8 [0x2];
let mut local_6e: u32;
let mut local_6a: u32;
let local_66: u8 [0x2];
let local_64: u8 [0x2];
let mut local_62: u32;
let local_5d: u8;
let local_5c: u8;
let local_5b: u8 [0x3];
let mut local_58: u32;
let local_54: i16;
let uStack82: u16;
let local_50: u16;
let mut local_4c: i32;
let mut local_44: i32;
let local_40: u32;
let local_3c: u32;
let mut local_38: i32;
let mut local_34: i32;
let mut local_30: i32;
i32 **local_2c;
i32 **local_28;
i32 **local_24;
let mut local_20: u32;
u32 local_1c [0x2];
let local_14: u8;

local_44 = 0x0;
local_34 = 0x0;
local_30 = 0x1;
local_4c = 0x0;
local_3c = FUN_0047d5ab(param_1);
local_28 = (i32 **)0x0;
local_24 = (i32 **)0x0;
uVar3 = FUN_004a7970(local_1c,0x2,0x1,param_1);
local_20 = (uVar3 == 0x0);
local_44 = local_44 + local_20;
while (local_1c[0] != -0x2) {
local_4c = local_4c + 0x1;
local_54 = local_1c[0];
uVar3 = FUN_004a7970(&uStack82,0x2,0x1,param_1);
local_88 = (uVar3 == 0x0);
local_44 = local_44 + local_88;
uVar3 = FUN_004a7970(&local_50,0x2,0x1,param_1);
local_8c = (uVar3 == 0x0);
local_44 = local_44 + local_8c;
local_90 = &local_84;
local_94 = CONCAT22(uStack82,local_54);
local_84 = CONCAT22(uStack82,local_54);
local_80 = local_50;
uVar3 = FUN_004a7970(&local_7e,0x1,0x1,param_1);
local_a0 = (uVar3 == 0x0);
local_44 = local_44 + local_a0;
uVar3 = FUN_004a7970(&local_7d,0x1,0x1,param_1);
local_a4 = (uVar3 == 0x0);
local_44 = local_44 + local_a4;
uVar3 = FUN_004a7970(&local_7c,0x1,0x1,param_1);
local_a8 = (uVar3 == 0x0);
local_44 = local_44 + local_a8;
uVar3 = FUN_004a7970(&local_14,0x1,0x1,param_1);
local_ac = (uVar3 == 0x0);
local_44 = local_44 + local_ac;
FUN_00486b6b(&local_84,local_14);
uVar3 = FUN_004a7970(local_78,0x2,0x1,param_1);
local_b0 = (uVar3 == 0x0);
local_44 = local_44 + local_b0;
uVar3 = FUN_004a7970((local_78 + 0x2),0x1,0x1,param_1);
local_b4 = (uVar3 == 0x0);
local_44 = local_44 + local_b4;
uVar3 = FUN_004a7970(local_75,0x1,0x1,param_1);
local_b8 = (uVar3 == 0x0);
local_44 = local_44 + local_b8;
uVar3 = FUN_004a7970((local_75 + 0x1),0x1,0x1,param_1);
local_bc = (uVar3 == 0x0);
local_44 = local_44 + local_bc;
uVar3 = FUN_004a7970(local_73,0x2,0x1,param_1);
local_c0 = (uVar3 == 0x0);
local_44 = local_44 + local_c0;
uVar3 = FUN_004a7970((local_73 + 0x2),0x1,0x1,param_1);
local_c4 = (uVar3 == 0x0);
local_44 = local_44 + local_c4;
uVar3 = FUN_004a7970(local_70,0x1,0x1,param_1);
local_c8 = (uVar3 == 0x0);
local_44 = local_44 + local_c8;
uVar3 = FUN_004a7970((local_70 + 0x1),0x1,0x1,param_1);
local_cc = (uVar3 == 0x0);
local_44 = local_44 + local_cc;
uVar3 = FUN_004a7970(&local_6e,0x4,0x1,param_1);
local_d0 = (uVar3 == 0x0);
local_44 = local_44 + local_d0;
uVar3 = FUN_004a7970(&local_6a,0x4,0x1,param_1);
local_d4 = (uVar3 == 0x0);
local_44 = local_44 + local_d4;
uVar3 = FUN_004a7970(local_66,0x1,0x1,param_1);
local_d8 = (uVar3 == 0x0);
local_44 = local_44 + local_d8;
uVar3 = FUN_004a7970((local_66 + 0x1),0x1,0x1,param_1);
local_dc = (uVar3 == 0x0);
local_44 = local_44 + local_dc;
uVar3 = FUN_004a7970(local_64,0x1,0x1,param_1);
local_e0 = (uVar3 == 0x0);
local_44 = local_44 + local_e0;
uVar3 = FUN_004a7970((local_64 + 0x1),0x1,0x1,param_1);
local_e4 = (uVar3 == 0x0);
local_44 = local_44 + local_e4;
uVar3 = FUN_004a7970(&local_62,0x1,0x1,param_1);
local_e8 = (uVar3 == 0x0);
local_44 = local_44 + local_e8;
uVar3 = FUN_004a7970(&local_5c,0x1,0x1,param_1);
local_ec = (uVar3 == 0x0);
local_44 = local_44 + local_ec;
uVar3 = FUN_004a7970(local_5b,0x1,0x1,param_1);
if (uVar3 != 0x0) {
local_f0 = 0x0;
}
else {
local_f0 = 0x1;
}
local_f0 = (uVar3 == 0x0);
local_44 = local_44 + local_f0;
if ((0xeaa00 < param_4) || (0xeaa02 < param_3)) {
uVar3 = FUN_004a7970((local_5b + 0x1),0x2,0x1,param_1);
local_f4 = (uVar3 == 0x0);
local_44 = local_44 + local_f4;
uVar3 = FUN_004a7970(&local_58,0x1,0x1,param_1);
if (uVar3 != 0x0) {
local_f8 = 0x0;
}
else {
local_f8 = 0x1;
}
local_f8 = (uVar3 == 0x0);
local_44 = local_44 + local_f8;
}
if ((local_5c == '\x02') && (*(*(&DAT_00582938 + local_7d * 0x18 + local_7c * 0x4) + 0xa9) == 0x0)) {
local_5c = '\0';
}
if ((local_7e != local_70[1]) && ((local_70[1] < '\n' || ('\f' < local_70[1])))) {
local_70[1] = local_7e;
}
if ((local_5b._1_2_ == -0x1) && (local_7e != '\x06')) {
local_5b._1_2_ = -0x3;
}
local_fc = &local_84;
local_100 = local_6a & 0x1;
if ((local_6a & 0x1) == 0x0) {
local_104 = &local_84;
local_108 = local_84;
local_10c = *(&DAT_005b7076 + local_84 * 0x4e) >> 0x10;
local_110 = &local_84;
local_114 = local_84;
local_118 = *(&DAT_005b7078 + local_84 * 0x4e) >> 0x10;
local_11c = &local_84;
if ((local_70[1] == -0x1) || (local_7e == local_70[1])) {
local_120 = 0x0;
}
else {
local_120 = 0x1;
}
local_124 = local_120;
if (local_120 == 0x0) {
local_128 = FUN_004729ba(local_7e,&local_10c,&local_118,0x0);
}
else {
local_128 = FUN_004729ba(local_70[1],&local_10c,&local_118,0x0);
}
local_12c = &local_84;
local_130 = local_84;
local_134 = local_84;
local_138 = &local_84;
local_13c = local_84;
local_140 = local_10c;
local_144 = local_118;
sVar1 = local_84;
local_84 = local_84 & 0xffff | local_10c << 0x10;
local_80 = local_118;
local_148 = &local_84;
local_14c = local_84;
*(&DAT_005b707e + sVar1 * 0x4e + local_128 * 0x4) = 0x0;
}
local_7a = *(*(&DAT_00582938 + local_7d * 0x18 + local_7c * 0x4) + 0x41);
local_5d = 0x0;
if ((local_70[1] < '\n') || ('\f' < local_70[1])) {
local_70[1] = local_7e;
local_6a = local_6a & 0xffffff7f;
}
puVar5 = &local_84;
puVar6 = local_17c;
for (iVar4 = 0xb; iVar4 != 0x0; iVar4 = iVar4 + -0x1) {
*puVar6 = *puVar5;
puVar5 = puVar5 + 0x1;
puVar6 = puVar6 + 0x1;
}
puVar6 = puVar5;
puVar5 = local_17c;
puVar6 = &stack0xfffffe20;
for (iVar4 = 0xb; iVar4 != 0x0; iVar4 = iVar4 + -0x1) {
*puVar6 = *puVar5;
puVar5 = puVar5 + 0x1;
puVar6 = puVar6 + 0x1;
}
puVar6 = puVar5;
local_2c = FUN_00485a91(in_stack_fffffe20,in_stack_fffffe24);
if (local_30 == 0x0) {
local_24[0x2] = local_2c;
local_2c[0x3] = local_24;
}
if (*(*(&DAT_00582938 +
(*(local_2c + 0x25) >> 0x18) * 0x4 + (local_2c[0x9] >> 0x18) * 0x18) + 0xa5)
== 0x0) {
local_184 = local_2c + 0x8;
local_188 = *(local_2c + 0x3a) & 0x40;
local_180 = local_184;
if (local_188 == 0x0) {
local_28 = (i32 **)0x0;
}
else {
local_28[local_34 + 0x4] = local_2c;
if (local_34 == 0x0) {
local_2c[0x3] = local_28;
local_28[0x2] = local_2c;
}
else {
local_2c[0x3] = local_28[local_34 + 0x3];
local_28[local_34 + 0x3][0x2] = local_2c;
}
local_34 = local_34 + 0x1;
}
}
else {
local_34 = 0x0;
local_28 = local_2c;
}
local_24 = local_2c;
uVar3 = FUN_004a7970(local_1c,0x2,0x1,param_1);
local_18c = (uVar3 == 0x0);
local_44 = local_44 + local_18c;
if (local_1c[0] == -0x3) {
local_30 = 0x1;
uVar3 = FUN_004a7970(local_1c,0x2,0x1,param_1);
local_190 = (uVar3 == 0x0);
local_44 = local_44 + local_190;
}
else {
local_30 = 0x0;
}
local_40 = FUN_004aa690(param_1);
local_38 = (local_40 * 0x64) / local_3c;
FUN_004a36b0(local_38);
}
for (local_4c = 0x0; local_4c < 0xe; local_4c = local_4c + 0x1) {
FUN_00486ba4(local_4c);
}
if (param_2 == 0x0) {
FUN_004840cd(&local_198,&local_194,-0x1);
while (puVar2 = local_198, local_194 != 0x0) {
(local_194 + 0x3e) =

(*(&DAT_00582938 +
(*(local_194 + 0x25) >> 0x18) * 0x4 + (local_194[0x9] >> 0x18) * 0x18) + 0xed);
(local_194 + 0x3f) = 0x0;
if (local_198 != 0x0) {
local_198 = *local_198;
}
local_194 = puVar2;
}
FUN_0048418d(&local_198);
}
FUN_00483da3();
return local_44;
}



fn FUN_004881e3(param_1: i32,param_2: i32,param_3: i32)

{
    let puVar1: *mut u32;
    let puVar2: *mut u32;

    puVar1 = (*(&DAT_004d7d50 + param_1 * 0x3890 + param_2 * 0x4) + param_3 * 0x4 + 0x4);
    *puVar1 = *puVar1 | *(&DAT_004be9b0 + DAT_004c9754 * 0x4);
    puVar2 = FUN_00481784(param_1,param_2,param_3);
    if (puVar2 != 0x0) {
        puVar2[0xb] = puVar2[0xb] | *(&DAT_004be9b0 + DAT_004c9754 * 0x4);
    }
    return;
}



fn FUN_00488257()

{
    let mut pcVar1: String;
    let mut iVar2: i32;
    let mut in_stack_ffffffcc: i32;
    let local_1c: *mut u32;
    let mut local_18: i32;
    let local_14: *mut u32;

    iVar2 = 0x50;
    pcVar1 = FUN_00499050(DAT_0059679c,0x73ee);
    FUN_004a2ff0(0x0,pcVar1,iVar2,in_stack_ffffffcc);
    local_18 = 0x0;
    for (local_1c = *DAT_005967b0; local_1c != 0x0; local_1c = *local_1c) {
        if ((local_1c + 0x8) != local_18) {
            local_18 = (local_1c + 0x8);
            FUN_004a36b0(local_18);
        }
        FUN_00459e8f(local_1c);
        FUN_0044add9(local_1c);
    }
    local_18 = 0x0;
    for (local_14 = *DAT_005967c8; local_14 != 0x0; local_14 = *local_14) {
        if (*(local_14 + 0x6) >> 0x10 != local_18) {
            local_18 = *(local_14 + 0x6) >> 0x10;
            FUN_004a36b0(local_18 + 0x28);
        }
        if (*(local_14 + 0xe) >> 0x10 < 0x5) {
            DAT_004c9754 = *(local_14 + 0xe) >> 0x10;
            FUN_00482843(*(local_14 + 0x6) >> 0x10,local_14[0x2] >> 0x10,
                         *(local_14 + 0xa) >> 0x10,0x5,FUN_004881e3);
        }
    }
    FUN_004a36b0(0x50);
    FUN_004a3800();
    return;
}



fn FUN_004883c0(param_1: i32,param_2: i32,param_3: i32)

{
    let local_14: *mut u32;

    local_14 = (&DAT_005b8b44 + param_1 * 0x4);
    while( true ) {
        if (local_14 == 0x0) {
            return;
        }
        if ((local_14 + 0x8) != param_1) break;
        if (*(local_14 + 0x23) >> 0x18 == param_2) {
            if (param_3 < 0x0) {
                FUN_00486b30((local_14 + 0x8),-param_3);
            }
            else {
                FUN_00486af8((local_14 + 0x8),param_3);
            }
        }
        local_14 = *local_14;
    }
    return;
}



fn FUN_00488456(param_1: i32,param_2: i32)

{
    let local_14: *mut u32;

    local_14 = (&DAT_005b8b44 + param_1 * 0x4);
    while( true ) {
        if (local_14 == 0x0) {
            return;
        }
        if ((local_14 + 0x8) != param_1) break;
        *(local_14 + 0x3a) = *(local_14 + 0x3a) | *(&DAT_004be9b0 + param_2 * 0x4);
        local_14 = *local_14;
    }
    return;
}



fn FUN_004884c0(param_1: i32,param_2: i32) -> i32

{
let puVar1: *mut u32;
let local_2c: *mut u32;
let mut local_20: i32;

local_20 = (*(&DAT_00569b66 + param_1 * 0x1e22) >> 0x18) + (*(&DAT_00569b6f + param_1 * 0x1e22) >> 0x18)
+ (*(&DAT_00569ba2 + param_1 * 0x1e22) >> 0x18) +
*(&DAT_00569ae8 + DAT_004c9754 * 0x1e22) + -0x19;
if ((*(param_1 * 0x1e22 + 0x569b77) & 0x1) != 0x0) {
local_2c = (&DAT_005b89f8 + param_2 * 0x4);
while( true ) {
if ((local_2c == 0x0) || (*(local_2c + 0x6) >> 0x10 != param_2))^ // goto LAB_004885e6;
if (((local_2c + 0xe) == 0x0) &&
(((*(local_2c + 0xe) >> 0x10 == param_1 && ((*(local_2c + 0x2d) & 0x1) == 0x0)) &&
(puVar1 = FUN_00488613(*(local_2c + 0x6) >> 0x10,local_2c[0x2] >> 0x10,
*(local_2c + 0xa) >> 0x10,0x2d), puVar1 != 0x0)))) break;
local_2c = *local_2c;
}
local_20 = local_20 + 0x14;
}
// LAB_004885e6:
if (local_20 < 0x0) {
local_20 = 0x0;
}
else {
if (0x64 < local_20) {
local_20 = 0x64;
}
}
return local_20;
}



fn FUN_00488613(param_1: i32,param_2: i32,param_3: i32,param_4: i32) -> *mut u32

{
    let local_14: *mut u32;

    local_14 = (&DAT_005b8b44 + param_1 * 0x4);
    while( true ) {
        if ((local_14 == 0x0) || ((local_14 + 0x8) != param_1)) {
            return 0x0;
        }
        if ((((local_14 + 0x22) == param_2) && ((local_14 + 0x9) == param_3)) &&
            (local_14[0x9] >> 0x18 == param_4)) break;
        local_14 = *local_14;
    }
    return local_14;
}



fn FUN_004886d4(param_1: i32,param_2: i32)

{
    let local_14: *mut u32;

    for (local_14 = *DAT_005967b0; local_14 != 0x0; local_14 = *local_14) {
        if ((local_14 + 0x8) == param_1) {
            if (*(local_14 + 0x31) >> 0x18 == param_2) {
                FUN_00486af8((local_14 + 0x8),0x5);
            }
            else {
                FUN_00486b30((local_14 + 0x8),0x5);
            }
        }
    }
    return;
}



fn FUN_00488757(param_1: i32,param_2: i32) -> u32

{
    let local_14: *mut u32;

    local_14 = *DAT_005967b0;
    while( true ) {
        if (local_14 == 0x0) {
            return 0x0;
        }
        if (((((local_14 + 0x27) == '\x1d') && (*(local_14 + 0x2d) >> 0x18 == param_2)) &&
            ((local_14 + 0x8) == param_1)) && (*(local_14 + 0x23) >> 0x18 == DAT_004c9754)) break;
        local_14 = *local_14;
    }
    return 0x1;
}



fn FUN_004887e9()

{
    u32 auStack131260 [0x8000];
    u32 auStack188 [0x28];
    let local_1c: *mut u32;
    let mut local_18: i32;
    let local_14: *mut u32;

    for (local_18 = 0x0; local_18 < 0x28; local_18 = local_18 + 0x1) {
        auStack188[local_18] = 0x0;
    }
    for (local_1c = *DAT_005967b0; local_1c != 0x0; local_1c = *local_1c) {
        auStack188[(local_1c + 0x8)] =
            auStack188[(local_1c + 0x8)] |
                *(&DAT_004be9b0 + (*(local_1c + 0x23) >> 0x18) * 0x4);
    }
    for (local_14 = *DAT_005967c8; local_14 != 0x0; local_14 = *local_14) {
        local_14[0xb] = local_14[0xb] | auStack188[*(local_14 + 0x6) >> 0x10];
    }
    return;
}



fn FUN_004888c0(param_1: i32,param_2: i32,param_3: i32,param_4: i32) -> i32

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
if ((*(local_14 + 0x3a) & 0x80) != 0x0) {
return *(local_14 + 0x32) >> 0x18;
}
return -0x1;
}
if ((*(local_14 + 0x3a) & 0x80) != 0x0) {
return *(local_14 + 0x32) >> 0x18;
}
return -0x1;
}



fn FUN_004889db(ushort *param_1,param_2: i32) -> u32

{
    byte *pbVar1;
    let puVar2: *mut u32;
    let mut iVar3: i32;
    let mut uVar4: u32;
    i32 local_68 [0x7];
    let mut local_4c: i32;
    let mut local_48: u32;
    let mut local_44: u32;
    let mut local_40: u32;
    let mut local_3c: i32;
    let mut local_38: i32;
    let mut local_34: i32;
    let mut local_30: i32;
    ushort *local_2c;
    ushort *local_28;
    let mut local_24: i32;
    ushort *local_20;
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: u32;

    local_2c = param_1;
    local_1c = param_1 & 0xffff0000 | *param_1;
    local_3c = *param_1;
    FUN_004a0430(local_68,0x0,0x18);
    local_28 = param_1;
    local_14 = param_1 & 0xffff0000 | param_1[0x2];
    local_24 = param_1[0x2];
    local_20 = param_1;
    local_18 = param_1 & 0xffff0000 | param_1[0x1];
    local_38 = FUN_00485ea2(local_3c,param_1[0x1],local_24,0x1);
    if (local_38 < 0x14) {
        if (param_2 == 0x0) {
            return 0x1;
        }
        local_30 = FUN_004888c0(local_3c,param_1[0x1],param_1[0x2],0x0);
        if (((*(param_1 + 0xd) & 0x80) != 0x0) || (local_30 == -0x1)) {
            return 0x1;
        }
    }
    local_4c = 0x0;
    loop {
    if (0x5 < local_4c) {
        local_4c = 0x0;
        while( true ) {
            if (0x5 < local_4c) {
                return 0x0;
            }
            if (local_68[local_4c] != 0x0) break;
            local_4c = local_4c + 0x1;
        }
        local_48 = FUN_0043a8a2(param_1[0x1] + (&DAT_004bea60)[local_4c]);
        uVar4 = FUN_0043a8d5(local_48,(&DAT_004bea7c)[local_4c] + param_1[0x2]);
        *param_1 = *param_1;
        param_1[0x1] = (ushort)local_48;
        param_1[0x2] = (ushort)uVar4;
        pbVar1 = (*(&DAT_004d7d50 + local_48 * 0x4 + *param_1 * 0x3890) + uVar4 * 0x4 + 0x4);
        *pbVar1 = *pbVar1 | 0x10;
        return 0x1;
    }
    local_48 = FUN_0043a8a2(param_1[0x1] + (&DAT_004bea60)[local_4c]);
    local_44 = FUN_0043a8d5(local_48,(&DAT_004bea7c)[local_4c] + param_1[0x2]);
    local_40 = *(*(&DAT_004d7d50 + local_48 * 0x4 + local_3c * 0x3890) + local_44 * 0x4);
    if (((local_40 & 0xf) != 0x0) && ((local_40 & 0xf) != 0xa)) {
        puVar2 = FUN_00481784(local_3c,local_48,local_44);
        local_34 = FUN_0044ace5(local_3c,local_48,local_44,0x0);
        if (*(param_1 + 0x3) >> 0x18 == local_34) {
            local_38 = FUN_00485ea2(local_3c,local_48,local_44,0x1);
            if ((local_38 < 0x14) &&
                ((local_30 = FUN_004888c0(local_3c,local_48,local_44,0x0), (*(param_1 + 0xd) & 0x80) != 0x0 ||
                    (local_30 == -0x1)))) {
                *param_1 = *param_1;
                param_1[0x1] = (ushort)local_48;
                param_1[0x2] = (ushort)local_44;
                pbVar1 = (*(&DAT_004d7d50 + local_48 * 0x4 + *param_1 * 0x3890) + local_44 * 0x4 + 0x4);
                *pbVar1 = *pbVar1 | 0x10;
                return 0x1;
            }
        }
        else {
            if ((local_34 == -0x1) && ((puVar2 == 0x0 || (iVar3 = FUN_00481a44(puVar2), iVar3 != 0x0))))
            {
                local_68[local_4c] = 0x1;
            }
        }
    }
    local_4c = local_4c + 0x1;
} while( true );
}



fn FUN_00488ec0(param_1: i32,param_2: i32)

{
    let mut iVar1: i32;
    let local_14: u8;

    iVar1 = param_2 + (*(param_1 + 0x30) >> 0x18);
    if (iVar1 < 0x65) {
        local_14 = iVar1;
        (param_1 + 0x33) = local_14;
    }
    else {
        (param_1 + 0x33) = 0x64;
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

bool  FUN_00488efd(param_1: i32,param_2: i32)

{
let mut iVar1: i32;
let local_14: u8;

iVar1 = (*(param_1 + 0x30) >> 0x18) - param_2;
if (iVar1 < 0x1) {
(param_1 + 0x33) = 0x0;
*(param_1 + 0x3d) = *(param_1 + 0x3d) | 0x80;
_DAT_005b8be4 = 0x1;
}
else {
local_14 = iVar1;
(param_1 + 0x33) = local_14;
}
return iVar1 < 0x1;
}



fn FUN_00488f5c(param_1: *mut i32)

{
    let mut local_14: i32;

    if ((param_1 + 0x26) == (param_1 + 0x35)) {
        for (local_14 = 0x0; local_14 < 0xd; local_14 = local_14 + 0x1) {
            if (*(local_14 * 0x4 +
                *(&DAT_00582938 +
                    (*(param_1 + 0x25) >> 0x18) * 0x4 + (param_1[0x9] >> 0x18) * 0x18) + 0xb9) != 0x0
            ) {
                FUN_00465f06((param_1 + 0x8),(param_1 + 0x22),(param_1 + 0x9),
                             local_14,(*(local_14 * 0x4 +
                        *(&DAT_00582938 +
                            (*(param_1 + 0x25) >> 0x18) * 0x4 +
                            (param_1[0x9] >> 0x18) * 0x18) + 0xb9) * give_back_res_per_opt_00599d6c)
                                 / 0x64,*(param_1 + 0x23) >> 0x18,0x0);
            }
        }
    }
    if (*(*(&DAT_00582938 + (*(param_1 + 0x25) >> 0x18) * 0x4 + (param_1[0x9] >> 0x18) * 0x18) +
        0xed) == -0x1) {
        FUN_00484b4e(param_1);
    }
    else {
        (param_1 + 0x27) =

            (*(&DAT_00582938 + (*(param_1 + 0x25) >> 0x18) * 0x4 + (param_1[0x9] >> 0x18) * 0x18) +
                0xed);
        (param_1 + 0xa) = 0x0;
        (param_1 + 0x3e) = 0xff;
        (param_1 + 0x3f) = 0x0;
        *(param_1 + 0x2a) =
            *
                (*(&DAT_00582938 + (*(param_1 + 0x25) >> 0x18) * 0x4 + (param_1[0x9] >> 0x18) * 0x18) +
                    0x41);
        (param_1 + 0x2f) =

            (*(&DAT_00582938 + (*(param_1 + 0x25) >> 0x18) * 0x4 + (param_1[0x9] >> 0x18) * 0x18) +
                0x45);
    }
    return;
}



fn FUN_00489176(param_1: i32,param_2: i32,param_3: i32,param_4: i32) -> u32

{
    let local_14: *mut u32;

    local_14 = (&DAT_005b8b44 + param_1 * 0x4);
    while( true ) {
        if ((local_14 == 0x0) || ((local_14 + 0x8) != param_1)) {
            return 0x0;
        }
        if (((((local_14 + 0x22) == param_2) && ((local_14 + 0x9) == param_3)) &&
            ((local_14 + 0x27) == '\x1d')) && (*(local_14 + 0x2d) >> 0x18 == param_4)) break;
        local_14 = *local_14;
    }
    return 0x1;
}



fn FUN_00489246(param_1: i32,param_2: i32)

{
    let mut iVar1: i32;
    let local_24: *mut u32;
    let mut local_1c: i32;
    let mut local_18: i32;
    let local_14: *mut u32;

    local_1c = -0x1;
    *(&DAT_00569ab9 + param_1 * 0x1e22) = 0x0;
    *(&DAT_00569afc + param_1 * 0x1e22) = 0x0;
    *(&DAT_00569af4 + param_1 * 0x1e22) = 0x0;
    *(&DAT_00569af8 + param_1 * 0x1e22) = 0x0;
    *(&DAT_00569ab5 + param_1 * 0x1e22) = 0x0;
    *(&DAT_00569a99 + param_1 * 0x1e22) = 0x0;
    *(&DAT_00569ab1 + param_1 * 0x1e22) = 0x0;
    *(&DAT_00569ac1 + param_1 * 0x1e22) = 0x0;
    for (local_18 = 0x1; local_18 < 0x72; local_18 = local_18 + 0x1) {
        if (((&DAT_00569c30)[local_18 * 0x9 + param_1 * 0x1e22] & 0x1) != 0x0) {
            *(&DAT_00569a99 + param_1 * 0x1e22) = *(&DAT_00569a99 + param_1 * 0x1e22) + 0x1;
        }
    }
    for (local_24 = *DAT_005967b0; local_24 != 0x0; local_24 = *local_24) {
        if (*(local_24 + 0x23) >> 0x18 == param_1) {
            *(&DAT_00569ab9 + param_1 * 0x1e22) = *(&DAT_00569ab9 + param_1 * 0x1e22) + 0x1;
            if ((*(local_24 + 0x3a) & 0x80) == 0x0) {
                *(&DAT_00569afc + param_1 * 0x1e22) =
                    *(&DAT_00569afc + param_1 * 0x1e22) +
                        *(*(&DAT_00582938 +
                            (*(local_24 + 0x25) >> 0x18) * 0x4 + (local_24[0x9] >> 0x18) * 0x18) +
                            0xb1);
            }
            if ((local_24 + 0x27) == '\x1c') {
                *(&DAT_00569ab5 + param_1 * 0x1e22) = *(&DAT_00569ab5 + param_1 * 0x1e22) + 0x1;
            }
        }
    }
    FUN_0048976a(param_1,param_2);
    for (local_14 = *DAT_005967c8; local_14 != 0x0; local_14 = *local_14) {
        if (*(local_14 + 0xe) >> 0x10 == param_1) {
            *(&DAT_00569ac1 + param_1 * 0x1e22) = *(&DAT_00569ac1 + param_1 * 0x1e22) + 0x1;
            if ((local_14 + 0xe) == 0x17) {
                *(&DAT_00569ab1 + param_1 * 0x1e22) = *(&DAT_00569ab1 + param_1 * 0x1e22) + 0x1;
            }
            if ((*(local_14 + 0x6) >> 0x10 != local_1c) &&
                (iVar1 = FUN_00482655(*(local_14 + 0x6) >> 0x10), iVar1 == param_1)) {
                local_1c = *(local_14 + 0x6) >> 0x10;
            }
            if (((*(local_14 + 0x2d) & 0x1) == 0x0) &&
                (*(&DAT_00569afc + param_1 * 0x1e22) =
                     *(&DAT_00569afc + param_1 * 0x1e22) +
                         *(&DAT_00583214 + (local_14[0x3] >> 0x10) * 0x50),
                 *(local_14 + 0x6) >> 0x10 == local_1c)) {
                *(&DAT_00569af8 + param_1 * 0x1e22) =
                    *(&DAT_00569af8 + param_1 * 0x1e22) +
                        ((*(local_14 + 0x26) >> 0x10) * *(&DAT_00569ae0 + param_1 * 0x1e22)) / 0xa;
            }
        }
    }
    return;
}



fn FUN_004894bf(param_1: i32) -> u32

{
    let local_14: *mut u32;

    *(&DAT_00569ab5 + param_1 * 0x1e22) = 0x0;
    for (local_14 = *DAT_005967b0; local_14 != 0x0; local_14 = *local_14) {
        if ((*(local_14 + 0x23) >> 0x18 == param_1) && ((local_14 + 0x27) == '\x1c')) {
            *(&DAT_00569ab5 + param_1 * 0x1e22) = *(&DAT_00569ab5 + param_1 * 0x1e22) + 0x1;
        }
    }
    return *(&DAT_00569ab5 + param_1 * 0x1e22);
}



fn FUN_00489539(param_1: i32,param_2: i32,param_3: i32) -> u32

{
    let mut bVar1: bool;
    let local_24: *mut u32;

    local_24 = (&DAT_005b8b44 + (param_1 + 0x20) * 0x4);
    while( true ) {
        if ((local_24 == 0x0) || ((local_24 + 0x8) != (param_1 + 0x20))) {
            return 0x1;
        }
        if ((((*(local_24 + 0x3a) & 0x1) != 0x0) && ((local_24 + 0x22) == param_2)) &&
            ((local_24 + 0x9) == param_3)) break;
        local_24 = *local_24;
    }
    if (*(local_24 + 0x23) >> 0x18 != DAT_004c9754) {
        return 0x1;
    }
    if (((param_1 + 0x35) == -0x1) || ((param_1 + 0x26) == (param_1 + 0x35))) {
        bVar1 = false;
    }
    else {
        bVar1 = true;
    }
    if (bVar1) {
        if (((local_24 + 0x35) == -0x1) ||
            ((local_24 + 0x26) == (local_24 + 0x35))) {
            bVar1 = false;
        }
        else {
            bVar1 = true;
        }
        if (bVar1) {
            if ((local_24 + 0x35) == (param_1 + 0x35)) {
                return 0x1;
            }
            return 0x0;
        }
        return 0x0;
    }
    if (((local_24 + 0x35) == -0x1) || ((local_24 + 0x26) == (local_24 + 0x35)))
    {
        bVar1 = false;
    }
    else {
        bVar1 = true;
    }
    if (bVar1) {
        return 0x0;
    }
    return 0x1;
}



fn FUN_0048976a(param_1: i32,param_2: i32)

{
    let mut bVar1: bool;
    let mut iVar2: i32;
    undefined3 extraout_var;
    let mut pcVar3: String;
    u32 local_ac [0x20];
    let local_2c: *mut u32;
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    local_28 = -0x1;
    local_24 = 0x0;
    local_20 = 0x0;
    local_1c = -0x1;
    *(&DAT_00569af4 + param_1 * 0x1e22) = 0x0;
    for (local_2c = *DAT_005967c8; local_2c != 0x0; local_2c = *local_2c) {
        if ((*(local_2c + 0xe) >> 0x10 == param_1) && ((*(local_2c + 0x2d) & 0x1) == 0x0)) {
            if (*(local_2c + 0x6) >> 0x10 != local_28) {
                local_20 = 0x0;
                iVar2 = FUN_004825ee(*(local_2c + 0x6) >> 0x10);
                if ((((iVar2 == param_1) && (local_28 = *(local_2c + 0x6) >> 0x10, DAT_004d7792 >> 0x18 == param_1))
                    && (bVar1 = FUN_0046292a((&DAT_00568210 + param_1 * 0x1e22)), CONCAT31(extraout_var,bVar1) != 0x0)) &&
                    (iVar2 = FUN_00488757(*(local_2c + 0x6) >> 0x10,0x0), iVar2 != 0x0)) {
                    local_20 = 0x1;
                    local_1c = *(local_2c + 0x6) >> 0x10;
                }
            }
            if ((*(local_2c + 0x6) >> 0x10 == local_28) &&
                (*(&DAT_00569af4 + param_1 * 0x1e22) =
                     *(&DAT_00569af4 + param_1 * 0x1e22) +
                         ((*(local_2c + 0x26) >> 0x10) * *(&DAT_00569adc + param_1 * 0x1e22) * 0xa) / 0x64,
                 local_20 != 0x0)) {
                local_24 = local_24 +
                    ((*(local_2c + 0x26) >> 0x10) * *(&DAT_00569adc + param_1 * 0x1e22) * 0xa) / 0x64;
            }
        }
    }
    local_14 = 0x0;
    if ((*(param_1 * 0x1e22 + 0x569b7a) & 0x1) != 0x0) {
        local_18 = *(&DAT_00569b78 + param_1 * 0x1e22) >> 0x18;
        local_14 = (*(&DAT_00569af4 + param_1 * 0x1e22) * local_18) / 0x64;
        if ((param_2 != 0x0) && ((*(param_1 * 0x1e22 + 0x569b7a) & 0x10) != 0x0)) {
            FUN_00499050(DAT_0059679c,0x106e);
            pcVar3 = FUN_00499050(DAT_0059679c,0x691a);
            FUN_0049c2e0(local_ac,pcVar3);
            FUN_0045518a(0x1 << ((byte)param_1 & 0x1f),0xffffffff,0x106e,0xffffffff,local_ac,0xffffffff,0x0);
        }
    }
    if ((*(param_1 * 0x1e22 + 0x569baa) & 0x1) != 0x0) {
        local_18 = (*(&DAT_00569af4 + param_1 * 0x1e22) * (*(&DAT_00569ba8 + param_1 * 0x1e22) >> 0x18)) /
            0x64;
        if (local_18 < 0x0) {
            local_18 = -local_18;
        }
        local_14 = local_14 - local_18;
        if ((param_2 != 0x0) && ((*(param_1 * 0x1e22 + 0x569baa) & 0x10) != 0x0)) {
            FUN_00499050(DAT_0059679c,0x10a0);
            pcVar3 = FUN_00499050(DAT_0059679c,0x694c);
            FUN_0049c2e0(local_ac,pcVar3);
            FUN_0045518a(0x1 << ((byte)param_1 & 0x1f),0xffffffff,0x10a0,0xffffffff,local_ac,0xffffffff,0x0);
        }
    }
    if ((*(param_1 * 0x1e22 + 0x569bad) & 0x1) != 0x0) {
        local_18 = (*(&DAT_00569af4 + param_1 * 0x1e22) * (*(&DAT_00569bab + param_1 * 0x1e22) >> 0x18)) /
            0x64;
        if (local_18 < 0x0) {
            local_18 = -local_18;
        }
        local_14 = local_14 - local_18;
        if ((param_2 != 0x0) && ((*(param_1 * 0x1e22 + 0x569bad) & 0x10) != 0x0)) {
            FUN_00499050(DAT_0059679c,0x10a1);
            pcVar3 = FUN_00499050(DAT_0059679c,0x694d);
            FUN_0049c2e0(local_ac,pcVar3);
            FUN_0045518a(0x1 << ((byte)param_1 & 0x1f),0xffffffff,0x10a1,0xffffffff,local_ac,0xffffffff,0x0);
        }
    }
    if (local_24 != 0x0) {
        local_18 = (DAT_004d7730 * local_24) / 0x64;
        local_14 = local_14 + local_18;
        if (param_2 != 0x0) {
            pcVar3 = FUN_00499050(DAT_0059679c,0x7454);
            FUN_0049c2e0(local_ac,pcVar3);
            FUN_0045518a(0x1 << ((byte)param_1 & 0x1f),0xffffffff,0x74cc,0xffffffff,local_ac,0xffffffff,0x0);
        }
    }
    *(&DAT_00569af4 + param_1 * 0x1e22) = *(&DAT_00569af4 + param_1 * 0x1e22) + local_14;
    return;
}



fn FUN_00489d55(param_1: i32) -> u32

{
    let mut bVar1: bool;
    undefined3 extraout_var;
    undefined3 extraout_var_00;
    let mut iVar2: i32;
    let mut local_310: u32;
    let mut uStack780: u32;
    let mut local_308: u32;
    let mut uStack772: u32;
    let mut local_300: u32;
    let mut local_2fc: u32;
    let local_2f8: *mut u32;
    let mut local_2f4: u32;
    ushort *local_2f0;
    ushort *local_2ec;
    let mut local_2e8: i32;
    let mut local_2e4: u32;
    let mut local_2e0: u32;
    let mut local_2dc: u32;
    let mut local_2d8: i32;
    let mut local_2d4: u32;
    let mut local_2d0: u32;
    let mut local_2cc: u32;
    let mut local_2c8: u32;
    let mut local_2c4: i32;
    let mut local_2c0: i32;
    let mut local_2bc: i32;
    let mut local_2b8: u32;
    let mut local_2b4: u32;
    let local_2b0: *mut i32;;
    let local_2ac: *mut i32;;
    let mut local_2a8: u32;
    ushort *local_2a4;
    ushort *local_2a0;
    let local_29c: *mut i32;;
    i32 **local_298;
    let local_294: *mut i32;;
    let mut local_290: u32;
    ushort *local_28c;
    ushort *local_288;
    let mut local_284: i32;
    let mut local_280: u32;
    let mut local_27c: u32;
    let mut local_278: u32;
    let mut local_274: i32;
    let mut local_270: u32;
    let mut local_26c: u32;
    let mut local_268: u32;
    let mut local_264: u32;
    let mut local_260: i32;
    let mut local_25c: i32;
    let mut local_258: i32;
    let mut local_254: u32;
    let mut local_250: i32;
    let mut local_24c: i32;
    let mut local_248: String; [0x3f];
    let mut local_14c: *mut u32 [0x7];
    let mut local_12f: i32;
    let mut local_12b: i32;
    let ppuStack263: *mut *mut u8;;
    let mut local_5b: String;;
    let mut local_54: u32;
    let mut iStack80: i32;
    let mut local_4c: u32;
    let mut local_48: u32;
    let mut local_44: u32;
    let mut local_40: i32;
    let local_3c: *mut i32;;
    i32 **local_38;
    i32 **local_34;
    let local_30: *mut i32;;
    let mut local_2c: u32;
    let local_28: *mut i32;;
    let local_24: *mut i32;;
    let local_20: *mut i32;;
    let local_1c: *mut u32;
    let mut local_18: u32;

    local_1c = &DAT_005967b8;
    local_18 = (DAT_005967bc == 0x0);
    local_2c = local_18;
    if (local_18 == 0x0) {
        local_54 = DAT_005967b8;
        iStack80 = DAT_005967bc;
        local_250 = DAT_005967bc + 0x20;
        local_254 = *(DAT_005967bc + 0x3a) & 0x1;
        local_40 = *(DAT_005967bc + 0x23) >> 0x18;
        DAT_00599d3c = 0x1;
        local_24c = local_250;
        local_44 = local_254;
        if ((DAT_00595f68 & 0x1) != 0x0) {
            local_258 = *(DAT_005967bc + 0x23) >> 0x18;
            local_260 = DAT_005967bc + 0x20;
            local_264 = *(DAT_005967bc + 0x3a) & 0x1;
            local_26c = DAT_005967bc + 0x20;
            local_270 = local_26c & 0xffff0000 | (DAT_005967bc + 0x24);
            local_274 = (DAT_005967bc + 0x24);
            local_27c = DAT_005967bc + 0x20;
            local_280 = local_27c & 0xffff0000 | (DAT_005967bc + 0x22);
            local_284 = (DAT_005967bc + 0x22);
            local_28c = (DAT_005967bc + 0x20);
            local_290 = local_28c & 0xffff0000 | *local_28c;
            local_288 = local_28c;
            local_278 = local_27c;
            local_268 = local_26c;
            local_25c = local_260;
            FUN_00450dbf(&DAT_00595740,*local_28c,local_284,local_274,DAT_005967bc,local_264,local_258,0x0);
        }
        if (((param_1 != 0x1c8a) && (sRam00595f5a == DAT_004c9754)) &&
            (bVar1 = FUN_00452ec6(&DAT_00595740), CONCAT31(extraout_var,bVar1) != 0x0)) {
            FUN_00452f5b(&DAT_00595740);
        }
        DAT_005b8c34 = FUN_00481784(DAT_00595f52._2_2_,DAT_00595f56,DAT_00595f56._2_2_);
        DAT_005b8c30 = FUN_00452a43(&DAT_00595740);
        local_294 = FUN_004990e0(local_14c,0x0,s_efs_res_004c2728,s_UnitInfoDlg_004c271c);
        local_298 = (i32 **)FUN_004a2831(0x10);
        local_38 = local_298;
        local_34 = local_298;
        if (local_298 != (i32 **)0x0) {
            local_298 = FUN_004a2874(local_298,local_14c,0x96);
        }
        DAT_005b8c2c = FUN_0049c2c9(0x104);
        FUN_0049c2e0(DAT_005b8c2c,s_flc__s_004c2730);
        local_30 = FUN_0049c4bd(DAT_005b8c2c,&DAT_004c2737);
        if (local_30 == 0x0) {
            FUN_0049c2e0(DAT_005b8c2c,PTR_s_blank_flc_004bea98);
        }
        else {
            FUN_0049ca40(local_30);
        }
        local_29c = FUN_0049cc70(local_248,local_14c,0xbb7,local_12f + 0x1ba,local_12b + 0xb,0x0,0x0,0x0,DAT_005b8c2c);
        FUN_0049bf40(local_14c,local_248);
        local_2ac = FUN_004a2831(0xb9);
        local_3c = local_2ac;
        local_28 = local_2ac;
        if (local_2ac != 0x0) {
            local_2a4 = (DAT_005b8c30 + 0x20);
            local_2a8 = local_2a4 & 0xffff0000 | *local_2a4;
            local_2a0 = local_2a4;
            local_2ac = FUN_00438792(local_2ac,local_14c,0x1f4,0x10,0x82,0x58,0x3f,0x40,0x2,0x2,*local_2a4);
        }
        DAT_005b8c38 = local_2ac;
        local_2b0 = FUN_004a2831(0x95);
        local_24 = local_2b0;
        local_20 = local_2b0;
        if (local_2b0 != 0x0) {
            local_2b0 = FUN_0047157e(local_2b0,local_14c,0x1f5,0xa,0xa,0x64,0x60,0x40,0x2);
        }
        DAT_005b8c3c = local_2b0;
        FUN_0049bf40(local_14c,DAT_005b8c38);
        FUN_0049bf40(local_14c,DAT_005b8c3c);
        if (param_1 != 0x1c8a) {
            FUN_0049bf80(local_14c,0x6c,0x414,0x0,0x0);
            FUN_0049bf80(local_14c,0x6c,0x410,0x0,0x0);
            FUN_0049bf80(local_14c,0x6d,0x414,0x0,0x0);
            FUN_0049bf80(local_14c,0x6d,0x410,0x0,0x0);
            FUN_0049bf80(local_14c,0x6f,0x414,0x0,0x0);
            FUN_0049bf80(local_14c,0x6f,0x410,0x0,0x0);
        }
        local_4c = FUN_0049bb50(local_14c,&DAT_0048a62e);
        FUN_004a2965(local_14c);
        FUN_0049af50(DAT_005b8c2c);
        if (DAT_005b8c38 == 0x0) {
            local_2b4 = 0x0;
        }
        else {
            local_2b4 = ((*(DAT_005b8c38 + 0x45) + 0x8))(DAT_005b8c38,0x2);
        }
        if (DAT_005b8c3c == 0x0) {
            local_2b8 = 0x0;
        }
        else {
            local_2b8 = ((*(DAT_005b8c3c + 0x45) + 0x8))(DAT_005b8c3c,0x2);
        }
        if (sRam00595f5a == DAT_004c9754) {
            FUN_004864f7();
            if (param_1 == 0x1c8a) {
                if (local_4c == 0x0) {
                    FUN_00431d5a(&DAT_005967b8,&local_54);
                    local_2bc = *(DAT_005967bc + 0x23) >> 0x18;
                    local_2c4 = DAT_005967bc + 0x20;
                    local_2c8 = *(DAT_005967bc + 0x3a) & 0x1;
                    local_2d0 = DAT_005967bc + 0x20;
                    local_2d4 = local_2d0 & 0xffff0000 | (DAT_005967bc + 0x24);
                    local_2d8 = (DAT_005967bc + 0x24);
                    local_2e0 = DAT_005967bc + 0x20;
                    local_2e4 = local_2e0 & 0xffff0000 | (DAT_005967bc + 0x22);
                    local_2e8 = (DAT_005967bc + 0x22);
                    local_2f0 = (DAT_005967bc + 0x20);
                    local_2f4 = local_2f0 & 0xffff0000 | *local_2f0;
                    local_2ec = local_2f0;
                    local_2dc = local_2e0;
                    local_2cc = local_2d0;
                    local_2c0 = local_2c4;
                    FUN_00450dbf(&DAT_00595740,*local_2f0,local_2e8,local_2d8,DAT_005967bc,local_2c8,local_2bc,
                                 0x0);
                }
                else {
                    bVar1 = FUN_00452ec6(&DAT_00595740);
                    if (CONCAT31(extraout_var_00,bVar1) != 0x0) {
                        FUN_00452f5b(&DAT_00595740);
                    }
                }
            }
            else {
                iVar2 = FUN_00434bfa(&DAT_005967b8);
                if (iVar2 == 0x0) {
                    local_2f8 = &DAT_005967b8;
                    local_2fc = (DAT_005967bc == 0x0);
                    local_300 = local_2fc;
                    if ((local_2fc == 0x0) && (*(DAT_005967bc + 0x23) >> 0x18 == DAT_004c9754)) {
                        FUN_00452d01(&DAT_00595740);
                        local_310 = local_308;
                        uStack780 = uStack772;
                        FUN_00431d5a(&DAT_005967b8,&local_310);
                    }
                }
                else {
                    FUN_00431d0a(&DAT_005967b8);
                }
            }
        }
        local_48 = local_4c;
        FUN_0049cef0(local_248,0x0);
        ppuStack263 = &PTR_FUN_004c3d34;
        if (local_5b != 0x0) {
            FUN_00499b30(local_14c,local_5b);
        }
        FUN_0049a1c0(local_14c,0x1);
    }
    else {
        local_48 = 0x0;
    }
    return local_48;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0048c9aa(param_1: i32) -> u32

{
    let mut iVar1: i32;
    let mut pcVar2: String;
    let local_28c: *mut u32;
    let local_250: *mut u32;
    let local_204: *mut u32;
    let mut local_1b4: *mut u32 [0x11];
    let ppuStack367: *mut *mut u8;;
    let mut local_c3: String;;
    i32 aiStack188 [0x20];
    let mut local_3c: u32;
    let local_38: *mut u32;
    let local_34: *mut u32;
    let mut local_30: i32;
    let mut local_2c: i32;
    let local_28: *mut u32;
    ushort *local_24;
    let mut local_20: u32;
    ushort *local_1c;
    let mut local_18: u32;

    local_24 = (param_1 + 0x20);
    local_18 = local_24 & 0xffff0000 | *local_24;
    local_1c = local_24;
    if (*local_24 == DAT_004d557c) {
        pcVar2 = FUN_00499050(DAT_0059679c,0x73fe);
        FUN_0049d2e0(0x0,0x1,pcVar2);
        local_3c = 0x0;
    }
    else {
        DAT_005b8c30 = param_1;
        DAT_005b8c58 = 0xffffffff;
        DAT_005b8c54 = FUN_0049c2c9(0x780);
        FUN_004990e0(local_1b4,0x0,s_efs_res_004c278c,s_BuildStruct_004c2780);
        local_30 = 0x0;
        for (local_2c = 0x0; local_2c < 0x20; local_2c = local_2c + 0x1) {
            if (*(&DAT_0058320c + local_2c * 0x50) == 0x0) {
                aiStack188[local_2c] = 0x0;
                *(&DAT_005b8c64 + local_2c * 0xc) = 0xffffffff;
                *(&DAT_005b8c68 + local_2c * 0xc) = 0xffffffff;
            }
            else {
                if (local_30 < 0xa) {
                    *(&DAT_005b8c64 + local_2c * 0xc) = 0xa;
                    *(&DAT_005b8c68 + local_2c * 0xc) = local_30 * 0x2f + 0x5;
                    local_204 = FUN_004a2831(0x5d);
                    if (local_204 != 0x0) {
                        local_204 = FUN_0049a030(local_204,local_1b4,local_2c + 0x12c,0xa,local_30 * 0x2f + 0x5,0x30,0x28,
                                                 0x4002,0xcaccce,0xe0e0e);
                        $1: &mut String(local_204 + 0x45) = &PTR_FUN_004c3d94;
                        *(local_204 + 0x51) = 0x0;
                        *(local_204 + 0x55) = 0x0;
                        *(local_204 + 0x4d) = 0x0;
                        *(local_204 + 0x49) = 0x2;
                    }
                    aiStack188[local_2c] = local_204;
                }
                else {
                    *(&DAT_005b8c64 + local_2c * 0xc) = 0x14a;
                    *(&DAT_005b8c68 + local_2c * 0xc) = (local_30 + -0xa) * 0x2f + 0x5;
                    local_250 = FUN_004a2831(0x5d);
                    if (local_250 != 0x0) {
                        local_250 = FUN_0049a030(local_250,local_1b4,local_2c + 0x12c,0x14a,(local_30 + -0xa) * 0x2f + 0x5,0x30
                                                 ,0x28,0x4002,0xcaccce,0xe0e0e);
                        $1: &mut String(local_250 + 0x45) = &PTR_FUN_004c3d94;
                        *(local_250 + 0x51) = 0x0;
                        *(local_250 + 0x55) = 0x0;
                        *(local_250 + 0x4d) = 0x0;
                        *(local_250 + 0x49) = 0x2;
                    }
                    aiStack188[local_2c] = local_250;
                }
                FUN_0049bf40(local_1b4,aiStack188[local_2c]);
                local_30 = local_30 + 0x1;
            }
        }
        local_28c = FUN_004a2831(0x5d);
        local_38 = local_28c;
        local_28 = local_28c;
        if (local_28c != 0x0) {
            local_28c = FUN_0049a030(local_28c,local_1b4,0x190,0x14a,0x1ac,0x30,0x28,0x4002,0xcaccce,0xe0e0e);
            $1: &mut String(local_28c + 0x45) = &PTR_FUN_004c3d94;
            *(local_28c + 0x51) = 0x0;
            *(local_28c + 0x55) = 0x0;
            *(local_28c + 0x4d) = 0x0;
            *(local_28c + 0x49) = 0x2;
        }
        local_34 = local_28c;
        FUN_0049bf40(local_1b4,local_28c);
        _DAT_005b8de4 = 0x14a;
        _DAT_005b8de8 = 0x1ac;
        local_20 = FUN_0049bb50(local_1b4,FUN_0048d209);
        for (local_2c = 0x0; local_2c < 0x20; local_2c = local_2c + 0x1) {
            if ((aiStack188[local_2c] != 0x0) && (iVar1 = aiStack188[local_2c], iVar1 != 0x0)) {
                ((*(iVar1 + 0x45) + 0x8))(iVar1,0x2);
            }
        }
        if ((local_34 != 0x0) && (local_34 != 0x0)) {
            ((*(local_34 + 0x45) + 0x8))(local_34,0x2);
        }
        FUN_0049af50(DAT_005b8c54);
        FUN_004864f7();
        local_3c = local_20;
        ppuStack367 = &PTR_FUN_004c3d34;
        if (local_c3 != 0x0) {
            FUN_00499b30(local_1b4,local_c3);
        }
        FUN_0049a1c0(local_1b4,0x1);
    }
    return local_3c;
}



fn FUN_0048d183(param_1: i32,param_2: i32)

{
    FUN_004953d7();
    if (param_2 != -0x1) {
        FUN_0049e640(*(&DAT_005b8c64 + param_2 * 0xc),*(&DAT_005b8c68 + param_2 * 0xc),0x30,0x28,0xce,0xca,
                     0xcc,0x1);
    }
    FUN_0049e640(*(&DAT_005b8c64 + param_1 * 0xc),*(&DAT_005b8c68 + param_1 * 0xc),0x30,0x28,0x27,0x27,0x27,
                 0x1);
    FUN_0049536f();
    return;
}



// WARNING: Switch with 1 destination removed at 0x0048de38 : 5 cases all go to same destination
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0048d209(param_1: *mut *mut u32,param_2: u32,param_3: u32) -> u32

{
    let sVar1: i16;
    let sVar2: i16;
    let sVar3: i16;
    let mut iVar4: i32;
    let mut pcVar5: String;
    let mut uVar6: u32;
    let mut uVar7: u32;
    let mut iVar8: i32;
    let mut uVar9: u32;
    let puVar10: *mut u32;
    let bVar11: u8;
    let local_1e8: u8 [0x80];
    let mut local_168: String;
    let mut local_164: u32;
    let local_160: *mut i32;;
    let local_15c: *mut i32;;
    let mut local_158: i32;
    let local_154: *mut i32;;
    let local_150: *mut i32;;
    let mut local_14c: i32;
    let mut local_148: u32;
    let local_144: *mut i32;;
    let local_140: *mut i32;;
    let mut local_13c: u32;
    let mut local_138: u32;
    let local_134: *mut i32;;
    let local_130: *mut i32;;
    let local_12c: *mut i32;;
    let local_128: *mut i32;;
    let mut local_124: *mut u8;
    let local_120: *mut i32;;
    let local_11c: *mut i32;;
    let local_118: *mut i32;;
    let mut local_114: u32;
    let mut local_110: u32;
    let mut local_10c: u32;
    let mut local_108: u32;
    let local_104: *mut u32;
    let local_100: *mut u32;
    let local_fc: *mut u32;
    let mut local_f8: u32;
    let local_f4: *mut i32;;
    let local_f0: *mut i32;;
    let mut local_ec: u32;
    let mut local_e8: u32;
    let local_e4: *mut i32;;
    let local_e0: *mut i32;;
    let mut local_dc: i32;
    let mut local_d8: u32;
    let local_d4: *mut i32;;
    let local_d0: *mut i32;;
    let mut local_cc: u32;
    let local_c8: *mut i32;;
    let local_c4: *mut i32;;
    let mut local_c0: u32;
    let local_bc: *mut i32;;
    let local_b8: *mut i32;;
    let mut local_b4: i32;
    let local_b0: *mut i32;;
    let local_ac: *mut i32;;
    let mut local_a8: u32;
    let mut local_a4: u32;
    let local_a0: *mut u32;
    let local_9c: *mut u32;
    let local_98: *mut u32;
    let mut local_94: u32;
    let mut local_90: i32;
    let mut local_8c: i32;
    let mut local_88: u32;
    let local_84: *mut i32;;
    let local_80: *mut i32;;
    let mut local_7c: u32;
    let local_78: *mut i32;;
    let local_74: *mut i32;;
    let mut local_70: i32;
    let local_6c: *mut i32;;
    let local_68: *mut i32;;
    let mut local_64: i32;
    let mut local_60: u32;
    let local_5c: *mut i32;;
    let local_58: *mut i32;;
    let mut local_54: u32;
    let mut local_50: i32;
    let mut local_4c: u32;
    let local_48: *mut i32;;
    let local_44: *mut i32;;
    let local_40: *mut i32;;
    let local_3c: *mut i32;;
    let mut local_38: *mut u8;
    let local_34: *mut i32;;
    let local_30: *mut i32;;
    let local_2c: *mut i32;;
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_20: u32;
    let mut local_1c: i32;
    let mut local_14: u32;

    local_14 = param_2;
    if (param_2 < 0x405) {
        if (((param_2 == 0x401) && (_DAT_004c9750 != 0x0)) && (*(DAT_005b8c30 + 0x23) >> 0x18 == DAT_004c9754))
        {
            local_48 = DAT_005b8c30 + 0x8;
            local_24 = local_48 & 0xffff0000 | local_48;
            local_3c = DAT_005b8c30 + 0x8;
            local_38 = &DAT_004d7d50 + local_48 * 0x3890;
            local_20 = local_3c & 0xffff0000 | (DAT_005b8c30 + 0x22);
            local_30 = (local_38 + (DAT_005b8c30 + 0x22) * 0x4);
            local_40 = DAT_005b8c30 + 0x8;
            local_28 = local_40 & 0xffff0000 | (DAT_005b8c30 + 0x9);
            DAT_005b8c4c = *(*local_30 + (DAT_005b8c30 + 0x9) * 0x4);
            local_44 = local_48;
            local_34 = local_3c;
            local_2c = local_40;
            for (local_1c = 0x0; local_1c < 0x20; local_1c = local_1c + 0x1) {
                if (*(&DAT_0058320c + local_1c * 0x50) != 0x0) {
                    if (((&DAT_00569c30)[DAT_004c9754 * 0x1e22 + *(&DAT_00583220 + local_1c * 0x50) * 0x9] & 0x1) == 0x0) {
                        FUN_0049bf80(param_1,local_1c + 0x12c,0x410,0x0,0x0);
                        *(&DAT_005b8c60 + local_1c * 0xc) = 0x2;
                    }
                    else {
                        local_50 = local_1c;
                        local_54 = DAT_005b8c4c & 0xf;
                        local_5c = DAT_005b8c30 + 0x8;
                        local_60 = local_5c & 0xffff0000 | (DAT_005b8c30 + 0x9);
                        local_64 = (DAT_005b8c30 + 0x9);
                        local_6c = DAT_005b8c30 + 0x8;
                        local_4c = local_6c & 0xffff0000 | (DAT_005b8c30 + 0x22);
                        local_70 = (DAT_005b8c30 + 0x22);
                        local_78 = DAT_005b8c30 + 0x8;
                        local_7c = local_78 & 0xffff0000 | local_78;
                        local_74 = local_78;
                        local_68 = local_6c;
                        local_58 = local_5c;
                        iVar4 = FUN_0043a96a(local_78,local_70,local_64,local_54,local_1c);
                        if (iVar4 == 0x0) {
                            FUN_0049bf80(param_1,local_1c + 0x12c,0x410,0x0,0x0);
                            *(&DAT_005b8c60 + local_1c * 0xc) = 0x1;
                        }
                        else {
                            if (DAT_005b8c58 == -0x1) {
                                DAT_005b8c58 = local_1c;
                            }
                            *(&DAT_005b8c60 + local_1c * 0xc) = 0x0;
                        }
                    }
                }
            }
            if (((((DAT_005b8c4c & 0xf) == 0xb) || ((DAT_005b8c4c & 0xf) == 0x0)) || ((DAT_005b8c4c & 0xf) == 0xa)) &&
                (FUN_0049bf80(param_1,0x190,0x410,0x0,0x0), DAT_005b8c58 == -0x1)) {
                DAT_005b8c58 = 0x20;
            }
            FUN_0049f99b(DAT_005b9bd4,param_1,0x407,0x3039,0x0);
        }
    }
    else {
        if (param_2 < 0x406) {
            FUN_004953d7();
            local_84 = DAT_005b8c30 + 0x8;
            local_88 = local_84 & 0xffff0000 | local_84;
            local_8c = *(&DAT_004daab0 + local_84 * 0x3890);
            local_90 = 0x0;
            local_80 = local_84;
            for (local_94 = 0x0; local_94 < 0x20; local_94 = local_94 + 0x1) {
                if (*(&DAT_0058320c + local_94 * 0x50) != 0x0) {
                    local_9c = (local_8c * 0x940 + 0x565c90 + local_94 * 0x4);
                    local_a0 = *local_9c;
                    local_98 = local_9c;
                    FUN_00401010(DAT_005b8c54,local_a0,0x30,0x28,0x1c);
                    if (local_90 < 0xa) {
                        FUN_00496ee6(DAT_005b8c54,0xa,local_90 * 0x2f + 0x5,0x30,0x28);
                        FUN_00497567(0x3f,local_90 * 0x2f + 0x5,(&DAT_005831e8 + local_94 * 0x50),0x78,0xcaccce,-0x1,0xcaccce,
                                     LPCSTR_005b9218,0x10);
                        FUN_0049e640(0xa,local_90 * 0x2f + 0x5,0x30,0x28,0xce,0xca,0xcc,0x1);
                        local_b0 = DAT_005b8c30 + 0x8;
                        local_a4 = local_b0 & 0xffff0000 | (DAT_005b8c30 + 0x9);
                        local_b4 = (DAT_005b8c30 + 0x9);
                        local_bc = DAT_005b8c30 + 0x8;
                        local_a8 = local_bc & 0xffff0000 | (DAT_005b8c30 + 0x22);
                        local_c0 = SEXT24((DAT_005b8c30 + 0x22));
                        local_c8 = DAT_005b8c30 + 0x8;
                        local_cc = local_c8 & 0xffff0000 | local_c8;
                        local_c4 = local_c8;
                        local_b8 = local_bc;
                        local_ac = local_b0;
                        FUN_0043667f(local_94,0x3f,local_90 * 0x2f + 0x11,local_c8,local_c0,local_b4,
                                     *(&DAT_005b8c60 + local_94 * 0xc),0x0);
                    }
                    else {
                        FUN_00496ee6(DAT_005b8c54,0x14a,(local_90 + -0xa) * 0x2f + 0x5,0x30,0x28);
                        FUN_00497567(0x17f,(local_90 + -0xa) * 0x2f + 0x5,(&DAT_005831e8 + local_94 * 0x50),0x78,0xcaccce,-0x1,
                                     0xcaccce,LPCSTR_005b9218,0x10);
                        FUN_0049e640(0x14a,(local_90 + -0xa) * 0x2f + 0x5,0x30,0x28,0xce,0xca,0xcc,0x1);
                        local_d4 = DAT_005b8c30 + 0x8;
                        local_d8 = local_d4 & 0xffff0000 | (DAT_005b8c30 + 0x9);
                        local_dc = (DAT_005b8c30 + 0x9);
                        local_e4 = DAT_005b8c30 + 0x8;
                        local_e8 = local_e4 & 0xffff0000 | (DAT_005b8c30 + 0x22);
                        local_ec = SEXT24((DAT_005b8c30 + 0x22));
                        local_f4 = DAT_005b8c30 + 0x8;
                        local_f8 = local_f4 & 0xffff0000 | local_f4;
                        local_f0 = local_f4;
                        local_e0 = local_e4;
                        local_d0 = local_d4;
                        FUN_0043667f(local_94,0x17f,(local_90 + -0xa) * 0x2f + 0x11,local_f4,local_ec,
                                     local_dc,*(&DAT_005b8c60 + local_94 * 0xc),0x0);
                    }
                    local_90 = local_90 + 0x1;
                }
            }
            local_100 = (local_8c * 0x940 + 0x5659d8);
            local_104 = *local_100;
            local_fc = local_100;
            FUN_00401010(DAT_005b8c54,local_104,0x30,0x28,0x1c);
            FUN_00496ee6(DAT_005b8c54,0x14a,0x1ac,0x30,0x28);
            bVar11 = 0x10;
            uVar9 = 0xcaccce;
            iVar8 = -0x1;
            uVar7 = 0xcaccce;
            iVar4 = 0x96;
            puVar10 = LPCSTR_005b9218;
            pcVar5 = FUN_00499050(DAT_0059679c,0x713b);
            FUN_00497567(0x17f,0x1ac,pcVar5,iVar4,uVar7,iVar8,uVar9,puVar10,bVar11);
            FUN_0049e640(0x14a,0x1ac,0x30,0x28,0xce,0xca,0xcc,0x1);
            if (DAT_005b8c58 == 0x20) {
                FUN_0048d183(0x20,-0x1);
            }
            else {
                FUN_0048d183(DAT_005b8c58,-0x1);
            }
            FUN_0049536f();
            return 0x0;
        }
        if (0x406 < param_2) {
            if (param_2 < 0x408) {
                if ((param_3 < 0x12c) || (0x18f < param_3)) {
                    if (param_3 == 0x190) {
                        FUN_0048d183(0x20,DAT_005b8c58);
                        DAT_005b8c58 = 0x20;
                    }
                    else {
                        local_108 = param_3;
                        if (param_3 < 0xc1) {
                            if (param_3 == 0x65) {
                                FUN_0049c140(param_1,0x0);
                            }
                        }
                        else {
                            if (param_3 < 0xc2) {
                                if (DAT_005b8c58 == 0x20) {
                                    local_11c = DAT_005b8c30 + 0x8;
                                    local_114 = local_11c & 0xffff0000 | local_11c;
                                    local_128 = DAT_005b8c30 + 0x8;
                                    local_124 = &DAT_004d7d50 + local_11c * 0x3890;
                                    local_10c = local_128 & 0xffff0000 | (DAT_005b8c30 + 0x22);
                                    local_12c = (local_124 + (DAT_005b8c30 + 0x22) * 0x4);
                                    local_134 = DAT_005b8c30 + 0x8;
                                    local_110 = local_134 & 0xffff0000 | (DAT_005b8c30 + 0x9);
                                    local_138 = *(*local_12c + (DAT_005b8c30 + 0x9) * 0x4);
                                    if ((((local_138 & 0xf) != 0xb) && ((local_138 & 0xf) != 0x0)) && ((local_138 & 0xf) != 0xa)) {
                                        local_144 = DAT_005b8c30 + 0x8;
                                        local_148 = local_144 & 0xffff0000 | (DAT_005b8c30 + 0x9);
                                        local_14c = (DAT_005b8c30 + 0x9);
                                        local_154 = DAT_005b8c30 + 0x8;
                                        local_13c = local_154 & 0xffff0000 | (DAT_005b8c30 + 0x22);
                                        local_158 = (DAT_005b8c30 + 0x22);
                                        local_160 = DAT_005b8c30 + 0x8;
                                        local_164 = local_160 & 0xffff0000 | local_160;
                                        local_15c = local_160;
                                        local_150 = local_154;
                                        local_140 = local_144;
                                        local_130 = local_134;
                                        local_120 = local_128;
                                        local_118 = local_11c;
                                        FUN_0043b7fd(local_160,local_158,local_14c);
                                        FUN_0049c140(param_1,0x1);
                                    }
                                }
                                else {
                                    iVar4 = FUN_00430d15(&DAT_005831e8 + DAT_005b8c58 * 0x50);
                                    if (iVar4 == 0x0) {
                                        local_168 = FUN_00499050(DAT_0059679c,0x7132);
                                    }
                                    else {
                                        local_168 = FUN_00499050(DAT_0059679c,0x7133);
                                    }
                                    pcVar5 = FUN_00499050(DAT_0059679c,0x73bb);
                                    FUN_0049c2e0(local_1e8,pcVar5);
                                    uVar6 = FUN_0049d2e0(param_1,0x3,local_1e8);
                                    if (uVar6 != 0x0) {
                                        sVar1 = (DAT_005b8c30 + 0x8);
                                        sVar2 = (DAT_005b8c30 + 0x22);
                                        sVar3 = (DAT_005b8c30 + 0x9);
                                        iVar4 = FUN_0043ab95(sVar1,sVar2,sVar3,*(DAT_005b8c30 + 0x23) >> 0x18,
                                                             *(DAT_005b8c30 + 0x32) >> 0x18,DAT_005b8c58);
                                        if (iVar4 - 0x7148U < 0x5) {
                                            pcVar5 = FUN_00499050(DAT_0059679c,iVar4);
                                            FUN_0049d2e0(param_1,0x1,pcVar5);
                                        }
                                        else {
                                            FUN_00484b4e(DAT_005b8c30);
                                            FUN_00450dbf(&DAT_00595740,sVar1,sVar2,sVar3,DAT_005967bc,0x1,-0x1,0x0);
                                            FUN_0049c140(param_1,0x1);
                                        }
                                    }
                                }
                            }
                            else {
                                if (param_3 == 0x3039) {
                                    FUN_00483355(0x1e);
                                }
                            }
                        }
                    }
                }
                else {
                    FUN_0048d183(param_3 - 0x12c,DAT_005b8c58);
                    DAT_005b8c58 = param_3 - 0x12c;
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



fn FUN_0048deed(param_1: *mut *mut u32) -> String

{
    let mut puVar1: *mut u8;
    let mut local_14: *mut u8;

    if ((DAT_005b8c30 + 0x27) == '\x1d') {
        puVar1 = FUN_0049bf80(param_1,0x3fc,0x414,0x1,0x0);
        for (local_14 = 0x3fd; local_14 < 0x403; local_14 = local_14 + 0x1) {
            FUN_0049bf80(param_1,local_14,0x414,0x0,0x0);
            puVar1 = local_14;
        }
    }
    else {
        FUN_0049bf80(param_1,0x3fc,0x414,0x0,0x0);
        FUN_0049bf80(param_1,0x3fd,0x414,0x1,0x0);
        if ((DAT_005b8c30 + 0x27) == '[') {
            FUN_0049bf80(param_1,0x3fe,0x414,0x0,0x0);
            FUN_0049bf80(param_1,0x3ff,0x414,0x1,0x0);
        }
        else {
            FUN_0049bf80(param_1,0x3ff,0x414,0x0,0x0);
            FUN_0049bf80(param_1,0x3fe,0x414,0x1,0x0);
        }
        FUN_0049bf80(param_1,0x400,0x414,0x1,0x0);
        FUN_0049bf80(param_1,0x401,0x414,0x1,0x0);
        puVar1 = FUN_0049bf80(param_1,0x402,0x414,0x1,0x0);
    }
    return puVar1;
}



fn FUN_0048e055(param_1: i32,param_2: i32)

{
    let mut pcVar1: String;
    let mut bVar2: bool;
    let mut iVar3: i32;
    let mut uVar4: u32;
    let mut iVar5: i32;
    let mut uVar6: u32;
    let puVar7: *mut u32;
    let bVar8: u8;
    let local_56c: u8 [0x18];
    let mut local_554: i32;
    let mut local_550: i32;
    let mut local_54c: i32;
    let mut local_548: u32;
    let mut local_544: i32;
    let mut local_540: i32;
    let local_53c: *mut u32;
    let local_538: *mut u32;
    let local_534: *mut u32;
    let mut local_530: i32;
    let mut local_52c: i32;
    u32 local_528 [0x121];
    let local_a4: u8 [0x80];
    let mut local_24: u32;
    let local_20: *mut *mut char;
    let local_1c: *mut *mut char;
    let mut local_18: String;
    let mut local_14: i32;

    FUN_004953d7();
    local_14 = *(DAT_005b8c30 + 0x30) >> 0x18;
    local_20 = (&DAT_004d6058 + (*(DAT_005b8c30 + 0x24) >> 0x18) * 0x1c);
    local_18 = *local_20;
    local_1c = local_20;
    FUN_004906c1(local_528,local_18,
                 (char)*(&DAT_004be9e8 + (*(DAT_005b8c30 + 0x23) >> 0x18) * 0x4),local_14,0x22,-0x1);
    FUN_00496ac0(local_528,*(param_1 + 0x1d) + 0x190,*(param_1 + 0x21) + 0x17,0x22,0x22);
    FUN_0049e640(*(param_1 + 0x1d) + 0x190,*(param_1 + 0x21) + 0x17,0x22,0x22,0xce,0xca,0xcc,0x1);
    bVar2 = *(*(&DAT_00582938 +
        (*(DAT_005b8c30 + 0x25) >> 0x18) * 0x4 +
        (*(DAT_005b8c30 + 0x24) >> 0x18) * 0x18) + 0xed) != -0x1;
    if (bVar2) {
        local_530 = *(param_1 + 0x21) + 0xa6;
        local_52c = *(param_1 + 0x1d) + 0x87;
        local_53c =
            (&DAT_004d6058 +
                *(*(&DAT_00582938 +
                    (*(DAT_005b8c30 + 0x25) >> 0x18) * 0x4 +
                    (*(DAT_005b8c30 + 0x24) >> 0x18) * 0x18) + 0xed) * 0x1c);
        local_534 = *local_53c;
        local_538 = local_53c;
        FUN_00496ac0(local_534,local_52c,local_530,0x20,0x20);
    }
    local_24 = bVar2;
    if (param_2 != 0x0) {
        local_544 = 0x0;
        for (local_540 = 0x0; local_540 < 0xd; local_540 = local_540 + 0x1) {
            if ((local_540 != 0x4) && (local_540 != 0x1)) {
                if ((local_540 == 0x0) && (local_24 != 0x0)) {
                    local_544 = local_544 + 0x1;
                }
                else {
                    FUN_00496ee6(&DAT_00596a58 + local_540 * 0x3da,*(param_1 + 0x1d) + 0x86 + local_544 * 0x26,
                                 *(param_1 + 0x21) + 0xa7,0x22,0x1d);
                    local_548 = *
                        (local_540 * 0x4 +
                            *(&DAT_00582938 +
                                (*(DAT_005b8c30 + 0x25) >> 0x18) * 0x4 +
                                (*(DAT_005b8c30 + 0x24) >> 0x18) * 0x18) + 0xb9);
                    FUN_0049c2e0(local_a4,&DAT_004c2794);
                    FUN_00497567(local_544 * 0x26 + *(param_1 + 0x1d) + 0x97,*(param_1 + 0x21) + 0xbe,local_a4,
                                 0x1e,0xcaccce,-0x1,0xcaccce,LPCSTR_005b9218,0x11);
                    local_544 = local_544 + 0x1;
                }
            }
        }
    }
    FUN_0048eb27(param_1,*(DAT_005b8c30 + 0x24) >> 0x18,*(DAT_005b8c30 + 0x25) >> 0x18,0x0);
    if ((DAT_005b8c30 + 0x27) == '\x1d') {
        FUN_00497567(*(param_1 + 0x1d) + 0xe7,*(param_1 + 0x21) + 0x7,
                     (&DAT_004d7734 + (*(DAT_005b8c30 + 0x2d) >> 0x18) * 0x67),0xcd,0xcaccce,0xe0e0e,0xcaccce,
                     LPCSTR_005b9218,0x10);
        FUN_004968e7(*(param_1 + 0x1d) + 0xe7,*(param_1 + 0x21) + 0x17,0x87,0x37,0xe);
        local_550 = FUN_004a3840(&DAT_004d7754 + (*(DAT_005b8c30 + 0x2d) >> 0x18) * 0x67,local_56c,0x87,0x3,
                                 &local_554,LPCSTR_005b9218,0x0);
        for (local_54c = 0x0; local_54c < local_550; local_54c = local_54c + 0x1) {
            FUN_00497567(*(param_1 + 0x1d) + 0xe7,local_54c * 0xb + *(param_1 + 0x21) + 0x17,
                         *(local_56c + local_54c * 0x6),*(local_56c + local_54c * 0x6 + 0x2) >> 0x10,0xcaccce,
                         0xe0e0e,0xcaccce,LPCSTR_005b9218,0x0);
        }
    }
    else {
        FUN_0049c2e0(local_a4,&DAT_004c2797);
        FUN_00497567(*(param_1 + 0x1d) + 0xe7,*(param_1 + 0x21) + 0x7,local_a4,0xcd,0xcaccce,0xe0e0e,
                     0xcaccce,LPCSTR_005b9218,0x10);
        if ((DAT_005b8c30 + 0x27) == '[') {
            FUN_0049c2e0(local_a4,s__s__d_004c279a);
        }
        else {
            FUN_0049c2e0(local_a4,&DAT_004c27a0);
        }
        FUN_00497567(*(param_1 + 0x1d) + 0xe7,*(param_1 + 0x21) + 0x22,local_a4,0x87,0xcaccce,0xe0e0e,
                     0xcaccce,LPCSTR_005b9218,0x10);
        bVar8 = 0x10;
        uVar6 = 0xcaccce;
        iVar5 = 0xe0e0e;
        uVar4 = 0xcaccce;
        iVar3 = 0x87;
        puVar7 = LPCSTR_005b9218;
        pcVar1 = FUN_00499050(DAT_0059679c,(*(DAT_005b8c30 + 0x31) >> 0x18) + 0x40a);
        FUN_00497567(*(param_1 + 0x1d) + 0xe7,*(param_1 + 0x21) + 0x17,pcVar1,iVar3,uVar4,iVar5,uVar6,
                     puVar7,bVar8);
        FUN_0049c2e0(local_a4,&DAT_004c27a5);
        FUN_00497567(*(param_1 + 0x1d) + 0xe7,*(param_1 + 0x21) + 0x2d,local_a4,0x87,0xcaccce,0xe0e0e,
                     0xcaccce,LPCSTR_005b9218,0x10);
        FUN_00499050(DAT_0059679c,(*(DAT_005b8c30 + 0x2b) >> 0x18) + 0x40c);
        FUN_0049c2e0(local_a4,&DAT_004c27aa);
        FUN_00497567(*(param_1 + 0x1d) + 0xe7,*(param_1 + 0x21) + 0x38,local_a4,0x87,0xcaccce,0xe0e0e,
                     0xcaccce,LPCSTR_005b9218,0x10);
        FUN_0049c2e0(local_a4,&DAT_004c27ad);
        FUN_00497567(*(param_1 + 0x1d) + 0xe7,*(param_1 + 0x21) + 0x43,local_a4,0x87,0xcaccce,0xe0e0e,
                     0xcaccce,LPCSTR_005b9218,0x10);
        if (*(DAT_005b8c30 + 0x23) >> 0x18 == DAT_004c9754) {
            if ((*(DAT_005b8c30 + 0x3b) & 0x4) == 0x0) {
                if ((*(DAT_005b8c30 + 0x3b) & 0x8) == 0x0) {
                    FUN_004968e7(*(param_1 + 0x1d) + 0x172,*(param_1 + 0x21) + 0x43,0x41,0xa,0xe);
                }
                else {
                    FUN_00499050(DAT_0059679c,0x713d);
                    FUN_0049c2e0(local_a4,&DAT_004c27b5);
                    FUN_00497567(*(param_1 + 0x1d) + 0x1b2,*(param_1 + 0x21) + 0x43,local_a4,0x41,0xdedede,
                                 0xe0e0e,0xdedede,LPCSTR_005b9218,0x14);
                }
            }
            else {
                FUN_00499050(DAT_0059679c,0x713d);
                FUN_0049c2e0(local_a4,&DAT_004c27b0);
                FUN_00497567(*(param_1 + 0x1d) + 0x1b2,*(param_1 + 0x21) + 0x43,local_a4,0x41,0xe3e5e7,
                             0xe0e0e,0xe3e5e7,LPCSTR_005b9218,0x14);
            }
            if ((*(DAT_005b8c30 + 0x3b) & 0x1) == 0x0) {
                FUN_004968e7(*(param_1 + 0x1d) + 0x172,*(param_1 + 0x21) + 0x4d,0x41,0xb,0xe);
            }
            else {
                FUN_00499050(DAT_0059679c,0x713c);
                FUN_0049c2e0(local_a4,&DAT_004c27ba);
                FUN_00497567(*(param_1 + 0x1d) + 0x1b2,*(param_1 + 0x21) + 0x4e,local_a4,0x41,0xe3e5e7,-0x1,
                             0xe3e5e7,LPCSTR_005b9218,0x14);
            }
        }
    }
    FUN_0049536f();
    return;
}



fn FUN_0048e988(param_1: *mut u8)

{
    let mut local_20: i32;
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: u32;

    local_20 = 0x85;
    local_1c = 0xa5;
    local_18 = 0x19a;
    local_14 = 0x25;
    FUN_00498a5b(&local_20);
    FUN_0049a770(param_1,0x405,0x1,&local_20);
    FUN_00498ae4();
    return;
}



fn FUN_0048e9e0(param_1: *mut u8)

{
    let mut local_30: i32;
    let mut local_2c: u32;
    let mut local_28: u32;
    let mut local_24: u32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    local_20 = DAT_005b8c30 + 0x20;
    if (((DAT_005b8c30 + 0x35) == -0x1) || ((DAT_005b8c30 + 0x26) == (DAT_005b8c30 + 0x35))) {
        local_14 = 0x0;
    }
    else {
        local_14 = 0x1;
    }
    local_18 = local_14;
    local_1c = local_20;
    if (local_14 == 0x0) {
        FUN_00496d7e(0x1c,0x181,0x4b,0x4b,*(&DAT_00595700 + (*(DAT_005b8c30 + 0x23) >> 0x18) * 0x4),0x64,0x64)
        ;
    }
    else {
        FUN_00496d7e(0x1c,0x181,0x4b,0x4b,*(&DAT_00595700 + (*(DAT_005b8c30 + 0x32) >> 0x18) * 0x4),0x64,0x64)
        ;
    }
    FUN_0048e988(param_1);
    local_30 = 0x0;
    local_2c = 0xd4;
    local_28 = 0x280;
    local_24 = 0xa1;
    FUN_00498a5b(&local_30);
    FUN_0049a770(param_1,0x405,0x1,&local_30);
    FUN_00498ae4();
    local_30 = 0x7c;
    local_2c = 0x17e;
    local_28 = 0x201;
    local_24 = 0x2d;
    FUN_00498a5b(&local_30);
    FUN_0049a770(param_1,0x405,0x1,&local_30);
    FUN_00498ae4();
    return;
}



fn FUN_0048eb27(param_1: i32,param_2: i32,param_3: i32,param_4: i32) -> i32

{
let mut iVar1: i32;
let mut pcVar2: String;
let local_6c: u8 [0x9];
let local_63: u8;
let local_60: u8 [0x40];
let mut local_20: i32;
let mut local_1c: i32;
let mut local_18: i32;
let mut local_14: i32;

if (param_4 == 0x0) {
local_20 = *(param_1 + 0x1d) + 0xd1;
local_1c = *(param_1 + 0x21) + 0x76;
}
else {
local_20 = *(param_1 + 0x1d) + 0x55;
local_1c = *(param_1 + 0x21) + 0x75;
}
FUN_0049c2e0(local_60,&DAT_004c27bf);
FUN_00497567(local_20,local_1c,local_60,0x18,0xcaccce,0xe0e0e,0xcaccce,LPCSTR_005b9218,0x14);
FUN_0049c2e0(local_60,&DAT_004c27c2);
FUN_00497567(local_20,local_1c + 0xb,local_60,0x18,0xcaccce,0xe0e0e,0xcaccce,LPCSTR_005b9218,0x14);
FUN_0049c2e0(local_60,&DAT_004c27c5);
FUN_00497567(local_20,local_1c + 0x16,local_60,0x18,0xcaccce,0xe0e0e,0xcaccce,LPCSTR_005b9218,0x14)
;
FUN_0049c2e0(local_60,&DAT_004c27c8);
FUN_00497567(local_20,local_1c + 0x21,local_60,0x18,0xcaccce,0xe0e0e,0xcaccce,LPCSTR_005b9218,0x14)
;
FUN_0049c2e0(local_60,&DAT_004c27cb);
FUN_00497567(local_20 + 0x50,local_1c,local_60,0x18,0xcaccce,0xe0e0e,0xcaccce,LPCSTR_005b9218,0x14)
;
FUN_00499050(DAT_0059679c,*(*(&DAT_00582938 + param_2 * 0x18 + param_3 * 0x4) + 0x41) + 0x438);
FUN_0049c2e0(local_60,s__1_1s_004c27ce);
FUN_00497567(local_20 + 0x50,local_1c + 0xb,local_60,0x18,0xcaccce,0xe0e0e,0xcaccce,LPCSTR_005b9218
,0x14);
FUN_0049c2e0(local_60,&DAT_004c27d4);
FUN_00497567(local_20 + 0x50,local_1c + 0x16,local_60,0x18,0xcaccce,0xe0e0e,0xcaccce,
LPCSTR_005b9218,0x14);
FUN_0049c2e0(local_60,&DAT_004c27d7);
FUN_00497567(local_20 + 0x50,local_1c + 0x21,local_60,0x18,0xcaccce,0xe0e0e,0xcaccce,
LPCSTR_005b9218,0x14);
iVar1 = FUN_004968e7(local_20 + 0x5f,local_1c,0x82,0x2c,0xe);
local_18 = 0x0;
local_14 = 0x0;
loop {
if (0x8 < local_14) {
return iVar1;
}
if (*(*(&DAT_00582938 + param_3 * 0x4 + param_2 * 0x18) + local_14 * 0x4 + 0x5d) != 0x0) {
iVar1 = 0x9;
pcVar2 = FUN_00499050(DAT_0059679c,local_14 + 0x1af4);
FUN_004a9a00(local_6c,pcVar2,iVar1);
local_63 = 0x0;
FUN_00497567(local_20 + 0x5f,local_18 * 0xb + local_1c,local_6c,0x4b,0xcaccce,0xe0e0e,0xcaccce,
LPCSTR_005b9218,0x10);
FUN_0049c2e0(local_60,s__d__d_004c27da);
iVar1 = FUN_00497567(local_20 + 0xe0,local_18 * 0xb + local_1c,local_60,0x37,0xcaccce,0xe0e0e,0xcaccce,
LPCSTR_005b9218,0x14);
local_18 = local_18 + 0x1;
if (local_18 == 0x4) {
return iVar1;
}
}
iVar1 = local_14;
local_14 = local_14 + 0x1;
} while( true );
}



fn FUN_0048efa1(param_1: *mut *mut u32)

{
    let mut bVar1: bool;

    if (*(DAT_005b8c30 + 0x23) >> 0x18 == DAT_004c9754) {
        if (((DAT_005b8c30 + 0x35) == -0x1) || ((DAT_005b8c30 + 0x26) == (DAT_005b8c30 + 0x35)))
        {
            bVar1 = false;
        }
        else {
            bVar1 = true;
        }
        if ((((!bVar1) && ((DAT_005b8c30 + 0x27) != '-')) && ((DAT_005b8c30 + 0x27) != '\x1d')) &&
            ((DAT_005b8c30 + 0x27) != '\x1c')) {
            FUN_0049bf80(param_1,0xd4,0x40f,0x0,0x0);
            FUN_0049bf80(param_1,0xd4,0x414,0x1,0x0);
            return;
        }
    }
    FUN_0049bf80(param_1,0xd4,0x414,0x0,0x0);
    FUN_0049bf80(param_1,0xd4,0x410,0x0,0x0);
    return;
}



fn FUN_0048f0a0(param_1: i32) -> i32

{
byte *pbVar1;
let mut pcVar2: String;
let mut local_68: i32;
let mut local_3c: u32;
let mut local_38: i32;
let mut local_30: i32;
let mut local_2c: i32;
let mut local_28: i32;
let mut local_24: i32;
let mut local_20: i32;
let mut local_1c: i32;
let mut local_18: i32;
let mut local_14: u32;

FUN_00431d31(&local_3c);
local_30 = FUN_00452a43(param_1);
local_1c = local_30 + 0x20;
local_14 = *(local_30 + 0x3a) & 0x1;
local_18 = local_1c;
if (local_14 == 0x0) {
local_2c = 0x1;
}
else {
if (((*((*(param_1 + 0x816) >> 0x10) * 0x4 +
*(&DAT_004d7d50 +
(*(param_1 + 0x812) >> 0x10) * 0x3890 + (*(param_1 + 0x814) >> 0x10) * 0x4)) &
0xf) == 0x0) &&
((*((*(param_1 + 0x816) >> 0x10) * 0x4 +
*(&DAT_004d7d50 +
(*(param_1 + 0x812) >> 0x10) * 0x3890 + (*(param_1 + 0x814) >> 0x10) * 0x4) +
0x4) & 0x80) == 0x0)) {
FUN_004a2d6b();
pcVar2 = FUN_00499050(DAT_0059679c,0x739d);
FUN_0049d2e0(0x0,0x1,pcVar2);
return 0x0;
}
local_2c = 0x0;
}
local_28 = 0x0;
local_24 = 0x0;
for (local_20 = 0x0; local_20 < 0x14; local_20 = local_20 + 0x1) {
if ((((local_20 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) &&
(*(*(&DAT_00582938 +
(*(*(local_20 * 0x5e + param_1) + 0x25) >> 0x18) * 0x4 +
(*(*(local_20 * 0x5e + param_1) + 0x24) >> 0x18) * 0x18) + 0xa5) != 0x0)) {
for (local_68 = 0x0;
local_68 <
*(*(&DAT_00582938 +
(*(*(local_20 * 0x5e + param_1) + 0x25) >> 0x18) * 0x4 +
(*(*(local_20 * 0x5e + param_1) + 0x24) >> 0x18) * 0x18) + 0xa5);
local_68 = local_68 + 0x1) {
if (*(*(local_20 * 0x5e + param_1) + local_68 * 0x4 + 0x10) != 0x0) {
if ((((local_2c == 0x0) ||
((*(*(local_20 * 0x5e + param_1) + local_68 * 0x4 + 0x10) + 0x2a) == 0x5)) ||
((*(*(local_20 * 0x5e + param_1) + local_68 * 0x4 + 0x10) + 0x2a) == 0x7)) ||
((*(*(local_20 * 0x5e + param_1) + local_68 * 0x4 + 0x10) + 0x2a) == 0x9)) {
pbVar1 = (*(*(local_20 * 0x5e + param_1) + local_68 * 0x4 + 0x10) + 0x3a);
*pbVar1 = *pbVar1 & 0xbf;
FUN_004841ea(*(i32 ***)(local_68 * 0x4 + *(local_20 * 0x5e + param_1) + 0x10),
(*(local_20 * 0x5e + param_1) + 0x20),*(param_1 + 0x814) >> 0x10,
*(param_1 + 0x816) >> 0x10);
local_38 = FUN_00434de1(*(*(local_20 * 0x5e + param_1) + local_68 * 0x4 + 0x10));
FUN_00431dec(&local_3c,*(local_68 * 0x4 + *(local_20 * 0x5e + param_1) + 0x10));
*(local_68 * 0x4 + *(local_20 * 0x5e + param_1) + 0x10) = 0x0;
local_28 = 0x1;
}
else {
local_24 = 0x1;
}
}
}
}
}
if (((local_28 == 0x0) && (local_24 != 0x0)) && (local_2c != 0x0)) {
FUN_004a2d6b();
pcVar2 = FUN_00499050(DAT_0059679c,0x739a);
FUN_0049d2e0(0x0,0x1,pcVar2);
}
return local_28;
}



// WARNING: Could not reconcile some variable overlaps

fn FUN_0048f4cb(param_1: i32)

{
    let mut iVar1: i32;
    let puVar2: *mut u32;
    let puVar3: *mut u32;
    let mut in_stack_ffffff98: u32;
    let mut in_stack_ffffff9c: u32;
    let mut in_stack_ffffffa0: u32;
    let mut in_stack_ffffffa4: u32;
    let mut in_stack_ffffffa8: u32;
    let mut in_stack_ffffffac: u32;
    let mut in_stack_ffffffb0: u32;
    let mut in_stack_ffffffb4: u32;
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: u32;
    let mut local_1c: u32;
    let mut local_18: u32;

    if (param_1 < 0x14) {
        local_28 = param_1 % 0x5;
        local_24 = param_1 / 0x5;
        local_20 = 0xce;
        local_20._0_1_ = 0xce;
        local_1c = 0xca;
        local_1c._0_1_ = 0xca;
        local_18 = 0xcc;
        local_18._0_1_ = 0xcc;
        if (*(&DAT_00595740 + param_1 * 0x5e) == 0x0) {
            FUN_004968e7(local_28 * 0x80 + 0xa,local_24 * 0x29 + 0xd7,0x6c,0x22,0xe);
        }
        else {
            if (((&DAT_00595744 + param_1 * 0x5e) & 0x2) != 0x0) {
                local_18 = 0xa0a0a;
                local_1c = 0xa0a0a;
                local_20 = 0xa0a0a;
            }
            iVar1 = 0xb;
            puVar2 = (*(&DAT_00595740 + param_1 * 0x5e) + 0x20);
            puVar3 = &stack0xffffff98;
            while( true ) {
                if (iVar1 == 0x0) break;
                iVar1 = iVar1 + -0x1;
                *puVar3 = *puVar2;
                puVar2 = puVar2 + 0x1;
                puVar3 = puVar3 + 0x1;
            }
            puVar3 = puVar2;
            FUN_0049009c(local_28 * 0x80 + 0xa,local_24 * 0x29 + 0xd7,in_stack_ffffff98,in_stack_ffffff9c,in_stack_ffffffa0,
                         in_stack_ffffffa4,in_stack_ffffffa8,in_stack_ffffffac,CONCAT44(in_stack_ffffffb4,in_stack_ffffffb0));
        }
        FUN_0049e640(local_28 * 0x80 + 0xa,local_24 * 0x29 + 0xd7,0x6c,0x22,local_20,local_1c,
                     local_18,0x1);
    }
    return;
}



fn FUN_0048f614(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: i32) -> String

{
    let mut iVar1: i32;
    let mut local_14: *mut u8;

    local_14 = 0xe0e0e;
    iVar1 = (param_1 * 0x64) / param_2;
    if (param_3 < iVar1) {
        if (param_4 < iVar1) {
            if (iVar1 <= param_5) {
                local_14 = &DAT_00575757;
            }
        }
        else {
            local_14 = 0xdedede;
        }
    }
    else {
        local_14 = 0xe7e7e7;
    }
    return local_14;
}



void
FUN_0048f678(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: i32,param_6: i32,param_7: u32,param_8: i32,
param_9: i32,param_10: i32)

{
let mut iVar1: i32;

if (param_9 != 0x0) {
FUN_0049e640(param_1,param_2,param_3,param_4,0x1f,0x13,0x13,0x2);
}
iVar1 = (param_5 * param_3) / param_6;
if (param_10 != 0x0) {
param_7 = FUN_00499f60(param_7);
}
FUN_004968e7(param_1,param_2,iVar1,param_4,(char)param_7);
if (param_8 != -0x1) {
FUN_004968e7(param_1 + iVar1,param_2,param_3 - iVar1,param_4,(char)param_8);
}
return;
}



fn FUN_0048f724(param_1: i32,param_2: i32)

{
    let mut iVar1: i32;
    let puVar2: *mut u32;
    let puVar3: *mut u32;
    let mut in_stack_ffffff88: u32;
    let mut in_stack_ffffff8c: u32;
    let mut in_stack_ffffff90: u32;
    let mut in_stack_ffffff94: u32;
    let mut in_stack_ffffff98: u32;
    let mut in_stack_ffffff9c: i32;
    let mut in_stack_ffffffa0: i32;
    let mut in_stack_ffffffa4: u32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let local_18: *mut u32;
    let mut local_14: i32;

    if (param_2 == 0x1) {
        local_18 = FUN_0049c2c9(0x400);
        FUN_0049c57b(s_bin_fist_bin_004c27fc,local_18,0x400);
    }
    FUN_004953d7();
    for (local_14 = 0x0; local_14 < 0x14; local_14 = local_14 + 0x1) {
        local_20 = local_14 % 0x5;
        local_1c = local_14 / 0x5;
        if (((local_14 * 0x5e + param_1 + 0x4) & 0x1) == 0x0) {
            if (param_2 != 0x1) {
                FUN_004968e7(local_20 * 0x80 + 0xa,local_1c * 0x29 + 0xd7,0x6c,0x22,0xe);
                in_stack_ffffffa4 = 0x6c;
                in_stack_ffffffa0 = local_1c * 0x29 + 0xd7;
                in_stack_ffffff9c = local_20 * 0x80 + 0xa;
                in_stack_ffffff98 = 0x48f99b;
                FUN_0049e640(in_stack_ffffff9c,in_stack_ffffffa0,0x6c,0x22,0xce,0xca,0xcc,0x1);
            }
        }
        else {
            puVar2 = (*(local_14 * 0x5e + param_1) + 0x20);
            puVar3 = &stack0xffffff88;
            for (iVar1 = 0xb; iVar1 != 0x0; iVar1 = iVar1 + -0x1) {
                *puVar3 = *puVar2;
                puVar2 = puVar2 + 0x1;
                puVar3 = puVar3 + 0x1;
            }
            puVar3 = puVar2;
            FUN_0049009c(local_20 * 0x80 + 0xa,local_1c * 0x29 + 0xd7,in_stack_ffffff88,in_stack_ffffff8c,in_stack_ffffff90,
                         in_stack_ffffff94,in_stack_ffffff98,in_stack_ffffff9c,CONCAT44(in_stack_ffffffa4,in_stack_ffffffa0));
            if ((param_2 == 0x1) && ((*(*(local_14 * 0x5e + param_1) + 0x3b) & 0x2) != 0x0)) {
                FUN_00496ee6(local_18,local_20 * 0x80 + 0xa,local_1c * 0x29 + 0xd7,0x20,0x20);
            }
            if ((((local_14 * 0x5e + param_1 + 0x4) & 0x2) == 0x0) || (param_2 == 0x1)) {
                in_stack_ffffffa4 = 0x6c;
                in_stack_ffffffa0 = local_1c * 0x29 + 0xd7;
                in_stack_ffffff9c = local_20 * 0x80 + 0xa;
                in_stack_ffffff98 = 0x48f90b;
                FUN_0049e640(in_stack_ffffff9c,in_stack_ffffffa0,0x6c,0x22,0xce,0xca,0xcc,0x1);
            }
            else {
                in_stack_ffffffa4 = 0x6c;
                in_stack_ffffffa0 = local_1c * 0x29 + 0xd7;
                in_stack_ffffff9c = local_20 * 0x80 + 0xa;
                in_stack_ffffff98 = 0x48f8d8;
                FUN_0049e640(in_stack_ffffff9c,in_stack_ffffffa0,0x6c,0x22,0xa,0xa,0xa,0x1);
            }
            if ((*(local_14 * 0x5e + param_1 + 0x5) & 0x8) != 0x0) {
                in_stack_ffffffa4 = 0x48f93d;
                FUN_0049e5a0(local_20 * 0x80 + 0xa,local_1c * 0x29 + 0xd7,0x22,0x22,0xa);
            }
        }
        if ((*(local_14 * 0x5e + param_1 + 0x4) & 0x4) != 0x0) {
            in_stack_ffffffa4 = 0x48f9ce;
            FUN_00496ee6(&DAT_005b8ef0,local_20 * 0x80 + -0xa,local_1c * 0x29 + 0xd9,0x14,0x1f);
        }
    }
    FUN_0049536f();
    if (param_2 == 0x1) {
        FUN_0049af50(local_18);
    }
    return;
}



fn FUN_0048f9f5(param_1: i32,param_2: i32,param_3: i32,param_4: i32)

{
    let cVar1: u8;
    let mut uVar2: u32;
    let mut pcVar3: String;
    let mut local_78: i32;
    let local_48: *mut i32; [0x6];
    let local_30: u8 [0x10];
    let local_20: *mut u32;
    let mut local_1c: u32;
    let local_18: *mut u32;

    local_20 = FUN_0049c2c9(0x484);
    local_18 = FUN_00498ba4(local_48,local_20,0x22,0x22);
    if (param_4 == 0x0) {
        local_1c = *(&DAT_004be9e8 + (*(param_3 + 0x23) >> 0x18) * 0x4);
    }
    else {
        local_1c = (byte)(&DAT_005b8df0)[*(&DAT_004be9e8 + (*(param_3 + 0x23) >> 0x18) * 0x4) & 0xff];
    }
    if (((*(param_3 + 0x3a) & 0x40) == 0x0) || (*(param_3 + 0x23) >> 0x18 == DAT_004c9754)) {
        FUN_004906c1(local_20,*(&DAT_004d6058 + (*(param_3 + 0x24) >> 0x18) * 0x1c),(char)local_1c,
                     *(param_3 + 0x30) >> 0x18,0x22,-0x1);
        if (*(param_3 + 0x23) >> 0x18 == DAT_004c9754) {
            FUN_0049c2e0(local_30,&DAT_004c280b);
            if (((*(param_3 + 0x32) >> 0x18 == DAT_004c9754) || (*(param_3 + 0x32) >> 0x18 < 0xa)) ||
                (0xc < *(param_3 + 0x32) >> 0x18)) {
                uVar2 = 0xffffffff;
                pcVar3 = local_30;
                loop {
                    if (uVar2 == 0x0) break;
                    uVar2 = uVar2 - 0x1;
                    cVar1 = *pcVar3;
                    pcVar3 = pcVar3 + 0x1;
                } while (cVar1 != '\0');
                FUN_00497567(0x20,0x1,local_30,~uVar2 - 0x1,0x272727,0xe0e0e,0x272727,DAT_004d6a6c,0x4);
            }
            else {
                uVar2 = 0xffffffff;
                pcVar3 = local_30;
                loop {
                    if (uVar2 == 0x0) break;
                    uVar2 = uVar2 - 0x1;
                    cVar1 = *pcVar3;
                    pcVar3 = pcVar3 + 0x1;
                } while (cVar1 != '\0');
                FUN_00497567(0x20,0x1,local_30,~uVar2 - 0x1,0xe0e0e,0x272727,0xe0e0e,DAT_004d6a6c,0x4);
            }
        }
        if ((param_3 + 0x27) == '[') {
            FUN_0049c2e0(local_30,s__2_2s_004c280e);
            for (local_78 = 0x0; local_78 < 0x4; local_78 = local_78 + 0x1) {
                FUN_00497567(*(&DAT_004bf3c0 + local_78 * 0x4) + 0x11,*(&DAT_004bf3e0 + local_78 * 0x4) + 0xb,
                             local_30,0x1e,0xe0e0e,-0x1,0x0,DAT_004d6a6c,0x11);
            }
            FUN_00497567(0x11,0xb,local_30,0x1e,0x272727,-0x1,0x272727,DAT_004d6a6c,0x11);
        }
    }
    else {
        FUN_004906c1(local_20,DAT_004d6a4c,(char)local_1c,0x0,0x22,-0x1);
        FUN_00497567(0x18,0x1,&DAT_004c2809,0x6,0x272727,-0x1,0x272727,LPCSTR_005b9218,0x10);
    }
    if ((*(param_3 + 0x3a) & 0x40) != 0x0) {
        FUN_00497567(0x1,0x1,&DAT_004c2814,0x6,0x272727,-0x1,0x272727,DAT_004d6a6c,0x10);
    }
    if (*(param_3 + 0x23) >> 0x18 == DAT_004c9754) {
        if ((*(param_3 + 0x3b) & 0x4) == 0x0) {
            if ((*(param_3 + 0x3b) & 0x8) != 0x0) {
                FUN_00497567(0x8,0x1,&DAT_004c2818,0x6,0xdedede,-0x1,0xdedede,DAT_004d6a6c,0x10);
            }
        }
        else {
            FUN_00497567(0x8,0x1,&DAT_004c2816,0x6,0xe7e7e7,-0x1,0xe7e7e7,DAT_004d6a6c,0x10);
        }
        if ((*(param_3 + 0x3b) & 0x1) != 0x0) {
            FUN_00497567(0xe,0x1,&DAT_004c281a,0x6,0xe7e7e7,-0x1,0xe7e7e7,DAT_004d6a6c,0x10);
        }
    }
    if ((*(param_3 + 0x3a) & 0x4) != 0x0) {
        FUN_00498799(local_20,local_20,0x484,&DAT_005b96d0);
    }
    FUN_00498cf4(local_48);
    FUN_004953d7();
    FUN_00496ac0(local_20,param_1,param_2,0x22,0x22);
    FUN_0049536f();
    FUN_0049af50(local_20);
    return;
}



fn FUN_0048fe33(param_1: i32,param_2: i32)

{
    let mut bVar1: bool;
    let puVar2: *mut u32;
    let mut iVar3: i32;
    let mut local_58: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    if (param_2 == 0x0) {
        local_14 = 0x0;
    }
    else {
        local_14 = 0x3a;
    }
    local_14 = local_14 + 0xb5;
    if ((*(param_1 + 0x828) & 0x1) == 0x0) {
        bVar1 = false;
        FUN_004953d7();
        for (local_18 = 0x0; local_18 < 0x14; local_18 = local_18 + 0x1) {
            if (((local_18 * 0x5e + param_1 + 0x4) & 0x1) == 0x0) {
                FUN_004968e7((local_18 % 0x3) * 0x22 + 0x9,(local_18 / 0x3) * 0x22 + local_14,0x22,0x22,0xe);
            }
            else {
                if ((*(*(local_18 * 0x5e + param_1) + 0x3a) & 0x1) != 0x0) {
                    bVar1 = true;
                }
                FUN_0048f9f5((local_18 % 0x3) * 0x22 + 0x9,(local_18 / 0x3) * 0x22 + local_14,
                             *(local_18 * 0x5e + param_1),(local_18 * 0x5e + param_1 + 0x4) & 0x2);
                if (((local_18 * 0x5e + param_1 + 0x4) & 0x2) != 0x0) {
                    FUN_0049e5a0((local_18 % 0x3) * 0x22 + 0x9,(local_18 / 0x3) * 0x22 + local_14,0x22,0x22,0xa);
                }
            }
        }
        if (((bVar1) &&
            (puVar2 = FUN_00481784(*(param_1 + 0x812) >> 0x10,*(param_1 + 0x814) >> 0x10,
                                   *(param_1 + 0x816) >> 0x10), puVar2 != 0x0)) &&
            (iVar3 = FUN_00481a44(puVar2), iVar3 == 0x0)) {
            if (param_2 == 0x0) {
                local_58 = 0x0;
            }
            else {
                local_58 = 0x3c;
            }
            FUN_004817f9(0x9,local_58 + 0x89,(short *)(puVar2 + 0x2));
        }
        FUN_0049536f();
    }
    return;
}



// WARNING: Restarted to delay deadcode elimination for space: stack

void
FUN_0049009c(param_1: i32,param_2: i32,param_3: u32,param_4: u32,param_5: u32,param_6: u32,
param_7: u32,param_8: u32,ulonglong param_9)

{
let mut puVar1: *mut u8;
let mut in_stack_0000003c: i32;
let mut iVar2: i32;
let mut iVar3: i32;
let mut iVar4: i32;
let mut local_108: i32;
let mut local_ec: i32;
let local_b4: *mut i32; [0x6];
let local_9c: u8 [0x40];
let local_5c: u8 [0x20];
let local_3c: u8 [0x20];
let local_1c: *mut u32;
let mut local_18: u32;

local_1c = FUN_0049c2c9(0xe58);
if (in_stack_0000003c == 0x0) {
local_18 = *(&DAT_004be9e8 + param_4._2_1_ * 0x4);
}
else {
local_18 = (byte)(&DAT_005b8df0)[*(&DAT_004be9e8 + param_4._2_1_ * 0x4) & 0xff];
}
FUN_00498ba4(local_b4,local_1c,0x22,0x6c);
if (((param_9 & 0x400000) != 0x0) && (param_4._2_1_ != DAT_004c9754)) {
FUN_004906c1(local_1c,DAT_004d6a4c,(char)local_18,0x0,0x6c,-0x1);
FUN_00497567(0x32,0xc,&DAT_004c281c,0x32,0x272727,-0x1,0x272727,LPCSTR_005b9218,0x10);^
// goto LAB_004904d4;
}
FUN_004906c1(local_1c,*(&DAT_004d6058 + (char)param_4._3_1_ * 0x1c),(char)local_18,param_7._3_1_
,0x6c,-0x1);
if (param_4._3_1_ == 0x5b) {
iVar4 = 0x1;
iVar3 = 0x1;
iVar2 = 0xe0e0e;
puVar1 = FUN_0048f614(param_7._1_2_,0x3e7,0x21,0x42,0x64);
FUN_0048f678(0x2d,0x1c,0x32,0x2,param_7._1_2_,0x3e7,puVar1,iVar2,iVar3,iVar4);
FUN_0049c2e0(local_9c,&DAT_004c2820);
FUN_0049c2e0(local_5c,s__d_pts_004c2823);
FUN_00497567(0x26,0xf,local_5c,0x3c,0xcaccce,-0x1,0xcaccce,DAT_004d6a6c,0x10);
FUN_0049c2e0(local_3c,s__1_1s_2d_004c282c);
}
else {
if ((param_4._3_1_ == 0x1d) && ((char)param_7 != -0x1)) {
FUN_0049c2e0(local_9c,&DAT_004c2835);
FUN_0049c2e0(local_3c,s__1_1s_2d_004c283a);
}
else {
FUN_0049c2e0(local_9c,&DAT_004c2843);
if (param_6._2_1_ != '\0') {
FUN_00496ac0((&DAT_005921a8 + (param_6._2_1_ + -0x1) * 0x114),0x38,0xc,0x17,0xc);
}
local_ec = param_5._1_1_ / 0x14;
if (local_ec == 0x5) {
local_ec = 0x4;
}
if (param_4._3_1_ < 0x1d) {
if (param_4._3_1_ != 0x1c)^ // goto LAB_004903e6;
}
else {
if ((0x1d < param_4._3_1_) && (param_4._3_1_ != 0x2d)) {
// LAB_004903e6:
FUN_00496ac0((&DAT_00593140 + local_ec * 0xc3),0x24,0xc,0xf,0xd);
}
}
FUN_0049c2e0(local_3c,s__1_1s_2d_004c284a);
}
}
FUN_00497567(0x23,0x1,local_9c,0x3c,0xcaccce,-0x1,0xcaccce,DAT_004d6a6c,0x10);
FUN_00497567(0x52,0xf,local_3c,0xc,0xcaccce,-0x1,0xcaccce,DAT_004d6a6c,0x10);
// LAB_004904d4:
if ((param_9 & 0x400000) == 0x0) {
if (*(*(&DAT_00582938 + (char)param_4._3_1_ * 0x18 + (char)param_5 * 0x4) + 0xa5) != 0x0) {
for (local_108 = 0x0;
local_108 < *(*(&DAT_00582938 + (char)param_5 * 0x4 + (char)param_4._3_1_ * 0x18) + 0xa5);
local_108 = local_108 + 0x1) {
FUN_00496d7e(local_108 * 0x11 + 0x24,0x1a,0x10,0x6,&DAT_00594a28,0x18,0x9);
}
}
}
else {
FUN_00497567(0x62,0x18,&DAT_004c2853,0x6,0xcaccce,-0x1,0xcaccce,DAT_004d6a6c,0x10);
}
if (param_4._2_1_ == DAT_004c9754) {
if ((param_9 & 0x4000000) == 0x0) {
if ((param_9 & 0x8000000) != 0x0) {
FUN_00497567(0x8,0x1,&DAT_004c2857,0x6,0xdedede,-0x1,0xdedede,DAT_004d6a6c,0x10);
}
}
else {
FUN_00497567(0x8,0x1,&DAT_004c2855,0x6,0xe7e7e7,-0x1,0xe7e7e7,DAT_004d6a6c,0x10);
}
if ((param_9 & 0x1000000) != 0x0) {
FUN_00497567(0xe,0x1,&DAT_004c2859,0x6,0xe7e7e7,-0x1,0xe7e7e7,DAT_004d6a6c,0x10);
}
}
if ((param_9 & 0x40000) != 0x0) {
FUN_00498799(local_1c,local_1c,0xe58,&DAT_005b96d0);
}
FUN_00498cf4(local_b4);
FUN_004953d7();
FUN_00496ac0(local_1c,param_1,param_2,0x6c,0x22);
FUN_0049536f();
FUN_0049af50(local_1c);
return;
}



fn FUN_004906c1(param_1: &mut String,param_2: &mut String,param_3: u8,param_4: i32,param_5: i32,param_6: i32)

{
    let mut pcVar1: String;
    let mut puVar2: *mut u8;
    let mut iVar3: i32;
    let local_34: u8;
    let local_30: u8;
    let local_2c: u8;
    let local_28: u8;
    let mut local_24: i32;
    let mut local_20: i32;

    puVar2 = FUN_0048f614(param_4,0x64,0x21,0x42,0x64);
    iVar3 = param_5 + -0x22;
    for (local_24 = 0x0; local_24 < 0x22; local_24 = local_24 + 0x1) {
        *param_1 = '\x1f';
        param_1 = param_1 + 0x1;
    }
    for (local_24 = 0x0; local_24 < iVar3; local_24 = local_24 + 0x1) {
        *param_1 = '\x0e';
        param_1 = param_1 + 0x1;
    }
    if (param_6 == -0x1) {
        for (local_24 = 0x0; local_24 < 0x1e; local_24 = local_24 + 0x1) {
            *param_1 = '\x1f';
            pcVar1 = param_1;
            for (local_20 = 0x0; param_1 = pcVar1 + 0x1, local_20 < 0x20; local_20 = local_20 + 0x1) {
                if (*param_2 == '\0') {
                    local_28 = param_3;
                }
                else {
                    local_28 = *param_2;
                }
                *param_1 = local_28;
                param_2 = param_2 + 0x1;
                pcVar1 = param_1;
            }
                *param_1 = '\x13';
            param_1 = pcVar1 + 0x2;
            for (local_20 = 0x0; local_20 < iVar3; local_20 = local_20 + 0x1) {
                *param_1 = '\x0e';
                param_1 = param_1 + 0x1;
            }
        }
    }
    else {
        for (local_24 = 0x0; local_24 < 0xf; local_24 = local_24 + 0x1) {
            *param_1 = '\x1f';
            pcVar1 = param_1;
            for (local_20 = 0x0; param_1 = pcVar1 + 0x1, local_20 < 0x20; local_20 = local_20 + 0x1) {
                if (*param_2 == '\0') {
                    local_2c = param_3;
                }
                else {
                    local_2c = *param_2;
                }
                *param_1 = local_2c;
                param_2 = param_2 + 0x1;
                pcVar1 = param_1;
            }
                *param_1 = '\x13';
            param_1 = pcVar1 + 0x2;
            for (local_20 = 0x0; local_20 < iVar3; local_20 = local_20 + 0x1) {
                *param_1 = '\x0e';
                param_1 = param_1 + 0x1;
            }
        }
        for (local_24 = 0xf; local_24 < 0x1e; local_24 = local_24 + 0x1) {
            *param_1 = '\x1f';
            pcVar1 = param_1;
            for (local_20 = 0x0; param_1 = pcVar1 + 0x1, local_20 < 0x20; local_20 = local_20 + 0x1) {
                if (*param_2 == '\0') {
                    local_30 = (char)param_6;
                }
                else {
                    local_30 = *param_2;
                }
                *param_1 = local_30;
                param_2 = param_2 + 0x1;
                pcVar1 = param_1;
            }
                *param_1 = '\x13';
            param_1 = pcVar1 + 0x2;
            for (local_20 = 0x0; local_20 < iVar3; local_20 = local_20 + 0x1) {
                *param_1 = '\x0e';
                param_1 = param_1 + 0x1;
            }
        }
    }
    for (local_24 = 0x0; local_24 < 0x2; local_24 = local_24 + 0x1) {
        *param_1 = '\x1f';
        for (local_20 = 0x0; param_1 = param_1 + 0x1, local_20 < (param_4 << 0x5) / 0x64; local_20 = local_20 + 0x1) {
            *param_1 = (char)puVar2;
            param_2 = param_2 + 0x1;
        }
        for (; local_20 < 0x20; local_20 = local_20 + 0x1) {
            if (*param_2 == '\0') {
                local_34 = param_3;
            }
            else {
                local_34 = *param_2;
            }
            *param_1 = local_34;
            param_1 = param_1 + 0x1;
            param_2 = param_2 + 0x1;
        }
            *param_1 = '\x13';
        for (local_20 = 0x0; param_1 = param_1 + 0x1, local_20 < iVar3; local_20 = local_20 + 0x1) {
            *param_1 = '\x0e';
        }
    }
    for (local_24 = 0x0; local_24 < 0x22; local_24 = local_24 + 0x1) {
        *param_1 = '\x13';
        param_1 = param_1 + 0x1;
    }
    for (local_24 = 0x0; local_24 < iVar3; local_24 = local_24 + 0x1) {
        *param_1 = '\x0e';
        param_1 = param_1 + 0x1;
    }
    return;
}



fn FUN_00490a17(param_1: u32,param_2: u32,param_3: i32)

{
    let cVar1: u8;
    let mut uVar2: u32;
    let mut pcVar3: String;
    let local_40: u8 [0x10];
    let local_30: *mut u32;
    let local_2c: *mut i32; [0x7];

    if (param_3 != 0x0) {
        local_30 = FUN_00498ba4(local_2c,param_2,0x22,0x22);
        FUN_0049c2e0(local_40,&DAT_004c285b);
        uVar2 = 0xffffffff;
        pcVar3 = local_40;
        loop {
            if (uVar2 == 0x0) break;
            uVar2 = uVar2 - 0x1;
            cVar1 = *pcVar3;
            pcVar3 = pcVar3 + 0x1;
        } while (cVar1 != '\0');
        FUN_00497567(0x2,0x15,local_40,~uVar2 - 0x1,0x272727,0xe0e0e,0x272727,DAT_004d6a6c,0x0);
        FUN_00498cf4(local_2c);
    }
    return;
}



fn FUN_00490aa9(param_1: i32,param_2: i32,param_3: i32) -> i32

{
let mut iVar1: i32;
let mut local_14: i32;

for (local_14 = 0x0; local_14 < 0x3; local_14 = local_14 + 0x1) {
iVar1 = FUN_0046295d((&DAT_0058aca8 + param_1 * 0xda),local_14);
if ((iVar1 != 0x320) && (iVar1 != 0x0)) {
*(iVar1 * 0x4 + param_2) = 0x1;
param_3 = FUN_00490aa9(iVar1,param_2,param_3 + 0x1);
}
}
return param_3;
}



fn FUN_00490b38()

{
    let bVar1: u8;
    let cVar2: u8;
    let mut pcVar3: String;
    let mut uVar4: u32;
    let mut iVar5: i32;
    let mut pCVar6: String;;
    let mut extraout_ECX: u32;
    let mut extraout_ECX_00: u32;
    let mut extraout_ECX_01: u32;
    let mut extraout_EDX: u32;
    let mut extraout_EDX_00: u32;
    let mut extraout_EDX_01: u32;
    let puVar7: *mut u32;
    let mut pcVar8: String;
    let puVar9: *mut u32;
    ulonglong uVar10;
    let mut local_3ec: i32;
    let local_3e4: u8 [0x200];
    let mut local_1e4: u32;
    let local_1c4: u8 [0x10];
    let local_1b4: u8 [0x11];
    let mut local_1a3: u32;
    u32 local_b8 [0x20];
    let local_38: u8;
    byte *local_28;
    byte **local_24;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    local_28 = DAT_005b9160;
    local_20 = 0x0;
    local_24 = (byte **)FUN_0049c4bd(s_dat_unit_dat_004c2861,&DAT_004c285e);
    if (local_24 == (byte **)0x0) {
    pcVar3 = FUN_00499050(DAT_0059679c,0x7394);
    pop_err_msg_box_and_exit_004a02f5(pcVar3);
}
    local_1c = 0x0;
    local_18 = 0x0;
    local_14 = 0x0;
    loop {
    pcVar3 = FUN_004a2f60(local_28,0x200,local_24);
    if (pcVar3 == 0x0) {
        FUN_0049ca40(local_24);
        return;
    }
    local_20 = local_20 + 0x1;
    local_28 = FUN_00493dfe(local_28);
    bVar1 = *local_28;
    if (bVar1 < 0x2f) {
        if (bVar1 != 0xa)^ // goto LAB_00490c21;
    }
    else {
        if (0x2f < bVar1) {
            if (bVar1 < 0x7b) {
                LAB_00490c21:
                if (local_1c == 0x1) {
                    if (local_14 == 0x0) {
                        for (local_3ec = 0x0; local_3ec < 0x6; local_3ec = local_3ec + 0x1) {
                            *(&DAT_00582938 + local_3ec * 0x4 + local_18 * 0x18) = 0x0;
                        }
                    }
                    FUN_004a0430(&local_1e4,0x0,0x129);
                    local_28 = FUN_00493d9c(local_28,local_b8);
                    local_28 = FUN_00493d9c(local_28,local_3e4);
                    uVar10 = FUN_004a2ae0(extraout_ECX,extraout_EDX,local_b8,&DAT_004c287b);
                    if (uVar10 == 0x0) {
                        pcVar3 = local_3e4;
                        puVar7 = &local_1e4;
                        loop {
                            cVar2 = *pcVar3;
                            puVar7 = cVar2;
                            if (cVar2 == '\0') break;
                            cVar2 = pcVar3[0x1];
                            pcVar3 = pcVar3 + 0x2;
                            (puVar7 + 0x1) = cVar2;
                            puVar7 = (puVar7 + 0x2);
                        } while (cVar2 != '\0');
                        local_28 = FUN_00493d9c(local_28,local_b8);
                        FUN_004aa9bc(local_b8,&DAT_004c2880);
                        local_28 = FUN_00493d9c(local_28,local_b8);
                        local_28 = FUN_00493d9c(local_28,local_3e4);
                        uVar10 = FUN_004a2ae0(extraout_ECX_00,extraout_EDX_00,local_b8,s_abbrev_004c2883);
                        if (uVar10 == 0x0) {
                            pcVar3 = local_3e4;
                            pcVar8 = local_1c4;
                            loop {
                                cVar2 = *pcVar3;
                                *pcVar8 = cVar2;
                                if (cVar2 == '\0') break;
                                cVar2 = pcVar3[0x1];
                                pcVar3 = pcVar3 + 0x2;
                                pcVar8[0x1] = cVar2;
                                pcVar8 = pcVar8 + 0x2;
                            } while (cVar2 != '\0');
                            local_28 = FUN_00493d9c(local_28,local_b8);
                            local_28 = FUN_00493d9c(local_28,local_3e4);
                            uVar10 = FUN_004a2ae0(extraout_ECX_01,extraout_EDX_01,local_b8,s_stats_004c288a);
                            if (uVar10 == 0x0) {
                                FUN_004aa9bc(local_3e4,s__s__d__d__d__d__d__d__d__d__d__d_004c2890);
                                uVar4 = FUN_004a11c0(local_38);
                                if (uVar4 < 0x4a) {
                                    if (uVar4 < 0x43) {
                                        if (uVar4 == 0x41) {
                                            local_1a3 = 0x3;
                                        }
                                    }
                                    else {
                                        if (uVar4 < 0x44) {
                                            local_1a3 = 0x8;
                                        }
                                        else {
                                            if (0x45 < uVar4) {
                                                if (uVar4 < 0x47) {
                                                    local_1a3 = 0x0;
                                                }
                                                else {
                                                    if (uVar4 == 0x48) {
                                                        local_1a3 = 0x6;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                else {
                                    if (uVar4 < 0x4b) {
                                        local_1a3 = 0x7;
                                    }
                                    else {
                                        if (uVar4 < 0x53) {
                                            if (0x4b < uVar4) {
                                                if (uVar4 < 0x4d) {
                                                    local_1a3 = 0x9;
                                                }
                                                else {
                                                    if (uVar4 == 0x4e) {
                                                        local_1a3 = 0x4;
                                                    }
                                                }
                                            }
                                        }
                                        else {
                                            if (uVar4 < 0x54) {
                                                local_1a3 = 0x5;
                                            }
                                            else {
                                                if (uVar4 < 0x55) {
                                                    local_1a3 = 0x2;
                                                }
                                                else {
                                                    if (uVar4 == 0x57) {
                                                        local_1a3 = 0x1;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                local_28 = FUN_00493d9c(local_28,local_b8);
                                local_28 = FUN_00493d9c(local_28,local_3e4);
                                iVar5 = FUN_004a2f10(local_b8,&DAT_004c293b);
                                if (iVar5 == 0x0) {
                                    pcVar3 = local_3e4;
                                    pcVar8 = local_1b4;
                                    loop {
                                        cVar2 = *pcVar3;
                                        *pcVar8 = cVar2;
                                        if (cVar2 == '\0') break;
                                        cVar2 = pcVar3[0x1];
                                        pcVar3 = pcVar3 + 0x2;
                                        pcVar8[0x1] = cVar2;
                                        pcVar8 = pcVar8 + 0x2;
                                    } while (cVar2 != '\0');
                                }
                                else {
                                    local_1b4[0] = '\0';
                                }
                                pCVar6 = FUN_004a2831(0x129);
                                *(&DAT_00582938 + local_18 * 0x18 + local_14 * 0x4) = pCVar6;
                                puVar7 = &local_1e4;
                                puVar9 = (&DAT_00582938 + local_18 * 0x18 + local_14 * 0x4);
                                for (iVar5 = 0x4a; iVar5 != 0x0; iVar5 = iVar5 + -0x1) {
                                    *puVar9 = *puVar7;
                                    puVar7 = puVar7 + 0x1;
                                    puVar9 = puVar9 + 0x1;
                                }
                                puVar9 = puVar7;
                                local_14 = local_14 + 0x1;
                            }
                        }
                    }
                }
            }
            else {
                if (bVar1 < 0x7c) {
                    local_1c = 0x1;
                }
                else {
                    if (bVar1 != 0x7d)^ // goto LAB_00490c21;
                    local_18 = local_18 + 0x1;
                    local_14 = 0x0;
                    local_1c = 0x0;
                }
            }
        }
    }
    local_28 = DAT_005b9160;
} while( true );
}



fn FUN_0049122c()

{
    let bVar1: u8;
    let cVar2: u8;
    let mut pcVar3: String;
    let mut iVar4: i32;
    let mut extraout_ECX: u32;
    let mut uVar5: u32;
    let mut extraout_ECX_00: u32;
    let mut extraout_EDX: u32;
    let mut extraout_EDX_00: u32;
    let mut pcVar6: String;
    ulonglong uVar7;
    let mut local_2ac: u32;
    let local_2a4: u8 [0x200];
    u32 local_a4 [0x20];
    byte *local_24;
    byte **local_20;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    local_24 = DAT_005b9160;
    local_1c = 0x0;
    local_20 = (byte **)FUN_0049c4bd(s_dat_strbuild_dat_004c2942,&DAT_004c293f);
    if (local_20 == (byte **)0x0) {
    pcVar3 = FUN_00499050(DAT_0059679c,0x7394);
    pop_err_msg_box_and_exit_004a02f5(pcVar3);
}
    local_18 = 0x0;
    local_14 = 0x0;
    LAB_00491294:
        pcVar3 = FUN_004a2f60(local_24,0x200,local_20);
    if (pcVar3 == 0x0) {
        FUN_0049ca40(local_20);
        return;
    }
    local_1c = local_1c + 0x1;
    local_24 = FUN_00493dfe(local_24);
    bVar1 = *local_24;
    if (bVar1 < 0x2f) {
        if (bVar1 != 0xa)^ // goto LAB_00491301;
    }
    else {
        if (0x2f < bVar1) {
            if (bVar1 < 0x7b) {
                LAB_00491301:
                if (local_18 == 0x1) {
                    local_24 = FUN_00493d9c(local_24,local_a4);
                    local_24 = FUN_00493d9c(local_24,local_2a4);
                    uVar7 = FUN_004a2ae0(extraout_ECX,extraout_EDX,local_a4,&DAT_004c2964);
                    if (uVar7 == 0x0) {
                        pcVar3 = local_2a4;
                        pcVar6 = &DAT_005831d8 + local_14 * 0x50;
                        loop {
                            cVar2 = *pcVar3;
                            *pcVar6 = cVar2;
                            if (cVar2 == '\0') break;
                            cVar2 = pcVar3[0x1];
                            pcVar3 = pcVar3 + 0x2;
                            pcVar6[0x1] = cVar2;
                            pcVar6 = pcVar6 + 0x2;
                        } while (cVar2 != '\0');
                        pcVar6 = local_2a4;
                        pcVar3 = &DAT_005831e8 + local_14 * 0x50;
                        loop {
                            cVar2 = *pcVar6;
                            *pcVar3 = cVar2;
                            if (cVar2 == '\0') break;
                            cVar2 = pcVar6[0x1];
                            pcVar6 = pcVar6 + 0x2;
                            pcVar3[0x1] = cVar2;
                            pcVar3 = pcVar3 + 0x2;
                        } while (cVar2 != '\0');
                        local_2ac = 0x1;
                        loop {
                            uVar5 = 0xffffffff;
                            pcVar3 = local_2a4;
                            loop {
                                if (uVar5 == 0x0) break;
                                uVar5 = uVar5 - 0x1;
                                cVar2 = *pcVar3;
                                pcVar3 = pcVar3 + 0x1;
                            } while (cVar2 != '\0');
                            if (~uVar5 - 0x1 <= local_2ac)^ // goto LAB_00491435;
                            if ((&DAT_005831e8)[local_14 * 0x50 + local_2ac] == ' ') {
                                local_2ac = local_2ac + 0x1;
                            }
                            else {
                                iVar4 = FUN_004aa9f0(*(local_14 * 0x50 + local_2ac + 0x5831e5) >> 0x18);
                                (&DAT_005831e8)[local_14 * 0x50 + local_2ac] = (char)iVar4;
                            }
                            local_2ac = local_2ac + 0x1;
                        } while( true );
                    }
                }
            }
            else {
                if (bVar1 < 0x7c) {
                    local_18 = 0x1;
                }
                else {
                    if (bVar1 != 0x7d)^ // goto LAB_00491301;
                    local_18 = 0x0;
                }
            }
        }
    }^
    // goto LAB_004915b1;
    LAB_00491435:
        local_24 = FUN_00493d9c(local_24,local_a4);
    local_24 = FUN_00493d9c(local_24,local_2a4);
    uVar7 = FUN_004a2ae0(extraout_ECX_00,extraout_EDX_00,local_a4,s_stats_004c2969);
    if (uVar7 == 0x0) {
        FUN_004aa9bc(local_2a4,s__d__d__d__d__d__d__d__d__d__d__d_004c296f);
        local_14 = local_14 + 0x1;
    }
    LAB_004915b1:
        local_24 = DAT_005b9160;^
    // goto LAB_00491294;
}



fn FUN_004915d2() -> u32

{
    let bVar1: u8;
    let mut pcVar2: String;
    let mut uVar3: u32;
    let mut extraout_ECX: u32;
    let mut extraout_ECX_00: u32;
    let mut extraout_EDX: u32;
    let mut extraout_EDX_00: u32;
    ulonglong uVar4;
    let mut iVar5: i32;
    let mut local_480: u32;
    let mut local_47c: u32;
    let mut local_478: u32;
    let local_470: u8 [0x200];
    i32 local_270 [0x72];
    u32 local_a8 [0x20];
    byte *local_28;
    byte **local_24;
    let mut local_20: u32;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    local_28 = DAT_005b9160;
    local_14 = 0x0;
    local_20 = 0x0;
    local_24 = (byte **)FUN_0049c4bd(s_dat_tech_dat_004c2996,&DAT_004c2993);
    if (local_24 == (byte **)0x0) {
    pcVar2 = FUN_00499050(DAT_0059679c,0x7394);
    pop_err_msg_box_and_exit_004a02f5(pcVar2);
}
    loop {
    pcVar2 = FUN_004a2f60(local_28,0x200,local_24);
    if (pcVar2 == 0x0) {
        uVar3 = FUN_0049ca40(local_24);
        for (local_20 = 0x1; local_20 < 0x72; local_20 = local_20 + 0x1) {
            FUN_004a0430(local_270,0x0,0x1c8);
            local_1c = FUN_00490aa9(local_20,local_270,0x0);
            if (local_1c < 0x1) {
                *(&DAT_0058ad7e + local_20 * 0xda) = 0x0;
            }
            else {
                local_18 = 0x0;
                for (iVar5 = 0x1; iVar5 < 0x72; iVar5 = iVar5 + 0x1) {
                    if (local_270[iVar5] != 0x0) {
                        local_18 = local_18 + *(&DAT_0058ad6a + iVar5 * 0xda);
                    }
                }
                    *(&DAT_0058ad7e + local_20 * 0xda) = local_18 / 0x14;
            }
            uVar3 = local_20;
        }
        return uVar3;
    }
    local_28 = FUN_00493dfe(local_28);
    bVar1 = *local_28;
    if (bVar1 < 0x7b) {
        if (bVar1 != 0x2f) {
            LAB_0049168c:
            if (local_14 == 0x1) {
                FUN_004a0430(&DAT_0058aca8 + local_20 * 0xda,0x0,0xda);
                local_28 = FUN_00493d9c(local_28,local_a8);
                local_28 = FUN_00493d9c(local_28,local_470);
                uVar4 = FUN_004a2ae0(extraout_ECX,extraout_EDX,local_a8,&DAT_004c29b0);
                if (uVar4 == 0x0) {
                    FUN_004a9a00(&DAT_0058aca8 + local_20 * 0xda,local_470,0x1f);
                    (&DAT_0058acc7)[local_20 * 0xda] = 0x0;
                    local_28 = FUN_00493d9c(local_28,local_a8);
                    local_28 = FUN_00493d9c(local_28,local_470);
                    FUN_004aa9bc(local_470,s__d__d__d__d__d__d__d_004c29b5);
                    *(&DAT_0058ad5e + local_20 * 0xda) = local_480;
                    *(local_20 * 0xda + 0x58ad62) = local_47c;
                    *(local_20 * 0xda + 0x58ad66) = local_478;
                    local_28 = FUN_00493d9c(local_28,local_a8);
                    local_28 = FUN_00493d9c(local_28,local_470);
                    uVar4 = FUN_004a2ae0(extraout_ECX_00,extraout_EDX_00,local_a8,s_extra_004c29ca);
                    if (uVar4 == 0x0) {
                        FUN_004a9a00((local_20 * 0xda + 0x58acc8),local_470,0x95);
                        (&DAT_0058ad5d)[local_20 * 0xda] = 0x0;
                        local_20 = local_20 + 0x1;
                    }
                }
            }
        }
    }
    else {
        if (bVar1 < 0x7c) {
            local_14 = 0x1;
        }
        else {
            if (bVar1 != 0x7d)^ // goto LAB_0049168c;
            local_14 = 0x0;
        }
    }
    local_28 = DAT_005b9160;
} while( true );
}



fn FUN_00491a89()

{
    byte **ppbVar1;
    let mut pcVar2: String;
    let ppcVar3: *mut *mut char;
    let mut iVar4: i32;
    let mut local_18: String;
    let mut local_14: i32;

    local_18 = DAT_005b9160;
    ppbVar1 = (byte **)FUN_0049c4bd(s_dat_unitrulz_dat_004c29d3,&DAT_004c29d0);
    if (ppbVar1 == (byte **)0x0) {
    pcVar2 = FUN_00499050(DAT_0059679c,0x7394);
    pop_err_msg_box_and_exit_004a02f5(pcVar2);
}
    ppcVar3 = FUN_0049c4bd(s_bin_efsunit_bin_004c29f8,&DAT_004c29f5);
    if (ppcVar3 == 0x0) {
        pcVar2 = FUN_00499050(DAT_0059679c,0x7394);
        pop_err_msg_box_and_exit_004a02f5(pcVar2);
    }
    local_14 = 0x0;
    while (local_14 < 0x5c) {
        FUN_004a2f60(local_18,0x200,ppbVar1);
        if (*local_18 != '/') {
            FUN_004aa9bc(local_18,s__d__d__d__d__d__d_004c2a18);
            if (*(&DAT_004d6058 + local_14 * 0x1c) == 0x0) {
                iVar4 = FUN_0049c2c9(0x400);
                *(&DAT_004d6058 + local_14 * 0x1c) = iVar4;
            }
            FUN_004a7970((&DAT_004d6058 + local_14 * 0x1c),0x400,0x1,ppcVar3);
            local_14 = local_14 + 0x1;
        }
        local_18 = DAT_005b9160;
    }
    FUN_0049ca40(ppbVar1);
    FUN_0049ca40(ppcVar3);
    return;
}



fn FUN_00491c54()

{
    let bVar1: u8;
    let mut pcVar2: String;
    float10 fVar3;
    let mut local_2d4: i32;
    let local_2cc: u8 [0x200];
    let local_cc: u8 [0xa8];
    byte **local_24;
    byte *local_20;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    local_20 = DAT_005b9160;
    local_24 = (byte **)FUN_0049c4bd(s_dat_terrcost_dat_004c2a2d,&DAT_004c2a2a);
    if (local_24 == (byte **)0x0) {
    pcVar2 = FUN_00499050(DAT_0059679c,0x7394);
    pop_err_msg_box_and_exit_004a02f5(pcVar2);
}
    local_1c = 0x0;
    local_18 = 0x0;
    local_14 = 0x0;
    loop {
    pcVar2 = FUN_004a2f60(local_20,0x200,local_24);
    if (pcVar2 == 0x0) {
        FUN_0049ca40(local_24);
        return;
    }
    local_20 = FUN_00493dfe(local_20);
    bVar1 = *local_20;
    if (bVar1 < 0x2f) {
        if (bVar1 != 0xa)^ // goto LAB_00491d27;
    }
    else {
        if (0x2f < bVar1) {
            if (bVar1 < 0x7b) {
                LAB_00491d27:
                if (local_1c == 0x1) {
                    local_20 = FUN_00493d9c(local_20,local_cc);
                    local_20 = FUN_00493d9c(local_20,local_2cc);
                    FUN_004aa9bc(local_2cc,s__f__f__f__f__f__f__f__f__f__f_004c2a4f);
                    for (local_2d4 = 0x0; local_2d4 < 0xa; local_2d4 = local_2d4 + 0x1) {
                        fVar3 = FUN_004a9b12();
                        (&DAT_00590dc0 + local_2d4 * 0x2 + local_14 * 0x14 + local_18 * 0x64) = ROUND(fVar3)
                        ;
                    }
                    local_14 = local_14 + 0x1;
                }
            }
            else {
                if (bVar1 < 0x7c) {
                    local_14 = 0x0;
                    local_1c = 0x1;
                }
                else {
                    if (bVar1 != 0x7d)^ // goto LAB_00491d27;
                    local_1c = 0x0;
                    local_18 = local_18 + 0x1;
                }
            }
        }
    }
    local_20 = DAT_005b9160;
} while( true );
}



fn FUN_00491e92()

{
    let bVar1: u8;
    let mut pcVar2: String;
    let local_2a0: u8 [0x200];
    let local_a0: u8 [0x80];
    byte *local_20;
    byte **local_1c;
    let mut local_18: i32;
    let mut local_14: i32;

    local_20 = DAT_005b9160;
    local_1c = (byte **)FUN_0049c4bd(s_dat_agility_dat_004c2a75,&DAT_004c2a72);
    if (local_1c == (byte **)0x0) {
    pcVar2 = FUN_00499050(DAT_0059679c,0x7394);
    pop_err_msg_box_and_exit_004a02f5(pcVar2);
}
    local_18 = 0x0;
    local_14 = 0x0;
    loop {
    pcVar2 = FUN_004a2f60(local_20,0x200,local_1c);
    if (pcVar2 == 0x0) {
        FUN_0049ca40(local_1c);
        return;
    }
    local_20 = FUN_00493dfe(local_20);
    bVar1 = *local_20;
    if (bVar1 < 0x2f) {
        if (bVar1 != 0xa)^ // goto LAB_00491f4c;
    }
    else {
        if (0x2f < bVar1) {
            if (bVar1 < 0x7b) {
                LAB_00491f4c:
                if (local_18 == 0x1) {
                    local_20 = FUN_00493d9c(local_20,local_a0);
                    local_20 = FUN_00493d9c(local_20,local_2a0);
                    FUN_004aa9bc(local_2a0,s__d__d__d__d__d__d__d__d__d__d_004c2a95);
                    local_14 = local_14 + 0x1;
                }
            }
            else {
                if (bVar1 < 0x7c) {
                    local_18 = 0x1;
                }
                else {
                    if (bVar1 != 0x7d)^ // goto LAB_00491f4c;
                    local_18 = 0x0;
                }
            }
        }
    }
    local_20 = DAT_005b9160;
} while( true );
}



fn FUN_004920ad()

{
    let bVar1: u8;
    let mut pcVar2: String;
    let local_2a0: u8 [0x200];
    let local_a0: u8 [0x80];
    byte *local_20;
    byte **local_1c;
    let mut local_18: i32;
    let mut local_14: i32;

    local_20 = DAT_005b9160;
    local_1c = (byte **)FUN_0049c4bd(s_dat_tercolor_dat_004c2ab6,&DAT_004c2ab3);
    if (local_1c == (byte **)0x0) {
    pcVar2 = FUN_00499050(DAT_0059679c,0x7394);
    pop_err_msg_box_and_exit_004a02f5(pcVar2);
}
    local_18 = 0x0;
    local_14 = 0x0;
    loop {
    pcVar2 = FUN_004a2f60(local_20,0x200,local_1c);
    if (pcVar2 == 0x0) {
        FUN_0049ca40(local_1c);
        return;
    }
    local_20 = FUN_00493dfe(local_20);
    bVar1 = *local_20;
    if (bVar1 < 0x2f) {
        if (bVar1 != 0xa)^ // goto LAB_0049216c;
    }
    else {
        if (0x2f < bVar1) {
            if (bVar1 < 0x7b) {
                LAB_0049216c:
                if (local_18 == 0x1) {
                    local_20 = FUN_00493d9c(local_20,local_a0);
                    local_20 = FUN_00493d9c(local_20,local_2a0);
                    FUN_004aa9bc(local_2a0,s__d__d__d__d__d_004c2ad8);
                    local_14 = local_14 + 0x1;
                }
            }
            else {
                if (bVar1 < 0x7c) {
                    local_18 = 0x1;
                }
                else {
                    if (bVar1 != 0x7d)^ // goto LAB_0049216c;
                    local_18 = 0x0;
                }
            }
        }
    }
    local_20 = DAT_005b9160;
} while( true );
}



fn FUN_00492287()

{
    let bVar1: u8;
    let mut pcVar2: String;
    float10 fVar3;
    let mut local_2d4: i32;
    let local_2cc: u8 [0x200];
    let local_cc: u8 [0xa8];
    byte **local_24;
    byte *local_20;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    local_20 = DAT_005b9160;
    local_24 = (byte **)FUN_0049c4bd(s_dat_unitspot_dat_004c2aea,&DAT_004c2ae7);
    if (local_24 == (byte **)0x0) {
    pcVar2 = FUN_00499050(DAT_0059679c,0x7394);
    pop_err_msg_box_and_exit_004a02f5(pcVar2);
}
    local_1c = 0x0;
    local_18 = 0x0;
    local_14 = 0x0;
    loop {
    pcVar2 = FUN_004a2f60(local_20,0x200,local_24);
    if (pcVar2 == 0x0) {
        FUN_0049ca40(local_24);
        return;
    }
    local_20 = FUN_00493dfe(local_20);
    bVar1 = *local_20;
    if (bVar1 < 0x2f) {
        if (bVar1 != 0xa)^ // goto LAB_0049235a;
    }
    else {
        if (0x2f < bVar1) {
            if (bVar1 < 0x7b) {
                LAB_0049235a:
                if (local_1c == 0x1) {
                    local_20 = FUN_00493d9c(local_20,local_cc);
                    local_20 = FUN_00493d9c(local_20,local_2cc);
                    FUN_004aa9bc(local_2cc,s__f__f__f__f__f__f__f__f__f__f_004c2b0c);
                    for (local_2d4 = 0x0; local_2d4 < 0xa; local_2d4 = local_2d4 + 0x1) {
                        fVar3 = FUN_004a9b12();
                        (&DAT_00591270 + local_2d4 * 0x2 + local_14 * 0x14 + local_18 * 0x64) = ROUND(fVar3)
                        ;
                    }
                    local_14 = local_14 + 0x1;
                }
            }
            else {
                if (bVar1 < 0x7c) {
                    local_14 = 0x0;
                    local_1c = 0x1;
                }
                else {
                    if (bVar1 != 0x7d)^ // goto LAB_0049235a;
                    local_1c = 0x0;
                    local_18 = local_18 + 0x1;
                }
            }
        }
    }
    local_20 = DAT_005b9160;
} while( true );
}



fn FUN_004924c5()

{
    let bVar1: u8;
    let mut pcVar2: String;
    let local_220: u8 [0x200];
    byte **local_20;
    byte *local_1c;
    let mut local_18: i32;
    let mut local_14: i32;

    local_1c = DAT_005b9160;
    local_20 = (byte **)FUN_0049c4bd(s_dat_damage_dat_004c2b31,&DAT_004c2b2e);
    if (local_20 == (byte **)0x0) {
    pcVar2 = FUN_00499050(DAT_0059679c,0x7394);
    pop_err_msg_box_and_exit_004a02f5(pcVar2);
}
    local_18 = 0x0;
    local_14 = 0x0;
    loop {
    pcVar2 = FUN_004a2f60(local_1c,0x200,local_20);
    if (pcVar2 == 0x0) {
        FUN_0049ca40(local_20);
        return;
    }
    local_1c = FUN_00493dfe(local_1c);
    bVar1 = *local_1c;
    if (bVar1 < 0x2f) {
        if (bVar1 != 0xa)^ // goto LAB_00492584;
    }
    else {
        if (0x2f < bVar1) {
            if (bVar1 < 0x7b) {
                LAB_00492584:
                if (local_18 == 0x1) {
                    local_1c = FUN_00493d9c(local_1c,local_220);
                    FUN_004aa9bc(local_220,s__d__d__d__d__d__d__d__d__d__d__d_004c2b4f);
                    local_14 = local_14 + 0x1;
                }
            }
            else {
                if (bVar1 < 0x7c) {
                    local_18 = 0x1;
                }
                else {
                    if (bVar1 != 0x7d)^ // goto LAB_00492584;
                    local_18 = 0x0;
                }
            }
        }
    }
    local_1c = DAT_005b9160;
} while( true );
}



fn FUN_004926f4()

{
    let bVar1: u8;
    let mut pcVar2: String;
    let local_2a0: u8 [0x200];
    let local_a0: u8 [0x80];
    byte *local_20;
    byte **local_1c;
    let mut local_18: i32;
    let mut local_14: i32;

    local_20 = DAT_005b9160;
    local_1c = (byte **)FUN_0049c4bd(s_dat_target_dat_004c2b7c,&DAT_004c2b79);
    if (local_1c == (byte **)0x0) {
    pcVar2 = FUN_00499050(DAT_0059679c,0x7394);
    pop_err_msg_box_and_exit_004a02f5(pcVar2);
}
    local_18 = 0x0;
    local_14 = 0x0;
    loop {
    pcVar2 = FUN_004a2f60(local_20,0x200,local_1c);
    if (pcVar2 == 0x0) {
        FUN_0049ca40(local_1c);
        return;
    }
    local_20 = FUN_00493dfe(local_20);
    bVar1 = *local_20;
    if (bVar1 < 0x2f) {
        if (bVar1 != 0xa)^ // goto LAB_004927b3;
    }
    else {
        if (0x2f < bVar1) {
            if (bVar1 < 0x7b) {
                LAB_004927b3:
                if (local_18 == 0x1) {
                    local_20 = FUN_00493d9c(local_20,local_a0);
                    local_20 = FUN_00493d9c(local_20,local_2a0);
                    FUN_004aa9bc(local_2a0,s__d__d__d__d__d__d__d__d__d_004c2b9a);
                    local_14 = local_14 + 0x1;
                }
            }
            else {
                if (bVar1 < 0x7c) {
                    local_18 = 0x1;
                }
                else {
                    if (bVar1 != 0x7d)^ // goto LAB_004927b3;
                    local_18 = 0x0;
                }
            }
        }
    }
    local_20 = DAT_005b9160;
} while( true );
}



fn FUN_0049290c(byte *param_1)

{
    let bVar1: u8;
    let mut pcVar2: String;
    let mut local_100: i32;
    let local_f8: u8 [0x80];
    let local_78: u8 [0x40];
    byte **local_38;
    byte *local_34;
    PCHAR local_30;
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_24: u32;
    let mut local_20: i32;
    let mut local_1c: u32;
    let mut local_18: i32;
    let mut local_14: i32;

    local_34 = DAT_005b9160;
    local_30 = FUN_0049c2c9(0x104);
    local_2c = FUN_0049488e(param_1);
    if (local_2c != -0x1) {
        FUN_0049c2e0(local_30,s_dat__s_dat_004c2bb5);
        local_38 = (byte **)FUN_0049c4bd(local_30,&DAT_004c2bc0);
        if (local_38 == (byte **)0x0) {
            pcVar2 = FUN_00499050(DAT_0059679c,0x7394);
            pop_err_msg_box_and_exit_004a02f5(pcVar2);
        }
        local_20 = 0x0;
        local_1c = 0x0;
        local_18 = 0x0;
        local_14 = 0x0;
        while (pcVar2 = FUN_004a2f60(local_34,0x200,local_38), pcVar2 != 0x0) {
            local_34 = FUN_00493dfe(local_34);
            bVar1 = *local_34;
            if (bVar1 < 0x2f) {
                if (bVar1 != 0xa)^ // goto LAB_00492a22;
            }
            else {
                if (0x2f < bVar1) {
                    if (bVar1 < 0x7b) {
                        LAB_00492a22:
                        if (local_20 == 0x1) {
                            if (local_2c == -0x1) {
                                local_20 = 0x0;
                            }
                            local_34 = FUN_00493d9c(local_34,local_78);
                            local_100 = 0x0;
                            local_34 = FUN_00493d9c(local_34,local_78);
                            while (local_78[0] != 0x40) {
                                local_28 = FUN_004948ea(local_78);
                                if (local_28 == -0x1) {
                                    local_20 = 0x0;
                                    break;
                                }
                                local_34 = FUN_00493d9c(local_34,local_f8);
                                FUN_004aa9bc(local_f8,&DAT_004c2bc3);
                                *(&DAT_00583c3c + local_100 * 0x4 + local_18 * 0x50 + local_2c * 0x384 + local_14 * 0x10)
                                    = local_24;
                                *(&DAT_00583c3c + local_100 * 0x4 + local_2c * 0x384 + local_18 * 0x50 + local_14 * 0x10) =
                                    *(&DAT_00583c3c + local_100 * 0x4 + local_2c * 0x384 + local_18 * 0x50 + local_14 * 0x10) |
                                        local_28 << 0x10;
                                *(&DAT_00583c28 + local_2c * 0x384) = 0xffffffff;
                                local_34 = FUN_00493d9c(local_34,local_78);
                                local_100 = local_100 + 0x1;
                            }
                            local_14 = local_14 + 0x1;
                        }
                    }
                    else {
                        if (bVar1 < 0x7c) {
                            local_14 = 0x0;
                            local_20 = 0x1;
                        }
                        else {
                            if (bVar1 != 0x7d)^ // goto LAB_00492a22;
                            local_20 = 0x0;
                            local_18 = local_18 + 0x1;
                        }
                    }
                }
            }
            local_34 = DAT_005b9160;
        }
        FUN_0049af50(local_30);
        FUN_0049ca40(local_38);
    }
    return;
}



fn FUN_00492be3()

{
    let bVar1: u8;
    let mut pcVar2: String;
    let mut iVar3: i32;
    let local_134: u8 [0x80];
    let local_b4: u8 [0x40];
    let local_74: u8 [0x40];
    byte *local_34;
    byte **local_30;
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_24: u32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: u32;

    local_34 = DAT_005b9160;
    local_2c = -0x2;
    local_30 = (byte **)FUN_0049c4bd(s_dat_prod_dat_004c2bc9,&DAT_004c2bc6);
    if (local_30 == (byte **)0x0) {
    pcVar2 = FUN_00499050(DAT_0059679c,0x7394);
    pop_err_msg_box_and_exit_004a02f5(pcVar2);
}
    for (local_1c = 0x0; local_1c < 0x20; local_1c = local_1c + 0x1) {
        *(&DAT_00583c28 + local_1c * 0x384) = 0xffffffff;
        *(&DAT_00583c2c + local_1c * 0x384) = 0xffffffff;
        *(&DAT_00583c30 + local_1c * 0x384) = 0xffffffff;
        *(&DAT_00583c34 + local_1c * 0x384) = 0xffffffff;
        *(&DAT_00583c38 + local_1c * 0x384) = 0xffffffff;
        FUN_004a0430(&DAT_00583c3c + local_1c * 0x384,0xff,0x370);
    }
    local_18 = 0x0;
    local_14 = 0x0;
    loop {
    pcVar2 = FUN_004a2f60(local_34,0x200,local_30);
    if (pcVar2 == 0x0) {
        FUN_0049290c(&DAT_004c2bf8);
        FUN_0049290c(&DAT_004c2bfd);
        FUN_0049290c(&DAT_004c2c02);
        FUN_0049290c(s_arborium_004c2c07);
        FUN_0049ca40(local_30);
        return;
    }
    local_34 = FUN_00493dfe(local_34);
    bVar1 = *local_34;
    if (bVar1 < 0x2f) {
        if (bVar1 != 0xa)^ // goto LAB_00492d40;
    }
    else {
        if (0x2f < bVar1) {
            if (bVar1 < 0x7b) {
                LAB_00492d40:
                if (local_18 == 0x1) {
                    if (local_2c == -0x1) {
                        local_18 = 0x0;
                    }
                    local_34 = FUN_00493d9c(local_34,local_74);
                    local_34 = FUN_00493d9c(local_34,local_134);
                    iVar3 = FUN_004a2f10(local_74,&DAT_004c2be3);
                    if (iVar3 == 0x0) {
                        local_2c = FUN_0049488e(local_134);
                    }
                    else {
                        iVar3 = FUN_004a2f10(local_74,&DAT_004c2be8);
                        if (iVar3 == 0x0) {
                            local_34 = FUN_00493d9c(local_34,local_b4);
                            local_28 = FUN_004948ea(local_134);
                            if (local_28 == -0x1) {
                                local_18 = 0x0;
                            }
                            else {
                                FUN_004aa9bc(local_b4,&DAT_004c2bed);
                                *(&DAT_00583c2c + local_20 * 0x4 + local_2c * 0x384) = local_24;
                                *(&DAT_00583c2c + local_20 * 0x4 + local_2c * 0x384) =
                                    *(&DAT_00583c2c + local_20 * 0x4 + local_2c * 0x384) | local_28 << 0x10;
                                local_20 = local_20 + 0x1;
                            }
                        }
                        else {
                            iVar3 = FUN_004a2f10(local_74,&DAT_004c2bf0);
                            if (iVar3 == 0x0) {
                                local_34 = FUN_00493d9c(local_34,local_b4);
                                local_28 = FUN_004948ea(local_134);
                                if (local_28 == -0x1) {
                                    local_18 = 0x0;
                                }
                                else {
                                    FUN_004aa9bc(local_b4,&DAT_004c2bf5);
                                    *(&DAT_00583c28 + local_2c * 0x384) = local_24;
                                    *(&DAT_00583c28 + local_2c * 0x384) =
                                        *(&DAT_00583c28 + local_2c * 0x384) | local_28 << 0x10;
                                }
                            }
                            else {
                                local_18 = 0x0;
                            }
                        }
                    }
                }
            }
            else {
                if (bVar1 < 0x7c) {
                    local_18 = 0x1;
                    local_20 = 0x0;
                }
                else {
                    if (bVar1 != 0x7d)^ // goto LAB_00492d40;
                    local_18 = 0x0;
                }
            }
        }
    }
    local_34 = DAT_005b9160;
} while( true );
}



fn FUN_00492fad()

{
    let bVar1: u8;
    let mut pcVar2: String;
    let local_2a0: u8 [0x200];
    let local_a0: u8 [0x80];
    byte *local_20;
    byte **local_1c;
    let mut local_18: i32;
    let mut local_14: i32;

    local_20 = DAT_005b9160;
    local_1c = (byte **)FUN_0049c4bd(s_dat_stock_dat_004c2c13,&DAT_004c2c10);
    if (local_1c == (byte **)0x0) {
    pcVar2 = FUN_00499050(DAT_0059679c,0x7394);
    pop_err_msg_box_and_exit_004a02f5(pcVar2);
}
    local_18 = 0x0;
    local_14 = 0x0;
    loop {
    pcVar2 = FUN_004a2f60(local_20,0x200,local_1c);
    if (pcVar2 == 0x0) {
        FUN_0049ca40(local_1c);
        return;
    }
    local_20 = FUN_00493dfe(local_20);
    bVar1 = *local_20;
    if (bVar1 < 0x2f) {
        if (bVar1 != 0xa)^ // goto LAB_0049306c;
    }
    else {
        if (0x2f < bVar1) {
            if (bVar1 < 0x7b) {
                LAB_0049306c:
                if (local_18 == 0x1) {
                    local_20 = FUN_00493d9c(local_20,local_a0);
                    local_20 = FUN_00493d9c(local_20,local_2a0);
                    FUN_004aa9bc(local_2a0,s__d__d__d__d__d_004c2c2f);
                    local_14 = local_14 + 0x1;
                }
            }
            else {
                if (bVar1 < 0x7c) {
                    local_18 = 0x1;
                }
                else {
                    if (bVar1 != 0x7d)^ // goto LAB_0049306c;
                    local_18 = 0x0;
                }
            }
        }
    }
    local_20 = DAT_005b9160;
} while( true );
}



fn FUN_00493189()

{
    let bVar1: u8;
    let mut pcVar2: String;
    let mut uVar3: u32;
    let mut iVar4: i32;
    let mut local_2f4: i32;
    let mut local_2ec: i32;
    let local_2e8: u8 [0x200];
    let local_e8: u8 [0x80];
    let local_68: u8 [0x40];
    byte *local_28;
    let mut local_24: i32;
    let mut local_20: i32;
    byte **local_1c;
    let mut local_18: i32;
    let mut local_14: i32;

    local_28 = DAT_005b9160;
    local_20 = 0x0;
    local_1c = (byte **)FUN_0049c4bd(s_dat_traits_dat_004c2c41,&DAT_004c2c3e);
    if (local_1c == (byte **)0x0) {
    pcVar2 = FUN_00499050(DAT_0059679c,0x7394);
    pop_err_msg_box_and_exit_004a02f5(pcVar2);
}
    for (local_18 = 0x0; local_18 < 0x5; local_18 = local_18 + 0x1) {
        for (local_2ec = 0x0; local_2ec < 0x3; local_2ec = local_2ec + 0x1) {
            uVar3 = FUN_0049c2c9(0x10);
            *(&DAT_004d7ba0 + local_2ec * 0x4 + local_18 * 0xc) = uVar3;
        }
    }
    local_14 = 0x0;
    loop {
    pcVar2 = FUN_004a2f60(local_28,0x200,local_1c);
    if (pcVar2 == 0x0) {
        FUN_0049ca40(local_1c);
        return;
    }
    local_28 = FUN_00493dfe(local_28);
    bVar1 = *local_28;
    if (bVar1 < 0x2f) {
        if (bVar1 != 0xa)^ // goto LAB_004932ae;
    }
    else {
        if (0x2f < bVar1) {
            if (bVar1 < 0x7b) {
                LAB_004932ae:
                if (local_14 == 0x1) {
                    local_28 = FUN_00493d9c(local_28,local_e8);
                    local_28 = FUN_00493d9c(local_28,local_2e8);
                    iVar4 = FUN_004a2f10(local_e8,s_house_004c2c5f);
                    if (iVar4 == 0x0) {
                        local_28 = FUN_00493d9c(local_28,local_e8);
                        FUN_0049c2e0(*(PCHAR *)(&DAT_004d7ba0 + local_20 * 0xc),local_e8);
                        local_28 = FUN_00493d9c(local_28,local_e8);
                        FUN_0049c2e0(*(PCHAR *)(&DAT_004d7ba4 + local_20 * 0xc),local_e8);
                        local_28 = FUN_00493d9c(local_28,local_e8);
                        FUN_0049c2e0(*(PCHAR *)(&DAT_004d7ba8 + local_20 * 0xc),local_e8);
                    }
                    else {
                        FUN_004aa9bc(local_2e8,s__d__d__d__d__d__d__d__d__d__d__d_004c2c65);
                        for (local_2f4 = 0x0; local_2f4 < 0x10; local_2f4 = local_2f4 + 0x1) {
                            (&DAT_004d7410)[local_24 * 0x20 + local_20 * 0xa0 + local_2f4] = local_68[local_2f4 * 0x4];
                        }
                        local_28 = FUN_00493d9c(local_28,local_2e8);
                        FUN_004aa9bc(local_2e8,s__d__d__d__d__d__d__d__d__d__d__d_004c2c95);
                        for (local_2f4 = 0x0; local_2f4 < 0x10; local_2f4 = local_2f4 + 0x1) {
                            (&DAT_004d7420)[local_24 * 0x20 + local_20 * 0xa0 + local_2f4] = local_68[local_2f4 * 0x4];
                        }
                        local_24 = local_24 + 0x1;
                    }
                }
            }
            else {
                if (bVar1 < 0x7c) {
                    local_14 = 0x1;
                    local_24 = 0x0;
                }
                else {
                    if (bVar1 != 0x7d)^ // goto LAB_004932ae;
                    local_14 = 0x0;
                    local_20 = local_20 + 0x1;
                }
            }
        }
    }
    local_28 = DAT_005b9160;
} while( true );
}



fn FUN_00493573()

{
    let bVar1: u8;
    let mut pcVar2: String;
    let mut iVar3: i32;
    byte *pbVar4;
    byte *pbVar5;
    let local_228: u8 [0x200];
    let mut local_28: String;;
    byte *local_24;
    byte **local_20;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    local_28 = FUN_0049c2c9(0x200);
    local_24 = local_28;
    local_20 = (byte **)FUN_0049c4bd(s_dat_relics_dat_004c2cc8,&DAT_004c2cc5);
    if (local_20 == (byte **)0x0) {
    pcVar2 = FUN_00499050(DAT_0059679c,0x7394);
    pop_err_msg_box_and_exit_004a02f5(pcVar2);
}
    for (local_18 = 0x0; local_18 < 0xb; local_18 = local_18 + 0x1) {
        (&DAT_004d7792 + local_18 * 0x67 + 0x3) = 0xff;
    }
    local_14 = 0x0;
    loop {
    pcVar2 = FUN_004a2f60(local_24,0x200,local_20);
    if (pcVar2 == 0x0) {
        FUN_0049ca40(local_20);
        FUN_0049af50(local_28);
        return;
    }
    local_24 = FUN_00493dfe(local_24);
    bVar1 = *local_24;
    if (bVar1 < 0x2f) {
        if (bVar1 != 0xa)^ // goto LAB_00493662;
    }
    else {
        if (0x2f < bVar1) {
            if (bVar1 < 0x7b) {
                LAB_00493662:
                if ((local_14 == 0x1) && (local_1c != 0xb)) {
                    local_24 = FUN_00493d9c(local_24,local_228);
                    iVar3 = FUN_004a2f10(local_228,s_galaxy_004c2ce6);
                    if (iVar3 == 0x0) {
                        pbVar5 = (&DAT_004d7792 + local_1c * 0x67 + 0x2);
                        *pbVar5 = *pbVar5 | 0x1;
                    }
                    else {
                        iVar3 = FUN_004a2f10(local_228,s_planet_004c2ced);
                        if (iVar3 == 0x0) {
                            pbVar5 = (&DAT_004d7792 + local_1c * 0x67 + 0x2);
                            *pbVar5 = *pbVar5 | 0x2;
                        }
                        else {
                            iVar3 = FUN_004a2f10(local_228,s_stack_004c2cf4);
                            if (iVar3 == 0x0) {
                                pbVar5 = (&DAT_004d7792 + local_1c * 0x67 + 0x2);
                                *pbVar5 = *pbVar5 | 0x4;
                            }
                        }
                    }
                    local_24 = FUN_00493d9c(local_24,local_228);
                    FUN_004aa9bc(local_228,&DAT_004c2cfa);
                    local_24 = FUN_00493d9c(local_24,local_228);
                    pbVar4 = local_228;
                    pbVar5 = &DAT_004d7734 + local_1c * 0x67;
                    loop {
                        bVar1 = *pbVar4;
                        *pbVar5 = bVar1;
                        if (bVar1 == 0x0) break;
                        bVar1 = pbVar4[0x1];
                        pbVar4 = pbVar4 + 0x2;
                        pbVar5[0x1] = bVar1;
                        pbVar5 = pbVar5 + 0x2;
                    } while (bVar1 != 0x0);
                    local_24 = FUN_00493d9c(local_24,local_228);
                    pbVar4 = local_228;
                    pbVar5 = &DAT_004d7754 + local_1c * 0x67;
                    loop {
                        bVar1 = *pbVar4;
                        *pbVar5 = bVar1;
                        if (bVar1 == 0x0) break;
                        bVar1 = pbVar4[0x1];
                        pbVar4 = pbVar4 + 0x2;
                        pbVar5[0x1] = bVar1;
                        pbVar5 = pbVar5 + 0x2;
                    } while (bVar1 != 0x0);
                    local_1c = local_1c + 0x1;
                }
            }
            else {
                if (bVar1 < 0x7c) {
                    local_14 = 0x1;
                    local_1c = 0x0;
                }
                else {
                    if (bVar1 != 0x7d)^ // goto LAB_00493662;
                    local_14 = 0x0;
                }
            }
        }
    }
    local_24 = local_28;
} while( true );
}



fn FUN_00493850()

{
    let bVar1: u8;
    let mut pcVar2: String;
    let mut iVar3: i32;
    let local_260: u8 [0x200];
    let local_60: u8 [0x40];
    byte *local_20;
    byte **local_1c;
    let mut local_18: i32;
    let mut local_14: i32;

    local_20 = DAT_005b9160;
    local_1c = (byte **)FUN_0049c4bd(s_dat_res_dat_004c2d00,&DAT_004c2cfd);
    if (local_1c == (byte **)0x0) {
    pcVar2 = FUN_00499050(DAT_0059679c,0x7394);
    pop_err_msg_box_and_exit_004a02f5(pcVar2);
}
    local_14 = 0x0;
    loop {
    pcVar2 = FUN_004a2f60(local_20,0x200,local_1c);
    if (pcVar2 == 0x0) {
        FUN_0049ca40(local_1c);
        return;
    }
    local_20 = FUN_00493dfe(local_20);
    bVar1 = *local_20;
    if (bVar1 < 0x2f) {
        if (bVar1 != 0xa)^ // goto LAB_0049390f;
    }
    else {
        if (0x2f < bVar1) {
            if (bVar1 < 0x7b) {
                LAB_0049390f:
                if (local_14 == 0x1) {
                    local_20 = FUN_00493d9c(local_20,local_60);
                    local_20 = FUN_00493d9c(local_20,local_260);
                    iVar3 = FUN_004a2f10(local_60,&DAT_004c2d18);
                    if (iVar3 == 0x0) {
                        FUN_004a9a00((local_18 * 0xa8 + 0x4d6a78),local_260,0xf);
                        (&DAT_004d6a87)[local_18 * 0xa8] = 0x0;
                    }
                    local_20 = FUN_00493d9c(local_20,local_60);
                    local_20 = FUN_00493d9c(local_20,local_260);
                    iVar3 = FUN_004a2f10(local_60,&DAT_004c2d1d);
                    if (iVar3 == 0x0) {
                        FUN_004a9a00((local_18 * 0xa8 + 0x4d6a88),local_260,0xf);
                        (&DAT_004d6a97)[local_18 * 0xa8] = 0x0;
                    }
                    local_20 = FUN_00493d9c(local_20,local_60);
                    local_20 = FUN_00493d9c(local_20,local_260);
                    iVar3 = FUN_004a2f10(local_60,s_price_004c2d22);
                    if (iVar3 == 0x0) {
                        FUN_004aa9bc(local_260,&DAT_004c2d28);
                    }
                    local_20 = FUN_00493d9c(local_20,local_60);
                    local_20 = FUN_00493d9c(local_20,local_260);
                    iVar3 = FUN_004a2f10(local_60,&DAT_004c2d2b);
                    if (iVar3 == 0x0) {
                        FUN_004a9a00((local_18 * 0xa8 + 0x4d6a98),local_260,0x7f);
                        (&DAT_004d6b17)[local_18 * 0xa8] = 0x0;
                    }
                    local_18 = local_18 + 0x1;
                }
            }
            else {
                if (bVar1 < 0x7c) {
                    local_14 = 0x1;
                    local_18 = 0x0;
                }
                else {
                    if (bVar1 != 0x7d)^ // goto LAB_0049390f;
                    local_14 = 0x0;
                }
            }
        }
    }
    local_20 = DAT_005b9160;
} while( true );
}



fn FUN_00493b56()

{
    let puVar1: *mut u32;
    let mut pcVar2: String;
    let mut iVar3: i32;
    let mut local_18: i32;

    DAT_005b9160 = FUN_0049c2c9(0x200);
    if (DAT_005b9160 == 0x0) {
        pop_err_msg_box_and_exit_004a02f5(s_Couldn_t_allocate_memory_in_Init_004c2d30);
    }
    puVar1 = FUN_004a2831(0x49);
    if (puVar1 != 0x0) {
        puVar1 = FUN_0049a030(puVar1,0x0,0x1,0x0,0x0,0x280,0x1e0,0x4000,0x0,0x0);
    }
    DAT_005b915c = puVar1;
    puVar1 = DAT_005b915c;
    FUN_0049ae00(DAT_005b915c,FUN_00494bbd);
    FUN_0042feac();
    FUN_0049a8a0(DAT_005b915c);
    FUN_00494bbd(DAT_005b915c,0x405,0x0,0x0);
    FUN_0042feac();
    iVar3 = 0xa;
    pcVar2 = FUN_00499050(DAT_0059679c,0x73ae);
    FUN_004a2ff0(0x0,pcVar2,iVar3,puVar1);
    FUN_00490b38();
    FUN_00494949();
    FUN_004a36b0(0x1);
    FUN_0049122c();
    FUN_00493850();
    FUN_004915d2();
    FUN_004a36b0(0x2);
    FUN_00491a89();
    FUN_004a36b0(0x3);
    FUN_00491c54();
    FUN_004a36b0(0x4);
    FUN_004920ad();
    FUN_004a36b0(0x5);
    FUN_00492287();
    FUN_004a36b0(0x6);
    FUN_004924c5();
    FUN_004a36b0(0x7);
    FUN_00492be3();
    FUN_004926f4();
    FUN_00493189();
    FUN_00491e92();
    FUN_004a36b0(0xa);
    FUN_0049ab50(DAT_005b915c,0x0);
    if (DAT_005b915c != 0x0) {
        ((*(DAT_005b915c + 0x45) + 0x8))(DAT_005b915c,0x2);
    }
    FUN_0049af50(DAT_005b9160);
    for (local_18 = 0x0; local_18 < 0xd; local_18 = local_18 + 0x1) {
        *(&DAT_004d6a74 + local_18 * 0xa8) = (*(&DAT_004d6a70 + local_18 * 0xa8) * 0xd + 0x9) / 0xa;
    }
    FUN_0041d054();
    FUN_004a3800();
    return;
}



char *  FUN_00493d9c(param_1: &mut String,param_2: i32)

{
let cVar1: u8;
let mut pcVar2: String;
let mut local_14: i32;

loop {
if (*param_1 == '\0') break;
pcVar2 = param_1 + 0x1;
cVar1 = *param_1;
param_1 = pcVar2;
} while (cVar1 != '\"');
local_14 = 0x0;
while (*param_1 != '\"') {
(local_14 + param_2) = *param_1;
param_1 = param_1 + 0x1;
local_14 = local_14 + 0x1;
}
(param_2 + local_14) = 0x0;
return param_1 + 0x1;
}



char *  FUN_00493dfe(param_1: &mut String)

{
for (; (*param_1 == ' ' || (*param_1 == '\t')); param_1 = param_1 + 0x1) {
}
return param_1;
}



void  get_priv_profile_ints_00493e33(param_1: i32,LPCSTR file_name)

{
UINT private_profile_int;
let app_name: u8 [0x40];
let mut local_14: i32;

local_14 = param_1;
switch(param_1) {
case 0x0:
case 0x1:
case 0x2:
case 0x3:
case 0x4:
FUN_0049c2e0(app_name,s_House__d_004c2d58);
break;
case 0x5:
FUN_0049c2e0(app_name,s_League_004c2d61);
break;
default:^
// goto LAB_004940c3;
case 0x8:
FUN_0049c2e0(app_name,&DAT_004c2d68);
}
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
private_profile_int = GetPrivateProfileIntA(app_name,s_Li_Halan_004c2d6c,0x0,file_name);
*(UINT *)(&DAT_00569b14 + param_1 * 0x1e22) = private_profile_int;
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
private_profile_int = GetPrivateProfileIntA(app_name,s_The_Hazat_004c2d75,0x0,file_name);
*(UINT *)(&DAT_00569b18 + param_1 * 0x1e22) = private_profile_int;
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
private_profile_int = GetPrivateProfileIntA(app_name,s_Decados_004c2d7f,0x0,file_name);
*(UINT *)(&DAT_00569b1c + param_1 * 0x1e22) = private_profile_int;
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
private_profile_int = GetPrivateProfileIntA(app_name,s_Hawkwood_004c2d87,0x0,file_name);
*(UINT *)(&DAT_00569b20 + param_1 * 0x1e22) = private_profile_int;
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
private_profile_int = GetPrivateProfileIntA(app_name,s_Al_Malik_004c2d90,0x0,file_name);
*(UINT *)(&DAT_00569b24 + param_1 * 0x1e22) = private_profile_int;
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
private_profile_int = GetPrivateProfileIntA(app_name,s_League_004c2d99,0x0,file_name);
*(UINT *)(&DAT_00569b28 + param_1 * 0x1e22) = private_profile_int;
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
private_profile_int = GetPrivateProfileIntA(app_name,s_Church_004c2da0,0x0,file_name);
*(UINT *)(&DAT_00569b2c + param_1 * 0x1e22) = private_profile_int;
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
private_profile_int = GetPrivateProfileIntA(app_name,s_Symbiot_004c2da7,0x0,file_name);
*(UINT *)(&DAT_00569b30 + param_1 * 0x1e22) = private_profile_int;
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
private_profile_int = GetPrivateProfileIntA(app_name,&DAT_004c2daf,0x0,file_name);
*(UINT *)(&DAT_00569b34 + param_1 * 0x1e22) = private_profile_int;
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
private_profile_int = GetPrivateProfileIntA(app_name,s_Imperial_004c2db3,0x0,file_name);
*(UINT *)(&DAT_00569b38 + param_1 * 0x1e22) = private_profile_int;
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
private_profile_int = GetPrivateProfileIntA(app_name,s_Imperial_Fleet_004c2dbc,0x0,file_name);
*(UINT *)(&DAT_00569b3c + param_1 * 0x1e22) = private_profile_int;
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
private_profile_int = GetPrivateProfileIntA(app_name,s_Stigmata_004c2dcb,0x0,file_name);
*(UINT *)(&DAT_00569b40 + param_1 * 0x1e22) = private_profile_int;
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
private_profile_int = GetPrivateProfileIntA(app_name,s_Imperial_Eye_004c2dd4,0x0,file_name);
*(UINT *)(&DAT_00569b44 + param_1 * 0x1e22) = private_profile_int;
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
private_profile_int = GetPrivateProfileIntA(app_name,s_Neutral_004c2de1,0x0,file_name);
*(UINT *)(&DAT_00569b48 + param_1 * 0x1e22) = private_profile_int;
// LAB_004940c3:
return;
}



void read_set_options_004940cb(void)

{
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
grid_opt_00599de0 = GetPrivateProfileIntA(s_Options_004c2dee,s_grid_004c2de9,0x1,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
sound_opt_00599de8 = GetPrivateProfileIntA(s_Options_004c2dfc,s_sound_004c2df6,0x1,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
autosave_opt_00599df0 = GetPrivateProfileIntA(s_Options_004c2e0d,s_autosave_004c2e04,0x1,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
turns_till_vote_opt_00599d44 =
GetPrivateProfileIntA(s_Defaults_004c2e24,s_turns_til_vote_004c2e15,0xa,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
turns_till_patriarch_dies_00599d48 =
GetPrivateProfileIntA(s_Defaults_004c2e46,s_turns_til_patriarch_dies_004c2e2d,0x19,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
normal_damage_opt_00599d4c =
GetPrivateProfileIntA(s_Defaults_004c2e5d,s_normal_damage_004c2e4f,0x32,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
feint_damage_opt_00599d50 =
GetPrivateProfileIntA(s_Defaults_004c2e73,s_feint_damage_004c2e66,0x4b,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
loyalty_noble_bonus_00599d54 =
GetPrivateProfileIntA(s_Defaults_004c2e90,s_loyalty_noble_bonus_004c2e7c,0x1e,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
loyalty_officer_bonus_00599d58 =
GetPrivateProfileIntA(s_Defaults_004c2eaf,s_loyalty_officer_bonus_004c2e99,0x14,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
default_leadership_00599d5c =
GetPrivateProfileIntA(s_Defaults_004c2ecb,s_default_leadership_004c2eb8,0x5a,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
starting_credits_opts_00599d60 =
GetPrivateProfileIntA(s_Defaults_004c2ee5,s_starting_credits_004c2ed4,0x2710,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
dflt_tax_rate_00599d64 =
GetPrivateProfileIntA(s_Defaults_004c2eff,s_default_tax_rate_004c2eee,0xa,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
dflt_tithe_rate_00599d68 =
GetPrivateProfileIntA(s_Defaults_004c2f1b,s_default_tithe_rate_004c2f08,0xa,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
give_back_res_per_opt_00599d6c =
GetPrivateProfileIntA(s_Defaults_004c2f36,s_give_back_res_per_004c2f24,0x4b,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
dflt_church_like_00599d70 =
GetPrivateProfileIntA(s_Defaults_004c2f53,s_default_church_like_004c2f3f,0x64,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
dflt_league_like_00599d74 =
GetPrivateProfileIntA(s_Defaults_004c2f70,s_default_league_like_004c2f5c,0x64,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
dflt_interest_rate_00599d78 =
GetPrivateProfileIntA(s_Defaults_004c2f8f,s_default_interest_rate_004c2f79,0xa,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
dflt_loan_turns_00599d7c =
GetPrivateProfileIntA(s_Defaults_004c2fab,s_default_loan_turns_004c2f98,0xa,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
pct_asset_is_loan_00599d80 =
GetPrivateProfileIntA(s_Defaults_004c2fca,s_percent_asset_is_loan_004c2fb4,0x32,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
min_loan_amt_00599d84 =
GetPrivateProfileIntA(s_Defaults_004c2fe7,s_minimum_loan_amount_004c2fd3,0x3e8,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
shield_radius_00599d9c =
GetPrivateProfileIntA(s_Defaults_004c2ffe,s_shield_radius_004c2ff0,0x1,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
move_pause_00599da0 =
GetPrivateProfileIntA(s_Defaults_004c3012,s_move_pause_004c3007,0x1f4,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
health_check_for_plague_00599da4 =
GetPrivateProfileIntA(s_Defaults_004c3033,s_health_check_for_plague_004c301b,0x19,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
health_loss_for_famine_00599da8 =
GetPrivateProfileIntA(s_Defaults_004c3053,s_health_loss_for_famine_004c303c,0x5,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
immune_plague_bonus_00599dac =
GetPrivateProfileIntA(s_Defaults_004c3070,s_immune_plague_bonus_004c305c,0x5,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
city_heal_rate_00599db0 =
GetPrivateProfileIntA(s_Defaults_004c3088,s_city_heal_rate_004c3079,0x8,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
unit_heal_in_city_00599db4 =
GetPrivateProfileIntA(s_Defaults_004c30a3,s_unit_heal_in_city_004c3091,0xc,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
unit_heal_00599db8 = GetPrivateProfileIntA(s_Defaults_004c30b6,s_unit_heal_004c30ac,0x2,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
third_republic_min_00599dbc =
GetPrivateProfileIntA(s_Defaults_004c30d2,s_third_republic_min_004c30bf,0x493e0,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
third_republic_max_00599dc0 =
GetPrivateProfileIntA(s_Defaults_004c30ee,s_third_republic_max_004c30db,0x7a120,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
famine_damage_00599dc4 =
GetPrivateProfileIntA(s_Defaults_004c3105,s_famine_damage_004c30f7,0x19,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
not_enough_garrison_00599d40 =
GetPrivateProfileIntA(s_Defaults_004c3120,s_not_enuf_garrison_004c310e,0x14,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
credits_received_00582928 =
GetPrivateProfileIntA(s_Defaults_004c313a,s_credits_received_004c3129,0x2,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
per_tech_received_0058292c =
GetPrivateProfileIntA(s_Defaults_004c3155,s_per_tech_received_004c3143,0x5,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
recv_unfound_city_00582930 =
GetPrivateProfileIntA(s_Defaults_004c3173,s_receive_unfound_city_004c315e,0x1,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
promise_votes_00582934 =
GetPrivateProfileIntA(s_Defaults_004c318a,s_promise_votes_004c317c,0x5,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
league_hall_stock_00599d88 =
GetPrivateProfileIntA(s_League_004c31a4,s_leaguehall_stock_004c3193,0x3e8,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
league_like_up_00599d8c =
GetPrivateProfileIntA(s_League_004c31ba,s_league_like_up_004c31ab,0x1,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
league_like_down_00599d90 =
GetPrivateProfileIntA(s_League_004c31d2,s_league_like_down_004c31c1,0x5,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
league_int_rate_up_00599d94 =
GetPrivateProfileIntA(s_League_004c31ec,s_league_int_rate_up_004c31d9,0x5,&filename_00599c80);
// LPCSTR lpFileName for GetPrivateProfileIntA
// INT nDefault for GetPrivateProfileIntA
// LPCSTR lpKeyName for GetPrivateProfileIntA
// LPCSTR lpAppName for GetPrivateProfileIntA
league_int_rate_down_00599d98 =
GetPrivateProfileIntA(s_League_004c3208,s_league_int_rate_down_004c31f3,0x1,&filename_00599c80);
DWORD_00599dc8 = 0x1;
DWORD_00599dcc = 0x1;
DWORD_00599dd0 = 0x1;
DWORD_00599dd4 = 0x1;
return;
}



fn FUN_00494639()

{
    let mut uVar1: u32;
    let local_118: u8 [0x104];
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0xe; local_14 = local_14 + 0x1) {
        uVar1 = FUN_0049c2c9(0x2710);
        *(&DAT_00595700 + local_14 * 0x4) = uVar1;
        switch(local_14) {
            case 0x0:
                case 0x1:
                case 0x2:
                case 0x3:
                case 0x4:
                FUN_0049c2e0(local_118,s_bin_house_d_bin_004c320f);
            FUN_0049c57b(local_118,(&DAT_00595700 + local_14 * 0x4),0x2710);
            break;
            case 0x5:
                FUN_0049c57b(s_bin_bnrleg_bin_004c321f,(&DAT_00595700 + local_14 * 0x4),0x2710);
            break;
            case 0x6:
                FUN_0049c57b(s_bin_bnrchu_bin_004c322e,(&DAT_00595700 + local_14 * 0x4),0x2710);
            break;
            case 0x7:
                FUN_0049c57b(s_bin_sym100_bin_004c324c,(&DAT_00595700 + local_14 * 0x4),0x2710);
            break;
            case 0x8:
                FUN_0049c57b(s_bin_bnrvau_bin_004c323d,(&DAT_00595700 + local_14 * 0x4),0x2710);
            break;
            case 0x9:
                FUN_0049c57b(s_bin_rgt100_bin_004c326a,(&DAT_00595700 + local_14 * 0x4),0x2710);
            break;
            case 0xa:
                FUN_0049c57b(s_bin_flet100_bin_004c3279,(&DAT_00595700 + local_14 * 0x4),0x2710);
            break;
            case 0xb:
                FUN_0049c57b(s_bin_stig100_bin_004c3289,(&DAT_00595700 + local_14 * 0x4),0x2710);
            break;
            case 0xc:
                FUN_0049c57b(s_bin_eye100_bin_004c3299,(&DAT_00595700 + local_14 * 0x4),0x2710);
            break;
            case 0xd:
                FUN_0049c57b(s_bin_bnrreb_bin_004c325b,(&DAT_00595700 + local_14 * 0x4),0x2710);
            break;
            default:
                FUN_004a0430(*(&DAT_00595700 + local_14 * 0x4),(char)*(&DAT_004be9e8 + local_14 * 0x4)
            ,0x2710);
        }
    }
    return;
}



fn FUN_0049488e(byte *param_1) -> i32

{
let mut iVar1: i32;
let mut local_14: i32;

local_14 = 0x0;
while( true ) {
if (0x1f < local_14) {
return -0x1;
}
iVar1 = FUN_004a2f10(param_1,&DAT_005831e8 + local_14 * 0x50);
if (iVar1 == 0x0) break;
local_14 = local_14 + 0x1;
}
return local_14;
}



fn FUN_004948ea(byte *param_1) -> i32

{
let mut iVar1: i32;
let mut local_14: i32;

local_14 = 0x0;
while( true ) {
if (0xc < local_14) {
return -0x1;
}
iVar1 = FUN_004a2f10(param_1,(local_14 * 0xa8 + 0x4d6a78));
if (iVar1 == 0x0) break;
local_14 = local_14 + 0x1;
}
return local_14;
}



fn FUN_00494949()

{
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0x5c; local_14 = local_14 + 0x1) {
        for (local_24 = 0x0; local_24 < 0x6; local_24 = local_24 + 0x1) {
            if (*(&DAT_00582938 + local_24 * 0x4 + local_14 * 0x18) != 0x0) {
                local_1c = 0x0;
                local_20 = 0x0;
                for (local_28 = 0x0; local_28 < 0x9; local_28 = local_28 + 0x1) {
                    if (*(*(&DAT_00582938 + local_24 * 0x4 + local_14 * 0x18) + local_28 * 0x4 + 0x81) != 0x0) {
                        local_1c = local_1c +
                            *(local_28 * 0x4 + *(&DAT_00582938 + local_14 * 0x18 + local_24 * 0x4) + 0x5d) *
                                *(local_28 * 0x4 + *(&DAT_00582938 + local_24 * 0x4 + local_14 * 0x18) + 0x81);
                        local_20 = local_20 + 0x1;
                    }
                }
                if (local_20 == 0x0) {
                    local_18 = 0x0;
                }
                else {
                    local_18 = local_1c / 0xa;
                    if (local_18 < 0x1) {
                        local_18 = 0x1;
                    }
                    local_18 = local_18 +
                        (*(*(&DAT_00582938 + local_24 * 0x4 + local_14 * 0x18) + 0x51) *
                            *(*(&DAT_00582938 + local_14 * 0x18 + local_24 * 0x4) + 0x55)) / 0xa;
                }
                switch(*(*(&DAT_00582938 + local_24 * 0x4 + local_14 * 0x18) + 0x41)) {
                    case 0x0:
                        case 0x1:
                        case 0x2:
                        case 0x3:
                        case 0x4:
                        case 0x6:
                        case 0x8:
                        *(*(&DAT_00582938 + local_14 * 0x18 + local_24 * 0x4) + 0x121) = local_18;
                    *(*(&DAT_00582938 + local_14 * 0x18 + local_24 * 0x4) + 0x125) = 0x0;
                    break;
                    case 0x5:
                        case 0x7:
                        case 0x9:
                        *(*(&DAT_00582938 + local_14 * 0x18 + local_24 * 0x4) + 0x121) = 0x0;
                    *(*(&DAT_00582938 + local_14 * 0x18 + local_24 * 0x4) + 0x125) = local_18;
                    break;
                    default:
                        *(*(&DAT_00582938 + local_14 * 0x18 + local_24 * 0x4) + 0x121) = 0x0;
                    *(*(&DAT_00582938 + local_14 * 0x18 + local_24 * 0x4) + 0x125) = 0x0;
                }
            }
        }
    }
    return;
}



fn FUN_00494bbd(param_1: i32,param_2: i32,param_3: u32,param_4: u32) -> u32

{
    let mut local_18: u32;

    if (param_2 == 0x405) {
        FUN_004953d7();
        FUN_004a08c5(s_pcx_cathed3_pcx_004c32a8,0x0,0x0,0x280,0x1e0,0x0,0x0,0x0,0x1);
        FUN_0049536f();
        local_18 = 0x1;
    }
    else {
        local_18 = ((*(param_1 + 0x45) + 0xc))(param_1,param_1,param_2,param_3,param_4);
    }
    return local_18;
}



fn FUN_00494c43(param_1: *mut u32,param_2: *mut u32)

{
    *param_1 = DAT_005b91a0;
    *param_2 = DAT_005b91a4;
    return;
}



fn FUN_00494c6a()

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let puVar3: *mut u32;
    let piVar4: *mut i32;;
    let mut bVar5: bool;
    let mut bVar6: bool;
    let local_88: *mut i32; [0x6];
    let mut local_70: i32;
    let mut local_6c: u32;
    let local_68: *mut i32;;
    i32 local_64 [0x10];
    let mut local_24: i32;
    let mut local_20: u32;
    let mut local_1c: i32;
    let mut local_18: i32;

    if (DAT_005b9220 == 0x0) {
        DAT_005b91b8 = 0x0;
    }
    else {
        if (DAT_005b91b8 != 0x0) {
            if (DAT_005b91bc < 0x1) {
                FUN_004aaa04(DAT_005b91c4,DAT_005b91d8,DAT_005b91dc,DAT_005b91e0,DAT_005b91e4,0x0);
                DAT_005b91b8 = 0x0;
            }
            else {
                local_24 = DAT_005b91a0 - DAT_005b91a8;
                local_20 = DAT_005b91a4 - DAT_005b91ac;
                local_1c = DAT_005b9190;
                local_18 = DAT_005b9194;
                if (DAT_005b9228 < (DAT_005b9190 + local_24)) {
                    local_1c = DAT_005b9228 - local_24;
                }
                if (DAT_005b922c < DAT_005b9194 + local_20) {
                    local_18 = DAT_005b922c - local_20;
                }
                if (local_24 < 0x0) {
                    local_1c = local_1c + local_24;
                    local_24 = 0x0;
                }
                if (local_20 < 0x0) {
                    local_18 = local_18 + local_20;
                    local_20 = 0x0;
                }
                iVar1 = FUN_004aa144(&local_24,&DAT_005b91d8);
                if (iVar1 == 0x0) {
                    FUN_004aaa04(DAT_005b91c4,DAT_005b91d8,DAT_005b91dc,DAT_005b91e0,DAT_005b91e4,0x0);
                    DAT_005b91d8 = local_24;
                    DAT_005b91dc = DAT_005b91dc & 0xffffff00 | local_20 & 0xff;
                    FUN_004aaa6c(local_24,DAT_005b91dc,DAT_005b91c4,DAT_005b91e0,DAT_005b91e4,0x0);
                }
                else {
                    iVar2 = 0x10;
                    bVar5 = false;
                    iVar1 = 0x0;
                    bVar6 = true;
                    puVar3 = &DAT_005b91d8;
                    piVar4 = &local_24;
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
                        DAT_005b91b8 = 0x1;
                    }
                    else {
                        local_68 = local_64;
                        local_6c = FUN_004aa20c(&DAT_005b91d8,&local_24,local_64);
                        for (local_70 = 0x0; local_70 < local_6c; local_70 = local_70 + 0x1) {
                            FUN_004aaa04(
                                (DAT_005b91c4 +
                                    DAT_005b91e0 * (local_68[0x1] - DAT_005b91dc) + (*local_68 - DAT_005b91d8)),*local_68,
                                local_68[0x1],local_68[0x2],local_68[0x3],DAT_005b91e0 - local_68[0x2]);
                            local_68 = local_68 + 0x4;
                        }
                        FUN_004aaa6c(local_24,local_20,DAT_005b91c8,local_1c,local_18,0x0);
                        FUN_00498ba4(local_88,DAT_005b91c8,local_18,local_1c);
                        FUN_00496ac0(DAT_005b91c4,DAT_005b91d8 - local_24,DAT_005b91dc - local_20,DAT_005b91e0,DAT_005b91e4);
                        FUN_00498cf4(local_88);
                        puVar3 = DAT_005b91c4;
                        DAT_005b91c4 = DAT_005b91c8;
                        DAT_005b91c8 = puVar3;
                        DAT_005b91d8 = local_24;
                        DAT_005b91dc = DAT_005b91dc & 0xffffff00 | local_20 & 0xff;
                    }
                }
            }
        }
    }
    return;
}



fn FUN_00494f54()

{
    let local_44: *mut i32; [0x6];
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_24: u32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: i32;

    if (0x0 < DAT_005b91bc) {
        if (DAT_005b91b8 == 0x0) {
            local_28 = DAT_005b9194;
            local_24 = DAT_005b9190;
            local_2c = DAT_005b91a0 - DAT_005b91a8;
            local_1c = DAT_005b91a4 - DAT_005b91ac;
            if (DAT_005b9228 < DAT_005b9190 + local_2c) {
                local_24 = DAT_005b9228 - local_2c;
            }
            if (DAT_005b922c < (DAT_005b9194 + local_1c)) {
                local_28 = DAT_005b922c - local_1c;
            }
            local_20 = local_2c;
            if (local_2c < 0x0) {
                local_24 = local_24 + local_2c;
                local_20 = 0x0;
            }
            local_18 = local_1c;
            if (local_1c < 0x0) {
                local_28 = local_28 + local_1c;
                local_18 = 0x0;
            }
            FUN_004aaa6c(local_20,local_18,DAT_005b91c4,local_24,local_28,0x0);
            DAT_005b91b8 = 0x1;
        }
        else {
            local_2c = DAT_005b91a0 - DAT_005b91a8;
            local_1c = DAT_005b91a4 - DAT_005b91ac;
            local_20 = DAT_005b91d8;
            local_18 = DAT_005b91dc;
            local_24 = DAT_005b91e0;
            local_28 = DAT_005b91e4;
        }
        DAT_005b91d8 = local_20;
        DAT_005b91dc = local_18;
        DAT_005b91e0 = local_24;
        DAT_005b91e4 = local_28;
        FUN_00498ba4(local_44,DAT_005b91c8,DAT_005b9194,DAT_005b9190);
        FUN_00496ac0(DAT_005b91c4,local_20 - local_2c,local_18 - local_1c,local_24,local_28);
        FUN_00496ee6(DAT_005b91d0,0x0,0x0,DAT_005b9190,DAT_005b9194);
        FUN_00498cf4(local_44);
        FUN_004aaa04(((local_20 - local_2c) + DAT_005b91c8 + DAT_005b9190 * (local_18 - local_1c)),local_20,
                     local_18,local_24,local_28,DAT_005b9190 - local_24);
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00495145(param_1: i32,param_2: i32)

{
    let mut iVar1: i32;

    if ((_DAT_005b9170 == 0x1) &&
        (iVar1 = FUN_004aa0ab(&DAT_005b9180,param_1 - DAT_005b91a8,param_2 - DAT_005b91ac,DAT_005b9190,DAT_005b9194),
         iVar1 != 0x0)) {
        _DAT_005b9170 = _DAT_005b9170 + 0x1;
        _DAT_005b9174 = DAT_005b91a0;
        _DAT_005b9178 = DAT_005b91a4;
    }
    DAT_005b91a0 = param_1;
    DAT_005b91a4 = param_2;
    if ((DAT_005b9220 == 0x0) && (iVar1 = FUN_0049613c(), iVar1 != 0x0)) {
        FUN_00494c6a();
        if (_DAT_005b9170 < 0x2) {
            FUN_00494f54();
        }
        FUN_0049621b();
        return;
    }
    DAT_005b91b8 = 0x0;
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_004951f3()

{
    let mut iVar1: i32;

    DAT_005b91bc = 0x0;
    if (((DAT_005b91b8 != 0x0) && (_DAT_005b91c0 != 0x0)) && (iVar1 = FUN_0049613c(), iVar1 != 0x0)) {
        FUN_00494c6a();
        FUN_0049621b();
    }
    _DAT_005b91c0 = 0x0;
    if (DAT_005b91c4 != 0x0) {
        FUN_004aaae0(DAT_005b91c4);
    }
    if (DAT_005b91cc != 0x0) {
        FUN_004aaae0(DAT_005b91cc);
    }
    if (DAT_005b91c8 != 0x0) {
        FUN_004aaae0(DAT_005b91c4);
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00495286(param_1: u32) -> u32

{
    let mut local_18: u32;

    _DAT_005b919c = param_1;
    DAT_005b91c4 = FUN_004aac00(param_1);
    DAT_005b91cc = FUN_004aac00(param_1);
    DAT_005b91c8 = FUN_004aac00(param_1);
    if (((DAT_005b91c4 == 0x0) || (DAT_005b91cc == 0x0)) || (DAT_005b91c8 == 0x0)) {
        local_18 = 0x0;
    }
    else {
        DAT_005b91d0 = DAT_005b91cc;
        _DAT_005b91c0 = 0x1;
        FUN_004aad20(FUN_004951f3);
        DAT_005b9194 = 0x20;
        DAT_005b9190 = 0x20;
        DAT_005b9198 = 0x400;
        FUN_0049c57b(s_MOUSE_MSK_004c32b8,DAT_005b91d0,0x400);
        DAT_005b91b8 = 0x0;
        DAT_005b91bc = 0x0;
        local_18 = 0x1;
    }
    return local_18;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0049536f()

{
    let mut iVar1: i32;

    iVar1 = DAT_005b91bc;
    if ((((_DAT_005b91c0 != 0x0) && (_DAT_005b9170 == 0x0)) && (DAT_005b920c == 0x0)) &&
        (DAT_005b91bc = DAT_005b91bc + 0x1, iVar1 == 0x0)) {
        if (DAT_005b9220 == 0x0) {
            FUN_0049613c();
        }
        if (DAT_005b9220 != 0x0) {
            FUN_00494f54();
            FUN_004960df();
        }
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_004953d7()

{
    let mut iVar1: i32;

    if ((((_DAT_005b91c0 != 0x0) && (_DAT_005b9170 == 0x0)) && (DAT_005b920c == 0x0)) &&
        (DAT_005b91bc = DAT_005b91bc + -0x1, DAT_005b91bc == 0x0)) {
        iVar1 = FUN_0049607e();
        if (iVar1 == 0x0) {
            DAT_005b91b8 = 0x0;
        }
        else {
            FUN_00494c6a();
        }
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00495438(param_1: u32,param_2: i32,param_3: i32)

{
    if ((_DAT_005b91c0 != 0x0) && ((param_2 * param_3) < _DAT_005b919c || param_2 * param_3 - _DAT_005b919c == 0x0))
    {
        FUN_004953d7();
        DAT_005b91d0 = param_1;
        DAT_005b9190 = param_2;
        DAT_005b9194 = param_3;
        DAT_005b9198 = param_2 * param_3;
        FUN_0049536f();
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00495491()

{
    if (_DAT_005b91c0 != 0x0) {
        FUN_004953d7();
        DAT_005b91a8 = DAT_005b91b0;
        DAT_005b91ac = DAT_005b91b4;
        DAT_005b9194 = 0x20;
        DAT_005b9190 = 0x20;
        DAT_005b9198 = 0x400;
        DAT_005b91d0 = DAT_005b91cc;
        FUN_0049536f();
    }
    return;
}

