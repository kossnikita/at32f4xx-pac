#[doc = "Register `SPT1` reader"]
pub type R = crate::R<SPT1_SPEC>;
#[doc = "Register `SPT1` writer"]
pub type W = crate::W<SPT1_SPEC>;
#[doc = "Field `CSPT10` reader - Selection sample time of channel ADC_IN10"]
pub type CSPT10_R = crate::FieldReader;
#[doc = "Field `CSPT10` writer - Selection sample time of channel ADC_IN10"]
pub type CSPT10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CSPT11` reader - Selection sample time of channel ADC_IN11"]
pub type CSPT11_R = crate::FieldReader;
#[doc = "Field `CSPT11` writer - Selection sample time of channel ADC_IN11"]
pub type CSPT11_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CSPT12` reader - Selection sample time of channel ADC_IN12"]
pub type CSPT12_R = crate::FieldReader;
#[doc = "Field `CSPT12` writer - Selection sample time of channel ADC_IN12"]
pub type CSPT12_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CSPT13` reader - Selection sample time of channel ADC_IN13"]
pub type CSPT13_R = crate::FieldReader;
#[doc = "Field `CSPT13` writer - Selection sample time of channel ADC_IN13"]
pub type CSPT13_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CSPT14` reader - Selection sample time of channel ADC_IN14"]
pub type CSPT14_R = crate::FieldReader;
#[doc = "Field `CSPT14` writer - Selection sample time of channel ADC_IN14"]
pub type CSPT14_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CSPT15` reader - Selection sample time of channel ADC_IN15"]
pub type CSPT15_R = crate::FieldReader;
#[doc = "Field `CSPT15` writer - Selection sample time of channel ADC_IN15"]
pub type CSPT15_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CSPT16` reader - Selection sample time of channel ADC_IN16"]
pub type CSPT16_R = crate::FieldReader;
#[doc = "Field `CSPT16` writer - Selection sample time of channel ADC_IN16"]
pub type CSPT16_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CSPT17` reader - Selection sample time of channel ADC_IN17"]
pub type CSPT17_R = crate::FieldReader;
#[doc = "Field `CSPT17` writer - Selection sample time of channel ADC_IN17"]
pub type CSPT17_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CSPT18` reader - Selection sample time of channel ADC_IN18"]
pub type CSPT18_R = crate::FieldReader;
#[doc = "Field `CSPT18` writer - Selection sample time of channel ADC_IN18"]
pub type CSPT18_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
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
    #[doc = "Bits 6:8 - Selection sample time of channel ADC_IN12"]
    #[inline(always)]
    pub fn cspt12(&self) -> CSPT12_R {
        CSPT12_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Selection sample time of channel ADC_IN13"]
    #[inline(always)]
    pub fn cspt13(&self) -> CSPT13_R {
        CSPT13_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Selection sample time of channel ADC_IN14"]
    #[inline(always)]
    pub fn cspt14(&self) -> CSPT14_R {
        CSPT14_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Selection sample time of channel ADC_IN15"]
    #[inline(always)]
    pub fn cspt15(&self) -> CSPT15_R {
        CSPT15_R::new(((self.bits >> 15) & 7) as u8)
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
    #[doc = "Bits 24:26 - Selection sample time of channel ADC_IN18"]
    #[inline(always)]
    pub fn cspt18(&self) -> CSPT18_R {
        CSPT18_R::new(((self.bits >> 24) & 7) as u8)
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
    #[doc = "Bits 6:8 - Selection sample time of channel ADC_IN12"]
    #[inline(always)]
    #[must_use]
    pub fn cspt12(&mut self) -> CSPT12_W<SPT1_SPEC, 6> {
        CSPT12_W::new(self)
    }
    #[doc = "Bits 9:11 - Selection sample time of channel ADC_IN13"]
    #[inline(always)]
    #[must_use]
    pub fn cspt13(&mut self) -> CSPT13_W<SPT1_SPEC, 9> {
        CSPT13_W::new(self)
    }
    #[doc = "Bits 12:14 - Selection sample time of channel ADC_IN14"]
    #[inline(always)]
    #[must_use]
    pub fn cspt14(&mut self) -> CSPT14_W<SPT1_SPEC, 12> {
        CSPT14_W::new(self)
    }
    #[doc = "Bits 15:17 - Selection sample time of channel ADC_IN15"]
    #[inline(always)]
    #[must_use]
    pub fn cspt15(&mut self) -> CSPT15_W<SPT1_SPEC, 15> {
        CSPT15_W::new(self)
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
    #[doc = "Bits 24:26 - Selection sample time of channel ADC_IN18"]
    #[inline(always)]
    #[must_use]
    pub fn cspt18(&mut self) -> CSPT18_W<SPT1_SPEC, 24> {
        CSPT18_W::new(self)
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
