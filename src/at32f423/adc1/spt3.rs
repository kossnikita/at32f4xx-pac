#[doc = "Register `SPT3` reader"]
pub type R = crate::R<SPT3_SPEC>;
#[doc = "Register `SPT3` writer"]
pub type W = crate::W<SPT3_SPEC>;
#[doc = "Field `CSPT(20-27)` reader - Selection sample time of channel ADC_IN%s"]
pub type CSPT_R = crate::FieldReader;
#[doc = "Field `CSPT(20-27)` writer - Selection sample time of channel ADC_IN%s"]
pub type CSPT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
impl R {
    #[doc = "Selection sample time of channel ADC_IN(20-27)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CSPT20` field"]
    #[inline(always)]
    pub fn cspt(&self, n: u8) -> CSPT_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CSPT_R::new(((self.bits >> (n * 3)) & 7) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Selection sample time of channel ADC_IN(20-27)"]
    #[inline(always)]
    pub fn cspt_iter(&self) -> impl Iterator<Item = CSPT_R> + '_ {
        (0..8).map(move |n| CSPT_R::new(((self.bits >> (n * 3)) & 7) as u8))
    }
    #[doc = "Bits 0:2 - Selection sample time of channel ADC_IN20"]
    #[inline(always)]
    pub fn cspt20(&self) -> CSPT_R {
        CSPT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Selection sample time of channel ADC_IN21"]
    #[inline(always)]
    pub fn cspt21(&self) -> CSPT_R {
        CSPT_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Selection sample time of channel ADC_IN22"]
    #[inline(always)]
    pub fn cspt22(&self) -> CSPT_R {
        CSPT_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Selection sample time of channel ADC_IN23"]
    #[inline(always)]
    pub fn cspt23(&self) -> CSPT_R {
        CSPT_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Selection sample time of channel ADC_IN24"]
    #[inline(always)]
    pub fn cspt24(&self) -> CSPT_R {
        CSPT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Selection sample time of channel ADC_IN25"]
    #[inline(always)]
    pub fn cspt25(&self) -> CSPT_R {
        CSPT_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Selection sample time of channel ADC_IN26"]
    #[inline(always)]
    pub fn cspt26(&self) -> CSPT_R {
        CSPT_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Selection sample time of channel ADC_IN27"]
    #[inline(always)]
    pub fn cspt27(&self) -> CSPT_R {
        CSPT_R::new(((self.bits >> 21) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPT3")
            .field("cspt20", &format_args!("{}", self.cspt20().bits()))
            .field("cspt21", &format_args!("{}", self.cspt21().bits()))
            .field("cspt22", &format_args!("{}", self.cspt22().bits()))
            .field("cspt23", &format_args!("{}", self.cspt23().bits()))
            .field("cspt24", &format_args!("{}", self.cspt24().bits()))
            .field("cspt25", &format_args!("{}", self.cspt25().bits()))
            .field("cspt26", &format_args!("{}", self.cspt26().bits()))
            .field("cspt27", &format_args!("{}", self.cspt27().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<SPT3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Selection sample time of channel ADC_IN(20-27)"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CSPT20` field"]
    #[inline(always)]
    #[must_use]
    pub fn cspt(&mut self, n: u8) -> CSPT_W<SPT3_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CSPT_W::new(self, n * 3)
    }
    #[doc = "Bits 0:2 - Selection sample time of channel ADC_IN20"]
    #[inline(always)]
    #[must_use]
    pub fn cspt20(&mut self) -> CSPT_W<SPT3_SPEC> {
        CSPT_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Selection sample time of channel ADC_IN21"]
    #[inline(always)]
    #[must_use]
    pub fn cspt21(&mut self) -> CSPT_W<SPT3_SPEC> {
        CSPT_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Selection sample time of channel ADC_IN22"]
    #[inline(always)]
    #[must_use]
    pub fn cspt22(&mut self) -> CSPT_W<SPT3_SPEC> {
        CSPT_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Selection sample time of channel ADC_IN23"]
    #[inline(always)]
    #[must_use]
    pub fn cspt23(&mut self) -> CSPT_W<SPT3_SPEC> {
        CSPT_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Selection sample time of channel ADC_IN24"]
    #[inline(always)]
    #[must_use]
    pub fn cspt24(&mut self) -> CSPT_W<SPT3_SPEC> {
        CSPT_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Selection sample time of channel ADC_IN25"]
    #[inline(always)]
    #[must_use]
    pub fn cspt25(&mut self) -> CSPT_W<SPT3_SPEC> {
        CSPT_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Selection sample time of channel ADC_IN26"]
    #[inline(always)]
    #[must_use]
    pub fn cspt26(&mut self) -> CSPT_W<SPT3_SPEC> {
        CSPT_W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Selection sample time of channel ADC_IN27"]
    #[inline(always)]
    #[must_use]
    pub fn cspt27(&mut self) -> CSPT_W<SPT3_SPEC> {
        CSPT_W::new(self, 21)
    }
}
#[doc = "sample time register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spt3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spt3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPT3_SPEC;
impl crate::RegisterSpec for SPT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spt3::R`](R) reader structure"]
impl crate::Readable for SPT3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spt3::W`](W) writer structure"]
impl crate::Writable for SPT3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPT3 to value 0"]
impl crate::Resettable for SPT3_SPEC {
    const RESET_VALUE: u32 = 0;
}
