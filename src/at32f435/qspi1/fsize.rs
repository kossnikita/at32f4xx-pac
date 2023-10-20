#[doc = "Register `FSIZE` reader"]
pub type R = crate::R<FSIZE_SPEC>;
#[doc = "Register `FSIZE` writer"]
pub type W = crate::W<FSIZE_SPEC>;
#[doc = "Field `SPIFSIZE` reader - SPI flash size"]
pub type SPIFSIZE_R = crate::FieldReader<u32>;
#[doc = "Field `SPIFSIZE` writer - SPI flash size"]
pub type SPIFSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SPI flash size"]
    #[inline(always)]
    pub fn spifsize(&self) -> SPIFSIZE_R {
        SPIFSIZE_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSIZE")
            .field("spifsize", &format_args!("{}", self.spifsize().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<FSIZE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - SPI flash size"]
    #[inline(always)]
    #[must_use]
    pub fn spifsize(&mut self) -> SPIFSIZE_W<FSIZE_SPEC> {
        SPIFSIZE_W::new(self, 0)
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
#[doc = "SPI flash size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsize::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsize::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FSIZE_SPEC;
impl crate::RegisterSpec for FSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsize::R`](R) reader structure"]
impl crate::Readable for FSIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fsize::W`](W) writer structure"]
impl crate::Writable for FSIZE_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSIZE to value 0"]
impl crate::Resettable for FSIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
