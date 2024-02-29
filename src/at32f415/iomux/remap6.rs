#[doc = "Register `REMAP6` reader"]
pub type R = crate::R<REMAP6_SPEC>;
#[doc = "Register `REMAP6` writer"]
pub type W = crate::W<REMAP6_SPEC>;
#[doc = "CAN1 muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAN1_GMUX_A {
    #[doc = "0: RX/PA11, TX/PA12"]
    Mux0 = 0,
    #[doc = "2: RX/PB8, TX/PB9"]
    Mux1 = 2,
}
impl From<CAN1_GMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: CAN1_GMUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CAN1_GMUX_A {
    type Ux = u8;
}
#[doc = "Field `CAN1_GMUX` reader - CAN1 muxing"]
pub type CAN1_GMUX_R = crate::FieldReader<CAN1_GMUX_A>;
impl CAN1_GMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CAN1_GMUX_A> {
        match self.bits {
            0 => Some(CAN1_GMUX_A::Mux0),
            2 => Some(CAN1_GMUX_A::Mux1),
            _ => None,
        }
    }
    #[doc = "RX/PA11, TX/PA12"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == CAN1_GMUX_A::Mux0
    }
    #[doc = "RX/PB8, TX/PB9"]
    #[inline(always)]
    pub fn is_mux1(&self) -> bool {
        *self == CAN1_GMUX_A::Mux1
    }
}
#[doc = "Field `CAN1_GMUX` writer - CAN1 muxing"]
pub type CAN1_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CAN1_GMUX_A>;
impl<'a, REG> CAN1_GMUX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RX/PA11, TX/PA12"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(CAN1_GMUX_A::Mux0)
    }
    #[doc = "RX/PB8, TX/PB9"]
    #[inline(always)]
    pub fn mux1(self) -> &'a mut crate::W<REG> {
        self.variant(CAN1_GMUX_A::Mux1)
    }
}
#[doc = "SDIO1 muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDIO1_GMUX_A {
    #[doc = "0: D0/PC8, D1/PC9, D2/PC10, D3/PC11, D4/PB8, D5/PB9, D6/PC6, D7/PC7, CK/PC12, CMD/PD2"]
    Mux0 = 0,
    #[doc = "4: D0/PC0, D1/PC1, D2/PC2, D3/PC3, D4/PA4, D5/PA5, D6/PA6, D7/PA7, CK/PC4, CMD/PC5"]
    Mux1 = 4,
    #[doc = "5: D0/PA4, D1/PA5, D2/PA6, D3/PA7, CK/PC4, CMD/PC5"]
    Mux2 = 5,
    #[doc = "6: D0/PC0, D1/PC1, D2/PC2, D3/PC3, D4/PA4, D5/PA5, D6/PA6, D7/PA7, CK/PA2, CMD/PA3"]
    Mux3 = 6,
    #[doc = "7: D0/PA4, D1/PA5, D2/PA6, D3/PA7, CK/PA2, CMD/PA3"]
    Mux4 = 7,
}
impl From<SDIO1_GMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: SDIO1_GMUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SDIO1_GMUX_A {
    type Ux = u8;
}
#[doc = "Field `SDIO1_GMUX` reader - SDIO1 muxing"]
pub type SDIO1_GMUX_R = crate::FieldReader<SDIO1_GMUX_A>;
impl SDIO1_GMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SDIO1_GMUX_A> {
        match self.bits {
            0 => Some(SDIO1_GMUX_A::Mux0),
            4 => Some(SDIO1_GMUX_A::Mux1),
            5 => Some(SDIO1_GMUX_A::Mux2),
            6 => Some(SDIO1_GMUX_A::Mux3),
            7 => Some(SDIO1_GMUX_A::Mux4),
            _ => None,
        }
    }
    #[doc = "D0/PC8, D1/PC9, D2/PC10, D3/PC11, D4/PB8, D5/PB9, D6/PC6, D7/PC7, CK/PC12, CMD/PD2"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == SDIO1_GMUX_A::Mux0
    }
    #[doc = "D0/PC0, D1/PC1, D2/PC2, D3/PC3, D4/PA4, D5/PA5, D6/PA6, D7/PA7, CK/PC4, CMD/PC5"]
    #[inline(always)]
    pub fn is_mux1(&self) -> bool {
        *self == SDIO1_GMUX_A::Mux1
    }
    #[doc = "D0/PA4, D1/PA5, D2/PA6, D3/PA7, CK/PC4, CMD/PC5"]
    #[inline(always)]
    pub fn is_mux2(&self) -> bool {
        *self == SDIO1_GMUX_A::Mux2
    }
    #[doc = "D0/PC0, D1/PC1, D2/PC2, D3/PC3, D4/PA4, D5/PA5, D6/PA6, D7/PA7, CK/PA2, CMD/PA3"]
    #[inline(always)]
    pub fn is_mux3(&self) -> bool {
        *self == SDIO1_GMUX_A::Mux3
    }
    #[doc = "D0/PA4, D1/PA5, D2/PA6, D3/PA7, CK/PA2, CMD/PA3"]
    #[inline(always)]
    pub fn is_mux4(&self) -> bool {
        *self == SDIO1_GMUX_A::Mux4
    }
}
#[doc = "Field `SDIO1_GMUX` writer - SDIO1 muxing"]
pub type SDIO1_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4, SDIO1_GMUX_A>;
impl<'a, REG> SDIO1_GMUX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "D0/PC8, D1/PC9, D2/PC10, D3/PC11, D4/PB8, D5/PB9, D6/PC6, D7/PC7, CK/PC12, CMD/PD2"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(SDIO1_GMUX_A::Mux0)
    }
    #[doc = "D0/PC0, D1/PC1, D2/PC2, D3/PC3, D4/PA4, D5/PA5, D6/PA6, D7/PA7, CK/PC4, CMD/PC5"]
    #[inline(always)]
    pub fn mux1(self) -> &'a mut crate::W<REG> {
        self.variant(SDIO1_GMUX_A::Mux1)
    }
    #[doc = "D0/PA4, D1/PA5, D2/PA6, D3/PA7, CK/PC4, CMD/PC5"]
    #[inline(always)]
    pub fn mux2(self) -> &'a mut crate::W<REG> {
        self.variant(SDIO1_GMUX_A::Mux2)
    }
    #[doc = "D0/PC0, D1/PC1, D2/PC2, D3/PC3, D4/PA4, D5/PA5, D6/PA6, D7/PA7, CK/PA2, CMD/PA3"]
    #[inline(always)]
    pub fn mux3(self) -> &'a mut crate::W<REG> {
        self.variant(SDIO1_GMUX_A::Mux3)
    }
    #[doc = "D0/PA4, D1/PA5, D2/PA6, D3/PA7, CK/PA2, CMD/PA3"]
    #[inline(always)]
    pub fn mux4(self) -> &'a mut crate::W<REG> {
        self.variant(SDIO1_GMUX_A::Mux4)
    }
}
#[doc = "USART1 muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART1_GMUX_A {
    #[doc = "0: TX/PA9, RX/PA10"]
    Mux0 = 0,
    #[doc = "1: TX/PB6, RX/PB7"]
    Mux1 = 1,
}
impl From<USART1_GMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: USART1_GMUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART1_GMUX_A {
    type Ux = u8;
}
#[doc = "Field `USART1_GMUX` reader - USART1 muxing"]
pub type USART1_GMUX_R = crate::FieldReader<USART1_GMUX_A>;
impl USART1_GMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<USART1_GMUX_A> {
        match self.bits {
            0 => Some(USART1_GMUX_A::Mux0),
            1 => Some(USART1_GMUX_A::Mux1),
            _ => None,
        }
    }
    #[doc = "TX/PA9, RX/PA10"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == USART1_GMUX_A::Mux0
    }
    #[doc = "TX/PB6, RX/PB7"]
    #[inline(always)]
    pub fn is_mux1(&self) -> bool {
        *self == USART1_GMUX_A::Mux1
    }
}
#[doc = "Field `USART1_GMUX` writer - USART1 muxing"]
pub type USART1_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4, USART1_GMUX_A>;
impl<'a, REG> USART1_GMUX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TX/PA9, RX/PA10"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(USART1_GMUX_A::Mux0)
    }
    #[doc = "TX/PB6, RX/PB7"]
    #[inline(always)]
    pub fn mux1(self) -> &'a mut crate::W<REG> {
        self.variant(USART1_GMUX_A::Mux1)
    }
}
#[doc = "USART3 muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART3_GMUX_A {
    #[doc = "0: TX/PB10, RX/PB11, CK/PB12, CTS/PB13, RTS/PB14"]
    Mux0 = 0,
    #[doc = "1: TX/PC10, RX/PC11, CK/PC12, CTS/PB13, RTS/PB14"]
    Mux1 = 1,
    #[doc = "2: TX/PA7, RX/PA6, CK/PA5, CTS/PB1 RTS/PB0"]
    Mux2 = 2,
}
impl From<USART3_GMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: USART3_GMUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USART3_GMUX_A {
    type Ux = u8;
}
#[doc = "Field `USART3_GMUX` reader - USART3 muxing"]
pub type USART3_GMUX_R = crate::FieldReader<USART3_GMUX_A>;
impl USART3_GMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<USART3_GMUX_A> {
        match self.bits {
            0 => Some(USART3_GMUX_A::Mux0),
            1 => Some(USART3_GMUX_A::Mux1),
            2 => Some(USART3_GMUX_A::Mux2),
            _ => None,
        }
    }
    #[doc = "TX/PB10, RX/PB11, CK/PB12, CTS/PB13, RTS/PB14"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == USART3_GMUX_A::Mux0
    }
    #[doc = "TX/PC10, RX/PC11, CK/PC12, CTS/PB13, RTS/PB14"]
    #[inline(always)]
    pub fn is_mux1(&self) -> bool {
        *self == USART3_GMUX_A::Mux1
    }
    #[doc = "TX/PA7, RX/PA6, CK/PA5, CTS/PB1 RTS/PB0"]
    #[inline(always)]
    pub fn is_mux2(&self) -> bool {
        *self == USART3_GMUX_A::Mux2
    }
}
#[doc = "Field `USART3_GMUX` writer - USART3 muxing"]
pub type USART3_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4, USART3_GMUX_A>;
impl<'a, REG> USART3_GMUX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TX/PB10, RX/PB11, CK/PB12, CTS/PB13, RTS/PB14"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(USART3_GMUX_A::Mux0)
    }
    #[doc = "TX/PC10, RX/PC11, CK/PC12, CTS/PB13, RTS/PB14"]
    #[inline(always)]
    pub fn mux1(self) -> &'a mut crate::W<REG> {
        self.variant(USART3_GMUX_A::Mux1)
    }
    #[doc = "TX/PA7, RX/PA6, CK/PA5, CTS/PB1 RTS/PB0"]
    #[inline(always)]
    pub fn mux2(self) -> &'a mut crate::W<REG> {
        self.variant(USART3_GMUX_A::Mux2)
    }
}
#[doc = "UART4 muxing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UART4_GMUX_A {
    #[doc = "0: TX/PC10 RX/PC11"]
    Mux0 = 0,
    #[doc = "1: TX/PF4 RX/PF5"]
    Mux1 = 1,
}
impl From<UART4_GMUX_A> for u8 {
    #[inline(always)]
    fn from(variant: UART4_GMUX_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UART4_GMUX_A {
    type Ux = u8;
}
#[doc = "Field `UART4_GMUX` reader - UART4 muxing"]
pub type UART4_GMUX_R = crate::FieldReader<UART4_GMUX_A>;
impl UART4_GMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<UART4_GMUX_A> {
        match self.bits {
            0 => Some(UART4_GMUX_A::Mux0),
            1 => Some(UART4_GMUX_A::Mux1),
            _ => None,
        }
    }
    #[doc = "TX/PC10 RX/PC11"]
    #[inline(always)]
    pub fn is_mux0(&self) -> bool {
        *self == UART4_GMUX_A::Mux0
    }
    #[doc = "TX/PF4 RX/PF5"]
    #[inline(always)]
    pub fn is_mux1(&self) -> bool {
        *self == UART4_GMUX_A::Mux1
    }
}
#[doc = "Field `UART4_GMUX` writer - UART4 muxing"]
pub type UART4_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4, UART4_GMUX_A>;
impl<'a, REG> UART4_GMUX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TX/PC10 RX/PC11"]
    #[inline(always)]
    pub fn mux0(self) -> &'a mut crate::W<REG> {
        self.variant(UART4_GMUX_A::Mux0)
    }
    #[doc = "TX/PF4 RX/PF5"]
    #[inline(always)]
    pub fn mux1(self) -> &'a mut crate::W<REG> {
        self.variant(UART4_GMUX_A::Mux1)
    }
}
impl R {
    #[doc = "Bits 0:3 - CAN1 muxing"]
    #[inline(always)]
    pub fn can1_gmux(&self) -> CAN1_GMUX_R {
        CAN1_GMUX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SDIO1 muxing"]
    #[inline(always)]
    pub fn sdio1_gmux(&self) -> SDIO1_GMUX_R {
        SDIO1_GMUX_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - USART1 muxing"]
    #[inline(always)]
    pub fn usart1_gmux(&self) -> USART1_GMUX_R {
        USART1_GMUX_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - USART3 muxing"]
    #[inline(always)]
    pub fn usart3_gmux(&self) -> USART3_GMUX_R {
        USART3_GMUX_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - UART4 muxing"]
    #[inline(always)]
    pub fn uart4_gmux(&self) -> UART4_GMUX_R {
        UART4_GMUX_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMAP6")
            .field("uart4_gmux", &format_args!("{}", self.uart4_gmux().bits()))
            .field(
                "usart3_gmux",
                &format_args!("{}", self.usart3_gmux().bits()),
            )
            .field(
                "usart1_gmux",
                &format_args!("{}", self.usart1_gmux().bits()),
            )
            .field("sdio1_gmux", &format_args!("{}", self.sdio1_gmux().bits()))
            .field("can1_gmux", &format_args!("{}", self.can1_gmux().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<REMAP6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - CAN1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn can1_gmux(&mut self) -> CAN1_GMUX_W<REMAP6_SPEC> {
        CAN1_GMUX_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - SDIO1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn sdio1_gmux(&mut self) -> SDIO1_GMUX_W<REMAP6_SPEC> {
        SDIO1_GMUX_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - USART1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn usart1_gmux(&mut self) -> USART1_GMUX_W<REMAP6_SPEC> {
        USART1_GMUX_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - USART3 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn usart3_gmux(&mut self) -> USART3_GMUX_W<REMAP6_SPEC> {
        USART3_GMUX_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - UART4 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn uart4_gmux(&mut self) -> UART4_GMUX_W<REMAP6_SPEC> {
        UART4_GMUX_W::new(self, 28)
    }
}
#[doc = "IO MUX remap register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REMAP6_SPEC;
impl crate::RegisterSpec for REMAP6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap6::R`](R) reader structure"]
impl crate::Readable for REMAP6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`remap6::W`](W) writer structure"]
impl crate::Writable for REMAP6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REMAP6 to value 0"]
impl crate::Resettable for REMAP6_SPEC {
    const RESET_VALUE: u32 = 0;
}
