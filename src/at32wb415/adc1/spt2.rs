#[doc = "Register `SPT2` reader"]
pub type R = crate::R<SPT2_SPEC>;
#[doc = "Register `SPT2` writer"]
pub type W = crate::W<SPT2_SPEC>;
#[doc = "Field `CSPT0` reader - Selection sample time of channel ADC_IN0"]
pub type CSPT0_R = crate::FieldReader;
#[doc = "Field `CSPT0` writer - Selection sample time of channel ADC_IN0"]
pub type CSPT0_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `CSPT1` reader - Selection sample time of channel ADC_IN1"]
pub type CSPT1_R = crate::FieldReader;
#[doc = "Field `CSPT1` writer - Selection sample time of channel ADC_IN1"]
pub type CSPT1_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `CSPT2` reader - Selection sample time of channel ADC_IN2"]
pub type CSPT2_R = crate::FieldReader;
#[doc = "Field `CSPT2` writer - Selection sample time of channel ADC_IN2"]
pub type CSPT2_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `CSPT3` reader - Selection sample time of channel ADC_IN3"]
pub type CSPT3_R = crate::FieldReader;
#[doc = "Field `CSPT3` writer - Selection sample time of channel ADC_IN3"]
pub type CSPT3_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `CSPT4` reader - Selection sample time of channel ADC_IN4"]
pub type CSPT4_R = crate::FieldReader;
#[doc = "Field `CSPT4` writer - Selection sample time of channel ADC_IN4"]
pub type CSPT4_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
#[doc = "Field `CSPT5` reader - Selection sample time of channel ADC_IN5"]
pub type CSPT5_R = crate::FieldReader;
#[doc = "Field `CSPT5` writer - Selection sample time of channel ADC_IN5"]
pub type CSPT5_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
impl R {
    #[doc = "Bits 0:2 - Selection sample time of channel ADC_IN0"]
    #[inline(always)]
    pub fn cspt0(&self) -> CSPT0_R {
        CSPT0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Selection sample time of channel ADC_IN1"]
    #[inline(always)]
    pub fn cspt1(&self) -> CSPT1_R {
        CSPT1_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Selection sample time of channel ADC_IN2"]
    #[inline(always)]
    pub fn cspt2(&self) -> CSPT2_R {
        CSPT2_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Selection sample time of channel ADC_IN3"]
    #[inline(always)]
    pub fn cspt3(&self) -> CSPT3_R {
        CSPT3_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Selection sample time of channel ADC_IN4"]
    #[inline(always)]
    pub fn cspt4(&self) -> CSPT4_R {
        CSPT4_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Selection sample time of channel ADC_IN5"]
    #[inline(always)]
    pub fn cspt5(&self) -> CSPT5_R {
        CSPT5_R::new(((self.bits >> 15) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPT2")
            .field("cspt5", &self.cspt5())
            .field("cspt4", &self.cspt4())
            .field("cspt3", &self.cspt3())
            .field("cspt2", &self.cspt2())
            .field("cspt1", &self.cspt1())
            .field("cspt0", &self.cspt0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Selection sample time of channel ADC_IN0"]
    #[inline(always)]
    #[must_use]
    pub fn cspt0(&mut self) -> CSPT0_W<SPT2_SPEC> {
        CSPT0_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Selection sample time of channel ADC_IN1"]
    #[inline(always)]
    #[must_use]
    pub fn cspt1(&mut self) -> CSPT1_W<SPT2_SPEC> {
        CSPT1_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Selection sample time of channel ADC_IN2"]
    #[inline(always)]
    #[must_use]
    pub fn cspt2(&mut self) -> CSPT2_W<SPT2_SPEC> {
        CSPT2_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Selection sample time of channel ADC_IN3"]
    #[inline(always)]
    #[must_use]
    pub fn cspt3(&mut self) -> CSPT3_W<SPT2_SPEC> {
        CSPT3_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Selection sample time of channel ADC_IN4"]
    #[inline(always)]
    #[must_use]
    pub fn cspt4(&mut self) -> CSPT4_W<SPT2_SPEC> {
        CSPT4_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Selection sample time of channel ADC_IN5"]
    #[inline(always)]
    #[must_use]
    pub fn cspt5(&mut self) -> CSPT5_W<SPT2_SPEC> {
        CSPT5_W::new(self, 15)
    }
}
#[doc = "sample time register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`spt2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spt2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPT2_SPEC;
impl crate::RegisterSpec for SPT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spt2::R`](R) reader structure"]
impl crate::Readable for SPT2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spt2::W`](W) writer structure"]
impl crate::Writable for SPT2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPT2 to value 0"]
impl crate::Resettable for SPT2_SPEC {
    const RESET_VALUE: u32 = 0;
}
