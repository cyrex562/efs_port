
fn FUN_0040e6c5()

{
    let mut pcVar1: String;
    let local_90: u8 [0x80];

    if (DAT_004c6118 == -0x1) {
        FUN_004968e7(0xc8,0x5a,0xf0,0x14,0xe);
    }
    else {
        FUN_00499050(DAT_0059679c,DAT_004c6118 + 0x1af4);
        pcVar1 = FUN_00499050(DAT_0059679c,0x7356);
        FUN_0049c2e0(local_90,pcVar1);
        FUN_00497567(0x140,0x5e,local_90,0xf0,0xcaccce,0xe0e0e,0xcaccce,LPCSTR_005b9218,0x11);
    }
    return;
}



fn FUN_0040e76e()

{
    let mut pcVar1: String;
    let mut iVar2: i32;
    let mut uVar3: u32;
    let mut iVar4: i32;
    let mut uVar5: u32;
    let puVar6: *mut u32;
    let bVar7: u8;
    let local_90: u8 [0x80];

    FUN_004a08c5(s_pcx_bg0_pcx_004c08a2,0x0,0x0,0x280,0x1e0,0x0,0x0,0x0,0x1);
    FUN_004968e7(0xc8,0x5a,0xf0,0x14,0xe);
    FUN_0049e640(0xc8,0x5a,0xf0,0x14,0xce,0xca,0xcc,0x1);
    FUN_0040e6c5();
    bVar7 = 0x11;
    uVar5 = 0xcaccce;
    iVar4 = -0x1;
    uVar3 = 0xcaccce;
    iVar2 = 0xc8;
    puVar6 = LPCSTR_005b9218;
    pcVar1 = FUN_00499050(DAT_0059679c,0x73f1);
    FUN_00497567(0x73,0x91,pcVar1,iVar2,uVar3,iVar4,uVar5,puVar6,bVar7);
    bVar7 = 0x11;
    uVar5 = 0xcaccce;
    iVar4 = -0x1;
    uVar3 = 0xcaccce;
    iVar2 = 0xc8;
    puVar6 = LPCSTR_005b9218;
    pcVar1 = FUN_00499050(DAT_0059679c,0x73f1);
    FUN_00497567(0x20d,0x91,pcVar1,iVar2,uVar3,iVar4,uVar5,puVar6,bVar7);
    FUN_004968e7(0xf,0xa0,0xc8,0x46,0xe);
    FUN_004968e7(0x1a9,0xa0,0xc8,0x46,0xe);
    FUN_0049e640(0xf,0xa0,0xc8,0x46,0xce,0xca,0xcc,0x1);
    FUN_0049e640(0x1a9,0xa0,0xc8,0x46,0xce,0xca,0xcc,0x1);
    FUN_0040e175(&DAT_004c50b8,&DAT_004c58e8,0xf,0xa0);
    FUN_0040e175(&DAT_004c58e8,&DAT_004c50b8,0x1a9,0xa0);
    FUN_004968e7(0xf,0xfa,0xc8,0xa0,0xe);
    FUN_004968e7(0x1a9,0xfa,0xc8,0xa0,0xe);
    FUN_0049e640(0xf,0xfa,0xc8,0xa0,0xce,0xca,0xcc,0x1);
    FUN_0049e640(0x1a9,0xfa,0xc8,0xa0,0xce,0xca,0xcc,0x1);
    FUN_0049e640(0xf,0xa,0x64,0x64,0xce,0xca,0xcc,0x1);
    FUN_0049e640(0x20d,0xa,0x64,0x64,0xce,0xca,0xcc,0x1);
    FUN_00499050(DAT_0059679c,(DAT_004c58d0 >> 0x10) + 0x414);
    pcVar1 = FUN_00499050(DAT_0059679c,0x7354);
    FUN_0049c2e0(local_90,pcVar1);
    FUN_00497567(0x7d,0xa,local_90,0xa0,0xcaccce,-0x1,0xcaccce,LPCSTR_005b9218,0x10);
    FUN_00499050(DAT_0059679c,(DAT_004c6100 >> 0x10) + 0x414);
    pcVar1 = FUN_00499050(DAT_0059679c,0x7355);
    FUN_0049c2e0(local_90,pcVar1);
    FUN_00497567(0x203,0xa,local_90,0xa0,0xcaccce,-0x1,0xcaccce,LPCSTR_005b9218,0x14);
    FUN_00497567(0x7d,0x1e,(&DAT_00569b50 + (DAT_004c58d0 >> 0x10) * 0x1e22),0xa0,0xcaccce,-0x1,0xcaccce,
                 LPCSTR_005b9218,0x10);
    FUN_00497567(0x203,0x1e,(&DAT_00569b50 + (DAT_004c6100 >> 0x10) * 0x1e22),0xa0,0xcaccce,-0x1,0xcaccce,
                 LPCSTR_005b9218,0x14);
    FUN_00496ac0((&DAT_00595700 + (DAT_004c58d0 >> 0x10) * 0x4),0xf,0xa,0x64,0x64);
    FUN_00496ac0((&DAT_00595700 + (DAT_004c6100 >> 0x10) * 0x4),0x20d,0xa,0x64,0x64);
    return;
}



fn FUN_0040eba9()

