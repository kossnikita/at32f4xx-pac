#[doc = "Register `ESTS` reader"]
pub type R = crate::R<ESTS_SPEC>;
#[doc = "Register `ESTS` writer"]
pub type W = crate::W<ESTS_SPEC>;
#[doc = "Field `EAF` reader - Error active flag"]
pub type EAF_R = crate::BitReader;
#[doc = "Field `EPF` reader - Error passive flag"]
pub type EPF_R = crate::BitReader;
#[doc = "Field `BOF` reader - Bus-off flag"]
pub type BOF_R = crate::BitReader;
#[doc = "Field `ETR` reader - Error type record"]
pub type ETR_R = crate::FieldReader;
#[doc = "Field `ETR` writer - Error type record"]
pub type ETR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `TEC` reader - Transmit error counter"]
pub type TEC_R = crate::FieldReader;
#[doc = "Field `REC` reader - Receive error counter"]
pub type REC_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Error active flag"]
    #[inline(always)]
    pub fn eaf(&self) -> EAF_R {
        EAF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error passive flag"]
    #[inline(always)]
    pub fn epf(&self) -> EPF_R {
        EPF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bus-off flag"]
    #[inline(always)]
    pub fn bof(&self) -> BOF_R {
        BOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Error type record"]
    #[inline(always)]
    pub fn etr(&self) -> ETR_R {
        ETR_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 16:23 - Transmit error counter"]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Receive error counter"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Error type record"]
    #[inline(always)]
    #[must_use]
    pub fn etr(&mut self) -> ETR_W<ESTS_SPEC, 4> {
        ETR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ests::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ests::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ESTS_SPEC;
impl crate::RegisterSpec for ESTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ests::R`](R) reader structure"]
impl crate::Readable for ESTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ests::W`](W) writer structure"]
impl crate::Writable for ESTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ESTS to value 0"]
impl crate::Resettable for ESTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
