#[doc = "Register `MACISTS` reader"]
pub type R = crate::R<MACISTS_SPEC>;
#[doc = "Register `MACISTS` writer"]
pub type W = crate::W<MACISTS_SPEC>;
#[doc = "Field `PIS` reader - PMT interrupt status"]
pub type PIS_R = crate::BitReader;
#[doc = "Field `PIS` writer - PMT interrupt status"]
pub type PIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MIS` reader - MMC interrupt status"]
pub type MIS_R = crate::BitReader;
#[doc = "Field `MIS` writer - MMC interrupt status"]
pub type MIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MRIS` reader - MMC receive interrupt status"]
pub type MRIS_R = crate::BitReader;
#[doc = "Field `MRIS` writer - MMC receive interrupt status"]
pub type MRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MTIS` reader - MMC transmit interrupt status"]
pub type MTIS_R = crate::BitReader;
#[doc = "Field `MTIS` writer - MMC transmit interrupt status"]
pub type MTIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIS` reader - Timestamp interrupt status"]
pub type TIS_R = crate::BitReader;
#[doc = "Field `TIS` writer - Timestamp interrupt status"]
pub type TIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 3 - PMT interrupt status"]
    #[inline(always)]
    pub fn pis(&self) -> PIS_R {
        PIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC interrupt status"]
    #[inline(always)]
    pub fn mis(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC receive interrupt status"]
    #[inline(always)]
    pub fn mris(&self) -> MRIS_R {
        MRIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC transmit interrupt status"]
    #[inline(always)]
    pub fn mtis(&self) -> MTIS_R {
        MTIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp interrupt status"]
    #[inline(always)]
    pub fn tis(&self) -> TIS_R {
        TIS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - PMT interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn pis(&mut self) -> PIS_W<MACISTS_SPEC, 3> {
        PIS_W::new(self)
    }
    #[doc = "Bit 4 - MMC interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn mis(&mut self) -> MIS_W<MACISTS_SPEC, 4> {
        MIS_W::new(self)
    }
    #[doc = "Bit 5 - MMC receive interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn mris(&mut self) -> MRIS_W<MACISTS_SPEC, 5> {
        MRIS_W::new(self)
    }
    #[doc = "Bit 6 - MMC transmit interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn mtis(&mut self) -> MTIS_W<MACISTS_SPEC, 6> {
        MTIS_W::new(self)
    }
    #[doc = "Bit 9 - Timestamp interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn tis(&mut self) -> TIS_W<MACISTS_SPEC, 9> {
        TIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ethernet MAC interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macists::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macists::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACISTS_SPEC;
impl crate::RegisterSpec for MACISTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macists::R`](R) reader structure"]
impl crate::Readable for MACISTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macists::W`](W) writer structure"]
impl crate::Writable for MACISTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACISTS to value 0"]
impl crate::Resettable for MACISTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
