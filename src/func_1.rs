// #include "EFS_decompiled_round_1.h"
unsafe fn FUN_00401010(param_1: *mut u32,param_2: *mut u32,param_3: i32,param_4: u32,param_5: u32)
{
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut local_8: u32;

    uVar2 = param_3 - param_5 >> 0x1;
    // TODO:
    // for (local_8 = param_4 >> 0x1; uVar1 = param_4 >> 0x1, local_8 != 0x0; local_8 = local_8 - 0x1) {
    //   for (uVar1 = uVar2 >> 0x2; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
    //     *param_1 = 0x0;
    //     param_1 = param_1 + 0x1;
    //   }
    //   for (uVar1 = uVar2 & 0x3; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
    //     param_1 = 0x0;
    //     param_1 = (param_1 + 0x1);
    //   }
    //   for (uVar1 = param_5 >> 0x2; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
    //     *param_1 = *param_2;
    //     param_2 = param_2 + 0x1;
    //     param_1 = param_1 + 0x1;
    //   }
    //   for (uVar1 = param_5 & 0x3; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
    //     param_1 = param_2;
    //     param_2 = (param_2 + 0x1);
    //     param_1 = (param_1 + 0x1);
    //   }
    //   for (uVar1 = uVar2 >> 0x2; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
    //     *param_1 = 0x0;
    //     param_1 = param_1 + 0x1;
    //   }
    //   for (uVar1 = uVar2 & 0x3; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
    //     param_1 = 0x0;
    //     param_1 = (param_1 + 0x1);
    //   }
    //   if ((local_8 & 0x1) == 0x0) {
    //     param_5 = param_5 + 0x2;
    //     uVar2 = uVar2 - 0x1;
    //   }
    // }
    while (local_8 = uVar1, local_8 != 0x0) {
        // TODO
        // for (uVar1 = uVar2 >> 0x2; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
        //   *param_1 = 0x0;
        //   param_1 = param_1 + 0x1;
        // }
        // TODO
        // for (uVar1 = uVar2 & 0x3; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
        //   param_1 = 0x0;
        //   param_1 = (param_1 + 0x1);
        // }
        // TODO
        // for (uVar1 = param_5 >> 0x2; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
        //   *param_1 = *param_2;
        //   param_2 = param_2 + 0x1;
        //   param_1 = param_1 + 0x1;
        // }
        // TODO
        // for (uVar1 = param_5 & 0x3; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
        //   param_1 = param_2;
        //   param_2 = (param_2 + 0x1);
        //   param_1 = (param_1 + 0x1);
        // }
        // TODO
        // for (uVar1 = uVar2 >> 0x2; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
        //   *param_1 = 0x0;
        //   param_1 = param_1 + 0x1;
        // }
        // TODO:
        // for (uVar1 = uVar2 & 0x3; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
        //   param_1 = 0x0;
        //   param_1 = (param_1 + 0x1);
        // }
        if ((local_8 & 0x1) == 0x0) {
            param_5 = param_5 - 0x2;
            uVar2 = uVar2 + 0x1;
        }
        uVar1 = local_8 - 0x1;
    }
    return;
}



unsafe fn FUN_004010c4(param_1: *mut u32,param_2: &mut String,param_3: i32,param_4: u32,param_5: i32)

{
    let mut uVar1: u32;
    let mut iVar2: i32;
    let mut uVar3: u32;
    let mut local_8: u32;

    uVar3 = (param_3 - param_5) >> 0x1;
    // TODO
    // for (local_8 = param_4 >> 0x1; uVar1 = param_4 >> 0x1, local_8 != 0x0; local_8 = local_8 - 0x1) {
    //   for (uVar1 = uVar3 >> 0x2; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
    //     *param_1 = 0x0;
    //     param_1 = param_1 + 0x1;
    //   }
    //   for (uVar1 = uVar3 & 0x3; iVar2 = param_5, uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
    //     param_1 = 0x0;
    //     param_1 = (param_1 + 0x1);
    //   }
    //   for (; iVar2 != 0x0; iVar2 = iVar2 + -0x1) {
    //     if (*param_2 != '\0') {
    //       param_1 = *param_2;
    //     }
    //     param_2 = param_2 + 0x1;
    //     param_1 = (param_1 + 0x1);
    //   }
    //   for (uVar1 = uVar3 >> 0x2; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
    //     *param_1 = 0x0;
    //     param_1 = param_1 + 0x1;
    //   }
    //   for (uVar1 = uVar3 & 0x3; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
    //     param_1 = 0x0;
    //     param_1 = (param_1 + 0x1);
    //   }
    //   if ((local_8 & 0x1) == 0x0) {
    //     param_5 = param_5 + 0x2;
    //     uVar3 = uVar3 - 0x1;
    //   }
    // }
    while (local_8 = uVar1, local_8 != 0x0) {
        // TODO
        // for (uVar1 = uVar3 >> 0x2; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
        //   *param_1 = 0x0;
        //   param_1 = param_1 + 0x1;
        // }
        // TODO
        // for (uVar1 = uVar3 & 0x3; iVar2 = param_5, uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
        //   param_1 = 0x0;
        //   param_1 = (param_1 + 0x1);
        // }
        // TODO
        // for (; iVar2 != 0x0; iVar2 = iVar2 + -0x1) {
        //   if (*param_2 != '\0') {
        //     param_1 = *param_2;
        //   }
        //   param_2 = param_2 + 0x1;
        //   param_1 = (param_1 + 0x1);
        // }
        // TODO
        // for (uVar1 = uVar3 >> 0x2; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
        //   *param_1 = 0x0;
        //   param_1 = param_1 + 0x1;
        // }
        // TODO
        // for (uVar1 = uVar3 & 0x3; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
        //   param_1 = 0x0;
        //   param_1 = (param_1 + 0x1);
        // }
        if ((local_8 & 0x1) == 0x0) {
            param_5 = param_5 + -0x2;
            uVar3 = uVar3 + 0x1;
        }
        uVar1 = local_8 - 0x1;
    }
    return;
}



unsafe fn FUN_00401188(param_1: *mut u32,param_2: i32,param_3: i32,param_4: u32,param_5: i32,param_6: i32)

{
    let mut uVar1: u32;
    let mut iVar2: i32;
    let mut uVar3: u32;
    let mut pcVar4: String;
    let mut local_8: u32;

    pcVar4 = (param_2 + param_6 + -0x1);
    uVar3 = (param_3 - param_5) >> 0x1;
    // TODO
    // for (local_8 = param_4 >> 0x1; uVar1 = param_4 >> 0x1, local_8 != 0x0; local_8 = local_8 - 0x1) {
    //   for (uVar1 = uVar3 >> 0x2; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
    //     *param_1 = 0x0;
    //     param_1 = param_1 + 0x1;
    //   }
    //   for (uVar1 = uVar3 & 0x3; iVar2 = param_5, uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
    //     param_1 = 0x0;
    //     param_1 = (param_1 + 0x1);
    //   }
    //   for (; iVar2 != 0x0; iVar2 = iVar2 + -0x1) {
    //     if (*pcVar4 != '\0') {
    //       param_1 = *pcVar4;
    //     }
    //     pcVar4 = pcVar4 + -0x1;
    //     param_1 = (param_1 + 0x1);
    //   }
    //   for (uVar1 = uVar3 >> 0x2; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
    //     *param_1 = 0x0;
    //     param_1 = param_1 + 0x1;
    //   }
    //   for (uVar1 = uVar3 & 0x3; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
    //     param_1 = 0x0;
    //     param_1 = (param_1 + 0x1);
    //   }
    //   if ((local_8 & 0x1) == 0x0) {
    //     param_5 = param_5 + 0x2;
    //     uVar3 = uVar3 - 0x1;
    //   }
    // }
    while (local_8 = uVar1, local_8 != 0x0) {
        // TODO
        // for (uVar1 = uVar3 >> 0x2; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
        //   *param_1 = 0x0;
        //   param_1 = param_1 + 0x1;
        // }
        // TODO
        // for (uVar1 = uVar3 & 0x3; iVar2 = param_5, uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
        //   param_1 = 0x0;
        //   param_1 = (param_1 + 0x1);
        // }
        // TODO
        // for (; iVar2 != 0x0; iVar2 = iVar2 + -0x1) {
        //   if (*pcVar4 != '\0') {
        //     param_1 = *pcVar4;
        //   }
        //   pcVar4 = pcVar4 + -0x1;
        //   param_1 = (param_1 + 0x1);
        // }
        // TODO
        // for (uVar1 = uVar3 >> 0x2; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
        //   *param_1 = 0x0;
        //   param_1 = param_1 + 0x1;
        // }
        // TODO
        // for (uVar1 = uVar3 & 0x3; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
        //   param_1 = 0x0;
        //   param_1 = (param_1 + 0x1);
        // }
        if ((local_8 & 0x1) == 0x0) {
            param_5 = param_5 + -0x2;
            uVar3 = uVar3 + 0x1;
        }
        uVar1 = local_8 - 0x1;
    }
    return;
}



unsafe fn FUN_00401250(param_1: *mut u32,param_2: &mut String,param_3: i32,param_4: u32,param_5: i32)

{
    let mut uVar1: u32;
    let mut iVar2: i32;
    let mut uVar3: u32;
    let mut pcVar4: String;
    let mut local_8: u32;

    uVar3 = (param_3 - param_5) >> 0x1;
    // TODO
    // for (local_8 = param_4 >> 0x1; uVar1 = param_4 >> 0x1, local_8 != 0x0; local_8 = local_8 - 0x1) {
    //   for (uVar1 = uVar3 >> 0x2; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
    //     *param_1 = 0x0;
    //     param_1 = param_1 + 0x1;
    //   }
    //   for (uVar1 = uVar3 & 0x3; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
    //     param_1 = 0x0;
    //     param_1 = (param_1 + 0x1);
    //   }
    //   param_2 = param_2 + param_5;
    //   pcVar4 = param_2;
    //   for (iVar2 = param_5; pcVar4 = pcVar4 + -0x1, iVar2 != 0x0; iVar2 = iVar2 + -0x1) {
    //     if (*pcVar4 != '\0') {
    //       param_1 = *pcVar4;
    //     }
    //     param_1 = (param_1 + 0x1);
    //   }
    //   for (uVar1 = uVar3 >> 0x2; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
    //     *param_1 = 0x0;
    //     param_1 = param_1 + 0x1;
    //   }
    //   for (uVar1 = uVar3 & 0x3; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
    //     param_1 = 0x0;
    //     param_1 = (param_1 + 0x1);
    //   }
    //   if ((local_8 & 0x1) == 0x0) {
    //     param_5 = param_5 + 0x2;
    //     uVar3 = uVar3 - 0x1;
    //   }
    // }
    while (local_8 = uVar1, local_8 != 0x0) {
        // TODO
        // for (uVar1 = uVar3 >> 0x2; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
        //   *param_1 = 0x0;
        //   param_1 = param_1 + 0x1;
        // }
        // TODO
        // for (uVar1 = uVar3 & 0x3; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
        //   param_1 = 0x0;
        //   param_1 = (param_1 + 0x1);
        // }
        param_2 = param_2 + param_5;
        pcVar4 = param_2;
        // TODO
        // for (iVar2 = param_5; pcVar4 = pcVar4 + -0x1, iVar2 != 0x0; iVar2 = iVar2 + -0x1) {
        //   if (*pcVar4 != '\0') {
        //     param_1 = *pcVar4;
        //   }
        //   param_1 = (param_1 + 0x1);
        // }
        // TODO
        // for (uVar1 = uVar3 >> 0x2; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
        //   *param_1 = 0x0;
        //   param_1 = param_1 + 0x1;
        // }
        // TODO
        // for (uVar1 = uVar3 & 0x3; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
        //   param_1 = 0x0;
        //   param_1 = (param_1 + 0x1);
        // }
        if ((local_8 & 0x1) == 0x0) {
            param_5 = param_5 + -0x2;
            uVar3 = uVar3 + 0x1;
        }
        uVar1 = local_8 - 0x1;
    }
    return;
}



unsafe fn FUN_00401320(param_1: *mut u32,param_2: i32,param_3: i32,param_4: u32,param_5: i32,param_6: i32)

{
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut pcVar4: String;
    let mut local_8: u32;

    iVar3 = param_2 + param_6;
    uVar2 = (param_3 - param_5) >> 0x1;
    // TODO
    // for (local_8 = param_4 >> 0x1; uVar1 = param_4 >> 0x1, local_8 != 0x0; local_8 = local_8 - 0x1) {
    //   for (uVar1 = uVar2 >> 0x2; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
    //     *param_1 = 0x0;
    //     param_1 = param_1 + 0x1;
    //   }
    //   for (uVar1 = uVar2 & 0x3; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
    //     param_1 = 0x0;
    //     param_1 = (param_1 + 0x1);
    //   }
    //   pcVar4 = (iVar3 - param_5);
    //   for (iVar3 = param_5; iVar3 != 0x0; iVar3 = iVar3 + -0x1) {
    //     if (*pcVar4 != '\0') {
    //       param_1 = *pcVar4;
    //     }
    //     pcVar4 = pcVar4 + 0x1;
    //     param_1 = (param_1 + 0x1);
    //   }
    //   iVar3 = pcVar4 - param_5;
    //   for (uVar1 = uVar2 >> 0x2; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
    //     *param_1 = 0x0;
    //     param_1 = param_1 + 0x1;
    //   }
    //   for (uVar1 = uVar2 & 0x3; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
    //     param_1 = 0x0;
    //     param_1 = (param_1 + 0x1);
    //   }
    //   if ((local_8 & 0x1) == 0x0) {
    //     param_5 = param_5 + 0x2;
    //     uVar2 = uVar2 - 0x1;
    //   }
    // }
    while (local_8 = uVar1, local_8 != 0x0) {
        // TODO
        // for (uVar1 = uVar2 >> 0x2; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
        //   *param_1 = 0x0;
        //   param_1 = param_1 + 0x1;
        // }
        // TODO
        // for (uVar1 = uVar2 & 0x3; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
        //   param_1 = 0x0;
        //   param_1 = (param_1 + 0x1);
        // }
        pcVar4 = (iVar3 - param_5);
        // TODO
        // for (iVar3 = param_5; iVar3 != 0x0; iVar3 = iVar3 + -0x1) {
        //   if (*pcVar4 != '\0') {
        //     param_1 = *pcVar4;
        //   }
        //   pcVar4 = pcVar4 + 0x1;
        //   param_1 = (param_1 + 0x1);
        // }
        iVar3 = pcVar4 - param_5;
        // TODO
        // for (uVar1 = uVar2 >> 0x2; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
        //   *param_1 = 0x0;
        //   param_1 = param_1 + 0x1;
        // }
        // TODO
        // for (uVar1 = uVar2 & 0x3; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
        //   param_1 = 0x0;
        //   param_1 = (param_1 + 0x1);
        // }
        if ((local_8 & 0x1) == 0x0) {
            param_5 = param_5 + -0x2;
            uVar2 = uVar2 + 0x1;
        }
        uVar1 = local_8 - 0x1;
    }
    return;
}



unsafe fn FUN_004013f4(param_1: i32,param_2: *mut u32) -> u32

{
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let puVar4: *mut u32;
    let mut local_8: u32;

    puVar4 = (param_1 + 0xa);
    iVar3 = 0xa;
    uVar2 = 0x1c;
    // TODO
    // for (local_8 = 0xa; local_8 != 0x0; local_8 = local_8 - 0x1) {
    //   for (uVar1 = uVar2 >> 0x2; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
    //     *puVar4 = *param_2;
    //     param_2 = param_2 + 0x1;
    //     puVar4 = puVar4 + 0x1;
    //   }
    //   for (uVar1 = uVar2 & 0x3; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
    //     puVar4 = param_2;
    //     param_2 = (param_2 + 0x1);
    //     puVar4 = (puVar4 + 0x1);
    //   }
    //   if ((local_8 & 0x1) == 0x0) {
    //     uVar2 = uVar2 - 0x2;
    //     iVar3 = iVar3 + 0x1;
    //     puVar4 = (puVar4 + -0x1);
    //   }
    //   puVar4 = (puVar4 + iVar3 * 0x2);
    // }
    return 0x0;
}



unsafe fn FUN_0040144c(param_1: i32,param_2: i32)

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut puVar3: *mut u8;
    let mut puVar4: *mut u8;
    let mut local_10: i32;
    let mut local_c: i32;
    let mut local_8: u32;

    puVar3 = (param_2 + 0x2f7);
    puVar4 = (param_1 + 0x3bf);
    local_c = 0x1c;
    local_10 = 0x0;
    iVar2 = 0x14;
    // TODO
    // for (local_8 = 0x14; iVar1 = iVar2, local_8 != 0x0; local_8 = local_8 - 0x1) {
    //   for (; iVar1 != 0x0; iVar1 = iVar1 + -0x1) {
    //     *puVar4 = *puVar3;
    //     puVar3 = puVar3 + -0x1;
    //     puVar4 = puVar4 + -0x1;
    //   }
    //   if ((local_8 & 0x1) == 0x0) {
    //     iVar2 = iVar2 + -0x2;
    //     local_c = local_c + 0x1;
    //     local_10 = local_10 + 0x1;
    //     puVar4 = puVar4 + 0x1;
    //   }
    //   puVar4 = puVar4 + (-local_10 - local_c);
    //   puVar3 = puVar3 + -0x14;
    // }
    return;
}



unsafe fn FUN_004014b8(param_1: i32,param_2: i32) -> u32

{
    let mut uVar1: u32;
    let mut uVar2: u32;
    let puVar3: *mut u32;
    let puVar4: *mut u32;
    let mut local_10: i32;
    let mut local_c: i32;
    let mut local_8: u32;

    puVar3 = (param_2 + 0x314);
    puVar4 = (param_1 + 0x3dc);
    local_c = 0x1c;
    local_10 = 0x0;
    uVar2 = 0x14;
    // TODO
    // for (local_8 = 0x14; local_8 != 0x0; local_8 = local_8 - 0x1) {
    //   for (uVar1 = uVar2 >> 0x2; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
    //     *puVar4 = *puVar3;
    //     puVar3 = puVar3 + 0x1;
    //     puVar4 = puVar4 + 0x1;
    //   }
    //   for (uVar1 = uVar2 & 0x3; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
    //     puVar4 = puVar3;
    //     puVar3 = (puVar3 + 0x1);
    //     puVar4 = (puVar4 + 0x1);
    //   }
    //   if ((local_8 & 0x1) == 0x0) {
    //     uVar2 = uVar2 - 0x2;
    //     local_c = local_c + 0x1;
    //     local_10 = local_10 + 0x1;
    //     puVar4 = (puVar4 + -0x1);
    //   }
    //   puVar4 = (puVar4 + local_10 + local_c);
    //   puVar3 = (puVar3 + uVar2);
    // }
    return 0x0;
}



unsafe fn FUN_00401528(param_1: i32,param_2: i32) -> u32

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut puVar4: *mut u8;
    let mut puVar5: *mut u8;
    let mut local_8: u32;

    puVar4 = (param_2 + 0x5ef);
    puVar5 = (param_1 + 0x775);
    iVar3 = 0xa;
    iVar2 = 0x1c;
    // TODO
    // for (local_8 = 0xa; iVar1 = iVar2, local_8 != 0x0; local_8 = local_8 - 0x1) {
    //   for (; iVar1 != 0x0; iVar1 = iVar1 + -0x1) {
    //     *puVar5 = *puVar4;
    //     puVar4 = puVar4 + -0x1;
    //     puVar5 = puVar5 + -0x1;
    //   }
    //   if ((local_8 & 0x1) == 0x0) {
    //     iVar2 = iVar2 + -0x2;
    //     iVar3 = iVar3 + 0x1;
    //     puVar5 = puVar5 + 0x1;
    //   }
    //   puVar5 = puVar5 + iVar3 * -0x2;
    // }
    return 0x0;
}



unsafe fn FUN_00401584(param_1: i32,param_2: i32) -> u32

{
    let mut uVar1: u32;
    let mut uVar2: u32;
    let puVar3: *mut u32;
    let puVar4: *mut u32;
    let mut local_10: i32;
    let mut local_c: i32;
    let mut local_8: u32;

    puVar3 = (param_2 + 0x2f8);
    puVar4 = (param_1 + 0x3c0);
    local_c = 0x0;
    local_10 = 0x1c;
    uVar2 = 0x14;
    // TODO
    // for (local_8 = 0x14; local_8 != 0x0; local_8 = local_8 - 0x1) {
    //   for (uVar1 = uVar2 >> 0x2; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
    //     *puVar4 = *puVar3;
    //     puVar3 = puVar3 + 0x1;
    //     puVar4 = puVar4 + 0x1;
    //   }
    //   for (uVar1 = uVar2 & 0x3; uVar1 != 0x0; uVar1 = uVar1 - 0x1) {
    //     puVar4 = puVar3;
    //     puVar3 = (puVar3 + 0x1);
    //     puVar4 = (puVar4 + 0x1);
    //   }
    //   if ((local_8 & 0x1) == 0x0) {
    //     uVar2 = uVar2 - 0x2;
    //     local_c = local_c + 0x1;
    //     local_10 = local_10 + 0x1;
    //     puVar4 = (puVar4 + -0x1);
    //   }
    //   puVar4 = (puVar4 + local_10 + local_c);
    //   puVar3 = (puVar3 + uVar2);
    // }
    return 0x0;
}



unsafe fn FUN_004015f4(param_1: i32,param_2: i32) -> u32

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut puVar3: *mut u8;
    let mut puVar4: *mut u8;
    let mut local_10: i32;
    let mut local_c: i32;
    let mut local_8: u32;

    puVar3 = (param_2 + 0x2db);
    puVar4 = (param_1 + 0x3a3);
    local_c = 0x0;
    local_10 = 0x1c;
    iVar2 = 0x14;
    // TODO
    // for (local_8 = 0x14; iVar1 = iVar2, local_8 != 0x0; local_8 = local_8 - 0x1) {
    //   for (; iVar1 != 0x0; iVar1 = iVar1 + -0x1) {
    //     *puVar4 = *puVar3;
    //     puVar3 = puVar3 + -0x1;
    //     puVar4 = puVar4 + -0x1;
    //   }
    //   if ((local_8 & 0x1) == 0x0) {
    //     iVar2 = iVar2 + -0x2;
    //     local_c = local_c + 0x1;
    //     local_10 = local_10 + 0x1;
    //     puVar4 = puVar4 + 0x1;
    //   }
    //   puVar4 = puVar4 + (-local_10 - local_c);
    //   puVar3 = puVar3 + -0x14;
    // }
    return 0x0;
}



fn FUN_00401660(param_1: i32,param_2: i32,param_3: i32,param_4: i32)

{
    FUN_004953d7();
    FUN_00497c58(param_1,param_2,param_3 + -0x1,param_2,&DAT_004bd010,0x0,0x4);
    FUN_00497c58(param_1,param_2,param_1,param_4 + -0x1,&DAT_004bd010,0x0,0x4);
    FUN_00497c58(param_1,param_4 + -0x1,param_3 + -0x1,param_4 + -0x1,&DAT_004bd014,0x0,0x4);
    FUN_00497c58(param_3 + -0x1,param_2,param_3 + -0x1,param_4 + -0x1,&DAT_004bd014,0x0,0x4);
    FUN_00497896(param_1 + 0x1,param_2 + 0x1,param_3 + -0x2,param_2 + 0x1,0xe);
    FUN_00497896(param_1 + 0x1,param_2 + 0x1,param_1 + 0x1,param_4 + -0x2,0xe);
    FUN_00497896(param_1 + 0x1,param_4 + -0x2,param_3 + -0x2,param_4 + -0x2,0xe);
    FUN_00497896(param_3 + -0x2,param_2 + 0x1,param_3 + -0x2,param_4 + -0x2,0xe);
    FUN_0049536f();
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00401796(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: i32) -> u32

{
    let mut local_224: i32;
    let mut local_218: String;
    let mut local_11c: [u32;0x7];
    let mut local_ff: i32;
    let mut local_fb: i32;
    let ppuStack215: *mut *mut u8;
    let mut local_2b: String;
    let mut local_24: u32;
    let mut local_20: u32;
    let mut local_1c: u32;
    let local_18: *mut i32;

    _DAT_004c5020 = param_3;
    DAT_00599d3c = 0x1;
    DAT_004c5014 = FUN_0049c2c9(0x104);
    DAT_0059a9f4 = param_1;
    local_1c = 0x0;
    if ((DAT_00595f56 != (param_1 + 0xa)) || (DAT_00595f56._2_2_ != (param_1 + 0xc))) {
        local_1c = ((DAT_00595f68 & 0x1) == 0x0);
        DAT_00595f68 = DAT_00595f68 | 0x1;
    }
    FUN_004990e0(local_11c,0x0,s_efs_res_004c05fa,s_BuildUnit_004c05f0);
    FUN_0049c2e0(DAT_004c5014,s_blank_flc_004c0602);
    local_18 = FUN_0049c4bd(DAT_004c5014,&DAT_004c060c);
    if (local_18 == 0x0) {
        FUN_0049c2e0(DAT_004c5014,PTR_s_blank_flc_004bea98);
    }
    else {
        FUN_0049ca40(local_18);
    }
    FUN_0049cc70(local_218,local_11c,0xbb7,local_ff + 0x13e,local_fb + 0xb,0x0,0x0,0x0,DAT_004c5014);
    FUN_0049bf40(local_11c,local_218);
    if ((param_4 != -0x1) && (param_5 != -0x1)) {
        FUN_0049a770(local_11c,0x415,param_4,param_5);
    }
    if (param_3 != 0x0) {
        FUN_0049bf80(local_11c,0x65,0x414,0x0,0x0);
        FUN_0049bf80(local_11c,0x65,0x410,0x0,0x0);
    }
    local_24 = FUN_0049bb50(local_11c,FUN_00401ec1);
    if (param_2 != 0x0) {
        FUN_00431d0a(&DAT_005967b8);
        // TODO:
        // for (local_224 = 0x0; local_224 < 0x14; local_224 = local_224 + 0x1) {
        //   if (((((&DAT_00595744 + local_224 * 0x5e) & 0x1) != 0x0) &&
        //       (((&DAT_00595744 + local_224 * 0x5e) & 0x2) != 0x0)) &&
        //      ((*(*(&DAT_00595740 + local_224 * 0x5e) + 0x3a) & 0x4) == 0x0)) {
        //     FUN_00431efd(&DAT_005967b8,*(&DAT_00595740 + local_224 * 0x5e));
        //   }
        // }
    }
    DAT_00599d3c = 0x0;
    FUN_004864f7();
    FUN_0049af50(DAT_004c5014);
    if (local_1c != 0x0) {
        DAT_00595f68 = DAT_00595f68 & 0xfe;
    }
    local_20 = local_24;
    FUN_0049cef0(&local_218,0x0);
    ppuStack215 = &PTR_FUN_004c3d34;
    if (local_2b != 0x0) {
        FUN_00499b30(local_11c,local_2b);
    }
    FUN_0049a1c0(local_11c,0x1);
    return local_20;
}



fn FUN_00401ba7(param_1: i32,param_2: &mut String)

{
    let mut local_68: [u8;0x40];
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_1c: *mut u8;
    let mut local_18: i32;
    let mut local_14: i32;

    if ((*param_2 == -0x1) || (*(*(&DAT_00582938 + param_2[0x1] * 0x4 + *param_2 * 0x18) + 0xed) == -0x1)) {
        local_18 = 0x0;
    }
    else {
        local_18 = 0x1;
    }
    local_14 = 0x0;
    // TODO
    // for (local_28 = 0x0; local_28 < 0xd; local_28 = local_28 + 0x1) {
    //   if ((local_28 != 0x4) && (local_28 != 0x1)) {
    //     if ((local_28 == 0x0) && (local_18 != 0x0)) {
    //       local_14 = local_14 + 0x1;
    //     }
    //     else {
    //       FUN_004968e7(local_14 * 0x26 + *(param_1 + 0x1d) + 0xa,*(param_1 + 0x21) + 0xa5,0x22,0x24,0xe);
    //       FUN_00496ee6(&DAT_00596a58 + local_28 * 0x3da,local_14 * 0x26 + *(param_1 + 0x1d) + 0xa,
    //                    *(param_1 + 0x21) + 0xa5,0x22,0x1d);
    //       if (*param_2 == -0x1) {
    //         local_14 = local_14 + 0x1;
    //       }
    //       else {
    //         local_24 = *(local_28 * 0x4 + *(&DAT_00582938 + param_2[0x1] * 0x4 + *param_2 * 0x18) + 0xb9);
    //         FUN_0049c2e0(local_68,&DAT_004c060f);
    //         local_20 = FUN_00462571((&DAT_00568210 + DAT_004c9754 * 0x1e22),DAT_00595f52 >> 0x10,local_28);
    //         local_20 = local_24 - local_20;
    //         if (local_20 < 0x1) {
    //           local_1c = &DAT_00575757;
    //         }
    //         else {
    //           local_1c = 0xe2e3e4;
    //         }
    //         FUN_00497567(local_14 * 0x26 + *(param_1 + 0x1d) + 0x1b,*(param_1 + 0x21) + 0xbe,local_68,
    //                      0x1e,local_1c,-0x1,local_1c,LPCSTR_005b9218,0x11);
    //         local_14 = local_14 + 0x1;
    //       }
    //     }
    //   }
    // }
    if (*param_2 != -0x1) {
        if (*(*(&DAT_00582938 + param_2[0x1] * 0x4 + *param_2 * 0x18) + 0xed) != -0x1) {
            FUN_004968e7(*(param_1 + 0x1d) + 0xa,*(param_1 + 0x21) + 0xa5,0x22,0x24,0xe);
            FUN_00496ac0(
                (&DAT_004d6058 +
                    *(*(&DAT_00582938 + param_2[0x1] * 0x4 + *param_2 * 0x18) + 0xed) * 0x1c),
                *(param_1 + 0x1d) + 0xb,*(param_1 + 0x21) + 0xa6,0x20,0x20);
        }
        if ((param_2[0x2] & 0x2U) == 0x0) {
            FUN_0049e5a0(*(param_1 + 0x1d) + 0xa,*(param_1 + 0x21) + 0xa5,0x22,0x24,0xe4);
        }
        else {
            FUN_0049e5a0(*(param_1 + 0x1d) + 0xa,*(param_1 + 0x21) + 0xa5,0x22,0x24,0xe);
        }
        FUN_0048eb27(param_1,*param_2,param_2[0x1],0x1);
    }
    return;
}



// WARNING: Type propagation algorithm not settling
// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

unsafe fn FUN_00401ec1(param_1: *mut *mut u32,param_2: u32,param_3: u32,param_4: *mut i32) -> u32

{
    let mut uVar1: u32;
    let mut pcVar2: String;
    let mut uVar3: u32;
    let mut iVar4: i32;
    let puVar5: *mut u32;
    let puVar6: *mut u32;
    let in_stack_fffff904: i16;
    let mut in_stack_fffff908: u32;
    let mut iVar7: i32;
    let mut uVar8: u32;
    let bVar9: u8;
    let mut local_674: u32;
    let local_670: u16;
    let local_66e: u8;
    let local_66d: u8;
    let local_66c: u8;
    let local_66a: i16;
    let local_666: u8;
    let local_665: u8;
    let local_661: u8;
    let local_65f: u8;
    let mut local_65a: u32;
    let local_656: u8;
    let local_655: u8;
    let mut local_644: String;
    let mut local_640: String;
    let mut local_63c: String;
    let local_638: u8 [0x100];
    let mut local_538: String;
    let mut local_534: *mut u8;
    let mut local_530: u32;
    let mut local_52c: String;
    let local_528: *mut i32;;
    let local_524: u8 [0x40];
    let mut local_4e4: i32;
    let mut local_4e0: u32;
    let mut local_4dc: i32;
    let mut local_4d8: i32;
    let mut local_4d4: String;
    let local_4d0: *mut i32;;
    let local_4cc: *mut i32;;
    let mut local_4c8: [u8;0x121];
    let mut local_44: String;
    let local_40: *mut *mut char;
    let local_3c: *mut *mut char;
    let mut local_38: String;
    let mut local_34: i32;
    let mut local_30: i32;
    let local_2c: *mut u32;
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_1c: u32;
    let mut local_14: u32;

    local_14 = param_2;
    if (param_2 < 0x406) {
        if (param_2 < 0x401) {
            if (param_2 != 0x100) {
                return 0x0;
            }
            if ((param_4 != 0x0) && (iVar4 = FUN_004a11c0(param_4), iVar4 == 0x1b)) {
                return 0x1;
            }
            return 0x0;
        }
        if (param_2 < 0x402) {
            DAT_004c5010 = FUN_00482796(DAT_00595f52 >> 0x10);
            _DAT_004c5018 = 0x0;
            local_28 = 0x1;
            local_24 = 0x0;
            local_2c = FUN_0049ea90();
            if (local_2c == 0x0) {
                pcVar2 = FUN_00499050(DAT_005b9bd8,0x7d01);
                pop_err_msg_box_and_exit_004a02f5(pcVar2);
            }
            local_20 = FUN_00403cdc();
            local_1c._0_1_ = -0x1;
            local_1c._1_1_ = '\0';
            local_1c._2_1_ = local_1c._2_1_ | 0x3;
            FUN_0049eae0(local_2c,&local_1c,0x3);
            // TODO
            // for (local_30 = 0x0; local_30 < 0x5c; local_30 = local_30 + 0x1) {
            //   local_34 = 0x0;
            //   while ((local_34 < 0x5 && (*(&DAT_00582938 + local_34 * 0x4 + local_30 * 0x18) != 0x0))) {
            //     if (((*(DAT_0059a9f4 + 0xc) >> 0x10 ==
            //           *(*(&DAT_00582938 + local_30 * 0x18 + local_34 * 0x4) + 0xf1)) ||
            //         (*(*(&DAT_00582938 + local_34 * 0x4 + local_30 * 0x18) + 0xf1) == 0x63)) &&
            //        (((((((&DAT_00569c30)
            //              [*(*(&DAT_00582938 + local_30 * 0x18 + local_34 * 0x4) + 0xf9) * 0x9 +
            //               DAT_004c9754 * 0x1e22] & 0x1) != 0x0 &&
            //            (((&DAT_00569c30)
            //              [*(*(&DAT_00582938 + local_30 * 0x18 + local_34 * 0x4) + 0xfd) * 0x9 +
            //               DAT_004c9754 * 0x1e22] & 0x1) != 0x0)) &&
            //           (((&DAT_00569c30)
            //             [*(*(&DAT_00582938 + local_30 * 0x18 + local_34 * 0x4) + 0x101) * 0x9 +
            //              DAT_004c9754 * 0x1e22] & 0x1) != 0x0)) &&
            //          (((&DAT_00569c30)
            //            [*(*(&DAT_00582938 + local_30 * 0x18 + local_34 * 0x4) + 0x105) * 0x9 +
            //             DAT_004c9754 * 0x1e22] & 0x1) != 0x0)) &&
            //         ((*(*(&DAT_00582938 + local_34 * 0x4 + local_30 * 0x18) + 0x41) != 0x4 ||
            //          (iVar4 = FUN_0044ab7b(*(DAT_0059a9f4 + 0x6) >> 0x10,*(DAT_0059a9f4 + 0x8) >> 0x10,
            //                                *(DAT_0059a9f4 + 0xa) >> 0x10), iVar4 != 0x0)))))) {
            //       FUN_004a0430(&local_1c,0x0,0x3);
            //       local_1c._0_1_ = (char)local_30;
            //       local_1c._1_1_ = (char)local_34;
            //       iVar4 = FUN_00403dd7(DAT_00595f52 >> 0x10,local_30,local_34);
            //       if (iVar4 == 0x0) {
            //         local_1c._2_1_ = local_1c._2_1_ & 0xfe;
            //       }
            //       else {
            //         local_1c._2_1_ = local_1c._2_1_ | 0x1;
            //       }
            //       iVar4 = FUN_00452bf8(&DAT_00595740,local_30,local_34);
            //       if ((iVar4 == 0x0) &&
            //          ((*(DAT_0059a9f4 + 0x20) >> 0x10 != local_30 ||
            //           (*(DAT_0059a9f4 + 0x22) >> 0x10 != local_34)))) {
            //         local_1c._2_1_ = local_1c._2_1_ & 0xfd;
            //       }
            //       else {
            //         local_1c._2_1_ = local_1c._2_1_ | 0x2;
            //       }
            //       if (((local_20 != 0x0) && ((local_20 + 0x27) == (char)local_1c)) &&
            //          ((local_20 + 0x28) == local_1c._1_1_)) {
            //         local_1c._2_1_ = local_1c._2_1_ | 0x4;
            //       }
            //       FUN_0049eae0(local_2c,&local_1c,0x3);
            //       if (*(DAT_0059a9f4 + 0x12) >> 0x10 == local_30) {
            //         _DAT_004c5018 = 0x1;
            //         local_24 = local_28;
            //       }
            //       else {
            //         if ((local_1c._2_1_ & 0x4) != 0x0) {
            //           local_24 = local_28;
            //         }
            //       }
            //       local_28 = local_28 + 0x1;
            //     }
            //     local_34 = local_34 + 0x1;
            //   }
            // }
            FUN_0049bf80(param_1,0xc8,0x503,0x0,local_2c);
            FUN_0049bf80(param_1,0xc8,0x502,local_24,0x0);
            uVar1 = FUN_004a06f6(LPCSTR_005b9218);
            FUN_0049bf80(param_1,0xc8,0x509,0x0,uVar1);
            return 0x1;
        }
        if (param_2 != 0x405) {
            return 0x0;
        }
        FUN_004953d7();
        local_528 = FUN_0049bf80(param_1,0xc8,0x501,0x0,0x0);
        FUN_00401ec1(param_1,0x406,0xc8,local_528);
        local_52c = FUN_0049bf80(param_1,0xc8,0x510,local_528,0x0);
        FUN_00401ba7(param_1,&local_52c);
        FUN_0049536f();
        return 0x0;
    }
    if (param_2 < 0x407) {
        if (param_3 != 0xc8) {
            return 0x0;
        }
        local_38 = FUN_0049bf80(param_1,0xc8,0x510,param_4,0x0);
        FUN_004953d7();
        if (*local_38 == -0x1) {
            FUN_004968e7(*(param_1 + 0x1d) + 0xa,*(param_1 + 0x21) + 0xa,0x22,0x22,0xe);
        }
        else {
            local_40 = (&DAT_004d6058 + *local_38 * 0x1c);
            local_44 = *local_40;
            local_3c = local_40;
            FUN_004906c1(local_4c8,&local_44,(char)*(&DAT_004be9e8 + DAT_004c9754 * 0x4),0x0,0x22,-0x1);
            FUN_00496ac0(local_4c8,*(param_1 + 0x1d) + 0xa,*(param_1 + 0x21) + 0xa,0x22,0x22);
        }
        FUN_0049e640(*(param_1 + 0x1d) + 0xa,*(param_1 + 0x21) + 0xa,0x22,0x22,0xce,0xca,0xcc,0x1);
        if (*local_38 == -0x1) {
            FUN_0049c2e0(DAT_004c5014,s_blank_flc_004c0622);
        }
        else {
            FUN_0049c2e0(DAT_004c5014,s_flc__s_004c062c);
            local_4cc = FUN_0049c4bd(DAT_004c5014,&DAT_004c0633);
            if (local_4cc == 0x0) {
                FUN_0049c2e0(DAT_004c5014,PTR_s_blank_flc_004bea98);
            }
            else {
                FUN_0049ca40(local_4cc);
            }
        }
        FUN_0049bf80(param_1,0xbb7,0x501,0x0,DAT_004c5014);
        FUN_00401ba7(param_1,&local_38);
        FUN_0049536f();
        return 0x0;
    }
    if (0x40b < param_2) {
        if (param_2 < 0x40d) {
            FUN_004953d7();
            FUN_004a08c5(s_pcx_unitbg2_pcx_004c0612,*(param_1 + 0x1d),*(param_1 + 0x21),
                         *(param_1 + 0x25),*(param_1 + 0x29),0x0,0x0,0x0,0x1);
            FUN_0049536f();
            return 0x1;
        }
        if (param_2 < 0x411) {
            return 0x0;
        }
        if (param_2 < 0x412) {
            return 0x1;
        }
        if (param_2 != 0x412) {
            return 0x0;
        }
        FUN_004953d7();
        local_4d0 = param_4;
        local_4d4 = param_4[0x5];
        local_4d8 = *local_4d4;
        local_4dc = local_4d4[0x1];
        if (param_4[0x4] == 0x1) {
            if ((*(DAT_0059a9f4 + 0x12) >> 0x10 == local_4d8) && (*(DAT_0059a9f4 + 0x14) >> 0x10 == local_4dc))
            {
                local_4e0 = 0x272727;
                local_4e4 = 0xcaccce;
            }
            else {
                local_4e0 = 0xe0e0e;
                if (((local_4d4[0x2] & 0x1U) == 0x0) || ((local_4d4[0x2] & 0x2U) == 0x0)) {
                    local_4e4 = 0xe3e5e7;
                }
                else {
                    local_4e4 = 0xcaccce;
                }
            }
        }
        else {
            if ((*(DAT_0059a9f4 + 0x12) >> 0x10 == local_4d8) && (*(DAT_0059a9f4 + 0x14) >> 0x10 == local_4dc))
            {
                local_4e0 = 0x272727;
            }
            else {
                if (((local_4d4[0x2] & 0x1U) == 0x0) || ((local_4d4[0x2] & 0x2U) == 0x0)) {
                    local_4e0 = 0xe3e5e7;
                }
                else {
                    local_4e0 = 0xcaccce;
                }
            }
            local_4e4 = 0xe0e0e;
        }
        if (local_4d8 == -0x1) {
            bVar9 = 0x10;
            uVar8 = 0xcaccce;
            iVar4 = param_4[0x2];
            uVar1 = local_4e0;
            iVar7 = local_4e4;
            puVar5 = LPCSTR_005b9218;
            pcVar2 = FUN_00499050(DAT_0059679c,0x7145);
            FUN_00497567(*local_4d0,local_4d0[0x1],pcVar2,iVar4,uVar1,iVar7,uVar8,puVar5,bVar9);
            FUN_0049536f();
            return 0x1;
        }
        if ((*(DAT_0059a9f4 + 0x12) >> 0x10 == local_4d8) && (*(DAT_0059a9f4 + 0x14) >> 0x10 == local_4dc)) {
            FUN_0049c2e0(local_524,&DAT_004c0636);
        }
        else {
            FUN_0049c2e0(local_524,&DAT_004c0639);
        }
        FUN_00497567(*local_4d0,local_4d0[0x1],*(&DAT_00582938 + local_4d8 * 0x18 + local_4dc * 0x4),0xc8,local_4e0,
                     local_4e4,0xcaccce,LPCSTR_005b9218,0x10);
        FUN_00497567(*local_4d0 + local_4d0[0x2],local_4d0[0x1],local_524,local_4d0[0x2] + -0xc8,local_4e0,local_4e4,
                     0xcaccce,LPCSTR_005b9218,0x14);
        FUN_0049536f();
        return 0x1;
    }
    if (param_2 != 0x407) {
        return 0x0;
    }
    local_530 = param_3;
    if (param_3 < 0xc1) {
        if (param_3 != 0x65) {
            return 0x0;
        }
        FUN_0049c140(param_1,0x1);
        return 0x0;
    }
    if ((0xc1 < param_3) && (param_3 != 0xc8)) {
        return 0x0;
    }
    local_534 = FUN_0049bf80(param_1,0xc8,0x501,0x0,0x0);
    if (local_534 == 0xffffffff) {
        return 0x0;
    }
    local_538 = FUN_0049bf80(param_1,0xc8,0x510,local_534,0x0);
    if ((*local_538 == -0x1) && (_DAT_004c5020 != 0x0)) {
        pcVar2 = FUN_00499050(DAT_0059679c,0x736f);
        uVar3 = FUN_0049d2e0(param_1,0x3,&pcVar2);
        if (uVar3 != 0x0) {
            FUN_0049c140(param_1,0x1);
            return 0x0;
        }
    }
    if (_DAT_004c5018 != 0x0) {
        if (*local_538 == -0x1) {
            pcVar2 = FUN_00499050(DAT_0059679c,0x73be);
            FUN_0049c2e0(local_638,pcVar2);
        }
        else {
            pcVar2 = FUN_00499050(DAT_0059679c,0x73b0);
            FUN_0049c2e0(local_638,pcVar2);
        }
        uVar3 = FUN_0049d2e0(param_1,0x3,local_638);
        if (uVar3 == 0x0) {
            return 0x0;
        }
    }
    if (((local_538[0x2] & 0x1U) == 0x0) && ((local_538[0x2] & 0x2U) != 0x0)) {
        uVar3 = FUN_00402fd5(DAT_004c5010,DAT_00595f52 >> 0x10,*local_538,local_538[0x1]);
    }
    else {
        if ((local_538[0x2] & 0x2U) == 0x0) {
            pcVar2 = FUN_00499050(DAT_0059679c,0x7399);
            FUN_0049c2e0(local_638,pcVar2);
            FUN_0049d2e0(param_1,0x1,local_638);
            return 0x0;
        }
        if (_DAT_004c5018 != 0x0)^ // goto LAB_00402b1f;
        if (*local_538 == -0x1) {
            pcVar2 = FUN_00499050(DAT_0059679c,0x736f);
            FUN_0049c2e0(local_638,pcVar2);
        }
        else {
            local_640 = *(&DAT_00582938 + local_538[0x1] * 0x4 + *local_538 * 0x18);
            local_63c = local_640;
            iVar4 = FUN_00430d15(&local_640);
            if (iVar4 == 0x0) {
                local_644 = FUN_00499050(DAT_0059679c,0x7132);
            }
            else {
                local_644 = FUN_00499050(DAT_0059679c,0x7133);
            }
            pcVar2 = FUN_00499050(DAT_0059679c,0x73bb);
            FUN_0049c2e0(local_638,pcVar2);
        }
        uVar3 = FUN_0049d2e0(param_1,0x3,local_638);
    }
    if (uVar3 == 0x0) {
        return 0x0;
    }
    LAB_00402b1f:
    if ((_DAT_004c5018 != 0x0) &&
        (FUN_00403d5d(*(DAT_0059a9f4 + 0x6) >> 0x10,*(DAT_0059a9f4 + 0x8) >> 0x10,
                      *(DAT_0059a9f4 + 0xa) >> 0x10,*(DAT_0059a9f4 + 0x12) >> 0x10,
                      *(DAT_0059a9f4 + 0x14) >> 0x10,DAT_0059a9f4), (DAT_0059a9f4 + 0x22) != -0x1)) {
        FUN_00486065(&local_674);
        local_674._0_2_ = (DAT_0059a9f4 + 0x8);
        local_674._2_2_ = *(DAT_0059a9f4 + 0xa);
        local_670 = *(DAT_0059a9f4 + 0xc);
        local_66e = DAT_004c9754;
        local_66d = (DAT_0059a9f4 + 0x22);
        local_66c = (DAT_0059a9f4 + 0x24);
        FUN_00486b6b(&local_674,*(DAT_0059a9f4 + 0x1c) >> 0x10);
        local_66a = (*(&DAT_00582938 + local_66c * 0x4 + local_66d * 0x18) + 0x41);
        local_665 = (*(&DAT_00582938 + local_66c * 0x4 + local_66d * 0x18) + 0x45);
        local_666 = (&DAT_00569b75)[DAT_004c9754 * 0x1e22] != '\0';
        local_661 = (DAT_0059a9f4 + 0x30);
        local_656 = (*(&DAT_00582938 + local_66d * 0x18 + local_66c * 0x4) + 0xed);
        local_655 = 0x0;
        if ((*(DAT_0059a9f4 + 0x2d) & 0x1) == 0x0) {
            local_65a = local_65a | 0x1;
            iVar4 = FUN_004888c0(*(DAT_0059a9f4 + 0x6) >> 0x10,*(DAT_0059a9f4 + 0x8) >> 0x10,
                                 *(DAT_0059a9f4 + 0xa) >> 0x10,0x0);
            local_65f = iVar4;
        }
        else {
            local_65f = (DAT_0059a9f4 + 0x12);
            local_65a = local_65a | 0x81;
        }
        iVar4 = FUN_004889db(&local_674,0x0);
        if (iVar4 != 0x0) {
            puVar5 = &local_674;
            puVar6 = &stack0xfffff93c;
            // TODO:
            // for (iVar4 = 0xb; iVar4 != 0x0; iVar4 = iVar4 + -0x1) {
            //   *puVar6 = *puVar5;
            //   puVar5 = puVar5 + 0x1;
            //   puVar6 = puVar6 + 0x1;
            // }
            puVar6 = puVar5;
            puVar5 = &stack0xfffff93c;
            puVar6 = &stack0xfffff904;
            // TODO
            // for (iVar4 = 0xb; iVar4 != 0x0; iVar4 = iVar4 + -0x1) {
            //   *puVar6 = *puVar5;
            //   puVar5 = puVar5 + 0x1;
            //   puVar6 = puVar6 + 0x1;
            // }
            puVar6 = puVar5;
            FUN_00485463(in_stack_fffff904,in_stack_fffff908);
        }
    }
    if (*local_538 == -0x1) {
        *(DAT_0059a9f4 + 0x14) = 0xffff;
        *(DAT_0059a9f4 + 0x16) = 0xffff;
        *(DAT_0059a9f4 + 0x18) = 0xffff;
    }
    else {
        (DAT_0059a9f4 + 0x14) = *local_538;
        (DAT_0059a9f4 + 0x16) = local_538[0x1];
        *(DAT_0059a9f4 + 0x18) =
            *(*(&DAT_00582938 + local_538[0x1] * 0x4 + *local_538 * 0x18) + 0xf5);
        FUN_004664c9(*(DAT_0059a9f4 + 0x6) >> 0x10,*(DAT_0059a9f4 + 0x8) >> 0x10,
                     *(DAT_0059a9f4 + 0xa) >> 0x10,*local_538,local_538[0x1],DAT_0059a9f4);
        FUN_00466d5a(*(DAT_0059a9f4 + 0x6) >> 0x10,*local_538,local_538[0x1],DAT_004c9754);
    }
    FUN_0049c140(param_1,0x1);
    return 0x0;
}


fn FUN_00416457(param_1: i32,param_2: *mut u32,param_3: *mut u32) -> u32

{
    let bVar1: u8;
    let sVar2: i16;
    let mut iVar3: i32;
    let mut bVar4: bool;
    let puVar5: *mut u32;
    let mut iVar6: i32;
    let mut uVar7: u32;
    let mut uVar8: u32;
    let mut iVar9: i32;
    let mut local_10c: u32;
    let mut local_108: i32;
    let mut local_f8: i32;
    let mut local_98: u32;
    let mut local_80: i32;
    let mut local_6c: i32;
    let local_30: *mut u32;
    let local_2c: *mut u32;
    let mut local_20: i32;

    if ((((param_1 + 0x48) == '\0') && ((*(param_1 + 0x3a) & 0x1) == 0x0)) &&
        (iVar9 = *(*(&DAT_00582938 +
            (*(param_1 + 0x25) >> 0x18) * 0x4 + (*(param_1 + 0x24) >> 0x18) * 0x18) +
            0xa5), iVar9 != 0x0)) {
        for (local_20 = 0x0; local_20 < iVar9; local_20 = local_20 + 0x1) {
            iVar3 = *(local_20 * 0x4 + param_1 + 0x10);
            if (iVar3 != 0x0) {
                if ((param_1 + 0x20) != *(iVar3 + 0x46) >> 0x18) {
                    return 0x0;
                }
                if ((-0x1 < *(iVar3 + 0x48) >> 0x10) &&
                    ((*(&DAT_004c6168 + (*(iVar3 + 0x48) >> 0x10) * 0x4) >> 0x18 & 0xfffffffeU) == 0x2)) {
                    (&DAT_004c616b)[(*(iVar3 + 0x48) >> 0x10) * 0x4] = 0x0;
                    puVar5 = FUN_0041dd5a(*(&DAT_004c6167 + (*(iVar3 + 0x48) >> 0x10) * 0x4) >> 0x18,
                                          *(&DAT_004c6165 + (*(iVar3 + 0x48) >> 0x10) * 0x4) >> 0x18,
                                          *(&DAT_004c6166 + (*(iVar3 + 0x48) >> 0x10) * 0x4) >> 0x18);
                    if (puVar5 == 0x0) {
                        return 0x0;
                    }
                    iVar6 = FUN_0041d5ed(*(iVar3 + 0x24) >> 0x18,(iVar3 + 0x20),
                                         *(&DAT_004c6165 + (*(iVar3 + 0x48) >> 0x10) * 0x4) >> 0x18,
                                         *(&DAT_004c6166 + (*(iVar3 + 0x48) >> 0x10) * 0x4) >> 0x18,param_2,param_3)
                    ;
                    if (iVar6 != 0x0) {
                        for (local_6c = 0x0; local_6c < iVar9; local_6c = local_6c + 0x1) {
                            if (*(local_6c * 0x4 + param_1 + 0x10) != 0x0) {
                                (*(local_6c * 0x4 + param_1 + 0x10) + 0x41) =
                                    (&DAT_004c6168)[(*(iVar3 + 0x48) >> 0x10) * 0x4];
                                (*(local_6c * 0x4 + param_1 + 0x10) + 0x42) =
                                    (&DAT_004c6169)[(*(iVar3 + 0x48) >> 0x10) * 0x4];
                                (*(local_6c * 0x4 + param_1 + 0x10) + 0x48) = 0xf;
                            }
                        }
                        return 0x1;
                    }
                    return 0x0;
                }
                bVar4 = false;
                bVar1 = *(iVar3 + 0x27);
                if (bVar1 < 0x2d) {
                    if (bVar1 != 0x2c)^ // goto LAB_004168b2;
                    local_80 = 0x9c4;
                    sVar2 = (iVar3 + 0x20);
                    while (local_80 = local_80 + -0x1, local_80 != -0x1) {
                        uVar7 = FUN_004a2edc();
                        uVar7 = uVar7 % 0x2c;
                        uVar8 = FUN_004a2edc();
                        local_98 = (uVar8 % 0x20) * 0x2;
                        if ((uVar7 & 0x1) == 0x0) {
                            local_98 = local_98 | 0x1;
                        }
                        uVar8 = *(*(&DAT_004d7d50 + uVar7 * 0x4 + sVar2 * 0x3890) + local_98 * 0x4) & 0xf;
                        if ((uVar8 != 0x0) && (uVar8 != 0x4)) {
                            *param_2 = uVar7;
                            *param_3 = local_98;
                            return 0x1;
                        }
                    }
                }
                else {
                    if ((0x2d < bVar1) && (bVar1 != 0x35)) {
                        LAB_004168b2:
                        if (((&DAT_004c7108)[(iVar3 + 0x20) * 0x4] & 0x1) == 0x0) {
                            return 0x0;
                        }
                        for (local_30 = *DAT_005967c8; local_30 != 0x0;
                            local_30 = *local_30) {
                            if (((local_30 + 0xe) == 0xf) || ((local_30 + 0xe) == 0x10)) {
                                *(local_30 + 0x32) = 0x4b;
                            }
                            else {
                                *(local_30 + 0x32) = 0x0;
                            }
                        }
                        for (local_2c = *DAT_005967b0; local_2c != 0x0;
                            local_2c = *local_2c) {
                            if ((((iVar3 + 0x20) == (local_2c + 0x8)) &&
                                (iVar9 = *(*(&DAT_00582938 +
                                    (*(local_2c + 0x25) >> 0x18) * 0x4 +
                                    (local_2c[0x9] >> 0x18) * 0x18) + 0x121), iVar9 != 0x0)) &&
                                (puVar5 = FUN_00481784((local_2c + 0x8),(local_2c + 0x22),
                                                       (local_2c + 0x9)), puVar5 != 0x0)) {
                                (puVar5 + 0x32) = (puVar5 + 0x32) + iVar9;
                            }
                        }
                        local_f8 = 0x1869f;
                        for (local_30 = *DAT_005967c8; local_30 != 0x0;
                            local_30 = *local_30) {
                            if ((((((iVar3 + 0x20) == (local_30 + 0x2)) && ((local_30 + 0x4) == 0xd)) &&
                                ((iVar9 = FUN_00481a44(local_30), iVar9 == 0x0 ||
                                    (((local_30 + 0xe) == 0xf || ((local_30 + 0xe) == 0x10)))))) &&
                                ((local_30 + 0xe) != 0x2)) && (local_30[0xc] >> 0x10 < local_f8)) {
                                local_108 = local_30[0x2] >> 0x10;
                                local_10c = *(local_30 + 0xa) >> 0x10;
                                local_f8 = local_30[0xc] >> 0x10;
                            }
                        }
                        if (local_f8 != 0x1869f) {
                            iVar9 = FUN_0041d5ed(*(iVar3 + 0x24) >> 0x18,(iVar3 + 0x20),local_108,local_10c,
                                                 param_2,param_3);
                            if (iVar9 != 0x0) {
                                return 0x1;
                            }
                            return 0x0;
                        }
                        return 0x0;
                    }
                    loop {
                        for (local_30 = *DAT_005967c8; local_30 != 0x0;
                            local_30 = *local_30) {
                            if ((*(local_30 + 0xe) >> 0x10 == DAT_004c9754) &&
                                ((iVar3 + 0x20) == (local_30 + 0x2))) {
                                bVar4 = true;
                                uVar7 = FUN_004a2edc();
                                if (uVar7 % 0xa == 0x0) {
                                    *param_2 = local_30[0x2] >> 0x10;
                                    *param_3 = *(local_30 + 0xa) >> 0x10;
                                    return 0x1;
                                }
                            }
                        }
                    } while (bVar4);
                }
            }
        }
    }
    return 0x0;
}



fn FUN_00416cb8()

{
    let mut iVar1: i32;
    let local_28: *mut u32;
    let local_24: *mut u32;
    let mut local_20: u32;
    let mut local_1c: u32;
    let local_18: *mut u32;

    local_18 = FUN_004840cd(&local_28,&local_24,-0x1);
    while (local_24 != 0x0) {
        if ((*(local_24 + 0x23) >> 0x18 == DAT_004c9754) &&
            (iVar1 = FUN_00416457(local_24,&local_20,&local_1c), iVar1 != 0x0)) {
            FUN_00431d0a(&DAT_005967b8);
            DAT_005967bc = FUN_00434de1(local_24);
            iVar1 = FUN_00457f10(&DAT_005967b8,(local_24 + 0x8),local_20,local_1c,0x1);
            if ((iVar1 != 0x0) && (DAT_005967bc != 0x0)) {
                DAT_005967bc = FUN_00434e1a(&DAT_005967b8,(DAT_005967bc + 0x22),
                                            (DAT_005967bc + 0x24),0x0);
            }
        }
        local_24 = local_28;
        if (local_28 != 0x0) {
            local_28 = *local_28;
        }
    }
    FUN_0048418d(&local_28);
    return;
}



fn FUN_00416e5d(param_1: i32) -> i32

{
let mut uVar1: u32;
let mut local_4c: i32;
let mut local_20: i32;
let mut local_1c: u32;
let mut local_18: i32;
let mut local_14: i32;

local_20 = -0x1;
local_1c = 0x3e7;
local_18 = -0xa;
for (local_14 = 0x0; local_14 < 0x28; local_14 = local_14 + 0x1) {
if ((((&DAT_004c7109)[local_14 * 0x4] & 0x40) == 0x0) && (((&DAT_004c7108)[local_14 * 0x4] & 0x1) != 0x0)) {
DAT_005967bc = FUN_00434de1(param_1);
uVar1 = FUN_004710a6(&DAT_005967b8,*(&DAT_005b7076 + (param_1 + 0x20) * 0x4e) >> 0x10,
*(&DAT_005b7078 + (param_1 + 0x20) * 0x4e) >> 0x10,
*(&DAT_005b7076 + local_14 * 0x4e) >> 0x10,
*(&DAT_005b7078 + local_14 * 0x4e) >> 0x10);
local_4c = 0x0;
if (((&DAT_004c7109)[local_14 * 0x4] & 0x8) != 0x0) {
local_4c = -0x5;
}
if (((&DAT_004c7108)[local_14 * 0x4] & 0x20) != 0x0) {
local_4c = local_4c + 0x1;
}
if (((&DAT_004c7108)[local_14 * 0x4] & 0x40) != 0x0) {
local_4c = local_4c + 0x1;
}
if (((&DAT_004c7108)[local_14 * 0x4] & 0x80) != 0x0) {
local_4c = local_4c + 0x1;
}
if (((&DAT_004c7109)[local_14 * 0x4] & 0x1) != 0x0) {
local_4c = local_4c + 0x1;
}
if (((&DAT_004c7108)[local_14 * 0x4] & 0x80) != 0x0) {
local_4c = local_4c + 0x1;
}
if (((&DAT_004c7109)[local_14 * 0x4] & 0x20) != 0x0) {
local_4c = local_4c + -0x2;
}
if ((uVar1 < local_1c) || ((uVar1 == local_1c && (local_18 < local_4c)))) {
local_20 = local_14;
local_18 = local_4c;
local_1c = uVar1;
}
}
}
return local_20;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00417035()

{
    let bVar1: u8;
    let mut uVar2: u32;
    let mut iVar3: i32;
    i32 aiStackY560 [0x73];
    let mut local_54: i32;
    let mut local_50: i32;
    i32 local_30 [0x4];
    let local_20: *mut u32;
    let mut local_1c: i32;
    let mut local_18: *mut u32 [0x2];

    FUN_004a0430(local_30,0x0,0x10);
    for (local_1c = 0x0; local_1c < _DAT_004c6160; local_1c = local_1c + 0x1) {
        local_30[*(&DAT_004c6168 + local_1c * 0x4) >> 0x18] =
            local_30[*(&DAT_004c6168 + local_1c * 0x4) >> 0x18] + 0x1;
    }
    FUN_004840cd(local_18,&local_20,-0x1);
    loop {
    if (local_20 == 0x0) {
        FUN_0048418d(local_18);
        return;
    }
    if ((*(local_20 + 0x23) >> 0x18 == DAT_004c9754) && ((local_20 + 0x12) == '\x02')) {
        *(local_20 + 0x4a) = 0xfffe;
        if (((local_20 + 0x2a) == 0x8) || ((local_20 + 0x8) == DAT_004d557c)) {
            (local_20 + 0x12) = 0x4;
        }
        else {
            bVar1 = *(local_20 + 0x27);
            if (bVar1 < 0x2d) {
                if (bVar1 == 0x2c) {
                    LAB_00417166:
                    if (DAT_004c9754 == 0x7) {
                        local_50 = 0x9c4;
                        loop {
                            local_50 = local_50 + -0x1;
                            if (local_50 == 0x0) break;
                            uVar2 = FUN_004a2edc();
                            local_54 = uVar2 % 0x28;
                        } while (((&DAT_004c7108)[local_54 * 0x4] & 0x8) == 0x0);
                        if (local_50 == 0x0) {
                            (local_20 + 0x12) = 0x4;
                        }
                        else {
                            (local_20 + 0x49) = local_54;
                        }^
                        // goto LAB_0041709a;
                    }
                }
            }
            else {
                if (bVar1 < 0x2e) {
                    LAB_00417134:
                        (local_20 + 0x12) = 0x4;^
                    // goto LAB_0041709a;
                }
                if (0x2e < bVar1) {
                    if (bVar1 < 0x30) {
                        if (DAT_004c9754 != 0x6)^ // goto LAB_00417166;
                        if (local_30[2] == 0x0) {
                            (local_20 + 0x12) = 0x4;
                        }
                        else {
                            *(local_20 + 0x4a) = 0xffff;
                        }^
                        // goto LAB_0041709a;
                    }
                    if (bVar1 == 0x35)^ // goto LAB_00417134;
                }
            }
            iVar3 = FUN_00416e5d(local_20);
            (local_20 + 0x49) = (char)iVar3;
            if ((local_20 + 0x49) == -0x1) {
                (local_20 + 0x12) = 0x4;
            }
        }
    }
    LAB_0041709a:
        local_20 = local_18[0];
    if (local_18[0] != 0x0) {
        local_18[0] = *local_18[0];
    }
} while( true );
}



fn FUN_00417257()

{
    i32 **local_1c;
    i32 **local_18 [0x2];

    FUN_004840cd(local_18,&local_1c,-0x1);
    while (local_1c != (i32 **)0x0) {
    if (((*(local_1c + 0x23) >> 0x18 == DAT_004c9754) && ((*(local_1c + 0x3a) & 0x1) == 0x0))
        && (((*(*(&DAT_00582938 +
        (*(local_1c + 0x25) >> 0x18) * 0x4 + (local_1c[0x9] >> 0x18) * 0x18) +
        0xa5) != 0x0 && ((local_1c + 0x12) == '\n')) || ((local_1c + 0x12) == '\x10'))))
    {
        FUN_0041361b(local_1c);
    }
    local_1c = local_18[0];
    if (local_18[0] != (i32 **)0x0) {
        local_18[0] = (i32 **)*local_18[0];
    }
}
    FUN_0048418d(local_18);
    return;
}



// WARNING: Could not reconcile some variable overlaps

fn FUN_00417354()

{
    let mut iVar1: i32;
    let mut local_178: i32;
    let mut local_174: i32;
    let mut local_104: u32;
    let mut local_100: u32;
    let mut local_fc: u32;
    let local_f8: *mut u32;
    let local_f4: *mut u32;
    let mut local_f0: u32;
    let local_ec: *mut u32;
    let local_e8: *mut u32;
    let mut local_e4: i32;
    let mut local_e0: i32;
    let mut local_dc: i32;
    let mut local_d8: i32;
    let local_d4: *mut u32;
    u32 **local_d0;
    let local_cc: *mut u32;
    let mut local_c8: u32;
    let local_c4: *mut u32;
    let local_c0: *mut u32;
    i32 local_bc [0x28];
    let local_1c: *mut u32;
    let mut local_18: *mut u32 [0x2];

    FUN_004a0430(local_bc,0x0,0xa0);
    for (local_18[0] = *DAT_005967b0; local_18[0] != 0x0;
        local_18[0] = *local_18[0]) {
        if ((*(local_18[0] + 0x23) >> 0x18 == DAT_004c9754) && ((local_18[0] + 0x12) == '\x02')) {
            local_c4 = local_18[0] + 0x8;
            local_c8 = local_c4 & 0xffff0000 | local_c4;
            local_bc[local_c4] = local_bc[local_c4] + 0x1;
            local_c0 = local_c4;
        }
    }
    local_cc = FUN_004840cd(&local_1c,local_18,-0x1);
    loop {
    if (local_18[0] == 0x0) {
        FUN_0048418d(&local_1c);
        return;
    }
    if ((((*(local_18[0] + 0x23) >> 0x18 == DAT_004c9754) && ((local_18[0] + 0x2f) != '\0'))
        && ((local_18[0] + 0x12) == '\0')) &&
        (local_d8 = *(*(&DAT_00582938 +
            (*(local_18[0] + 0x25) >> 0x18) * 0x4 +
            (local_18[0][0x9] >> 0x18) * 0x18) + 0xa5), local_d8 != 0x0)) {
        for (local_dc = 0x0; (local_dc < local_d8 && (local_18[0][local_dc + 0x4] == 0x0)); local_dc = local_dc + 0x1) {
        }
        if (local_dc < local_d8) {
            local_e0 = local_18[0][local_dc + 0x4];
            if ((local_e0 + 0x49) != -0x1) {
                local_e4 = *(local_e0 + 0x46) >> 0x18;
                local_ec = local_18[0] + 0x8;
                local_f0 = local_ec & 0xffff0000 | local_ec;
                local_e8 = local_ec;
                if (local_ec != local_e4) {
                    if ((*(local_18[0] + 0x3a) & 0x1) != 0x0) {
                        DAT_005967bc = FUN_00434de1(local_18[0]);
                        iVar1 = FUN_004579ea(&DAT_005967b8);
                        if (iVar1 == 0x0) {
                            FUN_00418d7a((DAT_005967bc + 0x20));
                            iVar1 = FUN_004579ea(&DAT_005967b8);
                            if (iVar1 == 0x0)^ // goto LAB_00417400;
                        }
                    }
                    (local_18[0] + 0x41) = (local_e0 + 0x49);
                    (local_18[0] + 0x12) = 0xa;^
                    // goto LAB_00417400;
                }
            }
            local_f8 = local_18[0] + 0x8;
            local_fc = *(local_18[0] + 0x3a) & 0x1;
            local_f4 = local_f8;
            if (local_fc == 0x0) {
                local_100 = 0xffffffff;
                local_104 = 0xffffffff;
                FUN_00425e18(local_18[0],&local_100,&local_104);
                if (local_100 == 0xffffffff) {
                    FUN_00425fb6(local_18[0],&local_100,&local_104);
                }
                if (local_100 != 0xffffffff) {
                    FUN_00431d0a(&DAT_005967b8);
                    DAT_005967bc = FUN_00434de1(local_18[0]);
                    iVar1 = FUN_00457f10(&DAT_005967b8,(local_18[0] + 0x8),local_100,local_104,0x1);
                    if ((iVar1 != 0x0) && (DAT_005967bc != 0x0)) {
                        DAT_005967bc = FUN_00434e1a(&DAT_005967b8,(DAT_005967bc + 0x22),
                                                    (DAT_005967bc + 0x24),0x0);
                    }
                }
            }
        }
        else {
            local_174 = -0x1;
            local_178 = 0x0;
            for (local_dc = 0x0; local_dc < 0x28; local_dc = local_dc + 0x1) {
                if ((local_178 < local_bc[local_dc]) && ((local_18[0] + 0x8) != local_174)) {
                    local_174 = local_dc;
                    local_178 = local_bc[local_dc];
                }
            }
            if (local_174 == -0x1) {
                (local_18[0] + 0x12) = 0x4;
            }
            else {
                if ((*(local_18[0] + 0x3a) & 0x1) != 0x0) {
                    DAT_005967bc = FUN_00434de1(local_18[0]);
                    iVar1 = FUN_004579ea(&DAT_005967b8);
                    if (iVar1 == 0x0) {
                        FUN_00418d7a((DAT_005967bc + 0x20));
                        iVar1 = FUN_004579ea(&DAT_005967b8);
                        if (iVar1 == 0x0)^ // goto LAB_00417400;
                    }
                }
                (local_18[0] + 0x41) = local_174;
                (local_18[0] + 0x12) = 0xa;
            }
        }
    }
    LAB_00417400:
        local_18[0] = local_1c;
    local_d0 = &local_1c;
    local_d4 = local_1c;
    if (local_1c != 0x0) {
        local_1c = *local_1c;
    }
} while( true );
}



fn FUN_00417a10()

{
    let piVar1: *mut i32;;
    let mut iVar2: i32;
    let mut local_278: u32;
    let mut local_274: i32;
    let mut local_270: u32;
    let mut local_26c: i32;
    let mut local_268: i32;
    let mut local_264: i32;
    let mut local_260: i32;
    i32 aiStack604 [0x32];
    let mut local_194: u32;
    let local_190: *mut u32;
    let local_18c: *mut u32;
    let mut local_188: u32;
    let local_184: *mut u32;
    let local_180: *mut u32;
    i32 aiStack380 [0x32];
    let mut local_b4: u32;
    let local_b0: *mut u32;
    let local_ac: *mut u32;
    let mut local_a8: i32;
    let mut local_a4: i32;
    let local_a0: *mut u32;
    let local_9c: *mut u32;
    let mut local_98: i32;
    let mut local_94: i32;
    let local_90: *mut u32;
    let local_8c: *mut u32;
    let mut local_88: i32;
    let mut local_84: i32;
    let local_80: *mut u32;
    let local_7c: *mut u32;
    let mut local_78: i32;
    let mut local_74: i32;
    let local_70: *mut u32;
    let local_6c: *mut u32;
    let mut local_68: u32;
    let local_64: *mut u32;
    let local_60: *mut u32;
    let local_5c: *mut u32;
    let mut local_58: i32;
    let mut local_54: i32;
    let mut local_50: u32;
    let local_4c: *mut u32;
    let local_48: *mut u32;
    let mut local_44: u32;
    let local_40: *mut u32;
    let local_3c: *mut u32;
    let mut local_38: u32;
    let local_34: *mut u32;
    let local_30: *mut u32;
    let local_2c: *mut u32;
    u32 **local_28;
    let local_24: *mut u32;
    let mut local_20: i32;
    let local_1c: *mut u32;
    let mut local_18: *mut u32 [0x2];

    local_20 = -0x1;
    local_24 = FUN_004840cd(local_18,&local_1c,-0x1);
    loop {
    if (local_1c == 0x0) {
        FUN_0048418d(local_18);
        return;
    }
    if ((*(local_1c + 0x23) >> 0x18 == DAT_004c9754) &&
        (((((local_1c + 0x2a) == 0x5 || ((local_1c + 0x2a) == 0x7)) ||
            ((local_1c + 0x2a) == 0x9)) &&
            (*(*(&DAT_00582938 +
                (*(local_1c + 0x25) >> 0x18) * 0x4 + (local_1c[0x9] >> 0x18) * 0x18) + 0xa5)
                != 0x0)))) {
        local_34 = local_1c + 0x8;
        local_38 = *(local_1c + 0x3a) & 0x1;
        local_30 = local_34;
        if (local_38 == 0x0) {
            local_40 = local_1c + 0x8;
            local_44 = local_40 & 0xffff0000 | local_40;
            local_3c = local_40;
            if (local_40 != local_20) {
                local_4c = local_1c + 0x8;
                local_50 = local_4c & 0xffff0000 | local_4c;
                local_20 = local_4c;
                local_54 = 0x0;
                local_58 = 0x0;
                local_48 = local_4c;
                for (local_5c = (&DAT_005b8b44 + local_20 * 0x4); local_5c != 0x0;
                    local_5c = *local_5c) {
                    local_64 = local_5c + 0x8;
                    local_68 = local_64 & 0xffff0000 | local_64;
                    local_60 = local_64;
                    if (local_64 != local_20) break;
                    if ((*(local_5c + 0x23) >> 0x18 == DAT_004c9754) &&
                        ((((local_5c + 0x2a) == 0x5 || ((local_5c + 0x2a) == 0x7)) ||
                            ((local_5c + 0x2a) == 0x9)))) {
                        local_70 = local_5c + 0x8;
                        if (((local_5c + 0x35) == -0x1) ||
                            ((local_5c + 0x26) == (local_5c + 0x35))) {
                            local_74 = 0x0;
                        }
                        else {
                            local_74 = 0x1;
                        }
                        local_78 = local_74;
                        local_6c = local_70;
                        if (local_74 != 0x0) {
                            local_80 = local_1c + 0x8;
                            if (((local_1c + 0x35) == -0x1) ||
                                ((local_1c + 0x26) == (local_1c + 0x35))) {
                                local_84 = 0x0;
                            }
                            else {
                                local_84 = 0x1;
                            }
                            local_88 = local_84;
                            local_7c = local_80;
                            if (local_84 == 0x0)^ // goto LAB_00417b4c;
                        }
                        local_90 = local_5c + 0x8;
                        if (((local_5c + 0x35) == -0x1) ||
                            ((local_5c + 0x26) == (local_5c + 0x35))) {
                            local_94 = 0x0;
                        }
                        else {
                            local_94 = 0x1;
                        }
                        local_98 = local_94;
                        local_8c = local_90;
                        if (local_94 == 0x0) {
                            local_a0 = local_1c + 0x8;
                            if (((local_1c + 0x35) == -0x1) ||
                                ((local_1c + 0x26) == (local_1c + 0x35))) {
                                local_a4 = 0x0;
                            }
                            else {
                                local_a4 = 0x1;
                            }
                            local_a8 = local_a4;
                            local_9c = local_a0;
                            if (local_a4 != 0x0)^ // goto LAB_00417b4c;
                        }
                        if (*(*(&DAT_00582938 +
                            (*(local_5c + 0x25) >> 0x18) * 0x4 + (local_5c[0x9] >> 0x18) * 0x18)
                            + 0xa5) != 0x0) {
                            local_b0 = local_5c + 0x8;
                            local_b4 = *(local_5c + 0x3a) & 0x1;
                            local_ac = local_b0;
                            if (local_b4 == 0x0) {
                                if (local_54 < 0x32) {
                                    piVar1 = aiStack380 + local_54;
                                    local_54 = local_54 + 0x1;
                                    *piVar1 = local_5c;
                                }^
                                // goto LAB_00417b4c;
                            }
                        }
                        if ((local_58 < 0x32) && ((local_5c + 0x2a) == 0x7)) {
                            local_184 = local_5c + 0x8;
                            local_188 = *(local_5c + 0x3a) & 0x1;
                            local_180 = local_184;
                            if ((local_188 == 0x0) || (local_5c[0xb] >> 0x18 < 0x2)) {
                                local_190 = local_5c + 0x8;
                                local_194 = *(local_5c + 0x3a) & 0x1;
                                local_18c = local_190;
                                if ((local_194 != 0x0) || ((local_5c + 0x2f) == '\0'))^ // goto LAB_00417b4c;
                            }
                            aiStack604[local_58] = local_5c;
                            local_58 = local_58 + 0x1;
                        }
                    }
                    LAB_00417b4c:
                }
                if (local_54 != 0x0) {
                    if (local_54 * 0x4 < local_58) {
                        local_58 = local_54 << 0x2;
                    }
                    local_260 = 0x0;
                    for (local_264 = 0x0; local_264 < local_58; local_264 = local_264 + 0x1) {
                        local_26c = aiStack604[local_264] + 0x20;
                        local_270 = *(aiStack604[local_264] + 0x3a) & 0x1;
                        local_268 = local_26c;
                        if (local_270 == 0x0) {
                            LAB_00417f13:
                                FUN_00431d31(&local_278);
                            piVar1 = aiStack380 + local_260;
                            local_260 = local_260 + 0x1;
                            local_274 = FUN_00434de1(*piVar1);
                            FUN_00431efd(&local_278,aiStack604[local_264]);
                            (aiStack604[local_264] + 0x48) = 0xe;
                            if (local_260 == local_54) {
                                local_260 = 0x0;
                            }
                        }
                        else {
                            DAT_005967bc = FUN_00434de1(aiStack604[local_264]);
                            iVar2 = FUN_004579ea(&DAT_005967b8);
                            if (iVar2 != 0x0)^ // goto LAB_00417f13;
                        }
                    }
                }
            }
        }
    }
    local_1c = local_18[0];
    local_28 = local_18;
    local_2c = local_18[0];
    if (local_18[0] != 0x0) {
        local_18[0] = *local_18[0];
    }
} while( true );
}



fn FUN_00417fbe(param_1: *mut i32) -> u32

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let local_a4: *mut u32;
    let mut local_a0: i32;
    let local_30: *mut u32;
    let mut local_2c: i32;
    let local_20: *mut i32;;

    iVar1 = (param_1 + 0x8);
    if (((&DAT_004c7108)[iVar1 * 0x4] & 0x20) != 0x0) {
        DAT_005967bc = FUN_00434de1(param_1);
        for (local_2c = 0x0; local_2c < 0x3; local_2c = local_2c + 0x1) {
            local_30 = (&DAT_005b8b44 + iVar1 * 0x4);
            while( true ) {
                if ((local_30 == 0x0) || ((local_30 + 0x8) != iVar1))^ // goto LAB_00418016;
                if (((local_30 + 0x26) != '\r') &&
                    (((((&DAT_004d55a8)[DAT_004c9754 * 0xe + (*(local_30 + 0x23) >> 0x18)] == '\x02' &&
                        ((local_30 + 0x2a) != 0x0)) &&
                        (*(*(&DAT_00582938 +
                            (*(local_30 + 0x25) >> 0x18) * 0x4 + (local_30[0x9] >> 0x18) * 0x18) +
                            0xad) == 0x0)) && (iVar2 = FUN_0041be7a(local_30), iVar2 == 0x0)))) break;
                local_30 = *local_30;
            }
            if ((*(DAT_005967bc + 0x3a) & 0x1) != 0x0) {
                FUN_004579ea(&DAT_005967b8);
            }
            if ((*(DAT_005967bc + 0x3a) & 0x1) != 0x0) {
                return 0x0;
            }
            (DAT_005967bc + 0x41) = (char)*(local_30 + 0x22);
            (DAT_005967bc + 0x42) = (char)*(local_30 + 0x9);
            (DAT_005967bc + 0x12) = 0x11;
            FUN_0041361b(DAT_005967bc);
            if (DAT_005967bc == 0x0) {
                return 0x0;
            }
            LAB_00418016:
        }
    }
    if ((DAT_004c9754 < 0x5) && ((*(&DAT_004c7108 + iVar1 * 0x4) & 0x280) == 0x280)) {
        DAT_005967bc = FUN_00434de1(param_1);
        for (local_a0 = 0x0; local_a0 < 0x3; local_a0 = local_a0 + 0x1) {
            local_a4 = (&DAT_005b8b44 + iVar1 * 0x4);
            while( true ) {
                if ((local_a4 == 0x0) || ((local_a4 + 0x8) != iVar1))^ // goto LAB_0041828b;
                if (((local_a4 + 0x26) == '\r') &&
                    (((local_a4 + 0x2a) != 0x0 &&
                        (*(*(&DAT_00582938 +
                            (*(local_a4 + 0x25) >> 0x18) * 0x4 + (local_a4[0x9] >> 0x18) * 0x18) +
                            0xad) == 0x0)))) break;
                local_a4 = *local_a4;
            }
            if ((*(DAT_005967bc + 0x3a) & 0x1) != 0x0) {
                FUN_004579ea(&DAT_005967b8);
            }
            if ((*(DAT_005967bc + 0x3a) & 0x1) != 0x0) {
                return 0x0;
            }
            (DAT_005967bc + 0x41) = (char)*(local_a4 + 0x22);
            (DAT_005967bc + 0x42) = (char)*(local_a4 + 0x9);
            (DAT_005967bc + 0x12) = 0x11;
            FUN_0041361b(DAT_005967bc);
            if (DAT_005967bc == 0x0) {
                return 0x0;
            }
            LAB_0041828b:
        }
    }
    local_20 = DAT_005967bc;
    while( true ) {
        if (local_20 == 0x0) {
            return 0x0;
        }
        if (local_20 == param_1) break;
        local_20 = local_20[0x2];
    }
    return 0x1;
}



// WARNING: Could not reconcile some variable overlaps

fn FUN_00418529()

{
    let mut iVar1: i32;
    let mut uVar2: u32;
    let mut local_30: i32;
    let mut local_2c: i32;
    i32 **local_1c;
    i32 **local_18 [0x2];

    if (DAT_004c9754 != 0x7) {
        FUN_004840cd(local_18,&local_1c,-0x1);
        while (local_1c != (i32 **)0x0) {
            if ((((*(local_1c + 0x23) >> 0x18 == DAT_004c9754) &&
                (((local_1c + 0x12) == '\0' || ((local_1c + 0x12) == '\f')))) &&
                (((local_1c + 0x2a) == 0x7 || ((local_1c + 0x2a) == 0x9)))) &&
                (((*(*(&DAT_00582938 +
                    (*(local_1c + 0x25) >> 0x18) * 0x4 + (local_1c[0x9] >> 0x18) * 0x18) +
                    0x99) != 0x0 && (iVar1 = FUN_00417fbe(local_1c), iVar1 != 0x0)) &&
                    ((local_1c + 0x2f) != '\0')))) {
                if (((local_1c + 0x12) == '\f') &&
                    ((((&DAT_004c7108)[*(local_1c + 0x41) * 0x4] & 0x20) != 0x0 ||
                        ((DAT_004c9754 < 0x5 &&
                            ((*(&DAT_004c7108 + *(local_1c + 0x41) * 0x4) & 0x280) == 0x280)))))) {
                    FUN_0041361b(local_1c);
                }
                else {
                    local_2c = 0x0;
                    for (local_30 = 0x0; local_30 < 0x28; local_30 = local_30 + 0x1) {
                        if (((&DAT_004c7108)[local_30 * 0x4] & 0x20) == 0x0) {
                            if ((DAT_004c9754 < 0x5) && ((*(&DAT_004c7108 + local_30 * 0x4) & 0x280) == 0x280)) {
                                local_2c = local_2c + 0x1;
                            }
                        }
                        else {
                            local_2c = local_2c + 0x1;
                        }
                    }
                    if (local_2c != 0x0) {
                        uVar2 = FUN_004a2edc();
                        local_2c = uVar2 % local_2c + 0x1;
                        for (local_30 = 0x0; local_30 < 0x28; local_30 = local_30 + 0x1) {
                            if (((&DAT_004c7108)[local_30 * 0x4] & 0x20) == 0x0) {
                                if ((DAT_004c9754 < 0x5) && ((*(&DAT_004c7108 + local_30 * 0x4) & 0x280) == 0x280)) {
                                    local_2c = local_2c + -0x1;
                                }
                            }
                            else {
                                local_2c = local_2c + -0x1;
                            }
                            if (local_2c == 0x0) break;
                        }
                        DAT_005967bc = FUN_00434de1(local_1c);
                        if ((*(DAT_005967bc + 0x3a) & 0x1) != 0x0) {
                            FUN_004579ea(&DAT_005967b8);
                        }
                        if ((*(DAT_005967bc + 0x3a) & 0x1) == 0x0) {
                            (local_1c + 0x41) = local_30;
                            (local_1c + 0x12) = 0xc;
                            FUN_0041361b(local_1c);
                        }
                    }
                }
            }
            local_1c = local_18[0];
            if (local_18[0] != (i32 **)0x0) {
                local_18[0] = (i32 **)*local_18[0];
            }
        }
        FUN_0048418d(local_18);
    }
    return;
}



// WARNING: Could not reconcile some variable overlaps

i32 ** FUN_00418819()

{
i32 **ppiVar1;
let mut uVar2: u32;
let mut local_8f4: i32;
i32 local_8d8 [0x230];
i32 **local_18;
i32 **local_14;

FUN_004a0430(local_8d8,0x0,0x8c0);
for (local_18 = (i32 **)*DAT_005967b0; local_18 != (i32 **)0x0; local_18 = (i32 **)*local_18) {
if ((((((local_18 + 0x2a) == 0x5) || ((local_18 + 0x2a) == 0x7)) ||
((local_18 + 0x2a) == 0x9)) &&
(((*(local_18 + 0x3a) & 0x1) == 0x0 ||
((*(local_18 + 0x23) >> 0x18 == DAT_004c9754 && (0x1 < local_18[0xb] >> 0x18)))))) &&
((*(local_18 + 0x23) >> 0x18 != DAT_004c9754 ||
((*(*(&DAT_00582938 +
(*(local_18 + 0x25) >> 0x18) * 0x4 + (local_18[0x9] >> 0x18) * 0x18) + 0xa5)
== 0x0 && ((local_18 + 0x2f) != '\0')))))) {
local_8d8[(local_18 + 0x8) * 0xe + (*(local_18 + 0x23) >> 0x18)] =
local_8d8[(local_18 + 0x8) * 0xe + (*(local_18 + 0x23) >> 0x18)] +
*(*(&DAT_00582938 +
(*(local_18 + 0x25) >> 0x18) * 0x4 + (local_18[0x9] >> 0x18) * 0x18) +
0x125) + 0x1;
}
}
local_14 = (i32 **)0x0;
ppiVar1 = local_18;
loop {
if (0x27 < local_14) {
return ppiVar1;
}
if (local_8d8[local_14 * 0xe + DAT_004c9754] != 0x0) {
for (local_8f4 = 0x0; local_8f4 < 0xe; local_8f4 = local_8f4 + 0x1) {
if (((local_8d8[local_14 * 0xe + local_8f4] != 0x0) &&
((&DAT_004d55a8)[DAT_004c9754 * 0xe + local_8f4] == '\x02')) &&
((local_8d8[local_8f4 + local_14 * 0xe] <= (local_8d8[DAT_004c9754 + local_14 * 0xe] * 0x3) / 0x2
|| ((uVar2 = FUN_004a2edc(), uVar2 % 0x2 == 0x0 &&
(local_8d8[local_14 * 0xe + local_8f4] <= local_8d8[DAT_004c9754 + local_14 * 0xe])))))) {
for (local_18 = *(i32 ***)(&DAT_005b8b44 + local_14 * 0x4);
(local_18 != (i32 **)0x0 && ((i32 **)(local_18 + 0x8) == local_14));
local_18 = (i32 **)*local_18) {
if (((((local_18 + 0x2a) == 0x5) ||
(((local_18 + 0x2a) == 0x7 || ((local_18 + 0x2a) == 0x9)))) &&
((*(local_18 + 0x3a) & 0x1) != 0x0)) &&
((((*(local_18 + 0x23) >> 0x18 == DAT_004c9754 && (0x1 < local_18[0xb] >> 0x18)) &&
(*(*(&DAT_00582938 +
(*(local_18 + 0x25) >> 0x18) * 0x4 + (local_18[0x9] >> 0x18) * 0x18)
+ 0xa5) == 0x0)) &&
(*(*(&DAT_00582938 +
(*(local_18 + 0x25) >> 0x18) * 0x4 + (local_18[0x9] >> 0x18) * 0x18)
+ 0x125) != 0x0)))) {
DAT_005967bc = FUN_00434de1(local_18);
FUN_004579ea(&DAT_005967b8);
}
}
local_18 = *(i32 ***)(&DAT_005b8b44 + local_14 * 0x4);
while( true ) {
if ((local_18 == (i32 **)0x0) || ((i32 **)(local_18 + 0x8) != local_14))^ // goto LAB_004189d1;
if (((((local_18 + 0x2a) == 0x5) ||
(((local_18 + 0x2a) == 0x7 || ((local_18 + 0x2a) == 0x9)))) &&
(((*(local_18 + 0x3a) & 0x1) == 0x0 &&
(((*(local_18 + 0x23) >> 0x18 == DAT_004c9754 && ((local_18 + 0x2f) != '\0'))
&& (*(*(&DAT_00582938 +
(*(local_18 + 0x25) >> 0x18) * 0x4 +
(local_18[0x9] >> 0x18) * 0x18) + 0xa5) == 0x0)))))) &&
(*(*(&DAT_00582938 +
(*(local_18 + 0x25) >> 0x18) * 0x4 + (local_18[0x9] >> 0x18) * 0x18) +
0x125) != 0x0)) break;
local_18 = (i32 **)*local_18;
}
(local_18 + 0x12) = 0xb;
(local_18 + 0x49) = local_8f4;
FUN_0041361b(local_18);
}
// LAB_004189d1:
}
}
ppiVar1 = local_14;
local_14 = (i32 **)(local_14 + 0x1);
} while( true );
}



fn FUN_00418d7a(param_1: i32)

{
    let mut local_44: u32;
    let mut local_40: u32;
    let mut local_3c: u32;
    let local_38: *mut u32;
    let local_34: *mut u32;
    let mut local_30: u32;
    let local_2c: *mut u32;
    let local_28: *mut u32;
    let local_24: *mut u32;
    let local_20: *mut u32;
    let mut local_1c: u32;
    let local_18: *mut u32;
    let mut local_14: i32;

    local_14 = DAT_005967bc;
    local_18 = (&DAT_005b8b44 + param_1 * 0x4);
    while( true ) {
        if (local_18 == 0x0) {
            DAT_005967bc = local_14;
            return;
        }
        local_24 = local_18 + 0x8;
        local_1c = local_24 & 0xffff0000 | local_24;
        if (local_24 != param_1) break;
        local_2c = local_18 + 0x8;
        local_30 = *(local_18 + 0x3a) & 0x1;
        if ((((local_30 == 0x0) && (*(local_18 + 0x23) >> 0x18 == DAT_004c9754)) &&
            (((local_18 + 0x2a) == 0x5 ||
                (((local_18 + 0x2a) == 0x7 || ((local_18 + 0x2a) == 0x9)))))) &&
            ((local_18 + 0x2f) != '\0')) {
            local_38 = local_18 + 0x8;
            local_3c = *(local_18 + 0x3a) & 0x40;
            if (local_3c == 0x0) {
                local_34 = local_38;
                local_28 = local_2c;
                local_20 = local_24;
                DAT_005967bc = FUN_00434de1(local_18);
                FUN_00425e18(DAT_005967bc,&local_44,&local_40);
                if (local_44 == 0xffffffff) {
                    FUN_00425fb6(DAT_005967bc,&local_44,&local_40);
                }
                if (local_44 != 0xffffffff) {
                    FUN_00457f10(&DAT_005967b8,(DAT_005967bc + 0x20),local_44,local_40,0x1);
                }
            }
        }
        local_18 = *local_18;
    }
    DAT_005967bc = local_14;
    return;
}



fn FUN_00418efd(param_1: i32,param_2: i32) -> *mut u32

{
    let mut bVar1: bool;
    let mut uVar2: u32;
    let local_44: *mut u32;
    let local_40: *mut u32;
    let mut local_3c: u32;

    local_40 = 0x0;
    local_3c = 0xf423f;
    if (((param_1 + 0x35) == -0x1) || ((param_1 + 0x26) == (param_1 + 0x35))) {
        bVar1 = false;
    }
    else {
        bVar1 = true;
    }
    local_44 = (&DAT_005b89f8 + (param_1 + 0x20) * 0x4);
    loop {
    if ((local_44 == 0x0) || (*(local_44 + 0x6) >> 0x10 != (param_1 + 0x20))) {
        return local_40;
    }
    if ((*(local_44 + 0xe) >> 0x10 == DAT_004c9754) &&
        (local_44[0xc] >> 0x10 < *(&DAT_004bd79c + param_2 * 0x4 + (local_44[0x3] >> 0x10) * 0x3c))) {
        if ((*(local_44 + 0x2d) & 0x1) == 0x0) {
            if (!bVar1)^ // goto LAB_0041901d;
        }
        else {
            if ((bVar1) && (*(param_1 + 0x32) >> 0x18 == local_44[0x4] >> 0x10)) {
                LAB_0041901d:
                    uVar2 = *(*((local_44[0x2] >> 0x10) * 0x4 + DAT_005b2d50) +
                    (*(local_44 + 0xa) >> 0x10));
                if (((uVar2 != 0xff) && (uVar2 != 0x0)) && ((local_40 == 0x0 || (uVar2 < local_3c)))) {
                    local_40 = local_44;
                    local_3c = uVar2;
                }
            }
        }
    }
    local_44 = *local_44;
} while( true );
}



fn FUN_00419085()

{
    return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00419099()

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut uVar5: u32;
    short *psVar6;
    let mut uVar7: u32;
    let acStackY32844: u8 [0x7e20];
    let local_1fc: *mut u32;
    let mut local_1dc: u32;
    let mut local_1b4: u32;
    let mut local_1b0: u32;
    let mut local_1a8: u32;
    let mut local_1a4: u32;
    let mut local_1a0: u32;
    let mut local_19c: u32;
    let mut local_198: i32;
    let local_180: *mut u32;
    let local_f8: *mut u32;
    let mut local_f0: i32;
    let mut local_94: i32;
    let local_90: *mut i32;;
    let local_4c: u8 [0x28];
    let mut local_24: String;;
    let mut local_20: u32;
    i32 **local_1c;
    let local_18: *mut u32;
    let mut local_14: i32;

    FUN_004a0430(local_4c,0x0,0x28);
    _DAT_005b2d64 = 0x1;
    for (local_1c = (i32 **)*DAT_005967b0; local_1c != (i32 **)0x0; local_1c = (i32 **)*local_1c) {
        if (((&DAT_004d55a8)[DAT_004c9754 * 0xe + (*(local_1c + 0x23) >> 0x18)] == '\x02') &&
            ((*(local_1c + 0x3a) & 0x1) != 0x0)) {
            local_4c[(local_1c + 0x8)] = '\x01';
        }
    }
    for (local_18 = *DAT_005967c8; local_18 != 0x0; local_18 = *local_18) {
        if (((&DAT_004d55a8)[DAT_004c9754 * 0xe + (*(local_18 + 0xe) >> 0x10)] == '\x02') &&
            (*(&DAT_004be8b0 + (local_18[0x3] >> 0x10) * 0x4) == 0x0)) {
            local_4c[*(local_18 + 0x6) >> 0x10] = '\x01';
        }
    }
    FUN_004133bd(&local_20,&local_24);
    DAT_00599d3c = 0x0;
    FUN_004864f7();
    DAT_00599d3c = 0x1;
    FUN_0041a516(0x3);
    local_14 = 0x0;
    loop {
    if (0x27 < local_14) {
        FUN_0049af50(local_24);
        FUN_0049af50((local_20 - 0x4));
        _DAT_005b2d64 = 0x0;
        return;
    }
    if (local_4c[local_14] == '\0') {
        while (local_1c = (i32 **)FUN_0041a2e3(local_14), local_1c != (i32 **)0x0) {
            FUN_00413546(&DAT_005967b8);
            if ((DAT_004c9754 != 0x5) && (DAT_004c9754 != 0x6)) {
                FUN_00413584(&DAT_005967b8);
            }
        }
    }
    else {
        local_90 = 0x0;
        local_94 = 0x0;
        LAB_004192a2:
            local_1c = (i32 **)FUN_0041a2e3(local_14);
        if (local_1c != (i32 **)0x0) {
            FUN_00431d0a(&DAT_005967b8);
            DAT_005967bc = local_1c;
            FUN_00432c94(&DAT_005967b8);
            iVar1 = FUN_00432bd3(&DAT_005967b8);
            if ((local_1c != (i32 **)local_90) || (iVar1 != local_94)) {
                local_90 = local_1c;
                iVar2 = FUN_0041ac19(local_1c);
                FUN_0045af67(&DAT_005967b8,(local_1c + 0x22),(local_1c + 0x9),
                             (local_1c + 0x22),(local_1c + 0x9),0x1869f);
                iVar3 = (local_1c + 0x8);
                local_f0 = FUN_0041a46a(&DAT_005967b8,0x0);
                iVar4 = FUN_0041a46a(&DAT_005967b8,iVar2);
                FUN_004a0430(local_24,0x0,0x1842);
                for (local_f8 = (&DAT_005b8b44 + iVar3 * 0x4);
                    (local_f8 != 0x0 && ((local_f8 + 0x8) == iVar3));
                    local_f8 = *local_f8) {
                    if (((&DAT_004d55a8)[DAT_004c9754 * 0xe + (*(local_f8 + 0x23) >> 0x18)] == '\x02') &&
                        ((*(local_f8 + 0x3a) & 0x1) != 0x0)) {
                        if ((*(*(&DAT_00582938 +
                            (*(local_f8 + 0x25) >> 0x18) * 0x4 + (local_f8[0x9] >> 0x18) * 0x18
                        ) + 0x121) == 0x0) &&
                            ((*(local_20 + (local_f8 + 0x22) * 0x4) +
                                (local_f8 + 0x9) * 0x2) == 0x0)) {
                            psVar6 = (short *)(*(local_20 + (local_f8 + 0x22) * 0x4) +
                                (local_f8 + 0x9) * 0x2);
                            *psVar6 = *psVar6 + 0x1;
                        }
                        else {
                            psVar6 = (short *)(*(local_20 + (local_f8 + 0x22) * 0x4) +
                                (local_f8 + 0x9) * 0x2);
                            *psVar6 = *psVar6 + *(*(&DAT_00582938 +
                                (*(local_f8 + 0x25) >> 0x18) * 0x4 +
                                (local_f8[0x9] >> 0x18) * 0x18) + 0x121);
                        }
                    }
                }
                for (local_180 = (&DAT_005b89f8 + iVar3 * 0x4);
                    (local_180 != 0x0 && (*(local_180 + 0x6) >> 0x10 == iVar3));
                    local_180 = *local_180) {
                    if (((&DAT_004d55a8)[DAT_004c9754 * 0xe + (*(local_180 + 0xe) >> 0x10)] == '\x02') &&
                        (*(&DAT_004be8b0 + (local_180[0x3] >> 0x10) * 0x4) == 0x0)) {
                        if (DAT_004c9754 < 0x5) {
                            if ((((local_180 + 0xe) == 0xf) || ((local_180 + 0xe) == 0x10)) &&
                                ((*(local_180 + 0xb) & 0x4) == 0x0)) {
                                if ((local_180 + 0xe) == 0x10) {
                                    local_198 = 0x320;
                                }
                                else {
                                    local_198 = 0x190;
                                }
                                if (local_198 <= local_f0) {
                                    psVar6 = (short *)(*((local_180[0x2] >> 0x10) * 0x4 + local_20) +
                                        (*(local_180 + 0xa) >> 0x10) * 0x2);
                                    *psVar6 = *psVar6 + local_198;
                                }
                            }
                            else {
                                LAB_00419922:
                                    psVar6 = (short *)(*((local_180[0x2] >> 0x10) * 0x4 + local_20) +
                                    (*(local_180 + 0xa) >> 0x10) * 0x2);
                                *psVar6 = *psVar6 + 0x1;
                            }
                        }
                        else {
                            if ((local_180 + 0xe) != 0x2)^ // goto LAB_00419922;
                        }
                    }
                }
                local_19c = 0x1869f;
                for (local_1a0 = 0x0; local_1a0 < 0x2c; local_1a0 = local_1a0 + 0x1) {
                    local_1a4 = ((local_1a0 & 0x1) == 0x0);
                    for (local_1a8 = local_1a4; local_1a8 < (0x41 - (local_1a0 & 0x1)); local_1a8 = local_1a8 + 0x2) {
                        uVar5 = SEXT24((*(local_1a0 * 0x4 + local_20) + local_1a8 * 0x2));
                        if (uVar5 != 0x0) {
                            if (iVar1 + 0x8 < *(*(local_1a0 * 0x4 + DAT_005b2d50) + local_1a8)) {
                                if ((0x0 < local_19c) && (iVar2 != 0x0)) {
                                    if ((uVar5 * 0x2) < iVar4) {
                                        if (local_19c < uVar5) {
                                            local_1b0 = local_1a0;
                                            local_1b4 = local_1a8;
                                            local_19c = uVar5;
                                        }
                                    }
                                    else {
                                        if (uVar5 < local_19c) {
                                            local_1b0 = local_1a0;
                                            local_1b4 = local_1a8;
                                            local_19c = uVar5;
                                        }
                                    }
                                }
                            }
                            else {
                                uVar7 = local_19c >> 0x1f;
                                if ((uVar5 * 0x2) < local_f0) {
                                    if (((local_19c ^ uVar7) - uVar7) < uVar5) {
                                        local_19c = -uVar5;
                                        local_1b0 = local_1a0;
                                        local_1b4 = local_1a8;
                                    }
                                }
                                else {
                                    if (uVar5 < ((local_19c ^ uVar7) - uVar7)) {
                                        local_19c = -uVar5;
                                        local_1b0 = local_1a0;
                                        local_1b4 = local_1a8;
                                    }
                                }
                            }
                        }
                        if ((((*(*(local_1a0 * 0x4 + DAT_005b2d50) + local_1a8) <= iVar1 + 0x8) ||
                            ((0x0 < local_19c && (iVar2 != 0x0)))) &&
                            ((*(local_1a0 * 0x4 + local_20) + local_1a8 * 0x2) != 0x0)) &&
                            ((*(local_1a0 * 0x4 + local_20) + local_1a8 * 0x2) <
                                ((local_19c ^ local_19c >> 0x1f) - (local_19c >> 0x1f)))) {
                            if (iVar1 + 0x8 < *(*(local_1a0 * 0x4 + DAT_005b2d50) + local_1a8)) {
                                local_19c = SEXT24((*(local_1a0 * 0x4 + local_20) + local_1a8 * 0x2));
                            }
                            else {
                                local_19c = -(*(local_1a0 * 0x4 + local_20) + local_1a8 * 0x2);
                            }
                            local_1b0 = local_1a0;
                            local_1b4 = local_1a8;
                        }
                    }
                }
                local_94 = iVar1;
                if (local_19c == 0x1869f) {
                    local_19c = 0x1869f;
                    for (local_1a0 = 0x0; local_1a0 < 0x2c; local_1a0 = local_1a0 + 0x1) {
                        local_1dc = ((local_1a0 & 0x1) == 0x0);
                        for (local_1a8 = local_1dc; local_1a8 < (0x41 - (local_1a0 & 0x1)); local_1a8 = local_1a8 + 0x2)
                        {
                            if ((*(*(local_1a0 * 0x4 + DAT_005b2d50) + local_1a8) < local_19c) &&
                                ((*(local_1a0 * 0x4 + local_20) + local_1a8 * 0x2) != 0x0)) {
                                local_19c = *(*(DAT_005b2d50 + local_1a0 * 0x4) + local_1a8);
                                local_1b0 = local_1a0;
                                local_1b4 = local_1a8;
                            }
                        }
                    }
                    if (local_19c == 0x1869f) {
                        if ((DAT_004c9754 != 0x5) && (DAT_004c9754 != 0x6)) {
                            FUN_00413546(&DAT_005967b8);
                            FUN_00413584(&DAT_005967b8);
                        }
                    }
                    else {
                        iVar1 = FUN_0041a46a(&DAT_005967b8,0x0);
                        if (iVar1 < local_19c) {
                            local_19c = 0x3e7;
                            for (local_180 = (&DAT_005b89f8 + (local_1c + 0x8) * 0x4);
                                (local_180 != 0x0 && ((local_1c + 0x8) == (local_180 + 0x2)));
                                local_180 = *local_180) {
                                if ((*(local_180 + 0xe) >> 0x10 == DAT_004c9754) &&
                                    ((iVar1 = FUN_0044a87f(local_180[0x2] >> 0x10,*(local_180 + 0xa) >> 0x10,local_1b0
                                                           ,local_1b4), iVar1 < local_19c &&
                                          (((*(local_180 + 0xa) >> 0x10) +
                                              *((local_180[0x2] >> 0x10) * 0x4 + DAT_005b2d50)) != -0x1)))) {
                                    local_19c = *(*((local_180[0x2] >> 0x10) * 0x4 + DAT_005b2d50) +
                                        (*(local_180 + 0xa) >> 0x10));
                                    local_1fc = local_180;
                                }
                            }
                            if (local_19c == 0x3e7) {
                                if ((DAT_004c9754 == 0x5) || (DAT_004c9754 == 0x6)) {
                                    FUN_00413500(&DAT_005967b8,'\t');
                                }
                                else {
                                    FUN_00413546(&DAT_005967b8);
                                    FUN_00413584(&DAT_005967b8);
                                }
                            }
                            else {
                                (local_1c + 0x12) = 0x6;
                                (local_1c + 0x41) = (local_1fc + 0xa);
                                (local_1c + 0x42) = (local_1fc + 0x3);
                                DAT_005967bc = local_1c;
                                FUN_0041361b(local_1c);
                                if (DAT_005967bc != 0x0) {
                                    FUN_00413546(&DAT_005967b8);
                                }
                            }
                        }
                        else {
                            (local_1c + 0x12) = 0x6;
                            (local_1c + 0x41) = local_1b0;
                            (local_1c + 0x42) = local_1b4;
                            DAT_005967bc = local_1c;
                            FUN_0041361b(local_1c);
                            if (DAT_005967bc != 0x0) {
                                FUN_00413546(&DAT_005967b8);
                            }
                        }
                    }^
                    // goto LAB_004192a2;
                }
                if (0x0 < local_19c) {
                    local_f0 = iVar4;
                }
                FUN_0041a700((local_1c + 0x8),local_1b0,local_1b4);
                iVar1 = (local_19c ^ local_19c >> 0x1f) - (local_19c >> 0x1f);
                if (0x0 < local_19c) {
                    iVar2 = FUN_0041b05a(local_1c,local_1b0,local_1b4);
                    if (iVar2 != 0x0) {
                        if (((DAT_005967bc != 0x0) && (FUN_00413546(&DAT_005967b8), DAT_004c9754 != 0x5)) &&
                            (DAT_004c9754 != 0x6)) {
                            FUN_00413584(&DAT_005967b8);
                        }^
                        // goto LAB_004192a2;
                    }
                    local_1c = (i32 **)DAT_005967bc;
                    local_f0 = FUN_0041a46a(&DAT_005967b8,0x0);
                }
                iVar2 = FUN_0041a3e0(local_f0,iVar1);
                if (iVar2 == 0x0) {
                    FUN_0041bbec(local_1c);
                    local_f0 = FUN_0041a46a(&DAT_005967b8,0x0);
                }
                iVar1 = FUN_0041a3e0(local_f0,iVar1);
                if (iVar1 == 0x0) {
                    FUN_00413500(&DAT_005967b8,'\t');
                }
                else {
                    _DAT_005b2d64 = 0x0;
                    (local_1c + 0x12) = 0x7;
                    (local_1c + 0x41) = local_1b0;
                    (local_1c + 0x42) = local_1b4;
                    FUN_0041361b(local_1c);
                    _DAT_005b2d64 = 0x1;
                }^
                // goto LAB_004192a2;
            }
            FUN_00413546(&DAT_005967b8);
            if ((DAT_004c9754 != 0x5) && (DAT_004c9754 != 0x6)) {
                FUN_00413584(&DAT_005967b8);
            }^
            // goto LAB_004192a2;
        }
    }
    local_14 = local_14 + 0x1;
} while( true );
}



fn FUN_0041a2e3(param_1: i32) -> *mut u32

{
    let mut iVar1: i32;
    let local_1c: *mut u32;
    let local_18: *mut u32;
    let mut local_14: i32;

    local_18 = 0x0;
    local_14 = 0x0;
    for (local_1c = (&DAT_005b8b44 + param_1 * 0x4);
        (local_1c != 0x0 && ((local_1c + 0x8) == param_1)); local_1c = *local_1c) {
        if (((local_1c + 0x12) == '\x03') &&
            (((*(local_1c + 0x23) >> 0x18 == DAT_004c9754 && ((*(local_1c + 0x3a) & 0x1) != 0x0)) &&
                (local_1c[0x3] == 0x0)))) {
            FUN_00431d0a(&DAT_005967b8);
            DAT_005967bc = local_1c;
            iVar1 = FUN_0041a46a(&DAT_005967b8,0x0);
            if (local_14 < iVar1) {
                local_18 = local_1c;
                local_14 = iVar1;
            }
        }
    }
    return local_18;
}



fn FUN_0041a3e0(param_1: i32,param_2: i32) -> u32

{
    let mut uVar1: u32;
    let mut local_14: u32;

    if (((param_1 < (param_2 * 0x3) / 0x2) && ((param_1 < param_2 || (uVar1 = FUN_004a2edc(), uVar1 % 0x4 == 0x0))))
        && ((param_1 < param_2 / 0x2 || (uVar1 = FUN_004a2edc(), ((longlong)uVar1 % 0x4 & 0x1U) == 0x0)))) {
local_14 = 0x0;
}
    else {
local_14 = 0x1;
}
    return local_14;
}



fn FUN_0041a46a(param_1: i32,param_2: i32) -> i32

{
let mut local_18: i32;
let mut local_14: i32;

local_18 = 0x0;
local_14 = *(param_1 + 0x4);
while ((local_14 != 0x0 &&
((((param_2 != 0x0 &&
(*(*(&DAT_00582938 +
(*(local_14 + 0x25) >> 0x18) * 0x4 + (*(local_14 + 0x24) >> 0x18) * 0x18) +
0xa9) == 0x0)) ||
(local_18 = local_18 +
*(*(&DAT_00582938 +
(*(local_14 + 0x25) >> 0x18) * 0x4 +
(*(local_14 + 0x24) >> 0x18) * 0x18) + 0x121), param_2 < 0x0)) ||
(param_2 = param_2 + -0x1, param_2 != 0x0))))) {
local_14 = *(local_14 + 0x8);
}
return local_18;
}



fn FUN_0041a516(param_1: i32)

{
    let local_24: *mut u32;
    let local_14: *mut u32;

    for (local_14 = *DAT_005967b0; local_14 != 0x0; local_14 = *local_14) {
        if ((((*(local_14 + 0x23) >> 0x18 == DAT_004c9754) && ((*(local_14 + 0x3a) & 0x1) != 0x0))
            && (*(local_14 + 0x45) >> 0x18 == param_1)) && ((local_14[0x3] == 0x0 && (local_14[0x2] == 0x0)))) {
            for (local_24 = *local_14;
                ((local_24 != 0x0 && ((local_14 + 0x22) == (local_24 + 0x22))) &&
                (((local_14 + 0x9) == (local_24 + 0x9) &&
                (((*(local_24 + 0x23) >> 0x18 == DAT_004c9754 && ((*(local_24 + 0x3a) & 0x1) != 0x0)
                ) && ((local_14 + 0x8) == (local_24 + 0x8))))))); local_24 = *local_24) {
                if (((*(local_24 + 0x45) >> 0x18 == param_1) && (local_24[0x3] == 0x0)) && (local_24[0x2] == 0x0)) {
                    local_14[0x2] = local_24;
                    local_24[0x3] = local_14;
                    local_14 = local_24;
                }
            }
        }
    }
    return;
}



fn FUN_0041a700(param_1: i32,param_2: i32,param_3: i32)

{
    let mut iVar1: i32;
    i32 **local_1c;
    i32 **local_18 [0x2];

    if (DAT_004c9754 != 0x7) {
        FUN_004840cd(local_18,&local_1c,-0x1);
        while (local_1c != (i32 **)0x0) {
            if ((((((local_1c + 0x8) == param_1) && (*(local_1c + 0x23) >> 0x18 == DAT_004c9754)) &&
                (((local_1c + 0x12) == '\0' || ((local_1c + 0x12) == '\f')))) &&
                (((((local_1c + 0x2a) == 0x5 || ((local_1c + 0x2a) == 0x7)) ||
                    ((local_1c + 0x2a) == 0x9)) &&
                    (*(*(&DAT_00582938 +
                        (*(local_1c + 0x25) >> 0x18) * 0x4 + (local_1c[0x9] >> 0x18) * 0x18) +
                        0x99) != 0x0)))) &&
                ((((local_1c + 0x12) != '\f' || (0x1 < local_1c[0xb] >> 0x18)) &&
                    (iVar1 = FUN_0041a862(local_1c,param_2,param_3), iVar1 != 0x0)))) {
                FUN_0048418d(local_18);
                return;
            }
            local_1c = local_18[0];
            if (local_18[0] != (i32 **)0x0) {
                local_18[0] = (i32 **)*local_18[0];
            }
        }
        FUN_0048418d(local_18);
    }
    return;
}



fn FUN_0041a862(param_1: *mut i32,param_2: i32,param_3: i32) -> u32

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let local_b4: *mut i32;;
    let local_2c: *mut u32;

    iVar2 = (param_1 + 0x8);
    DAT_005967bc = FUN_00434de1(param_1);
    while( true ) {
        if ((param_1 + 0x2f) == '\0') {
            return 0x0;
        }
        iVar1 = param_1[0xb];
        if (((param_1 + 0x12) == '\f') && (param_1[0xb] >> 0x18 < 0x2)) {
            return 0x0;
        }
        local_2c = (&DAT_005b8b44 + iVar2 * 0x4);
        while (((local_2c != 0x0 && ((local_2c + 0x8) == iVar2)) &&
            (((local_2c + 0x22) != param_2 ||
                (((((local_2c + 0x9) != param_3 ||
                    ((&DAT_004d55a8)[DAT_004c9754 * 0xe + (*(local_2c + 0x23) >> 0x18)] != '\x02')) ||
                    ((*(local_2c + 0x3a) & 0x1) == 0x0)) ||
                    (((local_2c + 0x2a) == 0x0 ||
                        (*(*(&DAT_00582938 +
                            (*(local_2c + 0x25) >> 0x18) * 0x4 + (local_2c[0x9] >> 0x18) * 0x18) +
                            0xad) != 0x0))))))))) {
            local_2c = *local_2c;
        }
        if ((local_2c == 0x0) || ((local_2c + 0x8) != iVar2)) {
            return 0x1;
        }
        iVar3 = FUN_0041be7a(local_2c);
        if (iVar3 != 0x0) {
            return 0x1;
        }
        if ((*(DAT_005967bc + 0x3a) & 0x1) != 0x0) {
            FUN_004579ea(&DAT_005967b8);
            if ((*(DAT_005967bc + 0x3a) & 0x1) != 0x0) {
                return 0x0;
            }
            if (((param_1 + 0x12) == '\f') && (param_1[0xb] >> 0x18 < 0x2)) {
                return 0x0;
            }
        }
        (DAT_005967bc + 0x41) = param_2;
        (DAT_005967bc + 0x42) = param_3;
        (DAT_005967bc + 0x12) = 0x11;
        FUN_0041361b(DAT_005967bc);
        if (DAT_005967bc == 0x0) {
            return 0x1;
        }
        for (local_b4 = DAT_005967bc; (local_b4 != 0x0 && (local_b4 != param_1)); local_b4 = local_b4[0x2]) {
        }
        if (local_b4 == 0x0) break;
        if (param_1[0xb] >> 0x18 == iVar1 >> 0x18) {
            return 0x1;
        }
    }
    return 0x1;
}



fn FUN_0041ac19(param_1: i32) -> i32

{
let mut iVar1: i32;
let mut bVar2: bool;
let mut bVar3: bool;
let mut local_70: i32;
let local_3c: *mut u32;
let mut local_38: i32;

local_38 = 0x0;
if (((param_1 + 0x35) == -0x1) || ((param_1 + 0x26) == (param_1 + 0x35))) {
bVar3 = false;
}
else {
bVar3 = true;
}
local_3c = (&DAT_005b8b44 + (param_1 + 0x20) * 0x4);
loop {
if ((local_3c == 0x0) || ((local_3c + 0x8) != (param_1 + 0x20))) {
return local_38;
}
if ((((*(local_3c + 0x3a) & 0x1) == 0x0) && (*(local_3c + 0x23) >> 0x18 == DAT_004c9754))
&& (((local_3c + 0x2a) == 0x9 || ((local_3c + 0x2a) == 0x7)))) {
if (((local_3c + 0x35) == -0x1) ||
((local_3c + 0x26) == (local_3c + 0x35))) {
bVar2 = false;
}
else {
bVar2 = true;
}
if (bVar2) {
if ((bVar3) && ((param_1 + 0x35) == (local_3c + 0x35))) {
// LAB_0041ad97:
iVar1 = *(*(&DAT_00582938 +
(*(local_3c + 0x25) >> 0x18) * 0x4 + (local_3c[0x9] >> 0x18) * 0x18)
+ 0xa5);
if ((iVar1 != 0x0) && ((local_3c + 0x2f) != '\0')) {
for (local_70 = 0x0; (local_70 < iVar1 && (local_3c[local_70 + 0x4] == 0x0)); local_70 = local_70 + 0x1) {
}
if (iVar1 <= local_70) {
local_38 = local_38 + iVar1;
}
}
}
}
else {
if (!bVar3)^ // goto LAB_0041ad97;
}
}
local_3c = *local_3c;
} while( true );
}



fn FUN_0041ae21(param_1: i32,param_2: *mut i32,param_3: i32,param_4: i32,param_5: *mut u32,param_6: *mut u32) -> i32

{
let mut iVar1: i32;
let mut iVar2: i32;
let mut iVar3: i32;
let mut uVar4: u32;
let mut local_14: i32;

local_14 = 0x0;
DAT_005967bc = *(param_1 + 0x4);
*param_6 = 0x0;
iVar2 = FUN_004579ea(&DAT_005967b8);
if (iVar2 == 0x0) {
// LAB_0041b034:
*param_5 = 0x0;
*(param_1 + 0x4) = DAT_005967bc;
return local_14;
}
// LAB_0041ae5e:
if ((*param_2 == 0x0) || (iVar2 = FUN_00432c94(&DAT_005967b8), 0x14 < iVar2))^ // goto LAB_0041af1b;
iVar2 = *param_2;
iVar1 = *(*param_2 + 0x8);
if (*(*(&DAT_00582938 + (*(iVar2 + 0x25) >> 0x18) * 0x4 + (*(iVar2 + 0x24) >> 0x18) * 0x18
) + 0xa9) != 0x0) {
*(iVar2 + 0x3a) = *(iVar2 + 0x3a) & 0xfe;
iVar3 = FUN_004330dc(&DAT_005967b8,iVar2);
if (iVar3 == 0x0) {
*(iVar2 + 0x3a) = *(iVar2 + 0x3a) | 0x1;
// LAB_0041af1b:
iVar2 = FUN_00457f10(&DAT_005967b8,(DAT_005967bc + 0x20),param_3,param_4,0x1);
if ((iVar2 == 0x0) || (DAT_005967bc == 0x0)) {
*(param_1 + 0x4) = DAT_005967bc;
*param_5 = 0x2;
return local_14;
}
uVar4 = FUN_00434e1a(&DAT_005967b8,(DAT_005967bc + 0x22),(DAT_005967bc + 0x24),
0x1);
*param_6 = uVar4;^
// goto LAB_0041b034;
}
(iVar2 + 0x48) = 0x4;
(iVar2 + 0x49) = (char)*(iVar2 + 0x20);
local_14 = local_14 + 0x1;
}
*param_2 = iVar1;^
// goto LAB_0041ae5e;
}



fn FUN_0041b05a(param_1: i32,param_2: i32,param_3: u32) -> u32

{
    let mut bVar1: bool;
    let mut iVar2: i32;
    let mut local_194: i32;
    let mut local_190: i32;
    let mut local_160: u32;
    let mut local_15c: i32;
    let mut local_158: u32;
    let mut local_154: i32;
    let mut local_150: i32;
    let mut local_14c: i32;
    let mut local_148: i32;
    let mut local_144: i32;
    let mut local_140: u32;
    let mut local_13c: u32;
    let local_138: *mut u32;
    ushort *local_134;
    ushort *local_130;
    let local_12c: *mut u32;
    let mut local_128: i32;
    let mut local_124: i32;
    let mut local_120: u32;
    let mut local_11c: u32;
    let mut local_118: u32;
    let mut local_114: u32;
    let local_110: *mut u32;
    let mut local_10c: i32;
    let mut local_108: i32;
    ushort *local_104;
    ushort *local_100;
    let mut local_fc: i32;
    let mut local_f8: u32;
    let mut local_f4: u32;
    let mut local_f0: i32;
    let mut local_ec: i32;
    let mut local_e8: u32;
    let local_e4: *mut u32;
    let local_e0: *mut u32;
    let mut local_dc: i32;
    let mut local_d8: i32;
    let mut local_d4: i32;
    let local_d0: *mut u32;
    let local_cc: *mut u32;
    let mut local_c8: u32;
    let mut local_c4: u32;
    let local_c0: *mut u32;
    let local_bc: *mut u32;
    let mut local_b8: u32;
    let mut local_b4: u32;
    let local_b0: *mut u32;
    let local_ac: *mut u32;
    let mut local_a8: u32;
    let local_a4: *mut u32;
    let local_a0: *mut u32;
    let mut local_9c: u32;
    let mut local_98: u32;
    let mut local_94: u32;
    let mut local_90: u32;
    let mut local_8c: u32;
    let mut local_88: u32;
    let local_84: *mut u32;
    let local_80: *mut u32;
    let mut local_7c: u32;
    let mut local_78: u32;
    let mut local_74: i32;
    let mut local_70: u32;
    let mut local_6c: i32;
    let mut local_68: u32;
    let mut local_64: i32;
    let local_5c: *mut u32;
    let mut local_58: i32;
    let mut local_54: i32;
    let mut local_50: i32;
    let mut local_4c: i32;
    let mut local_48: i32;
    let mut local_44: i32;
    let mut local_40: u32;
    let mut local_3c: u32;
    let mut local_38: u32;
    let mut local_34: u32;
    let mut local_30: u32;
    let mut local_2c: u32;
    let mut local_28: u32;
    ushort *local_24;
    ushort *local_20;
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: u32;

    FUN_00419085();
    FUN_0042feac();
    FUN_00431d0a(&DAT_005967b8);
    DAT_005967bc = param_1;
    local_58 = FUN_00432c94(&DAT_005967b8);
    local_34 = param_1 + 0x20;
    local_1c = local_34 & 0xffff0000 | (param_1 + 0x22);
    local_50 = (param_1 + 0x22);
    local_2c = param_1 + 0x20;
    local_14 = local_2c & 0xffff0000 | (param_1 + 0x24);
    local_4c = (param_1 + 0x24);
    local_24 = (param_1 + 0x20);
    local_18 = local_24 & 0xffff0000 | *local_24;
    local_48 = *local_24;
    local_44 = 0x0;
    local_54 = local_58;
    local_30 = local_34;
    local_28 = local_2c;
    local_20 = local_24;
    FUN_00431d31(&local_68);
    FUN_00431d31(&local_70);
    FUN_00431d31(&local_78);
    for (local_5c = (&DAT_005b8b44 + local_48 * 0x4); local_5c != 0x0;
        local_5c = *local_5c) {
        local_84 = local_5c + 0x8;
        local_7c = local_84 & 0xffff0000 | local_84;
        local_80 = local_84;
        if (local_84 != local_48) break;
        local_a4 = local_5c + 0x8;
        local_a8 = *(local_5c + 0x3a) & 0x1;
        local_a0 = local_a4;
        if (local_a8 == 0x0) {
            LAB_0041b285:
            if ((((local_5c + 0x2a) == 0x9) || ((local_5c + 0x2a) == 0x7)) &&
                (*(local_5c + 0x23) >> 0x18 == DAT_004c9754)) {
                local_d0 = local_5c + 0x8;
                if (((local_5c + 0x35) == -0x1) ||
                    ((local_5c + 0x26) == (local_5c + 0x35))) {
                    local_d4 = 0x0;
                }
                else {
                    local_d4 = 0x1;
                }
                local_d8 = local_d4;
                local_cc = local_d0;
                if ((local_d4 == 0x0) &&
                    (local_dc = *(*(&DAT_00582938 +
                        (*(local_5c + 0x25) >> 0x18) * 0x4 +
                        (local_5c[0x9] >> 0x18) * 0x18) + 0xa5), local_dc != 0x0)) {
                    local_e4 = local_5c + 0x8;
                    local_e8 = *(local_5c + 0x3a) & 0x1;
                    local_e0 = local_e4;
                    if (local_e8 == 0x0) {
                        if ((local_5c + 0x2f) == '\x03') {
                            local_ec = 0x0;
                            for (local_f0 = 0x0; local_f0 < local_dc; local_f0 = local_f0 + 0x1) {
                                if (local_5c[local_f0 + 0x4] != 0x0) {
                                    local_ec = local_ec + 0x1;
                                }
                            }
                            if (local_ec < local_dc) {
                                local_44 = local_44 + (local_dc - local_ec);
                                FUN_00431efd(&local_68,local_5c);
                                if (local_58 <= local_44) break;
                            }
                        }
                    }
                    else {
                        if (0x1 < local_5c[0xb] >> 0x18) {
                            FUN_00431efd(&local_70,local_5c);
                        }
                    }
                }
            }
        }
        else {
            local_b0 = local_5c + 0x8;
            local_9c = local_b0 & 0xffff0000 | (local_5c + 0x22);
            local_b8 = param_1 + 0x20;
            local_90 = local_b8 & 0xffff0000 | (param_1 + 0x22);
            local_b4 = local_b8;
            local_ac = local_b0;
            local_8c = local_9c;
            if ((param_1 + 0x22) == (local_5c + 0x22)) {
                local_c0 = local_5c + 0x8;
                local_98 = local_c0 & 0xffff0000 | (local_5c + 0x9);
                local_c8 = param_1 + 0x20;
                local_88 = local_c8 & 0xffff0000 | (param_1 + 0x24);
                local_c4 = local_c8;
                local_bc = local_c0;
                local_94 = local_98;
                if ((param_1 + 0x24) == (local_5c + 0x9))^ // goto LAB_0041b285;
            }
        }
    }
    local_f8 = param_3;
    local_fc = param_2;
    local_104 = (param_1 + 0x20);
    local_f4 = local_104 & 0xffff0000 | *local_104;
    local_100 = local_104;
    iVar2 = FUN_0041d5ed(*(param_1 + 0x24) >> 0x18,*local_104,param_2,param_3,&local_40,&local_3c);
    if (iVar2 == 0x0) {
        return 0x2;
    }
    if (local_6c != 0x0) {
        iVar2 = FUN_0041ae21(&local_70,&param_1,local_40,local_3c,&local_10c,&local_108);
        local_58 = local_58 - iVar2;
        if (local_10c == 0x2) {
            return 0x2;
        }
        local_74 = local_108;
    }
    local_38 = 0x2;
    if (local_58 != 0x0) {
        local_110 = &local_68;
        local_114 = (local_64 == 0x0);
        local_118 = local_114;
        if (local_114 == 0x0) {
            DAT_005967bc = local_64;
            local_124 = local_4c;
            local_128 = local_50;
            local_12c = &DAT_005967b8;
            local_134 = (local_64 + 0x20);
            local_120 = local_134 & 0xffff0000 | *local_134;
            local_130 = local_134;
            local_11c = local_120;
            iVar2 = FUN_00457f10(&DAT_005967b8,*local_134,local_50,local_4c,0x0);
            if (iVar2 == 0x0)^ // goto LAB_0041b9ec;
            local_138 = &DAT_005967b8;
            local_13c = (DAT_005967bc == 0x0);
            local_140 = local_13c;
            if (local_13c != 0x0)^ // goto LAB_0041b9ec;
            local_64 = DAT_005967bc;
            iVar2 = FUN_0041ae21(&local_68,&param_1,local_40,local_3c,&local_148,&local_144);
            local_58 = local_58 - iVar2;
            if (local_148 == 0x2)^ // goto LAB_0041b9ec;
            while (local_144 != 0x0) {
                for (local_14c = *(local_144 + 0x8); local_14c != 0x0; local_14c = *(local_14c + 0x8)) {
                    local_154 = local_14c + 0x20;
                    local_158 = *(local_14c + 0x3a) & 0x40;
                    local_150 = local_154;
                    if (local_158 == 0x0) break;
                }
                FUN_00431efd(&local_78,local_144);
                local_144 = local_14c;
            }
        }
    }
    if (local_58 != 0x0) {
        FUN_00431d31(&local_160);
        local_44 = 0x0;
        for (local_5c = (&DAT_005b8b44 + local_48 * 0x4);
            (local_5c != 0x0 && ((local_5c + 0x8) == local_48)); local_5c = *local_5c)
        {
            if ((((local_5c + 0x2a) == 0x9) || ((local_5c + 0x2a) == 0x7)) &&
                (*(local_5c + 0x23) >> 0x18 == DAT_004c9754)) {
                if (((local_5c + 0x35) == -0x1) ||
                    ((local_5c + 0x26) == (local_5c + 0x35))) {
                    bVar1 = false;
                }
                else {
                    bVar1 = true;
                }
                if ((((!bVar1) &&
                    (iVar2 = *(*(&DAT_00582938 +
                        (*(local_5c + 0x25) >> 0x18) * 0x4 +
                        (local_5c[0x9] >> 0x18) * 0x18) + 0xa5), iVar2 != 0x0)) &&
                    ((local_5c + 0x2f) != '\0')) && ((*(local_5c + 0x3a) & 0x1) == 0x0)) {
                    local_190 = 0x0;
                    for (local_194 = 0x0; local_194 < iVar2; local_194 = local_194 + 0x1) {
                        if (local_5c[local_194 + 0x4] != 0x0) {
                            local_190 = local_190 + 0x1;
                        }
                    }
                    if (local_190 < iVar2) {
                        local_44 = local_44 + (iVar2 - local_190);
                        FUN_00431efd(&local_160,local_5c);
                        if (local_58 <= local_44) break;
                    }
                }
            }
        }
        if (local_15c != 0x0) {
            DAT_005967bc = local_15c;
            FUN_00457f10(&DAT_005967b8,(local_15c + 0x20),local_50,local_4c,0x0);
        }
    }
    LAB_0041b9ec:
        DAT_005967bc = param_1;
    FUN_00413584(&DAT_005967b8);
    DAT_005967bc = local_6c;
    FUN_004579ea(&DAT_005967b8);
    if (local_74 != 0x0) {
        local_38 = 0x0;
    }
    DAT_005967bc = local_74;
    return local_38;
}



fn FUN_0041ba73(param_1: *mut u32) -> *mut u32

{
    let bVar1: u8;
    let mut iVar2: i32;
    let mut local_58: u32;
    let local_54: *mut u32;
    let mut local_50: u32;
    let mut local_4c: u32;
    let local_48: *mut u32;
    let local_44: *mut u32;
    let local_40: *mut u32;
    let local_3c: *mut u32;
    let local_38: *mut u32;
    let local_34: *mut u32;
    let mut local_30: u32;
    let mut local_2c: u32;
    let mut local_28: u32;
    let local_20: *mut u32;
    let local_1c: *mut u32;
    let local_18: *mut u32;
    let mut local_14: u32;

    local_1c = param_1 + 0x8;
    local_14 = local_1c & 0xffff0000 | local_1c;
    local_20 = (&DAT_005b8b44 + local_1c * 0x4);
    local_18 = local_1c;
    while( true ) {
        if (local_20 == 0x0) {
            return 0x0;
        }
        local_38 = local_20 + 0x8;
        local_30 = local_38 & 0xffff0000 | local_38;
        local_40 = param_1 + 0x8;
        local_28 = local_40 & 0xffff0000 | local_40;
        if (local_40 != local_38) break;
        if (((local_20 + 0x12) == '\x03') && (*(local_20 + 0x23) >> 0x18 == DAT_004c9754)) {
            local_48 = local_20 + 0x8;
            local_4c = *(local_20 + 0x3a) & 0x1;
            if ((local_4c != 0x0) && ((local_20[0x3] == 0x0 && (local_20 != param_1)))) {
                local_44 = local_48;
                local_3c = local_40;
                local_34 = local_38;
                local_2c = local_30;
                FUN_00431d31(&local_58);
                local_54 = local_20;
                local_50 = (local_20 + 0x8) & 0xffff0000 | (local_20 + 0x9);
                bVar1 = *(*(DAT_005b2d50 + (local_20 + 0x22) * 0x4) +
                    (local_20 + 0x9));
                iVar2 = FUN_00432bd3(&local_58);
                if (bVar1 <= iVar2 + 0x8) {
                    return local_20;
                }
            }
        }
        local_20 = *local_20;
    }
    return 0x0;
}



fn FUN_0041bbec(param_1: *mut u32)

{
    let piVar1: *mut i32;;
    let local_14: *mut i32;;

    while (piVar1 = FUN_0041ba73(param_1), piVar1 != 0x0) {
        FUN_00419085();
        FUN_0042feac();
        (piVar1 + 0x12) = 0x6;
        (piVar1 + 0x41) = (char)*(param_1 + 0x22);
        (piVar1 + 0x42) = (char)*(param_1 + 0x9);
        DAT_005967bc = piVar1;
        FUN_0041361b(piVar1);
        if (DAT_005967bc != 0x0) {
            local_14 = DAT_005967bc;
            if (((param_1 + 0x22) == (DAT_005967bc + 0x22)) &&
                ((param_1 + 0x9) == (DAT_005967bc + 0x9))) {
                DAT_005967bc = param_1;
                while (local_14 != 0x0) {
                    piVar1 = local_14[0x2];
                    FUN_00431efd(&DAT_005967b8,local_14);
                    (local_14 + 0x12) = 0x3;
                    local_14 = piVar1;
                }
            }
            else {
                FUN_00413546(&DAT_005967b8);
            }
        }
        DAT_005967bc = param_1;
        FUN_0045af67(&DAT_005967b8,(param_1 + 0x22),(param_1 + 0x9),
                     (param_1 + 0x22),(param_1 + 0x9),0x1869f);
    }
    return;
}



fn FUN_0041be7a(param_1: i32) -> u32

{
    let mut iVar1: i32;
    let local_24: *mut u32;

    local_24 = (&DAT_005b8b44 + (param_1 + 0x20) * 0x4);
    while( true ) {
        if ((local_24 == 0x0) || ((param_1 + 0x20) != (local_24 + 0x8))) {
            return 0x0;
        }
        if ((((local_24 + 0x2a) == 0x8) &&
            ((&DAT_004d55a8)[DAT_004c9754 * 0xe + (*(local_24 + 0x23) >> 0x18)] == '\x02')) &&
            (iVar1 = FUN_0044a87f((local_24 + 0x22),(local_24 + 0x9),
                                  (param_1 + 0x22),(param_1 + 0x24)), iVar1 < 0x6)) break;
        local_24 = *local_24;
    }
    return 0x1;
}



fn FUN_0041c01d(param_1: i32)

{
    let sVar1: i16;
    let mut bVar2: bool;
    let mut bVar3: bool;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let mut iVar6: i32;
    let mut local_78: i32;
    let local_38: *mut u32;

    if (((param_1 + 0x35) == -0x1) || ((param_1 + 0x26) == (param_1 + 0x35))) {
        bVar3 = false;
    }
    else {
        bVar3 = true;
    }
    sVar1 = (param_1 + 0x20);
    FUN_00431d0a(&DAT_005967b8);
    local_38 = (&DAT_005b8b44 + sVar1 * 0x4);
    loop {
    if ((local_38 == 0x0) || ((local_38 + 0x8) != sVar1)) {
        return;
    }
    if (((*(local_38 + 0x3a) & 0x1) == 0x0) && ((local_38 + 0x2f) != '\0')) {
        iVar4 = local_38[0x9] >> 0x18;
        iVar5 = *(local_38 + 0x25) >> 0x18;
        if (((*(*(&DAT_00582938 + iVar4 * 0x18 + iVar5 * 0x4) + 0xad) == 0x0) &&
            ((((*(*(&DAT_00582938 + iVar5 * 0x4 + iVar4 * 0x18) + 0x95) != 0x0 ||
                (*(*(&DAT_00582938 + iVar5 * 0x4 + iVar4 * 0x18) + 0x99) != 0x0)) ||
                (*(*(&DAT_00582938 + iVar5 * 0x4 + iVar4 * 0x18) + 0x9d) != 0x0)) ||
                (*(*(&DAT_00582938 + iVar5 * 0x4 + iVar4 * 0x18) + 0xa1) != 0x0)))) &&
            (((*(local_38 + 0x3a) & 0x40) == 0x0 &&
                ((local_38 + 0x26) == (param_1 + 0x26))))) {
            if (((local_38 + 0x35) == -0x1) ||
                ((local_38 + 0x26) == (local_38 + 0x35))) {
                bVar2 = false;
            }
            else {
                bVar2 = true;
            }
            if (bVar2) {
                if ((bVar3) && ((param_1 + 0x35) == (local_38 + 0x35))) {
                    LAB_0041c268:
                    if (((*(*(&DAT_00582938 + iVar5 * 0x4 + iVar4 * 0x18) + 0xa5) == 0x0) || (iVar4 == 0x10)) &&
                        (FUN_00431efd(&DAT_005967b8,local_38), iVar4 == 0x10)) {
                        for (local_78 = 0x0; local_78 < *(*(iVar5 * 0x4 + 0x582ab8) + 0xa5);
                            local_78 = local_78 + 0x1) {
                            if ((local_38[local_78 + 0x4] != 0x0) && ((local_38 + 0x2f) != '\0')) {
                                iVar4 = local_38[0x9] >> 0x18;
                                iVar6 = *(local_38 + 0x25) >> 0x18;
                                if ((*(*(&DAT_00582938 + iVar4 * 0x18 + iVar6 * 0x4) + 0xad) == 0x0) &&
                                    ((((*(*(&DAT_00582938 + iVar6 * 0x4 + iVar4 * 0x18) + 0x95) != 0x0 ||
                                        (*(*(&DAT_00582938 + iVar6 * 0x4 + iVar4 * 0x18) + 0x99) != 0x0)) ||
                                        (*(*(&DAT_00582938 + iVar6 * 0x4 + iVar4 * 0x18) + 0x9d) != 0x0)) ||
                                        (*(*(&DAT_00582938 + iVar6 * 0x4 + iVar4 * 0x18) + 0xa1) != 0x0)))) {
                                    FUN_00431efd(&DAT_005967b8,local_38[local_78 + 0x4]);
                                }
                            }
                        }
                    }
                }
            }
            else {
                if (!bVar3)^ // goto LAB_0041c268;
            }
        }
    }
    local_38 = *local_38;
} while( true );
}



// WARNING: Could not reconcile some variable overlaps

fn FUN_0041c3cf(param_1: i32,param_2: i32)

{
    let mut iVar1: i32;
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_1c: i32;

    FUN_004a0430(param_2,0xff,0x5c);
    for (local_1c = 0x0; local_1c < 0x3; local_1c = local_1c + 0x1) {
        for (local_20 = 0x0; local_20 < 0x46; local_20 = local_20 + 0x1) {
            if (*(&DAT_004c71ac + local_20 * 0x4 + local_1c * 0x118) != 0x0) {
                (*(&DAT_004c71ac + local_20 * 0x4 + local_1c * 0x118) + 0xc) = 0x0;
            }
        }
    }
    for (local_1c = 0x0; local_1c < 0x5c; local_1c = local_1c + 0x1) {
        local_24 = 0x0;
        while ((local_24 < 0x5 && (iVar1 = *(&DAT_00582938 + local_24 * 0x4 + local_1c * 0x18), iVar1 != 0x0))) {
            if (((((*(param_1 + 0xc) >> 0x10 == *(iVar1 + 0xf1)) || (*(iVar1 + 0xf1) == 0x63)) &&
                ((((((&DAT_00569c30)[*(iVar1 + 0xf9) * 0x9 + DAT_004c9754 * 0x1e22] & 0x1) != 0x0 &&
                    (((&DAT_00569c30)[*(iVar1 + 0xfd) * 0x9 + DAT_004c9754 * 0x1e22] & 0x1) != 0x0)) &&
                    (((&DAT_00569c30)[*(iVar1 + 0x101) * 0x9 + DAT_004c9754 * 0x1e22] & 0x1) != 0x0)) &&
                    (((&DAT_00569c30)[*(iVar1 + 0x105) * 0x9 + DAT_004c9754 * 0x1e22] & 0x1) != 0x0)))) &&
                ((*(iVar1 + 0xed) == -0x1 ||
                    (((*(iVar1 + 0xed) != 0x2d || ((*(param_1 + 0x36) & 0x1) == 0x0)) &&
                        ((*(iVar1 + 0xed) != 0x33 || ((*(param_1 + 0x36) & 0x2) == 0x0)))))))) &&
                (*(iVar1 + 0x41) != 0x4)) {
                (param_2 + local_1c) = local_24;
                iVar1 = *(&DAT_004bd138 + *(iVar1 + 0x41) * 0x4);
                for (local_28 = 0x0; local_28 < 0x46; local_28 = local_28 + 0x1) {
                    if (((*(&DAT_004c71ac + local_28 * 0x4 + iVar1 * 0x118) != 0x0) &&
                        (**(i32 **)(&DAT_004c71ac + local_28 * 0x4 + iVar1 * 0x118) == local_1c)) &&
                        (*(*(&DAT_004c71ac + local_28 * 0x4 + iVar1 * 0x118) + 0x4) == local_24)) {
                        (*(&DAT_004c71ac + local_28 * 0x4 + iVar1 * 0x118) + 0xc) = 0x1;
                    }
                }
            }
            local_24 = local_24 + 0x1;
        }
    }
    if (*(&DAT_004be7b4 + DAT_004c9754 * 0x4) == 0x0) {
        (param_2 + 0x2c) = 0xff;
        (param_2 + 0x35) = (param_2 + 0x2c);
    }
    return;
}



fn FUN_0041c67a(param_1: i32,param_2: i32,param_3: i32)

{
    if (*(param_1 + 0x16) >> 0x10 < 0x1) {
        (param_1 + 0x14) = param_2;
        (param_1 + 0x16) = param_3;
        *(param_1 + 0x18) = *(*(&DAT_00582938 + param_2 * 0x18 + param_3 * 0x4) + 0xf5);
        FUN_004664c9(*(param_1 + 0x6) >> 0x10,*(param_1 + 0x8) >> 0x10,*(param_1 + 0xa) >> 0x10,param_2
                     ,param_3,param_1);
        FUN_00466d5a(*(param_1 + 0x6) >> 0x10,param_2,param_3,DAT_004c9754);
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0041c726(param_1: i32) -> u32

{
    let mut iVar1: i32;
    let mut uVar2: u32;
    let mut bVar3: bool;
    let mut local_34: u32;
    let mut local_30: i32;
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    if ((((DAT_004c9754 == 0x6) && (*(param_1 + 0x6) >> 0x10 == _DAT_004d5580)) &&
        (((param_1 + 0xe) == 0xd || ((param_1 + 0xe) == 0x1)))) && (_DAT_004c74f4 < 0xa)) {
        _DAT_004c74f4 = _DAT_004c74f4 + 0x1;
        FUN_0041c67a(param_1,0x2f,0x0);
        local_34 = 0x1;
    }
    else {
        if ((param_1 + 0xe) == 0xe) {
            local_30 = 0x0;
        }
        else {
            uVar2 = FUN_004a2edc();
            if (uVar2 % 0x64 < 0x50) {
                local_30 = 0x1;
            }
            else {
                local_30 = 0x2;
            }
        }
        local_2c = 0x0;
        for (local_28 = 0x0; local_28 < 0x46; local_28 = local_28 + 0x1) {
            if ((*(&DAT_004c71ac + local_28 * 0x4 + local_30 * 0x118) != 0x0) &&
                ((*(&DAT_004c71ac + local_28 * 0x4 + local_30 * 0x118) + 0xc) != '\0')) {
                local_2c = local_2c + 0x1;
            }
        }
        if (local_2c == 0x0) {
            for (local_28 = 0x0; local_28 < 0x46; local_28 = local_28 + 0x1) {
            }
            local_34 = 0x0;
        }
        else {
            uVar2 = FUN_004a2edc();
            local_1c = 0x0;
            local_18 = 0x0;
            for (local_14 = local_2c; local_14 <= uVar2 % (((local_2c + 0x1) * local_2c) / 0x2);
                local_14 = local_14 + (local_2c - local_18)) {
                local_18 = local_18 + 0x1;
            }
            local_2c = 0x0;
            for (local_28 = 0x0; local_28 < 0x46; local_28 = local_28 + 0x1) {
                if (((*(&DAT_004c71ac + local_28 * 0x4 + local_30 * 0x118) != 0x0) &&
                    ((*(&DAT_004c71ac + local_28 * 0x4 + local_30 * 0x118) + 0xc) != '\0')) &&
                    (iVar1 = local_2c + 0x1, bVar3 = local_2c == local_18, local_2c = iVar1, bVar3)) {
                    local_1c = local_28;
                }
            }
            FUN_0041c67a(param_1,**(i32 **)(&DAT_004c71ac + local_30 * 0x118 + local_1c * 0x4),
                         *(*(&DAT_004c71ac + local_30 * 0x118 + local_1c * 0x4) + 0x4));
            local_34 = 0x1;
        }
    }
    return local_34;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0041c9b2(param_1: i32,param_2: i32) -> u32

{
    let mut bVar1: bool;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut local_1c: u32;
    let mut local_18: i32;

    bVar1 = false;
    local_18 = 0x0;
    loop {
    if (0x5b < local_18) {
        LAB_0041ca52:
        if (bVar1) {
            loop {
                uVar2 = FUN_004a2edc();
                iVar3 = uVar2 % 0x7;
            } while ((param_2 + *(&DAT_004bd11c + iVar3 * 0x4)) == -0x1);
            FUN_0041c67a(param_1,*(&DAT_004bd11c + iVar3 * 0x4),
                         *(param_2 + *(&DAT_004bd11c + iVar3 * 0x4)));
            local_1c = 0x1;
        }
        else {
            local_1c = 0x0;
        }
        return local_1c;
    }
    if (((param_2 + local_18) != -0x1) &&
        ((((local_18 == _DAT_004bd11c || (local_18 == _DAT_004bd120)) || (local_18 == _DAT_004bd124)) ||
            (((local_18 == _DAT_004bd128 || (local_18 == _DAT_004bd12c)) ||
                ((local_18 == _DAT_004bd130 || (local_18 == _DAT_004bd134)))))))) {
        bVar1 = true;^
        // goto LAB_0041ca52;
    }
    local_18 = local_18 + 0x1;
} while( true );
}



fn FUN_0041cad6()

{
    let mut iVar1: i32;
    let mut uVar2: u32;
    let mut local_b8: i32;
    let local_78: u8 [0x35];
    let local_43: u8;
    let local_1c: *mut u32;
    let local_18: *mut u32;
    let mut local_14: i32;

    local_14 = DAT_004c9754;
    if (DAT_004c9754 == 0x7) {
        FUN_00419085();
        for (local_18 = *DAT_005967c8; local_18 != 0x0; local_18 = *local_18) {
            if (((*(local_18 + 0x16) >> 0x10 < 0x1) && (*(local_18 + 0xe) >> 0x10 == DAT_004c9754)) &&
                ((local_18 + 0xe) != 0x17)) {
                if ((local_18 + 0x1a) == -0x1) {
                    *(local_18 + 0x1a) = 0x0;
                }
                FUN_0041c67a(local_18,*(&DAT_004bd164 + (local_18[0x6] >> 0x10) * 0x4),0x0);
                (local_18 + 0x1a) = (local_18 + 0x1a) + 0x1;
                if ((local_18 + 0x1a) == 0x14) {
                    *(local_18 + 0x1a) = 0x8;
                }
            }
        }
    }
    else {
        FUN_004a0430(local_78,0x0,0x5c);
        FUN_004144a5(local_78,0x1);
        FUN_00414614(0x1,0x0,local_78,0xa,0x0);
        FUN_00419085();
        for (local_18 = *DAT_005967c8; local_18 != 0x0; local_18 = *local_18) {
            *(local_18 + 0x36) = 0x0;
        }
        for (local_1c = *DAT_005967b0; local_1c != 0x0; local_1c = *local_1c) {
            if ((((local_1c + 0x27) == '-') || ((local_1c + 0x27) == '3')) &&
                ((*(local_1c + 0x23) >> 0x18 == DAT_004c9754 &&
                    (((*(local_1c + 0x3a) & 0x1) != 0x0 &&
                        (local_18 = FUN_00481784((local_1c + 0x8),(local_1c + 0x22),
                                                 (local_1c + 0x9)), local_18 != 0x0)))))) {
                if ((local_1c + 0x27) == '-') {
                    *(local_18 + 0x36) = *(local_18 + 0x36) | 0x1;
                }
                else {
                    *(local_18 + 0x36) = *(local_18 + 0x36) | 0x2;
                }
            }
        }
        FUN_00419085();
        for (local_18 = *DAT_005967c8; local_18 != 0x0; local_18 = *local_18) {
            if ((*(local_18 + 0x16) >> 0x10 < 0x1) && (*(local_18 + 0xe) >> 0x10 == DAT_004c9754)) {
                FUN_0041c3cf(local_18,local_78);
                if ((local_18 + 0xe) == 0x4) {
                    local_b8 = 0x0;
                    iVar1 = *(local_18 + 0x6) >> 0x10;
                    for (local_1c = (&DAT_005b8b44 + iVar1 * 0x4);
                        (local_1c != 0x0 && ((local_1c + 0x8) == iVar1));
                        local_1c = *local_1c) {
                        if (((local_1c + 0x22) == local_18[0x2] >> 0x10) &&
                            (((local_1c + 0x9) == *(local_18 + 0xa) >> 0x10 &&
                                ((local_1c + 0x27) != '[')))) {
                            local_b8 = local_b8 + 0x1;
                        }
                    }
                    if ((local_b8 < 0x7) && (iVar1 = FUN_0041c9b2(local_18,local_78), iVar1 == 0x0)) {
                        FUN_0041c726(local_18);
                    }
                }
                else {
                    if ((((local_18 + 0xe) == 0x3) || ((local_18 + 0xe) == 0xd)) ||
                        ((local_18 + 0xe) == 0xe)) {
                        FUN_0041c726(local_18);
                    }
                    else {
                        if (local_18[0xc] >> 0x10 < not_enough_garrison_00599d40) {
                            if (((local_18[0xc] >> 0x10 < 0x1) || (uVar2 = FUN_004a2edc(), 0x0 < uVar2 % DAT_004bd160)) ||
                                (local_43 == -0x1)) {
                                iVar1 = FUN_0041c9b2(local_18,local_78);
                                if (iVar1 == 0x0) {
                                    FUN_0041c726(local_18);
                                }
                            }
                            else {
                                FUN_0041c67a(local_18,0x35,0x0);
                            }
                        }
                        else {
                            if ((local_43 != -0x1) && (uVar2 = FUN_004a2edc(), uVar2 % DAT_004bd160 < 0x1)) {
                                FUN_0041c67a(local_18,0x35,0x0);
                            }
                        }
                    }
                }
            }
        }
    }
    return;
}



fn FUN_0041cffc(param_1: i32,param_2: i32) -> u32

{
    let mut local_1c: u32;

    if (*(param_1 + 0x8) == *(param_2 + 0x8)) {
        local_1c = 0x0;
    }
    else {
        if (*(param_1 + 0x8) < *(param_2 + 0x8)) {
            local_1c = 0xffffffff;
        }
        else {
            local_1c = 0x1;
        }
    }
    return local_1c;
}



fn FUN_0041d054() -> i32

{
let mut uVar1: u32;
let mut bVar2: bool;
let mut pCVar3: String;;
let mut iVar4: i32;
i32 local_2c [0x4];
let mut local_1c: i32;
let mut local_18: i32;
let mut local_14: i32;

local_2c[2] = 0x0;
local_2c[1] = 0x0;
local_2c[0] = 0x0;
iVar4 = 0x0;
for (local_18 = 0x0; local_18 < 0x3; local_18 = local_18 + 0x1) {
for (local_14 = 0x0; local_14 < 0x46; local_14 = local_14 + 0x1) {
*(&DAT_004c71ac + local_14 * 0x4 + local_18 * 0x118) = 0x0;
}
iVar4 = local_18;
}
for (local_18 = 0x0; local_18 < 0x5c; local_18 = local_18 + 0x1) {
if ((((local_18 != 0x54) && (local_18 != 0x55)) && (local_18 != 0x56)) &&
(((local_18 != 0x57 && (local_18 != 0x58)) && (local_18 != 0x35)))) {
local_14 = 0x0;
while ((local_14 < 0x5 &&
(local_2c[3] = *(&DAT_00582938 + local_14 * 0x4 + local_18 * 0x18), local_2c[3] != 0x0))) {
if (((*(local_2c[3] + 0xad) == 0x0) &&
(((*(local_2c[3] + 0xf1) != -0x1 && (*(local_2c[3] + 0xf1) != 0x7)) &&
(*(local_2c[3] + 0xf1) != 0xa)))) && (*(local_2c[3] + 0x41) != 0x4)) {
local_1c = *(&DAT_004bd138 + *(local_2c[3] + 0x41) * 0x4);
pCVar3 = FUN_004a2831(0xd);
*(&DAT_004c71ac + local_1c * 0x118 + local_2c[local_1c] * 0x4) = pCVar3;
**(i32 **)(&DAT_004c71ac + local_2c[local_1c] * 0x4 + local_1c * 0x118) = local_18;
*(*(&DAT_004c71ac + local_2c[local_1c] * 0x4 + local_1c * 0x118) + 0x4) = local_14;
(*(&DAT_004c71ac + local_1c * 0x118 + local_2c[local_1c] * 0x4) + 0xc) = 0x0;
*(*(&DAT_004c71ac + local_2c[local_1c] * 0x4 + local_1c * 0x118) + 0x8) =
*(&DAT_0058ad7e + *(local_2c[3] + 0x105) * 0xda) +
*(local_2c[3] + 0x121) + *(local_2c[3] + 0x125) +
*(&DAT_0058ad7e + *(local_2c[3] + 0xf9) * 0xda) +
*(&DAT_0058ad7e + *(local_2c[3] + 0xfd) * 0xda) +
*(&DAT_0058ad7e + *(local_2c[3] + 0x101) * 0xda);
local_2c[local_1c] = local_2c[local_1c] + 0x1;
}
local_14 = local_14 + 0x1;
}
}
iVar4 = local_18;
}
for (local_18 = 0x0; local_18 < 0x3; local_18 = local_18 + 0x1) {
bVar2 = false;
while (!bVar2) {
bVar2 = true;
for (local_14 = 0x0; local_14 < local_2c[local_18] + -0x1; local_14 = local_14 + 0x1) {
if (*(*(&DAT_004c71ac + local_18 * 0x118 + local_14 * 0x4) + 0x8) <
*(*(&DAT_004c71b0 + local_14 * 0x4 + local_18 * 0x118) + 0x8)) {
uVar1 = *(&DAT_004c71b0 + local_14 * 0x4 + local_18 * 0x118);
*(&DAT_004c71b0 + local_18 * 0x118 + local_14 * 0x4) =
*(&DAT_004c71ac + local_14 * 0x4 + local_18 * 0x118);
*(&DAT_004c71ac + local_14 * 0x4 + local_18 * 0x118) = uVar1;
bVar2 = false;
}
}
}
iVar4 = local_18;
}
return iVar4;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0041d3ba()

{
    let puVar1: *mut u32;
    let mut local_24: i32;
    i32 **local_20;
    i32 **local_1c;
    let local_18: *mut u32;

    _DAT_00582910 = 0x0;
    FUN_0041a516(0xf);
    local_18 = FUN_004840cd(&local_1c,&local_20,-0x1);
    while (local_20 != (i32 **)0x0) {
    if ((local_20 + 0x12) == '\x0f') {
        puVar1 = FUN_00481784((local_20 + 0x8),*(local_20 + 0x41),
                              *(local_20 + 0x42));
        if ((puVar1 == 0x0) || ((puVar1 + 0xe) != 0x17)) {
            (local_20 + 0x12) = 0x0;
        }
        else {
            FUN_0041361b(local_20);
        }
    }
    local_20 = local_1c;
    if (local_1c != (i32 **)0x0) {
        local_1c = (i32 **)*local_1c;
    }
}
    if (_DAT_00582910 == 0x0) {
        FUN_0041da19();
    }
    for (local_24 = 0x0; local_24 < 0x5; local_24 = local_24 + 0x1) {
        if (((&DAT_00569a98)[local_24 * 0x1e22] & 0x10) == 0x0) {
            if (*(&DAT_00574fe0 + local_24 * 0x4) < 0xb) {
                FUN_004082e7(local_24);
                FUN_00419085();
            }
            *(&DAT_004c74f8 + local_24 * 0x4) = ((&DAT_00569a98)[local_24 * 0x1e22] & 0x1);
        }
    }
    FUN_0041d83c();
    FUN_0041d539();
    FUN_0048418d(&local_1c);
    return;
}



fn FUN_0041d539()

{
    let puVar1: *mut u32;
    let local_14: *mut u32;

    puVar1 = *DAT_005967b0;
    while (local_14 = puVar1, local_14 != 0x0) {
        puVar1 = *local_14;
        if (((((local_14 + 0x27) == '/') && ((local_14 + 0x26) == '\x06')) &&
            ((local_14 + 0x12) != '\x04')) &&
            ((((local_14 + 0x12) != '\x0f' && ((*(local_14 + 0x3a) & 0x1) != 0x0)) &&
                ((*(local_14 + 0x3a) & 0x40) == 0x0)))) {
            (local_14 + 0x12) = 0x2;
        }
    }
    return;
}



fn FUN_0041d5ed(param_1: i32,param_2: i32,param_3: i32,param_4: u32,param_5: *mut u32,param_6: *mut u32) -> u32

{
    let mut uVar1: u32;
    let mut iVar2: i32;
    let mut local_88: u32;
    let local_80: u8 [0x8];
    let mut local_78: u32;
    let mut local_74: u32;
    let local_60: u16;
    let local_5e: u16;
    let local_5c: u16;
    let local_5a: u8;
    let local_59: u8;
    let local_56: u16;
    let local_51: u8;
    let mut local_30: u32;
    let mut local_2c: *mut u8;
    let mut local_24: i32;
    let mut local_20: u32;
    let mut local_1c: u32;
    let mut local_18: i32;
    let mut local_14: i32;

    local_24 = 0x0;
    FUN_00486065(&local_60);
    local_5a = DAT_004c9754;
    local_59 = param_1;
    local_56 = *(*(&DAT_00582938 + param_1 * 0x18) + 0x41);
    local_51 = 0xff;
    FUN_00431d31(&local_30);
    local_2c = local_80;
    local_74 = 0x0;
    local_78 = 0x0;
    FUN_00432a04(&local_30,(char)param_3,(char)param_4);
    local_18 = 0x7;
    local_14 = 0x4;
    loop {
    if (0xf9 < local_24) {
        *param_5 = 0xffffffff;
        *param_6 = 0xffffffff;
        return 0x0;
    }
    uVar1 = FUN_004a2edc();
    local_20 = FUN_0043a8a2((uVar1 % local_18 - local_18 / 0x2) + param_3);
    uVar1 = FUN_004a2edc();
    local_88 = ((local_20 & 0x1) == 0x0);
    local_1c = FUN_0043a8d5(local_20,(param_4 & 0xfffffffe) + (uVar1 % local_18 - local_18 / 0x2) * 0x2 + local_88)
    ;
    if (((*(*(&DAT_004d7d50 + local_20 * 0x4 + param_2 * 0x3890) + local_1c * 0x4) & 0xf) != 0x0) &&
        (iVar2 = FUN_0044ace5(param_2,local_20,local_1c,0x1), iVar2 == -0x1)) {
        local_60 = (undefined2)param_2;
        local_5e = (undefined2)local_20;
        local_5c = (undefined2)local_1c;
        uVar1 = FUN_0045af67(&local_30,param_3,param_4,local_20,local_1c,0x0);
        if ((uVar1 <= local_14) || (0x96 < local_24)) {
            *param_5 = local_20;
            *param_6 = local_1c;
            return 0x1;
        }
    }
    local_24 = local_24 + 0x1;
    if (local_24 % 0x32 == 0x0) {
        local_14 = local_14 + 0x2;
        local_18 = local_18 + 0x2;
    }
} while( true );
}



fn FUN_0041d83c()

{
    let local_14: *mut u32;

    for (local_14 = *DAT_005967c8; local_14 != 0x0; local_14 = *local_14) {
        if ((local_14 + 0xe) == 0x17) {
            *(local_14 + 0xb) = *(local_14 + 0xb) & 0xfd;
        }
    }
    return;
}



// WARNING: Could not reconcile some variable overlaps

fn FUN_0041d87d()

{
    let mut iVar1: i32;
    let mut pcVar2: String;
    u32 local_a0 [0x20];
    let mut local_20: i32;
    let local_1c: *mut u32;
    let local_18: *mut u32;
    let mut local_14: i32;

    FUN_00419085();
    local_1c = *DAT_005967c8;
    loop {
    if (local_1c == 0x0) {
        return;
    }
    local_18 = *local_1c;
    if (((((local_1c + 0xe) == 0x17) && ((*(local_1c + 0xb) & 0x2) == 0x0)) &&
        (*(local_1c + 0xe) >> 0x10 < 0x5)) &&
        (*(&DAT_004c74f8 + (*(local_1c + 0xe) >> 0x10) * 0x4) == 0x0)) {
        for (local_20 = 0x1; local_20 < 0x72; local_20 = local_20 + 0x1) {
            if ((*(&DAT_0058ad72 + local_20 * 0xda) != 0x0) &&
                ((((&DAT_00569c30)[(*(local_1c + 0xe) >> 0x10) * 0x1e22 + local_20 * 0x9] & 0x1) != 0x0 ||
                    (((&DAT_00569c30)[(*(local_1c + 0xe) >> 0x10) * 0x1e22 + local_20 * 0x9] & 0x4) != 0x0)))) {
                local_14 = *(local_1c + 0xe) >> 0x10;
                iVar1 = FUN_0041dc37(local_1c[0x2] >> 0x10,*(local_1c + 0xa) >> 0x10,
                                     *(local_1c + 0x6) >> 0x10,0x2);
                if (-0x1 < iVar1) {
                    if (local_14 < 0x5) {
                        pcVar2 = FUN_00499050(DAT_0059679c,0x73d1);
                        FUN_0049c2e0(local_a0,pcVar2);
                        FUN_0045518a(0x1 << ((byte)local_14 & 0x1f),0xffffffff,0x73d1,0xffffffff,local_a0,0xffffffff,0x0);
                    }
                    *(&DAT_004c74f8 + local_14 * 0x4) = 0x1;
                }
                break;
            }
        }
    }
    local_1c = local_18;
} while( true );
}



fn FUN_0041da19()

{
    let piVar1: *mut i32;;
    let mut iVar2: i32;
    let mut uVar3: u32;
    i32 local_1ec [0x72];
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    FUN_00419085();
    local_1c = 0x0;
    local_14 = 0x0;
    local_20 = 0xa;
    for (local_18 = 0x1; local_18 < 0x72; local_18 = local_18 + 0x1) {
        iVar2 = FUN_0046295d((&DAT_0058aca8 + local_18 * 0xda),0x0);
        if ((iVar2 != 0x320) && (*(&DAT_0058ad72 + local_18 * 0xda) == 0x0)) {
            if ((0x1 < *(&DAT_0058ad6e + local_18 * 0xda)) &&
                ((*(&DAT_0058ad6e + local_18 * 0xda) < 0xa && (uVar3 = FUN_004a2edc(), uVar3 % 0x14 == 0x0)))) {
                *(&DAT_0058ad6e + local_18 * 0xda) = *(&DAT_0058ad6e + local_18 * 0xda) + -0x1;
            }
            if ((*(&DAT_0058ad6e + local_18 * 0xda) < 0x6) && (*(&DAT_0058ad6e + local_18 * 0xda) < local_20)) {
                local_14 = local_18;
                local_20 = *(&DAT_0058ad6e + local_18 * 0xda);
            }
        }
    }
    if (local_14 == 0x0) {
        FUN_00419085();
    }
    else {
        local_24 = 0x0;
        for (local_18 = 0x1; local_18 < 0x72; local_18 = local_18 + 0x1) {
            if ((*(&DAT_0058ad72 + local_18 * 0xda) == 0x0) && (*(&DAT_0058ad6e + local_18 * 0xda) == local_20))
            {
                piVar1 = local_1ec + local_24;
                local_24 = local_24 + 0x1;
                *piVar1 = local_18;
            }
        }
        if (local_24 == 0x1) {
            local_1c = local_1ec[0];
        }
        else {
            uVar3 = FUN_004a2edc();
            local_1c = local_1ec[uVar3 % local_24];
        }
        FUN_004083b2(local_1c);
        FUN_00419085();
    }
    return;
}



fn FUN_0041dbe9() -> u32

{
    let mut local_14: i32;

    local_14 = 0x1;
    while( true ) {
        if (0x71 < local_14) {
            return 0x0;
        }
        if (*(&DAT_0058ad6e + local_14 * 0xda) == 0x0) break;
        local_14 = local_14 + 0x1;
    }
    return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0041dc37(param_1: i32,param_2: i32,param_3: i32,param_4: u32) -> i32

{
let mut local_14: i32;

local_14 = 0x0;
while( true ) {
if (_DAT_004c6160 <= local_14) {
local_14 = 0x0;
while( true ) {
if (0x3e7 < local_14) {
return -0x1;
}
if ((&DAT_004c616b)[local_14 * 0x4] == '\0') break;
local_14 = local_14 + 0x1;
}
(&DAT_004c6168)[local_14 * 0x4] = param_1;
(&DAT_004c6169)[local_14 * 0x4] = param_2;
(&DAT_004c616a)[local_14 * 0x4] = param_3;
(&DAT_004c616b)[local_14 * 0x4] = param_4;
if (_DAT_004c6160 <= local_14) {
_DAT_004c6160 = local_14 + 0x1;
}
return local_14;
}
if (((((*(&DAT_004c6168 + local_14 * 0x4) >> 0x18 & 0xfffffffeU) == param_4) &&
(*(&DAT_004c6165 + local_14 * 0x4) >> 0x18 == param_1)) &&
(*(&DAT_004c6166 + local_14 * 0x4) >> 0x18 == param_2)) &&
(*(&DAT_004c6167 + local_14 * 0x4) >> 0x18 == param_3)) break;
local_14 = local_14 + 0x1;
}
return -0x2;
}



fn FUN_0041dd5a(param_1: i32,param_2: i32,param_3: i32) -> *mut u32

{
    let puVar1: *mut u32;
    let mut local_1c: i32;

    puVar1 = FUN_00481784(param_1,param_2,param_3);
    if ((puVar1 != 0x0) && ((puVar1 + 0xe) == 0x17)) {
        for (local_1c = 0x1; local_1c < 0x72; local_1c = local_1c + 0x1) {
            if (*(&DAT_0058ad72 + local_1c * 0xda) != 0x0) {
                if (((&DAT_00569c30)[(*(puVar1 + 0xe) >> 0x10) * 0x1e22 + local_1c * 0x9] & 0x1) != 0x0) {
                    return puVar1;
                }
                if (((&DAT_00569c30)[(*(puVar1 + 0xe) >> 0x10) * 0x1e22 + local_1c * 0x9] & 0x4) != 0x0) {
                    return puVar1;
                }
            }
        }
    }
    return 0x0;
}



fn FUN_0041de20()

{
    FUN_00420c5e(&DAT_004c7510);
    return;
}



fn FUN_0041de42()

{
    let mut iVar1: i32;
    let mut uVar2: u32;
    i32 **local_a8;
    let mut local_a4: i32;
    let mut local_a0: i32;
    let mut local_9c: u32;
    i32 **local_98;
    i32 **local_94;
    let mut local_90: u32;
    let mut local_8c: u32;
    let mut local_88: u32;
    let mut local_84: u32;
    i32 **local_80;
    i32 **local_7c;
    let mut local_78: i32;
    let mut local_74: u32;
    i32 **local_70;
    i32 **local_6c;
    let mut local_68: i32;
    let mut local_64: u32;
    i32 **local_60;
    i32 **local_5c;
    i32 **local_58;
    i32 ***local_54;
    let local_50: *mut u32;
    i32 **local_4c;
    let local_48: u8 [0x20];
    i32 **local_28;
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_1c: u32;
    let mut local_18: u32;

    if (DAT_004c9754 != 0x8) {
        local_1c = 0xffffffff;
        local_18 = 0xffffffff;
        local_20 = 0x0;
        FUN_004133bd(&DAT_004c9318,&DAT_004c931c);
        FUN_00420e3e();
        FUN_00420edd(&DAT_004c8adc);
        loop {
            local_24 = 0x0;
            local_50 = FUN_004840cd(&local_4c,&local_28,-0x1);
            while (local_28 != (i32 **)0x0) {
                if ((((local_28 + 0x27) == '5') || ((local_28 + 0x27) == ',')) &&
                    (*(local_28 + 0x23) >> 0x18 == DAT_004c9754)) {
                    local_60 = local_28 + 0x8;
                    local_64 = *(local_28 + 0x3a) & 0x1;
                    local_5c = local_60;
                    if (((local_64 != 0x0) && ((local_28 + 0x12) == '\x01')) && ((local_28 + 0x2f) != '\0')
                    ) {
                        if (((local_28 + 0x41) == -0x1) || ((local_28 + 0x42) == -0x1)) {
                            (local_28 + 0x12) = 0x4;
                        }
                        else {
                            local_68 = *(local_28 + 0x46) >> 0x18;
                            local_70 = local_28 + 0x8;
                            local_74 = local_70 & 0xffff0000 | local_70;
                            local_6c = local_70;
                            iVar1 = FUN_0041e4f1(local_70,local_68);
                            if (iVar1 != 0x0) {
                                local_78 = *(local_28 + 0x46) >> 0x18;
                                local_80 = local_28 + 0x8;
                                local_84 = local_80 & 0xffff0000 | local_80;
                                local_88 = *(*(local_28 + 0x42) * 0x4 +
                                    *(&DAT_004d7d50 +
                                        *(local_28 + 0x41) * 0x4 +
                                        local_80 * 0x3890)) & 0xf;
                                local_8c = *(local_28 + 0x42);
                                local_90 = *(local_28 + 0x41);
                                local_98 = local_28 + 0x8;
                                local_9c = local_98 & 0xffff0000 | local_98;
                                local_94 = local_98;
                                local_7c = local_80;
                                iVar1 = FUN_0043a96a(local_98,local_90,local_8c,local_88,local_78);
                                if (iVar1 != 0x0) {
                                    FUN_0041361b(local_28);^
                                    // goto LAB_0041deb6;
                                }
                            }
                            (local_28 + 0x12) = 0x0;
                        }
                    }
                }
                LAB_0041deb6:
                    local_28 = local_4c;
                local_54 = &local_4c;
                local_58 = local_4c;
                if (local_4c != (i32 **)0x0) {
                    local_4c = (i32 **)*local_4c;
                }
            }
            for (local_a0 = 0x0; local_a0 < 0x28; local_a0 = local_a0 + 0x1) {
                local_a4 = 0x0;
                FUN_004a0430(local_48,0x0,0x20);
                FUN_004840cd(&local_a8,&local_28,local_a0);
                while ((local_28 != (i32 **)0x0 && ((local_28 + 0x8) == local_a0))) {
                if ((((local_28 + 0x27) == '5') || ((local_28 + 0x27) == ',')) &&
                ((((*(local_28 + 0x23) >> 0x18 == DAT_004c9754 &&
                ((*(local_28 + 0x3a) & 0x1) != 0x0)) && ((local_28 + 0x12) == '\0')) &&
                ((local_28 + 0x2f) != '\0')))) {
                while (uVar2 = FUN_0041e418((local_28 + 0x8),local_48), uVar2 != 0xffffffff) {
                iVar1 = FUN_0041ec1d(local_28,uVar2);
                if (iVar1 != 0x0) {
                if ((DAT_004c930c < 0x9) || (0x4 < local_20)) {
                FUN_0041361b(local_28);
                }
                else {
                iVar1 = FUN_0043a96a((local_28 + 0x8),*(local_28 + 0x41),
                *(local_28 + 0x42),
                *(*(local_28 + 0x42) * 0x4 +
                *(&DAT_004d7d50 +
                *(local_28 + 0x41) * 0x4 +
                (local_28 + 0x8) * 0x3890)) & 0xf,
                *(local_28 + 0x46) >> 0x18);
                if (iVar1 != 0x0) {
                local_24 = 0x1;
                }
                }
                break;
                }
                local_48[uVar2] = 0x1;
                }
                if (uVar2 == 0xffffffff) {
                if ((*(*(&DAT_00582938 +
                (*(local_28 + 0x25) >> 0x18) * 0x4 + (local_28[0x9] >> 0x18) * 0x18
                ) + 0xa9) == 0x0) || (0x3 < local_a4)) {
                FUN_00488f5c(local_28);
                }
                else {
                (local_28 + 0x12) = 0x2;
                local_a4 = local_a4 + 0x1;
                }
                }
                }
                local_28 = local_a8;
                if (local_a8 != (i32 **)0x0) {
                local_a8 = (i32 **)*local_a8;
                }
                }
                FUN_0048418d(&local_a8);
            }
            local_20 = local_20 + 0x1;
            FUN_0048418d(&local_4c);
        } while ((local_24 != 0x0) && (local_20 < 0x8));
        FUN_0049af50(DAT_004c931c);
        FUN_0049af50((DAT_004c9318 + -0x4));
    }
    return;
}



fn FUN_0041e418(param_1: i32,param_2: i32) -> i32

{
let mut iVar1: i32;
let mut local_20: i32;
let mut local_1c: i32;
let mut local_18: i32;
let mut local_14: i32;

if (DAT_004c9754 == 0x7) {
if ((param_2 + 0x7) == '\0') {
local_20 = 0x7;
}
else {
local_20 = -0x1;
}
}
else {
if (DAT_004c9754 == 0x8) {
if ((param_2 + 0x8) == '\0') {
local_20 = 0xa;
}
else {
local_20 = -0x1;
}
}
else {
local_1c = 0x0;
local_18 = 0x0;
for (local_14 = 0x0; local_14 < 0x20; local_14 = local_14 + 0x1) {
if (((param_2 + local_14) == '\0') && (iVar1 = FUN_0041e4f1(param_1,local_14), local_18 < iVar1)) {
local_1c = local_14;
local_18 = iVar1;
}
}
if (local_18 == 0x0) {
local_20 = -0x1;
}
else {
local_20 = local_1c;
}
}
}
return local_20;
}



fn FUN_0041e4f1(param_1: i32,param_2: i32) -> i32

{
let mut iVar1: i32;
let mut iVar2: i32;
let mut local_9c: i32;
let mut local_98: i32;
let mut local_90: i32;
let mut local_8c: i32;
let mut local_84: i32;
let mut local_80: i32;
let mut local_78: i32;
let mut local_74: i32;
let mut local_6c: i32;
let mut local_68: i32;
let mut local_60: i32;
let mut local_5c: i32;
let mut local_54: i32;
let mut local_50: i32;
let mut local_48: i32;
let mut local_44: i32;
let mut local_34: i32;
let mut local_28: i32;
let mut local_20: i32;
let mut local_1c: i32;
let mut local_14: i32;

if (((&DAT_00569c30)[DAT_004c9754 * 0x1e22 + *(&DAT_00583220 + param_2 * 0x50) * 0x9] & 0x1) == 0x0) {
local_14 = 0x0;
}
else {
switch(param_2) {
case 0x0:
iVar2 = FUN_00482588(param_1,0x0);
if (iVar2 == 0x0) {
local_14 = 0x15;
}
else {
local_14 = 0x0;
}
break;
case 0x1:
iVar2 = FUN_00482588(param_1,0x1);
if (iVar2 == 0x0) {
local_14 = 0x17;
}
else {
local_14 = 0x0;
}
break;
case 0x2:
case 0x4:
case 0x7:
case 0xa:
case 0xf:
case 0x10:
case 0x1a:
case 0x1b:
case 0x1c:
case 0x1d:
case 0x1e:
case 0x1f:
local_14 = 0x0;
break;
case 0x3:
if (*(&DAT_004c7648 + param_1 * 0x80) * 0xc - *(&DAT_004c8a3c + param_1 * 0x4) == 0x0 ||
*(&DAT_004c7648 + param_1 * 0x80) * 0xc < *(&DAT_004c8a3c + param_1 * 0x4)) {
if (*(&DAT_004c7648 + param_1 * 0x80) == 0x0) {
local_14 = 0x46;
}
else {
local_14 = 0x23;
}
}
else {
local_14 = 0x0;
}
break;
case 0x5:
if (DAT_004c9760 == 0x0) {
local_54 = (&DAT_004c8b00)[param_1 * 0xd];
}
else {
local_54 = DAT_004c8b00;
}
if (local_54 < 0x19) {
local_50 = (0x19 - local_54) / 0xa + 0x1;
}
else {
local_50 = 0x0;
}
local_14 = local_50;
break;
case 0x6:
if (DAT_004c9760 == 0x0) {
local_60 = (&DAT_004c8af8)[param_1 * 0xd];
}
else {
local_60 = DAT_004c8af8;
}
if (local_60 < 0x32) {
local_5c = (0x32 - local_60) / 0x5 + 0x1;
}
else {
local_5c = 0x0;
}
local_14 = local_5c;
break;
case 0x8:
if (DAT_004c9760 == 0x0) {
local_6c = (&DAT_004c8afc)[param_1 * 0xd];
}
else {
local_6c = DAT_004c8afc;
}
if (local_6c < 0x28) {
local_68 = (0x28 - local_6c) / 0x5 + 0x1;
}
else {
local_68 = 0x0;
}
local_14 = local_68;
break;
case 0x9:
if (DAT_004c9760 == 0x0) {
local_78 = (&DAT_004c8af4)[param_1 * 0xd];
}
else {
local_78 = DAT_004c8af4;
}
if (local_78 < 0x19) {
local_74 = (0x19 - local_78) / 0x5 + 0x1;
}
else {
local_74 = 0x0;
}
local_14 = local_74;
break;
case 0xb:
if (DAT_004c9760 == 0x0) {
local_84 = (&DAT_004c8af0)[param_1 * 0xd];
}
else {
local_84 = DAT_004c8af0;
}
if (local_84 < 0x32) {
local_80 = (0x32 - local_84) / 0x5 + 0x1;
}
else {
local_80 = 0x0;
}
local_14 = local_80;
break;
case 0xc:
if (DAT_004c9760 == 0x0) {
local_90 = (&DAT_004c8b0c)[param_1 * 0xd];
}
else {
local_90 = DAT_004c8b0c;
}
if (local_90 < 0x1) {
local_8c = 0x2 - local_90;
}
else {
local_8c = 0x0;
}
local_14 = local_8c;
break;
case 0xd:
if (*(&DAT_004c7670 + param_1 * 0x80) * 0x14 - *(&DAT_004c8a3c + param_1 * 0x4) == 0x0 ||
*(&DAT_004c7670 + param_1 * 0x80) * 0x14 < *(&DAT_004c8a3c + param_1 * 0x4)) {
if (*(&DAT_004c7670 + param_1 * 0x80) == 0x0) {
local_14 = 0x1e;
}
else {
local_14 = 0xf;
}
}
else {
local_14 = 0x0;
}
break;
case 0xe:
if (*(&DAT_004c7674 + param_1 * 0x80) * 0x1e - *(&DAT_004c8a3c + param_1 * 0x4) == 0x0 ||
*(&DAT_004c7674 + param_1 * 0x80) * 0x1e < *(&DAT_004c8a3c + param_1 * 0x4)) {
if (*(&DAT_004c7674 + param_1 * 0x80) == 0x0) {
local_14 = 0x96;
}
else {
local_14 = 0x4b;
}
}
else {
local_14 = 0x0;
}
break;
case 0x11:
iVar2 = FUN_00482588(param_1,0x11);
if (iVar2 == 0x0) {
local_14 = 0x19;
}
else {
local_14 = 0x0;
}
break;
case 0x12:
if (DAT_004c9760 == 0x0) {
iVar2 = (&DAT_004c8ae4)[param_1 * 0xd];
iVar1 = (&DAT_004c8ae8)[param_1 * 0xd];
local_28 = (&DAT_004c8b08)[param_1 * 0xd];
}
else {
local_28 = DAT_004c8b08;
iVar2 = DAT_004c8ae4;
iVar1 = DAT_004c8ae8;
}
local_34 = 0x0;
if (0xc7 < iVar2) {
local_34 = (0xc8 - iVar2) / 0xa + 0x1;
}
if (0x63 < iVar1) {
local_34 = local_34 + (0x64 - iVar1) / 0x5 + 0x1;
}
if (0x9 < local_28) {
local_34 = local_34 + (0xb - local_28);
}
local_14 = local_34;
break;
case 0x13:
if (DAT_004c9760 == 0x0) {
local_48 = (&DAT_004c8ae0)[param_1 * 0xd];
}
else {
local_48 = DAT_004c8ae0;
}
if (local_48 < 0x32) {
local_44 = (0x32 - local_48) / 0xa + 0x1;
}
else {
local_44 = 0x0;
}
local_14 = local_44;
break;
case 0x14:
if (DAT_004c9760 == 0x0) {
local_9c = (&DAT_004c8b04)[param_1 * 0xd];
}
else {
local_9c = DAT_004c8b04;
}
if (local_9c < 0xa) {
local_98 = (0xa - local_9c) / 0x5 + 0x1;
}
else {
local_98 = 0x0;
}
local_14 = local_98;
break;
default:
local_14 = 0x0;
break;
case 0x16:
if (*(&DAT_004c7694 + param_1 * 0x80) == 0x0) {
local_14 = 0xc;
}
else {
local_14 = 0x0;
}
break;
case 0x17:
if (*(&DAT_004c7698 + param_1 * 0x80) * 0xc - *(&DAT_004c8a3c + param_1 * 0x4) == 0x0 ||
*(&DAT_004c7698 + param_1 * 0x80) * 0xc < *(&DAT_004c8a3c + param_1 * 0x4)) {
if (*(&DAT_004c7698 + param_1 * 0x80) == 0x0) {
local_14 = 0x28;
}
else {
local_14 = 0x14;
}
}
else {
local_14 = 0x0;
}
break;
case 0x18:
if (((&DAT_00569c30)[DAT_005839f0 * 0x9 + DAT_004c9754 * 0x1e22] & 0x1) != 0x0) {
return 0x0;
}
case 0x19:
if (DAT_004c9760 == 0x0) {
local_20 = (&DAT_004c8adc)[param_1 * 0xd];
}
else {
local_20 = DAT_004c8adc;
}
if (local_20 < 0x12c) {
local_1c = (0x12c - local_20) / 0x5 + 0x1;
}
else {
local_1c = 0x0;
}
local_14 = local_1c;
}
}
return local_14;
}



fn FUN_0041ec1d(param_1: i32,param_2: u32) -> u32

{
    let mut local_18: u32;

    switch(param_2) {
    case 0x0:
        case 0x1:
        case 0x3:
        case 0x4:
        case 0xe:
        local_18 = FUN_00420343(param_1,(char)param_2);
    break;
    default:
        local_18 = 0x0;
    break;
    case 0x5:
        case 0x6:
        case 0x8:
        case 0x9:
        case 0xb:
        case 0xc:
        case 0xd:
        case 0x14:
        case 0x16:
        case 0x17:
        local_18 = FUN_0041fa61(param_1,(char)param_2);
    break;
    case 0x7:
        local_18 = FUN_0042082d(param_1);
    break;
    case 0xa:
        local_18 = 0x0;
    break;
    case 0x11:
        local_18 = FUN_0042074b();
    break;
    case 0x12:
        case 0x13:
        case 0x18:
        case 0x19:
        local_18 = FUN_0041ed44(param_1,param_2);
}
    return local_18;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0041ed44(param_1: i32,param_2: u32) -> u32

{
    ushort uVar1;
    let mut bVar2: bool;
    let mut iVar3: i32;
    undefined3 extraout_var;
    let mut local_114: u32;
    let mut local_110: u32;
    let mut local_10c: u32;
    let mut local_108: u32;
    ushort *local_104;
    ushort *local_100;
    let mut local_fc: u32;
    let mut local_f8: u32;
    let mut local_f4: u32;
    let mut local_f0: u32;
    let mut local_ec: u32;
    let mut local_e8: u32;
    ushort *local_e4;
    ushort *local_e0;
    let mut local_dc: u32;
    let mut local_d8: u32;
    let mut local_d4: u32;
    let mut local_d0: u32;
    let mut local_cc: i32;
    let mut local_c8: u32;
    let mut local_c4: u32;
    let mut local_c0: u32;
    let mut local_bc: u32;
    ushort *local_b8;
    ushort *local_b4;
    let mut local_b0: u32;
    let mut local_ac: u32;
    let mut local_a8: u32;
    let mut local_a4: u32;
    let mut local_a0: i32;
    let mut local_9c: u32;
    let mut local_98: u32;
    let mut local_94: u32;
    let mut local_90: u32;
    ushort *local_8c;
    ushort *local_88;
    let mut local_84: u32;
    let mut local_80: u32;
    let mut local_7c: u32;
    let mut local_78: u32;
    let mut local_74: i32;
    let mut local_70: u32;
    let mut local_6c: u32;
    let mut local_68: u32;
    let mut local_64: u32;
    let mut local_60: u32;
    let mut local_5c: u32;
    let mut local_58: i32;
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
    let local_24: *mut u32;
    let mut local_20: i32;
    ushort *local_1c;
    ushort *local_18;
    let mut local_14: u32;

    local_1c = (param_1 + 0x20);
    local_14 = local_1c & 0xffff0000 | *local_1c;
    local_20 = *local_1c;
    local_18 = local_1c;
    FUN_004a0430(DAT_004c931c,0x0,0x1842);
    for (local_24 = (&DAT_005b89f8 + local_20 * 0x4);
        (local_24 != 0x0 && (*(local_24 + 0x6) >> 0x10 == local_20));
        local_24 = *local_24) {
        if (*(&DAT_004be8b0 + (local_24[0x3] >> 0x10) * 0x4) == 0x0) {
            FUN_0041f4d4(DAT_004c9318,local_24,*(&DAT_00583210 + param_2 * 0x50));
        }
    }
    for (local_24 = (&DAT_005b89f8 + local_20 * 0x4);
        (local_24 != 0x0 && (*(local_24 + 0x6) >> 0x10 == local_20));
        local_24 = *local_24) {
        if ((*(local_24 + 0xe) >> 0x10 != DAT_004c9754) &&
            (*(&DAT_004be930 + (local_24[0x3] >> 0x10) * 0x4) == 0x0)) {
            FUN_0041f524(0x4,local_24[0x2] >> 0x10,*(local_24 + 0xa) >> 0x10,DAT_004c9318,0x1);
        }
    }
    FUN_00431d0a(&DAT_005967b8);
    DAT_005967bc = param_1;
    local_30 = param_1 + 0x20;
    local_34 = local_30 & 0xffff0000 | (param_1 + 0x24);
    local_38 = SEXT24((param_1 + 0x24));
    local_40 = param_1 + 0x20;
    local_44 = local_40 & 0xffff0000 | (param_1 + 0x22);
    local_48 = SEXT24((param_1 + 0x22));
    local_50 = param_1 + 0x20;
    local_54 = local_50 & 0xffff0000 | (param_1 + 0x24);
    local_58 = (param_1 + 0x24);
    local_60 = param_1 + 0x20;
    local_64 = local_60 & 0xffff0000 | (param_1 + 0x22);
    local_5c = local_60;
    local_4c = local_50;
    local_3c = local_40;
    local_2c = local_30;
    FUN_0045af67(&DAT_005967b8,(param_1 + 0x22),local_58,local_48,local_38,0x18);
    DAT_004c9314 = param_2;
    DAT_004c930c = 0x3e7;
    DAT_004c9310 = 0x0;
    _DAT_004c92fc = 0x8;
    _DAT_004c9300 = 0x0;
    FUN_00419085();
    FUN_0042feac();
    local_6c = param_1 + 0x20;
    local_70 = local_6c & 0xffff0000 | (param_1 + 0x24);
    local_74 = (param_1 + 0x24);
    local_7c = param_1 + 0x20;
    local_80 = local_7c & 0xffff0000 | (param_1 + 0x22);
    local_84 = SEXT24((param_1 + 0x22));
    local_8c = (param_1 + 0x20);
    local_90 = local_8c & 0xffff0000 | *local_8c;
    local_88 = local_8c;
    local_78 = local_7c;
    local_68 = local_6c;
    FUN_00482843(*local_8c,local_84,local_74,0x8,FUN_0041f600);
    DAT_004c9310 = DAT_004c9310 + 0x4;
    _DAT_004c92fc = 0x10;
    _DAT_004c9300 = 0x9;
    FUN_00419085();
    FUN_0042feac();
    local_98 = param_1 + 0x20;
    local_9c = local_98 & 0xffff0000 | (param_1 + 0x24);
    local_a0 = (param_1 + 0x24);
    local_a8 = param_1 + 0x20;
    local_ac = local_a8 & 0xffff0000 | (param_1 + 0x22);
    local_b0 = SEXT24((param_1 + 0x22));
    local_b8 = (param_1 + 0x20);
    local_bc = local_b8 & 0xffff0000 | *local_b8;
    local_b4 = local_b8;
    local_a4 = local_a8;
    local_94 = local_98;
    FUN_00482843(*local_b8,local_b0,local_a0,0x10,FUN_0041f600);
    DAT_004c9310 = DAT_004c9310 + 0x4;
    _DAT_004c92fc = 0x18;
    _DAT_004c9300 = 0x11;
    FUN_00419085();
    FUN_0042feac();
    local_c4 = param_1 + 0x20;
    local_c8 = local_c4 & 0xffff0000 | (param_1 + 0x24);
    local_cc = (param_1 + 0x24);
    local_d4 = param_1 + 0x20;
    local_d8 = local_d4 & 0xffff0000 | (param_1 + 0x22);
    local_dc = SEXT24((param_1 + 0x22));
    local_e4 = (param_1 + 0x20);
    local_e8 = local_e4 & 0xffff0000 | *local_e4;
    local_e0 = local_e4;
    local_d0 = local_d4;
    local_c0 = local_c4;
    FUN_00482843(*local_e4,local_dc,local_cc,0x18,FUN_0041f600);
    if (DAT_004c930c < 0x3e7) {
        local_f0 = DAT_004c9304;
        local_f4 = DAT_004c9308;
        local_f8 = DAT_004c9308;
        local_fc = DAT_004c9304;
        local_104 = (param_1 + 0x20);
        local_ec = local_104 & 0xffff0000 | *local_104;
        local_100 = local_104;
        iVar3 = FUN_00420dbb(&DAT_004c7510,*local_104,DAT_004c9304,DAT_004c9308);
        if (iVar3 == 0x0) {
            uVar1 = (param_1 + 0x20);
            local_108 = (param_1 + 0x20) & 0xffff0000 | uVar1;
            bVar2 = FUN_0041f754(uVar1,local_f0,local_f4,0x4,&local_114,&local_110,&local_10c);
            if (CONCAT31(extraout_var,bVar2) != 0x0) {
                local_f0 = local_114;
                local_f4 = local_110;
                param_2 = local_10c;
            }
            (param_1 + 0x48) = 0x1;
            (param_1 + 0x49) = param_2;
            (param_1 + 0x41) = local_f0;
            (param_1 + 0x42) = local_f4;
            iVar3 = FUN_0041f6d8((param_1 + 0x20),local_f0,local_f4,param_2);
            if (param_2 < 0x13) {
                if ((param_2 == 0x12) && (iVar3 < 0xf)) {
                    FUN_00419085();
                    return 0x0;
                }
            }
            else {
                if (param_2 < 0x14) {
                    if (iVar3 < 0xf) {
                        FUN_00419085();
                        return 0x0;
                    }
                }
                else {
                    if ((0x17 < param_2) && (((param_2 < 0x19 || (param_2 == 0x19)) && (iVar3 < 0x19)))) {
                        FUN_00419085();
                        return 0x0;
                    }
                }
            }
            FUN_00420cb1(&DAT_004c7510,(char)*(param_1 + 0x20),(char)local_f0,(char)local_f4);
            local_28 = 0x1;
        }
        else {
            local_28 = 0x0;
        }
    }
    else {
        local_28 = 0x0;
    }
    return local_28;
}



fn FUN_0041f4d4(param_1: i32,param_2: i32,param_3: i32)

{
    FUN_0041f524(*(&DAT_00583210 + (*(param_2 + 0xc) >> 0x10) * 0x50) + param_3,
                 *(param_2 + 0x8) >> 0x10,*(param_2 + 0xa) >> 0x10,param_1,0x1);
    return;
}



fn FUN_0041f524(param_1: i32,param_2: u32,param_3: i32,param_4: i32,undefined2 param_5)

{
    let mut uVar1: u32;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut local_30: u32;
    let mut local_14: i32;

    for (local_14 = param_2 - param_1; local_14 <= (param_2 + param_1); local_14 = local_14 + 0x1) {
        uVar1 = FUN_0043a8a2(local_14);
        for (local_30 = (param_3 + param_1 * -0x2 + (uVar1 & 0x1)) - (param_2 & 0x1);
            local_30 <= param_3 + param_1 * 0x2; local_30 = local_30 + 0x2) {
            uVar2 = FUN_0043a8d5(uVar1,local_30);
            iVar3 = FUN_0044a87f(param_2,param_3,uVar1,uVar2);
            if (iVar3 <= param_1) {
                *(*(uVar1 * 0x4 + param_4) + uVar2 * 0x2) = param_5;
            }
        }
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0041f600(param_1: u32,param_2: u32,param_3: i32)

{
    let mut iVar1: i32;

    if ((((*(*(param_2 * 0x4 + DAT_005b2d50) + param_3) <= _DAT_004c92fc) &&
        (_DAT_004c9300 <= *(*(param_2 * 0x4 + DAT_005b2d50) + param_3))) &&
        ((*(param_2 * 0x4 + DAT_004c9318) + param_3 * 0x2) == 0x0)) &&
        (iVar1 = FUN_0041f6d8(param_1,param_2,param_3,DAT_004c9314), DAT_004c9310 < iVar1)) {
        DAT_004c9304 = param_2;
        DAT_004c9308 = param_3;
        DAT_004c930c = *(*(param_2 * 0x4 + DAT_005b2d50) + param_3);
        DAT_004c9310 = iVar1;
    }
    return;
}



fn FUN_0041f6d8(param_1: u32,param_2: u32,param_3: i32,param_4: i32) -> i32

{
i32 local_50 [0xe];
let mut local_18: i32;
let mut local_14: i32;

local_18 = 0x0;
FUN_00466271(param_1,param_2,param_3,param_4,local_50);
for (local_14 = 0x0; local_14 < 0xd; local_14 = local_14 + 0x1) {
local_18 = local_18 + local_50[local_14];
}
return local_18 + local_50[4] * 0x4 + local_50[3] * 0x4 + local_50[11] * 0x20;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

bool
FUN_0041f754(param_1: u32,param_2: u32,param_3: i32,param_4: i32,param_5: *mut u32,param_6: *mut u32,
param_7: *mut u32)

{
let mut uVar1: u32;
let mut iVar2: i32;
let mut local_14: i32;

uVar1 = DAT_004c930c;
local_14 = 0x3e7;
_DAT_004c92fc = 0x64;
_DAT_004c9300 = 0x0;
if (((&DAT_00569c30)[DAT_005839f0 * 0x9 + DAT_004c9754 * 0x1e22] & 0x1) == 0x0) {
if (((&DAT_00569c30)[DAT_005839a0 * 0x9 + DAT_004c9754 * 0x1e22] & 0x1) != 0x0) {
DAT_004c930c = 0x3e7;
DAT_004c9310 = 0x0;
DAT_004c9314 = 0x18;
FUN_00482843(param_1,param_2,param_3,param_4,FUN_0041f600);
iVar2 = (DAT_004c9310 * 0x3e8) / 0x3c;
if (0x3e7 < iVar2) {
*param_7 = 0x18;
*param_5 = DAT_004c9304;
*param_6 = DAT_004c9308;
local_14 = iVar2;
}
}
}
else {
DAT_004c930c = 0x3e7;
DAT_004c9310 = 0x0;
DAT_004c9314 = 0x19;
FUN_00482843(param_1,param_2,param_3,param_4,FUN_0041f600);
iVar2 = (DAT_004c9310 * 0x3e8) / 0x3c;
if (0x3e7 < iVar2) {
*param_7 = 0x19;
*param_5 = DAT_004c9304;
*param_6 = DAT_004c9308;
local_14 = iVar2;
}
}
if (((&DAT_00569c30)[DAT_00583810 * 0x9 + DAT_004c9754 * 0x1e22] & 0x1) != 0x0) {
DAT_004c930c = 0x3e7;
DAT_004c9310 = 0x0;
DAT_004c9314 = 0x13;
FUN_00482843(param_1,param_2,param_3,param_4,FUN_0041f600);
iVar2 = (DAT_004c9310 * 0x3e8) / 0x28;
if (local_14 < iVar2) {
*param_7 = 0x13;
*param_5 = DAT_004c9304;
*param_6 = DAT_004c9308;
local_14 = iVar2;
}
}
if (((&DAT_00569c30)[DAT_005837c0 * 0x9 + DAT_004c9754 * 0x1e22] & 0x1) != 0x0) {
DAT_004c930c = 0x3e7;
DAT_004c9310 = 0x0;
DAT_004c9314 = 0x12;
FUN_00482843(param_1,param_2,param_3,param_4,FUN_0041f600);
iVar2 = (DAT_004c9310 * 0x3e8) / 0x32;
if (local_14 < iVar2) {
*param_7 = 0x12;
*param_5 = DAT_004c9304;
*param_6 = DAT_004c9308;
local_14 = iVar2;
}
}
if (0x3e7 < local_14) {
FUN_00419085();
DAT_004c930c = 0x1;
uVar1 = DAT_004c930c;
}
DAT_004c930c = uVar1;
return 0x3e7 < local_14;
}



// WARNING: Could not reconcile some variable overlaps

fn FUN_0041fa61(param_1: i32,undefined param_2) -> u32

{
    ushort uVar1;
    let mut bVar2: bool;
    let mut iVar3: i32;
    undefined3 extraout_var;
    let mut local_114: u32;
    let mut local_110: u32;
    let mut local_10c: u32;
    let mut local_108: u32;
    let mut local_104: u32;
    ushort *local_100;
    ushort *local_fc;
    let mut local_f8: u32;
    let mut local_f4: u32;
    let mut local_f0: u32;
    let mut local_ec: u32;
    let mut local_e8: u32;
    ushort *local_e4;
    ushort *local_e0;
    let mut local_dc: u32;
    let mut local_d8: u32;
    let mut local_d4: u32;
    let mut local_d0: i32;
    let mut local_cc: u32;
    let mut local_c8: u32;
    let mut local_c4: u32;
    let mut local_c0: u32;
    let mut local_bc: u32;
    ushort *local_b8;
    ushort *local_b4;
    let mut local_b0: u32;
    let mut local_ac: u32;
    let mut local_a8: u32;
    let mut local_a4: i32;
    let mut local_a0: u32;
    let mut local_9c: u32;
    let mut local_98: u32;
    let mut local_94: u32;
    let mut local_90: u32;
    ushort *local_8c;
    ushort *local_88;
    let mut local_84: u32;
    let mut local_80: u32;
    let mut local_7c: u32;
    let mut local_78: u32;
    let mut local_74: i32;
    let mut local_70: u32;
    let mut local_6c: u32;
    let mut local_68: u32;
    let mut local_64: u32;
    let mut local_60: u32;
    let mut local_5c: u32;
    let mut local_58: i32;
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
    let local_24: *mut u32;
    let mut local_20: i32;
    ushort *local_1c;
    ushort *local_18;
    let mut local_14: u32;

    local_1c = (param_1 + 0x20);
    local_14 = local_1c & 0xffff0000 | *local_1c;
    local_20 = *local_1c;
    local_18 = local_1c;
    FUN_004a0430(DAT_004c931c,0x0,0x1842);
    for (local_24 = (&DAT_005b89f8 + local_20 * 0x4);
        (local_24 != 0x0 && (*(local_24 + 0x6) >> 0x10 == local_20));
        local_24 = *local_24) {
        if (*(&DAT_004be930 + (local_24[0x3] >> 0x10) * 0x4) == 0x0) {
            FUN_0041f524(*(&DAT_00583210 + (local_24[0x3] >> 0x10) * 0x50) + 0x2,local_24[0x2] >> 0x10,
                         *(local_24 + 0xa) >> 0x10,DAT_004c9318,0x2);
        }
    }
    for (local_24 = (&DAT_005b89f8 + local_20 * 0x4);
        (local_24 != 0x0 && (*(local_24 + 0x6) >> 0x10 == local_20));
        local_24 = *local_24) {
        if (*(&DAT_004be8b0 + (local_24[0x3] >> 0x10) * 0x4) == 0x0) {
            FUN_0041f524(*(&DAT_00583210 + (local_24[0x3] >> 0x10) * 0x50),local_24[0x2] >> 0x10,
                         *(local_24 + 0xa) >> 0x10,DAT_004c9318,0x1);
        }
    }
    local_24 = (&DAT_005b89f8 + local_20 * 0x4);
    while( true ) {
        if ((local_24 == 0x0) || (*(local_24 + 0x6) >> 0x10 != local_20))^ // goto LAB_0041fc1d;
        if ((local_24 + 0xe) == 0x11) break;
        local_24 = *local_24;
    }
    if (*(local_24 + 0xe) >> 0x10 == DAT_004c9754) {
        FUN_0041f524(0x1,local_24[0x2] >> 0x10,*(local_24 + 0xa) >> 0x10,DAT_004c9318,0x1);
    }
    LAB_0041fc1d:
    for (local_24 = (&DAT_005b89f8 + local_20 * 0x4);
        (local_24 != 0x0 && (*(local_24 + 0x6) >> 0x10 == local_20));
        local_24 = *local_24) {
        if ((*(local_24 + 0xe) >> 0x10 != DAT_004c9754) &&
            (*(&DAT_004be930 + (local_24[0x3] >> 0x10) * 0x4) == 0x0)) {
            FUN_0041f524(0x4,local_24[0x2] >> 0x10,*(local_24 + 0xa) >> 0x10,DAT_004c9318,0x1);
        }
    }
    FUN_00431d0a(&DAT_005967b8);
    DAT_005967bc = param_1;
    local_30 = param_1 + 0x20;
    local_34 = local_30 & 0xffff0000 | (param_1 + 0x24);
    local_38 = SEXT24((param_1 + 0x24));
    local_40 = param_1 + 0x20;
    local_44 = local_40 & 0xffff0000 | (param_1 + 0x22);
    local_48 = SEXT24((param_1 + 0x22));
    local_50 = param_1 + 0x20;
    local_54 = local_50 & 0xffff0000 | (param_1 + 0x24);
    local_58 = (param_1 + 0x24);
    local_60 = param_1 + 0x20;
    local_64 = local_60 & 0xffff0000 | (param_1 + 0x22);
    local_5c = local_60;
    local_4c = local_50;
    local_3c = local_40;
    local_2c = local_30;
    FUN_0045af67(&DAT_005967b8,(param_1 + 0x22),local_58,local_48,local_38,0x18);
    DAT_004c930c = 0x19;
    FUN_00419085();
    FUN_0042feac();
    local_6c = param_1 + 0x20;
    local_70 = local_6c & 0xffff0000 | (param_1 + 0x24);
    local_74 = (param_1 + 0x24);
    local_7c = param_1 + 0x20;
    local_80 = local_7c & 0xffff0000 | (param_1 + 0x22);
    local_84 = SEXT24((param_1 + 0x22));
    local_8c = (param_1 + 0x20);
    local_90 = local_8c & 0xffff0000 | *local_8c;
    local_88 = local_8c;
    local_78 = local_7c;
    local_68 = local_6c;
    FUN_00482843(*local_8c,local_84,local_74,0x18,FUN_00420142);
    if (DAT_004c930c == 0x19) {
        local_9c = param_1 + 0x20;
        local_a0 = local_9c & 0xffff0000 | (param_1 + 0x24);
        local_a4 = (param_1 + 0x24);
        local_ac = param_1 + 0x20;
        local_94 = local_ac & 0xffff0000 | (param_1 + 0x22);
        local_b0 = SEXT24((param_1 + 0x22));
        local_b8 = (param_1 + 0x20);
        local_bc = local_b8 & 0xffff0000 | *local_b8;
        local_b4 = local_b8;
        local_a8 = local_ac;
        local_98 = local_9c;
        FUN_00482843(*local_b8,local_b0,local_a4,0x18,FUN_0042023b);
    }
    if (DAT_004c930c == 0x19) {
        local_c8 = param_1 + 0x20;
        local_cc = local_c8 & 0xffff0000 | (param_1 + 0x24);
        local_d0 = (param_1 + 0x24);
        local_d8 = param_1 + 0x20;
        local_c0 = local_d8 & 0xffff0000 | (param_1 + 0x22);
        local_dc = SEXT24((param_1 + 0x22));
        local_e4 = (param_1 + 0x20);
        local_e8 = local_e4 & 0xffff0000 | *local_e4;
        local_e0 = local_e4;
        local_d4 = local_d8;
        local_c4 = local_c8;
        FUN_00482843(*local_e4,local_dc,local_d0,0x18,FUN_004202bf);
    }
    if (DAT_004c930c < 0x19) {
        local_ec = DAT_004c9304;
        local_f0 = DAT_004c9308;
        local_f4 = DAT_004c9308;
        local_f8 = DAT_004c9304;
        local_100 = (param_1 + 0x20);
        local_104 = local_100 & 0xffff0000 | *local_100;
        local_fc = local_100;
        iVar3 = FUN_00420dbb(&DAT_004c7510,*local_100,DAT_004c9304,DAT_004c9308);
        if (iVar3 == 0x0) {
            uVar1 = (param_1 + 0x20);
            local_108 = (param_1 + 0x20) & 0xffff0000 | uVar1;
            bVar2 = FUN_0041f754(uVar1,local_ec,local_f0,0x2,&local_114,&local_110,&local_10c);
            if (CONCAT31(extraout_var,bVar2) != 0x0) {
                local_ec = local_114;
                local_f0 = local_110;
                param_2 = local_10c;
            }
            (param_1 + 0x48) = 0x1;
            (param_1 + 0x49) = param_2;
            (param_1 + 0x41) = local_ec;
            (param_1 + 0x42) = local_f0;
            FUN_00420cb1(&DAT_004c7510,(char)*(param_1 + 0x20),local_ec,local_f0);
            local_28 = 0x1;
        }
        else {
            local_28 = 0x0;
        }
    }
    else {
        local_28 = 0x0;
    }
    return local_28;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00420142(param_1: u32,param_2: u32,param_3: i32)

{
    if ((*(*(param_2 * 0x4 + DAT_005b2d50) + param_3) <= DAT_004c930c) &&
        ((*(param_2 * 0x4 + DAT_004c9318) + param_3 * 0x2) == 0x2)) {
        _DAT_004c9320 = 0x1;
        FUN_00482843(param_1,param_2,param_3,0x2,FUN_004201f5);
        if (_DAT_004c9320 != 0x0) {
            DAT_004c9304 = param_2;
            DAT_004c9308 = param_3;
            DAT_004c930c = *(*(param_2 * 0x4 + DAT_005b2d50) + param_3);
        }
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_004201f5(param_1: u32,param_2: i32,param_3: i32)

{
    if ((_DAT_004c9320 != 0x0) && ((*(param_2 * 0x4 + DAT_004c9318) + param_3 * 0x2) == 0x0)) {
        _DAT_004c9320 = 0x0;
    }
    return;
}



fn FUN_0042023b(param_1: u32,param_2: i32,param_3: i32)

{
    if ((*(*(param_2 * 0x4 + DAT_005b2d50) + param_3) <= DAT_004c930c) &&
        ((*(param_2 * 0x4 + DAT_004c9318) + param_3 * 0x2) == 0x2)) {
        DAT_004c9304 = param_2;
        DAT_004c9308 = param_3;
        DAT_004c930c = *(*(param_2 * 0x4 + DAT_005b2d50) + param_3);
    }
    return;
}



fn FUN_004202bf(param_1: u32,param_2: i32,param_3: i32)

{
    if ((*(*(param_2 * 0x4 + DAT_005b2d50) + param_3) <= DAT_004c930c) &&
        ((*(param_2 * 0x4 + DAT_004c9318) + param_3 * 0x2) != 0x1)) {
        DAT_004c9304 = param_2;
        DAT_004c9308 = param_3;
        DAT_004c930c = *(*(param_2 * 0x4 + DAT_005b2d50) + param_3);
    }
    return;
}



fn FUN_00420343(param_1: i32,undefined param_2) -> u32

{
    let mut iVar1: i32;
    let mut local_28: u32;
    let local_24: *mut u32;

    iVar1 = (param_1 + 0x20);
    FUN_00419085();
    FUN_0042feac();
    FUN_004a0430(DAT_004c931c,0x0,0x1842);
    local_24 = (&DAT_005b89f8 + iVar1 * 0x4);
    while( true ) {
        if ((local_24 == 0x0) || (*(local_24 + 0x6) >> 0x10 != iVar1))^ // goto LAB_0042040e;
        if ((local_24 + 0xe) == 0x11) break;
        local_24 = *local_24;
    }
    if (*(local_24 + 0xe) >> 0x10 == DAT_004c9754) {
        FUN_0041f524(0x1,local_24[0x2] >> 0x10,*(local_24 + 0xa) >> 0x10,DAT_004c9318,0x2);
    }
    LAB_0042040e:
    for (local_24 = (&DAT_005b89f8 + iVar1 * 0x4);
        (local_24 != 0x0 && (*(local_24 + 0x6) >> 0x10 == iVar1));
        local_24 = *local_24) {
        if (*(&DAT_004be8b0 + (local_24[0x3] >> 0x10) * 0x4) == 0x0) {
            FUN_0041f524(0x0,local_24[0x2] >> 0x10,*(local_24 + 0xa) >> 0x10,DAT_004c9318,0x1);
        }
    }
    FUN_00431d0a(&DAT_005967b8);
    DAT_005967bc = param_1;
    FUN_0045af67(&DAT_005967b8,(param_1 + 0x22),(param_1 + 0x24),
                 (param_1 + 0x22),(param_1 + 0x24),0x18);
    DAT_004c930c = 0x3e7;
    FUN_00482843((param_1 + 0x20),(param_1 + 0x22),(param_1 + 0x24),0x18,
                 FUN_004206c7);
    if (DAT_004c930c < 0x3e7) {
        iVar1 = FUN_00420dbb(&DAT_004c7510,(param_1 + 0x20),DAT_004c9304,DAT_004c9308);
        if (iVar1 == 0x0) {
            (param_1 + 0x48) = 0x1;
            (param_1 + 0x49) = param_2;
            (param_1 + 0x41) = DAT_004c9304;
            (param_1 + 0x42) = DAT_004c9308;
            FUN_00420cb1(&DAT_004c7510,(char)*(param_1 + 0x20),(char)DAT_004c9304,(char)DAT_004c9308);
            local_28 = 0x1;
        }
        else {
            local_28 = FUN_0041fa61(param_1,param_2);
        }
    }
    else {
        local_28 = FUN_0041fa61(param_1,param_2);
    }
    return local_28;
}



fn FUN_004206c7(param_1: u32,param_2: i32,param_3: i32)

{
    if ((*(*(param_2 * 0x4 + DAT_005b2d50) + param_3) <= DAT_004c930c) &&
        ((*(param_2 * 0x4 + DAT_004c9318) + param_3 * 0x2) == 0x2)) {
        DAT_004c9304 = param_2;
        DAT_004c9308 = param_3;
        DAT_004c930c = *(*(param_2 * 0x4 + DAT_005b2d50) + param_3);
    }
    return;
}



fn FUN_0042074b() -> u32

{
    return 0x0;
}



fn FUN_00420770(param_1: i32,param_2: i32,param_3: i32)

{
    let mut uVar1: u32;

    if ((((*(*(param_2 * 0x4 + DAT_005b2d50) + param_3) <= DAT_004c930c) &&
        ((*(param_2 * 0x4 + DAT_004c9318) + param_3 * 0x2) == 0x0)) &&
        (uVar1 = *(*(&DAT_004d7d50 + param_2 * 0x4 + param_1 * 0x3890) + param_3 * 0x4) & 0xf, uVar1 != 0x0
        )) && (uVar1 != 0x4)) {
        DAT_004c9304 = param_2;
        DAT_004c9308 = param_3;
        DAT_004c930c = *(*(param_2 * 0x4 + DAT_005b2d50) + param_3);
    }
    return;
}



fn FUN_0042082d(param_1: i32) -> u32

{
    let mut iVar1: i32;
    let mut local_2c: u32;
    let local_28: *mut u32;
    let local_24: *mut u32;

    iVar1 = (param_1 + 0x20);
    FUN_004a0430(DAT_004c931c,0x0,0x1842);
    for (local_24 = (&DAT_005b89f8 + iVar1 * 0x4);
        (local_24 != 0x0 && (*(local_24 + 0x6) >> 0x10 == iVar1));
        local_24 = *local_24) {
        if ((local_24 + 0xe) == 0x7) {
            FUN_0041f524(0x4,local_24[0x2] >> 0x10,*(local_24 + 0xa) >> 0x10,DAT_004c9318,0x1);
        }
        else {
            *
                ((*(local_24 + 0xa) >> 0x10) * 0x2 + *(DAT_004c9318 + (local_24[0x2] >> 0x10) * 0x4)) =
                0x1;
        }
    }
    for (local_28 = (&DAT_005b8b44 + iVar1 * 0x4);
        (local_28 != 0x0 && ((local_28 + 0x8) == iVar1)); local_28 = *local_28) {
        if (((*(local_28 + 0x3a) & 0x1) != 0x0) && ((local_28 + 0x26) != '\a')) {
            *
                (*(DAT_004c9318 + (local_28 + 0x22) * 0x4) + (local_28 + 0x9) * 0x2) = 0x1;
        }
    }
    FUN_00431d0a(&DAT_005967b8);
    DAT_005967bc = param_1;
    FUN_0045af67(&DAT_005967b8,(param_1 + 0x22),(param_1 + 0x24),
                 (param_1 + 0x22),(param_1 + 0x24),0x18);
    DAT_004c930c = 0x19;
    FUN_00419085();
    FUN_0042feac();
    FUN_00482843((param_1 + 0x20),(param_1 + 0x22),(param_1 + 0x24),0x18,
                 FUN_00420770);
    if (DAT_004c930c < 0x19) {
        iVar1 = FUN_00420dbb(&DAT_004c7510,(param_1 + 0x20),DAT_004c9304,DAT_004c9308);
        if (iVar1 == 0x0) {
            (param_1 + 0x48) = 0x1;
            (param_1 + 0x49) = 0x7;
            (param_1 + 0x41) = DAT_004c9304;
            (param_1 + 0x42) = DAT_004c9308;
            FUN_00420cb1(&DAT_004c7510,(char)*(param_1 + 0x20),(char)DAT_004c9304,(char)DAT_004c9308);
            local_2c = 0x1;
        }
        else {
            local_2c = 0x0;
        }
    }
    else {
        local_2c = 0x0;
    }
    return local_2c;
}



fn FUN_00420c5e(param_1: u32) -> u32

{
    FUN_00420c87(param_1);
    return param_1;
}



fn FUN_00420c87(param_1: u32)

{
    FUN_004a0430(param_1,0xff,0x12c);
    return;
}



fn FUN_00420cb1(param_1: i32,undefined param_2,undefined param_3,undefined param_4)

{
    let mut local_14: i32;

    local_14 = 0x0;
    while( true ) {
        if (0x63 < local_14) {
            return;
        }
        if ((local_14 * 0x3 + param_1) == -0x1) break;
        local_14 = local_14 + 0x1;
    }
    (local_14 * 0x3 + param_1) = param_2;
    (local_14 * 0x3 + param_1 + 0x1) = param_3;
    (local_14 * 0x3 + param_1 + 0x2) = param_4;
    return;
}



fn FUN_00420d18(param_1: i32,param_2: u32,param_3: u32,param_4: u32)

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut local_14: i32;

    local_14 = 0x0;
    while( true ) {
        if (0x63 < local_14) {
            return;
        }
        if (((*(local_14 * 0x3 + param_1) == param_2) && (*(local_14 * 0x3 + param_1 + 0x1) == param_3)) &&
            (*(local_14 * 0x3 + param_1 + 0x2) == param_4)) break;
        local_14 = local_14 + 0x1;
    }
    iVar1 = local_14 * 0x3 + param_1;
    (iVar1 + 0x2) = 0xff;
    iVar2 = local_14 * 0x3 + param_1;
    (iVar2 + 0x1) = (iVar1 + 0x2);
    (local_14 * 0x3 + param_1) = (iVar2 + 0x1);
    return;
}



fn FUN_00420dbb(param_1: i32,param_2: u32,param_3: u32,param_4: u32) -> u32

{
    let mut local_14: i32;

    local_14 = 0x0;
    while( true ) {
        if (0x63 < local_14) {
            return 0x0;
        }
        if (((*(local_14 * 0x3 + param_1) == param_2) && (*(local_14 * 0x3 + param_1 + 0x1) == param_3)) &&
            (*(local_14 * 0x3 + param_1 + 0x2) == param_4)) break;
        local_14 = local_14 + 0x1;
    }
    return 0x1;
}



fn FUN_00420e3e()

{
    let local_14: *mut u32;

    FUN_004a0430(&DAT_004c763c,0x0,0x1400);
    FUN_004a0430(&DAT_004c8a3c,0x0,0xa0);
    for (local_14 = *DAT_005967c8; local_14 != 0x0; local_14 = *local_14) {
        if (*(local_14 + 0xe) >> 0x10 == DAT_004c9754) {
            *(&DAT_004c763c + (local_14[0x3] >> 0x10) * 0x4 + (*(local_14 + 0x6) >> 0x10) * 0x80) =
                *(&DAT_004c763c + (local_14[0x3] >> 0x10) * 0x4 + (*(local_14 + 0x6) >> 0x10) * 0x80)
                    + 0x1;
            *(&DAT_004c8a3c + (*(local_14 + 0x6) >> 0x10) * 0x4) =
                *(&DAT_004c8a3c + (*(local_14 + 0x6) >> 0x10) * 0x4) + 0x1;
        }
    }
    return;
}



fn FUN_00420edd(param_1: i32)

{
    let piVar1: *mut i32;;
    let local_14: *mut u32;

    FUN_004a0430(param_1,0x0,0x820);
    if (DAT_004c9760 == 0x0) {
        for (local_14 = *DAT_005967b0; local_14 != 0x0; local_14 = *local_14) {
            if (((local_14 + 0x27) == '[') && (*(local_14 + 0x23) >> 0x18 == DAT_004c9754)) {
                piVar1 = ((*(local_14 + 0x2d) >> 0x18) * 0x4 + param_1 + (local_14 + 0x8) * 0x34);
                *piVar1 = *piVar1 + (*(local_14 + 0x2f) >> 0x10);
            }
        }
    }
    else {
        for (local_14 = *DAT_005967b0; local_14 != 0x0; local_14 = *local_14) {
            if (((local_14 + 0x27) == '[') && (*(local_14 + 0x23) >> 0x18 == DAT_004c9754)) {
                piVar1 = ((*(local_14 + 0x2d) >> 0x18) * 0x4 + param_1);
                *piVar1 = *piVar1 + (*(local_14 + 0x2f) >> 0x10);
            }
        }
    }
    return;
}



fn FUN_00420fd9()

{
    let local_14: *mut u32;

    FUN_004a0430(&DAT_004c9324,0x0,0x38);
    for (local_14 = *DAT_005967b0; local_14 != 0x0; local_14 = *local_14) {
        *(&DAT_004c9324 + (*(local_14 + 0x23) >> 0x18) * 0x4) =
            *(&DAT_004c9324 + (*(local_14 + 0x23) >> 0x18) * 0x4) +
                *(*(&DAT_00582938 +
                    (*(local_14 + 0x25) >> 0x18) * 0x4 + (local_14[0x9] >> 0x18) * 0x18) + 0x121)
                + *(*(&DAT_00582938 +
                (*(local_14 + 0x25) >> 0x18) * 0x4 + (local_14[0x9] >> 0x18) * 0x18) +
                0x125);
    }
    return;
}



fn FUN_00421079(param_1: i32) -> i32

{
return (*(&DAT_004c9324 + param_1 * 0x4) * 0xa + 0x5) / (*(&DAT_004c9324 + DAT_004c9754 * 0x4) + 0x1);
}



fn FUN_004210ba(param_1: i32,param_2: i32,param_3: i32) -> i32

{
let mut iVar1: i32;
let mut uVar2: u32;
i32 aiStackY131264 [0x7ffb];
let local_c8: *mut u32;
let local_c4: *mut u32;
i32 local_c0 [0x29];
let mut local_1c: i32;
let mut local_18: i32;
let mut local_14: i32;

FUN_004a0430(local_c0,0x0,0xa0);
if (param_1 == 0x6) {
for (local_c4 = *DAT_005967c8; local_c4 != 0x0; local_c4 = *local_c4) {
if ((((local_c4 + 0xe) == 0x17) &&
((*(&DAT_004be9b0 + DAT_004c9754 * 0x4) & local_c4[0xb]) == 0x0)) &&
((*(&DAT_004be9b0 + param_2 * 0x4) & local_c4[0xb]) != 0x0)) {
local_c0[*(local_c4 + 0x6) >> 0x10] = local_c0[*(local_c4 + 0x6) >> 0x10] + 0x1;
}
}
}
else {
for (local_c8 = *DAT_005967c8; local_c8 != 0x0; local_c8 = *local_c8) {
iVar1 = FUN_00481a44(local_c8);
if (((iVar1 == 0x0) && ((*(&DAT_004be9b0 + param_1 * 0x4) & local_c8[0xb]) == 0x0)) &&
((*(&DAT_004be9b0 + param_2 * 0x4) & local_c8[0xb]) != 0x0)) {
local_c0[*(local_c8 + 0x6) >> 0x10] = local_c0[*(local_c8 + 0x6) >> 0x10] + 0x1;
}
}
}
local_1c = -0x1;
for (local_14 = 0x0; local_14 < 0x28; local_14 = local_14 + 0x1) {
if ((local_c0[local_14] != 0x0) &&
((local_1c < 0x0 ||
(uVar2 = local_c0[local_14] - param_3 >> 0x1f, ((local_c0[local_14] - param_3 ^ uVar2) - uVar2) < local_18)
))) {
local_1c = local_14;
uVar2 = local_c0[local_14] - param_3 >> 0x1f;
local_18 = (local_c0[local_14] - param_3 ^ uVar2) - uVar2;
}
}
return local_1c;
}



fn FUN_0042126f(param_1: i32,param_2: i32) -> i32

{
let mut uVar1: u32;
let mut iVar2: i32;
let mut iVar3: i32;
let mut local_1c: i32;
let mut local_18: i32;

local_18 = 0x0;
uVar1 = FUN_004a2edc();
if (uVar1 % 0xa < 0x3) {
if (param_2 < 0x4) {
local_1c = param_2;
}
else {
local_1c = 0x3;
}
iVar2 = FUN_004210ba(param_1,DAT_004c9754,local_1c * 0x3);
if ((iVar2 != -0x1) && (iVar3 = FUN_00481a7f(iVar2,param_1,DAT_004c9754), iVar3 * 0x3 + -0x3 < param_2)) {
*(&DAT_004c97f8 + param_1 * 0xf8 + DAT_004c9754 * 0xd90) = 0x140;
*(&DAT_004c9804 + DAT_004c9754 * 0xd90 + param_1 * 0xf8) = iVar2;
local_18 = 0x1;
param_2 = param_2 - iVar3 / 0x3;
}
}
iVar3 = FUN_0042d4ca(param_1,DAT_004c9754);
uVar1 = FUN_004a2edc();
iVar2 = uVar1 / 0xa;
if (((uVar1 % 0xa < 0x7) && (0x8 < param_2)) && (iVar3 != 0x0)) {
*(&DAT_004c97f8 + local_18 * 0x28 + DAT_004c9754 * 0xd90 + param_1 * 0xf8) = 0x136;
*(&DAT_004c9800 + local_18 * 0x28 + param_1 * 0xf8 + DAT_004c9754 * 0xd90) = iVar3;
param_2 = param_2 + -0x8;
iVar2 = local_18;
local_18 = local_18 + 0x1;
}
if (0x0 < param_2) {
*(&DAT_004c97f8 + param_1 * 0xf8 + DAT_004c9754 * 0xd90 + local_18 * 0x28) = 0x12c;
uVar1 = FUN_004a2edc();
iVar2 = DAT_004c9754 * 0xd90 + param_1 * 0xf8;
*(&DAT_004c97fc + local_18 * 0x28 + iVar2) = param_2 * ((uVar1 % 0xb) * 0xa + 0xc8);
}
return iVar2;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0042145e()

{
    byte *pbVar1;
    let mut uVar2: u32;
    let puVar3: *mut u32;
    let mut iVar4: i32;
    let mut pcVar5: String;
    let uVar6: u16;
    u32 local_2ac [0x2f];
    i32 aiStack496 [0x4];
    let local_1e0: *mut u32;
    u32 local_1dc [0x20];
    let mut local_15c: u32;
    let mut local_158: i32;
    let mut local_154: i32;
    let mut local_150: *mut u8;
    let mut local_14c: u32;
    let mut local_148: i32;
    let mut local_144: i32;
    let mut local_140: *mut u8;
    let mut local_13c: u32;
    let local_138: *mut u32;
    let mut local_134: i32;
    let mut local_130: i32;
    let mut local_12c: i32;
    let local_128: u8;
    let local_124: u8;
    let local_120: u8;
    let mut local_11c: i32;
    let local_118: u8;
    let local_114: u8;
    let local_110: u8;
    byte *local_10c;
    let mut local_108: u32;
    let mut local_104: u32;
    let mut local_100: i32;
    let mut local_fc: i32;
    let mut local_f8: *mut u8;
    let mut local_f4: u32;
    let mut local_f0: i32;
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
    let mut local_a0: i32;
    let mut local_9c: u32;
    let mut local_98: u32;
    let mut local_94: u32;
    let mut local_90: u32;
    let mut local_8c: u32;
    let mut local_88: u32;
    let mut local_84: *mut u8;
    let mut local_80: i32;
    let mut local_7c: i32;
    let mut local_78: u32;
    let mut local_74: u32;
    let mut local_70: *mut u8;
    let mut local_6c: i32;
    let mut local_68: i32;
    let mut local_64: u32;
    let mut local_60: u32;
    let mut local_5c: u32;
    let mut local_58: *mut u8;
    let mut local_54: i32;
    let mut local_50: i32;
    let mut local_4c: u32;
    let mut local_48: u32;
    let local_44: *mut *mut u8;;
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

    if (DAT_004c9754 < 0x5) {
        FUN_00420fd9();
        for (local_14 = 0x0; local_14 < 0x5; local_14 = local_14 + 0x1) {
            if (local_14 != DAT_004c9754) {
                FUN_004a0430(&local_ec,0x0,0x68);
                local_84 = &DAT_004d55a8;
                local_80 = DAT_004c9754;
                local_7c = local_14;
                local_78 = (byte)(&DAT_004d55a8)[DAT_004c9754 * 0xe + local_14];
                local_74 = (local_78 == 0x1);
                local_ec = local_74;
                local_e8 = 0xfff0bdc1;
                local_e4 = 0xfff0bdc1;
                local_e0 = 0xfff0bdc1;
                local_dc = 0xfff0bdc1;
                local_d8 = 0xfff0bdc1;
                local_d4 = 0xfff0bdc1;
                local_d0 = 0xfff0bdc1;
                local_cc = 0xfff0bdc1;
                local_c8 = 0xfff0bdc1;
                local_c4 = 0xfff0bdc1;
                local_c0 = *(&DAT_00569b14 + local_14 * 0x4 + DAT_004c9754 * 0x1e22);
                local_bc = 0xfff0bdc1;
                local_b8 = 0xfff0bdc1;
                local_b4 = 0xfff0bdc1;
                local_70 = &DAT_004d55a8;
                local_6c = DAT_004c9754;
                local_68 = local_14;
                local_64 = (byte)(&DAT_004d55a8)[DAT_004c9754 * 0xe + local_14];
                local_60 = (local_64 == 0x0);
                local_b0 = local_60;
                local_ac = 0xfff0bdc1;
                local_a8 = 0xfff0bdc1;
                local_a4 = 0xfff0bdc1;
                local_a0 = FUN_00421079(local_14);
                local_9c = 0xfff0bdc1;
                if ((_DAT_004d5560 == 0x1) || ((_DAT_004d5560 == 0x2 && (uVar2 = FUN_004a2edc(), uVar2 % 0xa < 0x2)))) {
                    local_5c = 0x1;
                }
                else {
                    local_5c = 0x0;
                }
                local_98 = local_5c;
                local_58 = &DAT_004d55a8;
                local_54 = DAT_004c9754;
                local_50 = local_14;
                local_4c = (byte)(&DAT_004d55a8)[DAT_004c9754 * 0xe + local_14];
                local_48 = (local_4c == 0x2);
                local_94 = local_48;
                local_90 = 0xfff0bdc1;
                local_8c = 0xfff0bdc1;
                local_88 = 0xfff0bdc1;
                if (DAT_004c9754 < 0x5) {
                    local_44 = &PTR_s_T_30_T_40_004bd1c0;
                    local_40 = 0x1f;
                }
                else {
                    local_44 = (undefined **)0x0;
                    local_40 = 0x0;
                }
                for (local_3c = 0x0; local_3c < local_40; local_3c = local_3c + 0x1) {
                    (local_44 + local_3c * 0x2a + 0x29) = 0x1;
                }
                local_38 = FUN_004210ba(local_14,DAT_004c9754,0x6);
                local_34 = FUN_004210ba(0x6,DAT_004c9754,0x1);
                local_30 = FUN_004210ba(DAT_004c9754,local_14,0x8);
                for (local_3c = 0x0; local_3c < local_40; local_3c = local_3c + 0x1) {
                    for (local_f0 = 0x0; local_f0 < 0x3; local_f0 = local_f0 + 0x1) {
                        local_f4 = *(local_44 + local_f0 * 0x4 + local_3c * 0x2a + 0x8);
                        switch(local_f4) {
                            case 0x0:
                            break;
                            case 0x1:
                            if (local_b0 == 0x0) {
                                (local_44 + local_3c * 0x2a + 0x29) = 0x0;
                            }
                            break;
                            case 0x2:
                            if (local_94 == 0x0) {
                                (local_44 + local_3c * 0x2a + 0x29) = 0x0;
                            }
                            break;
                            case 0x3:
                            if (local_30 == -0x1) {
                                (local_44 + local_3c * 0x2a + 0x29) = 0x0;
                            }
                            break;
                            case 0x4:
                            if (*(&DAT_00569abd + local_14 * 0x1e22) < *(local_44 + local_3c * 0x2a + 0x21)) {
                                (local_44 + local_3c * 0x2a + 0x29) = 0x0;
                            }
                            break;
                            case 0x5:
                                iVar4 = FUN_0042d4ca(DAT_004c9754,local_14);
                            if (iVar4 == 0x0) {
                                (local_44 + local_3c * 0x2a + 0x29) = 0x0;
                            }
                            break;
                            case 0x6:
                            if (*(&DAT_00569ac5 + local_14 * 0x1e22) != -0x1) {
                                (local_44 + local_3c * 0x2a + 0x29) = 0x0;
                            }
                            break;
                            case 0x7:
                            if (local_94 != 0x0) {
                                (local_44 + local_3c * 0x2a + 0x29) = 0x0;
                            }
                            for (local_2c = 0x0; local_2c < 0x5; local_2c = local_2c + 0x1) {
                                if ((local_14 != local_2c) && (DAT_004c9754 != local_2c)) {
                                    local_f8 = &DAT_004d55a8;
                                    local_fc = DAT_004c9754;
                                    local_100 = local_2c;
                                    local_104 = (byte)(&DAT_004d55a8)[DAT_004c9754 * 0xe + local_2c];
                                    if (local_104 == 0x0) break;
                                }
                            }
                            if (local_2c == 0x5) {
                                (local_44 + 0xfb) = 0x0;
                            }
                            break;
                            case 0x8:
                            if (local_94 != 0x0) {
                                (local_44 + local_3c * 0x2a + 0x29) = 0x0;
                            }
                            break;
                            case 0x9:
                            if (local_94 != 0x0) {
                                (local_44 + local_3c * 0x2a + 0x29) = 0x0;
                            }
                            break;
                            case 0xa:
                            break;
                            case 0xb:
                            if (local_34 == -0x1) {
                                (local_44 + local_3c * 0x2a + 0x29) = 0x0;
                            }
                            break;
                            case 0xc:
                        }
                        local_108 = *(local_44 + local_f0 * 0x4 + local_3c * 0x2a + 0x14);
                        switch(local_108) {
                            case 0x0:
                            break;
                            case 0x1:
                            break;
                            case 0x2:
                            if (local_b0 == 0x0) {
                                (local_44 + local_3c * 0x2a + 0x29) = 0x0;
                            }
                            break;
                            case 0x3:
                            if (local_94 == 0x0) {
                                (local_44 + local_3c * 0x2a + 0x29) = 0x0;
                            }
                            break;
                            case 0x4:
                            if (local_38 == -0x1) {
                                (local_44 + local_3c * 0x2a + 0x29) = 0x0;
                            }
                            break;
                            case 0x5:
                            if (*(&DAT_00569abd + DAT_004c9754 * 0x1e22) < *(local_44 + local_3c * 0x2a + 0x21)) {
                                (local_44 + local_3c * 0x2a + 0x29) = 0x0;
                            }
                            break;
                            case 0x6:
                                iVar4 = FUN_0042d4ca(local_14,DAT_004c9754);
                            if (iVar4 == 0x0) {
                                (local_44 + local_3c * 0x2a + 0x29) = 0x0;
                            }
                            break;
                            case 0x7:
                            if (*(&DAT_00569ac5 + DAT_004c9754 * 0x1e22) != -0x1) {
                                (local_44 + local_3c * 0x2a + 0x29) = 0x0;
                            }
                            break;
                            case 0x8:
                            break;
                            case 0x9:
                            break;
                            case 0xa:
                            if (local_ec == 0x0) {
                                (local_44 + local_3c * 0x2a + 0x29) = 0x0;
                            }
                            break;
                            case 0xb:
                            if (local_b0 == 0x0) {
                                (local_44 + local_3c * 0x2a + 0x29) = 0x0;
                            }
                            break;
                            case 0xc:
                            break;
                            case 0xd:
                            break;
                            case 0xe:
                        }
                    }
                }
                for (local_3c = 0x0; local_3c < local_40; local_3c = local_3c + 0x1) {
                    local_10c = *(byte **)(local_3c * 0x2a + local_44);
                    while ((*local_10c != 0x0 && ((local_44 + local_3c * 0x2a + 0x29) != '\0'))) {
                        pbVar1 = local_10c + 0x1;
                        local_114 = *local_10c;
                        local_118 = 0x0;
                        local_11c = 0x0;
                        local_124 = *pbVar1;
                        if (local_124 < 0x2c) {
                            if (local_124 != 0x0) {
                                if (local_124 == 0x21)^ // goto LAB_00421baf;^
                                // goto LAB_00421c86;
                            }
                            LAB_00421c09:
                                local_118 = 0x21;
                        }
                        else {
                            if (local_124 < 0x2d)^ // goto LAB_00421c09;
                            if (local_124 < 0x3d) {
                                if (local_124 == 0x3c)^ // goto LAB_00421baf;
                            }
                            else {
                                if ((local_124 < 0x3e) || (local_124 == 0x3e)) {
                                    LAB_00421baf:
                                        local_118 = *pbVar1;
                                    for (local_10c = local_10c + 0x2;
                                        (pbVar1 = local_10c, '/' < (char)*local_10c && ((char)*local_10c < ':'));
                                        local_10c = local_10c + 0x1) {
                                        local_11c = local_11c * 0xa + (char)*local_10c + -0x30;
                                    }
                                }
                            }
                        }
                        LAB_00421c86:
                            local_10c = pbVar1;
                        local_120 = local_118;
                        local_128 = local_118;
                        if (local_118 < 0x3c) {
                            if ((local_118 == 0x21) && (aiStack496[(char)local_114] == local_11c)) {
                            (local_44 + local_3c * 0x2a + 0x29) = 0x0;
                            }
                        }
                        else {
                            if (local_118 < 0x3d) {
                                if (local_11c <= aiStack496[(char)local_114]) {
                                (local_44 + local_3c * 0x2a + 0x29) = 0x0;
                                }
                            }
                            else {
                                if (local_118 < 0x3e) {
                                    if (aiStack496[(char)local_114] != local_11c) {
                                        (local_44 + local_3c * 0x2a + 0x29) = 0x0;
                                    }
                                }
                                else {
                                    if ((local_118 == 0x3e) && (aiStack496[(char)local_114] < local_11c)) {
                                    (local_44 + local_3c * 0x2a + 0x29) = 0x0;
                                    }
                                }
                            }
                        }
                        local_110 = local_124;
                        if (*local_10c != 0x0) {
                            local_10c = local_10c + 0x1;
                        }
                    }
                }
                local_24 = -0x1;
                local_20 = 0x0;
                for (local_3c = 0x0; local_3c < local_40; local_3c = local_3c + 0x1) {
                    if (((local_44 + local_3c * 0x2a + 0x29) != '\0') &&
                        (*(local_44 + local_3c * 0x2a + 0x4) <= local_24)) {
                        if (*(local_44 + local_3c * 0x2a + 0x4) < local_24) {
                            local_24 = *(local_44 + local_3c * 0x2a + 0x4);
                            local_20 = 0x0;
                        }
                        else {
                            local_20 = local_20 + 0x1;
                        }
                    }
                }
                if (local_20 == 0x0) {
                    local_130 = 0x0;
                    for (local_3c = 0x0; local_3c < local_40; local_3c = local_3c + 0x1) {
                        if ((local_44 + local_3c * 0x2a + 0x29) != '\0') {
                            local_130 = local_130 + *(local_44 + local_3c * 0x2a + 0x4);
                        }
                    }
                    if (local_130 == 0x0)^ // goto LAB_00421486;
                    uVar2 = FUN_004a2edc();
                    local_134 = uVar2 % local_130;
                    local_1c = 0x0;
                    while ((local_1c < local_40 &&
                        (((local_44 + local_1c * 0x2a + 0x29) == '\0' ||
                            (local_134 = local_134 - *(local_44 + local_1c * 0x2a + 0x4), -0x1 < local_134))))) {
                        local_1c = local_1c + 0x1;
                    }
                }
                else {
                    uVar2 = FUN_004a2edc();
                    local_12c = uVar2 % local_20 + 0x1;
                    local_1c = 0x0;
                    while ((local_1c < local_40 &&
                        ((((local_44 + local_1c * 0x2a + 0x29) == '\0' ||
                            (*(local_44 + local_1c * 0x2a + 0x4) != local_24)) ||
                            (local_12c = local_12c + -0x1, local_12c != 0x0))))) {
                        local_1c = local_1c + 0x1;
                    }
                }
                local_18 = 0x0;
                FUN_0042d188(local_14,DAT_004c9754);
                for (local_3c = 0x0; local_3c < 0x3; local_3c = local_3c + 0x1) {
                    local_138 = (&DAT_004c9780 + local_3c * 0x28 + local_14 * 0xf8 + DAT_004c9754 * 0xd90);
                    local_13c = *(local_44 + local_3c * 0x4 + local_1c * 0x2a + 0x8);
                    switch(local_13c) {
                        case 0x0:
                        break;
                        case 0x1:
                            *local_138 = 0x14c;
                        local_138[0x4] = 0x1;
                        local_138[0x5] = local_14;
                        *(&DAT_004c977c + DAT_004c9754 * 0xd90 + local_14 * 0xf8) = 0x0;
                        break;
                        case 0x2:
                            *local_138 = 0x14b;
                        local_138[0x4] = 0x0;
                        local_138[0x5] = local_14;
                        *(&DAT_004c977c + DAT_004c9754 * 0xd90 + local_14 * 0xf8) = 0x0;
                        break;
                        case 0x3:
                            *local_138 = 0x140;
                        local_138[0x3] = local_30;
                        *(&DAT_004c977c + DAT_004c9754 * 0xd90 + local_14 * 0xf8) = 0x0;
                        break;
                        case 0x4:
                            *local_138 = 0x12c;
                        local_138[0x1] = *(local_44 + local_1c * 0x2a + 0x21);
                        *(&DAT_004c977c + local_14 * 0xf8 + DAT_004c9754 * 0xd90) = 0x0;
                        break;
                        case 0x5:
                            *local_138 = 0x136;
                        iVar4 = FUN_0042d4ca(DAT_004c9754,local_14);
                        local_138[0x2] = iVar4;
                        *(&DAT_004c977c + local_14 * 0xf8 + DAT_004c9754 * 0xd90) = 0x0;
                        local_18 = (CONCAT44(*(&DAT_0058ad6a + local_138[0x2] * 0xda) >> 0x1f,
                                             *(&DAT_0058ad6a + local_138[0x2] * 0xda)) / 0x23);
                        break;
                        case 0x6:
                            *local_138 = 0x14a;
                        *(&DAT_004c977c + DAT_004c9754 * 0xd90 + local_14 * 0xf8) = 0x0;
                        break;
                        case 0x7:
                            uVar2 = FUN_004a2edc();
                        local_2c = uVar2 % 0x5;
                        for (local_28 = 0x0; local_28 < 0x5; local_28 = local_28 + 0x1) {
                            if (local_2c == 0x5) {
                                local_2c = 0x0;
                            }
                            if ((local_14 != local_2c) && (DAT_004c9754 != local_2c)) {
                                local_140 = &DAT_004d55a8;
                                local_144 = DAT_004c9754;
                                local_148 = local_2c;
                                local_14c = (byte)(&DAT_004d55a8)[DAT_004c9754 * 0xe + local_2c];
                                if (local_14c == 0x0) break;
                            }
                            local_2c = local_2c + 0x1;
                        }
                        if (local_2c != 0x5) {
                            *local_138 = 0x14d;
                            local_138[0x4] = 0x2;
                            local_138[0x5] = local_2c;
                            *(&DAT_004c977c + local_14 * 0xf8 + DAT_004c9754 * 0xd90) = 0x0;
                            FUN_00419085();
                        }
                        break;
                        case 0x8:
                            FUN_00462a28(&DAT_004d55a8,DAT_004c9754,local_14,0x2);
                        uVar6 = 0x0;
                        uVar2 = 0xffffffff;
                        puVar3 = FUN_00499050(DAT_005967a4,0x68);
                        FUN_0045518a(0x1 << ((byte)local_14 & 0x1f),DAT_004c9754,0x74cc,0xffffffff,puVar3,uVar2,uVar6);
                        FUN_00419085();
                        break;
                        case 0x9:
                        break;
                        case 0xa:
                        break;
                        case 0xb:
                        break;
                        case 0xc:
                            uVar2 = FUN_004a2edc();
                        local_2c = uVar2 % 0x5;
                        for (local_28 = 0x0; local_28 < 0x5; local_28 = local_28 + 0x1) {
                            if (local_2c == 0x5) {
                                local_2c = 0x0;
                            }
                            if (((local_2c != local_14) && (local_2c != DAT_004c9754)) &&
                                (((&DAT_00569a98)[local_2c * 0x1e22] & 0x2) == 0x0)) {
                                local_150 = &DAT_004d55a8;
                                local_154 = DAT_004c9754;
                                local_158 = local_2c;
                                local_15c = (byte)(&DAT_004d55a8)[DAT_004c9754 * 0xe + local_2c];
                                if (local_15c == 0x1) break;
                            }
                            local_2c = local_2c + 0x1;
                        }
                        if (local_2c == 0x5) {
                            loop {
                                loop {
                                    uVar2 = FUN_004a2edc();
                                    local_2c = uVar2 % 0x5;
                                } while (local_2c == local_14);
                            } while (local_2c == DAT_004c9754);
                        }
                        FUN_00499050(DAT_0059679c,local_14 + 0x414);
                        pcVar5 = FUN_00499050(DAT_005967a4,0x65);
                        FUN_0049c2e0(local_1dc,pcVar5);
                        FUN_0045518a(0x1 << ((byte)local_2c & 0x1f),DAT_004c9754,0x74cc,0xffffffff,local_1dc,0xffffffff,0x0);
                        FUN_00419085();
                    }
                }
                for (local_3c = 0x0; local_3c < 0x3; local_3c = local_3c + 0x1) {
                    local_1e0 = (&DAT_004c97f8 + local_3c * 0x28 + local_14 * 0xf8 + DAT_004c9754 * 0xd90);
                    aiStack496[3] = *(local_44 + local_3c * 0x4 + local_1c * 0x2a + 0x14);
                    switch(aiStack496[3]) {
                        case 0x0:^
                            // goto LAB_0042249f;
                        case 0x1:
                        if (local_18 == 0x0) {
                            local_18 = *(local_44 + local_1c * 0x2a + 0x1d) >> 0x18;
                        }
                        FUN_0042126f(local_14,local_18);
                        break;
                        case 0x2:
                            *local_1e0 = 0x14c;
                        local_1e0[0x4] = 0x1;
                        local_1e0[0x5] = DAT_004c9754;
                        break;
                        case 0x3:
                            *local_1e0 = 0x14b;
                        local_1e0[0x4] = 0x0;
                        local_1e0[0x5] = DAT_004c9754;
                        break;
                        case 0x4:
                            *local_1e0 = 0x140;
                        local_1e0[0x3] = local_38;
                        break;
                        case 0x5:
                            *local_1e0 = 0x12c;
                        local_1e0[0x1] = *(local_44 + local_1c * 0x2a + 0x21);
                        break;
                        case 0x6:
                            *local_1e0 = 0x136;
                        iVar4 = FUN_0042d4ca(local_14,DAT_004c9754);
                        local_1e0[0x2] = iVar4;
                        break;
                        case 0x7:
                            *local_1e0 = 0x14a;
                        break;
                        case 0x8:
                            *local_1e0 = 0x188;
                        break;
                        case 0x9:
                            *local_1e0 = 0x187;
                        break;
                        case 0xa:
                            *(&DAT_004c9778 + local_14 * 0xf8 + DAT_004c9754 * 0xd90) = 0x1;
                        *local_1e0 = 0x14b;
                        local_1e0[0x4] = 0x0;
                        local_1e0[0x5] = DAT_004c9754;
                        break;
                        case 0xb:
                            *(&DAT_004c9778 + local_14 * 0xf8 + DAT_004c9754 * 0xd90) = 0x1;
                        *local_1e0 = 0x14d;
                        local_1e0[0x4] = 0x2;
                        local_1e0[0x5] = DAT_004c9754;
                        break;
                        case 0xc:
                            *(&DAT_004c9778 + DAT_004c9754 * 0xd90 + local_14 * 0xf8) = 0x1;
                        *local_1e0 = 0x172;
                        break;
                        case 0xd:
                            *(&DAT_004c9778 + DAT_004c9754 * 0xd90 + local_14 * 0xf8) = 0x1;
                        *local_1e0 = 0x17c;
                        break;
                        case 0xe:
                            *(&DAT_004c9778 + DAT_004c9754 * 0xd90 + local_14 * 0xf8) = 0x1;
                        *local_1e0 = 0x186;
                    }
                    *(&DAT_004c977c + local_14 * 0xf8 + DAT_004c9754 * 0xd90) = 0x0;
                    LAB_0042249f:
                }
                if ((*(&DAT_004c977c + local_14 * 0xf8 + DAT_004c9754 * 0xd90) != 0x0) &&
                    (*(local_44 + local_1c * 0x2a + 0x25) != 0x0)) {
                    pcVar5 = FUN_00499050(DAT_005967a4,*(local_44 + local_1c * 0x2a + 0x25));
                    FUN_0049c2e0(local_2ac,pcVar5);
                    FUN_0045518a(0x1 << ((byte)local_14 & 0x1f),DAT_004c9754,0x74cc,0xffffffff,local_2ac,0xffffffff,0x0);
                }
            }
            LAB_00421486:
        }
    }
    return;
}



fn FUN_004954f3(param_1: u32,param_2: u32)

{
    FUN_004953d7();
    DAT_005b91a8 = param_1;
    DAT_005b91ac = param_2;
    FUN_0049536f();
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00495520(param_1: u32,param_2: u32,param_3: u32,param_4: u32,param_5: i32)

{
    let mut iVar1: i32;

    if (((_DAT_005b91c0 != 0x0) && (0x0 < DAT_005b91bc)) && (DAT_005b920c == 0x0)) {
        DAT_005b9180 = param_1;
        DAT_005b9184 = param_2;
        DAT_005b9188 = param_3;
        _DAT_005b918c = param_4;
        iVar1 = FUN_004aa0ab(&DAT_005b9180,DAT_005b91a0 - DAT_005b91a8,DAT_005b91a4 - DAT_005b91ac,DAT_005b9190,DAT_005b9194
        );
        if (iVar1 == 0x0) {
            _DAT_005b9170 = 0x1;
        }
        else {
            _DAT_005b9174 = DAT_005b91a0;
            _DAT_005b9178 = DAT_005b91a4;
            _DAT_005b9170 = 0x3;
            if (param_5 != 0x0) {
                _DAT_005b9170 = 0x0;
                FUN_004953d7();
                _DAT_005b9170 = 0x4;
                return;
            }
        }
        FUN_0049607e();
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00495607(param_1: i32)

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let puVar6: *mut u32;
    let puVar7: *mut u32;
    let mut local_44: i32;
    let mut local_40: i32;
    let mut local_3c: u32;
    let mut local_38: i32;
    let mut local_34: i32;
    let mut local_30: i32;
    let mut local_1c: u32;

    if ((DAT_005b920c == 0x0) && (_DAT_005b91c0 != 0x0)) {
        if (_DAT_005b9170 == 0x4) {
            _DAT_005b9170 = 0x0;
            FUN_0049536f();
        }
        iVar3 = DAT_005b91e4;
        iVar2 = DAT_005b91e0;
        iVar1 = DAT_005b91dc;
        iVar5 = DAT_005b91d8;
        if ((0x0 < DAT_005b91bc) && (DAT_005b9220 != 0x0)) {
            if (_DAT_005b9170 < 0x2) {
                _DAT_005b9170 = 0x0;
                FUN_004960b0();
            }
            else {
                if ((0x2 < _DAT_005b9170) && (DAT_005b91b8 != 0x0)) {
                    local_30 = DAT_005b9188;
                    local_34 = 0x0;
                    iVar4 = (DAT_005b9180 + DAT_005b9188) - (DAT_005b91d8 + DAT_005b91e0);
                    if (0x0 < iVar4) {
                        local_30 = DAT_005b9188 - iVar4;
                    }
                    local_38 = DAT_005b91d8 - DAT_005b9180;
                    if (local_38 < 0x1) {
                        local_34 = -local_38;
                        local_38 = 0x0;
                    }
                    else {
                        local_30 = local_30 - local_38;
                    }
                    if (0x0 < local_30) {
                        for (local_1c = DAT_005b9184; local_1c < DAT_005b9184 + _DAT_005b918c; local_1c = local_1c + 0x1) {
                            if ((iVar1 <= local_1c) && (local_1c < iVar1 + iVar3)) {
                                if (param_1 == 0x0) {
                                    FUN_004aaa6c(iVar5 + local_34,local_1c,
                                                 ((local_1c - iVar1) * iVar2 + DAT_005b91c4 + local_34),local_30,0x1,0x0);
                                }
                                else {
                                    puVar6 = ((local_1c - DAT_005b9184) * DAT_005b9188 + param_1 + local_38);
                                    puVar7 = (DAT_005b91c4 + iVar2 * (local_1c - iVar1) + local_34);
                                    *puVar7 = *puVar6;
                                    (puVar7 + 0x1) = (puVar6 + 0x1);
                                }
                            }
                        }
                    }
                }
                _DAT_005b9170 = 0x0;
                if ((DAT_005b91b8 == 0x0) || ((DAT_005b91a0 == _DAT_005b9174 && (DAT_005b91a4 == _DAT_005b9178)))) {
                    if (DAT_005b91b8 == 0x0) {
                        FUN_00494f54();
                    }
                    else {
                        local_3c = DAT_005b9190;
                        local_40 = DAT_005b91a0 - DAT_005b91a8;
                        local_44 = 0x0;
                        if (local_40 < 0x0) {
                            local_44 = -local_40;
                            local_3c = DAT_005b9190 + local_40;
                            local_40 = 0x0;
                        }
                        if (DAT_005b9228 < DAT_005b9190 + local_40) {
                            local_3c = DAT_005b9228 - local_40;
                        }
                        for (local_1c = 0x0; local_1c < DAT_005b9194; local_1c = local_1c + 0x1) {
                            iVar5 = local_1c + (DAT_005b91a4 - DAT_005b91ac);
                            if ((DAT_005b9184 <= iVar5) && (iVar5 < DAT_005b9184 + _DAT_005b918c)) {
                                FUN_00496ee6((DAT_005b91d0 + local_44 + local_1c * DAT_005b9190),local_40,iVar5,local_3c,0x1);
                            }
                        }
                    }
                }
                else {
                    FUN_00494c6a();
                    FUN_00494f54();
                }
                FUN_004960b0();
            }
        }
    }
    return;
}



fn FUN_004958f8(param_1: *mut u32)

{
    *param_1 = DAT_005b9190;
    param_1[0x1] = DAT_005b9194;
    param_1[0x2] = DAT_005b91a8;
    param_1[0x3] = DAT_005b91ac;
    param_1[0x4] = DAT_005b91d0;
    param_1[0x5] = DAT_005b91bc;
    param_1[0x6] = DAT_005b96b8;
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0049595c(param_1: *mut i32)

{
    if (_DAT_005b91c0 != 0x0) {
        FUN_004960df();
        if (DAT_005b91bc < 0x1) {
            DAT_005b91bc = 0x0;
            FUN_0049607e();
        }
        else {
            DAT_005b91bc = 0x1;
            FUN_004953d7();
        }
        DAT_005b91d0 = param_1[0x4];
        DAT_005b9190 = *param_1;
        DAT_005b9194 = param_1[0x1];
        DAT_005b9198 = DAT_005b9190 * DAT_005b9194;
        DAT_005b91a8 = param_1[0x2];
        DAT_005b91ac = param_1[0x3];
        DAT_005b91bc = param_1[0x5] + -0x1;
        FUN_0049536f();
        FUN_004960ff(param_1[0x6]);
    }
    return;
}



fn FUN_00495a10(param_1: u32,param_2: u32)

{
    DAT_005b91b0 = param_1;
    DAT_005b91b4 = param_2;
    return;
}



fn FUN_00495a31()

{
    FUN_004960df();
    if (DAT_005b91bc < 0x1) {
        DAT_005b91bc = 0x0;
        FUN_0049607e();
    }
    else {
        DAT_005b91bc = 0x1;
        FUN_004953d7();
    }
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn direct_draw_create_00495a72(HWND window_handle_1,param_2: i32,LPCSTR param_3,LPCSTR param_4) -> u32

{
    let cVar1: u8;
    let puVar2: *mut u32;
    let ppcVar3: *mut *mut char;
    HRESULT HVar4;
    let mut iVar5: i32;
    undefined3 extraout_var;
    let local_18: u32;

    _DAT_005b921c = window_handle_1;
    if ((DAT_005b9230 == 0x0) && (param_3 != 0x0)) {
        puVar2 = FUN_0049c2c9(0x300);
        DAT_005b9230 = puVar2;
        if ((puVar2 == 0x0) ||
            (ppcVar3 = FUN_0049c4bd(param_3,&DAT_004c32c2), ppcVar3 == 0x0)) {
            pop_err_msg_box_and_exit_004a02f5(s_No_Palette_File_004c32c5);
        }
        else {
            FUN_004a7970(puVar2,0x300,0x1,ppcVar3);
            FUN_0049ca40(ppcVar3);
        }
    }
    // IUnknown * p_unk_outer for DirectDrawCreate
    // LPDIRECTDRAW * lp_lp_dd for DirectDrawCreate
    // GUID * lp_guid for DirectDrawCreate
    HVar4 = DirectDrawCreate((GUID *)0x0,&LPDIRECTDRAW_005b9638,(IUnknown *)0x0);
    if (HVar4 == 0x0) {
        // WARNING: Load size is inaccurate
        iVar5 = ((*LPDIRECTDRAW_005b9638 + 0x50))(LPDIRECTDRAW_005b9638,window_handle_1,0x11);
        if (iVar5 == 0x0) {
            // WARNING: Load size is inaccurate
            ((*LPDIRECTDRAW_005b9638 + 0x20))(LPDIRECTDRAW_005b9638,0x0,0x0,0x0,FUN_00495d65);
            if (_DAT_005b96b4 == 0x0) {
                // UINT uType for MessageBoxA
                // LPCSTR lpCaption for MessageBoxA
                // LPCSTR lpText for MessageBoxA
                // HWND hWnd for MessageBoxA
                MessageBoxA(window_handle_1,s_EnumDisplayModes_Failed_004c3321,s_Exiting_004c3319,0x0);
                local_18 = 0x0;
            }
            else {
                if (param_2 == 0x0) {
                    iVar5 = palette_fn_00495df2(0x280,0x1e0,0x8);
                    if (iVar5 == 0x0) {
                        return 0x0;
                    }
                }
                else {
                    iVar5 = palette_fn_00495df2(0x140,0xc8,0x8);
                    if (iVar5 == 0x0) {
                        return 0x0;
                    }
                }
                if (param_4 == 0x0) {
                    if (LPCSTR_005b9218 == 0x0) {
                        DAT_005b9210 = 0x10;
                        DAT_005b9214 = 0xc;
                        LPCSTR_005b9218 = FUN_004971ad(s_610FULL_004c3339);
                    }
                }
                else {
                    if (LPCSTR_005b9218 != 0x0) {
                        FUN_0049af50(LPCSTR_005b9218);
                    }
                    LPCSTR_005b9218 = FUN_004971ad(param_4);
                    cVar1 = FUN_004a06b1(0x57,LPCSTR_005b9218);
                    DAT_005b9214 = CONCAT31(extraout_var,cVar1);
                    DAT_005b9210 = FUN_004a06f6(LPCSTR_005b9218);
                }
                local_18 = 0x1;
            }
        }
        else {
            // UINT uType for MessageBoxA
            // LPCSTR lpCaption for MessageBoxA
            // LPCSTR lpText for MessageBoxA
            // HWND hWnd for MessageBoxA
            MessageBoxA(window_handle_1,s_Set_CooperativeLevel_Failed_004c32fd,s_Exiting_004c32f5,0x0);
            local_18 = 0x0;
        }
    }
    else {
        // UINT uType for MessageBoxA
        // LPCSTR lpCaption for MessageBoxA
        // LPCSTR lpText for MessageBoxA
        // HWND hWnd for MessageBoxA
        MessageBoxA(window_handle_1,s_DirectDrawCreate_Failed_004c32dd,s_Exiting_004c32d5,0x0);
        local_18 = 0x0;
    }
    return local_18;
}



fn FUN_00495cd1()

{
    if (DAT_005b963c != 0x0) {
        ((*DAT_005b963c + 0x8))(DAT_005b963c);
        DAT_005b963c = 0x0;
    }
    if (DAT_005b9640 != 0x0) {
        ((*DAT_005b9640 + 0x8))(DAT_005b9640);
        DAT_005b9640 = 0x0;
    }
    if (LPDIRECTDRAW_005b9638 != (LPDIRECTDRAW)0x0) {
    // WARNING: Load size is inaccurate
    ((*LPDIRECTDRAW_005b9638 + 0x4c))(LPDIRECTDRAW_005b9638);
    // WARNING: Load size is inaccurate
    ((*LPDIRECTDRAW_005b9638 + 0x8))(LPDIRECTDRAW_005b9638);
    LPDIRECTDRAW_005b9638 = (LPDIRECTDRAW)0x0;
    DAT_005b9220 = 0x0;
}
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_00495d65(param_1: i32) -> u32

{
    let mut local_14: u32;

    if (*(param_1 + 0x54) == 0x8) {
        if ((*(param_1 + 0xc) == 0x140) && (*(param_1 + 0x8) == 0xc8)) {
            _DAT_005b96b0 = 0x1;
        }
        if ((*(param_1 + 0xc) == 0x280) && (*(param_1 + 0x8) == 0x1e0)) {
            _DAT_005b96b4 = 0x1;
        }
    }
    if ((_DAT_005b96b0 == 0x0) || (_DAT_005b96b4 == 0x0)) {
        local_14 = 0x1;
    }
    else {
        local_14 = 0x0;
    }
    return local_14;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn palette_fn_00495df2(DWORD param_1,DWORD param_2,DWORD param_3) -> u32

{
    let mut iVar1: i32;
    HDC hdc;
    let mut device_caps_val: u32;
    let mut index_20: i32;
    let local_1c: u32;

    // WARNING: Load size is inaccurate
    iVar1 = ((*LPDIRECTDRAW_005b9638 + 0x54))(LPDIRECTDRAW_005b9638,param_1,param_2,param_3);
    if (iVar1 == 0x0) {
        if (DAT_005b963c != 0x0) {
            ((*DAT_005b963c + 0x8))(DAT_005b963c);
            DAT_005b963c = 0x0;
        }
        if (DAT_005b9640 != 0x0) {
            ((*DAT_005b9640 + 0x8))(DAT_005b9640);
            DAT_005b9640 = 0x0;
        }
        FUN_004a0430(&DAT_005b9644,0x0,0x6c);
        _DAT_005b9644 = 0x6c;
        _DAT_005b96ac = 0x200;
        _DAT_005b9648 = 0x1;
        // WARNING: Load size is inaccurate
        iVar1 = ((*LPDIRECTDRAW_005b9638 + 0x18))(LPDIRECTDRAW_005b9638,&DAT_005b9644,&DAT_005b963c,0x0);
        if (iVar1 == 0x0) {
            FUN_004a0430(&DAT_005b9644,0x0,0x6c);
            _DAT_005b96ac = 0x200;
            _DAT_005b9648 = 0xff9ee;
            _DAT_005b9644 = 0x6c;
            iVar1 = ((*DAT_005b963c + 0x58))(DAT_005b963c,&DAT_005b9644);
            if (iVar1 == 0x0) {
                DAT_005b9224 = DAT_005b9654;
                DAT_005b9228 = DAT_005b9650;
                DAT_005b922c = DAT_005b964c;
                FUN_004953d7();
                FUN_004968e7(0x0,0x0,DAT_005b9228,DAT_005b922c,0x0);
                FUN_0049536f();
                // HWND hWnd for GetDC
                hdc = GetDC((HWND)0x0);
                // i32 index for GetDeviceCaps
                // HDC hdc for GetDeviceCaps
                device_caps_val = GetDeviceCaps(hdc,0x26);
                if ((device_caps_val & 0x100) != 0x0) {
                    // LPPALETTEENTRY pPalEntries for GetSystemPaletteEntries
                    // UINT cEntries for GetSystemPaletteEntries
                    // UINT iStart for GetSystemPaletteEntries
                    // HDC hdc for GetSystemPaletteEntries
                    GetSystemPaletteEntries(hdc,0x0,0x100,(LPPALETTEENTRY)&palette_entry_005b9238);
                    for (index_20 = 0xa; index_20 < 0xf6; index_20 = index_20 + 0x1) {
                        (&palette_entry_005b9238)[index_20].peRed = (DAT_005b9230 + index_20 * 0x3) << 0x2;
                        (&palette_entry_005b9238)[index_20].peGreen = (DAT_005b9230 + index_20 * 0x3 + 0x1) << 0x2;
                        (&palette_entry_005b9238)[index_20].peBlue = (index_20 * 0x3 + DAT_005b9230 + 0x2) << 0x2;
                    }
                    // WARNING: Load size is inaccurate
                    iVar1 = ((*LPDIRECTDRAW_005b9638 + 0x14))
                        (LPDIRECTDRAW_005b9638,0x4c,&palette_entry_005b9238,&DAT_005b9640,0x0);
                    if (iVar1 != 0x0) {
                        return 0x0;
                    }
                    ((*DAT_005b963c + 0x7c))(DAT_005b963c,DAT_005b9640);
                }
                // HDC hDC for ReleaseDC
                // HWND hWnd for ReleaseDC
                ReleaseDC((HWND)0x0,hdc);
                local_1c = 0x1;
            }
            else {
                local_1c = 0x0;
            }
        }
        else {
            local_1c = 0x0;
        }
    }
    else {
        local_1c = 0x0;
    }
    return local_1c;
}



fn FUN_0049607e() -> u32

{
    let mut iVar1: i32;

    iVar1 = DAT_005b96b8;
    DAT_005b96b8 = DAT_005b96b8 + 0x1;
    if (iVar1 == 0x0) {
        FUN_0049613c();
    }
    return DAT_005b9220;
}



fn FUN_004960b0()

{
    code *pcVar1;

    DAT_005b96b8 = DAT_005b96b8 + -0x1;
    if (DAT_005b96b8 < 0x0) {
        pcVar1 = (code *)swi(0x3);
        (*pcVar1)();
        return;
    }
    if (DAT_005b96b8 == 0x0) {
        FUN_0049621b();
    }
    return;
}



fn FUN_004960df()

{
    FUN_0049621b();
    DAT_005b96b8 = 0x0;
    return;
}



fn FUN_004960ff(param_1: i32)

{
    if (param_1 < 0x1) {
        if (0x0 < DAT_005b96b8) {
            FUN_0049621b();
        }
    }
    else {
        if (DAT_005b96b8 < 0x1) {
            FUN_0049613c();
        }
    }
    DAT_005b96b8 = param_1;
    return;
}



fn FUN_0049613c() -> u32

{
    let mut local_24: u32;
    let mut local_20: u32;
    let mut local_1c: i32;

    if ((DAT_005b963c == 0x0) || ((DAT_005b91f0 != 0x0 && (DAT_005b91fc == 0x0)))) {
        local_20 = 0x0;
    }
    else {
        FUN_0049621b();
        local_1c = ((*DAT_005b963c + 0x64))(DAT_005b963c,0x0,&DAT_005b9644,0x0,0x0);
        if (local_1c == -0x7789fe3e) {
            ((*DAT_005b963c + 0x6c))(DAT_005b963c);
            local_1c = ((*DAT_005b963c + 0x64))(DAT_005b963c,0x0,&DAT_005b9644,0x0,0x0);
        }
        if (local_1c == 0x0) {
            local_24 = DAT_005b9668;
        }
        else {
            local_24 = 0x0;
        }
        DAT_005b9220 = local_24;
        local_20 = local_24;
    }
    return local_20;
}



fn FUN_0049621b()

{
    if ((DAT_005b9220 != 0x0) && (DAT_005b963c != 0x0)) {
        ((*DAT_005b963c + 0x80))(DAT_005b963c,DAT_005b9220);
        DAT_005b9220 = 0x0;
    }
    return;
}



fn FUN_00496263(param_1: i32,param_2: i32,param_3: i32)

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut local_18: i32;

    iVar1 = DAT_005b9220;
    if (DAT_005b9220 != 0x0) {
        FUN_0049621b();
    }
    if (DAT_005b9640 != 0x0) {
        for (local_18 = 0x0; local_18 < param_3; local_18 = local_18 + 0x1) {
            (&palette_entry_005b9238)[param_2 + local_18].peRed = (local_18 * 0x3 + param_1) << 0x2;
            (&palette_entry_005b9238)[param_2 + local_18].peGreen = (local_18 * 0x3 + param_1 + 0x1) << 0x2;
            (&palette_entry_005b9238)[param_2 + local_18].peBlue = (local_18 * 0x3 + param_1 + 0x2) << 0x2;
        }
        iVar2 = ((*DAT_005b9640 + 0x18))(DAT_005b9640,0x0,param_2,param_3,&palette_entry_005b9238 + param_2);
        if (iVar2 == -0x7789fe3e) {
            ((*DAT_005b963c + 0x6c))(DAT_005b963c);
            ((*DAT_005b9640 + 0x18))(DAT_005b9640,0x0,param_2,param_3,&palette_entry_005b9238 + param_2);
        }
    }
    if (iVar1 != 0x0) {
        FUN_0049613c();
    }
    return;
}



bool  FUN_0049637b(param_1: &mut String,param_2: *mut u32)

{
let ppcVar1: *mut *mut char;

ppcVar1 = FUN_0049c4bd(param_1,&DAT_004c32c2);
if (ppcVar1 != 0x0) {
FUN_004a7970(param_2,0x300,0x1,ppcVar1);
FUN_0049ca40(ppcVar1);
}
return ppcVar1 != 0x0;
}



fn FUN_004963db()

{
    FUN_00496263(DAT_005b9230 + 0x1e,0xa,0xec);
    return;
}



fn FUN_00496404(param_1: i32)

{
    FUN_00496263(param_1 + 0x1e,0xa,0xec);
    return;
}



fn FUN_0049642b(param_1: i32,param_2: i32)

{
    let local_3d0: u8 [0x3b0];
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    local_18 = DAT_005b9220;
    if (DAT_005b9220 != 0x0) {
        FUN_0049621b();
    }
    if (param_1 == 0x0) {
        param_1 = DAT_005b9230;
    }
    for (local_14 = 0x1; local_14 < param_2; local_14 = local_14 + 0x1) {
        for (local_1c = 0x0; local_1c < 0xec; local_1c = local_1c + 0x1) {
            local_3d0[local_1c * 0x4] =
                (char)((local_14 * *(param_1 + (local_1c + 0xa) * 0x3) * 0x4) / param_2);
            local_3d0[local_1c * 0x4 + 0x1] =
                (char)((local_14 * *((local_1c + 0xa) * 0x3 + param_1 + 0x1) * 0x4) / param_2);
            local_3d0[local_1c * 0x4 + 0x2] =
                (char)((local_14 * *((local_1c + 0xa) * 0x3 + param_1 + 0x2) * 0x4) / param_2);
        }
            // WARNING: Load size is inaccurate
            ((*LPDIRECTDRAW_005b9638 + 0x58))(LPDIRECTDRAW_005b9638,0x1,0x0);
        local_20 = ((*DAT_005b9640 + 0x18))(DAT_005b9640,0x0,0xa,0xec,local_3d0);
        if (local_20 == -0x7789fe3e) {
            ((*DAT_005b963c + 0x6c))(DAT_005b963c);
            local_20 = ((*DAT_005b9640 + 0x18))(DAT_005b9640,0x0,0xa,0xec,local_3d0);
        }
    }
        // WARNING: Load size is inaccurate
        ((*LPDIRECTDRAW_005b9638 + 0x58))(LPDIRECTDRAW_005b9638,0x1,0x0);
    FUN_00496263(param_1 + 0x1e,0xa,0xec);
    if (local_18 != 0x0) {
        FUN_0049613c();
    }
    return;
}



fn FUN_004965e4(param_1: i32,param_2: i32)

{
    let local_6d0: u8 [0x3b0];
    let mut local_320: i32;
    let mut local_31c: i32;
    let local_318: u8 [0x1e];
    let auStack762: u8 [0x2e2];
    let mut local_18: i32;
    let mut local_14: i32;

    local_18 = DAT_005b9220;
    if (DAT_005b9220 != 0x0) {
        FUN_0049621b();
    }
    if (param_1 == 0x0) {
        param_1 = DAT_005b9230;
    }
    for (local_14 = param_2; 0x0 < local_14; local_14 = local_14 + -0x1) {
        for (local_31c = 0x0; local_31c < 0xec; local_31c = local_31c + 0x1) {
            local_6d0[local_31c * 0x4] =
                (char)((local_14 * *(param_1 + (local_31c + 0xa) * 0x3) * 0x4) / param_2);
            local_6d0[local_31c * 0x4 + 0x1] =
                (char)((local_14 * *((local_31c + 0xa) * 0x3 + param_1 + 0x1) * 0x4) / param_2);
            local_6d0[local_31c * 0x4 + 0x2] =
                (char)((local_14 * *((local_31c + 0xa) * 0x3 + param_1 + 0x2) * 0x4) / param_2);
        }
            // WARNING: Load size is inaccurate
            ((*LPDIRECTDRAW_005b9638 + 0x58))(LPDIRECTDRAW_005b9638,0x1,0x0);
        local_320 = ((*DAT_005b9640 + 0x18))(DAT_005b9640,0x0,0xa,0xec,local_6d0);
        if (local_320 == -0x7789fe3e) {
            ((*DAT_005b963c + 0x6c))(DAT_005b963c);
            local_320 = ((*DAT_005b9640 + 0x18))(DAT_005b9640,0x0,0xa,0xec,local_6d0);
        }
    }
    FUN_004a0430(local_318,0x0,0x300);
    // WARNING: Load size is inaccurate
    ((*LPDIRECTDRAW_005b9638 + 0x58))(LPDIRECTDRAW_005b9638,0x1,0x0);
    FUN_00496263(auStack762,0xa,0xec);
    if (local_18 != 0x0) {
        FUN_0049613c();
    }
    return;
}



fn FUN_004967d7(param_1: *mut u32,param_2: i32,param_3: i32)

{
    let uVar1: u8;
    let uVar2: u8;
    let uVar3: u8;
    let mut iVar4: i32;

    uVar1 = param_1;
    uVar2 = (param_1 + 0x1);
    uVar3 = (param_1 + 0x2);
    iVar4 = param_3 + -0x1;
    *param_1 = *(param_1 + 0x3);
    (param_1 + 0x1) = (param_1 + 0x7);
    (iVar4 * 0x3 + param_1) = uVar1;
    (param_1 + iVar4 * 0x3 + 0x1) = uVar2;
    (param_1 + iVar4 * 0x3 + 0x2) = uVar3;
    FUN_00496263(param_1,param_2,iVar4);
    return;
}



fn FUN_00496863()

{
    let mut iVar1: i32;

    if (((DAT_005b9640 != 0x0) && (DAT_005b963c != 0x0)) &&
        (iVar1 = ((*DAT_005b9640 + 0x18))(DAT_005b9640,0x0,0xa,0xec,&DAT_005b9260), iVar1 == -0x7789fe3e)) {
        ((*DAT_005b963c + 0x6c))(DAT_005b963c);
        ((*DAT_005b9640 + 0x18))(DAT_005b9640,0x0,0xa,0xec,&DAT_005b9260);
    }
    return;
}



fn FUN_004968e7(param_1: i32,param_2: i32,param_3: i32,param_4: i32,undefined param_5)

{
    if (((param_3 != 0x0) && (param_4 != 0x0)) && (DAT_005b9220 != 0x0)) {
        if (DAT_005b91f0 != 0x0) {
            if (param_1 + param_3 <= DAT_005b91f4) {
                return;
            }
            if (DAT_005b9204 < param_1) {
                return;
            }
            if (param_2 + param_4 <= DAT_005b91f8) {
                return;
            }
            if (DAT_005b9208 < param_2) {
                return;
            }
            if (param_1 < DAT_005b91f4) {
                param_3 = param_3 - (DAT_005b91f4 - param_1);
                param_1 = DAT_005b91f4;
            }
            if (param_2 < DAT_005b91f8) {
                param_4 = param_4 - (DAT_005b91f8 - param_2);
                param_2 = DAT_005b91f8;
            }
            if (DAT_005b91f4 + DAT_005b91fc < param_1 + param_3) {
                param_3 = (DAT_005b9204 + 0x1) - param_1;
            }
            if (DAT_005b91f8 + DAT_005b9200 < param_2 + param_4) {
                param_4 = (DAT_005b9208 + 0x1) - param_2;
            }
        }
        FUN_004aad90(param_1,param_2,param_3,param_4,param_5);
    }
    return;
}



fn FUN_00496a10()

{
    FUN_004968e7(0x0,0x0,DAT_005b9228,DAT_005b922c,0x0);
    return;
}



fn FUN_00496a3b(param_1: i32,param_2: i32,undefined param_3)

{
    if (((DAT_005b91f0 == 0x0) ||
        ((((DAT_005b91f4 <= param_1 && (param_1 <= DAT_005b9204)) && (DAT_005b91f8 <= param_2)) &&
            (param_2 <= DAT_005b9208)))) && (DAT_005b9220 != 0x0)) {
        FUN_004aae04(param_1,param_2,param_3);
    }
    return;
}



fn FUN_00496ac0(param_1: *mut u32,param_2: i32,param_3: i32,param_4: i32,param_5: i32)

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut local_14: i32;

    if (DAT_005b9220 != 0x0) {
        if (DAT_005b91f0 == 0x0) {
            FUN_004aaa04(param_1,param_2,param_3,param_4,param_5,0x0);
        }
        else {
            iVar1 = param_4 + param_2 + -0x1;
            iVar2 = param_5 + param_3 + -0x1;
            if ((((DAT_005b91f4 <= iVar1) && (DAT_005b91f8 <= iVar2)) && (param_2 <= DAT_005b9204)) &&
                (param_3 <= DAT_005b9208)) {
                local_14 = 0x0;
                if (DAT_005b9208 < iVar2) {
                    param_5 = (DAT_005b9208 + 0x1) - param_3;
                }
                if (param_3 < DAT_005b91f8) {
                    param_5 = param_5 - (DAT_005b91f8 - param_3);
                    param_1 = (param_1 + (DAT_005b91f8 - param_3) * param_4);
                    param_3 = DAT_005b91f8;
                }
                if (DAT_005b9204 < iVar1) {
                    local_14 = iVar1 - DAT_005b9204;
                    param_4 = param_4 - local_14;
                }
                if (param_2 < DAT_005b91f4) {
                    iVar1 = DAT_005b91f4 - param_2;
                    param_4 = param_4 - iVar1;
                    local_14 = local_14 + iVar1;
                    param_1 = (param_1 + iVar1);
                    param_2 = DAT_005b91f4;
                }
                FUN_004aaa04(param_1,param_2,param_3,param_4,param_5,local_14);
            }
        }
    }
    return;
}



fn FUN_00496c1f(param_1: i32,param_2: i32,param_3: *mut u32,param_4: i32,param_5: i32)

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut local_14: i32;

    if (DAT_005b9220 != 0x0) {
        if (DAT_005b91f0 == 0x0) {
            FUN_004aaa6c(param_1,param_2,param_3,param_4,param_5,0x0);
        }
        else {
            iVar1 = param_4 + param_1 + -0x1;
            iVar2 = param_5 + param_2 + -0x1;
            if ((((DAT_005b91f4 <= iVar1) && (DAT_005b91f8 <= iVar2)) && (param_1 <= DAT_005b9204)) &&
                (param_2 <= DAT_005b9208)) {
                local_14 = 0x0;
                if (DAT_005b9208 < iVar2) {
                    param_5 = (DAT_005b9208 + 0x1) - param_2;
                }
                if (param_2 < DAT_005b91f8) {
                    param_5 = param_5 - (DAT_005b91f8 - param_2);
                    param_3 = (param_3 + (DAT_005b91f8 - param_2) * param_4);
                    param_2 = DAT_005b91f8;
                }
                if (DAT_005b9204 < iVar1) {
                    local_14 = iVar1 - DAT_005b9204;
                    param_4 = param_4 - local_14;
                }
                if (param_1 < DAT_005b91f4) {
                    iVar1 = DAT_005b91f4 - param_1;
                    param_4 = param_4 - iVar1;
                    local_14 = local_14 + iVar1;
                    param_3 = (param_3 + iVar1);
                    param_1 = DAT_005b91f4;
                }
                FUN_004aaa6c(param_1,param_2,param_3,param_4,param_5,local_14);
            }
        }
    }
    return;
}



fn FUN_00496d7e(param_1: i32,param_2: i32,param_3: u32,param_4: u32,param_5: i32,param_6: u32,param_7: u32)

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut uVar3: u32;
    let mut uVar4: u32;
    let mut iVar5: i32;
    let mut iVar6: i32;

    uVar4 = param_4;
    uVar3 = param_3;
    iVar2 = param_2;
    iVar1 = param_1;
    if (DAT_005b9220 != 0x0) {
        if (DAT_005b91f0 == 0x0) {
            FUN_004aae28(param_1,param_2,param_3,param_4,param_5,param_6,param_7,param_1,param_2,param_3,param_4);
        }
        else {
            iVar5 = param_3 + param_1 + -0x1;
            iVar6 = param_4 + param_2 + -0x1;
            if ((((DAT_005b91f4 <= iVar5) && (DAT_005b91f8 <= iVar6)) && (param_1 <= DAT_005b9204)) &&
                (param_2 <= DAT_005b9208)) {
                if (DAT_005b9204 < iVar5) {
                    param_3 = (DAT_005b9204 + 0x1) - param_1;
                }
                if (DAT_005b9208 < iVar6) {
                    param_4 = (DAT_005b9208 + 0x1) - param_2;
                }
                if (param_2 < DAT_005b91f8) {
                    param_4 = param_4 - (DAT_005b91f8 - param_2);
                    param_2 = DAT_005b91f8;
                }
                if (param_1 < DAT_005b91f4) {
                    param_3 = param_3 - (DAT_005b91f4 - param_1);
                    param_1 = DAT_005b91f4;
                }
                FUN_004aae28(param_1,param_2,param_3,param_4,param_5,param_6,param_7,iVar1,iVar2,uVar3,uVar4);
            }
        }
    }
    return;
}



fn FUN_00496ee6(param_1: &mut String,param_2: i32,param_3: i32,param_4: u32,param_5: i32)

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut local_14: i32;

    if (DAT_005b9220 != 0x0) {
        if (DAT_005b91f0 == 0x0) {
            FUN_004aaf98(param_1,param_2,param_3,param_4,param_5,0x0);
        }
        else {
            iVar1 = param_4 + param_2 + -0x1;
            iVar2 = param_5 + param_3 + -0x1;
            if ((((DAT_005b91f4 <= iVar1) && (DAT_005b91f8 <= iVar2)) && (param_2 <= DAT_005b9204)) &&
                (param_3 <= DAT_005b9208)) {
                local_14 = 0x0;
                if (DAT_005b9208 < iVar2) {
                    param_5 = (DAT_005b9208 + 0x1) - param_3;
                }
                if (param_3 < DAT_005b91f8) {
                    param_5 = param_5 - (DAT_005b91f8 - param_3);
                    param_1 = param_1 + (DAT_005b91f8 - param_3) * param_4;
                    param_3 = DAT_005b91f8;
                }
                if (DAT_005b9204 < iVar1) {
                    local_14 = iVar1 - DAT_005b9204;
                    param_4 = param_4 - local_14;
                }
                if (param_2 < DAT_005b91f4) {
                    iVar1 = DAT_005b91f4 - param_2;
                    param_4 = param_4 - iVar1;
                    local_14 = local_14 + iVar1;
                    param_1 = param_1 + iVar1;
                    param_2 = DAT_005b91f4;
                }
                FUN_004aaf98(param_1,param_2,param_3,param_4,param_5,local_14);
            }
        }
    }
    return;
}



fn FUN_00497045(param_1: i32,param_2: i32,param_3: u32,param_4: u32,param_5: i32,param_6: i32,param_7: u32)

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut uVar3: u32;
    let mut uVar4: u32;
    let mut iVar5: i32;
    let mut iVar6: i32;

    uVar4 = param_4;
    uVar3 = param_3;
    iVar2 = param_2;
    iVar1 = param_1;
    if (DAT_005b9220 != 0x0) {
        if (DAT_005b91f0 == 0x0) {
            FUN_004ab060(param_1,param_2,param_3,param_4,param_5,param_6,param_7,param_1,param_2,param_3,param_4);
        }
        else {
            iVar5 = param_3 + param_1 + -0x1;
            iVar6 = param_4 + param_2 + -0x1;
            if ((((DAT_005b91f4 <= iVar5) && (DAT_005b91f8 <= iVar6)) && (param_1 <= DAT_005b9204)) &&
                (param_2 <= DAT_005b9208)) {
                if (DAT_005b9204 < iVar5) {
                    param_3 = (DAT_005b9204 + 0x1) - param_1;
                }
                if (DAT_005b9208 < iVar6) {
                    param_4 = (DAT_005b9208 + 0x1) - param_2;
                }
                if (param_2 < DAT_005b91f8) {
                    param_4 = param_4 - (DAT_005b91f8 - param_2);
                    param_2 = DAT_005b91f8;
                }
                if (param_1 < DAT_005b91f4) {
                    param_3 = param_3 - (DAT_005b91f4 - param_1);
                    param_1 = DAT_005b91f4;
                }
                FUN_004ab060(param_1,param_2,param_3,param_4,param_5,param_6,param_7,iVar1,iVar2,uVar3,uVar4);
            }
        }
    }
    return;
}



fn FUN_004971ad(LPCSTR param_1) -> String

{
    let cVar1: u8;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut pcVar4: String;
    let mut pcVar5: String;
    let mut pcVar6: String;
    let local_70: u8 [0x4b];
    let local_25: u8;
    let mut local_1c: u32;
    let local_18: u32;
    let mut local_14: String;

    FUN_004a9a00(local_70,param_1,0x4c);
    local_25 = 0x0;
    pcVar4 = &DAT_004c3341;
    iVar3 = -0x1;
    pcVar5 = local_70;
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
    if (cVar1 == '\0') break;
    cVar1 = pcVar4[0x1];
    pcVar4 = pcVar4 + 0x2;
    pcVar6[0x1] = cVar1;
    pcVar6 = pcVar6 + 0x2;
} while (cVar1 != '\0');
    local_1c = FUN_0049c993(local_70,0x200);
    if (local_1c != 0xffffffff) {
        local_18 = FUN_004a91d0(local_1c);
        local_14 = FUN_0049c2c9(local_18);
        if ((local_14 != 0x0) && (uVar2 = read_file_func_004ab180(local_1c,local_14,local_18), uVar2 == local_18)) {
            FUN_004ab390(local_1c);
            return local_14;
        }
    }
    pop_err_msg_box_and_exit_004a02f5(s_Error_loading__s__004c3346);
    return 0x0;
}



fn FUN_00497282(param_1: i32,ushort *param_2,param_3: i32) -> i32

{
undefined3 extraout_var;
let cVar1: u8;
let cStack32: u8;
let mut local_18: i32;
let mut local_14: i32;

local_18 = 0x0;
for (local_14 = 0x0; (local_14 < param_3 || ((param_3 < 0x0 && ((param_1 + local_14) != '\0'))));
local_14 = local_14 + 0x1) {
if ((param_1 + local_14) == '\0') {
local_14 = local_14 + -0x1;
param_3 = param_3 + -0x1;
cVar1 = '\x02';
}
else {
cVar1 = (param_1 + local_14) == '~';
if ((bool)cVar1) {
local_14 = local_14 + 0x1;
param_3 = param_3 + 0x1;
}
}
if (cVar1 == '\x02') {
cStack32 = ' ';
}
else {
cStack32 = (param_1 + local_14);
}
cVar1 = FUN_004a06b1(cStack32,param_2);
local_18 = local_18 + CONCAT31(extraout_var,cVar1);
}
return local_18;
}



// WARNING: Removing unreachable block (ram,0x00497480)
// WARNING: Removing unreachable block (ram,0x00497498)
// WARNING: Removing unreachable block (ram,0x0049749e)
// WARNING: Removing unreachable block (ram,0x0049746e)
// WARNING: Removing unreachable block (ram,0x0049744a)
// WARNING: Removing unreachable block (ram,0x00497432)
// WARNING: Removing unreachable block (ram,0x00497456)
// WARNING: Removing unreachable block (ram,0x00497486)
// WARNING: Removing unreachable block (ram,0x004974a4)
// WARNING: Removing unreachable block (ram,0x00497409)

void
FUN_00497330(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: u32,param_6: i32,param_7: u32)

{
let mut pCVar1: String;;
let cVar2: u8;
let mut iVar3: i32;
let mut iVar4: i32;
undefined3 extraout_var;
let mut local_54: i32;
let mut local_50: i32;
let mut local_48: i32;
let local_38: *mut u32;
let mut local_28: i32;
let mut local_14: u32;

pCVar1 = LPCSTR_005b9218;
local_54 = param_1;
local_50 = param_2;
local_48 = param_4;
local_38 = LPCSTR_005b9218;
iVar3 = FUN_004a06f6(LPCSTR_005b9218);
if (DAT_005b9220 != 0x0) {
if ((0x1388 < param_1) && (local_54 = param_1 + -0x2710, 0x140 < DAT_005b9228)) {
local_54 = local_54 * 0x2;
}
if ((0x1388 < param_2) && (local_50 = param_2 + -0x2710, 0xc8 < DAT_005b922c)) {
local_50 = local_50 * 0x2;
}
if (pCVar1 == 0x0) {
local_38 = LPCSTR_005b9218;
}
if (param_6 != -0x1) {
iVar4 = FUN_00497282(param_3,local_38,param_4);
FUN_004968e7(local_54,local_50,iVar4,iVar3,(char)param_6);
}
for (local_28 = 0x0; (local_28 < local_48 && ((param_3 + local_28) != '\0')); local_28 = local_28 + 0x1) {
if ((param_3 + local_28) == '~') {
local_28 = local_28 + 0x1;
local_48 = local_48 + 0x1;
if ((param_3 + local_28) == '~')^ // goto LAB_00497530;
local_14 = param_7;
}
else {
// LAB_00497530:
local_14 = param_5;
}
cVar2 = FUN_00497765((param_3 + local_28),local_54,local_50,local_14,local_38);
local_54 = local_54 + CONCAT31(extraout_var,cVar2);
}
}
return;
}



void
FUN_00497567(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: u32,param_6: i32,param_7: u32,
param_8: *mut u32,byte param_9)

{
let cVar1: u8;
let mut iVar2: i32;
undefined3 extraout_var;
let mut local_30: u32;
let mut local_28: i32;
let mut local_24: i32;
let mut local_1c: i32;
let mut local_18: i32;

local_18 = 0x0;
iVar2 = FUN_004a06f6(param_8);
if (DAT_005b9220 != 0x0) {
if ((0x1388 < param_1) && (param_1 = param_1 + -0x2710, 0x140 < DAT_005b9228)) {
param_1 = param_1 * 0x2;
}
local_24 = param_1;
if ((0x1388 < param_2) && (param_2 = param_2 + -0x2710, 0xc8 < DAT_005b922c)) {
param_2 = param_2 * 0x2;
}
if (param_8 == 0x0) {
param_8 = LPCSTR_005b9218;
}
if (((param_6 != -0x1) || ((param_9 & 0x5) != 0x0)) || ((param_9 & 0xa) != 0x0)) {
if ((param_9 & 0x10) == 0x0) {
local_28 = param_4;
}
else {
local_28 = -0x2710;
}
local_18 = FUN_00497282(param_3,param_8,local_28);
}
if ((param_9 & 0x1) == 0x0) {
if ((param_9 & 0x4) != 0x0) {
param_1 = param_1 - local_18;
}
}
else {
param_1 = param_1 - local_18 / 0x2;
}
if ((param_9 & 0x2) == 0x0) {
if ((param_9 & 0x8) != 0x0) {
param_2 = param_2 - iVar2;
}
}
else {
param_2 = param_2 - iVar2 / 0x2;
}
if (param_6 != -0x1) {
if ((param_9 & 0x10) == 0x0) {
FUN_004968e7(param_1,param_2,local_18,iVar2,(char)param_6);
}
else {
if ((param_9 & 0x1) == 0x0) {
if ((param_9 & 0x4) != 0x0) {
local_24 = local_24 - param_4;
}
}
else {
local_24 = local_24 - param_4 / 0x2;
}
FUN_004968e7(local_24,param_2,param_4,iVar2,(char)param_6);
}
}
for (local_1c = 0x0; (local_1c < param_4 && ((param_3 + local_1c) != '\0')); local_1c = local_1c + 0x1) {
if ((param_3 + local_1c) == '~') {
local_1c = local_1c + 0x1;
param_4 = param_4 + 0x1;
if ((param_3 + local_1c) == '~')^ // goto LAB_0049772e;
local_30 = param_7;
}
else {
// LAB_0049772e:
local_30 = param_5;
}
cVar1 = FUN_00497765((param_3 + local_1c),param_1,param_2,local_30,param_8);
param_1 = param_1 + CONCAT31(extraout_var,cVar1);
}
}
return;
}



char  FUN_00497765(param_1: u32,param_2: i32,param_3: i32,param_4: u32,param_5: *mut u32)

{
let cVar1: u8;
let mut local_2c: u32;
let mut local_28: u32;

if (DAT_005b9220 == 0x0) {
cVar1 = '\0';
}
else {
if (DAT_005b91f0 == 0x0) {
cVar1 = FUN_004a0450(param_1,param_2,param_3,param_4,param_5,0x0,DAT_005b9228,0x0,DAT_005b922c);
}
else {
if ((DAT_005b9204 < param_2) || (DAT_005b9208 < param_3)) {
cVar1 = FUN_004a06b1(param_1,param_5);
}
else {
if (param_3 < DAT_005b91f8) {
local_2c = DAT_005b91f8 - param_3;
}
else {
local_2c = 0x0;
}
if (param_2 < DAT_005b91f4) {
local_28 = DAT_005b91f4 - param_2;
}
else {
local_28 = 0x0;
}
cVar1 = FUN_004a0450(param_1,param_2,param_3,param_4,param_5,local_28,(DAT_005b91f4 + DAT_005b91fc) - param_2,
local_2c,(DAT_005b91f8 + DAT_005b9200) - param_3);
}
}
}
return cVar1;
}



// WARNING: Could not reconcile some variable overlaps

fn FUN_00497896(param_1: i32,param_2: i32,param_3: i32,param_4: i32,undefined param_5)

{
    let mut iVar1: i32;
    let mut local_20: u32;
    let mut local_1c: u32;

    if (DAT_005b9220 == 0x0) {
        return;
    }
    if (DAT_005b91f0 == 0x0)^ // goto LAB_00497c3a;
    if (param_1 < DAT_005b91f4) {
        local_20 = 0x1000;
    }
    else {
        if (DAT_005b9204 < param_1) {
            local_20 = 0x100;
        }
        else {
            local_20 = 0x0;
        }
    }
    if (param_2 < DAT_005b91f8) {
        local_20 = CONCAT31(local_20._1_3_,0x10);
    }
    else {
        if (DAT_005b9208 < param_2) {
            local_20 = CONCAT31(local_20._1_3_,0x1);
        }
    }
    if (param_3 < DAT_005b91f4) {
        local_1c = 0x1000;
    }
    else {
        if (DAT_005b9204 < param_3) {
            local_1c = 0x100;
        }
        else {
            local_1c = 0x0;
        }
    }
    if (param_4 < DAT_005b91f8) {
        local_1c = CONCAT31(local_1c._1_3_,0x10);
    }
    else {
        if (DAT_005b9208 < param_4) {
            local_1c = CONCAT31(local_1c._1_3_,0x1);
        }
    }
    if ((local_1c & local_20) != 0x0) {
        return;
    }
    if ((local_20 | local_1c) == 0x0)^ // goto LAB_00497c3a;
    if ((local_20 & 0x1000) == 0x0) {
        if ((local_20 & 0x100) != 0x0) {
            iVar1 = ((param_1 - DAT_005b9204) * (param_4 - param_2) * 0xa + 0x5) / ((param_1 - param_3) * 0xa);
            param_1 = DAT_005b9204;^
            // goto LAB_00497a04;
        }
    }
    else {
        iVar1 = ((DAT_005b91f4 - param_1) * (param_4 - param_2) * 0xa + 0x5) / ((param_3 - param_1) * 0xa);
        param_1 = DAT_005b91f4;
        LAB_00497a04:
            param_2 = param_2 + iVar1;
        if ((DAT_005b9208 < param_2) && (DAT_005b9208 < param_4)) {
            return;
        }
        if ((param_2 < DAT_005b91f8) && (param_4 < DAT_005b91f8)) {
            return;
        }
    }
    if (param_2 < DAT_005b91f8) {
        iVar1 = ((DAT_005b91f8 - param_2) * (param_3 - param_1) * 0xa + 0x5) / ((param_4 - param_2) * 0xa);
        param_2 = DAT_005b91f8;
        LAB_00497ab4:
            param_1 = param_1 + iVar1;
        if ((DAT_005b9204 < param_1) && (DAT_005b9204 < param_3)) {
            return;
        }
        if ((param_1 < DAT_005b91f4) && (param_3 < DAT_005b91f4)) {
            return;
        }
    }
    else {
        if (DAT_005b9208 < param_2) {
            iVar1 = ((param_2 - DAT_005b9208) * (param_3 - param_1) * 0xa + 0x5) / ((param_2 - param_4) * 0xa);
            param_2 = DAT_005b9208;^
            // goto LAB_00497ab4;
        }
    }
    if ((local_1c & 0x1000) == 0x0) {
        if ((local_1c & 0x100) != 0x0) {
            iVar1 = ((param_3 - DAT_005b9204) * (param_2 - param_4) * 0xa + 0x5) / ((param_3 - param_1) * 0xa);
            param_3 = DAT_005b9204;^
            // goto LAB_00497b5a;
        }
    }
    else {
        iVar1 = ((DAT_005b91f4 - param_3) * (param_2 - param_4) * 0xa + 0x5) / ((param_1 - param_3) * 0xa);
        param_3 = DAT_005b91f4;
        LAB_00497b5a:
            param_4 = param_4 + iVar1;
        if ((DAT_005b9208 < param_2) && (DAT_005b9208 < param_4)) {
            return;
        }
        if ((param_2 < DAT_005b91f8) && (param_4 < DAT_005b91f8)) {
            return;
        }
    }
    if (param_4 < DAT_005b91f8) {
        iVar1 = ((DAT_005b91f8 - param_4) * (param_1 - param_3) * 0xa + 0x5) / ((param_2 - param_4) * 0xa);
        param_4 = DAT_005b91f8;
    }
    else {
        if (param_4 <= DAT_005b9208)^ // goto LAB_00497c3a;
        iVar1 = ((param_4 - DAT_005b9208) * (param_1 - param_3) * 0xa + 0x5) / ((param_4 - param_2) * 0xa);
        param_4 = DAT_005b9208;
    }
    param_3 = param_3 + iVar1;
    if ((DAT_005b9204 < param_1) && (DAT_005b9204 < param_3)) {
        return;
    }
    if ((param_1 < DAT_005b91f4) && (param_3 < DAT_005b91f4)) {
        return;
    }
    LAB_00497c3a:
        FUN_004ab3e4(param_1,param_2,param_3,param_4,param_5);
    return;
}



// WARNING: Could not reconcile some variable overlaps

fn FUN_00497c58(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: i32,param_6: i32,param_7: i32) -> i32

{
let mut bVar1: bool;
let mut iVar2: i32;
let mut uVar3: u32;
let mut uVar4: u32;
let mut local_60: i32;
let mut local_5c: i32;
let mut local_58: i32;
let mut local_54: i32;
let mut local_50: i32;
let mut local_4c: i32;
let mut local_48: i32;
let mut local_44: i32;
let mut local_40: i32;
let mut local_3c: u32;
let mut local_30: i32;
let mut local_2c: i32;
let mut local_28: u32;
let mut local_24: u32;
let mut local_18: i32;

local_2c = param_6;
if (DAT_005b9220 == 0x0) {
return 0x0;
}
if (DAT_005b91f0 == 0x0) {
// LAB_00498263:
iVar2 = FUN_004ab5b0(param_1,param_2,param_3,param_4,param_5,param_6,param_7);
return iVar2;
}
if (param_1 < DAT_005b91f4) {
local_28 = 0x1000;
}
else {
if (DAT_005b9204 < param_1) {
local_28 = 0x100;
}
else {
local_28 = 0x0;
}
}
if (param_2 < DAT_005b91f8) {
local_28 = CONCAT31(local_28._1_3_,0x10);
}
else {
if (DAT_005b9208 < param_2) {
local_28 = CONCAT31(local_28._1_3_,0x1);
}
}
if (param_3 < DAT_005b91f4) {
local_24 = 0x1000;
}
else {
if (DAT_005b9204 < param_3) {
local_24 = 0x100;
}
else {
local_24 = 0x0;
}
}
if (param_4 < DAT_005b91f8) {
local_24 = CONCAT31(local_24._1_3_,0x10);
}
else {
if (DAT_005b9208 < param_4) {
local_24 = CONCAT31(local_24._1_3_,0x1);
}
}
if ((local_24 & local_28) == 0x0) {
if ((local_28 | local_24) == 0x0)^ // goto LAB_00498263;
uVar3 = param_3 - param_1 >> 0x1f;
uVar4 = param_4 - param_2 >> 0x1f;
bVar1 = ((param_3 - param_1 ^ uVar3) - uVar3) <= ((param_4 - param_2 ^ uVar4) - uVar4);
if ((local_28 & 0x1000) == 0x0) {
if ((local_28 & 0x100) != 0x0) {
local_3c = ((param_1 - DAT_005b9204) * (param_4 - param_2) * 0xa + 0x5) / ((param_1 - param_3) * 0xa);
if (bVar1) {
local_48 = (local_3c ^ local_3c >> 0x1f) - (local_3c >> 0x1f);
}
else {
local_48 = param_1 - DAT_005b9204;
}
local_44 = local_48;
param_1 = DAT_005b9204;^
// goto LAB_00497eb5;
}
local_44 = 0x0;
}
else {
local_3c = ((DAT_005b91f4 - param_1) * (param_4 - param_2) * 0xa + 0x5) / ((param_3 - param_1) * 0xa);
if (bVar1) {
local_40 = (local_3c ^ local_3c >> 0x1f) - (local_3c >> 0x1f);
}
else {
local_40 = DAT_005b91f4 - param_1;
}
local_44 = local_40;
param_1 = DAT_005b91f4;
// LAB_00497eb5:
param_2 = param_2 + local_3c;
if (((DAT_005b9208 < param_2) && (DAT_005b9208 < param_4)) ||
((param_2 < DAT_005b91f8 && (param_4 < DAT_005b91f8))))^ // goto LAB_00497d4f;
}
if (param_2 < DAT_005b91f8) {
local_3c = ((DAT_005b91f8 - param_2) * (param_3 - param_1) * 0xa + 0x5) / ((param_4 - param_2) * 0xa);
if (bVar1) {
local_4c = DAT_005b91f8 - param_2;
}
else {
local_4c = (local_3c ^ local_3c >> 0x1f) - (local_3c >> 0x1f);
}
param_2 = DAT_005b91f8;
// LAB_00497fce:
param_6 = param_6 - (local_44 + local_4c);
param_1 = param_1 + local_3c;
if (((DAT_005b9204 < param_1) && (DAT_005b9204 < param_3)) ||
((param_1 < DAT_005b91f4 && (param_3 < DAT_005b91f4))))^ // goto LAB_00497d4f;
}
else {
if (DAT_005b9208 < param_2) {
local_3c = ((param_2 - DAT_005b9208) * (param_3 - param_1) * 0xa + 0x5) / ((param_2 - param_4) * 0xa);
if (bVar1) {
local_50 = param_2 - DAT_005b9208;
}
else {
local_50 = (local_3c ^ local_3c >> 0x1f) - (local_3c >> 0x1f);
}
param_2 = DAT_005b9208;
local_4c = local_50;^
// goto LAB_00497fce;
}
param_6 = param_6 - local_44;
}
for (; param_6 < 0x0; param_6 = param_6 + param_7) {
}
if ((local_24 & 0x1000) == 0x0) {
if ((local_24 & 0x100) != 0x0) {
local_3c = ((param_3 - DAT_005b9204) * (param_2 - param_4) * 0xa + 0x5) / ((param_3 - param_1) * 0xa);
if (bVar1) {
local_58 = (local_3c ^ local_3c >> 0x1f) - (local_3c >> 0x1f);
}
else {
local_58 = param_3 - DAT_005b91f4;
}
local_44 = local_58;
param_3 = DAT_005b9204;^
// goto LAB_004980e4;
}
local_44 = 0x0;
}
else {
local_3c = ((DAT_005b91f4 - param_3) * (param_2 - param_4) * 0xa + 0x5) / ((param_1 - param_3) * 0xa);
if (bVar1) {
local_54 = (local_3c ^ local_3c >> 0x1f) - (local_3c >> 0x1f);
}
else {
local_54 = DAT_005b91f4 - param_3;
}
local_44 = local_54;
param_3 = DAT_005b91f4;
// LAB_004980e4:
param_4 = param_4 + local_3c;
if (((DAT_005b9208 < param_2) && (DAT_005b9208 < param_4)) ||
((param_2 < DAT_005b91f8 && (param_4 < DAT_005b91f8))))^ // goto LAB_00497d4f;
}
if (param_4 < DAT_005b91f8) {
local_3c = ((DAT_005b91f8 - param_4) * (param_1 - param_3) * 0xa + 0x5) / ((param_2 - param_4) * 0xa);
if (bVar1) {
local_5c = DAT_005b91f8 - param_4;
}
else {
local_5c = (local_3c ^ local_3c >> 0x1f) - (local_3c >> 0x1f);
}
param_4 = DAT_005b91f8;
// LAB_004981f7:
local_44 = local_44 + local_5c;
param_3 = param_3 + local_3c;
if (((DAT_005b9204 < param_1) && (DAT_005b9204 < param_3)) ||
((param_1 < DAT_005b91f4 && (param_3 < DAT_005b91f4))))^ // goto LAB_00497d4f;
}
else {
if (DAT_005b9208 < param_4) {
local_3c = ((param_4 - DAT_005b9208) * (param_1 - param_3) * 0xa + 0x5) / ((param_4 - param_2) * 0xa);
if (bVar1) {
local_60 = param_4 - DAT_005b9208;
}
else {
local_60 = (local_3c ^ local_3c >> 0x1f) - (local_3c >> 0x1f);
}
param_4 = DAT_005b9208;
local_5c = local_60;^
// goto LAB_004981f7;
}
}
iVar2 = FUN_004ab5b0(param_1,param_2,param_3,param_4,param_5,param_6,param_7);
for (local_44 = iVar2 - local_44; local_44 < 0x0; local_44 = local_44 + param_7) {
}
local_18 = local_44;
}
else {
// LAB_00497d4f:
uVar3 = param_3 - param_1 >> 0x1f;
uVar4 = param_4 - param_2 >> 0x1f;
if (((param_4 - param_2 ^ uVar4) - uVar4) < ((param_3 - param_1 ^ uVar3) - uVar3)) {
uVar3 = param_3 - param_1 >> 0x1f;
local_30 = (param_3 - param_1 ^ uVar3) - uVar3;
}
else {
uVar3 = param_4 - param_2 >> 0x1f;
local_30 = (param_4 - param_2 ^ uVar3) - uVar3;
}
for (local_2c = local_2c - local_30; local_2c < 0x0; local_2c = local_2c + param_7) {
}
local_18 = local_2c;
}
return local_18;
}



fn FUN_00498293(param_1: *mut i32,param_2: i32,param_3: i32,param_4: i32,param_5: i32,param_6: i32)

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let mut iVar5: i32;
    let mut uVar6: u32;
    let mut uVar7: u32;
    let mut local_38: i32;
    let mut local_34: i32;
    let mut local_30: i32;
    let mut local_2c: i32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    local_30 = 0x0;
    iVar1 = *param_1;
    iVar2 = param_1[0x1];
    if (DAT_005b9220 != 0x0) {
        local_20 = 0x186a0;
        local_1c = 0x186a0;
        local_18 = 0x0;
        local_14 = 0x0;
        for (local_2c = 0x0; local_2c <= param_2 + -0x1; local_2c = local_2c + 0x1) {
            iVar4 = *param_1;
            iVar3 = param_1[0x1];
            local_38 = iVar1;
            local_34 = iVar2;
            if (local_2c != param_2 + -0x1) {
                local_38 = param_1[0x2];
                local_34 = param_1[0x3];
            }
            if (iVar4 < local_20) {
                local_20 = iVar4;
            }
            if (iVar3 < local_1c) {
                local_1c = iVar3;
            }
            if (local_18 < iVar4) {
                local_18 = iVar4;
            }
            if (local_14 < iVar3) {
                local_14 = iVar3;
            }
            if (0x1 < param_3) {
                iVar5 = (local_30 + param_6) % param_5;
                uVar6 = local_38 - iVar4 >> 0x1f;
                uVar7 = local_34 - iVar3 >> 0x1f;
                if (((local_34 - iVar3 ^ uVar7) - uVar7) < ((local_38 - iVar4 ^ uVar6) - uVar6)) {
                    FUN_00497c58(iVar4,iVar3 + 0x1,local_38,local_34 + 0x1,param_4,iVar5,param_5);
                    if (0x2 < param_3) {
                        FUN_00497c58(iVar4,iVar3 + -0x1,local_38,local_34 + -0x1,param_4,((local_30 - param_6) + param_5) % param_5,
                                     param_5);
                    }
                }
                else {
                    FUN_00497c58(iVar4 + 0x1,iVar3,local_38 + 0x1,local_34,param_4,iVar5,param_5);
                    if (0x2 < param_3) {
                        FUN_00497c58(iVar4 + -0x1,iVar3,local_38 + -0x1,local_34,param_4,(local_30 + param_6) % param_5,param_5);
                    }
                }
            }
            iVar4 = FUN_00497c58(iVar4,iVar3,local_38,local_34,param_4,local_30,param_5);
            local_30 = (iVar4 + 0x1) % param_5;
            param_1 = param_1 + 0x2;
        }
    }
    return;
}



fn FUN_0049848b(param_1: *mut u8,param_2: i32,param_3: i32,param_4: i32)

{
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: *mut u8;
    let mut local_14: *mut u8;

    local_18 = param_1;
    local_14 = (param_2 + param_3 + -0x1);
    for (local_1c = param_4; local_1c != 0x0; local_1c = local_1c + -0x1) {
        for (local_20 = param_3; local_20 != 0x0; local_20 = local_20 + -0x1) {
            *local_14 = *local_18;
            local_14 = local_14 + -0x1;
            local_18 = local_18 + 0x1;
        }
        local_14 = local_14 + param_3 * 0x2;
    }
    return;
}



fn FUN_004984ed(param_1: *mut u8,param_2: i32,param_3: i32,param_4: i32)

{
    let mut puVar1: *mut u8;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: *mut u8;
    let mut local_14: *mut u8;

    local_18 = param_1;
    puVar1 = (param_2 + param_4);
    for (local_1c = param_4; local_14 = puVar1 + -0x1, puVar1 = local_14, local_1c != 0x0; local_1c = local_1c + -0x1) {
        for (local_20 = param_3; local_20 != 0x0; local_20 = local_20 + -0x1) {
            *local_14 = *local_18;
            local_14 = local_14 + param_4;
            local_18 = local_18 + 0x1;
        }
    }
    return;
}



fn FUN_00498556(param_1: *mut u8,param_2: i32,param_3: i32,param_4: i32)

{
    let mut puVar1: *mut u8;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: *mut u8;
    let mut local_14: *mut u8;

    local_18 = param_1;
    puVar1 = (param_4 * param_3 + param_2);
    for (local_1c = param_4; local_14 = puVar1 + -0x1, puVar1 = local_14, local_1c != 0x0; local_1c = local_1c + -0x1) {
        for (local_20 = param_3; local_20 != 0x0; local_20 = local_20 + -0x1) {
            *local_14 = *local_18;
            local_14 = local_14 + -param_4;
            local_18 = local_18 + 0x1;
        }
    }
    return;
}



fn FUN_004985c3(param_1: *mut u8,param_2: i32,param_3: i32,param_4: i32)

{
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: *mut u8;
    let mut local_14: *mut u8;

    local_18 = param_1;
    local_14 = (param_4 * param_3 + param_2 + -0x1);
    for (local_1c = param_4; local_1c != 0x0; local_1c = local_1c + -0x1) {
        for (local_20 = param_3; local_20 != 0x0; local_20 = local_20 + -0x1) {
            *local_14 = *local_18;
            local_14 = local_14 + -0x1;
            local_18 = local_18 + 0x1;
        }
    }
    return;
}



fn FUN_00498620(param_1: *mut u8,param_2: i32,param_3: i32,param_4: i32)

{
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: *mut u8;
    let mut local_14: *mut u8;

    local_18 = param_1;
    local_14 = ((param_4 * param_3 + param_2) - param_3);
    for (local_1c = param_4; local_1c != 0x0; local_1c = local_1c + -0x1) {
        for (local_20 = param_3; local_20 != 0x0; local_20 = local_20 + -0x1) {
            *local_14 = *local_18;
            local_14 = local_14 + 0x1;
            local_18 = local_18 + 0x1;
        }
        local_14 = local_14 + param_3 * -0x2;
    }
    return;
}



fn FUN_00498688(param_1: *mut u8,param_2: i32,param_3: i32,param_4: i32)

{
    let mut puVar1: *mut u8;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: *mut u8;
    let mut local_14: *mut u8;

    local_18 = param_1;
    local_14 = ((param_4 * param_3 + param_2) - param_4);
    for (local_1c = param_4; puVar1 = local_14, local_1c != 0x0; local_1c = local_1c + -0x1) {
        for (local_20 = param_3; local_20 != 0x0; local_20 = local_20 + -0x1) {
            *local_14 = *local_18;
            local_14 = local_14 + -param_4;
            local_18 = local_18 + 0x1;
        }
        local_14 = puVar1 + 0x1;
    }
    return;
}



fn FUN_004986f7(param_1: *mut u8,param_2: *mut u8,param_3: i32,param_4: i32)

{
    let mut puVar1: *mut u8;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: *mut u8;
    let mut local_14: *mut u8;

    local_18 = param_1;
    local_14 = param_2;
    for (local_1c = param_4; puVar1 = local_14, local_1c != 0x0; local_1c = local_1c + -0x1) {
        for (local_20 = param_3; local_20 != 0x0; local_20 = local_20 + -0x1) {
            *local_14 = *local_18;
            local_14 = local_14 + param_4;
            local_18 = local_18 + 0x1;
        }
        local_14 = puVar1 + 0x1;
    }
    return;
}



fn FUN_0049875c(byte *param_1,param_2: i32,param_3: i32)

{
    let mut local_14: i32;

    for (local_14 = param_2; local_14 != 0x0; local_14 = local_14 + -0x1) {
        *param_1 = *(*param_1 + param_3);
        param_1 = param_1 + 0x1;
    }
    return;
}



fn FUN_00498799(byte *param_1,param_2: *mut u8,param_3: i32,param_4: i32)

{
    let mut local_14: i32;

    for (local_14 = param_3; local_14 != 0x0; local_14 = local_14 + -0x1) {
        *param_2 = (*param_1 + param_4);
        param_1 = param_1 + 0x1;
        param_2 = param_2 + 0x1;
    }
    return;
}



i32 *  FUN_004987d6(param_1: *mut i32,param_2: i32,param_3: i32,param_4: i32,param_5: i32)

{
i32 local_24 [0x5];

FUN_00498b2e(local_24);
*param_1 = param_2;
param_1[0x1] = param_3;
param_1[0x2] = param_4;
param_1[0x3] = param_5;
FUN_004aa45a(local_24,param_1,param_1);
param_1[0x4] = DAT_005b91f0;
DAT_005b91f0 = param_1;
FUN_004988ab(param_1);
return param_1;
}



i32 *  FUN_00498852(param_1: *mut i32,param_2: *mut i32)

{
i32 local_24 [0x5];

FUN_00498b2e(local_24);
FUN_004aa45a(local_24,param_2,param_1);
param_1[0x4] = DAT_005b91f0;
DAT_005b91f0 = param_1;
FUN_004988ab(param_1);
return param_1;
}



fn FUN_004988ab(param_1: *mut i32)

{
    DAT_005b91f4 = *param_1;
    DAT_005b91f8 = param_1[0x1];
    DAT_005b91fc = param_1[0x2];
    DAT_005b9200 = param_1[0x3];
    DAT_005b9204 = *param_1 + param_1[0x2] + -0x1;
    DAT_005b9208 = param_1[0x1] + param_1[0x3] + -0x1;
    return;
}



i32 *  FUN_0049890a(param_1: *mut i32)

{
let piVar1: *mut i32;;

while ((DAT_005b91f0 != 0x0 && (param_1 != DAT_005b91f0))) {
if (DAT_005b91f0 != 0x0) {
piVar1 = FUN_0049890a(DAT_005b91f0);
FUN_0049af50(piVar1);
}
}
if (param_1 == DAT_005b91f0) {
DAT_005b91f0 = param_1[0x4];
}
if (DAT_005b91f0 != 0x0) {
FUN_004988ab(DAT_005b91f0);
}
return param_1;
}



fn FUN_00498999(param_1: i32,param_2: i32,param_3: i32,param_4: i32)

{
    i32 local_48 [0x4];
    let local_38: *mut i32;;
    let local_34: *mut i32;;
    let local_2c: *mut i32;;
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_1c: i32;

    local_38 = FUN_004a2831(0x14);
    if (local_38 != 0x0) {
        local_28 = param_1;
        local_24 = param_2;
        local_20 = param_3;
        local_1c = param_4;
        local_34 = local_38;
        local_2c = local_38;
        FUN_00498b2e(local_48);
        *local_2c = local_28;
        local_2c[0x1] = local_24;
        local_2c[0x2] = local_20;
        local_2c[0x3] = local_1c;
        FUN_004aa45a(local_48,local_2c,local_2c);
        local_2c[0x4] = DAT_005b91f0;
        DAT_005b91f0 = local_2c;
        FUN_004988ab(local_2c);
    }
    return;
}



fn FUN_00498a5b(param_1: *mut i32)

{
    i32 local_3c [0x4];
    let local_2c: *mut i32;;
    let local_28: *mut i32;;
    let local_20: *mut i32;;
    let local_1c: *mut i32;;

    local_2c = FUN_004a2831(0x14);
    if (local_2c != 0x0) {
        local_1c = param_1;
        local_28 = local_2c;
        local_20 = local_2c;
        FUN_00498b2e(local_3c);
        FUN_004aa45a(local_3c,local_1c,local_20);
        local_20[0x4] = DAT_005b91f0;
        DAT_005b91f0 = local_20;
        FUN_004988ab(local_20);
    }
    return;
}



fn FUN_00498ae4()

{
    let piVar1: *mut i32;;

    if ((DAT_005b91f0 != 0x0) && (DAT_005b91f0 != 0x0)) {
        piVar1 = FUN_0049890a(DAT_005b91f0);
        FUN_0049af50(piVar1);
    }
    return;
}



fn FUN_00498b2e(param_1: *mut u32)

{
    if (DAT_005b91f0 == 0x0) {
        *param_1 = 0x0;
        param_1[0x1] = 0x0;
        param_1[0x2] = DAT_005b9228;
        param_1[0x3] = DAT_005b922c;
    }
    else {
        *param_1 = DAT_005b91f4;
        param_1[0x1] = DAT_005b91f8;
        param_1[0x2] = DAT_005b91fc;
        param_1[0x3] = DAT_005b9200;
    }
    return;
}



fn FUN_00498ba4(param_1: *mut u32,param_2: u32,param_3: i32,param_4: i32) -> *mut u32

{
    i32 local_5c [0x5];
    let mut local_48: i32;
    let mut local_44: i32;
    let mut local_40: i32;
    let mut local_3c: i32;
    let local_38: *mut i32;;
    let local_34: *mut i32;;
    let local_2c: *mut i32;;
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_1c: i32;

    param_1[0x1] = DAT_005b9220;
    param_1[0x2] = DAT_005b9224;
    param_1[0x3] = DAT_005b9228;
    param_1[0x4] = DAT_005b922c;
    DAT_005b9220 = param_2;
    DAT_005b922c = param_3;
    DAT_005b9228 = param_4;
    DAT_005b9224 = param_4;
    *param_1 = DAT_005b91f0;
    DAT_005b91f0 = 0x0;
    local_48 = 0x0;
    local_44 = 0x0;
    local_40 = param_4;
    local_3c = param_3;
    local_38 = FUN_004a2831(0x14);
    if (local_38 != 0x0) {
        local_28 = local_48;
        local_24 = local_44;
        local_20 = local_40;
        local_1c = local_3c;
        local_34 = local_38;
        local_2c = local_38;
        FUN_00498b2e(local_5c);
        *local_2c = local_28;
        local_2c[0x1] = local_24;
        local_2c[0x2] = local_20;
        local_2c[0x3] = local_1c;
        FUN_004aa45a(local_5c,local_2c,local_2c);
        local_2c[0x4] = DAT_005b91f0;
        DAT_005b91f0 = local_2c;
        FUN_004988ab(local_2c);
    }
    DAT_005b920c = DAT_005b920c + 0x1;
    return param_1;
}



i32 **  FUN_00498cf4(i32 **param_1)

{
let piVar1: *mut i32;;

DAT_005b9220 = param_1[0x1];
DAT_005b922c = param_1[0x4];
DAT_005b9228 = param_1[0x3];
DAT_005b9224 = param_1[0x2];
if ((DAT_005b91f0 != 0x0) && (DAT_005b91f0 != 0x0)) {
piVar1 = FUN_0049890a(DAT_005b91f0);
FUN_0049af50(piVar1);
}
if (*param_1 != 0x0) {
DAT_005b91f0 = *param_1;
FUN_004988ab(DAT_005b91f0);
}
DAT_005b920c = DAT_005b920c + -0x1;
return param_1;
}



bool FUN_00498d99()

{
return DAT_005b91f0 != 0x0;
}



i32 *  FUN_00498dce(param_1: i32,param_2: u32)

{
if (param_1 == 0x0) {
return 0x0;
}
FUN_004a7132(param_1,*(param_1 + -0x4),param_2);
return (param_1 + -0x4);
}



fn FUN_00498df5(LPCSTR param_1)

{
    FUN_0049af50(param_1);
    return;
}



char **  FUN_00498e10(param_1: &mut String,byte *param_2)

{
let ppcVar1: *mut *mut char;
let mut uVar2: u32;
let mut iVar3: i32;
u32 local_20 [0x5];
let mut local_c: i32;

ppcVar1 = FUN_0049c4bd(param_1,&DAT_004c3385);
if (ppcVar1 == 0x0) {
return 0x0;
}
uVar2 = FUN_004a7970(local_20,0x10,0x1,ppcVar1);
if (uVar2 != 0x0) {
iVar3 = FUN_004a9800(local_20,PTR_DAT_004bf8f8,0xc);
if (iVar3 == 0x0) {
while( true ) {
uVar2 = FUN_004a7970(local_20,0xc,0x1,ppcVar1);
if (uVar2 == 0x0) break;
FUN_004a7970(&local_c,0x4,0x1,ppcVar1);
if ((byte)local_20[0] == 0x0) break;
iVar3 = FUN_004a2f10(local_20,param_2);
if (iVar3 == 0x0) {
FUN_004aa75c(ppcVar1,local_c,0x0);
return ppcVar1;
}
}
}
}
FUN_0049ca40(ppcVar1);
return 0x0;
}



i32 *  FUN_00498ee0(param_1: *mut i32,LPSTR param_2,LPSTR param_3)

{
let ppcVar1: *mut *mut char;
let mut iVar2: i32;
let mut iVar3: i32;
let puVar4: *mut u32;
let sStack20: i16;

ppcVar1 = FUN_00498e10(param_2,param_3);
if (ppcVar1 == 0x0) {
*param_1 = 0x0;
return param_1;
}
*param_1 = 0x0;
FUN_004a7970(param_1,0x2,0x1,ppcVar1);
iVar2 = FUN_0049c2c9(*param_1 * 0x6);
param_1[0x1] = iVar2;
if (iVar2 != 0x0) {
FUN_004a0430(param_1[0x1],0x0,*param_1 * 0x6);
puVar4 = param_1[0x1];
iVar2 = 0x0;
if (0x0 < *param_1) {
loop {
FUN_004a7970(puVar4,0x2,0x1,ppcVar1);
FUN_004a7970(&sStack20,0x2,0x1,ppcVar1);
iVar3 = FUN_0049c2c9(sStack20 + 0x1);
*(puVar4 + 0x2) = iVar3;
if (iVar3 == 0x0) {
pop_err_msg_box_and_exit_004a02f5(s_Memory_Error__004c3388);
}
FUN_004a7970((puVar4 + 0x2),0x1,sStack20,ppcVar1);
(sStack20 + *(puVar4 + 0x2)) = 0x0;
iVar2 = iVar2 + 0x1;
puVar4 = (puVar4 + 0x6);
} while (iVar2 < *param_1);
}
}
FUN_0049ca40(ppcVar1);
return param_1;
}



char *  FUN_00499050(param_1: *mut i32,param_2: i32)

{
let mut iVar1: i32;
short *psVar2;
let mut iVar3: i32;
short *psVar4;

psVar4 = (short *)param_1[0x1];
iVar3 = *param_1;
loop {
iVar1 = iVar3;
if (iVar1 < 0x1) {
return s__null__004c3396;
}
while( true ) {
iVar3 = iVar1 >> 0x1;
psVar2 = psVar4 + iVar3 * 0x3;
if (param_2 == *psVar2) {
return *(psVar2 + 0x1);
}
if (param_2 < *psVar2) break;
psVar4 = psVar2 + 0x3;
iVar1 = (iVar1 - iVar3) + -0x1;
if (iVar1 < 0x1) {
return s__null__004c3396;
}
}
} while( true );
}



i32 *  FUN_004990e0(param_1: *mut u32,param_2: i32,param_3: &mut String,byte *param_4)

{
let piVar1: *mut i32;;
let piVar2: *mut i32;;
let ppcVar3: *mut *mut char;
let mut uVar4: u32;
let mut uVar5: u32;
let mut pCVar6: String;;
let mut uVar7: u32;
let mut uVar8: u32;
let mut uVar9: u32;
let mut uVar10: u32;
let mut uVar11: u32;
let mut uVar12: u32;
let mut uVar13: u32;
let mut uVar14: u32;
let puVar15: *mut u32;
let mut iVar16: i32;
let puVar17: *mut u32;
let puVar18: *mut u32;
let bVar19: u8;
u32 **ppuVar20;
let mut local_80: u32;
let local_7c: *mut u32;
let local_78: *mut u32;
let local_74: *mut u32;
let local_70: *mut u32;
let mut local_6c: u32;
let local_68: *mut u32;
let local_64: *mut u32;
let local_60: *mut u32;
let local_5c: *mut u32;
let local_58: *mut u32;
let local_54: *mut u32;
let mut local_50: u32;
let mut local_4c: u32;
let local_48: *mut u32;
let local_44: *mut u32;
let local_40: *mut u32;
let local_3c: *mut u32;
let local_38: *mut u32;
let local_34: *mut u32;
let local_30: *mut u32;
let mut local_2c: String;;
let mut local_28: i32;
let local_24: *mut *mut char;
let mut local_20: u32;
let mut local_1c: String;;
let mut local_18: String;;
let local_14: *mut i32;;

piVar1 = FUN_0049a030(param_1,param_2,0x0,0x0,0x0,0x0,0x0,0x0,0x0,0x0);
$1: &mut String(piVar1 + 0x45) = &PTR_FUN_004c3d34;
*(piVar1 + 0x5d) = 0x0;
*(piVar1 + 0x49) = 0x0;
*(piVar1 + 0x4d) = 0x0;
*(piVar1 + 0x51) = 0x0;
*(piVar1 + 0x59) = 0x0;
*(piVar1 + 0x55) = *(piVar1 + 0x59);
piVar2 = FUN_004a2831(0x16);
if (piVar2 != 0x0) {
local_14 = piVar2;
FUN_004a0430(piVar2,0x0,0x16);
ppcVar3 = FUN_00498e10(param_3,param_4);
if (ppcVar3 == 0x0) {
pop_err_msg_box_and_exit_004a02f5(s_Invalid_copy_of_game____004c339d);
return piVar1;
}
uVar4 = FUN_004a7970(piVar2 + 0x1,0x2,0x1,ppcVar3);
local_20 = FUN_004a7970((piVar2 + 0x6),0x2,0x1,ppcVar3);
local_20 = uVar4 & local_20;
uVar5 = FUN_004a7970(piVar2 + 0x2,0x2,0x1,ppcVar3);
local_20 = local_20 & uVar5;
uVar5 = FUN_004a7970((piVar2 + 0xa),0x2,0x1,ppcVar3);
local_20 = local_20 & uVar5;
uVar5 = FUN_004a7970(piVar2 + 0x3,0x2,0x1,ppcVar3);
local_20 = local_20 & uVar5;
uVar5 = FUN_004a7970((piVar2 + 0x2),0x2,0x1,ppcVar3);
local_2c = 0x0;
local_20 = local_20 & uVar5;
for (local_28 = 0x0; (local_20 != 0x0 && (local_28 < *local_14 >> 0x10)); local_28 = local_28 + 0x1) {
pCVar6 = FUN_004a2831(0x38);
if (pCVar6 == 0x0) {
return piVar1;
}
local_18 = pCVar6;
FUN_004a0430(pCVar6,0x0,0x38);
local_24 = ppcVar3;
local_1c = pCVar6;
uVar5 = FUN_004a7970((pCVar6 + 0x4),0x4,0x1,ppcVar3);
uVar7 = FUN_004a7970((pCVar6 + 0xc),0x4,0x1,ppcVar3);
uVar8 = FUN_004a7970((pCVar6 + 0x10),0x4,0x1,ppcVar3);
uVar9 = FUN_004a7970((pCVar6 + 0x14),0x4,0x1,ppcVar3);
uVar10 = FUN_004a7970((pCVar6 + 0x1c),0x4,0x1,ppcVar3);
uVar11 = FUN_004a7970((pCVar6 + 0x20),0x4,0x1,ppcVar3);
uVar12 = FUN_004a7970((pCVar6 + 0x24),0x4,0x1,ppcVar3);
uVar13 = FUN_004a7970((pCVar6 + 0x28),0x4,0x1,ppcVar3);
uVar14 = FUN_004a7970((pCVar6 + 0x2c),0x4,0x1,ppcVar3);
uVar14 = uVar5 & uVar7 & uVar8 & uVar9 & uVar10 & uVar11 & uVar12 & uVar13 & uVar14;
puVar15 = FUN_00499920(ppcVar3);
(pCVar6 + 0x8) = puVar15;
switch(*(pCVar6 + 0x4)) {
case 0x0:
case 0x6:
break;
case 0x1:
case 0x3:
puVar15 = FUN_00499920(local_24);
(local_1c + 0x30) = puVar15;
puVar15 = FUN_00499920(local_24);
(local_1c + 0x34) = puVar15;
break;
case 0x2:
case 0x7:
puVar15 = FUN_00499920(local_24);
(local_1c + 0x30) = puVar15;
if (puVar15 == 0x0)^ // goto switchD_00499343_caseD_8;
break;
case 0x4:
uVar5 = FUN_004a7970((pCVar6 + 0x30),0x4,0x1,ppcVar3);
uVar14 = uVar14 & uVar5;
break;
case 0x5:
uVar5 = FUN_004a7970((pCVar6 + 0x30),0x4,0x1,ppcVar3);
uVar7 = FUN_004a7970((pCVar6 + 0x34),0x4,0x1,ppcVar3);
uVar14 = uVar14 & uVar5 & uVar7;
break;
default:
switchD_00499343_caseD_8:
uVar14 = 0x0;
}
local_20 = local_20 & uVar14;
if (local_20 == 0x0) {
FUN_0049af50(local_18);
}
else {
if (local_2c == 0x0) {
*(local_14 + 0x12) = local_18;
}
else {
*(local_2c + 0x18) = local_18;
}
local_2c = local_18;
*(local_18 + 0x18) = 0x0;
}
}
FUN_0049ca40(ppcVar3);
if (local_20 != 0x0) {
iVar16 = FUN_00499fb0(*(local_14 + 0x2) >> 0x10);
*(piVar1 + 0x1d) = iVar16;
iVar16 = FUN_00499ff0(local_14[0x1] >> 0x10);
*(piVar1 + 0x21) = iVar16;
iVar16 = FUN_00499fb0(*(local_14 + 0x6) >> 0x10);
*(piVar1 + 0x25) = iVar16;
iVar16 = FUN_00499ff0(local_14[0x2] >> 0x10);
*(piVar1 + 0x29) = iVar16;
iVar16 = *(local_14 + 0xa);
*(piVar1 + 0x31) = 0xff;
*(piVar1 + 0x35) = 0x0;
*(piVar1 + 0x2d) = iVar16 >> 0x10;
*(i32 **)(piVar1 + 0xf1) = local_14;
ppuVar20 = *(u32 ***)(local_14 + 0x12);
iVar16 = 0x0;
if (ppuVar20 != (u32 **)0x0) {
while (iVar16 < *local_14 >> 0x10) {
switch(ppuVar20[0x1]) {
case 0x1:
puVar17 = ppuVar20[0xc];
local_30 = ppuVar20[0xd];
if (((*(ppuVar20 + 0x9) & 0x2) == 0x0) ||
(((puVar17 == 0x0 ||
(puVar17 = FUN_00499c00(local_14,puVar17,&local_50), puVar17 != 0x0)) &&
((ppuVar20[0xd] == 0x0 ||
(local_30 = FUN_00499c00(local_14,ppuVar20[0xd],&local_4c),
local_30 != 0x0)))))) {
puVar18 = FUN_004a2831(0x5d);
if (puVar18 != 0x0) {
local_48 = ppuVar20[0x4];
local_44 = ppuVar20[0x5];
local_40 = ppuVar20[0x7];
local_3c = ppuVar20[0x8];
local_38 = ppuVar20[0x9];
local_34 = ppuVar20[0xa];
puVar18 = FUN_0049a030(puVar18,piVar1,ppuVar20[0x3],local_48,local_44,local_40,
local_3c,local_38,local_34,ppuVar20[0xb]);
$1: &mut String(puVar18 + 0x45) = &PTR_FUN_004c3d94;
*(puVar18 + 0x4d) = 0x0;
*(puVar18 + 0x49) = 0x2;
(puVar18 + 0x51) = puVar17;
(puVar18 + 0x55) = local_30;
}
if (puVar18 != 0x0) {
*ppuVar20 = puVar18;
FUN_0049bf40(piVar1,puVar18);
}
}
break;
case 0x2:
puVar17 = ppuVar20[0xc];
if ((((*(ppuVar20 + 0x9) & 0x1) == 0x0) || (puVar17 == 0x0)) ||
(puVar17 = FUN_00499c00(local_14,puVar17,&local_6c), puVar17 != 0x0)) {
puVar18 = FUN_004a2831(0x51);
if (puVar18 != 0x0) {
local_68 = ppuVar20[0x3];
local_58 = ppuVar20[0x9];
local_64 = ppuVar20[0x4];
local_54 = ppuVar20[0xa];
local_60 = ppuVar20[0x5];
local_5c = ppuVar20[0x7];
puVar18 = FUN_0049a030(puVar18,piVar1,local_68,local_64,local_60,local_5c,
ppuVar20[0x8],local_58,local_54,ppuVar20[0xb]);
*(puVar18 + 0x4d) = DAT_004bf958;
$1: &mut String(puVar18 + 0x45) = &PTR_FUN_004c3f14;
bVar19 = *(puVar18 + 0x2e) | 0x78;
*(puVar18 + 0x2e) = bVar19;
(puVar18 + 0x49) = puVar17;
if ((*(puVar18 + 0x2d) & 0xc) != 0x0) {
*(puVar18 + 0x2e) = bVar19 & 0xbf;
}
}
if (puVar18 != 0x0) {
*ppuVar20 = puVar18;
FUN_0049bf40(piVar1,puVar18);
}
}
break;
case 0x3:
puVar17 = FUN_004a2831(0x89);
if (puVar17 != 0x0) {
puVar17 = FUN_004a5f80(puVar17,piVar1,ppuVar20[0x3],ppuVar20[0x4],ppuVar20[0x5],
ppuVar20[0x7],ppuVar20[0x8],ppuVar20[0x9],ppuVar20[0xa],ppuVar20[0xb],
ppuVar20[0xc],ppuVar20[0xd]);
}
if (puVar17 != 0x0) {
*ppuVar20 = puVar17;
FUN_0049bf40(piVar1,puVar17);
}
break;
case 0x4:
puVar17 = FUN_004a2831(0x6d);
if (puVar17 != 0x0) {
puVar17 = FUN_004abdf0(puVar17,piVar1,ppuVar20[0x3],ppuVar20[0x4],ppuVar20[0x5],
ppuVar20[0x7],ppuVar20[0x8],ppuVar20[0x9],ppuVar20[0xa],ppuVar20[0xb],
ppuVar20[0xc]);
}
if (puVar17 != 0x0) {
*ppuVar20 = puVar17;
FUN_0049bf40(piVar1,puVar17);
}
break;
case 0x5:
puVar17 = FUN_004a2831(0x69);
if (puVar17 != 0x0) {
puVar17 = FUN_004a7c00(puVar17,piVar1,ppuVar20[0x3],ppuVar20[0x4],ppuVar20[0x5],
ppuVar20[0x7],ppuVar20[0x8],ppuVar20[0x9],ppuVar20[0xa],ppuVar20[0xb],
ppuVar20[0xc],ppuVar20[0xd],uVar4);
}
if (puVar17 != 0x0) {
*ppuVar20 = puVar17;
FUN_0049bf40(piVar1,puVar17);
}
break;
case 0x6:
puVar17 = FUN_004a2831(0x65);
if (puVar17 != 0x0) {
puVar17 = FUN_004ad360(puVar17,piVar1,ppuVar20[0x3],ppuVar20[0x4],ppuVar20[0x5],
ppuVar20[0x7],ppuVar20[0x8],ppuVar20[0x9],ppuVar20[0xa],ppuVar20[0xb],0x0
);
}
if (puVar17 != 0x0) {
*ppuVar20 = puVar17;
FUN_0049bf40(piVar1,puVar17);
}
break;
case 0x7:
pCVar6 = FUN_00499c00(local_14,ppuVar20[0xc],&local_80);
puVar17 = FUN_004a2831(0x5d);
if (puVar17 != 0x0) {
local_7c = ppuVar20[0x3];
local_78 = ppuVar20[0x4];
local_74 = ppuVar20[0x5];
local_70 = ppuVar20[0x8];
puVar17 = FUN_0049a030(puVar17,piVar1,local_7c,local_78,local_74,ppuVar20[0x7],
local_70,ppuVar20[0x9],0x0,0x0);
*(puVar17 + 0x4d) = pCVar6;
$1: &mut String(puVar17 + 0x45) = &PTR_FUN_004c3fc4;
*(puVar17 + 0x49) = 0x0;
*(puVar17 + 0x59) = 0x2;
*(puVar17 + 0x55) = *(puVar17 + 0x25) * *(puVar17 + 0x29);
}
if (puVar17 != 0x0) {
*ppuVar20 = puVar17;
FUN_0049bf40(piVar1,puVar17);
}
}
if (*ppuVar20 == 0x0) {
return piVar1;
}
ppuVar20 = (u32 **)ppuVar20[0x6];
iVar16 = iVar16 + 0x1;
if (ppuVar20 == (u32 **)0x0) {
return piVar1;
}
}
}
}
}
return piVar1;
}



fn FUN_00499920(char **param_1) -> *mut u32

{
    let mut uVar1: u32;
    let puVar2: *mut u32;
    let mut uVar3: u32;
    let local_c: i16;

    uVar1 = FUN_004a7970(&local_c,0x2,0x1,param_1);
    if ((local_c != 0x0) && (uVar1 != 0x0)) {
        puVar2 = FUN_0049c2c9(local_c + 0x1);
        uVar3 = FUN_004a7970(puVar2,local_c,0x1,param_1);
        if ((uVar3 & uVar1) != 0x0) {
            (puVar2 + local_c) = 0x0;
            return puVar2;
        }
        FUN_0049af50(puVar2);
        return 0x0;
    }
    return 0x0;
}



fn FUN_00499b30(param_1: u32,LPCSTR param_2)

{
    LPCSTR *ppCVar1;
    let mut iVar2: i32;
    let piVar3: *mut i32;;
    LPCSTR *ppCVar4;
    let piVar5: *mut i32;;

    ppCVar4 = *(LPCSTR **)(param_2 + 0xe);
    while (ppCVar4 != 0x0) {
        if (ppCVar4[0x1] != 0x0) {
            FUN_0049af50(ppCVar4[0x1]);
        }
        if (*ppCVar4 != 0x0) {
            FUN_0049af50(*ppCVar4);
        }
        ppCVar1 = ppCVar4[0x3];
        FUN_0049af50(ppCVar4);
        ppCVar4 = ppCVar1;
    }
    *(param_2 + 0xe) = 0x0;
    piVar5 = *(i32 **)(param_2 + 0x12);
    loop {
    if (piVar5 == 0x0) {
        FUN_0049af50(param_2);
        return;
    }
    iVar2 = *piVar5;
    if ((iVar2 != 0x0) && (iVar2 != 0x0)) {
        ((*(iVar2 + 0x45) + 0x8))(iVar2,0x2);
    }
    if (piVar5[0x2] != 0x0) {
        FUN_0049af50(piVar5[0x2]);
    }
    switch(piVar5[0x1]) {
        case 0x1:
            case 0x3:
        if (piVar5[0xd] != 0x0) {
            FUN_0049af50(piVar5[0xd]);
        }
        case 0x2:
            case 0x7:
        if (piVar5[0xc] != 0x0) {
            FUN_0049af50(piVar5[0xc]);
        }
        default:
            piVar3 = piVar5[0x6];
        FUN_0049af50(piVar5);
        piVar5 = piVar3;
    }
} while( true );
}



fn FUN_00499c00(param_1: i32,byte *param_2,param_3: *mut u32) -> String

{
    let bVar1: u8;
    let mut iVar2: i32;
    let ppcVar3: *mut *mut char;
    let DVar4: u32;
    LPCSTR *ppCVar5;
    byte *pbVar6;
    let mut pCVar7: String;;
    let mut iVar8: i32;
    let mut uVar9: u32;

    for (ppCVar5 = *(LPCSTR **)(param_1 + 0xe); ppCVar5 != 0x0; ppCVar5 = ppCVar5[0x3]) {
        iVar2 = FUN_004a2f10(ppCVar5[0x1],param_2);
        if (iVar2 == 0x0) {
            return *ppCVar5;
        }
    }
    ppcVar3 = FUN_0049c4bd(param_2,&DAT_004c33b5);
    if (ppcVar3 == 0x0) {
        return 0x0;
    }
    DVar4 = FUN_004a91d0(ppcVar3[0x4]);
    *param_3 = DVar4;
    if ((DVar4 != 0x0) && (ppCVar5 = FUN_004a2831(0x10), ppCVar5 != 0x0)) {
        uVar9 = 0xffffffff;
        pbVar6 = param_2;
        loop {
            if (uVar9 == 0x0) break;
            uVar9 = uVar9 - 0x1;
            bVar1 = *pbVar6;
            pbVar6 = pbVar6 + 0x1;
        } while (bVar1 != 0x0);
        pbVar6 = FUN_0049c2c9(~uVar9);
        ppCVar5[0x1] = pbVar6;
        if (pbVar6 != 0x0) {
            loop {
                bVar1 = *param_2;
                *pbVar6 = bVar1;
                if (bVar1 == 0x0) break;
                bVar1 = param_2[0x1];
                param_2 = param_2 + 0x2;
                pbVar6[0x1] = bVar1;
                pbVar6 = pbVar6 + 0x2;
            } while (bVar1 != 0x0);
            pCVar7 = FUN_0049c2c9(*param_3);
            *ppCVar5 = pCVar7;
            if (pCVar7 != 0x0) {
                uVar9 = FUN_004a7970(*ppCVar5,*param_3,0x1,ppcVar3);
                if (uVar9 != 0x0) {
                    iVar2 = *(param_1 + 0xe);
                    if (*(param_1 + 0xe) == 0x0) {
                        *(LPCSTR **)(param_1 + 0xe) = ppCVar5;
                    }
                    else {
                        loop {
                            iVar8 = iVar2;
                            iVar2 = *(iVar8 + 0xc);
                        } while (iVar2 != 0x0);
                        *(LPCSTR **)(iVar8 + 0xc) = ppCVar5;
                    }
                    ppCVar5[0x3] = 0x0;
                    FUN_0049ca40(ppcVar3);
                    return *ppCVar5;
                }
                FUN_0049af50(*ppCVar5);
            }
            FUN_0049af50(ppCVar5[0x1]);
        }
        FUN_0049af50(ppCVar5);
    }
    FUN_0049ca40(ppcVar3);
    return 0x0;
}



fn FUN_00499f60(param_1: u32) -> u32

{
    return CONCAT12((&DAT_005b96d0)[(param_1 & 0xff0000) >> 0x10],
                    CONCAT11((&DAT_005b96d0)[(param_1 & 0xff00) >> 0x8],(&DAT_005b96d0)[param_1 & 0xff]));
}



fn FUN_00499fb0(param_1: i32) -> i32

{
if (0x3a98 < param_1) {
param_1 = (param_1 + -0x4e20) * DAT_005b9214;
}
if (0x1388 < param_1) {
if (DAT_005b9228 != 0x280) {
return param_1 + -0x2710;
}
param_1 = (param_1 + -0x2710) * 0x2;
}
return param_1;
}



fn FUN_00499ff0(param_1: i32) -> i32

{
if (0x3a98 < param_1) {
param_1 = (param_1 + -0x4e20) * DAT_005b9210;
}
if (0x1388 < param_1) {
if (DAT_005b922c < 0x190) {
return param_1 + -0x2710;
}
param_1 = (param_1 + -0x2710) * 0x2;
}
return param_1;
}



u32 *
FUN_0049a030(param_1: *mut u32,param_2: i32,param_3: u32,param_4: i32,param_5: i32,param_6: i32,param_7: i32,
param_8: u32,param_9: u32,param_10: u32)

{
let mut iVar1: i32;
let mut puVar2: *mut u8;
let puVar3: *mut u32;

$1: &mut String(param_1 + 0x45) = &PTR_FUN_004c4014;
*param_1 = 0x0;
iVar1 = FUN_00499fb0(param_4);
*(param_1 + 0x1d) = iVar1;
iVar1 = FUN_00499ff0(param_5);
*(param_1 + 0x21) = iVar1;
iVar1 = FUN_00499fb0(param_6);
*(param_1 + 0x25) = iVar1;
iVar1 = FUN_00499ff0(param_7);
*(param_1 + 0x39) = 0x0;
param_1[0x4] = 0x0;
param_1[0x3] = 0x0;
param_1[0x6] = 0x0;
(param_1 + 0x7) = 0x0;
*(param_1 + 0x29) = iVar1;
*(param_1 + 0x2d) = param_8;
*(param_1 + 0x31) = param_9;
*(param_1 + 0x35) = param_10;
if (param_2 == 0x0) {
param_1[0x1] = DAT_005b9820;
}
else {
param_1[0x1] = param_2;
}
param_1[0x5] = 0x0;
*(param_1 + 0x3d) = param_3;
if (param_1 != &DAT_005b97d0) {
puVar2 = &DAT_005b97d0;
iVar1 = DAT_005b97e4;
while (iVar1 != 0x0) {
puVar2 = (puVar2 + 0x14);
iVar1 = *(puVar2 + 0x14);
}
(puVar2 + 0x14) = param_1;
}
if (DAT_005b9828 != 0x0) {
FUN_0049edc0(DAT_005b9828);
iVar1 = 0x0;
puVar3 = FUN_0049ec50(DAT_005b9828);
while ((puVar3 != param_1 && (puVar3 != 0x0))) {
if (puVar3 != &DAT_005b97d0) {
iVar1 = iVar1 + 0x1;
}
puVar3 = FUN_0049ec90(DAT_005b9828);
}
FUN_0049edd0(DAT_005b9828);
if ((param_1[0x1] == DAT_005b9820) && (DAT_005b9820 != &DAT_005b97d0)) {
iVar1 = iVar1 + 0x1;
}
*(param_1 + 0x41) = iVar1;
return param_1;
}
if (((param_1 != DAT_005b9820) && (DAT_005b9820 != &DAT_005b97d0)) &&
(DAT_005b9820 != 0x0)) {
*(param_1 + 0x41) = 0x1;
return param_1;
}
*(param_1 + 0x41) = 0x0;
return param_1;
}



fn FUN_0049a1c0(param_1: &mut String,byte param_2) -> String

{
    LPCSTR *ppCVar1;
    LPCSTR *ppCVar2;
    let piVar3: *mut i32;;

    if ((param_2 & 0x4) != 0x0) {
        piVar3 = FUN_00498dce(param_1,&DAT_004c4030);
        FUN_00498df5(piVar3);
        return param_1;
    }
    $1: &mut String(param_1 + 0x45) = &PTR_FUN_004c4014;
    ppCVar1 = 0x0;
    for (ppCVar2 = &DAT_005b97d0; (param_1 != ppCVar2 && (ppCVar2 != 0x0));
        ppCVar2 = ppCVar2[0x5]) {
        ppCVar1 = ppCVar2;
    }
    if ((ppCVar2 != 0x0) && (ppCVar1 != 0x0)) {
        ppCVar1[0x5] = ppCVar2[0x5];
    }
    if (*param_1 != 0x0) {
        FUN_0049ed30(*param_1);
    }
    if (param_1[0x6] != 0x0) {
        FUN_0049af50(param_1[0x6]);
    }
    if ((param_2 & 0x2) != 0x0) {
        FUN_0049af50(param_1);
        return param_1;
    }
    return param_1;
}



fn FUN_0049a250(param_1: i32,param_2: u32)

{
    let mut uVar1: u32;

    uVar1 = FUN_0049c2c9(param_2);
    *(param_1 + 0x18) = uVar1;
    return;
}



// WARNING: Could not reconcile some variable overlaps
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0049a270(param_1: i32,param_2: *mut u8,param_3: u32,param_4: i32,param_5: i32) -> u32

{
    let mut bVar1: bool;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut local_34: i32;
    let mut local_30: i32;
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let mut local_18: i32;
    let mut local_14: i32;

    if (param_3 < 0x40b) {
        if (param_3 < 0x405) {
            if (0x203 < param_3) {
                if (param_3 < 0x205) {
                    if ((param_1 + 0x4) != 0x0) {
                        FUN_0049a770((param_1 + 0x4),0x413,*(param_1 + 0x3d),0x0);
                    }
                }
                else {
                    if (param_3 != 0x404) {
                        return 0x0;
                    }
                    if (((*(param_1 + 0x2e) & 0x40) == 0x0) && ((param_1 + 0x1c) != '\0')) {
                        if (_DAT_005b96c0 == (code *)0x0) {
                            ((*(param_1 + 0x45) + 0x10))(param_1);
                            return 0x1;
                        }
                        (*_DAT_005b96c0)(*(param_1 + 0x1d),*(param_1 + 0x21),
                                         *(param_1 + 0x1d) + *(param_1 + 0x25),
                                         *(param_1 + 0x21) + *(param_1 + 0x29),*(param_1 + 0x41));
                        return 0x1;
                    }
                }
            }
        }
        else {
            if (param_3 < 0x406) {
                FUN_004968e7(*(param_2 + 0x1d),*(param_2 + 0x21),*(param_2 + 0x25),*(param_2 + 0x29)
                             ,0x0);
                return 0x1;
            }
            if (0x407 < param_3) {
                if (param_3 < 0x409) {
                    return *(param_1 + 0x39);
                }
                if (param_3 != 0x40a) {
                    return 0x0;
                }
                *(param_1 + 0x39) = param_4;
                return 0x1;
            }
        }
    }
    else {
        if (param_3 < 0x40c) {
            if (((((*(param_1 + 0x2e) & 0x4) == 0x0) && (*(param_1 + 0x1d) <= param_4)) &&
                (param_4 < *(param_1 + 0x1d) + *(param_1 + 0x25))) &&
                ((*(param_1 + 0x21) <= param_5 && (param_5 < *(param_1 + 0x29) + *(param_1 + 0x21))))) {
                return 0x1;
            }
        }
        else {
            if (param_3 < 0x414) {
                if (0x40e < param_3) {
                    if (param_3 < 0x410) {
                        *(param_1 + 0x2e) = *(param_1 + 0x2e) & 0xcf;
                        if (param_4 != 0x0) {
                            return 0x1;
                        }
                        FUN_0049a770(param_2,0x405,0x0,0x0);
                        return 0x1;
                    }
                    if (param_3 != 0x410) {
                        return 0x0;
                    }
                    *(param_1 + 0x2e) = *(param_1 + 0x2e) | 0x30;
                    if (param_4 != 0x0) {
                        return 0x1;
                    }
                    FUN_0049a770(param_2,0x405,0x0,0x0);
                    return 0x1;
                }
            }
            else {
                if (param_3 < 0x415) {
                    if (param_4 != 0x0) {
                        if (((param_1 + 0x1c) == '\0') || ((*(param_1 + 0x2e) & 0x4) != 0x0)) {
                            bVar1 = true;
                        }
                        else {
                            bVar1 = false;
                        }
                        if ((bVar1) && (param_5 != 0xfedc)) {
                            uVar2 = 0x1;
                        }
                        else {
                            uVar2 = 0x0;
                        }
                        (***(code ***)(param_1 + 0x45))(param_1,uVar2);
                        return 0x0;
                    }
                    *(param_1 + 0x2e) = *(param_1 + 0x2e) | 0x4;
                    if ((((param_1 + 0x1c) != '\0') &&
                        ((param_1 + 0x1c) = 0x0, *(param_1 + 0x4) != 0x0)) && (param_5 != 0xfedc)) {
                        local_34 = *(param_2 + 0x1d) + -0x2;
                        local_30 = *(param_2 + 0x21) + -0x2;
                        local_2c = *(param_2 + 0x25) + 0x4;
                        local_28 = *(param_2 + 0x29) + 0x4;
                        FUN_00498a5b(&local_34);
                        FUN_0049a770((param_1 + 0x4),0x405,0x1,&local_34);
                        FUN_00498ae4();
                    }
                    ((*(param_1 + 0x45) + 0x4))(param_1);
                    return 0x0;
                }
                if ((0x415 < param_3) && (param_3 != 0x416)) {
                    return 0x0;
                }
                local_14 = *(param_1 + 0x19) >> 0x18;
                (param_1 + 0x1c) = 0x0;
                if ((*(param_1 + 0x4) != 0x0) && (local_14 != 0x0)) {
                    local_24 = *(param_2 + 0x1d) + -0x2;
                    local_20 = *(param_2 + 0x21) + -0x2;
                    local_1c = *(param_2 + 0x25) + 0x4;
                    local_18 = *(param_2 + 0x29) + 0x4;
                    FUN_00498a5b(&local_24);
                    if (*(param_1 + 0x10) == 0x0) {
                        ((*(param_1 + 0x45) + 0xc))(param_1,*(param_1 + 0x4),0x405,0x1,&local_24);
                    }
                    else {
                        ((param_1 + 0x10))(*(param_1 + 0x4),0x405,0x1,&local_24);
                    }
                    FUN_00498ae4();
                }
                if (param_3 == 0x415) {
                    iVar3 = FUN_00499fb0(param_4);
                    *(param_1 + 0x1d) = iVar3;
                    iVar3 = FUN_00499ff0(param_5);
                    *(param_1 + 0x21) = iVar3;
                }
                else {
                    iVar3 = FUN_00499fb0(param_4);
                    *(param_1 + 0x25) = iVar3;
                    iVar3 = FUN_00499ff0(param_5);
                    *(param_1 + 0x29) = iVar3;
                }
                (param_1 + 0x1c) = (char)local_14;
                if ((char)local_14 != '\0') {
                    FUN_0049a770(param_2,0x405,0x0,0x0);
                    FUN_0049a770(param_2,0x404,0x0,0x0);
                    return 0x0;
                }
            }
        }
    }
    return 0x0;
}



fn FUN_0049a6b0(param_1: i32)

{
    let mut iVar1: i32;

    FUN_004953d7();
    FUN_00497896(*(param_1 + 0x1d),*(param_1 + 0x21),
                 *(param_1 + 0x1d) + *(param_1 + 0x25) + -0x1,*(param_1 + 0x21),
                 (char)*(param_1 + 0x31));
    FUN_00497896(*(param_1 + 0x1d),*(param_1 + 0x21),*(param_1 + 0x1d),
                 *(param_1 + 0x21) + *(param_1 + 0x29) + -0x1,(char)*(param_1 + 0x31));
    iVar1 = *(param_1 + 0x21) + *(param_1 + 0x29) + -0x1;
    FUN_00497896(*(param_1 + 0x1d),iVar1,*(param_1 + 0x1d) + *(param_1 + 0x25) + -0x1,iVar1,
                 (char)*(param_1 + 0x31));
    iVar1 = *(param_1 + 0x1d) + *(param_1 + 0x25) + -0x1;
    FUN_00497896(iVar1,*(param_1 + 0x21),iVar1,*(param_1 + 0x21) + *(param_1 + 0x29) + -0x1,
                 (char)*(param_1 + 0x31));
    FUN_0049536f();
    return;
}



fn FUN_0049a770(param_1: *mut u8,param_2: i32,param_3: u32,param_4: u32) -> String

{
    let mut puVar1: *mut u8;

    puVar1 = param_1;
    if (((param_1 == 0x0) && (puVar1 = PTR_DAT_004bf920, PTR_DAT_004bf920 == &DAT_005b97d0)) &&
        (puVar1 = param_1, DAT_005b9820 != 0x0)) {
        puVar1 = DAT_005b9820;
    }
    if (puVar1 != 0x0) {
        if (((param_2 == 0x404) || (param_2 == 0x405)) && ((puVar1[0x1c] == '\0' || ((puVar1[0x2e] & 0x4) != 0x0)))) {
            return 0x0;
        }
        if (*(puVar1 + 0x10) == 0x0) {
            puVar1 = ((*(puVar1 + 0x45) + 0xc))(puVar1,puVar1,param_2,param_3,param_4);
            return puVar1;
        }
        puVar1 = ((puVar1 + 0x10))(puVar1,param_2,param_3,param_4);
    }
    return puVar1;
}



fn FUN_0049a800(param_1: *mut u8,param_2: i32)

{
    param_1[0x1c] = 0x1;
    param_1[0x2e] = param_1[0x2e] & 0xfb;
    if (param_2 == 0x0) {
        return;
    }
    FUN_0049a770(param_1,0x405,0x0,0x0);
    FUN_0049a770(param_1,0x404,0x0,0x0);
    return;
}



fn FUN_0049a850(param_1: *mut u8,param_2: i32,param_3: u32,param_4: u32)

{
    if ((param_1 == 0x0) && (PTR_DAT_004bf920 != &DAT_005b97d0)) {
        FUN_0049a770(PTR_DAT_004bf920,param_2,param_3,param_4);
        return;
    }
    FUN_0049a770(param_1,param_2,param_3,param_4);
    return;
}



fn FUN_0049a8a0(param_1: *mut u8) -> String

{
    let mut puVar1: *mut u8;

    puVar1 = DAT_005b9820;
    if (DAT_005b9820 != 0x0) {
        if (DAT_005b9828 == 0x0) {
            DAT_005b9828 = FUN_0049ea90();
        }
        FUN_0049eb40(DAT_005b9828,DAT_005b9820);
    }
    FUN_0049a770(DAT_005b9820,0x403,0x0,param_1);
    DAT_005b9820 = param_1;
    FUN_0049a770(param_1,0x402,0x0,param_1);
    return puVar1;
}



fn FUN_0049a9c0() -> u32

{
    let mut uVar1: u32;
    byte *pbVar2;
    let mut iVar3: i32;

    if (DAT_005b9828 != 0x0) {
        FUN_0049edc0(DAT_005b9828);
        uVar1 = FUN_0049ec50(DAT_005b9828);
        while (uVar1 != 0x0) {
            pbVar2 = ((*(uVar1 + 0x45) + 0x14))(uVar1);
            iVar3 = FUN_004a2f10(s_DIALOG_004c33c9,pbVar2);
            if (((iVar3 == 0x0) && (*(uVar1 + 0x51) != 0x0)) && (*(uVar1 + 0x51) != DAT_005b96c8)) {
                return 0x0;
            }
            uVar1 = FUN_0049ec90(DAT_005b9828);
        }
        FUN_0049edd0(DAT_005b9828);
        if (DAT_005b9820 != &DAT_005b97d0) {
            pbVar2 = ((*(DAT_005b9820 + 0x45) + 0x14))(DAT_005b9820);
            iVar3 = FUN_004a2f10(s_DIALOG_004c33d0,pbVar2);
            if (((iVar3 == 0x0) && (*(DAT_005b9820 + 0x51) != 0x0)) && (*(DAT_005b9820 + 0x51) != DAT_005b96c8))
            {
                return 0x0;
            }
        }
    }
    return 0x1;
}



fn FUN_0049aaa0(param_1: i32,param_2: i32,param_3: i32,param_4: i32,param_5: *mut u32) -> u32

{
    let puVar1: *mut u32;

    puVar1 = FUN_0049c2c9(param_3 * param_4);
    if (puVar1 == 0x0) {
        return 0x0;
    }
    FUN_004953d7();
    if (param_5 == 0x0) {
        FUN_00496c1f(param_1,param_2,puVar1,param_3,param_4);
        param_5 = puVar1;
    }
    FUN_00498799(param_5,puVar1,param_3 * param_4,&DAT_005b96d0);
    FUN_00496ac0(puVar1,param_1,param_2,param_3,param_4);
    FUN_0049536f();
    FUN_0049af50(puVar1);
    return 0x1;
}



fn FUN_0049ab40() -> u32

{
    return DAT_005b9820;
}



fn FUN_0049ab50(param_1: u32,byte param_2)

{
    if ((param_2 & 0x1) == 0x0) {
        FUN_0049a770(DAT_005b9820,0x403,0x0,0x0);
    }
    if (DAT_005b9828 != 0x0) {
        DAT_005b9820 = FUN_0049ec70(DAT_005b9828);
        if (DAT_005b9820 != 0x0) {
            FUN_0049ee30(DAT_005b9828,DAT_005b9820);^
            // goto LAB_0049ab6c;
        }
    }
    DAT_005b9820 = &DAT_005b97d0;
    LAB_0049ab6c:
        FUN_0049a770(DAT_005b9820,0x402,0x0,0x0);
    return;
}



fn FUN_0049abd0(param_1: *mut i32,param_2: *mut u8,param_3: i32)

{
    let mut uVar1: u32;
    let piVar2: *mut i32;;
    let mut iVar3: i32;
    i32 aiStack104 [0x10];
    let mut local_28: i32;
    let mut local_24: u32;
    let mut local_20: u32;
    let mut local_1c: u32;
    let mut local_18: i32;
    let mut local_14: u32;

    local_18 = FUN_0049ede0(DAT_005b9828);
    piVar2 = aiStack104;
    loop {
    uVar1 = FUN_0049ec90(DAT_005b9828);
    if ((uVar1 == 0x0) && (param_3 == 0x0)) {
        if (((DAT_005b9820 + 0x1c) == '\0') ||
            (uVar1 = DAT_005b9820, (*(DAT_005b9820 + 0x2e) & 0x4) != 0x0))^ // goto LAB_0049acd3;
        break;
    }
} while ((uVar1 != 0x0) && (((uVar1 + 0x1c) == '\0' || ((*(uVar1 + 0x2e) & 0x4) != 0x0))));
    if ((uVar1 == 0x0) || ((*(uVar1 + 0x25) == 0x0 || (*(uVar1 + 0x29) == 0x0)))) {
        LAB_0049acd3:
            FUN_00498a5b(param_1);
        FUN_0049a770(param_2,0x405,0x1,param_1);
        FUN_0049a770(param_2,0x404,0x0,0x0);
        FUN_00498ae4();
    }
    else {
        local_28 = *(uVar1 + 0x1d);
        local_24 = *(uVar1 + 0x21);
        local_20 = *(uVar1 + 0x25);
        local_1c = *(uVar1 + 0x29);
        local_14 = FUN_004aa20c(param_1,&local_28,aiStack104);
        iVar3 = 0x0;
        if (0x0 < local_14) {
            loop {
                FUN_0049abd0(piVar2,param_2,(uVar1 == DAT_005b9820));
                iVar3 = iVar3 + 0x1;
                piVar2 = piVar2 + 0x4;
            } while (iVar3 < local_14);
        }
    }
    FUN_0049edf0(DAT_005b9828,local_18);
    return;
}



fn FUN_0049ad10(param_1: *mut i32)

{
    let mut puVar1: *mut u8;
    let mut iVar2: i32;
    let piVar3: *mut i32;;
    let mut local_20: i32;
    let mut local_1c: u32;
    let mut local_18: u32;
    let mut local_14: u32;

    FUN_004953d7();
    if (DAT_005b9820 != 0x0) {
        if (param_1 != 0x0) {
            FUN_00498a5b(param_1);
        }
        FUN_0049a770(DAT_005b9820,0x405,(param_1 != 0x0),param_1);
        FUN_0049a770(DAT_005b9820,0x404,0x0,0x0);
        if (param_1 != 0x0) {
            FUN_00498ae4();
        }
    }
    if ((DAT_005b9828 == 0x0) || (puVar1 = FUN_0049ec70(DAT_005b9828), puVar1 == 0x0)) {
        FUN_0049536f();
        return;
    }
    loop {
    local_20 = *(puVar1 + 0x1d);
    local_1c = *(puVar1 + 0x21);
    local_18 = *(puVar1 + 0x25);
    local_14 = *(puVar1 + 0x29);
    piVar3 = param_1;
    if (param_1 == 0x0) {
        LAB_0049adba:
            FUN_0049abd0(&local_20,puVar1,piVar3);
    }
    else {
        iVar2 = FUN_004aa45a(param_1,&local_20,&local_20);
        if (iVar2 != 0x0) {
            piVar3 = 0x0;^
            // goto LAB_0049adba;
        }
    }
    puVar1 = FUN_0049ecc0(DAT_005b9828);
    if (puVar1 == 0x0) {
        FUN_0049536f();
        return;
    }
} while( true );
}



fn FUN_0049ae00(param_1: i32,param_2: u32) -> u32

{
    let mut uVar1: u32;
    let mut in_EAX: u32;

    if (param_1 == 0x0) {
        return in_EAX;
    }
    uVar1 = *(param_1 + 0x10);
    *(param_1 + 0x10) = param_2;
    return uVar1;
}



fn FUN_0049ae20(param_1: *mut u8)

{
    let mut puVar1: *mut u8;

    puVar1 = PTR_DAT_004bf920;
    PTR_DAT_004bf920 = param_1;
    (param_1 + 0xc) = puVar1;
    return;
}



fn FUN_0049ae40(param_1: *mut u8)

{
    let puVar1: *mut u32;

    if (param_1 != PTR_DAT_004bf920) {
        return;
    }
    puVar1 = (PTR_DAT_004bf920 + 0xc);
    PTR_DAT_004bf920 = (PTR_DAT_004bf920 + 0xc);
    *puVar1 = 0x0;
    return;
}



fn FUN_0049ae60(param_1: *mut *mut u32,param_2: i32,param_3: i32,param_4: i32)

{
    if (param_2 <= param_3) {
        loop {
            FUN_0049bf80(param_1,param_2,0x502,(param_4 == param_2),0x0);
            param_2 = param_2 + 0x1;
        } while (param_2 <= param_3);
    }
    return;
}



fn FUN_0049aea0(param_1: *mut *mut u32,param_2: i32,param_3: i32) -> i32

{
let mut puVar1: *mut u8;

if (param_2 <= param_3) {
loop {
puVar1 = FUN_0049bf80(param_1,param_2,0x501,0x0,0x0);
if (puVar1 != 0x0) {
return param_2;
}
param_2 = param_2 + 0x1;
} while (param_2 <= param_3);
}
return 0x0;
}



fn FUN_0049aee0()

{
    let local_18: u8 [0x10];

    FUN_0049c2e0(local_18,s_CHAR_d_d_004c33d7);
    DAT_005b982c = FUN_004971ad(local_18);
    return;
}



fn FUN_0049af50(LPCSTR param_1)

{
    if (param_1 == 0x0) {
        return;
    }
    FUN_004aaae0(param_1);
    return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_0049bb50(param_1: *mut *mut u32,param_2: i32) -> u32

{
    DWORD *pDVar1;
    let mut uVar2: u32;
    let mut puVar3: *mut u8;
    let mut iVar4: i32;
    let DVar5: u32;
    let mut extraout_ECX: u32;
    undefined8 uVar6;
    ulonglong uVar7;
    let mut iVar8: i32;
    let mut uVar9: u32;
    u32 **ppuVar10;
    let piVar11: *mut i32;;
    let mut pcVar12: String;
    u32 local_74 [0x9];
    i32 local_50 [0x7];
    let mut local_34: i32;
    let mut local_30: u32;
    let mut local_2c: u32;
    let mut local_28: u32;
    let mut local_24: *mut u8;
    let mut local_20: u32;
    let mut local_1c: i32;
    let local_18: i32;
    u32 **local_14;

    if (*(param_1 + 0x51) == 0x0) {
        FUN_004a2d6b();
    }
    else {
        FUN_004ae8ec();
    }
    if (((*(param_1 + 0x2e) & 0x40) == 0x0) && (_DAT_005b96c4 != (code *)0x0)) {
(*_DAT_005b96c4)(param_1 + 0x1d,param_1 + 0x21,param_1 + 0x25,param_1 + 0x29);
}
    *(param_1 + 0x59) = 0x0;
    *(param_1 + 0x49) = 0x0;
    local_14 = DAT_005b9830;
    DAT_005b9830 = param_1;
    *(param_1 + 0x55) = *(param_1 + 0x59);
    uVar2 = FUN_0049ec50(*param_1);
    *(param_1 + 0x55) = uVar2;
    if (uVar2 != 0x0) {
        uVar9 = 0x0;
        iVar8 = 0x403;
        ppuVar10 = param_1;
        puVar3 = FUN_0049ab40();
        FUN_0049a770(puVar3,iVar8,uVar9,ppuVar10);
        FUN_004a778a(local_74);
        FUN_004a789b();
        FUN_004958f8(local_50);
        FUN_00495491();
        FUN_00495a31();
        *(param_1 + 0x4d) = 0x1;
        if (*(param_1 + 0x51) != 0x0) {
            if ((*(param_1 + 0x2d) & 0x2) == 0x0) {
                if (param_1[0x1] == 0x0) {
                    FUN_004965e4(*(param_1 + 0x51),0x14);
                }
                else {
                    FUN_004965e4(DAT_005b96c8,0x14);
                }
            }
            _DAT_005b9834 = 0x1;
            FUN_004ae8ec();
        }
        FUN_004a0430(param_1 + 0x61,0x0,0x90);
        iVar8 = -0x1;
        FUN_0049f97a(DAT_005b9bd4);
        uVar2 = *(param_1 + 0x55);
        while (uVar2 != 0x0) {
            FUN_0049a770((param_1 + 0x55),0x401,0x0,0x0);
            puVar3 = FUN_0049a770((param_1 + 0x55),0x408,0x0,0x0);
            iVar4 = FUN_004a11c0(puVar3);
            if ((iVar4 < 0x41) || (0x5a < iVar4)) {
                if ((0x2f < iVar4) && (iVar4 < 0x3a)) {
                    iVar8 = iVar4 + -0x16;
                }
            }
            else {
                iVar8 = iVar4 + -0x41;
            }
            if ((-0x1 < iVar8) && (*((param_1 + iVar8) + 0x61) == 0x0)) {
                pcVar12 = s_STATIC_004c341c;
                uVar6 = ((*(*(param_1 + 0x55) + 0x45) + 0x14))(*(param_1 + 0x55));
                uVar7 = FUN_004a2ae0(extraout_ECX,((ulonglong)uVar6 >> 0x20),uVar6,pcVar12);
                if (uVar7 != 0x0) {
                    *((param_1 + iVar8) + 0x61) = *(param_1 + 0x55);
                }
            }
            uVar2 = FUN_0049ec90(*param_1);
            *(param_1 + 0x55) = uVar2;
        }
        *(param_1 + 0x5d) = param_2;
        if (param_2 != 0x0) {
            ((param_1 + 0x5d))(param_1,0x401,0x0,0x0);
        }
        uVar2 = FUN_0049ec50(*param_1);
        *(param_1 + 0x55) = uVar2;
        *(param_1 + 0x59) = uVar2;
        while ((uVar2 != 0x0 && ((*(*(param_1 + 0x55) + 0x2e) & 0x18) != 0x0))) {
            uVar2 = FUN_0049ec90(*param_1);
            *(param_1 + 0x55) = uVar2;
            *(param_1 + 0x59) = uVar2;
        }
        if (*(param_1 + 0x55) == 0x0) {
            uVar2 = FUN_0049ec50(*param_1);
            *(param_1 + 0x55) = uVar2;
            *(param_1 + 0x59) = uVar2;
        }
        (***(code ***)(param_1 + 0x45))(param_1,0x0);
        ((*(param_1 + 0x45) + 0xc))(param_1,param_1,0x405,0x0,0x0);
        FUN_004ae8ec();
        ((*(param_1 + 0x45) + 0xc))(param_1,param_1,0x404,0x0,0x0);
        FUN_0049a8a0(param_1);
        FUN_0049536f();
        loop {
            loop {
                DVar5 = win_func_0049fe83(DAT_005b9bd4,(DWORD *)&local_24,&local_20,&local_1c,&local_18);
            } while (DVar5 == 0x0);
            FUN_0049a850(local_24,local_20,local_1c,local_18);
        } while (*(param_1 + 0x49) == 0x0);
        if ((param_1 + 0x55) != 0x0) {
            FUN_0049a770((param_1 + 0x55),0x403,0x0,0x0);
        }
        if (*(param_1 + 0x5d) != 0x0) {
            ((param_1 + 0x5d))(param_1,0x403,0x0,0x0);
        }
        if (*(param_1 + 0x5d) != 0x0) {
            ((param_1 + 0x5d))(param_1,0x415,0x0,0x0);
        }
        pDVar1 = DAT_005b9bd4;
        *(param_1 + 0x4d) = 0x0;
        FUN_0049f97a(pDVar1);
        ((*(param_1 + 0x45) + 0x4))(param_1);
        FUN_0049595c(local_50);
        FUN_004a7806(local_74);
        DAT_005b9830 = local_14;
        if ((*(param_1 + 0x2d) & 0x1) == 0x0) {
            if ((*(param_1 + 0x2d) & 0x4) == 0x0) {
                local_34 = *(param_1 + 0x1d);
                local_30 = *(param_1 + 0x21);
                local_2c = *(param_1 + 0x25);
                local_28 = *(param_1 + 0x29);
                FUN_004a756b();
                piVar11 = &local_34;
            }
            else {
                FUN_004a756b();
                piVar11 = 0x0;
            }
            FUN_0049ad10(piVar11);
            FUN_004a75a6();
            FUN_004ae8ec();
        }
        FUN_0049ab50(param_1,0x1);
        uVar2 = *(param_1 + 0x49) - 0x1;
    }
    return uVar2;
}



fn FUN_0049bf40(param_1: *mut i32,param_2: u32)

{
    let puVar1: *mut u32;

    if (*param_1 != 0x0) {
        FUN_0049eb40(*param_1,param_2);
        return;
    }
    puVar1 = FUN_0049ea90();
    *param_1 = puVar1;
    FUN_0049eb40(*param_1,param_2);
    return;
}



fn FUN_0049bf80(param_1: *mut *mut u32,param_2: i32,param_3: i32,param_4: i32,param_5: u32) -> String

{
    let mut uVar1: u32;
    let mut puVar2: *mut u8;

    FUN_0049edc0(*param_1);
    if (param_3 == 0x40d) {
        loop {
            uVar1 = FUN_0049ec90(*param_1);
            *(param_1 + 0x55) = uVar1;
            if (uVar1 == 0x0) {
                uVar1 = FUN_0049ec50(*param_1);
                *(param_1 + 0x55) = uVar1;
            }
            if (((*(*(param_1 + 0x55) + 0x2e) & 0x18) == 0x0) &&
                (*(param_1 + 0x55) != *(param_1 + 0x59)))^ // goto LAB_0049c04b;
        } while (*(param_1 + 0x55) != *(param_1 + 0x59));
    }
    else {
        if (param_3 == 0x40e) {
            uVar1 = FUN_0049ec50(*param_1);
            while ((uVar1 != 0x0 && (param_4 != *(uVar1 + 0x3d)))) {
                uVar1 = FUN_0049ec90(*param_1);
            }
            if (uVar1 == 0x0) {
                FUN_0049edd0(*param_1);
            }
            else {
                *(param_1 + 0x55) = uVar1;
            }
            LAB_0049c04b:
            if (((((param_1 + 0x55))[0x2e] & 0x10) == 0x0) &&
                ((param_1 + 0x55) != (param_1 + 0x59))) {
                FUN_0049a770((param_1 + 0x59),0x403,0x0,0x0);
                FUN_0049a770((param_1 + 0x55),0x402,0x0,0x0);
                *(param_1 + 0x59) = *(param_1 + 0x55);
                return 0x1;
            }
            return 0x0;
        }
    }
    if ((param_2 == 0x0) && (*(param_1 + 0x5d) != 0x0)) {
        if (((param_3 == 0x404) || (param_3 == 0x405)) &&
            (((param_1 + 0x7) == '\0' || ((*(param_1 + 0x2e) & 0x4) != 0x0)))) {
            return 0x0;
        }
        puVar2 = ((param_1 + 0x5d))(param_1,param_3,param_4,param_5);
        return puVar2;
    }
    puVar2 = FUN_0049ec50(*param_1);
    while ((puVar2 != 0x0 && (param_2 != *(puVar2 + 0x3d)))) {
        puVar2 = FUN_0049ec90(*param_1);
    }
    if (puVar2 == 0x0) {
        FUN_0049edd0(*param_1);
        return 0x0;
    }
    puVar2 = FUN_0049a770(puVar2,param_3,param_4,param_5);
    FUN_0049edd0(*param_1);
    return puVar2;
}



fn FUN_0049c140(param_1: i32,param_2: i32)

{
    *(param_1 + 0x49) = param_2 + 0x1;
    return;
}



fn FUN_0049c160(param_1: *mut *mut u32,param_2: i32)

{
    let mut uVar1: u32;

    uVar1 = FUN_0049ec50(*param_1);
    if (uVar1 != 0x0) {
        while (param_2 != *(uVar1 + 0x3d)) {
            uVar1 = FUN_0049ec90(*param_1);
            if (uVar1 == 0x0) {
                return;
            }
        }
    }
    return;
}



fn FUN_0049c1a0(param_1: *mut *mut u32,param_2: i32)

{
    let mut uVar1: u32;

    FUN_0049edc0(*param_1);
    uVar1 = FUN_0049ec50(*param_1);
    while (uVar1 != 0x0) {
        if ((*(uVar1 + 0x2d) & 0x400) == 0x0) {
            (***(code ***)(uVar1 + 0x45))(uVar1,0x0);
        }
        uVar1 = FUN_0049ec90(*param_1);
    }
    FUN_0049a800(param_1,param_2);
    FUN_0049edd0(*param_1);
    return;
}



fn FUN_0049c2c9(param_1: u32)

{
    FUN_004a2831(param_1);
    return;
}



fn FUN_0049c2e0(Pparam_1: u8,Pparam_2: u8)

{
    let local_8: *mut u32;

    local_8 = &stack0x0000000c;
    FUN_004ae978(param_1,param_2,&local_8);
    return;
}



fn FUN_0049c305()

{
    DAT_005b9840 = &DAT_005b9850;
    return;
}



void set_curr_dir_0049c320(void)

{
let mut lpPathName: String;;
let mut uVar1: u32;

uVar1 = DAT_005b9848;
lpPathName = DAT_005b9840;
DAT_005b9840 = lpPathName_005b9844;
lpPathName_005b9844 = lpPathName;
DAT_005b9848 = DAT_005b984c;
DAT_005b984c = uVar1;
// LPCSTR lpPathName for SetCurrentDirectoryA
SetCurrentDirectoryA(lpPathName);
return;
}



undefined1 *  FUN_0049c374(param_1: &mut String)

{
let cVar1: u8;
let mut pcVar2: String;
let mut uVar3: u32;
let mut iVar4: i32;
let mut pcVar5: String;

pcVar2 = FUN_004ae9a0(*DAT_005b9c8c,'\\');
if (pcVar2 == 0x0) {
pcVar2 = FUN_004ae9a0(*DAT_005b9c8c,':');
if (pcVar2 == 0x0) {
pop_err_msg_box_and_exit_004a02f5(s_System_error_101_004c342c);
}
else {
pcVar2[0x1] = '\0';
}
}
else {
*pcVar2 = '\0';
}
pcVar2 = *DAT_005b9c8c;
pcVar5 = &DAT_005b9850;
loop {
cVar1 = *pcVar2;
*pcVar5 = cVar1;
if (cVar1 == '\0') break;
cVar1 = pcVar2[0x1];
pcVar2 = pcVar2 + 0x2;
pcVar5[0x1] = cVar1;
pcVar5 = pcVar5 + 0x2;
} while (cVar1 != '\0');
lpPathName_005b9844 = &DAT_005b9850;
uVar3 = 0xffffffff;
pcVar2 = &DAT_005b9850;
loop {
if (uVar3 == 0x0) break;
uVar3 = uVar3 - 0x1;
cVar1 = *pcVar2;
pcVar2 = pcVar2 + 0x1;
} while (cVar1 != '\0');
DAT_005b984c = ~uVar3 - 0x1;
if (param_1 == 0x0) {
iVar4 = 0x0;
}
else {
uVar3 = 0xffffffff;
pcVar2 = param_1;
loop {
if (uVar3 == 0x0) break;
uVar3 = uVar3 - 0x1;
cVar1 = *pcVar2;
pcVar2 = pcVar2 + 0x1;
} while (cVar1 != '\0');
iVar4 = ~uVar3 - 0x1;
}
if ((iVar4 == 0x0) || (0x103 < iVar4)) {
DAT_005b9840 = &DAT_005b9850;
}
else {
pcVar2 = &DAT_005b9958;
loop {
cVar1 = *param_1;
*pcVar2 = cVar1;
if (cVar1 == '\0') break;
cVar1 = param_1[0x1];
param_1 = param_1 + 0x2;
pcVar2[0x1] = cVar1;
pcVar2 = pcVar2 + 0x2;
} while (cVar1 != '\0');
DAT_005b9840 = &DAT_005b9958;
}
uVar3 = 0xffffffff;
pcVar2 = DAT_005b9840;
loop {
if (uVar3 == 0x0) break;
uVar3 = uVar3 - 0x1;
cVar1 = *pcVar2;
pcVar2 = pcVar2 + 0x1;
} while (cVar1 != '\0');
DAT_005b9848 = ~uVar3 - 0x1;
// LPCSTR lpPathName for SetCurrentDirectoryA
SetCurrentDirectoryA(lpPathName_005b9844);
return &DAT_005b9850;
}



i32 *  FUN_0049c4bd(char *str_param_1,byte *param_2)

{
byte *pbVar1;
byte *pbVar2;
let mut uVar3: u32;
let local_18: *mut i32;;

uVar3 = 0x40;
pbVar2 = param_2;
pbVar1 = FUN_0049c7b5(str_param_1);
local_18 = FUN_004a9670(pbVar1,pbVar2,uVar3);
if (local_18 == 0x0) {
uVar3 = 0x40;
pbVar2 = FUN_0049c8a4(str_param_1);
local_18 = FUN_004a9670(pbVar2,param_2,uVar3);
}
return local_18;
}



i32 *  FUN_0049c52b(param_1: &mut String,byte *param_2)

{
byte *pbVar1;
let piVar2: *mut i32;;
let mut uVar3: u32;

uVar3 = 0x40;
pbVar1 = FUN_0049c8a4(param_1);
piVar2 = FUN_004a9670(pbVar1,param_2,uVar3);
return piVar2;
}



void set_curr_dir_0049c55d(void)

{
// LPCSTR lpPathName for SetCurrentDirectoryA
SetCurrentDirectoryA(lpPathName_005b9844);
return;
}



bool  FUN_0049c57b(param_1: &mut String,param_2: *mut u32,param_3: u32)

{
let ppcVar1: *mut *mut char;
let mut uVar2: u32;
let mut bVar3: bool;

bVar3 = false;
ppcVar1 = FUN_0049c4bd(param_1,&DAT_004c343d);
if (ppcVar1 != 0x0) {
FUN_004aa75c(ppcVar1,0x0,0x0);
uVar2 = FUN_004a7970(param_2,0x1,param_3,ppcVar1);
bVar3 = uVar2 == param_3;
FUN_0049ca40(ppcVar1);
}
return bVar3;
}



bool  FUN_0049c60b(param_1: &mut String,param_2: *mut u32,param_3: u32,param_4: i32)

{
let ppcVar1: *mut *mut char;
let mut uVar2: u32;
let mut bVar3: bool;

bVar3 = false;
ppcVar1 = FUN_0049c4bd(param_1,&DAT_004c343d);
if (ppcVar1 != 0x0) {
FUN_004aa75c(ppcVar1,param_4,0x0);
uVar2 = FUN_004a7970(param_2,0x1,param_3,ppcVar1);
bVar3 = uVar2 == param_3;
FUN_0049ca40(ppcVar1);
}
return bVar3;
}



bool  FUN_0049c67c(param_1: &mut String,param_2: *mut u32,param_3: u32)

{
byte *pbVar1;
let ppcVar2: *mut *mut char;
let mut bVar3: bool;
byte *pbVar4;
let mut uVar5: u32;

bVar3 = false;
uVar5 = 0x40;
pbVar4 = &DAT_004c3440;
pbVar1 = FUN_0049c8a4(param_1);
ppcVar2 = FUN_004a9670(pbVar1,pbVar4,uVar5);
if (ppcVar2 != 0x0) {
FUN_004aa75c(ppcVar2,0x0,0x0);
uVar5 = FUN_004a7970(param_2,0x1,param_3,ppcVar2);
bVar3 = uVar5 == param_3;
FUN_0049ca40(ppcVar2);
}
return bVar3;
}



bool  FUN_0049c728(param_1: &mut String,param_2: *mut u32,param_3: u32,param_4: i32)

{
byte *pbVar1;
let ppcVar2: *mut *mut char;
let mut bVar3: bool;
byte *pbVar4;
let mut uVar5: u32;

bVar3 = false;
uVar5 = 0x40;
pbVar4 = &DAT_004c3440;
pbVar1 = FUN_0049c8a4(param_1);
ppcVar2 = FUN_004a9670(pbVar1,pbVar4,uVar5);
if (ppcVar2 != 0x0) {
FUN_004aa75c(ppcVar2,param_4,0x0);
uVar5 = FUN_004a7970(param_2,0x1,param_3,ppcVar2);
bVar3 = uVar5 == param_3;
FUN_0049ca40(ppcVar2);
}
return bVar3;
}



char *  FUN_0049c7b5(param_1: &mut String)

{
let cVar1: u8;
let mut pCVar2: String;;
let puVar3: *mut u32;
let mut pcVar4: String;
let mut local_20: u32;
let mut local_18: u32;
let mut local_14: String;

if ((*param_1 == '\\') || (param_1[0x1] == ':')) {
local_14 = param_1;
}
else {
local_18 = param_1 & 0xffff0000 | DAT_004bf940;
if (lpPathName_005b9844[DAT_005b984c + -0x1] == '\\') {
local_18 = param_1 & 0xffff0000 | DAT_004bf940 & 0xffffff00;
}
pcVar4 = &DAT_005b9a60;
pCVar2 = lpPathName_005b9844;
loop {
cVar1 = *pCVar2;
*pcVar4 = cVar1;
if (cVar1 == '\0') break;
cVar1 = pCVar2[0x1];
pCVar2 = pCVar2 + 0x2;
pcVar4[0x1] = cVar1;
pcVar4 = pcVar4 + 0x2;
} while (cVar1 != '\0');
puVar3 = &local_18;
pcVar4 = &DAT_005b9a60 + DAT_005b984c;
loop {
cVar1 = puVar3;
*pcVar4 = cVar1;
if (cVar1 == '\0') break;
cVar1 = (puVar3 + 0x1);
puVar3 = (puVar3 + 0x2);
pcVar4[0x1] = cVar1;
pcVar4 = pcVar4 + 0x2;
} while (cVar1 != '\0');
local_20 = ((char)local_18 != '\0');
pcVar4 = &DAT_005b9a60 + local_20 + DAT_005b984c;
loop {
cVar1 = *param_1;
*pcVar4 = cVar1;
if (cVar1 == '\0') break;
cVar1 = param_1[0x1];
param_1 = param_1 + 0x2;
pcVar4[0x1] = cVar1;
pcVar4 = pcVar4 + 0x2;
} while (cVar1 != '\0');
local_14 = &DAT_005b9a60;
}
return local_14;
}



char *  FUN_0049c8a4(param_1: &mut String)

{
let cVar1: u8;
let mut pcVar2: String;
let puVar3: *mut u32;
let mut pcVar4: String;
let mut local_20: u32;
let mut local_18: u32;
let mut local_14: String;

if ((*param_1 == '\\') || (param_1[0x1] == ':')) {
local_14 = param_1;
}
else {
local_18 = param_1 & 0xffff0000 | DAT_004bf942;
if (DAT_005b9840[DAT_005b9848 + -0x1] == '\\') {
local_18 = param_1 & 0xffff0000 | DAT_004bf942 & 0xffffff00;
}
pcVar4 = &DAT_005b9a60;
pcVar2 = DAT_005b9840;
loop {
cVar1 = *pcVar2;
*pcVar4 = cVar1;
if (cVar1 == '\0') break;
cVar1 = pcVar2[0x1];
pcVar2 = pcVar2 + 0x2;
pcVar4[0x1] = cVar1;
pcVar4 = pcVar4 + 0x2;
} while (cVar1 != '\0');
puVar3 = &local_18;
pcVar2 = &DAT_005b9a60 + DAT_005b9848;
loop {
cVar1 = puVar3;
*pcVar2 = cVar1;
if (cVar1 == '\0') break;
cVar1 = (puVar3 + 0x1);
puVar3 = (puVar3 + 0x2);
pcVar2[0x1] = cVar1;
pcVar2 = pcVar2 + 0x2;
} while (cVar1 != '\0');
local_20 = ((char)local_18 != '\0');
pcVar2 = &DAT_005b9a60 + local_20 + DAT_005b9848;
loop {
cVar1 = *param_1;
*pcVar2 = cVar1;
if (cVar1 == '\0') break;
cVar1 = param_1[0x1];
param_1 = param_1 + 0x2;
pcVar2[0x1] = cVar1;
pcVar2 = pcVar2 + 0x2;
} while (cVar1 != '\0');
local_14 = &DAT_005b9a60;
}
return local_14;
}



fn FUN_0049c993(param_1: &mut String,param_2: u32) -> i32

{
byte *pbVar1;
let mut uVar2: u32;
let mut local_18: i32;

uVar2 = param_2;
pbVar1 = FUN_0049c7b5(param_1);
local_18 = FUN_004ae9c0(pbVar1,uVar2);
if (local_18 == -0x1) {
uVar2 = 0x40;
pbVar1 = FUN_0049c8a4(param_1);
local_18 = FUN_004ae9e4(pbVar1,param_2,uVar2);
}
return local_18;
}



fn FUN_0049ca0b(param_1: &mut String,param_2: u32) -> u32

{
    byte *pbVar1;
    let mut uVar2: u32;
    let mut uVar3: u32;

    uVar3 = 0x40;
    pbVar1 = FUN_0049c8a4(param_1);
    uVar2 = FUN_004ae9e4(pbVar1,param_2,uVar3);
    return uVar2;
}



fn FUN_0049ca40(param_1: *mut u32) -> u32

{
    let puVar1: *mut u32;
    let mut uVar2: u32;

    (PTR_FUN_004bfb88)();
    puVar1 = DAT_005ba410;
    while( true ) {
        if (puVar1 == 0x0) {
            (PTR_FUN_004bfb8c)();
            return 0xffffffff;
        }
        if (param_1 == puVar1[0x1]) break;
        puVar1 = *puVar1;
    }
    (PTR_FUN_004bfb8c)();
    uVar2 = FUN_0049ca84(param_1,0x1);
    return uVar2;
}



fn FUN_0049ca84(param_1: *mut u32,param_2: i32) -> u32

{
    let mut uVar1: u32;

    uVar1 = FUN_0049cb70(param_1,param_2);
    FUN_004af080(param_1);
    return uVar1;
}



fn FUN_0049caac(param_1: i32) -> i32

{
let mut iVar1: i32;

iVar1 = param_1 + 0x30;
if (0x39 < iVar1) {
iVar1 = param_1 + 0x57;
}
return iVar1;
}



fn FUN_0049cabc(param_1: &mut String,param_2: u32)

{
    let cVar1: u8;
    let mut uVar2: u32;
    let mut pcVar3: String;
    let mut iVar4: i32;
    let mut uVar5: u32;
    let mut pcVar6: String;
    let mut pcVar7: String;

    uVar2 = FUN_004af0e0();
    uVar2 = uVar2 >> 0x10 | uVar2;
    pcVar3 = FUN_004af21c();
    pcVar7 = param_1;
    loop {
    cVar1 = *pcVar3;
    *pcVar7 = cVar1;
    if (cVar1 == '\0') break;
    cVar1 = pcVar3[0x1];
    pcVar3 = pcVar3 + 0x2;
    pcVar7[0x1] = cVar1;
    pcVar7 = pcVar7 + 0x2;
} while (cVar1 != '\0');
    uVar5 = 0xffffffff;
    pcVar7 = param_1;
    loop {
    if (uVar5 == 0x0) break;
    uVar5 = uVar5 - 0x1;
    cVar1 = *pcVar7;
    pcVar7 = pcVar7 + 0x1;
} while (cVar1 != '\0');
    pcVar7 = param_1 + (~uVar5 - 0x1);
    *pcVar7 = 't';
    pcVar3 = pcVar7 + 0x4;
    loop {
    uVar5 = uVar2 & 0xf;
    pcVar6 = pcVar3 + -0x1;
    uVar2 = uVar2 >> 0x4;
    iVar4 = FUN_0049caac(uVar5);
    *pcVar3 = (char)iVar4;
    pcVar3 = pcVar6;
} while (pcVar6 != pcVar7);
    pcVar7[0x5] = '_';
    iVar4 = FUN_0049caac(param_2 >> 0x4 & 0xf);
    pcVar7[0x6] = (char)iVar4;
    iVar4 = FUN_0049caac(param_2 & 0xf);
    pcVar7[0x8] = '.';
    pcVar7[0x9] = 't';
    pcVar7[0xa] = 'm';
    pcVar7[0xb] = 'p';
    pcVar7[0xc] = '\0';
    pcVar7[0x7] = (char)iVar4;
    return;
}



fn FUN_0049cb70(param_1: *mut u32,param_2: i32) -> u32

{
    let mut uVar1: u32;
    let DVar2: u32;
    let acStack292: u8 [0x114];

    if (param_1[0x3] == 0x0) {
        uVar1 = 0xffffffff;
    }
    else {
        uVar1 = 0x0;
        if ((*(param_1 + 0xd) & 0x10) != 0x0) {
            uVar1 = FUN_004af2f0(param_1);
        }
        (PTR_FUN_004bfb78)(param_1[0x4]);
        DVar2 = FUN_004aa690(param_1);
        if (DVar2 != 0xffffffff) {
            set_file_pointer_004af420(param_1[0x4],DVar2,0x0);
        }
        if (param_2 != 0x0) {
            DVar2 = FUN_004af4b0(param_1[0x4]);
            uVar1 = uVar1 | DVar2;
        }
        if ((*(param_1 + 0x3) & 0x8) != 0x0) {
            FUN_004aaae0(*(param_1[0x2] + 0x8));
            *(param_1[0x2] + 0x8) = 0x0;
        }
        if ((*(param_1 + 0xd) & 0x8) != 0x0) {
            FUN_0049cabc(acStack292,*(param_1[0x2] + 0x14));
            FUN_004a5420(acStack292);
        }
        (PTR_FUN_004bfb7c)(param_1[0x4]);
        if (param_2 != 0x0) {
            (PTR_FUN_004bfb84)(param_1[0x4]);
        }
    }
    return uVar1;
}



i32 *
FUN_0049cc70(param_1: *mut u32,param_2: i32,param_3: u32,param_4: i32,param_5: i32,param_6: i32,param_7: i32,
param_8: u32,param_9: &mut String)

{
let piVar1: *mut i32;;
let bVar2: u8;

piVar1 = FUN_0049a030(param_1,param_2,param_3,param_4,param_5,param_6,param_7,param_8,0x0,0x0);
bVar2 = *(piVar1 + 0x2e) | 0x78;
*(piVar1 + 0x2e) = bVar2;
$1: &mut String(piVar1 + 0x45) = &PTR_FUN_004c4054;
if ((*(piVar1 + 0x2d) & 0xc) != 0x0) {
*(piVar1 + 0x2e) = bVar2 & 0xbf;
}
FUN_0049cce0(piVar1,param_9);
return piVar1;
}



fn FUN_0049cce0(param_1: *mut i32,param_2: &mut String)

{
    byte *pbVar1;
    let mut iVar2: i32;
    i32 **ppiVar3;

    iVar2 = FUN_004af560((param_1 + 0x49),param_2,0x3d090,0x0,0x0,0x6);
    if (iVar2 < 0x0) {
        *(param_1 + 0xe1) = 0x1;
        return;
    }
    FUN_004a0430(*(param_1 + 0xe9),0x0,
                 (param_1 + 0x51) * (param_1 + 0x53));
    *(param_1 + 0xf1) = 0x0;
    *(param_1 + 0x25) = (param_1 + 0x51);
    *(param_1 + 0x29) = (param_1 + 0x53);
    ppiVar3 = (i32 **)FUN_004a2831(0x10);
    if (ppiVar3 != (i32 **)0x0) {
    FUN_004a2874(ppiVar3,param_1,*(i32 **)(param_1 + 0x59));
}
    pbVar1 = (*(param_1 + 0xdd) + 0x1c);
    *pbVar1 = *pbVar1 | 0x4;
    *(param_1 + 0xe1) = 0x0;
    *(param_1 + 0xf5) = 0x0;
    *(param_1 + 0xd5) = *(param_1 + 0x1d);
    *(param_1 + 0xd9) = *(param_1 + 0x21);
    return;
}



fn FUN_0049cda0(param_1: i32)

{
    let mut iVar1: i32;

    if (*(param_1 + 0xe1) != 0x0) {
        FUN_004a2965(param_1);
        return;
    }
    if (((*(param_1 + 0x2d) & 0x1) != 0x0) && ((param_1 + 0x4) != 0x0)) {
        FUN_0049a770((param_1 + 0x4),0x406,*(param_1 + 0xf5),0x0);
    }
    FUN_00495520(*(param_1 + 0x1d),*(param_1 + 0x21),*(param_1 + 0x25),
                 *(param_1 + 0x29),0x0);
    iVar1 = FUN_004af840(param_1 + 0x49);
    if (-0x1 < iVar1) {
        if (*(param_1 + 0xf5) == 0x0) {
            *(param_1 + 0x9d) = *(param_1 + 0x99) + DAT_005ba420;
        }
        if (((*(param_1 + 0x2d) & 0x1) != 0x0) && ((param_1 + 0x4) != 0x0)) {
            FUN_0049a770((param_1 + 0x4),0x406,*(param_1 + 0xf5),0x1);
        }
        iVar1 = *(param_1 + 0xf5) + 0x1;
        *(param_1 + 0xf5) = iVar1;
        if ((param_1 + 0x4f) < iVar1) {
            FUN_004aff90(*(param_1 + 0xdd));
            FUN_004b00f0(*(param_1 + 0xdd),*(param_1 + 0x9d),0x0);
            FUN_004b0070(*(param_1 + 0xdd),*(param_1 + 0x9d),0x0);
            *(param_1 + 0xf5) = 0x1;
        }
        FUN_00495607(*(param_1 + 0xe9));
        return;
    }
    FUN_004a2965(param_1);
    FUN_00495607(*(param_1 + 0xe9));
    return;
}



fn FUN_0049cef0(param_1: &mut String,byte param_2) -> String

{
    LPCSTR *ppCVar1;
    let piVar2: *mut i32;;

    if ((param_2 & 0x4) != 0x0) {
        piVar2 = FUN_00498dce(param_1,&DAT_004c3d70);
        FUN_00498df5(piVar2);
        return param_1;
    }
    $1: &mut String(param_1 + 0x45) = &PTR_FUN_004c4054;
    FUN_004a2965(param_1);
    FUN_004af7d0(param_1 + 0x49);
    ppCVar1 = FUN_0049a1c0(param_1,0x1);
    if ((param_2 & 0x2) == 0x0) {
        return ppCVar1;
    }
    FUN_0049af50(ppCVar1);
    return ppCVar1;
}



fn FUN_0049d2b0(param_1: u32,param_2: u32)

{
    DAT_004bf94c = param_1;
    DAT_004bf950 = param_2;
    return;
}



fn FUN_0049d2e0(param_1: i32,param_2: u32,param_3: &mut String) -> u32

{
    let mut iVar1: i32;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let puVar5: *mut u32;
    let puVar6: *mut u32;
    let mut pcVar7: String;
    let mut iVar8: i32;
    let mut apuStack388: *mut u32 [0xb];
    let bStack343: u8;
    let local_13f: *mut *mut u8;;
    let mut local_13b: u32;
    let mut local_137: u32;
    let mut local_133: u32;
    let mut local_12f: u32;
    let mut local_12b: u32;
    let mut uStack295: u32;
    let auStack147: u8 [0x7];
    let mut uStack140: u32;
    let uStack136: u8;
    let mut local_48: String;
    let local_44: u16;
    let mut local_40: i32;
    let mut local_3c: i32;
    let mut local_38: String;
    let mut local_34: String;
    let mut local_30: i32;
    let mut local_2c: i32;
    let mut local_28: i32;
    let mut local_24: i32;
    let mut local_20: i32;
    let local_1c: *mut u32;
    let mut local_18: u32;
    let local_14: *mut u32;

    iVar1 = DAT_005b9210;
    local_1c = param_2;
    pcVar7 = 0x0;
    local_30 = param_1;
    local_18 = 0x0;
    local_20 = 0x0;
    if (((param_2 & 0xf) == 0x1) && ((param_2 & 0x20) != 0x0)) {
        local_1c = (param_2 | 0x10);
    }
    uVar2 = local_1c & 0xf00;
    local_1c = (local_1c & 0xfffff0ff);
    if (0xff < uVar2) {
        if (uVar2 < 0x101) {
            pcVar7 = s_VQUEST_BIN_004c344c;
        }
        else {
            if (uVar2 == 0x200) {
                pcVar7 = s_VEXCLAM_BIN_004c3457;
            }
        }
    }
    local_44 = SUB42(local_1c,0x0);
    iVar3 = (DAT_005b9228 * 0x55) / 0x64;
    if (pcVar7 != 0x0) {
        iVar3 = iVar3 - (DAT_005b9210 * 0x2 + DAT_004bf94c);
    }
    local_48 = pcVar7;
    iVar3 = FUN_004a3840(param_3,(auStack147 + 0x3),iVar3,0xc,&local_40,LPCSTR_005b9218,0x0);
    if (iVar3 == 0x0) {
        pop_err_msg_box_and_exit_004a02f5((LPSTR)0x0);
    }
    *(&uStack140 + iVar3 * 0x6) = 0x0;
    if (pcVar7 != 0x0) {
        iVar4 = DAT_004bf950 / DAT_005b9210 + 0x1;
        if (iVar3 < iVar4) {
            iVar3 = iVar4;
        }
        local_3c = local_3c + iVar1 * 0x2 + DAT_004bf94c;
    }
    local_3c = local_3c / DAT_005b9214;
    if (local_3c < (((local_18 & 0xf) == 0x4) * 0x5 + 0x14)) {
        local_3c = 0x14;
    }
    iVar8 = (local_3c + 0x4) * DAT_005b9214;
    iVar4 = DAT_005b922c / 0x2 - iVar1 * (iVar3 / 0x2 + 0x3);
    iVar3 = iVar1 * (iVar3 + 0x4);
    FUN_0049a030(apuStack388,local_2c,0x1,(DAT_005b9228 - iVar8) / 0x2,iVar4,iVar8,iVar3,0x0,DAT_005b9b70,DAT_004bf948);
    local_13f = &PTR_FUN_004c3d34;
    uStack295 = 0x0;
    local_13b = 0x0;
    local_137 = 0x0;
    local_133 = 0x0;
    auStack147._0_4_ = 0x0;
    local_12b = 0x0;
    local_12f = 0x0;
    puVar5 = FUN_0049a250(apuStack388,0x4e);
    *puVar5 = uStack140;
    (puVar5 + 0x1) = uStack136;
    switch(local_18 & 0xf) {
    default:
        local_30 = DAT_005b9228 + DAT_005b9214 * -0x6 >> 0x1;
    puVar5 = FUN_004a2831(0x5d);
    puVar6 = local_1c;
    if (puVar5 != 0x0) {
        pcVar7 = FUN_00499050(DAT_005b9bd8,0x791d);
        puVar5 = FUN_0049a030(puVar5,apuStack388,0xd,local_30,iVar3 + iVar4 + iVar1 * -0x2,0x4e26,0x4e21,0x0,
                              DAT_005b9b70,DAT_004bf948);
        $1: &mut String(puVar5 + 0x45) = &PTR_FUN_004c3d94;
        *(puVar5 + 0x55) = 0x0;
        *(puVar5 + 0x4d) = 0x0;
        *(puVar5 + 0x49) = 0x2;
        *(puVar5 + 0x51) = pcVar7;
        puVar6 = local_1c;
    }
    break;
    case 0x2:
        local_28 = DAT_005b9228 + DAT_005b9214 * -0x10 >> 0x1;
    puVar5 = FUN_004a2831(0x5d);
    if (puVar5 != 0x0) {
        pcVar7 = FUN_00499050(DAT_005b9bd8,0x791d);
        puVar5 = FUN_0049a030(puVar5,apuStack388,0xd,local_28,iVar4 + iVar3 + iVar1 * -0x2,0x4e26,0x4e21,0x0,
                              DAT_005b9b70,DAT_004bf948);
        $1: &mut String(puVar5 + 0x45) = &PTR_FUN_004c3d94;
        *(puVar5 + 0x55) = 0x0;
        *(puVar5 + 0x4d) = 0x0;
        *(puVar5 + 0x49) = 0x2;
        *(puVar5 + 0x51) = pcVar7;
    }
    local_14 = FUN_004a2831(0x5d);
    puVar6 = local_1c;
    if (local_14 != 0x0) {
        local_34 = FUN_00499050(DAT_005b9bd8,0x791a);
        local_14 = FUN_0049a030(local_14,apuStack388,0xe,DAT_005b9214 * 0x8 + local_28,iVar3 + iVar4 + iVar1 * -0x2,
                                0x4e28,0x4e21,0x0,DAT_005b9b70,DAT_004bf948);
        $1: &mut String(local_14 + 0x45) = &PTR_FUN_004c3d94;
        *(local_14 + 0x55) = 0x0;
        *(local_14 + 0x4d) = 0x0;
        *(local_14 + 0x49) = 0x2;
        *(local_14 + 0x51) = local_34;
        puVar6 = local_1c;
    }
    break;
    case 0x3:
        local_24 = DAT_005b9228 + DAT_005b9214 * -0xc >> 0x1;
    puVar5 = FUN_004a2831(0x5d);
    if (puVar5 != 0x0) {
        pcVar7 = FUN_00499050(DAT_005b9bd8,0x791b);
        puVar5 = FUN_0049a030(puVar5,apuStack388,0xd,local_24,iVar4 + iVar3 + iVar1 * -0x2,0x4e25,0x4e21,0x0,
                              DAT_005b9b70,DAT_004bf948);
        $1: &mut String(puVar5 + 0x45) = &PTR_FUN_004c3d94;
        *(puVar5 + 0x55) = 0x0;
        *(puVar5 + 0x4d) = 0x0;
        *(puVar5 + 0x49) = 0x2;
        *(puVar5 + 0x51) = pcVar7;
    }
    local_14 = FUN_004a2831(0x5d);
    puVar6 = local_1c;
    if (local_14 != 0x0) {
        local_34 = FUN_00499050(DAT_005b9bd8,0x791c);
        local_14 = FUN_0049a030(local_14,apuStack388,0xe,local_24 + DAT_005b9214 * 0x8,iVar3 + iVar4 + iVar1 * -0x2,
                                0x4e24,0x4e21,0x0,DAT_005b9b70,DAT_004bf948);
        $1: &mut String(local_14 + 0x45) = &PTR_FUN_004c3d94;
        *(local_14 + 0x55) = 0x0;
        *(local_14 + 0x4d) = 0x0;
        *(local_14 + 0x49) = 0x2;
        *(local_14 + 0x51) = local_34;
        puVar6 = local_1c;
    }
    break;
    case 0x4:
        local_20 = DAT_005b9228 + DAT_005b9214 * -0x15 >> 0x1;
    puVar5 = FUN_004a2831(0x5d);
    if (puVar5 != 0x0) {
        pcVar7 = FUN_00499050(DAT_005b9bd8,0x791b);
        puVar5 = FUN_0049a030(puVar5,apuStack388,0xd,local_20,iVar4 + iVar3 + iVar1 * -0x2,0x4e25,0x4e21,0x0,
                              DAT_005b9b70,DAT_004bf948);
        $1: &mut String(puVar5 + 0x45) = &PTR_FUN_004c3d94;
        *(puVar5 + 0x55) = 0x0;
        *(puVar5 + 0x4d) = 0x0;
        *(puVar5 + 0x49) = 0x2;
        *(puVar5 + 0x51) = pcVar7;
    }
    local_14 = FUN_004a2831(0x5d);
    if (local_14 != 0x0) {
        local_38 = FUN_00499050(DAT_005b9bd8,0x791c);
        local_34 = (iVar4 + iVar3 + iVar1 * -0x2);
        local_14 = FUN_0049a030(local_14,apuStack388,0xe,DAT_005b9214 * 0x7 + local_20,local_34,0x4e24,0x4e21,
                                0x0,DAT_005b9b70,DAT_004bf948);
        $1: &mut String(local_14 + 0x45) = &PTR_FUN_004c3d94;
        *(local_14 + 0x55) = 0x0;
        *(local_14 + 0x4d) = 0x0;
        *(local_14 + 0x49) = 0x2;
        *(local_14 + 0x51) = local_38;
    }
    puVar6 = FUN_004a2831(0x5d);
    if (puVar6 != 0x0) {
        local_34 = FUN_00499050(DAT_005b9bd8,0x791a);
        puVar6 = FUN_0049a030(puVar6,apuStack388,0xf,local_20 + DAT_005b9214 * 0xd,iVar3 + iVar4 + iVar1 * -0x2,
                              0x4e28,0x4e21,0x0,DAT_005b9b70,DAT_004bf948);
        $1: &mut String(puVar6 + 0x45) = &PTR_FUN_004c3d94;
        *(puVar6 + 0x55) = 0x0;
        *(puVar6 + 0x4d) = 0x0;
        *(puVar6 + 0x49) = 0x2;
        *(puVar6 + 0x51) = local_34;
    }
}
    local_1c = puVar6;
    if (puVar5 == 0x0) {
        pop_err_msg_box_and_exit_004a02f5((LPSTR)0x0);
    }
    FUN_0049bf40(apuStack388,puVar5);
    if (local_14 != 0x0) {
        FUN_0049bf40(apuStack388,local_14);
    }
    if (local_1c != 0x0) {
        FUN_0049bf40(apuStack388,local_1c);
    }
    if ((local_18 & 0x40) != 0x0) {
        bStack343 = bStack343 | 0x4;
    }
    uVar2 = FUN_0049bb50(apuStack388,FUN_0049e7a0);
    if (puVar5 != 0x0) {
        ((*(puVar5 + 0x45) + 0x8))(puVar5,0x2);
    }
    if ((local_14 != 0x0) && (local_14 != 0x0)) {
        ((*(local_14 + 0x45) + 0x8))(local_14,0x2);
    }
    if ((local_1c != 0x0) && (local_1c != 0x0)) {
        ((*(local_1c + 0x45) + 0x8))(local_1c,0x2);
    }
    FUN_004b08b0();
    if (uVar2 == 0x2) {
        uVar2 = 0xffffffff;
    }
    local_13f = &PTR_FUN_004c3d34;
    if (auStack147._0_4_ != 0x0) {
        FUN_00499b30(apuStack388,auStack147._0_4_);
    }
    FUN_0049a1c0(apuStack388,0x1);
    return uVar2;
}



fn FUN_0049dc40(param_1: u32,param_2: u32,param_3: &mut String,param_4: &mut String) -> u32

{
    let mut iVar1: i32;
    let mut uVar2: u32;
    let mut iVar3: i32;
    let mut iVar4: i32;
    let puVar5: *mut u32;
    let mut pcVar6: String;
    let puVar7: *mut u32;
    let mut iVar8: i32;
    let mut apuStack372: *mut u32 [0xb];
    let bStack327: u8;
    let local_12f: *mut *mut u8;;
    let mut local_12b: u32;
    let mut local_127: u32;
    let mut local_123: u32;
    let mut local_11f: u32;
    let mut local_11b: u32;
    let mut uStack279: u32;
    let auStack131: u8 [0x7];
    let mut uStack124: u32;
    let uStack120: u8;
    let mut local_38: String;
    let local_34: u16;
    let mut local_30: i32;
    let mut local_2c: i32;
    let mut local_28: String;
    let mut local_24: i32;
    let mut local_20: i32;
    let mut local_1c: i32;
    let local_18: *mut u32;
    let mut local_14: i32;

    iVar1 = DAT_005b9210;
    puVar7 = 0x0;
    local_1c = 0x0;
    if (((param_2 & 0xf) == 0x1) && ((param_2 & 0x20) != 0x0)) {
        param_2 = param_2 | 0x10;
    }
    uVar2 = param_2 & 0xf00;
    param_2 = param_2 & 0xfffff0ff;
    if ((param_4 == 0x0) && (0xff < uVar2)) {
        if (uVar2 < 0x101) {
            param_4 = s_VQUEST_BIN_004c344c;
        }
        else {
            if (uVar2 == 0x200) {
                param_4 = s_VEXCLAM_BIN_004c3457;
            }
        }
    }
    local_34 = (undefined2)param_2;
    iVar3 = (DAT_005b9228 * 0x55) / 0x64;
    if (param_4 != 0x0) {
        iVar3 = iVar3 - (DAT_005b9210 * 0x2 + DAT_004bf94c);
    }
    local_38 = param_4;
    iVar3 = FUN_004a3840(param_3,(auStack131 + 0x3),iVar3,0xc,&local_30,LPCSTR_005b9218,0x0);
    if (iVar3 == 0x0) {
        pop_err_msg_box_and_exit_004a02f5((LPSTR)0x0);
    }
    *(&uStack124 + iVar3 * 0x6) = 0x0;
    if (param_4 != 0x0) {
        iVar4 = DAT_004bf950 / DAT_005b9210 + 0x1;
        if (iVar3 < iVar4) {
            iVar3 = iVar4;
        }
        local_2c = local_2c + iVar1 * 0x2 + DAT_004bf94c;
    }
    local_2c = local_2c / DAT_005b9214;
    if (local_2c < (((param_3 & 0xf) == 0x4) * 0x5 + 0x14)) {
        local_2c = 0x14;
    }
    iVar8 = (local_2c + 0x4) * DAT_005b9214;
    local_28 = (iVar1 * (iVar3 / 0x2 + 0x3));
    iVar4 = DAT_005b922c / 0x2 - local_28;
    local_14 = iVar1 * (iVar3 + 0x4);
    FUN_0049a030(apuStack372,param_2,0x1,(DAT_005b9228 - iVar8) / 0x2,iVar4,iVar8,local_14,0x0,DAT_005b9b70,DAT_004bf948);
    local_12f = &PTR_FUN_004c3d34;
    uStack279 = 0x0;
    local_12b = 0x0;
    local_127 = 0x0;
    local_123 = 0x0;
    auStack131._0_4_ = 0x0;
    local_11b = 0x0;
    local_11f = 0x0;
    puVar5 = FUN_0049a250(apuStack372,0x4e);
    *puVar5 = uStack124;
    (puVar5 + 0x1) = uStack120;
    switch(param_3 & 0xf) {
    default:
        local_24 = DAT_005b9228 + DAT_005b9214 * -0x6 >> 0x1;
    puVar5 = FUN_004a2831(0x5d);
    if (puVar5 != 0x0) {
        pcVar6 = FUN_00499050(DAT_005b9bd8,0x791d);
        puVar5 = FUN_0049a030(puVar5,apuStack372,0xd,local_24,local_14 + iVar4 + iVar1 * -0x2,0x4e26,0x4e21,0x0,
                              DAT_005b9b70,DAT_004bf948);
        $1: &mut String(puVar5 + 0x45) = &PTR_FUN_004c3d94;
        *(puVar5 + 0x55) = 0x0;
        *(puVar5 + 0x4d) = 0x0;
        *(puVar5 + 0x49) = 0x2;
        *(puVar5 + 0x51) = pcVar6;
    }
    break;
    case 0x2:
        iVar3 = DAT_005b9228 + DAT_005b9214 * -0x10;
    puVar5 = FUN_004a2831(0x5d);
    iVar3 = iVar3 >> 0x1;
    if (puVar5 != 0x0) {
        pcVar6 = FUN_00499050(DAT_005b9bd8,0x791d);
        local_28 = (iVar4 + local_14 + iVar1 * -0x2);
        puVar5 = FUN_0049a030(puVar5,apuStack372,0xd,iVar3,local_28,0x4e26,0x4e21,0x0,DAT_005b9b70,DAT_004bf948)
        ;
        $1: &mut String(puVar5 + 0x45) = &PTR_FUN_004c3d94;
        *(puVar5 + 0x55) = 0x0;
        *(puVar5 + 0x4d) = 0x0;
        *(puVar5 + 0x49) = 0x2;
        *(puVar5 + 0x51) = pcVar6;
    }
    puVar7 = FUN_004a2831(0x5d);
    if (puVar7 != 0x0) {
        local_28 = FUN_00499050(DAT_005b9bd8,0x791a);
        puVar7 = FUN_0049a030(puVar7,apuStack372,0xe,iVar3 + DAT_005b9214 * 0x8,iVar4 + local_14 + iVar1 * -0x2,
                              0x4e28,0x4e21,0x0,DAT_005b9b70,DAT_004bf948);
        $1: &mut String(puVar7 + 0x45) = &PTR_FUN_004c3d94;
        *(puVar7 + 0x55) = 0x0;
        *(puVar7 + 0x4d) = 0x0;
        *(puVar7 + 0x49) = 0x2;
        *(puVar7 + 0x51) = local_28;
    }
    break;
    case 0x3:
        local_20 = DAT_005b9228 + DAT_005b9214 * -0xc >> 0x1;
    puVar5 = FUN_004a2831(0x5d);
    if (puVar5 != 0x0) {
        pcVar6 = FUN_00499050(DAT_005b9bd8,0x791b);
        puVar5 = FUN_0049a030(puVar5,apuStack372,0xd,local_20,iVar4 + local_14 + iVar1 * -0x2,0x4e25,0x4e21,0x0,
                              DAT_005b9b70,DAT_004bf948);
        $1: &mut String(puVar5 + 0x45) = &PTR_FUN_004c3d94;
        *(puVar5 + 0x55) = 0x0;
        *(puVar5 + 0x4d) = 0x0;
        *(puVar5 + 0x49) = 0x2;
        *(puVar5 + 0x51) = pcVar6;
    }
    puVar7 = FUN_004a2831(0x5d);
    if (puVar7 != 0x0) {
        pcVar6 = FUN_00499050(DAT_005b9bd8,0x791c);
        puVar7 = FUN_0049a030(puVar7,apuStack372,0xe,DAT_005b9214 * 0x8 + local_20,iVar4 + local_14 + iVar1 * -0x2,
                              0x4e24,0x4e21,0x0,DAT_005b9b70,DAT_004bf948);
        $1: &mut String(puVar7 + 0x45) = &PTR_FUN_004c3d94;
        *(puVar7 + 0x55) = 0x0;
        *(puVar7 + 0x4d) = 0x0;
        *(puVar7 + 0x49) = 0x2;
        *(puVar7 + 0x51) = pcVar6;
    }
    break;
    case 0x4:
        local_1c = DAT_005b9228 + DAT_005b9214 * -0x15 >> 0x1;
    puVar5 = FUN_004a2831(0x5d);
    if (puVar5 != 0x0) {
        pcVar6 = FUN_00499050(DAT_005b9bd8,0x791b);
        puVar5 = FUN_0049a030(puVar5,apuStack372,0xd,local_1c,iVar4 + local_14 + iVar1 * -0x2,0x4e25,0x4e21,0x0,
                              DAT_005b9b70,DAT_004bf948);
        $1: &mut String(puVar5 + 0x45) = &PTR_FUN_004c3d94;
        *(puVar5 + 0x55) = 0x0;
        *(puVar5 + 0x4d) = 0x0;
        *(puVar5 + 0x49) = 0x2;
        *(puVar5 + 0x51) = pcVar6;
    }
    puVar7 = FUN_004a2831(0x5d);
    if (puVar7 != 0x0) {
        pcVar6 = FUN_00499050(DAT_005b9bd8,0x791c);
        local_28 = (iVar4 + local_14 + iVar1 * -0x2);
        puVar7 = FUN_0049a030(puVar7,apuStack372,0xe,local_1c + DAT_005b9214 * 0x7,local_28,0x4e24,0x4e21,0x0,
                              DAT_005b9b70,DAT_004bf948);
        $1: &mut String(puVar7 + 0x45) = &PTR_FUN_004c3d94;
        *(puVar7 + 0x55) = 0x0;
        *(puVar7 + 0x4d) = 0x0;
        *(puVar7 + 0x49) = 0x2;
        *(puVar7 + 0x51) = pcVar6;
    }
    local_18 = FUN_004a2831(0x5d);
    if (local_18 != 0x0) {
        local_28 = FUN_00499050(DAT_005b9bd8,0x791a);
        local_18 = FUN_0049a030(local_18,apuStack372,0xf,local_1c + DAT_005b9214 * 0xd,
                                iVar4 + local_14 + iVar1 * -0x2,0x4e28,0x4e21,0x0,DAT_005b9b70,DAT_004bf948);
        $1: &mut String(local_18 + 0x45) = &PTR_FUN_004c3d94;
        *(local_18 + 0x55) = 0x0;
        *(local_18 + 0x4d) = 0x0;
        *(local_18 + 0x49) = 0x2;
        *(local_18 + 0x51) = local_28;
    }
}
    if (puVar5 == 0x0) {
        pop_err_msg_box_and_exit_004a02f5((LPSTR)0x0);
    }
    FUN_0049bf40(apuStack372,puVar5);
    if (puVar7 != 0x0) {
        FUN_0049bf40(apuStack372,puVar7);
    }
    if (local_18 != 0x0) {
        FUN_0049bf40(apuStack372,local_18);
    }
    if ((param_3 & 0x40) != 0x0) {
        bStack327 = bStack327 | 0x4;
    }
    uVar2 = FUN_0049bb50(apuStack372,FUN_0049e7a0);
    if (puVar5 != 0x0) {
        ((*(puVar5 + 0x45) + 0x8))(puVar5,0x2);
    }
    if ((puVar7 != 0x0) && (puVar7 != 0x0)) {
        ((*(puVar7 + 0x45) + 0x8))(puVar7,0x2);
    }
    if ((local_18 != 0x0) && (local_18 != 0x0)) {
        ((*(local_18 + 0x45) + 0x8))(local_18,0x2);
    }
    FUN_004b08b0();
    if (uVar2 == 0x2) {
        uVar2 = 0xffffffff;
    }
    local_12f = &PTR_FUN_004c3d34;
    if (auStack131._0_4_ != 0x0) {
        FUN_00499b30(apuStack372,auStack131._0_4_);
    }
    FUN_0049a1c0(apuStack372,0x1);
    return uVar2;
}



fn FUN_0049e5a0(param_1: i32,param_2: i32,param_3: i32,param_4: i32,undefined param_5)

{
    let mut iVar1: i32;
    let mut iVar2: i32;
    let mut iVar3: i32;
    let mut iVar4: i32;

    iVar1 = FUN_00499fb0(param_1);
    iVar2 = FUN_00499fb0(param_2);
    iVar3 = FUN_00499ff0(param_3);
    iVar4 = FUN_00499ff0(param_4);
    iVar4 = iVar4 + iVar2 + -0x1;
    FUN_004953d7();
    FUN_00497896(iVar1,iVar2,iVar1,iVar4,param_5);
    iVar3 = iVar3 + iVar1 + -0x1;
    FUN_00497896(iVar1,iVar2,iVar3,iVar2,param_5);
    FUN_00497896(iVar1,iVar4,iVar3,iVar4,param_5);
    FUN_00497896(iVar3,iVar2,iVar3,iVar4,param_5);
    FUN_0049536f();
    return;
}



void
FUN_0049e640(param_1: i32,param_2: i32,param_3: i32,param_4: i32,undefined param_5,undefined param_6,undefined param_7,
param_8: i32)

{
let mut iVar1: i32;
let mut iVar2: i32;
let mut iVar3: i32;
let mut iVar4: i32;
let mut iVar5: i32;

iVar1 = FUN_00499fb0(param_1);
iVar2 = FUN_00499fb0(param_3);
iVar3 = FUN_00499ff0(param_2);
iVar4 = FUN_00499ff0(param_4);
FUN_004953d7();
iVar5 = iVar4 + iVar3 + -0x1 + param_8;
iVar4 = iVar1 - param_8;
iVar3 = iVar3 - param_8;
FUN_00497896(iVar4,iVar3,iVar4,iVar5,param_5);
iVar1 = iVar2 + iVar1 + -0x1 + param_8;
FUN_00497896(iVar4,iVar3,iVar1,iVar3,param_5);
FUN_00497896(iVar4,iVar5,iVar1,iVar5,param_6);
FUN_00497896(iVar1,iVar3,iVar1,iVar5,param_6);
FUN_00496a3b(iVar4,iVar5,param_7);
FUN_00496a3b(iVar1,iVar3,param_7);
FUN_0049536f();
return;
}



void  timer_func_0049e710(DWORD param_time_val_1)

{
let sys_time_millis: u32;
let sys_time_millis_2: u32;
let mut time_elapsed_millis: u32;

sys_time_millis = timeGetTime();
while( true ) {
sys_time_millis_2 = timeGetTime();
time_elapsed_millis = (sys_time_millis_2 - sys_time_millis) >> 0x1f;
if (param_time_val_1 <=
((sys_time_millis_2 - sys_time_millis ^ time_elapsed_millis) - time_elapsed_millis)) break;
win_handle_func_0049fb83(DAT_005b9bd4);
FUN_004ae8ec();
}
return;
}



fn FUN_0049e750(param_1: *mut u32) -> *mut u32

{
    let cVar1: u8;
    let puVar2: *mut u32;
    let mut uVar3: u32;

    uVar3 = 0xffffffff;
    puVar2 = param_1;
    loop {
    if (uVar3 == 0x0) break;
    uVar3 = uVar3 - 0x1;
    cVar1 = puVar2;
    puVar2 = (puVar2 + 0x1);
} while (cVar1 != '\0');
    puVar2 = FUN_0049c2c9(~uVar3);
    if (puVar2 == 0x0) {
        return 0x0;
    }
    *puVar2 = *param_1;
    (puVar2 + 0x1) = (param_1 + 0x1);
    return puVar2;
}



fn FUN_0049e7a0(param_1: i32,param_2: u32,param_3: i32,param_4: i32) -> u32

{
    let piVar1: *mut i32;;
    let mut bVar2: bool;
    let mut iVar3: i32;
    let puVar4: *mut u32;
    undefined3 extraout_var;
    let mut iVar5: i32;
    let mut uVar6: u32;
    let mut iVar7: i32;
    let mut iVar8: i32;
    let piVar9: *mut i32;;
    let piVar10: *mut i32;;
    let mut iVar11: i32;
    let mut local_14: i32;

    if (param_2 < 0x405) {
        if (((param_2 == 0x100) && (param_4 == 0x1b)) &&
            ((*(*(param_1 + 0x18) + 0x4a) >> 0x10 & 0x10U) != 0x0)) {
            *(param_1 + 0x2d) = *(param_1 + 0x2d) | 0x1;
        }
        return 0x0;
    }
    if (0x405 < param_2) {
        if (param_2 != 0x407) {
            return 0x0;
        }
        uVar6 = *(*(param_1 + 0x18) + 0x4a) >> 0x10;
        if (param_3 == 0xd) {
            if ((uVar6 & 0x20) != 0x0) {
                *(param_1 + 0x2d) = *(param_1 + 0x2d) | 0x1;
            }
            iVar11 = 0x1;
        }
        else {
            if ((uVar6 & 0x10) != 0x0) {
                *(param_1 + 0x2d) = *(param_1 + 0x2d) | 0x1;
            }
            if (param_3 == 0xf) {
                iVar11 = 0x2;
            }
            else {
                iVar11 = 0x0;
            }
        }
        FUN_0049c140(param_1,iVar11);
        return 0x1;
    }
    FUN_004953d7();
    piVar10 = *(i32 **)(param_1 + 0x18);
    iVar7 = 0x0;
    iVar11 = *piVar10;
    piVar9 = piVar10;
    while (iVar11 != 0x0) {
        piVar1 = (piVar9 + 0x6);
        piVar9 = (piVar9 + 0x6);
        iVar7 = iVar7 + 0x1;
        iVar11 = *piVar1;
    }
    iVar11 = *(param_1 + 0x21) + (*(param_1 + 0x29) - (iVar7 + 0x2) * DAT_005b9210) / 0x2;
    local_14 = DAT_005b9228 / 0x2;
    if (piVar10[0x12] != 0x0) {
        iVar7 = DAT_004bf94c;
        if (DAT_004bf94c < 0x0) {
            if (DAT_005b9210 == 0x10) {
                iVar7 = DAT_004bf94c * -0x2;
            }
            else {
                iVar7 = -DAT_004bf94c;
            }
        }
        iVar3 = DAT_004bf950;
        if (DAT_004bf950 < 0x0) {
            if (DAT_005b9210 == 0x10) {
                iVar3 = DAT_004bf950 * -0x2;
            }
            else {
                iVar3 = -DAT_004bf950;
            }
        }
        local_14 = local_14 + (DAT_005b9214 + iVar7 >> 0x1);
        if (DAT_004bf94c < 0x0) {
            if (DAT_005b9228 < 0x1f5) {
                piVar10[0x12] = 0x56;
            }
            else {
                piVar10[0x12] = 0x53;
            }
        }
        puVar4 = FUN_0049c2c9(iVar3 * iVar7);
        if (puVar4 != 0x0) {
            bVar2 = FUN_0049c57b(piVar10[0x12],puVar4,iVar3 * iVar7);
            if (CONCAT31(extraout_var,bVar2) != 0x0) {
                iVar8 = *(param_1 + 0x21) + ((*(param_1 + 0x29) + DAT_005b9210 * -0x2) - iVar3 >> 0x1);
                if (DAT_005b9210 == 0x10) {
                    iVar5 = 0xe;
                }
                else {
                    iVar5 = 0x0;
                }
                FUN_0049e640(iVar5 + 0xe + *(param_1 + 0x1d),iVar8,iVar7,iVar3,(char)DAT_004bf924,(char)DAT_005b981c,
                             (char)DAT_004bf928,0x1);
                if (DAT_005b9210 == 0x10) {
                    iVar5 = 0xe;
                }
                else {
                    iVar5 = 0x0;
                }
                FUN_00496ac0(puVar4,iVar5 + 0xe + *(param_1 + 0x1d),iVar8,iVar7,iVar3);
            }
            FUN_0049af50(puVar4);
        }
    }
    iVar7 = *piVar10;
    while (iVar7 != 0x0) {
        piVar9 = (piVar10 + 0x2);
        iVar7 = *piVar10;
        piVar10 = (piVar10 + 0x6);
        FUN_00497567(local_14,iVar11,iVar7,*piVar9 >> 0x10,(&DAT_005b9b74)[*(param_1 + 0x41) % 0x2],-0x1,
                     (&DAT_005b9b74)[*(param_1 + 0x41) % 0x2],LPCSTR_005b9218,0x1);
        iVar11 = iVar11 + DAT_005b9210;
        iVar7 = *piVar10;
    }
    FUN_0049536f();
    return 0x0;
}



fn FUN_0049ea90() -> *mut u32

{
    let puVar1: *mut u32;
    let mut pcVar2: String;

    puVar1 = FUN_004a2831(0x16);
    if (puVar1 != 0x0) {
        *puVar1 = 0x0;
        puVar1[0x2] = puVar1;
        puVar1[0x1] = puVar1;
        (puVar1 + 0xe) = puVar1;
        (puVar1 + 0x12) = puVar1;
        return puVar1;
    }
    pcVar2 = FUN_00499050(DAT_005b9bd8,0x7d01);
    pop_err_msg_box_and_exit_004a02f5(pcVar2);
    return 0x0;
}



fn FUN_0049eae0(param_1: i32,param_2: *mut u32,param_3: u32) -> u32

{
    let puVar1: *mut u32;
    let puVar2: *mut u32;

    puVar1 = FUN_0049c2c9(param_3);
    if (puVar1 == 0x0) {
        return 0x0;
    }
    *puVar1 = *param_2;
    (puVar1 + 0x1) = (param_2 + 0x1);
    puVar2 = FUN_0049eb60(param_1,puVar1,0x0);
    if (puVar2 != 0x0) {
        return 0x1;
    }
    FUN_0049af50(puVar1);
    return 0x0;
}



fn FUN_0049eb40(param_1: i32,param_2: u32)

{
    FUN_0049eb60(param_1,param_2,0x1);
    return;
}



fn FUN_0049eb60(param_1: i32,param_2: u32,undefined2 param_3) -> *mut u32

{
    let puVar1: *mut u32;
    let mut iVar2: i32;

    if (param_1 == *(param_1 + 0x12)) {
        puVar1 = FUN_004a2831(0xe);
        if (puVar1 != 0x0) {
            *puVar1 = param_2;
            iVar2 = *(param_1 + 0x4);
            if (*(param_1 + 0x4) == 0x0) {
                iVar2 = param_1;
            }
            puVar1[0x1] = iVar2;
            puVar1[0x2] = *(iVar2 + 0x8);
            (iVar2 + 0x8) = puVar1;
            (puVar1[0x2] + 0x4) = puVar1;
            *(puVar1 + 0x3) = param_3;
            return 0x1;
        }
    }
    else {
        puVar1 = 0x0;
    }
    return puVar1;
}

nm
