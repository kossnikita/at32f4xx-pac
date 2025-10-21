#[doc = "Register `SPT1` reader"]
pub type R = crate::R<SPT1_SPEC>;
#[doc = "Register `SPT1` writer"]
pub type W = crate::W<SPT1_SPEC>;
#[doc = "Field `CSPT(10-17)` reader - Selection sample time of channel ADC_IN%s"]
pub type CSPT_R = crate::FieldReader;
#[doc = "Field `CSPT(10-17)` writer - Selection sample time of channel ADC_IN%s"]
pub type CSPT_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
impl R {
    #[doc = "Selection sample time of channel ADC_IN(10-17)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CSPT10` field.</div>"]
    #[inline(always)]
    pub fn cspt(&self, n: u8) -> CSPT_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CSPT_R::new(((self.bits >> (n * 3)) & 7) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Selection sample time of channel ADC_IN(10-17)"]
    #[inline(always)]
    pub fn cspt_iter(&self) -> impl Iterator<Item = CSPT_R> + '_ {
        (0..8).map(move |n| CSPT_R::new(((self.bits >> (n * 3)) & 7) as u8))
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPT1")
            .field("cspt10", &self.cspt10())
            .field("cspt11", &self.cspt11())
            .field("cspt12", &self.cspt12())
            .field("cspt13", &self.cspt13())
            .field("cspt14", &self.cspt14())
            .field("cspt15", &self.cspt15())
            .field("cspt16", &self.cspt16())
            .field("cspt17", &self.cspt17())
            .finish()
    }
}
impl W {
    #[doc = "Selection sample time of channel ADC_IN(10-17)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CSPT10` field.</div>"]
    #[inline(always)]
    pub fn cspt(&mut self, n: u8) -> CSPT_W<'_, SPT1_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CSPT_W::new(self, n * 3)
    }
    #[doc = "Bits 0:2 - Selection sample time of channel ADC_IN10"]
    #[inline(always)]
    pub fn cspt10(&mut self) -> CSPT_W<'_, SPT1_SPEC> {
        CSPT_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Selection sample time of channel ADC_IN11"]
    #[inline(always)]
    pub fn cspt11(&mut self) -> CSPT_W<'_, SPT1_SPEC> {
        CSPT_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Selection sample time of channel ADC_IN12"]
    #[inline(always)]
    pub fn cspt12(&mut self) -> CSPT_W<'_, SPT1_SPEC> {
        CSPT_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Selection sample time of channel ADC_IN13"]
    #[inline(always)]
    pub fn cspt13(&mut self) -> CSPT_W<'_, SPT1_SPEC> {
        CSPT_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Selection sample time of channel ADC_IN14"]
    #[inline(always)]
    pub fn cspt14(&mut self) -> CSPT_W<'_, SPT1_SPEC> {
        CSPT_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Selection sample time of channel ADC_IN15"]
    #[inline(always)]
    pub fn cspt15(&mut self) -> CSPT_W<'_, SPT1_SPEC> {
        CSPT_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Selection sample time of channel ADC_IN16"]
    #[inline(always)]
    pub fn cspt16(&mut self) -> CSPT_W<'_, SPT1_SPEC> {
        CSPT_W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Selection sample time of channel ADC_IN17"]
    #[inline(always)]
    pub fn cspt17(&mut self) -> CSPT_W<'_, SPT1_SPEC> {
        CSPT_W::new(self, 21)
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
}
#[doc = "`reset()` method sets SPT1 to value 0"]
impl crate::Resettable for SPT1_SPEC {}
