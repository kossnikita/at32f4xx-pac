#[doc = "Register `SPT1` reader"]
pub type R = crate::R<SPT1_SPEC>;
#[doc = "Register `SPT1` writer"]
pub type W = crate::W<SPT1_SPEC>;
#[doc = "Field `CSPT10` reader - Selection sample time of channel ADC_IN10"]
pub type CSPT10_R = crate::FieldReader;
#[doc = "Field `CSPT10` writer - Selection sample time of channel ADC_IN10"]
pub type CSPT10_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `CSPT11` reader - Selection sample time of channel ADC_IN11"]
pub type CSPT11_R = crate::FieldReader;
#[doc = "Field `CSPT11` writer - Selection sample time of channel ADC_IN11"]
pub type CSPT11_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `CSPT16` reader - Selection sample time of channel ADC_IN16"]
pub type CSPT16_R = crate::FieldReader;
#[doc = "Field `CSPT16` writer - Selection sample time of channel ADC_IN16"]
pub type CSPT16_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `CSPT17` reader - Selection sample time of channel ADC_IN17"]
pub type CSPT17_R = crate::FieldReader;
#[doc = "Field `CSPT17` writer - Selection sample time of channel ADC_IN17"]
pub type CSPT17_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
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
            .field("cspt17", &self.cspt17())
            .field("cspt16", &self.cspt16())
            .field("cspt11", &self.cspt11())
            .field("cspt10", &self.cspt10())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Selection sample time of channel ADC_IN10"]
    #[inline(always)]
    #[must_use]
    pub fn cspt10(&mut self) -> CSPT10_W<SPT1_SPEC> {
        CSPT10_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Selection sample time of channel ADC_IN11"]
    #[inline(always)]
    #[must_use]
    pub fn cspt11(&mut self) -> CSPT11_W<SPT1_SPEC> {
        CSPT11_W::new(self, 3)
    }
    #[doc = "Bits 18:20 - Selection sample time of channel ADC_IN16"]
    #[inline(always)]
    #[must_use]
    pub fn cspt16(&mut self) -> CSPT16_W<SPT1_SPEC> {
        CSPT16_W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Selection sample time of channel ADC_IN17"]
    #[inline(always)]
    #[must_use]
    pub fn cspt17(&mut self) -> CSPT17_W<SPT1_SPEC> {
        CSPT17_W::new(self, 21)
    }
}
#[doc = "sample time register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`spt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPT1_SPEC;
impl crate::RegisterSpec for SPT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spt1::R`](R) reader structure"]
impl crate::Readable for SPT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spt1::W`](W) writer structure"]
impl crate::Writable for SPT1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPT1 to value 0"]
impl crate::Resettable for SPT1_SPEC {
    const RESET_VALUE: u32 = 0;
}
