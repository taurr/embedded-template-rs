use strum_macros::{AsRefStr, Display, EnumProperty, EnumString};

#[allow(non_camel_case_types)]
#[derive(EnumString, AsRefStr, EnumProperty, Clone, Copy, Debug, Display, PartialEq, Eq)]
pub(crate) enum Chip {
    //
    // nRF51 Series
    //
    #[strum(props(target = "thumbv6m-none-eabi"))]
    nRF51422_xxAA,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    nRF51422_xxAB,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    nRF51422_xxAC,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    nRF51801_xxAB,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    nRF51802_xxAA,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    nRF51822_xxAA,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    nRF51822_xxAB,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    nRF51822_xxAC,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    nRF51824_xxAA,
    //
    // nRF52 Series
    //
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    nRF52805_xxAA,
    #[strum(props(target = "thumbv7em-none-eabi"))]
    nRF52810_xxAA,
    #[strum(props(target = "thumbv7em-none-eabi"))]
    nRF52811_xxAA,
    #[strum(props(target = "thumbv7em-none-eabi"))]
    nRF52820_xxAA,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    nRF52832_xxAA,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    nRF52832_xxAB,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    nRF52833_xxAA,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    nRF52840_xxAA,
    //
    // nRF91 Series
    //
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    nRF9160_xxAA,
    //
    // SAMD21
    //
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21E15A,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21E15B,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21E15BU,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21E15CU,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21E15L,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21E16A,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21E16B,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21E16BU,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21E16CU,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21E16L,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21E17A,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21E17D,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21E17DU,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21E17L,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21E18A,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21G15A,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21G15B,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21G15L,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21G16A,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21G16B,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21G16L,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21G17A,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21G17AU,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21G17D,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21G17L,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21G18A,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21G18AU,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21J15A,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21J15B,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21J16A,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21J16B,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21J17A,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21J17D,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    ATSAMD21J18A,
    //
    // STM32F0 Series
    //
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F030C6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F030C8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F030CCTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F030F4Px,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F030K6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F030R8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F030RCTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F031C4Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F031C6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F031E6Yx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F031F4Px,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F031F6Px,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F031G4Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F031G6Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F031K4Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F031K6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F031K6Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F038C6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F038E6Yx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F038F6Px,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F038G6Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F038K6Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F042C4Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F042C4Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F042C6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F042C6Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F042F4Px,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F042F6Px,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F042G4Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F042G6Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F042K4Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F042K4Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F042K6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F042K6Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F042T6Yx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F048C6Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F048G6Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F048T6Yx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F051C4Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F051C4Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F051C6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F051C6Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F051C8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F051C8Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F051K4Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F051K4Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F051K6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F051K6Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F051K8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F051K8Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F051R4Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F051R6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F051R8Hx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F051R8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F051T8Yx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F058C8Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F058R8Hx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F058R8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F058T8Yx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F070C6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F070CBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F070F6Px,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F070RBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F071C8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F071C8Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F071CBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F071CBUx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F071CBYx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F071RBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F071V8Hx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F071V8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F071VBHx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F071VBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F072C8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F072C8Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F072CBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F072CBUx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F072CBYx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F072R8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F072RBHx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F072RBIx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F072RBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F072V8Hx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F072V8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F072VBHx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F072VBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F078CBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F078CBUx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F078CBYx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F078RBHx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F078RBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F078VBHx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F078VBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F091CBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F091CBUx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F091CCTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F091CCUx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F091RBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F091RCHx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F091RCTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F091RCYx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F091VBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F091VCHx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F091VCTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F098CCTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F098CCUx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F098RCHx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F098RCTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F098RCYx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F098VCHx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32F098VCTx,
    //
    // STM32F1 Series
    //
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F100C4,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F100C6,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F100C8,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F100CB,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F100R4,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F100R6,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F100R8,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F100RB,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F100RC,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F100RD,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F100RE,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F100V8,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F100VB,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F100VC,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F100VD,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F100VE,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F100ZC,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F100ZD,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F100ZE,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F101C4,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F101C6,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F101C8,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F101CB,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F101R4,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F101R6,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F101R8,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F101RB,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F101RC,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F101RD,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F101RE,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F101RF,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F101RG,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F101T4,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F101T6,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F101T8,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F101TB,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F101V8,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F101VB,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F101VC,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F101VD,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F101VE,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F101VF,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F101VG,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F101ZC,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F101ZD,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F101ZE,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F101ZF,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F101ZG,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F102C4,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F102C6,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F102C8,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F102CB,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F102R4,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F102R6,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F102R8,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F102RB,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F103C4,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F103C6,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F103C8,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F103CB,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F103R4,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F103R6,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F103R8,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F103RB,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F103RC,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F103RD,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F103RE,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F103RF,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F103RG,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F103T4,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F103T6,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F103T8,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F103TB,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F103V8,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F103VB,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F103VC,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F103VD,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F103VE,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F103VF,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F103VG,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F103ZC,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F103ZD,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F103ZE,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F103ZF,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F103ZG,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F105R8,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F105RB,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F105RC,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F105V8,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F105VB,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F105VC,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F107RB,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F107RC,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F107VB,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F107VC,
    //
    // STM32F2 Series
    //
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F205RBTx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F205RCTx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F205RETx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F205REYx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F205RFTx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F205RGTx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F205RGYx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F205VBTx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F205VCTx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F205VETx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F205VFTx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F205VGTx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F205ZCTx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F205ZETx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F205ZFTx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F205ZGTx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F207ICHx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F207ICTx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F207IEHx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F207IETx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F207IFHx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F207IFTx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F207IGHx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F207IGTx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F207VCTx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F207VETx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F207VFTx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F207VGTx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F207ZCTx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F207ZETx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F207ZFTx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F207ZGTx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F215RETx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F215RGTx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F215VETx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F215VGTx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F215ZETx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F215ZGTx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F217IEHx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F217IETx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F217IGHx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F217IGTx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F217VETx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F217VGTx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F217ZETx,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32F217ZGTx,
    //
    //STM32F3 Series
    //
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F301C6Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F301C8Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F301C8Yx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F301K6Ux,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F301K8Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F301K8Ux,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F301R6Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F301R8Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F302C6Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F302C8Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F302C8Yx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F302CBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F302CCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F302K6Ux,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F302K8Ux,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F302R6Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F302R8Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F302RBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F302RCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F302RDTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F302RETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F302VBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F302VCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F302VCYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F302VDHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F302VDTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F302VEHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F302VETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F302ZDTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F302ZETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F303C6Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F303C8Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F303CBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F303CCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F303K6Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F303K6Ux,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F303K8Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F303K8Ux,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F303R6Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F303R8Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F303RBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F303RCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F303RDTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F303RETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F303VBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F303VCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F303VCYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F303VDHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F303VDTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F303VEHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F303VETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F303VEYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F303ZDTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F303ZETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F318C8Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F318C8Yx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F318K8Ux,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F328C8Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F334C4Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F334C6Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F334C8Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F334C8Yx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F334K4Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F334K4Ux,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F334K6Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F334K6Ux,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F334K8Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F334K8Ux,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F334R6Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F334R8Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F358CCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F358RCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F358VCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F373C8Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F373CBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F373CCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F373R8Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F373RBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F373RCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F373V8Hx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F373V8Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F373VBHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F373VBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F373VCHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F373VCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F378CCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F378RCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F378RCYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F378VCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F398VETx,
    //
    // STM32F4 Series
    //
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F401CBUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F401CBYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F401CCUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F401CCYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F401CDUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F401CDYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F401CEUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F401CEYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F401RBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F401RCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F401RDTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F401RETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F401VBHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F401VBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F401VCHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F401VCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F401VDHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F401VDTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F401VEHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F401VETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F405OEYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F405OGYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F405RGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F405VGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F405ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F407IEHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F407IETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F407IGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F407IGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F407VETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F407VGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F407ZETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F407ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F410C8Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F410C8Ux,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F410CBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F410CBUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F410R8Ix,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F410R8Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F410RBIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F410RBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F410T8Yx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F410TBYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F411CCUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F411CCYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F411CEUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F411CEYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F411RCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F411RETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F411VCHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F411VCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F411VEHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F411VETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F412CEUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F412CGUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F412RETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F412REYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F412RGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F412RGYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F412VEHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F412VETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F412VGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F412VGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F412ZEJx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F412ZETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F412ZGJx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F412ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F413CGUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F413CHUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F413MGYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F413MHYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F413RGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F413RHTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F413VGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F413VGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F413VHHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F413VHTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F413ZGJx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F413ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F413ZHJx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F413ZHTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F415OGYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F415RGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F415VGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F415ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F417IEHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F417IETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F417IGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F417IGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F417VETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F417VGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F417ZETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F417ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F423CHUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F423MHYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F423RHTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F423VHHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F423VHTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F423ZHJx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F423ZHTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F427AGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F427AIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F427IGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F427IGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F427IIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F427IITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F427VGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F427VITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F427ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F427ZITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F429AGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F429AIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F429BETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F429BGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F429BITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F429IEHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F429IETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F429IGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F429IGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F429IIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F429IITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F429NEHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F429NGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F429NIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F429VETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F429VGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F429VITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F429ZETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F429ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F429ZITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F429ZIYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F437AIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F437IGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F437IGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F437IIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F437IITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F437VGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F437VITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F437ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F437ZITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F439AIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F439BGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F439BITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F439IGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F439IGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F439IIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F439IITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F439NGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F439NIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F439VGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F439VITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F439ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F439ZITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F439ZIYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F446MCYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F446MEYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F446RCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F446RETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F446VCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F446VETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F446ZCHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F446ZCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F446ZEHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F446ZEJx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F446ZETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F469AEHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F469AEYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F469AGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F469AGYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F469AIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F469AIYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F469BETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F469BGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F469BITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F469IEHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F469IETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F469IGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F469IGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F469IIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F469IITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F469NEHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F469NGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F469NIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F469VETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F469VGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F469VITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F469ZETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F469ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F469ZITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F479AGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F479AGYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F479AIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F479AIYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F479BGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F479BITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F479IGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F479IGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F479IIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F479IITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F479NGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F479NIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F479VGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F479VITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F479ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F479ZITx,
    //
    // STM32F7 Series
    //
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F722ICKx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F722ICTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F722IEKx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F722IETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F722RCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F722RETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F722VCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F722VETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F722ZCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F722ZETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F723ICKx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F723ICTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F723IEKx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F723IETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F723VEYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F723ZCIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F723ZCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F723ZEIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F723ZETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F730I8Kx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F730R8Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F730V8Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F730Z8Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F732IEKx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F732IETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F732RETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F732VETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F732ZETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F733IEKx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F733IETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F733VETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F733VEYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F733ZEKx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F733ZETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F745IEKx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F745IETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F745IGKx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F745IGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F745VEHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F745VETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F745VGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F745VGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F745ZETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F745ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F746BETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F746BGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F746IEKx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F746IETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F746IGKx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F746IGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F746NEHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F746NGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F746VEHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F746VETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F746VGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F746VGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F746ZETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F746ZEYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F746ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F746ZGYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F750N8Hx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F750V8Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F750Z8Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F756BGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F756IGKx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F756IGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F756NGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F756VGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F756VGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F756ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F756ZGYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F765BGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F765BITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F765IGKx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F765IGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F765IIKx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F765IITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F765NGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F765NIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F765VGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F765VITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F765ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F765ZITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F767BGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F767BITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F767IGKx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F767IGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F767IIKx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F767IITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F767NGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F767NIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F767VGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F767VITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F767ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F767ZITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F768AIYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F769AGYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F769AIYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F769BGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F769BITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F769IGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F769IITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F769NGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F769NIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F777BITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F777IIKx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F777IITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F777NIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F777VITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F777ZITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F778AIYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F779AIYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F779BITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F779IITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32F779NIHx,
    //
    // STM32G0 Series
    //
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G030C6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G030C8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G030F6Px,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G030J6Mx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G030K6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G030K8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G031C4Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G031C4Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G031C6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G031C6Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G031C8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G031C8Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G031F4Px,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G031F6Px,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G031F8Px,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G031G4Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G031G6Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G031G8Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G031J4Mx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G031J6Mx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G031K4Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G031K4Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G031K6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G031K6Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G031K8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G031K8Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G031Y8Yx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G041C6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G041C6Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G041C8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G041C8Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G041F6Px,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G041F8Px,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G041G6Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G041G8Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G041J6Mx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G041K6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G041K6Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G041K8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G041K8Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G041Y8Yx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G070CBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G070KBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G070RBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G071C6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G071C6Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G071C8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G071C8Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G071CBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G071CBUx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G071EBYx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G071G6Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G071G8Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G071G8UxN,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G071GBUx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G071GBUxN,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G071K6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G071K6Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G071K8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G071K8TxN,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G071K8Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G071K8UxN,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G071KBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G071KBTxN,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G071KBUx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G071KBUxN,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G071R6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G071R8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G071RBIx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G071RBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G081CBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G081CBUx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G081EBYx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G081GBUx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G081GBUxN,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G081KBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G081KBTxN,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G081KBUx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G081KBUxN,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G081RBIx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32G081RBTx,
    //
    // STM32G4 Series
    //
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G431C6Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G431C6Ux,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G431C8Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G431C8Ux,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G431CBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G431CBUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G431CBYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G431K6Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G431K6Ux,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G431K8Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G431K8Ux,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G431KBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G431KBUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G431M6Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G431M8Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G431R6Ix,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G431R6Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G431R8Ix,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G431R8Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G431RBIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G431RBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G431V6Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G431V8Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G431VBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G441CBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G441CBUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G441CBYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G441KBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G441KBUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G441RBIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G441RBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G441VBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G471CCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G471CCUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G471CETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G471CEUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G471MCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G471METx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G471MEYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G471QCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G471QETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G471RCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G471RE,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G471VCHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G471VCIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G471VCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G471VEHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G471VEIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G471VETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G473CBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G473CBUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G473CCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G473CCUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G473CETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G473CEUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G473MBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G473MCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G473METx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G473MEUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G473QBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G473QCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G473QETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G473RBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G473RCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G473RETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G473VBHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G473VBIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G473VBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G473VCHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G473VCIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G473VCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G473VEHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G473VEIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G473VETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G474CBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G474CBUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G474CCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G474CCUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G474CETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G474CEUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G474MBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G474MCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G474METx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G474MEYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G474QBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G474QCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G474QETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G474RBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G474RCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G474RETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G474VBHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G474VBIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G474VBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G474VCHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G474VCIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G474VCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G474VEHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G474VEIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G474VETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G483CETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G483CEUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G483METx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G483MEYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G483QETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G483RETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G483VEHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G483VEIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G483VETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G484CETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G484CEUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G484METx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G484MEYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G484QETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G484RETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G484VEHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G484VEIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32G484VETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32GBK1CB,
    //
    // STM32H7 Series
    //
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H742AGIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H742AIIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H742BGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H742BITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H742IGKx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H742IGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H742IIKx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H742IITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H742VGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H742VGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H742VIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H742VITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H742XGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H742XIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H742ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H742ZITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H743AGIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H743AIIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H743BGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H743BITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H743IGKx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H743IGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H743IIKx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H743IITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H743VGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H743VGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H743VIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H743VITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H743XGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H743XIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H743ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H743ZITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H745BGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H745BITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H745IGKx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H745IGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H745IIKx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H745IITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H745XGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H745XIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H745ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H745ZITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H747AGIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H747AIIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H747BGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H747BITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H747IGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H747IITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H747XGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H747XIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H747ZIYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H750IBKx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H750IBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H750VBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H750XBHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H750ZBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H753AIIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H753BITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H753IIKx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H753IITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H753VIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H753VITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H753XIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H753ZITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H755BITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H755IIKx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H755IITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H755XIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H755ZITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H757AIIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H757BITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H757IITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H757XIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H757ZIYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7A3AGIxQ,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7A3AIIxQ,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7A3IGKx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7A3IGKxQ,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7A3IGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7A3IGTxQ,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7A3IIKx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7A3IIKxQ,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7A3IITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7A3IITxQ,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7A3LGHxQ,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7A3LIHxQ,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7A3NGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7A3NIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7A3QIYxQ,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7A3RGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7A3RITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7A3VGHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7A3VGHxQ,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7A3VGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7A3VGTxQ,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7A3VIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7A3VIHxQ,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7A3VITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7A3VITxQ,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7A3ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7A3ZGTxQ,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7A3ZITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7A3ZITxQ,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7B0ABIxQ,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7B0IBKxQ,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7B0IBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7B0RBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7B0VBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7B0ZBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7B3AIIxQ,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7B3IIKx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7B3IIKxQ,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7B3IITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7B3IITxQ,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7B3LIHxQ,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7B3NIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7B3QIYxQ,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7B3RITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7B3VIHx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7B3VIHxQ,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7B3VITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7B3VITxQ,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7B3ZITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32H7B3ZITxQ,
    //
    // STM32L0 Series
    //
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L010C6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L010F4Px,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L010K4Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L010K8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L010R8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L010RBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L011D3Px,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L011D4Px,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L011E4Yx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L011F3Px,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L011F3Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L011F4Px,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L011F4Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L011G3Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L011G4Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L011K3Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L011K3Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L011K4Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L011K4Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L021D4Px,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L021F4Px,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L021F4Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L021G4Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L021K4Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L021K4Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L031C4Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L031C6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L031E4Yx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L031E6Yx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L031F4Px,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L031F6Px,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L031G4Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L031G6Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L031G6UxS,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L031K4Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L031K4Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L031K6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L031K6Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L041C4Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L041C6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L041E6Yx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L041F6Px,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L041G6Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L041K6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L041K6Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L051C6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L051C8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L051K6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L051K6Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L051K8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L051K8Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L051R6Hx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L051R6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L051R8Hx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L051R8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L051T6Yx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L051T8Yx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L052C6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L052C8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L052K6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L052K6Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L052K8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L052K8Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L052R6Hx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L052R6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L052R8Hx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L052R8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L052T6Yx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L052T8Yx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L053C6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L053C8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L053R6Hx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L053R6Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L053R8Hx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L053R8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L062K8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L062K8Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L063C8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L063R8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L071C8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L071CBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L071CBYx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L071CZTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L071CZYx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L071K8Ux,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L071KBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L071KBUx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L071KZTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L071KZUx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L071RBHx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L071RBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L071RZHx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L071RZTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L071V8Ix,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L071V8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L071VBIx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L071VBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L071VZIx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L071VZTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L072CBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L072CBYx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L072CZEx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L072CZTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L072CZYx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L072KBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L072KBUx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L072KZTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L072KZUx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L072RBHx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L072RBIx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L072RBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L072RZHx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L072RZIx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L072RZTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L072V8Ix,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L072V8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L072VBIx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L072VBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L072VZIx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L072VZTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L073CBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L073CZTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L073RBHx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L073RBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L073RZHx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L073RZTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L073V8Ix,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L073V8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L073VBIx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L073VBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L073VZIx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L073VZTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L081CZTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L081KZTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L081KZUx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L082CZYx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L082KBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L082KBUx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L082KZTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L082KZUx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L083CBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L083CZTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L083RBHx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L083RBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L083RZHx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L083RZTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L083V8Ix,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L083V8Tx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L083VBIx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L083VBTx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L083VZIx,
    #[strum(props(target = "thumbv6m-none-eabi"))]
    STM32L083VZTx,
    //
    // STM32L1 Series
    //
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L100C6,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L100C6xxA,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L100R8,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L100R8xxA,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L100RB,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L100RBxxA,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L100RC,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151C6,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151C6xxA,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151C8,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151C8xxA,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151CB,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151CBxxA,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151CC,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151QC,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151QD,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151QE,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151R6,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151R6xxA,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151R8,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151R8xxA,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151RB,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151RBxxA,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151RC,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151RCxxA,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151RD,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151RE,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151UC,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151V8,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151V8xxA,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151VB,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151VBxxA,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151VC,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151VCxxA,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151VD,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151VE,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151ZC,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151ZD,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L151ZE,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152C6,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152C6xxA,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152C8,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152C8xxA,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152CB,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152CBxxA,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152CC,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152QC,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152QD,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152QE,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152R6,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152R6xxA,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152R8,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152R8xxA,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152RB,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152RBxxA,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152RC,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152RCxxA,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152RD,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152RE,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152V8,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152V8xxA,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152VB,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152VBxxA,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152VC,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152VCxxA,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152VD,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152VE,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152ZC,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152ZD,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L152ZE,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L162QD,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L162RC,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L162RCxxA,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L162RD,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L162RE,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L162VC,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L162VCxxA,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L162VD,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L162VE,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L162ZD,
    #[strum(props(target = "thumbv7m-none-eabi"))]
    STM32L162ZE,
    //
    // STM32L4 Series
    //
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L412C8Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L412C8Ux,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L412CBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L412CBUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L412K8Ix,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L412K8Tx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L412K8Ux,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L412KBIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L412KBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L412KBUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L412T8Yx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L412TBYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L422CBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L422CBUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L422KBIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L422KBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L422KBUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L422TBYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L431CBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L431CBUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L431CBYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L431CCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L431CCUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L431CCYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L431KBUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L431KCUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L431RBIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L431RBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L431RBYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L431RCIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L431RCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L431RCYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L431VCIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L431VCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L432KBUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L432KCUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L433CBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L433CBUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L433CBYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L433CCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L433CCUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L433CCYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L433RBIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L433RBTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L433RBYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L433RCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L433VCIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L433VCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L442KCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L443CCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L443CCUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L443CCYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L443RCIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L443RCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L443RCYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L443VCIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L443VCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L451CCUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L451CEUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L451RCIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L451RCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L451RCYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L451REIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L451RETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L451REYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L451VCIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L451VCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L451VEIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L451VETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L452CCUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L452CEUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L452RCIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L452RCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L452RCYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L452REIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L452RETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L452REYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L452VCIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L452VCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L452VEIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L452VETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L462CEUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L462REIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L462RETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L462REYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L462VEIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L462VETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L471JEYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L471JGYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L471QEIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L471QGIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L471RCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L471RCYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L471RETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L471RGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L471RGYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L471VCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L471VETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L471VGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L471ZETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L471ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L475JEYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L475JGYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L475QEIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L475QGIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L475RCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L475RETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L475RGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L475VCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L475VETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L475VGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L475ZETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L475ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L476JEYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L476JGYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L476MEYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L476MGYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L476QEIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L476QGIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L476RCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L476RETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L476RGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L476VCTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L476VETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L476VGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L476ZETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L476ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L485JCYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L486JGYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L486QGIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L486RGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L486VGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L486ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L496AGIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L496QGIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L496RGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L496VETx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L496VEYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L496VGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L496VGYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L496ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L496ZGYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4A6AGIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4A6QGIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4A6RGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4A6VGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4A6VGYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4A6ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4A6ZGYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4P5AGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4P5CGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4P5QGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4P5RGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4P5VGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4P5ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4Q5AGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4Q5CGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4Q5QGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4Q5RGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4Q5VGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4Q5ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4R5AGIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4R5AIIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4R5QGIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4R5QIIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4R5VGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4R5VITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4R5ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4R5ZGYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4R5ZITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4R5ZIYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4R7AIIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4R7VITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4R7ZITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4R9AGIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4R9AIIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4R9VGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4R9VITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4R9ZGJx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4R9ZGTx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4R9ZGYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4R9ZIJx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4R9ZITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4R9ZIYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4S5AIIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4S5QIIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4S5VITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4S5ZITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4S5ZIYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4S7AIIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4S7VITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4S7ZITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4S9AIIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4S9ZITx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4S9ZIYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32L4S9ZJIx,
    //
    // STM32L5 Series
    //
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L552CCTx,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L552CCUx,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L552CETx,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L552CETxP,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L552CEUx,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L552CEUxP,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L552MEYxP,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L552MEYxQ,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L552QCIxQ,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L552QEIx,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L552QEIxP,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L552QEIxQ,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L552RCTx,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L552RETx,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L552RETxP,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L552RETxQ,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L552VCTxQ,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L552VETx,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L552VETxQ,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L552ZCTxQ,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L552ZETx,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L552ZETxQ,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L562CETx,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L562CETxP,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L562CEUx,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L562CEUxP,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L562MEYxP,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L562MEYxQ,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L562QEIxP,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L562QEIxQ,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L562RETx,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L562RETxP,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L562VETx,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L562VETxQ,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L562ZETx,
    #[strum(props(target = "thumbv8m.main-none-eabihf"))]
    STM32L562ZETxQ,
    //
    // STM32WB Series
    //
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32WB30CEUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32WB35CCUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32WB35CEUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32WB50CGUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32WB55CCUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32WB55CEUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32WB55CGUx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32WB55RCVx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32WB55REVx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32WB55RGVx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32WB55VCQx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32WB55VCYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32WB55VEQx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32WB55VEYx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32WB55VGQx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32WB55VGYx,
    //
    // STM32WL Series
    //
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32WLE5J8Ix,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32WLE5JBIx,
    #[strum(props(target = "thumbv7em-none-eabihf"))]
    STM32WLE5JCIx,
    //
    // Generic Cortex-M0
    //
    #[strum(serialize = "cortex-m0", props(target = "thumbv6m-none-eabi"))]
    cortex_m0,
    //
    // Generic Cortex-M4
    //
    #[strum(serialize = "cortex-m4", props(target = "thumbv7em-none-eabi"))]
    cortex_m4,
    //
    // Generic Cortex-M3
    //
    #[strum(serialize = "cortex-m3", props(target = "thumbv7m-none-eabi"))]
    cortex_m3,
    //
    // Generic Cortex-M33
    //
    #[strum(serialize = "cortex-m33", props(target = "thumbv8m.base-none-eabi"))]
    cortex_m33,
    //
    // Generic Cortex-M7
    //
    #[strum(serialize = "cortex-m7", props(target = "thumbv7em-none-eabi"))]
    cortex_m7,
    //
    // Generic Riscv
    //
    #[strum(props(target = "riscv32i-unknown-none-elf"))]
    riscv,
}
