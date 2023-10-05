#[doc = "Register `PSQ` reader"]
pub type R = crate::R<PSQ_SPEC>;
#[doc = "Register `PSQ` writer"]
pub type W = crate::W<PSQ_SPEC>;
#[doc = "Field `PSN[1-4]` reader - Number of %s conversion in preempted sequence"]
pub type PSN_R = crate::FieldReader;
#[doc = "Field `PSN[1-4]` writer - Number of %s conversion in preempted sequence"]
pub type PSN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `PCLEN` reader - Preempted conversion sequence length"]
pub type PCLEN_R = crate::FieldReader;
#[doc = "Field `PCLEN` writer - Preempted conversion sequence length"]
pub type PCLEN_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O>;
impl R {
    #[doc = "Number of [1-4]
conversion in preempted sequence"]
    #[inline(always)]
    pub unsafe fn psn(&self, n: u8) -> PSN_R {
        PSN_R::new(((self.bits >> ((n - 1) * 5)) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - Number of 1 conversion in preempted sequence"]
    #[inline(always)]
    pub fn psn1(&self) -> PSN_R {
        PSN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Number of 2 conversion in preempted sequence"]
    #[inline(always)]
    pub fn psn2(&self) -> PSN_R {
        PSN_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Number of 3 conversion in preempted sequence"]
    #[inline(always)]
    pub fn psn3(&self) -> PSN_R {
        PSN_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Number of 4 conversion in preempted sequence"]
    #[inline(always)]
    pub fn psn4(&self) -> PSN_R {
        PSN_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:21 - Preempted conversion sequence length"]
    #[inline(always)]
    pub fn pclen(&self) -> PCLEN_R {
        PCLEN_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Number of [1-4]
conversion in preempted sequence"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn psn<const O: u8>(&mut self) -> PSN_W<PSQ_SPEC, O> {
        PSN_W::new(self)
    }
    #[doc = "Bits 0:4 - Number of 1 conversion in preempted sequence"]
    #[inline(always)]
    #[must_use]
    pub fn psn1(&mut self) -> PSN_W<PSQ_SPEC, 0> {
        PSN_W::new(self)
    }
    #[doc = "Bits 5:9 - Number of 2 conversion in preempted sequence"]
    #[inline(always)]
    #[must_use]
    pub fn psn2(&mut self) -> PSN_W<PSQ_SPEC, 5> {
        PSN_W::new(self)
    }
    #[doc = "Bits 10:14 - Number of 3 conversion in preempted sequence"]
    #[inline(always)]
    #[must_use]
    pub fn psn3(&mut self) -> PSN_W<PSQ_SPEC, 10> {
        PSN_W::new(self)
    }
    #[doc = "Bits 15:19 - Number of 4 conversion in preempted sequence"]
    #[inline(always)]
    #[must_use]
    pub fn psn4(&mut self) -> PSN_W<PSQ_SPEC, 15> {
        PSN_W::new(self)
    }
    #[doc = "Bits 20:21 - Preempted conversion sequence length"]
    #[inline(always)]
    #[must_use]
    pub fn pclen(&mut self) -> PCLEN_W<PSQ_SPEC, 20> {
        PCLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Preempted sequence register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSQ_SPEC;
impl crate::RegisterSpec for PSQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psq::R`](R) reader structure"]
impl crate::Readable for PSQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psq::W`](W) writer structure"]
impl crate::Writable for PSQ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSQ to value 0"]
impl crate::Resettable for PSQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
