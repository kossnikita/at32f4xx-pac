#[doc = "Register `AHBLPEN3` reader"]
pub type R = crate::R<AHBLPEN3_SPEC>;
#[doc = "Register `AHBLPEN3` writer"]
pub type W = crate::W<AHBLPEN3_SPEC>;
#[doc = "Field `QSPI1LP` reader - QSPI1 clock enable during sleep mode"]
pub type QSPI1LP_R = crate::BitReader;
#[doc = "Field `QSPI1LP` writer - QSPI1 clock enable during sleep mode"]
pub type QSPI1LP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - QSPI1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn qspi1lp(&self) -> QSPI1LP_R {
        QSPI1LP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBLPEN3")
            .field("qspi1lp", &format_args!("{}", self.qspi1lp().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<AHBLPEN3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 1 - QSPI1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn qspi1lp(&mut self) -> QSPI1LP_W<AHBLPEN3_SPEC, 1> {
        QSPI1LP_W::new(self)
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
#[doc = "AHB peripheral Low-power clock enable register 3 (CRM_AHBLPEN3)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblpen3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblpen3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBLPEN3_SPEC;
impl crate::RegisterSpec for AHBLPEN3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahblpen3::R`](R) reader structure"]
impl crate::Readable for AHBLPEN3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahblpen3::W`](W) writer structure"]
impl crate::Writable for AHBLPEN3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBLPEN3 to value 0xc003"]
impl crate::Resettable for AHBLPEN3_SPEC {
    const RESET_VALUE: Self::Ux = 0xc003;
}
