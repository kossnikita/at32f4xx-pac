#[doc = "Register `CDT` reader"]
pub type R = crate::R<CDT_SPEC>;
#[doc = "Register `CDT` writer"]
pub type W = crate::W<CDT_SPEC>;
#[doc = "Field `CDT` reader - Common Data"]
pub type CDT_R = crate::BitReader;
#[doc = "Field `CDT` writer - Common Data"]
pub type CDT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Common Data"]
    #[inline(always)]
    pub fn cdt(&self) -> CDT_R {
        CDT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Common Data"]
    #[inline(always)]
    #[must_use]
    pub fn cdt(&mut self) -> CDT_W<CDT_SPEC, 0> {
        CDT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Common data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CDT to value 0"]
impl crate::Resettable for CDT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