{
    if ((DAT_004c611c == 0x2) && (DAT_004c6114 != 0x0)) {
        FUN_004968e7(0x10e,0x177,0x64,0x28,0xe);
        FUN_004817f9(0x10e,0x177,(short *)(DAT_004c6114 + 0x8));
        FUN_0049e640(0x10e,0x177,0x64,0x28,0xce,0xca,0xcc,0x1);
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0040ec2e()

{
    let mut bVar1: bool;
    let local_48: u8 [0x20];
    let mut local_28: i32;
    let local_24: *mut u32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    local_28 = 0x1;
    local_24 = FUN_0049c2c9(0x484);
    FUN_004a1651();
    if (_DAT_004c612c != 0x0) {
        FUN_00430418(0x734c,0x3e8,0x0);
    }
    for (local_20 = 0x0; local_20 < 0x14; local_20 = local_20 + 0x1) {
        if ((((&DAT_004c50bc + local_20 * 0x5e) & 0x8) != 0x0) &&
            (((&DAT_004c50bd)[local_20 * 0x5e] & 0x10) == 0x0)) {
            (&DAT_004c50bd)[local_20 * 0x5e] = (&DAT_004c50bd)[local_20 * 0x5e] | 0x2;
            FUN_004906c1(local_24,*(&DAT_004d6058 + *(&DAT_004c510a + local_20 * 0x5e) * 0x1c),
                         (char)*(&DAT_004be9e8 + (DAT_004c58d0 >> 0x10) * 0x4),
                         *(&DAT_004c5112 + local_20 * 0x5e),0x22,-0x1);
            FUN_00496ac0(local_24,*(&DAT_004c5102 + local_20 * 0x5e),*(&DAT_004c5106 + local_20 * 0x5e),0x22,
                         0x22);
        }
        if ((((&DAT_004c58ec + local_20 * 0x5e) & 0x8) != 0x0) &&
            (((&DAT_004c58ed)[local_20 * 0x5e] & 0x10) == 0x0)) {
            (&DAT_004c58ed)[local_20 * 0x5e] = (&DAT_004c58ed)[local_20 * 0x5e] | 0x2;
            FUN_004906c1(local_24,*(&DAT_004d6058 + *(&DAT_004c593a + local_20 * 0x5e) * 0x1c),
                         (char)*(&DAT_004be9e8 + (DAT_004c6100 >> 0x10) * 0x4),
                         *(&DAT_004c5942 + local_20 * 0x5e),0x22,-0x1);
            FUN_00496ac0(local_24,*(&DAT_004c5932 + local_20 * 0x5e),*(&DAT_004c5936 + local_20 * 0x5e),0x22,
                         0x22);
        }
    }
    local_28 = 0x1;
    for (local_20 = 0x0; local_20 < 0x14; local_20 = local_20 + 0x1) {
        if ((((&DAT_004c50bc + local_20 * 0x5e) & 0x8) != 0x0) &&
            (((&DAT_004c50bd)[local_20 * 0x5e] & 0x10) != 0x0)) {
            if (local_28 == 0x0) {
                timer_func_0049e710(0xc8);
            }
            else {
                FUN_00430418(0x73a2,0x5dc,0x0);
                local_28 = 0x0;
            }
            (&DAT_004c50bd)[local_20 * 0x5e] = (&DAT_004c50bd)[local_20 * 0x5e] | 0x2;
            FUN_004906c1(local_24,*(&DAT_004d6058 + *(&DAT_004c510a + local_20 * 0x5e) * 0x1c),
                         (char)*(&DAT_004be9e8 + (DAT_004c58d0 >> 0x10) * 0x4),
                         *(&DAT_004c5112 + local_20 * 0x5e),0x22,-0x1);
            FUN_00496ac0(local_24,*(&DAT_004c5102 + local_20 * 0x5e),*(&DAT_004c5106 + local_20 * 0x5e),0x22,
                         0x22);
        }
    }
    local_28 = 0x1;
    for (local_20 = 0x0; local_20 < 0x14; local_20 = local_20 + 0x1) {
        if ((((&DAT_004c58ec + local_20 * 0x5e) & 0x8) != 0x0) &&
            (((&DAT_004c58ed)[local_20 * 0x5e] & 0x10) != 0x0)) {
            if (local_28 == 0x0) {
                timer_func_0049e710(0xc8);
            }
            else {
                FUN_00430418(0x73a3,0x5dc,0x0);
                local_28 = 0x0;
            }
            (&DAT_004c58ed)[local_20 * 0x5e] = (&DAT_004c58ed)[local_20 * 0x5e] | 0x2;
            FUN_004906c1(local_24,*(&DAT_004d6058 + *(&DAT_004c593a + local_20 * 0x5e) * 0x1c),
                         (char)*(&DAT_004be9e8 + (DAT_004c6100 >> 0x10) * 0x4),
                         *(&DAT_004c5942 + local_20 * 0x5e),0x22,-0x1);
            FUN_00496ac0(local_24,*(&DAT_004c5932 + local_20 * 0x5e),*(&DAT_004c5936 + local_20 * 0x5e),0x22,
                         0x22);
        }
    }
    DAT_004c6118 = -0x1;
    timer_func_0049e710(0x1f4);
    for (local_1c = 0x0; local_1c < 0x9; local_1c = local_1c + 0x1) {
        for (local_20 = 0x0; local_20 < 0x14; local_20 = local_20 + 0x1) {
            bVar1 = true;
            if ((((((&DAT_004c50bc + local_20 * 0x5e) & 0x1) != 0x0) &&
                (((&DAT_004c50bc + local_20 * 0x5e) & 0x8) != 0x0)) &&
                (*(&DAT_004c50d2 + local_1c * 0x4 + local_20 * 0x5e) != 0x0)) &&
                ((*(&DAT_004c50f6 + local_20 * 0x5e) == 0x0 || (local_1c <= *(&DAT_004c50f6 + local_20 * 0x5e))))
            ) {
                timer_func_0049e710(0xc8);
                bVar1 = false;
                DAT_004c6118 = local_1c;
                FUN_0040e6c5();
                FUN_004a2edc();
                FUN_0049c2e0(local_48,s_s_explo_d_wav_004c08bd);
                FUN_004a2d6b();
                FUN_0040c771(DAT_004c50b4,*(&DAT_004c510a + local_20 * 0x5e),DAT_004c58d0 >> 0x10,
                             *(&DAT_004c5112 + local_20 * 0x5e),*(&DAT_004c5102 + local_20 * 0x5e),
                             *(&DAT_004c5106 + local_20 * 0x5e),
                             *(&DAT_004c50d2 + local_20 * 0x5e + local_1c * 0x4));
            }
            if ((((&DAT_004c50bc + local_20 * 0x5e) & 0x8000) != 0x0) &&
                (*(&DAT_004c50f6 + local_20 * 0x5e) == local_1c)) {
                if (bVar1) {
                    bVar1 = false;
                    timer_func_0049e710(0xc8);
                }
                FUN_00496ee6(&DAT_00593da8,*(&DAT_004c5102 + local_20 * 0x5e) + -0x3,
                             *(&DAT_004c5106 + local_20 * 0x5e) + -0x3,0x28,0x28);
            }
            if ((((((&DAT_004c58ec + local_20 * 0x5e) & 0x1) != 0x0) &&
                (((&DAT_004c58ec + local_20 * 0x5e) & 0x8) != 0x0)) &&
                (*(&DAT_004c5902 + local_1c * 0x4 + local_20 * 0x5e) != 0x0)) &&
                ((*(&DAT_004c5926 + local_20 * 0x5e) == 0x0 || (local_1c <= *(&DAT_004c5926 + local_20 * 0x5e))))
            ) {
                if (bVar1) {
                    bVar1 = false;
                    timer_func_0049e710(0xc8);
                }
                DAT_004c6118 = local_1c;
                FUN_0040e6c5();
                FUN_004a2edc();
                FUN_0049c2e0(local_48,s_s_explo_d_wav_004c08cb);
                FUN_004a2d6b();
                FUN_0040c771(DAT_004c50b4,*(&DAT_004c593a + local_20 * 0x5e),DAT_004c6100 >> 0x10,
                             *(&DAT_004c5942 + local_20 * 0x5e),*(&DAT_004c5932 + local_20 * 0x5e),
                             *(&DAT_004c5936 + local_20 * 0x5e),
                             *(&DAT_004c5902 + local_20 * 0x5e + local_1c * 0x4));
            }
            if ((((&DAT_004c58ec + local_20 * 0x5e) & 0x8000) != 0x0) &&
                (*(&DAT_004c5926 + local_20 * 0x5e) == local_1c)) {
                if (bVar1) {
                    timer_func_0049e710(0xc8);
                }
                FUN_00496ee6(&DAT_00593da8,*(&DAT_004c5932 + local_20 * 0x5e) + -0x3,
                             *(&DAT_004c5936 + local_20 * 0x5e) + -0x3,0x28,0x28);
            }
            (&DAT_004c50bd)[local_20 * 0x5e] = (&DAT_004c50bd)[local_20 * 0x5e] | 0x1;
            (&DAT_004c58ed)[local_20 * 0x5e] = (&DAT_004c58ed)[local_20 * 0x5e] | 0x1;
        }
        if ((((DAT_004c611c == 0x2) && (local_1c == 0x6)) && (DAT_004c6114 != 0x0)) &&
            ((DAT_004c6114 + 0x38) != 0x0)) {
            DAT_004c6118 = local_1c;
            FUN_0040e6c5();
            sleep_0040c6d8(DAT_004c50b4);
            FUN_0040eba9();
        }
    }
    local_18 = 0x1;
    local_14 = 0x1;
    for (local_20 = 0x0; local_20 < 0x14; local_20 = local_20 + 0x1) {
        if (((&DAT_004c50bc + local_20 * 0x5e) & 0x1) != 0x0) {
            if (((&DAT_004c50bc + local_20 * 0x5e) & 0x40) == 0x0) {
                if (((&DAT_004c50bc + local_20 * 0x5e) & 0x80) != 0x0) {
                    FUN_00496ee6(&DAT_005943e8,*(&DAT_004c5102 + local_20 * 0x5e) + -0x3,
                                 *(&DAT_004c5106 + local_20 * 0x5e) + -0x3,0x28,0x28);
                }
            }
            else {
                if (local_18 != 0x0) {
                    FUN_00430418(0x73a4,0x5dc,0x0);
                    local_18 = 0x0;
                }
            }
        }
        if (((&DAT_004c58ec + local_20 * 0x5e) & 0x1) != 0x0) {
            if (((&DAT_004c58ec + local_20 * 0x5e) & 0x40) == 0x0) {
                if (((&DAT_004c58ec + local_20 * 0x5e) & 0x80) != 0x0) {
                    FUN_00496ee6(&DAT_005943e8,*(&DAT_004c5932 + local_20 * 0x5e) + -0x3,
                                 *(&DAT_004c5936 + local_20 * 0x5e) + -0x3,0x28,0x28);
                }
            }
            else {
                if (local_14 != 0x0) {
                    FUN_00430418(0x73a5,0x5dc,0x0);
                    local_14 = 0x0;
                }
            }
        }
    }
    FUN_0049af50(local_24);
    return;
}



fn FUN_0040f7ac(param_1: i32,param_2: i32)

{
    let mut iVar1: i32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    local_1c = 0x0;
    local_18 = 0x0;
    for (local_14 = 0x0; local_14 < 0xa; local_14 = local_14 + 0x1) {
        for (local_20 = 0x0; local_20 < 0x14; local_20 = local_20 + 0x1) {
            if (((((local_20 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) &&
                (((local_20 * 0x5e + param_1 + 0x4) & 0x8) != 0x0)) &&
                (iVar1 = local_20 * 0x5e + param_1,
                 *(iVar1 + 0x46) =
                     *
                         (*(&DAT_00582938 +
                             (*(*(local_20 * 0x5e + param_1) + 0x25) >> 0x18) * 0x4 +
                             (*(*(local_20 * 0x5e + param_1) + 0x24) >> 0x18) * 0x18) + 0x11d),
                 local_14 == *(iVar1 + 0x46))) {
                if (param_2 == 0x0) {
                    *(local_20 * 0x5e + param_1 + 0x4a) = (local_18 % 0x5) * 0x28 + 0x1ad;
                    *(local_20 * 0x5e + param_1 + 0x4e) = (local_18 / 0x5) * 0x28 + 0xfe;
                    local_18 = local_18 + 0x1;
                }
                else {
                    *(local_20 * 0x5e + param_1 + 0x4a) = (local_1c % 0x5) * 0x28 + 0x13;
                    *(local_20 * 0x5e + param_1 + 0x4e) = (local_1c / 0x5) * 0x28 + 0xfe;
                    local_1c = local_1c + 0x1;
                }
            }
        }
    }
    return;
}



fn FUN_0040f94b(param_1: i32,param_2: i32)

{
    byte *pbVar1;
    let mut iVar2: i32;
    let piVar3: *mut i32;;
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0x14; local_14 = local_14 + 0x1) {
        if (((local_14 * 0x5e + param_1 + 0x4) & 0x8) != 0x0) {
            *(local_14 * 0x5e + param_1 + 0x52) = *(*(local_14 * 0x5e + param_1) + 0x24) >> 0x18;
            *(local_14 * 0x5e + param_1 + 0x56) = *(*(local_14 * 0x5e + param_1) + 0x25) >> 0x18;
            *(local_14 * 0x5e + param_1 + 0x5a) = *(*(local_14 * 0x5e + param_1) + 0x30) >> 0x18;
            if (param_2 == 0x0) {
                *(local_14 * 0x5e + param_1 + 0xe) =
                    *
                        (*(&DAT_00582938 +
                            (*(*(local_14 * 0x5e + param_1) + 0x25) >> 0x18) * 0x4 +
                            (*(*(local_14 * 0x5e + param_1) + 0x24) >> 0x18) * 0x18) + 0x51);
                if ((*(&DAT_004be9b0 + (DAT_004c58d0 >> 0x10) * 0x4) &
                    *(*(local_14 * 0x5e + param_1) + 0x3a)) == 0x0) {
                    pbVar1 = (local_14 * 0x5e + param_1 + 0x5);
                    *pbVar1 = *pbVar1 | 0x10;
                    piVar3 = (local_14 * 0x5e + param_1 + 0xe);
                    *piVar3 = *piVar3 + 0x3;
                }
                if ((DAT_004c611c != 0x0) && (DAT_004c611c != 0x3)) {
                    piVar3 = (param_1 + local_14 * 0x5e + 0xe);
                    *piVar3 = *piVar3 + *(&DAT_00591784 +
                        (*(*(local_14 * 0x5e + param_1) + 0x28) >> 0x10) * 0x4 +
                        (*((*(param_1 + 0x816) >> 0x10) * 0x4 +
                            *(&DAT_004d7d50 +
                                (*(param_1 + 0x814) >> 0x10) * 0x4 +
                                (*(param_1 + 0x812) >> 0x10) * 0x3890)) & 0xf) * 0x28);
                }
                iVar2 = FUN_00411242(*(local_14 * 0x5e + param_1 + 0xe),
                                     *(*(local_14 * 0x5e + param_1) + 0x23) >> 0x18,
                                     (*(local_14 * 0x5e + param_1) + 0x20));
                *(local_14 * 0x5e + param_1 + 0xe) = iVar2;
            }
            else {
                pbVar1 = (*(local_14 * 0x5e + param_1) + 0x2c);
                *pbVar1 = *pbVar1 & 0xe3;
                (*(local_14 * 0x5e + param_1) + 0x2c) =
                    (ushort)(param_2 << 0x2) | (*(local_14 * 0x5e + param_1) + 0x2c);
                *(local_14 * 0x5e + param_1 + 0xe) =
                    *
                        (*(&DAT_00582938 +
                            (*(*(local_14 * 0x5e + param_1) + 0x25) >> 0x18) * 0x4 +
                            (*(*(local_14 * 0x5e + param_1) + 0x24) >> 0x18) * 0x18) + 0x51);
                if ((*(&DAT_004be9b0 + (DAT_004c6100 >> 0x10) * 0x4) &
                    *(*(local_14 * 0x5e + param_1) + 0x3a)) == 0x0) {
                    pbVar1 = (local_14 * 0x5e + param_1 + 0x5);
                    *pbVar1 = *pbVar1 | 0x10;
                    piVar3 = (local_14 * 0x5e + param_1 + 0xe);
                    *piVar3 = *piVar3 + 0x3;
                }
                iVar2 = FUN_00411242(*(local_14 * 0x5e + param_1 + 0xe),
                                     *(*(local_14 * 0x5e + param_1) + 0x23) >> 0x18,
                                     (*(local_14 * 0x5e + param_1) + 0x20));
                *(local_14 * 0x5e + param_1 + 0xe) = iVar2;
            }
        }
    }
    return;
}



fn FUN_0040fcb0(i32 **param_1,param_2: u32,param_3: u32)

{
    let mut bVar1: bool;
    undefined3 extraout_var;
    i32 **local_bc;
    let mut local_b8: u32;
    i32 **local_b4;
    let mut local_b0: i32;
    i32 **local_ac;
    i32 **local_a8;
    let mut local_a4: u32;
    let mut local_a0: u32;
    let mut local_9c: u32;
    let mut local_98: i32;
    i32 **local_94;
    i32 **local_90;
    let mut local_8c: u32;
    let mut local_88: u32;
    let mut local_84: u32;
    let mut local_80: u32;
    i32 **local_7c;
    i32 **local_78;
    i32 **local_74;
    i32 **local_70;
    let mut local_6c: u32;
    let mut local_68: u32;
    let mut local_64: u32;
    i32 **local_60;
    i32 **local_5c;
    let mut local_58: i32;
    i32 **local_54;
    i32 **local_50;
    let mut local_4c: i32;
    let mut local_48: u32;
    let mut local_44: u32;
    i32 **local_40;
    i32 **local_3c;
    i32 **local_38;
    i32 **local_34;
    let mut local_30: u32;
    let mut local_2c: u32;
    let mut local_28: i32;
    let mut local_24: u32;
    let mut local_20: u32;
    let mut local_1c: u32;
    let local_18: *mut u32;
    let mut local_14: i32;

    for (local_28 = 0x0; local_28 < 0x6; local_28 = local_28 + 0x1) {
        local_24 = FUN_0043a8a2(param_2 + (&DAT_004bea60)[local_28]);
        local_20 = FUN_0043a8d5(local_24,param_3 + (&DAT_004bea7c)[local_28]);
        local_40 = param_1 + 0x8;
        local_30 = local_40 & 0xffff0000 | (param_1 + 0x22);
        local_3c = local_40;
        if ((param_1 + 0x22) == local_24) {
            local_38 = param_1 + 0x8;
            local_2c = local_38 & 0xffff0000 | (param_1 + 0x9);
            local_34 = local_38;
            if ((param_1 + 0x9) == local_20) break;
        }
    }
    for (local_14 = 0x0; local_14 < 0x6; local_14 = local_14 + 0x1) {
        local_4c = local_28 * 0x4;
        local_54 = param_1 + 0x8;
        local_48 = local_54 & 0xffff0000 | (param_1 + 0x22);
        local_50 = local_54;
        local_24 = FUN_0043a8a2((param_1 + 0x22) + (&DAT_004bea60)[local_28]);
        local_58 = local_28 * 0x4;
        local_60 = param_1 + 0x8;
        local_44 = local_60 & 0xffff0000 | (param_1 + 0x9);
        local_5c = local_60;
        local_20 = FUN_0043a8d5(local_24,(&DAT_004bea7c)[local_28] + (param_1 + 0x9));
        local_28 = local_28 + 0x1;
        if (local_28 == 0x6) {
            local_28 = 0x0;
        }
        if ((local_24 != param_2) || (local_20 != param_3)) {
            local_6c = local_24;
            local_74 = param_1 + 0x8;
            local_64 = local_74 & 0xffff0000 | local_74;
            local_70 = local_74;
            local_68 = local_20;
            local_18 = FUN_00481784(local_74,local_24,local_20);
            if ((local_18 == 0x0) ||
                ((((local_18 + 0xe) != 0x2 && ((local_18 + 0xe) != 0xf)) &&
                    ((local_18 + 0xe) != 0x10)))) {
                local_7c = param_1 + 0x8;
                local_80 = local_7c & 0xffff0000 | local_7c;
                local_1c = *(*(&DAT_004d7d50 + local_7c * 0x3890 + local_24 * 0x4) +
                    local_20 * 0x4);
                local_78 = local_7c;
                bVar1 = FUN_0045a870(param_1,local_1c);
                if (CONCAT31(extraout_var,bVar1) != 0x0) {
                    local_88 = local_20;
                    local_8c = local_24;
                    local_94 = param_1 + 0x8;
                    local_84 = local_94 & 0xffff0000 | local_94;
                    local_90 = local_94;
                    local_98 = FUN_0044ace5(local_94,local_24,local_20,0x1);
                    if (local_98 == -0x1) break;
                    if (*(param_1 + 0x23) >> 0x18 == local_98) {
                        local_a0 = local_20;
                        local_a4 = local_24;
                        local_ac = param_1 + 0x8;
                        local_9c = local_ac & 0xffff0000 | local_ac;
                        local_a8 = local_ac;
                        local_b0 = FUN_00485ea2(local_ac,local_24,local_20,0x1);
                        if (local_b0 < 0x14) break;
                    }
                }
            }
        }
    }
    if (local_14 == 0x6) {
        FUN_00484b4e(param_1);
    }
    else {
        FUN_00431d31(&local_b8);
        for (local_b4 = param_1; (local_b4 != (i32 **)0x0 && (local_b4[0x3] != 0x0));
            local_b4 = (i32 **)local_b4[0x3]) {
        }
        FUN_00431dec(&local_b8,param_1);
        for (local_bc = param_1; local_bc[0x3] != 0x0; local_bc = (i32 **)local_bc[0x3]) {
        }
        for (; local_bc != (i32 **)0x0; local_bc = (i32 **)local_bc[0x2]) {
            FUN_004841ea(local_bc,(local_bc + 0x8),local_24,local_20);
            FUN_00459e8f(local_bc);
            FUN_0044add9(local_bc);
        }
    }
    return;
}



fn FUN_004100fe(param_1: i32,param_2: u32,param_3: u32,param_4: i32,param_5: i32)

{
    byte *pbVar1;
    let mut pcVar2: String;
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0x14; local_14 = local_14 + 0x1) {
        if (((local_14 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) {
            if (((local_14 * 0x5e + param_1 + 0x4) & 0x8000) == 0x0) {
                if (((local_14 * 0x5e + param_1 + 0x4) & 0x80) == 0x0) {
                    if ((((param_4 != 0x0) && (((local_14 * 0x5e + param_1 + 0x4) & 0x8) != 0x0)) && (param_5 != 0x1))
                        && (0x0 < *(*(local_14 * 0x5e + param_1) + 0x2c) >> 0x18)) {
                        pcVar2 = (*(local_14 * 0x5e + param_1) + 0x2f);
                        *pcVar2 = *pcVar2 + -0x1;
                    }
                }
                else {
                    pbVar1 = (local_14 * 0x5e + param_1 + 0x4);
                    *pbVar1 = *pbVar1 & 0xfe;
                    pbVar1 = (local_14 * 0x5e + param_1 + 0x4);
                    *pbVar1 = *pbVar1 & 0x7f;
                    if (param_4 == 0x0) {
                        if (param_5 == 0x3) {
                            pbVar1 = (*(local_14 * 0x5e + param_1) + 0x3b);
                            *pbVar1 = *pbVar1 | 0x20;
                        }
                        else {
                            if (param_5 == 0x1) {
                                pbVar1 = (*(local_14 * 0x5e + param_1) + 0x3a);
                                *pbVar1 = *pbVar1 | 0x2;
                            }
                            FUN_0040fcb0(*(i32 ***)(local_14 * 0x5e + param_1),param_2,param_3);
                        }
                    }
                    else {
                        if (*(*(local_14 * 0x5e + param_1) + 0x28) >> 0x10 == 0x3) {
                            if (0x0 < *(*(local_14 * 0x5e + param_1) + 0x2c) >> 0x18) {
                                pcVar2 = (*(local_14 * 0x5e + param_1) + 0x2f);
                                *pcVar2 = *pcVar2 + -0x1;
                            }
                        }
                        else {
                            (*(local_14 * 0x5e + param_1) + 0x2f) = 0x0;
                        }
                    }
                }
            }
            else {
                pbVar1 = (local_14 * 0x5e + param_1 + 0x4);
                *pbVar1 = *pbVar1 & 0xfe;
                pbVar1 = (local_14 * 0x5e + param_1 + 0x5);
                *pbVar1 = *pbVar1 & 0x7f;
                *(local_14 * 0x5e + param_1) = 0x0;
            }
        }
    }
    if (param_4 == 0x0) {
        local_14 = 0x0;
        while ((local_14 < 0x14 &&
            (((((local_14 * 0x5e + param_1 + 0x4) & 0x1) == 0x0 ||
                (*(*(local_14 * 0x5e + param_1) + 0x24) >> 0x18 == 0x31)) ||
                (*(*(&DAT_00582938 +
                    (*(*(local_14 * 0x5e + param_1) + 0x25) >> 0x18) * 0x4 +
                    (*(*(local_14 * 0x5e + param_1) + 0x24) >> 0x18) * 0x18) + 0xad) != 0x0)))))
        {
            local_14 = local_14 + 0x1;
        }
        if (local_14 == 0x14) {
            for (local_14 = 0x0; local_14 < 0x14; local_14 = local_14 + 0x1) {
                if ((((local_14 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) &&
                    (*(*(local_14 * 0x5e + param_1) + 0x24) >> 0x18 == 0x31)) {
                    FUN_0040fcb0(*(i32 ***)(local_14 * 0x5e + param_1),param_2,param_3);
                }
            }
        }
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00410497()

{
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0x14; local_14 = local_14 + 0x1) {
        if (((((&DAT_004c50bc + local_14 * 0x5e) & 0x1) != 0x0) &&
            (*(&DAT_004c50ce + local_14 * 0x5e) != 0x0)) &&
            (FUN_00488efd(*(&DAT_004c50b8 + local_14 * 0x5e),*(&DAT_004c50ce + local_14 * 0x5e)),
             ((&DAT_004c50bc + local_14 * 0x5e) & 0x8000) != 0x0)) {
            if (*(*(&DAT_004c50b8 + local_14 * 0x5e) + 0x24) >> 0x18 == 0x2d) {
                _DAT_004c6134 = _DAT_004c6134 + 0x1;
            }
            else {
                if (*(*(&DAT_00582938 +
                    (*(*(&DAT_004c50b8 + local_14 * 0x5e) + 0x25) >> 0x18) * 0x4 +
                    (*(*(&DAT_004c50b8 + local_14 * 0x5e) + 0x24) >> 0x18) * 0x18) + 0xa5)
                    != 0x0) {
                    FUN_00410b8a(&DAT_004c50b8,local_14,DAT_004c611c);
                }
            }
        }
        if (((((&DAT_004c58ec + local_14 * 0x5e) & 0x1) != 0x0) &&
            (*(&DAT_004c58fe + local_14 * 0x5e) != 0x0)) &&
            (FUN_00488efd(*(&DAT_004c58e8 + local_14 * 0x5e),*(&DAT_004c58fe + local_14 * 0x5e)),
             ((&DAT_004c58ec + local_14 * 0x5e) & 0x8000) != 0x0)) {
            if (*(*(&DAT_004c58e8 + local_14 * 0x5e) + 0x24) >> 0x18 == 0x2d) {
                DAT_004c6138 = DAT_004c6138 + 0x1;
            }
            else {
                if (*(*(&DAT_00582938 +
                    (*(*(&DAT_004c58e8 + local_14 * 0x5e) + 0x25) >> 0x18) * 0x4 +
                    (*(*(&DAT_004c58e8 + local_14 * 0x5e) + 0x24) >> 0x18) * 0x18) + 0xa5)
                    != 0x0) {
                    FUN_00410b8a(&DAT_004c58e8,local_14,DAT_004c611c);
                }
            }
        }
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00410758() -> i32

{
let mut iVar1: i32;
let mut local_14: i32;

DAT_004c6154 = 0x0;
_DAT_004c6150 = 0x0;
iVar1 = 0x0;
for (local_14 = 0x0; local_14 < 0x14; local_14 = local_14 + 0x1) {
if ((((&DAT_004c50bc + local_14 * 0x5e) & 0x8) != 0x0) &&
((*(&DAT_004be9b0 + (DAT_004c6100 >> 0x10) * 0x4) &
*(*(&DAT_004c50b8 + local_14 * 0x5e) + 0x3a)) == 0x0)) {
_DAT_004c6150 = _DAT_004c6150 + 0x1;
}
if ((((&DAT_004c58ec + local_14 * 0x5e) & 0x8) != 0x0) &&
((*(&DAT_004be9b0 + (DAT_004c58d0 >> 0x10) * 0x4) &
*(*(&DAT_004c58e8 + local_14 * 0x5e) + 0x3a)) == 0x0)) {
DAT_004c6154 = DAT_004c6154 + 0x1;
}
iVar1 = local_14;
}
return iVar1;
}



fn FUN_00410846()

{
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0x14; local_14 = local_14 + 0x1) {
    }
    return;
}



fn FUN_004109dc(param_1: i32,param_2: i32)

{
    let mut pcVar1: String;
    let puVar2: *mut u32;
    let mut uVar3: u32;
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0x14; local_14 = local_14 + 0x1) {
        if ((((local_14 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) &&
            (((local_14 * 0x5e + param_1 + 0x4) & 0x8000) == 0x0)) {
            if ((((local_14 * 0x5e + param_1 + 0x4) & 0x80) == 0x0) &&
                ((*(*(local_14 * 0x5e + param_1) + 0x2b) >> 0x18 < 0x2 &&
                    (uVar3 = FUN_004a2edc(), uVar3 % 0x64 < 0x19)))) {
                pcVar1 = (*(local_14 * 0x5e + param_1) + 0x2e);
                *pcVar1 = *pcVar1 + '\x01';
            }
            puVar2 = (*(local_14 * 0x5e + param_1) + 0x3a);
            *puVar2 = *puVar2 | *(&DAT_004be9b0 + param_2 * 0x4);
        }
    }
    return;
}



fn FUN_00410ae6(param_1: i32) -> u32

{
    let mut local_14: i32;

    local_14 = 0x0;
    while( true ) {
        if (0x13 < local_14) {
            return 0x1;
        }
        if ((((local_14 * 0x5e + param_1 + 0x4) & 0x1) != 0x0) &&
            (*(*(&DAT_00582938 +
                (*(*(local_14 * 0x5e + param_1) + 0x25) >> 0x18) * 0x4 +
                (*(*(local_14 * 0x5e + param_1) + 0x24) >> 0x18) * 0x18) + 0xad) == 0x0)) break;
        local_14 = local_14 + 0x1;
    }
    return 0x0;
}



fn FUN_00410b8a(param_1: i32,param_2: i32,param_3: i32)

{
    byte *pbVar1;
    let mut iVar2: i32;
    let mut local_14: i32;

    local_14 = 0x0;
    loop {
    if (*(*(&DAT_00582938 +
        (*(*(param_2 * 0x5e + param_1) + 0x25) >> 0x18) * 0x4 +
        (*(*(param_2 * 0x5e + param_1) + 0x24) >> 0x18) * 0x18) + 0xa5) <= local_14) {
        return;
    }
    if (*(*(param_2 * 0x5e + param_1) + local_14 * 0x4 + 0x10) != 0x0) {
        iVar2 = local_14 + param_2 + 0x1;
        if (*(iVar2 * 0x5e + param_1) == 0x0) {
            return;
        }
        if (((param_3 == 0x0) && (*(*(param_2 * 0x5e + param_1) + 0x24) >> 0x18 == 0x10)) &&
            (*(*(iVar2 * 0x5e + param_1) + 0x28) >> 0x10 == 0x5)) {
            FUN_0040a011(param_1,iVar2);
        }
        else {
            FUN_00484b4e(*(i32 **)(iVar2 * 0x5e + param_1));
            pbVar1 = (iVar2 * 0x5e + param_1 + 0x5);
            *pbVar1 = *pbVar1 | 0x80;
            pbVar1 = (iVar2 * 0x5e + param_1 + 0x4);
            *pbVar1 = *pbVar1 & 0xfe;
            *(iVar2 * 0x5e + param_1) = 0x0;
        }
    }
    local_14 = local_14 + 0x1;
} while( true );
}



fn FUN_00410ce7() -> u32

{
    let mut local_14: i32;

    local_14 = 0x0;
    while( true ) {
        if (0x13 < local_14) {
            return 0x1;
        }
        if ((((((&DAT_004c50bc + local_14 * 0x5e) & 0x8) != 0x0) &&
            (((&DAT_004c50bc + local_14 * 0x5e) & 0x8000) == 0x0)) &&
            (((&DAT_004c50bc + local_14 * 0x5e) & 0x80) == 0x0)) &&
            ((((&DAT_004c50bc + local_14 * 0x5e) & 0x1) != 0x0 &&
                (*(*(&DAT_00582938 +
                    (*(*(&DAT_004c50b8 + local_14 * 0x5e) + 0x25) >> 0x18) * 0x4 +
                    (*(*(&DAT_004c50b8 + local_14 * 0x5e) + 0x24) >> 0x18) * 0x18) + 0xad) ==
                    0x0)))) break;
        local_14 = local_14 + 0x1;
    }
    return 0x0;
}



fn FUN_00410e4d() -> u32

{
    let mut local_14: i32;

    local_14 = 0x0;
    while( true ) {
        if (0x13 < local_14) {
            return 0x1;
        }
        if ((((((&DAT_004c58ec + local_14 * 0x5e) & 0x8) != 0x0) &&
            (((&DAT_004c58ec + local_14 * 0x5e) & 0x8000) == 0x0)) &&
            (((&DAT_004c58ec + local_14 * 0x5e) & 0x80) == 0x0)) &&
            ((((&DAT_004c58ec + local_14 * 0x5e) & 0x1) != 0x0 &&
                (*(*(&DAT_00582938 +
                    (*(*(&DAT_004c58e8 + local_14 * 0x5e) + 0x25) >> 0x18) * 0x4 +
                    (*(*(&DAT_004c58e8 + local_14 * 0x5e) + 0x24) >> 0x18) * 0x18) + 0xad) ==
                    0x0)))) break;
        local_14 = local_14 + 0x1;
    }
    return 0x0;
}



// WARNING: Removing unreachable block (ram,0x00411151)

fn FUN_00410fb3(param_1: i32,param_2: u32) -> u32

{
    let mut pcVar1: String;
    let mut local_24: u32;
    let mut local_20: i32;

    if ((*(param_1 + 0x4) + 0x20) == DAT_004d557c) {
        if (DAT_004c9770 == -0x1) {
            if (param_2 == 0x3) {
                local_24 = 0x0;
            }
            else {
                if (param_2 == 0x2) {
                    if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
                        pcVar1 = FUN_00499050(DAT_0059679c,0x73f2);
                        FUN_0049d2e0(0x0,0x1,pcVar1);
                    }
                    local_24 = 0x0;
                }
                else {
                    for (local_20 = *(param_1 + 0x4); local_20 != 0x0; local_20 = *(local_20 + 0x8)) {
                        if (param_2 == 0x0) {
                            if ((local_20 + 0x27) != '\v') {
                                if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
                                    pcVar1 = FUN_00499050(DAT_0059679c,0x73f2);
                                    FUN_0049d2e0(0x0,0x1,pcVar1);
                                }
                                return 0x0;
                            }
                        }
                        else {
                            if ((0x1 < param_2) && (param_2 != 0x4)) {
                                return 0x0;
                            }
                            if (((local_20 + 0x27) != '1') && ((local_20 + 0x27) != '\x17')) {
                                if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
                                    pcVar1 = FUN_00499050(DAT_0059679c,0x73f2);
                                    FUN_0049d2e0(0x0,0x1,pcVar1);
                                }
                                return 0x0;
                            }
                        }
                    }
                    local_24 = 0x1;
                }
            }
        }
        else {
            local_24 = 0x1;
        }
    }
    else {
        local_24 = 0x1;
    }
    return local_24;
}



fn FUN_0041116a(param_1: i32,param_2: i32,param_3: i32)

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut uVar3: u32;

    if (DAT_004c6114 != 0x0) {
        iVar1 = *(DAT_004c6114 + 0xe) >> 0x10;
        iVar2 = FUN_004515ca(param_1,0x6);
        iVar1 = FUN_0040bd7e(*(param_2 * 0x5e + param_1),iVar2,param_3,iVar1);
        iVar1 = FUN_0040bc1e(iVar1,0x3e8);
        uVar3 = FUN_004a2edc();
        iVar1 = iVar1 * 0x4 + (uVar3 % 0xa) * 0x30;
        (DAT_004c6114 + 0x38) =
            (DAT_004c6114 + 0x38) +
                (CONCAT44(*(&DAT_00591964 + iVar1) >> 0x1f,*(&DAT_00591964 + iVar1)) / 0xa);
        iVar2 = *(DAT_004c6114 + 0x26) >> 0x10;
        iVar1 = *(DAT_004c6114 + 0x36) >> 0x10;
        if (iVar2 == iVar1 || iVar2 - iVar1 < 0x0) {
            *(DAT_004c6114 + 0x2d) = *(DAT_004c6114 + 0x2d) | 0x80;
        }
    }
    return;
}



fn FUN_00411242(param_1: i32,param_2: i32,param_3: i32) -> i32

{
let mut uVar1: u32;

uVar1 = *(&DAT_004daab0 + param_3 * 0x3890);
if (uVar1 < 0x3) {
if ((uVar1 == 0x2) && (((&DAT_00569ced)[param_2 * 0x1e22] & 0x1) != 0x0)) {
param_1 = param_1 + 0x2;
}
}
else {
if (uVar1 < 0x4) {
if (((&DAT_00569cf6)[param_2 * 0x1e22] & 0x1) != 0x0) {
param_1 = param_1 + 0x2;
}
}
else {
if ((uVar1 == 0x4) && (((&DAT_00569ce4)[param_2 * 0x1e22] & 0x1) != 0x0)) {
param_1 = param_1 + 0x2;
}
}
}
return param_1;
}



fn FUN_004112d1(param_1: i32,param_2: i32)

{
    let puVar1: *mut u32;
    let mut pcVar2: String;
    let mut uVar3: u32;
    let uVar4: u16;
    u32 local_bc [0x20];
    let mut local_3c: String;
    let mut local_38: String;
    let mut local_34: *mut u8;
    let mut local_30: i32;
    let mut local_2c: i32;
    let mut local_28: u32;
    let mut local_24: i32;
    let mut local_20: *mut u8;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: u32;

    local_24 = 0x0;
    local_20 = &DAT_004d55a8;
    local_1c = param_1;
    local_18 = param_2;
    local_14 = (byte)(&DAT_004d55a8)[param_1 * 0xe + param_2];
    if (local_14 == 0x0) {
        FUN_00462a28(&DAT_004d55a8,param_1,param_2,0x2);
        uVar4 = 0x1;
        uVar3 = 0xffffffff;
        puVar1 = FUN_00499050(DAT_0059679c,0x7396);
        FUN_0045518a(0x1 << ((byte)DAT_004c9754 & 0x1f),param_2,0x74cc,0xffffffff,puVar1,uVar3,uVar4);
        local_24 = 0x1;
    }
    else {
        local_34 = &DAT_004d55a8;
        local_30 = param_1;
        local_2c = param_2;
        local_28 = (byte)(&DAT_004d55a8)[param_1 * 0xe + param_2];
        if (local_28 == 0x1) {
            FUN_00462a28(&DAT_004d55a8,param_1,param_2,0x2);
            uVar4 = 0x1;
            uVar3 = 0xffffffff;
            puVar1 = FUN_00499050(DAT_0059679c,0x7396);
            FUN_0045518a(0x1 << ((byte)DAT_004c9754 & 0x1f),param_2,0x74cc,0xffffffff,puVar1,uVar3,uVar4);
            local_24 = 0x1;
        }
    }
    if (local_24 != 0x0) {
        FUN_00499050(DAT_0059679c,param_2 + 0x414);
        if (param_2 < 0x5) {
            local_3c = FUN_00499050(DAT_0059679c,0x713e);
        }
        else {
            local_3c = FUN_00499050(DAT_0059679c,0x713a);
        }
        FUN_00499050(DAT_0059679c,param_1 + 0x414);
        if (param_1 < 0x5) {
            local_38 = FUN_00499050(DAT_0059679c,0x713e);
        }
        else {
            local_38 = FUN_00499050(DAT_0059679c,0x7130);
        }
        pcVar2 = FUN_00499050(DAT_0059679c,0x7377);
        FUN_0049c2e0(local_bc,pcVar2);
        FUN_0045518a(0x1f,0xffffffff,0x74cc,0xffffffff,local_bc,0xffffffff,0x1);
    }
    return;
}


fn FUN_0045e9de()

{
    let mut bVar1: bool;
    let puVar2: *mut u32;
    let mut iVar3: i32;
    let local_24: *mut u32;
    let mut local_1c: i32;
    let mut local_18: i32;

    FUN_00486ba4(DAT_004c9754);
    local_1c = -0x1;
    local_18 = 0x0;
    bVar1 = false;
    DAT_00599e10 = 0x0;
    for (local_24 = *DAT_005967b0; local_24 != 0x0; local_24 = *local_24) {
        if (*(local_24 + 0x23) >> 0x18 == DAT_004c9754) {
            if ((local_24 + 0x8) != local_1c) {
                local_1c = (local_24 + 0x8);
                local_18 = FUN_00481d4f(local_1c);
            }
            *(local_24 + 0x3a) = *(local_24 + 0x3a) & 0xdf;
            *(local_24 + 0x3a) = *(local_24 + 0x3a) & 0xef;
            *(local_24 + 0x3b) = *(local_24 + 0x3b) & 0xbf;
            *(local_24 + 0x3b) = *(local_24 + 0x3b) & 0xfe;
            *(local_24 + 0x3a) = *(local_24 + 0x3a) & 0xfd;
            *(local_24 + 0xf) = *(local_24 + 0xf) & 0xfe;
            (local_24 + 0x47) = 0x0;
            (local_24 + 0x13) = 0x0;
            (local_24 + 0x2f) =

                (*(&DAT_00582938 +
                    (*(local_24 + 0x25) >> 0x18) * 0x4 + (local_24[0x9] >> 0x18) * 0x18) + 0x45);
            if (local_24[0xc] >> 0x18 < 0x64) {
                if ((*(local_24 + 0x3a) & 0x1) == 0x0) {
                    FUN_00488ec0(local_24,local_18 + 0x2);
                }
                else {
                    puVar2 = FUN_00481784((local_24 + 0x8),(local_24 + 0x22),
                                          (local_24 + 0x9));
                    if ((puVar2 == 0x0) || (iVar3 = FUN_00481a44(puVar2), iVar3 != 0x0)) {
                        FUN_00488ec0(local_24,unit_heal_00599db8 + local_18);
                    }
                    else {
                        FUN_00488ec0(local_24,unit_heal_in_city_00599db4 + local_18);
                    }
                }
            }
            if (((local_24 + 0x2a) == 0x3) && ((*(local_24 + 0x3a) & 0x40) == 0x0)) {
                puVar2 = FUN_00481784((local_24 + 0x8),(local_24 + 0x22),
                                      (local_24 + 0x9));
                if (puVar2 == 0x0) {
                    if ((*(local_24 + 0x3b) & 0x10) == 0x0) {
                        *(local_24 + 0x3b) = *(local_24 + 0x3b) | 0x10;
                    }
                    else {
                        puVar2 = FUN_00488613((local_24 + 0x8),(local_24 + 0x22),
                                              (local_24 + 0x9),0x1a);
                        if (puVar2 == 0x0) {
                            bVar1 = true;
                            *(local_24 + 0x3d) = *(local_24 + 0x3d) | 0x80;
                            _DAT_005b8be4 = 0x1;
                        }
                        else {
                            *(local_24 + 0x3b) = *(local_24 + 0x3b) & 0xef;
                        }
                    }
                }
                else {
                    iVar3 = FUN_00481a44(puVar2);
                    if (iVar3 == 0x0) {
                        *(local_24 + 0x3b) = *(local_24 + 0x3b) & 0xef;
                    }
                    else {
                        *(local_24 + 0x3b) = *(local_24 + 0x3b) | 0x10;
                    }
                }
            }
        }
    }
    if (bVar1) {
        FUN_0046fc49();
    }
    return;
}



fn FUN_0045ee03()

{
    let mut uVar1: u32;
    let mut local_110: *mut u32 [0x11];
    let ppuStack203: *mut *mut u8;;
    let mut local_1f: String;;
    let mut local_18: i32;

    for (local_18 = 0x0; local_18 < 0x3; local_18 = local_18 + 0x1) {
        uVar1 = FUN_0049c2c9(0xc350);
        (&DAT_005b2e3c)[local_18] = uVar1;
        FUN_0049c60b(s_bin_portrait_bin_004c2150,(&DAT_005b2e3c)[local_18],0xc350,
                     DAT_004c9754 * 0xc350 + local_18 * 0x3d090);
    }
    FUN_004990e0(local_110,0x0,s_efs_res_004c216a,s_Portrait_004c2161);
    FUN_0049bb50(local_110,FUN_0045ef35);
    FUN_0049af50(DAT_005b2e3c);
    FUN_0049af50(DAT_005b2e40);
    FUN_0049af50(DAT_005b2e44);
    ppuStack203 = &PTR_FUN_004c3d34;
    if (local_1f != 0x0) {
        FUN_00499b30(local_110,local_1f);
    }
    FUN_0049a1c0(local_110,0x1);
    return;
}



fn FUN_0045ef35(param_1: i32,param_2: u32,param_3: u32) -> u32

{
    let cVar1: u8;
    let mut pcVar2: String;
    let mut pcVar3: String;

    if (param_2 < 0x407) {
        if (param_2 == 0x405) {
            FUN_004953d7();
            FUN_00496ac0(DAT_005b2e3c,0x28,0x23,0xfa,0xc8);
            FUN_00496ac0(DAT_005b2e40,0x15e,0x23,0xfa,0xc8);
            FUN_00496ac0(DAT_005b2e44,0xc3,0x104,0xfa,0xc8);
            FUN_00497567(0xa5,0xf0,*(&DAT_004d7ba0 + DAT_004c9754 * 0xc),0xc8,0xcaccce,-0x1,0xcaccce,
                         LPCSTR_005b9218,0x11);
            FUN_00497567(0x1db,0xf0,*(&DAT_004d7ba4 + DAT_004c9754 * 0xc),0xc8,0xcaccce,-0x1,0xcaccce,
                         LPCSTR_005b9218,0x11);
            FUN_00497567(0x140,0x1d1,*(&DAT_004d7ba8 + DAT_004c9754 * 0xc),0xc8,0xcaccce,-0x1,0xcaccce,
                         LPCSTR_005b9218,0x11);
            FUN_0049536f();
        }
    }
    else {
        if (param_2 < 0x408) {
            if (param_3 < 0x1f5) {
                if (param_3 != 0x1f4) {
                    return 0x0;
                }
            }
            else {
                if ((0x1f5 < param_3) && (param_3 != 0x1f6)) {
                    return 0x0;
                }
            }
            *(&DAT_00569b60 + DAT_004c9754 * 0x1e22) = param_3 - 0x1f4;
            pcVar2 = *(&DAT_004d7ba0 + *(&DAT_00569b60 + DAT_004c9754 * 0x1e22) * 0x4 + DAT_004c9754 * 0xc);
            pcVar3 = &DAT_00569b50 + DAT_004c9754 * 0x1e22;
            loop {
                cVar1 = *pcVar2;
                *pcVar3 = cVar1;
                if (cVar1 == '\0') break;
                cVar1 = pcVar2[0x1];
                pcVar2 = pcVar2 + 0x2;
                pcVar3[0x1] = cVar1;
                pcVar3 = pcVar3 + 0x2;
            } while (cVar1 != '\0');
            FUN_0049c140(param_1,0x1);
        }
        else {
            if (param_2 == 0x40c) {
                FUN_004953d7();
                FUN_004a08c5(s_pcx_cathed3_pcx_004c2172,0x0,0x0,0x280,0x1e0,0x0,0x0,0x0,0x1);
                FUN_0049536f();
                return 0x1;
            }
        }
    }
    return 0x0;
}



fn FUN_0045f1a0()

{
    let mut local_11c: *mut u32 [0x11];
    let ppuStack215: *mut *mut u8;;
    let mut local_2b: String;;
    let local_24: *mut i32;;
    u32 **local_20;
    let mut local_1c: u32;

    DAT_005b2e3c = FUN_0049c2c9(0xc350);
    FUN_0049c60b(s_bin_portrait_bin_004c2182,DAT_005b2e3c,0xc350,
                 *(&DAT_00569b60 + DAT_004c9754 * 0x1e22) * 0x3d090 + DAT_004c9754 * 0xc350);
    local_24 = FUN_004990e0(local_11c,0x0,s_efs_res_004c219a,s_ChName_004c2193);
    FUN_0049bb50(local_11c,FUN_0045f290);
    FUN_0049af50(DAT_005b2e3c);
    local_20 = local_11c;
    local_1c = 0x0;
    ppuStack215 = &PTR_FUN_004c3d34;
    if (local_2b != 0x0) {
        FUN_00499b30(local_11c,local_2b);
    }
    FUN_0049a1c0(local_11c,0x1);
    return;
}



fn FUN_0045f290(param_1: *mut *mut u32,param_2: u32,param_3: i32) -> u32

{
    let mut iVar1: i32;

    if (param_2 < 0x405) {
        if (param_2 == 0x401) {
            iVar1 = FUN_0049c160(param_1,0xc8);
            *(iVar1 + 0x2d) = *(iVar1 + 0x2d) & 0xffffff7f;
            FUN_0049bf80(param_1,0xc8,0x502,0x10,&DAT_00569b50 + DAT_004c9754 * 0x1e22);
            return 0x0;
        }
    }
    else {
        if (param_2 < 0x406) {
            FUN_004953d7();
            FUN_00496ac0(DAT_005b2e3c,0x32,0x64,0xfa,0xc8);
            FUN_0049e640(0x32,0x64,0xfa,0xc8,0xce,0xca,0xcc,0x1);
            FUN_00496ac0((&DAT_00595700 + DAT_004c9754 * 0x4),0x15e,0x64,0x64,0x64);
            FUN_0049e640(0x15e,0x64,0x64,0x64,0xce,0xca,0xcc,0x1);
            FUN_0049536f();
        }
        else {
            if (0x406 < param_2) {
                if (param_2 < 0x408) {
                    if ((param_3 == 0x64) || (param_3 == 0xc8)) {
                        FUN_0049bf80(param_1,0xc8,0x501,0xf,&DAT_00569b50 + DAT_004c9754 * 0x1e22);
                        FUN_0049c140(param_1,0x1);
                    }
                    return 0x0;
                }
                if (param_2 == 0x40c) {
                    FUN_004953d7();
                    FUN_004a08c5(s_pcx_cathed3_pcx_004c21a2,0x0,0x0,0x280,0x1e0,0x0,0x0,0x0,0x1);
                    FUN_0049536f();
                    return 0x1;
                }
            }
        }
    }
    return 0x0;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0045f499(param_1: u32)

{
    let mut iVar1: i32;
    let local_22c: *mut u32;
    let local_1e0: *mut u32;
    let mut local_194: *mut u32 [0x11];
    let ppuStack335: *mut *mut u8;;
    let mut local_a3: String;;
    i32 aiStack156 [0x10];
    i32 aiStack92 [0x10];
    let mut local_1c: i32;
    let local_18: *mut i32;;

    _DAT_005b2e48 = param_1;
    local_18 = FUN_004990e0(local_194,0x0,s_efs_res_004c21bb,s_TraitDlg_004c21b2);
    for (local_1c = 0x0; local_1c < 0xf; local_1c = local_1c + 0x1) {
        local_1e0 = FUN_004a2831(0x5d);
        if (local_1e0 != 0x0) {
            local_1e0 = FUN_0049a030(local_1e0,local_194,local_1c + 0x1068,0x14,local_1c * 0x19 + 0x3c,0xf0,0x14,0x4002,
                                     0xcaccce,0xe0e0e);
            $1: &mut String(local_1e0 + 0x45) = &PTR_FUN_004c3d94;
            *(local_1e0 + 0x51) = 0x0;
            *(local_1e0 + 0x55) = 0x0;
            *(local_1e0 + 0x4d) = 0x0;
            *(local_1e0 + 0x49) = 0x2;
        }
        aiStack92[local_1c] = local_1e0;
        FUN_0049bf40(local_194,aiStack92[local_1c]);
    }
    for (local_1c = 0x0; local_1c < 0xb; local_1c = local_1c + 0x1) {
        local_22c = FUN_004a2831(0x5d);
        if (local_22c != 0x0) {
            local_22c = FUN_0049a030(local_22c,local_194,local_1c + 0x109a,0x154,local_1c * 0x19 + 0x3c,0xf0,0x14,0x4002,
                                     0xcaccce,0xe0e0e);
            $1: &mut String(local_22c + 0x45) = &PTR_FUN_004c3d94;
            *(local_22c + 0x51) = 0x0;
            *(local_22c + 0x55) = 0x0;
            *(local_22c + 0x4d) = 0x0;
            *(local_22c + 0x49) = 0x2;
        }
        aiStack156[local_1c] = local_22c;
        FUN_0049bf40(local_194,aiStack156[local_1c]);
    }
    FUN_0049bb50(local_194,FUN_0045fbe0);
    for (local_1c = 0x0; local_1c < 0xf; local_1c = local_1c + 0x1) {
        if ((aiStack92[local_1c] != 0x0) && (iVar1 = aiStack92[local_1c], iVar1 != 0x0)) {
            ((*(iVar1 + 0x45) + 0x8))(iVar1,0x2);
        }
    }
    for (local_1c = 0x0; local_1c < 0xb; local_1c = local_1c + 0x1) {
        if ((aiStack156[local_1c] != 0x0) && (iVar1 = aiStack156[local_1c], iVar1 != 0x0)) {
            ((*(iVar1 + 0x45) + 0x8))(iVar1,0x2);
        }
    }
    ppuStack335 = &PTR_FUN_004c3d34;
    if (local_a3 != 0x0) {
        FUN_00499b30(local_194,local_a3);
    }
    FUN_0049a1c0(local_194,0x1);
    return;
}



fn FUN_0045f9d1()

{
    let local_20: u8 [0x10];

    FUN_004968e7(0xf5,0x1c7,0x28,0x10,0xe);
    FUN_0049e640(0xf5,0x1c7,0x28,0x10,0xce,0xca,0xcc,0x1);
    FUN_0049e640(0xf5,0x1c7,0x28,0x10,0xce,0xca,0xcc,0x2);
    FUN_0049c2e0(local_20,&DAT_004c21c3);
    FUN_00497567(0x109,0x1c8,local_20,0x28,0xcaccce,-0x1,0xcaccce,LPCSTR_005b9218,0x11);
    return;
}



fn FUN_0045fa9c()

{
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0xf; local_14 = local_14 + 0x1) {
        if (((&DAT_00569b68)[local_14 * 0x3 + DAT_004c9754 * 0x1e22] & 0x1) == 0x0) {
            FUN_00496ee6(&DAT_00591f30,0x19,local_14 * 0x19 + 0x39,0x19,0x19);
        }
        else {
            FUN_00496ee6(&DAT_00591cb8,0x19,local_14 * 0x19 + 0x39,0x19,0x19);
        }
    }
    for (local_14 = 0x0; local_14 < 0xb; local_14 = local_14 + 0x1) {
        if (((&DAT_00569b98)[local_14 * 0x3 + DAT_004c9754 * 0x1e22] & 0x1) == 0x0) {
            FUN_00496ee6(&DAT_00591f30,0x159,local_14 * 0x19 + 0x39,0x19,0x19);
        }
        else {
            FUN_00496ee6(&DAT_00591cb8,0x159,local_14 * 0x19 + 0x39,0x19,0x19);
        }
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0045fbe0(param_1: *mut *mut u32,param_2: u32,param_3: i32,param_4: u32) -> u32

{
    let mut bVar1: bool;
    let mut pcVar2: String;
    let mut puVar3: *mut u8;
    let mut iVar4: i32;
    let mut uVar5: u32;
    let mut iVar6: i32;
    let mut uVar7: u32;
    let puVar8: *mut u32;
    let bVar9: u8;
    let mut local_20: i32;
    let mut local_1c: i32;

    if (param_2 < 0x405) {
        if (0x1ff < param_2) {
            if (param_2 < 0x201) {
                for (local_20 = 0x0; local_20 < 0xf; local_20 = local_20 + 0x1) {
                    puVar3 = FUN_0049bf80(param_1,local_20 + 0x1068,0x40b,param_3,param_4);
                    if (puVar3 != 0x0) {
                        if (local_20 + 0x1068 != _DAT_005b2e18) {
                            _DAT_005b2e18 = local_20 + 0x1068;
                            pcVar2 = FUN_00499050(DAT_0059679c,local_20 + 0x107c);
                            FUN_0049bf80(param_1,0xc0,0x502,0x0,pcVar2);
                        }
                        return 0x0;
                    }
                }
                local_20 = 0x0;
                while( true ) {
                    if (0xa < local_20) {
                        if (_DAT_005b2e18 != -0x1) {
                            _DAT_005b2e18 = -0x1;
                            FUN_0049bf80(param_1,0xc0,0x502,0x0,&DAT_004c21d6);
                        }
                        return 0x0;
                    }
                    puVar3 = FUN_0049bf80(param_1,local_20 + 0x109a,0x40b,param_3,param_4);
                    if (puVar3 != 0x0) break;
                    local_20 = local_20 + 0x1;
                }
                if (local_20 + 0x109a != _DAT_005b2e18) {
                    _DAT_005b2e18 = local_20 + 0x109a;
                    pcVar2 = FUN_00499050(DAT_0059679c,local_20 + 0x10ae);
                    FUN_0049bf80(param_1,0xc0,0x502,0x0,pcVar2);
                }
                return 0x0;
            }
            if (param_2 == 0x401) {
                if (_DAT_005b2e48 == 0x0) {
                    FUN_0049bf80(param_1,0x3e7,0x414,0x0,0x0);
                    FUN_0049bf80(param_1,0x3e7,0x410,0x0,0x0);
                }
                else {
                    FUN_0049bf80(param_1,0x3e8,0x414,0x0,0x0);
                    FUN_0049bf80(param_1,0x3e8,0x410,0x0,0x0);
                    FUN_0049bf80(param_1,0x3e9,0x414,0x0,0x0);
                    FUN_0049bf80(param_1,0x3e9,0x410,0x0,0x0);
                }
                _DAT_005b2e18 = 0xffffffff;
                return 0x0;
            }
        }
    }
    else {
        if (param_2 < 0x406) {
            FUN_004953d7();
            for (local_1c = 0x0; local_1c < 0xf; local_1c = local_1c + 0x1) {
                bVar9 = 0x10;
                uVar7 = 0xcaccce;
                iVar6 = -0x1;
                uVar5 = 0xcaccce;
                iVar4 = 0xc8;
                puVar8 = LPCSTR_005b9218;
                pcVar2 = FUN_00499050(DAT_0059679c,local_1c + 0x1068);
                FUN_00497567(0x37,local_1c * 0x19 + 0x3e,pcVar2,iVar4,uVar5,iVar6,uVar7,puVar8,bVar9);
            }
            for (local_1c = 0x0; local_1c < 0xb; local_1c = local_1c + 0x1) {
                bVar9 = 0x10;
                uVar7 = 0xcaccce;
                iVar6 = -0x1;
                uVar5 = 0xcaccce;
                iVar4 = 0xc8;
                puVar8 = LPCSTR_005b9218;
                pcVar2 = FUN_00499050(DAT_0059679c,local_1c + 0x109a);
                FUN_00497567(0x177,local_1c * 0x19 + 0x3e,pcVar2,iVar4,uVar5,iVar6,uVar7,puVar8,bVar9);
            }
            if (_DAT_005b2e48 == 0x0) {
                FUN_0045f9d1();
            }
            FUN_0045fa9c();
            FUN_0049536f();
        }
        else {
            if (param_2 < 0x40c) {
                if (param_2 == 0x407) {
                    if ((param_3 < 0x1068) || (0x1077 < param_3)) {
                        if ((param_3 < 0x109a) || (0x10a9 < param_3)) {
                            if (param_3 == 0x64) {
                                FUN_0049c140(param_1,0x1);
                            }
                        }
                        else {
                            if (_DAT_005b2e48 != 0x0) {
                                return 0x0;
                            }
                            bVar1 = false;
                            if (((&DAT_00569b98)[(param_3 + -0x109a) * 0x3 + DAT_004c9754 * 0x1e22] & 0x1) == 0x0) {
                                (&DAT_00569b98)[(param_3 + -0x109a) * 0x3 + DAT_004c9754 * 0x1e22] =
                                    (&DAT_00569b98)[(param_3 + -0x109a) * 0x3 + DAT_004c9754 * 0x1e22] | 0x11;
                                *(&DAT_00569b64 + DAT_004c9754 * 0x1e22) = *(&DAT_00569b64 + DAT_004c9754 * 0x1e22) + 0x1;
                                bVar1 = true;
                            }
                            else {
                                if (*(&DAT_00569b64 + DAT_004c9754 * 0x1e22) == 0x0) {
                                    return 0x0;
                                }
                                (&DAT_00569b98)[(param_3 + -0x109a) * 0x3 + DAT_004c9754 * 0x1e22] =
                                    (&DAT_00569b98)[(param_3 + -0x109a) * 0x3 + DAT_004c9754 * 0x1e22] & 0xfe;
                                *(&DAT_00569b64 + DAT_004c9754 * 0x1e22) = *(&DAT_00569b64 + DAT_004c9754 * 0x1e22) + -0x1
                                ;
                            }
                            FUN_004953d7();
                            FUN_0045f9d1();
                            FUN_0045fa9c();
                            FUN_0049536f();
                            if (bVar1) {
                                FUN_00483355(0x2);
                            }
                        }
                    }
                    else {
                        if (_DAT_005b2e48 != 0x0) {
                            return 0x0;
                        }
                        if (((&DAT_00569b68)[(param_3 + -0x1068) * 0x3 + DAT_004c9754 * 0x1e22] & 0x1) == 0x0) {
                            if (*(&DAT_00569b64 + DAT_004c9754 * 0x1e22) == 0x0) {
                                FUN_00483355(0x1);
                                return 0x0;
                            }
                            (&DAT_00569b68)[(param_3 + -0x1068) * 0x3 + DAT_004c9754 * 0x1e22] =
                                (&DAT_00569b68)[(param_3 + -0x1068) * 0x3 + DAT_004c9754 * 0x1e22] | 0x11;
                            *(&DAT_00569b64 + DAT_004c9754 * 0x1e22) = *(&DAT_00569b64 + DAT_004c9754 * 0x1e22) + -0x1;
                        }
                        else {
                            (&DAT_00569b68)[(param_3 + -0x1068) * 0x3 + DAT_004c9754 * 0x1e22] =
                                (&DAT_00569b68)[(param_3 + -0x1068) * 0x3 + DAT_004c9754 * 0x1e22] & 0xfe;
                            *(&DAT_00569b64 + DAT_004c9754 * 0x1e22) = *(&DAT_00569b64 + DAT_004c9754 * 0x1e22) + 0x1;
                        }
                        FUN_004953d7();
                        FUN_0045f9d1();
                        FUN_0045fa9c();
                        FUN_0049536f();
                    }
                }
            }
            else {
                if (param_2 < 0x40d) {
                    FUN_004953d7();
                    FUN_004a08c5(s_pcx_cathed3_pcx_004c21c6,0x0,0x0,0x280,0x1e0,0x0,0x0,0x0,0x1);
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



char  FUN_00460210(param_1: i32,char **param_2)

{
let mut uVar1: u32;
let mut uVar2: u32;

uVar1 = FUN_004a7970((param_1 + 0x65),0x1,0x1,param_2);
uVar2 = FUN_004a7970((param_1 + 0x66),0x1,0x1,param_2);
return (uVar1 == 0x0) + (uVar2 == 0x0);
}



char  FUN_00460296(param_1: i32,param_2: *mut i32)

{
let mut uVar1: u32;
let mut uVar2: u32;

uVar1 = FUN_004a7160((param_1 + 0x65),0x1,0x1,param_2);
uVar2 = FUN_004a7160((param_1 + 0x66),0x1,0x1,param_2);
return (uVar1 == 0x0) + (uVar2 == 0x0);
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0046031c(param_1: i32,param_2: i32) -> i32

{
let mut iVar1: i32;
i32 aiStackY612 [0x75];
let local_84: *mut u32;
let mut local_80: i32;
i32 local_64 [0xd];
let mut local_30: i32;
let local_2c: *mut u32;
let mut local_28: i32;
let mut local_24: i32;
let mut local_20: i32;
let mut local_1c: i32;
let mut local_18: i32;
let local_14: *mut u32;

if (((&DAT_00569a98)[param_1 * 0x1e22] & 0x1) == 0x0) {
local_24 = 0x0;
FUN_004a0430(local_64,0x0,0x34);
for (local_2c = *DAT_005967b0; local_2c != 0x0; local_2c = *local_2c) {
if ((*(local_2c + 0x23) >> 0x18 == param_1) && ((local_2c + 0x27) == '[')) {
local_64[*(local_2c + 0x2d) >> 0x18] =
local_64[*(local_2c + 0x2d) >> 0x18] + (*(local_2c + 0x2f) >> 0x10);
}
}
for (local_28 = 0x0; local_28 < 0xd; local_28 = local_28 + 0x1) {
local_24 = local_24 + *(&DAT_00569bcc + param_1 * 0x1e22 + local_28 * 0x8) * local_64[local_28];
}
local_24 = (local_24 + *(&DAT_00569abd + param_1 * 0x1e22)) / 0x3e8 +
*(&DAT_00569aad + param_1 * 0x1e22) * 0xa;
for (local_28 = 0x1; local_28 < 0x72; local_28 = local_28 + 0x1) {
if (((&DAT_00569c30)[local_28 * 0x9 + param_1 * 0x1e22] & 0x1) != 0x0) {
local_24 = local_24 + 0x5;
}
}
local_20 = 0x0;
for (local_2c = *DAT_005967b0; local_2c != 0x0; local_2c = *local_2c) {
if (*(local_2c + 0x23) >> 0x18 == param_1) {
local_1c = 0x0;
for (local_28 = 0x0; local_28 < 0xd; local_28 = local_28 + 0x1) {
local_1c = local_1c +
*(&DAT_00569bcc + local_28 * 0x8 + param_1 * 0x1e22) *
*(local_28 * 0x4 +
*(&DAT_00582938 +
(*(local_2c + 0x25) >> 0x18) * 0x4 +
(local_2c[0x9] >> 0x18) * 0x18) + 0xb9);
}
local_20 = local_20 + ((local_2c + 0x29) * local_1c) / 0x64;
}
}
local_18 = 0x0;
for (local_28 = 0x0; local_28 < 0xd; local_28 = local_28 + 0x1) {
local_18 = local_18 +
*(&DAT_00569bcc + local_28 * 0x8 + param_1 * 0x1e22) *
*(local_28 * 0x4 + DAT_00582e30 + 0xb9);
}
for (local_14 = *DAT_005967c8; local_14 != 0x0; local_14 = *local_14) {
if (*(local_14 + 0xe) >> 0x10 == param_1) {
local_20 = local_20 + ((local_14 + 0x8) * local_18) / 0x64;
}
}
local_24 = local_24 + local_20 / 0x1f4;
if (param_2 != 0x0) {
local_24 = local_24 + 0x32;
local_80 = -0x1;
for (local_84 = *DAT_005967c8; local_84 != 0x0; local_84 = *local_84) {
if ((*(local_84 + 0x6) >> 0x10 != local_80) &&
(((local_84 + 0x4) == 0x8 || ((local_84 + 0x4) == 0x7)))) {
local_24 = local_24 + -0xa;
local_80 = *(local_84 + 0x6) >> 0x10;
}
}
}
iVar1 = (local_24 * 0x1e) / 0x64;
if (((&DAT_00569a98)[param_1 * 0x1e22] & 0x20) != 0x0) {
local_24 = local_24 - iVar1;
}
if (_DAT_004c9764 != 0x0) {
local_24 = local_24 - iVar1;
}
local_30 = ((*(&DAT_00569b4c + param_1 * 0x1e22) * 0x14 + 0x3c) * local_24) / 0x64;
if (0x7fff < local_30) {
local_30 = 0x7fff;
}
}
else {
local_30 = 0x0;
}
return local_30;
}



fn FUN_004606b6(param_1: i32)

{
    let cVar1: u8;
    let mut iVar2: i32;
    let mut pcVar3: String;
    let puVar4: *mut u32;
    let mut pcVar5: String;
    let mut local_d4: i32;
    let mut local_d0: u32;
    let local_cc: *mut u32;
    let local_c8: *mut u32;
    let mut local_c4: i32;
    let mut local_c0: u32;
    let local_bc: *mut u32;
    let local_b8: *mut u32;
    let mut local_b4: u32;
    let local_b0: *mut u32;
    let local_ac: *mut u32;
    let mut local_a8: i32;
    let mut local_a4: i32;
    let local_a0: *mut u32;
    let local_9c: *mut u32;
    let local_98: *mut u32;
    u32 local_94 [0x20];
    let local_14: *mut u32;
    let puVar6: *mut u32;

    for (local_14 = *DAT_005967c8; local_14 != 0x0; local_14 = *local_14) {
        if ((*(local_14 + 0xe) >> 0x10 == param_1) && ((*(local_14 + 0x2d) & 0x1) == 0x0)) {
            *(local_14 + 0x6) = 0xffff;
            *(local_14 + 0x22) = 0xffff;
            *(local_14 + 0x9) = 0xffff;
            *(local_14 + 0x5) = 0xffff;
            *(local_14 + 0x16) = 0xffff;
            *(local_14 + 0x26) = 0xffff;
            *(local_14 + 0x4) = 0xd;
            *(local_14 + 0x12) = 0xd;
            FUN_00482992((local_14 + 0x2),0x4b);
        }
    }
    for (local_98 = *DAT_005967b0; local_98 != 0x0; local_98 = *local_98) {
        if (*(local_98 + 0x23) >> 0x18 == param_1) {
            local_a0 = local_98 + 0x8;
            if (((local_98 + 0x35) == -0x1) ||
                ((local_98 + 0x26) == (local_98 + 0x35))) {
                local_a4 = 0x0;
            }
            else {
                local_a4 = 0x1;
            }
            local_a8 = local_a4;
            local_9c = local_a0;
            if (local_a4 == 0x0) {
                (local_98 + 0x26) = 0xd;
                local_b0 = local_98 + 0x8;
                local_b4 = *(local_98 + 0x3a) & 0x1;
                local_ac = local_b0;
                if (local_b4 == 0x0) {
                    local_bc = local_98 + 0x8;
                    local_c0 = local_bc & 0xffff0000 | local_bc;
                    local_c4 = *(&DAT_005b7076 + local_bc * 0x4e) >> 0x10;
                    local_cc = local_98 + 0x8;
                    local_d0 = local_cc & 0xffff0000 | local_cc;
                    local_d4 = *(&DAT_005b7078 + local_cc * 0x4e) >> 0x10;
                    local_c8 = local_cc;
                    local_b8 = local_bc;
                    iVar2 = FUN_004729ba(0xd,&local_c4,&local_d4,0x0);
                    *(local_98 + 0x8) = *(local_98 + 0x8);
                    (local_98 + 0x22) = local_c4;
                    (local_98 + 0x9) = local_d4;
                    *(&DAT_005b707e + (local_98 + 0x8) * 0x4e + iVar2 * 0x4) = 0x0;
                }
                (local_98 + 0x12) = 0x0;
                (local_98 + 0x49) = 0xff;
                *(local_98 + 0x4a) = 0xfffd;
                (local_98 + 0x42) = 0xff;
                (local_98 + 0x41) = (local_98 + 0x42);
            }
        }
    }
        (&DAT_00569a98)[param_1 * 0x1e22] = (&DAT_00569a98)[param_1 * 0x1e22] | 0x1;
    if (((&DAT_00569a98)[param_1 * 0x1e22] & 0x2) == 0x0) {
        *(&DAT_00569a9d + param_1 * 0x1e22) = 0x1;
    }
    FUN_00499050(DAT_0059679c,param_1 + 0x414);
    pcVar3 = FUN_00499050(DAT_0059679c,0x73cc);
    FUN_0049c2e0(local_94,pcVar3);
    pcVar3 = FUN_00499050(DAT_0059679c,0x7361);
    iVar2 = -0x1;
    puVar4 = local_94;
    loop {
    puVar6 = puVar4;
    if (iVar2 == 0x0) break;
    iVar2 = iVar2 + -0x1;
    puVar6 = (puVar4 + 0x1);
    cVar1 = puVar4;
    puVar4 = puVar6;
} while (cVar1 != '\0');
    pcVar5 = (puVar6 + -0x1);
    loop {
    cVar1 = *pcVar3;
    *pcVar5 = cVar1;
    if (cVar1 == '\0') break;
    cVar1 = pcVar3[0x1];
    pcVar3 = pcVar3 + 0x2;
    pcVar5[0x1] = cVar1;
    pcVar5 = pcVar5 + 0x2;
} while (cVar1 != '\0');
    FUN_0045518a(0x1f,0xffffffff,0x74cc,0xffffffff,local_94,0xffffffff,0x0);
    return;
}



fn FUN_00460b04(byte *param_1,param_2: i32,ushort *param_3) -> u32

{
    byte *pbVar1;

    pbVar1 = param_1 + param_2;
    for (; param_1 < pbVar1; param_1 = param_1 + 0x1) {
        *param_3 = *param_3 << 0x8 ^ (DAT_005b2e34 + (*param_1 ^ *param_3 >> 0x8) * 0x2);
    }
    return param_3 & 0xffff0000 | *param_3;
}



fn FUN_00460b74() -> u32

{
    let mut uVar1: u32;
    undefined2 *puVar2;
    let mut iVar3: i32;
    let mut local_18: i32;

    DAT_005b2e30 = 0x0;
    if (DAT_005b2e34 == 0x0) {
        DAT_005b2e34 = FUN_004266f4(0x1021);
        if (DAT_005b2e34 == 0x0) {
            pop_err_msg_box_and_exit_004a02f5(s_Memory_Error__004c21d8);
        }
    }
    FUN_00460b04(&turns_till_vote_opt_00599d44,0x4,&DAT_005b2e30);
    FUN_00460b04(&turns_till_patriarch_dies_00599d48,0x4,&DAT_005b2e30);
    FUN_00460b04(&normal_damage_opt_00599d4c,0x4,&DAT_005b2e30);
    FUN_00460b04(&feint_damage_opt_00599d50,0x4,&DAT_005b2e30);
    FUN_00460b04(&loyalty_noble_bonus_00599d54,0x4,&DAT_005b2e30);
    FUN_00460b04(&loyalty_officer_bonus_00599d58,0x4,&DAT_005b2e30);
    FUN_00460b04(&default_leadership_00599d5c,0x4,&DAT_005b2e30);
    FUN_00460b04(&starting_credits_opts_00599d60,0x4,&DAT_005b2e30);
    FUN_00460b04(&dflt_tax_rate_00599d64,0x4,&DAT_005b2e30);
    FUN_00460b04(&dflt_tithe_rate_00599d68,0x4,&DAT_005b2e30);
    FUN_00460b04(&give_back_res_per_opt_00599d6c,0x4,&DAT_005b2e30);
    FUN_00460b04(&dflt_church_like_00599d70,0x4,&DAT_005b2e30);
    FUN_00460b04(&dflt_league_like_00599d74,0x4,&DAT_005b2e30);
    FUN_00460b04(&dflt_interest_rate_00599d78,0x4,&DAT_005b2e30);
    FUN_00460b04(&dflt_loan_turns_00599d7c,0x4,&DAT_005b2e30);
    FUN_00460b04(&pct_asset_is_loan_00599d80,0x4,&DAT_005b2e30);
    FUN_00460b04(&min_loan_amt_00599d84,0x4,&DAT_005b2e30);
    FUN_00460b04(&shield_radius_00599d9c,0x4,&DAT_005b2e30);
    FUN_00460b04(&health_check_for_plague_00599da4,0x4,&DAT_005b2e30);
    FUN_00460b04(&health_loss_for_famine_00599da8,0x4,&DAT_005b2e30);
    FUN_00460b04(&immune_plague_bonus_00599dac,0x4,&DAT_005b2e30);
    FUN_00460b04(&city_heal_rate_00599db0,0x4,&DAT_005b2e30);
    FUN_00460b04(&unit_heal_in_city_00599db4,0x4,&DAT_005b2e30);
    FUN_00460b04(&unit_heal_00599db8,0x4,&DAT_005b2e30);
    FUN_00460b04(&third_republic_min_00599dbc,0x4,&DAT_005b2e30);
    FUN_00460b04(&third_republic_max_00599dc0,0x4,&DAT_005b2e30);
    FUN_00460b04(&famine_damage_00599dc4,0x4,&DAT_005b2e30);
    FUN_00460b04(&not_enough_garrison_00599d40,0x4,&DAT_005b2e30);
    FUN_00460b04(&credits_received_00582928,0x4,&DAT_005b2e30);
    FUN_00460b04(&per_tech_received_0058292c,0x4,&DAT_005b2e30);
    FUN_00460b04(&recv_unfound_city_00582930,0x4,&DAT_005b2e30);
    FUN_00460b04(&promise_votes_00582934,0x4,&DAT_005b2e30);
    FUN_00460b04(&league_hall_stock_00599d88,0x4,&DAT_005b2e30);
    FUN_00460b04(&league_like_up_00599d8c,0x4,&DAT_005b2e30);
    FUN_00460b04(&league_like_down_00599d90,0x4,&DAT_005b2e30);
    FUN_00460b04(&league_int_rate_up_00599d94,0x4,&DAT_005b2e30);
    FUN_00460b04(&league_int_rate_down_00599d98,0x4,&DAT_005b2e30);
    FUN_00460b04(&DAT_004d5584,0x4,&DAT_005b2e30);
    for (puVar2 = 0x0; puVar2 < 0x5c; puVar2 = (puVar2 + 0x1)) {
        for (local_18 = 0x0; local_18 < 0x6; local_18 = local_18 + 0x1) {
            if (*(&DAT_00582938 + local_18 * 0x4 + puVar2 * 0x18) != 0x0) {
                puVar2 = &DAT_005b2e30;
                FUN_00460b04(*(byte **)(local_18 * 0x4 + 0x8e47db8),0x129,&DAT_005b2e30);
            }
        }
    }
    FUN_00460b04(&DAT_005831d8,0xa00,&DAT_005b2e30);
    FUN_00460b04(&DAT_004d6a70,0x888,&DAT_005b2e30);
    FUN_00460b04(&DAT_0058aca8,0x6114,&DAT_005b2e30);
    iVar3 = 0x0;
    while (iVar3 < 0x5c) {
        FUN_00460b04(0xa466d9c,0x4,&DAT_005b2e30);
        FUN_00460b04(0xa466da0,0x4,&DAT_005b2e30);
        FUN_00460b04(0xa466da4,0x4,&DAT_005b2e30);
        FUN_00460b04(0xa466da8,0x4,&DAT_005b2e30);
        FUN_00460b04(0xa466dac,0x4,&DAT_005b2e30);
        puVar2 = &DAT_005b2e30;
        FUN_00460b04(0xa466db0,0x4,&DAT_005b2e30);
        iVar3 = puVar2 + 0x1;
    }
    FUN_00460b04(&DAT_00590dc0,0x4b0,&DAT_005b2e30);
    FUN_00460b04(&DAT_00591270,0x514,&DAT_005b2e30);
    FUN_00460b04(&DAT_00591964,0x1e0,&DAT_005b2e30);
    FUN_00460b04(&DAT_00583c28,0x7080,&DAT_005b2e30);
    FUN_00460b04(&DAT_00591b44,0x168,&DAT_005b2e30);
    uVar1 = FUN_00460b04(&DAT_00591784,0x1e0,&DAT_005b2e30);
    return uVar1 & 0xffff0000 | DAT_005b2e30;
}



fn FUN_00461135(byte *param_1) -> u32

{
    ushort *puVar1;
    let mut local_14: u32;

    local_14 = 0x0;
    if (DAT_005b2e34 == 0x0) {
        DAT_005b2e34 = FUN_004266f4(0x1021);
        if (DAT_005b2e34 == 0x0) {
            pop_err_msg_box_and_exit_004a02f5(s_Memory_Error__004c21e6);
        }
    }
    for (; *param_1 != 0x0; param_1 = param_1 + 0x1) {
        puVar1 = (((local_14 & 0xffff) >> 0x8 ^ *param_1) * 0x2 + DAT_005b2e34);
        local_14 = (puVar1 & 0xffff0000 | *puVar1) ^ local_14 << 0x8;
    }
    return local_14;
}



fn FUN_004611cf(param_1: i32,char **param_2,param_3: i32) -> i32

{
let mut uVar1: u32;
let mut local_c0: u32;
let mut local_bc: u32;
let mut local_b8: u32;
let mut local_b4: u32;
let mut local_b0: u32;
let mut local_ac: u32;
let mut local_a8: u32;
let mut local_a4: u32;
let mut local_a0: u32;
let mut local_9c: u32;
let mut local_98: u32;
let local_94: u8;
let mut local_90: u32;
let mut local_8c: u32;
let mut local_88: u32;
let mut local_84: u32;
let mut local_80: u32;
let mut local_7c: u32;
let mut local_78: u32;
let mut local_74: u32;
let mut local_70: u32;
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
let mut local_1c: i32;
let mut local_18: i32;
let mut local_14: u32;

local_1c = 0x0;
uVar1 = FUN_004a7970((param_1 + 0x1888),0x1,0x1,param_2);
local_14 = (uVar1 == 0x0);
local_1c = local_1c + local_14;
if (param_3 < 0xea9f1) {
*(param_1 + 0x1895) = 0x0;
*(param_1 + 0x1899) = 0x0;
}
else {
uVar1 = FUN_004a7970((param_1 + 0x1895),0x4,0x1,param_2);
local_28 = (uVar1 == 0x0);
local_1c = local_1c + local_28;
uVar1 = FUN_004a7970((param_1 + 0x1899),0x4,0x1,param_2);
local_24 = (uVar1 == 0x0);
local_1c = local_1c + local_24;
}
uVar1 = FUN_004a7970((param_1 + 0x189d),0x4,0x1,param_2);
local_68 = (uVar1 == 0x0);
local_1c = local_1c + local_68;
uVar1 = FUN_004a7970((param_1 + 0x18a1),0x4,0x1,param_2);
local_64 = (uVar1 == 0x0);
local_1c = local_1c + local_64;
uVar1 = FUN_004a7970((param_1 + 0x18a5),0x4,0x1,param_2);
local_60 = (uVar1 == 0x0);
local_1c = local_1c + local_60;
uVar1 = FUN_004a7970((param_1 + 0x18a9),0x4,0x1,param_2);
local_5c = (uVar1 == 0x0);
local_1c = local_1c + local_5c;
uVar1 = FUN_004a7970((param_1 + 0x18ad),0x4,0x1,param_2);
local_58 = (uVar1 == 0x0);
local_1c = local_1c + local_58;
uVar1 = FUN_004a7970((param_1 + 0x18b1),0x4,0x1,param_2);
local_54 = (uVar1 == 0x0);
local_1c = local_1c + local_54;
uVar1 = FUN_004a7970((param_1 + 0x18b5),0x4,0x1,param_2);
local_50 = (uVar1 == 0x0);
local_1c = local_1c + local_50;
uVar1 = FUN_004a7970((param_1 + 0x18b9),0x4,0x1,param_2);
local_4c = (uVar1 == 0x0);
local_1c = local_1c + local_4c;
uVar1 = FUN_004a7970((param_1 + 0x18bd),0x4,0x1,param_2);
local_48 = (uVar1 == 0x0);
local_1c = local_1c + local_48;
uVar1 = FUN_004a7970((param_1 + 0x18c1),0x3,0x1,param_2);
local_44 = (uVar1 == 0x0);
local_1c = local_1c + local_44;
uVar1 = FUN_004a7970((param_1 + 0x18c4),0x4,0x1,param_2);
local_40 = (uVar1 == 0x0);
local_1c = local_1c + local_40;
uVar1 = FUN_004a7970((param_1 + 0x18c8),0x4,0x1,param_2);
local_3c = (uVar1 == 0x0);
local_1c = local_1c + local_3c;
uVar1 = FUN_004a7970((param_1 + 0x18cc),0x4,0x1,param_2);
local_38 = (uVar1 == 0x0);
local_1c = local_1c + local_38;
uVar1 = FUN_004a7970((param_1 + 0x18d0),0x4,0x1,param_2);
local_34 = (uVar1 == 0x0);
local_1c = local_1c + local_34;
uVar1 = FUN_004a7970((param_1 + 0x18d4),0x4,0x1,param_2);
local_30 = (uVar1 == 0x0);
local_1c = local_1c + local_30;
uVar1 = FUN_004a7970((param_1 + 0x18d8),0x4,0x1,param_2);
local_2c = (uVar1 == 0x0);
local_1c = local_1c + local_2c;
if (param_3 < 0xea9a0) {
*(param_1 + 0x18dc) = dflt_tax_rate_00599d64;
*(param_1 + 0x18e0) = 0x64;
}
else {
uVar1 = FUN_004a7970((param_1 + 0x18dc),0x4,0x1,param_2);
local_70 = (uVar1 == 0x0);
local_1c = local_1c + local_70;
uVar1 = FUN_004a7970((param_1 + 0x18e0),0x4,0x1,param_2);
local_6c = (uVar1 == 0x0);
local_1c = local_1c + local_6c;
}
uVar1 = FUN_004a7970((param_1 + 0x18e4),0x4,0x1,param_2);
local_90 = (uVar1 == 0x0);
local_1c = local_1c + local_90;
uVar1 = FUN_004a7970((param_1 + 0x18e8),0x4,0x1,param_2);
local_8c = (uVar1 == 0x0);
local_1c = local_1c + local_8c;
uVar1 = FUN_004a7970((param_1 + 0x18ec),0x4,0x1,param_2);
local_88 = (uVar1 == 0x0);
local_1c = local_1c + local_88;
uVar1 = FUN_004a7970((param_1 + 0x18f0),0x4,0x1,param_2);
local_84 = (uVar1 == 0x0);
local_1c = local_1c + local_84;
uVar1 = FUN_004a7970((param_1 + 0x18f4),0x4,0x1,param_2);
local_80 = (uVar1 == 0x0);
local_1c = local_1c + local_80;
uVar1 = FUN_004a7970((param_1 + 0x18f8),0x4,0x1,param_2);
local_7c = (uVar1 == 0x0);
local_1c = local_1c + local_7c;
uVar1 = FUN_004a7970((param_1 + 0x18fc),0x4,0x1,param_2);
local_78 = (uVar1 == 0x0);
local_1c = local_1c + local_78;
uVar1 = FUN_004a7970((param_1 + 0x1900),0x4,0x1,param_2);
local_74 = (uVar1 == 0x0);
local_1c = local_1c + local_74;
if (param_3 < 0xeaa5b) {
for (local_18 = 0x0; local_18 < 0xe; local_18 = local_18 + 0x1) {
uVar1 = FUN_004a7970(&local_94,0x4,0x1,param_2);
local_98 = (uVar1 == 0x0);
local_1c = local_1c + local_98;
FUN_00462a28(&DAT_004d55a8,*(param_1 + 0x1891),local_18,local_94);
}
}
for (local_18 = 0x0; local_18 < 0xe; local_18 = local_18 + 0x1) {
uVar1 = FUN_004a7970((local_18 * 0x4 + param_1 + 0x1904),0x4,0x1,param_2);
local_9c = (uVar1 == 0x0);
local_1c = local_1c + local_9c;
}
uVar1 = FUN_004a7970((param_1 + 0x193c),0x4,0x1,param_2);
local_a0 = (uVar1 == 0x0);
local_1c = local_1c + local_a0;
uVar1 = FUN_004a7970((param_1 + 0x1940),0x10,0x1,param_2);
local_a4 = (uVar1 == 0x0);
local_1c = local_1c + local_a4;
uVar1 = FUN_004a7970((param_1 + 0x1950),0x4,0x1,param_2);
local_a8 = (uVar1 == 0x0);
local_1c = local_1c + local_a8;
uVar1 = FUN_004a7970((param_1 + 0x1954),0x4,0x1,param_2);
local_ac = (uVar1 == 0x0);
local_1c = local_1c + local_ac;
uVar1 = FUN_004a7970((param_1 + 0x1958),0x30,0x1,param_2);
local_b0 = (uVar1 == 0x0);
local_1c = local_1c + local_b0;
uVar1 = FUN_004a7970((param_1 + 0x1988),0x30,0x1,param_2);
local_b4 = (uVar1 == 0x0);
local_1c = local_1c + local_b4;
for (local_18 = 0x0; local_18 < 0xd; local_18 = local_18 + 0x1) {
uVar1 = FUN_004a7970((local_18 * 0x8 + param_1 + 0x19b8),0x8,0x1,param_2);
local_b8 = (uVar1 == 0x0);
local_1c = local_1c + local_b8;
}
for (local_18 = 0x0; local_18 < 0x28; local_18 = local_18 + 0x1) {
uVar1 = FUN_004a7970((local_18 * 0x9d + param_1),0x9d,0x1,param_2);
local_bc = (uVar1 == 0x0);
local_1c = local_1c + local_bc;
}
for (local_18 = 0x0; local_18 < 0x72; local_18 = local_18 + 0x1) {
uVar1 = FUN_004a7970((local_18 * 0x9 + param_1 + 0x1a20),0x9,0x1,param_2);
local_c0 = (uVar1 == 0x0);
local_1c = local_1c + local_c0;
}
return local_1c;
}



fn FUN_00461b77(param_1: i32,param_2: *mut i32) -> i32

{
let mut uVar1: u32;
let mut local_b8: u32;
let mut local_b4: u32;
let mut local_b0: u32;
let mut local_ac: u32;
let mut local_a8: u32;
let mut local_a4: u32;
let mut local_a0: u32;
let mut local_9c: u32;
let mut local_98: u32;
let mut local_94: u32;
let mut local_8c: i32;
let mut local_88: i32;
let mut local_84: u32;
let mut local_80: u32;
let mut local_7c: u32;
let mut local_78: u32;
let mut local_74: u32;
let mut local_70: u32;
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
let mut local_18: u32;
let mut local_14: u32;

uVar1 = FUN_004a7160((param_1 + 0x1888),0x1,0x1,param_2);
local_84 = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x1895),0x4,0x1,param_2);
local_80 = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x1899),0x4,0x1,param_2);
local_7c = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x189d),0x4,0x1,param_2);
local_78 = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x18a1),0x4,0x1,param_2);
local_74 = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x18a5),0x4,0x1,param_2);
local_70 = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x18a9),0x4,0x1,param_2);
local_6c = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x18ad),0x4,0x1,param_2);
local_68 = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x18b1),0x4,0x1,param_2);
local_64 = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x18b5),0x4,0x1,param_2);
local_60 = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x18b9),0x4,0x1,param_2);
local_5c = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x18bd),0x4,0x1,param_2);
local_58 = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x18c1),0x3,0x1,param_2);
local_54 = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x18c4),0x4,0x1,param_2);
local_50 = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x18c8),0x4,0x1,param_2);
local_4c = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x18cc),0x4,0x1,param_2);
local_48 = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x18d0),0x4,0x1,param_2);
local_44 = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x18d4),0x4,0x1,param_2);
local_40 = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x18d8),0x4,0x1,param_2);
local_3c = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x18dc),0x4,0x1,param_2);
local_38 = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x18e0),0x4,0x1,param_2);
local_34 = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x18e4),0x4,0x1,param_2);
local_30 = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x18e8),0x4,0x1,param_2);
local_2c = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x18ec),0x4,0x1,param_2);
local_28 = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x18f0),0x4,0x1,param_2);
local_24 = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x18f4),0x4,0x1,param_2);
local_20 = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x18f8),0x4,0x1,param_2);
local_1c = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x18fc),0x4,0x1,param_2);
local_18 = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x1900),0x4,0x1,param_2);
local_14 = (uVar1 == 0x0);
local_8c = local_84 + local_80 + local_7c + local_78 + local_74 + local_70 + local_6c + local_68 + local_64 + local_60
+ local_5c + local_58 + local_54 + local_50 + local_4c + local_48 + local_44 + local_40 + local_3c +
local_38 + local_34 + local_30 + local_2c + local_28 + local_24 + local_20 + local_1c + local_18 + local_14
;
for (local_88 = 0x0; local_88 < 0xe; local_88 = local_88 + 0x1) {
uVar1 = FUN_004a7160((local_88 * 0x4 + param_1 + 0x1904),0x4,0x1,param_2);
local_94 = (uVar1 == 0x0);
local_8c = local_8c + local_94;
}
uVar1 = FUN_004a7160((param_1 + 0x193c),0x4,0x1,param_2);
local_ac = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x1940),0x10,0x1,param_2);
local_a8 = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x1950),0x4,0x1,param_2);
local_a4 = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x1954),0x4,0x1,param_2);
local_a0 = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x1958),0x30,0x1,param_2);
local_9c = (uVar1 == 0x0);
uVar1 = FUN_004a7160((param_1 + 0x1988),0x30,0x1,param_2);
local_98 = (uVar1 == 0x0);
local_8c = local_8c + local_ac + local_a8 + local_a4 + local_a0 + local_9c + local_98;
for (local_88 = 0x0; local_88 < 0xd; local_88 = local_88 + 0x1) {
uVar1 = FUN_004a7160((local_88 * 0x8 + param_1 + 0x19b8),0x8,0x1,param_2);
local_b0 = (uVar1 == 0x0);
local_8c = local_8c + local_b0;
}
for (local_88 = 0x0; local_88 < 0x28; local_88 = local_88 + 0x1) {
uVar1 = FUN_004a7160((local_88 * 0x9d + param_1),0x9d,0x1,param_2);
local_b4 = (uVar1 == 0x0);
local_8c = local_8c + local_b4;
}
for (local_88 = 0x0; local_88 < 0x72; local_88 = local_88 + 0x1) {
uVar1 = FUN_004a7160((local_88 * 0x9 + param_1 + 0x1a20),0x9,0x1,param_2);
local_b8 = (uVar1 == 0x0);
local_8c = local_8c + local_b8;
}
return local_8c;
}



