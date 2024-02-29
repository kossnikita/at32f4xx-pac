#[doc = "Register `TAMP` reader"]
pub type R = crate::R<TAMP_SPEC>;
#[doc = "Register `TAMP` writer"]
pub type W = crate::W<TAMP_SPEC>;
#[doc = "Field `TP1EN` reader - Tamper detection 1 enable"]
pub type TP1EN_R = crate::BitReader;
#[doc = "Field `TP1EN` writer - Tamper detection 1 enable"]
pub type TP1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TP1EDG` reader - Tamper detection 1 valid edge"]
pub type TP1EDG_R = crate::BitReader;
#[doc = "Field `TP1EDG` writer - Tamper detection 1 valid edge"]
pub type TP1EDG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPIEN` reader - Tamper detection interrupt enable"]
pub type TPIEN_R = crate::BitReader;
#[doc = "Field `TPIEN` writer - Tamper detection interrupt enable"]
pub type TPIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPTSEN` reader - Tamper detection timestamp enable"]
pub type TPTSEN_R = crate::BitReader;
#[doc = "Field `TPTSEN` writer - Tamper detection timestamp enable"]
pub type TPTSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPFREQ` reader - Tamper detection frequency"]
pub type TPFREQ_R = crate::FieldReader;
#[doc = "Field `TPFREQ` writer - Tamper detection frequency"]
pub type TPFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TPFLT` reader - Tamper detection filter time"]
pub type TPFLT_R = crate::FieldReader;
#[doc = "Field `TPFLT` writer - Tamper detection filter time"]
pub type TPFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TPPR` reader - Tamper detection pre-charge time"]
pub type TPPR_R = crate::FieldReader;
#[doc = "Field `TPPR` writer - Tamper detection pre-charge time"]
pub type TPPR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TPPU` reader - Tamper detection pull-up"]
pub type TPPU_R = crate::BitReader;
#[doc = "Field `TPPU` writer - Tamper detection pull-up"]
pub type TPPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTTYPE` reader - Output type"]
pub type OUTTYPE_R = crate::BitReader;
#[doc = "Field `OUTTYPE` writer - Output type"]
pub type OUTTYPE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Tamper detection 1 enable"]
    #[inline(always)]
    pub fn tp1en(&self) -> TP1EN_R {
        TP1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper detection 1 valid edge"]
    #[inline(always)]
    pub fn tp1edg(&self) -> TP1EDG_R {
        TP1EDG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper detection interrupt enable"]
    #[inline(always)]
    pub fn tpien(&self) -> TPIEN_R {
        TPIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Tamper detection timestamp enable"]
    #[inline(always)]
    pub fn tptsen(&self) -> TPTSEN_R {
        TPTSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Tamper detection frequency"]
    #[inline(always)]
    pub fn tpfreq(&self) -> TPFREQ_R {
        TPFREQ_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - Tamper detection filter time"]
    #[inline(always)]
    pub fn tpflt(&self) -> TPFLT_R {
        TPFLT_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - Tamper detection pre-charge time"]
    #[inline(always)]
    pub fn tppr(&self) -> TPPR_R {
        TPPR_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Tamper detection pull-up"]
    #[inline(always)]
    pub fn tppu(&self) -> TPPU_R {
        TPPU_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - Output type"]
    #[inline(always)]
    pub fn outtype(&self) -> OUTTYPE_R {
        OUTTYPE_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TAMP")
            .field("outtype", &format_args!("{}", self.outtype().bit()))
            .field("tppu", &format_args!("{}", self.tppu().bit()))
            .field("tppr", &format_args!("{}", self.tppr().bits()))
            .field("tpflt", &format_args!("{}", self.tpflt().bits()))
            .field("tpfreq", &format_args!("{}", self.tpfreq().bits()))
            .field("tptsen", &format_args!("{}", self.tptsen().bit()))
            .field("tpien", &format_args!("{}", self.tpien().bit()))
            .field("tp1edg", &format_args!("{}", self.tp1edg().bit()))
            .field("tp1en", &format_args!("{}", self.tp1en().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<TAMP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper detection 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn tp1en(&mut self) -> TP1EN_W<TAMP_SPEC> {
        TP1EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Tamper detection 1 valid edge"]
    #[inline(always)]
    #[must_use]
    pub fn tp1edg(&mut self) -> TP1EDG_W<TAMP_SPEC> {
        TP1EDG_W::new(self, 1)
    }
    #[doc = "Bit 2 - Tamper detection interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tpien(&mut self) -> TPIEN_W<TAMP_SPEC> {
        TPIEN_W::new(self, 2)
    }
    #[doc = "Bit 7 - Tamper detection timestamp enable"]
    #[inline(always)]
    #[must_use]
    pub fn tptsen(&mut self) -> TPTSEN_W<TAMP_SPEC> {
        TPTSEN_W::new(self, 7)
    }
    #[doc = "Bits 8:10 - Tamper detection frequency"]
    #[inline(always)]
    #[must_use]
    pub fn tpfreq(&mut self) -> TPFREQ_W<TAMP_SPEC> {
        TPFREQ_W::new(self, 8)
    }
    #[doc = "Bits 11:12 - Tamper detection filter time"]
    #[inline(always)]
    #[must_use]
    pub fn tpflt(&mut self) -> TPFLT_W<TAMP_SPEC> {
        TPFLT_W::new(self, 11)
    }
    #[doc = "Bits 13:14 - Tamper detection pre-charge time"]
    #[inline(always)]
    #[must_use]
    pub fn tppr(&mut self) -> TPPR_W<TAMP_SPEC> {
        TPPR_W::new(self, 13)
    }
    #[doc = "Bit 15 - Tamper detection pull-up"]
    #[inline(always)]
    #[must_use]
    pub fn tppu(&mut self) -> TPPU_W<TAMP_SPEC> {
        TPPU_W::new(self, 15)
    }
    #[doc = "Bit 18 - Output type"]
    #[inline(always)]
    #[must_use]
    pub fn outtype(&mut self) -> OUTTYPE_W<TAMP_SPEC> {
        OUTTYPE_W::new(self, 18)
    }
}
#[doc = "tamper and alternate function configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAMP_SPEC;
impl crate::RegisterSpec for TAMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamp::R`](R) reader structure"]
impl crate::Readable for TAMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tamp::W`](W) writer structure"]
impl crate::Writable for TAMP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAMP to value 0"]
impl crate::Resettable for TAMP_SPEC {
    const RESET_VALUE: u32 = 0;
}
