#[doc = "Register `BTM_MODE_SET` writer"]
pub type W = crate::W<BTM_MODE_SET_SPEC>;
#[doc = "Field `BTM_MODE_SET` writer - Boot memory mode setting"]
pub type BTM_MODE_SET_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl core::fmt::Debug for crate::generic::Reg<BTM_MODE_SET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - Boot memory mode setting"]
    #[inline(always)]
    #[must_use]
    pub fn btm_mode_set(&mut self) -> BTM_MODE_SET_W<BTM_MODE_SET_SPEC> {
        BTM_MODE_SET_W::new(self, 0)
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
#[doc = "Boot memory mode setting register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btm_mode_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BTM_MODE_SET_SPEC;
impl crate::RegisterSpec for BTM_MODE_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`btm_mode_set::W`](W) writer structure"]
impl crate::Writable for BTM_MODE_SET_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BTM_MODE_SET to value 0"]
impl crate::Resettable for BTM_MODE_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
