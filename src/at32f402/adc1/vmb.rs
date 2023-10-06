#[doc = "Register `VM%sB` reader"]
pub type R = crate::R<VMB_SPEC>;
#[doc = "Register `VM%sB` writer"]
pub type W = crate::W<VMB_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<VMB_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Voltage monitoring %s boundary register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vmb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VMB_SPEC;
impl crate::RegisterSpec for VMB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`vmb::R`](R) reader structure"]
impl crate::Readable for VMB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vmb::W`](W) writer structure"]
impl crate::Writable for VMB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VM%sB to value 0x0fff"]
impl crate::Resettable for VMB_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff;
}
