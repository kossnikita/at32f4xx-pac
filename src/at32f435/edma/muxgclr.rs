#[doc = "Register `MUXGCLR` reader"]
pub type R = crate::R<MUXGCLR_SPEC>;
#[doc = "Register `MUXGCLR` writer"]
pub type W = crate::W<MUXGCLR_SPEC>;
#[doc = "Field `TRGOVFC1` reader - Clear trigger overrun interrupt flag"]
pub type TRGOVFC1_R = crate::BitReader;
#[doc = "Field `TRGOVFC1` writer - Clear trigger overrun interrupt flag"]
pub type TRGOVFC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGOVFC2` reader - Clear trigger overrun interrupt flag"]
pub type TRGOVFC2_R = crate::BitReader;
#[doc = "Field `TRGOVFC2` writer - Clear trigger overrun interrupt flag"]
pub type TRGOVFC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGOVFC3` reader - Clear trigger overrun interrupt flag"]
pub type TRGOVFC3_R = crate::BitReader;
#[doc = "Field `TRGOVFC3` writer - Clear trigger overrun interrupt flag"]
pub type TRGOVFC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGOVFC4` reader - Clear trigger overrun interrupt flag"]
pub type TRGOVFC4_R = crate::BitReader;
#[doc = "Field `TRGOVFC4` writer - Clear trigger overrun interrupt flag"]
pub type TRGOVFC4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear trigger overrun interrupt flag"]
    #[inline(always)]
    pub fn trgovfc1(&self) -> TRGOVFC1_R {
        TRGOVFC1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear trigger overrun interrupt flag"]
    #[inline(always)]
    pub fn trgovfc2(&self) -> TRGOVFC2_R {
        TRGOVFC2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear trigger overrun interrupt flag"]
    #[inline(always)]
    pub fn trgovfc3(&self) -> TRGOVFC3_R {
        TRGOVFC3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear trigger overrun interrupt flag"]
    #[inline(always)]
    pub fn trgovfc4(&self) -> TRGOVFC4_R {
        TRGOVFC4_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUXGCLR")
            .field("trgovfc1", &self.trgovfc1())
            .field("trgovfc2", &self.trgovfc2())
            .field("trgovfc3", &self.trgovfc3())
            .field("trgovfc4", &self.trgovfc4())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Clear trigger overrun interrupt flag"]
    #[inline(always)]
    pub fn trgovfc1(&mut self) -> TRGOVFC1_W<'_, MUXGCLR_SPEC> {
        TRGOVFC1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear trigger overrun interrupt flag"]
    #[inline(always)]
    pub fn trgovfc2(&mut self) -> TRGOVFC2_W<'_, MUXGCLR_SPEC> {
        TRGOVFC2_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear trigger overrun interrupt flag"]
    #[inline(always)]
    pub fn trgovfc3(&mut self) -> TRGOVFC3_W<'_, MUXGCLR_SPEC> {
        TRGOVFC3_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear trigger overrun interrupt flag"]
    #[inline(always)]
    pub fn trgovfc4(&mut self) -> TRGOVFC4_W<'_, MUXGCLR_SPEC> {
        TRGOVFC4_W::new(self, 3)
    }
}
#[doc = "Generator Interrupt Clear Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxgclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxgclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MUXGCLR_SPEC;
impl crate::RegisterSpec for MUXGCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxgclr::R`](R) reader structure"]
impl crate::Readable for MUXGCLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`muxgclr::W`](W) writer structure"]
impl crate::Writable for MUXGCLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MUXGCLR to value 0"]
impl crate::Resettable for MUXGCLR_SPEC {}