fn FUN_004624f3(param_1: i32) -> u32

{
    let mut local_14: i32;

    local_14 = 0x1;
    while( true ) {
        if (0x71 < local_14) {
            return 0x0;
        }
        if ((*(local_14 * 0x9 + param_1 + 0x1a20) & 0x1) != 0x0) break;
        local_14 = local_14 + 0x1;
    }
    return 0x1;
}



fn FUN_00462543(param_1: i32)

{
    *(param_1 + 0x18e4) = 0x0;
    *(param_1 + 0x18ec) = 0x0;
    return;
}



fn FUN_00462571(param_1: i32,param_2: i32,param_3: i32) -> i32

{
let local_18: *mut u32;
let mut local_14: i32;

local_14 = 0x0;
if (DAT_004c9760 == 0x0) {
for (local_18 = (&DAT_005b8b44 + param_2 * 0x4);
(local_18 != 0x0 && ((local_18 + 0x8) == param_2)); local_18 = *local_18)
{
if (((local_18 + 0x27) == '[') &&
((*(local_18 + 0x2d) >> 0x18 == param_3 &&
(*(local_18 + 0x23) >> 0x18 == *(param_1 + 0x1891))))) {
local_14 = local_14 + (*(local_18 + 0x2f) >> 0x10);
}
}
}
else {
for (local_18 = *DAT_005967b0; local_18 != 0x0; local_18 = *local_18) {
if ((((local_18 + 0x27) == '[') && (*(local_18 + 0x2d) >> 0x18 == param_3)) &&
(*(local_18 + 0x23) >> 0x18 == *(param_1 + 0x1891))) {
local_14 = local_14 + (*(local_18 + 0x2f) >> 0x10);
}
}
}
return local_14;
}



