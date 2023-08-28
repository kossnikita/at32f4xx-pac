#[doc = "Register `MACMIIDT` reader"]
pub type R = crate::R<MACMIIDT_SPEC>;
#[doc = "Register `MACMIIDT` writer"]
pub type W = crate::W<MACMIIDT_SPEC>;
#[doc = "Field `MD` reader - MII data"]
pub type MD_R = crate::FieldReader<u16>;
#[doc = "Field `MD` writer - MII data"]
pub type MD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - MII data"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MII data"]
    #[inline(always)]
    #[must_use]
    pub fn md(&mut self) -> MD_W<MACMIIDT_SPEC, 0> {
        MD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ethernet MAC MII data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macmiidt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macmiidt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACMIIDT_SPEC;
impl crate::RegisterSpec for MACMIIDT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macmiidt::R`](R) reader structure"]
impl crate::Readable for MACMIIDT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macmiidt::W`](W) writer structure"]
impl crate::Writable for MACMIIDT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACMIIDT to value 0"]
impl crate::Resettable for MACMIIDT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}