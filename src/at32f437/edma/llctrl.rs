#[doc = "Register `LLCTRL` reader"]
pub type R = crate::R<LLCTRL_SPEC>;
#[doc = "Register `LLCTRL` writer"]
pub type W = crate::W<LLCTRL_SPEC>;
#[doc = "Field `S1LLEN` reader - Stream 1 link list enable"]
pub type S1LLEN_R = crate::BitReader;
#[doc = "Field `S1LLEN` writer - Stream 1 link list enable"]
pub type S1LLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S2LLEN` reader - Stream 2 link list enable"]
pub type S2LLEN_R = crate::BitReader;
#[doc = "Field `S2LLEN` writer - Stream 2 link list enable"]
pub type S2LLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S3LLEN` reader - Stream 3 link list enable"]
pub type S3LLEN_R = crate::BitReader;
#[doc = "Field `S3LLEN` writer - Stream 3 link list enable"]
pub type S3LLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S4LLEN` reader - Stream 4 link list enable"]
pub type S4LLEN_R = crate::BitReader;
#[doc = "Field `S4LLEN` writer - Stream 4 link list enable"]
pub type S4LLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S5LLEN` reader - Stream 5 link list enable"]
pub type S5LLEN_R = crate::BitReader;
#[doc = "Field `S5LLEN` writer - Stream 5 link list enable"]
pub type S5LLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S6LLEN` reader - Stream 6 link list enable"]
pub type S6LLEN_R = crate::BitReader;
#[doc = "Field `S6LLEN` writer - Stream 6 link list enable"]
pub type S6LLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S7LLEN` reader - Stream 7 link list enable"]
pub type S7LLEN_R = crate::BitReader;
#[doc = "Field `S7LLEN` writer - Stream 7 link list enable"]
pub type S7LLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S8LLEN` reader - Stream 8 link list enable"]
pub type S8LLEN_R = crate::BitReader;
#[doc = "Field `S8LLEN` writer - Stream 8 link list enable"]
pub type S8LLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stream 1 link list enable"]
    #[inline(always)]
    pub fn s1llen(&self) -> S1LLEN_R {
        S1LLEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stream 2 link list enable"]
    #[inline(always)]
    pub fn s2llen(&self) -> S2LLEN_R {
        S2LLEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stream 3 link list enable"]
    #[inline(always)]
    pub fn s3llen(&self) -> S3LLEN_R {
        S3LLEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stream 4 link list enable"]
    #[inline(always)]
    pub fn s4llen(&self) -> S4LLEN_R {
        S4LLEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stream 5 link list enable"]
    #[inline(always)]
    pub fn s5llen(&self) -> S5LLEN_R {
        S5LLEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stream 6 link list enable"]
    #[inline(always)]
    pub fn s6llen(&self) -> S6LLEN_R {
        S6LLEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stream 7 link list enable"]
    #[inline(always)]
    pub fn s7llen(&self) -> S7LLEN_R {
        S7LLEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Stream 8 link list enable"]
    #[inline(always)]
    pub fn s8llen(&self) -> S8LLEN_R {
        S8LLEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LLCTRL")
            .field("s1llen", &self.s1llen())
            .field("s2llen", &self.s2llen())
            .field("s3llen", &self.s3llen())
            .field("s4llen", &self.s4llen())
            .field("s5llen", &self.s5llen())
            .field("s6llen", &self.s6llen())
            .field("s7llen", &self.s7llen())
            .field("s8llen", &self.s8llen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Stream 1 link list enable"]
    #[inline(always)]
    #[must_use]
    pub fn s1llen(&mut self) -> S1LLEN_W<LLCTRL_SPEC> {
        S1LLEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Stream 2 link list enable"]
    #[inline(always)]
    #[must_use]
    pub fn s2llen(&mut self) -> S2LLEN_W<LLCTRL_SPEC> {
        S2LLEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Stream 3 link list enable"]
    #[inline(always)]
    #[must_use]
    pub fn s3llen(&mut self) -> S3LLEN_W<LLCTRL_SPEC> {
        S3LLEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Stream 4 link list enable"]
    #[inline(always)]
    #[must_use]
    pub fn s4llen(&mut self) -> S4LLEN_W<LLCTRL_SPEC> {
        S4LLEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Stream 5 link list enable"]
    #[inline(always)]
    #[must_use]
    pub fn s5llen(&mut self) -> S5LLEN_W<LLCTRL_SPEC> {
        S5LLEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Stream 6 link list enable"]
    #[inline(always)]
    #[must_use]
    pub fn s6llen(&mut self) -> S6LLEN_W<LLCTRL_SPEC> {
        S6LLEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Stream 7 link list enable"]
    #[inline(always)]
    #[must_use]
    pub fn s7llen(&mut self) -> S7LLEN_W<LLCTRL_SPEC> {
        S7LLEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - Stream 8 link list enable"]
    #[inline(always)]
    #[must_use]
    pub fn s8llen(&mut self) -> S8LLEN_W<LLCTRL_SPEC> {
        S8LLEN_W::new(self, 7)
    }
}
#[doc = "DMA Link List Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`llctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LLCTRL_SPEC;
impl crate::RegisterSpec for LLCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`llctrl::R`](R) reader structure"]
impl crate::Readable for LLCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`llctrl::W`](W) writer structure"]
impl crate::Writable for LLCTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LLCTRL to value 0"]
impl crate::Resettable for LLCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
