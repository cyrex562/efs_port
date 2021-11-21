







fn FUN_004af950(param_1: i32,param_2: *mut u8, param_3: &mut i32,param_4: i32) -> i32

{
  let DVar1: u32;
  let puVar2: *mut u32;
  let mut iVar3: i32;
  let DVar4: u32;
  let mut iVar5: i32;
  let mut unaff_EDI: i32 = 0;
  let DStack48: u32;
  let mut uStack44: u32;
  let mut iStack40: i32;
  let LStack36: i32;
  let mut local_20: i32;
  let local_1c: u32;
  let mut uStack24: u32;
  let mut local_14: u32;
  
  (param_1 + 0xa8) = param_2;
  if param_3 < 0x1 {
    *param_3 = *(param_1 + 0x10);
  }
  local_14 = (param_1 + 0x6) as u32;
  if param_4 != 0x0 {
    local_14 = local_14 + 0x1;
    *(*(param_1 + 0x94) + 0x1c) = *(*(param_1 + 0x94) + 0x1c) | 0x4;
  }
  *(param_1 + 0x98) = 0x0;
  iVar5 = 0x0;
  loop {
    while local_14 <= iVar5 {
      if (param_4 != 0x0) && (*(param_1 + 0x98) == 0x0) {
        FUN_004aff90(*(param_1 + 0x94));
        FUN_004b00f0(*(param_1 + 0x94),*(param_1 + 0x54),0x0);
        iVar5 = 0x1;
        FUN_004b0070(*(param_1 + 0x94),*(param_1 + 0x54),0x0);
      }
      if (param_4 == 0x0) || (*(param_1 + 0x98) != 0x0) {
        return unaff_EDI;
      }
    }
    local_1c = timeGetTime();
    local_1c = param_3 + local_1c;
    if (*(param_1 + 0xa8) != 0x0) {
      ((param_1 + 0xa8))(param_1,iVar5,0x0);
    }
    local_20 = param_1;
    puVar2 = FUN_004b00f0(*(param_1 + 0x94),0x10,0x0);
    if (puVar2 + 0x1) == -0xe06 {
      uStack24 = *puVar2;
      if uStack24 < 0x11 {
        unaff_EDI = 0x0;
      }
      else {
        iVar3 = FUN_004b00f0(*(param_1 + 0x94),uStack24,0x0);
        unaff_EDI = FUN_004b6450(param_1,puVar2,(iVar3 + 0x10));
      }
      FUN_004b0070(*(local_20 + 0x94),uStack24,0x0);
      DAT_005ba420 = uStack24;
    }
    else {
      unaff_EDI = -0x4;
    }
    if unaff_EDI < 0x0 {
        break;
    }
    if iVar5 == 0x0 {
      *(param_1 + 0x54) = *(param_1 + 0x50) + DAT_005ba420;
    }
    DVar1 = local_1c;
    if param_2 != 0x0 {
      (param_2)(param_1,iVar5,0x1);
      DVar1 = local_1c;
    }
    loop {
      DVar4 = win_func_0049fe83(DAT_005b9bd4,&DStack48,&uStack44,&iStack40,&LStack36);
      if (DVar4 != 0x0) && ((uStack44 == 0x100 || (uStack44 == 0x205)) || (uStack44 == 0x202)) {
        unaff_EDI = -0xa;
        //^ // goto LAB_004afad5;
      }
      DVar4 = timeGetTime();
      if DVar1 == DVar4 {
        unaff_EDI = 0x0;^
        // goto LAB_004afad5;
      }
      DVar4 = timeGetTime();
    } while DVar1 - DVar4 < 0x3e8;
    unaff_EDI = 0x0;
// LAB_004afad5:
    if unaff_EDI < 0x0 {
        break;
    }
    iVar5 = iVar5 + 0x1;
  }
  *(param_1 + 0x98) = 0x1;
  return unaff_EDI;
}



fn fun_004afbf0(param_1: i32)

{
  if *(param_1 + 0xa4) == 0x0 {
    return;
  }
  FUN_00496ac0((param_1 + 0xa4),*(param_1 + 0x8c),*(param_1 + 0x90),
               (param_1 + 0x8),(param_1 + 0xa));
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_004afc30(param_1: i32,param_2: i32)

{
  let bVar1: u8;
  let DVar2: u32;
  let DVar3: u32;
  let DVar4: u32;
  let mut iVar5: i32;
  let mut iVar6: i32;
  
  if (_DAT_005ba424 != 0x0) {
    param_2 = 0x1;
  }
// LAB_004afc49:
  loop {
    if ((_DAT_005ba424 == 0x0) && (DAT_005b9db4 != 0x0))^ // goto LAB_004afc63;
    DVar2 = timeGetTime();
    if ((DVar2 == *(DWORD *)(param_1 + 0xc)) || (*(param_1 + 0x38) < 0x1))^ // goto LAB_004afc63;
    DVar2 = 0x0;
    *(DWORD *)(param_1 + 0xc) = *(DWORD *)(param_1 + 0xc) + 0x5;
    iVar5 = *(param_1 + 0x44) + *(param_1 + 0x18);
    iVar6 = *(param_1 + 0x14);
    *(param_1 + 0x44) = iVar5;
    if (iVar5 < 0x2000) {
      if ((param_2 == 0x0) ||
         (DVar2 = timeGetTime(), *(param_1 + 0xc) <= DVar2 && DVar2 != *(param_1 + 0xc)))^
      // goto LAB_004afc49;
      DVar2 = *(DWORD *)(param_1 + 0x44);
      *(param_1 + 0x44) = 0x0;
    }
    else {
      loop {
        iVar5 = *(param_1 + 0x44) + -0x2000;
        DVar2 = DVar2 + 0x2000;
        *(param_1 + 0x44) = iVar5;
      } while (0x1fff < iVar5);
    }
    if (*(DWORD *)(param_1 + 0x38) < DVar2) {
      DVar2 = *(DWORD *)(param_1 + 0x38);
    }
    if (iVar6 == *(param_1 + 0x10)) {
      return;
    }
    if (*(param_1 + 0x8) < (*(param_1 + 0x10) + DVar2)) {
      if (iVar6 < *(param_1 + 0x10)) {
        DVar3 = *(param_1 + 0x8) - *(param_1 + 0x10);
        DVar4 = timeGetTime();
        read_file_func_004ab180
                  (*(param_1 + 0x4),(*(param_1 + 0x20) + *(param_1 + 0x10)),DVar3);
        *(param_1 + 0x10) = 0x0;
        DVar2 = DVar2 - DVar3;
        *(DWORD *)(param_1 + 0x38) = *(param_1 + 0x38) - DVar3;
        DVar3 = timeGetTime();
        if ((DVar3 < DVar4 + 0x5) || (0x2000 < DVar2))^ // goto LAB_004afe0b;
// LAB_004afe7a:
        DVar2 = timeGetTime();
        *(DWORD *)(param_1 + 0xc) = DVar2;
      }
      else {
        if (param_2 != 0x0) {
          DVar3 = timeGetTime();
          read_file_func_004ab180
                    (*(param_1 + 0x4),(*(param_1 + 0x20) + *(param_1 + 0x10)),
                     iVar6 - *(param_1 + 0x10));
          iVar5 = *(param_1 + 0x10);
          *(param_1 + 0x10) = iVar6;
          *(param_1 + 0x38) = *(param_1 + 0x38) - (iVar6 - iVar5);
          DVar4 = timeGetTime();
          if ((DVar3 + 0x5 <= DVar4) && (DVar2 < 0x2001)) {
            DVar2 = timeGetTime();
            *(DWORD *)(param_1 + 0xc) = DVar2;
          }
        }
      }^
      // goto LAB_004afc63;
    }
// LAB_004afe0b:
    if ((*(param_1 + 0x10) <= iVar6) && (iVar6 <= (*(param_1 + 0x10) + DVar2))) {
      if ((iVar6 != *(param_1 + 0x10)) && (param_2 != 0x0)) {
        DVar2 = timeGetTime();
        read_file_func_004ab180
                  (*(param_1 + 0x4),(*(param_1 + 0x20) + *(param_1 + 0x10)),
                   iVar6 - *(param_1 + 0x10));
        iVar5 = *(param_1 + 0x10);
        *(param_1 + 0x10) = iVar6;
        *(param_1 + 0x38) = *(param_1 + 0x38) - (iVar6 - iVar5);
        DVar3 = timeGetTime();
        if (DVar2 + 0x5 <= DVar3)^ // goto LAB_004afe7a;
      }^
      // goto LAB_004afc63;
    }
    DVar3 = timeGetTime();
    read_file_func_004ab180
              (*(param_1 + 0x4),(*(param_1 + 0x20) + *(param_1 + 0x10)),DVar2);
    *(DWORD *)(param_1 + 0x38) = *(param_1 + 0x38) - DVar2;
    *(DWORD *)(param_1 + 0x10) = *(param_1 + 0x10) + DVar2;
    DVar2 = timeGetTime();
    if (DVar3 + 0x5 <= DVar2) break;
  }
  DVar2 = timeGetTime();
  *(DWORD *)(param_1 + 0xc) = DVar2;
// LAB_004afc63:
  if ((*(param_1 + 0x38) != 0x0) && (_DAT_005ba424 == 0x0)) {
    _DAT_005ba424 = 0x1;
    iVar6 = 0x3;
    loop {
      iVar5 = *(param_1 + 0x10);
      if (iVar5 < *(param_1 + 0x14)) {
        iVar5 = iVar5 + *(param_1 + 0x8);
      }
      if (((*(param_1 + 0x38) == 0x0) || (iVar5 == *(param_1 + 0x14))) ||
         (*(param_1 + 0x14) + *(param_1 + 0x48) <= iVar5)) break;
      *(param_1 + 0x44) =
           *(param_1 + 0x44) + (((*(param_1 + 0x48) * 0x3) / 0x2 + *(param_1 + 0x14)) - iVar5);
      *(param_1 + 0xc) = *(param_1 + 0xc) + -0x2;
      FUN_004afc30(param_1,0x0);
      iVar6 = iVar6 + -0x1;
    } while (iVar6 != 0x0);
    _DAT_005ba424 = 0x0;
  }
  bVar1 = *(param_1 + 0x1c);
  if ((((bVar1 & 0x4) != 0x0) && (*(param_1 + 0x38) == 0x0)) && ((bVar1 & 0x8) == 0x0)) {
    if ((bVar1 & 0x1) == 0x0) {
      iVar6 = 0x0;
    }
    else {
      iVar6 = 0x4;
    }
    set_file_pointer_004af420(*(param_1 + 0x4),iVar6,0x0);
    if ((*(param_1 + 0x1c) & 0x1) == 0x0) {
      *(param_1 + 0x38) = *(param_1 + 0x34);
      return;
    }
    *(param_1 + 0x38) = *(param_1 + 0x34) + -0x4;
  }
  return;
}



fn FUN_004aff90(param_1: i32)

{
  let mut iVar1: i32;
  let DVar2: u32;
  
  if ((*(param_1 + 0x1c) & 0x1) == 0x0) {
    if ((*(param_1 + 0x1c) & 0x8) != 0x0) {
      *(param_1 + 0x48) = 0x0;
      *(param_1 + 0x44) = *(param_1 + 0x48);
      DVar2 = FUN_004a91d0(*(param_1 + 0x4));
      *(param_1 + 0x14) = 0x0;
      *(DWORD *)(param_1 + 0x34) = DVar2;
      *(param_1 + 0x2c) = *(param_1 + 0x14);
      *(param_1 + 0x30) = *(param_1 + 0x14);
      return;
    }
    iVar1 = *(param_1 + 0x10) - *(param_1 + 0x14);
    if (iVar1 < 0x1) {
      iVar1 = iVar1 + *(param_1 + 0x8);
    }
    iVar1 = (*(param_1 + 0x38) + iVar1) - *(param_1 + 0x34);
    if (iVar1 != 0x0) {
      FUN_004b00f0(param_1,iVar1,0x0);
      FUN_004b0070(param_1,iVar1,0x0);
      return;
    }
  }
  else {
    if (*(param_1 + 0x40) != 0x0) {
      FUN_004b00f0(param_1,*(param_1 + 0x40),0x0);
      FUN_004b0070(param_1,*(param_1 + 0x40),0x0);
      return;
    }
  }
  return;
}



fn FUN_004b0070(param_1: i32,param_2: i32,param_3: u32)

{
  let mut iVar1: i32;
  
  if (((*(param_1 + 0x1c) & 0x1) == 0x0) || ((param_3 & 0x2) != 0x0)) {
    iVar1 = *(param_1 + 0x14) + param_2;
    *(param_1 + 0x14) = iVar1;
    if (*(param_1 + 0x8) <= iVar1) {
      *(param_1 + 0x14) = iVar1 - *(param_1 + 0x8);
    }
  }
  else {
    *(param_1 + 0x30) = *(param_1 + 0x30) + param_2;
    iVar1 = *(param_1 + 0x40) - param_2;
    *(param_1 + 0x40) = iVar1;
    if (((*(param_1 + 0x1c) & 0x4) != 0x0) && (iVar1 < 0x0)) {
      *(param_1 + 0x40) = iVar1 + *(param_1 + 0x3c);
      if ((param_3 & 0x1) != 0x0) {
        return;
      }^
      // goto LAB_004b00da;
    }
  }
  if ((param_3 & 0x1) != 0x0) {
    return;
  }
// LAB_004b00da:
  FUN_004afc30(param_1,0x0);
  return;
}



fn FUN_004b00f0(param_1: i32,param_2: i32,param_3: u32) -> i32

{
  let mut iVar1: i32;
  let puVar2: *mut u32;
  let mut iVar3: i32;
  ushort *puVar4;
  let mut uVar5: u32;
  let bVar6: u8;
  let cVar8: u8;
  let mut iVar7: i32;
  let mut uVar9: u32;
  ushort *puVar10;
  ushort *puVar11;
  let mut puVar12: *mut u8; 
  let mut puVar13: *mut u8; 
  let mut puVar14: *mut u8; 
  let puVar15: *mut u32;
  let mut local_28: i32;
  
  if ((param_3 & 0x1) == 0x0) {
    FUN_004afc30(param_1,0x0);
  }
  if (((*(param_1 + 0x1c) & 0x1) == 0x0) || ((param_3 & 0x2) != 0x0)) {
    if ((*(param_1 + 0x38) != 0x0) || ((*(param_1 + 0x1c) & 0x8) == 0x0)) {
      iVar3 = *(param_1 + 0x10);
      if (iVar3 < *(param_1 + 0x14)) {
        iVar3 = iVar3 + *(param_1 + 0x8);
      }
      while ((iVar3 != *(param_1 + 0x14) &&
             ((iVar7 = *(param_1 + 0x14) + param_2, iVar3 < iVar7 ||
              ((iVar7 == iVar3 && (((*(param_1 + 0x1c) & 0x4) != 0x0 || (*(param_1 + 0x38) != 0x0))))))))
            ) {
        if ((param_3 & 0x1) != 0x0) {
          return 0x0;
        }
        *(param_1 + 0xc) = *(param_1 + 0xc) + -0x1;
        if (((*(param_1 + 0x1c) & 0x4) == 0x0) &&
           (iVar3 = (*(param_1 + 0x14) + param_2) - iVar3, *(param_1 + 0x38) < iVar3)) {
          *(param_1 + 0x38) = iVar3;
        }
        FUN_004afc30(param_1,0x1);
        iVar3 = *(param_1 + 0x10);
        if (iVar3 < *(param_1 + 0x14)) {
          iVar3 = iVar3 + *(param_1 + 0x8);
        }
      }
    }
    if (*(param_1 + 0x8) < *(param_1 + 0x14) + param_2) {
      puVar2 = (param_1 + 0x20);
      puVar15 = (*(param_1 + 0x8) + puVar2);
      *puVar15 = *puVar2;
      (puVar15 + 0x1) = (puVar2 + 0x1);
    }
    return *(param_1 + 0x20) + *(param_1 + 0x14);
  }
  if (*(param_1 + 0x28) <= *(param_1 + 0x30) + param_2 + 0x100) {
    puVar2 = (param_1 + 0x24);
    iVar3 = *(param_1 + 0x30) + puVar2;
    *puVar2 = *(iVar3 + -0x2000);
    (puVar2 + 0x1) = (iVar3 + -0x1ffc);
    iVar3 = *(param_1 + 0x30);
    *(param_1 + 0x30) = 0x2000;
    *(param_1 + 0x2c) = (*(param_1 + 0x2c) - iVar3) + 0x2000;
  }
  local_28 = *(param_1 + 0x2c);
  iVar3 = *(param_1 + 0x24);
  iVar7 = *(param_1 + 0x24);
  iVar1 = *(param_1 + 0x28);
  loop {
    if (param_2 + *(param_1 + 0x30) <= local_28) {
      return *(param_1 + 0x24) + *(param_1 + 0x30);
    }
    puVar4 = FUN_004b00f0(param_1,0x811,param_3 | 0x2);
    if (puVar4 == 0x0) {
      return 0x0;
    }
    puVar12 = (iVar3 + local_28);
    puVar11 = puVar4;
    loop {
      uVar9 = CONCAT11(0x8,puVar11);
      puVar10 = (puVar11 + 0x1);
      puVar13 = puVar12;
      loop {
        while( true ) {
          bVar6 = (byte)uVar9 >> 0x1;
          cVar8 = (char)(uVar9 >> 0x8);
          if (!(bool)((byte)uVar9 & 0x1)) break;
          puVar12 = puVar13 + 0x1;
          puVar11 = (puVar10 + 0x1);
          *puVar13 = puVar10;
          cVar8 = cVar8 + -0x1;
          uVar9 = CONCAT11(cVar8,bVar6);
          puVar10 = puVar11;
          puVar13 = puVar12;
          if (cVar8 == '\0')^ // goto LAB_004b02c5;
        }
        puVar11 = puVar10 + 0x1;
        uVar9 = (*puVar10 >> 0x3);
        if (uVar9 == 0x0)^ // goto LAB_004b02d5;
        puVar12 = puVar13 + -uVar9;
        puVar14 = puVar13;
        for (uVar5 = *puVar10 & 0xffff0007; uVar5 != 0x0; uVar5 = uVar5 - 0x1) {
          *puVar14 = *puVar12;
          puVar12 = puVar12 + 0x1;
          puVar14 = puVar14 + 0x1;
        }
        *puVar14 = *puVar12;
        puVar13 = puVar14 + 0x2;
        puVar14[0x1] = puVar12[0x1];
        cVar8 = cVar8 + -0x1;
        uVar9 = CONCAT11(cVar8,bVar6);
        puVar10 = puVar11;
        puVar12 = puVar13;
      } while (cVar8 != '\0');
// LAB_004b02c5:
      puVar13 = puVar12;
    } while ((puVar11 < puVar4 + 0x400) && (puVar12 < (iVar7 + iVar1 + -0xc8)));
// LAB_004b02d5:
    local_28 = puVar13 - iVar3;
    FUN_004b0070(param_1,puVar11 - puVar4,param_3 | 0x2);
    *(param_1 + 0x2c) = local_28;
  } while( true );
}



fn FUN_004b03a0(param_1: i32,param_2: i32)

{
  *(param_1 + 0x44) = *(param_1 + 0x44) + param_2;
  *(param_1 + 0xc) = *(param_1 + 0xc) + -0x2;
  FUN_004afc30(param_1,0x0);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

i32 **  FUN_004b0530(param_1: &mut String,param_2: *mut i32,param_3: i32,param_4: *mut i32)

{
  let mut iVar1: i32;
  i32 **ppiVar2;
  let piVar3: *mut i32;;
  i32 **ppiVar4;
  let piVar5: *mut i32;;
  let mut uVar6: u32;
  let mut read_buffer: String;;
  let DVar7: u32;
  let DVar8: u32;
  let mut iVar9: i32;
  
  if (_DAT_005ba430 == 0x0) {
    FUN_004ae7e3(&LAB_004b0040);
  }
  _DAT_005ba430 = 0x1;
  piVar3 = FUN_0049c993(param_1,0x200);
  if (piVar3 == 0xffffffff) {
    return (i32 **)0x0;
  }
  ppiVar4 = (i32 **)FUN_004a2831(0x4c);
  if (ppiVar4 == (i32 **)0x0) {
    return (i32 **)0x0;
  }
  if (DAT_005ba428 == 0x0) {
    uVar6 = FUN_0049ca0b(s_INTRO_C13_004c36dc,0x200);
    if (uVar6 == 0xffffffff) {
      DAT_005ba428 = 0x5dc;
    }
    else {
      read_buffer = FUN_0049c2c9(0x2000);
      if (read_buffer == 0x0) {
        FUN_004ab390(uVar6);
        DAT_005ba428 = 0x5dc;
      }
      else {
        DVar7 = timeGetTime();
        read_file_func_004ab180(uVar6,read_buffer,0xa9f);
        loop {
          read_file_func_004ab180(uVar6,read_buffer,0x2000);
          DVar8 = timeGetTime();
          iVar1 = 0x0;
        } while (DVar8 - DVar7 < 0x64);
        loop {
          iVar9 = iVar1;
          read_file_func_004ab180(uVar6,read_buffer,0x2000);
          DVar8 = timeGetTime();
          iVar1 = iVar9 + 0x2000;
        } while (DVar8 - DVar7 < 0xc8);
        FUN_0049af50(read_buffer);
        DAT_005ba428 = (((iVar9 + 0x144f8) / 0x249f0) * 0x5dc);
        if (DAT_005ba428 < 0x5dc) {
          DAT_005ba428 = 0x5dc;
        }
        else {
          if (0x2328 < DAT_005ba428) {
            DAT_005ba428 = 0x2328;
            FUN_004ab390(uVar6);^
            // goto LAB_004b05a1;
          }
        }
        FUN_004ab390(uVar6);
      }
    }
  }
// LAB_004b05a1:
  piVar5 = DAT_005ba428;
  ppiVar4[0x11] = 0x0;
  ppiVar4[0x12] = 0x0;
  ppiVar4[0x6] = piVar5;
  piVar5 = FUN_004a91d0(piVar3);
  ppiVar4[0xd] = piVar5;
  if (ppiVar4[0xd] < ppiVar4[0x6]) {
    ppiVar4[0x6] = ppiVar4[0xd];
  }
  ppiVar4[0xa] = 0x0;
  ppiVar4[0xc] = 0x0;
  ppiVar4[0x1] = piVar3;
  ppiVar4[0x7] = param_4;
  ppiVar4[0xb] = 0x0;
  ppiVar4[0xe] = (ppiVar4[0xd] - ppiVar4[0x6]);
  ppiVar4[0x5] = 0x0;
  if ((param_4 & 0x1) == 0x0) {
    ppiVar4[0xf] = ppiVar4[0xd];
    ppiVar4[0x9] = 0x0;
    if (ppiVar4[0xd] < param_2) {
      *(ppiVar4 + 0x7) = *(ppiVar4 + 0x7) | 0x8;
      param_2 = ppiVar4[0xd];
    }
    uVar6 = param_2 + param_3;
  }
  else {
    ppiVar4[0xa] = (param_3 + 0x2600);
    piVar3 = FUN_0049c2c9((param_3 + 0x2600));
    ppiVar4[0x9] = piVar3;
    if (piVar3 == 0x0)^ // goto LAB_004b0822;
    read_file_func_004ab180(ppiVar4[0x1],(ppiVar4 + 0xf),0x4);
    uVar6 = param_2 + 0xc11;
    ppiVar4[0xe] = ppiVar4[0xe] + -0x1;
  }
  piVar3 = FUN_0049c2c9(uVar6);
  ppiVar4[0x8] = piVar3;
  if (piVar3 != 0x0) {
    read_file_func_004ab180(ppiVar4[0x1],piVar3,(DWORD)ppiVar4[0x6]);
    ppiVar4[0x2] = param_2;
    ppiVar4[0x10] = ppiVar4[0xf];
    ppiVar4[0x4] = ppiVar4[0x6];
    piVar3 = timeGetTime();
    ppiVar4[0x3] = piVar3;
    ppiVar2 = ppiVar4;
    *ppiVar4 = DAT_005ba42c;
    DAT_005ba42c = ppiVar2;
    return ppiVar4;
  }
// LAB_004b0822:
  ppiVar4 = FUN_004b0840(ppiVar4);
  FUN_0049af50(ppiVar4);
  return (i32 **)0x0;
}



i32 **  FUN_004b0840(i32 **param_1)

{
  i32 **ppiVar1;
  i32 **ppiVar2;
  
  if (param_1[0x8] != 0x0) {
    FUN_0049af50(param_1[0x8]);
  }
  if (param_1[0x9] != 0x0) {
    FUN_0049af50(param_1[0x9]);
  }
  if (-0x1 < param_1[0x1]) {
    FUN_004ab390(param_1[0x1]);
  }
  ppiVar1 = DAT_005ba42c;
  if (DAT_005ba42c == param_1) {
    DAT_005ba42c = (i32 **)*param_1;
  }
  else {
    loop {
      ppiVar2 = ppiVar1;
      if (ppiVar2 == (i32 **)0x0) break;
      ppiVar1 = (i32 **)*ppiVar2;
    } while (param_1 != (i32 **)*ppiVar2);
    if (ppiVar2 != (i32 **)0x0) {
      *ppiVar2 = *param_1;
      return param_1;
    }
  }
  return param_1;
}



fn FUN_004b08b0() -> u32

{
  let piVar1: *mut i32;;
  let piVar2: *mut i32;;
  
  (PTR_FUN_004bfb90)();
  piVar1 = DAT_004bfae8;
  while (piVar2 = piVar1, piVar2 != 0x0) {
    piVar1 = piVar2[0x2];
    if (*piVar2 + -0x2c == *piVar2[0x9]) {
      FUN_004b094c(piVar2);
    }
  }
  (PTR_FUN_004bfb98)();
  return 0x0;
}



fn free_mem_004b08ec(LPVOID param_1) -> u32

{
  LPVOID pvVar1;
  let mut BVar2: bool;
  
                    // DWORD dwFreeType for VirtualFree
                    // SIZE_T dwSize for VirtualFree
                    // LPVOID lpAddress for VirtualFree
  pvVar1 = *(LPVOID *)(param_1 + 0x8);
  BVar2 = VirtualFree(param_1,0x0,0x8000);
  if (BVar2 != 0x0) {
    if ((param_1 == DAT_004bfaec) && (DAT_004bfaec = pvVar1, pvVar1 == (LPVOID)0x0)) {
      DAT_004bfaec = DAT_004bfae8;
      DAT_004bfaf0 = pvVar1;
    }
    if (param_1 == DAT_005b9d10) {
      DAT_005b9d10 = (LPVOID)(param_1 ^ DAT_005b9d10);
    }
    return 0x0;
  }
  return 0xffffffff;
}



fn FUN_004b094c(LPVOID param_1)

{
  let mut iVar1: i32;
  let mut iVar2: i32;
  let mut iVar3: i32;
  let DVar4: u32;
  
  iVar1 = *(param_1 + 0x4);
  iVar2 = *(param_1 + 0x8);
  DVar4 = free_mem_004b08ec(param_1);
  if (DVar4 == 0x0) {
    iVar3 = iVar2;
    if (iVar1 != 0x0) {
      *(iVar1 + 0x8) = iVar2;
      iVar3 = DAT_004bfae8;
    }
    DAT_004bfae8 = iVar3;
    if (iVar2 != 0x0) {
      *(iVar2 + 0x4) = iVar1;
    }
  }
  return;
}



// WARNING: Could not reconcile some variable overlaps

fn FUN_004b097e(param_1: i32,param_2: u32,param_3: i32) -> u32

{
  let mut uVar1: u32;
  let mut iVar2: i32;
  let mut bVar3: bool;
  let mut local_3c: u32;
  let mut local_34: i32;
  let mut local_20: i32;
  let mut local_18: u32;
  
  local_18 = param_2 & 0xff;
  bVar3 = (param_2 & 0x1000000) != 0x0;
  uVar1 = param_2 & 0x7f0000;
  param_2 = uVar1 >> 0x10;
  if ((param_3 == 0x105) || (param_3 == 0x101)) {
    if (param_2 < 0x2a) {
      if (param_2 == 0x1d) {
        if (bVar3) {
          *(param_1 + 0xd) = *(param_1 + 0xd) & 0xfd;
        }
        else {
          *(param_1 + 0xd) = *(param_1 + 0xd) & 0xef;
        }
      }
    }
    else {
      if (param_2 < 0x2b) {
        *(param_1 + 0xd) = *(param_1 + 0xd) & 0xf7;
      }
      else {
        if (0x35 < param_2) {
          if (param_2 < 0x37) {
            *(param_1 + 0xd) = *(param_1 + 0xd) & 0xfe;
          }
          else {
            if (param_2 == 0x38) {
              if (bVar3) {
                *(param_1 + 0xd) = *(param_1 + 0xd) & 0xfb;
              }
              else {
                *(param_1 + 0xd) = *(param_1 + 0xd) & 0xdf;
              }
            }
          }
        }
      }
    }
  }
  if (param_2 < 0x59) {
    if ((*(param_1 + 0xc) & 0x900) == 0x0) {
      local_20 = *(&DAT_004bfeed + param_2);
    }
    else {
      local_20 = *(&DAT_004bff4d + param_2);
    }
    if (((*(param_1 + 0xd) & 0x40) != 0x0) &&
       (local_20._0_1_ = (char)(local_20 >> 0x18),
       (*(&DAT_004bf9c1 + (byte)((char)local_20 + 0x1)) >> 0x18 & 0xc0U) != 0x0)) {
      local_20 = *(&DAT_004bff4d + param_2);
    }
    local_20 = local_20 >> 0x18;
  }
  else {
    local_20 = 0x0;
  }
  if ((local_20 == 0x0) && ((param_3 == 0x104 || (param_3 == 0x100)))) {
    if (param_2 < 0x36) {
      if (0x1c < param_2) {
        if (param_2 < 0x1e) {
          local_18 = 0x1;
          if (bVar3) {
            if ((*(param_1 + 0xd) & 0x2) != 0x0) {
              return 0x1;
            }
            *(param_1 + 0xd) = *(param_1 + 0xd) | 0x2;
          }
          else {
            if ((*(param_1 + 0xd) & 0x10) != 0x0) {
              return 0x1;
            }
            *(param_1 + 0xd) = *(param_1 + 0xd) | 0x10;
          }
        }
        else {
          if (param_2 == 0x2a) {
            if (((*(param_1 + 0xd) & 0x8) != 0x0) || (bVar3)) {
              return 0x1;
            }
            local_18 = 0x1;
            *(param_1 + 0xd) = *(param_1 + 0xd) | 0x8;
          }
        }
      }
    }
    else {
      if (param_2 < 0x37) {
        if (((*(param_1 + 0xd) & 0x1) != 0x0) || (bVar3)) {
          return 0x1;
        }
        local_18 = 0x1;
        *(param_1 + 0xd) = *(param_1 + 0xd) | 0x1;
      }
      else {
        if (param_2 < 0x3a) {
          if (param_2 == 0x38) {
            local_18 = 0x1;
            if (bVar3) {
              if ((*(param_1 + 0xd) & 0x4) != 0x0) {
                return 0x1;
              }
              *(param_1 + 0xd) = *(param_1 + 0xd) | 0x4;
            }
            else {
              if ((*(param_1 + 0xd) & 0x20) != 0x0) {
                return 0x1;
              }
              *(param_1 + 0xd) = *(param_1 + 0xd) | 0x20;
            }
          }
        }
        else {
          if (param_2 < 0x3b) {
            if ((*(param_1 + 0xd) & 0x40) == 0x0) {
              *(param_1 + 0xd) = *(param_1 + 0xd) | 0x40;
            }
            else {
              *(param_1 + 0xd) = *(param_1 + 0xd) & 0xbf;
            }
            return 0x0;
          }
          if (param_2 == 0x45) {
            if ((*(param_1 + 0xd) & 0x80) == 0x0) {
              *(param_1 + 0xd) = *(param_1 + 0xd) | 0x80;
            }
            else {
              *(param_1 + 0xd) = *(param_1 + 0xd) & 0x7f;
            }
            return 0x0;
          }
        }
      }
    }
  }
  if ((0x46 < param_2) &&
     ((bVar3 || (((*(param_1 + 0xd) & 0x80) == 0x0 &&
                 ((local_20 == 0x2e || (((&DAT_004bf9c4)[(byte)((char)local_20 + 0x1)] & 0x20) != 0x0)))))))) {
    local_20 = 0x0;
  }
  if (param_3 == 0x105) {
    param_3 = 0x101;
  }
  if (param_3 == 0x104) {
    param_3 = 0x100;
  }
  if (DAT_005ba440 == 0x0) {
    if (param_2 == 0x38) {
      if (param_3 == 0x100) {
        if (bVar3) {
          local_3c = 0x80;
        }
        else {
          local_3c = 0x0;
        }
        DAT_005ba440 = local_3c | 0x1;
      }
      return 0xfffffffe;
    }
  }
  else {
    for (local_34 = 0x0; local_34 < 0x4; local_34 = local_34 + 0x1) {
      if (*(&DAT_004bffac + local_34 * 0x4) == param_2) {
        DAT_005ba440 = 0x0;
        return 0xffffffff;
      }
    }
    if ((DAT_005ba440 & 0x7f) < 0x2) {
      iVar2 = FUN_0049f713(param_1);
      if (-0x1 < iVar2) {
        *(iVar2 * 0xe + param_1 + 0x18) = 0x0;
        *(iVar2 * 0xe + param_1 + 0x1c) = 0x100;
        *(iVar2 * 0xe + param_1 + 0x1e) = DAT_005ba440 & 0x80 | *(param_1 + 0xc) | 0x38;
        *(iVar2 * 0xe + param_1 + 0x22) = 0x0;
      }
      DAT_005ba440 = DAT_005ba440 + 0x1;
    }
    if ((param_2 == 0x38) && (param_3 == 0x101)) {
      DAT_005ba440 = 0x0;
    }
  }
  if (bVar3) {
    param_2 = uVar1 >> 0x10 | 0x80;
  }
  uVar1 = *(param_1 + 0xc);
  while ((local_18 != 0x0 && (iVar2 = FUN_0049f713(param_1), -0x1 < iVar2))) {
    *(iVar2 * 0xe + param_1 + 0x18) = 0x0;
    (iVar2 * 0xe + param_1 + 0x1c) = param_3;
    *(iVar2 * 0xe + param_1 + 0x1e) = param_2 | uVar1;
    *(iVar2 * 0xe + param_1 + 0x22) = local_20;
    local_18 = local_18 - 0x1;
  }
  return 0x1;
}



void  check_key_state_004b0e63(DWORD *param_1)

{
  let key_state: i16;
  let mut local_18: u32;
  let mut local_14: u32;
  
  param_1[0x3] = 0x0;
                    // i32 nVirtKey for GetKeyState
  key_state = GetKeyState(VK_CAPITAL);
  if (key_state == 0x0) {
    local_18 = 0x0;
  }
  else {
    local_18 = 0x4000;
  }
  param_1[0x3] = param_1[0x3] | local_18;
                    // i32 nVirtKey for GetKeyState
  key_state = GetKeyState(VK_NUMLOCK);
  if (key_state == 0x0) {
    local_14 = 0x0;
  }
  else {
    local_14 = 0x8000;
  }
  param_1[0x3] = param_1[0x3] | local_14;
  return;
}



fn FUN_004b0ef0()

{
  (PTR_FUN_004bffbc)();
  FUN_004b67a0(s_ABNORMAL_TERMINATION_004c36e8,0xff);
  return;
}



fn FUN_004b0ef8()

{
  FUN_004b67a0(s_ABNORMAL_TERMINATION_004c36e8,0xff);
  return;
}



void  FUN_004b0f10(param_1: u32,param_2: u32)

{
  let mut in_EAX: *mut u8; 
  let mut puVar1: *mut u8; 
  let mut uVar2: u32;
  let uVar3: u8;
  undefined6 uVar4;
  
  if (param_1 != 0x0) {
    loop {
      if ((in_EAX & 0x3) == 0x0) break;
      *in_EAX = (char)param_2;
      in_EAX = in_EAX + 0x1;
      param_2 = param_2 >> 0x8 | param_2 << 0x18;
      param_1 = param_1 - 0x1;
    } while (param_1 != 0x0);
    uVar4 = FUN_004b0f47(param_1 >> 0x2,param_2);
    puVar1 = uVar4;
    uVar2 = param_1 & 0x3;
    if (uVar2 != 0x0) {
      uVar3 = ((uint6)uVar4 >> 0x20);
      *puVar1 = uVar3;
      if ((uVar2 != 0x1) && (puVar1[0x1] = (char)((uint6)uVar4 >> 0x28), uVar2 != 0x2)) {
        puVar1[0x2] = uVar3;
      }
    }
  }
  return;
}



void  FUN_004b0f47(param_1: u32,param_2: u32)

{
  let in_EAX: *mut u32;
  let puVar1: *mut u32;
  let mut iVar2: i32;
  let mut uVar3: u32;
  
  if (param_1 != 0x0) {
    loop {
      if ((in_EAX & 0x1f) == 0x0) break;
      *in_EAX = param_2;
      in_EAX = in_EAX + 0x1;
      param_1 = param_1 - 0x1;
    } while (param_1 != 0x0);
    if (param_1 >> 0x2 != 0x0) {
      iVar2 = (param_1 >> 0x2) - 0x1;
      if (iVar2 != 0x0) {
        loop {
          puVar1 = in_EAX;
          *puVar1 = param_2;
          puVar1[0x1] = param_2;
          puVar1[0x2] = param_2;
          puVar1[0x3] = param_2;
          if (iVar2 == 0x1)^ // goto LAB_004b0f86;
          puVar1[0x4] = param_2;
          puVar1[0x5] = param_2;
          iVar2 = iVar2 + -0x2;
          puVar1[0x6] = param_2;
          puVar1[0x7] = param_2;
          in_EAX = puVar1 + 0x8;
        } while (iVar2 != 0x0);
        puVar1 = puVar1 + 0x4;
// LAB_004b0f86:
        in_EAX = puVar1 + 0x4;
      }
      *in_EAX = param_2;
      in_EAX[0x1] = param_2;
      in_EAX[0x2] = param_2;
      in_EAX[0x3] = param_2;
      in_EAX = in_EAX + 0x4;
    }
    uVar3 = param_1 & 0x3;
    if (uVar3 != 0x0) {
      *in_EAX = param_2;
      if (uVar3 != 0x1) {
        in_EAX[0x1] = param_2;
        if (uVar3 != 0x2) {
          in_EAX[0x2] = param_2;
        }
      }
    }
  }
  return;
}



fn FUN_004b0fb3(param_1: &mut String,UINT param_2)

{
  let mut iVar1: i32;
  let in_DS: u16;
  
  iVar1 = FUN_004b67d0(param_1,in_DS);
  if (iVar1 != 0x0) {
    FUN_004a9a64(param_2);
    return;
  }
  write_to_file_004b6760(param_1,param_2);
  return;
}



fn FUN_004b0ff3(param_1: u32) -> u32

{
  let mut in_EAX: u32;
  
  FUN_004b1003();
  return in_EAX;
}



fn FUN_004b1003() -> u32

{
  let mut bVar1: bool;
  let mut in_EAX: *mut u8; 
  let mut iVar2: i32;
  let puVar3: *mut u32;
  let mut uVar4: u32;
  let mut iVar5: i32;
  let mut iVar6: i32;
  
  if (in_EAX < &stack0xfffffffc) {
    iVar2 = in_EAX - &stack0xfffffffc;
    puVar3 = (PTR_FUN_004bfb74)();
    if (*puVar3 <= -iVar2 && -iVar2 != *puVar3) {
      uVar4 = FUN_004b102f(in_EAX);
      return uVar4;
    }
  }
  iVar2 = 0x1;
  uVar4 = FUN_004b67a0(s_Stack_Overflow__004bffc0,0x1);
  iVar6 = -0x4;
  loop {
    *(&stack0xfffffff0 + iVar6) = iVar6;
    iVar6 = iVar6 + -0x1000;
    iVar5 = iVar2 + -0x1000;
    bVar1 = 0xfff < iVar2;
    iVar2 = iVar5;
  } while (iVar5 != 0x0 && bVar1);
  return uVar4;
}



fn FUN_004b102f(param_1: i32) -> u32

{
  let mut bVar1: bool;
  let mut in_EAX: u32;
  let mut iVar2: i32;
  let mut iVar3: i32;
  
  iVar3 = -0x4;
  loop {
    *(&stack0xfffffff8 + iVar3) = iVar3;
    iVar3 = iVar3 + -0x1000;
    iVar2 = param_1 + -0x1000;
    bVar1 = 0xfff < param_1;
    param_1 = iVar2;
  } while (iVar2 != 0x0 && bVar1);
  return in_EAX;
}



fn FUN_004b1158(param_1: &mut String,param_2: i32,char **param_3) -> i32

{
  let mut pcVar1: String; 
  let cVar3: u8;
  let cVar4: u8;
  let mut uVar2: u32;
  let mut pcVar5: String; 
  let mut iVar6: i32;
  
  iVar6 = 0x0;
// LAB_004b116f:
  loop {
    for (; (cVar4 = *param_1, cVar4 == ' ' || (cVar4 == '\t')); param_1 = param_1 + 0x1) {
    }
    if (cVar4 == '\0')^ // goto LAB_004b1242;
    pcVar1 = param_1;
    if (*param_1 == '\"') {
      pcVar1 = param_1 + 0x1;
    }
    uVar2 = (*param_1 == '\"');
    param_1 = pcVar1;
    pcVar5 = pcVar1;
// LAB_004b1197:
    cVar4 = (char)uVar2;
    if (*param_1 != '\"') {
// LAB_004b11b8:
      if (((*param_1 == ' ') || (*param_1 == '\t')) && (cVar4 == '\0'))^ // goto LAB_004b1217;
      uVar2 = CONCAT11(*param_1,cVar4);
      cVar3 = (char)(uVar2 >> 0x8);
      if (cVar3 == '\0')^ // goto LAB_004b1217;
      if (cVar3 == '\\') {
        if (DAT_005bac28 == 0x0) {
          if (param_1[0x1] == '\"') {
            uVar2 = CONCAT11(param_1[-0x1],cVar4);
            param_1 = param_1 + 0x1;
            if ((char)(uVar2 >> 0x8) == '\\')^ // goto LAB_004b1197;
          }
        }
        else {
          if (cVar4 == '\x01') {
            uVar2 = CONCAT11(param_1[0x1],0x1);
            cVar4 = (char)(uVar2 >> 0x8);
            if ((cVar4 == '\"') || (cVar4 == '\\')) {
// LAB_004b1206:
              param_1 = param_1 + 0x1;
            }
          }
          else {
            if (param_1[0x1] == '\"')^ // goto LAB_004b1206;
          }
        }
      }
      if (param_2 == 0x0) {
        param_1 = param_1 + 0x1;
      }
      else {
        uVar2 = CONCAT11(*param_1,(char)uVar2);
        param_1 = param_1 + 0x1;
        *pcVar5 = (char)(uVar2 >> 0x8);
        pcVar5 = pcVar5 + 0x1;
      }^
      // goto LAB_004b1197;
    }
    if (DAT_005bac28 == 0x0) {
      param_1 = param_1 + 0x1;
      if (cVar4 == '\0') {
        uVar2 = 0x2;
      }
      else {
        if (cVar4 == '\0')^ // goto LAB_004b11b8;
        uVar2 = 0x0;
      }^
      // goto LAB_004b1197;
    }
    if (cVar4 != '\x01')^ // goto LAB_004b11b8;
// LAB_004b1217:
    if (param_2 != 0x0) {
      *(param_2 + iVar6 * 0x4) = pcVar1;
      iVar6 = iVar6 + 0x1;
      if (*param_1 == '\0') {
        *pcVar5 = '\0';
// LAB_004b1242:
        *param_3 = param_1;
        return iVar6;
      }
      param_1 = param_1 + 0x1;
      *pcVar5 = '\0';^
      // goto LAB_004b116f;
    }
    iVar6 = iVar6 + 0x1;
    if (*param_1 == '\0')^ // goto LAB_004b1242;
    param_1 = param_1 + 0x1;
  } while( true );
}



fn FUN_004b1278(param_1: u32,param_2: i32)

{
  if (param_2 != 0x0) {
    FUN_004b1290(param_1);
  }
  return;
}



fn FUN_004b1290(param_1: u32) -> u32

{
  let mut iVar1: i32;
  
  FUN_004b1778(param_1);
  if (param_1 == 0x7b) {
    iVar1 = 0x1;
  }
  else {
    if (param_1 == 0xce) {
      FUN_004b1740(0x9);
      return 0xffffffff;
    }
    if (param_1 == 0xb7) {
      FUN_004b1740(0x7);
      return 0xffffffff;
    }
    if (0x13 < param_1) {
      param_1 = 0x13;
    }
    iVar1 = *(s_Stack_Overflow__004bffc0 + param_1 + 0x11) >> 0x18;
  }
  FUN_004b1740(iVar1);
  return 0xffffffff;
}



void handle_err_fn_004b12fc(void)

{
  let err_code: u32;
  
  err_code = GetLastError();
  FUN_004b1290(err_code);
  return;
}



void  file_time_fn_004b1310(FILETIME *file_time_to_convert,LPWORD fat_date,LPWORD fat_time)

{
  _FILETIME file_time;
  
                    // LPFILETIME lpLocalFileTime for FileTimeToLocalFileTime
                    // FILETIME * lpFileTime for FileTimeToLocalFileTime
  FileTimeToLocalFileTime(file_time_to_convert,(LPFILETIME)&file_time);
                    // LPWORD lpFatTime for FileTimeToDosDateTime
                    // LPWORD lpFatDate for FileTimeToDosDateTime
                    // FILETIME * lpFileTime for FileTimeToDosDateTime
  FileTimeToDosDateTime((FILETIME *)&file_time,fat_date,fat_time);
  return;
}



fn FUN_004b1374(param_1: i32,param_2: *mut u8)

{
  file_time_fn_004b1310((FILETIME *)(param_2 + 0x14),(LPWORD)(param_1 + 0x18),(LPWORD)(param_1 + 0x16));
  (param_1 + 0x15) = *param_2;
  *(param_1 + 0x1a) = *(param_2 + 0x20);
  FUN_004a9a00((param_1 + 0x1e),param_2 + 0x2c,0xff);
  (param_1 + 0x11d) = 0x0;
  return;
}



fn find_file_fn_004b13bc(HANDLE find_file_handle,param_2: u32,LPWIN32_FIND_DATAA find_file_data) -> u32

{
  let mut BVar1: bool;
  
  while( true ) {
    if (find_file_data->dwFileAttributes == 0x0) {
      find_file_data->dwFileAttributes = 0x80;
    }
    if ((find_file_data->dwFileAttributes & param_2) != 0x0) break;
                    // LPWIN32_FIND_DATAA lpFindFileData for FindNextFileA
                    // HANDLE hFindFile for FindNextFileA
    BVar1 = FindNextFileA(find_file_handle,find_file_data);
    if (BVar1 == 0x0) {
      return 0x0;
    }
  }
  return 0x1;
}



WORD  get_time_info_004b1400(u32 *time_info)

{
  _SYSTEMTIME sys_time;
  
                    // LPSYSTEMTIME lpSystemTime for GetLocalTime
  GetLocalTime((LPSYSTEMTIME)&sys_time);
  time_info[0x5] = sys_time.wYear - 0x76c;
  time_info[0x4] = sys_time.wMonth - 0x1;
  time_info[0x3] = sys_time.wDay;
  time_info[0x2] = sys_time.wHour;
  time_info[0x1] = sys_time.wMinute;
  time_info[0x8] = 0xffffffff;
  *time_info = sys_time.wSecond;
  return sys_time.wMilliseconds;
}



fn FUN_004b1470(param_1: *mut i32) -> i32

{
  let mut iVar1: i32;
  let mut iVar2: i32;
  let mut iVar3: i32;
  let mut uVar4: u32;
  let mut puVar5: *mut u8; 
  
  iVar3 = param_1[0x4] % 0xc;
  puVar5 = &DAT_004c41bc;
  if (param_1[0x5] < -0xb04815f) {
    iVar1 = -0x1;
  }
  else {
    iVar1 = param_1[0x5] + param_1[0x4] / 0xc;
    for (; iVar3 < 0x0; iVar3 = iVar3 + 0xc) {
      iVar1 = iVar1 + -0x1;
    }
    if (iVar1 < 0x0) {
      return -0x1;
    }
    iVar2 = FUN_004b6800(iVar1 + 0x76c);
    if (iVar2 != 0x0) {
      puVar5 = &DAT_004c41d6;
    }
    iVar2 = iVar1 + 0x3 >> 0x1f;
    iVar3 = (puVar5 + iVar3 * 0x2) +
            ((iVar1 * 0x16d + (((iVar1 + 0x3 + iVar2 * -0x4) - (iVar2 << 0x1 < 0x0)) >> 0x2)) -
            (iVar1 + 0x63) / 0x64) + (iVar1 + 0x12b) / 0x190 + param_1[0x3];
    for (uVar4 = *param_1 + (param_1[0x1] + param_1[0x2] * 0x3c) * 0x3c; iVar3 = iVar3 + -0x1, uVar4 < 0x0;
        uVar4 = uVar4 + 0x15180) {
    }
    FUN_004b6ca0(iVar3,uVar4,0x0,param_1);
    time_zone_fn_004b6f68();
    iVar1 = uVar4 + DAT_004c0328;
    if (param_1[0x8] < 0x0) {
      FUN_004b6994(param_1);
    }
    if (0x0 < param_1[0x8]) {
      iVar1 = iVar1 - DAT_004c0330;
    }
    for (; iVar1 < 0x0; iVar1 = iVar1 + 0x15180) {
      iVar3 = iVar3 + -0x1;
    }
    if (iVar3 < 0x63de) {
      return -0x1;
    }
    if (iVar3 == 0x63de) {
      iVar1 = iVar1 + -0x15180;
      if ((DAT_004c0328 < 0x1) || (iVar1 < 0x0)) {
        return -0x1;
      }
    }
    else {
      iVar1 = iVar1 + (iVar3 + -0x63df) * 0x15180;
    }
  }
  return iVar1;
}



fn delete_file_fn_004b1630(LPCSTR file_name) -> u32

{
  let err_code: u32;
  let mut file_delete: bool;
  
                    // LPCSTR lpFileName for DeleteFileA
  file_delete = DeleteFileA(file_name);
  if (file_delete != 0x0) {
    return 0x0;
  }
  err_code = GetLastError();
  err_code = FUN_004b1290(err_code);
  return err_code;
}



// WARNING: Unable to track spacebase fully for stack
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_004b1650()

{
  let mut iVar1: i32;
  let mut iVar2: i32;
  let mut puVar3: *mut u8; 
  HMODULE mod_handle;
  let dVar4: u32;
  let mut puVar5: *mut u8; 
  DWORD *puVar7;
  DWORD *pDVar6;
  let mut uStack32: u32;
  let mut uStack28: u32;
  let DStack24: u32;
  let piStack20: *mut i32;;
  let local_10: u8 [0x8];
  
  piStack20 = 0x4b1661;
  FUN_004b1cf0();
  iVar1 = -(DAT_004c0338 + 0x3U & 0xfffffffc);
  *(local_10 + iVar1 + -0x4) = DAT_004c0338;
  *(&DStack24 + iVar1) = 0x0;
  (&uStack28 + iVar1) = local_10 + iVar1;
  *(&uStack32 + iVar1) = 0x4b167e;
  FUN_004a0430(*(&uStack28 + iVar1),(&DStack24 + iVar1),
               *(local_10 + iVar1 + -0x4));
  (local_10 + iVar1 + -0x4) = local_10 + iVar1;
  *(&stack0x000000e0 + iVar1) = DAT_004c0338;
  (&DStack24 + iVar1) = local_10;
  *(&uStack28 + iVar1) = 0x4b1696;
  get_mod_handle_fn_004aeee8(*(DWORD *)(&DStack24 + iVar1),*(i32 **)(local_10 + iVar1 + -0x4));
  puVar7 = (DWORD *)(local_10 + iVar1);
  puVar5 = (DAT_004c0150 + 0x3U & 0xfffffffc);
  *(local_10 + iVar1 + -0x4) = 0x4b16aa;
  puVar3 = FUN_004b7330();
  if (puVar5 < puVar3) {
    (&piStack20 + iVar1) = puVar5;
    *(&DStack24 + iVar1) = 0x4b16b4;
    FUN_004b102f(*(&piStack20 + iVar1));
    iVar2 = -(DAT_004c0150 + 0x3U & 0xfffffffc);
    puVar7 = (DWORD *)(local_10 + iVar2 + iVar1);
    _DAT_004c0154 = local_10 + iVar2 + iVar1;
  }
  else {
    _DAT_004c0154 = 0x0;
  }
  _DAT_004c0154 = _DAT_004c0154 + DAT_004c0150;
  puVar7[-0x1] = 0x4b16d8;
  FUN_004b7340();
  puVar7[-0x1] = 0xa;
  puVar7[-0x2] = DAT_004c012c;
  puVar7[-0x3] = 0x0;
                    // LPCSTR lpModuleName for GetModuleHandleA
  puVar7[-0x4] = 0x0;
  pDVar6 = puVar7 + -0x5;
  puVar7[-0x5] = 0x4b16ec;
  mod_handle = GetModuleHandleA(puVar7[-0x4]);
  pDVar6[-0x1] = (DWORD)mod_handle;
  pDVar6[-0x2] = 0x4b16f2;
  dVar4 = important_func_0042d653((HINSTANCE)pDVar6[-0x1],*pDVar6,pDVar6[0x1],pDVar6[0x2]);
  pDVar6[0x2] = dVar4;
  pDVar6[0x1] = 0x4b16f8;
  FUN_004a9a64(pDVar6[0x2]);
  return;
}



fn FUN_004b1727(param_1: *mut u32)

{
  *param_1 = DAT_004c0340;
  DAT_004c0340 = param_1;
  return;
}



fn FUN_004b1740(param_1: u32)

{
  let mut iVar1: i32;
  
  iVar1 = (PTR_FUN_004bfb74)();
  *(iVar1 + 0x4) = param_1;
  return;
}



fn FUN_004b175c()

{
  FUN_004b1740(0xe);
  return;
}



fn FUN_004b1778(param_1: u32)

{
  let mut iVar1: i32;
  
  iVar1 = (PTR_FUN_004bfb74)();
  *(iVar1 + 0x8) = param_1;
  return;
}



fn FUN_004b1790(param_1: *mut u32)

{
  let mut uVar1: u32;
  let mut pCVar2: String;;
  let bVar3: u8;
  
  FUN_004b1b60(param_1);
  if (param_1[0x5] == 0x0) {
    if ((*(param_1 + 0xd) & 0x2) == 0x0) {
      if ((*(param_1 + 0xd) & 0x4) == 0x0) {
        param_1[0x5] = 0x1000;
      }
      else {
        param_1[0x5] = 0x1;
      }
    }
    else {
      param_1[0x5] = 0x86;
    }
  }
  pCVar2 = FUN_004aac00(param_1[0x5]);
  *(param_1[0x2] + 0x8) = pCVar2;
  if (*(param_1[0x2] + 0x8) == 0x0) {
    bVar3 = *(param_1 + 0xd) & 0xf8;
    *(param_1 + 0xd) = bVar3;
    *(param_1 + 0xd) = bVar3 | 0x4;
    (param_1[0x2] + 0x8) = param_1 + 0x6;
    param_1[0x5] = 0x1;
  }
  else {
    *(param_1 + 0x3) = *(param_1 + 0x3) | 0x8;
  }
  uVar1 = *(param_1[0x2] + 0x8);
  param_1[0x1] = 0x0;
  *param_1 = uVar1;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn write_file_004b1830(param_1: u32,LPCVOID write_buf,DWORD num_bytes_to_write) -> u32

{
  let mut uVar1: u32;
  let DVar2: u32;
  let mut iVar3: i32;
  let mut success: bool;
  let num_bytes_written: u32;
  HANDLE file_handle;
  
  if ((param_1 < 0x0) || (DAT_004bffe8 < param_1)) {
    FUN_004b1740(0x4);
    num_bytes_written = 0xffffffff;
  }
  else {
    file_handle = *(HANDLE *)(DAT_004c0190 + param_1 * 0x4);
    (PTR_FUN_004bfb78)(param_1);
    uVar1 = FUN_004b1a30(param_1);
    if ((uVar1 & 0x80) != 0x0) {
                    // DWORD dwMoveMethod for SetFilePointer
                    // PLONG lpDistanceToMoveHigh for SetFilePointer
                    // LONG lDistanceToMove for SetFilePointer
                    // HANDLE hFile for SetFilePointer
      DVar2 = SetFilePointer(file_handle,0x0,(PLONG)0x0,0x2);
      if (DVar2 == 0xffffffff) {
        (PTR_FUN_004bfb7c)(param_1);
        DVar2 = handle_err_fn_004b12fc();
        return DVar2;
      }
    }
    if (_DAT_004c0070 != (code *)0x0) {
      iVar3 = (*DAT_004c0044)(param_1);
      if (iVar3 != 0x0) {
        DVar2 = (*_DAT_004c0070)(iVar3,write_buf,num_bytes_to_write);
        (PTR_FUN_004bfb7c)(param_1);
        return DVar2;
      }
    }
                    // LPOVERLAPPED lpOverlapped for WriteFile
                    // LPDWORD lpNumberOfBytesWritten for WriteFile
                    // DWORD nNumberOfBytesToWrite for WriteFile
                    // LPCVOID lpBuffer for WriteFile
                    // HANDLE hFile for WriteFile
    success = WriteFile(file_handle,write_buf,num_bytes_to_write,&num_bytes_written,(LPOVERLAPPED)0x0);
    if (success == 0x0) {
      (PTR_FUN_004bfb7c)(param_1);
      DVar2 = handle_err_fn_004b12fc();
      return DVar2;
    }
    if (num_bytes_to_write != num_bytes_written) {
      FUN_004b1740(0xc);
    }
    (PTR_FUN_004bfb7c)(param_1);
  }
  return num_bytes_written;
}



fn read_file_004b1940(param_1: u32,LPVOID read_buffer,DWORD num_bytes_to_read) -> u32

{
  let mut iVar1: i32;
  let DVar2: u32;
  let mut success: bool;
  let num_bytes_read: u32;
  
  if ((param_1 < 0x0) || (DAT_004bffe8 < param_1)) {
    FUN_004b1740(0x4);
    num_bytes_read = 0xffffffff;
  }
  else {
    (PTR_FUN_004bfb78)(param_1);
    if (DAT_004c006c != (code *)0x0) {
      iVar1 = (*DAT_004c0044)(param_1);
      if (iVar1 != 0x0) {
        DVar2 = (*DAT_004c006c)(iVar1,read_buffer,num_bytes_to_read);
        (PTR_FUN_004bfb7c)(param_1);
        return DVar2;
      }
    }
                    // LPOVERLAPPED lpOverlapped for ReadFile
                    // LPDWORD lpNumberOfBytesRead for ReadFile
                    // DWORD nNumberOfBytesToRead for ReadFile
                    // LPVOID lpBuffer for ReadFile
                    // HANDLE hFile for ReadFile
    success = ReadFile(*(HANDLE *)(DAT_004c0190 + param_1 * 0x4),read_buffer,num_bytes_to_read,&num_bytes_read,
                       (LPOVERLAPPED)0x0);
    if (success == 0x0) {
      DVar2 = GetLastError();
      FUN_004b1290(DVar2);
      if ((DVar2 != 0x6d) || (num_bytes_read != 0x0)) {
        (PTR_FUN_004bfb7c)(param_1);
        return 0xffffffff;
      }
    }
    (PTR_FUN_004bfb7c)(param_1);
  }
  return num_bytes_read;
}



fn FUN_004b1a30(param_1: u32) -> u32

{
  let bVar1: u8;
  let DVar2: u32;
  
  if (DAT_004bffe8 <= param_1) {
    return 0x0;
  }
  if (param_1 < 0x3) {
    bVar1 = (PTR_DAT_004c003c + param_1 * 0x4)[0x1];
    if ((bVar1 & 0x40) == 0x0) {
      (PTR_DAT_004c003c + param_1 * 0x4)[0x1] = bVar1 | 0x40;
      DVar2 = get_file_type_fn_004b5340(param_1);
      if (DVar2 != 0x0) {
        PTR_DAT_004c003c[param_1 * 0x4 + 0x1] = PTR_DAT_004c003c[param_1 * 0x4 + 0x1] | 0x20;
      }
    }
  }
  return *(PTR_DAT_004c003c + param_1 * 0x4);
}



fn FUN_004b1a88(param_1: i32,param_2: u32)

{
  if (param_2 != 0x0) {
    *(PTR_DAT_004c003c + param_1 * 0x4) = param_2 | 0x4000;
    return;
  }
  *(PTR_DAT_004c003c + param_1 * 0x4) = 0x0;
  return;
}



fn FUN_004b1ac0()

{
  FUN_004b1acc(0xffffffff);
  return;
}



fn FUN_004b1acc(param_1: u32) -> i32

{
  let puVar1: *mut u32;
  let puVar2: *mut u32;
  let mut iVar3: i32;
  
  (PTR_FUN_004bfb88)();
  iVar3 = 0x0;
  for (puVar2 = DAT_005ba410; puVar2 != 0x0; puVar2 = *puVar2) {
    puVar1 = puVar2[0x1];
    if (((puVar1[0x3] & param_1) != 0x0) && (iVar3 = iVar3 + 0x1, (*(puVar1 + 0xd) & 0x10) != 0x0)) {
      FUN_004af2f0(puVar1);
    }
  }
  (PTR_FUN_004bfb8c)();
  return iVar3;
}



fn FUN_004b1b20() -> u32

{
  let mut iVar1: i32;
  let mut uVar2: u32;
  
  iVar1 = DAT_004c0158;
  uVar2 = console_fn_004b7468();
  if ((uVar2 != 0xffffffff) && (iVar1 == 0x0)) {
    write_to_console_004b7500(uVar2);
  }
  return uVar2;
}



fn FUN_004b1b60(param_1: i32)

{
  let bVar1: u8;
  let DVar2: u32;
  
  if ((*(param_1 + 0xd) & 0x20) == 0x0) {
    DVar2 = get_file_type_fn_004b5340(*(param_1 + 0x10));
    if (DVar2 != 0x0) {
      bVar1 = *(param_1 + 0xd);
      *(param_1 + 0xd) = bVar1 | 0x20;
      if ((bVar1 & 0x7) == 0x0) {
        *(param_1 + 0xd) = bVar1 | 0x22;
      }
    }
  }
  return;
}



fn FUN_004b1ba0(byte *param_1,param_2: u32) -> i32

{
  let mut iVar1: i32;
  let mut uVar2: u32;
  let mut iVar3: i32;
  
  iVar3 = 0x0;
  for (; (iVar1 = FUN_004b7570(param_1), iVar1 == 0x0 && (param_2 != 0x0)); param_2 = param_2 - uVar2) {
    uVar2 = FUN_004b75b0(param_1);
    if (param_2 < uVar2) {
      return iVar3;
    }
    iVar3 = iVar3 + 0x1;
    param_1 = param_1 + uVar2;
  }
  return iVar3;
}



char *  FUN_004b1be0(param_1: &mut String,byte *param_2,param_3: i32)

{
  let mut iVar1: i32;
  
  iVar1 = FUN_004b1c10(param_2,param_3);
  FUN_004b75e0(param_1,param_2,iVar1 + 0x1);
  return param_1;
}



fn FUN_004b1c10(byte *param_1,param_2: i32) -> i32

{
  let mut iVar1: i32;
  let mut iVar2: i32;
  
  iVar2 = 0x0;
  for (; (iVar1 = FUN_004b7570(param_1), iVar1 == 0x0 && (param_2 != 0x0)); param_2 = param_2 + -0x1) {
    if ((DAT_005bac40 == 0x0) || (((&DAT_005bac51)[*param_1] & 0x1) == 0x0)) {
      iVar2 = iVar2 + 0x1;
    }
    else {
      if (param_1[0x1] == 0x0) {
        return iVar2;
      }
      iVar1 = FUN_004b75b0(param_1);
      iVar2 = iVar2 + iVar1;
    }
    param_1 = FUN_004b1c70(param_1);
  }
  return iVar2;
}



byte *  FUN_004b1c70(byte *param_1)

{
  if (((DAT_005bac40 != 0x0) && (((&DAT_005bac51)[*param_1] & 0x1) != 0x0)) && (param_1[0x1] != 0x0)) {
    return param_1 + 0x2;
  }
  return param_1 + 0x1;
}



ushort  FUN_004b1ca0(byte *param_1)

{
  if ((DAT_005bac40 != 0x0) && (((&DAT_005bac51)[*param_1] & 0x1) != 0x0)) {
    return CONCAT11(*param_1,param_1[0x1]);
  }
  return (ushort)*param_1;
}



fn FUN_004b1ce0(undefined **param_1)

{
  if (*param_1 != 0x0) {
    (*param_1)();
  }
  return;
}



fn FUN_004b1cf0()

{
  let in_AL: u8;
  let mut pcVar1: String; 
  let bVar2: u8;
  let mut pcVar3: String; 
  
  while( true ) {
    pcVar1 = &DAT_004c41f8;
    pcVar3 = &DAT_004c4258;
    bVar2 = in_AL;
    loop {
      if ((*pcVar1 != '\x02') && ((byte)pcVar1[0x1] <= bVar2)) {
        bVar2 = pcVar1[0x1];
        pcVar3 = pcVar1;
      }
      pcVar1 = pcVar1 + 0x6;
    } while (pcVar1 < &DAT_004c4258);
    if (pcVar3 == &DAT_004c4258) break;
    FUN_004b1ce0((undefined **)(pcVar3 + 0x2));
    *pcVar3 = '\x02';
  }
  return;
}



void  FUN_004b1d48(param_1: u32,byte param_2)

{
  let in_AL: u8;
  let mut pcVar1: String; 
  let bVar2: u8;
  let mut pcVar3: String; 
  
  while( true ) {
    pcVar1 = &DAT_004c4258;
    pcVar3 = &DAT_004c4288;
    bVar2 = in_AL;
    loop {
      if ((*pcVar1 != '\x02') && (bVar2 <= (byte)pcVar1[0x1])) {
        bVar2 = pcVar1[0x1];
        pcVar3 = pcVar1;
      }
      pcVar1 = pcVar1 + 0x6;
    } while (pcVar1 < &DAT_004c4288);
    if (pcVar3 == &DAT_004c4288) break;
    if ((byte)pcVar3[0x1] <= param_2) {
      FUN_004b1ce0((undefined **)(pcVar3 + 0x2));
    }
    *pcVar3 = '\x02';
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

double  FUN_004b1dab(param_1: u32,param_2: u32)

{
  double dVar1;
  double local_18 [0x2];
  
  dVar1 = FUN_004b7670(param_1,param_2,local_18);
  return dVar1;
}



float10  FUN_004b1e09(byte *param_1,param_2: *mut u32)

{
  let mut in_EAX: u32;
  let mut extraout_EDX: u32;
  double dVar1;
  
  dVar1 = FUN_004b7bb2(param_1,(byte **)0x0);
  *param_2 = in_EAX;
  param_2[0x1] = extraout_EDX;
  return (float10)dVar1;
}



fn FUN_004b1eb4(float10 *param_1,param_2: u32)

{
  float10 *pfVar1;
  
  if (0x1fff < param_2) {
    param_2 = 0x2000;
  }
  pfVar1 = (float10 *)&DAT_004c0088;
  for (; 0x0 < param_2; param_2 = param_2 >> 0x1) {
    if ((param_2 & 0x1) != 0x0) {
      *param_1 = *pfVar1 * *param_1;
    }
    pfVar1 = pfVar1 + 0x1;
  }
  return;
}



fn FUN_004b1f0c(float10 *param_1,param_2: u32)

{
  let mut local_1c: u32;
  let mut uStack24: u32;
  let uStack20: u16;
  
  if (param_2 != 0x0) {
    uStack20 = 0x3fff;
    local_1c = 0x0;
    uStack24 = 0x80000000;
    if (param_2 < 0x0) {
      FUN_004b1eb4((float10 *)&local_1c,-param_2);
      *param_1 = *param_1 / (float10)CONCAT28(uStack20,CONCAT44(uStack24,local_1c));
    }
    else {
      FUN_004b1eb4((float10 *)&local_1c,param_2);
      *param_1 = (float10)CONCAT28(uStack20,CONCAT44(uStack24,local_1c)) * *param_1;
    }
  }
  return;
}



fn FUN_004b1fa0(float10 *param_1,param_2: u32)

{
  let mut uVar1: u32;
  
  if (param_2 < 0x1001) {
    if (-0x1001 < param_2)^ // goto LAB_004b1fe0;
    uVar1 = 0xfffff000;
    param_2 = param_2 + 0x1000;
  }
  else {
    uVar1 = 0x1000;
    param_2 = param_2 - 0x1000;
  }
  FUN_004b1f0c(param_1,uVar1);
// LAB_004b1fe0:
  FUN_004b1f0c(param_1,param_2);
  return;
}



fn FUN_004b2008(param_1: *mut u32,param_2: *mut i32,param_3: *mut u32) -> u32

{
  ushort uVar1;
  let mut uVar2: u32;
  let mut uVar3: u32;
  let mut iVar4: i32;
  let puVar5: *mut u32;
  let puVar6: *mut u32;
  let mut iVar7: i32;
  let puVar8: *mut u32;
  let mut pcVar9: String; 
  ushort in_FPUControlWord;
  float10 fVar10;
  let local_98: u8;
  u32 local_97 [0xf];
  float10 local_58;
  let local_4c: u8 [0x8];
  let uStack68: u8;
  let bStack67: u8;
  float10 local_40;
  float10 local_34;
  let local_28: *mut u32;
  let mut local_24: u32;
  let local_20: *mut u32;
  let mut local_1c: i32;
  let mut local_18: u32;
  let local_14: u8;
  
  local_18 = in_FPUControlWord;
  param_2[0x5] = 0x0;
  uVar1 = (param_1 + 0x2);
  uStack68 = uVar1;
  bStack67 = (byte)(uVar1 >> 0x8);
  local_4c._4_4_ = param_1[0x1];
  local_4c._0_4_ = *param_1;
  if ((uVar1 & 0x8000) != 0x0) {
    param_2[0x5] = -0x1;
  }
  bStack67 = bStack67 & 0x7f;
  param_2[0x7] = 0x0;
  param_2[0x8] = 0x0;
  param_2[0x9] = 0x0;
  param_2[0xa] = 0x0;
  iVar7 = 0x0;
  param_2[0x6] = 0x0;
  uVar2 = FUN_004b7c98(local_4c);
  switch(uVar2) {
  case 0x0:
  case 0x4:
    param_2[0x5] = 0x0;
    local_24 = 0x0;
    break;
  case 0x1:
    local_24 = ((CONCAT11(bStack67,uStack68) - 0x3ffe) * 0x7597) / 0x186a0 - 0x4;
    if (local_24 != 0x0) {
      if (local_24 < 0x0) {
        uVar3 = 0x3 - local_24 & 0xfffffffc;
        local_24 = -uVar3;
// LAB_004b2208:
        FUN_004b1fa0((float10 *)local_4c,uVar3);
      }
      else {
        if ((CONCAT11(bStack67,uStack68) < 0x4019) ||
           ((CONCAT11(bStack67,uStack68) == 0x4019 && (local_4c._4_4_ < 0xbebc2000)))) {
          local_24 = 0x0;
        }
        else {
          if ((0x4033 < CONCAT11(bStack67,uStack68)) &&
             ((CONCAT11(bStack67,uStack68) != 0x4034 ||
              ((0x8e1bc9be < local_4c._4_4_ && ((local_4c._4_4_ != 0x8e1bc9bf || (0x3ffffff < local_4c._0_4_)))))))) {
            local_24 = local_24 & 0xfffffffc;
            uVar3 = -local_24;^
            // goto LAB_004b2208;
          }
          local_40 = (float10)((unkuint10)0xbebc200000000000 & 0xffffffff00000000);
          iVar7 = ROUND((float10)CONCAT19(bStack67,CONCAT18(uStack68,CONCAT44(local_4c._4_4_,local_4c._0_4_))) /
                             local_40);
          local_34 = (float10)iVar7;
          local_40 = local_40 * (float10)iVar7;
          fVar10 = (float10)CONCAT19(bStack67,CONCAT18(uStack68,CONCAT44(local_4c._4_4_,local_4c._0_4_))) - local_40;
          local_4c._0_4_ = SUB104(fVar10,0x0);
          local_4c._4_4_ = ((unkuint10)fVar10 >> 0x20);
          uStack68 = ((unkuint10)fVar10 >> 0x40);
          bStack67 = (byte)((unkuint10)fVar10 >> 0x48);
          local_24 = 0x8;
        }
      }
    }
    break;
  case 0x2:
    param_3 = 0x6e;
    (param_3 + 0x1) = 0x61;
    (param_3 + 0x2) = 0x6e;^
    // goto LAB_004b20bd;
  case 0x3:
    param_3 = 0x69;
    (param_3 + 0x1) = 0x6e;
    (param_3 + 0x2) = 0x66;
// LAB_004b20bd:
    (param_3 + 0x3) = 0x0;
    param_2[0x7] = 0x3;^
    // goto LAB_004b1f96;
  }
  if ((*(param_2 + 0x2) & 0x2) == 0x0) {
    local_1c = *param_2 + 0x7;
  }
  else {
    local_1c = *param_2 + local_24 + 0xa;
    if (0x0 < param_2[0x1]) {
      local_1c = local_1c + param_2[0x1];
    }
  }
  iVar4 = 0xf;
  if ((*(param_2 + 0x2) & 0x20) != 0x0) {
    iVar4 = 0x14;
  }
  if ((*(param_2 + 0x2) & 0x40) != 0x0) {
    iVar4 = iVar4 * 0x2;
  }
  if (iVar4 + 0x4 < local_1c) {
    local_1c = iVar4 + 0x4;
  }
  local_98 = 0x30;
  local_97[0]._0_1_ = 0x0;
  local_28 = 0x0;
  local_20 = local_97;
  while (puVar5 = local_20, 0x0 < local_1c) {
    local_1c = local_1c + -0x8;
    if (iVar7 == 0x0) {
      if ((CONCAT11(bStack67,uStack68) & 0x7fff) == 0x0) break;
      if (0x0 < local_1c) {
        local_58 = (float10)((unkuint10)0xbebc200000000000 & 0xffffffff00000000);
        fVar10 = local_58 *
                 ((float10)CONCAT19(bStack67,CONCAT18(uStack68,CONCAT44(local_4c._4_4_,local_4c._0_4_))) -
                 (float10)ROUND((float10)CONCAT19(bStack67,CONCAT18(uStack68,CONCAT44(local_4c._4_4_,local_4c._0_4_
                                                                                          )))));
        local_4c._0_4_ = SUB104(fVar10,0x0);
        local_4c._4_4_ = ((unkuint10)fVar10 >> 0x20);
        uStack68 = ((unkuint10)fVar10 >> 0x40);
        bStack67 = (byte)((unkuint10)fVar10 >> 0x48);
      }
    }
    FUN_004b2317();
    puVar5 = 0x0;
    local_20 = puVar5;
    iVar7 = 0x0;
    local_28 = local_28 + 0x2;
  }
  iVar7 = local_24 + 0x7;
  puVar8 = local_28;
  for (puVar5 = local_97; local_24 = iVar7, puVar5 == '0'; puVar5 = (puVar5 + 0x1)) {
    puVar8 = (puVar8 + -0x1);
    iVar7 = iVar7 + -0x1;
  }
  puVar6 = *param_2;
  if ((*(param_2 + 0x2) & 0x2) == 0x0) {
    if ((*(param_2 + 0x2) & 0x1) != 0x0) {
      if (param_2[0x1] < 0x1) {
        puVar6 = (puVar6 + param_2[0x1]);
      }
      else {
        puVar6 = (puVar6 + 0x1);
      }
      local_24 = (iVar7 + 0x1) - param_2[0x1];
    }
  }
  else {
    local_24 = iVar7 + param_2[0x1];
    puVar6 = (puVar6 + iVar7 + param_2[0x1] + 0x1);
  }
  if (-0x1 < puVar6) {
    if (puVar8 < puVar6) {
      puVar6 = puVar8;
    }
    iVar7 = 0xf;
    if ((*(param_2 + 0x2) & 0x20) != 0x0) {
      iVar7 = 0x14;
    }
    if ((*(param_2 + 0x2) & 0x40) != 0x0) {
      iVar7 = iVar7 * 0x2;
    }
    if (iVar7 < puVar6) {
      puVar6 = (iVar7 + 0x1);
    }
    local_14 = '0';
    if ((puVar6 < puVar8) && (0x34 < *(puVar6 + puVar5))) {
      local_14 = '9';
    }
    pcVar9 = (puVar6 + puVar5);
    puVar8 = puVar6;
    while( true ) {
      pcVar9 = pcVar9 + -0x1;
      puVar8 = (puVar8 + -0x1);
      if (*pcVar9 != local_14) break;
      puVar6 = (puVar6 + -0x1);
    }
    if (local_14 == '9') {
      *pcVar9 = *pcVar9 + '\x01';
    }
    if (puVar8 < 0x0) {
      puVar5 = (puVar5 + -0x1);
      local_24 = local_24 + 0x1;
      puVar6 = (puVar6 + 0x1);
    }
  }
  if (puVar6 < 0x1) {
    puVar6 = 0x1;
    local_24 = 0x0;
    local_98 = 0x30;
    puVar5 = &local_98;
    param_2[0x5] = 0x0;
  }
  if (((*(param_2 + 0x2) & 0x2) == 0x0) &&
     (((*(param_2 + 0x2) & 0x4) == 0x0 ||
      (((local_24 < -0x4 || (*param_2 <= local_24)) && ((*(param_2 + 0x2) & 0x8) == 0x0)))))) {
    FUN_004b26b8(param_2,puVar5,puVar6,local_24,param_3);
  }
  else {
    FUN_004b24a7(param_2,puVar5,puVar6,local_24,param_3);
  }
// LAB_004b1f96:
  return local_18 & 0xffff;
}



fn FUN_004b2317()

{
  ushort uVar1;
  let mut in_EAX: u32;
  let mut unaff_EBX: String; 
  
  if (0x270f < in_EAX) {
    in_EAX = in_EAX % 0x2710;
  }
  FUN_004b232d();
  if (0x63 < in_EAX) {
    in_EAX = (in_EAX & 0xffff) % 0x64;
  }
  FUN_004b2344();
  uVar1 = (ushort)((in_EAX & 0xff) << 0x8) | (ushort)(in_EAX >> 0x8) & 0xff;
  if (0x9 < (byte)in_EAX) {
    uVar1 = (ushort)((in_EAX >> 0x8 & 0xff) << 0x8) | (ushort)(byte)(in_EAX & 0xff);
    uVar1 = uVar1 / 0xa & 0xff | uVar1 % 0xa << 0x8;
  }
  *unaff_EBX = (char)uVar1 + '0';
  unaff_EBX[0x1] = (char)(uVar1 >> 0x8) + '0';
  return;
}



fn FUN_004b232d()

{
  ushort uVar1;
  let mut in_EAX: u32;
  let mut unaff_EBX: String; 
  
  if (0x63 < in_EAX) {
    in_EAX = (in_EAX & 0xffff) % 0x64;
  }
  FUN_004b2344();
  uVar1 = (ushort)((in_EAX & 0xff) << 0x8) | (ushort)(in_EAX >> 0x8) & 0xff;
  if (0x9 < (byte)in_EAX) {
    uVar1 = (ushort)((in_EAX >> 0x8 & 0xff) << 0x8) | (ushort)(byte)(in_EAX & 0xff);
    uVar1 = uVar1 / 0xa & 0xff | uVar1 % 0xa << 0x8;
  }
  *unaff_EBX = (char)uVar1 + '0';
  unaff_EBX[0x1] = (char)(uVar1 >> 0x8) + '0';
  return;
}



fn FUN_004b2344()

{
  ushort in_AX;
  let cVar1: u8;
  let bVar2: u8;
  let mut unaff_EBX: String; 
  
  bVar2 = (byte)in_AX;
  cVar1 = (char)(in_AX >> 0x8);
  if (0x9 < bVar2) {
    cVar1 = (char)(in_AX / 0xa);
    bVar2 = (byte)(in_AX % 0xa);
  }
  *unaff_EBX = cVar1 + '0';
  unaff_EBX[0x1] = bVar2 + 0x30;
  return;
}



u32 * 
FUN_004b24a7(param_1: *mut u32,param_2: *mut u32,param_3: *mut u32,param_4: i32,param_5: *mut u32)

{
  let mut iVar1: i32;
  let mut iVar2: i32;
  let puVar3: *mut u32;
  let local_14: *mut u32;
  
  local_14 = *param_1;
  iVar1 = param_4 + 0x1;
  iVar2 = 0x0;
  if ((*(param_1 + 0x2) & 0x4) != 0x0) {
    if ((param_3 < local_14) && ((*(param_1 + 0x2) & 0x10) == 0x0)) {
      local_14 = param_3;
    }
    local_14 = (local_14 - iVar1);
    if (local_14 < 0x0) {
      local_14 = 0x0;
    }
  }
  if (iVar1 < 0x1) {
    if ((*(param_1 + 0x2) & 0x8) == 0x0) {
      param_5 = 0x30;
      iVar2 = 0x1;
      if ((0x0 < local_14) || ((*(param_1 + 0x2) & 0x10) != 0x0)) {
        iVar2 = 0x2;
        (param_5 + 0x1) = 0x2e;
      }
    }
    param_1[0x7] = iVar2;
    param_4 = iVar1;
    if (-local_14 != iVar1 && local_14 <= -iVar1) {
      param_4 = -local_14;
    }
    param_1[0x6] = param_4;
    local_14 = (local_14 + param_4);
    param_1[0x8] = param_4;
    param_1[0x8] = -param_4;
    if (local_14 < param_3) {
      param_3 = local_14;
    }
    *(param_5 + iVar2) = *param_2;
    ((param_5 + iVar2) + 0x1) = (param_2 + 0x1);
    param_1[0x9] = param_3;
    local_14 = (local_14 - param_3);
    puVar3 = (iVar2 + param_3);
  }
  else {
    if (iVar1 <= param_3) {
      *param_5 = *param_2;
      (param_5 + 0x1) = (param_2 + 0x1);
      param_3 = (param_3 - iVar1);
      param_1[0x6] = iVar1;
      iVar2 = iVar1;
      if ((*(param_1 + 0x2) & 0x8) == 0x0) {
        if ((0x0 < local_14) || ((*(param_1 + 0x2) & 0x10) != 0x0)) {
          (iVar1 + param_5) = 0x2e;
          iVar2 = param_4 + 0x2;
        }
      }
      else {
        if (param_5 == '0') {
          param_1[0x6] = 0x0;
        }
      }
      if (local_14 < param_3) {
        param_3 = local_14;
      }
      *(param_5 + iVar2) = *(param_2 + iVar1);
      ((param_5 + iVar2) + 0x1) =
           ((param_2 + iVar1) + 0x1);
      puVar3 = (iVar2 + param_3);
      param_1[0x7] = puVar3;
      param_1[0x8] = local_14 - param_3;^
      // goto LAB_004b2698;
    }
    *param_5 = *param_2;
    (param_5 + 0x1) = (param_2 + 0x1);
    param_1[0x7] = param_3;
    param_1[0x8] = iVar1 - param_3;
    param_1[0x6] = iVar1;
    puVar3 = param_3;
    if (((*(param_1 + 0x2) & 0x8) == 0x0) &&
       ((0x0 < local_14 || ((*(param_1 + 0x2) & 0x10) != 0x0)))) {
      (param_3 + param_5) = 0x2e;
      puVar3 = (param_3 + 0x1);
      param_1[0x9] = 0x1;
      param_3 = param_5;
    }
  }
  param_1[0xa] = local_14;
// LAB_004b2698:
  (puVar3 + param_5) = 0x0;
  return param_3;
}



// WARNING: Could not reconcile some variable overlaps

fn FUN_004b26b8(param_1: *mut i32,param_2: *mut u32,param_3: i32,param_4: i32,param_5: *mut u32)

{
  let mut puVar1: *mut u8; 
  let mut pcVar2: String; 
  let mut uVar3: u32;
  let mut iVar4: i32;
  let mut iVar5: i32;
  let mut local_18: i32;
  let mut local_14: i32;
  
  local_18 = param_1[0x1];
  if (local_18 < 0x1) {
    local_18 = *param_1 + local_18;
  }
  else {
    local_18 = (*param_1 - local_18) + 0x1;
  }
  if ((*(param_1 + 0x2) & 0x4) != 0x0) {
    if (param_3 < local_18) {
      local_18 = param_3;
    }
    local_18 = local_18 + -0x1;
    if (local_18 < 0x0) {
      local_18 = 0x0;
    }
  }
  local_14 = param_1[0x1];
  if (local_14 < 0x1) {
    local_14 = 0x1;
    param_5 = 0x30;
  }
  else {
    if (param_3 < local_14) {
      local_14 = param_3;
    }
    *param_5 = *param_2;
    (param_5 + 0x1) = (param_2 + 0x1);
    param_2 = (param_2 + local_14);
    param_3 = param_3 - local_14;
    if (local_14 < param_1[0x1]) {
      uVar3 = param_1[0x1] - local_14;
      FUN_004a0430(param_5 + local_14,0x30,uVar3);
      local_14 = local_14 + uVar3;
    }
  }
  param_1[0x6] = local_14;
  if (((*(param_1 + 0x2) & 0x8) == 0x0) && ((0x0 < local_18 || ((*(param_1 + 0x2) & 0x10) != 0x0)))) {
    puVar1 = (local_14 + param_5);
    local_14 = local_14 + 0x1;
    *puVar1 = 0x2e;
  }
  if (param_1[0x1] < 0x0) {
    uVar3 = -param_1[0x1];
    FUN_004a0430(param_5 + local_14,0x30,uVar3);
    local_14 = local_14 + uVar3;
  }
  if (0x0 < local_18) {
    if (local_18 < param_3) {
      param_3 = local_18;
    }
    if (param_3 != 0x0) {
      *(param_5 + local_14) = *param_2;
      ((param_5 + local_14) + 0x1) = (param_2 + 0x1);
      local_14 = local_14 + param_3;
    }
    param_1[0x7] = local_14;
    param_1[0x8] = local_18 - param_3;
  }
  if (param_1[0x3] != 0x0) {
    puVar1 = (local_14 + param_5);
    local_14 = local_14 + 0x1;
    *puVar1 = (param_1 + 0x3);
  }
  iVar5 = local_14 + 0x1;
  if (param_4 < 0x0) {
    param_4 = -param_4;
    (local_14 + param_5) = 0x2d;
  }
  else {
    (local_14 + param_5) = 0x2b;
  }
  iVar4 = param_1[0x4];
  switch(iVar4) {
  case 0x0:
    if (param_4 < 0x3e8) {
      iVar4 = 0x3;^
      // goto switchD_004b287b_caseD_4;
    }^
    // goto LAB_004b28b2;
  case 0x1:
    if (0x9 < param_4) {
      iVar4 = 0x2;
    }
  case 0x2:
    break;
  case 0x3:^
    // goto switchD_004b287b_caseD_3;
  default:^
    // goto switchD_004b287b_caseD_4;
  }
  if (0x63 < param_4) {
    iVar4 = 0x3;
  }
switchD_004b287b_caseD_3:
  if (0x3e7 < param_4) {
// LAB_004b28b2:
    iVar4 = 0x4;
  }
switchD_004b287b_caseD_4:
  param_1[0x4] = iVar4;
  if (0x3 < iVar4) {
    local_14._0_1_ = '\0';
    if (0x3e7 < param_4) {
      local_14._0_1_ = (char)(param_4 / 0x3e8);
      param_4 = param_4 % 0x3e8;
    }
    pcVar2 = (iVar5 + param_5);
    iVar5 = local_14 + 0x2;
    *pcVar2 = (char)local_14 + '0';
  }
  if (0x2 < iVar4) {
    local_14._0_1_ = '\0';
    if (0x63 < param_4) {
      local_14._0_1_ = (char)(param_4 / 0x64);
      param_4 = param_4 % 0x64;
    }
    pcVar2 = (iVar5 + param_5);
    iVar5 = iVar5 + 0x1;
    *pcVar2 = (char)local_14 + '0';
  }
  if (0x1 < iVar4) {
    local_14._0_1_ = '\0';
    if (0x9 < param_4) {
      local_14._0_1_ = (char)(param_4 / 0xa);
      param_4 = param_4 % 0xa;
    }
    pcVar2 = (iVar5 + param_5);
    iVar5 = iVar5 + 0x1;
    *pcVar2 = (char)local_14 + '0';
  }
  (iVar5 + param_5) = (char)param_4 + '0';
  param_1[0x9] = (iVar5 + 0x1) - param_1[0x7];
  (iVar5 + 0x1 + param_5) = 0x0;
  return;
}



fn FUN_004b2a90(param_1: *mut u32,byte *param_2,param_3: *mut *mut u32) -> u32

{
  let mut iVar1: i32;
  let mut uVar2: u32;
  let mut uVar3: u32;
  let bVar4: u8;
  let mut bVar5: bool;
  
  (PTR_FUN_004bfb78)(param_1[0x4]);
  iVar1 = *(param_1[0x2] + 0xc);
  if (iVar1 != 0x1) {
    if (iVar1 != 0x0) {
      (PTR_FUN_004bfb7c)(param_1[0x4]);
      return 0x0;
    }
    *(param_1[0x2] + 0xc) = 0x1;
  }
  uVar2 = param_1[0x3];
  *(param_1 + 0x3) = *(param_1 + 0x3) & 0xcf;
  if (*(param_1[0x2] + 0x8) == 0x0) {
    FUN_004b1790(param_1);
  }
  bVar5 = (*(param_1 + 0xd) & 0x4) != 0x0;
  if (bVar5) {
    bVar4 = *(param_1 + 0xd) & 0xfa;
    *(param_1 + 0xd) = bVar4;
    *(param_1 + 0xd) = bVar4 | 0x1;
  }
  uVar3 = FUN_004b42c0(param_1,param_2,param_3,&LAB_004b2a70);
  if (bVar5) {
    bVar4 = *(param_1 + 0xd) & 0xfa;
    *(param_1 + 0xd) = bVar4;
    *(param_1 + 0xd) = bVar4 | 0x4;
    FUN_004af2f0(param_1);
  }
  if ((*(param_1 + 0x3) & 0x20) != 0x0) {
    uVar3 = 0xffffffff;
  }
  param_1[0x3] = param_1[0x3] | uVar2 & 0x30;
  (PTR_FUN_004bfb7c)(param_1[0x4]);
  return uVar3;
}



fn FUN_004b2b70(param_1: *mut u32) -> u32

{
  let DVar1: u32;
  
  if (param_1 == 0x0) {
    FUN_004b1ac0();
    return 0x0;
  }
  DVar1 = FUN_004af2f0(param_1);
  return DVar1;
}



fn set_file_ptr_004b2b90(param_1: u32) -> u32

{
  HANDLE hFile;
  let DVar1: u32;
  
  if ((-0x1 < param_1) && (param_1 <= DAT_004bffe8)) {
    hFile = *(HANDLE *)(DAT_004c0190 + param_1 * 0x4);
    (PTR_FUN_004bfb78)(param_1);
                    // DWORD dwMoveMethod for SetFilePointer
                    // PLONG lpDistanceToMoveHigh for SetFilePointer
                    // LONG lDistanceToMove for SetFilePointer
                    // HANDLE hFile for SetFilePointer
    DVar1 = SetFilePointer(hFile,0x0,(PLONG)0x0,0x1);
    (PTR_FUN_004bfb7c)(param_1);
    if (DVar1 == 0xffffffff) {
      handle_err_fn_004b12fc();
    }
    return DVar1;
  }
  FUN_004b1740(0x4);
  return 0xffffffff;
}



fn FUN_004b2bf0(undefined **param_1)

{
  (*param_1)(param_1);
  return;
}



fn FUN_004b2bfc(param_1: u32,param_2: i32)

{
  ((param_2 + 0x4))(param_1,param_2);
  return;
}



fn FUN_004b2c10(undefined **param_1,byte *param_2,param_3: *mut *mut u32) -> i32

{
  byte *pbVar1;
  let bVar2: u8;
  let mut iVar3: i32;
  let mut uVar4: u32;
  let mut uVar5: u32;
  let mut iVar6: i32;
  let mut iVar7: i32;
  let local_14: *mut u32;
  
  local_14 = *param_3;
  iVar6 = 0x0;
  iVar7 = 0x0;
  *(param_1 + 0x4) = *(param_1 + 0x4) & 0xfd;
  loop {
    pbVar1 = param_2 + 0x1;
    uVar5 = *param_2;
    if (uVar5 == 0x0)^ // goto LAB_004b2ec0;
    if (((&DAT_004bf9c4)[(byte)(*param_2 + 0x1)] & 0x2) != 0x0) {
      param_2 = pbVar1;
      iVar3 = FUN_004b2fd8(param_1);
      iVar7 = iVar7 + iVar3;^
      // goto LAB_004b2e84;
    }
    if (uVar5 == 0x25) {
      param_2 = pbVar1;
      param_2 = FUN_004b2edc(pbVar1,param_1);
      bVar2 = *param_2;
      if (bVar2 != 0x0) {
        param_2 = param_2 + 0x1;
      }
      if (bVar2 < 0x64) {
        if (bVar2 < 0x47) {
          if (bVar2 < 0x43) {
            if (bVar2 == 0x25) {
              iVar3 = FUN_004b2bf0(param_1);
              if (iVar3 == 0x25)^ // goto LAB_004b2e83;
              if ((*(param_1 + 0x4) & 0x2) == 0x0) {
                FUN_004b2bfc(iVar3,param_1);
              }^
              // goto LAB_004b2ec0;
            }
          }
          else {
            if (bVar2 < 0x44) {
              *(param_1 + 0x4) = *(param_1 + 0x4) | 0x20;
// LAB_004b2e32:
              iVar3 = FUN_004b3014(param_1,(byte **)&local_14);^
              // goto LAB_004b2e3e;
            }
            if (bVar2 == 0x45)^ // goto LAB_004b2dff;
          }
        }
        else {
          if (bVar2 < 0x48) {
// LAB_004b2dff:
            iVar3 = FUN_004b3438(param_1,&local_14);^
            // goto LAB_004b2e3e;
          }
          if (bVar2 < 0x58) {
            if (bVar2 == 0x53) {
              *(param_1 + 0x4) = *(param_1 + 0x4) | 0x20;
// LAB_004b2e0e:
              iVar3 = FUN_004b310c(param_1,&local_14);^
              // goto LAB_004b2e3e;
            }
          }
          else {
            if (bVar2 < 0x59)^ // goto LAB_004b2deb;
            if (0x5a < bVar2) {
              if (bVar2 < 0x5c) {
                iVar3 = FUN_004b3324(param_1,&local_14,&param_2);^
                // goto LAB_004b2e3e;
              }
              if (bVar2 == 0x63)^ // goto LAB_004b2e32;
            }
          }
        }
      }
      else {
        if (bVar2 < 0x65) {
          iVar3 = FUN_004b373c(param_1,(i32 **)&local_14,0xa,0x1);
// LAB_004b2e3e:
          if (iVar3 < 0x1)^ // goto LAB_004b2ec0;
          iVar7 = iVar7 + iVar3;
          if ((*(param_1 + 0x4) & 0x1) != 0x0) {
            iVar6 = iVar6 + 0x1;
          }
        }
        else {
          if (bVar2 < 0x6f) {
            if (bVar2 < 0x69) {
              if (bVar2 < 0x68)^ // goto LAB_004b2dff;
            }
            else {
              if (bVar2 < 0x6a) {
                iVar3 = FUN_004b373c(param_1,(i32 **)&local_14,0x0,0x1);^
                // goto LAB_004b2e3e;
              }
              if (bVar2 == 0x6e) {
                FUN_004b3270(param_1,&local_14,iVar7);
              }
            }
          }
          else {
            if (bVar2 < 0x70) {
              iVar3 = FUN_004b373c(param_1,(i32 **)&local_14,0x8,0x1);^
              // goto LAB_004b2e3e;
            }
            if (bVar2 < 0x73) {
              if (bVar2 == 0x70) {
// LAB_004b2deb:
                iVar3 = FUN_004b373c(param_1,(i32 **)&local_14,0x10,0x1);^
                // goto LAB_004b2e3e;
              }
            }
            else {
              if (bVar2 < 0x74)^ // goto LAB_004b2e0e;
              if (0x74 < bVar2) {
                if (bVar2 < 0x76) {
                  iVar3 = FUN_004b373c(param_1,(i32 **)&local_14,0xa,0x0);^
                  // goto LAB_004b2e3e;
                }
                if (bVar2 == 0x78)^ // goto LAB_004b2deb;
              }
            }
          }
        }
      }
    }
    else {
      param_2 = pbVar1;
      uVar4 = FUN_004b2bf0(param_1);
      if (uVar4 != uVar5) {
        if ((*(param_1 + 0x4) & 0x2) == 0x0) {
          FUN_004b2bfc(uVar4,param_1);
        }^
        // goto LAB_004b2ec0;
      }
// LAB_004b2e83:
      iVar7 = iVar7 + 0x1;
    }
// LAB_004b2e84:
  } while ((*(param_1 + 0x4) & 0x2) == 0x0);
  if (*param_2 == 0x25) {
    param_2 = param_2 + 0x1;
    param_2 = FUN_004b2edc(param_2,param_1);
    if (*param_2 == 0x6e) {
      FUN_004b3270(param_1,&local_14,iVar7);
    }
  }
// LAB_004b2ec0:
  if ((iVar6 == 0x0) && ((*(param_1 + 0x4) & 0x2) != 0x0)) {
    iVar6 = -0x1;
  }
  return iVar6;
}



byte *  FUN_004b2edc(byte *param_1,param_2: i32)

{
  let bVar1: u8;
  let mut iVar2: i32;
  
  bVar1 = *(param_2 + 0x10);
  *(param_2 + 0xc) = 0xffffffff;
  *(param_2 + 0x10) = bVar1 | 0x1;
  *(param_2 + 0x10) = bVar1 & 0x3 | 0x1;
  if (*param_1 == 0x2a) {
    param_1 = param_1 + 0x1;
    *(param_2 + 0x10) = *(param_2 + 0x10) & 0xfe;
  }
  bVar1 = *param_1;
  if (((&DAT_004bf9c4)[(byte)(bVar1 + 0x1)] & 0x20) != 0x0) {
    iVar2 = 0x0;
    loop {
      iVar2 = iVar2 * 0xa + (bVar1 - 0x30);
      bVar1 = param_1[0x1];
      param_1 = param_1 + 0x1;
    } while (((&DAT_004bf9c4)[(byte)(bVar1 + 0x1)] & 0x20) != 0x0);
    *(param_2 + 0xc) = iVar2;
  }
  if (*param_1 == 0x4e) {
    *(param_2 + 0x10) = *(param_2 + 0x10) | 0x8;
    param_1 = param_1 + 0x1;
  }
  else {
    if (*param_1 == 0x46) {
      *(param_2 + 0x10) = *(param_2 + 0x10) | 0x4;
      param_1 = param_1 + 0x1;
    }
  }
  bVar1 = *param_1;
  if (bVar1 < 0x68) {
    if (0x48 < bVar1) {
      if (bVar1 < 0x4a) {
        if ((param_1[0x1] == 0x36) && (param_1[0x2] == 0x34)) {
          param_1 = param_1 + 0x3;
          *(param_2 + 0x10) = *(param_2 + 0x10) | 0x40;
        }
      }
      else {
        if (bVar1 == 0x4c) {
          *(param_2 + 0x10) = *(param_2 + 0x10) | 0x40;
          param_1 = param_1 + 0x1;
        }
      }
    }
  }
  else {
    if (bVar1 < 0x69) {
      *(param_2 + 0x10) = *(param_2 + 0x10) | 0x10;
      param_1 = param_1 + 0x1;
    }
    else {
      if ((0x6b < bVar1) && ((bVar1 < 0x6d || (bVar1 == 0x77)))) {
        *(param_2 + 0x10) = *(param_2 + 0x10) | 0x20;
        param_1 = param_1 + 0x1;
      }
    }
  }
  return param_1;
}



fn FUN_004b2fd8(undefined **param_1) -> i32

{
  let mut uVar1: u32;
  let mut iVar2: i32;
  
  iVar2 = 0x0;
  while( true ) {
    uVar1 = FUN_004b2bf0(param_1);
    if (((&DAT_004bf9c4)[(byte)((char)uVar1 + 0x1)] & 0x2) == 0x0) break;
    iVar2 = iVar2 + 0x1;
  }
  if ((*(param_1 + 0x4) & 0x2) == 0x0) {
    FUN_004b2bfc(uVar1,param_1);
  }
  return iVar2;
}



fn FUN_004b3014(undefined **param_1,byte **param_2) -> i32

{
  let bVar1: u8;
  undefined6 *puVar2;
  byte **ppbVar3;
  let bVar4: u8;
  let mut uVar5: u32;
  undefined2 *unaff_EBX;
  let mut iVar6: i32;
  let mut puVar7: *mut u8; 
  let mut local_1c: u32;
  let local_18: u8;
  let local_17: u8;
  
  bVar1 = *(param_1 + 0x4);
  if ((bVar1 & 0x1) != 0x0) {
    if ((bVar1 & 0x4) == 0x0) {
      if ((bVar1 & 0x8) == 0x0) {
        ppbVar3 = (byte **)*param_2;
        *param_2 = (ppbVar3 + 0x1);
        unaff_EBX = *ppbVar3;
      }
      else {
        ppbVar3 = (byte **)*param_2;
        *param_2 = (ppbVar3 + 0x1);
        unaff_EBX = *ppbVar3;
      }
    }
    else {
      puVar2 = (undefined6 *)*param_2;
      *param_2 = (puVar2 + 0x8);
      unaff_EBX = *puVar2;
    }
  }
  puVar7 = param_1[0x3];
  iVar6 = 0x0;
  if (puVar7 == 0xffffffff) {
    puVar7 = 0x1;
  }
  while( true ) {
    if (puVar7 < 0x1) {
      return iVar6;
    }
    bVar4 = FUN_004b2bf0(param_1);
    bVar1 = *(param_1 + 0x4);
    if ((bVar1 & 0x2) != 0x0) break;
    iVar6 = iVar6 + 0x1;
    puVar7 = puVar7 + -0x1;
    if ((bVar1 & 0x1) != 0x0) {
      if ((bVar1 & 0x20) == 0x0) {
        *unaff_EBX = bVar4;
        unaff_EBX = (unaff_EBX + 0x1);
      }
      else {
        local_18 = bVar4;
        if ((DAT_005bac40 != 0x0) && (((&DAT_005bac51)[bVar4] & 0x1) != 0x0)) {
          local_17 = FUN_004b2bf0(param_1);
        }
        uVar5 = conv_pchar_pwchar_004b7de0((WCHAR *)&local_1c,&local_18,0x2);
        if (uVar5 == 0xffffffff) {
          return 0x0;
        }
        *unaff_EBX = local_1c;
        unaff_EBX = unaff_EBX + 0x1;
      }
    }
  }
  return iVar6;
}



fn FUN_004b310c(undefined **param_1,param_2: *mut i32) -> i32

{
  undefined6 *puVar1;
  let puVar2: *mut u32;
  let mut puVar3: *mut u8; 
  let mut uVar4: u32;
  let bVar5: u8;
  let mut iVar6: i32;
  undefined2 *unaff_EDI;
  let mut local_20: u32;
  let local_1c: u8;
  let local_1b: u8;
  let local_18: u8;
  
  if ((*(param_1 + 0x4) & 0x20) == 0x0) {
    local_18 = 0x1;
  }
  else {
    local_18 = 0x2;
  }
  bVar5 = *(param_1 + 0x4);
  if ((bVar5 & 0x1) != 0x0) {
    if ((bVar5 & 0x4) == 0x0) {
      if ((bVar5 & 0x8) == 0x0) {
        puVar2 = *param_2;
        *param_2 = (puVar2 + 0x1);
        unaff_EDI = *puVar2;
      }
      else {
        puVar2 = *param_2;
        *param_2 = (puVar2 + 0x1);
        unaff_EDI = *puVar2;
      }
    }
    else {
      puVar1 = (undefined6 *)*param_2;
      *param_2 = puVar1 + 0x8;
      unaff_EDI = *puVar1;
    }
  }
  iVar6 = 0x0;
  while (uVar4 = FUN_004b2bf0(param_1), ((&DAT_004bf9c4)[(byte)((char)uVar4 + 0x1)] & 0x2) != 0x0) {
    iVar6 = iVar6 + 0x1;
  }
  if ((*(param_1 + 0x4) & 0x2) == 0x0) {
    puVar3 = param_1[0x3];
    param_1[0x3] = puVar3 + -0x1;
    if (puVar3 != 0x0) {
      loop {
        iVar6 = iVar6 + 0x1;
        if ((*(param_1 + 0x4) & 0x1) != 0x0) {
          bVar5 = (byte)uVar4;
          if (local_18 == 0x1) {
            *unaff_EDI = bVar5;
          }
          else {
            local_1c = bVar5;
            if ((DAT_005bac40 != 0x0) && (((&DAT_005bac51)[uVar4 & 0xff] & 0x1) != 0x0)) {
              local_1b = FUN_004b2bf0(param_1);
            }
            uVar4 = conv_pchar_pwchar_004b7de0((WCHAR *)&local_20,&local_1c,0x2);
            if (uVar4 == 0xffffffff) {
              return 0x0;
            }
            *unaff_EDI = local_20;
          }
          unaff_EDI = (unaff_EDI + local_18);
        }
        uVar4 = FUN_004b3bd8(param_1);
        if (uVar4 == 0xffffffff)^ // goto LAB_004b3248;
      } while (((&DAT_004bf9c4)[(byte)((char)uVar4 + 0x1)] & 0x2) == 0x0);
    }
    FUN_004b2bfc(uVar4,param_1);
  }
  else {
    iVar6 = 0x0;
  }
// LAB_004b3248:
  if (((*(param_1 + 0x4) & 0x1) != 0x0) && (0x0 < iVar6)) {
    if (local_18 == 0x1) {
      unaff_EDI = 0x0;
    }
    else {
      *unaff_EDI = 0x0;
    }
  }
  return iVar6;
}



fn FUN_004b3270(param_1: i32,param_2: *mut i32,param_3: u32)

{
  let bVar1: u8;
  undefined6 *puVar2;
  let puVar3: *mut u32;
  
  bVar1 = *(param_1 + 0x10);
  if ((bVar1 & 0x1) != 0x0) {
    if ((bVar1 & 0x4) == 0x0) {
      if ((bVar1 & 0x8) == 0x0) {
        puVar3 = *param_2;
        *param_2 = (puVar3 + 0x1);
        puVar3 = *puVar3;
      }
      else {
        puVar3 = *param_2;
        *param_2 = (puVar3 + 0x1);
        puVar3 = *puVar3;
      }
    }
    else {
      puVar2 = (undefined6 *)*param_2;
      *param_2 = puVar2 + 0x8;
      puVar3 = *puVar2;
    }
    if ((*(param_1 + 0x10) & 0x10) != 0x0) {
      puVar3 = param_3;
      return;
    }
    *puVar3 = param_3;
  }
  return;
}



byte *  FUN_004b32dc(byte *param_1,param_2: i32)

{
  byte *pbVar1;
  let mut uVar2: u32;
  byte *pbVar3;
  
  pbVar3 = param_1 + 0x1;
  FUN_004a0430(param_2,0x0,0x20);
  uVar2 = *param_1;
  if (uVar2 != 0x0) {
    loop {
      pbVar1 = ((uVar2 >> 0x3) + param_2);
      *pbVar1 = *pbVar1 | (&DAT_004c0188)[uVar2 & 0x7];
      uVar2 = *pbVar3;
      if (uVar2 == 0x0) {
        return pbVar3;
      }
      pbVar3 = pbVar3 + 0x1;
    } while (uVar2 != 0x5d);
  }
  return pbVar3;
}



// WARNING: Could not reconcile some variable overlaps

fn FUN_004b3324(undefined **param_1,param_2: *mut i32,byte **param_3) -> i32

{
  let bVar1: u8;
  undefined6 *puVar2;
  let puVar3: *mut u32;
  byte *pbVar4;
  let mut iVar5: i32;
  let mut unaff_ESI: *mut u8; 
  let mut puVar6: *mut u8; 
  let abStack64: u8 [0x20];
  let mut local_20: u32;
  let mut local_1c: u32;
  let mut local_18: u32;
  
  local_1c = (**param_3 == 0x5e);
  if (local_1c != 0x0) {
    *param_3 = *param_3 + 0x1;
  }
  pbVar4 = FUN_004b32dc(*param_3,abStack64);
  *param_3 = pbVar4;
  bVar1 = *(param_1 + 0x4);
  if ((bVar1 & 0x1) != 0x0) {
    if ((bVar1 & 0x4) == 0x0) {
      if ((bVar1 & 0x8) == 0x0) {
        puVar3 = *param_2;
        *param_2 = (puVar3 + 0x1);
        unaff_ESI = *puVar3;
      }
      else {
        puVar3 = *param_2;
        *param_2 = (puVar3 + 0x1);
        unaff_ESI = *puVar3;
      }
    }
    else {
      puVar2 = (undefined6 *)*param_2;
      *param_2 = puVar2 + 0x8;
      unaff_ESI = *puVar2;
    }
  }
  puVar6 = param_1[0x3];
  iVar5 = 0x0;
  loop {
    if (puVar6 == 0x0) {
// LAB_004b341e:
      if (((*(param_1 + 0x4) & 0x1) != 0x0) && (0x0 < iVar5)) {
        *unaff_ESI = 0x0;
      }
      return iVar5;
    }
    local_18 = FUN_004b2bf0(param_1);
    if ((*(param_1 + 0x4) & 0x2) != 0x0)^ // goto LAB_004b341e;
    local_20 = abStack64[local_18 >> 0x3];
    if ((((&DAT_004c0188)[local_18 & 0x7] & abStack64[local_18 >> 0x3]) == 0x0) != local_1c) {
      FUN_004b2bfc(local_18,param_1);^
      // goto LAB_004b341e;
    }
    iVar5 = iVar5 + 0x1;
    puVar6 = puVar6 + -0x1;
    if ((*(param_1 + 0x4) & 0x1) != 0x0) {
      *unaff_ESI = local_18;
      unaff_ESI = unaff_ESI + 0x1;
    }
  } while( true );
}



fn FUN_004b3438(undefined **param_1,param_2: *mut *mut u32) -> i32

{
  let bVar1: u8;
  let bVar2: u8;
  let mut puVar3: *mut u8; 
  undefined6 *puVar4;
  u32 **ppuVar5;
  let mut iVar6: i32;
  let mut uVar7: u32;
  let mut extraout_ECX: String; 
  let mut extraout_ECX_00: String; 
  let mut extraout_ECX_01: String; 
  let mut extraout_ECX_02: String; 
  let mut pcVar8: String; 
  let mut extraout_ECX_03: String; 
  let mut extraout_ECX_04: String; 
  let mut extraout_ECX_05: String; 
  let mut extraout_ECX_06: String; 
  let mut extraout_ECX_07: String; 
  let puVar9: *mut u32;
  let mut pcVar10: String; 
  let mut pcVar11: String; 
  let mut iVar12: i32;
  let mut iVar13: i32;
  let local_80: u8;
  let local_7f: u8 [0x4f];
  let mut local_30: u32;
  let mut local_2c: u32;
  let mut local_28: i32;
  let mut local_24: i32;
  let mut local_20: u32;
  let local_1c: u16;
  let sStack26: i16;
  let local_18: u16;
  let uStack22: u8;
  let uStack21: u8;
  
  pcVar10 = &local_80;
  iVar12 = 0x0;
  local_24 = 0x0;
  while( true ) {
    iVar6 = FUN_004b2bf0(param_1);
    pcVar8 = (byte)((char)iVar6 + 0x1);
    if ((pcVar8[0x4bf9c4] & 0x2U) == 0x0) break;
    local_24 = local_24 + 0x1;
  }
  if ((*(param_1 + 0x4) & 0x2) != 0x0)^ // goto LAB_004b366d;
  puVar3 = param_1[0x3];
  param_1[0x3] = puVar3 + -0x1;
  pcVar10 = &local_80;
  if (puVar3 != 0x0) {
    if ((iVar6 == 0x2b) || (pcVar10 = &local_80, iVar6 == 0x2d)) {
      local_24 = local_24 + 0x1;
      pcVar10 = local_7f;
      local_80 = (char)iVar6;
      iVar6 = FUN_004b3bd8(param_1);
      pcVar8 = extraout_ECX;
      if (iVar6 == -0x1)^ // goto LAB_004b366d;
    }
    if ((((&DAT_004bf9c4)[(byte)((char)iVar6 + 0x1)] & 0x20) != 0x0) || (iVar6 == 0x2e)) {
      local_1c = 0x0;
      sStack26 = 0x0;
      local_28 = 0x0;
      if (((&DAT_004bf9c4)[(byte)((char)iVar6 + 0x1)] & 0x20) != 0x0) {
        local_28 = 0x1;
        loop {
          *pcVar10 = (char)iVar6;
          pcVar10 = pcVar10 + 0x1;
          if ((*(param_1 + 0x4) & 0x10) != 0x0) {
            sStack26 = iVar6 + sStack26 * 0xa + -0x30;
          }
          iVar12 = iVar12 + 0x1;
          iVar6 = FUN_004b3bd8(param_1);
          pcVar8 = extraout_ECX_00;
          if (iVar6 == -0x1)^ // goto LAB_004b366d;
        } while (((&DAT_004bf9c4)[(byte)((char)iVar6 + 0x1)] & 0x20) != 0x0);
      }
      pcVar11 = pcVar10;
      iVar13 = iVar12;
      if (iVar6 == 0x2e) {
        *pcVar10 = '.';
        pcVar10 = pcVar10 + 0x1;
        iVar6 = FUN_004b3bd8(param_1);
        pcVar8 = extraout_ECX_01;
        if (iVar6 == -0x1)^ // goto LAB_004b366d;
        if ((local_28 == 0x0) && (((&DAT_004bf9c4)[(byte)((char)iVar6 + 0x1)] & 0x20) == 0x0))^ // goto LAB_004b3663;
        iVar12 = iVar12 + 0x1;
        loop {
          if (((&DAT_004bf9c4)[(byte)((char)iVar6 + 0x1)] & 0x20) == 0x0) break;
          iVar12 = iVar12 + 0x1;
          *pcVar10 = (char)iVar6;
          pcVar10 = pcVar10 + 0x1;
          iVar6 = FUN_004b3bd8(param_1);
          pcVar8 = extraout_ECX_02;
        } while (iVar6 != -0x1);
        if ((*(param_1 + 0x4) & 0x10) != 0x0) {
          local_18 = 0x0;
          uStack22 = 0x0;
          pcVar8 = pcVar10;
          while( true ) {
            uStack21 = 0x0;
            pcVar11 = pcVar8 + -0x1;
            pcVar8 = pcVar8 + -0x1;
            if (*pcVar11 == '.') break;
            local_20 = 0xa;
            uVar7 = CONCAT12(*pcVar11 + -0x30,local_18) / 0xa;
            local_18 = (undefined2)uVar7;
            uStack22 = (uVar7 >> 0x10);
          }
          local_1c = local_18;
        }
        pcVar11 = pcVar10;
        iVar13 = iVar12;
        if (iVar6 == -0x1)^ // goto LAB_004b366d;
      }
      pcVar10 = pcVar11;
      iVar12 = iVar13;
      if (((*(param_1 + 0x4) & 0x10) == 0x0) && ((iVar6 == 0x65 || (iVar6 == 0x45)))) {
        iVar12 = iVar13 + 0x1;
        *pcVar11 = (char)iVar6;
        pcVar10 = pcVar11 + 0x1;
        iVar6 = FUN_004b3bd8(param_1);
        pcVar8 = extraout_ECX_03;
        if (iVar6 == -0x1)^ // goto LAB_004b366d;
        if ((iVar6 == 0x2b) || (iVar6 == 0x2d)) {
          iVar12 = iVar13 + 0x2;
          *pcVar10 = (char)iVar6;
          pcVar10 = pcVar11 + 0x2;
          iVar6 = FUN_004b3bd8(param_1);
          pcVar8 = extraout_ECX_04;
          if (iVar6 == -0x1)^ // goto LAB_004b366d;
        }
        if (((&DAT_004bf9c4)[(byte)((char)iVar6 + 0x1)] & 0x20) == 0x0) {
          iVar12 = 0x0;
        }
        else {
          loop {
            iVar12 = iVar12 + 0x1;
            *pcVar10 = (char)iVar6;
            pcVar10 = pcVar10 + 0x1;
            iVar6 = FUN_004b3bd8(param_1);
            pcVar8 = extraout_ECX_05;
            if (iVar6 == -0x1)^ // goto LAB_004b366d;
          } while (((&DAT_004bf9c4)[(byte)((char)iVar6 + 0x1)] & 0x20) != 0x0);
        }
      }
    }
  }
// LAB_004b3663:
  FUN_004b2bfc(iVar6,param_1);
  pcVar8 = extraout_ECX_06;
// LAB_004b366d:
  if ((0x0 < iVar12) && (iVar12 = iVar12 + local_24, (*(param_1 + 0x4) & 0x1) != 0x0)) {
    *pcVar10 = '\0';
    if ((*(param_1 + 0x4) & 0x10) == 0x0) {
      (PTR_FUN_004c034c)(&local_80,&local_30);
      pcVar8 = extraout_ECX_07;
    }
    else {
      if (local_80 == '-') {
        iVar6 = CONCAT22(sStack26,local_1c);
        local_1c = (undefined2)-iVar6;
        sStack26 = (-iVar6 >> 0x10);
      }
    }
    bVar1 = *(param_1 + 0x4);
    if ((bVar1 & 0x4) == 0x0) {
      if ((bVar1 & 0x8) == 0x0) {
        ppuVar5 = (u32 **)*param_2;
        *param_2 = (ppuVar5 + 0x1);
        puVar9 = *ppuVar5;
      }
      else {
        ppuVar5 = (u32 **)*param_2;
        *param_2 = (ppuVar5 + 0x1);
        puVar9 = *ppuVar5;
      }
    }
    else {
      puVar4 = (undefined6 *)*param_2;
      *param_2 = (puVar4 + 0x8);
      puVar9 = *puVar4;
    }
    bVar2 = *(param_1 + 0x4);
    if ((bVar2 & 0x10) == 0x0) {
      if (((bVar2 & 0x20) != 0x0) || ((bVar2 & 0x40) != 0x0)) {
        *puVar9 = local_30;
        puVar9[0x1] = local_2c;
        return iVar12;
      }
      uVar7 = FUN_004b7e8c(pcVar8 & 0xffff0000 | CONCAT11(bVar1,(char)pcVar8),local_2c);
    }
    else {
      uVar7 = CONCAT22(sStack26,local_1c);
    }
    *puVar9 = uVar7;
  }
  return iVar12;
}



// WARNING: Could not reconcile some variable overlaps

fn FUN_004b373c(undefined **param_1,i32 **param_2,param_3: i32,param_4: i32) -> i32

{
  let bVar1: u8;
  let mut puVar2: *mut u8; 
  undefined6 *puVar3;
  i32 **ppiVar4;
  longlong lVar5;
  longlong lVar6;
  let mut iVar7: i32;
  let mut uVar8: u32;
  let piVar9: *mut i32;;
  let mut iVar10: i32;
  let puVar11: *mut u32;
  ulonglong uVar12;
  let mut local_3c: u32;
  let mut local_38: u32;
  let mut local_24: i32;
  let mut local_20: i32;
  let mut local_1c: i32;
  let mut local_18: i32;
  
  iVar10 = 0x0;
  lVar5 = 0x0;
  local_18 = 0x0;
  local_1c = 0x0;
  while( true ) {
    local_24 = FUN_004b2bf0(param_1);
    if (((&DAT_004bf9c4)[(byte)((char)local_24 + 0x1)] & 0x2) == 0x0) break;
    local_1c = local_1c + 0x1;
  }
  lVar6 = 0x0;
  if ((*(param_1 + 0x4) & 0x2) == 0x0) {
    puVar2 = param_1[0x3];
    param_1[0x3] = puVar2 + -0x1;
    if (puVar2 == 0x0) {
// LAB_004b3a46:
      FUN_004b2bfc(local_24,param_1);
      lVar6 = lVar5;
    }
    else {
      local_20 = 0x2b;
      iVar7 = local_24;
      if ((param_4 != 0x0) && ((local_24 == 0x2b || (local_24 == 0x2d)))) {
        local_1c = local_1c + 0x1;
        iVar7 = FUN_004b3bd8(param_1);
        local_20 = local_24;
        lVar6 = 0x0;
        if (iVar7 == -0x1)^ // goto LAB_004b3a58;
      }
      local_24 = iVar7;
      if (param_3 == 0x0) {
        if (local_24 == 0x30) {
          local_24 = FUN_004b3bd8(param_1);
          iVar10 = 0x1;
          lVar6 = 0x0;
          if (local_24 == -0x1)^ // goto LAB_004b3a58;
          if ((local_24 == 0x78) || (local_24 == 0x58)) {
            local_1c = local_1c + 0x2;
            local_24 = FUN_004b3bd8(param_1);
            iVar10 = 0x0;
            lVar6 = 0x0;
            if (local_24 == -0x1)^ // goto LAB_004b3a58;
            param_3 = 0x10;
          }
          else {
            param_3 = 0x8;
          }
        }
        else {
          param_3 = 0xa;
        }
      }
      else {
        if ((param_3 == 0x10) && (local_24 == 0x30)) {
          local_24 = FUN_004b3bd8(param_1);
          iVar10 = 0x1;
          lVar6 = 0x0;
          if (local_24 == -0x1)^ // goto LAB_004b3a58;
          if ((local_24 == 0x78) || (local_24 == 0x58)) {
            local_1c = local_1c + 0x2;
            local_24 = FUN_004b3bd8(param_1);
            iVar10 = 0x0;
            lVar6 = 0x0;
            if (local_24 == -0x1)^ // goto LAB_004b3a58;
          }
        }
      }
      if ((*(param_1 + 0x4) & 0x40) == 0x0) {
        loop {
          iVar7 = FUN_004b3ba8(local_24);
          if (param_3 <= iVar7) {
            if ((local_24 != 0x3a) || ((*(param_1 + 0x4) & 0x80) == 0x0))^ // goto LAB_004b3a46;^
            // goto LAB_004b3a13;
          }
          local_18 = local_18 * param_3 + iVar7;
          local_24 = FUN_004b3bd8(param_1);
          iVar10 = iVar10 + 0x1;
          lVar6 = 0x0;
        } while (local_24 != -0x1);
      }
      else {
        loop {
          local_38 = ((ulonglong)lVar5 >> 0x20);
          uVar8 = FUN_004b3ba8(local_24);
          if (param_3 <= uVar8) {
            if ((local_24 != 0x3a) || ((*(param_1 + 0x4) & 0x80) == 0x0))^ // goto LAB_004b3a46;^
            // goto LAB_004b395e;
          }
          uVar12 = FUN_004b7ed8(0x0,local_38);
          lVar5 = uVar12 + uVar8;
          local_24 = FUN_004b3bd8(param_1);
          iVar10 = iVar10 + 0x1;
          lVar6 = lVar5;
        } while (local_24 != -0x1);
      }
    }
  }
// LAB_004b3a58:
  local_38 = ((ulonglong)lVar6 >> 0x20);
  local_3c = lVar6;
  if ((*(param_1 + 0x4) & 0x40) == 0x0) {
    if (local_20 == 0x2d) {
      local_18 = -local_18;
    }
    if (0x0 < iVar10) {
      bVar1 = *(param_1 + 0x4);
      iVar10 = iVar10 + local_1c;
      if ((bVar1 & 0x1) != 0x0) {
        if ((bVar1 & 0x4) == 0x0) {
          if ((bVar1 & 0x8) == 0x0) {
            ppiVar4 = (i32 **)*param_2;
            *param_2 = (ppiVar4 + 0x1);
            piVar9 = *ppiVar4;
          }
          else {
            ppiVar4 = (i32 **)*param_2;
            *param_2 = (ppiVar4 + 0x1);
            piVar9 = *ppiVar4;
          }
        }
        else {
          puVar3 = (undefined6 *)*param_2;
          *param_2 = (puVar3 + 0x8);
          piVar9 = *puVar3;
        }
        if ((*(param_1 + 0x4) & 0x10) != 0x0) {
          piVar9 = local_18;
          return iVar10;
        }
        *piVar9 = local_18;
      }
    }
  }
  else {
    if (local_20 == 0x2d) {
      local_38 = ~local_38;
      if (~local_3c + 0x1 == 0x0) {
        local_38 = local_38 + 0x1;
      }
      lVar6 = CONCAT44(local_38,~local_3c + 0x1);
    }
    local_38 = ((ulonglong)lVar6 >> 0x20);
    local_3c = lVar6;
    if (0x0 < iVar10) {
      bVar1 = *(param_1 + 0x4);
      iVar10 = iVar10 + local_1c;
      if ((bVar1 & 0x1) != 0x0) {
        if ((bVar1 & 0x4) == 0x0) {
          if ((bVar1 & 0x8) != 0x0) {
            piVar9 = *param_2;
            *param_2 = piVar9 + 0x1;
            puVar11 = *piVar9;
            *puVar11 = local_3c;
            puVar11[0x1] = local_38;
            return iVar10;
          }
          piVar9 = *param_2;
          *param_2 = piVar9 + 0x1;
          puVar11 = *piVar9;
        }
        else {
          piVar9 = *param_2;
          *param_2 = piVar9 + 0x2;
          puVar11 = *piVar9;
        }
        *puVar11 = local_3c;
        puVar11[0x1] = local_38;
        return iVar10;
      }
    }
  }
  return iVar10;
// LAB_004b3a13:
  local_24 = FUN_004b3bd8(param_1);
  iVar10 = iVar10 + 0x1;
  lVar6 = 0x0;
  if (local_24 == -0x1)^ // goto LAB_004b3a58;
  iVar7 = FUN_004b3ba8(local_24);
  if (param_3 <= iVar7)^ // goto LAB_004b3a46;
  local_18 = local_18 * param_3 + iVar7;^
  // goto LAB_004b3a13;
// LAB_004b395e:
  local_38 = ((ulonglong)lVar5 >> 0x20);
  local_24 = FUN_004b3bd8(param_1);
  iVar10 = iVar10 + 0x1;
  lVar6 = lVar5;
  if (local_24 == -0x1)^ // goto LAB_004b3a58;
  uVar8 = FUN_004b3ba8(local_24);
  if (param_3 <= uVar8)^ // goto LAB_004b3a46;
  uVar12 = FUN_004b7ed8(0x0,local_38);
  lVar5 = uVar12 + uVar8;^
  // goto LAB_004b395e;
}



fn FUN_004b3ba8(param_1: i32) -> i32

{
  let mut iVar1: i32;
  
  if ((0x2f < param_1) && (param_1 < 0x3a)) {
    return param_1 + -0x30;
  }
  iVar1 = FUN_004aa9f0(param_1);
  if ((0x60 < iVar1) && (iVar1 < 0x67)) {
    return iVar1 + -0x57;
  }
  return 0x10;
}



fn FUN_004b3bd8(undefined **param_1) -> u32

{
  let mut puVar1: *mut u8; 
  let mut uVar2: u32;
  
  puVar1 = param_1[0x3];
  param_1[0x3] = puVar1 + -0x1;
  if ((puVar1 != 0x0) && (uVar2 = FUN_004b2bf0(param_1), (*(param_1 + 0x4) & 0x2) == 0x0)) {
    return uVar2;
  }
  return 0xffffffff;
}



fn FUN_004b3c10() -> *mut u32

{
  let puVar1: *mut u32;
  let mut uVar2: u32;
  let mut in_EAX: u32;
  let puVar3: *mut u32;
  let mut uVar4: u32;
  let mut unaff_EBX: i32;
  let mut uVar5: u32;
  
  if ((in_EAX != 0x0) && (in_EAX <= in_EAX + 0xb)) {
    uVar4 = in_EAX + 0xb & 0xfffffff8;
    if (uVar4 < 0x10) {
      uVar4 = 0x10;
    }
    if (uVar4 <= *(unaff_EBX + 0x14)) {
      uVar5 = *(unaff_EBX + 0x10);
      puVar3 = (unaff_EBX + 0xc);
      if (uVar4 <= uVar5) {
        puVar3 = (unaff_EBX + 0x28);
        uVar5 = 0x0;
      }
      loop {
        uVar2 = *puVar3;
        if (uVar4 <= uVar2) {
          *(unaff_EBX + 0x10) = uVar5;
          *(unaff_EBX + 0x18) = *(unaff_EBX + 0x18) + 0x1;
          if (uVar2 - uVar4 < 0x10) {
            *(unaff_EBX + 0x1c) = *(unaff_EBX + 0x1c) + -0x1;
            uVar4 = puVar3[0x1];
            *(unaff_EBX + 0xc) = uVar4;
            uVar5 = puVar3[0x2];
            *(uVar4 + 0x8) = uVar5;
            *(uVar5 + 0x4) = uVar4;
          }
          else {
            puVar1 = (puVar3 + uVar4);
            (unaff_EBX + 0xc) = puVar1;
            *puVar1 = uVar2 - uVar4;
            *puVar3 = uVar4;
            uVar4 = puVar3[0x1];
            puVar1[0x1] = uVar4;
            uVar5 = puVar3[0x2];
            puVar1[0x2] = uVar5;
            (uVar4 + 0x8) = puVar1;
            (uVar5 + 0x4) = puVar1;
          }
          *puVar3 = *puVar3 | 0x1;
          return puVar3 + 0x1;
        }
        if (uVar5 < uVar2) {
          uVar5 = uVar2;
        }
        puVar3 = puVar3[0x2];
      } while (puVar3 != (unaff_EBX + 0x20));
      *(unaff_EBX + 0x14) = uVar5;
    }
  }
  return 0x0;
}



fn FUN_004b3cc0()

{
  i32 **ppiVar1;
  let mut uVar2: u32;
  i32 **in_EAX;
  let mut uVar3: u32;
  i32 **ppiVar4;
  let mut iVar5: i32;
  let piVar6: *mut i32;;
  let piVar7: *mut i32;;
  let mut unaff_EBX: i32;
  i32 **ppiVar8;
  
  if (in_EAX == (i32 **)0x0) {
    return;
  }
  ppiVar1 = in_EAX + -0x1;
  if ((*ppiVar1 & 0x1) == 0x0) {
    return;
  }
  piVar7 = (*ppiVar1 & 0xfffffffe);
  piVar6 = (ppiVar1 + piVar7);
  if ((*piVar6 & 0x1) == 0x0) {
    *ppiVar1 = (piVar7 + *piVar6);
    if (piVar6 == *(i32 **)(unaff_EBX + 0xc)) {
      *(i32 ***)(unaff_EBX + 0xc) = ppiVar1;
    }
    piVar7 = piVar6[0x1];
    ppiVar4 = (i32 **)piVar6[0x2];
    piVar7[0x2] = ppiVar4;
    ppiVar4[0x1] = piVar7;
    *(unaff_EBX + 0x1c) = *(unaff_EBX + 0x1c) + -0x1;
  }
  else {
    *ppiVar1 = piVar7;
    ppiVar4 = *(i32 ***)(unaff_EBX + 0xc);
    if (ppiVar1 < ppiVar4) {
      if ((ppiVar4[0x1] <= ppiVar1 && ppiVar1 != (i32 **)ppiVar4[0x1]) ||
         (ppiVar4 = *(i32 ***)(unaff_EBX + 0x28), ppiVar1 < ppiVar4))^ // goto LAB_004b3d9e;
    }
    else {
      ppiVar4 = (i32 **)ppiVar4[0x2];
      if ((ppiVar1 < ppiVar4) || (ppiVar4 = (i32 **)(unaff_EBX + 0x20), *(i32 ***)(unaff_EBX + 0x24) < ppiVar1))^
      // goto LAB_004b3d9e;
    }
    uVar2 = *(unaff_EBX + 0x1c);
    uVar3 = *(unaff_EBX + 0x18) / (uVar2 + 0x1);
    if (uVar3 < uVar2) {
      iVar5 = uVar3 * 0x2;
      if (*(unaff_EBX + 0x18) - uVar2 <= uVar2) {
        iVar5 = -0x1;
      }
      ppiVar4 = (i32 **)(*ppiVar1 + ppiVar1);
      loop {
        piVar6 = *ppiVar4;
        if ((piVar6 & 0x1) == 0x0)^ // goto LAB_004b3d9e;
        if (piVar6 == 0xffffffff) break;
        ppiVar4 = (i32 **)(ppiVar4 + (piVar6 & 0xfffffffe));
        iVar5 = iVar5 + -0x1;
      } while (iVar5 != 0x0);
    }
    ppiVar4 = *(i32 ***)(unaff_EBX + 0xc);
    if (ppiVar1 < ppiVar4) {
      ppiVar4 = *(i32 ***)(unaff_EBX + 0x28);
    }
    while (((ppiVar4 <= ppiVar1 && (ppiVar4 = (i32 **)ppiVar4[0x2], ppiVar4 <= ppiVar1)) &&
           (ppiVar4 = (i32 **)ppiVar4[0x2], ppiVar4 <= ppiVar1))) {
      ppiVar4 = (i32 **)ppiVar4[0x2];
    }
  }
// LAB_004b3d9e:
  ppiVar8 = (i32 **)ppiVar4[0x1];
  piVar6 = *ppiVar1;
  if ((i32 **)(*ppiVar8 + ppiVar8) == ppiVar1) {
    piVar6 = (piVar6 + *ppiVar8);
    *ppiVar8 = piVar6;
    if (ppiVar1 == *(i32 ***)(unaff_EBX + 0xc)) {
      *(i32 ***)(unaff_EBX + 0xc) = ppiVar8;
    }
  }
  else {
    *(unaff_EBX + 0x1c) = *(unaff_EBX + 0x1c) + 0x1;
    in_EAX[0x1] = ppiVar4;
    *in_EAX = ppiVar8;
    ppiVar8[0x2] = ppiVar1;
    ppiVar4[0x1] = ppiVar1;
    ppiVar8 = ppiVar1;
  }
  *(unaff_EBX + 0x18) = *(unaff_EBX + 0x18) + -0x1;
  if ((ppiVar8 < *(i32 ***)(unaff_EBX + 0xc)) &&
     (*(i32 **)(unaff_EBX + 0x10) <= piVar6 && piVar6 != *(i32 **)(unaff_EBX + 0x10))) {
    *(i32 **)(unaff_EBX + 0x10) = piVar6;
  }
  if (*(i32 **)(unaff_EBX + 0x14) <= piVar6 && piVar6 != *(i32 **)(unaff_EBX + 0x14)) {
    *(i32 **)(unaff_EBX + 0x14) = piVar6;
  }
  return;
}



i32 *  FUN_004b3df0(param_1: *mut i32)

{
  let piVar1: *mut i32;;
  let piVar2: *mut i32;;
  let piVar3: *mut i32;;
  
  piVar2 = DAT_004bfae8;
  piVar3 = 0x0;
  if (DAT_004bfae8 != 0x0) {
    loop {
      piVar1 = piVar2;
      piVar2 = piVar1;
      if (param_1 < piVar1) break;
      piVar2 = piVar1[0x2];
      piVar3 = piVar1;
    } while (piVar2 != 0x0);
  }
  param_1[0x1] = piVar3;
  param_1[0x2] = piVar2;
  if (piVar3 == 0x0) {
    DAT_004bfae8 = param_1;
  }
  else {
    piVar3[0x2] = param_1;
  }
  if (piVar2 != 0x0) {
    piVar2[0x1] = param_1;
  }
  piVar2 = param_1 + 0x8;
  piVar3 = param_1 + 0xb;
  param_1[0x8] = 0x0;
  param_1[0x4] = 0x0;
  param_1[0x6] = 0x0;
  param_1[0x7] = 0x0;
  param_1[0x9] = piVar2;
  param_1[0xa] = piVar2;
  param_1[0x3] = piVar2;
  *piVar3 = *param_1 + -0x2c;
  *(piVar3 + *param_1 + -0x2c) = 0xffffffff;
  return piVar3;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 *  alloc_mem_004b3e68(param_1: u32)

{
  let mut bVar1: bool;
  undefined3 extraout_var;
  let puVar2: *mut u32;
  let mut uVar3: u32;
  let puVar4: *mut u32;
  
  if (_DAT_004c035c == 0x0) {
    return 0x0;
  }
  if (_DAT_004c0128 == -0x2) {
    return 0x0;
  }
  bVar1 = FUN_004b3f2c(&param_1);
  puVar2 = CONCAT31(extraout_var,bVar1);
  if (puVar2 != 0x0) {
                    // DWORD flProtect for VirtualAlloc
                    // DWORD flAllocationType for VirtualAlloc
                    // SIZE_T dwSize for VirtualAlloc
                    // LPVOID lpAddress for VirtualAlloc
    puVar2 = VirtualAlloc((LPVOID)0x0,param_1,0x1000,0x40);
    if (puVar2 != 0x0) {
      uVar3 = param_1 - 0x4;
      if (param_1 < uVar3) {
        return 0x0;
      }
      if (uVar3 < 0x38) {
        return 0x0;
      }
      *puVar2 = uVar3;
      param_1 = uVar3;
      puVar4 = FUN_004b3df0(puVar2);
      param_1 = *puVar4;
      *puVar4 = param_1 | 0x1;
      puVar2[0x5] = 0x0;
      puVar2[0x6] = puVar2[0x6] + 0x1;
      FUN_004aaaf0((puVar4 + 0x1));
      puVar2 = 0x1;
    }
  }
  return puVar2;
}



fn FUN_004b3f18(param_1: u32)

{
  FUN_004b08b0();
  alloc_mem_004b3e68(param_1);
  return;
}



bool  FUN_004b3f2c(param_1: *mut u32)

{
  let mut uVar1: u32;
  
  uVar1 = *param_1 + 0x7 & 0xfffffff8;
  if (uVar1 == 0x0) {
    return false;
  }
  *param_1 = uVar1;
  uVar1 = uVar1 + 0x3c;
  if (uVar1 < *param_1) {
    return false;
  }
  if (uVar1 < DAT_004c0360) {
    uVar1 = DAT_004c0360 & 0xfffffffe;
  }
  *param_1 = uVar1;
  uVar1 = uVar1 + 0xfff;
  if (uVar1 < *param_1) {
    return false;
  }
  uVar1 = uVar1 & 0xffff0000 | (uVar1 >> 0x8 & 0xf0) << 0x8;
  *param_1 = uVar1;
  return uVar1 != 0x0;
}



fn FUN_004b3f80() -> u32

{
  return 0x0;
}



fn FUN_004b3f90() -> u32

{
  let mut uVar1: u32;
  let mut iVar2: i32;
  
  if (DAT_004c0194 < DAT_004bffe8) {
    uVar1 = 0x0;
  }
  else {
    if (0x0 < DAT_004c0194) {
      iVar2 = 0x0;
      loop {
        if (*(DAT_004c0190 + iVar2) == 0x0) {
          return 0x0;
        }
        iVar2 = iVar2 + 0x4;
      } while (iVar2 < (DAT_004c0194 * 0x4));
    }
    uVar1 = 0x1;
  }
  return uVar1;
}



fn FUN_004b3fe0(param_1: i32) -> i32

{
  let mut iVar1: i32;
  let mut iVar2: i32;
  
  (PTR_FUN_004bfba8)();
  iVar2 = 0x0;
  if (0x0 < DAT_004c0194) {
    iVar1 = 0x0;
    loop {
      if (*(DAT_004c0190 + iVar1) == 0x0) {
        *(DAT_004c0190 + iVar1) = param_1;
        (PTR_FUN_004bfbac)();
        return iVar2;
      }
      iVar1 = iVar1 + 0x4;
      iVar2 = iVar2 + 0x1;
    } while (iVar1 < DAT_004c0194 * 0x4);
  }
  DAT_004c0190 = FUN_004b7f20(DAT_004c0190,(DAT_004c0194 + 0x1) * 0x4);
  iVar2 = DAT_004c0194 * 0x4;
  DAT_004c0194 = DAT_004c0194 + 0x1;
  *(DAT_004c0190 + iVar2) = param_1;
  (PTR_FUN_004bfbac)();
  return DAT_004c0194 + -0x1;
}



void  set_std_handle_004b406c(HANDLE param_1,param_2: u32)

{
  let mut iVar1: i32;
  let nStdHandle: u32;
  
  if (param_2 < 0x0) {
    return;
  }
  (PTR_FUN_004bfba8)();
  if (param_2 == 0x0) {
    nStdHandle = 0xfffffff6;
  }
  else {
    if (param_2 < 0x2) {
      nStdHandle = 0xfffffff5;
    }
    else {
      if (param_2 != 0x2)^ // goto LAB_004b40ba;
                    // HANDLE hHandle for SetStdHandle
                    // DWORD nStdHandle for SetStdHandle
      nStdHandle = 0xfffffff4;
    }
  }
  SetStdHandle(nStdHandle,param_1);
// LAB_004b40ba:
  if (param_2 < DAT_004c0194) {
    DAT_004c0190[param_2] = param_1;
  }
  else {
    DAT_004c0190 = FUN_004b7f20(DAT_004c0190,param_2 * 0x4 + 0x4);
    if (DAT_004c0194 < param_2) {
      iVar1 = DAT_004c0194 * 0x4;
      loop {
        *(DAT_004c0190 + iVar1) = 0x0;
        iVar1 = iVar1 + 0x4;
      } while (iVar1 < (param_2 * 0x4));
    }
    DAT_004c0194 = param_2 + 0x1;
    DAT_004c0190[param_2] = param_1;
  }
  (PTR_FUN_004bfbac)();
  return;
}



fn FUN_004b4144(param_1: i32)

{
  (PTR_FUN_004bfba8)();
  if ((0x0 < param_1) && (param_1 < DAT_004c0194)) {
    *(DAT_004c0190 + param_1 * 0x4) = 0x0;
  }
  (PTR_FUN_004bfbac)();
  return;
}



void get_std_handles_004b4170(void)

{
  HANDLE pvVar1;
  
                    // DWORD nStdHandle for GetStdHandle
  pvVar1 = GetStdHandle(0xfffffff6);
  if ((pvVar1 == (HANDLE)0x0) || (pvVar1 == (HANDLE)0xffffffff)) {
    pvVar1 = create_event_fn_004b41d8();
  }
  FUN_004b3fe0(pvVar1);
                    // DWORD nStdHandle for GetStdHandle
  pvVar1 = GetStdHandle(0xfffffff5);
  if ((pvVar1 == (HANDLE)0x0) || (pvVar1 == (HANDLE)0xffffffff)) {
    pvVar1 = create_event_fn_004b41d8();
  }
  FUN_004b3fe0(pvVar1);
                    // DWORD nStdHandle for GetStdHandle
  pvVar1 = GetStdHandle(0xfffffff4);
  if ((pvVar1 == (HANDLE)0x0) || (pvVar1 == (HANDLE)0xffffffff)) {
    pvVar1 = create_event_fn_004b41d8();
  }
  FUN_004b3fe0(pvVar1);
  return;
}



HANDLE create_event_fn_004b41d8(void)

{
  HANDLE event_handle;
  
  (PTR_FUN_004bfba8)();
                    // LPCSTR lpName for CreateEventA
                    // BOOL bInitialState for CreateEventA
                    // BOOL bManualReset for CreateEventA
                    // LPSECURITY_ATTRIBUTES lpEventAttributes for CreateEventA
  event_handle = CreateEventA((LPSECURITY_ATTRIBUTES)0x0,0x0,0x0,0x0);
  if (event_handle == (HANDLE)0x0) {
    event_handle = (HANDLE)(DAT_004c01a0 + 0x1);
    DAT_004c01a0 = event_handle;
  }
  else {
    DAT_004c0198 = FUN_004b7f20(DAT_004c0198,(DAT_004c019c + 0x1) * 0x4);
    DAT_004c0198[DAT_004c019c] = event_handle;
    DAT_004c019c = DAT_004c019c + 0x1;
  }
  (PTR_FUN_004bfbac)();
  return event_handle;
}



fn FUN_004b4244()

{
  let mut iVar1: i32;
  let mut iVar2: i32;
  
  if (DAT_004c0190 != 0x0) {
    FUN_004aaae0(DAT_004c0190);
    DAT_004c0190 = 0x0;
  }
  if (DAT_004c0198 != 0x0) {
    iVar1 = 0x0;
    if (0x0 < DAT_004c019c) {
      iVar2 = 0x0;
      loop {
                    // HANDLE hObject for CloseHandle
        iVar1 = iVar1 + 0x1;
        CloseHandle(*(HANDLE *)(DAT_004c0198 + iVar2));
        iVar2 = iVar2 + 0x4;
      } while (iVar1 < DAT_004c019c);
    }
    FUN_004aaae0(DAT_004c0198);
    DAT_004c0198 = 0x0;
  }
  return;
}



// WARNING: Type propagation algorithm not settling

fn FUN_004b42c0(param_1: u32,byte *param_2,param_3: *mut *mut u32,param_4: *mut u8) -> u32

{
  let bVar1: u8;
  undefined6 *puVar2;
  let puVar3: *mut u32;
  let uVar4: u16;
  byte *pbVar5;
  void *this;
  ushort *puVar6;
  undefined8 uVar7;
  let mut local_88: u32;
  let mut local_84: i32;
  let mut local_78: u32;
  let local_73: u8;
  let local_72: u8;
  let local_6c: u16;
  ushort local_6a;
  let mut local_68: i32;
  let mut local_64: i32;
  let mut local_60: i32;
  let mut local_5c: i32;
  let mut local_58: i32;
  let mut local_54: i32;
  ushort local_50 [0x14];
  let local_28: *mut u32;
  let local_24: *mut u32;
  ushort *local_20;
  let mut local_1c: u32;
  let local_18: u8;
  
  local_18 = 0x0;
  local_6a = 0x0;
  local_6c = 0x64;
  local_78 = 0x0;
  bVar1 = *param_2;
  local_88 = param_1;
  loop {
    if (bVar1 == 0x0) {
      return local_78;
    }
// LAB_004b4310:
    while( true ) {
      bVar1 = *param_2;
      param_2 = param_2 + 0x1;
      if (bVar1 == 0x25) break;
      (param_4)(&local_88,bVar1);
// LAB_004b46b1:
      if (*param_2 == 0x0) {
        return local_78;
      }
    }
    local_28 = *param_3;
    pbVar5 = FUN_004b46e8(param_2,(i32 **)&local_28,&local_88);
    *param_3 = local_28;
    local_73 = *pbVar5;
    param_2 = pbVar5 + 0x1;
    if (local_73 == 0x0) {
      return local_78;
    }
    if (local_73 != 0x6e) {
      local_24 = *param_3;
      uVar7 = FUN_004b4bc4(this,local_50,&local_24,&local_88);
      local_1c = ((ulonglong)uVar7 >> 0x20);
      puVar6 = uVar7;
      *param_3 = local_24;
      local_84 = local_84 - (local_68 + local_64 + local_60 + local_5c + local_58 + local_54);
      if (((local_6a & 0x8) == 0x0) && (local_72 == ' ')) {
        for (; 0x0 < local_84; local_84 = local_84 + -0x1) {
          (param_4)(&local_88,0x20);
        }
      }
      local_20 = local_50;
      for (; 0x0 < local_68; local_68 = local_68 + -0x1) {
        (param_4)(&local_88,local_20);
        local_20 = (local_20 + 0x1);
      }
      while( true ) {
        if (local_64 < 0x1) break;
        (param_4)(&local_88,0x30);
        local_64 = local_64 + -0x1;
      }
      if (local_73 == 0x73) {
        if ((local_6a & 0x20) == 0x0) {
          for (; 0x0 < local_60; local_60 = local_60 + -0x1) {
            (param_4)(&local_88,puVar6);
            puVar6 = (puVar6 + 0x1);
          }
        }
        else {
          FUN_004b4b4c(puVar6,local_1c & 0xffff,&local_88,param_4);
        }
      }
      else {
        if (local_73 == 0x53) {
          FUN_004b4b4c(puVar6,local_1c & 0xffff,&local_88,param_4);
        }
        else {
          for (; 0x0 < local_60; local_60 = local_60 + -0x1) {
            (param_4)(&local_88,puVar6);
            puVar6 = (puVar6 + 0x1);
          }
        }
      }
      for (; 0x0 < local_5c; local_5c = local_5c + -0x1) {
        (param_4)(&local_88,0x30);
      }
      for (; 0x0 < local_58; local_58 = local_58 + -0x1) {
        (param_4)(&local_88,puVar6);
        puVar6 = (puVar6 + 0x1);
      }
      for (; 0x0 < local_54; local_54 = local_54 + -0x1) {
        (param_4)(&local_88,0x30);
      }
      if ((local_6a & 0x8) != 0x0) {
        for (; 0x0 < local_84; local_84 = local_84 + -0x1) {
          (param_4)(&local_88,0x20);
        }
      }^
      // goto LAB_004b46b1;
    }
    if ((local_6a & 0x20) == 0x0) {
      if ((local_6a & 0x10) == 0x0) {
        if ((local_6a & 0x80) == 0x0) {
          if ((local_6a & 0x40) == 0x0) {
            puVar3 = *param_3;
            *param_3 = puVar3 + 0x1;
            **puVar3 = local_78;
            if (*param_2 == 0x0) {
              return local_78;
            }
          }
          else {
            puVar3 = *param_3;
            *param_3 = puVar3 + 0x1;
            **puVar3 = local_78;
            if (*param_2 == 0x0) {
              return local_78;
            }
          }
        }
        else {
          puVar2 = (undefined6 *)*param_3;
          *param_3 = (puVar2 + 0x8);
          **puVar2 = local_78;
          if (*param_2 == 0x0) {
            return local_78;
          }
        }
      }
      else {
        uVar4 = (undefined2)local_78;
        if ((local_6a & 0x80) == 0x0) {
          if ((local_6a & 0x40) == 0x0) {
            puVar3 = *param_3;
            *param_3 = puVar3 + 0x1;
            **puVar3 = uVar4;
            if (*param_2 == 0x0) {
              return local_78;
            }
          }
          else {
            puVar3 = *param_3;
            *param_3 = puVar3 + 0x1;
            **puVar3 = uVar4;
            if (*param_2 == 0x0) {
              return local_78;
            }
          }
        }
        else {
          puVar2 = (undefined6 *)*param_3;
          *param_3 = (puVar2 + 0x8);
          **puVar2 = uVar4;
          if (*param_2 == 0x0) {
            return local_78;
          }
        }
      }^
      // goto LAB_004b4310;
    }
    if ((local_6a & 0x80) == 0x0) {
      if ((local_6a & 0x40) == 0x0) {
        puVar3 = *param_3;
        *param_3 = puVar3 + 0x1;
        **puVar3 = local_78;
        bVar1 = *param_2;
      }
      else {
        puVar3 = *param_3;
        *param_3 = puVar3 + 0x1;
        **puVar3 = local_78;
        bVar1 = *param_2;
      }
    }
    else {
      puVar2 = (undefined6 *)*param_3;
      *param_3 = (puVar2 + 0x8);
      **puVar2 = local_78;
      bVar1 = *param_2;
    }
  } while( true );
}



byte *  FUN_004b46e8(param_1: &mut String,i32 **param_2,param_3: i32)

{
  let bVar1: u8;
  let piVar2: *mut i32;;
  let mut iVar3: i32;
  byte *pbVar4;
  byte *pbVar5;
  
  (param_3 + 0x16) = 0x20;
  pbVar4 = FUN_004b484c(param_1,param_3);
  *(param_3 + 0x4) = 0x0;
  if (*pbVar4 == 0x2a) {
    piVar2 = *param_2;
    *param_2 = piVar2 + 0x1;
    iVar3 = *piVar2;
    *(param_3 + 0x4) = iVar3;
    if (iVar3 < 0x0) {
      *(param_3 + 0x4) = -iVar3;
      *(param_3 + 0x1e) = *(param_3 + 0x1e) | 0x8;
    }
    pbVar4 = pbVar4 + 0x1;
  }
  else {
    for (; (0x2f < *pbVar4 && (*pbVar4 < 0x3a)); pbVar4 = pbVar4 + 0x1) {
      *(param_3 + 0x4) = *(param_3 + 0x4) * 0xa + (*pbVar4 - 0x30);
    }
  }
  *(param_3 + 0x8) = 0xffffffff;
  if (*pbVar4 == 0x2e) {
    *(param_3 + 0x8) = 0x0;
    if (pbVar4[0x1] == 0x2a) {
      piVar2 = *param_2;
      *param_2 = piVar2 + 0x1;
      iVar3 = *piVar2;
      *(param_3 + 0x8) = iVar3;
      if (iVar3 < 0x0) {
        *(param_3 + 0x8) = 0xffffffff;
      }
      pbVar4 = pbVar4 + 0x2;
    }
    else {
      while( true ) {
        pbVar4 = pbVar4 + 0x1;
        if ((*pbVar4 < 0x30) || (0x39 < *pbVar4)) break;
        *(param_3 + 0x8) = *(param_3 + 0x8) * 0xa + (*pbVar4 - 0x30);
      }
    }
    if (*(param_3 + 0x8) != -0x1) {
      (param_3 + 0x16) = 0x20;
    }
  }
  bVar1 = *pbVar4;
  pbVar5 = pbVar4 + 0x1;
  if (bVar1 < 0x4e) {
    if (bVar1 < 0x49) {
      if (bVar1 == 0x46) {
        *(param_3 + 0x1e) = *(param_3 + 0x1e) | 0x80;
        return pbVar5;
      }
      return pbVar4;
    }
    if (0x49 < bVar1) {
      if (bVar1 == 0x4c) {
        *(param_3 + 0x1f) = *(param_3 + 0x1f) | 0x1;
        return pbVar5;
      }
      return pbVar4;
    }
    pbVar5 = pbVar4;
    if ((pbVar4[0x1] == 0x36) && (pbVar4[0x2] == 0x34)) {
      *(param_3 + 0x1f) = *(param_3 + 0x1f) | 0x1;
      return pbVar4 + 0x3;
    }
  }
  else {
    if (bVar1 < 0x4f) {
      *(param_3 + 0x1e) = *(param_3 + 0x1e) | 0x40;
    }
    else {
      if (0x6b < bVar1) {
        if ((0x6c < bVar1) && (bVar1 != 0x77)) {
          return pbVar4;
        }
        *(param_3 + 0x1e) = *(param_3 + 0x1e) | 0x20;
        return pbVar4 + 0x1;
      }
      if (bVar1 != 0x68) {
        return pbVar4;
      }
      *(param_3 + 0x1e) = *(param_3 + 0x1e) | 0x10;
    }
  }
  return pbVar5;
}



char *  FUN_004b484c(param_1: &mut String,param_2: i32)

{
  let bVar1: u8;
  let cVar2: u8;
  
  *(param_2 + 0x1e) = 0x0;
  cVar2 = *param_1;
  if (cVar2 != '-')^ // goto LAB_004b4868;
  *(param_2 + 0x1e) = *(param_2 + 0x1e) | 0x8;
  while( true ) {
    while( true ) {
      while( true ) {
        while( true ) {
          while( true ) {
            param_1 = param_1 + 0x1;
            cVar2 = *param_1;
            if (cVar2 != '-') break;
            *(param_2 + 0x1e) = *(param_2 + 0x1e) | 0x8;
          }
// LAB_004b4868:
          if (cVar2 != '#') break;
          *(param_2 + 0x1e) = *(param_2 + 0x1e) | 0x1;
        }
        if (cVar2 != '+') break;
        bVar1 = *(param_2 + 0x1e);
        *(param_2 + 0x1e) = bVar1 | 0x4;
        *(param_2 + 0x1e) = bVar1 & 0xfd | 0x4;
      }
      if (cVar2 != ' ') break;
      if ((*(param_2 + 0x1e) & 0x4) == 0x0) {
        *(param_2 + 0x1e) = *(param_2 + 0x1e) | 0x2;
      }
    }
    if (cVar2 != '0') break;
    (param_2 + 0x16) = 0x30;
  }
  return param_1;
}



fn FUN_004b48bc(param_1: &mut String,param_2: u32,param_3: i32)

{
  let cVar1: u8;
  let mut iVar2: i32;
  
  for (iVar2 = 0x0; (cVar1 = *param_1, param_1 = param_1 + 0x1, cVar1 != '\0' && (iVar2 != param_3));
      iVar2 = iVar2 + 0x1) {
  }
  return;
}



fn FUN_004b48ec(ushort *param_1,param_2: u32,param_3: i32) -> i32

{
  ushort uVar1;
  let DVar2: u32;
  let mut iVar3: i32;
  let local_14: u8 [0x8];
  
  iVar3 = 0x0;
  if (param_3 == -0x1) {
    while (uVar1 = *param_1, uVar1 != 0x0) {
      param_1 = param_1 + 0x1;
      DVar2 = conv_pwchar_to_pchar_004b7fd0(local_14,(LPWSTR)uVar1);
      if (DVar2 != 0xffffffff) {
        iVar3 = iVar3 + DVar2;
      }
    }
  }
  else {
    while ((uVar1 = *param_1, uVar1 != 0x0 && (iVar3 <= param_3))) {
      param_1 = param_1 + 0x1;
      DVar2 = conv_pwchar_to_pchar_004b7fd0(local_14,(LPWSTR)uVar1);
      if (DVar2 != 0xffffffff) {
        iVar3 = iVar3 + DVar2;
      }
    }
    if (param_3 < iVar3) {
      return param_3;
    }
  }
  return iVar3;
}



fn FUN_004b4968(param_1: u32,param_2: &mut String,param_3: i32)

{
  let mut pcVar1: String; 
  let cVar2: u8;
  let mut iVar3: i32;
  let mut uVar4: u32;
  let mut pcVar5: String; 
  let mut pcVar6: String; 
  
  FUN_004b8068(param_1,param_2,0x10);
  uVar4 = 0xffffffff;
  pcVar5 = param_2;
  loop {
    if (uVar4 == 0x0) break;
    uVar4 = uVar4 - 0x1;
    cVar2 = *pcVar5;
    pcVar5 = pcVar5 + 0x1;
  } while (cVar2 != '\0');
  iVar3 = param_3 + -0x1;
  if (~uVar4 - 0x1 != 0x0) {
    pcVar5 = param_2 + (~uVar4 - 0x1);
    pcVar6 = param_2 + iVar3;
    loop {
      pcVar1 = pcVar5 + -0x1;
      pcVar5 = pcVar5 + -0x1;
      iVar3 = iVar3 + -0x1;
      *pcVar6 = *pcVar1;
      pcVar6 = pcVar6 + -0x1;
    } while (pcVar5 != param_2);
  }
  pcVar5 = param_2 + iVar3;
  for (; -0x1 < iVar3; iVar3 = iVar3 + -0x1) {
    *pcVar5 = '0';
    pcVar5 = pcVar5 + -0x1;
  }
  param_2[param_3] = '\0';
  return;
}



// WARNING: Could not reconcile some variable overlaps

fn FUN_004b49cc(param_1: &mut String,param_2: u32,param_3: i32)

{
  let cVar1: u8;
  let mut iVar2: i32;
  let mut pcVar3: String; 
  let mut pcVar4: String; 
  let mut local_14: u32;
  
  local_14 = param_2;
  if (param_2 < 0x0) {
    *param_1 = '-';
    local_14 = -param_2;
    param_1 = param_1 + 0x1;
  }
  if (*(param_3 + 0x8) == -0x1) {
    *(param_3 + 0x8) = 0x4;
  }
  FUN_004b8068(local_14 >> 0x10,param_1,0xa);
  cVar1 = *param_1;
  pcVar3 = param_1;
  while (cVar1 != '\0') {
    pcVar4 = pcVar3 + 0x1;
    pcVar3 = pcVar3 + 0x1;
    cVar1 = *pcVar4;
  }
  if (*(param_3 + 0x8) != 0x0) {
    *pcVar3 = '.';
    iVar2 = 0x0;
    pcVar3 = pcVar3 + 0x1;
    if (0x0 < *(param_3 + 0x8)) {
      loop {
        local_14 = local_14 & 0xffff;
        local_14 = local_14 * 0xa;
        local_14._2_1_ = (char)(local_14 >> 0x10);
        *pcVar3 = local_14._2_1_ + '0';
        iVar2 = iVar2 + 0x1;
        pcVar3 = pcVar3 + 0x1;
      } while (iVar2 < *(param_3 + 0x8));
    }
    *pcVar3 = '\0';
  }
  if ((local_14 & 0x8000) != 0x0) {
    while (pcVar3 != param_1) {
      pcVar4 = pcVar3 + -0x1;
      if (pcVar3[-0x1] == '.') {
        pcVar4 = pcVar3 + -0x2;
      }
      if (*pcVar4 != '9') {
        *pcVar4 = *pcVar4 + '\x01';
        return;
      }
      *pcVar4 = '0';
      pcVar3 = pcVar4;
    }
    pcVar3 = param_1 + 0x1;
    *param_1 = '1';
    cVar1 = *pcVar3;
    while (cVar1 == '0') {
      pcVar4 = pcVar3 + 0x1;
      pcVar3 = pcVar3 + 0x1;
      cVar1 = *pcVar4;
    }
    if (*pcVar3 == '.') {
      *pcVar3 = '0';
      pcVar3[0x1] = '.';
      pcVar4 = pcVar3 + 0x2;
      pcVar3 = pcVar3 + 0x2;
      cVar1 = *pcVar4;
      while (cVar1 == '0') {
        pcVar4 = pcVar3 + 0x1;
        pcVar3 = pcVar3 + 0x1;
        cVar1 = *pcVar4;
      }
    }
    *pcVar3 = '0';
    pcVar3[0x1] = '\0';
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_004b4ad0(param_1: u32,param_2: u32,param_3: i32)

{
  let uVar1: u8;
  
  uVar1 = (param_3 + 0x17);
  (param_3 + 0x17) = 0xff;
  (param_3 + 0x14) = (param_3 + 0x1e);
  (*_DAT_004c0348)(param_1,param_2,param_3);
  (param_3 + 0x17) = uVar1;
  return;
}



fn FUN_004b4b08(param_1: i32)

{
  let mut iVar1: i32;
  
  if (((*(param_1 + 0x1e) & 0x8) == 0x0) && ((param_1 + 0x16) == '0')) {
    iVar1 = (((((*(param_1 + 0x4) - *(param_1 + 0x20)) - *(param_1 + 0x24)) -
              *(param_1 + 0x28)) - *(param_1 + 0x2c)) - *(param_1 + 0x30)) -
            *(param_1 + 0x34);
    if (0x0 < iVar1) {
      *(param_1 + 0x24) = *(param_1 + 0x24) + iVar1;
    }
  }
  return;
}



fn FUN_004b4b4c(ushort *param_1,param_2: u32,param_3: i32,param_4: *mut u8)

{
  ushort uVar1;
  let mut iVar2: i32;
  let DVar3: u32;
  let mut pCVar4: String; 
  let local_18: u8 [0x4];
  
  iVar2 = *(param_3 + 0x28);
  loop {
    if (iVar2 < 0x1) {
      return;
    }
    pCVar4 = local_18;
    uVar1 = *param_1;
    param_1 = param_1 + 0x1;
    DVar3 = conv_pwchar_to_pchar_004b7fd0(local_18,(LPWSTR)uVar1);
    if (DVar3 != 0xffffffff) {
      if (*(param_3 + 0x28) < DVar3) {
        *(param_3 + 0x28) = 0x0;
        return;
      }
      while (DVar3 = DVar3 - 0x1, DVar3 != 0xffffffff) {
        (param_4)(param_3,*pCVar4);
        pCVar4 = pCVar4 + 0x1;
        *(param_3 + 0x28) = *(param_3 + 0x28) + -0x1;
      }
    }
    iVar2 = *(param_3 + 0x28);
  } while( true );
}



undefined8 __thiscall FUN_004b4bc4(void *this,ushort *param_1,param_2: *mut *mut u32,param_3: i32)

{
  let bVar1: u8;
  u32 **ppuVar2;
  let puVar3: *mut u32;
  ushort **ppuVar4;
  let mut iVar5: i32;
  let mut uVar6: u32;
  let mut in_EAX: u32;
  ushort *puVar7;
  let mut iVar8: i32;
  let mut uVar9: u32;
  let DVar10: u32;
  let puVar11: *mut u32;
  let mut uVar12: u32;
  ushort in_DS;
  let mut local_24: u32;
  let local_20: *mut u32;
  let local_1c: u8;
  let local_1b: u8;
  let mut local_18: u32;
  
  *(param_3 + 0x20) = 0x0;
  *(param_3 + 0x24) = 0x0;
  *(param_3 + 0x28) = 0x0;
  *(param_3 + 0x2c) = 0x0;
  uVar6 = local_18 & 0xffff0000;
  local_18 = uVar6 | in_DS;
  *(param_3 + 0x30) = 0x0;
  bVar1 = *(param_3 + 0x15);
  puVar11 = (in_EAX & 0xffffff00);
  *(param_3 + 0x34) = 0x0;
  if (bVar1 < 0x69) {
    if (0x57 < bVar1) {
      if (bVar1 < 0x59)^ // goto LAB_004b4c40;
      if (bVar1 == 0x64)^ // goto LAB_004b4c9a;
    }^
    // goto LAB_004b4d61;
  }
  if (bVar1 < 0x6a) {
// LAB_004b4c9a:
    if ((*(param_3 + 0x1f) & 0x1) == 0x0) {
      if ((*(param_3 + 0x1e) & 0x20) == 0x0) {
        ppuVar2 = (u32 **)*param_2;
        *param_2 = (ppuVar2 + 0x1);
        this = *ppuVar2;
        if ((*(param_3 + 0x1e) & 0x10) != 0x0) {
          this = (void *)this;
        }
      }
      else {
        ppuVar2 = (u32 **)*param_2;
        *param_2 = (ppuVar2 + 0x1);
        this = *ppuVar2;
      }
    }
    else {
      puVar11 = *param_2;
      *param_2 = puVar11 + 0x1;
      local_24 = *puVar11;
      ppuVar2 = (u32 **)*param_2;
      *param_2 = (ppuVar2 + 0x1);
      local_20 = *ppuVar2;
    }
    if ((*(param_3 + 0x1f) & 0x1) == 0x0) {
      if (-0x1 < this)^ // goto LAB_004b4d38;
    }
    else {
      if ((local_20 & 0x80000000) == 0x0) {
// LAB_004b4d38:
        puVar11 = (*(param_3 + 0x1e) << 0x8);
        if ((puVar11 & 0x400) == 0x0) {
          if ((*(param_3 + 0x1e) & 0x2) != 0x0) {
            puVar11 = (param_3 + 0x20);
            *(param_3 + 0x20) = puVar11 + 0x1;
            (param_1 + puVar11) = 0x20;
          }
        }
        else {
          puVar11 = (param_3 + 0x20);
          *(param_3 + 0x20) = puVar11 + 0x1;
          (param_1 + puVar11) = 0x2b;
        }^
        // goto LAB_004b4d61;
      }
    }
    puVar11 = (param_3 + 0x20);
    *(param_3 + 0x20) = puVar11 + 0x1;
    (param_1 + puVar11) = 0x2d;
    if ((*(param_3 + 0x1f) & 0x1) == 0x0) {
      this = (void *)-this;
    }
    else {
      puVar11 = ~local_20;
      local_24 = ~local_24 + 0x1;
      local_20 = puVar11;
      if (local_24 == 0x0) {
        puVar11 = (puVar11 + 0x1);
        local_20 = puVar11;
      }
    }
  }
  else {
    if (bVar1 < 0x75) {
      if (bVar1 == 0x6f)^ // goto LAB_004b4c40;
    }
    else {
      if ((bVar1 < 0x76) || (bVar1 == 0x78)) {
// LAB_004b4c40:
        if ((*(param_3 + 0x1f) & 0x1) == 0x0) {
          if ((*(param_3 + 0x1e) & 0x20) == 0x0) {
            puVar3 = *param_2;
            puVar11 = puVar3 + 0x1;
            *param_2 = puVar11;
            this = (void *)*puVar3;
            if ((*(param_3 + 0x1e) & 0x10) != 0x0) {
              this = (void *)(this & 0xffff);
            }
          }
          else {
            ppuVar2 = (u32 **)*param_2;
            *param_2 = (ppuVar2 + 0x1);
            this = *ppuVar2;
          }
        }
        else {
          puVar11 = *param_2;
          *param_2 = puVar11 + 0x1;
          local_24 = *puVar11;
          ppuVar2 = (u32 **)*param_2;
          *param_2 = (ppuVar2 + 0x1);
          puVar11 = *ppuVar2;
          local_20 = puVar11;
        }
      }
    }
  }
// LAB_004b4d61:
  bVar1 = *(param_3 + 0x15);
  uVar12 = 0xa;
  if (bVar1 < 0x64) {
    if (bVar1 < 0x47) {
      if (0x44 < bVar1) {
        if (bVar1 < 0x46)^ // goto LAB_004b4e55;
// LAB_004b4e1f:
        if ((*(param_3 + 0x1e) & 0x10) == 0x0) {
// LAB_004b4e55:
          FUN_004b4ad0(param_1,param_2,param_3);
          FUN_004b4b08(param_3);
          local_18 = local_18 & 0xffff0000 | in_DS;
          return CONCAT44(local_18,param_1 + 0x1);
        }
        puVar11 = *param_2;
        *param_2 = puVar11 + 0x1;
        FUN_004b49cc(param_1,*puVar11,param_3);
        uVar9 = FUN_004b48bc(param_1,in_DS,-0x1);
        *(param_3 + 0x28) = uVar9;^
        // goto LAB_004b5219;
      }
      if (bVar1 == 0x43) {
        puVar11 = *param_2;
        *param_2 = puVar11 + 0x1;
        DVar10 = conv_pwchar_to_pchar_004b7fd0((LPSTR)param_1,(LPWSTR)puVar11);
        if (DVar10 == 0xffffffff) {
          *(param_3 + 0x20) = 0x0;
          return CONCAT44(local_18,param_1);
        }
        *(DWORD *)(param_3 + 0x20) = DVar10;
        return CONCAT44(local_18,param_1);
      }
    }
    else {
      if (bVar1 < 0x48)^ // goto LAB_004b4e55;
      if (bVar1 < 0x53) {
        if (bVar1 == 0x50) {
// LAB_004b508e:
          if (*(param_3 + 0x4) == 0x0) {
            if ((*(param_3 + 0x1e) & 0x80) == 0x0) {
              *(param_3 + 0x4) = 0x8;
            }
            else {
              *(param_3 + 0x4) = 0xd;
            }
          }
          *(param_3 + 0x1e) = *(param_3 + 0x1e) & 0xf9;
          puVar11 = *param_2;
          *param_2 = puVar11 + 0x1;
          uVar6 = *puVar11;
          puVar7 = param_1;
          if ((*(param_3 + 0x1e) & 0x80) != 0x0) {
            *param_2 = puVar11 + 0x2;
            FUN_004b4968(puVar11[0x1] & 0xffff,param_1,0x4);
            (param_1 + 0x2) = 0x3a;
            puVar7 = (param_1 + 0x5);
          }
          FUN_004b4968(uVar6,puVar7,0x8);
          if ((param_3 + 0x15) == 'P') {
            FUN_004b5228(param_1);
          }
          uVar9 = FUN_004b48bc(param_1,local_18 & 0xffff,-0x1);
          *(param_3 + 0x20) = uVar9;
          return CONCAT44(local_18,param_1);
        }
      }
      else {
        if (bVar1 < 0x54) {
// LAB_004b4e7f:
          param_1 = 0x0;
          if ((*(param_3 + 0x1e) & 0x80) == 0x0) {
            if ((*(param_3 + 0x1e) & 0x40) == 0x0) {
              ppuVar4 = (ushort **)*param_2;
              *param_2 = (ppuVar4 + 0x1);
              puVar7 = *ppuVar4;
            }
            else {
              ppuVar4 = (ushort **)*param_2;
              *param_2 = (ppuVar4 + 0x1);
              puVar7 = *ppuVar4;
            }
            if (puVar7 != 0x0) {
              local_18 = uVar6 | in_DS;
              param_1 = puVar7;
            }
          }
          else {
            ppuVar4 = (ushort **)*param_2;
            *param_2 = (ppuVar4 + 0x2);
            if ((*ppuVar4 != 0x0) || ((ppuVar4 + 0x1) != 0x0)) {
              param_1 = *ppuVar4;
              local_18 = puVar11 & 0xffff0000 | (ppuVar4 + 0x1);
            }
          }
          if ((param_3 + 0x15) == 'S') {
            if ((*(param_3 + 0x1e) & 0x10) == 0x0) {
              iVar8 = FUN_004b48ec(param_1,local_18 & 0xffff,*(param_3 + 0x8));
            }
            else {
              iVar8 = *(param_3 + 0x8);
// LAB_004b4f1a:
              iVar8 = FUN_004b48bc(param_1,local_18 & 0xffff,iVar8);
            }
          }
          else {
            if ((*(param_3 + 0x1e) & 0x20) == 0x0) {
              iVar8 = *(param_3 + 0x8);^
              // goto LAB_004b4f1a;
            }
            iVar8 = FUN_004b48ec(param_1,local_18 & 0xffff,*(param_3 + 0x8));
          }
          iVar5 = *(param_3 + 0x8);
          *(param_3 + 0x28) = iVar8;
          if ((-0x1 < iVar5) && (iVar5 < iVar8)) {
            *(param_3 + 0x28) = iVar5;
            return CONCAT44(local_18,param_1);
          }^
          // goto LAB_004b5219;
        }
        if (0x57 < bVar1) {
          if (bVar1 < 0x59) {
// LAB_004b4f51:
            if (((*(param_3 + 0x1e) & 0x1) != 0x0) &&
               (((puVar11 = this, (*(param_3 + 0x1f) & 0x1) != 0x0 &&
                 (puVar11 = local_20, local_24 != 0x0)) || (puVar11 != 0x0)))) {
              iVar8 = *(param_3 + 0x20);
              *(param_3 + 0x20) = iVar8 + 0x1;
              (param_1 + iVar8) = 0x30;
              iVar8 = *(param_3 + 0x20);
              *(param_3 + 0x20) = iVar8 + 0x1;
              (param_1 + iVar8) = (param_3 + 0x15);
            }
            uVar12 = 0x10;^
            // goto LAB_004b4f93;
          }
          if (bVar1 == 0x63) {
            *(param_3 + 0x20) = 0x1;
            if ((*(param_3 + 0x1e) & 0x20) == 0x0) {
              puVar11 = *param_2;
              *param_2 = puVar11 + 0x1;
              param_1 = puVar11;
              return CONCAT44(local_18,param_1);
            }
            puVar11 = *param_2;
            *param_2 = puVar11 + 0x1;
            DVar10 = conv_pwchar_to_pchar_004b7fd0((LPSTR)&local_1c,(LPWSTR)puVar11);
            iVar8 = DAT_005bac40;
            if (((DVar10 != 0xffffffff) && (*param_1 = local_1c, iVar8 != 0x0)) &&
               (((&DAT_005bac51)[local_1c] & 0x1) != 0x0)) {
              (param_1 + 0x1) = local_1b;
              *(param_3 + 0x20) = *(param_3 + 0x20) + 0x1;
              return CONCAT44(local_18,param_1);
            }^
            // goto LAB_004b5219;
          }
        }
      }
    }
// LAB_004b5206:
    *(param_3 + 0x4) = 0x0;
    param_1 = (param_3 + 0x15);
    *(param_3 + 0x20) = 0x1;^
    // goto LAB_004b5219;
  }
  if (0x64 < bVar1) {
    if (bVar1 < 0x6f) {
      if (bVar1 < 0x66)^ // goto LAB_004b4e55;
      if (bVar1 < 0x67)^ // goto LAB_004b4e1f;
      if (bVar1 < 0x68)^ // goto LAB_004b4e55;
      if (bVar1 != 0x69)^ // goto LAB_004b5206;
    }
    else {
      if (0x6f < bVar1) {
        if (bVar1 < 0x73) {
          if (bVar1 == 0x70)^ // goto LAB_004b508e;
        }
        else {
          if (bVar1 < 0x74)^ // goto LAB_004b4e7f;
          if (0x74 < bVar1) {
            if (bVar1 < 0x76)^ // goto LAB_004b4fb3;
            if (bVar1 == 0x78)^ // goto LAB_004b4f51;
          }
        }^
        // goto LAB_004b5206;
      }
// LAB_004b4f93:
      if (((param_3 + 0x15) == 'o') && (uVar12 = 0x8, (*(param_3 + 0x1e) & 0x1) != 0x0)) {
        iVar8 = *(param_3 + 0x20);
        *(param_3 + 0x20) = iVar8 + 0x1;
        (param_1 + iVar8) = 0x30;
      }
    }
  }
// LAB_004b4fb3:
  local_18 = uVar6 | in_DS;
  puVar7 = (*(param_3 + 0x20) + param_1);
  if ((*(param_3 + 0x1f) & 0x1) == 0x0) {
    if ((*(param_3 + 0x8) != 0x0) || (this != 0x0)) {
      FUN_004b81b0(this,(*(param_3 + 0x20) + param_1),uVar12);
      if ((param_3 + 0x15) == 'X') {
        FUN_004b5228(param_1);
      }^
      // goto LAB_004b5046;
    }
    puVar7 = '\0';
    iVar8 = 0x0;
  }
  else {
    if (((*(param_3 + 0x8) == 0x0) && (local_24 == 0x0)) && (local_20 == 0x0)) {
      puVar7 = '\0';
      iVar8 = 0x0;
    }
    else {
      FUN_004b80a0((longlong *)&local_24,(*(param_3 + 0x20) + param_1),uVar12);
      if ((param_3 + 0x15) == 'X') {
        FUN_004b5228(param_1);
      }
// LAB_004b5046:
      iVar8 = FUN_004b48bc(puVar7,local_18 & 0xffff,-0x1);
    }
  }
  *(param_3 + 0x28) = iVar8;
  if (iVar8 < *(param_3 + 0x8)) {
    *(param_3 + 0x24) = *(param_3 + 0x8) - iVar8;
  }
  param_1 = puVar7;
  if (*(param_3 + 0x8) == -0x1) {
    FUN_004b4b08(param_3);
    return CONCAT44(local_18,puVar7);
  }
// LAB_004b5219:
  return CONCAT44(local_18,param_1);
}



fn FUN_004b5228(byte *param_1)

{
  let bVar1: u8;
  let mut iVar2: i32;
  
  bVar1 = *param_1;
  while (bVar1 != 0x0) {
    iVar2 = FUN_004a11c0(*param_1);
    *param_1 = (byte)iVar2;
    bVar1 = param_1[0x1];
    param_1 = param_1 + 0x1;
  }
  return;
}



fn FUN_004b5290(param_1: i32,param_2: *mut u32,param_3: *mut u32)

{
  if (param_1 == 0x2) {
    *param_2 = 0xc0000000;
    *param_3 = 0x80;
    return;
  }
  if (param_1 == 0x1) {
    *param_2 = 0x40000000;
    *param_3 = 0x80;
    return;
  }
  *param_2 = 0x80000000;
  *param_3 = 0x1;
  return;
}



fn FUN_004b52d4(param_1: u32,param_2: *mut u32)

{
  let mut uVar1: u32;
  
  uVar1 = param_1 & 0x70;
  if (uVar1 < 0x20) {
    if (uVar1 != 0x0) {
      if (uVar1 != 0x10) {
        return;
      }
      *param_2 = 0x0;
      return;
    }
    *param_2 = 0x1;
    if ((param_1 & 0x7) == 0x0) {
      *param_2 = *param_2 | 0x2;
      return;
    }
  }
  else {
    if (uVar1 < 0x21) {
      *param_2 = 0x1;
      return;
    }
    if (0x2f < uVar1) {
      if (uVar1 < 0x31) {
        *param_2 = 0x2;
        return;
      }
      if (uVar1 != 0x40) {
        return;
      }
      *param_2 = 0x3;
    }
  }
  return;
}



fn get_file_type_fn_004b5340(param_1: i32) -> u32

{
  let mut iVar1: i32;
  let DVar2: u32;
  
  (PTR_FUN_004bfb78)(param_1);
  if (DAT_004c0044 != (code *)0x0) {
    iVar1 = (*DAT_004c0044)(param_1);
    if (iVar1 != 0x0) {
      (PTR_FUN_004bfb7c)(param_1);
      return 0x1;
    }
  }
                    // HANDLE hFile for GetFileType
  DVar2 = GetFileType(*(HANDLE *)(DAT_004c0190 + param_1 * 0x4));
  if (DVar2 == 0x2) {
    (PTR_FUN_004bfb7c)(param_1);
    return 0x1;
  }
  (PTR_FUN_004bfb7c)(param_1);
  return 0x0;
}



fn FUN_004b53b0()

{
  return;
}



LPCRITICAL_SECTION init_crit_sec_004b53b4(void)

{
  LPCRITICAL_SECTION lpCriticalSection;
  
  if (DAT_005babf0 < 0x40) {
    lpCriticalSection = (LPCRITICAL_SECTION)(&DAT_005ba5c0 + DAT_005babf0 * 0x18);
    DAT_005babf0 = DAT_005babf0 + 0x1;
  }
  else {
    lpCriticalSection = (LPCRITICAL_SECTION)FUN_004b8240(0x1,0x18);
    if (lpCriticalSection == (LPCRITICAL_SECTION)0x0) {
      FUN_004b67a0(s_Unable_to_allocate_semaphore_dat_004c3750,0x1);
    }
    DAT_005babf8 = FUN_004b7f20(DAT_005babf8,(DAT_005babf4 + 0x1) * 0x4);
    if (DAT_005babf8 == 0x0) {
      FUN_004b67a0(s_Unable_to_allocate_semaphore_dat_004c3774,0x1);
    }
    DAT_005babf8[DAT_005babf4] = lpCriticalSection;
    DAT_005babf4 = DAT_005babf4 + 0x1;
  }
                    // LPCRITICAL_SECTION lpCriticalSection for InitializeCriticalSection
  InitializeCriticalSection(lpCriticalSection);
  return lpCriticalSection;
}



fn FUN_004b5458()

{
  LPCRITICAL_SECTION lpCriticalSection;
  let mut iVar1: i32;
  
  iVar1 = 0x0;
  if (0x0 < DAT_005babf0) {
    lpCriticalSection = (LPCRITICAL_SECTION)&DAT_005ba5c0;
    loop {
                    // LPCRITICAL_SECTION lpCriticalSection for DeleteCriticalSection
      iVar1 = iVar1 + 0x1;
      DeleteCriticalSection(lpCriticalSection);
      lpCriticalSection = lpCriticalSection + 0x1;
    } while (iVar1 < DAT_005babf0);
  }
  return;
}



void crit_sec_fn_004b5484(void)

{
  LPCSTR *ppCVar1;
  let mut iVar2: i32;
  let mut iVar3: i32;
  
  iVar3 = 0x0;
  if (0x0 < DAT_005babf4) {
    iVar2 = 0x0;
    loop {
                    // LPCRITICAL_SECTION lpCriticalSection for DeleteCriticalSection
      DeleteCriticalSection(*(LPCRITICAL_SECTION *)(DAT_005babf8 + iVar2));
      ppCVar1 = (DAT_005babf8 + iVar2);
      iVar2 = iVar2 + 0x4;
      iVar3 = iVar3 + 0x1;
      FUN_004aaae0(*ppCVar1);
    } while (iVar3 < DAT_005babf4);
  }
  if (DAT_005babf8 != 0x0) {
    FUN_004aaae0(DAT_005babf8);
  }
  return;
}



fn FUN_004b54dc(param_1: i32)

{
  *(param_1 + 0x8) = 0x0;
  *(param_1 + 0xc) = 0x0;
  return;
}



void  enter_crit_sec_004b54f0(LPCRITICAL_SECTION *crit_sec_ptr)

{
  LPCRITICAL_SECTION p_Var1;
  LPCRITICAL_SECTION p_Var2;
  
  p_Var1 = (LPCRITICAL_SECTION)GetCurrentThreadId();
  if (p_Var1 != crit_sec_ptr[0x2]) {
    if (crit_sec_ptr[0x1] == (LPCRITICAL_SECTION)0x0) {
      enter_crit_sec_004b54f0((LPCRITICAL_SECTION *)&DAT_005babc0);
      if (crit_sec_ptr[0x1] == (LPCRITICAL_SECTION)0x0) {
        p_Var2 = init_crit_sec_004b53b4();
        crit_sec_ptr[0x1] = (LPCRITICAL_SECTION)0x1;
        *crit_sec_ptr = p_Var2;
      }
      leave_crit_sec_004b5554((LPCRITICAL_SECTION *)&DAT_005babc0);
    }
                    // LPCRITICAL_SECTION lpCriticalSection for EnterCriticalSection
    EnterCriticalSection(*crit_sec_ptr);
    crit_sec_ptr[0x2] = p_Var1;
  }
  crit_sec_ptr[0x3] = (LPCRITICAL_SECTION)(&crit_sec_ptr[0x3]->DebugInfo + 0x1);
  return;
}



void  leave_crit_sec_004b5554(LPCRITICAL_SECTION *param_1)

{
  LPCRITICAL_SECTION p_Var1;
  
  if (param_1[0x3] != (LPCRITICAL_SECTION)0x0) {
    p_Var1 = (LPCRITICAL_SECTION)(&param_1[0x3][-0x1].SpinCount + 0x3);
    param_1[0x3] = p_Var1;
    if (p_Var1 == (LPCRITICAL_SECTION)0x0) {
                    // LPCRITICAL_SECTION lpCriticalSection for LeaveCriticalSection
      param_1[0x2] = (LPCRITICAL_SECTION)0x0;
      LeaveCriticalSection(*param_1);
    }
  }
  return;
}



Pfn tls_get_val_fn_004b568c(void) -> u32

{
  let dwErrCode: u32;
  PDWORD pDVar1;
  
  dwErrCode = GetLastError();
                    // DWORD dwTlsIndex for TlsGetValue
  pDVar1 = (PDWORD)TlsGetValue(tls_index);
  if (pDVar1 == (PDWORD)0x0) {
    pDVar1 = (PDWORD)get_tls_val_004b8270();
  }
  else {
    if ((pDVar1 + 0x53) != '\0') {
      pDVar1 = FUN_004b82a8();
    }
  }
                    // DWORD dwErrCode for SetLastError
  SetLastError(dwErrCode);
  return pDVar1;
}



i32 *  FUN_004b56cc(param_1: *mut i32)

{
  let mut iVar1: i32;
  
  if (param_1 == 0x0) {
    param_1 = FUN_004b8240(0x1,DAT_004c0338);
    iVar1 = DAT_004c0338;
    if (param_1 != 0x0) {
      (param_1 + 0x52) = 0x1;
      param_1[0x3c] = iVar1;
    }
  }
  FUN_004b84a0(param_1);
  return param_1;
}



bool alloc_tls_fn_004b570c(void)

{
  if (tls_index == 0xffffffff) {
    tls_index = TlsAlloc();
    if ((0x7fff < USHORT_min_vers_004c0171) && ((byte)USHORT_maj_vers_004c016f < 0x4)) {
      while ((tls_index != 0xffffffff && (tls_index < 0x3))) {
        tls_index = TlsAlloc();
      }
    }
  }
  return tls_index != 0xffffffff;
}



i32 *  FUN_004b5768(param_1: *mut i32)

{
  let lpTlsValue: *mut i32;;
  let mut iVar1: i32;
  
  if (tls_index == 0xffffffff) {
    return 0x0;
  }
  lpTlsValue = FUN_004b56cc(param_1);
  if (lpTlsValue != 0x0) {
    iVar1 = FUN_004b8380(*(lpTlsValue + 0xda),lpTlsValue);
    if (iVar1 == 0x0) {
      FUN_004aaae0(lpTlsValue);
      return 0x0;
    }
                    // LPVOID lpTlsValue for TlsSetValue
                    // DWORD dwTlsIndex for TlsSetValue
    TlsSetValue(tls_index,lpTlsValue);
    lpTlsValue = 0x1;
  }
  return lpTlsValue;
}



void  check_set_tls_val_004b57c4(param_1: i32)

{
  HANDLE hObject;
  LPVOID tls_val;
  
  if (tls_index != 0xffffffff) {
                    // DWORD dwTlsIndex for TlsGetValue
    tls_val = TlsGetValue(tls_index);
    if (tls_val != (LPVOID)0x0) {
      hObject = *(HANDLE *)(tls_val + 0xde);
      FUN_004b83ec(*(tls_val + 0xda));
                    // LPVOID lpTlsValue for TlsSetValue
                    // DWORD dwTlsIndex for TlsSetValue
      TlsSetValue(tls_index,(LPVOID)0x0);
      if ((hObject != (HANDLE)0x0) && (param_1 != 0x0)) {
                    // HANDLE hObject for CloseHandle
        CloseHandle(hObject);
      }
    }
  }
  return;
}



BOOL tls_fn_004b5818(void)

{
  let mut BVar1: bool;
  
  BVar1 = check_set_tls_val_004b57c4(0x1);
  if (tls_index != 0xffffffff) {
    BVar1 = TlsFree(tls_index);
    tls_index = 0xffffffff;
  }
  return BVar1;
}



fn FUN_004b5824()

{
  if (tls_index != 0xffffffff) {
                    // DWORD dwTlsIndex for TlsFree
    TlsFree(tls_index);
    tls_index = 0xffffffff;
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_004b5844()

{
  PTR_FUN_004bfb78 = &LAB_004b559c;
  PTR_FUN_004bfb7c = &LAB_004b55b8;
  PTR_FUN_004bfb80 = &LAB_004b55d4;
  PTR_FUN_004bfb84 = &LAB_004b55e4;
  PTR_FUN_004bfb88 = &LAB_004b557c;
  PTR_FUN_004bfb8c = &LAB_004b558c;
  PTR_FUN_004bfba0 = &LAB_004b564c;
  PTR_FUN_004bfba4 = &LAB_004b565c;
  PTR_FUN_004c01a8 = enter_crit_sec_004b54f0;
  PTR_FUN_004c01ac = leave_crit_sec_004b5554;
  PTR_FUN_004c01b0 = FUN_004b54dc;
  PTR_FUN_004bfb90 = &LAB_004b560c;
  PTR_FUN_004bfb94 = &LAB_004b562c;
  PTR_FUN_004bfb98 = &LAB_004b561c;
  PTR_FUN_004bfb9c = &LAB_004b563c;
  _DAT_005babc0 = init_crit_sec_004b53b4();
  _DAT_005babc4 = 0x1;
  PTR_FUN_004bfbac = &LAB_004b567c;
  PTR_FUN_004bfbb0 = tls_fn_004b5818;
  PTR_FUN_004bfba8 = &LAB_004b566c;
  FUN_004b8380(*(DAT_005b9dec + 0xda),DAT_005b9dec);
                    // LPVOID lpTlsValue for TlsSetValue
                    // DWORD dwTlsIndex for TlsSetValue
  TlsSetValue(tls_index,DAT_005b9dec);
  PTR_FUN_004bfb74 = tls_get_val_fn_004b568c;
  return;
}



LPCSTR  get_mod_file_name_fn_004b59f0(HMODULE module_handle,LPWSTR file_name,DWORD size)

{
  let num_chars_written_2: u32;
  let mut file_name_2: String;;
  let mod_file_name_str_len: u32;
  let mut num_chars_written: i32;
  
  mod_file_name_str_len = GetVersion();
  if ((ushort)(mod_file_name_str_len >> 0x10) < 0x8000) {
                    // DWORD nSize for GetModuleFileNameW
                    // LPWSTR lpFilename for GetModuleFileNameW
                    // HMODULE hModule for GetModuleFileNameW
    num_chars_written_2 = GetModuleFileNameW(module_handle,file_name,size);
    return num_chars_written_2;
  }
  file_name_2 = FUN_004aac00(0x208);
  if (file_name_2 != 0x0) {
                    // DWORD nSize for GetModuleFileNameA
                    // LPSTR lpFilename for GetModuleFileNameA
                    // HMODULE hModule for GetModuleFileNameA
    mod_file_name_str_len = GetModuleFileNameA(module_handle,file_name_2,0x208);
    if (mod_file_name_str_len == 0x0) {
      FUN_004aaae0(file_name_2);
      return 0x0;
    }
                    // i32 cchWideChar for MultiByteToWideChar
                    // LPWSTR lpWideCharStr for MultiByteToWideChar
                    // i32 cbMultiByte for MultiByteToWideChar
                    // LPCSTR lpMultiByteStr for MultiByteToWideChar
                    // DWORD dwFlags for MultiByteToWideChar
                    // UINT CodePage for MultiByteToWideChar
    num_chars_written = MultiByteToWideChar(0x1,0x1,file_name_2,-0x1,file_name,size);
    FUN_004aaae0(file_name_2);
    if (num_chars_written == 0x0) {
      return 0x0;
    }
    file_name[size - 0x1] = L'\0';
    file_name_2 = FUN_004b8630(file_name);
  }
  return file_name_2;
}



fn FUN_004b5aa0(LPSTR param_1) -> *mut u32

{
  let cVar1: u8;
  let puVar2: *mut u32;
  let mut uVar3: u32;
  LPSTR pCVar4;
  
  uVar3 = 0xffffffff;
  pCVar4 = param_1;
  loop {
    if (uVar3 == 0x0) break;
    uVar3 = uVar3 - 0x1;
    cVar1 = *pCVar4;
    pCVar4 = pCVar4 + 0x1;
  } while (cVar1 != '\0');
  puVar2 = FUN_004aac00(~uVar3);
  if (puVar2 != 0x0) {
    *puVar2 = *param_1;
    (puVar2 + 0x1) = param_1[0x4];
  }
  return puVar2;
}



fn FUN_004b5af0(LPWSTR param_1) -> *mut u32

{
  let mut pCVar1: String;;
  let puVar2: *mut u32;
  
  pCVar1 = FUN_004b8630(param_1);
  puVar2 = FUN_004aac00((pCVar1 + 0x1) * 0x2);
  if (puVar2 != 0x0) {
    FUN_004b8660(puVar2,param_1);
  }
  return puVar2;
}



void  query_virt_mem_004b5b30(param_1: *mut i32,param_2: *mut i32)

{
  let mut iVar1: i32;
  _MEMORY_BASIC_INFORMATION local_28;
  let local_c: u8 [0x4];
  
                    // SIZE_T dwLength for VirtualQuery
                    // PMEMORY_BASIC_INFORMATION lpBuffer for VirtualQuery
                    // LPCVOID lpAddress for VirtualQuery
  VirtualQuery(local_c,(PMEMORY_BASIC_INFORMATION)&local_28,0x1c);
  if (USHORT_min_vers_004c0171 < 0x8000) {
    iVar1 = local_28.AllocationBase + 0x3000;
  }
  else {
    if ((USHORT_min_vers_004c0171 < 0x8000) || (0x3 < (byte)USHORT_maj_vers_004c016f)) {
      iVar1 = local_28.AllocationBase + 0x13000;
    }
    else {
      iVar1 = local_28.AllocationBase + 0x12000;
    }
  }
  if (param_1 != 0x0) {
    *param_1 = iVar1;
  }
  if (param_2 != 0x0) {
    *param_2 = local_28.BaseAddress + local_28.RegionSize;
  }
  return;
}



bool call_get_active_window_004b5ba0(void)

{
  HMODULE hModule;
  FARPROC pFVar1;
  let mut iVar2: i32;
  
                    // LPCSTR lpLibFileName for LoadLibraryA
  hModule = LoadLibraryA(s_USER32_DLL_004c3798);
  iVar2 = 0x0;
  if (hModule != (HMODULE)0x0) {
                    // LPCSTR lpProcName for GetProcAddress
                    // HMODULE hModule for GetProcAddress
    pFVar1 = GetProcAddress(hModule,s_GetActiveWindow_004c37a4);
    if (pFVar1 != (FARPROC)0x0) {
      iVar2 = (*pFVar1)();
    }
  }
  return iVar2 != 0x0;
}



fn FUN_004b5bd8(param_1: &mut String,param_2: &mut String,param_3: u32)

{
  let cVar1: u8;
  let mut uVar2: u32;
  let mut pcVar3: String; 
  let mut pcVar4: String; 
  
  pcVar3 = 0x0;
  cVar1 = *param_1;
  while (cVar1 != '\0') {
    pcVar4 = param_1 + 0x1;
    param_1 = param_1 + 0x1;
    cVar1 = *pcVar4;
  }
  pcVar4 = param_1 + 0x9;
  while( true ) {
    cVar1 = *param_2;
    *param_1 = cVar1;
    if (cVar1 == '\0') break;
    if ((cVar1 == '0') && (param_2[0x1] == 'x')) {
      pcVar3 = pcVar4;
    }
    pcVar4 = pcVar4 + 0x1;
    param_1 = param_1 + 0x1;
    param_2 = param_2 + 0x1;
  }
  if ((pcVar3 != 0x0) && (param_3 != 0x0)) {
    loop {
      uVar2 = param_3 & 0xf;
      param_3 = param_3 >> 0x4;
      *pcVar3 = s_0123456789abcdef_004c01bc[uVar2];
      pcVar3 = pcVar3 + -0x1;
    } while (param_3 != 0x0);
  }
  return;
}



// lpTopLevelExceptionFilter parameter of SetUnhandledExceptionFilter
// 

fn write_err_to_file_004b5c34(param_1: *mut *mut u32) -> u32

{
  let cVar1: u8;
  let puVar2: *mut u32;
  let puVar3: *mut u32;
  let mut bVar4: bool;
  undefined3 extraout_var;
  let mut iVar5: i32;
  let mut uVar6: u32;
  let mut pcVar7: String; 
  let local_114: u8 [0x100];
  let local_14: u32;
  
  puVar2 = *param_1;
  puVar3 = param_1[0x1];
  bVar4 = call_get_active_window_004b5ba0();
  if ((CONCAT31(extraout_var,bVar4) != 0x0) || (iVar5 = FUN_004b873c(), iVar5 == -0x1)) {
    return 0x0;
  }
  local_114[0] = '\0';
  uVar6 = *puVar2;
  if (uVar6 < 0xc0000090) {
    if (0xc000008c < uVar6) {
      if (uVar6 < 0xc000008e) {
        uVar6 = puVar2[0x3];
        pcVar7 = s_The_instruction_at_0x00000000_ca_004c385c;
      }
      else {
        if (uVar6 < 0xc000008f) {
          uVar6 = puVar2[0x3];
          pcVar7 = s_The_instruction_at_0x00000000_ca_004c38b0;
        }
        else {
          uVar6 = puVar2[0x3];
          pcVar7 = s_The_instruction_at_0x00000000_ca_004c3904;
        }
      }^
      // goto LAB_004b5df8;
    }
    if (0xc0000004 < uVar6) {
      if (uVar6 < 0xc0000006) {
        FUN_004b5bd8(local_114,s_The_instruction_at_0x00000000_re_004c3a4c,puVar2[0x3]);
        FUN_004b5bd8(local_114,s_at_0x00000000__The_memory_could_n_004c3a80,puVar2[0x6]);
        uVar6 = puVar2[0x5];
        if (uVar6 == 0x0) {
          pcVar7 = s_read__004c3aa8;
        }
        else {
          uVar6 = 0x0;
          pcVar7 = s_written__004c3ab0;
        }^
        // goto LAB_004b5df8;
      }
      if (uVar6 == 0xc000001d) {
        uVar6 = puVar2[0x3];
        pcVar7 = s_An_illegal_instruction_was_execu_004c3afc;^
        // goto LAB_004b5df8;
      }
    }
  }
  else {
    if (uVar6 < 0xc0000091) {
      uVar6 = puVar2[0x3];
      pcVar7 = s_The_instruction_at_0x00000000_ca_004c39f4;^
      // goto LAB_004b5df8;
    }
    if (uVar6 < 0xc0000093) {
      if (uVar6 < 0xc0000092) {
        uVar6 = puVar2[0x3];
        pcVar7 = s_The_instruction_at_0x00000000_ca_004c3958;
      }
      else {
        if ((*(puVar3 + 0x21) & 0x2) == 0x0) {
          uVar6 = puVar2[0x3];
          pcVar7 = s_The_instruction_at_0x00000000_ca_004c3808;
        }
        else {
          uVar6 = puVar2[0x3];
          pcVar7 = s_The_instruction_at_0x00000000_ca_004c37b4;
        }
      }^
      // goto LAB_004b5df8;
    }
    if (uVar6 < 0xc0000094) {
      uVar6 = puVar2[0x3];
      pcVar7 = s_The_instruction_at_0x00000000_ca_004c39a4;^
      // goto LAB_004b5df8;
    }
    if (uVar6 < 0xc0000096) {
      if (uVar6 == 0xc0000094) {
        uVar6 = puVar2[0x3];
        pcVar7 = s_An_integer_divide_by_zero_was_en_004c3b38;^
        // goto LAB_004b5df8;
      }
    }
    else {
      if (uVar6 < 0xc0000097) {
        uVar6 = puVar2[0x3];
        pcVar7 = s_A_privileged_instruction_was_exe_004c3abc;^
        // goto LAB_004b5df8;
      }
      if (uVar6 == 0xc00000fd) {
        uVar6 = puVar2[0x3];
        pcVar7 = s_A_stack_overflow_was_encountered_004c3b7c;^
        // goto LAB_004b5df8;
      }
    }
  }
  FUN_004b5bd8(local_114,s_The_program_encountered_exceptio_004c3bb8,*puVar2);
  uVar6 = puVar2[0x3];
  pcVar7 = s_address_0x00000000_and_cannot_co_004c3bec;
// LAB_004b5df8:
  FUN_004b5bd8(local_114,pcVar7,uVar6);
                    // LPOVERLAPPED lpOverlapped for WriteFile
                    // LPDWORD lpNumberOfBytesWritten for WriteFile
  uVar6 = 0xffffffff;
  pcVar7 = local_114;
  loop {
    if (uVar6 == 0x0) break;
    uVar6 = uVar6 - 0x1;
    cVar1 = *pcVar7;
    pcVar7 = pcVar7 + 0x1;
  } while (cVar1 != '\0');
                    // DWORD nNumberOfBytesToWrite for WriteFile
                    // LPCVOID lpBuffer for WriteFile
                    // HANDLE hFile for WriteFile
  WriteFile(*(HANDLE *)(DAT_004c0190 + 0x8),local_114,~uVar6 - 0x1,&local_14,(LPOVERLAPPED)0x0);
  return 0x1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn err_handling_fn_004b5e68(PEXCEPTION_RECORD param_1,DWORD param_2,PCONTEXT param_3) -> u32

{
  let sVar1: i16;
  short *psVar2;
  let mut iVar3: i32;
  let LVar4: i32;
  let mut iVar5: i32;
  _EXCEPTION_POINTERS local_14;
  
  if ((*&param_1->ExceptionFlags & 0x6) != 0x0) {
    return 0x1;
  }
  switch(param_1->ExceptionCode) {
  default:
switchD_004b5e90_caseD_7:
    if (_DAT_004c01b8 != (code *)0x0) {
      iVar5 = 0x1;
      loop {
        iVar3 = (*_DAT_004c01b4)(iVar5,param_1->ExceptionCode);
        if (iVar3 != 0x0) {
          if (((iVar3 == 0x1) || (iVar3 == 0x2)) || (iVar3 == 0x3)) break;
          DAT_005bac14 = '\x01';
          (*_DAT_004c01b8)(iVar5);
          if (DAT_005bac14 != '\0') {
            return 0x0;
          }
        }
        iVar5 = iVar5 + 0x1;
      } while (iVar5 < 0xd);
    }^
    // goto LAB_004b600b;
  case 0xc000008d:
    iVar5 = 0x82;
    break;
  case 0xc000008e:
switchD_004b5e90_caseD_c000008e:
    iVar5 = 0x83;
    break;
  case 0xc000008f:
    iVar5 = 0x86;
    break;
  case 0xc0000090:
    psVar2 = (short *)(param_3->FloatSave).ErrorOffset;
    sVar1 = *psVar2;
    iVar5 = 0x81;
    if (sVar1 == -0x527) {
      iVar5 = 0x88;
    }
    else {
      if (sVar1 != -0xe27) {
        if (sVar1 != -0xe27) {
          if (((*psVar2 == 0xdb) || (*psVar2 == 0xdf)) &&
             ((*(psVar2 + 0x1) & 0x30) == 0x10)) {
            iVar5 = 0x8d;
          }
          if ((((*psVar2 & 0x1) == 0x0) && ((*(psVar2 + 0x1) & 0x30) == 0x30)) &&
             ((((param_3->FloatSave).TagWord & 0xffff) >>
               ((byte)((((param_3->FloatSave).StatusWord & 0xffff) << 0x2) >> 0xd) & 0x7) * '\x02' & 0x1) == 0x1))^
          // goto switchD_004b5e90_caseD_c000008e;
          if (iVar5 == -0x1)^ // goto switchD_004b5e90_caseD_7;
        }
        else {
          iVar5 = 0x8f;
        }
      }
      else {
        iVar5 = 0x8e;
      }
    }
    break;
  case 0xc0000091:
    iVar5 = 0x84;
    break;
  case 0xc0000092:
    if ((*(&(param_3->FloatSave).StatusWord + 0x1) & 0x2) == 0x0) {
      iVar5 = 0x8b;
    }
    else {
      iVar5 = 0x8a;
    }
    break;
  case 0xc0000093:
    iVar5 = 0x85;
  }
  DAT_005bac14 = '\x01';
  FUN_004b8750();
  iVar5 = FUN_004b8928(iVar5);
  if ((iVar5 != -0x1) && (DAT_005bac14 != '\0')) {
    &(param_3->FloatSave).StatusWord = &(param_3->FloatSave).StatusWord & 0x7f00;
    return 0x0;
  }
// LAB_004b600b:
                    // _EXCEPTION_POINTERS * ExceptionInfo for UnhandledExceptionFilter
  local_14.ExceptionRecord = param_1;
  local_14.ContextRecord = param_3;
  LVar4 = UnhandledExceptionFilter(&local_14);
  if (LVar4 == 0x0) {
    return 0x1;
  }
                    // UINT uExitCode for ExitProcess
                    // WARNING: Subroutine does not return
  ExitProcess(0xffffffff);
}



void  set_unhandled_except_filter_004b6038(param_1: u32)

{
  let mut uVar1: u32;
  let mut iVar2: i32;
  let in_FS_OFFSET: *mut u32;
  
  iVar2 = (PTR_FUN_004bfb74)();
  *(iVar2 + 0x54) = param_1;
  uVar1 = *in_FS_OFFSET;
  iVar2 = (PTR_FUN_004bfb74)();
  *(iVar2 + 0x54) = uVar1;
  iVar2 = (PTR_FUN_004bfb74)();
  *(code **)(*(iVar2 + 0x54) + 0x4) = err_handling_fn_004b5e68;
  iVar2 = (PTR_FUN_004bfb74)();
  *in_FS_OFFSET = *(iVar2 + 0x54);
                    // LPTOP_LEVEL_EXCEPTION_FILTER lpTopLevelExceptionFilter for SetUnhandledExceptionFilter
  SetUnhandledExceptionFilter(write_err_to_file_004b5c34);
  return;
}



fn FUN_004b6084()

{
  let mut iVar1: i32;
  let in_FS_OFFSET: *mut u32;
  
  iVar1 = (PTR_FUN_004bfb74)();
  if ((iVar1 + 0x54) != 0x0) {
    *in_FS_OFFSET = *(iVar1 + 0x54);
  }
  iVar1 = (PTR_FUN_004bfb74)();
  *(iVar1 + 0x54) = 0x0;
  return;
}



fn FUN_004b616c(param_1: i32) -> i32

{
  let piVar1: *mut i32;;
  let puVar2: *mut u32;
  let piVar3: *mut i32;;
  let mut iVar4: i32;
  let mut iVar5: i32;
  
  iVar5 = 0x0;
  piVar3 = DAT_005ba410;
  if (DAT_005ba410 != 0x0) {
    loop {
      piVar1 = *piVar3;
      puVar2 = piVar3[0x1];
      iVar4 = 0x1;
      if (((*(puVar2 + 0xd) & 0x40) == 0x0) && ((*(puVar2 + 0xd) & 0x8) == 0x0)) {
        if (&DAT_004bfbc0 + param_1 * 0x1a <= puVar2) {
          if (puVar2 < &DAT_004bfc0e) {
            iVar4 = 0x0;
          }^
          // goto LAB_004b61b9;
        }
      }
      else {
// LAB_004b61b9:
        iVar5 = iVar5 + 0x1;
        FUN_0049ca84(puVar2,iVar4);
      }
      piVar3 = piVar1;
    } while (piVar1 != 0x0);
  }
  return iVar5;
}



fn FUN_004b61d0(LPCSTR param_1,byte param_2) -> u32

{
  let DVar1: u32;
  let mut uVar2: u32;
  
                    // LPCSTR lpFileName for GetFileAttributesA
  DVar1 = GetFileAttributesA(param_1);
  if (DVar1 == 0xffffffff) {
    DVar1 = GetLastError();
    uVar2 = FUN_004b1290(DVar1);
    return uVar2;
  }
  if (((param_2 & 0x2) != 0x0) && ((DVar1 & 0x1) != 0x0)) {
    uVar2 = FUN_004b1290(0x5);
    return uVar2;
  }
  return 0x0;
}



fn FUN_004b6200() -> i32

{
  let mut iVar1: i32;
  
  iVar1 = (PTR_FUN_004bfb74)();
  return iVar1 + 0x4;
}



byte *  FUN_004b6220(byte *param_1)

{
  let bVar1: u8;
  let mut iVar2: i32;
  let mut uVar3: u32;
  let mut iVar4: i32;
  byte **ppbVar5;
  byte *pbVar6;
  
  if ((DAT_005bac04 != (byte **)0x0) && (param_1 != 0x0)) {
    uVar3 = 0xffffffff;
    pbVar6 = param_1;
    loop {
      if (uVar3 == 0x0) break;
      uVar3 = uVar3 - 0x1;
      bVar1 = *pbVar6;
      pbVar6 = pbVar6 + 0x1;
    } while (bVar1 != 0x0);
    iVar4 = ~uVar3 - 0x1;
    for (ppbVar5 = DAT_005bac04; pbVar6 = *ppbVar5, pbVar6 != 0x0; ppbVar5 = ppbVar5 + 0x1) {
      iVar2 = FUN_004b8b30(pbVar6,param_1,iVar4);
      if ((iVar2 == 0x0) && (pbVar6[iVar4] == 0x3d)) {
        return pbVar6 + ~uVar3;
      }
    }
  }
  return 0x0;
}



u32 *  get_file_path_fn_004b6280(u32 *path_name_buf,byte *file_name,u32 buffer_len)

{
  let puVar1: *mut u32;
  let mut iVar2: i32;
  let str_len: u32;
  let unaff_EBP: *mut u32;
  LPSTR file_part;
  
  if (path_name_buf == 0x0) {
    buffer_len = 0x104;
    path_name_buf = FUN_004aac00(0x104);
    unaff_EBP = path_name_buf;
    if (path_name_buf == 0x0) {
      FUN_004b1740(0x5);
      return 0x0;
    }
  }
  if ((file_name != 0x0) && (*file_name != 0x0)) {
    iVar2 = FUN_004a2f10(file_name,&DAT_004c3c48);
    if (iVar2 == 0x0) {
      if (buffer_len < 0x4) {
        FUN_004aaae0(unaff_EBP);
        FUN_004b1740(0xe);
        return 0x0;
      }
      *path_name_buf = DAT_004c3c48;
    }
    else {
                    // LPSTR * lpFilePart for GetFullPathNameA
                    // LPSTR lpBuffer for GetFullPathNameA
                    // DWORD nBufferLength for GetFullPathNameA
                    // LPCSTR lpFileName for GetFullPathNameA
      str_len = GetFullPathNameA(file_name,buffer_len,(LPSTR)path_name_buf,&file_part);
      if (str_len == 0x0) {
        handle_err_fn_004b12fc();
        return 0x0;
      }
    }
    return path_name_buf;
  }
  puVar1 = FUN_004b6340(path_name_buf,buffer_len);
  return puVar1;
}



fn FUN_004b6340(param_1: *mut u32,param_2: u32) -> *mut u32

{
  let DVar1: u32;
  let mut uVar2: u32;
  let mut uStack272: u32;
  let CStack268: u8;
  
                    // LPSTR lpBuffer for GetCurrentDirectoryA
                    // DWORD nBufferLength for GetCurrentDirectoryA
  DVar1 = GetCurrentDirectoryA(0x104,(LPSTR)&uStack272);
  if (DVar1 == 0x0) {
    handle_err_fn_004b12fc();
    param_1 = 0x0;
  }
  else {
    if (param_1 == 0x0) {
      uVar2 = DVar1 + 0x1;
      if (DVar1 + 0x1 < param_2) {
        uVar2 = param_2;
      }
      param_1 = FUN_004aac00(uVar2);
      if (param_1 == 0x0) {
        FUN_004b1740(0x5);
        return 0x0;
      }
    }
    else {
      if (param_2 < DVar1) {
        FUN_004b1740(0xe);
        return 0x0;
      }
    }
    *param_1 = uStack272;
    (param_1 + 0x1) = CStack268;
  }
  return param_1;
}



fn flush_file_buffers_fn_004b63f0(param_1: u32) -> u32

{
  let mut flushed: bool;
  let DVar1: u32;
  
  DVar1 = 0x0;
  if ((-0x1 < param_1) && (param_1 <= DAT_004bffe8)) {
    (PTR_FUN_004bfb78)(param_1);
                    // HANDLE hFile for FlushFileBuffers
    flushed = FlushFileBuffers(*(HANDLE *)(DAT_004c0190 + param_1 * 0x4));
    if (flushed == 0x0) {
      handle_err_fn_004b12fc();
      DVar1 = 0xffffffff;
    }
    (PTR_FUN_004bfb7c)(param_1);
    return DVar1;
  }
  FUN_004b1740(0x4);
  return 0xffffffff;
}



fn FUN_004b6450(param_1: i32,param_2: i32,param_3: *mut i32) -> u32

{
  ushort uVar1;
  i32 **ppiVar2;
  let puVar3: *mut u32;
  let mut iVar4: i32;
  let mut local_1c: i32;
  let mut local_18: i32;
  i32 **local_14;
  
  local_14 = (i32 **)0x0;
  if ((*(param_1 + 0x9c) & 0x3) != 0x0) {
    local_14 = (i32 **)FUN_004a2831(0x18);
    if (local_14 != (i32 **)0x0) {
      local_14 = (i32 **)FUN_00498ba4(local_14,*(param_1 + 0xa0),(param_1 + 0xa),
                                      (param_1 + 0x8));
    }
    local_18 = *(param_1 + 0x8c);
    local_1c = *(param_1 + 0x90);
    *(param_1 + 0x90) = 0x0;
    *(param_1 + 0x8c) = *(param_1 + 0x90);
  }
  iVar4 = 0x0;
// LAB_004b6476:
  if (*(param_2 + 0x4) >> 0x10 <= iVar4) {
    if (local_14 != (i32 **)0x0) {
      if (local_14 != (i32 **)0x0) {
        ppiVar2 = FUN_00498cf4(local_14);
        FUN_0049af50(ppiVar2);
      }
      if ((*(param_1 + 0x9c) & 0x1) != 0x0) {
        FUN_004a1dc0((param_1 + 0xa4),(param_1 + 0xa0),
                     (param_1 + 0xa) * (param_1 + 0x8));
        if (*(param_1 + 0xa8) != 0x0) {
          ppiVar2 = (i32 **)FUN_004a2831(0x18);
          if (ppiVar2 != (i32 **)0x0) {
            ppiVar2 = (i32 **)FUN_00498ba4(ppiVar2,*(param_1 + 0xa4),(param_1 + 0xa),
                                           (param_1 + 0x8));
          }
          ((param_1 + 0xa8))(param_1,0x0,0x2);
          if ((ppiVar2 != (i32 **)0x0) && (ppiVar2 != (i32 **)0x0)) {
            ppiVar2 = FUN_00498cf4(ppiVar2);
            FUN_0049af50(ppiVar2);
          }
        }
      }
      *(param_1 + 0x8c) = local_18;
      *(param_1 + 0x90) = local_1c;
      FUN_00496ac0((param_1 + 0xa4),local_18,local_1c,(param_1 + 0x8),
                   (param_1 + 0xa));
    }
    return 0x0;
  }
  puVar3 = (param_3 + 0x6);
  uVar1 = (param_3 + 0x1);
  param_3 = (param_3 + *param_3);
  if (uVar1 < 0xd) {
    if (uVar1 < 0x7) {
      if ((uVar1 == 0x4) && ((*(param_1 + 0x9c) & 0x4) == 0x0)) {
        FUN_004b8bc0((short *)puVar3,0x1);
        iVar4 = iVar4 + 0x1;^
        // goto LAB_004b6476;
      }
    }
    else {
      if (uVar1 < 0x8) {
        FUN_004b8d80((short *)puVar3,param_1);
        iVar4 = iVar4 + 0x1;^
        // goto LAB_004b6476;
      }
      if (0xa < uVar1) {
        if (0xb < uVar1) {
          FUN_004b8cd0((short *)puVar3,param_1);
          iVar4 = iVar4 + 0x1;^
          // goto LAB_004b6476;
        }
        if ((*(param_1 + 0x9c) & 0x4) == 0x0) {
          FUN_004b8bc0((short *)puVar3,0x0);
          iVar4 = iVar4 + 0x1;^
          // goto LAB_004b6476;
        }
      }
    }
  }
  else {
    if (uVar1 < 0xe) {
      FUN_004968e7(*(param_1 + 0x8c),*(param_1 + 0x90),(param_1 + 0x8),
                   (param_1 + 0xa),0x0);
      iVar4 = iVar4 + 0x1;^
      // goto LAB_004b6476;
    }
    if (uVar1 < 0x65) {
      if (uVar1 < 0xf)^ // goto LAB_004b64ab;
      if (uVar1 < 0x10) {
        FUN_004b8c40(puVar3,param_1);
        iVar4 = iVar4 + 0x1;
      }
      else {
        if (uVar1 == 0x10) {
          FUN_00496ac0(puVar3,*(param_1 + 0x8c),*(param_1 + 0x90),(param_1 + 0x8),
                       (param_1 + 0xa));
          iVar4 = iVar4 + 0x1;
        }
        else {
          iVar4 = iVar4 + 0x1;
        }
      }^
      // goto LAB_004b6476;
    }
    if (uVar1 < 0x66) {
      FUN_004b95c0(puVar3);
      iVar4 = iVar4 + 0x1;^
      // goto LAB_004b6476;
    }
    if (0x66 < uVar1) {
      if (uVar1 < 0x68) {
        FUN_004b8ea0(puVar3);
        iVar4 = iVar4 + 0x1;
      }
      else {
        if (uVar1 == 0x68) {
          FUN_004b9280(puVar3,param_1);
          iVar4 = iVar4 + 0x1;
        }
        else {
          iVar4 = iVar4 + 0x1;
        }
      }^
      // goto LAB_004b6476;
    }
  }
// LAB_004b64ab:
  iVar4 = iVar4 + 0x1;^
  // goto LAB_004b6476;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void write_to_file_004b6760(param_1: &mut String,UINT param_2)

{
  let cVar1: u8;
  let mut pcVar2: String; 
  let mut extraout_ECX: u32;
  let nNumberOfBytesToWrite: u32;
  let local_8: u32;
  
  nNumberOfBytesToWrite = 0x0;
  pcVar2 = param_1;
  while (cVar1 = *pcVar2, pcVar2 = pcVar2 + 0x1, cVar1 != '\0') {
    nNumberOfBytesToWrite = nNumberOfBytesToWrite + 0x1;
  }
                    // LPOVERLAPPED lpOverlapped for WriteFile
                    // LPDWORD lpNumberOfBytesWritten for WriteFile
                    // DWORD nNumberOfBytesToWrite for WriteFile
                    // LPCVOID lpBuffer for WriteFile
                    // HANDLE hFile for WriteFile
  WriteFile(*(HANDLE *)(DAT_004c0190 + 0x8),param_1,nNumberOfBytesToWrite,&local_8,(LPOVERLAPPED)0x0);
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
                    // WARNING: Subroutine does not return
  ExitProcess(param_2);
}



fn FUN_004b67a0(param_1: &mut String,UINT param_2)

{
  let mut iVar1: i32;
  let in_DS: u16;
  
  iVar1 = FUN_004b67d0(param_1,in_DS);
  if (iVar1 == 0x0) {
    write_to_file_004b6760(param_1,param_2);
  }
  return;
}



fn FUN_004b67d0(param_1: u32,undefined2 param_2) -> u32

{
  code *pcVar1;
  let mut uVar2: u32;
  
  if (DAT_004c01d0 != '\0') {
    pcVar1 = (code *)swi(0x3);
    uVar2 = (*pcVar1)(param_2);
    return uVar2;
  }
  return 0x0;
}



fn FUN_004b6800(param_1: u32) -> u32

{
  if ((param_1 & 0x3) == 0x0) {
    if (((ulonglong)param_1 % 0x64) != 0x0) {
      return 0x1;
    }
    if ((((ulonglong)param_1 % 0x64 << 0x20 | (ulonglong)param_1) % 0x190) == 0x0) {
      return 0x1;
    }
  }
  return 0x0;
}



fn FUN_004b6838(param_1: i32,param_2: i32) -> i32

{
  let mut iVar1: i32;
  let mut iVar2: i32;
  let mut iVar3: i32;
  let mut iVar4: i32;
  let mut local_34: i32;
  let mut local_30: u32;
  let mut local_2c: u32;
  let mut local_28: u32;
  let mut local_24: u32;
  let mut local_20: i32;
  let mut local_1c: i32;
  let mut local_18: i32;
  let mut local_14: u32;
  
  if (*(param_1 + 0x20) == 0x0) {
    iVar1 = FUN_004b6800(param_2 + 0x76c);
    if (iVar1 == 0x0) {
      iVar1 = *(&DAT_004c41bc + *(param_1 + 0x10) * 0x2);
      iVar2 = *(&DAT_004c41ba + *(param_1 + 0x10) * 0x2);
    }
    else {
      iVar1 = *(&DAT_004c41d6 + *(param_1 + 0x10) * 0x2);
      iVar2 = *(&DAT_004c41d4 + *(param_1 + 0x10) * 0x2);
    }
    local_34 = 0x0;
    local_30 = 0x0;
    local_2c = 0x0;
    local_28 = 0x1;
    local_24 = *(param_1 + 0x10);
    local_20 = param_2;
    local_14 = 0x0;
    FUN_004b1470(&local_34);
    iVar4 = ((*(param_1 + 0x18) - local_1c) + 0x7) % 0x7;
    if (*(param_1 + 0xc) == 0x5) {
      iVar3 = 0x4;
      if ((iVar1 >> 0x10) - (iVar2 >> 0x10) < iVar4 + 0x1d) {
        iVar3 = *(param_1 + 0xc) + -0x2;
      }
    }
    else {
      iVar3 = *(param_1 + 0xc) + -0x1;
    }
    return iVar3 * 0x7 + local_18 + iVar4;
  }
  if (*(param_1 + 0x20) != 0x1) {
    return *(param_1 + 0x1c);
  }
  return *(param_1 + 0x1c) + -0x1;
}



fn FUN_004b6938(param_1: i32,param_2: i32,param_3: i32) -> u32

{
  let mut iVar1: i32;
  let mut iVar2: i32;
  
  if ((*(param_1 + 0x20) == 0x0) && (*(param_2 + 0x20) == 0x0)) {
    if (*(param_2 + 0x10) < *(param_1 + 0x10)) {
      return 0x1;
    }
    if (*(param_1 + 0x10) < *(param_2 + 0x10)) {
      return 0x0;
    }
  }
  iVar1 = FUN_004b6838(param_1,param_3);
  iVar2 = FUN_004b6838(param_2,param_3);
  if (iVar1 <= iVar2) {
    return 0x0;
  }
  return 0x1;
}



fn FUN_004b6994(param_1: *mut i32) -> u32

{
  let mut uVar1: u32;
  let mut iVar2: i32;
  let mut iVar3: i32;
  let mut iVar4: i32;
  let mut iVar5: i32;
  let mut iVar6: i32;
  let piVar7: *mut i32;;
  let mut uVar8: u32;
  let piVar9: *mut i32;;
  let mut local_14: i32;
  
  uVar8 = 0x0;
  if (*PTR_DAT_004c0324 == '\0')^ // goto LAB_004b6c5b;
  uVar1 = FUN_004b6938(&DAT_004c01d4,&DAT_004c01f8,param_1[0x5]);
  if (uVar1 == 0x0) {
    piVar7 = &DAT_004c01d4;
    piVar9 = &DAT_004c01f8;
  }
  else {
    piVar7 = &DAT_004c01f8;
    piVar9 = &DAT_004c01d4;
  }
  iVar4 = param_1[0x4];
  iVar2 = FUN_004b6800(param_1[0x5] + 0x76c);
  iVar6 = iVar4 * 0x2;
  if (iVar2 == 0x0) {
    iVar2 = *(&DAT_004c41bc + iVar6);
    iVar6 = *(&DAT_004c41ba + iVar6);
  }
  else {
    iVar2 = *(&DAT_004c41d6 + iVar6);
    iVar6 = *(&DAT_004c41d4 + iVar6);
  }
  iVar6 = (iVar2 >> 0x10) - (iVar6 >> 0x10);
  if (piVar7[0x8] == 0x0) {
    if (piVar7[0x4] < iVar4) {
      uVar8 = 0x1;
    }
    else {
      if (iVar4 == piVar7[0x4]) {
        iVar2 = param_1[0x3] - ((param_1[0x6] + 0x7) - piVar7[0x6]) % 0x7;
        iVar5 = (param_1[0x3] + -0x1) - ((param_1[0x6] + 0x6) - piVar7[0x6]) % 0x7;
        if (piVar7[0x3] == 0x5) {
          iVar3 = iVar6 + -0x7;
          if ((iVar3 < iVar2) && (uVar8 = 0x1, iVar5 <= iVar3)) {
// LAB_004b6b35:
            iVar2 = FUN_004b6c68(param_1,piVar7);
            uVar8 = (iVar2 == 0x0);
          }
        }
        else {
          iVar3 = (piVar7[0x3] + -0x1) * 0x7 + 0x1;
          if ((iVar3 <= iVar2) && (uVar8 = 0x1, iVar5 < iVar3))^ // goto LAB_004b6b35;
        }
      }
    }
  }
  else {
    local_14 = piVar7[0x7];
    if (piVar7[0x8] == 0x1) {
      iVar2 = FUN_004b6800(param_1[0x5] + 0x76c);
      if ((iVar2 != 0x0) && (DAT_004c41be >> 0x10 < local_14)) {
        local_14 = local_14 + 0x1;
      }
      local_14 = local_14 + -0x1;
    }
    if ((local_14 <= param_1[0x7]) && (uVar8 = 0x1, param_1[0x7] == local_14))^ // goto LAB_004b6b35;
  }
  if (uVar8 == 0x0) {
    if (uVar1 != 0x0) {
      uVar8 = uVar1;
    }^
    // goto LAB_004b6c5b;
  }
  if (piVar9[0x8] == 0x0) {
    if (piVar9[0x4] < iVar4) {
      uVar8 = 0x0;
    }
    else {
      if (iVar4 == piVar9[0x4]) {
        iVar2 = param_1[0x3] - ((param_1[0x6] + 0x7) - piVar9[0x6]) % 0x7;
        uVar8 = 0x0;
        iVar4 = (param_1[0x3] + -0x1) - ((param_1[0x6] + 0x6) - piVar9[0x6]) % 0x7;
        if (piVar9[0x3] == 0x5) {
          iVar6 = iVar6 + -0x7;
          if (iVar6 < iVar2) {
            if (iVar4 <= iVar6) {
// LAB_004b6c43:
              uVar8 = FUN_004b6c68(param_1,piVar9);
            }
          }
          else {
            uVar8 = 0x1;
          }
        }
        else {
          iVar6 = (piVar9[0x3] + -0x1) * 0x7 + 0x1;
          if (iVar2 < iVar6) {
            uVar8 = 0x1;
          }
          else {
            if (iVar4 < iVar6)^ // goto LAB_004b6c43;
          }
        }
      }
    }
  }
  else {
    iVar4 = piVar9[0x7];
    if (piVar9[0x8] == 0x1) {
      iVar6 = FUN_004b6800(param_1[0x5] + 0x76c);
      if ((iVar6 != 0x0) && (DAT_004c41be >> 0x10 < iVar4)) {
        iVar4 = iVar4 + 0x1;
      }
      iVar4 = iVar4 + -0x1;
    }
    if ((iVar4 <= param_1[0x7]) && (uVar8 = 0x0, iVar4 == param_1[0x7]))^ // goto LAB_004b6c43;
  }
  if (uVar1 != 0x0) {
    uVar8 = uVar1 - uVar8;
  }
// LAB_004b6c5b:
  param_1[0x8] = uVar8;
  return uVar8;
}



fn FUN_004b6c68(param_1: *mut i32,param_2: *mut i32) -> u32

{
  let mut uVar1: u32;
  
  uVar1 = 0x0;
  if ((param_1[0x2] < param_2[0x2]) ||
     ((param_1[0x2] == param_2[0x2] &&
      ((param_1[0x1] < param_2[0x1] || ((param_1[0x1] == param_2[0x1] && (*param_1 < *param_2)))))))) {
    uVar1 = 0x1;
  }
  return uVar1;
}



fn FUN_004b6ca0(param_1: i32,param_2: u32,param_3: i32,param_4: *mut u32) -> *mut u32

{
  ulonglong uVar1;
  let mut iVar2: i32;
  let mut uVar3: u32;
  let mut uVar4: u32;
  let mut uVar5: u32;
  let mut local_14: *mut u8; 
  
  if ((param_2 < 0xa8c0) && (0x0 < param_3)) {
    uVar3 = (param_2 + 0x15180) - param_3;
    uVar5 = (param_1 + uVar3 / 0x15180) - 0x1;
  }
  else {
    uVar3 = param_2 - param_3;
    uVar5 = param_1 + uVar3 / 0x15180;
  }
  param_4[0x2] = (((ulonglong)uVar3 % 0x15180) / 0xe10);
  uVar1 = ((ulonglong)uVar3 % 0x15180) % 0xe10;
  param_4[0x1] = (uVar1 / 0x3c);
  *param_4 = (uVar1 % 0x3c);
  uVar3 = uVar5 / 0x16e;
  uVar4 = uVar5 + uVar3 * -0x16d;
  if (uVar3 != 0x0) {
    uVar4 = uVar4 - (uVar3 - 0x1 >> 0x2);
  }
  iVar2 = FUN_004b6800(uVar3 + 0x76c);
  for (; iVar2 + 0x16dU <= uVar4; uVar4 = uVar4 - (iVar2 + 0x16dU)) {
    uVar3 = uVar3 + 0x1;
  }
  param_4[0x5] = uVar3;
  param_4[0x7] = uVar4;
  local_14 = &DAT_004c41bc;
  iVar2 = FUN_004b6800(uVar3 + 0x76c);
  if (iVar2 != 0x0) {
    local_14 = &DAT_004c41d6;
  }
  uVar3 = uVar4 / 0x1f;
  if ((*(local_14 + uVar3 * 0x2) >> 0x10) <= uVar4) {
    uVar3 = uVar3 + 0x1;
  }
  param_4[0x4] = uVar3;
  param_4[0x3] = (uVar4 - (local_14 + uVar3 * 0x2)) + 0x1;
  param_4[0x6] = (uVar5 + 0x1) % 0x7;
  return param_4;
}



fn FUN_004b6de8(param_1: *mut u32,param_2: *mut u32)

{
  param_2[0x8] = 0x0;
  FUN_004b6ca0(0x63df,*param_1,0x0,param_2);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void time_zone_fn_004b6f68(void)

{
  let DVar1: u32;
  let puVar2: *mut u32;
  _TIME_ZONE_INFORMATION local_b4;
  
  puVar2 = FUN_004b6220(&DAT_004c3c4c);
  if (puVar2 != 0x0) {
    FUN_004b71dc(puVar2);
    return;
  }
  if (((DAT_004c0334 & 0x1) == 0x0) || ((DAT_004c0334 & 0x2) == 0x0)) {
    DAT_004c0334 = DAT_004c0334 | 0x2;
                    // LPTIME_ZONE_INFORMATION lpTimeZoneInformation for GetTimeZoneInformation
    DVar1 = GetTimeZoneInformation((LPTIME_ZONE_INFORMATION)&local_b4);
    if (DVar1 != 0x0) {
      if (DVar1 < 0x2) {
        _DAT_004c032c = 0x0;
      }
      else {
        if (DVar1 != 0x2) {
          return;
        }
        _DAT_004c032c = 0x1;
        DAT_004c0330 = local_b4.DaylightBias * -0x3c;
      }
      DAT_004c0328 = (local_b4.StandardBias + local_b4.Bias) * 0x3c;
      DVar1 = FUN_004b9600(&DAT_004c021c,local_b4.StandardName,0x80);
      if (DVar1 == 0xffffffff) {
        DAT_004c021c = 0x0;
      }
      else {
        DAT_004c029c = 0x0;
      }
      DVar1 = FUN_004b9600(&DAT_004c029d,local_b4.DaylightName,0x80);
      if (DVar1 == 0xffffffff) {
        DAT_004c029d = 0x0;
      }
      else {
        DAT_004c031d = 0x0;
      }
    }
  }
  return;
}



fn FUN_004b6f88(byte *param_1,param_2: *mut i32)

{
  let bVar1: u8;
  let bVar2: u8;
  let mut iVar3: i32;
  
  bVar1 = *param_1;
  iVar3 = 0x0;
  while ((0x2f < bVar1 && (*param_1 < 0x3a))) {
    bVar2 = *param_1;
    param_1 = param_1 + 0x1;
    bVar1 = *param_1;
    iVar3 = iVar3 * 0xa + bVar2 + -0x30;
  }
  *param_2 = iVar3;
  return;
}



fn FUN_004b6fb8(param_1: *mut u32,param_2: *mut u32,param_3: *mut i32) -> *mut u32

{
  let bVar1: u8;
  let mut bVar2: bool;
  let puVar3: *mut u32;
  let mut iVar4: i32;
  let mut local_1c: i32;
  let mut local_18: i32;
  let mut local_14: i32;
  
  puVar3 = param_1;
  if (param_1 == ':') {
    param_1 = (param_1 + 0x1);
    puVar3 = param_1;
  }
  for (; ((((bVar1 = *param_1, bVar1 != 0x0 && (bVar1 != 0x2c)) && (bVar1 != 0x2d)) && (bVar1 != 0x2b)) &&
         ((bVar1 < 0x30 || (0x39 < bVar1)))); param_1 = (param_1 + 0x1)) {
  }
  iVar4 = param_1 - puVar3;
  if (0x80 < iVar4) {
    iVar4 = 0x80;
  }
  *param_2 = *puVar3;
  (param_2 + 0x1) = (puVar3 + 0x1);
  bVar2 = false;
  (iVar4 + param_2) = 0x0;
  if (bVar1 == 0x2d) {
    bVar2 = true;
  }
  else {
    if (bVar1 != 0x2b)^ // goto LAB_004b7038;
  }
  param_1 = (param_1 + 0x1);
// LAB_004b7038:
  if ((0x2f < *param_1) && (*param_1 < 0x3a)) {
    local_1c = 0x0;
    local_18 = 0x0;
    local_14 = 0x0;
    param_1 = FUN_004b6f88(param_1,&local_14);
    if (param_1 == ':') {
      param_1 = FUN_004b6f88((param_1 + 0x1),&local_18);
      if (param_1 == ':') {
        param_1 = FUN_004b6f88((param_1 + 0x1),&local_1c);
      }
    }
    local_1c = local_1c + (local_18 + local_14 * 0x3c) * 0x3c;
    *param_3 = local_1c;
    if (bVar2) {
      *param_3 = -local_1c;
    }
  }
  return param_1;
}



char *  FUN_004b70d4(byte *param_1,param_2: *mut i32)

{
  let mut pcVar1: String; 
  let mut iVar2: i32;
  let mut local_18: i32;
  let mut local_14: i32;
  let mut local_10: i32;
  let mut local_c: i32;
  
  iVar2 = -0x1;
  if (*param_1 == 0x4a) {
    iVar2 = 0x1;
    param_1 = param_1 + 0x1;
  }
  if (*param_1 == 0x4d) {
    param_1 = param_1 + 0x1;
    iVar2 = 0x0;
  }
  param_2[0x8] = iVar2;
  pcVar1 = FUN_004b6f88(param_1,&local_c);
  if (iVar2 == 0x0) {
    param_2[0x4] = local_c + -0x1;
    if (*pcVar1 == '.') {
      pcVar1 = FUN_004b6f88((pcVar1 + 0x1),&local_c);
      param_2[0x3] = local_c;
      if (*pcVar1 == '.') {
        pcVar1 = FUN_004b6f88((pcVar1 + 0x1),&local_c);
        param_2[0x6] = local_c;
      }
    }
    param_2[0x7] = 0x0;
  }
  else {
    param_2[0x7] = local_c;
  }
  local_10 = 0x2;
  local_18 = 0x0;
  local_14 = 0x0;
  if (*pcVar1 == '/') {
    pcVar1 = FUN_004b6f88((pcVar1 + 0x1),&local_10);
    if (*pcVar1 == ':') {
      pcVar1 = FUN_004b6f88((pcVar1 + 0x1),&local_14);
      if (*pcVar1 == ':') {
        pcVar1 = FUN_004b6f88((pcVar1 + 0x1),&local_18);
      }
    }
  }
  *param_2 = local_18;
  param_2[0x1] = local_14;
  param_2[0x2] = local_10;
  return pcVar1;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

fn FUN_004b71dc(param_1: *mut u32)

{
  let cVar1: u8;
  let puVar2: *mut u32;
  let mut local_14: i32;
  
  _DAT_004c032c = 0x0;
  puVar2 = FUN_004b6fb8(param_1,&DAT_004c021c,&DAT_004c0328);
  cVar1 = puVar2;
  if (puVar2 != '\0') {
    local_14 = DAT_004c0328 + -0xe10;
    _DAT_004c032c = 0x1;
    puVar2 = FUN_004b6fb8(puVar2,&DAT_004c029d,&local_14);
    DAT_004c0330 = DAT_004c0328 - local_14;
    if (puVar2 == ',') {
      puVar2 = FUN_004b70d4((puVar2 + 0x1),&DAT_004c01d4);
    }
    cVar1 = DAT_004c029d;
    if (puVar2 == ',') {
      FUN_004b70d4((puVar2 + 0x1),&DAT_004c01f8);
      DAT_004c0200 = DAT_004c0200 - DAT_004c0330 / 0xe10;
      DAT_004c01fc = DAT_004c01fc -
                     ((longlong)
                           ((ulonglong)(((longlong)DAT_004c0330 / 0x3c) >> 0x1f) << 0x20 |
                           (longlong)DAT_004c0330 / 0x3c & 0xffffffffU) % 0x3c);
      DAT_004c01f8 = DAT_004c01f8 - DAT_004c0330 % 0x3c;
      cVar1 = DAT_004c029d;
    }
  }
  DAT_004c029d = cVar1;
  return;
}



fn FUN_004b7330() -> String

{
  let piVar1: *mut i32;;
  
  piVar1 = (PTR_FUN_004bfb74)();
  return &stack0xfffffffc + -*piVar1;
}



fn FUN_004b7340()

{
  return;
}



u32  read_console_input_004b7350(HANDLE param_1)

{
  let mut BVar1: bool;
  let mut iVar2: i32;
  let local_28: u8 [0xc];
  ushort local_1c;
  let local_1a: u8;
  let local_17: u8;
  let local_14: u32;
  
  if (DAT_004c0344 != 0x0) {
    if (DAT_004c0344 < 0x2) {
      DAT_005bac38 = DAT_005bac38 + -0x1;
      if (DAT_005bac34 == 0x0) {
        DAT_004c0344 = 0x2;
        return 0x0;
      }
      if (DAT_005bac38 != 0x0) {
        return DAT_005bac34;
      }
      DAT_004c0344 = DAT_005bac38;
      return DAT_005bac34;
    }
    if (DAT_004c0344 == 0x2) {
      DAT_004c0344 = (DAT_005bac38 != 0x0);
      return DAT_005bac30;
    }
  }
  loop {
                    // LPDWORD lpNumberOfEventsRead for ReadConsoleInputA
                    // DWORD nLength for ReadConsoleInputA
                    // PINPUT_RECORD lpBuffer for ReadConsoleInputA
                    // HANDLE hConsoleInput for ReadConsoleInputA
    BVar1 = ReadConsoleInputA(param_1,(PINPUT_RECORD)local_28,0x1,&local_14);
    if (BVar1 == 0x0) {
      return 0xffffffff;
    }
    iVar2 = FUN_004b8690((short *)local_28);
  } while (iVar2 == 0x0);
  DAT_005bac34 = local_1a;
  if (((local_17 & 0x1) == 0x0) && (DAT_005bac34 != 0x0)) {
    if (local_28._8_2_ - 0x1 != 0x0) {
      DAT_004c0344 = 0x1;
    }
  }
  else {
    DAT_005bac34 = 0x0;
    DAT_005bac30 = local_1c;
    DAT_004c0344 = 0x2;
  }
  DAT_005bac38 = local_28._8_2_ - 0x1;
  return DAT_005bac34;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

u32 console_fn_004b7468(void)

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  HANDLE hConsoleHandle;
  let DStack12: u32;
  
  uVar2 = DAT_004c0158;
  if (DAT_004c0158 == 0x0) {
    if (_DAT_004c0078 == (code *)0x0) {
      (PTR_FUN_004bfb78)(0x0);
      hConsoleHandle = (HANDLE)FUN_004b8730();
                    // LPDWORD lpMode for GetConsoleMode
                    // HANDLE hConsoleHandle for GetConsoleMode
      GetConsoleMode(hConsoleHandle,&DStack12);
                    // DWORD dwMode for SetConsoleMode
                    // HANDLE hConsoleHandle for SetConsoleMode
      SetConsoleMode(hConsoleHandle,0x0);
      uVar2 = read_console_input_004b7350(hConsoleHandle);
                    // DWORD dwMode for SetConsoleMode
                    // HANDLE hConsoleHandle for SetConsoleMode
      SetConsoleMode(hConsoleHandle,DStack12);
      (PTR_FUN_004bfb7c)(0x0);
    }
    else {
      uVar1 = (*DAT_004c0044)(0x0);
      uVar2 = (*_DAT_004c0078)(uVar1);
    }
  }
  else {
    DAT_004c0158 = 0x0;
  }
  return uVar2;
}



u32  write_to_console_004b7500(param_1: u32)

{
  let mut uVar1: u32;
  HANDLE hConsoleOutput;
  let DStack12: u32;
  let local_8: u8 [0x4];
  
  local_8[0] = param_1;
  if (DAT_004c0080 == (code *)0x0) {
    (PTR_FUN_004bfb78)(0x1);
    hConsoleOutput = (HANDLE)FUN_004b873c();
                    // LPVOID lpReserved for WriteConsoleA
                    // LPDWORD lpNumberOfCharsWritten for WriteConsoleA
                    // DWORD nNumberOfCharsToWrite for WriteConsoleA
                    // void * lpBuffer for WriteConsoleA
                    // HANDLE hConsoleOutput for WriteConsoleA
    WriteConsoleA(hConsoleOutput,local_8,0x1,&DStack12,(LPVOID)0x0);
    (PTR_FUN_004bfb7c)(0x1);
  }
  else {
    uVar1 = (*DAT_004c0044)(0x1);
    (*DAT_004c0080)(uVar1,param_1);
  }
  return param_1;
}



fn FUN_004b7570(byte *param_1) -> u32

{
  if (*param_1 == 0x0) {
    return 0x1;
  }
  if (((DAT_005bac40 != 0x0) && (((&DAT_005bac51)[*param_1] & 0x1) != 0x0)) && (param_1[0x1] == 0x0)) {
    return 0x2;
  }
  return 0x0;
}



fn FUN_004b75b0(byte *param_1) -> u32

{
  if ((DAT_005bac40 != 0x0) && (((&DAT_005bac51)[*param_1] & 0x1) != 0x0)) {
    return 0x2;
  }
  return 0x1;
}



char *  FUN_004b75e0(param_1: &mut String,param_2: &mut String,param_3: u32)

{
  let cVar1: u8;
  let mut pcVar2: String; 
  let mut pcVar3: String; 
  
  pcVar2 = param_1;
  pcVar3 = param_1;
  if (param_3 != 0x0) {
    loop {
      pcVar3 = pcVar2;
      if (*param_2 == '\0') break;
      pcVar3 = pcVar2 + 0x1;
      cVar1 = *param_2;
      param_2 = param_2 + 0x1;
      param_3 = param_3 - 0x1;
      *pcVar2 = cVar1;
      pcVar2 = pcVar3;
    } while (param_3 != 0x0);
  }
  if ((((*param_2 != '\0') && (param_1 < pcVar3)) && (DAT_005bac40 != 0x0)) &&
     (((&DAT_005bac51)[(byte)param_2[-0x1]] & 0x1) != 0x0)) {
    pcVar3[-0x1] = '\0';
  }
  if (param_3 != 0x0) {
    FUN_004a0430(pcVar3,0x0,param_3);
  }
  return param_1;
}



double  FUN_004b7670(param_1: u32,param_2: u32,double *param_3)

{
  float10 in_ST0;
  float10 fVar1;
  
  fVar1 = FUN_004a9b12();
  *param_3 = (double)fVar1;
  return (double)in_ST0;
}



fn FUN_004b7690(param_1: i32)

{
  let mut iVar1: i32;
  
  if (((*(param_1 + 0x1e) & 0x8) == 0x0) && ((param_1 + 0x16) == '0')) {
    iVar1 = (((((*(param_1 + 0x4) - *(param_1 + 0x20)) - *(param_1 + 0x24)) -
              *(param_1 + 0x28)) - *(param_1 + 0x2c)) - *(param_1 + 0x30)) -
            *(param_1 + 0x34);
    if (0x0 < iVar1) {
      *(param_1 + 0x24) = *(param_1 + 0x24) + iVar1;
    }
  }
  return;
}



byte *  FUN_004b76d3(byte *param_1,param_2: i32,byte param_3)

{
  let bVar1: u8;
  byte *pbVar2;
  let mut uVar3: u32;
  byte *pbVar4;
  
  uVar3 = 0xffffffff;
  pbVar2 = param_1;
  loop {
    if (uVar3 == 0x0) break;
    uVar3 = uVar3 - 0x1;
    bVar1 = *pbVar2;
    pbVar2 = pbVar2 + 0x1;
  } while (bVar1 != 0x0);
  pbVar2 = param_1 + (~uVar3 - 0x1);
  pbVar4 = pbVar2 + param_2;
  if (0x0 < param_2) {
    for (; param_1 <= pbVar2; pbVar2 = pbVar2 + -0x1) {
      *pbVar4 = *pbVar2;
      pbVar4 = pbVar4 + -0x1;
    }
    if (param_3 != 0xffffffff) {
      for (; 0x0 < param_2; param_2 = param_2 + -0x1) {
        *param_1 = param_3;
        param_1 = param_1 + 0x1;
      }
    }
  }
  return param_1;
}



fn FUN_004b7727(param_1: i32,byte *param_2)

{
  let bVar1: u8;
  let mut iVar2: i32;
  byte *pbVar3;
  byte *pbVar4;
  
  FUN_004b7690(param_1);
  iVar2 = *(param_1 + 0x2c) + *(param_1 + 0x20) + *(param_1 + 0x24) + *(param_1 + 0x28) +
          *(param_1 + 0x30) + *(param_1 + 0x34);
  if (0x27 < iVar2) {
    FUN_004a0430(param_2,0x2a,0x27);
    param_2[0x27] = 0x0;
    return;
  }
  if (*(param_1 + 0x20) == 0x0) {
    pbVar3 = param_2 + 0x1;
    pbVar4 = param_2;
    loop {
      bVar1 = *pbVar3;
      *pbVar4 = bVar1;
      if (bVar1 == 0x0) break;
      bVar1 = pbVar3[0x1];
      pbVar3 = pbVar3 + 0x2;
      pbVar4[0x1] = bVar1;
      pbVar4 = pbVar4 + 0x2;
    } while (bVar1 != 0x0);
  }
  iVar2 = *(param_1 + 0x4) - iVar2;
  *(param_1 + 0x4) = iVar2;
  if (((*(param_1 + 0x1e) & 0x8) == 0x0) && ((param_1 + 0x16) == ' ')) {
    param_2 = FUN_004b76d3(param_2,iVar2,0x20);
  }
  pbVar4 = FUN_004b76d3(param_2 + *(param_1 + 0x20),*(param_1 + 0x24),0x30);
  pbVar4 = FUN_004b76d3(pbVar4 + *(param_1 + 0x28),*(param_1 + 0x2c),0x30);
  pbVar4 = FUN_004b76d3(pbVar4 + *(param_1 + 0x30),*(param_1 + 0x34),0x30);
  if ((*(param_1 + 0x1e) & 0x8) != 0x0) {
    pbVar4 = FUN_004b76d3(pbVar4,*(param_1 + 0x4),0x20);
  }
  *pbVar4 = 0x0;
  return;
}



undefined8  FUN_004b781c(byte *param_1,double **param_2,param_3: *mut u32)

{
  let bVar1: u8;
  double *pdVar2;
  let mut iVar3: i32;
  let mut extraout_EDX: i32;
  let mut iVar4: i32;
  let extraout_var: u16;
  let unaff_EBX: *mut u32;
  let puVar6: *mut u32;
  let in_DS: u16;
  let mut local_88: u32;
  let uStack132: u8;
  let local_74: u8;
  ushort local_6a;
  let mut local_50: i32;
  let mut local_4c: u32;
  let mut local_48: u32;
  let mut local_44: u32;
  let mut local_40: u32;
  let mut local_3c: i32;
  let mut local_34: u32;
  let mut local_30: u32;
  let mut local_2c: u32;
  let mut local_28: u32;
  float10 local_24;
  let mut local_18: u32;
  let mut uStack20: u32;
  let uVar5: u16;
  
  puVar6 = param_3;
  if ((param_3 + 0x17) == '\0') {
    FUN_004a0430(&local_88,0x0,0x38);
    local_88 = *param_3;
    uStack132 = (param_3 + 0x1);
    puVar6 = &local_88;
    local_6a = (ushort)local_74;
    unaff_EBX = param_3;
  }
  bVar1 = *(puVar6 + 0x15);
  local_44 = bVar1;
  local_50 = puVar6[0x2];
  if ((bVar1 & 0x5f) == 0x47) {
    if (local_50 == 0x0) {
      local_50 = 0x1;
    }
    local_48 = 0x4;
    local_4c = 0x1;
    local_44 = bVar1 - 0x2;
  }
  else {
    if ((bVar1 & 0x5f) == 0x45) {
      local_48 = 0x1;
      local_4c = 0x1;
    }
    else {
      local_48 = 0x2;
      local_4c = 0x0;
    }
  }
  if ((*(puVar6 + 0x1e) & 0x1) != 0x0) {
    local_48 = local_48 | 0x10;
  }
  pdVar2 = *param_2;
  *param_2 = pdVar2 + 0x1;
  local_18 = *pdVar2;
  uStack20 = *(pdVar2 + 0x4);
  local_24 = (float10)*pdVar2;
  if (local_50 == -0x1) {
    local_50 = 0x6;
  }
  local_40 = 0x0;
  FUN_004b2008(&local_24,&local_50,(param_1 + 0x1));
  puVar6[0xa] = local_34;
  puVar6[0xb] = local_30;
  puVar6[0xc] = local_2c;
  puVar6[0xd] = local_28;
  if (local_3c < 0x0) {
    iVar3 = puVar6[0x8];
    iVar4 = iVar3 + 0x1;
    puVar6[0x8] = iVar4;
    param_1[iVar3] = 0x2d;
  }
  else {
    if ((*(puVar6 + 0x1e) & 0x4) == 0x0) {
      iVar4 = extraout_EDX;
      if ((*(puVar6 + 0x1e) & 0x2) != 0x0) {
        iVar3 = puVar6[0x8];
        iVar4 = iVar3 + 0x1;
        puVar6[0x8] = iVar4;
        param_1[iVar3] = 0x20;
      }
    }
    else {
      iVar3 = puVar6[0x8];
      iVar4 = iVar3 + 0x1;
      puVar6[0x8] = iVar4;
      param_1[iVar3] = 0x2b;
    }
  }
  uVar5 = (undefined2)(iVar4 >> 0x10);
  if ((puVar6 + 0x17) == '\0') {
    FUN_004b7727(puVar6,param_1);
    *unaff_EBX = local_88;
    (unaff_EBX + 0x1) = uStack132;
    (unaff_EBX + 0x5) = 0x0;
    unaff_EBX[0x2] = 0x0;
    uVar5 = extraout_var;
  }
  else {
    (puVar6 + 0x17) = 0x0;
  }
  return CONCAT44(CONCAT22(uVar5,in_DS),param_1);
}



fn FUN_004b79e0()

{
  FUN_004b67a0(s_Floating_point_support_not_loade_004c3c50,0x1);
  return;
}



// WARNING: Could not reconcile some variable overlaps

fn FUN_004b79f0(byte *param_1,param_2: *mut u32,byte **param_3) -> u32

{
  let bVar1: u8;
  byte *pbVar2;
  byte *pbVar3;
  let mut uVar4: u32;
  let bVar5: u8;
  let mut iVar6: i32;
  let mut iVar7: i32;
  let mut uVar8: u32;
  let mut iVar9: i32;
  let local_40: u8 [0x14];
  let local_2c: u8 [0x8];
  let mut local_24: u32;
  byte *local_20;
  byte *local_1c;
  let mut local_18: i32;
  let local_14: u8;
  
  local_20 = param_1;
  for (; (bVar5 = *param_1, bVar5 == 0x20 || ((0x8 < bVar5 && (bVar5 < 0xe)))); param_1 = param_1 + 0x1) {
  }
  local_14 = 0x0;
  pbVar2 = param_1 + 0x1;
  if ((bVar5 != 0x2b) && (pbVar2 = param_1, bVar5 == 0x2d)) {
    local_14 = 0x1;
    pbVar2 = param_1 + 0x1;
  }
  bVar5 = 0x30;
  iVar6 = 0x0;
  iVar9 = 0x0;
  while( true ) {
    while( true ) {
      bVar1 = *pbVar2;
      pbVar3 = pbVar2 + 0x1;
      if (bVar1 != 0x2e) break;
      if ((local_14 & 0x8) != 0x0)^ // goto LAB_004b7a78;
      local_14 = local_14 | 0x8;
      pbVar2 = pbVar3;
    }
    if ((bVar1 < 0x30) || (0x39 < bVar1)) break;
    if ((local_14 & 0x8) != 0x0) {
      iVar9 = iVar9 + 0x1;
    }
    bVar5 = bVar5 | bVar1;
    if (bVar5 != 0x30) {
      if (iVar6 < 0x13) {
        local_40[iVar6] = bVar1;
      }
      iVar6 = iVar6 + 0x1;
    }
    local_14 = local_14 | 0x4;
    pbVar2 = pbVar3;
  }
// LAB_004b7a78:
  iVar7 = 0x0;
  if (((local_14 & 0x4) != 0x0) && ((bVar1 == 0x65 || (local_20 = pbVar2, bVar1 == 0x45)))) {
    local_20 = pbVar2 + 0x2;
    if ((*pbVar3 != 0x2b) && (local_20 = pbVar3, *pbVar3 == 0x2d)) {
      local_14 = local_14 | 0x2;
      local_20 = pbVar2 + 0x2;
    }
    local_14 = local_14 & 0xfb;
    for (; (bVar5 = *local_20, 0x2f < bVar5 && (bVar5 < 0x3a)); local_20 = local_20 + 0x1) {
      if (iVar7 < 0x3e8) {
        local_18 = iVar7 * 0xa;
        iVar7 = bVar5 + local_18 + -0x30;
      }
      local_14 = local_14 | 0x4;
    }
    if ((local_14 & 0x2) != 0x0) {
      iVar7 = -iVar7;
    }
    local_1c = pbVar2;
    if ((local_14 & 0x4) == 0x0) {
      local_20 = pbVar2;
    }
  }
  if (param_3 != (byte **)0x0) {
    *param_3 = local_20;
  }
  uVar8 = iVar7 - iVar9;
  if (0x13 < iVar6) {
    uVar8 = uVar8 + iVar6 + -0x13;
    iVar6 = 0x13;
  }
  for (; (0x0 < iVar6 && ((&stack0xffffffbf)[iVar6] == '0')); iVar6 = iVar6 + -0x1) {
    uVar8 = uVar8 + 0x1;
  }
  if (iVar6 == 0x0) {
    *(param_2 + 0x2) = 0x0;
    param_2[0x1] = 0x0;
    *param_2 = 0x0;
    uVar4 = 0x0;
  }
  else {
    local_40[iVar6] = 0x0;
    FUN_004b9838(param_3,local_2c);
    if (uVar8 != 0x0) {
      FUN_004b1fa0((float10 *)local_2c,uVar8);
    }
    if ((local_14 & 0x1) != 0x0) {
      local_24._0_2_ = (ushort)local_24 | 0x8000;
    }
    (param_2 + 0x2) = (ushort)local_24;
    param_2[0x1] = local_2c._4_4_;
    *param_2 = local_2c._0_4_;
    iVar6 = (uVar8 - 0x1) + iVar6;
    if (iVar6 < 0x135) {
      if (iVar6 < -0x134) {
        uVar4 = 0x2;
      }
      else {
        uVar4 = 0x1;
      }
    }
    else {
      uVar4 = 0x3;
    }
  }
  return uVar4;
}



// WARNING: Could not reconcile some variable overlaps

double  FUN_004b7bb2(byte *param_1,byte **param_2)

{
  let mut iVar1: i32;
  let mut uVar2: u32;
  float10 extraout_ST0;
  float10 fVar3;
  undefined8 local_34;
  let uStack44: u8;
  let uStack43: u8;
  undefined8 local_28;
  let mut local_20: i32;
  let mut local_1c: u32;
  let mut local_18: u32;
  let mut local_14: u32;
  
  iVar1 = FUN_004b79f0(param_1,&local_34,param_2);
  fVar3 = extraout_ST0;
  if (iVar1 == 0x0)^ // goto LAB_004b7baa;
  uVar2 = (CONCAT11(uStack43,uStack44) & 0x7fff);
  if (0x43fe < uVar2) {
    fVar3 = (float10)FUN_004b175c();^
    // goto LAB_004b7baa;
  }
  if (uVar2 < 0x3bcd) {
    if (uVar2 < -0x34) {
      fVar3 = (float10)FUN_004b175c();^
      // goto LAB_004b7baa;
    }
    local_28 = (double)(float10)CONCAT19(uStack43,CONCAT18(uStack44,local_34));
    local_20 = local_28;
    local_28._4_4_ = ((ulonglong)local_28 >> 0x20);
    local_1c = local_28._4_4_;
    if ((((ulonglong)local_28 & 0x7fffffff00000000) != 0x0) || (local_28 != 0x0))^ // goto joined_r0x004b7c86;
  }
  else {
    local_28 = (double)(float10)CONCAT19(uStack43,CONCAT18(uStack44,local_34));
    if (iVar1 != 0x3) {
      local_28._4_4_ = ((ulonglong)local_28 >> 0x20);
      local_18 = local_28;
      local_14 = local_28._4_4_;
joined_r0x004b7c86:
      if (((ulonglong)local_28 & 0x7ff0000000000000) != 0x0)^ // goto LAB_004b7baa;
    }
  }
  fVar3 = (float10)FUN_004b175c();
// LAB_004b7baa:
  return (double)fVar3;
}



fn FUN_004b7c98(param_1: *mut i32) -> u32

{
  if (((param_1 + 0x2) & 0x7fff) == 0x7fff) {
    if ((param_1[0x1] == -0x80000000) && (*param_1 == 0x0)) {
      return 0x3;
    }
    return 0x2;
  }
  if (((param_1 + 0x2) & 0x7fff) != 0x0) {
    return 0x1;
  }
  if ((param_1[0x1] == 0x0) && (*param_1 == 0x0)) {
    return 0x0;
  }
  return 0x4;
}



char *  FUN_004b7ce6(param_1: &mut String,param_2: i32)

{
  let mut pcVar1: String; 
  
  if ((param_2 < 0x14) && (pcVar1 = param_1, ((&DAT_004bf9c4)[(byte)(*param_1 + 0x1)] & 0x20) != 0x0)) {
    for (; *pcVar1 != '\0'; pcVar1 = pcVar1 + 0x1) {
      param_2 = param_2 + -0x1;
    }
    for (; 0x0 < param_2; param_2 = param_2 + -0x1) {
      *pcVar1 = '0';
      pcVar1 = pcVar1 + 0x1;
    }
    *pcVar1 = '\0';
  }
  return param_1;
}



fn FUN_004b7d24(param_1: u32,param_2: u32,param_3: i32,param_4: *mut u32,param_5: *mut u32)

{
  let puVar1: *mut u32;
  
  puVar1 = FUN_004b98e0();
  puVar1 = FUN_004b98ec(param_1,param_2,param_3,param_4,param_5,0x47,puVar1);
  FUN_004b7ce6(puVar1,param_3);
  return;
}



fn FUN_004b7d60(param_1: u32,param_2: u32,param_3: i32,param_4: *mut i32,param_5: *mut u32)

{
  let puVar1: *mut u32;
  
  puVar1 = FUN_004b98e0();
  puVar1 = FUN_004b98ec(param_1,param_2,param_3,param_4,param_5,0x46,puVar1);
  FUN_004b7ce6(puVar1,param_3 + *param_4);
  return;
}



u32  conv_pchar_pwchar_004b7de0(Wparam_1: &mut String,byte *multi_byte_str,param_3: u32)

{
  let mut uVar1: u32;
  let mut multi_byte_str_len: u32;
  let mut chars_written: i32;
  WCHAR wide_char_str [0x2];
  
  if (multi_byte_str == 0x0) {
    multi_byte_str_len = 0x0;
  }
  else {
    if (param_3 != 0x0) {
      if (*multi_byte_str == 0x0) {
        if (param_1 != (WCHAR *)0x0) {
          *param_1 = L'\0';
        }
        return 0x0;
      }
      if (((DAT_005bac40 == 0x0) || (((&DAT_005bac51)[*multi_byte_str] & 0x1) == 0x0)) || (multi_byte_str[0x1] != 0x0))
      {
        uVar1 = FUN_004b75b0(multi_byte_str);
        multi_byte_str_len = uVar1;
        if (param_3 < uVar1) {
          multi_byte_str_len = param_3;
        }
                    // i32 cbMultiByte for MultiByteToWideChar
                    // LPCSTR lpMultiByteStr for MultiByteToWideChar
                    // DWORD dwFlags for MultiByteToWideChar
                    // UINT CodePage for MultiByteToWideChar
        chars_written =
             MultiByteToWideChar(code_page_004c0450,0x8,multi_byte_str,multi_byte_str_len,wide_char_str,0x1);
        if (chars_written != 0x0) {
          if (param_1 != (WCHAR *)0x0) {
            *param_1 = wide_char_str[0];
          }
          return uVar1;
        }
      }
    }
    multi_byte_str_len = 0xffffffff;
  }
  return multi_byte_str_len;
}



fn FUN_004b7e8c(param_1: u32,param_2: u32) -> u32

{
  let mut uVar1: u32;
  let mut in_EAX: u32;
  let mut uVar2: u32;
  let mut uVar3: u32;
  
  if ((param_2 & 0x7ff00000) != 0x0) {
    uVar1 = (CARRY4(param_2,param_2) || CARRY4(param_2 * 0x2,CARRY4(in_EAX,in_EAX))) << 0x1f;
    uVar2 = in_EAX * 0x2 + 0x20000000;
    uVar3 = param_2 * 0x2 + CARRY4(in_EAX,in_EAX) + (0xdfffffff < in_EAX * 0x2);
    if ((uVar3 == 0x0) || (0x8fdfffff < uVar3)) {
      return uVar1 | 0x7f800000;
    }
    if (0x701fffff < uVar3) {
      return ((uVar3 + 0x90000000) * 0x2 + CARRY4(uVar2,uVar2)) * 0x2 + CARRY4(uVar2 * 0x2,uVar2 * 0x2) |
             uVar1;
    }
  }
  return 0x0;
}



ulonglong  FUN_004b7ed8(param_1: i32,param_2: i32)

{
  let mut in_EAX: u32;
  let mut unaff_EBX: u32;
  
  if ((param_2 == 0x0) && (param_1 == 0x0)) {
    return (ulonglong)in_EAX * (ulonglong)unaff_EBX;
  }
  return (ulonglong)in_EAX * (ulonglong)unaff_EBX & 0xffffffff |
         (ulonglong)(((ulonglong)in_EAX * (ulonglong)unaff_EBX >> 0x20) + in_EAX * param_1 + param_2 * unaff_EBX)
         << 0x20;
}



fn FUN_004b7f20(param_1: *mut u32,param_2: u32)

{
  FUN_004b7f38(param_1,param_2);
  return;
}



fn FUN_004b7f38(param_1: *mut u32,param_2: u32) -> *mut u32

{
  let puVar1: *mut u32;
  let mut uVar2: u32;
  
  if (param_1 == 0x0) {
    puVar1 = FUN_004aac10(param_2);
    return puVar1;
  }
  if (param_2 == 0x0) {
    FUN_004aaaf0(param_1);
    return 0x0;
  }
  uVar2 = FUN_004b9970(param_1);
  puVar1 = FUN_004b9b54(param_1,param_2);
  if (puVar1 == 0x0) {
    puVar1 = FUN_004aac10(param_2);
    if (puVar1 == 0x0) {
      FUN_004b9b54(param_1,uVar2);
    }
    else {
      *puVar1 = *param_1;
      (puVar1 + 0x1) = (param_1 + 0x1);
      FUN_004aaaf0(param_1);
    }
  }
  return puVar1;
}



fn conv_pwchar_to_pchar_004b7fd0(LPSTR param_1,LPWSTR param_2) -> u32

{
  let chars_written: u32;
  
  if (param_1 == (LPSTR)0x0) {
    return (DWORD)param_1;
  }
                    // LPBOOL lpUsedDefaultChar for WideCharToMultiByte
                    // LPCSTR lpDefaultChar for WideCharToMultiByte
                    // i32 cbMultiByte for WideCharToMultiByte
                    // LPSTR lpMultiByteStr for WideCharToMultiByte
                    // i32 cchWideChar for WideCharToMultiByte
                    // LPCWSTR lpWideCharStr for WideCharToMultiByte
                    // DWORD dwFlags for WideCharToMultiByte
                    // UINT CodePage for WideCharToMultiByte
  chars_written =
       WideCharToMultiByte(code_page_004c0450,0x200,(LPCWSTR)&param_2,0x1,param_1,0x2,0x0,(LPBOOL)0x0);
  if ((LPSTR)chars_written != (LPSTR)0x0) {
    return chars_written;
  }
  return (DWORD)(LPSTR)0xffffffff;
}



char *  FUN_004b8010(param_1: u32,param_2: &mut String,param_3: u32)

{
  let cVar1: u8;
  let mut pcVar2: String; 
  let mut pcVar3: String; 
  let local_37: u8 [0x23];
  let mut local_14: u32;
  
  pcVar2 = local_37;
  loop {
    local_14 = param_1 / param_3;
    *pcVar2 = s_0123456789abcdefghijklmnopqrstuv_004c0364[param_1 % param_3];
    pcVar2 = pcVar2 + 0x1;
    param_1 = local_14;
    pcVar3 = param_2;
  } while (local_14 != 0x0);
  loop {
    cVar1 = pcVar2[-0x1];
    pcVar2 = pcVar2 + -0x1;
    *pcVar3 = cVar1;
    pcVar3 = pcVar3 + 0x1;
  } while (cVar1 != '\0');
  return param_2;
}



char *  FUN_004b8068(param_1: u32,param_2: &mut String,param_3: u32)

{
  let mut pcVar1: String; 
  
  pcVar1 = param_2;
  if ((param_3 == 0xa) && (param_1 < 0x0)) {
    param_1 = -param_1;
    pcVar1 = param_2 + 0x1;
    *param_2 = '-';
  }
  FUN_004b8010(param_1,pcVar1,param_3);
  return param_2;
}



// WARNING: Could not reconcile some variable overlaps

char *  FUN_004b80a0(longlong *param_1,param_2: &mut String,param_3: i32)

{
  let cVar1: u8;
  let mut iVar2: i32;
  let mut pcVar3: String; 
  longlong lVar4;
  let local_53: u8 [0x23];
  let mut local_30: i32;
  let mut local_28: i32;
  let mut local_24: u32;
  undefined8 local_20;
  let mut local_18: String; 
  
  local_18 = param_2;
  pcVar3 = local_53;
  local_20 = *param_1;
  local_28 = param_3;
  local_24 = 0x0;
  loop {
    iVar2 = local_28;
    local_20._4_4_ = ((ulonglong)local_20 >> 0x20);
    lVar4 = FUN_004b9bf3(local_24,local_20._4_4_);
    local_30 = iVar2;
    local_20 = lVar4;
    *pcVar3 = s_0123456789abcdefghijklmnopqrstuv_004c038c[iVar2];
    pcVar3 = pcVar3 + 0x1;
  } while (local_20 != 0x0);
  loop {
    cVar1 = pcVar3[-0x1];
    pcVar3 = pcVar3 + -0x1;
    *local_18 = cVar1;
    local_18 = local_18 + 0x1;
  } while (cVar1 != '\0');
  return param_2;
}



char *  FUN_004b81b0(param_1: u32,param_2: &mut String,param_3: u32)

{
  let cVar1: u8;
  let mut uVar2: u32;
  let mut pcVar3: String; 
  let mut pcVar4: String; 
  let mut pcVar5: String; 
  let local_37: u8 [0x27];
  
  pcVar3 = local_37;
  loop {
    uVar2 = param_1 / param_3;
    pcVar4 = pcVar3 + 0x1;
    *pcVar3 = s_0123456789abcdefghijklmnopqrstuv_004c03b4[param_1 % param_3];
    param_1 = uVar2;
    pcVar3 = pcVar4;
    pcVar5 = param_2;
  } while (uVar2 != 0x0);
  loop {
    cVar1 = pcVar4[-0x1];
    pcVar4 = pcVar4 + -0x1;
    *pcVar5 = cVar1;
    pcVar5 = pcVar5 + 0x1;
  } while (cVar1 != '\0');
  return param_2;
}



char *  FUN_004b8204(param_1: u32,param_2: &mut String,param_3: u32)

{
  let mut pcVar1: String; 
  
  pcVar1 = param_2;
  if ((param_3 == 0xa) && (param_1 < 0x0)) {
    param_1 = -param_1;
    pcVar1 = param_2 + 0x1;
    *param_2 = '-';
  }
  FUN_004b81b0(param_1,pcVar1,param_3);
  return param_2;
}



fn FUN_004b8240(param_1: i32,param_2: i32)

{
  let mut pCVar1: String;;
  
  pCVar1 = FUN_004aac00(param_1 * param_2);
  if (pCVar1 != 0x0) {
    FUN_004a0430(pCVar1,0x0,param_1 * param_2);
  }
  return;
}



LPVOID get_tls_val_004b8270(void)

{
  let piVar1: *mut i32;;
  LPVOID pvVar2;
  
  pvVar2 = (LPVOID)0x0;
  piVar1 = FUN_004b5768(0x0);
  if (piVar1 != 0x0) {
                    // DWORD dwTlsIndex for TlsGetValue
    pvVar2 = TlsGetValue(tls_index);
  }
  if (pvVar2 == (LPVOID)0x0) {
    FUN_004b67a0(s_Thread_has_no_thread_specific_da_004c3c74,0x1);
  }
  return pvVar2;
}



fn FUN_004b82a8() -> *mut u32

{
  let puVar1: *mut u32;
  let puVar2: *mut u32;
  let DVar3: u32;
  let lpTlsValue: *mut u32;
  
  (PTR_FUN_004bfba0)();
  DVar3 = GetCurrentThreadId();
  for (puVar1 = DAT_005bad60; (puVar1 != 0x0 && (DVar3 != puVar1[0x1])); puVar1 = *puVar1) {
  }
  if (puVar1[0x3] == 0x0) {
    lpTlsValue = FUN_004b8240(0x1,DAT_004c0338);
    if (lpTlsValue == 0x0) {
      FUN_004b67a0(s_Unable_to_resize_thread_specific_004c3cc4,0x1);
    }
    puVar2 = puVar1[0x2];
    *lpTlsValue = *puVar2;
    (lpTlsValue + 0x1) = (puVar2 + 0x1);
    puVar1[0x3] = 0x1;
  }
  else {
    lpTlsValue = FUN_004b7f20(puVar1[0x2],DAT_004c0338);
    if (lpTlsValue == 0x0) {
      FUN_004b67a0(s_Unable_to_resize_thread_specific_004c3c9c,0x1);
    }
  }
  puVar1[0x2] = lpTlsValue;
                    // LPVOID lpTlsValue for TlsSetValue
  lpTlsValue[0x3c] = DAT_004c0338;
  DVar3 = tls_index;
  (lpTlsValue + 0x52) = 0x1;
                    // DWORD dwTlsIndex for TlsSetValue
  (lpTlsValue + 0x53) = 0x0;
  TlsSetValue(DVar3,lpTlsValue);
  (PTR_FUN_004bfba4)();
  return lpTlsValue;
}



fn FUN_004b8380(param_1: u32,param_2: i32) -> u32

{
  let puVar1: *mut u32;
  let mut iVar2: i32;
  let mut uVar3: u32;
  
  (PTR_FUN_004bfba0)();
  uVar3 = 0x1;
  puVar1 = FUN_004b8240(0x1,0x10);
  if (puVar1 == 0x0) {
    uVar3 = 0x0;
  }
  else {
    iVar2 = FUN_004b9cc0(param_2);
    if (iVar2 == 0x0) {
      puVar1[0x2] = param_2;
      puVar1[0x1] = param_1;
      puVar1[0x3] = *(param_2 + 0x52);
      *puVar1 = DAT_005bad60;
      DAT_005bad60 = puVar1;
    }
    else {
      FUN_004aaae0(puVar1);
      uVar3 = 0x0;
    }
  }
  (PTR_FUN_004bfba4)();
  return uVar3;
}



fn FUN_004b83ec(param_1: i32)

{
  let puVar1: *mut u32;
  let puVar2: *mut u32;
  
  (PTR_FUN_004bfba0)();
  puVar1 = &DAT_005bad60;
  puVar2 = DAT_005bad60;
  loop {
    if (puVar2 == 0x0) {
// LAB_004b8436:
      (PTR_FUN_004bfba4)();
      return;
    }
    if (param_1 == puVar2[0x1]) {
      if (puVar2[0x3] != 0x0) {
        FUN_004aaae0(puVar2[0x2]);
      }
      *puVar1 = *puVar2;
      FUN_004aaae0(puVar2);^
      // goto LAB_004b8436;
    }
    puVar1 = puVar2;
    puVar2 = *puVar2;
  } while( true );
}



fn FUN_004b8440()

{
  let puVar1: *mut u32;
  
  (PTR_FUN_004bfba0)();
  for (puVar1 = DAT_005bad60; puVar1 != 0x0; puVar1 = *puVar1) {
    (puVar1[0x2] + 0x53) = 0x1;
  }
  (PTR_FUN_004bfba4)();
  return;
}



fn FUN_004b8464()

{
  let piVar1: *mut i32;;
  let piVar2: *mut i32;;
  
  piVar2 = DAT_005bad60;
  if (DAT_005bad60 != 0x0) {
    loop {
      piVar1 = *piVar2;
      if (piVar2[0x3] != 0x0) {
        FUN_004aaae0(piVar2[0x2]);
      }
      FUN_004aaae0(piVar2);
      piVar2 = piVar1;
    } while (piVar1 != 0x0);
  }
  return;
}



fn FUN_004b84a0(param_1: *mut i32)

{
  let DVar1: u32;
  
  if (param_1 != 0x0) {
    param_1[0x3] = 0x1;
    query_virt_mem_004b5b30(param_1,0x0);
    DVar1 = GetCurrentThreadId();
    *(DWORD *)(param_1 + 0xda) = DVar1;
  }
  return;
}



void thunk_FUN_004b8464()

{
  let piVar1: *mut i32;;
  let piVar2: *mut i32;;
  
  piVar2 = DAT_005bad60;
  if (DAT_005bad60 != 0x0) {
    loop {
      piVar1 = *piVar2;
      if (piVar2[0x3] != 0x0) {
        FUN_004aaae0(piVar2[0x2]);
      }
      FUN_004aaae0(piVar2);
      piVar2 = piVar1;
    } while (piVar1 != 0x0);
  }
  return;
}



fn FUN_004b84e0(param_1: u32)

{
  FUN_004aac00(param_1);
  return;
}



fn FUN_004b8630(short *param_1) -> String

{
  short *psVar1;
  let sVar2: i16;
  short *psVar3;
  
  sVar2 = *param_1;
  psVar3 = param_1;
  while (sVar2 != 0x0) {
    psVar1 = psVar3 + 0x1;
    psVar3 = psVar3 + 0x1;
    sVar2 = *psVar1;
  }
  return (psVar3 - param_1 >> 0x1);
}



fn FUN_004b8660(param_1: *mut u32,param_2: *mut u32) -> *mut u32

{
  *param_1 = *param_2;
  (param_1 + 0x1) = (param_2 + 0x1);
  return param_1;
}



fn FUN_004b8690(short *param_1) -> u32

{
  if (((*param_1 == 0x1) && (*(param_1 + 0x2) != 0x0)) &&
     (((ushort)param_1[0x5] < 0x10 || (0x12 < (ushort)param_1[0x5])))) {
    return 0x1;
  }
  return 0x0;
}



void open_files_004b86bc(void)

{
  (PTR_FUN_004bfb78)(0x0);
  if (DAT_004c03dc == (HANDLE)0xffffffff) {
                    // HANDLE hTemplateFile for CreateFileA
                    // DWORD dwFlagsAndAttributes for CreateFileA
                    // DWORD dwCreationDisposition for CreateFileA
                    // LPSECURITY_ATTRIBUTES lpSecurityAttributes for CreateFileA
                    // DWORD dwShareMode for CreateFileA
                    // DWORD dwDesiredAccess for CreateFileA
                    // LPCSTR lpFileName for CreateFileA
    DAT_004c03dc = CreateFileA(s_conin__004c3cec,0x80000000,0x1,(LPSECURITY_ATTRIBUTES)0x0,0x3,0x80,(HANDLE)0x0);
  }
  if (DAT_004c03e0 == (HANDLE)0xffffffff) {
                    // HANDLE hTemplateFile for CreateFileA
                    // DWORD dwFlagsAndAttributes for CreateFileA
                    // DWORD dwCreationDisposition for CreateFileA
                    // LPSECURITY_ATTRIBUTES lpSecurityAttributes for CreateFileA
                    // DWORD dwShareMode for CreateFileA
                    // DWORD dwDesiredAccess for CreateFileA
                    // LPCSTR lpFileName for CreateFileA
    DAT_004c03e0 = CreateFileA(s_conout__004c3cf4,0x40000000,0x2,(LPSECURITY_ATTRIBUTES)0x0,0x3,0x80,(HANDLE)0x0);
  }
  (PTR_FUN_004bfb7c)(0x0);
  return;
}



fn FUN_004b8730() -> u32

{
  open_files_004b86bc();
  return DAT_004c03dc;
}



fn FUN_004b873c() -> u32

{
  open_files_004b86bc();
  return DAT_004c03e0;
}



fn FUN_004b8750()

{
  return;
}



fn FUN_004b8760(param_1: i32,param_2: u32) -> u32

{
  let mut uVar1: u32;
  let mut iVar2: i32;
  
  if ((param_1 != 0x7) && (param_1 != 0x4)) {
    iVar2 = (PTR_FUN_004bfb74)();
    uVar1 = *(param_1 * 0x8 + 0x58 + iVar2);
    iVar2 = (PTR_FUN_004bfb74)();
    *(iVar2 + param_1 * 0x8 + 0x58) = param_2;
    return uVar1;
  }
  uVar1 = *(&DAT_004c03e4 + param_1 * 0x8);
  *(&DAT_004c03e4 + param_1 * 0x8) = param_2;
  return uVar1;
}



fn FUN_004b87b4(param_1: i32) -> u32

{
  let mut iVar1: i32;
  
  if ((param_1 != 0x7) && (param_1 != 0x4)) {
    iVar1 = (PTR_FUN_004bfb74)();
    return *(iVar1 + 0x58 + param_1 * 0x8);
  }
  return *(&DAT_004c03e4 + param_1 * 0x8);
}



fn FUN_004b87d8(param_1: i32) -> u32

{
  let mut iVar1: i32;
  
  if ((param_1 != 0x7) && (param_1 != 0x4)) {
    iVar1 = (PTR_FUN_004bfb74)();
    return *(iVar1 + 0x5c + param_1 * 0x8);
  }
  return *(&DAT_004c03e8 + param_1 * 0x8);
}



// HandlerRoutine parameter of SetConsoleCtrlHandler
// 

u32 HandlerRoutine_004b8820(param_1: i32)

{
  let mut iVar1: i32;
  
  if (param_1 == 0x0) {
    iVar1 = FUN_004b87b4(0x4);
    if (iVar1 == 0x0) {
      return 0x0;
    }
    FUN_004b8a28(0x4);
  }
  else {
    if (param_1 != 0x1) {
      return 0x0;
    }
    iVar1 = FUN_004b87b4(0x7);
    if (iVar1 == 0x0) {
      return 0x0;
    }
    FUN_004b8a28(0x7);
  }
  if ((iVar1 != 0x2) && (iVar1 != 0x3)) {
    return 0x1;
  }
  return 0x0;
}



fn FUN_004b8880() -> u32

{
  let mut iVar1: i32;
  let mut iVar2: i32;
  
  iVar1 = FUN_004b87b4(0x4);
  iVar2 = FUN_004b87b4(0x7);
  if (((iVar1 == 0x2) || (iVar1 == 0x3)) && ((iVar2 == 0x2 || (iVar2 == 0x3)))) {
    return 0x0;
  }
  return 0x1;
}



char set_console_ctrl_handler_004b88b8(void)

{
  let mut BVar1: bool;
  
  if (DAT_004c044c == '\0') {
                    // BOOL Add for SetConsoleCtrlHandler
                    // PHANDLER_ROUTINE HandlerRoutine for SetConsoleCtrlHandler
    BVar1 = SetConsoleCtrlHandler(HandlerRoutine_004b8820,0x1);
    if (BVar1 != 0x0) {
      DAT_004c044c = '\x01';
    }
  }
  return DAT_004c044c;
}



bool set_console_ctrl_handler_004b88e4(void)

{
  let mut BVar1: bool;
  
  if (DAT_004c044c != '\0') {
                    // BOOL Add for SetConsoleCtrlHandler
                    // PHANDLER_ROUTINE HandlerRoutine for SetConsoleCtrlHandler
    BVar1 = SetConsoleCtrlHandler(HandlerRoutine_004b8820,0x0);
    if (BVar1 != 0x0) {
      DAT_004c044c = '\0';
    }
  }
  return DAT_004c044c == '\0';
}



fn FUN_004b8928(param_1: u32) -> u32

{
  code *pcVar1;
  
  pcVar1 = (code *)FUN_004b87b4(0x2);
  if (((pcVar1 != (code *)0x1) && (pcVar1 != (code *)0x2)) && (pcVar1 != (code *)0x3)) {
    FUN_004b8760(0x2,0x2);
    (*pcVar1)(0x2,param_1);
    return 0x0;
  }
  return 0xffffffff;
}



fn FUN_004b8968(param_1: i32,param_2: i32) -> u32

{
  let mut iVar1: i32;
  let mut uVar2: u32;
  
  if ((0x0 < param_1) && (param_1 < 0xd)) {
    PTR_FUN_004bffbc = &LAB_004b891c;
    if ((param_2 != 0x2) && (param_2 != 0x3)) {
      iVar1 = FUN_004b87d8(param_1);
      if ((iVar1 != 0x0) && (param_1 == 0x2)) {
        FUN_004b9f00(0x0,0x9f);
      }
    }
    uVar2 = FUN_004b87b4(param_1);
    FUN_004b8760(param_1,param_2);
    iVar1 = FUN_004b8880();
    if (iVar1 == 0x0) {
      set_console_ctrl_handler_004b88e4();
    }
    else {
      set_console_ctrl_handler_004b88b8();
    }
    return uVar2;
  }
  FUN_004b1740(0x9);
  return 0x3;
}



fn FUN_004b8a28(param_1: i32) -> u32

{
  code *pcVar1;
  let mut iVar2: i32;
  
  pcVar1 = (code *)FUN_004b87b4(param_1);
  switch(param_1) {
  case 0x1:
    break;
  case 0x2:
    FUN_004b8928(0x8c);
    return 0x0;
  case 0x3:
  case 0x4:
  case 0x5:
  case 0x6:
  case 0x7:
  case 0x8:
  case 0x9:
  case 0xa:
  case 0xb:
  case 0xc:^
    // goto switchD_004b8a41_caseD_3;
  default:
    return 0xffffffff;
  }
  if (pcVar1 == (code *)0x2) {
    FUN_004b0ef8();
  }
switchD_004b8a41_caseD_3:
  if (((pcVar1 != (code *)0x1) && (pcVar1 != (code *)0x2)) && (pcVar1 != (code *)0x3)) {
    FUN_004b8760(param_1,0x2);
    (*pcVar1)(param_1);
  }
  iVar2 = FUN_004b8880();
  if (iVar2 != 0x0) {
    return 0x0;
  }
  set_console_ctrl_handler_004b88e4();
  return 0x0;
}



fn FUN_004b8b30(byte *param_1,byte *param_2,param_3: i32) -> i32

{
  let mut iVar1: i32;
  let mut iVar2: i32;
  
  iVar2 = 0x0;
  if (param_3 != 0x0) {
    loop {
      iVar1 = FUN_004b7570(param_1);
      if ((iVar1 != 0x0) || (iVar1 = FUN_004b7570(param_2), iVar1 != 0x0)) break;
      iVar2 = FUN_004b9f50(param_1,param_2);
      if (iVar2 != 0x0) {
        return iVar2;
      }
      param_1 = FUN_004b1c70(param_1);
      param_3 = param_3 + -0x1;
      param_2 = FUN_004b1c70(param_2);
    } while (param_3 != 0x0);
  }
  if ((param_3 != 0x0) &&
     ((iVar1 = FUN_004b7570(param_1), iVar1 != 0x0 || (iVar1 = FUN_004b7570(param_2), iVar1 != 0x0)))) {
    return *param_1 - *param_2;
  }
  return iVar2;
}



fn FUN_004b8bc0(short *param_1,param_2: i32)

{
  let bVar1: u8;
  short *psVar2;
  let mut iVar3: i32;
  let sVar4: i16;
  let mut iVar5: i32;
  short *psVar6;
  let mut uVar7: u32;
  
  psVar6 = param_1 + 0x1;
  iVar5 = 0x0;
  for (sVar4 = *param_1; sVar4 != 0x0; sVar4 = sVar4 + -0x1) {
    uVar7 = *(psVar6 + 0x1);
    bVar1 = *psVar6;
    psVar6 = psVar6 + 0x1;
    if (uVar7 == 0x0) {
      uVar7 = 0x100;
    }
    if (param_2 != 0x0) {
      iVar3 = 0x0;
      psVar2 = psVar6;
      if (uVar7 * 0x3 != 0x0) {
        loop {
          iVar3 = iVar3 + 0x1;
          *psVar2 = (byte)(*psVar2 >> 0x2);
          psVar2 = (short *)(psVar2 + 0x1);
        } while (iVar3 < (uVar7 * 0x3));
      }
    }
    FUN_00496263(psVar6,iVar5 + bVar1,uVar7);
    iVar5 = iVar5 + bVar1 + uVar7;
    psVar6 = (short *)(psVar6 + uVar7 * 0x3);
  }
  return;
}



fn FUN_004b8c40(param_1: &mut String,param_2: i32)

{
  ushort uVar1;
  let mut iVar2: i32;
  let puVar3: *mut u32;
  let mut iVar4: i32;
  let mut iVar5: i32;
  let mut uVar6: u32;
  let mut iVar7: i32;
  let mut bVar8: bool;
  
  uVar1 = (param_2 + 0x8);
  iVar2 = *(param_2 + 0x8c);
  uVar6 = (param_2 + 0xa);
  bVar8 = uVar6 != 0x0;
  if (bVar8) {
    iVar4 = *(param_2 + 0x90);
    while (bVar8) {
      param_1 = param_1 + 0x1;
      iVar7 = *(param_2 + 0x8c);
      loop {
        iVar5 = *param_1;
        puVar3 = (param_1 + 0x1);
        if (iVar5 < 0x0) {
          iVar5 = -iVar5;
          FUN_00496ac0(puVar3,iVar7,iVar4,iVar5,0x1);
          param_1 = (puVar3 + iVar5);
        }
        else {
          param_1 = param_1 + 0x2;
          FUN_004968e7(iVar7,iVar4,iVar5,0x1,puVar3);
        }
        iVar7 = iVar7 + iVar5;
      } while (iVar7 < (iVar2 + uVar1));
      iVar4 = iVar4 + 0x1;
      uVar6 = uVar6 - 0x1;
      bVar8 = uVar6 != 0x0;
    }
  }
  return;
}



fn FUN_004b8cd0(short *param_1,param_2: i32)

{
  let mut iVar1: i32;
  let mut iVar2: i32;
  let puVar3: *mut u32;
  short *psVar4;
  let mut uVar5: u32;
  let mut iVar6: i32;
  let mut iVar7: i32;
  
  psVar4 = param_1 + 0x2;
  iVar2 = *(param_2 + 0x90) + *param_1;
  for (iVar1 = param_1[0x1]; iVar1 != 0x0; iVar1 = iVar1 + -0x1) {
    iVar7 = *(param_2 + 0x8c);
    uVar5 = *psVar4;
    psVar4 = (short *)(psVar4 + 0x1);
    for (; uVar5 != 0x0; uVar5 = uVar5 - 0x1) {
      iVar6 = (char)*(psVar4 + 0x1);
      puVar3 = (psVar4 + 0x1);
      iVar7 = iVar7 + *psVar4;
      if (iVar6 < 0x0) {
        iVar6 = -iVar6;
        psVar4 = (short *)(psVar4 + 0x3);
        FUN_004968e7(iVar7,iVar2,iVar6,0x1,puVar3);
      }
      else {
        FUN_00496ac0(puVar3,iVar7,iVar2,iVar6,0x1);
        psVar4 = (short *)(puVar3 + iVar6);
      }
      iVar7 = iVar7 + iVar6;
    }
    iVar2 = iVar2 + 0x1;
  }
  return;
}



fn FUN_004b8d80(short *param_1,param_2: i32)

{
  ushort uVar1;
  let mut iVar2: i32;
  let sVar3: i16;
  let puVar4: *mut u32;
  short *psVar5;
  let mut iVar6: i32;
  let mut iVar7: i32;
  let mut uVar8: u32;
  let mut iVar9: i32;
  let local_18: i16;
  let sStack22: i16;
  
  uVar1 = (param_2 + 0x8);
  iVar2 = *(param_2 + 0x8c);
  psVar5 = param_1 + 0x1;
  local_18 = *param_1;
  iVar6 = *(param_2 + 0x90);
  sStack22 = local_18 >> 0xf;
  if (local_18 != 0x0) {
    loop {
      while( true ) {
        sVar3 = *psVar5;
        uVar8 = SEXT24(sVar3);
        psVar5 = psVar5 + 0x1;
        if ((uVar8 & 0xc000) == 0xc000) break;
        if ((uVar8 & 0x8000) == 0x0) {
          iVar9 = *(param_2 + 0x8c);
          for (; sVar3 != 0x0; sVar3 = sVar3 + -0x1) {
            iVar7 = (char)*(psVar5 + 0x1);
            puVar4 = (psVar5 + 0x1);
            iVar9 = iVar9 + *psVar5;
            if (iVar7 < 0x0) {
              iVar7 = iVar7 * -0x2;
              psVar5 = psVar5 + 0x2;
              FUN_00497c58(iVar9,iVar6,iVar9 + iVar7 + -0x1,iVar6,puVar4,0x0,0x2);
            }
            else {
              iVar7 = iVar7 * 0x2;
              FUN_00496ac0(puVar4,iVar9,iVar6,iVar7,0x1);
              psVar5 = (short *)(puVar4 + iVar7);
            }
            iVar9 = iVar9 + iVar7;
          }
          iVar9 = CONCAT22(sStack22,local_18) + -0x1;
          iVar6 = iVar6 + 0x1;
          local_18 = iVar9;
          sStack22 = (iVar9 >> 0x10);
          if (iVar9 == 0x0) {
            return;
          }
        }
        else {
          FUN_00496a3b(uVar1 + iVar2 + -0x1,iVar6,(char)sVar3);
          if (CONCAT22(sStack22,local_18) == 0x0) {
            return;
          }
        }
      }
      iVar6 = iVar6 - uVar8;
    } while (CONCAT22(sStack22,local_18) != 0x0);
  }
  return;
}



fn FUN_004b8ea0(param_1: *mut u32)

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  let mut iVar3: i32;
  let mut iVar4: i32;
  let mut iVar5: i32;
  let piVar6: *mut i32;;
  let piVar7: *mut i32;;
  let mut iVar8: i32;
  let mut iVar9: i32;
  let piVar10: *mut i32;;
  let mut local_14: i32;
  
  DAT_005bad8c = FUN_0049c2c9(0x30c);
  if (DAT_005bad8c == 0x0) {
    return;
  }
  FUN_004a0430(DAT_005bad8c,0x0,0x30c);
  *(DAT_005bad8c + 0x300) = 0x7fffffff;
  iVar4 = 0x0;
  loop {
    iVar4 = iVar4 + 0xc;
    uVar1 = *param_1;
    param_1 = param_1 + 0x1;
    *(DAT_005bad8c + -0xc + iVar4) = uVar1;
  } while (iVar4 != 0x180);
  DAT_005bad90 = 0x20;
  loop {
    local_14 = 0x0;
    iVar3 = 0x0;
    iVar8 = 0x40;
    iVar9 = 0x40;
    while( true ) {
      iVar5 = iVar3;
      if (DAT_005bad90 <= iVar5) break;
      uVar2 = *(local_14 + DAT_005bad8c);
      if (uVar2 == 0x0) {
// LAB_004b8f48:
        local_14 = local_14 + 0xc;
        iVar3 = iVar5 + 0x1;
      }
      else {
        if (uVar2 < *(DAT_005bad8c + iVar9 * 0xc)) {
          local_14 = local_14 + 0xc;
          iVar3 = iVar5 + 0x1;
          iVar8 = iVar9;
          iVar9 = iVar5;
        }
        else {
          if (*(iVar8 * 0xc + DAT_005bad8c) <= uVar2)^ // goto LAB_004b8f48;
          local_14 = local_14 + 0xc;
          iVar3 = iVar5 + 0x1;
          iVar8 = iVar5;
        }
      }
    }
    if (iVar8 == 0x40) {
      DAT_005bad90 = DAT_005bad90 + -0x1;
      return;
    }
    piVar10 = (iVar9 * 0xc + DAT_005bad8c);
    piVar6 = (iVar8 * 0xc + DAT_005bad8c);
    piVar7 = (DAT_005bad8c + iVar4);
    *piVar7 = *piVar10 + *piVar6;
    *piVar10 = 0x0;
    *piVar6 = 0x0;
    piVar7[0x1] = iVar9;
    piVar7[0x2] = iVar8;
    iVar4 = iVar4 + 0xc;
    DAT_005bad90 = DAT_005bad90 + 0x1;
  } while( true );
}



fn FUN_004b8fe0()

{
  let mut iVar1: i32;
  
  iVar1 = DAT_005bad90;
  loop {
    DAT_005bad84 = DAT_005bad84 >> 0x1;
    if (DAT_005bad84 == 0x0) {
      DAT_005bad84 = 0x80;
      DAT_005bad88 = DAT_005bad88 + 0x1;
    }
    if ((*DAT_005bad88 & DAT_005bad84) == 0x0) {
      iVar1 = *(iVar1 * 0xc + 0x4 + DAT_005bad8c);
    }
    else {
      iVar1 = *(DAT_005bad8c + 0x8 + iVar1 * 0xc);
    }
  } while (0x1f < iVar1);
  return;
}



fn FUN_004b9280(byte *param_1,param_2: i32)

{
  short *psVar1;
  byte *pbVar2;
  let mut iVar3: i32;
  undefined1 *puVar4;
  let bVar5: u8;
  let mut uVar6: u32;
  let mut iVar7: i32;
  byte *pbVar8;
  let mut uVar9: u32;
  let mut iVar10: i32;
  short *psVar11;
  let sVar12: i16;
  byte *pbVar13;
  undefined1 *puVar14;
  let mut uVar15: u32;
  let mut uVar16: u32;
  let mut iVar17: i32;
  let mut iVar18: i32;
  let mut uVar19: u32;
  let mut uVar20: u32;
  let mut uVar21: u32;
  let uStack32: u16;
  let local_1e: u16;
  
  iVar10 = *(param_2 + 0x90);
  psVar11 = (short *)(param_1 + 0x1);
  uVar15 = *param_1;
  FUN_004953d7();
  while (uVar15 != 0x0) {
    sVar12 = *psVar11;
    psVar11 = psVar11 + 0x1;
    if (sVar12 < 0x0) {
      iVar10 = iVar10 + sVar12 * -0x2;
    }
    else {
      iVar17 = *(param_2 + 0x8c);
      iVar3 = DAT_005bad80;
      for (; DAT_005bad80 = iVar3, sVar12 != 0x0; sVar12 = sVar12 + -0x1) {
        pbVar13 = &DAT_005bad98;
        iVar17 = iVar17 + *psVar11 * 0x2;
        psVar1 = psVar11 + 0x1;
        uVar6 = *(psVar11 + 0x1);
        if (DAT_005bad8c == 0x0) {
          pbVar13 = 0x0;
        }
        else {
          DAT_005bad84 = 0x100;
          iVar18 = uVar6 * 0x6;
          DAT_005bad88 = psVar1;
          while (iVar18 != 0x0) {
            iVar7 = FUN_004b8fe0();
            if (iVar7 < 0x10) {
              if (iVar7 == 0x0) {
                iVar7 = 0x13;
              }
              else {
                iVar7 = iVar7 + 0x3;
              }
              bVar5 = FUN_004b8fe0();
              iVar18 = iVar18 - iVar7;
              if (iVar7 != 0x0) {
                pbVar8 = pbVar13;
                loop {
                  pbVar13 = pbVar8 + 0x1;
                  *pbVar8 = bVar5 & 0x1f;
                  iVar7 = iVar7 + -0x1;
                  pbVar8 = pbVar13;
                } while (iVar7 != 0x0);
              }
            }
            else {
              iVar7 = iVar7 + -0xf;
              iVar18 = iVar18 - iVar7;
              for (; iVar7 != 0x0; iVar7 = iVar7 + -0x1) {
                bVar5 = FUN_004b8fe0();
                *pbVar13 = bVar5 & 0x1f;
                pbVar13 = pbVar13 + 0x1;
              }
            }
          }
          puVar14 = &DAT_005bb518;
          pbVar8 = &DAT_005bad98;
          pbVar13 = &DAT_005bad98 + uVar6 * 0x4;
          uStack32 = SUB42(pbVar13 + uVar6,0x0);
          local_1e = (undefined2)((pbVar13 + uVar6) >> 0x10);
          uVar16 = 0x0;
          puVar4 = &DAT_005bb518 + uVar6 * 0x2;
          for (uVar9 = uVar6; uVar9 != 0x0; uVar9 = uVar9 - 0x1) {
            uVar19 = (*pbVar13 + ((uVar16 << 0x6) >> 0xb & 0x1f) & 0x1f) << 0x5;
            pbVar2 = CONCAT22(local_1e,uStack32);
            iVar18 = CONCAT22(local_1e,uStack32) + 0x1;
            uStack32 = (undefined2)iVar18;
            local_1e = (undefined2)(iVar18 >> 0x10);
            uVar20 = (((uVar16 & 0xfffffc1f | uVar19) * 0x2 >> 0xb & 0x1f) + *pbVar2 & 0x1f) << 0xa;
            uVar16 = (uVar16 & 0x1f) + *pbVar8 & 0x1f;
            *puVar14 = ((uVar19 | uVar20 | uVar16) + iVar3);
            uVar16 = pbVar8[0x1] + uVar16 & 0x1f;
            puVar14[0x1] = ((uVar19 | uVar20 | uVar16) + iVar3);
            uVar16 = uVar16 + pbVar8[0x2] & 0x1f;
            *puVar4 = ((uVar19 | uVar20 | uVar16) + iVar3);
            uVar21 = pbVar8[0x3] + uVar16 & 0x1f;
            uVar16 = uVar19 | uVar20 | uVar21;
            puVar14 = puVar14 + 0x2;
            puVar4[0x1] = ((uVar19 | uVar20 | uVar21) + iVar3);
            pbVar8 = pbVar8 + 0x4;
            pbVar13 = pbVar13 + 0x1;
            puVar4 = puVar4 + 0x2;
          }
          FUN_00496ac0(&DAT_005bb518,iVar17,iVar10,uVar6 * 0x2,0x2);
          pbVar13 = (DAT_005bad88 + (0x1 - psVar1));
        }
        psVar11 = (short *)(psVar1 + pbVar13);
        iVar17 = iVar17 + uVar6 * 0x2;
        iVar3 = DAT_005bad80;
      }
      iVar10 = iVar10 + 0x2;
      uVar15 = uVar15 - 0x1;
    }
  }
  FUN_0049536f();
  return;
}



fn FUN_004b95c0(param_1: *mut u32) -> u32

{
  let puVar1: *mut u32;
  
  DAT_005bad80 = FUN_0049c2c9(0x8000);
  if (DAT_005bad80 == 0x0) {
    return 0x0;
  }
  puVar1 = DAT_005bad80 + 0x1;
  *DAT_005bad80 = *param_1;
  puVar1 = (param_1 + 0x1);
  return 0x8000;
}



fn FUN_004b9600(param_1: *mut u32,ushort *param_2,param_3: u32) -> u32

{
  let DVar1: u32;
  let mut unaff_EBP: u32;
  let DVar2: u32;
  let mut local_14: u32;
  
  DVar2 = 0x0;
  if (param_1 == 0x0) {
    while ((DVar1 = DVar2, *param_2 != 0x0 &&
           (DVar1 = conv_pwchar_to_pchar_004b7fd0((LPSTR)&local_14,(LPWSTR)*param_2), DVar1 != 0xffffffff))) {
      param_2 = param_2 + 0x1;
      DVar2 = DVar2 + DVar1;
    }
  }
  else {
    for (; DVar1 = DVar2, param_3 != 0x0; param_3 = param_3 - DVar1) {
      if (*param_2 == 0x0) {
        param_1 = 0x0;
        return DVar2;
      }
      DVar1 = conv_pwchar_to_pchar_004b7fd0((LPSTR)&local_14,(LPWSTR)*param_2);
      if (DVar1 == 0xffffffff) {
        return 0xffffffff;
      }
      if (param_3 < DVar1) {
        return DVar2;
      }
      *param_1 = local_14;
      (param_1 + 0x1) = (char)unaff_EBP;
      param_2 = param_2 + 0x1;
      DVar2 = DVar2 + DVar1;
      param_1 = (param_1 + DVar1);
    }
  }
  return DVar1;
}



fn FUN_004b96d0(UINT param_1) -> u32

{
  let mut iVar1: i32;
  let mut BVar2: bool;
  let mut uVar3: u32;
  let mut iVar4: i32;
  _cpinfo _Stack32;
  
  if (param_1 == 0xffffffff) {
    param_1 = GetACP();
  }
  else {
    if (param_1 == 0xfffffffe) {
      param_1 = GetOEMCP();
    }
    else {
      if (param_1 == 0xfffffffd) {
        FUN_004a0430(&DAT_005bac50,0x0,0x101);
        code_page_004c0450 = 0x0;
        DAT_005bac40 = 0x0;
        return 0x0;
      }
      if (param_1 == 0xfffffffc) {
        FUN_004a0430(&DAT_005bac50,0x0,0x101);
        iVar4 = 0x81;
        loop {
          iVar1 = iVar4 + 0x1;
          (&DAT_005bac51)[iVar4] = 0x1;
          iVar4 = iVar1;
        } while (iVar1 < 0xa0);
        iVar4 = 0xe0;
        loop {
          iVar1 = iVar4 + 0x1;
          (&DAT_005bac51)[iVar4] = 0x1;
          iVar4 = iVar1;
        } while (iVar1 < 0xfd);
        code_page_004c0450 = 0x3a4;
        DAT_005bac40 = 0x1;
        return 0x0;
      }
    }
  }
  if (param_1 == 0x0) {
    param_1 = 0x1;
  }
                    // LPCPINFO lpCPInfo for GetCPInfo
                    // UINT CodePage for GetCPInfo
  BVar2 = GetCPInfo(param_1,(LPCPINFO)&_Stack32);
  if (BVar2 == 0x0) {
    return 0x1;
  }
  FUN_004a0430(&DAT_005bac50,0x0,0x101);
  DAT_005bac40 = (_Stack32.LeadByte[0] != '\0');
  for (iVar4 = 0x0; (_Stack32.LeadByte[iVar4] != '\0' || (_Stack32.LeadByte[iVar4 + 0x1] != '\0')); iVar4 = iVar4 + 0x2)
  {
    for (uVar3 = _Stack32.LeadByte[iVar4]; uVar3 <= _Stack32.LeadByte[iVar4 + 0x1];
        uVar3 = uVar3 + 0x1) {
      (&DAT_005bac51)[uVar3] = 0x1;
    }
  }
  if (param_1 != 0x1) {
    code_page_004c0450 = param_1;
    return 0x0;
  }
  code_page_004c0450 = GetOEMCP();
  return 0x0;
}



void  FUN_004b9838(param_1: u32,param_2: *mut u32)

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  let mut uVar3: u32;
  let mut uVar4: u32;
  let mut uVar5: u32;
  byte *in_EAX;
  let mut uVar6: u32;
  let mut uVar7: u32;
  let mut uVar8: u32;
  let mut uVar9: u32;
  let mut uVar10: u32;
  let mut uVar11: u32;
  let mut uVar12: u32;
  let mut bVar13: bool;
  let mut bVar14: bool;
  let mut bVar15: bool;
  undefined8 uVar16;
  
  uVar11 = 0x0;
  uVar6 = 0x0;
  uVar12 = 0x0;
  for (; *in_EAX != 0x0; in_EAX = in_EAX + 0x1) {
    bVar13 = CARRY4(uVar12,uVar12);
    bVar14 = CARRY4(uVar6,uVar6);
    uVar5 = uVar6 * 0x2;
    uVar7 = uVar5 + bVar13;
    uVar1 = CARRY4(uVar12 * 0x2,uVar12 * 0x2);
    uVar8 = uVar7 * 0x2 + uVar1;
    uVar2 = CARRY4(uVar12 * 0x4,uVar12);
    bVar15 = CARRY4(uVar8,uVar6);
    uVar8 = uVar8 + uVar6;
    uVar9 = uVar8 + uVar2;
    uVar6 = uVar12 * 0xa;
    uVar3 = CARRY4(uVar12 * 0x5,uVar12 * 0x5);
    uVar10 = uVar9 * 0x2 + uVar3;
    uVar12 = uVar6 + (*in_EAX & 0xffffff0f);
    uVar4 = CARRY4(uVar6,*in_EAX & 0xffffff0f);
    uVar6 = uVar10 + uVar4;
    uVar11 = (uVar11 * 0x5 + (bVar14 || CARRY4(uVar5,bVar13)) * 0x2 +
              (CARRY4(uVar7,uVar7) || CARRY4(uVar7 * 0x2,uVar1)) + (bVar15 || CARRY4(uVar8,uVar2))) * 0x2 +
             (CARRY4(uVar9,uVar9) || CARRY4(uVar9 * 0x2,uVar3)) + CARRY4(uVar10,uVar4);
  }
  uVar16 = FUN_004b989a(uVar6,uVar11);
  param_2[0x1] = ((ulonglong)uVar16 >> 0x20);
  *param_2 = uVar16;
  (param_2 + 0x2) = in_EAX;
  return;
}



void  FUN_004b989a(param_1: u32,param_2: u32)

{
  let mut in_EAX: u32;
  let mut uVar1: u32;
  let mut uVar2: u32;
  let mut unaff_EBP: u32;
  let mut uVar3: u32;
  let mut uVar4: u32;
  let mut bVar5: bool;
  let mut bVar6: bool;
  
  if ((in_EAX | param_2 | unaff_EBP) != 0x0) {
    uVar1 = in_EAX;
    uVar3 = unaff_EBP;
    if (param_2 == 0x0) {
      uVar3 = 0x0;
      uVar1 = unaff_EBP;
      param_2 = in_EAX;
    }
    uVar2 = uVar1;
    uVar4 = uVar3;
    if (param_2 == 0x0) {
      uVar4 = 0x0;
      uVar2 = uVar3;
      param_2 = uVar1;
    }
    for (; -0x1 < param_2; param_2 = param_2 * 0x2 + (bVar6 || CARRY4(uVar1,bVar5))) {
      bVar5 = CARRY4(uVar4,uVar4);
      uVar4 = uVar4 * 0x2;
      bVar6 = CARRY4(uVar2,uVar2);
      uVar1 = uVar2 * 0x2;
      uVar2 = uVar1 + bVar5;
    }
  }
  return;
}



fn FUN_004b98e0() -> i32

{
  let mut iVar1: i32;
  
  iVar1 = (PTR_FUN_004bfb74)();
  return iVar1 + 0xc6;
}



u32 * 
FUN_004b98ec(param_1: u32,param_2: u32,param_3: i32,param_4: *mut u32,param_5: *mut u32,param_6: i32,
            param_7: *mut u32)

{
  let mut local_50: i32;
  let mut local_4c: u32;
  let mut local_48: u32;
  let mut local_44: i32;
  let mut local_40: u32;
  let mut local_3c: u32;
  let mut local_38: u32;
  float10 local_24;
  let mut local_18: u32;
  let mut uStack20: u32;
  
  local_18 = param_1;
  uStack20 = param_2;
  local_24 = (float10)(double)CONCAT44(param_2,param_1);
  if (param_6 != 0x46) {
    local_48 = 0xc;
  }
  else {
    local_48 = 0xa;
  }
  local_4c = (param_6 != 0x46);
  local_50 = param_3;
  local_40 = 0x0;
  local_44 = param_6;
  FUN_004b2008(&local_24,&local_50,param_7);
  *param_4 = local_38;
  *param_5 = local_3c;
  return param_7;
}



fn FUN_004b9970(param_1: i32) -> i32

{
  return (*(param_1 + -0x4) & 0xfffffffe) - 0x4;
}



fn FUN_004b9998(short param_1,param_2: *mut i32,param_3: u32,param_4: *mut u32) -> u32

{
  let puVar1: *mut u32;
  let mut uVar2: u32;
  let mut iVar3: i32;
  let mut uVar4: u32;
  let mut uVar5: u32;
  let mut uVar6: u32;
  let mut uVar7: u32;
  let unaff_EBX: *mut i32;;
  let puVar8: *mut u32;
  let in_DS: i16;
  
  uVar4 = param_3 + 0xb & 0xfffffff8;
  if (uVar4 < param_3) {
    uVar4 = 0xffffffff;
  }
  else {
    if (uVar4 < 0x10) {
      uVar4 = 0x10;
    }
  }
  puVar1 = (param_2 + -0x1);
  uVar7 = *puVar1 & 0xfffffffe;
  if (uVar7 < uVar4) {
    *param_4 = uVar4 - uVar7;
    for (puVar8 = (puVar1 + uVar7); uVar4 = *puVar8, uVar4 != 0xffffffff;
        puVar8 = (puVar8 + uVar4)) {
      if ((uVar4 & 0x1) != 0x0) {
        return 0x1;
      }
      uVar7 = puVar8[0x2];
      uVar2 = puVar8[0x1];
      if (in_DS == param_1) {
        iVar3 = DAT_004bfae8[0x2];
        unaff_EBX = DAT_004bfae8;
        while ((iVar3 != 0x0 && ((param_2 < unaff_EBX || ((*unaff_EBX + unaff_EBX) <= param_2))))) {
          unaff_EBX = unaff_EBX[0x2];
          iVar3 = unaff_EBX[0x2];
        }
      }
      if (puVar8 == unaff_EBX[0x3]) {
        unaff_EBX[0x3] = (unaff_EBX[0x3])[0x1];
      }
      if (*param_4 <= uVar4) {
        uVar6 = uVar4 - *param_4;
        if (0xf < uVar6) {
          puVar8 = (puVar8 + *param_4);
          *puVar8 = uVar6;
          puVar8[0x1] = uVar2;
          puVar8[0x2] = uVar7;
          (uVar2 + 0x8) = puVar8;
          (uVar7 + 0x4) = puVar8;
          *puVar1 = *puVar1 + *param_4;
          DAT_005ba481 = 0x0;^
          // goto LAB_004b9b48;
        }
      }
      *(uVar2 + 0x8) = uVar7;
      *(uVar7 + 0x4) = uVar2;
      *puVar1 = *puVar1 + uVar4;
      unaff_EBX[0x7] = unaff_EBX[0x7] + -0x1;
      DAT_005ba481 = 0x0;
      if (*param_4 <= uVar4)^ // goto LAB_004b9b48;
      *param_4 = *param_4 - uVar4;
    }
    uVar5 = 0x2;
  }
  else {
    if (0xf < uVar7 - uVar4) {
      *puVar1 = uVar4 | 0x1;
      *(uVar4 + puVar1) = uVar7 - uVar4 | 0x1;
      if (in_DS == param_1) {
        iVar3 = DAT_004bfae8[0x2];
        unaff_EBX = DAT_004bfae8;
        while ((iVar3 != 0x0 && ((param_2 < unaff_EBX || ((*unaff_EBX + unaff_EBX) <= param_2))))) {
          unaff_EBX = unaff_EBX[0x2];
          iVar3 = unaff_EBX[0x2];
        }
      }
      unaff_EBX[0x6] = unaff_EBX[0x6] + 0x1;
      FUN_004aaaf0(((uVar4 + puVar1) + 0x1));
    }
// LAB_004b9b48:
    uVar5 = 0x0;
  }
  return uVar5;
}



i32 *  FUN_004b9b54(param_1: *mut i32,param_2: u32)

{
  let mut iVar1: i32;
  let in_DS: i16;
  let mut uStack8: u32;
  
  (PTR_FUN_004bfb90)();
  iVar1 = FUN_004b9998(in_DS,param_1,param_2,&uStack8);
  if (iVar1 == 0x0) {
    (PTR_FUN_004bfb98)();
    return param_1;
  }
  (PTR_FUN_004bfb98)();
  return 0x0;
}



undefined8  FUN_004b9bf3(param_1: u32,param_2: u32)

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  let mut in_EAX: u32;
  let mut unaff_EBX: u32;
  let mut iVar3: i32;
  let mut uVar4: u32;
  let mut uVar5: u32;
  let mut bVar6: bool;
  let mut bVar7: bool;
  let mut bVar8: bool;
  let mut bVar9: bool;
  
  if (param_1 == 0x0) {
    if (unaff_EBX != 0x1) {
      if (unaff_EBX <= param_2) {
        param_1 = param_2 / unaff_EBX;
        param_2 = param_2 % unaff_EBX;
      }
      in_EAX = (CONCAT44(param_2,in_EAX) / (ulonglong)unaff_EBX);
      param_2 = param_1;
    }
    return CONCAT44(param_2,in_EAX);
  }
  if (param_2 <= param_1) {
    if ((param_1 == param_2) && (unaff_EBX <= in_EAX)) {
      return 0x1;
    }
    return 0x0;
  }
  uVar4 = 0x0;
  uVar5 = 0x0;
  iVar3 = 0x0;
  loop {
    bVar6 = CARRY4(unaff_EBX,unaff_EBX);
    unaff_EBX = unaff_EBX * 0x2;
    bVar7 = CARRY4(param_1,param_1) || CARRY4(param_1 * 0x2,bVar6);
    param_1 = param_1 * 0x2 + bVar6;
    if (bVar7)^ // goto LAB_004b9c53;
    iVar3 = iVar3 + 0x1;
  } while ((param_1 < param_2) || ((param_1 <= param_2 && (unaff_EBX <= in_EAX))));
  bVar6 = false;
  while( true ) {
    bVar7 = CARRY4(uVar4,uVar4);
    uVar1 = uVar4 * 0x2;
    uVar4 = uVar1 + bVar6;
    uVar1 = (bVar7 || CARRY4(uVar1,bVar6));
    bVar7 = CARRY4(uVar5,uVar5) || CARRY4(uVar5 * 0x2,uVar1);
    uVar5 = uVar5 * 0x2 + uVar1;
    iVar3 = iVar3 + -0x1;
    if (iVar3 < 0x0) break;
// LAB_004b9c53:
    uVar1 = param_1 & 0x1;
    param_1 = param_1 >> 0x1 | bVar7 << 0x1f;
    unaff_EBX = unaff_EBX >> 0x1 | (uVar1 != 0x0) << 0x1f;
    bVar6 = in_EAX < unaff_EBX;
    in_EAX = in_EAX - unaff_EBX;
    uVar1 = bVar6;
    bVar7 = param_1 <= param_2;
    uVar2 = param_2 - param_1;
    param_2 = uVar2 - uVar1;
    bVar6 = bVar7 && uVar1 <= uVar2;
    if (!bVar7 || uVar1 > uVar2) {
      loop {
        bVar6 = CARRY4(uVar4,uVar4);
        uVar4 = uVar4 * 0x2;
        uVar5 = uVar5 * 0x2 + bVar6;
        iVar3 = iVar3 + -0x1;
        if (iVar3 < 0x0)^ // goto LAB_004b9c75;
        uVar1 = param_1 & 0x1;
        param_1 = param_1 >> 0x1;
        unaff_EBX = unaff_EBX >> 0x1 | (uVar1 != 0x0) << 0x1f;
        bVar8 = CARRY4(in_EAX,unaff_EBX);
        in_EAX = in_EAX + unaff_EBX;
        bVar9 = CARRY4(param_2,param_1);
        bVar7 = CARRY4(param_2 + param_1,bVar8);
        bVar6 = bVar9 || bVar7;
        param_2 = param_2 + param_1 + bVar8;
      } while (!bVar9 && !bVar7);
    }
  }
// LAB_004b9c75:
  return CONCAT44(uVar5,uVar4);
}



fn FUN_004b9c90() -> u32

{
  return 0x0;
}



fn FUN_004b9cb8()

{
  (PTR_exit_from_thread_004c0458)();
  return;
}



fn FUN_004b9cc0(param_1: u32)

{
  (PTR_FUN_004c045c)(param_1);
  return;
}



fn FUN_004b9d20()

{
  byte *pbVar1;
  let mut uVar2: u32;
  let mut uVar3: u32;
  byte *pbVar4;
  byte *pbVar5;
  let local_20: u8 [0xc];
  HANDLE local_14;
  
  pbVar1 = FUN_004b6220(s_C_FILE_INFO_004c3cfc);
  if (pbVar1 != 0x0) {
    for (; pbVar5 = pbVar1, *pbVar1 != 0x0; pbVar1 = pbVar1 + 0x1) {
      loop {
        pbVar4 = pbVar5;
        if (*pbVar5 == 0x3a)^ // goto LAB_004b9d63;
        if (*pbVar5 == 0x0) break;
        pbVar4 = pbVar5 + 0x1;
        if (*pbVar4 == 0x3a)^ // goto LAB_004b9d63;
        pbVar5 = pbVar5 + 0x2;
      } while (*pbVar4 != 0x0);
      pbVar4 = 0x0;
// LAB_004b9d63:
      FUN_004a9a00(local_20,pbVar1,pbVar4 - pbVar1);
      local_20[pbVar4 - pbVar1] = 0x0;
      pbVar4 = pbVar4 + 0x1;
      uVar2 = FUN_004ba31c(local_20,(byte **)0x0,0x10);
      pbVar1 = pbVar4;
      loop {
        pbVar5 = pbVar1;
        if (*pbVar1 == 0x3a)^ // goto LAB_004b9dae;
        if (*pbVar1 == 0x0) break;
        pbVar5 = pbVar1 + 0x1;
        if (*pbVar5 == 0x3a)^ // goto LAB_004b9dae;
        pbVar1 = pbVar1 + 0x2;
      } while (*pbVar5 != 0x0);
      pbVar5 = 0x0;
// LAB_004b9dae:
      FUN_004a9a00(local_20,pbVar4,pbVar5 - pbVar4);
      local_20[pbVar5 - pbVar4] = 0x0;
      pbVar5 = pbVar5 + 0x1;
      local_14 = (HANDLE)FUN_004ba31c(local_20,(byte **)0x0,0x10);
      pbVar4 = pbVar5;
      loop {
        pbVar1 = pbVar4;
        if (*pbVar4 == 0x2a)^ // goto LAB_004b9dfb;
        if (*pbVar4 == 0x0) break;
        pbVar1 = pbVar4 + 0x1;
        if (*pbVar1 == 0x2a)^ // goto LAB_004b9dfb;
        pbVar4 = pbVar4 + 0x2;
      } while (*pbVar1 != 0x0);
      pbVar1 = 0x0;
// LAB_004b9dfb:
      FUN_004a9a00(local_20,pbVar5,pbVar1 - pbVar5);
      local_20[pbVar1 - pbVar5] = 0x0;
      uVar3 = FUN_004ba31c(local_20,(byte **)0x0,0x10);
      set_std_handle_004b406c(local_14,uVar2);
      FUN_004b1a88(uVar2,uVar3);
    }
    FUN_004ba3a0(s_C_FILE_INFO__004c3d08);
  }
  return;
}



fn FUN_004b9f00(param_1: u32,param_2: u32) -> u32

{
  ushort in_FPUControlWord;
  let mut local_c: u32;
  
  local_c = 0x0;
  if ((DAT_004bfacc != '\0') && (local_c = in_FPUControlWord, param_2 != 0x0)) {
    local_c = ~param_2 & local_c | param_1 & param_2 & 0xffff;
  }
  return local_c;
}



fn FUN_004b9f50(byte *param_1,byte *param_2)

{
  let mut iVar1: i32;
  let local_10: u8 [0x4];
  let local_c: u8 [0x4];
  
  FUN_004ba820(local_c,param_1);
  iVar1 = FUN_004b75b0(param_1);
  local_c[iVar1] = 0x0;
  FUN_004ba820(local_10,param_2);
  iVar1 = FUN_004b75b0(param_2);
  local_10[iVar1] = 0x0;
  FUN_004ba860(local_c);
  FUN_004ba860(local_10);
  FUN_004ba8d0(local_c,local_10);
  return;
}



// WARNING: Unable to track spacebase fully for stack
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
// lpStartAddress parameter of CreateThread
// 

void  set_handle_fn_004b9fd0(HANDLE *handles)

{
  let piVar1: *mut i32;;
  let mut iVar2: i32;
  let mut uStackY56: u32;
  let mut uStackY52: u32;
  let mut uStackY48: u32;
  let local_28: u8 [0x8];
  code *local_20;
  
  local_20 = (code *)*handles;
  if (DAT_005b9de0 == 0x0) {
    iVar2 = -(DAT_004c0338 + 0x3U & 0xfffffffc);
    *(&stack0xffffffd4 + iVar2) = DAT_004c0338;
    *(&uStackY48 + iVar2) = 0x0;
    (&uStackY52 + iVar2) = local_28 + iVar2;
    *(&uStackY56 + iVar2) = 0x4ba00f;
    FUN_004a0430(*(&uStackY52 + iVar2),(&uStackY48 + iVar2),
                 *(&stack0xffffffd4 + iVar2));
    (&stack0xffffffd4 + iVar2) = local_28 + iVar2;
    *(&stack0x000000c8 + iVar2) = DAT_004c0338;
    *(&uStackY48 + iVar2) = 0x4ba023;
    piVar1 = FUN_004b5768(*(i32 **)(&stack0xffffffd4 + iVar2));
    if (piVar1 == 0x0) {
      return;
    }
  }
  iVar2 = (PTR_FUN_004bfb74)();
  handles[0x4] = (HANDLE)(iVar2 + 0xde);
                    // HANDLE hEvent for SetEvent
  uStackY48 = 0x4ba043;
  SetEvent(handles[0x3]);
  uStackY48 = 0x4ba04c;
  set_unhandled_except_filter_004b6038(local_28);
  (*_DAT_004bfbb4)();
  uStackY48 = 0x4ba059;
  (*local_20)();
  FUN_004b9cb8();
  return;
}



HANDLE  create_thread_fn_004ba070(DWORD param_1,param_2: i32,DWORD param_3)

{
  let mut bVar1: bool;
  undefined3 extraout_var;
  let curr_thread_id: u32;
  HANDLE thread_handle;
  let mut pcVar2: String; 
  let mut uVar3: u32;
  let mut event_name: String;;
  let mut uStack72: u32;
  let local_44: u8 [0x18];
  let local_2c: u32;
  let local_28: u32;
  HANDLE curr_thread_handle;
  HANDLE event_handle;
  HANDLE *handle_1c;
  let local_18: u32;
  
  if (tls_index == -0x1) {
    bVar1 = alloc_tls_fn_004b570c();
    if (CONCAT31(extraout_var,bVar1) == 0x0) {
      return (HANDLE)0x0;
    }
    FUN_004b5844();
  }
  local_2c = param_1;
  local_28 = param_3;
  curr_thread_handle = GetCurrentThread();
  uVar3 = 0x10;
  pcVar2 = local_44;
  handle_1c = (HANDLE *)0x0;
  event_name = DAT_004c3d18;
  uStack72 = DAT_004c3d1c;
  local_44[0] = DAT_004c3d20;
  curr_thread_id = GetCurrentThreadId();
  FUN_004b8068(curr_thread_id,pcVar2,uVar3);
                    // LPCSTR lpName for CreateEventA
                    // BOOL bInitialState for CreateEventA
                    // BOOL bManualReset for CreateEventA
                    // LPSECURITY_ATTRIBUTES lpEventAttributes for CreateEventA
  event_handle = CreateEventA((LPSECURITY_ATTRIBUTES)0x0,0x0,0x0,&event_name);
                    // LPDWORD lpThreadId for CreateThread
                    // DWORD dwCreationFlags for CreateThread
                    // LPVOID lpParameter for CreateThread
                    // LPTHREAD_START_ROUTINE lpStartAddress for CreateThread
                    // SIZE_T dwStackSize for CreateThread
                    // LPSECURITY_ATTRIBUTES lpThreadAttributes for CreateThread
  thread_handle =
       CreateThread((LPSECURITY_ATTRIBUTES)0x0,param_2 + 0xfffU & 0xffff0000 | (param_2 + 0xfffU >> 0x8 & 0xf0) << 0x8,
                    set_handle_fn_004b9fd0,&local_2c,0x0,&local_18);
  if (thread_handle == (HANDLE)0x0) {
    local_18 = 0xffffffff;
  }
  else {
                    // DWORD dwMilliseconds for WaitForSingleObject
                    // HANDLE hHandle for WaitForSingleObject
    WaitForSingleObject(event_handle,0xffffffff);
    if (handle_1c == (HANDLE *)0x0) {
                    // HANDLE hObject for CloseHandle
      CloseHandle(thread_handle);
    }
    else {
      *handle_1c = thread_handle;
    }
  }
                    // HANDLE hObject for CloseHandle
  CloseHandle(event_handle);
  return thread_handle;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void exit_from_thread_004ba168(void)

{
  (*_DAT_004bfbb8)();
  FUN_004b6084();
  if (DAT_005b9de0 == 0x0) {
    check_set_tls_val_004b57c4(0x1);
  }
                    // DWORD dwExitCode for ExitThread
                    // WARNING: Subroutine does not return
  ExitThread(0x0);
}



fn FUN_004ba190(byte *param_1,byte **param_2,param_3: i32,param_4: i32) -> String

{
  let bVar1: u8;
  let mut bVar2: bool;
  let mut iVar3: i32;
  byte *pbVar4;
  byte *pbVar5;
  let mut puVar6: *mut u8; 
  let mut puVar7: *mut u8; 
  
  pbVar4 = param_1;
  if (param_2 != (byte **)0x0) {
    *param_2 = param_1;
  }
  while (((&DAT_004bf9c4)[(byte)(*pbVar4 + 0x1)] & 0x2) != 0x0) {
    pbVar4 = pbVar4 + 0x1;
  }
  bVar1 = *pbVar4;
  if ((bVar1 == 0x2b) || (bVar1 == 0x2d)) {
    pbVar4 = pbVar4 + 0x1;
  }
  if (param_3 == 0x0) {
    if ((*pbVar4 != 0x30) || ((pbVar4[0x1] != 0x78 && (pbVar4[0x1] != 0x58)))) {
      if (*pbVar4 == 0x30) {
        param_3 = 0x8;
      }
      else {
        param_3 = 0xa;
      }^
      // goto LAB_004ba234;
    }
    param_3 = 0x10;
  }
  else {
    if ((param_3 < 0x2) || (0x24 < param_3)) {
      FUN_004b1740(0xd);
      return 0x0;
    }
    if (param_3 != 0x10)^ // goto LAB_004ba234;
  }
  if ((*pbVar4 == 0x30) && ((pbVar4[0x1] == 0x78 || (pbVar4[0x1] == 0x58)))) {
    pbVar4 = pbVar4 + 0x2;
  }
// LAB_004ba234:
  bVar2 = false;
  pbVar5 = pbVar4;
  puVar7 = 0x0;
  while (iVar3 = FUN_004ba338(*pbVar5), iVar3 < param_3) {
    if ((&PTR_exit_from_thread_004c0458)[param_3] <= puVar7 && puVar7 != (&PTR_exit_from_thread_004c0458)[param_3]) {
      bVar2 = true;
    }
    puVar6 = (puVar7 * param_3 + iVar3);
    if (puVar6 < puVar7) {
      bVar2 = true;
    }
    pbVar5 = pbVar5 + 0x1;
    puVar7 = puVar6;
  }
  if (pbVar5 == pbVar4) {
    pbVar5 = param_1;
  }
  if (param_2 != (byte **)0x0) {
    *param_2 = pbVar5;
  }
  if ((((param_4 != 0x1) || (puVar7 < 0x80000000)) ||
      ((puVar7 == 0x80000000 && (bVar1 == 0x2d)))) && (!bVar2)) {
    if (bVar1 == 0x2d) {
      puVar7 = -puVar7;
    }
    return puVar7;
  }
  FUN_004b1740(0xe);
  if (param_4 == 0x0) {
    return 0xffffffff;
  }
  if (bVar1 == 0x2d) {
    return 0x80000000;
  }
  return 0x7fffffff;
}



fn FUN_004ba31c(byte *param_1,byte **param_2,param_3: i32)

{
  FUN_004ba190(param_1,param_2,param_3,0x1);
  return;
}



fn FUN_004ba338(byte param_1) -> i32

{
  let bVar1: u8;
  let mut uVar2: u32;
  
  if ((0x2f < param_1) && (param_1 < 0x3a)) {
    return param_1 - 0x30;
  }
  uVar2 = FUN_004aa9f0(param_1);
  bVar1 = (byte)uVar2;
  if ((0x60 < bVar1) && (bVar1 < 0x6a)) {
    return (uVar2 & 0xff) - 0x57;
  }
  if ((0x69 < bVar1) && (bVar1 < 0x73)) {
    return (uVar2 & 0xff) - 0x57;
  }
  if ((0x72 < bVar1) && (bVar1 < 0x7b)) {
    return (uVar2 & 0xff) - 0x57;
  }
  return 0x25;
}



fn FUN_004ba3a0(param_1: *mut u32) -> i32

{
  let puVar1: *mut u32;
  let mut iVar2: i32;
  let lpName: *mut u32;
  let puVar3: *mut u32;
  let mut BVar4: bool;
  ushort *puVar5;
  let mut uVar6: u32;
  let local_18: *mut u32;
  
  puVar1 = FUN_004ba930(param_1,0x3d);
  if (puVar1 == 0x0) {
    iVar2 = -0x1;
  }
  else {
    if (puVar1 == param_1) {
      return -0x1;
    }
    lpName = FUN_004aac00((puVar1 - param_1) + 0x1);
    if (lpName == 0x0) {
      return -0x1;
    }
    *lpName = *param_1;
    (lpName + 0x1) = (param_1 + 0x1);
    (lpName + (puVar1 - param_1)) = 0x0;
    local_18 = FUN_004ba9a0((puVar1 + 0x1));
    if (local_18 != 0x0) {
      puVar3 = FUN_004aac00(local_18 + 0x1);
      if (puVar3 == 0x0) {
        FUN_004aaae0(lpName);
        return -0x1;
      }
      *puVar3 = *(puVar1 + 0x1);
      *(puVar3 + 0x1) = *(puVar1 + 0x5);
      (puVar3 + local_18) = 0x0;
      local_18 = puVar3;
    }
                    // LPCSTR lpValue for SetEnvironmentVariableA
                    // LPCSTR lpName for SetEnvironmentVariableA
    BVar4 = SetEnvironmentVariableA(lpName,local_18);
    FUN_004aaae0(lpName);
    FUN_004aaae0(local_18);
    if (BVar4 == 0x0) {
      return -0x1;
    }
    iVar2 = FUN_004ba56c(param_1);
    if (iVar2 != 0x0) {
      return -0x1;
    }
    if (DAT_005bac08 != 0x0) {
      iVar2 = FUN_004ba9a0(param_1);
      puVar5 = FUN_004aac00((iVar2 + 0x1) * 0x2);
      if (puVar5 == 0x0) {
        FUN_004b1290(0x5);
        return -0x1;
      }
      uVar6 = FUN_004ba9d0((WCHAR *)puVar5,param_1,iVar2 + 0x1);
      if (uVar6 == 0xffffffff) {
        FUN_004aaae0(puVar5);
        return -0x1;
      }
      iVar2 = FUN_004bac04(puVar5);
    }
  }
  return iVar2;
}



fn FUN_004ba56c(byte *param_1) -> u32

{
  let puVar1: *mut u32;
  let bVar2: u8;
  byte *in_EAX;
  let puVar3: *mut u32;
  let mut iVar4: i32;
  let mut uVar5: u32;
  let mut uVar6: u32;
  let mut bVar7: bool;
  
  puVar3 = DAT_005bac04;
  if (param_1 == 0x0) {
    return 0xffffffff;
  }
  if (*param_1 != 0x0) {
    in_EAX = param_1 + 0x1;
    bVar2 = *in_EAX;
    for (; (bVar2 != 0x0 && (*in_EAX != 0x3d)); in_EAX = in_EAX + 0x1) {
      bVar2 = in_EAX[0x1];
    }
  }
  if (*in_EAX == 0x0) {
    return 0xffffffff;
  }
  bVar7 = in_EAX[0x1] == 0x0;
  uVar5 = bVar7;
  if (DAT_005bac04 == 0x0) {
    if (bVar7) {
      return 0x0;
    }
    puVar3 = FUN_004aac00(0x9);
    if (puVar3 == 0x0) {
      return 0xffffffff;
    }
    DAT_005bac00 = puVar3 + 0x2;
    DAT_005bac04 = puVar3;
    *puVar3 = 0x0;
    puVar3[0x1] = 0x0;
  }
  else {
    iVar4 = FUN_004ba710(param_1,uVar5);
    if (bVar7) {
      return 0x0;
    }
    if (iVar4 < 0x1) {
      uVar5 = -iVar4;
      uVar6 = iVar4 * -0x4 + 0x8 + uVar5 + 0x1;
      if (DAT_005bac00 == 0x0) {
        puVar3 = FUN_004aac00(uVar6);
        if (puVar3 == 0x0) {
          return 0xffffffff;
        }
        puVar1 = DAT_005bac04 + 0x1;
        *puVar3 = *DAT_005bac04;
        (puVar3 + 0x1) = puVar1;
        DAT_005bac00 = puVar3 + (0x2 - iVar4);
        FUN_004a0430(DAT_005bac00,0x0,uVar5 + 0x1);
      }
      else {
        puVar3 = FUN_004b7f20(puVar3,uVar6);
        if (puVar3 == 0x0) {
          return 0xffffffff;
        }
        FUN_004a1dc0(puVar3 + (0x2 - iVar4),DAT_005bac00,uVar5);
        DAT_005bac00 = puVar3 + (0x2 - iVar4);
      }
      puVar3[0x1 - iVar4] = 0x0;
      DAT_005bac04 = puVar3;
    }
    else {
      uVar5 = iVar4 - 0x1;
    }
  }
  puVar3[uVar5] = param_1;
  (DAT_005bac00 + uVar5) = 0x0;
  return 0x0;
}



fn FUN_004ba710(byte *param_1,param_2: i32) -> i32

{
  byte **ppbVar1;
  byte *pbVar2;
  let bVar3: u8;
  let mut iVar4: i32;
  let mut iVar5: i32;
  let mut puVar6: *mut u8; 
  byte *pbVar7;
  byte **ppbVar8;
  byte *pbVar9;
  let mut uVar10: u32;
  
  ppbVar8 = DAT_005bac04;
  loop {
    pbVar7 = *ppbVar8;
    if (pbVar7 == 0x0) {
      return DAT_005bac04 - ppbVar8 >> 0x2;
    }
    bVar3 = *param_1;
    pbVar9 = param_1;
    while (bVar3 != 0x0) {
      iVar4 = FUN_004a11c0(*pbVar7);
      iVar5 = FUN_004a11c0(*pbVar9);
      if (iVar4 != iVar5) break;
      if (*pbVar7 == 0x3d) {
        iVar4 = ppbVar8 - DAT_005bac04 >> 0x2;
        if (param_2 == 0x0) {
          return iVar4 + 0x1;
        }
        pbVar7 = *ppbVar8;
        pbVar9 = pbVar7;
        while (pbVar9 != 0x0) {
          *ppbVar8 = ppbVar8[0x1];
          ppbVar1 = ppbVar8 + 0x1;
          ppbVar8 = ppbVar8 + 0x1;
          pbVar9 = *ppbVar1;
        }
        if (DAT_005bac00 != (byte **)0x0) {
          if ((iVar4 + DAT_005bac00) != '\0') {
            FUN_004aaae0(pbVar7);
          }
          uVar10 = ppbVar8 - DAT_005bac04 >> 0x2;
          FUN_004a1dc0(ppbVar8,DAT_005bac00,uVar10);
          DAT_005bac00 = ppbVar8;
          if (iVar4 < uVar10) {
            puVar6 = (iVar4 + ppbVar8);
            loop {
              iVar4 = iVar4 + 0x1;
              *puVar6 = puVar6[0x1];
              puVar6 = puVar6 + 0x1;
            } while (iVar4 < uVar10);
          }
        }
        return 0x0;
      }
      pbVar7 = pbVar7 + 0x1;
      pbVar2 = pbVar9 + 0x1;
      pbVar9 = pbVar9 + 0x1;
      bVar3 = *pbVar2;
    }
    ppbVar8 = ppbVar8 + 0x1;
  } while( true );
}



fn FUN_004ba820(byte *param_1,byte *param_2)

{
  if ((DAT_005bac40 != 0x0) && (((&DAT_005bac51)[*param_2] & 0x1) != 0x0)) {
    *param_1 = *param_2;
    param_1[0x1] = param_2[0x1];
    return;
  }
  *param_1 = *param_2;
  return;
}



byte *  FUN_004ba860(byte *param_1)

{
  ushort uVar1;
  let mut iVar2: i32;
  let extraout_var: u16;
  let mut uVar3: u32;
  byte *pbVar4;
  let abStack12: u8 [0x4];
  
  pbVar4 = param_1;
  while( true ) {
    iVar2 = FUN_004b7570(pbVar4);
    if (iVar2 != 0x0) break;
    uVar1 = FUN_004b1ca0(pbVar4);
    uVar3 = FUN_004baec0(CONCAT22(extraout_var,uVar1));
    FUN_004baf40(uVar3,abStack12);
    iVar2 = FUN_004b75b0(abStack12);
    abStack12[iVar2] = 0x0;
    FUN_004ba820(pbVar4,abStack12);
    pbVar4 = FUN_004b1c70(pbVar4);
  }
  return param_1;
}



fn FUN_004ba8d0(byte *param_1,byte *param_2) -> i32

{
  if (*param_1 != *param_2) {
    return *param_1 - *param_2;
  }
  if ((DAT_005bac40 != 0x0) && (((&DAT_005bac51)[*param_1] & 0x1) != 0x0)) {
    if (param_1[0x1] != param_2[0x1]) {
      return param_1[0x1] - param_2[0x1];
    }
  }
  return 0x0;
}



byte *  FUN_004ba930(byte *param_1,param_2: i32)

{
  let mut iVar1: i32;
  let abStack8: u8 [0x4];
  
  FUN_004baf40(param_2,abStack8);
  iVar1 = FUN_004b75b0(abStack8);
  abStack8[iVar1] = 0x0;
  while( true ) {
    iVar1 = FUN_004b7570(param_1);
    if (iVar1 != 0x0) break;
    iVar1 = FUN_004ba8d0(param_1,abStack8);
    if (iVar1 == 0x0) break;
    param_1 = FUN_004b1c70(param_1);
  }
  iVar1 = FUN_004b7570(param_1);
  if ((iVar1 != 0x0) && (param_2 != 0x0)) {
    param_1 = 0x0;
  }
  return param_1;
}



fn FUN_004ba9a0(byte *param_1) -> i32

{
  let mut iVar1: i32;
  let mut iVar2: i32;
  
  iVar2 = 0x0;
  while( true ) {
    iVar1 = FUN_004b7570(param_1);
    if (iVar1 != 0x0) break;
    param_1 = FUN_004b1c70(param_1);
    iVar2 = iVar2 + 0x1;
  }
  return iVar2;
}



fn FUN_004ba9d0(Wparam_1: &mut String,byte *param_2,param_3: i32) -> u32

{
  let mut uVar1: u32;
  let mut uVar2: u32;
  
  uVar2 = 0x0;
  if (param_1 == (WCHAR *)0x0) {
    while ((uVar1 = uVar2, *param_2 != 0x0 &&
           (uVar1 = conv_pchar_pwchar_004b7de0((WCHAR *)0x0,param_2,0x2), uVar1 != 0xffffffff))) {
      param_2 = FUN_004b1c70(param_2);
      uVar2 = uVar2 + 0x1;
    }
  }
  else {
    for (; uVar1 = uVar2, param_3 != 0x0; param_3 = param_3 + -0x1) {
      if (*param_2 == 0x0) {
        *param_1 = L'\0';
        return uVar2;
      }
      uVar1 = conv_pchar_pwchar_004b7de0(param_1,param_2,0x2);
      if (uVar1 == 0xffffffff) {
        return 0xffffffff;
      }
      param_2 = FUN_004b1c70(param_2);
      param_1 = param_1 + 0x1;
      uVar2 = uVar2 + 0x1;
    }
  }
  return uVar1;
}



fn FUN_004bac04(ushort *param_1) -> u32

{
  ushort uVar1;
  ushort *in_EAX;
  let puVar2: *mut u32;
  let mut iVar3: i32;
  let mut uVar4: u32;
  let mut uVar5: u32;
  let mut bVar6: bool;
  
  puVar2 = DAT_005bac08;
  if (param_1 == 0x0) {
    return 0xffffffff;
  }
  if (*param_1 != 0x0) {
    uVar1 = param_1[0x1];
    for (in_EAX = param_1 + 0x1; (uVar1 != 0x0 && (*in_EAX != 0x3d)); in_EAX = in_EAX + 0x1) {
      uVar1 = in_EAX[0x1];
    }
  }
  if (*in_EAX == 0x0) {
    return 0xffffffff;
  }
  bVar6 = in_EAX[0x1] == 0x0;
  uVar4 = bVar6;
  if (DAT_005bac08 == 0x0) {
    if (bVar6) {
      return 0x0;
    }
    puVar2 = FUN_004aac00(0x9);
    if (puVar2 == 0x0) {
      return 0xffffffff;
    }
    DAT_005bac00 = puVar2 + 0x2;
    DAT_005bac08 = puVar2;
    *puVar2 = 0x0;
    puVar2[0x1] = 0x0;
  }
  else {
    iVar3 = FUN_004bada4(param_1,uVar4);
    if (bVar6) {
      return 0x0;
    }
    if (iVar3 < 0x1) {
      uVar4 = -iVar3;
      uVar5 = uVar4 + 0x1 + iVar3 * -0x4 + 0x8;
      if (DAT_005bac00 == 0x0) {
        puVar2 = FUN_004aac00(uVar5);
        if (puVar2 == 0x0) {
          return 0xffffffff;
        }
        FUN_004b8660(puVar2,DAT_005bac08);
        DAT_005bac00 = puVar2 + (0x2 - iVar3);
        FUN_004a0430(DAT_005bac00,0x0,uVar4 + 0x1);
      }
      else {
        puVar2 = FUN_004b7f20(puVar2,uVar5);
        if (puVar2 == 0x0) {
          return 0xffffffff;
        }
        FUN_004a1dc0(puVar2 + (0x2 - iVar3),DAT_005bac00,uVar4);
        DAT_005bac00 = puVar2 + (0x2 - iVar3);
      }
      puVar2[0x1 - iVar3] = 0x0;
      DAT_005bac08 = puVar2;
    }
    else {
      uVar4 = iVar3 - 0x1;
    }
  }
  puVar2[uVar4] = param_1;
  (uVar4 + DAT_005bac00) = 0x0;
  return 0x0;
}



fn FUN_004bada4(ushort *param_1,param_2: i32) -> i32

{
  ushort **ppuVar1;
  ushort *puVar2;
  ushort uVar3;
  let mut iVar4: i32;
  let mut iVar5: i32;
  let mut puVar6: *mut u8; 
  ushort *puVar7;
  let mut uVar8: u32;
  ushort **ppuVar9;
  ushort *puVar10;
  
  ppuVar9 = DAT_005bac08;
  loop {
    puVar7 = *ppuVar9;
    if (puVar7 == 0x0) {
      return DAT_005bac08 - ppuVar9 >> 0x2;
    }
    uVar3 = *param_1;
    puVar10 = param_1;
    while (uVar3 != 0x0) {
      iVar4 = FUN_004bb110(*puVar7);
      iVar5 = FUN_004bb110(*puVar10);
      if (iVar4 != iVar5) break;
      if (*puVar7 == 0x3d) {
        iVar4 = ppuVar9 - DAT_005bac08 >> 0x2;
        if (param_2 == 0x0) {
          return iVar4 + 0x1;
        }
        puVar7 = *ppuVar9;
        puVar10 = puVar7;
        while (puVar10 != 0x0) {
          *ppuVar9 = ppuVar9[0x1];
          ppuVar1 = ppuVar9 + 0x1;
          ppuVar9 = ppuVar9 + 0x1;
          puVar10 = *ppuVar1;
        }
        if (DAT_005bac00 != (ushort **)0x0) {
          if ((iVar4 + DAT_005bac00) != '\0') {
            FUN_004aaae0(puVar7);
          }
          uVar8 = ppuVar9 - DAT_005bac08 >> 0x2;
          FUN_004a1dc0(ppuVar9,DAT_005bac00,uVar8);
          DAT_005bac00 = ppuVar9;
          if (iVar4 < uVar8) {
            puVar6 = (iVar4 + ppuVar9);
            loop {
              iVar4 = iVar4 + 0x1;
              *puVar6 = puVar6[0x1];
              puVar6 = puVar6 + 0x1;
            } while (iVar4 < uVar8);
          }
        }
        return 0x0;
      }
      puVar7 = puVar7 + 0x1;
      puVar2 = puVar10 + 0x1;
      puVar10 = puVar10 + 0x1;
      uVar3 = *puVar2;
    }
    ppuVar9 = ppuVar9 + 0x1;
  } while( true );
}



fn FUN_004baec0(param_1: u32)

{
  let mut iVar1: i32;
  let local_c: u8 [0x4];
  
  FUN_004baf40(param_1,local_c);
  iVar1 = FUN_004b75b0(local_c);
  local_c[iVar1] = 0x0;
  if (((code_page_004c0450 == 0x3a4) && (DAT_005bac40 != 0x0)) && (((&DAT_005bac51)[local_c[0]] & 0x1) != 0x0)) {
    FUN_004bb130(param_1);
  }
  else {
                    // DWORD cchLength for CharUpperBuffA
                    // LPSTR lpsz for CharUpperBuffA
    CharUpperBuffA((LPSTR)local_c,0x1);
    FUN_004b1ca0(local_c);
  }
  return;
}



fn FUN_004baf40(param_1: u32,param_2: &mut String)

{
  let cVar1: u8;
  
  cVar1 = (char)(param_1 >> 0x8);
  if (cVar1 != '\0') {
    param_2[0x1] = (char)param_1;
    *param_2 = cVar1;
    return;
  }
  *param_2 = (char)param_1;
  return;
}



short *  FUN_004baf60(short *param_1,short param_2)

{
  let sVar1: i16;
  
  sVar1 = *param_1;
  while( true ) {
    if (param_2 == sVar1) {
      return param_1;
    }
    sVar1 = *param_1;
    param_1 = param_1 + 0x1;
    if (sVar1 == 0x0) break;
    sVar1 = *param_1;
  }
  return (short *)0x0;
}



fn FUN_004baf90(ushort *param_1,param_2: *mut u32) -> *mut u32

{
  let puVar1: *mut u32;
  let mut pCVar2: String;;
  let lpName: *mut u32;
  let lpValue: *mut u32;
  let DVar3: u32;
  let mut uVar4: u32;
  
  if (USHORT_min_vers_004c0171 < 0x8000) {
                    // LPCWSTR lpValue for SetEnvironmentVariableW
                    // LPCWSTR lpName for SetEnvironmentVariableW
    puVar1 = SetEnvironmentVariableW((LPCWSTR)param_1,(LPCWSTR)param_2);
    return puVar1;
  }
  pCVar2 = FUN_004b8630((short *)param_1);
  uVar4 = pCVar2 * 0x2 + 0x1;
  lpName = FUN_004aac00(uVar4);
  puVar1 = lpName;
  if (lpName != 0x0) {
    lpValue = param_2;
    if (param_2 != 0x0) {
      pCVar2 = FUN_004b8630((short *)param_2);
      uVar4 = pCVar2 * 0x2 + 0x1;
      lpValue = FUN_004aac00(uVar4);
      if (lpValue == 0x0) {
        FUN_004aaae0(lpName);
        return 0x0;
      }
    }
    DVar3 = FUN_004b9600(lpName,param_1,uVar4);
    if (DVar3 == 0xffffffff) {
      FUN_004aaae0(lpName);
      if (lpValue != 0x0) {
        FUN_004aaae0(lpValue);
      }
      return 0x0;
    }
    if ((lpValue != 0x0) && (DVar3 = FUN_004b9600(lpValue,param_2,uVar4), DVar3 == 0xffffffff))
    {
      FUN_004aaae0(lpValue);
      return 0x0;
    }
                    // LPCSTR lpValue for SetEnvironmentVariableA
                    // LPCSTR lpName for SetEnvironmentVariableA
    puVar1 = SetEnvironmentVariableA(lpName,lpValue);
    FUN_004aaae0(lpName);
    if (lpValue != 0x0) {
      FUN_004aaae0(lpValue);
    }
  }
  return puVar1;
}



fn FUN_004bb0a0()

{
  byte *pbVar1;
  let mut iVar2: i32;
  ushort *puVar3;
  let mut uVar4: u32;
  byte **ppbVar5;
  
  ppbVar5 = DAT_005bac04;
  if (DAT_005bac04 != (byte **)0x0) {
    while( true ) {
      pbVar1 = *ppbVar5;
      ppbVar5 = ppbVar5 + 0x1;
      if (pbVar1 == 0x0) break;
      iVar2 = FUN_004ba9a0(pbVar1);
      puVar3 = FUN_004aac00((iVar2 + 0x1) * 0x2);
      if (puVar3 != 0x0) {
        uVar4 = FUN_004ba9d0((WCHAR *)puVar3,pbVar1,iVar2 + 0x1);
        if (uVar4 == 0xffffffff) {
          FUN_004aaae0(puVar3);
        }
        else {
          FUN_004bac04(puVar3);
        }
      }
    }
  }
  return;
}



fn FUN_004bb110(param_1: i32) -> i32

{
  if ((0x60 < (ushort)param_1) && ((ushort)param_1 < 0x7b)) {
    param_1 = param_1 + -0x20;
  }
  return param_1;
}



fn FUN_004bb130(param_1: u32) -> u32

{
  let bVar1: u8;
  undefined3 extraout_var;
  
  bVar1 = FUN_004bb150(param_1);
  if (CONCAT31(extraout_var,bVar1) != 0x0) {
    return param_1 - 0x21;
  }
  return param_1;
}



byte  FUN_004bb150(param_1: u32)

{
  let mut iVar1: i32;
  
  iVar1 = FUN_004bb1a0(param_1);
  if (iVar1 == 0x0) {
    return (&DAT_004bf9c4)[(byte)((char)param_1 + 0x1)] & 0x80;
  }
  if ((0x8280 < param_1) && (param_1 < 0x829b)) {
    return 0x1;
  }
  return 0x0;
}



fn FUN_004bb1a0(param_1: u32) -> u32

{
  let mut uVar1: u32;
  
  if ((DAT_005bac40 != 0x0) && (((&DAT_005bac51)[param_1 >> 0x8 & 0xff] & 0x1) != 0x0)) {
    uVar1 = FUN_004bb1f0(param_1 & 0xff);
    if (uVar1 != 0x0) {
      return 0x1;
    }
  }
  return 0x0;
}



fn FUN_004bb1f0(param_1: u32) -> u32

{
  if (DAT_005bac40 == 0x0) {
    param_1 = 0x0;
  }
  else {
    if (code_page_004c0450 == 0x3a4) {
      return ((&DAT_004c04ed)[param_1 & 0xff] & 0x8);
    }
    if (param_1 != 0x0) {
      return 0x1;
    }
  }
  return param_1;
}






