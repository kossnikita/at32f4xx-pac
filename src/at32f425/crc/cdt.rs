#[doc = "Register `CDT` reader"]
pub type R = crate::R<CDT_SPEC>;
#[doc = "Register `CDT` writer"]
pub type W = crate::W<CDT_SPEC>;
#[doc = "Field `CDT` reader - Common Data"]
pub type CDT_R = crate::FieldReader;
#[doc = "Field `CDT` writer - Common Data"]
pub type CDT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Common Data"]
    #[inline(always)]
    pub fn cdt(&self) -> CDT_R {
        CDT_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CDT")
            .field("cdt", &format_args!("{}", self.cdt().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CDT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Common Data"]
    #[inline(always)]
    #[must_use]
    pub fn cdt(&mut self) -> CDT_W<CDT_SPEC> {
        CDT_W::new(self, 0)
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
#[doc = "Common data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CDT_SPEC;
impl crate::RegisterSpec for CDT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdt::R`](R) reader structure"]
impl crate::Readable for CDT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cdt::W`](W) writer structure"]
impl crate::Writable for CDT_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CDT to value 0"]
impl crate::Resettable for CDT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
