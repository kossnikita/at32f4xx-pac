#[doc = "Register `REMAP8` reader"]
pub type R = crate::R<REMAP8_SPEC>;
#[doc = "Register `REMAP8` writer"]
pub type W = crate::W<REMAP8_SPEC>;
#[doc = "Field `EMAC_GMUX` reader - Ethernet MAC muxing"]
pub type EMAC_GMUX_R = crate::FieldReader;
#[doc = "Field `EMAC_GMUX` writer - Ethernet MAC muxing"]
pub type EMAC_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MII_RMII_SEL_GMUX` reader - MII_RMII select muxing"]
pub type MII_RMII_SEL_GMUX_R = crate::BitReader;
#[doc = "Field `MII_RMII_SEL_GMUX` writer - MII_RMII select muxing"]
pub type MII_RMII_SEL_GMUX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTP_PPS_GMUX` reader - PTP_PPS muxing"]
pub type PTP_PPS_GMUX_R = crate::BitReader;
#[doc = "Field `PTP_PPS_GMUX` writer - PTP_PPS muxing"]
pub type PTP_PPS_GMUX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART6_GMUX` reader - USART6 muxing"]
pub type USART6_GMUX_R = crate::FieldReader;
#[doc = "Field `USART6_GMUX` writer - USART6 muxing"]
pub type USART6_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `UART7_GMUX` reader - UART7 muxing"]
pub type UART7_GMUX_R = crate::FieldReader;
#[doc = "Field `UART7_GMUX` writer - UART7 muxing"]
pub type UART7_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `UART8_GMUX` reader - UART8 muxing"]
pub type UART8_GMUX_R = crate::FieldReader;
#[doc = "Field `UART8_GMUX` writer - UART8 muxing"]
pub type UART8_GMUX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 16:17 - Ethernet MAC muxing"]
    #[inline(always)]
    pub fn emac_gmux(&self) -> EMAC_GMUX_R {
        EMAC_GMUX_R::new(((self.bits >> 16) & 3) as u8)
    }
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
            .field("emac_gmux", &self.emac_gmux())
            .field("mii_rmii_sel_gmux", &self.mii_rmii_sel_gmux())
            .field("ptp_pps_gmux", &self.ptp_pps_gmux())
            .field("usart6_gmux", &self.usart6_gmux())
            .field("uart7_gmux", &self.uart7_gmux())
            .field("uart8_gmux", &self.uart8_gmux())
            .finish()
    }
}
impl W {
    #[doc = "Bits 16:17 - Ethernet MAC muxing"]
    #[inline(always)]
    #[must_use]
    pub fn emac_gmux(&mut self) -> EMAC_GMUX_W<REMAP8_SPEC> {
        EMAC_GMUX_W::new(self, 16)
    }
    #[doc = "Bit 18 - MII_RMII select muxing"]
    #[inline(always)]
    #[must_use]
    pub fn mii_rmii_sel_gmux(&mut self) -> MII_RMII_SEL_GMUX_W<REMAP8_SPEC> {
        MII_RMII_SEL_GMUX_W::new(self, 18)
    }
    #[doc = "Bit 19 - PTP_PPS muxing"]
    #[inline(always)]
    #[must_use]
    pub fn ptp_pps_gmux(&mut self) -> PTP_PPS_GMUX_W<REMAP8_SPEC> {
        PTP_PPS_GMUX_W::new(self, 19)
    }
    #[doc = "Bits 20:23 - USART6 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn usart6_gmux(&mut self) -> USART6_GMUX_W<REMAP8_SPEC> {
        USART6_GMUX_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - UART7 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn uart7_gmux(&mut self) -> UART7_GMUX_W<REMAP8_SPEC> {
        UART7_GMUX_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - UART8 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn uart8_gmux(&mut self) -> UART8_GMUX_W<REMAP8_SPEC> {
        UART8_GMUX_W::new(self, 28)
    }
}
#[doc = "IO MUX remap register 8 (IOMUX_REMAP8)\n\nYou can [`read`](crate::Reg::read) this register and get [`remap8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remap8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REMAP8_SPEC;
impl crate::RegisterSpec for REMAP8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap8::R`](R) reader structure"]
impl crate::Readable for REMAP8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`remap8::W`](W) writer structure"]
impl crate::Writable for REMAP8_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REMAP8 to value 0"]
impl crate::Resettable for REMAP8_SPEC {
    const RESET_VALUE: u32 = 0;
}
