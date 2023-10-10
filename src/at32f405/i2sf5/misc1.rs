#[doc = "Register `MISC1` reader"]
pub type R = crate::R<MISC1_SPEC>;
#[doc = "Register `MISC1` writer"]
pub type W = crate::W<MISC1_SPEC>;
#[doc = "Field `I2SFPCMCKSEL` reader - I2S PCM clock edge select"]
pub type I2SFPCMCKSEL_R = crate::BitReader;
#[doc = "Field `I2SFPCMCKSEL` writer - I2S PCM clock edge select"]
pub type I2SFPCMCKSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - I2S PCM clock edge select"]
    #[inline(always)]
    pub fn i2sfpcmcksel(&self) -> I2SFPCMCKSEL_R {
        I2SFPCMCKSEL_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC1")
            .field(
                "i2sfpcmcksel",
                &format_args!("{}", self.i2sfpcmcksel().bit()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<MISC1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - I2S PCM clock edge select"]
    #[inline(always)]
    #[must_use]
    pub fn i2sfpcmcksel(&mut self) -> I2SFPCMCKSEL_W<MISC1_SPEC, 0> {
        I2SFPCMCKSEL_W::new(self)
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
#[doc = "MISC1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISC1_SPEC;
impl crate::RegisterSpec for MISC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc1::R`](R) reader structure"]
impl crate::Readable for MISC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`misc1::W`](W) writer structure"]
impl crate::Writable for MISC1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MISC1 to value 0"]
impl crate::Resettable for MISC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
