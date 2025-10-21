#[doc = "Register `GUSBCFG` reader"]
pub type R = crate::R<GUSBCFG_SPEC>;
#[doc = "Register `GUSBCFG` writer"]
pub type W = crate::W<GUSBCFG_SPEC>;
#[doc = "Field `TOUTCAL` reader - HS timeout calibration"]
pub type TOUTCAL_R = crate::FieldReader;
#[doc = "Field `TOUTCAL` writer - HS timeout calibration"]
pub type TOUTCAL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHYIF` reader - PHY Interface"]
pub type PHYIF_R = crate::BitReader;
#[doc = "Field `PHYIF` writer - PHY Interface"]
pub type PHYIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHYSEL` reader - USB 2.0 High-Speed PHY or USB 1.1 Full-Speed Serial Transceiver Select"]
pub type PHYSEL_R = crate::BitReader;
#[doc = "Field `PHYSEL` writer - USB 2.0 High-Speed PHY or USB 1.1 Full-Speed Serial Transceiver Select"]
pub type PHYSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBTRDTIM` reader - USB turnaround time"]
pub type USBTRDTIM_R = crate::FieldReader;
#[doc = "Field `USBTRDTIM` writer - USB turnaround time"]
pub type USBTRDTIM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FHSTMODE` reader - Force host mode"]
pub type FHSTMODE_R = crate::BitReader;
#[doc = "Field `FHSTMODE` writer - Force host mode"]
pub type FHSTMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDEVMODE` reader - Force device mode"]
pub type FDEVMODE_R = crate::BitReader;
#[doc = "Field `FDEVMODE` writer - Force device mode"]
pub type FDEVMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COTXPKT` reader - Corrupt Tx packet"]
pub type COTXPKT_R = crate::BitReader;
#[doc = "Field `COTXPKT` writer - Corrupt Tx packet"]
pub type COTXPKT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - HS timeout calibration"]
    #[inline(always)]
    pub fn toutcal(&self) -> TOUTCAL_R {
        TOUTCAL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - PHY Interface"]
    #[inline(always)]
    pub fn phyif(&self) -> PHYIF_R {
        PHYIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - USB 2.0 High-Speed PHY or USB 1.1 Full-Speed Serial Transceiver Select"]
    #[inline(always)]
    pub fn physel(&self) -> PHYSEL_R {
        PHYSEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 10:13 - USB turnaround time"]
    #[inline(always)]
    pub fn usbtrdtim(&self) -> USBTRDTIM_R {
        USBTRDTIM_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Force host mode"]
    #[inline(always)]
    pub fn fhstmode(&self) -> FHSTMODE_R {
        FHSTMODE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Force device mode"]
    #[inline(always)]
    pub fn fdevmode(&self) -> FDEVMODE_R {
        FDEVMODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Corrupt Tx packet"]
    #[inline(always)]
    pub fn cotxpkt(&self) -> COTXPKT_R {
        COTXPKT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GUSBCFG")
            .field("toutcal", &self.toutcal())
            .field("phyif", &self.phyif())
            .field("physel", &self.physel())
            .field("usbtrdtim", &self.usbtrdtim())
            .field("fhstmode", &self.fhstmode())
            .field("fdevmode", &self.fdevmode())
            .field("cotxpkt", &self.cotxpkt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - HS timeout calibration"]
    #[inline(always)]
    pub fn toutcal(&mut self) -> TOUTCAL_W<'_, GUSBCFG_SPEC> {
        TOUTCAL_W::new(self, 0)
    }
    #[doc = "Bit 3 - PHY Interface"]
    #[inline(always)]
    pub fn phyif(&mut self) -> PHYIF_W<'_, GUSBCFG_SPEC> {
        PHYIF_W::new(self, 3)
    }
    #[doc = "Bit 6 - USB 2.0 High-Speed PHY or USB 1.1 Full-Speed Serial Transceiver Select"]
    #[inline(always)]
    pub fn physel(&mut self) -> PHYSEL_W<'_, GUSBCFG_SPEC> {
        PHYSEL_W::new(self, 6)
    }
    #[doc = "Bits 10:13 - USB turnaround time"]
    #[inline(always)]
    pub fn usbtrdtim(&mut self) -> USBTRDTIM_W<'_, GUSBCFG_SPEC> {
        USBTRDTIM_W::new(self, 10)
    }
    #[doc = "Bit 29 - Force host mode"]
    #[inline(always)]
    pub fn fhstmode(&mut self) -> FHSTMODE_W<'_, GUSBCFG_SPEC> {
        FHSTMODE_W::new(self, 29)
    }
    #[doc = "Bit 30 - Force device mode"]
    #[inline(always)]
    pub fn fdevmode(&mut self) -> FDEVMODE_W<'_, GUSBCFG_SPEC> {
        FDEVMODE_W::new(self, 30)
    }
    #[doc = "Bit 31 - Corrupt Tx packet"]
    #[inline(always)]
    pub fn cotxpkt(&mut self) -> COTXPKT_W<'_, GUSBCFG_SPEC> {
        COTXPKT_W::new(self, 31)
    }
}
#[doc = "USB configuration register (OTGHS_GUSBCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`gusbcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gusbcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GUSBCFG_SPEC;
impl crate::RegisterSpec for GUSBCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gusbcfg::R`](R) reader structure"]
impl crate::Readable for GUSBCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gusbcfg::W`](W) writer structure"]
impl crate::Writable for GUSBCFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GUSBCFG to value 0x0a00"]
impl crate::Resettable for GUSBCFG_SPEC {
    const RESET_VALUE: u32 = 0x0a00;
}
