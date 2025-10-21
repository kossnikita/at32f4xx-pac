#[doc = "Register `PSQ` reader"]
pub type R = crate::R<PSQ_SPEC>;
#[doc = "Register `PSQ` writer"]
pub type W = crate::W<PSQ_SPEC>;
#[doc = "Field `PSN(1-4)` reader - Number of %s conversion in preempted sequence"]
pub type PSN_R = crate::FieldReader;
#[doc = "Field `PSN(1-4)` writer - Number of %s conversion in preempted sequence"]
pub type PSN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PCLEN` reader - Preempted conversion sequence length"]
pub type PCLEN_R = crate::FieldReader;
#[doc = "Field `PCLEN` writer - Preempted conversion sequence length"]
pub type PCLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
impl R {
    #[doc = "Number of (1-4) conversion in preempted sequence"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `PSN1` field.</div>"]
    #[inline(always)]
    pub fn psn(&self, n: u8) -> PSN_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        PSN_R::new(((self.bits >> (n * 5)) & 0x1f) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Number of (1-4) conversion in preempted sequence"]
    #[inline(always)]
    pub fn psn_iter(&self) -> impl Iterator<Item = PSN_R> + '_ {
        (0..4).map(move |n| PSN_R::new(((self.bits >> (n * 5)) & 0x1f) as u8))
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSQ")
            .field("pclen", &self.pclen())
            .field("psn1", &self.psn1())
            .field("psn2", &self.psn2())
            .field("psn3", &self.psn3())
            .field("psn4", &self.psn4())
            .finish()
    }
}
impl W {
    #[doc = "Number of (1-4) conversion in preempted sequence"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `PSN1` field.</div>"]
    #[inline(always)]
    pub fn psn(&mut self, n: u8) -> PSN_W<'_, PSQ_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        PSN_W::new(self, n * 5)
    }
    #[doc = "Bits 0:4 - Number of 1 conversion in preempted sequence"]
    #[inline(always)]
    pub fn psn1(&mut self) -> PSN_W<'_, PSQ_SPEC> {
        PSN_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - Number of 2 conversion in preempted sequence"]
    #[inline(always)]
    pub fn psn2(&mut self) -> PSN_W<'_, PSQ_SPEC> {
        PSN_W::new(self, 5)
    }
    #[doc = "Bits 10:14 - Number of 3 conversion in preempted sequence"]
    #[inline(always)]
    pub fn psn3(&mut self) -> PSN_W<'_, PSQ_SPEC> {
        PSN_W::new(self, 10)
    }
    #[doc = "Bits 15:19 - Number of 4 conversion in preempted sequence"]
    #[inline(always)]
    pub fn psn4(&mut self) -> PSN_W<'_, PSQ_SPEC> {
        PSN_W::new(self, 15)
    }
    #[doc = "Bits 20:21 - Preempted conversion sequence length"]
    #[inline(always)]
    pub fn pclen(&mut self) -> PCLEN_W<'_, PSQ_SPEC> {
        PCLEN_W::new(self, 20)
    }
}
#[doc = "Preempted sequence register\n\nYou can [`read`](crate::Reg::read) this register and get [`psq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSQ_SPEC;
impl crate::RegisterSpec for PSQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psq::R`](R) reader structure"]
impl crate::Readable for PSQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psq::W`](W) writer structure"]
impl crate::Writable for PSQ_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSQ to value 0"]
impl crate::Resettable for PSQ_SPEC {}
