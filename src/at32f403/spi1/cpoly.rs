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
            .field("cpoly", &self.cpoly())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC polynomial"]
    #[inline(always)]
    #[must_use]
    pub fn cpoly(&mut self) -> CPOLY_W<CPOLY_SPEC> {
        CPOLY_W::new(self, 0)
    }
}
#[doc = "CRC polynomial register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpoly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpoly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPOLY_SPEC;
impl crate::RegisterSpec for CPOLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpoly::R`](R) reader structure"]
impl crate::Readable for CPOLY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpoly::W`](W) writer structure"]
impl crate::Writable for CPOLY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPOLY to value 0x07"]
impl crate::Resettable for CPOLY_SPEC {
    const RESET_VALUE: u32 = 0x07;
}
