#[doc = "Register `CPOLY` reader"]
pub type R = crate::R<CPOLY_SPEC>;
#[doc = "Register `CPOLY` writer"]
pub type W = crate::W<CPOLY_SPEC>;
#[doc = "Field `CPOLY` reader - CRC polynomial"]
pub type CPOLY_R = crate::FieldReader<u16>;
#[doc = "Field `CPOLY` writer - CRC polynomial"]
pub type CPOLY_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - CRC polynomial"]
    #[inline(always)]
    pub fn cpoly(&self) -> CPOLY_R {
        CPOLY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC polynomial"]
    #[inline(always)]
    #[must_use]
    pub fn cpoly(&mut self) -> CPOLY_W<CPOLY_SPEC, 0> {
        CPOLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPOLY to value 0x07"]
impl crate::Resettable for CPOLY_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
