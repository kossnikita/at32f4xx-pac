#[doc = "Register `SPT2` reader"]
pub type R = crate::R<SPT2_SPEC>;
#[doc = "Register `SPT2` writer"]
pub type W = crate::W<SPT2_SPEC>;
#[doc = "Field `CSPT[0-9]` reader - Selection sample time of channel ADC_IN%s"]
pub type CSPT_R = crate::FieldReader;
#[doc = "Field `CSPT[0-9]` writer - Selection sample time of channel ADC_IN%s"]
pub type CSPT_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
impl R {
    #[doc = "Selection sample time of channel ADC_IN[0-9]\n\nNOTE: `n` is number of field in register starting from 0"]
    #[inline(always)]
    pub fn cspt(&self, n: u8) -> CSPT_R {
        assert!(n < 10);
        CSPT_R::new(((self.bits >> (n * 3)) & 7) as u8)
    }
    #[doc = "Bits 0:2 - Selection sample time of channel ADC_IN0"]
    #[inline(always)]
    pub fn cspt0(&self) -> CSPT_R {
        CSPT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Selection sample time of channel ADC_IN1"]
    #[inline(always)]
    pub fn cspt1(&self) -> CSPT_R {
        CSPT_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Selection sample time of channel ADC_IN2"]
    #[inline(always)]
    pub fn cspt2(&self) -> CSPT_R {
        CSPT_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Selection sample time of channel ADC_IN3"]
    #[inline(always)]
    pub fn cspt3(&self) -> CSPT_R {
        CSPT_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Selection sample time of channel ADC_IN4"]
    #[inline(always)]
    pub fn cspt4(&self) -> CSPT_R {
        CSPT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Selection sample time of channel ADC_IN5"]
    #[inline(always)]
    pub fn cspt5(&self) -> CSPT_R {
        CSPT_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Selection sample time of channel ADC_IN6"]
    #[inline(always)]
    pub fn cspt6(&self) -> CSPT_R {
        CSPT_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Selection sample time of channel ADC_IN7"]
    #[inline(always)]
    pub fn cspt7(&self) -> CSPT_R {
        CSPT_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Selection sample time of channel ADC_IN8"]
    #[inline(always)]
    pub fn cspt8(&self) -> CSPT_R {
        CSPT_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Selection sample time of channel ADC_IN9"]
    #[inline(always)]
    pub fn cspt9(&self) -> CSPT_R {
        CSPT_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPT2")
            .field("cspt0", &format_args!("{}", self.cspt0().bits()))
            .field("cspt1", &format_args!("{}", self.cspt1().bits()))
            .field("cspt2", &format_args!("{}", self.cspt2().bits()))
            .field("cspt3", &format_args!("{}", self.cspt3().bits()))
            .field("cspt4", &format_args!("{}", self.cspt4().bits()))
            .field("cspt5", &format_args!("{}", self.cspt5().bits()))
            .field("cspt6", &format_args!("{}", self.cspt6().bits()))
            .field("cspt7", &format_args!("{}", self.cspt7().bits()))
            .field("cspt8", &format_args!("{}", self.cspt8().bits()))
            .field("cspt9", &format_args!("{}", self.cspt9().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<SPT2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Selection sample time of channel ADC_IN[0-9]"]
    #[inline(always)]
    #[must_use]
    pub fn cspt(&mut self, n: u8) -> CSPT_W<SPT2_SPEC> {
        assert!(n < 10);
        CSPT_W::new(self, n * 3)
    }
    #[doc = "Bits 0:2 - Selection sample time of channel ADC_IN0"]
    #[inline(always)]
    #[must_use]
    pub fn cspt0(&mut self) -> CSPT_W<SPT2_SPEC> {
        CSPT_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Selection sample time of channel ADC_IN1"]
    #[inline(always)]
    #[must_use]
    pub fn cspt1(&mut self) -> CSPT_W<SPT2_SPEC> {
        CSPT_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Selection sample time of channel ADC_IN2"]
    #[inline(always)]
    #[must_use]
    pub fn cspt2(&mut self) -> CSPT_W<SPT2_SPEC> {
        CSPT_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Selection sample time of channel ADC_IN3"]
    #[inline(always)]
    #[must_use]
    pub fn cspt3(&mut self) -> CSPT_W<SPT2_SPEC> {
        CSPT_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Selection sample time of channel ADC_IN4"]
    #[inline(always)]
    #[must_use]
    pub fn cspt4(&mut self) -> CSPT_W<SPT2_SPEC> {
        CSPT_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Selection sample time of channel ADC_IN5"]
    #[inline(always)]
    #[must_use]
    pub fn cspt5(&mut self) -> CSPT_W<SPT2_SPEC> {
        CSPT_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Selection sample time of channel ADC_IN6"]
    #[inline(always)]
    #[must_use]
    pub fn cspt6(&mut self) -> CSPT_W<SPT2_SPEC> {
        CSPT_W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Selection sample time of channel ADC_IN7"]
    #[inline(always)]
    #[must_use]
    pub fn cspt7(&mut self) -> CSPT_W<SPT2_SPEC> {
        CSPT_W::new(self, 21)
    }
    #[doc = "Bits 24:26 - Selection sample time of channel ADC_IN8"]
    #[inline(always)]
    #[must_use]
    pub fn cspt8(&mut self) -> CSPT_W<SPT2_SPEC> {
        CSPT_W::new(self, 24)
    }
    #[doc = "Bits 27:29 - Selection sample time of channel ADC_IN9"]
    #[inline(always)]
    #[must_use]
    pub fn cspt9(&mut self) -> CSPT_W<SPT2_SPEC> {
        CSPT_W::new(self, 27)
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
#[doc = "sample time register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spt2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spt2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPT2_SPEC;
impl crate::RegisterSpec for SPT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spt2::R`](R) reader structure"]
impl crate::Readable for SPT2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spt2::W`](W) writer structure"]
impl crate::Writable for SPT2_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPT2 to value 0"]
impl crate::Resettable for SPT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
