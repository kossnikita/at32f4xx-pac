#[doc = "Register `MACMIIDT` reader"]
pub type R = crate::R<MACMIIDT_SPEC>;
#[doc = "Register `MACMIIDT` writer"]
pub type W = crate::W<MACMIIDT_SPEC>;
#[doc = "Field `MD` reader - MII data"]
pub type MD_R = crate::FieldReader<u16>;
#[doc = "Field `MD` writer - MII data"]
pub type MD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - MII data"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACMIIDT").field("md", &self.md()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - MII data"]
    #[inline(always)]
    pub fn md(&mut self) -> MD_W<'_, MACMIIDT_SPEC> {
        MD_W::new(self, 0)
    }
}
#[doc = "Ethernet MAC MII data register\n\nYou can [`read`](crate::Reg::read) this register and get [`macmiidt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macmiidt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACMIIDT_SPEC;
impl crate::RegisterSpec for MACMIIDT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macmiidt::R`](R) reader structure"]
impl crate::Readable for MACMIIDT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macmiidt::W`](W) writer structure"]
impl crate::Writable for MACMIIDT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACMIIDT to value 0"]
impl crate::Resettable for MACMIIDT_SPEC {}
