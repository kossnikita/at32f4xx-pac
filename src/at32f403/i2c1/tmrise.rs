#[doc = "Register `TMRISE` reader"]
pub type R = crate::R<TMRISE_SPEC>;
#[doc = "Register `TMRISE` writer"]
pub type W = crate::W<TMRISE_SPEC>;
#[doc = "Field `RISETIME` reader - I2C bus rise time"]
pub type RISETIME_R = crate::FieldReader;
#[doc = "Field `RISETIME` writer - I2C bus rise time"]
pub type RISETIME_W<'a, REG> = crate::FieldWriter<'a, REG, 6, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:5 - I2C bus rise time"]
    #[inline(always)]
    pub fn risetime(&self) -> RISETIME_R {
        RISETIME_R::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMRISE")
            .field("risetime", &self.risetime())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - I2C bus rise time"]
    #[inline(always)]
    #[must_use]
    pub fn risetime(&mut self) -> RISETIME_W<TMRISE_SPEC> {
        RISETIME_W::new(self, 0)
    }
}
#[doc = "TRISE register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmrise::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmrise::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMRISE_SPEC;
impl crate::RegisterSpec for TMRISE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmrise::R`](R) reader structure"]
impl crate::Readable for TMRISE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmrise::W`](W) writer structure"]
impl crate::Writable for TMRISE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMRISE to value 0x02"]
impl crate::Resettable for TMRISE_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
