#[doc = "Register `REMAP8` reader"]
pub type R = crate::R<REMAP8_SPEC>;
#[doc = "Register `REMAP8` writer"]
pub type W = crate::W<REMAP8_SPEC>;
#[doc = "Field `MII_RMII_SEL_GMUX` reader - MII_RMII select muxing"]
pub type MII_RMII_SEL_GMUX_R = crate::BitReader;
#[doc = "Field `MII_RMII_SEL_GMUX` writer - MII_RMII select muxing"]
pub type MII_RMII_SEL_GMUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PTP_PPS_GMUX` reader - PTP_PPS muxing"]
pub type PTP_PPS_GMUX_R = crate::BitReader;
#[doc = "Field `PTP_PPS_GMUX` writer - PTP_PPS muxing"]
pub type PTP_PPS_GMUX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USART6_GMUX` reader - USART6 muxing"]
pub type USART6_GMUX_R = crate::FieldReader;
#[doc = "Field `USART6_GMUX` writer - USART6 muxing"]
pub type USART6_GMUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UART7_GMUX` reader - UART7 muxing"]
pub type UART7_GMUX_R = crate::FieldReader;
#[doc = "Field `UART7_GMUX` writer - UART7 muxing"]
pub type UART7_GMUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `UART8_GMUX` reader - UART8 muxing"]
pub type UART8_GMUX_R = crate::FieldReader;
#[doc = "Field `UART8_GMUX` writer - UART8 muxing"]
pub type UART8_GMUX_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bit 18 - MII_RMII select muxing"]
    #[inline(always)]
    pub fn mii_rmii_sel_gmux(&self) -> MII_RMII_SEL_GMUX_R {
        MII_RMII_SEL_GMUX_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PTP_PPS muxing"]
    #[inline(always)]
    pub fn ptp_pps_gmux(&self) -> PTP_PPS_GMUX_R {
        PTP_PPS_GMUX_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - USART6 muxing"]
    #[inline(always)]
    pub fn usart6_gmux(&self) -> USART6_GMUX_R {
        USART6_GMUX_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - UART7 muxing"]
    #[inline(always)]
    pub fn uart7_gmux(&self) -> UART7_GMUX_R {
        UART7_GMUX_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - UART8 muxing"]
    #[inline(always)]
    pub fn uart8_gmux(&self) -> UART8_GMUX_R {
        UART8_GMUX_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMAP8")
            .field(
                "mii_rmii_sel_gmux",
                &format_args!("{}", self.mii_rmii_sel_gmux().bit()),
            )
            .field(
                "ptp_pps_gmux",
                &format_args!("{}", self.ptp_pps_gmux().bit()),
            )
            .field(
                "usart6_gmux",
                &format_args!("{}", self.usart6_gmux().bits()),
            )
            .field("uart7_gmux", &format_args!("{}", self.uart7_gmux().bits()))
            .field("uart8_gmux", &format_args!("{}", self.uart8_gmux().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<REMAP8_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 18 - MII_RMII select muxing"]
    #[inline(always)]
    #[must_use]
    pub fn mii_rmii_sel_gmux(&mut self) -> MII_RMII_SEL_GMUX_W<REMAP8_SPEC, 18> {
        MII_RMII_SEL_GMUX_W::new(self)
    }
    #[doc = "Bit 19 - PTP_PPS muxing"]
    #[inline(always)]
    #[must_use]
    pub fn ptp_pps_gmux(&mut self) -> PTP_PPS_GMUX_W<REMAP8_SPEC, 19> {
        PTP_PPS_GMUX_W::new(self)
    }
    #[doc = "Bits 20:23 - USART6 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn usart6_gmux(&mut self) -> USART6_GMUX_W<REMAP8_SPEC, 20> {
        USART6_GMUX_W::new(self)
    }
    #[doc = "Bits 24:27 - UART7 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn uart7_gmux(&mut self) -> UART7_GMUX_W<REMAP8_SPEC, 24> {
        UART7_GMUX_W::new(self)
    }
    #[doc = "Bits 28:31 - UART8 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn uart8_gmux(&mut self) -> UART8_GMUX_W<REMAP8_SPEC, 28> {
        UART8_GMUX_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "IO MUX remap register 8 (IOMUX_REMAP8)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REMAP8_SPEC;
impl crate::RegisterSpec for REMAP8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap8::R`](R) reader structure"]
impl crate::Readable for REMAP8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`remap8::W`](W) writer structure"]
impl crate::Writable for REMAP8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REMAP8 to value 0"]
impl crate::Resettable for REMAP8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
