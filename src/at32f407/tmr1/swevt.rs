#[doc = "Register `SWEVT` reader"]
pub type R = crate::R<SWEVT_SPEC>;
#[doc = "Register `SWEVT` writer"]
pub type W = crate::W<SWEVT_SPEC>;
#[doc = "Field `OVFSWTR` reader - Overflow event triggered by software"]
pub type OVFSWTR_R = crate::BitReader;
#[doc = "Field `OVFSWTR` writer - Overflow event triggered by software"]
pub type OVFSWTR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C1SWTR` reader - Channel 1 event triggered by software"]
pub type C1SWTR_R = crate::BitReader;
#[doc = "Field `C1SWTR` writer - Channel 1 event triggered by software"]
pub type C1SWTR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C2SWTR` reader - Channel 2 event triggered by software"]
pub type C2SWTR_R = crate::BitReader;
#[doc = "Field `C2SWTR` writer - Channel 2 event triggered by software"]
pub type C2SWTR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C3SWTR` reader - Channel 3 event triggered by software"]
pub type C3SWTR_R = crate::BitReader;
#[doc = "Field `C3SWTR` writer - Channel 3 event triggered by software"]
pub type C3SWTR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C4SWTR` reader - Channel 4 event triggered by software"]
pub type C4SWTR_R = crate::BitReader;
#[doc = "Field `C4SWTR` writer - Channel 4 event triggered by software"]
pub type C4SWTR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HALLSWTR` reader - HALL event triggered by software"]
pub type HALLSWTR_R = crate::BitReader;
#[doc = "Field `HALLSWTR` writer - HALL event triggered by software"]
pub type HALLSWTR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRGSWTR` reader - Trigger event triggered by software"]
pub type TRGSWTR_R = crate::BitReader;
#[doc = "Field `TRGSWTR` writer - Trigger event triggered by software"]
pub type TRGSWTR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BRKSWTR` reader - Brake event triggered by software"]
pub type BRKSWTR_R = crate::BitReader;
#[doc = "Field `BRKSWTR` writer - Brake event triggered by software"]
pub type BRKSWTR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Overflow event triggered by software"]
    #[inline(always)]
    pub fn ovfswtr(&self) -> OVFSWTR_R {
        OVFSWTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 event triggered by software"]
    #[inline(always)]
    pub fn c1swtr(&self) -> C1SWTR_R {
        C1SWTR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 event triggered by software"]
    #[inline(always)]
    pub fn c2swtr(&self) -> C2SWTR_R {
        C2SWTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 event triggered by software"]
    #[inline(always)]
    pub fn c3swtr(&self) -> C3SWTR_R {
        C3SWTR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 event triggered by software"]
    #[inline(always)]
    pub fn c4swtr(&self) -> C4SWTR_R {
        C4SWTR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HALL event triggered by software"]
    #[inline(always)]
    pub fn hallswtr(&self) -> HALLSWTR_R {
        HALLSWTR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger event triggered by software"]
    #[inline(always)]
    pub fn trgswtr(&self) -> TRGSWTR_R {
        TRGSWTR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Brake event triggered by software"]
    #[inline(always)]
    pub fn brkswtr(&self) -> BRKSWTR_R {
        BRKSWTR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn ovfswtr(&mut self) -> OVFSWTR_W<SWEVT_SPEC, 0> {
        OVFSWTR_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn c1swtr(&mut self) -> C1SWTR_W<SWEVT_SPEC, 1> {
        C1SWTR_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn c2swtr(&mut self) -> C2SWTR_W<SWEVT_SPEC, 2> {
        C2SWTR_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn c3swtr(&mut self) -> C3SWTR_W<SWEVT_SPEC, 3> {
        C3SWTR_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn c4swtr(&mut self) -> C4SWTR_W<SWEVT_SPEC, 4> {
        C4SWTR_W::new(self)
    }
    #[doc = "Bit 5 - HALL event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn hallswtr(&mut self) -> HALLSWTR_W<SWEVT_SPEC, 5> {
        HALLSWTR_W::new(self)
    }
    #[doc = "Bit 6 - Trigger event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn trgswtr(&mut self) -> TRGSWTR_W<SWEVT_SPEC, 6> {
        TRGSWTR_W::new(self)
    }
    #[doc = "Bit 7 - Brake event triggered by software"]
    #[inline(always)]
    #[must_use]
    pub fn brkswtr(&mut self) -> BRKSWTR_W<SWEVT_SPEC, 7> {
        BRKSWTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Software event register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swevt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swevt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWEVT_SPEC;
impl crate::RegisterSpec for SWEVT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swevt::R`](R) reader structure"]
impl crate::Readable for SWEVT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swevt::W`](W) writer structure"]
impl crate::Writable for SWEVT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWEVT to value 0"]
impl crate::Resettable for SWEVT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
