#[doc = "Register `SPT1` reader"]
pub type R = crate::R<SPT1_SPEC>;
#[doc = "Register `SPT1` writer"]
pub type W = crate::W<SPT1_SPEC>;
#[doc = "Field `CSPT[10-18]` reader - Selection sample time of channel ADC_IN%s"]
pub type CSPT_R = crate::FieldReader;
#[doc = "Field `CSPT[10-18]` writer - Selection sample time of channel ADC_IN%s"]
pub type CSPT_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O>;
impl R {
    #[doc = "Selection sample time of channel ADC_IN[10-18]"]
    #[inline(always)]
    pub unsafe fn cspt(&self, n: u8) -> CSPT_R {
        CSPT_R::new(((self.bits >> ((n - 10) * 3)) & 7) as u8)
    }
    #[doc = "Bits 0:2 - Selection sample time of channel ADC_IN10"]
    #[inline(always)]
    pub fn cspt10(&self) -> CSPT_R {
        CSPT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Selection sample time of channel ADC_IN11"]
    #[inline(always)]
    pub fn cspt11(&self) -> CSPT_R {
        CSPT_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Selection sample time of channel ADC_IN12"]
    #[inline(always)]
    pub fn cspt12(&self) -> CSPT_R {
        CSPT_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Selection sample time of channel ADC_IN13"]
    #[inline(always)]
    pub fn cspt13(&self) -> CSPT_R {
        CSPT_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Selection sample time of channel ADC_IN14"]
    #[inline(always)]
    pub fn cspt14(&self) -> CSPT_R {
        CSPT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Selection sample time of channel ADC_IN15"]
    #[inline(always)]
    pub fn cspt15(&self) -> CSPT_R {
        CSPT_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Selection sample time of channel ADC_IN16"]
    #[inline(always)]
    pub fn cspt16(&self) -> CSPT_R {
        CSPT_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Selection sample time of channel ADC_IN17"]
    #[inline(always)]
    pub fn cspt17(&self) -> CSPT_R {
        CSPT_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Selection sample time of channel ADC_IN18"]
    #[inline(always)]
    pub fn cspt18(&self) -> CSPT_R {
        CSPT_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPT1")
            .field("cspt10", &format_args!("{}", self.cspt10().bits()))
            .field("cspt11", &format_args!("{}", self.cspt11().bits()))
            .field("cspt12", &format_args!("{}", self.cspt12().bits()))
            .field("cspt13", &format_args!("{}", self.cspt13().bits()))
            .field("cspt14", &format_args!("{}", self.cspt14().bits()))
            .field("cspt15", &format_args!("{}", self.cspt15().bits()))
            .field("cspt16", &format_args!("{}", self.cspt16().bits()))
            .field("cspt17", &format_args!("{}", self.cspt17().bits()))
            .field("cspt18", &format_args!("{}", self.cspt18().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<SPT1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Selection sample time of channel ADC_IN[10-18]"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn cspt<const O: u8>(&mut self) -> CSPT_W<SPT1_SPEC, O> {
        CSPT_W::new(self)
    }
    #[doc = "Bits 0:2 - Selection sample time of channel ADC_IN10"]
    #[inline(always)]
    #[must_use]
    pub fn cspt10(&mut self) -> CSPT_W<SPT1_SPEC, 0> {
        CSPT_W::new(self)
    }
    #[doc = "Bits 3:5 - Selection sample time of channel ADC_IN11"]
    #[inline(always)]
    #[must_use]
    pub fn cspt11(&mut self) -> CSPT_W<SPT1_SPEC, 3> {
        CSPT_W::new(self)
    }
    #[doc = "Bits 6:8 - Selection sample time of channel ADC_IN12"]
    #[inline(always)]
    #[must_use]
    pub fn cspt12(&mut self) -> CSPT_W<SPT1_SPEC, 6> {
        CSPT_W::new(self)
    }
    #[doc = "Bits 9:11 - Selection sample time of channel ADC_IN13"]
    #[inline(always)]
    #[must_use]
    pub fn cspt13(&mut self) -> CSPT_W<SPT1_SPEC, 9> {
        CSPT_W::new(self)
    }
    #[doc = "Bits 12:14 - Selection sample time of channel ADC_IN14"]
    #[inline(always)]
    #[must_use]
    pub fn cspt14(&mut self) -> CSPT_W<SPT1_SPEC, 12> {
        CSPT_W::new(self)
    }
    #[doc = "Bits 15:17 - Selection sample time of channel ADC_IN15"]
    #[inline(always)]
    #[must_use]
    pub fn cspt15(&mut self) -> CSPT_W<SPT1_SPEC, 15> {
        CSPT_W::new(self)
    }
    #[doc = "Bits 18:20 - Selection sample time of channel ADC_IN16"]
    #[inline(always)]
    #[must_use]
    pub fn cspt16(&mut self) -> CSPT_W<SPT1_SPEC, 18> {
        CSPT_W::new(self)
    }
    #[doc = "Bits 21:23 - Selection sample time of channel ADC_IN17"]
    #[inline(always)]
    #[must_use]
    pub fn cspt17(&mut self) -> CSPT_W<SPT1_SPEC, 21> {
        CSPT_W::new(self)
    }
    #[doc = "Bits 24:26 - Selection sample time of channel ADC_IN18"]
    #[inline(always)]
    #[must_use]
    pub fn cspt18(&mut self) -> CSPT_W<SPT1_SPEC, 24> {
        CSPT_W::new(self)
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
