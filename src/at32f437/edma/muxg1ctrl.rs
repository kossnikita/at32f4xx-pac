#[doc = "Register `MUXG1CTRL` reader"]
pub type R = crate::R<MUXG1CTRL_SPEC>;
#[doc = "Register `MUXG1CTRL` writer"]
pub type W = crate::W<MUXG1CTRL_SPEC>;
#[doc = "Field `SIGSEL` reader - Signal select"]
pub type SIGSEL_R = crate::FieldReader;
#[doc = "Field `SIGSEL` writer - Signal select"]
pub type SIGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRGOVIEN` reader - Trigger overrun interrupt enable"]
pub type TRGOVIEN_R = crate::BitReader;
#[doc = "Field `TRGOVIEN` writer - Trigger overrun interrupt enable"]
pub type TRGOVIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN` reader - DMA request generator enable"]
pub type GEN_R = crate::BitReader;
#[doc = "Field `GEN` writer - DMA request generator enable"]
pub type GEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPOL` reader - DMA request generator trigger polarity"]
pub type GPOL_R = crate::FieldReader;
#[doc = "Field `GPOL` writer - DMA request generator trigger polarity"]
pub type GPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GREQCNT` reader - Number of DMA requests to be generated"]
pub type GREQCNT_R = crate::FieldReader;
#[doc = "Field `GREQCNT` writer - Number of DMA requests to be generated"]
pub type GREQCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Signal select"]
    #[inline(always)]
    pub fn sigsel(&self) -> SIGSEL_R {
        SIGSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Trigger overrun interrupt enable"]
    #[inline(always)]
    pub fn trgovien(&self) -> TRGOVIEN_R {
        TRGOVIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - DMA request generator enable"]
    #[inline(always)]
    pub fn gen(&self) -> GEN_R {
        GEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger polarity"]
    #[inline(always)]
    pub fn gpol(&self) -> GPOL_R {
        GPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to be generated"]
    #[inline(always)]
    pub fn greqcnt(&self) -> GREQCNT_R {
        GREQCNT_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUXG1CTRL")
            .field("sigsel", &self.sigsel())
            .field("trgovien", &self.trgovien())
            .field("gen", &self.gen())
            .field("gpol", &self.gpol())
            .field("greqcnt", &self.greqcnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Signal select"]
    #[inline(always)]
    #[must_use]
    pub fn sigsel(&mut self) -> SIGSEL_W<MUXG1CTRL_SPEC> {
        SIGSEL_W::new(self, 0)
    }
    #[doc = "Bit 8 - Trigger overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgovien(&mut self) -> TRGOVIEN_W<MUXG1CTRL_SPEC> {
        TRGOVIEN_W::new(self, 8)
    }
    #[doc = "Bit 16 - DMA request generator enable"]
    #[inline(always)]
    #[must_use]
    pub fn gen(&mut self) -> GEN_W<MUXG1CTRL_SPEC> {
        GEN_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger polarity"]
    #[inline(always)]
    #[must_use]
    pub fn gpol(&mut self) -> GPOL_W<MUXG1CTRL_SPEC> {
        GPOL_W::new(self, 17)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn greqcnt(&mut self) -> GREQCNT_W<MUXG1CTRL_SPEC> {
        GREQCNT_W::new(self, 19)
    }
}
#[doc = "Generator 1 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxg1ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxg1ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MUXG1CTRL_SPEC;
impl crate::RegisterSpec for MUXG1CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxg1ctrl::R`](R) reader structure"]
impl crate::Readable for MUXG1CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`muxg1ctrl::W`](W) writer structure"]
impl crate::Writable for MUXG1CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MUXG1CTRL to value 0"]
impl crate::Resettable for MUXG1CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
