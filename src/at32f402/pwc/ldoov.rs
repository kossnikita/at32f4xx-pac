#[doc = "Register `LDOOV` reader"]
pub type R = crate::R<LDOOV_SPEC>;
#[doc = "Register `LDOOV` writer"]
pub type W = crate::W<LDOOV_SPEC>;
#[doc = "Field `LDOOVSEL` reader - LDO output voltage select"]
pub type LDOOVSEL_R = crate::FieldReader;
#[doc = "Field `LDOOVSEL` writer - LDO output voltage select"]
pub type LDOOVSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `VREXLPEN` reader - Voltage regulator extra low power mode enable"]
pub type VREXLPEN_R = crate::BitReader;
#[doc = "Field `VREXLPEN` writer - Voltage regulator extra low power mode enable"]
pub type VREXLPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - LDO output voltage select"]
    #[inline(always)]
    pub fn ldoovsel(&self) -> LDOOVSEL_R {
        LDOOVSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Voltage regulator extra low power mode enable"]
    #[inline(always)]
    pub fn vrexlpen(&self) -> VREXLPEN_R {
        VREXLPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LDOOV")
            .field("ldoovsel", &format_args!("{}", self.ldoovsel().bits()))
            .field("vrexlpen", &format_args!("{}", self.vrexlpen().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<LDOOV_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - LDO output voltage select"]
    #[inline(always)]
    #[must_use]
    pub fn ldoovsel(&mut self) -> LDOOVSEL_W<LDOOV_SPEC, 0> {
        LDOOVSEL_W::new(self)
    }
    #[doc = "Bit 4 - Voltage regulator extra low power mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn vrexlpen(&mut self) -> VREXLPEN_W<LDOOV_SPEC, 4> {
        VREXLPEN_W::new(self)
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
#[doc = "LDO output voltage register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ldoov::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ldoov::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LDOOV_SPEC;
impl crate::RegisterSpec for LDOOV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ldoov::R`](R) reader structure"]
impl crate::Readable for LDOOV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ldoov::W`](W) writer structure"]
impl crate::Writable for LDOOV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LDOOV to value 0x12"]
impl crate::Resettable for LDOOV_SPEC {
    const RESET_VALUE: Self::Ux = 0x12;
}
