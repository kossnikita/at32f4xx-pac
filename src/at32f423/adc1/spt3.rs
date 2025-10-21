#[doc = "Register `SPT3` reader"]
pub type R = crate::R<SPT3_SPEC>;
#[doc = "Register `SPT3` writer"]
pub type W = crate::W<SPT3_SPEC>;
#[doc = "Field `CSPT(20-27)` reader - Selection sample time of channel ADC_IN%s"]
pub type CSPT_R = crate::FieldReader;
#[doc = "Field `CSPT(20-27)` writer - Selection sample time of channel ADC_IN%s"]
pub type CSPT_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
impl R {
    #[doc = "Selection sample time of channel ADC_IN(20-27)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CSPT20` field.</div>"]
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
            .field("cspt20", &self.cspt20())
            .field("cspt21", &self.cspt21())
            .field("cspt22", &self.cspt22())
            .field("cspt23", &self.cspt23())
            .field("cspt24", &self.cspt24())
            .field("cspt25", &self.cspt25())
            .field("cspt26", &self.cspt26())
            .field("cspt27", &self.cspt27())
            .finish()
    }
}
impl W {
    #[doc = "Selection sample time of channel ADC_IN(20-27)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CSPT20` field.</div>"]
    #[inline(always)]
    pub fn cspt(&mut self, n: u8) -> CSPT_W<'_, SPT3_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CSPT_W::new(self, n * 3)
    }
    #[doc = "Bits 0:2 - Selection sample time of channel ADC_IN20"]
    #[inline(always)]
    pub fn cspt20(&mut self) -> CSPT_W<'_, SPT3_SPEC> {
        CSPT_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Selection sample time of channel ADC_IN21"]
    #[inline(always)]
    pub fn cspt21(&mut self) -> CSPT_W<'_, SPT3_SPEC> {
        CSPT_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Selection sample time of channel ADC_IN22"]
    #[inline(always)]
    pub fn cspt22(&mut self) -> CSPT_W<'_, SPT3_SPEC> {
        CSPT_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Selection sample time of channel ADC_IN23"]
    #[inline(always)]
    pub fn cspt23(&mut self) -> CSPT_W<'_, SPT3_SPEC> {
        CSPT_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Selection sample time of channel ADC_IN24"]
    #[inline(always)]
    pub fn cspt24(&mut self) -> CSPT_W<'_, SPT3_SPEC> {
        CSPT_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Selection sample time of channel ADC_IN25"]
    #[inline(always)]
    pub fn cspt25(&mut self) -> CSPT_W<'_, SPT3_SPEC> {
        CSPT_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Selection sample time of channel ADC_IN26"]
    #[inline(always)]
    pub fn cspt26(&mut self) -> CSPT_W<'_, SPT3_SPEC> {
        CSPT_W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Selection sample time of channel ADC_IN27"]
    #[inline(always)]
    pub fn cspt27(&mut self) -> CSPT_W<'_, SPT3_SPEC> {
        CSPT_W::new(self, 21)
    }
}
#[doc = "sample time register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`spt3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spt3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPT3_SPEC;
impl crate::RegisterSpec for SPT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spt3::R`](R) reader structure"]
impl crate::Readable for SPT3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spt3::W`](W) writer structure"]
impl crate::Writable for SPT3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPT3 to value 0"]
impl crate::Resettable for SPT3_SPEC {}
