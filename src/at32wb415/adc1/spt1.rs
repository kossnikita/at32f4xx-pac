#[doc = "Register `SPT1` reader"]
pub type R = crate::R<SPT1_SPEC>;
#[doc = "Register `SPT1` writer"]
pub type W = crate::W<SPT1_SPEC>;
#[doc = "Field `CSPT10` reader - Selection sample time of channel ADC_IN10"]
pub type CSPT10_R = crate::FieldReader;
#[doc = "Field `CSPT10` writer - Selection sample time of channel ADC_IN10"]
pub type CSPT10_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O>;
#[doc = "Field `CSPT11` reader - Selection sample time of channel ADC_IN11"]
pub type CSPT11_R = crate::FieldReader;
#[doc = "Field `CSPT11` writer - Selection sample time of channel ADC_IN11"]
pub type CSPT11_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O>;
#[doc = "Field `CSPT16` reader - Selection sample time of channel ADC_IN16"]
pub type CSPT16_R = crate::FieldReader;
#[doc = "Field `CSPT16` writer - Selection sample time of channel ADC_IN16"]
pub type CSPT16_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O>;
#[doc = "Field `CSPT17` reader - Selection sample time of channel ADC_IN17"]
pub type CSPT17_R = crate::FieldReader;
#[doc = "Field `CSPT17` writer - Selection sample time of channel ADC_IN17"]
pub type CSPT17_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Selection sample time of channel ADC_IN10"]
    #[inline(always)]
    pub fn cspt10(&self) -> CSPT10_R {
        CSPT10_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Selection sample time of channel ADC_IN11"]
    #[inline(always)]
    pub fn cspt11(&self) -> CSPT11_R {
        CSPT11_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Selection sample time of channel ADC_IN16"]
    #[inline(always)]
    pub fn cspt16(&self) -> CSPT16_R {
        CSPT16_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Selection sample time of channel ADC_IN17"]
    #[inline(always)]
    pub fn cspt17(&self) -> CSPT17_R {
        CSPT17_R::new(((self.bits >> 21) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPT1")
            .field("cspt17", &format_args!("{}", self.cspt17().bits()))
            .field("cspt16", &format_args!("{}", self.cspt16().bits()))
            .field("cspt11", &format_args!("{}", self.cspt11().bits()))
            .field("cspt10", &format_args!("{}", self.cspt10().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<SPT1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selection sample time of channel ADC_IN10"]
    #[inline(always)]
    #[must_use]
    pub fn cspt10(&mut self) -> CSPT10_W<SPT1_SPEC, 0> {
        CSPT10_W::new(self)
    }
    #[doc = "Bits 3:5 - Selection sample time of channel ADC_IN11"]
    #[inline(always)]
    #[must_use]
    pub fn cspt11(&mut self) -> CSPT11_W<SPT1_SPEC, 3> {
        CSPT11_W::new(self)
    }
    #[doc = "Bits 18:20 - Selection sample time of channel ADC_IN16"]
    #[inline(always)]
    #[must_use]
    pub fn cspt16(&mut self) -> CSPT16_W<SPT1_SPEC, 18> {
        CSPT16_W::new(self)
    }
    #[doc = "Bits 21:23 - Selection sample time of channel ADC_IN17"]
    #[inline(always)]
    #[must_use]
    pub fn cspt17(&mut self) -> CSPT17_W<SPT1_SPEC, 21> {
        CSPT17_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "sample time register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spt1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spt1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPT1_SPEC;
impl crate::RegisterSpec for SPT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spt1::R`](R) reader structure"]
impl crate::Readable for SPT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spt1::W`](W) writer structure"]
impl crate::Writable for SPT1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPT1 to value 0"]
impl crate::Resettable for SPT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
