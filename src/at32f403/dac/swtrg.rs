#[doc = "Register `SWTRG` reader"]
pub type R = crate::R<SWTRG_SPEC>;
#[doc = "Register `SWTRG` writer"]
pub type W = crate::W<SWTRG_SPEC>;
#[doc = "Field `D1SWTRG` reader - DAC1 software trigger"]
pub type D1SWTRG_R = crate::BitReader;
#[doc = "Field `D1SWTRG` writer - DAC1 software trigger"]
pub type D1SWTRG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D2SWTRG` reader - DAC2 software trigger"]
pub type D2SWTRG_R = crate::BitReader;
#[doc = "Field `D2SWTRG` writer - DAC2 software trigger"]
pub type D2SWTRG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DAC1 software trigger"]
    #[inline(always)]
    pub fn d1swtrg(&self) -> D1SWTRG_R {
        D1SWTRG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC2 software trigger"]
    #[inline(always)]
    pub fn d2swtrg(&self) -> D2SWTRG_R {
        D2SWTRG_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWTRG")
            .field("d1swtrg", &format_args!("{}", self.d1swtrg().bit()))
            .field("d2swtrg", &format_args!("{}", self.d2swtrg().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<SWTRG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - DAC1 software trigger"]
    #[inline(always)]
    #[must_use]
    pub fn d1swtrg(&mut self) -> D1SWTRG_W<SWTRG_SPEC> {
        D1SWTRG_W::new(self, 0)
    }
    #[doc = "Bit 1 - DAC2 software trigger"]
    #[inline(always)]
    #[must_use]
    pub fn d2swtrg(&mut self) -> D2SWTRG_W<SWTRG_SPEC> {
        D2SWTRG_W::new(self, 1)
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
#[doc = "DAC software trigger register(DAC_SWTRIGR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swtrg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swtrg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWTRG_SPEC;
impl crate::RegisterSpec for SWTRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swtrg::R`](R) reader structure"]
impl crate::Readable for SWTRG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swtrg::W`](W) writer structure"]
impl crate::Writable for SWTRG_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWTRG to value 0"]
impl crate::Resettable for SWTRG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
