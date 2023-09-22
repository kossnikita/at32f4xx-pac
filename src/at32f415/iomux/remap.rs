#[doc = "Register `REMAP` reader"]
pub type R = crate::R<REMAP_SPEC>;
#[doc = "Register `REMAP` writer"]
pub type W = crate::W<REMAP_SPEC>;
#[doc = "Field `SPI1_MUX0` reader - SPI1 muxing bit0"]
pub type SPI1_MUX0_R = crate::BitReader;
#[doc = "Field `SPI1_MUX0` writer - SPI1 muxing bit0"]
pub type SPI1_MUX0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C1_MUX` reader - I2C1 muxing"]
pub type I2C1_MUX_R = crate::BitReader<I2C1_MUX_A>;
#[doc = "I2C1 muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1_MUX_A {
    #[doc = "0: SCL/PB6, SDA/PB7 SMBA/PB5"]
    Mux0 = 0,
    #[doc = "1: SCL/PB8, SDA/PB9 SMBA/PB5"]
    Mux1 = 1,
}
impl From<I2C1_MUX_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1_MUX_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C1_MUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1_MUX_A {
        match self.bits {
            false => I2C1_MUX_A::Mux0,
            true => I2C1_MUX_A::Mux1,
        }
    }
    #[doc = "SCL/PB6, SDA/PB7 SMBA/PB5"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == I2C1_MUX_A::Mux0
    }
    #[doc = "SCL/PB8, SDA/PB9 SMBA/PB5"]
    #[inline(always)]
    pub fn is_mux1(&self) -> bool {
        *self == I2C1_MUX_A::Mux1
    }
}
#[doc = "Field `I2C1_MUX` writer - I2C1 muxing"]
pub type I2C1_MUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, I2C1_MUX_A>;
impl<'a, REG, const O: u8> I2C1_MUX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SCL/PB6, SDA/PB7 SMBA/PB5"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1_MUX_A::Mux0)
    }
    #[doc = "SCL/PB8, SDA/PB9 SMBA/PB5"]
    #[inline(always)]
    pub fn mux1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C1_MUX_A::Mux1)
    }
}
#[doc = "Field `USART1_MUX` reader - USART1 muxing"]
pub type USART1_MUX_R = crate::BitReader<USART1_MUX_A>;
#[doc = "USART1 muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USART1_MUX_A {
    #[doc = "0: TX/PA9, RX/PA10"]
    Mux0 = 0,
    #[doc = "1: TX/PB6, RX/PB7"]
    Mux1 = 1,
}
impl From<USART1_MUX_A> for bool {
    #[inline(always)]
    fn from(variant: USART1_MUX_A) -> Self {
        variant as u8 != 0
    }
}
impl USART1_MUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART1_MUX_A {
        match self.bits {
            false => USART1_MUX_A::Mux0,
            true => USART1_MUX_A::Mux1,
        }
    }
    #[doc = "TX/PA9, RX/PA10"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == USART1_MUX_A::Mux0
    }
    #[doc = "TX/PB6, RX/PB7"]
    #[inline(always)]
    pub fn is_mux1(&self) -> bool {
        *self == USART1_MUX_A::Mux1
    }
}
#[doc = "Field `USART1_MUX` writer - USART1 muxing"]
pub type USART1_MUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, USART1_MUX_A>;
impl<'a, REG, const O: u8> USART1_MUX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TX/PA9, RX/PA10"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(USART1_MUX_A::Mux0)
    }
    #[doc = "TX/PB6, RX/PB7"]
    #[inline(always)]
    pub fn mux1(self) -> &'a mut crate::W<REG> {
        self.variant(USART1_MUX_A::Mux1)
    }
}
#[doc = "Field `USART3_MUX` reader - USART3 muxing"]
pub type USART3_MUX_R = crate::FieldReader<USART3_MUX_A>;
#[doc = "USART3 muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART3_MUX_A {
    #[doc = "0: TX/PB10, RX/PB11, CK/PB12, CTS/PB13, RTS/PB14"]
    Mux0 = 0,
    #[doc = "1: TX/PC10, RX/PC11, CK/PC12, CTS/PB13, RTS/PB14"]
    Mux1 = 1,
    #[doc = "2: TX/PA7, RX/PA6, CK/PA5, CTS/PB1, RTS/PB0"]
    Mux2 = 2,
}
impl From<USART3_MUX_A> for u8 {
    #[inline(always)]
    fn from(variant: USART3_MUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART3_MUX_A {
    type Ux = u8;
}
impl USART3_MUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USART3_MUX_A> {
        match self.bits {
            0 => Some(USART3_MUX_A::Mux0),
            1 => Some(USART3_MUX_A::Mux1),
            2 => Some(USART3_MUX_A::Mux2),
            _ => None,
        }
    }
    #[doc = "TX/PB10, RX/PB11, CK/PB12, CTS/PB13, RTS/PB14"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == USART3_MUX_A::Mux0
    }
    #[doc = "TX/PC10, RX/PC11, CK/PC12, CTS/PB13, RTS/PB14"]
    #[inline(always)]
    pub fn is_mux1(&self) -> bool {
        *self == USART3_MUX_A::Mux1
    }
    #[doc = "TX/PA7, RX/PA6, CK/PA5, CTS/PB1, RTS/PB0"]
    #[inline(always)]
    pub fn is_mux2(&self) -> bool {
        *self == USART3_MUX_A::Mux2
    }
}
#[doc = "Field `USART3_MUX` writer - USART3 muxing"]
pub type USART3_MUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, USART3_MUX_A>;
impl<'a, REG, const O: u8> USART3_MUX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TX/PB10, RX/PB11, CK/PB12, CTS/PB13, RTS/PB14"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(USART3_MUX_A::Mux0)
    }
    #[doc = "TX/PC10, RX/PC11, CK/PC12, CTS/PB13, RTS/PB14"]
    #[inline(always)]
    pub fn mux1(self) -> &'a mut crate::W<REG> {
        self.variant(USART3_MUX_A::Mux1)
    }
    #[doc = "TX/PA7, RX/PA6, CK/PA5, CTS/PB1, RTS/PB0"]
    #[inline(always)]
    pub fn mux2(self) -> &'a mut crate::W<REG> {
        self.variant(USART3_MUX_A::Mux2)
    }
}
#[doc = "Field `TMR1_MUX` reader - TMR1 muxing"]
pub type TMR1_MUX_R = crate::FieldReader<TMR1_MUX_A>;
#[doc = "TMR1 muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TMR1_MUX_A {
    #[doc = "0: EXT/PA12, CH1/PA8, CH2/PA9, CH3/PA10, CH4/PA11, BRK/PB12, CH1C/PB13, CH2C/PB14, CH3C/PB15"]
    Mux0 = 0,
    #[doc = "1: EXT/PA12, CH1/PA8, CH2/PA9, CH3/PA10, CH4/PA11, BRK/PA6, CH1C/PA7, CH2C/PB0, CH3C/PB1"]
    Mux1 = 1,
}
impl From<TMR1_MUX_A> for u8 {
    #[inline(always)]
    fn from(variant: TMR1_MUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TMR1_MUX_A {
    type Ux = u8;
}
impl TMR1_MUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TMR1_MUX_A> {
        match self.bits {
            0 => Some(TMR1_MUX_A::Mux0),
            1 => Some(TMR1_MUX_A::Mux1),
            _ => None,
        }
    }
    #[doc = "EXT/PA12, CH1/PA8, CH2/PA9, CH3/PA10, CH4/PA11, BRK/PB12, CH1C/PB13, CH2C/PB14, CH3C/PB15"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == TMR1_MUX_A::Mux0
    }
    #[doc = "EXT/PA12, CH1/PA8, CH2/PA9, CH3/PA10, CH4/PA11, BRK/PA6, CH1C/PA7, CH2C/PB0, CH3C/PB1"]
    #[inline(always)]
    pub fn is_mux1(&self) -> bool {
        *self == TMR1_MUX_A::Mux1
    }
}
#[doc = "Field `TMR1_MUX` writer - TMR1 muxing"]
pub type TMR1_MUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, TMR1_MUX_A>;
impl<'a, REG, const O: u8> TMR1_MUX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "EXT/PA12, CH1/PA8, CH2/PA9, CH3/PA10, CH4/PA11, BRK/PB12, CH1C/PB13, CH2C/PB14, CH3C/PB15"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(TMR1_MUX_A::Mux0)
    }
    #[doc = "EXT/PA12, CH1/PA8, CH2/PA9, CH3/PA10, CH4/PA11, BRK/PA6, CH1C/PA7, CH2C/PB0, CH3C/PB1"]
    #[inline(always)]
    pub fn mux1(self) -> &'a mut crate::W<REG> {
        self.variant(TMR1_MUX_A::Mux1)
    }
}
#[doc = "Field `TMR2_MUX` reader - TMR2 muxing"]
pub type TMR2_MUX_R = crate::FieldReader<TMR2_MUX_A>;
#[doc = "TMR2 muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TMR2_MUX_A {
    #[doc = "0: CH1/EXT/PA0, CH2/PA1, CH3/PA2, CH4/PA3"]
    Mux0 = 0,
    #[doc = "1: CH1/EXT/PA15, CH2/PB3, CH3/PA2, CH4/PA3"]
    Mux1 = 1,
    #[doc = "2: CH1/EXT/PA0, CH2/PA1, CH3/PB10, CH4/PB11"]
    Mux2 = 2,
    #[doc = "3: CH1/EXT/PA15, CH2/PB3, CH3/PB10, CH4/PB11"]
    Mux3 = 3,
}
impl From<TMR2_MUX_A> for u8 {
    #[inline(always)]
    fn from(variant: TMR2_MUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TMR2_MUX_A {
    type Ux = u8;
}
impl TMR2_MUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR2_MUX_A {
        match self.bits {
            0 => TMR2_MUX_A::Mux0,
            1 => TMR2_MUX_A::Mux1,
            2 => TMR2_MUX_A::Mux2,
            3 => TMR2_MUX_A::Mux3,
            _ => unreachable!(),
        }
    }
    #[doc = "CH1/EXT/PA0, CH2/PA1, CH3/PA2, CH4/PA3"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == TMR2_MUX_A::Mux0
    }
    #[doc = "CH1/EXT/PA15, CH2/PB3, CH3/PA2, CH4/PA3"]
    #[inline(always)]
    pub fn is_mux1(&self) -> bool {
        *self == TMR2_MUX_A::Mux1
    }
    #[doc = "CH1/EXT/PA0, CH2/PA1, CH3/PB10, CH4/PB11"]
    #[inline(always)]
    pub fn is_mux2(&self) -> bool {
        *self == TMR2_MUX_A::Mux2
    }
    #[doc = "CH1/EXT/PA15, CH2/PB3, CH3/PB10, CH4/PB11"]
    #[inline(always)]
    pub fn is_mux3(&self) -> bool {
        *self == TMR2_MUX_A::Mux3
    }
}
#[doc = "Field `TMR2_MUX` writer - TMR2 muxing"]
pub type TMR2_MUX_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, TMR2_MUX_A>;
impl<'a, REG, const O: u8> TMR2_MUX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH1/EXT/PA0, CH2/PA1, CH3/PA2, CH4/PA3"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(TMR2_MUX_A::Mux0)
    }
    #[doc = "CH1/EXT/PA15, CH2/PB3, CH3/PA2, CH4/PA3"]
    #[inline(always)]
    pub fn mux1(self) -> &'a mut crate::W<REG> {
        self.variant(TMR2_MUX_A::Mux1)
    }
    #[doc = "CH1/EXT/PA0, CH2/PA1, CH3/PB10, CH4/PB11"]
    #[inline(always)]
    pub fn mux2(self) -> &'a mut crate::W<REG> {
        self.variant(TMR2_MUX_A::Mux2)
    }
    #[doc = "CH1/EXT/PA15, CH2/PB3, CH3/PB10, CH4/PB11"]
    #[inline(always)]
    pub fn mux3(self) -> &'a mut crate::W<REG> {
        self.variant(TMR2_MUX_A::Mux3)
    }
}
#[doc = "Field `TMR3_MUX` reader - TMR3 muxing"]
pub type TMR3_MUX_R = crate::FieldReader<TMR3_MUX_A>;
#[doc = "TMR3 muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TMR3_MUX_A {
    #[doc = "0: CH1/PA6, CH2/PA7, CH3/PB0 and CH4/PB1"]
    Mux0 = 0,
    #[doc = "2: CH1/EXT/PA0, CH2/PA1, CH3/PB10, CH4/PB11"]
    Mux2 = 2,
    #[doc = "3: CH1/EXT/PA15, CH2/PB3, CH3/PB10, CH4/PB11"]
    Mux3 = 3,
}
impl From<TMR3_MUX_A> for u8 {
    #[inline(always)]
    fn from(variant: TMR3_MUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TMR3_MUX_A {
    type Ux = u8;
}
impl TMR3_MUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TMR3_MUX_A> {
        match self.bits {
            0 => Some(TMR3_MUX_A::Mux0),
            2 => Some(TMR3_MUX_A::Mux2),
            3 => Some(TMR3_MUX_A::Mux3),
            _ => None,
        }
    }
    #[doc = "CH1/PA6, CH2/PA7, CH3/PB0 and CH4/PB1"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == TMR3_MUX_A::Mux0
    }
    #[doc = "CH1/EXT/PA0, CH2/PA1, CH3/PB10, CH4/PB11"]
    #[inline(always)]
    pub fn is_mux2(&self) -> bool {
        *self == TMR3_MUX_A::Mux2
    }
    #[doc = "CH1/EXT/PA15, CH2/PB3, CH3/PB10, CH4/PB11"]
    #[inline(always)]
    pub fn is_mux3(&self) -> bool {
        *self == TMR3_MUX_A::Mux3
    }
}
#[doc = "Field `TMR3_MUX` writer - TMR3 muxing"]
pub type TMR3_MUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, TMR3_MUX_A>;
impl<'a, REG, const O: u8> TMR3_MUX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CH1/PA6, CH2/PA7, CH3/PB0 and CH4/PB1"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(TMR3_MUX_A::Mux0)
    }
    #[doc = "CH1/EXT/PA0, CH2/PA1, CH3/PB10, CH4/PB11"]
    #[inline(always)]
    pub fn mux2(self) -> &'a mut crate::W<REG> {
        self.variant(TMR3_MUX_A::Mux2)
    }
    #[doc = "CH1/EXT/PA15, CH2/PB3, CH3/PB10, CH4/PB11"]
    #[inline(always)]
    pub fn mux3(self) -> &'a mut crate::W<REG> {
        self.variant(TMR3_MUX_A::Mux3)
    }
}
#[doc = "Field `CAN_MUX` reader - CAN1 muxing"]
pub type CAN_MUX_R = crate::FieldReader<CAN_MUX_A>;
#[doc = "CAN1 muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAN_MUX_A {
    #[doc = "0: RX/PA11, TX/PA12"]
    Mux0 = 0,
    #[doc = "2: RX/PB8, TX/PB9"]
    Mux2 = 2,
}
impl From<CAN_MUX_A> for u8 {
    #[inline(always)]
    fn from(variant: CAN_MUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CAN_MUX_A {
    type Ux = u8;
}
impl CAN_MUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CAN_MUX_A> {
        match self.bits {
            0 => Some(CAN_MUX_A::Mux0),
            2 => Some(CAN_MUX_A::Mux2),
            _ => None,
        }
    }
    #[doc = "RX/PA11, TX/PA12"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == CAN_MUX_A::Mux0
    }
    #[doc = "RX/PB8, TX/PB9"]
    #[inline(always)]
    pub fn is_mux2(&self) -> bool {
        *self == CAN_MUX_A::Mux2
    }
}
#[doc = "Field `CAN_MUX` writer - CAN1 muxing"]
pub type CAN_MUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, CAN_MUX_A>;
impl<'a, REG, const O: u8> CAN_MUX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RX/PA11, TX/PA12"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(CAN_MUX_A::Mux0)
    }
    #[doc = "RX/PB8, TX/PB9"]
    #[inline(always)]
    pub fn mux2(self) -> &'a mut crate::W<REG> {
        self.variant(CAN_MUX_A::Mux2)
    }
}
#[doc = "Field `PD01_MUX` reader - PD0/PD1 muxing on OSCIN/OSCOUT"]
pub type PD01_MUX_R = crate::BitReader<PD01_MUX_A>;
#[doc = "PD0/PD1 muxing on OSCIN/OSCOUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PD01_MUX_A {
    #[doc = "0: Not PD0 and PD1 mapping"]
    Mux0 = 0,
    #[doc = "1: PD0 is mapped to HEXT_IN, while PD1 to HEXT_OUT"]
    Hext = 1,
}
impl From<PD01_MUX_A> for bool {
    #[inline(always)]
    fn from(variant: PD01_MUX_A) -> Self {
        variant as u8 != 0
    }
}
impl PD01_MUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PD01_MUX_A {
        match self.bits {
            false => PD01_MUX_A::Mux0,
            true => PD01_MUX_A::Hext,
        }
    }
    #[doc = "Not PD0 and PD1 mapping"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == PD01_MUX_A::Mux0
    }
    #[doc = "PD0 is mapped to HEXT_IN, while PD1 to HEXT_OUT"]
    #[inline(always)]
    pub fn is_hext(&self) -> bool {
        *self == PD01_MUX_A::Hext
    }
}
#[doc = "Field `PD01_MUX` writer - PD0/PD1 muxing on OSCIN/OSCOUT"]
pub type PD01_MUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PD01_MUX_A>;
impl<'a, REG, const O: u8> PD01_MUX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not PD0 and PD1 mapping"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(PD01_MUX_A::Mux0)
    }
    #[doc = "PD0 is mapped to HEXT_IN, while PD1 to HEXT_OUT"]
    #[inline(always)]
    pub fn hext(self) -> &'a mut crate::W<REG> {
        self.variant(PD01_MUX_A::Hext)
    }
}
#[doc = "Field `TMR5CH4_MUX` reader - TMR5 channel4 internal muxing"]
pub type TMR5CH4_MUX_R = crate::BitReader<TMR5CH4_MUX_A>;
#[doc = "TMR5 channel4 internal muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMR5CH4_MUX_A {
    #[doc = "0: TMR5_CH4 is connected to PA3"]
    Pa3 = 0,
    #[doc = "1: TMR5_CH4 is connected to LICK. LICK can be calibrated"]
    Lick = 1,
}
impl From<TMR5CH4_MUX_A> for bool {
    #[inline(always)]
    fn from(variant: TMR5CH4_MUX_A) -> Self {
        variant as u8 != 0
    }
}
impl TMR5CH4_MUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR5CH4_MUX_A {
        match self.bits {
            false => TMR5CH4_MUX_A::Pa3,
            true => TMR5CH4_MUX_A::Lick,
        }
    }
    #[doc = "TMR5_CH4 is connected to PA3"]
    #[inline(always)]
    pub fn is_pa3(&self) -> bool {
        *self == TMR5CH4_MUX_A::Pa3
    }
    #[doc = "TMR5_CH4 is connected to LICK. LICK can be calibrated"]
    #[inline(always)]
    pub fn is_lick(&self) -> bool {
        *self == TMR5CH4_MUX_A::Lick
    }
}
#[doc = "Field `TMR5CH4_MUX` writer - TMR5 channel4 internal muxing"]
pub type TMR5CH4_MUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, TMR5CH4_MUX_A>;
impl<'a, REG, const O: u8> TMR5CH4_MUX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TMR5_CH4 is connected to PA3"]
    #[inline(always)]
    pub fn pa3(self) -> &'a mut crate::W<REG> {
        self.variant(TMR5CH4_MUX_A::Pa3)
    }
    #[doc = "TMR5_CH4 is connected to LICK. LICK can be calibrated"]
    #[inline(always)]
    pub fn lick(self) -> &'a mut crate::W<REG> {
        self.variant(TMR5CH4_MUX_A::Lick)
    }
}
#[doc = "Field `ADC1_ETP_MUX` reader - ADC1 external trigger preempted conversion muxing"]
pub type ADC1_ETP_MUX_R = crate::BitReader<ADC1_ETP_MUX_A>;
#[doc = "ADC1 external trigger preempted conversion muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC1_ETP_MUX_A {
    #[doc = "0: ADC1 external trigger preempted conversion is connected to EXINT15"]
    Exint15 = 0,
    #[doc = "1: ADC1 external trigger preempted conversion is connected to TMR1 channel 4"]
    Tmr1ch4 = 1,
}
impl From<ADC1_ETP_MUX_A> for bool {
    #[inline(always)]
    fn from(variant: ADC1_ETP_MUX_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC1_ETP_MUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC1_ETP_MUX_A {
        match self.bits {
            false => ADC1_ETP_MUX_A::Exint15,
            true => ADC1_ETP_MUX_A::Tmr1ch4,
        }
    }
    #[doc = "ADC1 external trigger preempted conversion is connected to EXINT15"]
    #[inline(always)]
    pub fn is_exint15(&self) -> bool {
        *self == ADC1_ETP_MUX_A::Exint15
    }
    #[doc = "ADC1 external trigger preempted conversion is connected to TMR1 channel 4"]
    #[inline(always)]
    pub fn is_tmr1ch4(&self) -> bool {
        *self == ADC1_ETP_MUX_A::Tmr1ch4
    }
}
#[doc = "Field `ADC1_ETP_MUX` writer - ADC1 external trigger preempted conversion muxing"]
pub type ADC1_ETP_MUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADC1_ETP_MUX_A>;
impl<'a, REG, const O: u8> ADC1_ETP_MUX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC1 external trigger preempted conversion is connected to EXINT15"]
    #[inline(always)]
    pub fn exint15(self) -> &'a mut crate::W<REG> {
        self.variant(ADC1_ETP_MUX_A::Exint15)
    }
    #[doc = "ADC1 external trigger preempted conversion is connected to TMR1 channel 4"]
    #[inline(always)]
    pub fn tmr1ch4(self) -> &'a mut crate::W<REG> {
        self.variant(ADC1_ETP_MUX_A::Tmr1ch4)
    }
}
#[doc = "Field `ADC1_ETO_MUX` reader - ADC1 external trigger ordinary conversion muxing"]
pub type ADC1_ETO_MUX_R = crate::BitReader<ADC1_ETO_MUX_A>;
#[doc = "ADC1 external trigger ordinary conversion muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC1_ETO_MUX_A {
    #[doc = "0: ADC1 external trigger ordinary conversion is connected to EXINT11"]
    Exint11 = 0,
    #[doc = "1: ADC1 external trigger ordinary conversion TMR1_TRGO"]
    Tmr1Trgo = 1,
}
impl From<ADC1_ETO_MUX_A> for bool {
    #[inline(always)]
    fn from(variant: ADC1_ETO_MUX_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC1_ETO_MUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC1_ETO_MUX_A {
        match self.bits {
            false => ADC1_ETO_MUX_A::Exint11,
            true => ADC1_ETO_MUX_A::Tmr1Trgo,
        }
    }
    #[doc = "ADC1 external trigger ordinary conversion is connected to EXINT11"]
    #[inline(always)]
    pub fn is_exint11(&self) -> bool {
        *self == ADC1_ETO_MUX_A::Exint11
    }
    #[doc = "ADC1 external trigger ordinary conversion TMR1_TRGO"]
    #[inline(always)]
    pub fn is_tmr1_trgo(&self) -> bool {
        *self == ADC1_ETO_MUX_A::Tmr1Trgo
    }
}
#[doc = "Field `ADC1_ETO_MUX` writer - ADC1 external trigger ordinary conversion muxing"]
pub type ADC1_ETO_MUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ADC1_ETO_MUX_A>;
impl<'a, REG, const O: u8> ADC1_ETO_MUX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC1 external trigger ordinary conversion is connected to EXINT11"]
    #[inline(always)]
    pub fn exint11(self) -> &'a mut crate::W<REG> {
        self.variant(ADC1_ETO_MUX_A::Exint11)
    }
    #[doc = "ADC1 external trigger ordinary conversion TMR1_TRGO"]
    #[inline(always)]
    pub fn tmr1_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(ADC1_ETO_MUX_A::Tmr1Trgo)
    }
}
#[doc = "Field `SWJTAG_MUX` reader - SWD JTAG muxing"]
pub type SWJTAG_MUX_R = crate::FieldReader<SWJTAG_MUX_A>;
#[doc = "SWD JTAG muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWJTAG_MUX_A {
    #[doc = "0: Supports SWD and JTAG. All SWJTAG pins cannot be used as GPIOs"]
    SwdandJtag = 0,
    #[doc = "1: Supports SWD and JTAG. NJTRST is disabled. PB4 can be used as GPIO"]
    SwdandJtagwithoutNjtrst = 1,
    #[doc = "2: Supports SWD but JTAG is disabled. PA15/PB3/PB4 can be used as GPIOs"]
    Swd = 2,
    #[doc = "4: SWD and JTAG are disabled. All SWJTAG pins can be used as GPIOs"]
    Disable = 4,
}
impl From<SWJTAG_MUX_A> for u8 {
    #[inline(always)]
    fn from(variant: SWJTAG_MUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SWJTAG_MUX_A {
    type Ux = u8;
}
impl SWJTAG_MUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SWJTAG_MUX_A> {
        match self.bits {
            0 => Some(SWJTAG_MUX_A::SwdandJtag),
            1 => Some(SWJTAG_MUX_A::SwdandJtagwithoutNjtrst),
            2 => Some(SWJTAG_MUX_A::Swd),
            4 => Some(SWJTAG_MUX_A::Disable),
            _ => None,
        }
    }
    #[doc = "Supports SWD and JTAG. All SWJTAG pins cannot be used as GPIOs"]
    #[inline(always)]
    pub fn is_swdand_jtag(&self) -> bool {
        *self == SWJTAG_MUX_A::SwdandJtag
    }
    #[doc = "Supports SWD and JTAG. NJTRST is disabled. PB4 can be used as GPIO"]
    #[inline(always)]
    pub fn is_swdand_jtagwithout_njtrst(&self) -> bool {
        *self == SWJTAG_MUX_A::SwdandJtagwithoutNjtrst
    }
    #[doc = "Supports SWD but JTAG is disabled. PA15/PB3/PB4 can be used as GPIOs"]
    #[inline(always)]
    pub fn is_swd(&self) -> bool {
        *self == SWJTAG_MUX_A::Swd
    }
    #[doc = "SWD and JTAG are disabled. All SWJTAG pins can be used as GPIOs"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SWJTAG_MUX_A::Disable
    }
}
#[doc = "Field `SWJTAG_MUX` writer - SWD JTAG muxing"]
pub type SWJTAG_MUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, SWJTAG_MUX_A>;
impl<'a, REG, const O: u8> SWJTAG_MUX_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Supports SWD and JTAG. All SWJTAG pins cannot be used as GPIOs"]
    #[inline(always)]
    pub fn swdand_jtag(self) -> &'a mut crate::W<REG> {
        self.variant(SWJTAG_MUX_A::SwdandJtag)
    }
    #[doc = "Supports SWD and JTAG. NJTRST is disabled. PB4 can be used as GPIO"]
    #[inline(always)]
    pub fn swdand_jtagwithout_njtrst(self) -> &'a mut crate::W<REG> {
        self.variant(SWJTAG_MUX_A::SwdandJtagwithoutNjtrst)
    }
    #[doc = "Supports SWD but JTAG is disabled. PA15/PB3/PB4 can be used as GPIOs"]
    #[inline(always)]
    pub fn swd(self) -> &'a mut crate::W<REG> {
        self.variant(SWJTAG_MUX_A::Swd)
    }
    #[doc = "SWD and JTAG are disabled. All SWJTAG pins can be used as GPIOs"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(SWJTAG_MUX_A::Disable)
    }
}
#[doc = "Field `SPI1_MUX1` reader - SPI1 muxing bit1"]
pub type SPI1_MUX1_R = crate::BitReader<SPI1_MUX1_A>;
#[doc = "SPI1 muxing bit1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI1_MUX1_A {
    #[doc = "0: CS/PA4, SCK/PA5, MISO/PA6, MOSI/PA7, MCK/PB0"]
    Mux0 = 0,
    #[doc = "1: CS/PA15, SCK/PB3, MISO/PB4, MOSI/PB5,MCK/PB6"]
    Mux1 = 1,
}
impl From<SPI1_MUX1_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1_MUX1_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI1_MUX1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1_MUX1_A {
        match self.bits {
            false => SPI1_MUX1_A::Mux0,
            true => SPI1_MUX1_A::Mux1,
        }
    }
    #[doc = "CS/PA4, SCK/PA5, MISO/PA6, MOSI/PA7, MCK/PB0"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == SPI1_MUX1_A::Mux0
    }
    #[doc = "CS/PA15, SCK/PB3, MISO/PB4, MOSI/PB5,MCK/PB6"]
    #[inline(always)]
    pub fn is_mux1(&self) -> bool {
        *self == SPI1_MUX1_A::Mux1
    }
}
#[doc = "Field `SPI1_MUX1` writer - SPI1 muxing bit1"]
pub type SPI1_MUX1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, SPI1_MUX1_A>;
impl<'a, REG, const O: u8> SPI1_MUX1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CS/PA4, SCK/PA5, MISO/PA6, MOSI/PA7, MCK/PB0"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1_MUX1_A::Mux0)
    }
    #[doc = "CS/PA15, SCK/PB3, MISO/PB4, MOSI/PB5,MCK/PB6"]
    #[inline(always)]
    pub fn mux1(self) -> &'a mut crate::W<REG> {
        self.variant(SPI1_MUX1_A::Mux1)
    }
}
impl R {
    #[doc = "Bit 0 - SPI1 muxing bit0"]
    #[inline(always)]
    pub fn spi1_mux0(&self) -> SPI1_MUX0_R {
        SPI1_MUX0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C1 muxing"]
    #[inline(always)]
    pub fn i2c1_mux(&self) -> I2C1_MUX_R {
        I2C1_MUX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USART1 muxing"]
    #[inline(always)]
    pub fn usart1_mux(&self) -> USART1_MUX_R {
        USART1_MUX_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - USART3 muxing"]
    #[inline(always)]
    pub fn usart3_mux(&self) -> USART3_MUX_R {
        USART3_MUX_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - TMR1 muxing"]
    #[inline(always)]
    pub fn tmr1_mux(&self) -> TMR1_MUX_R {
        TMR1_MUX_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TMR2 muxing"]
    #[inline(always)]
    pub fn tmr2_mux(&self) -> TMR2_MUX_R {
        TMR2_MUX_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - TMR3 muxing"]
    #[inline(always)]
    pub fn tmr3_mux(&self) -> TMR3_MUX_R {
        TMR3_MUX_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 13:14 - CAN1 muxing"]
    #[inline(always)]
    pub fn can_mux(&self) -> CAN_MUX_R {
        CAN_MUX_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - PD0/PD1 muxing on OSCIN/OSCOUT"]
    #[inline(always)]
    pub fn pd01_mux(&self) -> PD01_MUX_R {
        PD01_MUX_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TMR5 channel4 internal muxing"]
    #[inline(always)]
    pub fn tmr5ch4_mux(&self) -> TMR5CH4_MUX_R {
        TMR5CH4_MUX_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC1 external trigger preempted conversion muxing"]
    #[inline(always)]
    pub fn adc1_etp_mux(&self) -> ADC1_ETP_MUX_R {
        ADC1_ETP_MUX_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC1 external trigger ordinary conversion muxing"]
    #[inline(always)]
    pub fn adc1_eto_mux(&self) -> ADC1_ETO_MUX_R {
        ADC1_ETO_MUX_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SWD JTAG muxing"]
    #[inline(always)]
    pub fn swjtag_mux(&self) -> SWJTAG_MUX_R {
        SWJTAG_MUX_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 31 - SPI1 muxing bit1"]
    #[inline(always)]
    pub fn spi1_mux1(&self) -> SPI1_MUX1_R {
        SPI1_MUX1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI1 muxing bit0"]
    #[inline(always)]
    #[must_use]
    pub fn spi1_mux0(&mut self) -> SPI1_MUX0_W<REMAP_SPEC, 0> {
        SPI1_MUX0_W::new(self)
    }
    #[doc = "Bit 1 - I2C1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_mux(&mut self) -> I2C1_MUX_W<REMAP_SPEC, 1> {
        I2C1_MUX_W::new(self)
    }
    #[doc = "Bit 2 - USART1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn usart1_mux(&mut self) -> USART1_MUX_W<REMAP_SPEC, 2> {
        USART1_MUX_W::new(self)
    }
    #[doc = "Bits 4:5 - USART3 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn usart3_mux(&mut self) -> USART3_MUX_W<REMAP_SPEC, 4> {
        USART3_MUX_W::new(self)
    }
    #[doc = "Bits 6:7 - TMR1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr1_mux(&mut self) -> TMR1_MUX_W<REMAP_SPEC, 6> {
        TMR1_MUX_W::new(self)
    }
    #[doc = "Bits 8:9 - TMR2 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr2_mux(&mut self) -> TMR2_MUX_W<REMAP_SPEC, 8> {
        TMR2_MUX_W::new(self)
    }
    #[doc = "Bits 10:11 - TMR3 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr3_mux(&mut self) -> TMR3_MUX_W<REMAP_SPEC, 10> {
        TMR3_MUX_W::new(self)
    }
    #[doc = "Bits 13:14 - CAN1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn can_mux(&mut self) -> CAN_MUX_W<REMAP_SPEC, 13> {
        CAN_MUX_W::new(self)
    }
    #[doc = "Bit 15 - PD0/PD1 muxing on OSCIN/OSCOUT"]
    #[inline(always)]
    #[must_use]
    pub fn pd01_mux(&mut self) -> PD01_MUX_W<REMAP_SPEC, 15> {
        PD01_MUX_W::new(self)
    }
    #[doc = "Bit 16 - TMR5 channel4 internal muxing"]
    #[inline(always)]
    #[must_use]
    pub fn tmr5ch4_mux(&mut self) -> TMR5CH4_MUX_W<REMAP_SPEC, 16> {
        TMR5CH4_MUX_W::new(self)
    }
    #[doc = "Bit 17 - ADC1 external trigger preempted conversion muxing"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_etp_mux(&mut self) -> ADC1_ETP_MUX_W<REMAP_SPEC, 17> {
        ADC1_ETP_MUX_W::new(self)
    }
    #[doc = "Bit 18 - ADC1 external trigger ordinary conversion muxing"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_eto_mux(&mut self) -> ADC1_ETO_MUX_W<REMAP_SPEC, 18> {
        ADC1_ETO_MUX_W::new(self)
    }
    #[doc = "Bits 24:26 - SWD JTAG muxing"]
    #[inline(always)]
    #[must_use]
    pub fn swjtag_mux(&mut self) -> SWJTAG_MUX_W<REMAP_SPEC, 24> {
        SWJTAG_MUX_W::new(self)
    }
    #[doc = "Bit 31 - SPI1 muxing bit1"]
    #[inline(always)]
    #[must_use]
    pub fn spi1_mux1(&mut self) -> SPI1_MUX1_W<REMAP_SPEC, 31> {
        SPI1_MUX1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "IO MUX remap register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REMAP_SPEC;
impl crate::RegisterSpec for REMAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap::R`](R) reader structure"]
impl crate::Readable for REMAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`remap::W`](W) writer structure"]
impl crate::Writable for REMAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REMAP to value 0"]
impl crate::Resettable for REMAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