fn FUN_00462679(param_1: i32,param_2: i32)

{
    *(param_2 * 0x8 + param_1 + 0x19bc) = (*(param_2 * 0x8 + param_1 + 0x19b8) * 0xd + 0x9) / 0xa;
    if (*(param_2 * 0x8 + param_1 + 0x19bc) == 0x0) {
        *(param_2 * 0x8 + param_1 + 0x19bc) = 0x1;
    }
    return;
}



fn FUN_004626e0()

{
    return;
}



fn FUN_004626f4(param_1: i32,param_2: i32,param_3: i32,param_4: i32)

{
    let piVar1: *mut i32;;
    let mut iVar2: i32;

    if (DAT_00591cb4 < 0x2) {
        return;
    }
    if (param_3 == 0x0) {
        if (param_4 == 0x0) {
            param_3 = 0x1;
            param_4 = 0x1;
        }
        else {
            if (param_4 < 0x4) {
                param_3 = 0x1;
            }
            else {
                param_3 = param_4 / 0x2;
            }
        }
    }
    if ((param_3 * 0xb < param_4 * 0xa) || (param_4 < param_3)) {
        if (param_4 < param_3) {
            iVar2 = (*(&DAT_004d6a70 + param_2 * 0xa8) * 0xc) / 0xa - *(param_2 * 0x8 + param_1 + 0x19b8);
            if (iVar2 == 0x0) {
                return;
            }
            iVar2 = ((param_3 - param_4) * iVar2) / param_3;
            if (iVar2 == 0x0) {
                piVar1 = (param_2 * 0x8 + param_1 + 0x19b8);
                *piVar1 = *piVar1 + 0x1;
            }
            else {
                piVar1 = (param_2 * 0x8 + param_1 + 0x19b8);
                *piVar1 = *piVar1 + iVar2;
            }
        }
        else {
            iVar2 = *(param_2 * 0x8 + param_1 + 0x19b8) - (*(&DAT_004d6a70 + param_2 * 0xa8) << 0x3) / 0xa;
            if (iVar2 == 0x0) {
                return;
            }
            iVar2 = ((param_4 - param_3) * iVar2) / param_4;
            if (iVar2 == 0x0) {
                piVar1 = (param_2 * 0x8 + param_1 + 0x19b8);
                *piVar1 = *piVar1 + -0x1;
            }
            else {
                piVar1 = (param_2 * 0x8 + param_1 + 0x19b8);
                *piVar1 = *piVar1 - iVar2;
            }
        }
    }
    else {
        iVar2 = *(&DAT_004d6a70 + param_2 * 0xa8) - *(param_2 * 0x8 + param_1 + 0x19b8);
        if (iVar2 / 0x2 == 0x0) {
            if (iVar2 == 0x0) {
                return;
            }
            piVar1 = (param_2 * 0x8 + param_1 + 0x19b8);
            *piVar1 = *piVar1 + iVar2;
        }
        else {
            piVar1 = (param_2 * 0x8 + param_1 + 0x19b8);
            *piVar1 = *piVar1 + iVar2 / 0x2;
        }
    }
    if (*(param_2 * 0x8 + param_1 + 0x19b8) < 0x0) {
        *(param_2 * 0x8 + param_1 + 0x19b8) = 0x0;
        *(param_2 * 0x8 + param_1 + 0x19bc) = 0x1;
    }
    else {
        FUN_00462679(param_1,param_2);
    }
    return;
}



bool  FUN_0046292a(param_1: i32)

{
return (*(param_1 + 0x1c60) & 0x1) != 0x0;
}



fn FUN_0046295d(param_1: i32,param_2: i32) -> u32

{
    let mut local_14: u32;

    if (*(param_2 * 0x4 + param_1 + 0xb6) < 0x384) {
        local_14 = *(param_2 * 0x4 + param_1 + 0xb6);
    }
    else {
        local_14 = 0x0;
    }
    return local_14;
}



fn FUN_004629a4(param_1: i32) -> i32

{
FUN_004629cd(param_1);
return param_1;
}



fn FUN_004629cd(param_1: i32)

{
    let mut local_18: i32;
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0xe; local_14 = local_14 + 0x1) {
        for (local_18 = 0x0; local_18 < 0xe; local_18 = local_18 + 0x1) {
            (local_14 * 0xe + param_1 + local_18) = (&DAT_004bf640)[local_14 * 0xe + local_18];
        }
    }
    return;
}



fn FUN_00462a28(param_1: i32,param_2: i32,param_3: i32,undefined param_4)

{
    let mut puVar1: *mut u8;

    puVar1 = (param_2 + param_3 * 0xe + param_1);
    *puVar1 = param_4;
    (param_2 * 0xe + param_1 + param_3) = *puVar1;
    return;
}



fn FUN_00462a5b(param_1: i32,param_2: *mut i32) -> i32

{
let mut uVar1: u32;
let mut local_24: u32;
let mut local_20: i32;
let mut local_18: i32;
let mut local_14: i32;

local_18 = 0x0;
for (local_14 = 0x0; local_14 < 0xe; local_14 = local_14 + 0x1) {
for (local_20 = 0x0; local_20 < 0xe; local_20 = local_20 + 0x1) {
uVar1 = FUN_004a7160((local_14 * 0xe + param_1 + local_20),0x1,0x1,param_2);
local_24 = (uVar1 == 0x0);
local_18 = local_18 + local_24;
}
}
return local_18;
}



fn FUN_00462ae2(param_1: i32,char **param_2) -> i32

{
let mut uVar1: u32;
let mut local_24: u32;
let mut local_20: i32;
let mut local_18: i32;
let mut local_14: i32;

local_18 = 0x0;
for (local_14 = 0x0; local_14 < 0xe; local_14 = local_14 + 0x1) {
for (local_20 = 0x0; local_20 < 0xe; local_20 = local_20 + 0x1) {
uVar1 = FUN_004a7970((local_14 * 0xe + param_1 + local_20),0x1,0x1,param_2);
local_24 = (uVar1 == 0x0);
local_18 = local_18 + local_24;
}
}
return local_18;
}



fn FUN_00462b69(param_1: *mut u32)

{
    *param_1 = 0x0;
    (param_1 + 0x1) = 0x0;
    return;
}



fn FUN_00462b8d(param_1: *mut i32,param_2: u32,undefined param_3) -> u32

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut local_24: u32;
    let mut local_20: i32;

    if (*param_1 == 0x78) {
        local_24 = 0x0;
    }
    else {
        *param_1 = *param_1 + 0x1;
        local_20 = *param_1;
        while( true ) {
            iVar1 = local_20 >> 0x1;
            if (*(param_1 + iVar1 * 0x5 + 0x4) <= (byte)param_2) break;
            iVar2 = iVar1 * 0x5 + param_1;
            iVar3 = local_20 * 0x5 + param_1;
            *(iVar3 + 0x4) = *(iVar2 + 0x4);
            (iVar3 + 0x8) = (iVar2 + 0x8);
            local_20 = iVar1;
        }
        iVar1 = local_20 * 0x5 + param_1;
        *(iVar1 + 0x4) = param_2;
        (iVar1 + 0x8) = param_3;
        local_24 = 0x1;
    }
    return local_24;
}



i32 *  FUN_00462c31(param_1: *mut i32)

{
byte *pbVar1;
let uVar2: u8;
let mut uVar3: u32;
let unaff_ESI: *mut i32;;
let mut iVar4: i32;
let mut iVar5: i32;
let local_34: u8;
let mut local_2c: i32;
let uStack40: u8;
let mut local_24: i32;
let mut local_20: i32;

if (*param_1 == 0x0) {
local_2c = param_1[0x1];
uStack40 = (param_1 + 0x2);
}
else {
local_2c = *(param_1 + 0x9);
uStack40 = (param_1 + 0xd);
iVar4 = *param_1;
*param_1 = *param_1 + -0x1;
iVar4 = iVar4 * 0x5 + param_1;
uVar3 = *(iVar4 + 0x4);
uVar2 = (iVar4 + 0x8);
local_24 = 0x1;
while (local_20 = local_24 * 0x2, local_20 <= *param_1) {
if ((local_20 != *param_1) &&
(*(param_1 + (local_20 + 0x1) * 0x5 + 0x4) < *(param_1 + local_24 * 0xa + 0x4))) {
local_20 = local_20 + 0x1;
}
local_34 = (byte)uVar3;
pbVar1 = (param_1 + local_20 * 0x5 + 0x4);
if (local_34 < *pbVar1 || local_34 == *pbVar1) break;
iVar4 = local_20 * 0x5 + param_1;
iVar5 = local_24 * 0x5 + param_1;
*(iVar5 + 0x4) = *(iVar4 + 0x4);
(iVar5 + 0x8) = (iVar4 + 0x8);
local_24 = local_20;
}
iVar4 = local_24 * 0x5 + param_1;
*(iVar4 + 0x4) = uVar3;
(iVar4 + 0x8) = uVar2;
}
*unaff_ESI = local_2c;
(unaff_ESI + 0x1) = uStack40;
return unaff_ESI;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00462d47()

{
    let mut bVar1: bool;
    undefined3 extraout_var;
    let mut iVar2: i32;
    let mut pcVar3: String;
    let mut local_a4: i32;
    u32 local_a0 [0x20];
    let mut local_20: i32;
    let local_1c: *mut u32;
    let mut local_18: i32;
    let mut local_14: i32;

    DAT_00599d3c = 0x1;
    _DAT_005b3ec4 = 0x0;
    DAT_005b3ed4 = 0x0;
    DAT_005b3ed8 = 0x0;
    DAT_005b3edc = 0x0;
    _DAT_005b3ee0 = 0x0;
    DAT_005b3ed0 = 0x0;
    DAT_005b3ec8 = 0x0;
    DAT_005b3ee4 = 0x0;
    DAT_005b3ef4 = 0x0;
    DAT_005b3ee8 = 0x0;
    local_20 = -0x1;
    FUN_00489246(DAT_004c9754,0x0);
    FUN_0046363e();
    FUN_00463f7b();
    FUN_004a0430(&DAT_005b36a4,0x0,0x820);
    FUN_00420edd(&DAT_005b2e84);
    local_18 = -0x1;
    for (local_1c = *DAT_005967c8; local_1c != 0x0; local_1c = *local_1c) {
        if (*(local_1c + 0xe) >> 0x10 == DAT_004c9754) {
            if (*(local_1c + 0x6) >> 0x10 != local_18) {
                local_18 = *(local_1c + 0x6) >> 0x10;
                if (((DAT_004d7860 >> 0x18 == DAT_004c9754) &&
                    (bVar1 = FUN_0046292a((&DAT_00568210 + DAT_004c9754 * 0x1e22)), CONCAT31(extraout_var,bVar1) != 0x0))
                    && (iVar2 = FUN_00488757(*(local_1c + 0x6) >> 0x10,0x2), iVar2 != 0x0)) {
                    DAT_005b3ecc = DAT_004d77fe;
                    local_20 = *(local_1c + 0x6) >> 0x10;
                    DAT_005b3ee4 = DAT_005b3ee4 + DAT_004d77fe;
                }
                else {
                    DAT_005b3ecc = 0x0;
                }
            }
            FUN_00464119(local_1c);
        }
    }
    for (local_14 = 0x0; local_14 < 0x28; local_14 = local_14 + 0x1) {
        for (local_a4 = 0x0; local_a4 < 0xd; local_a4 = local_a4 + 0x1) {
            if (*(&DAT_005b36a4 + local_a4 * 0x4 + local_14 * 0x34) != 0x0) {
                FUN_00465d24(local_14,local_a4,*(&DAT_005b36a4 + local_14 * 0x34 + local_a4 * 0x4),DAT_004c9754);
            }
        }
    }
    if ((((*(DAT_004c9754 * 0x1e22 + 0x569b6e) & 0x1) != 0x0) && (DAT_005b3edc != 0x0)) &&
        ((*(DAT_004c9754 * 0x1e22 + 0x569b6e) & 0x10) != 0x0)) {
        FUN_00499050(DAT_0059679c,0x106a);
        pcVar3 = FUN_00499050(DAT_0059679c,0x6916);
        FUN_0049c2e0(local_a0,pcVar3);
        FUN_0045518a(0x1 << ((byte)DAT_004c9754 & 0x1f),0xffffffff,0x106a,0xffffffff,local_a0,0xffffffff,0x0);
    }
    if ((((*(DAT_004c9754 * 0x1e22 + 0x569b6b) & 0x1) != 0x0) && (DAT_005b3ed4 != 0x0)) &&
        ((*(DAT_004c9754 * 0x1e22 + 0x569b6b) & 0x10) != 0x0)) {
        FUN_00499050(DAT_0059679c,0x1069);
        pcVar3 = FUN_00499050(DAT_0059679c,0x6915);
        FUN_0049c2e0(local_a0,pcVar3);
        FUN_0045518a(0x1 << ((byte)DAT_004c9754 & 0x1f),0xffffffff,0x1069,0xffffffff,local_a0,0xffffffff,0x0);
    }
    if (((((&DAT_00569b68)[DAT_004c9754 * 0x1e22] & 0x1) != 0x0) && (DAT_005b3ed8 != 0x0)) &&
        (((&DAT_00569b68)[DAT_004c9754 * 0x1e22] & 0x10) != 0x0)) {
        FUN_00499050(DAT_0059679c,0x109a);
        pcVar3 = FUN_00499050(DAT_0059679c,0x6946);
        FUN_0049c2e0(local_a0,pcVar3);
        FUN_0045518a(0x1 << ((byte)DAT_004c9754 & 0x1f),0xffffffff,0x109a,0xffffffff,local_a0,0xffffffff,0x0);
    }
    if ((((*(DAT_004c9754 * 0x1e22 + 0x569b6e) & 0x1) != 0x0) && (DAT_005b3ed0 != 0x0)) &&
        ((*(DAT_004c9754 * 0x1e22 + 0x569b6e) & 0x10) != 0x0)) {
        FUN_00499050(DAT_0059679c,0x109c);
        pcVar3 = FUN_00499050(DAT_0059679c,0x6948);
        FUN_0049c2e0(local_a0,pcVar3);
        FUN_0045518a(0x1 << ((byte)DAT_004c9754 & 0x1f),0xffffffff,0x109c,0xffffffff,local_a0,0xffffffff,0x0);
    }
    if ((((*(DAT_004c9754 * 0x1e22 + 0x569b74) & 0x1) != 0x0) && (_DAT_005b3ee0 != 0x0)) &&
        ((*(DAT_004c9754 * 0x1e22 + 0x569b74) & 0x10) != 0x0)) {
        FUN_00499050(DAT_0059679c,0x106c);
        pcVar3 = FUN_00499050(DAT_0059679c,0x6918);
        FUN_0049c2e0(local_a0,pcVar3);
        FUN_0045518a(0x1 << ((byte)DAT_004c9754 & 0x1f),0xffffffff,0x106c,0xffffffff,local_a0,0xffffffff,0x0);
    }
    if (DAT_005b3ee4 != 0x0) {
        pcVar3 = FUN_00499050(DAT_0059679c,0x7456);
        FUN_0049c2e0(local_a0,pcVar3);
        FUN_0045518a(0x1 << ((byte)DAT_004c9754 & 0x1f),0xffffffff,0x74cc,local_20,local_a0,0xffffffff,0x0);
    }
    if (DAT_005b3ef4 != 0x0) {
        pcVar3 = FUN_00499050(DAT_0059679c,0x745b);
        FUN_0049c2e0(local_a0,pcVar3);
        FUN_0045518a(0x1 << ((byte)DAT_004c9754 & 0x1f),0xffffffff,0x74cc,local_20,local_a0,0xffffffff,0x0);
    }
    if (DAT_005b3ee8 != 0x0) {
        pcVar3 = FUN_00499050(DAT_0059679c,0x745c);
        FUN_0049c2e0(local_a0,pcVar3);
        FUN_0045518a(0x1 << ((byte)DAT_004c9754 & 0x1f),0xffffffff,0x74cc,local_20,local_a0,0xffffffff,0x0);
    }
    DAT_00599d3c = 0x0;
    FUN_004864f7();
    if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
        FUN_0046672b();
    }
    return;
}



fn FUN_0046363e()

{
    let cVar1: u8;
    let mut bVar2: bool;
    let puVar3: *mut u32;
    let mut iVar4: i32;
    undefined3 extraout_var;
    let mut pcVar5: String;
    let mut iVar6: i32;
    let mut uVar7: u32;
    let puVar8: *mut u32;
    let mut pcVar9: String;
    let mut uVar10: u32;
    let uVar11: u16;
    let mut local_434: i32;
    u32 local_42c [0x100];
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let local_18: *mut u32;
    let mut local_14: i32;

    local_2c = 0x0;
    for (local_28 = 0x1; local_28 < 0x72; local_28 = local_28 + 0x1) {
        if ((((&DAT_00569c30)[local_28 * 0x9 + DAT_004c9754 * 0x1e22] & 0x1) != 0x0) &&
            (*(&DAT_0058ad6a + local_28 * 0xda) != 0x0)) {
            local_2c = local_2c + *(&DAT_0058ad6a + local_28 * 0xda);
        }
    }
    local_2c = local_2c / 0x64;
    if (*(&DAT_00569ab1 + DAT_004c9754 * 0x1e22) == 0x0) {
        bVar2 = false;
        for (local_28 = 0x1; local_28 < 0x72; local_28 = local_28 + 0x1) {
            if ((local_28 != 0x44) && (((&DAT_00569c30)[local_28 * 0x9 + DAT_004c9754 * 0x1e22] & 0x1) != 0x0)) {
                bVar2 = true;
                break;
            }
        }
        if (bVar2) {
            uVar11 = 0x0;
            uVar10 = 0xffffffff;
            puVar3 = FUN_00499050(DAT_0059679c,0x736a);
            FUN_0045518a(0x1 << ((byte)DAT_004c9754 & 0x1f),0xffffffff,0x74cc,0xffffffff,puVar3,uVar10,uVar11);
            (&DAT_00569c30)[DAT_004c9754 * 0x1e22] = (&DAT_00569c30)[DAT_004c9754 * 0x1e22] & 0xf7;
            for (local_28 = 0x1; local_28 < 0x72; local_28 = local_28 + 0x1) {
                (&DAT_00569c30)[local_28 * 0x9 + DAT_004c9754 * 0x1e22] =
                    (&DAT_00569c30)[local_28 * 0x9 + DAT_004c9754 * 0x1e22] & 0xfb;
                (&DAT_00569c30)[DAT_004c9754 * 0x1e22 + local_28 * 0x9] =
                    (&DAT_00569c30)[DAT_004c9754 * 0x1e22 + local_28 * 0x9] & 0xfe;
            }
            FUN_00463f7b();
        }
    }
    else {
        local_24 = *(&DAT_00569ab1 + DAT_004c9754 * 0x1e22);
        local_20 = local_2c / local_24;
        local_1c = 0x0;
        for (local_18 = *DAT_005967c8; local_18 != 0x0; local_18 = *local_18) {
            if (((local_18 + 0xe) == 0x17) && (*(local_18 + 0xe) >> 0x10 == DAT_004c9754)) {
                iVar4 = FUN_0046645d(*(local_18 + 0x6) >> 0x10,local_18[0x2] >> 0x10,
                                     *(local_18 + 0xa) >> 0x10);
                local_434 = (iVar4 * 0x64) / 0x64;
                if (0x0 < local_434) {
                    if (((DAT_004d7a63 >> 0x18 == DAT_004c9754) &&
                        (bVar2 = FUN_0046292a((&DAT_00568210 + DAT_004c9754 * 0x1e22)), CONCAT31(extraout_var,bVar2) != 0x0))
                        && (iVar4 = FUN_00489176(*(local_18 + 0x6) >> 0x10,local_18[0x2] >> 0x10,
                                                 *(local_18 + 0xa) >> 0x10,0x7), iVar4 != 0x0)) {
                        DAT_005b3ec8 = (local_434 * DAT_004d7a01) / 0x64;
                        DAT_005b3ef8 = *(local_18 + 0x6) >> 0x10;
                        DAT_005b3ef4 = DAT_005b3ec8;
                    }
                    DAT_005b3ec8 = DAT_005b3ec8 + ((*(&DAT_00569b6c + DAT_004c9754 * 0x1e22) >> 0x18) * local_434) / 0x64;
                    DAT_005b3edc = DAT_005b3edc + (*(&DAT_00569b6c + DAT_004c9754 * 0x1e22) >> 0x18);
                    local_434 = local_434 + DAT_005b3ec8;
                }
                (local_18 + 0x1a) = local_434;
                (local_18 + 0x1a) = (local_18 + 0x1a) - local_20;
                if (local_18[0x6] >> 0x10 < 0x0) {
                    local_1c = local_1c - (local_18[0x6] >> 0x10);
                    *(local_18 + 0x1a) = 0x0;
                    local_24 = local_24 + -0x1;
                }
            }
        }
        if (local_1c != 0x0) {
            local_14 = 0x1;
            while (0x0 < local_1c) {
                if (local_24 == 0x0) {
                    pcVar5 = FUN_00499050(DAT_0059679c,0x7154);
                    FUN_0049c2e0(local_42c,pcVar5);
                    iVar4 = local_1c;^
                    // goto LAB_004639f9;
                }
                if (local_1c < 0x1) {
                    return;
                }
                local_20 = local_1c / local_24;
                local_1c = 0x0;
                for (local_18 = *DAT_005967c8; local_18 != 0x0; local_18 = *local_18)
                {
                    if (((((local_18 + 0xe) == 0x17) && (*(local_18 + 0xe) >> 0x10 == DAT_004c9754)) &&
                        ((local_18 + 0x1a) != 0x0)) &&
                        ((local_18 + 0x1a) = (local_18 + 0x1a) - local_20,
                         local_18[0x6] >> 0x10 < 0x0)) {
                        local_1c = local_1c - (local_18[0x6] >> 0x10);
                        *(local_18 + 0x1a) = 0x0;
                        local_24 = local_24 + -0x1;
                    }
                }
            }
        }
    }
    return;
    LAB_004639f9:
    if (0x0 < local_1c) {
        iVar6 = FUN_004666a6();
        (&DAT_00569c30)[iVar6 * 0x9 + DAT_004c9754 * 0x1e22] = (&DAT_00569c30)[iVar6 * 0x9 + DAT_004c9754 * 0x1e22] & 0xfe;
        (&DAT_00569c30)[DAT_004c9754 * 0x1e22 + iVar6 * 0x9] = (&DAT_00569c30)[DAT_004c9754 * 0x1e22 + iVar6 * 0x9] & 0xfb;
        (&DAT_00569c30)[DAT_004c9754 * 0x1e22] = (&DAT_00569c30)[DAT_004c9754 * 0x1e22] & 0xf7;
        FUN_00463f7b();
        local_1c = (local_1c * 0x64 - *(&DAT_0058ad6a + iVar6 * 0xda)) / 0x64;
        if ((local_1c == 0x0) && (0x1 < iVar4)) {
            pcVar5 = FUN_00499050(DAT_0059679c,0x736b);
            FUN_0049c2e0(local_42c,pcVar5);
        }
        else {
            if (local_14 != 0x0) {
                uVar10 = 0xffffffff;
                puVar3 = local_42c;
                loop {
                    if (uVar10 == 0x0) break;
                    uVar10 = uVar10 - 0x1;
                    cVar1 = puVar3;
                    puVar3 = (puVar3 + 0x1);
                } while (cVar1 != '\0');
                uVar7 = 0xffffffff;
                pcVar5 = &DAT_0058aca8 + iVar6 * 0xda;
                loop {
                    if (uVar7 == 0x0) break;
                    uVar7 = uVar7 - 0x1;
                    cVar1 = *pcVar5;
                    pcVar5 = pcVar5 + 0x1;
                } while (cVar1 != '\0');
                if (~uVar10 + ~uVar7 + 0x3 < 0x400) {
                    FUN_0049c2e0(local_42c,s__s__s_004c21f4);
                }
                else {
                    pcVar5 = &DAT_004c21fa;
                    iVar6 = -0x1;
                    puVar3 = local_42c;
                    loop {
                        puVar8 = puVar3;
                        if (iVar6 == 0x0) break;
                        iVar6 = iVar6 + -0x1;
                        puVar8 = (puVar3 + 0x1);
                        cVar1 = puVar3;
                        puVar3 = puVar8;
                    } while (cVar1 != '\0');
                    pcVar9 = (puVar8 + -0x1);
                    loop {
                        cVar1 = *pcVar5;
                        *pcVar9 = cVar1;
                        if (cVar1 == '\0') break;
                        cVar1 = pcVar5[0x1];
                        pcVar5 = pcVar5 + 0x2;
                        pcVar9[0x1] = cVar1;
                        pcVar9 = pcVar9 + 0x2;
                    } while (cVar1 != '\0');
                    local_14 = 0x0;
                }
            }
        }
        if ((local_14 != 0x0) && (0x0 < local_1c)) {
            pcVar5 = &DAT_004c21fe;
            iVar6 = -0x1;
            puVar3 = local_42c;
            loop {
                puVar8 = puVar3;
                if (iVar6 == 0x0) break;
                iVar6 = iVar6 + -0x1;
                puVar8 = (puVar3 + 0x1);
                cVar1 = puVar3;
                puVar3 = puVar8;
            } while (cVar1 != '\0');
            pcVar9 = (puVar8 + -0x1);
            loop {
                cVar1 = *pcVar5;
                *pcVar9 = cVar1;
                if (cVar1 == '\0') break;
                cVar1 = pcVar5[0x1];
                pcVar5 = pcVar5 + 0x2;
                pcVar9[0x1] = cVar1;
                pcVar9 = pcVar9 + 0x2;
            } while (cVar1 != '\0');
        }^
        // goto LAB_004639f9;
    }
    if (local_14 != 0x0) {
        pcVar5 = &DAT_004c2200;
        iVar4 = -0x1;
        puVar3 = local_42c;
        loop {
            puVar8 = puVar3;
            if (iVar4 == 0x0) break;
            iVar4 = iVar4 + -0x1;
            puVar8 = (puVar3 + 0x1);
            cVar1 = puVar3;
            puVar3 = puVar8;
        } while (cVar1 != '\0');
        pcVar9 = (puVar8 + -0x1);
        loop {
            cVar1 = *pcVar5;
            *pcVar9 = cVar1;
            if (cVar1 == '\0') break;
            cVar1 = pcVar5[0x1];
            pcVar5 = pcVar5 + 0x2;
            pcVar9[0x1] = cVar1;
            pcVar9 = pcVar9 + 0x2;
        } while (cVar1 != '\0');
    }
    if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) != 0x0) {
        return;
    }
    FUN_0045518a(0x1 << ((byte)DAT_004c9754 & 0x1f),0xffffffff,0x74cc,0xffffffff,local_42c,0xffffffff,0x0);
    return;
}



fn FUN_00463cd9(param_1: i32) -> u32

{
    let mut iVar1: i32;
    let mut pcVar2: String;
    let local_98: u8 [0x80];
    let mut local_18: u32;
    let mut local_14: u32;

    if ((param_1 + 0x26) == 0x0) {
        local_18 = 0x0;
    }
    else {
        local_14 = 0x0;
        if ((param_1 + 0x1a) != 0x0) {
            if ((*(param_1 + 0x24) >> 0x10 < -0x1) || (0x71 < *(param_1 + 0x24) >> 0x10)) {
                *(param_1 + 0x26) = 0xffff;
            }
            if (((param_1 + 0x26) == -0x1) ||
                (((&DAT_00569c30)[(*(param_1 + 0x24) >> 0x10) * 0x9 + DAT_004c9754 * 0x1e22] & 0x1) != 0x0)) {
                *(param_1 + 0x26) = 0xffff;
                if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
                    FUN_0044c1ce(param_1);
                    FUN_004953d7();
                    FUN_00496a10();
                    FUN_0049536f();
                }
                else {
                    FUN_00424fe4();
                }
                local_14 = 0x1;
            }
            else {
                if ((((param_1 + 0x26) != -0x1) &&
                    (((&DAT_00569c30)[(*(param_1 + 0x24) >> 0x10) * 0x9 + DAT_004c9754 * 0x1e22] & 0x1) == 0x0)) &&
                    (iVar1 = (*(param_1 + 0x24) >> 0x10) * 0x9 + DAT_004c9754 * 0x1e22,
                     *(&DAT_00569c31 + iVar1) = *(&DAT_00569c31 + iVar1) - (*(param_1 + 0x18) >> 0x10),
                     *(&DAT_00569c31 + iVar1) < 0x0)) {
                    (param_1 + 0x1a) =
                        (char)-(&DAT_00569c31)[(*(param_1 + 0x24) >> 0x10) * 0x9 + DAT_004c9754 * 0x1e22];
                    if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
                        pcVar2 = FUN_00499050(DAT_0059679c,0x67bb);
                        FUN_0049c2e0(local_98,pcVar2);
                        FUN_0049d2e0(0x0,0x1,local_98);
                    }
                    (&DAT_00569c30)[(*(param_1 + 0x24) >> 0x10) * 0x9 + DAT_004c9754 * 0x1e22] =
                        (&DAT_00569c30)[(*(param_1 + 0x24) >> 0x10) * 0x9 + DAT_004c9754 * 0x1e22] & 0xfb;
                    (&DAT_00569c30)[(*(param_1 + 0x24) >> 0x10) * 0x9 + DAT_004c9754 * 0x1e22] =
                        (&DAT_00569c30)[(*(param_1 + 0x24) >> 0x10) * 0x9 + DAT_004c9754 * 0x1e22] | 0x1;
                    *(&DAT_00569c31 + (*(param_1 + 0x24) >> 0x10) * 0x9 + DAT_004c9754 * 0x1e22) = 0x0;
                    *(&DAT_00569c35 + (*(param_1 + 0x24) >> 0x10) * 0x9 + DAT_004c9754 * 0x1e22) = 0x0;
                    FUN_00463f7b();
                    *(param_1 + 0x26) = 0xffff;
                    if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
                        FUN_0044c1ce(param_1);
                        FUN_004953d7();
                        FUN_00496a10();
                        FUN_0049536f();
                    }
                    else {
                        FUN_00424fe4();
                    }
                    local_14 = 0x1;
                }
            }
            *(param_1 + 0x1a) = 0x0;
        }
        local_18 = local_14;
    }
    return local_18;
}



