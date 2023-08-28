#[doc = "Register `IDT` reader"]
pub type R = crate::R<IDT_SPEC>;
#[doc = "Register `IDT` writer"]
pub type W = crate::W<IDT_SPEC>;
#[doc = "Field `IDT` reader - Initial Data"]
pub type IDT_R = crate::FieldReader<u32>;
#[doc = "Field `IDT` writer - Initial Data"]
pub type IDT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Initial Data"]
    #[inline(always)]
    pub fn idt(&self) -> IDT_R {
        IDT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Initial Data"]
    #[inline(always)]
    #[must_use]
    pub fn idt(&mut self) -> IDT_W<IDT_SPEC, 0> {
        IDT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
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
