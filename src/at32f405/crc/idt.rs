#[doc = "Register `IDT` reader"]
pub type R = crate::R<IDT_SPEC>;
#[doc = "Register `IDT` writer"]
pub type W = crate::W<IDT_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<IDT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Initial data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDT_SPEC;
impl crate::RegisterSpec for IDT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idt::R`](R) reader structure"]
impl crate::Readable for IDT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`idt::W`](W) writer structure"]
impl crate::Writable for IDT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDT to value 0xffff_ffff"]
impl crate::Resettable for IDT_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