fn FUN_00463f7b()

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut local_14: i32;

    for (local_14 = 0x1; local_14 < 0x72; local_14 = local_14 + 0x1) {
        iVar1 = FUN_0046295d((&DAT_0058aca8 + local_14 * 0xda),0x0);
        if (iVar1 == 0x320) {
            (&DAT_00569c30)[local_14 * 0x9 + DAT_004c9754 * 0x1e22] =
                (&DAT_00569c30)[local_14 * 0x9 + DAT_004c9754 * 0x1e22] & 0xfe;
            (&DAT_00569c30)[DAT_004c9754 * 0x1e22 + local_14 * 0x9] =
                (&DAT_00569c30)[DAT_004c9754 * 0x1e22 + local_14 * 0x9] & 0xfd;
        }
        else {
            if (((&DAT_00569c30)[local_14 * 0x9 + DAT_004c9754 * 0x1e22] & 0x1) == 0x0) {
                iVar2 = DAT_004c9754 * 0x1e22;
                iVar1 = FUN_0046295d((&DAT_0058aca8 + local_14 * 0xda),0x0);
                if (((((&DAT_00569c30)[iVar1 * 0x9 + iVar2] & 0x1) == 0x0) ||
                    (iVar2 = DAT_004c9754 * 0x1e22, iVar1 = FUN_0046295d((&DAT_0058aca8 + local_14 * 0xda),0x1),
                     ((&DAT_00569c30)[iVar1 * 0x9 + iVar2] & 0x1) == 0x0)) ||
                    (iVar2 = DAT_004c9754 * 0x1e22, iVar1 = FUN_0046295d((&DAT_0058aca8 + local_14 * 0xda),0x2),
                     ((&DAT_00569c30)[iVar1 * 0x9 + iVar2] & 0x1) == 0x0)) {
                    (&DAT_00569c30)[local_14 * 0x9 + DAT_004c9754 * 0x1e22] =
                        (&DAT_00569c30)[local_14 * 0x9 + DAT_004c9754 * 0x1e22] & 0xfd;
                }
                else {
                    (&DAT_00569c30)[local_14 * 0x9 + DAT_004c9754 * 0x1e22] =
                        (&DAT_00569c30)[local_14 * 0x9 + DAT_004c9754 * 0x1e22] | 0x2;
                }
            }
            else {
                (&DAT_00569c30)[local_14 * 0x9 + DAT_004c9754 * 0x1e22] =
                    (&DAT_00569c30)[local_14 * 0x9 + DAT_004c9754 * 0x1e22] & 0xfd;
            }
        }
    }
        (&DAT_00569e94)[DAT_004c9754 * 0x1e22] = (&DAT_00569e94)[DAT_004c9754 * 0x1e22] | 0x1;
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00464119(param_1: *mut u32) -> u32

{
    ushort uVar1;
    let mut uVar2: u32;
    let mut bVar3: bool;
    let mut pcVar4: String;
    let puVar5: *mut u32;
    undefined3 extraout_var;
    let mut uVar6: u32;
    let mut iVar7: i32;
    undefined3 extraout_var_00;
    undefined3 extraout_var_01;
    let mut iVar8: i32;
    let uVar9: u16;
    let mut local_148: u32;
    i32 local_144 [0xd];
    let mut local_110: i32;
    let mut local_10c: u32;
    i32 local_108 [0xd];
    let mut local_d4: u32;
    let mut local_d0: i32;
    let mut local_cc: i32;
    let mut local_c8: u32;
    let mut local_c4: u32;
    let mut local_c0: i32;
    let mut local_bc: u32;
    let mut local_b8: u32;
    let mut local_b4: i32;
    let local_b0: u8 [0x80];
    let local_30: *mut u32;
    let mut local_2c: i32;
    let mut local_28: u32;
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_1c: u32;
    let mut local_18: i32;
    let mut local_14: i32;

    local_20 = *(param_1 + 0x6) >> 0x10;
    local_1c = param_1[0x2] >> 0x10;
    local_18 = *(param_1 + 0xa) >> 0x10;
    local_14 = param_1[0x3] >> 0x10;
    local_24 = local_20;
    if (DAT_004c9760 != 0x0) {
        local_24 = 0x0;
    }
    DAT_005b3ec8 = 0x0;
    uVar1 = (param_1 + 0xe);
    local_28 = param_1 & 0xffff0000 | uVar1;
    if (uVar1 < 0xc) {
        if (uVar1 < 0x8) {
            if (uVar1 < 0x5) {
                DAT_005b3ec8 = 0x0;
                return local_28;
            }
            if ((0x5 < uVar1) && (uVar1 != 0x6)) {
                DAT_005b3ec8 = 0x0;
                return local_28;
            }
        }
        else {
            if (((0x8 < uVar1) && (0x9 < uVar1)) && (uVar1 != 0xb)) {
                DAT_005b3ec8 = 0x0;
                return local_28;
            }
        }
    }
    else {
        if (0xc < uVar1) {
            if (uVar1 < 0x14) {
                if (uVar1 < 0x12) {
                    DAT_005b3ec8 = 0x0;
                    return local_28;
                }
                uVar6 = FUN_00466271(local_20,local_1c,local_18,param_1[0x3] >> 0x10,local_144);
                for (local_148 = 0x0; local_148 < 0xd; local_148 = local_148 + 0x1) {
                    if (local_144[local_148] != 0x0) {
                        iVar7 = FUN_0046645d(local_20,local_1c,local_18);
                        iVar7 = (local_144[local_148] * iVar7) / 0x64;
                        if (iVar7 != 0x0) {
                            if (((DAT_004d7aca >> 0x18 == DAT_004c9754) &&
                                (bVar3 = FUN_0046292a((&DAT_00568210 + DAT_004c9754 * 0x1e22)),
                                 CONCAT31(extraout_var_01,bVar3) != 0x0)) &&
                                (iVar8 = FUN_00489176(local_20,local_1c,local_18,0x8), iVar8 != 0x0)) {
                                DAT_005b3ec8 = DAT_005b3ec8 + (iVar7 * DAT_004d7a68) / 0x64;
                                DAT_005b3ee8 = DAT_005b3ec8;
                                DAT_005b3eec = local_20;
                                DAT_005b3ef0 = param_1[0x3] >> 0x10;
                            }
                            DAT_005b3ec8 = DAT_005b3ec8 + ((*(&DAT_00569b9c + DAT_004c9754 * 0x1e22) >> 0x18) * iVar7) / 0x64;
                            DAT_005b3ed0 = DAT_005b3ed0 + ((*(&DAT_00569b9c + DAT_004c9754 * 0x1e22) >> 0x18) * iVar7) / 0x64;
                            iVar7 = iVar7 + DAT_005b3ec8;
                            FUN_00465f06(local_20,local_1c,local_18,local_148,iVar7,DAT_004c9754,param_1);
                            *(&DAT_005b2e84 + local_148 * 0x4 + local_24 * 0x34) =
                                *(&DAT_005b2e84 + local_148 * 0x4 + local_24 * 0x34) + iVar7;
                        }
                    }
                    uVar6 = local_148;
                }
                return uVar6;
            }
            if (0x14 < uVar1) {
                if (0x17 < uVar1) {
                    if ((0x18 < uVar1) && (uVar1 != 0x19)) {
                        DAT_005b3ec8 = 0x0;
                        return local_28;
                    }
                    uVar6 = FUN_00466271(local_20,local_1c,local_18,param_1[0x3] >> 0x10,local_108);
                    for (local_10c = 0x0; uVar2 = local_10c, local_10c < 0xd; local_10c = local_10c + 0x1) {
                        if (local_108[local_10c] != 0x0) {
                            iVar7 = FUN_0046645d(local_20,local_1c,local_18);
                            local_110 = (local_108[uVar2] * iVar7) / 0x64;
                            if (local_110 != 0x0) {
                                if (local_108[0] != 0x0) {
                                    DAT_005b3ed4 = DAT_005b3ed4 + (*(&DAT_00569b69 + DAT_004c9754 * 0x1e22) >> 0x18);
                                    DAT_005b3ed8 = DAT_005b3ed8 + (*(&DAT_00569b96 + DAT_004c9754 * 0x1e22) >> 0x18);
                                }
                                DAT_005b3ec8 = (local_110 * DAT_005b3ecc) / 0x64;
                                DAT_005b3ee4 = DAT_005b3ee4 + DAT_005b3ec8;
                                if (((DAT_004d7aca >> 0x18 == DAT_004c9754) &&
                                    (bVar3 = FUN_0046292a((&DAT_00568210 + DAT_004c9754 * 0x1e22)),
                                     CONCAT31(extraout_var_00,bVar3) != 0x0)) &&
                                    (iVar7 = FUN_00489176(local_20,local_1c,local_18,0x8), iVar7 != 0x0)) {
                                    DAT_005b3ec8 = DAT_005b3ec8 + (local_110 * DAT_004d7a68) / 0x64;
                                    DAT_005b3ee8 = DAT_005b3ec8;
                                    DAT_005b3eec = local_20;
                                    DAT_005b3ef0 = param_1[0x3] >> 0x10;
                                }
                                DAT_005b3ec8 = DAT_005b3ec8 +
                                    (((*(&DAT_00569b9c + DAT_004c9754 * 0x1e22) >> 0x18) +
                                        (*(&DAT_00569b69 + DAT_004c9754 * 0x1e22) >> 0x18) +
                                        (*(&DAT_00569b96 + DAT_004c9754 * 0x1e22) >> 0x18)) * local_110) / 0x64;
                                DAT_005b3ed0 = DAT_005b3ed0 +
                                    ((*(&DAT_00569b9c + DAT_004c9754 * 0x1e22) >> 0x18) * local_110) / 0x64;
                                local_110 = local_110 + DAT_005b3ec8;
                                FUN_00465f06(local_20,local_1c,local_18,local_10c,local_110,DAT_004c9754,param_1);
                                *(&DAT_005b2e84 + local_10c * 0x4 + local_24 * 0x34) =
                                    *(&DAT_005b2e84 + local_10c * 0x4 + local_24 * 0x34) + local_110;
                            }
                        }
                        uVar6 = local_10c;
                    }
                    return uVar6;
                }
                if (uVar1 != 0x17) {
                    DAT_005b3ec8 = 0x0;
                    return local_28;
                }
                if (0x1f3 < *(DAT_004c9754 * 0x1e22 + 0x569abd)) {
                    *(&DAT_00569abd + DAT_004c9754 * 0x1e22) = *(&DAT_00569abd + DAT_004c9754 * 0x1e22) + -0x1f4;
                    local_2c = param_1[0x9] >> 0x10;
                    iVar7 = FUN_00463cd9(param_1);
                    if (iVar7 == 0x0) {
                        return 0x0;
                    }
                    puVar5 = *DAT_005967c8;
                    while (puVar5 != 0x0) {
                        local_30 = puVar5;
                        if (((*(puVar5 + 0xe) >> 0x10 == DAT_004c9754) && ((puVar5 + 0xe) == 0x17)) &&
                            (((puVar5[0x9] >> 0x10 == local_2c && ((puVar5 + 0x1a) != 0x0)) &&
                                (((puVar5 + 0xa) != (param_1 + 0xa) ||
                                    ((puVar5 + 0x3) != (param_1 + 0x3))))))) {
                            *(puVar5 + 0x26) = 0xffff;
                            if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
                                if (local_2c != -0x1) {
                                    pcVar4 = FUN_00499050(DAT_0059679c,0x7408);
                                    FUN_0049c2e0(local_b0,pcVar4);
                                    FUN_0049d2e0(0x0,0x1,local_b0);
                                }
                                FUN_0044c1ce(puVar5);
                                FUN_004953d7();
                                FUN_00496a10();
                                FUN_0049536f();
                            }
                            else {
                                FUN_00424fe4();
                            }
                            *(puVar5 + 0x1a) = 0x0;
                            param_1 = puVar5;
                        }
                        puVar5 = *local_30;
                    }
                    return 0x0;
                }
                if (_DAT_005b3ec4 != 0x0) {
                    DAT_005b3ec8 = 0x0;
                    return DAT_004c9754 * 0x1e22;
                }
                _DAT_005b3ec4 = 0x1;
                uVar9 = 0x0;
                uVar6 = 0xffffffff;
                puVar5 = FUN_00499050(DAT_0059679c,0x741e);
                uVar6 = FUN_0045518a(0x1 << ((byte)DAT_004c9754 & 0x1f),0xffffffff,0x74cc,0xffffffff,puVar5,uVar6,uVar9);
                return uVar6;
            }
        }
    }
    local_b4 = 0x0;
    local_b8 = (&DAT_00583c28 + (param_1[0x3] >> 0x10) * 0x384);
    local_bc = *(&DAT_00583c28 + (param_1[0x3] >> 0x10) * 0x384) >> 0x10;
    local_c0 = 0x0;
    while( true ) {
        if ((0x3 < local_c0) || (*(&DAT_00583c2c + (param_1[0x3] >> 0x10) * 0x384 + local_c0 * 0x4) == -0x1))^
        // goto LAB_0046448b;
        local_c4 = (&DAT_00583c2c + local_c0 * 0x4 + (param_1[0x3] >> 0x10) * 0x384);
        local_c8 = *(&DAT_00583c2c + local_c0 * 0x4 + (param_1[0x3] >> 0x10) * 0x384) >> 0x10;
        if ((((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) &&
            (*(&DAT_005b2e84 + local_c8 * 0x4 + local_24 * 0x34) < local_c4)) break;
        local_b4 = 0x1;
        local_c0 = local_c0 + 0x1;
    }
    local_b4 = 0x0;
    LAB_0046448b:
    if (local_b4 == 0x0) {
        local_d4 = FUN_00465456(local_20);
    }
    else {
        local_cc = 0x0;
        while ((local_cc < 0x4 && (*(&DAT_00583c2c + (param_1[0x3] >> 0x10) * 0x384 + local_cc * 0x4) != -0x1)))
        {
            local_c4 = (&DAT_00583c2c + local_cc * 0x4 + (param_1[0x3] >> 0x10) * 0x384);
            local_c8 = *(&DAT_00583c2c + local_cc * 0x4 + (param_1[0x3] >> 0x10) * 0x384) >> 0x10;
            *(&DAT_005b36a4 + local_24 * 0x34 + local_c8 * 0x4) =
                *(&DAT_005b36a4 + local_24 * 0x34 + local_c8 * 0x4) + local_c4;
            *(&DAT_005b2e84 + local_c8 * 0x4 + local_24 * 0x34) =
                *(&DAT_005b2e84 + local_c8 * 0x4 + local_24 * 0x34) - local_c4;
            local_cc = local_cc + 0x1;
        }
        local_d0 = FUN_0046645d(local_20,local_1c,local_18);
        local_d4 = (local_b8 * local_d0) / 0x64;
        if (((DAT_004d7aca >> 0x18 == DAT_004c9754) &&
            (bVar3 = FUN_0046292a((&DAT_00568210 + DAT_004c9754 * 0x1e22)), CONCAT31(extraout_var,bVar3) != 0x0)) &&
            (iVar7 = FUN_00489176(local_20,local_1c,local_18,0x8), iVar7 != 0x0)) {
            DAT_005b3ec8 = (local_d4 * DAT_004d7a68) / 0x64;
            DAT_005b3eec = local_20;
            DAT_005b3ef0 = param_1[0x3] >> 0x10;
            DAT_005b3ee8 = DAT_005b3ec8;
        }
        DAT_005b3ec8 = DAT_005b3ec8 + ((*(&DAT_00569b9c + DAT_004c9754 * 0x1e22) >> 0x18) * local_d4) / 0x64;
        DAT_005b3ed0 = DAT_005b3ed0 + ((*(&DAT_00569b9c + DAT_004c9754 * 0x1e22) >> 0x18) * local_d4) / 0x64;
        local_d4 = local_d4 + DAT_005b3ec8;
        if (local_d4 == 0x0) {
            if (local_d0 == 0x0) {
                return DAT_005b3ec8;
            }
            local_d4 = 0x1;
        }
        FUN_00465f06(local_20,local_1c,local_18,local_bc,local_d4,DAT_004c9754,param_1);
        *(&DAT_005b2e84 + local_bc * 0x4 + local_24 * 0x34) =
            *(&DAT_005b2e84 + local_bc * 0x4 + local_24 * 0x34) + local_d4;
    }
    return local_d4;
}



fn FUN_00464c97()

{
    let mut bVar1: bool;
    let mut pcVar2: String;
    i32 aiStackY131732 [0x28];
    i32 aiStackY131572 [0x28];
    i32 aiStackY131412 [0x7f87];
    let mut local_31c: i32;
    let mut local_2e8: i32;
    let mut local_2b0: i32;
    let mut local_2ac: i32;
    let local_29c: *mut u32;
    let local_298: *mut u32;
    i32 local_294 [0x28];
    i32 local_1f4 [0x28];
    i32 local_154 [0x28];
    u32 local_b4 [0x20];
    i32 **local_34;
    i32 **local_30;
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_24: i32;
    let local_20: *mut u32;
    let local_1c: *mut u32;
    let mut local_18: i32;

    if (DAT_004d5584 != 0x0) {
        local_34 = (i32 **)0x0;
        local_18 = 0x1;
        FUN_004a0430(local_154,0x0,0xa0);
        FUN_004a0430(local_1f4,0x0,0xa0);
        FUN_004a0430(local_294,0x0,0xa0);
        if (DAT_004c9760 == 0x0) {
            for (local_29c = *DAT_005967b0; local_29c != 0x0; local_29c = *local_29c
            ) {
                if ((((local_29c + 0x27) == '[') && ((local_29c + 0xc) == '\0')) &&
                    (*(local_29c + 0x23) >> 0x18 == DAT_004c9754)) {
                    local_1f4[(local_29c + 0x8)] =
                        local_1f4[(local_29c + 0x8)] + (*(local_29c + 0x2f) >> 0x10);
                }
            }
        }
        else {
            for (local_298 = *DAT_005967b0; local_298 != 0x0; local_298 = *local_298
            ) {
                if ((((local_298 + 0x27) == '[') && ((local_298 + 0xc) == '\0')) &&
                    (*(local_298 + 0x23) >> 0x18 == DAT_004c9754)) {
                    local_1f4[0] = local_1f4[0] + (*(local_298 + 0x2f) >> 0x10);
                }
            }
        }
        local_24 = 0x0;
        if (DAT_004c9760 != 0x0) {
            for (local_30 = (i32 **)*DAT_005967c8; local_30 != (i32 **)0x0; local_30 = (i32 **)*local_30) {
                if ((*(local_30 + 0xe) >> 0x10 == DAT_004c9754) && ((*(local_30 + 0x2d) & 0x1) == 0x0))
                {
                    local_24 = local_24 + (*(local_30 + 0x26) >> 0x10) / 0xa;
                }
            }
            if (local_24 <= local_1f4[0]) {
                local_18 = 0x0;
                local_1f4[0] = local_1f4[0] - local_24;
                FUN_00465d24(0x0,0x0,local_24,DAT_004c9754);
            }
        }
        if (local_18 != 0x0) {
            for (local_2ac = 0x64; 0x9 < local_2ac; local_2ac = local_2ac + -0xa) {
                local_30 = (i32 **)*DAT_005967c8;
                while (local_30 != (i32 **)0x0) {
                    local_34 = (i32 **)*local_30;
                    if (*(local_30 + 0xe) >> 0x10 == DAT_004c9754) {
                        if ((*(local_30 + 0x2d) & 0x1) == 0x0) {
                            if (local_2ac <= *(local_30 + 0x26) >> 0x10) {
                                if (DAT_004c9760 == 0x0) {
                                    local_2b0 = local_1f4[*(local_30 + 0x6) >> 0x10];
                                }
                                else {
                                    local_2b0 = local_1f4[0];
                                }
                                local_28 = local_2b0;
                                if (local_2b0 < 0x1) {
                                    local_154[*(local_30 + 0x6) >> 0x10] = 0x1;
                                    if (((&DAT_00569a98)[DAT_004c9754 * 0x1e22] & 0x2) == 0x0) {
                                        (local_30 + 0xa) = (local_30 + 0xa) + -0x5;
                                    }
                                    if (*(local_30 + 0x26) >> 0x10 < 0x1) {
                                        FUN_004811e6(local_30);
                                    }
                                    else {
                                        *(local_30 + 0xb) = *(local_30 + 0xb) | 0x8;
                                    }
                                }
                                else {
                                    if (DAT_004c9760 == 0x0) {
                                        local_294[*(local_30 + 0x6) >> 0x10] =
                                            local_294[*(local_30 + 0x6) >> 0x10] + 0x1;
                                        local_1f4[*(local_30 + 0x6) >> 0x10] =
                                            local_1f4[*(local_30 + 0x6) >> 0x10] + -0x1;
                                    }
                                    else {
                                        local_294[0] = local_294[0] + 0x1;
                                        local_1f4[0] = local_1f4[0] + -0x1;
                                    }
                                }
                            }
                            local_30 = local_34;
                            local_34 = local_30;
                        }
                        else {
                            local_30 = local_34;
                            local_34 = local_30;
                        }
                    }
                    else {
                        local_30 = local_34;
                        local_34 = local_30;
                    }
                }
            }
        }
        DAT_00599d3c = 0x1;
        FUN_004840cd(&local_1c,&local_20,-0x1);
        while (local_20 != 0x0) {
            if (*(local_20 + 0x23) >> 0x18 == DAT_004c9754) {
                if (((local_20 + 0x35) == -0x1) ||
                    ((local_20 + 0x26) == (local_20 + 0x35))) {
                    bVar1 = false;
                }
                else {
                    bVar1 = true;
                }
                if ((((!bVar1) && ((*(local_20 + 0x3a) & 0x1) != 0x0)) &&
                    ((*(local_20 + 0x3a) & 0x80000000) == 0x0)) &&
                    (*(*(&DAT_00582938 +
                        (*(local_20 + 0x25) >> 0x18) * 0x4 + (local_20[0x9] >> 0x18) * 0x18) +
                        0x115) != 0x0)) {
                    if (DAT_004c9760 == 0x0) {
                        local_2e8 = local_1f4[(local_20 + 0x8)];
                    }
                    else {
                        local_2e8 = local_1f4[0];
                    }
                    local_28 = local_2e8;
                    if (local_2e8 < 0x1) {
                        local_154[(local_20 + 0x8)] = 0x1;
                        *(local_20 + 0x3b) = *(local_20 + 0x3b) | 0x1;
                        *(local_20 + 0x3a) = *(local_20 + 0x3a) & 0xfb;
                        if (((&DAT_00569a98)[(*(local_20 + 0x23) >> 0x18) * 0x1e22] & 0x2) == 0x0) {
                            FUN_00488efd(local_20,health_loss_for_famine_00599da8);
                        }
                    }
                    else {
                        if (DAT_004c9760 == 0x0) {
                            local_294[(local_20 + 0x8)] = local_294[(local_20 + 0x8)] + 0x1;
                            local_1f4[(local_20 + 0x8)] = local_1f4[(local_20 + 0x8)] + -0x1;
                        }
                        else {
                            local_294[0] = local_294[0] + 0x1;
                            local_1f4[0] = local_1f4[0] + -0x1;
                        }
                    }
                }
            }
            local_20 = local_1c;
            if (local_1c != 0x0) {
                local_1c = *local_1c;
            }
        }
        if (DAT_004c9760 == 0x0) {
            for (local_31c = 0x0; local_31c < 0x28; local_31c = local_31c + 0x1) {
                if (local_294[local_31c] != 0x0) {
                    FUN_00465d24(local_31c,0x0,local_294[local_31c],DAT_004c9754);
                }
            }
        }
        else {
            if (local_294[0] != 0x0) {
                FUN_00465d24(0x0,0x0,local_294[0],DAT_004c9754);
            }
        }
        for (local_2c = 0x0; local_2c < 0x28; local_2c = local_2c + 0x1) {
            if (local_154[local_2c] != 0x0) {
                pcVar2 = FUN_00499050(DAT_0059679c,0x7376);
                FUN_0049c2e0(local_b4,pcVar2);
                FUN_0045518a(0x1 << ((byte)DAT_004c9754 & 0x1f),0xffffffff,0x7376,local_2c,local_b4,0xffffffff,0x0);
            }
        }
        FUN_004864f7();
        FUN_0048418d(&local_1c);
    }
    return;
}



fn FUN_00465456(param_1: u32)

{
    let mut pcVar1: String;
    u32 local_90 [0x20];

    pcVar1 = FUN_00499050(DAT_0059679c,0x7153);
    FUN_0049c2e0(local_90,pcVar1);
    FUN_0045518a(0x1 << ((byte)DAT_004c9754 & 0x1f),0xffffffff,0x74cc,param_1,local_90,0xffffffff,0x0);
    return;
}



fn FUN_004654df(param_1: *mut i32,param_2: *mut i32) -> u32

{
    let mut local_14: u32;

    (param_1 + 0x31) = (param_1 + 0x31) - param_2;
    if (*(param_1 + 0x2f) >> 0x10 < 0x1) {
        *param_2 = *(param_1 + 0x2f) >> 0x10;
        *param_2 = -*param_2;
        *(param_1 + 0x31) = 0x0;
    }
    else {
        *param_2 = 0x0;
    }
    if ((param_1 + 0x31) == 0x0) {
        FUN_00484b4e(param_1);
        if (*param_2 == 0x0) {
            local_14 = 0x1;
        }
        else {
            local_14 = 0x0;
        }
    }
    else {
        local_14 = 0x1;
    }
    return local_14;
}



fn FUN_0046556d(param_1: i32,param_2: i32,param_3: i32,param_4: i32)

{
    i32 **ppiVar1;
    let mut iVar2: i32;
    i32 **local_18;

    ppiVar1 = (i32 **)*(i32 **)(&DAT_005b8b44 + param_1 * 0x4);
    loop {
    local_18 = ppiVar1;
    if ((local_18 == (i32 **)0x0) || ((local_18 + 0x8) != param_1)) break;
    ppiVar1 = (i32 **)*local_18;
} while (((*(local_18 + 0x3a) & 0x80000000) != 0x0) ||
    ((((*(local_18 + 0x23) >> 0x18 != param_4 || ((local_18 + 0x27) != '[')) ||
        (*(local_18 + 0x2d) >> 0x18 != param_2)) ||
        (iVar2 = FUN_004654df(local_18,&param_3), iVar2 == 0x0))));
    if ((param_3 != 0x0) && (DAT_004c9760 != 0x0)) {
        ppiVar1 = (i32 **)*DAT_005967b0;
        loop {
            local_18 = ppiVar1;
            if (local_18 == (i32 **)0x0) {
                return;
            }
            ppiVar1 = (i32 **)*local_18;
        } while ((((*(local_18 + 0x3a) & 0x80000000) != 0x0) ||
            (*(local_18 + 0x23) >> 0x18 != param_4)) ||
            (((local_18 + 0x27) != '[' ||
                ((*(local_18 + 0x2d) >> 0x18 != param_2 ||
                    (iVar2 = FUN_004654df(local_18,&param_3), iVar2 == 0x0))))));
    }
    return;
}



fn FUN_004656d4(param_1: i32,param_2: *mut i32,param_3: i32) -> u32

{
    i32 **ppiVar1;
    let puVar2: *mut u32;
    let mut iVar3: i32;
    let mut local_20: u32;
    i32 **local_1c;

    local_20 = 0x0;
    ppiVar1 = (i32 **)*(i32 **)(&DAT_005b8b44 + param_1 * 0x4);
    loop {
    local_1c = ppiVar1;
    if ((local_1c == (i32 **)0x0) || ((local_1c + 0x8) != param_1))^ // goto LAB_0046582d;
    ppiVar1 = (i32 **)*local_1c;
} while ((((*(local_1c + 0x3a) & 0x80000000) != 0x0) ||
    (((*(local_1c + 0x23) >> 0x18 != param_3 || ((local_1c + 0x27) != '[')) ||
        ((local_1c + 0xc) != '\0')))) ||
    (((puVar2 = FUN_00481784((local_1c + 0x8),(local_1c + 0x22),
                             (local_1c + 0x9)), puVar2 == 0x0 ||
           ((puVar2 + 0xe) != 0x18)) || (iVar3 = FUN_004654df(local_1c,param_2), iVar3 == 0x0))))
;
    local_20 = 0x1;
    LAB_0046582d:
    if ((*param_2 != 0x0) && (DAT_004c9760 != 0x0)) {
        ppiVar1 = (i32 **)*DAT_005967b0;
        loop {
            local_1c = ppiVar1;
            if (local_1c == (i32 **)0x0) {
                return local_20;
            }
            ppiVar1 = (i32 **)*local_1c;
        } while ((((((*(local_1c + 0x3a) & 0x80000000) != 0x0) ||
            (*(local_1c + 0x23) >> 0x18 != param_3)) || ((local_1c + 0x27) != '[')) ||
            (((local_1c + 0xc) != '\0' ||
                (puVar2 = FUN_00481784((local_1c + 0x8),(local_1c + 0x22),
                                       (local_1c + 0x9)), puVar2 == 0x0)))) ||
            (((puVar2 + 0xe) != 0x18 || (iVar3 = FUN_004654df(local_1c,param_2), iVar3 == 0x0))));
        local_20 = 0x1;
    }
    return local_20;
}



fn FUN_00465996(param_1: i32,param_2: *mut i32,param_3: i32) -> u32

{
    i32 **ppiVar1;
    let mut iVar2: i32;
    let mut local_1c: u32;
    i32 **local_18;

    local_1c = 0x0;
    ppiVar1 = (i32 **)*(i32 **)(&DAT_005b8b44 + param_1 * 0x4);
    loop {
    local_18 = ppiVar1;
    if ((local_18 == (i32 **)0x0) || ((local_18 + 0x8) != param_1))^ // goto LAB_00465a87;
    ppiVar1 = (i32 **)*local_18;
} while ((((*(local_18 + 0x3a) & 0x80000000) != 0x0) ||
    (((*(local_18 + 0x23) >> 0x18 != param_3 || ((*(local_18 + 0x3a) & 0x1) == 0x0)) ||
        ((local_18 + 0x27) != '[')))) ||
    (((local_18 + 0xc) != '\0' || (iVar2 = FUN_004654df(local_18,param_2), iVar2 == 0x0))));
    local_1c = 0x1;
    LAB_00465a87:
    if ((*param_2 != 0x0) && (DAT_004c9760 != 0x0)) {
        ppiVar1 = (i32 **)*DAT_005967b0;
        loop {
            local_18 = ppiVar1;
            if (local_18 == (i32 **)0x0) {
                return local_1c;
            }
            ppiVar1 = (i32 **)*local_18;
        } while ((((((*(local_18 + 0x3a) & 0x80000000) != 0x0) ||
            (*(local_18 + 0x23) >> 0x18 != param_3)) || ((*(local_18 + 0x3a) & 0x1) == 0x0))
            || (((local_18 + 0x27) != '[' || ((local_18 + 0xc) != '\0')))) ||
            (iVar2 = FUN_004654df(local_18,param_2), iVar2 == 0x0));
        local_1c = 0x1;
    }
    return local_1c;
}



fn FUN_00465b5e(param_1: i32,param_2: i32,param_3: i32) -> u32

{
    i32 **ppiVar1;
    let mut iVar2: i32;
    let mut local_1c: u32;
    i32 **local_18;

    local_1c = 0x0;
    ppiVar1 = (i32 **)*(i32 **)(&DAT_005b8b44 + param_1 * 0x4);
    loop {
    local_18 = ppiVar1;
    if ((local_18 == (i32 **)0x0) || ((local_18 + 0x8) != param_1))^ // goto LAB_00465c4f;
    ppiVar1 = (i32 **)*local_18;
} while ((((*(local_18 + 0x3a) & 0x80000000) != 0x0) ||
    (((*(local_18 + 0x23) >> 0x18 != param_3 || ((*(local_18 + 0x3a) & 0x1) != 0x0)) ||
        ((local_18 + 0x27) != '[')))) ||
    (((local_18 + 0xc) != '\0' || (iVar2 = FUN_004654df(local_18,&param_2), iVar2 == 0x0))));
    local_1c = 0x1;
    LAB_00465c4f:
    if ((param_2 != 0x0) && (DAT_004c9760 != 0x0)) {
        ppiVar1 = (i32 **)*DAT_005967b0;
        loop {
            local_18 = ppiVar1;
            if (local_18 == (i32 **)0x0) {
                return local_1c;
            }
            ppiVar1 = (i32 **)*local_18;
        } while ((((((*(local_18 + 0x3a) & 0x80000000) != 0x0) ||
            (*(local_18 + 0x23) >> 0x18 != param_3)) || ((*(local_18 + 0x3a) & 0x1) != 0x0))
            || (((local_18 + 0x27) != '[' || ((local_18 + 0xc) != '\0')))) ||
            (iVar2 = FUN_004654df(local_18,&param_2), iVar2 == 0x0));
        local_1c = 0x1;
    }
    return local_1c;
}



fn FUN_00465d24(param_1: i32,param_2: i32,param_3: i32,param_4: i32)

{
    let mut iVar1: i32;

    if (0x0 < param_3) {
        if (param_2 == 0x0) {
            iVar1 = FUN_004656d4(param_1,&param_3,param_4);
            if ((iVar1 == 0x0) && (iVar1 = FUN_00465996(param_1,&param_3,param_4), iVar1 == 0x0)) {
                FUN_00465b5e(param_1,param_3,param_4);
            }
        }
        else {
            FUN_0046556d(param_1,param_2,param_3,param_4);
        }
    }
    return;
}



fn FUN_00465dae(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: i32,param_6: i32)

{
    i32 **local_14;

    if (0x0 < param_5) {
        for (local_14 = *(i32 ***)(&DAT_005b8b44 + param_1 * 0x4);
            (local_14 != (i32 **)0x0 && ((local_14 + 0x8) == param_1)); local_14 = (i32 **)*local_14) {
            if ((((*(local_14 + 0x3a) & 0x80000000) == 0x0) &&
                ((((local_14 + 0x22) == param_2 && ((local_14 + 0x9) == param_3)) &&
                    (*(local_14 + 0x23) >> 0x18 == param_6)))) &&
                (((local_14 + 0x27) == '[' && (*(local_14 + 0x2d) >> 0x18 == param_4)))) {
                (local_14 + 0x31) = (local_14 + 0x31) - param_5;
                if (*(local_14 + 0x2f) >> 0x10 < 0x1) {
                    param_5 = -(*(local_14 + 0x2f) >> 0x10);
                    *(local_14 + 0x31) = 0x0;
                }
                if ((local_14 + 0x31) != 0x0) {
                    return;
                }
                local_14 = (i32 **)FUN_00484b4e(local_14);
                if (param_5 == 0x0) {
                    return;
                }
            }
        }
    }
    return;
}



fn FUN_00465f06(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: i32,param_6: i32,param_7: i32) -> u32

{
    let mut iVar1: i32;
    let puVar2: *mut u32;
    let puVar3: *mut u32;
    let in_stack_fffffef4: i16;
    let mut in_stack_fffffef8: u32;
    u32 local_d4 [0xc];
    let mut local_a4: u32;
    let mut local_a0: i32;
    let mut local_9c: i32;
    let local_98: *mut u32;
    let mut local_94: i32;
    let mut local_90: u32;
    let local_8c: *mut u32;
    let local_88: *mut u32;
    let mut local_84: u32;
    let local_80: *mut u32;
    let local_7c: *mut u32;
    let mut local_78: u32;
    let local_74: *mut u32;
    let local_70: *mut u32;
    let mut local_6c: u32;
    let local_68: *mut u32;
    let local_64: *mut u32;
    let local_60: *mut u32;
    let local_5c: *mut u32;
    let mut local_58: u32;
    let local_54: u16;
    let local_52: u16;
    let local_50: u16;
    let local_4e: u8;
    let local_4d: u16;
    let local_4a: u16;
    let local_46: u8;
    let local_45: u8;
    let local_44: u8;
    let local_43: u16;
    let local_41: u8;
    let local_3f: u8;
    let mut local_3e: u32;
    let mut local_3a: u32;
    let mut local_24: u32;
    let local_20: *mut u32;
    let mut local_1c: i32;
    let mut local_18: i32;
    i32 **local_14;

    if (param_5 < 0x1) {
        local_24 = 0x0;
    }
    else {
        local_1c = 0x0;
        for (local_20 = (&DAT_005b8b44 + param_1 * 0x4); local_20 != 0x0;
            local_20 = *local_20) {
            local_60 = local_20 + 0x8;
            local_58 = local_60 & 0xffff0000 | local_60;
            local_5c = local_60;
            if (local_60 != param_1) break;
            local_68 = local_20 + 0x8;
            local_6c = *(local_20 + 0x3a) & 0x80000000;
            if (local_6c == 0x0) {
                local_74 = local_20 + 0x8;
                local_78 = local_74 & 0xffff0000 | (local_20 + 0x22);
                local_70 = local_74;
                if ((local_20 + 0x22) == param_2) {
                    local_80 = local_20 + 0x8;
                    local_84 = local_80 & 0xffff0000 | (local_20 + 0x9);
                    local_7c = local_80;
                    if ((local_20 + 0x9) == param_3) {
                        local_8c = local_20 + 0x8;
                        local_90 = *(local_20 + 0x3a) & 0x1;
                        local_88 = local_8c;
                        if ((((local_90 != 0x0) && (*(local_20 + 0x23) >> 0x18 == param_6)) &&
                            (local_1c = local_1c + 0x1, (local_20 + 0x27) == '[')) &&
                            (*(local_20 + 0x2d) >> 0x18 == param_4)) {
                            (local_20 + 0x31) = (local_20 + 0x31) + param_5;
                            if (*(local_20 + 0x2f) >> 0x10 < 0x3e8) {
                                return 0x1;
                            }
                            param_5 = (*(local_20 + 0x2f) >> 0x10) + -0x3e7;
                            *(local_20 + 0x31) = 0x3e7;
                        }
                    }
                }
            }
            local_64 = local_68;
        }
        loop {
            if (0x13 < local_1c) {
                return 0x0;
            }
            local_18 = 0x0;
            FUN_00486065(&local_54);
            local_98 = &local_54;
            local_9c = param_1;
            local_a0 = param_2;
            local_94 = param_3;
            local_54 = (undefined2)param_1;
            local_52 = (undefined2)param_2;
            local_50 = (undefined2)param_3;
            local_4d = 0x5b;
            FUN_00486b6b(&local_54,0x4b);
            local_4a = *(DAT_005831c0 + 0x41);
            local_45 = (DAT_005831c0 + 0x45);
            local_46 = (&DAT_00569b75)[DAT_004c9754 * 0x1e22] != '\0';
            local_a4 = (byte)local_46;
            local_44 = param_4;
            if (param_5 < 0x3e8) {
                local_43 = (undefined2)param_5;
            }
            else {
                local_43 = 0x3e7;
                param_5 = param_5 + -0x3e7;
                local_18 = 0x1;
            }
            local_41 = 0x64;
            local_4e = (char)param_6;
            if (param_7 == 0x0) {
                local_3f = (char)param_6;
            }
            else {
                local_3f = (param_7 + 0x12);
            }
            if (local_3f == -0x1) {
                local_3f = (char)param_6;
            }
            if (local_3f != (char)param_6) {
                local_3a = local_3a | 0x80;
            }
            local_3e = 0x0;
            local_3a = local_3a | 0x1;
            iVar1 = FUN_004889db(&local_54,param_7);
            if (iVar1 == 0x0) {
                return 0x0;
            }
            puVar2 = &local_54;
            puVar3 = local_d4;
            for (iVar1 = 0xb; iVar1 != 0x0; iVar1 = iVar1 + -0x1) {
                *puVar3 = *puVar2;
                puVar2 = puVar2 + 0x1;
                puVar3 = puVar3 + 0x1;
            }
            puVar3 = puVar2;
            puVar2 = local_d4;
            puVar3 = &stack0xfffffef4;
            for (iVar1 = 0xb; iVar1 != 0x0; iVar1 = iVar1 + -0x1) {
                *puVar3 = *puVar2;
                puVar2 = puVar2 + 0x1;
                puVar3 = puVar3 + 0x1;
            }
            puVar3 = puVar2;
            local_14 = FUN_00485463(in_stack_fffffef4,in_stack_fffffef8);
            local_1c = local_1c + 0x1;
        } while (local_18 != 0x0);
        local_24 = 0x1;
    }
    return local_24;
}



fn FUN_0046623d(param_1: i32,param_2: i32,param_3: i32)

{
    FUN_004662c9(param_1,param_2,param_3,DAT_005b4a30,DAT_005b4a2c);
    return;
}



fn FUN_00466271(param_1: u32,param_2: u32,param_3: i32,param_4: i32,param_5: u32)

{
    FUN_004a0430(param_5,0x0,0x34);
    DAT_005b4a2c = param_5;
    DAT_005b4a30 = param_4;
    FUN_00482843(param_1,param_2,param_3,*(&DAT_00583210 + param_4 * 0x50),FUN_0046623d);
    return;
}



i32 *  FUN_004662c9(param_1: i32,param_2: i32,param_3: i32,param_4: u32,param_5: *mut i32)

{
let mut iVar1: i32;
let piVar2: *mut i32;;
let mut puVar3: *mut u8;
let mut local_28: u32;
let local_20: *mut i32;;

iVar1 = *(&DAT_004daab1 + param_2 * 0x41 + param_1 * 0x3890 + param_3) >> 0x18;
if (iVar1 != -0x1) {
if (param_4 < 0x13) {
if (param_4 == 0x12) {
if (iVar1 == 0x1a) {
param_5[0x3] = param_5[0x3] + 0x14;
}
else {
if (iVar1 == 0x1b) {
param_5[0xb] = param_5[0xb] + 0x5;
}
else {
if (iVar1 == 0x1e) {
param_5[0x2] = param_5[0x2] + 0x14;
}
}
}
}
}
else {
if (param_4 < 0x14) {
if (iVar1 == 0x1f) {
param_5[0x1] = param_5[0x1] + 0x14;
}
}
else {
if ((0x17 < param_4) && ((param_4 < 0x19 || (param_4 == 0x19)))) {
if (iVar1 == 0x1c) {
param_5[0x4] = param_5[0x4] + 0xa;
}
else {
if (iVar1 == 0x1d) {
*param_5 = *param_5 + 0x14;
}
}
}
}
}
}
local_28 = *(*(&DAT_004d7d50 + param_2 * 0x4 + param_1 * 0x3890) + param_3 * 0x4);
if ((local_28 & 0xf) == 0xb) {
local_28 = local_28 >> 0x9;
}
puVar3 = &DAT_00583c3c + (local_28 & 0xf) * 0x50 + param_4 * 0x384 +
(*(&DAT_004daab0 + param_1 * 0x3890) * 0x10);
piVar2 = (*(&DAT_004daab0 + param_1 * 0x3890) * 0x10);
for (local_20 = 0x0; (local_20 < 0x4 && (piVar2 = (puVar3 + local_20 * 0x4), *piVar2 != -0x1))
; local_20 = (local_20 + 0x1)) {
param_5[*(puVar3 + local_20 * 0x4) >> 0x10] =
param_5[*(puVar3 + local_20 * 0x4) >> 0x10] + (puVar3 + local_20 * 0x4);
piVar2 = local_20;
}
return piVar2;
}



fn FUN_0046645d(param_1: i32,param_2: i32,param_3: i32) -> i32

{
let puVar1: *mut u32;

puVar1 = FUN_00481784(param_1,param_2,param_3);
return ((puVar1 + 0x8) * (*(puVar1 + 0x26) >> 0x10)) / 0x64;
}



fn FUN_004664c9(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: i32,param_6: i32) -> u32

{
    let mut local_18: u32;
    i32 **local_14;

    if (*(*(&DAT_00582938 + param_5 * 0x4 + param_4 * 0x18) + 0xed) == -0x1) {
        *(param_6 + 0x22) = 0xffff;
        *(param_6 + 0x24) = 0xffff;
        *(param_6 + 0x1e) = 0x4b;
        *(param_6 + 0x30) = 0x64;
        local_18 = 0x1;
    }
    else {
        for (local_14 = *(i32 ***)(&DAT_005b8b44 + param_1 * 0x4);
            (local_14 != (i32 **)0x0 && ((local_14 + 0x8) == param_1)); local_14 = (i32 **)*local_14) {
            if ((((*(local_14 + 0x3a) & 0x80000000) == 0x0) &&
                ((((local_14 + 0x22) == param_2 && ((local_14 + 0x9) == param_3)) &&
                    ((*(local_14 + 0x3a) & 0x1) != 0x0)))) &&
                ((local_14[0x9] >> 0x18 == *(*(&DAT_00582938 + param_4 * 0x18 + param_5 * 0x4) + 0xed) &&
                    ((local_14 + 0xa) == '\0')))) {
                (param_6 + 0x22) = (local_14 + 0x27);
                (param_6 + 0x24) = (local_14 + 0xa);
                (param_6 + 0x1e) = (local_14 + 0x29);
                (param_6 + 0x30) = (local_14 + 0x33);
                FUN_00484b4e(local_14);
                return 0x1;
            }
        }
        local_18 = 0x0;
    }
    return local_18;
}



fn FUN_004666a6() -> i32

{
let mut local_1c: i32;
let mut local_18: i32;
let mut local_14: i32;

local_1c = 0x0;
local_18 = 0x0;
for (local_14 = 0x1; local_14 < 0x72; local_14 = local_14 + 0x1) {
if ((((&DAT_00569c30)[local_14 * 0x9 + DAT_004c9754 * 0x1e22] & 0x1) != 0x0) &&
(local_18 < *(&DAT_0058ad6a + local_14 * 0xda))) {
local_18 = *(&DAT_0058ad6a + local_14 * 0xda);
local_1c = local_14;
}
}
return local_1c;
}



fn FUN_0046672b() -> *mut u32

{
    let puVar1: *mut u32;
    i32 aiStackY2616 [0x4c];
    let mut iStack2296: i32;
    let mut iStack2292: i32;
    let mut iStack2256: i32;
    let mut iStack2228: i32;
    let local_8a4: *mut u32;
    let local_8a0: *mut u32;
    let mut local_870: i32;
    let local_86c: *mut u32;
    let mut local_83c: i32;
    i32 local_838 [0x208];
    let local_18: *mut u32;
    let mut local_14: i32;

    FUN_004a0430(local_838,0x0,0x820);
    for (local_14 = 0x0; local_14 < 0x28; local_14 = local_14 + 0x1) {
        for (local_83c = 0x0; local_83c < 0xd; local_83c = local_83c + 0x1) {
            *(&DAT_00568210 + DAT_004c9754 * 0x1e22 + local_83c * 0x4 + local_14 * 0x9d + 0x34) =
                *(&DAT_00568210 + DAT_004c9754 * 0x1e22 + local_83c * 0x4 + local_14 * 0x9d + 0x68);
        }
    }
    if (DAT_004c9760 == 0x0) {
        for (local_18 = *DAT_005967b0; local_18 != 0x0; local_18 = *local_18) {
            if (((local_18 + 0x27) == '[') && (*(local_18 + 0x23) >> 0x18 == DAT_004c9754)) {
                local_838[(local_18 + 0x8) * 0xd + (*(local_18 + 0x2d) >> 0x18)] =
                    local_838[(local_18 + 0x8) * 0xd + (*(local_18 + 0x2d) >> 0x18)] +
                        (*(local_18 + 0x2f) >> 0x10);
            }
        }
        for (local_8a4 = 0x0; local_8a4 < 0x28; local_8a4 = (local_8a4 + 0x1)) {
            for (iStack2228 = 0x0; iStack2228 < 0xd; iStack2228 = iStack2228 + 0x1) {
                *(&DAT_00568210 + DAT_004c9754 * 0x1e22 + iStack2228 * 0x4 + local_8a4 * 0x9d + 0x68) =
                    local_838[iStack2228 + local_8a4 * 0xd];
            }
        }
        puVar1 = FUN_004a0430(local_838,0x0,0x820);
        for (local_8a4 = 0x0; local_8a4 < 0x28; local_8a4 = (local_8a4 + 0x1)) {
            for (iStack2256 = 0x0; iStack2256 < 0xd; iStack2256 = iStack2256 + 0x1) {
                local_838[iStack2256] =
                    *(&DAT_00568210 + DAT_004c9754 * 0x1e22 + iStack2256 * 0x4 + local_8a4 * 0x9d + 0x34);
                iStack2292 = iStack2256;
                iStack2296 = *(&DAT_00568210 + DAT_004c9754 * 0x1e22 + iStack2256 * 0x4 + local_8a4 * 0x9d + 0x68);
                local_838[iStack2256 + 0xd] =
                    *(&DAT_00568210 + DAT_004c9754 * 0x1e22 + iStack2256 * 0x4 + local_8a4 * 0x9d + 0x68);
            }
            puVar1 = local_8a4;
        }
        for (local_8a0 = 0x0; local_8a0 < 0xd; local_8a0 = (local_8a0 + 0x1)) {
            FUN_004626f4((&DAT_00568210 + DAT_004c9754 * 0x1e22),local_8a0,local_838[local_8a0],
                         local_838[local_8a0 + 0xd]);
            puVar1 = local_8a0;
        }
    }
    else {
        for (local_18 = *DAT_005967b0; local_18 != 0x0; local_18 = *local_18) {
            if (((local_18 + 0x27) == '[') && (*(local_18 + 0x23) >> 0x18 == DAT_004c9754)) {
                local_838[*(local_18 + 0x2d) >> 0x18] =
                    local_838[*(local_18 + 0x2d) >> 0x18] + (*(local_18 + 0x2f) >> 0x10);
            }
        }
        puVar1 = local_18;
        for (local_86c = 0x0; local_86c < 0xd; local_86c = (local_86c + 0x1)) {
            for (local_870 = 0x0; local_870 < 0x28; local_870 = local_870 + 0x1) {
                *(&DAT_00568210 + DAT_004c9754 * 0x1e22 + local_86c * 0x4 + local_870 * 0x9d + 0x68) =
                    local_838[local_86c];
            }
            FUN_004626f4((&DAT_00568210 + DAT_004c9754 * 0x1e22),local_86c,
                         *(&DAT_00568210 + local_86c * 0x4 + DAT_004c9754 * 0x1e22 + 0x34),
                         local_838[local_86c]);
            puVar1 = local_86c;
        }
    }
    return puVar1;
}



fn FUN_00466d5a(param_1: i32,param_2: i32,param_3: i32,param_4: i32)

{
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0xd; local_14 = local_14 + 0x1) {
        if (*(*(&DAT_00582938 + param_3 * 0x4 + param_2 * 0x18) + local_14 * 0x4 + 0xb9) != 0x0) {
            FUN_0046556d(param_1,local_14,
                         *(local_14 * 0x4 + *(&DAT_00582938 + param_2 * 0x18 + param_3 * 0x4) + 0xb9),param_4);
        }
    }
    return;
}



