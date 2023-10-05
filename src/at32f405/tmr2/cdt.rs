#[doc = "Register `C%sDT` reader"]
pub type R = crate::R<CDT_SPEC>;
#[doc = "Register `C%sDT` writer"]
pub type W = crate::W<CDT_SPEC>;
#[doc = "Field `CDT` reader - Channel data value"]
pub type CDT_R = crate::FieldReader<u32>;
#[doc = "Field `CDT` writer - Channel data value"]
pub type CDT_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel data value"]
    #[inline(always)]
    pub fn cdt(&self) -> CDT_R {
        CDT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel data value"]
    #[inline(always)]
    #[must_use]
    pub fn cdt(&mut self) -> CDT_W<CDT_SPEC, 0> {
        CDT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CDT_SPEC;
impl crate::RegisterSpec for CDT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdt::R`](R) reader structure"]
impl crate::Readable for CDT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cdt::W`](W) writer structure"]
impl crate::Writable for CDT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C%sDT to value 0"]
impl crate::Resettable for CDT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
