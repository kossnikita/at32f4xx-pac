#[doc = "Register `SPT3` reader"]
pub type R = crate::R<SPT3_SPEC>;
#[doc = "Register `SPT3` writer"]
pub type W = crate::W<SPT3_SPEC>;
#[doc = "Field `CSPT20` reader - Selection sample time of channel ADC_IN20"]
pub type CSPT20_R = crate::FieldReader;
#[doc = "Field `CSPT20` writer - Selection sample time of channel ADC_IN20"]
pub type CSPT20_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CSPT21` reader - Selection sample time of channel ADC_IN21"]
pub type CSPT21_R = crate::FieldReader;
#[doc = "Field `CSPT21` writer - Selection sample time of channel ADC_IN21"]
pub type CSPT21_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CSPT22` reader - Selection sample time of channel ADC_IN22"]
pub type CSPT22_R = crate::FieldReader;
#[doc = "Field `CSPT22` writer - Selection sample time of channel ADC_IN22"]
pub type CSPT22_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CSPT23` reader - Selection sample time of channel ADC_IN23"]
pub type CSPT23_R = crate::FieldReader;
#[doc = "Field `CSPT23` writer - Selection sample time of channel ADC_IN23"]
pub type CSPT23_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CSPT24` reader - Selection sample time of channel ADC_IN24"]
pub type CSPT24_R = crate::FieldReader;
#[doc = "Field `CSPT24` writer - Selection sample time of channel ADC_IN24"]
pub type CSPT24_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CSPT25` reader - Selection sample time of channel ADC_IN25"]
pub type CSPT25_R = crate::FieldReader;
#[doc = "Field `CSPT25` writer - Selection sample time of channel ADC_IN25"]
pub type CSPT25_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CSPT26` reader - Selection sample time of channel ADC_IN26"]
pub type CSPT26_R = crate::FieldReader;
#[doc = "Field `CSPT26` writer - Selection sample time of channel ADC_IN26"]
pub type CSPT26_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CSPT27` reader - Selection sample time of channel ADC_IN27"]
pub type CSPT27_R = crate::FieldReader;
#[doc = "Field `CSPT27` writer - Selection sample time of channel ADC_IN27"]
pub type CSPT27_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Selection sample time of channel ADC_IN20"]
    #[inline(always)]
    pub fn cspt20(&self) -> CSPT20_R {
        CSPT20_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Selection sample time of channel ADC_IN21"]
    #[inline(always)]
    pub fn cspt21(&self) -> CSPT21_R {
        CSPT21_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Selection sample time of channel ADC_IN22"]
    #[inline(always)]
    pub fn cspt22(&self) -> CSPT22_R {
        CSPT22_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Selection sample time of channel ADC_IN23"]
    #[inline(always)]
    pub fn cspt23(&self) -> CSPT23_R {
        CSPT23_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Selection sample time of channel ADC_IN24"]
    #[inline(always)]
    pub fn cspt24(&self) -> CSPT24_R {
        CSPT24_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Selection sample time of channel ADC_IN25"]
    #[inline(always)]
    pub fn cspt25(&self) -> CSPT25_R {
        CSPT25_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Selection sample time of channel ADC_IN26"]
    #[inline(always)]
    pub fn cspt26(&self) -> CSPT26_R {
        CSPT26_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Selection sample time of channel ADC_IN27"]
    #[inline(always)]
    pub fn cspt27(&self) -> CSPT27_R {
        CSPT27_R::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selection sample time of channel ADC_IN20"]
    #[inline(always)]
    #[must_use]
    pub fn cspt20(&mut self) -> CSPT20_W<SPT3_SPEC, 0> {
        CSPT20_W::new(self)
    }
    #[doc = "Bits 3:5 - Selection sample time of channel ADC_IN21"]
    #[inline(always)]
    #[must_use]
    pub fn cspt21(&mut self) -> CSPT21_W<SPT3_SPEC, 3> {
        CSPT21_W::new(self)
    }
    #[doc = "Bits 6:8 - Selection sample time of channel ADC_IN22"]
    #[inline(always)]
    #[must_use]
    pub fn cspt22(&mut self) -> CSPT22_W<SPT3_SPEC, 6> {
        CSPT22_W::new(self)
    }
    #[doc = "Bits 9:11 - Selection sample time of channel ADC_IN23"]
    #[inline(always)]
    #[must_use]
    pub fn cspt23(&mut self) -> CSPT23_W<SPT3_SPEC, 9> {
        CSPT23_W::new(self)
    }
    #[doc = "Bits 12:14 - Selection sample time of channel ADC_IN24"]
    #[inline(always)]
    #[must_use]
    pub fn cspt24(&mut self) -> CSPT24_W<SPT3_SPEC, 12> {
        CSPT24_W::new(self)
    }
    #[doc = "Bits 15:17 - Selection sample time of channel ADC_IN25"]
    #[inline(always)]
    #[must_use]
    pub fn cspt25(&mut self) -> CSPT25_W<SPT3_SPEC, 15> {
        CSPT25_W::new(self)
    }
    #[doc = "Bits 18:20 - Selection sample time of channel ADC_IN26"]
    #[inline(always)]
    #[must_use]
    pub fn cspt26(&mut self) -> CSPT26_W<SPT3_SPEC, 18> {
        CSPT26_W::new(self)
    }
    #[doc = "Bits 21:23 - Selection sample time of channel ADC_IN27"]
    #[inline(always)]
    #[must_use]
    pub fn cspt27(&mut self) -> CSPT27_W<SPT3_SPEC, 21> {
        CSPT27_W::new(self)
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