fn FUN_00466de5(param_1: i32,param_2: i32,param_3: i32,param_4: u32) -> u32

{
    let mut iVar1: i32;
    let mut pcVar2: String;
    let mut uVar3: u32;
    let piVar4: *mut i32;;
    let mut local_50: u32;
    let mut local_4c: u32;
    let mut local_44: u32;
    let mut local_40: u32;
    let mut local_3c: u32;
    let mut local_2c: u32;
    let mut local_28: u32;
    let mut local_1c: i32;
    let mut local_18: i32;

    DAT_005b4a48 = &DAT_004d7d50 + param_1 * 0x3890;
    iVar1 = FUN_004674aa();
    if (iVar1 == 0x0) {
        pcVar2 = FUN_00499050(DAT_0059679c,0x7394);
        pop_err_msg_box_and_exit_004a02f5(pcVar2);
    }
    iVar1 = DAT_005b4a34;
    uVar3 = FUN_004a2edc();
    piVar4 = (DAT_005b4a38 + (uVar3 % iVar1) * 0xac);
    *(&DAT_004daab0 + param_1 * 0x3890) = *piVar4;
    DAT_005b4a4c = &DAT_005653d0 + *piVar4 * 0x940;
    if (param_2 != 0x0) {
        *(param_2 + 0xad) = *piVar4;
        (param_2 + 0xb1) = DAT_005b4a4c;
        *(param_2 + 0xb5) = *piVar4 * 0x940 + 0x565c90;
    }
    if (param_3 != 0x0) {
        *(param_3 + 0xad) = *piVar4;
        (param_3 + 0xb1) = DAT_005b4a4c;
        *(param_3 + 0xb5) = *piVar4 * 0x940 + 0x565c90;
    }
    DAT_005b4a3c = FUN_0049c2c9(0x2cb0);
    DAT_005b4a40 = FUN_0049c2c9(0x2cb0);
    DAT_005b4a44 = FUN_0049c2c9(0x2cb0);
    local_1c = 0x0;
    local_18 = 0x0;
    for (local_2c = 0x0; local_2c < 0x5; local_2c = local_2c + 0x1) {
        if ((local_2c != 0x0) && (piVar4[local_1c + 0x1] < piVar4[local_2c + 0x1])) {
            local_1c = local_2c;
        }
        local_18 = local_18 + piVar4[local_2c + 0x1];
    }
    if (local_18 < 0x32) {
        for (local_2c = 0x0; local_2c < 0x2c; local_2c = local_2c + 0x1) {
            local_40 = ((local_2c & 0x1) == 0x0);
            for (local_28 = local_40; local_28 < (0x41 - (local_2c & 0x1)); local_28 = local_28 + 0x2) {
                *(*(DAT_005b4a48 + local_2c * 0x4) + local_28 * 0x4) = 0x0;
                *(*(DAT_005b4a48 + local_2c * 0x4) + local_28 * 0x4 + 0x4) = 0x0;
            }
        }
        FUN_00467c94(piVar4,local_1c + 0x1,local_18,0x0);
        FUN_00467aaf(local_1c,0x0);
        FUN_00467aaf(0x0,local_1c);
        FUN_00467aaf(local_1c,0x0);
    }
    else {
        for (local_2c = 0x0; local_2c < 0x2c; local_2c = local_2c + 0x1) {
            local_3c = ((local_2c & 0x1) == 0x0);
            for (local_28 = local_3c; local_28 < (0x41 - (local_2c & 0x1)); local_28 = local_28 + 0x2) {
                *(local_28 * 0x4 + *(DAT_005b4a48 + local_2c * 0x4)) = local_1c + 0x1;
                *(*(DAT_005b4a48 + local_2c * 0x4) + local_28 * 0x4 + 0x4) = 0x0;
            }
        }
        if (local_18 < 0x64) {
            FUN_00467c94(piVar4,0x0,0x64 - local_18,0x0);
            FUN_00467aaf(0x0,0x1);
            FUN_00467aaf(0x1,0x0);
            FUN_00467aaf(0x0,0x1);
        }
    }
    for (local_2c = 0x0; local_2c < 0x5; local_2c = local_2c + 0x1) {
        if (local_2c != local_1c) {
            FUN_00467c94(piVar4,local_2c + 0x1,piVar4[local_2c + 0x1],local_1c * 0x4 + 0x4U | 0x2);
            FUN_00467aaf(0x1,local_2c);
            FUN_00467aaf(local_2c,0x1);
        }
    }
    FUN_00467aaf(0x1,-0x1);
    for (local_2c = 0x0; local_2c < 0x2c; local_2c = local_2c + 0x1) {
        local_44 = ((local_2c & 0x1) == 0x0);
        for (local_28 = local_44; local_28 < (0x41 - (local_2c & 0x1)); local_28 = local_28 + 0x2) {
            uVar3 = *(*(&DAT_004d7d50 + param_1 * 0x3890 + local_2c * 0x4) + local_28 * 0x4);
            *(*(DAT_005b4a48 + local_2c * 0x4) + local_28 * 0x4) =
                uVar3 | *(DAT_005b4a4c + uVar3 * 0x8c + 0x88) << 0x4;
        }
    }
    for (local_2c = 0x0; local_2c < 0x2c; local_2c = local_2c + 0x1) {
        local_4c = ((local_2c & 0x1) == 0x0);
        for (local_28 = local_4c; local_28 < (0x41 - (local_2c & 0x1)); local_28 = local_28 + 0x2) {
            FUN_00439559((&DAT_004d7d50 + param_1 * 0x3890),local_2c,local_28,0x0,0x0);
        }
    }
    FUN_00468353(piVar4,0x8);
    for (local_2c = 0x0; local_2c < 0x2c; local_2c = local_2c + 0x1) {
        local_50 = ((local_2c & 0x1) == 0x0);
        for (local_28 = local_50; local_28 < (0x41 - (local_2c & 0x1)); local_28 = local_28 + 0x2) {
            FUN_00439559((&DAT_004d7d50 + param_1 * 0x3890),local_2c,local_28,0x0,0x0);
        }
    }
    FUN_00468353(piVar4,0x6);
    FUN_00468353(piVar4,0x7);
    for (local_2c = 0x0; local_2c < 0x2c; local_2c = local_2c + 0x1) {
        for (local_28 = ((local_2c & 0x1) == 0x0); local_28 < (0x41 - (local_2c & 0x1));
            local_28 = local_28 + 0x2) {
            FUN_00439559((&DAT_004d7d50 + param_1 * 0x3890),local_2c,local_28,0x0,0x0);
        }
    }
    if (local_18 < 0x64) {
        FUN_00468b26(param_1,piVar4[0x2a]);
    }
    FUN_0046ae9f(param_1);
    FUN_0046b5c5(param_1,param_4);
    FUN_0049af50(DAT_005b4a44);
    FUN_0049af50(DAT_005b4a40);
    FUN_0049af50(DAT_005b4a3c);
    FUN_0049af50(DAT_005b4a38);
    DAT_005b4a34 = 0x0;
    DAT_005b4a38 = 0x0;
    return 0x1;
}



fn FUN_004674aa() -> u32

{
    let mut bVar1: bool;
    let mut pcVar2: String;
    byte *pbVar3;
    let mut iVar4: i32;
    undefined3 extraout_var;
    byte *pbVar5;
    let local_124: u8 [0x100];
    let mut local_24: i32;
    let mut local_20: u32;
    byte **local_1c;
    let mut local_18: u32;
    let mut local_14: i32;

    local_24 = 0x0;
    local_14 = 0x0;
    local_18 = 0x0;
    if ((DAT_005b4a34 == 0x0) || (DAT_005b4a38 == 0x0)) {
        local_1c = (byte **)FUN_0049c4bd(PTR_s_RAND_PLANETS_TXT_004bf704,&DAT_004c2202);
        if (local_1c == (byte **)0x0) {
            local_20 = 0x0;
        }
        else {
            loop {
                loop {
                    while( true ) {
                        pcVar2 = FUN_004a2f60(local_124,0xff,local_1c);
                        if (pcVar2 == 0x0)^ // goto LAB_00467667;
                        pbVar3 = FUN_00467684(local_124);
                        if (local_14 != 0x0) break;
                        iVar4 = FUN_004a9800(pbVar3,s_COUNT_004c2205,0x5);
                        if (iVar4 == 0x0) {
                            loop {
                                pbVar5 = pbVar3;
                                if (*pbVar3 == 0x3d)^ // goto LAB_00467595;
                                if (*pbVar3 == 0x0) break;
                                pbVar5 = pbVar3 + 0x1;
                                if (*pbVar5 == 0x3d)^ // goto LAB_00467595;
                                pbVar3 = pbVar3 + 0x2;
                            } while (*pbVar5 != 0x0);
                            pbVar5 = 0x0;
                            LAB_00467595:
                            if (pbVar5 != 0x0) {
                                DAT_005b4a34 = FUN_004a1e60(pbVar5 + 0x1);
                                if (DAT_005b4a34 == 0x0)^ // goto LAB_00467667;
                                local_14 = 0x1;
                                DAT_005b4a38 = FUN_0049c2c9(DAT_005b4a34 * 0xac);
                                if (DAT_005b4a38 == 0x0) {
                                    DAT_005b4a34 = 0x0;^
                                    // goto LAB_00467667;
                                }
                            }
                        }
                    }
                } while ((local_14 != 0x1) || (((&DAT_004bf9c4)[(byte)(*pbVar3 + 0x1)] & 0x20) == 0x0));
                bVar1 = FUN_00467704(pbVar3,(DAT_005b4a38 + local_24 * 0xac));
                if (CONCAT31(extraout_var,bVar1) != 0x0) {
                    local_24 = local_24 + 0x1;
                }
            } while (local_24 != DAT_005b4a34);
            local_18 = 0x1;
            LAB_00467667:
                FUN_0049ca40(local_1c);
            local_20 = local_18;
        }
    }
    else {
        local_20 = 0x1;
    }
    return local_20;
}



char *  FUN_00467684(param_1: &mut String)

{
let cVar1: u8;
let mut uVar2: u32;
let mut pcVar3: String;
let mut local_18: String;
let mut local_14: String;

for (local_18 = param_1; *local_18 == ' '; local_18 = local_18 + 0x1) {
}
uVar2 = 0xffffffff;
pcVar3 = local_18;
loop {
if (uVar2 == 0x0) break;
uVar2 = uVar2 - 0x1;
cVar1 = *pcVar3;
pcVar3 = pcVar3 + 0x1;
} while (cVar1 != '\0');
local_14 = local_18 + (~uVar2 - 0x1);
if (local_14 != local_18) {
while (((local_14 = local_14 + -0x1, *local_14 == '\n' || (*local_14 == '\r')) || (*local_14 == ' '))) {
*local_14 = '\0';
}
}
return local_18;
}



bool  FUN_00467704(byte *param_1,param_2: *mut i32)

{
byte *pbVar1;
let mut local_14: i32;

param_1 = FUN_00467940(param_1,param_2);
for (local_14 = 0x0; local_14 < 0x5; local_14 = local_14 + 0x1) {
pbVar1 = FUN_00467940(param_1,param_2 + local_14 + 0x1);
pbVar1 = FUN_00467940(pbVar1,param_2 + local_14 + 0x6);
param_1 = FUN_00467940(pbVar1,param_2 + local_14 + 0xb);
}
pbVar1 = FUN_00467940(param_1,param_2 + 0x10);
pbVar1 = FUN_00467940(pbVar1,param_2 + 0x11);
param_1 = FUN_00467940(pbVar1,param_2 + 0x12);
for (local_14 = 0x0; local_14 < 0x5; local_14 = local_14 + 0x1) {
param_1 = FUN_004679f5(param_1,param_2 + local_14 + 0x13);
}
pbVar1 = FUN_00467940(param_1,param_2 + 0x21);
pbVar1 = FUN_00467940(pbVar1,param_2 + 0x22);
param_1 = FUN_00467940(pbVar1,param_2 + 0x23);
for (local_14 = 0x0; local_14 < 0x6; local_14 = local_14 + 0x1) {
param_1 = FUN_004679f5(param_1,param_2 + local_14 + 0x24);
}
pbVar1 = FUN_00467940(param_1,param_2 + 0x18);
pbVar1 = FUN_00467940(pbVar1,param_2 + 0x19);
param_1 = FUN_00467940(pbVar1,param_2 + 0x1a);
for (local_14 = 0x0; local_14 < 0x6; local_14 = local_14 + 0x1) {
param_1 = FUN_004679f5(param_1,param_2 + local_14 + 0x1b);
}
pbVar1 = FUN_00467940(param_1,param_2 + 0x2a);
return pbVar1 != 0x0;
}



byte *  FUN_00467940(byte *param_1,param_2: *mut i32)

{
let mut iVar1: i32;
byte *local_18;
byte *local_14;

if ((param_1 == 0x0) || (*param_1 == 0x0)) {
local_18 = 0x0;
}
else {
for (local_14 = param_1; (((&DAT_004bf9c4)[(byte)(*local_14 + 0x1)] & 0x20) != 0x0 || (*local_14 == 0x2d));
local_14 = local_14 + 0x1) {
}
*local_14 = 0x0;
if (*param_1 == 0x0) {
local_18 = 0x0;
}
else {
iVar1 = FUN_004a1e60(param_1);
*param_2 = iVar1;
loop {
loop {
param_1 = local_14 + 0x1;
local_14 = param_1;
} while (*param_1 == 0x20);
} while ((*param_1 == 0x9) || (*param_1 == 0x2c));
local_18 = param_1;
}
}
return local_18;
}



char *  FUN_004679f5(param_1: &mut String,param_2: *mut u32)

{
let mut iVar1: i32;
let mut local_14: String;

if ((param_1 == 0x0) || (*param_1 == '\0')) {
local_14 = 0x0;
}
else {
iVar1 = FUN_004a11c0(*param_1);
if (iVar1 == 0x59) {
*param_2 = 0x1;
}
else {
iVar1 = FUN_004a11c0(*param_1);
if (iVar1 != 0x4e) {
return 0x0;
}
*param_2 = 0x0;
}
for (; (*param_1 != ' ' && (*param_1 != ',')); param_1 = param_1 + 0x1) {
}
for (; ((*param_1 == ' ' || (*param_1 == '\t')) || (*param_1 == ',')); param_1 = param_1 + 0x1) {
}
local_14 = param_1;
}
return local_14;
}



