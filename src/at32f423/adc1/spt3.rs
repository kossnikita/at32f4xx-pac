#[doc = "Register `SPT3` reader"]
pub type R = crate::R<SPT3_SPEC>;
#[doc = "Register `SPT3` writer"]
pub type W = crate::W<SPT3_SPEC>;
#[doc = "Field `CSPT[20-27]` reader - Selection sample time of channel ADC_IN%s"]
pub type CSPT_R = crate::FieldReader;
#[doc = "Field `CSPT[20-27]` writer - Selection sample time of channel ADC_IN%s"]
pub type CSPT_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O>;
impl R {
    #[doc = "Selection sample time of channel ADC_IN[20-27]"]
    #[inline(always)]
    pub unsafe fn cspt(&self, n: u8) -> CSPT_R {
        CSPT_R::new(((self.bits >> ((n - 20) * 3)) & 7) as u8)
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
impl W {
    #[doc = "Selection sample time of channel ADC_IN[20-27]"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn cspt<const O: u8>(&mut self) -> CSPT_W<SPT3_SPEC, O> {
        CSPT_W::new(self)
    }
    #[doc = "Bits 0:2 - Selection sample time of channel ADC_IN20"]
    #[inline(always)]
    #[must_use]
    pub fn cspt20(&mut self) -> CSPT_W<SPT3_SPEC, 0> {
        CSPT_W::new(self)
    }
    #[doc = "Bits 3:5 - Selection sample time of channel ADC_IN21"]
    #[inline(always)]
    #[must_use]
    pub fn cspt21(&mut self) -> CSPT_W<SPT3_SPEC, 3> {
        CSPT_W::new(self)
    }
    #[doc = "Bits 6:8 - Selection sample time of channel ADC_IN22"]
    #[inline(always)]
    #[must_use]
    pub fn cspt22(&mut self) -> CSPT_W<SPT3_SPEC, 6> {
        CSPT_W::new(self)
    }
    #[doc = "Bits 9:11 - Selection sample time of channel ADC_IN23"]
    #[inline(always)]
    #[must_use]
    pub fn cspt23(&mut self) -> CSPT_W<SPT3_SPEC, 9> {
        CSPT_W::new(self)
    }
    #[doc = "Bits 12:14 - Selection sample time of channel ADC_IN24"]
    #[inline(always)]
    #[must_use]
    pub fn cspt24(&mut self) -> CSPT_W<SPT3_SPEC, 12> {
        CSPT_W::new(self)
    }
    #[doc = "Bits 15:17 - Selection sample time of channel ADC_IN25"]
    #[inline(always)]
    #[must_use]
    pub fn cspt25(&mut self) -> CSPT_W<SPT3_SPEC, 15> {
        CSPT_W::new(self)
    }
    #[doc = "Bits 18:20 - Selection sample time of channel ADC_IN26"]
    #[inline(always)]
    #[must_use]
    pub fn cspt26(&mut self) -> CSPT_W<SPT3_SPEC, 18> {
        CSPT_W::new(self)
    }
    #[doc = "Bits 21:23 - Selection sample time of channel ADC_IN27"]
    #[inline(always)]
    #[must_use]
    pub fn cspt27(&mut self) -> CSPT_W<SPT3_SPEC, 21> {
        CSPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPT3 to value 0"]
impl crate::Resettable for SPT3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
