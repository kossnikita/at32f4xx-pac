#[doc = "Register `CPOLY` reader"]
pub type R = crate::R<CPOLY_SPEC>;
#[doc = "Register `CPOLY` writer"]
pub type W = crate::W<CPOLY_SPEC>;
#[doc = "Field `CPOLY` reader - CRC polynomial"]
pub type CPOLY_R = crate::FieldReader<u16>;
#[doc = "Field `CPOLY` writer - CRC polynomial"]
pub type CPOLY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CRC polynomial"]
    #[inline(always)]
    pub fn cpoly(&self) -> CPOLY_R {
        CPOLY_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPOLY")
            .field("cpoly", &format_args!("{}", self.cpoly().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CPOLY_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC polynomial"]
    #[inline(always)]
    #[must_use]
    pub fn cpoly(&mut self) -> CPOLY_W<CPOLY_SPEC> {
        CPOLY_W::new(self, 0)
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
#[doc = "CRC polynomial register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpoly::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpoly::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPOLY_SPEC;
impl crate::RegisterSpec for CPOLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpoly::R`](R) reader structure"]
impl crate::Readable for CPOLY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpoly::W`](W) writer structure"]
impl crate::Writable for CPOLY_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPOLY to value 0x07"]
impl crate::Resettable for CPOLY_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