fn FUN_00467aaf(param_1: i32,param_2: i32)

{
    let mut iVar1: i32;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let mut local_40: i32;
    let mut local_3c: i32;
    let mut local_38: i32;
    let mut local_34: u32;
    let mut local_30: u32;
    i32 local_2c [0x6];
    let mut local_14: u32;

    for (local_14 = 0x0; local_14 < 0x2c; local_14 = local_14 + 0x1) {
        local_30 = ((local_14 & 0x1) == 0x0);
        for (local_34 = local_30; local_34 < (0x41 - (local_14 & 0x1)); local_34 = local_34 + 0x1) {
            if (*(*(local_14 * 0x4 + DAT_005b4a48) + local_34 * 0x4) == param_1) {
                local_40 = param_2;
                local_3c = 0x0;
                FUN_004a0430(local_2c,0x0,0x18);
                for (local_38 = 0x0; local_38 < 0x6; local_38 = local_38 + 0x1) {
                    uVar2 = FUN_0043a8a2(local_14 + (&DAT_004bea60)[local_38]);
                    uVar3 = FUN_0043a8d5(uVar2,local_34 + (&DAT_004bea7c)[local_38]);
                    if (((uVar2 != local_14) || (uVar3 != local_34)) &&
                        (iVar1 = *(*(uVar2 * 0x4 + DAT_005b4a48) + uVar3 * 0x4), iVar1 != param_1)) {
                        local_3c = local_3c + 0x1;
                        local_2c[iVar1] = local_2c[iVar1] + 0x1;
                        if ((0x2 < local_2c[iVar1]) || (local_40 == -0x1)) {
                            local_40 = iVar1;
                        }
                    }
                }
                if ((((local_3c == 0x4) && (uVar2 = FUN_004a2edc(), uVar2 % 0x80 < 0x46)) ||
                    ((local_3c == 0x5 && (uVar2 = FUN_004a2edc(), uVar2 % 0x80 < 0x64)))) || (local_3c == 0x6)) {
                    *(*(local_14 * 0x4 + DAT_005b4a48) + local_34 * 0x4) = local_40;
                }
            }
        }
    }
    return;
}



fn FUN_00467c94(param_1: i32,param_2: i32,param_3: i32,param_4: u32)

{
    let mut uVar1: u32;
    let mut bVar2: bool;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut uVar5: u32;
    let piVar6: *mut i32;;
    undefined3 extraout_var;
    let mut uVar7: u32;
    undefined3 extraout_var_00;
    let mut iVar8: i32;
    let mut local_78: u32;
    let mut local_6c: u32;
    let mut local_5c: u32;
    let mut local_58: u32;
    let mut local_3c: i32;
    let mut local_38: i32;
    let mut local_34: i32;
    let mut local_30: i32;

    if (param_3 != 0x0) {
        iVar3 = (param_4 & 0x1c) >> 0x2;
        iVar4 = (param_3 * 0xb2c) / 0xc8;
        uVar1 = *(param_2 * 0x4 + param_1 + 0x14);
        if (param_2 == 0x0) {
            local_3c = *(*(*DAT_005b4a48 + 0x4) * 0x4 + param_1 + 0x28);
        }
        else {
            local_3c = *(param_2 * 0x4 + param_1 + 0x28);
        }
        if (iVar4 < local_3c) {
            local_3c = iVar4;
        }
        local_34 = 0x0;
        local_38 = 0x0;
        local_30 = 0x0;
        loop {
            if (local_3c <= local_30)^ // goto LAB_0046802f;
            uVar5 = FUN_004a2edc();
            *(local_30 * 0x4 + DAT_005b4a3c) = uVar5 % 0x2c;
            if (((param_4 & 0x2) == 0x0) || ((uVar1 & 0x1) == 0x0)) {
                if (((param_4 & 0x2) == 0x0) || ((uVar1 & 0x2) == 0x0)) {
                    local_78 = ((*(local_30 * 0x4 + DAT_005b4a3c) & 0x1) == 0x0);
                    uVar5 = FUN_004a2edc();
                    *(DAT_005b4a40 + local_30 * 0x4) = local_78 + (uVar5 % 0x20) * 0x2;
                }
                else {
                    local_6c = ((*(local_30 * 0x4 + DAT_005b4a3c) & 0x1) == 0x0);
                    uVar5 = FUN_004a2edc();
                    *(local_30 * 0x4 + DAT_005b4a40) = local_6c + (uVar5 % 0x10) * 0x2;
                    piVar6 = (local_30 * 0x4 + DAT_005b4a40);
                    *piVar6 = *piVar6 + 0x10;
                }
            }
            else {
                uVar5 = FUN_004a2edc();
                if (uVar5 % 0x2 == 0x0) {
                    *(local_30 * 0x4 + DAT_005b4a40) = 0x40;
                    if ((*(DAT_005b4a40 + local_30 * 0x4) & 0x1) == 0x0) {
                        piVar6 = (DAT_005b4a40 + local_30 * 0x4);
                        local_5c = ((*(DAT_005b4a3c + local_30 * 0x4) & 0x1) == 0x0);
                        *piVar6 = *piVar6 - local_5c;
                    }
                    else {
                        piVar6 = (local_30 * 0x4 + DAT_005b4a40);
                        *piVar6 = *piVar6 - (*(local_30 * 0x4 + DAT_005b4a3c) & 0x1);
                    }
                }
                else {
                    local_58 = ((*(local_30 * 0x4 + DAT_005b4a3c) & 0x1) == 0x0);
                    *(local_30 * 0x4 + DAT_005b4a40) = local_58;
                }
            }
            iVar8 = *(DAT_005b4a48[*(local_30 * 0x4 + DAT_005b4a3c)] +
                *(DAT_005b4a40 + local_30 * 0x4) * 0x4);
            if (((((param_4 & 0x1) == 0x0) || (iVar8 != 0x0)) &&
                (((iVar3 != 0x0 && (iVar8 == iVar3)) || ((iVar3 == 0x0 && (iVar8 != param_2)))))) &&
                (((param_4 & 0x2) == 0x0 ||
                    (bVar2 = FUN_00468208(uVar1,*(DAT_005b4a3c + local_30 * 0x4),*(local_30 * 0x4 + DAT_005b4a40))
                     , CONCAT31(extraout_var,bVar2) != 0x0)))) {
                *(*(DAT_005b4a40 + local_30 * 0x4) * 0x4 + DAT_005b4a48[*(local_30 * 0x4 + DAT_005b4a3c)])
                    = param_2;
                local_30 = local_30 + 0x1;
            }
            local_34 = local_34 + 0x1;
        } while (local_34 < 0x1389);
        local_3c = local_30;
        LAB_0046802f:
        if (local_3c != 0x0) {
            local_34 = 0x0;
            while (local_38 < iVar4) {
                uVar5 = FUN_004a2edc();
                iVar8 = uVar5 % (local_3c + local_38);
                uVar5 = FUN_004a2edc();
                uVar7 = FUN_0043a8a2(*(DAT_005b4a3c + iVar8 * 0x4) + (&DAT_004bea60)[uVar5 % 0x6]);
                uVar5 = FUN_0043a8d5(uVar7,*(DAT_005b4a40 + iVar8 * 0x4) + (&DAT_004bea7c)[uVar5 % 0x6]);
                if (((((uVar7 != *(iVar8 * 0x4 + DAT_005b4a3c)) || (uVar5 != *(iVar8 * 0x4 + DAT_005b4a40))) &&
                    ((iVar8 = *(DAT_005b4a48[uVar7] + uVar5 * 0x4), (param_4 & 0x1) == 0x0 || (iVar8 != 0x0)))) &&
                    (((iVar3 != 0x0 && (iVar8 == iVar3)) || ((iVar3 == 0x0 && (iVar8 != param_2)))))) &&
                    (((param_4 & 0x2) == 0x0 || (bVar2 = FUN_00468208(uVar1,uVar7,uVar5), CONCAT31(extraout_var_00,bVar2) != 0x0)
                    ))) {
                    *(uVar5 * 0x4 + DAT_005b4a48[uVar7]) = param_2;
                    *((local_3c + local_38) * 0x4 + DAT_005b4a3c) = uVar7;
                    *(DAT_005b4a40 + (local_3c + local_38) * 0x4) = uVar5;
                    local_38 = local_38 + 0x1;
                }
                if (0xc350 < local_34) {
                    local_38 = iVar4;
                }
                local_34 = local_34 + 0x1;
            }
        }
    }
    return;
}



bool  FUN_00468208(param_1: u32,param_2: u32,param_3: u32)

{
let mut iVar1: i32;
let mut uVar2: u32;
let mut uVar3: u32;
let mut uVar4: u32;
let local_2c: u8;
let mut local_1c: bool;
let mut local_18: i32;
let mut local_14: i32;

local_18 = 0x0;
uVar4 = param_1 & 0xfffffffc;
if (uVar4 == 0x0) {
local_1c = true;
}
else {
for (local_14 = 0x0; local_14 < 0x6; local_14 = local_14 + 0x1) {
uVar2 = FUN_0043a8a2(param_2 + (&DAT_004bea60)[local_14]);
uVar3 = FUN_0043a8d5(uVar2,param_3 + (&DAT_004bea7c)[local_14]);
if ((uVar2 != param_2) || (uVar3 != param_3)) {
iVar1 = *(*(uVar2 * 0x4 + DAT_005b4a48) + uVar3 * 0x4);
local_2c = (byte)iVar1;
if ((uVar4 & 0x8 << (local_2c & 0x1f)) != 0x0) {
local_18 = local_18 + 0x1;
}
if ((iVar1 == 0x0) && ((param_1 & 0x4) != 0x0)) {
local_18 = local_18 + -0x1;
}
}
}
if (uVar4 < 0x5) {
local_1c = -0x1 < local_18;
}
else {
uVar4 = FUN_004a2edc();
local_1c = uVar4 % 0x3 < local_18;
}
}
return local_1c;
}



fn FUN_00468353(param_1: i32,param_2: u32)

{
    let mut iVar1: i32;
    let mut uVar2: u32;
    let mut bVar3: bool;
    let mut iVar4: i32;
    let mut uVar5: u32;
    let piVar6: *mut i32;;
    undefined3 extraout_var;
    let mut uVar7: u32;
    undefined3 extraout_var_00;
    let mut iVar8: i32;
    let mut iVar9: i32;
    let mut local_c0: u32;
    let mut local_b4: i32;
    let mut local_a0: i32;
    let mut local_80: u32;
    let mut local_74: u32;
    let mut local_64: u32;
    let mut local_60: u32;
    let mut local_50: u32;
    let mut local_44: i32;
    let mut local_40: i32;
    let mut local_3c: i32;
    let mut local_38: i32;
    let mut local_30: i32;
    let mut local_24: i32;
    let mut local_20: u32;
    let mut local_1c: i32;

    if (param_2 < 0x7) {
        if (param_2 == 0x6) {
            local_24 = *(param_1 + 0x84);
            local_44 = *(param_1 + 0x8c);
            local_20 = *(param_1 + 0x88);
            local_1c = param_1 + 0x90;
        }
    }
    else {
        if (param_2 < 0x8) {
            local_24 = *(param_1 + 0x60);
            local_44 = *(param_1 + 0x68);
            local_20 = *(param_1 + 0x64);
            local_1c = param_1 + 0x6c;
        }
        else {
            if (param_2 == 0x8) {
                local_24 = *(param_1 + 0x40);
                local_44 = *(param_1 + 0x48);
                local_20 = *(param_1 + 0x44);
                local_1c = param_1 + 0x4c;
            }
        }
    }
    if (local_24 != 0x0) {
        iVar4 = (local_24 * 0xb2c) / 0xc8;
        if (iVar4 < local_44) {
            local_44 = iVar4;
        }
        local_3c = 0x0;
        local_40 = 0x0;
        local_38 = 0x0;
        loop {
            if (local_44 <= local_38)^ // goto LAB_004687a1;
            uVar5 = FUN_004a2edc();
            *(local_38 * 0x4 + DAT_005b4a3c) = uVar5 % 0x2c;
            if ((local_20 & 0x1) == 0x0) {
                if ((local_20 & 0x2) == 0x0) {
                    local_80 = ((*(local_38 * 0x4 + DAT_005b4a3c) & 0x1) == 0x0);
                    uVar5 = FUN_004a2edc();
                    *(DAT_005b4a40 + local_38 * 0x4) = local_80 + (uVar5 % 0x20) * 0x2;
                }
                else {
                    local_74 = ((*(local_38 * 0x4 + DAT_005b4a3c) & 0x1) == 0x0);
                    uVar5 = FUN_004a2edc();
                    *(local_38 * 0x4 + DAT_005b4a40) = local_74 + (uVar5 % 0x10) * 0x2;
                    piVar6 = (local_38 * 0x4 + DAT_005b4a40);
                    *piVar6 = *piVar6 + 0x10;
                }
            }
            else {
                uVar5 = FUN_004a2edc();
                if (uVar5 % 0x2 == 0x0) {
                    *(local_38 * 0x4 + DAT_005b4a40) = 0x40;
                    if ((*(DAT_005b4a40 + local_38 * 0x4) & 0x1) == 0x0) {
                        piVar6 = (DAT_005b4a40 + local_38 * 0x4);
                        local_64 = ((*(DAT_005b4a3c + local_38 * 0x4) & 0x1) == 0x0);
                        *piVar6 = *piVar6 - local_64;
                    }
                    else {
                        piVar6 = (local_38 * 0x4 + DAT_005b4a40);
                        *piVar6 = *piVar6 - (*(local_38 * 0x4 + DAT_005b4a3c) & 0x1);
                    }
                }
                else {
                    local_60 = ((*(local_38 * 0x4 + DAT_005b4a3c) & 0x1) == 0x0);
                    *(local_38 * 0x4 + DAT_005b4a40) = local_60;
                }
            }
            uVar5 = *(*(*(local_38 * 0x4 + DAT_005b4a3c) * 0x4 + DAT_005b4a48) +
                *(DAT_005b4a40 + local_38 * 0x4) * 0x4);
            local_50 = uVar5 & 0xf;
            if (0x5 < local_50) {
                if ((local_50 == 0x8) && (param_2 != 0x8)) {
                    local_50 = 0x6;
                }
                else {
                    local_50 = 0x0;
                }
            }
            if (((local_50 != 0x0) && (*(local_50 * 0x4 + local_1c + -0x4) != 0x0)) &&
                (bVar3 = FUN_00468208(local_20,*(DAT_005b4a3c + local_38 * 0x4),
                                      *(local_38 * 0x4 + DAT_005b4a40)), CONCAT31(extraout_var,bVar3) != 0x0)) {
                *(*(DAT_005b4a48 + *(local_38 * 0x4 + DAT_005b4a3c) * 0x4) +
                    *(local_38 * 0x4 + DAT_005b4a40) * 0x4) =
                    param_2 | *(DAT_005b4a4c + param_2 * 0x8c + 0x88) << 0x4 | uVar5 << 0x9;
                if (param_2 != 0x8) {
                    uVar5 = FUN_004a2edc();
                    *(local_38 * 0x4 + DAT_005b4a44) = uVar5 % 0x6;
                }
                local_38 = local_38 + 0x1;
            }
            local_3c = local_3c + 0x1;
        } while (local_3c < 0x1389);
        local_44 = local_38;
        LAB_004687a1:
        if (local_44 != 0x0) {
            local_3c = 0x0;
            while (local_40 < iVar4) {
                uVar5 = FUN_004a2edc();
                iVar8 = uVar5 % (local_44 + local_40);
                if (param_2 == 0x8) {
                    uVar5 = FUN_004a2edc();
                    local_30 = uVar5 % 0x6;
                }
                else {
                    uVar5 = FUN_004a2edc();
                    iVar9 = uVar5 % 0x8;
                    local_b4 = *(DAT_005b4a44 + iVar8 * 0x4);
                    iVar1 = *(iVar8 * 0x4 + DAT_005b4a44);
                    local_a0 = iVar1 + 0x3;
                    if (0x5 < local_a0) {
                        local_a0 = iVar1 + -0x3;
                    }
                    local_30 = local_b4;
                    if (iVar9 != 0x7) {
                        if (iVar9 == 0x6) {
                            local_30 = local_a0;
                        }
                        else {
                            local_30 = iVar9;
                            if ((iVar9 != local_a0) && (iVar9 != local_b4)) {
                                uVar5 = FUN_004a2edc();
                                if (uVar5 % 0x5 != 0x0) {
                                    if (0x3 < uVar5 % 0x5) {
                                        local_b4 = local_a0;
                                    }
                                    local_30 = local_b4;
                                }
                            }
                        }
                    }
                }
                uVar5 = FUN_0043a8a2(*(DAT_005b4a3c + iVar8 * 0x4) + (&DAT_004bea60)[local_30]);
                uVar7 = FUN_0043a8d5(uVar5,*(DAT_005b4a40 + iVar8 * 0x4) + (&DAT_004bea7c)[local_30]);
                if ((uVar5 != *(iVar8 * 0x4 + DAT_005b4a3c)) || (uVar7 != *(iVar8 * 0x4 + DAT_005b4a40))) {
                    uVar2 = *(*(uVar5 * 0x4 + DAT_005b4a48) + uVar7 * 0x4);
                    local_c0 = uVar2 & 0xf;
                    if (0x5 < local_c0) {
                        if ((local_c0 == 0x8) && (param_2 != 0x8)) {
                            local_c0 = 0x6;
                        }
                        else {
                            local_c0 = 0x0;
                        }
                    }
                    if ((((local_c0 != 0x0) && (local_c0 < 0x7)) && (*(local_c0 * 0x4 + local_1c + -0x4) != 0x0)) &&
                        (bVar3 = FUN_00468208(local_20,*(DAT_005b4a3c + local_38 * 0x4),
                                              *(local_38 * 0x4 + DAT_005b4a40)), CONCAT31(extraout_var_00,bVar3) != 0x0)) {
                        *(*(DAT_005b4a48 + uVar5 * 0x4) + uVar7 * 0x4) =
                            param_2 | uVar2 << 0x9 | *(DAT_005b4a4c + param_2 * 0x8c + 0x88) << 0x4;
                        *(DAT_005b4a3c + (local_44 + local_40) * 0x4) = uVar5;
                        *(DAT_005b4a40 + (local_44 + local_40) * 0x4) = uVar7;
                        if (param_2 != 0x8) {
                            *(DAT_005b4a44 + (local_44 + local_40) * 0x4) = local_30;
                        }
                        local_40 = local_40 + 0x1;
                    }
                }
                if (0xc350 < local_3c) {
                    local_40 = iVar4;
                }
                local_3c = local_3c + 0x1;
            }
        }
    }
    return;
}



fn FUN_00468b26(param_1: i32,param_2: i32)

{
    let mut bVar1: bool;
    let mut uVar2: u32;
    let mut uVar3: u32;
    let mut iVar4: i32;
    let mut uVar5: u32;
    let mut uVar6: u32;
    let mut uVar7: u32;
    let mut uVar8: u32;
    undefined3 extraout_var;
    let mut local_84: i32;
    let mut local_80: i32;
    let mut local_78: i32;
    let mut local_74: i32;
    let mut local_5c: i32;
    let mut local_44: i32;
    let mut local_24: u32;
    let mut local_18: i32;
    let mut local_14: i32;

    local_18 = 0x0;
    local_14 = 0xc350;
    loop {
    loop {
        if ((param_2 <= local_18) || (local_14 == 0x0)) {
            return;
        }
        local_14 = local_14 + -0x1;
        uVar2 = FUN_004a2edc();
        uVar2 = uVar2 % 0x2a + 0x1;
        uVar3 = FUN_004a2edc();
        local_24 = ((uVar2 & 0x1) == 0x0);
        iVar4 = (uVar3 % 0x1e) * 0x2 + 0x2 + local_24;
        uVar3 = *(*(uVar2 * 0x4 + DAT_005b4a48) + iVar4 * 0x4) & 0xf;
    } while (((uVar3 == 0x0) || (0x5 < uVar3)) && (uVar3 != 0x8));
    for (local_44 = 0x0; local_44 < 0x6; local_44 = local_44 + 0x1) {
        uVar3 = FUN_0043a8a2(uVar2 + (&DAT_004bea60)[local_44]);
        uVar5 = FUN_0043a8d5(uVar3,iVar4 + (&DAT_004bea7c)[local_44]);
        if ((*(*(uVar3 * 0x4 + DAT_005b4a48) + uVar5 * 0x4) & 0xf) == 0x0) break;
    }
    if (local_44 < 0x6) {
        uVar3 = FUN_004a2edc();
        local_5c = uVar3 % 0x6;
        loop {
            uVar5 = FUN_0043a8a2(uVar2 + (&DAT_004bea60)[local_5c]);
            uVar6 = FUN_0043a8d5(uVar5,iVar4 + (&DAT_004bea7c)[local_5c]);
            uVar7 = *(*(uVar5 * 0x4 + DAT_005b4a48) + uVar6 * 0x4) & 0xf;
            if (((uVar7 != 0x0) && (uVar7 < 0x6)) || (uVar7 == 0x8)) {
                local_74 = local_5c + 0x1;
                if (0x5 < local_74) {
                    local_74 = 0x0;
                }
                uVar7 = FUN_0043a8a2(uVar2 + (&DAT_004bea60)[local_74]);
                uVar8 = FUN_0043a8d5(uVar7,iVar4 + (&DAT_004bea7c)[local_74]);
                uVar7 = *(*(uVar7 * 0x4 + DAT_005b4a48) + uVar8 * 0x4) & 0xf;
                if (((uVar7 != 0x0) && (uVar7 < 0x6)) || (uVar7 == 0x8)) {
                    local_74 = local_5c + -0x1;
                    if (local_74 < 0x0) {
                        local_74 = 0x5;
                    }
                    uVar7 = FUN_0043a8a2(uVar2 + (&DAT_004bea60)[local_74]);
                    uVar8 = FUN_0043a8d5(uVar7,iVar4 + (&DAT_004bea7c)[local_74]);
                    uVar7 = *(*(uVar7 * 0x4 + DAT_005b4a48) + uVar8 * 0x4) & 0xf;
                    if (((uVar7 != 0x0) && (uVar7 < 0x6)) || (uVar7 == 0x8)) {
                        local_84 = 0x0;
                        local_80 = 0x0;
                        bVar1 = true;
                        local_74 = local_5c + 0x2;
                        if (0x5 < local_74) {
                            local_74 = local_5c + -0x4;
                        }
                        for (local_78 = 0x0; local_78 < 0x3; local_78 = local_78 + 0x1) {
                            uVar7 = FUN_0043a8a2(uVar2 + (&DAT_004bea60)[local_74]);
                            uVar8 = FUN_0043a8d5(uVar7,iVar4 + (&DAT_004bea7c)[local_74]);
                            uVar7 = *(*(uVar7 * 0x4 + DAT_005b4a48) + uVar8 * 0x4) & 0xf;
                            if (uVar7 == 0x0) {
                                local_80 = local_80 + 0x1;
                                if (bVar1) {
                                    local_84 = local_84 + 0x1;
                                }
                                bVar1 = false;
                            }
                            else {
                                if ((uVar7 == 0x9) || (uVar7 == 0xa)) {
                                    local_84 = local_84 + 0x3;
                                }
                                if (!bVar1) {
                                    local_84 = local_84 + 0x1;
                                }
                                bVar1 = true;
                            }
                            local_74 = local_74 + 0x1;
                            if (0x5 < local_74) {
                                local_74 = 0x0;
                            }
                        }
                        if (((local_80 != 0x0) && (local_84 < 0x3)) &&
                            (bVar1 = FUN_004690a0(uVar5,uVar6,0x1), CONCAT31(extraout_var,bVar1) != 0x0)) {
                            uVar7 = FUN_0043b921((&DAT_004d7d50 + param_1 * 0x3890),
                                                 *(*(DAT_005b4a48 + uVar2 * 0x4) + iVar4 * 0x4));
                            *(*(uVar2 * 0x4 + DAT_005b4a48) + iVar4 * 0x4) =
                                uVar7 << 0xb | *(DAT_005b4a4c + 0x600) << 0x4 | 0xa;
                            FUN_00439559((&DAT_004d7d50 + param_1 * 0x3890),uVar2,iVar4,0x1,0x0);
                            *(*(uVar5 * 0x4 + DAT_005b4a48) + uVar6 * 0x4) =
                                *(DAT_005b4a4c + 0x574) << 0x4 |
                                    *(*(uVar5 * 0x4 + DAT_005b4a48) + uVar6 * 0x4) << 0x9 | 0x9;
                            FUN_00439559((&DAT_004d7d50 + param_1 * 0x3890),uVar5,uVar6,0x1,0x0);
                            FUN_004691f7(param_1,uVar5,uVar6,local_5c,0x1);
                            local_18 = local_18 + 0x1;
                        }
                    }
                }
            }
            local_5c = local_5c + 0x1;
            if (0x5 < local_5c) {
                local_5c = 0x0;
            }
        } while (uVar3 % 0x6 != local_5c);
    }
} while( true );
}



bool  FUN_004690a0(param_1: u32,param_2: u32,param_3: i32)

{
let mut uVar1: u32;
let mut uVar2: u32;
let mut uVar3: u32;
let mut bVar4: bool;
let mut local_20: i32;
let mut local_14: i32;

local_20 = 0x0;
uVar1 = *(*(param_1 * 0x4 + DAT_005b4a48) + param_2 * 0x4) & 0xf;
if (uVar1 == 0x0) {
bVar4 = false;
}
else {
for (local_14 = 0x0; local_14 < 0x6; local_14 = local_14 + 0x1) {
uVar2 = FUN_0043a8a2(param_1 + (&DAT_004bea60)[local_14]);
uVar3 = FUN_0043a8d5(uVar2,param_2 + (&DAT_004bea7c)[local_14]);
if ((uVar2 != param_1) || (uVar3 != param_2)) {
uVar2 = *(*(uVar2 * 0x4 + DAT_005b4a48) + uVar3 * 0x4) & 0xf;
if ((uVar2 == 0xa) || (uVar2 == 0x9)) {
local_20 = local_20 + 0x1;
if (0x1 < local_20) break;
}
else {
if (uVar2 == 0x0) {
local_20 = local_20 + 0x2;
break;
}
}
}
}
if (param_3 == 0x0) {
if (local_20 < 0x2) {
bVar4 = true;
}
else {
bVar4 = false;
}
}
else {
if ((uVar1 == 0x6) || (uVar1 == 0x7)) {
bVar4 = false;
}
else {
bVar4 = local_20 == 0x0;
}
}
}
return bVar4;
}



fn FUN_004691f7(param_1: i32,param_2: u32,param_3: u32,param_4: i32,param_5: u32)

{
    let mut bVar1: bool;
    let mut uVar2: u32;
    let mut uVar3: u32;
    undefined3 extraout_var;
    let mut uVar4: u32;
    let mut uVar5: u32;
    undefined3 extraout_var_00;
    let mut local_48: i32;
    let mut local_2c: u32;

    while( true ) {
        uVar2 = FUN_004a2edc();
        uVar3 = FUN_004a2edc();
        if (uVar2 % 0x8 + uVar3 % 0x8 < ((param_5 ^ param_5 >> 0x1f) - (param_5 >> 0x1f))) {
            return;
        }
        uVar2 = FUN_0043a8a2(param_2 + (&DAT_004bea60)[param_4]);
        uVar3 = FUN_0043a8d5(param_2,param_3 + (&DAT_004bea7c)[param_4]);
        if ((uVar2 == param_2) && (uVar3 == param_3)) break;
        bVar1 = FUN_004690a0(uVar2,uVar3,0x0);
        if (CONCAT31(extraout_var,bVar1) == 0x0) {
            return;
        }
        local_2c = *(*(uVar2 * 0x4 + DAT_005b4a48) + uVar3 * 0x4);
        if (((local_2c & 0xf) == 0x6) || ((local_2c & 0xf) == 0x7)) {
            local_2c = FUN_0043b921((&DAT_004d7d50 + param_1 * 0x3890),local_2c);
        }
        *(*(uVar2 * 0x4 + DAT_005b4a48) + uVar3 * 0x4) =
            *(DAT_005b4a4c + 0x574) << 0x4 | local_2c << 0x9 | 0x9;
        FUN_00439559((&DAT_004d7d50 + param_1 * 0x3890),uVar2,uVar3,0x1,0x0);
        uVar4 = FUN_004a2edc();
        param_2 = uVar2;
        param_3 = uVar3;
        if ((uVar4 % 0x5 == 0x0) && (0x0 < param_5)) {
            local_48 = param_4 + 0x1;
            if (0x5 < local_48) {
                local_48 = 0x0;
            }
            param_4 = param_4 + -0x1;
            if (param_4 < 0x0) {
                param_4 = 0x5;
            }
            param_5 = 0xfffffffd - param_5;
            uVar4 = FUN_0043a8a2(uVar2 + (&DAT_004bea60)[param_4]);
            uVar5 = FUN_0043a8d5(uVar2,uVar3 + (&DAT_004bea7c)[param_4]);
            if (((uVar4 != uVar2) || (uVar5 != uVar3)) &&
                (bVar1 = FUN_004690a0(uVar4,uVar5,0x0), CONCAT31(extraout_var_00,bVar1) != 0x0)) {
                FUN_004691f7(param_1,uVar2,uVar3,local_48,param_5);
                local_2c = *(*(DAT_005b4a48 + uVar4 * 0x4) + uVar5 * 0x4);
                if (((local_2c & 0xf) == 0x6) || ((local_2c & 0xf) == 0x7)) {
                    local_2c = FUN_0043b921((&DAT_004d7d50 + param_1 * 0x3890),local_2c);
                }
                *(*(uVar4 * 0x4 + DAT_005b4a48) + uVar5 * 0x4) =
                    *(DAT_005b4a4c + 0x574) << 0x4 | local_2c << 0x9 | 0x9;
                FUN_00439559((&DAT_004d7d50 + param_1 * 0x3890),uVar4,uVar5,0x1,0x0);
                param_2 = uVar4;
                param_3 = uVar5;
            }
        }
        uVar2 = FUN_004a2edc();
        if (uVar2 % 0x3 == 0x0) {
            uVar2 = FUN_004a2edc();
            param_4 = param_4 + uVar2 % 0x3 + -0x1;
            if (param_4 < 0x0) {
                param_4 = 0x5;
            }
            if (0x5 < param_4) {
                param_4 = 0x0;
            }
        }
    }
    return;
}



fn FUN_00469575()

