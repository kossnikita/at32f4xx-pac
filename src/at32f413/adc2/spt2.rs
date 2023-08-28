#[doc = "Register `SPT2` reader"]
pub type R = crate::R<SPT2_SPEC>;
#[doc = "Register `SPT2` writer"]
pub type W = crate::W<SPT2_SPEC>;
#[doc = "Field `CSPT0` reader - Selection sample time of channel ADC_IN0"]
pub type CSPT0_R = crate::FieldReader;
#[doc = "Field `CSPT0` writer - Selection sample time of channel ADC_IN0"]
pub type CSPT0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CSPT1` reader - Selection sample time of channel ADC_IN1"]
pub type CSPT1_R = crate::FieldReader;
#[doc = "Field `CSPT1` writer - Selection sample time of channel ADC_IN1"]
pub type CSPT1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CSPT2` reader - Selection sample time of channel ADC_IN2"]
pub type CSPT2_R = crate::FieldReader;
#[doc = "Field `CSPT2` writer - Selection sample time of channel ADC_IN2"]
pub type CSPT2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CSPT3` reader - Selection sample time of channel ADC_IN3"]
pub type CSPT3_R = crate::FieldReader;
#[doc = "Field `CSPT3` writer - Selection sample time of channel ADC_IN3"]
pub type CSPT3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CSPT4` reader - Selection sample time of channel ADC_IN4"]
pub type CSPT4_R = crate::FieldReader;
#[doc = "Field `CSPT4` writer - Selection sample time of channel ADC_IN4"]
pub type CSPT4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CSPT5` reader - Selection sample time of channel ADC_IN5"]
pub type CSPT5_R = crate::FieldReader;
#[doc = "Field `CSPT5` writer - Selection sample time of channel ADC_IN5"]
pub type CSPT5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CSPT6` reader - Selection sample time of channel ADC_IN6"]
pub type CSPT6_R = crate::FieldReader;
#[doc = "Field `CSPT6` writer - Selection sample time of channel ADC_IN6"]
pub type CSPT6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CSPT7` reader - Selection sample time of channel ADC_IN7"]
pub type CSPT7_R = crate::FieldReader;
#[doc = "Field `CSPT7` writer - Selection sample time of channel ADC_IN7"]
pub type CSPT7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CSPT8` reader - Selection sample time of channel ADC_IN8"]
pub type CSPT8_R = crate::FieldReader;
#[doc = "Field `CSPT8` writer - Selection sample time of channel ADC_IN8"]
pub type CSPT8_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `CSPT9` reader - Selection sample time of channel ADC_IN9"]
pub type CSPT9_R = crate::FieldReader;
#[doc = "Field `CSPT9` writer - Selection sample time of channel ADC_IN9"]
pub type CSPT9_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
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
    #[doc = "Bits 18:20 - Selection sample time of channel ADC_IN6"]
    #[inline(always)]
    pub fn cspt6(&self) -> CSPT6_R {
        CSPT6_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Selection sample time of channel ADC_IN7"]
    #[inline(always)]
    pub fn cspt7(&self) -> CSPT7_R {
        CSPT7_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Selection sample time of channel ADC_IN8"]
    #[inline(always)]
    pub fn cspt8(&self) -> CSPT8_R {
        CSPT8_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Selection sample time of channel ADC_IN9"]
    #[inline(always)]
    pub fn cspt9(&self) -> CSPT9_R {
        CSPT9_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selection sample time of channel ADC_IN0"]
    #[inline(always)]
    #[must_use]
    pub fn cspt0(&mut self) -> CSPT0_W<SPT2_SPEC, 0> {
        CSPT0_W::new(self)
    }
    #[doc = "Bits 3:5 - Selection sample time of channel ADC_IN1"]
    #[inline(always)]
    #[must_use]
    pub fn cspt1(&mut self) -> CSPT1_W<SPT2_SPEC, 3> {
        CSPT1_W::new(self)
    }
    #[doc = "Bits 6:8 - Selection sample time of channel ADC_IN2"]
    #[inline(always)]
    #[must_use]
    pub fn cspt2(&mut self) -> CSPT2_W<SPT2_SPEC, 6> {
        CSPT2_W::new(self)
    }
    #[doc = "Bits 9:11 - Selection sample time of channel ADC_IN3"]
    #[inline(always)]
    #[must_use]
    pub fn cspt3(&mut self) -> CSPT3_W<SPT2_SPEC, 9> {
        CSPT3_W::new(self)
    }
    #[doc = "Bits 12:14 - Selection sample time of channel ADC_IN4"]
    #[inline(always)]
    #[must_use]
    pub fn cspt4(&mut self) -> CSPT4_W<SPT2_SPEC, 12> {
        CSPT4_W::new(self)
    }
    #[doc = "Bits 15:17 - Selection sample time of channel ADC_IN5"]
    #[inline(always)]
    #[must_use]
    pub fn cspt5(&mut self) -> CSPT5_W<SPT2_SPEC, 15> {
        CSPT5_W::new(self)
    }
    #[doc = "Bits 18:20 - Selection sample time of channel ADC_IN6"]
    #[inline(always)]
    #[must_use]
    pub fn cspt6(&mut self) -> CSPT6_W<SPT2_SPEC, 18> {
        CSPT6_W::new(self)
    }
    #[doc = "Bits 21:23 - Selection sample time of channel ADC_IN7"]
    #[inline(always)]
    #[must_use]
    pub fn cspt7(&mut self) -> CSPT7_W<SPT2_SPEC, 21> {
        CSPT7_W::new(self)
    }
    #[doc = "Bits 24:26 - Selection sample time of channel ADC_IN8"]
    #[inline(always)]
    #[must_use]
    pub fn cspt8(&mut self) -> CSPT8_W<SPT2_SPEC, 24> {
        CSPT8_W::new(self)
    }
    #[doc = "Bits 27:29 - Selection sample time of channel ADC_IN9"]
    #[inline(always)]
    #[must_use]
    pub fn cspt9(&mut self) -> CSPT9_W<SPT2_SPEC, 27> {
        CSPT9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "sample time register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spt2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spt2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPT2_SPEC;
impl crate::RegisterSpec for SPT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spt2::R`](R) reader structure"]
impl crate::Readable for SPT2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spt2::W`](W) writer structure"]
impl crate::Writable for SPT2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPT2 to value 0"]
impl crate::Resettable for SPT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
