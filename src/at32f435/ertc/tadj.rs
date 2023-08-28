#[doc = "Register `TADJ` writer"]
pub type W = crate::W<TADJ_SPEC>;
#[doc = "Field `DECSBS` writer - Decrease sub-second value"]
pub type DECSBS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 15, O, u16>;
#[doc = "Field `ADD1S` writer - Add 1 second"]
pub type ADD1S_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bits 0:14 - Decrease sub-second value"]
    #[inline(always)]
    #[must_use]
    pub fn decsbs(&mut self) -> DECSBS_W<TADJ_SPEC, 0> {
        DECSBS_W::new(self)
    }
    #[doc = "Bit 31 - Add 1 second"]
    #[inline(always)]
    #[must_use]
    pub fn add1s(&mut self) -> ADD1S_W<TADJ_SPEC, 31> {
        ADD1S_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "time adjust register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tadj::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TADJ_SPEC;
impl crate::RegisterSpec for TADJ_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tadj::W`](W) writer structure"]
impl crate::Writable for TADJ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TADJ to value 0"]
impl crate::Resettable for TADJ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