{
    let cVar1: u8;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut pcVar5: String;
    let mut pcVar6: String;
    let mut in_stack_fffffe08: i32;
    let mut local_1ec: i32;
    let mut local_1e8: i32;
    let mut local_1e4: i32;
    let mut local_1e0: i32;
    let mut local_1dc: i32;
    let mut local_1cc: i32;
    let mut local_1b0: i32;
    let mut local_174: i32;
    let mut local_170: i32;
    let mut local_16c: i32;
    let mut local_168: i32;
    let mut local_164: u32;
    let mut local_154: i32;
    let mut local_150: i32;
    i32 local_128 [0x2d];
    i32 aiStack116 [0x7];
    let local_58: u8 [0x10];
    i32 local_48 [0x4];
    i32 local_38 [0x4];
    let mut local_28: i32;
    let mut local_24: u32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    local_1c = 0x0;
    FUN_004a2ff0(0x0,s_Please_wait_while_I_create_the_g_004c221c,0x21,in_stack_fffffe08);
    FUN_004a0430(local_128,0xff,0xa0);
    FUN_0046a4c3();
    for (local_38[3] = 0x0; local_38[3] < 0x9; local_38[3] = local_38[3] + 0x1) {
        if (local_38[3] < 0x7) {
            local_128[*(&DAT_005b4a50 + local_38[3] * 0x4)] = local_38[3];
        }
        else {
            local_128[*(&DAT_005b4a50 + local_38[3] * 0x4)] = 0x9;
        }
    }
    FUN_0046a6f9(local_128);
    FUN_0046a78d(local_128);
    FUN_0046a831();
    for (local_38[3] = 0x0; local_38[3] < 0x28; local_38[3] = local_38[3] + 0x1) {
        *(&DAT_005b7078 + local_38[3] * 0x4e) = 0x3e8;
        *(&DAT_005b707a + local_38[3] * 0x4e) = 0x3e8;
    }
    for (local_38[3] = 0x0; (local_38[3] < 0x28 && (*(&DAT_005b70c2 + local_38[3] * 0x4e) != 0x0));
        local_38[3] = local_38[3] + 0x1) {
        if (local_128[local_38[3]] == -0x1) {
            loop {
                uVar2 = FUN_004a2edc();
                (&DAT_005b7078 + local_38[3] * 0x4e) = (uVar2 % 0x2e) + 0x2;
                uVar2 = FUN_004a2edc();
                (&DAT_005b707a + local_38[3] * 0x4e) = (uVar2 % 0x2b) + 0x2;
                iVar3 = FUN_0046a873(local_38[3],local_38[3]);
            } while (iVar3 == 0x0);
            (&DAT_005b70c2)[local_38[3] * 0x4e] = (&DAT_005b70c2)[local_38[3] * 0x4e] & 0xf0;
            iVar3 = local_38[3] * 0x4e;
            uVar2 = FUN_004a2edc();
            *(&DAT_005b70c2 + iVar3) = *(&DAT_005b70c2 + iVar3) | uVar2 % 0x8 + 0x1U;
        }
    }
    local_38[2] = FUN_004a2edc();
    local_38[2] = local_38[2] % 0x4;
    if ((local_38[2] & 0x1U) == 0x0) {
        local_150 = 0x1;
    }
    else {
        local_150 = -0x1;
    }
    local_18 = local_150;
    if (local_38[2] < 0x2) {
        local_154 = 0x1;
    }
    else {
        local_154 = -0x1;
    }
    local_28 = local_154;
    local_20 = FUN_0046aa6f(local_48,local_58,0x4,local_150,local_154);
    *(&DAT_005b7078 + DAT_005b4a70 * 0x4e) = *(&DAT_005b7078 + local_48[3] * 0x4e);
    *(&DAT_005b707a + DAT_005b4a70 * 0x4e) = *(&DAT_005b707a + local_48[3] * 0x4e);
    FUN_0046a941(DAT_005b4a70);
    *(&DAT_005b7078 + local_48[3] * 0x4e) = 0x3e7;
    *(&DAT_005b707a + local_48[3] * 0x4e) = 0x3e7;
    local_48[3] = DAT_005b4a70;
    for (local_38[3] = 0x0; local_38[3] < 0x3; local_38[3] = local_38[3] + 0x1) {
        local_128[local_48[local_38[3]]] = 0x7;
        FUN_00466de5(local_48[local_38[3]],0x0,0x0,0x7);
        local_1c = local_1c + 0x1;
        FUN_004a36b0(local_1c);
    }
    FUN_0046aa15(local_48[0],local_48[1]);
    FUN_0046aa15(local_48[0],local_48[2]);
    FUN_0046aa15(local_48[1],local_48[2]);
    FUN_0046aa15(local_48[1],local_48[3]);
    FUN_0046aa15(local_48[2],local_48[3]);
    FUN_0046ac91(local_48);
    local_24 = FUN_004a2edc();
    local_24 = local_24 % 0x2;
    switch(local_38[2]) {
    case 0x0:
        local_24 = local_24 + 0x2;
    break;
    case 0x1:
    if (local_24 == 0x0) {
        local_164 = 0x0;
    }
    else {
        local_164 = 0x3;
    }
    local_24 = local_164;
    break;
    case 0x2:
        local_24 = local_24 + 0x1;
    break;
    case 0x3:
}
    if ((local_24 & 0x1) == 0x0) {
        if (local_24 == 0x0) {
            local_16c = 0x1;
        }
        else {
            local_16c = -0x1;
        }
        local_168 = local_16c;
    }
    else {
        local_168 = 0x0;
    }
    local_18 = local_168;
    if ((local_24 & 0x1) == 0x0) {
        local_174 = 0x0;
    }
    else {
        if (local_24 == 0x1) {
            local_170 = 0x1;
        }
        else {
            local_170 = -0x1;
        }
        local_174 = local_170;
    }
    local_28 = local_174;
    local_20 = FUN_0046aa6f(local_38,local_58,0x2,local_168,local_174);
    FUN_0046aa15(local_38[0],local_38[1]);
    for (local_38[3] = 0x0; local_38[3] < 0x2; local_38[3] = local_38[3] + 0x1) {
        local_128[local_38[local_38[3]]] = 0x8;
        FUN_00466de5(local_38[local_38[3]],0x0,0x0,0x8);
        local_1c = local_1c + 0x1;
        FUN_004a36b0(local_1c);
    }
    FUN_0046ac65(local_38);
    for (local_38[3] = 0x0; (local_38[3] < 0x28 && (*(&DAT_005b70c2 + local_38[3] * 0x4e) != 0x0));
        local_38[3] = local_38[3] + 0x1) {
        iVar3 = FUN_004a2f10(&DAT_005b709e + local_38[3] * 0x4e,&DAT_004c2243);
        if ((iVar3 == 0x0) && (local_38[3] != local_38[0])) {
            pcVar5 = &DAT_005b709e + local_38[0] * 0x4e;
            pcVar6 = &DAT_005b709e + local_38[3] * 0x4e;
            loop {
                cVar1 = *pcVar5;
                *pcVar6 = cVar1;
                if (cVar1 == '\0') break;
                cVar1 = pcVar5[0x1];
                pcVar5 = pcVar5 + 0x2;
                pcVar6[0x1] = cVar1;
                pcVar6 = pcVar6 + 0x2;
            } while (cVar1 != '\0');
            pcVar6 = &DAT_004c2247;
            pcVar5 = &DAT_005b709e + local_38[0] * 0x4e;
            loop {
                cVar1 = *pcVar6;
                *pcVar5 = cVar1;
                if (cVar1 == '\0') break;
                cVar1 = pcVar6[0x1];
                pcVar6 = pcVar6 + 0x2;
                pcVar5[0x1] = cVar1;
                pcVar5 = pcVar5 + 0x2;
            } while (cVar1 != '\0');
        }
        if ((&DAT_005b7078 + local_38[3] * 0x4e) == 0x3e8) {
            loop {
                uVar2 = FUN_004a2edc();
                (&DAT_005b7078 + local_38[3] * 0x4e) = (uVar2 % 0x1e) + 0x7;
                uVar2 = FUN_004a2edc();
                (&DAT_005b707a + local_38[3] * 0x4e) = (uVar2 % 0x1c) + 0x7;
                iVar3 = FUN_0046a873(local_38[3],0x28);
            } while (iVar3 == 0x0);
            FUN_0046a941(local_38[3]);
        }
        else {
            if ((&DAT_005b7078 + local_38[3] * 0x4e) == 0x3e7) {
                loop {
                    uVar2 = FUN_004a2edc();
                    (&DAT_005b7078 + local_38[3] * 0x4e) = (uVar2 % 0x2e) + 0x2;
                    uVar2 = FUN_004a2edc();
                    (&DAT_005b707a + local_38[3] * 0x4e) = (uVar2 % 0x2b) + 0x2;
                    iVar3 = FUN_0046a873(local_38[3],0x28);
                } while (iVar3 == 0x0);
                (&DAT_005b70c2)[local_38[3] * 0x4e] = (&DAT_005b70c2)[local_38[3] * 0x4e] & 0xf0;
                iVar3 = local_38[3] * 0x4e;
                uVar2 = FUN_004a2edc();
                *(&DAT_005b70c2 + iVar3) = *(&DAT_005b70c2 + iVar3) | uVar2 % 0x8 + 0x1U;
                FUN_00466de5(local_38[3],0x0,0x0,0xd);
                local_1c = local_1c + 0x1;
                FUN_004a36b0(local_1c);
            }
            else {
                if (local_128[local_38[3]] == -0x1) {
                    FUN_00466de5(local_38[3],0x0,0x0,0xd);
                    local_1c = local_1c + 0x1;
                    FUN_004a36b0(local_1c);
                }
            }
        }
    }
    for (local_38[3] = 0x0; (local_38[3] < 0x28 && (*(&DAT_005b70c2 + local_38[3] * 0x4e) != 0x0));
        local_38[3] = local_38[3] + 0x1) {
        if (local_128[local_38[3]] != 0x7) {
            iVar3 = FUN_0046abb5(local_38[3]);
            if (local_38[3] == DAT_005b4a6c) {
                uVar2 = FUN_004a2edc();
                local_1b0 = uVar2 % 0x2 + 0x4;
            }
            else {
                if (local_128[local_38[3]] == 0x8) {
                    uVar2 = FUN_004a2edc();
                    local_1b0 = uVar2 % 0x2 + 0x1;
                }
                else {
                    if (local_128[local_38[3]] == -0x1) {
                        uVar2 = FUN_004a2edc();
                        local_1b0 = uVar2 % 0x3 + 0x2;
                    }
                    else {
                        uVar2 = FUN_004a2edc();
                        local_1b0 = uVar2 % 0x2 + 0x2;
                    }
                }
            }
            local_1b0 = local_1b0 - iVar3;
            if (0x0 < local_1b0) {
                local_20 = 0x0;
                local_1cc = local_38[3];
                while ((local_1cc = local_1cc + 0x1, local_1cc < 0x28 && (*(&DAT_005b70c2 + local_1cc * 0x4e) != 0x0))) {
                    if ((local_128[local_1cc] != 0x7) && (iVar3 = FUN_0046abb5(local_1cc), iVar3 < 0x4)) {
                        iVar3 = (*(&DAT_005b7076 + local_38[3] * 0x4e) >> 0x10) -
                            (*(&DAT_005b7076 + local_1cc * 0x4e) >> 0x10);
                        iVar4 = (*(&DAT_005b7078 + local_38[3] * 0x4e) >> 0x10) -
                            (*(&DAT_005b7078 + local_1cc * 0x4e) >> 0x10);
                        iVar3 = iVar3 * iVar3 + iVar4 * iVar4;
                        for (local_1dc = 0x0; (local_1dc < local_20 && (aiStack116[local_1dc + 0x1] <= iVar3));
                            local_1dc = local_1dc + 0x1) {
                        }
                        if (local_1dc < local_1b0) {
                            if (local_20 < local_1b0) {
                                local_20 = local_20 + 0x1;
                            }
                            iVar4 = local_20;
                            if (local_1dc + 0x1 < local_20) {
                                while (local_1e0 = iVar4 + -0x1, local_1dc < local_1e0) {
                                    local_128[iVar4 + 0x27] = local_128[iVar4 + 0x26];
                                    aiStack116[iVar4] = aiStack116[local_1e0];
                                    iVar4 = local_1e0;
                                }
                            }
                            local_128[local_1dc + 0x28] = local_1cc;
                            aiStack116[local_1dc + 0x1] = iVar3;
                        }
                    }
                }
                for (local_1cc = 0x0; local_1cc < local_20; local_1cc = local_1cc + 0x1) {
                    FUN_0046aa15(local_38[3],local_128[local_1cc + 0x28]);
                }
            }
        }
    }
    loop {
    FUN_004a0430(local_128,0xff,0xa0);
    local_14 = 0x0;
    FUN_0046a318(DAT_005b4a6c,local_128);
    local_38[3] = 0x0;
    while( true ) {
        if ((0x27 < local_38[3]) || (*(&DAT_005b70c2 + local_38[3] * 0x4e) == 0x0))^ // goto LAB_0046a2f6;
        if ((local_128[local_38[3]] != 0x7) && ((local_128[local_38[3]] == -0x1 || (0x7 < local_128[local_38[3]]))))
        break;
        local_38[3] = local_38[3] + 0x1;
    }
    local_14 = 0x1;
    local_1e4 = -0x1;
    local_1e8 = 0x7fffffff;
    for (local_1ec = 0x0; (local_1ec < 0x28 && (*(&DAT_005b70c2 + local_1ec * 0x4e) != 0x0));
        local_1ec = local_1ec + 0x1) {
        if (((local_128[local_1ec] != 0x7) &&
            (((local_38[3] != local_1ec && (local_128[local_1ec] != -0x1)) && (local_128[local_1ec] < 0x4)))) &&
            (iVar3 = (*(&DAT_005b7076 + local_38[3] * 0x4e) >> 0x10) -
                (*(&DAT_005b7076 + local_1ec * 0x4e) >> 0x10),
             iVar4 = (*(&DAT_005b7078 + local_38[3] * 0x4e) >> 0x10) -
                 (*(&DAT_005b7078 + local_1ec * 0x4e) >> 0x10), iVar3 = iVar3 * iVar3 + iVar4 * iVar4,
             iVar3 < local_1e8)) {
            local_1e4 = local_1ec;
            local_1e8 = iVar3;
        }
    }
    FUN_0046aa15(local_38[3],local_1e4);
    LAB_0046a2f6:
    if (local_14 == 0x0) {
        FUN_004a36b0(0x21);
        FUN_004a3800();
        return;
    }
} while( true );
}



fn FUN_0046a318(param_1: i32,param_2: i32) -> i32

{
let mut iVar1: i32;
let mut bVar2: bool;
let mut iVar3: i32;
let mut iVar4: i32;
let mut local_2c: i32;
let mut local_28: i32;
let mut local_18: i32;

local_18 = 0x0;
*(param_1 * 0x4 + param_2) = 0x0;
loop {
bVar2 = false;
for (local_2c = 0x0; local_2c < 0x28; local_2c = local_2c + 0x1) {
if (local_18 == *(local_2c * 0x4 + param_2)) {
iVar4 = *(&DAT_005b7076 + local_2c * 0x4e);
iVar1 = *(&DAT_005b7078 + local_2c * 0x4e);
local_28 = 0x0;
while ((local_28 < DAT_005b7070 >> 0x10 && (((&DAT_005b7cb8)[local_28 * 0x14] & 0x1) != 0x0))) {
if ((*(&DAT_005b7ca8 + local_28 * 0x14) == iVar4 >> 0x10) &&
(((*(&DAT_005b7cac + local_28 * 0x14) == iVar1 >> 0x10 &&
(iVar3 = FUN_00472441(DAT_00596a38,*(&DAT_005b7cb0 + local_28 * 0x14),
*(&DAT_005b7cb4 + local_28 * 0x14)), iVar3 != -0x1)) &&
(*(iVar3 * 0x4 + param_2) == -0x1)))) {
bVar2 = true;
*(iVar3 * 0x4 + param_2) = local_18 + 0x1;
}
if (((*(&DAT_005b7cb0 + local_28 * 0x14) == iVar4 >> 0x10) &&
(*(&DAT_005b7cb4 + local_28 * 0x14) == iVar1 >> 0x10)) &&
((iVar3 = FUN_00472441(DAT_00596a38,*(&DAT_005b7ca8 + local_28 * 0x14),
*(&DAT_005b7cac + local_28 * 0x14)), iVar3 != -0x1 &&
(*(iVar3 * 0x4 + param_2) == -0x1)))) {
bVar2 = true;
*(iVar3 * 0x4 + param_2) = local_18 + 0x1;
}
local_28 = local_28 + 0x1;
}
}
}
iVar4 = local_18;
local_18 = local_18 + 0x1;
} while (bVar2);
return iVar4;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0046a4c3()

{
    let mut iVar1: i32;
    let mut pcVar2: String;
    let mut local_18: u32;
    let mut local_14: i32;

    local_18 = 0x0;
    for (local_14 = 0x0; (local_14 < 0x28 && (*(&DAT_005b70c2 + local_14 * 0x4e) != 0x0));
        local_14 = local_14 + 0x1) {
        iVar1 = FUN_004a2f10(&DAT_005b709e + local_14 * 0x4e,&DAT_004c224b);
        if (iVar1 == 0x0) {
            _DAT_005b4a50 = local_14;
            local_18 = local_18 | 0x1;
        }
        else {
            iVar1 = FUN_004a2f10(&DAT_005b709e + local_14 * 0x4e,s_Aragon_004c2250);
            if (iVar1 == 0x0) {
                _DAT_005b4a54 = local_14;
                local_18 = local_18 | 0x2;
            }
            else {
                iVar1 = FUN_004a2f10(&DAT_005b709e + local_14 * 0x4e,s_Severus_004c2257);
                if (iVar1 == 0x0) {
                    _DAT_005b4a58 = local_14;
                    local_18 = local_18 | 0x4;
                }
                else {
                    iVar1 = FUN_004a2f10(&DAT_005b709e + local_14 * 0x4e,s_Delphi_004c225f);
                    if (iVar1 == 0x0) {
                        _DAT_005b4a5c = local_14;
                        local_18 = local_18 | 0x8;
                    }
                    else {
                        iVar1 = FUN_004a2f10(&DAT_005b709e + local_14 * 0x4e,s_Istakhr_004c2266);
                        if (iVar1 == 0x0) {
                            _DAT_005b4a60 = local_14;
                            local_18 = local_18 | 0x10;
                        }
                        else {
                            iVar1 = FUN_004a2f10(&DAT_005b709e + local_14 * 0x4e,s_Leagueheim_004c226e);
                            if (iVar1 == 0x0) {
                                _DAT_005b4a64 = local_14;
                                local_18 = local_18 | 0x20;
                            }
                            else {
                                iVar1 = FUN_004a2f10(&DAT_005b709e + local_14 * 0x4e,s_Holy_Terra_004c2279);
                                if (iVar1 == 0x0) {
                                    _DAT_005b4a68 = local_14;
                                    local_18 = local_18 | 0x40;
                                }
                                else {
                                    iVar1 = FUN_004a2f10(&DAT_005b709e + local_14 * 0x4e,s_Byzantium_II_004c2284);
                                    if (iVar1 == 0x0) {
                                        DAT_005b4a6c = local_14;
                                        local_18 = local_18 | 0x80;
                                    }
                                    else {
                                        iVar1 = FUN_004a2f10(&DAT_005b709e + local_14 * 0x4e,s_Stigmata_004c2291);
                                        if (iVar1 == 0x0) {
                                            DAT_005b4a70 = local_14;
                                            local_18 = local_18 | 0x100;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if (local_18 != 0x1ff) {
        pcVar2 = FUN_00499050(DAT_0059679c,0x7394);
        pop_err_msg_box_and_exit_004a02f5(pcVar2);
    }
    return;
}



fn FUN_0046a6f9(param_1: i32)

{
    i32 **ppiVar1;
    i32 **local_18;

    ppiVar1 = (i32 **)*DAT_005967c8;
    while (local_18 = ppiVar1, local_18 != (i32 **)0x0) {
    ppiVar1 = (i32 **)*local_18;
    if (*((*(local_18 + 0x6) >> 0x10) * 0x4 + param_1) == -0x1) {
        FUN_004811e6(local_18);
    }
}
    ppiVar1 = (i32 **)*DAT_005967c8;
    while (local_18 = ppiVar1, local_18 != (i32 **)0x0) {
    ppiVar1 = (i32 **)*local_18;
    if (*((*(local_18 + 0x6) >> 0x10) * 0x4 + param_1) == -0x1) {
        FUN_004811e6(local_18);
    }
}
    return;
}



fn FUN_0046a78d(param_1: i32)

{
    i32 **local_20;
    i32 **local_1c;
    let local_18: *mut u32;

    local_18 = FUN_004840cd(&local_1c,&local_20,-0x1);
    while (local_20 != (i32 **)0x0) {
    if (*((local_20 + 0x8) * 0x4 + param_1) == -0x1) {
        FUN_00484b4e(local_20);
    }
    local_20 = local_1c;
    if (local_1c != (i32 **)0x0) {
        local_1c = (i32 **)*local_1c;
    }
}
    FUN_0048418d(&local_1c);
    return;
}



fn FUN_0046a831()

{
    let mut local_14: i32;

    for (local_14 = 0x0; local_14 < 0x78; local_14 = local_14 + 0x1) {
        *(&DAT_005b7cb8 + local_14 * 0x14) = 0x0;
    }
    DAT_005b7070._2_2_ = 0x0;
    return;
}



fn FUN_0046a873(param_1: i32,param_2: i32) -> u32

{
    let mut local_14: i32;

    local_14 = 0x0;
    while( true ) {
        if (param_2 <= local_14) {
            return 0x1;
        }
        if ((((local_14 != param_1) &&
            ((*(&DAT_005b7076 + local_14 * 0x4e) >> 0x10) + -0x3 <= *(&DAT_005b7076 + param_1 * 0x4e) >> 0x10
            )) && (*(&DAT_005b7076 + param_1 * 0x4e) >> 0x10 <=
            (*(&DAT_005b7076 + local_14 * 0x4e) >> 0x10) + 0x3)) &&
            (((*(&DAT_005b7078 + local_14 * 0x4e) >> 0x10) + -0x3 <= *(&DAT_005b7078 + param_1 * 0x4e) >> 0x10
                && (*(&DAT_005b7078 + param_1 * 0x4e) >> 0x10 <=
                (*(&DAT_005b7078 + local_14 * 0x4e) >> 0x10) + 0x3)))) break;
        local_14 = local_14 + 0x1;
    }
    return 0x0;
}



fn FUN_0046a941(param_1: i32)

{
    let mut local_38: i32;
    let mut local_34: i32;
    i32 **local_30;
    i32 **local_2c;
    let mut local_28: u32;
    i32 **local_24;
    i32 **local_20;
    let mut local_1c: u32;
    i32 **local_18;
    i32 **local_14;

    local_18 = (i32 **)*DAT_005967b0;
    while (local_18 != (i32 **)0x0) {
    local_14 = (i32 **)*local_18;
    local_30 = local_18 + 0x8;
    local_1c = local_30 & 0xffff0000 | local_30;
    if (local_30 == param_1) {
        local_24 = local_18 + 0x8;
        local_28 = *(local_18 + 0x3a) & 0x1;
        if (local_28 == 0x0) {
            local_34 = *(&DAT_005b7076 + param_1 * 0x4e) >> 0x10;
            local_38 = *(&DAT_005b7078 + param_1 * 0x4e) >> 0x10;
            local_2c = local_30;
            local_20 = local_24;
            FUN_004729ba(*(local_18 + 0x23) >> 0x18,&local_34,&local_38,0x0);
            FUN_004841ea(local_18,param_1,local_34,local_38);
        }
    }
    local_18 = local_14;
}
    return;
}



fn FUN_0046aa15(param_1: i32,param_2: i32)

{
    FUN_00473798(DAT_00596a38,*(&DAT_005b7076 + param_1 * 0x4e) >> 0x10,
                 *(&DAT_005b7078 + param_1 * 0x4e) >> 0x10,*(&DAT_005b7076 + param_2 * 0x4e) >> 0x10,
                 *(&DAT_005b7078 + param_2 * 0x4e) >> 0x10);
    return;
}



fn FUN_0046aa6f(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: i32) -> i32

{
let mut iVar1: i32;
let mut local_28: i32;
let mut local_20: i32;
let mut local_18: i32;
let mut local_14: i32;

local_14 = 0x0;
for (local_18 = 0x0; (local_18 < 0x28 && (*(&DAT_005b70c2 + local_18 * 0x4e) != 0x0));
local_18 = local_18 + 0x1) {
if (*(&DAT_005b7076 + local_18 * 0x4e) >> 0x10 < 0x33) {
iVar1 = (*(&DAT_005b7076 + local_18 * 0x4e) >> 0x10) * param_4 +
param_5 * (*(&DAT_005b7078 + local_18 * 0x4e) >> 0x10);
for (local_20 = 0x0; (local_20 < local_14 && (*(local_20 * 0x4 + param_2) <= iVar1));
local_20 = local_20 + 0x1) {
}
if (local_20 < param_3) {
if (local_14 < param_3) {
local_14 = local_14 + 0x1;
}
local_28 = local_14;
if (local_20 + 0x1 < local_14) {
while (local_28 = local_28 + -0x1, local_20 < local_28) {
*(local_28 * 0x4 + param_1) = *(local_28 * 0x4 + param_1 + -0x4);
*(local_28 * 0x4 + param_2) = *(local_28 * 0x4 + param_2 + -0x4);
}
}
*(local_20 * 0x4 + param_1) = local_18;
*(local_20 * 0x4 + param_2) = iVar1;
}
}
}
return local_14;
}



fn FUN_0046abb5(param_1: i32) -> i32

{
let mut local_20: i32;
let mut local_14: i32;

local_20 = 0x0;
local_14 = 0x0;
while ((local_14 < 0x78 && (((&DAT_005b7cb8)[local_14 * 0x14] & 0x1) != 0x0))) {
if (((*(&DAT_005b7ca8 + local_14 * 0x14) == *(&DAT_005b7076 + param_1 * 0x4e) >> 0x10) &&
(*(&DAT_005b7cac + local_14 * 0x14) == *(&DAT_005b7078 + param_1 * 0x4e) >> 0x10)) ||
((*(&DAT_005b7cb0 + local_14 * 0x14) == *(&DAT_005b7076 + param_1 * 0x4e) >> 0x10 &&
(*(&DAT_005b7cb4 + local_14 * 0x14) == *(&DAT_005b7078 + param_1 * 0x4e) >> 0x10)))) {
local_20 = local_20 + 0x1;
}
local_14 = local_14 + 0x1;
}
return local_20;
}



fn FUN_0046ac65(param_1: i32)

{
    FUN_0046acbd(param_1,0x2,&DAT_004bf710,0x4,0x8);
    return;
}



fn FUN_0046ac91(param_1: i32)

{
    FUN_0046acbd(param_1,0x3,&DAT_004bf730,0x4,0x7);
    return;
}



fn FUN_0046acbd(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: i32)

{
    i32 **ppiVar1;
    let mut iVar2: i32;
    let puVar3: *mut u32;
    let puVar4: *mut u32;
    let in_stack_ffffff20: i16;
    let mut in_stack_ffffff24: u32;
    u32 local_a4 [0xc];
    let mut local_74: u32;
    let local_70: u16;
    let local_6e: u8;
    let local_6d: u8;
    let local_6c: u8;
    let local_6a: u16;
    let local_66: u16;
    let local_64: u8;
    let local_63: u16;
    let local_61: u8;
    let local_5f: u8;
    let mut local_5e: u32;
    undefined2 *local_44;
    let mut local_40: i32;
    let mut local_3c: i32;
    let mut local_38: i32;
    let mut local_34: i32;
    let mut local_30: u32;
    i32 **local_2c;
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    local_14 = 0x0;
    loop {
    if (param_2 <= local_14) {
        return;
    }
    local_28 = *(local_14 * 0x4 + param_1);
    local_20 = *(&DAT_005b7076 + local_28 * 0x4e) >> 0x10;
    local_24 = *(&DAT_005b7078 + local_28 * 0x4e) >> 0x10;
    FUN_004729ba(param_5,&local_20,&local_24,0x0);
    FUN_00431d31(&local_30);
    local_2c = (i32 **)0x0;
    local_1c = 0x0;
    for (local_18 = 0x0; local_18 < param_4; local_18 = local_18 + 0x1) {
        for (local_34 = 0x0; local_34 < *(local_18 * 0x8 + param_3 + 0x4); local_34 = local_34 + 0x1) {
            FUN_00486065(&local_74);
            FUN_00486b6b(&local_74,0x4b);
            local_44 = &local_74;
            local_40 = local_28;
            local_3c = local_20;
            local_38 = local_24;
            local_74._0_2_ = local_28;
            local_74._2_2_ = local_20;
            local_70 = local_24;
            local_6d = (local_18 * 0x8 + param_3);
            local_6c = 0x0;
            local_6a = *(*(&DAT_00582938 + *(local_18 * 0x8 + param_3) * 0x18) + 0x41);
            local_66 = 0x0;
            local_64 = 0x0;
            local_63 = 0x0;
            local_61 = 0x64;
            local_6e = param_5;
            local_5f = param_5;
            local_5e = 0x0;
            puVar3 = &local_74;
            puVar4 = local_a4;
            for (iVar2 = 0xb; iVar2 != 0x0; iVar2 = iVar2 + -0x1) {
                *puVar4 = *puVar3;
                puVar3 = puVar3 + 0x1;
                puVar4 = puVar4 + 0x1;
            }
            puVar4 = puVar3;
            puVar3 = local_a4;
            puVar4 = &stack0xffffff20;
            for (iVar2 = 0xb; iVar2 != 0x0; iVar2 = iVar2 + -0x1) {
                *puVar4 = *puVar3;
                puVar3 = puVar3 + 0x1;
                puVar4 = puVar4 + 0x1;
            }
            puVar4 = puVar3;
            ppiVar1 = FUN_00485463(in_stack_ffffff20,in_stack_ffffff24);
            if (local_2c != (i32 **)0x0) {
                FUN_00431efd(&local_30,ppiVar1);
                ppiVar1 = local_2c;
            }
            local_2c = ppiVar1;
            local_1c = local_1c + 0x1;
            if (0x13 < local_1c) break;
        }
        if (0x13 < local_1c) break;
    }
    local_14 = local_14 + 0x1;
} while( true );
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0046ae9f(param_1: i32)

{
    let mut pcVar1: String;
    let mut iVar2: i32;
    let mut uVar3: u32;
    let mut uVar4: u32;
    let mut uVar5: u32;
    let piVar6: *mut i32;;
    let mut local_110: i32;
    let local_104: *mut i32;;
    let mut local_fc: u32;
    let mut local_f8: u32;
    let mut local_f4: i32;
    let mut local_f0: i32;
    let mut local_ec: i32;
    byte *local_e8;
    let local_e4: u8 [0x80];
    i32 local_64 [0x10];
    let mut local_24: i32;
    byte **local_20;
    let mut local_1c: i32;
    let local_18: *mut i32;;
    let mut local_14: u32;

    local_24 = 0x0;
    _DAT_00599df8 = 0xd;
    DAT_004c9754 = 0xd;
    FUN_004a0430(local_64,0x0,0x40);
    FUN_0049c2e0(local_e4,s_RAND_TYPE_d_TXT_004c22a5);
    local_20 = (byte **)FUN_004a96cc(local_e4,&DAT_004c22b5);
    local_1c = 0x0;
    if (local_20 != (byte **)0x0) {
    local_18 = FUN_0049c2c9(0x120);
    if (local_18 == 0x0) {
        FUN_0049ca40(local_20);
        return;
    }
    loop {
        loop {
            while( true ) {
                pcVar1 = FUN_004a2f60(local_e4,0x7f,local_20);
                if (pcVar1 == 0x0)^ // goto LAB_0046b083;
                local_e8 = FUN_00467684(local_e4);
                if (local_24 != 0x0) break;
                iVar2 = FUN_004a9800(local_e4,s_BEGIN_BONUS_004c22b8,0xb);
                if (iVar2 == 0x0) {
                    local_24 = local_24 + 0x1;
                }
            }
            iVar2 = FUN_004a9800(local_e4,&DAT_004c22c4,0x3);
            if (iVar2 == 0x0)^ // goto LAB_0046b083;
            local_e8 = FUN_00467940(local_e8,&local_f4);
            local_e8 = FUN_00467940(local_e8,&local_f0);
            local_e8 = FUN_00467940(local_e8,&local_ec);
        } while (((local_f0 == 0x0) || (local_f4 < 0x1a)) || (0x1f < local_f4));
        local_18[local_1c * 0x3] = local_f4;
        local_18[local_1c * 0x3 + 0x1] = local_f0;
        local_18[local_1c * 0x3 + 0x2] = local_ec;
        local_64[local_ec] = 0x1;
        local_1c = local_1c + 0x1;
    } while (local_1c < 0x18);
    LAB_0046b083:
        FUN_0049ca40(local_20);
}
    if (local_1c != 0x0) {
        for (local_14 = 0x0; local_14 < 0x2c; local_14 = local_14 + 0x1) {
            local_f8 = ((local_14 & 0x1) == 0x0);
            for (local_fc = local_f8; local_fc < (0x41 - (local_14 & 0x1)); local_fc = local_fc + 0x2) {
                uVar3 = *(*(DAT_005b4a48 + local_14 * 0x4) + local_fc * 0x4) & 0xf;
                if (local_64[uVar3] != 0x0) {
                    if (uVar3 == 0x0) {
                        for (local_110 = 0x0; local_110 < 0x6; local_110 = local_110 + 0x1) {
                            uVar4 = FUN_0043a8a2(local_14 + (&DAT_004bea60)[local_110]);
                            uVar5 = FUN_0043a8d5(uVar4,local_fc + (&DAT_004bea7c)[local_110]);
                            if (((uVar4 != local_14) || (uVar5 != local_fc)) &&
                                ((*(*(uVar4 * 0x4 + DAT_005b4a48) + uVar5 * 0x4) & 0xf) != 0x0)) break;
                        }
                        if (local_110 == 0x6)^ // goto LAB_0046b0e0;
                    }
                    piVar6 = local_18 + local_1c * 0x3;
                    for (local_104 = local_18; local_104 < piVar6; local_104 = local_104 + 0x3) {
                        if ((uVar3 == local_104[0x2]) && (uVar4 = FUN_004a2edc(), (uVar4 & 0x3ff) < local_104[0x1])) {
                            iVar2 = FUN_0043ab95(param_1,local_14,local_fc,0xd,0xd,*local_104);
                            if (iVar2 == 0x1) {
                                FUN_00439559((&DAT_004d7d50 + param_1 * 0x3890),local_14,local_fc,0x1,0x0);
                            }
                            break;
                        }
                    }
                }
                LAB_0046b0e0:
            }
        }
        FUN_0049af50(local_18);
    }
    return;
}
