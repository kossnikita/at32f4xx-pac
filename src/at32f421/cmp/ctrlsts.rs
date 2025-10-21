#[doc = "Register `CTRLSTS` reader"]
pub type R = crate::R<CTRLSTS_SPEC>;
#[doc = "Register `CTRLSTS` writer"]
pub type W = crate::W<CTRLSTS_SPEC>;
#[doc = "Field `CMPEN` reader - Comparator enable bit"]
pub type CMPEN_R = crate::BitReader;
#[doc = "Field `CMPEN` writer - Comparator enable bit"]
pub type CMPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPIS` reader - Comparator input shift"]
pub type CMPIS_R = crate::BitReader;
#[doc = "Field `CMPIS` writer - Comparator input shift"]
pub type CMPIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPSSEL` reader - Comparator speed selection"]
pub type CMPSSEL_R = crate::FieldReader;
#[doc = "Field `CMPSSEL` writer - Comparator speed selection"]
pub type CMPSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMPINVSEL` reader - Comparator inverting selection"]
pub type CMPINVSEL_R = crate::FieldReader;
#[doc = "Field `CMPINVSEL` writer - Comparator inverting selection"]
pub type CMPINVSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CMPNINVSEL` reader - Comparator non-inverting input selection"]
pub type CMPNINVSEL_R = crate::FieldReader;
#[doc = "Field `CMPNINVSEL` writer - Comparator non-inverting input selection"]
pub type CMPNINVSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMPTAG` reader - Comparator output target"]
pub type CMPTAG_R = crate::FieldReader;
#[doc = "Field `CMPTAG` writer - Comparator output target"]
pub type CMPTAG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CMPP` reader - Comparator polarity"]
pub type CMPP_R = crate::BitReader;
#[doc = "Field `CMPP` writer - Comparator polarity"]
pub type CMPP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPHYST` reader - Comparator hysteresis"]
pub type CMPHYST_R = crate::FieldReader;
#[doc = "Field `CMPHYST` writer - Comparator hysteresis"]
pub type CMPHYST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMPBLANKING` reader - Comparator blanking"]
pub type CMPBLANKING_R = crate::FieldReader;
#[doc = "Field `CMPBLANKING` writer - Comparator blanking"]
pub type CMPBLANKING_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BRGEN` reader - Comparator brgen"]
pub type BRGEN_R = crate::BitReader;
#[doc = "Field `BRGEN` writer - Comparator brgen"]
pub type BRGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCALEN` reader - Comparator scalen"]
pub type SCALEN_R = crate::BitReader;
#[doc = "Field `SCALEN` writer - Comparator scalen"]
pub type SCALEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPVALUE` reader - Comparator output value"]
pub type CMPVALUE_R = crate::BitReader;
#[doc = "Field `CMPWP` reader - Comparator write protect"]
pub type CMPWP_R = crate::BitReader;
#[doc = "Field `CMPWP` writer - Comparator write protect"]
pub type CMPWP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator enable bit"]
    #[inline(always)]
    pub fn cmpen(&self) -> CMPEN_R {
        CMPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator input shift"]
    #[inline(always)]
    pub fn cmpis(&self) -> CMPIS_R {
        CMPIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator speed selection"]
    #[inline(always)]
    pub fn cmpssel(&self) -> CMPSSEL_R {
        CMPSSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator inverting selection"]
    #[inline(always)]
    pub fn cmpinvsel(&self) -> CMPINVSEL_R {
        CMPINVSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:8 - Comparator non-inverting input selection"]
    #[inline(always)]
    pub fn cmpninvsel(&self) -> CMPNINVSEL_R {
        CMPNINVSEL_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 10:12 - Comparator output target"]
    #[inline(always)]
    pub fn cmptag(&self) -> CMPTAG_R {
        CMPTAG_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 15 - Comparator polarity"]
    #[inline(always)]
    pub fn cmpp(&self) -> CMPP_R {
        CMPP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator hysteresis"]
    #[inline(always)]
    pub fn cmphyst(&self) -> CMPHYST_R {
        CMPHYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Comparator blanking"]
    #[inline(always)]
    pub fn cmpblanking(&self) -> CMPBLANKING_R {
        CMPBLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 22 - Comparator brgen"]
    #[inline(always)]
    pub fn brgen(&self) -> BRGEN_R {
        BRGEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Comparator scalen"]
    #[inline(always)]
    pub fn scalen(&self) -> SCALEN_R {
        SCALEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 30 - Comparator output value"]
    #[inline(always)]
    pub fn cmpvalue(&self) -> CMPVALUE_R {
        CMPVALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator write protect"]
    #[inline(always)]
    pub fn cmpwp(&self) -> CMPWP_R {
        CMPWP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRLSTS")
            .field("cmpen", &self.cmpen())
            .field("cmpis", &self.cmpis())
            .field("cmpssel", &self.cmpssel())
            .field("cmpinvsel", &self.cmpinvsel())
            .field("cmpninvsel", &self.cmpninvsel())
            .field("cmptag", &self.cmptag())
            .field("cmpp", &self.cmpp())
            .field("cmphyst", &self.cmphyst())
            .field("cmpblanking", &self.cmpblanking())
            .field("brgen", &self.brgen())
            .field("scalen", &self.scalen())
            .field("cmpvalue", &self.cmpvalue())
            .field("cmpwp", &self.cmpwp())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Comparator enable bit"]
    #[inline(always)]
    pub fn cmpen(&mut self) -> CMPEN_W<'_, CTRLSTS_SPEC> {
        CMPEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator input shift"]
    #[inline(always)]
    pub fn cmpis(&mut self) -> CMPIS_W<'_, CTRLSTS_SPEC> {
        CMPIS_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Comparator speed selection"]
    #[inline(always)]
    pub fn cmpssel(&mut self) -> CMPSSEL_W<'_, CTRLSTS_SPEC> {
        CMPSSEL_W::new(self, 2)
    }
    #[doc = "Bits 4:6 - Comparator inverting selection"]
    #[inline(always)]
    pub fn cmpinvsel(&mut self) -> CMPINVSEL_W<'_, CTRLSTS_SPEC> {
        CMPINVSEL_W::new(self, 4)
    }
    #[doc = "Bits 7:8 - Comparator non-inverting input selection"]
    #[inline(always)]
    pub fn cmpninvsel(&mut self) -> CMPNINVSEL_W<'_, CTRLSTS_SPEC> {
        CMPNINVSEL_W::new(self, 7)
    }
    #[doc = "Bits 10:12 - Comparator output target"]
    #[inline(always)]
    pub fn cmptag(&mut self) -> CMPTAG_W<'_, CTRLSTS_SPEC> {
        CMPTAG_W::new(self, 10)
    }
    #[doc = "Bit 15 - Comparator polarity"]
    #[inline(always)]
    pub fn cmpp(&mut self) -> CMPP_W<'_, CTRLSTS_SPEC> {
        CMPP_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - Comparator hysteresis"]
    #[inline(always)]
    pub fn cmphyst(&mut self) -> CMPHYST_W<'_, CTRLSTS_SPEC> {
        CMPHYST_W::new(self, 16)
    }
    #[doc = "Bits 18:20 - Comparator blanking"]
    #[inline(always)]
    pub fn cmpblanking(&mut self) -> CMPBLANKING_W<'_, CTRLSTS_SPEC> {
        CMPBLANKING_W::new(self, 18)
    }
    #[doc = "Bit 22 - Comparator brgen"]
    #[inline(always)]
    pub fn brgen(&mut self) -> BRGEN_W<'_, CTRLSTS_SPEC> {
        BRGEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - Comparator scalen"]
    #[inline(always)]
    pub fn scalen(&mut self) -> SCALEN_W<'_, CTRLSTS_SPEC> {
        SCALEN_W::new(self, 23)
    }
    #[doc = "Bit 31 - Comparator write protect"]
    #[inline(always)]
    pub fn cmpwp(&mut self) -> CMPWP_W<'_, CTRLSTS_SPEC> {
        CMPWP_W::new(self, 31)
    }
}
#[doc = "CMP control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLSTS_SPEC;
impl crate::RegisterSpec for CTRLSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlsts::R`](R) reader structure"]
impl crate::Readable for CTRLSTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlsts::W`](W) writer structure"]
impl crate::Writable for CTRLSTS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLSTS to value 0"]
impl crate::Resettable for CTRLSTS_SPEC {}
