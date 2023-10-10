#[doc = "Register `MUXG4CTRL` reader"]
pub type R = crate::R<MUXG4CTRL_SPEC>;
#[doc = "Register `MUXG4CTRL` writer"]
pub type W = crate::W<MUXG4CTRL_SPEC>;
#[doc = "Field `SIGSEL` reader - Signal select"]
pub type SIGSEL_R = crate::FieldReader;
#[doc = "Field `SIGSEL` writer - Signal select"]
pub type SIGSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `TRGOVIEN` reader - Trigger overrun interrupt enable"]
pub type TRGOVIEN_R = crate::BitReader;
#[doc = "Field `TRGOVIEN` writer - Trigger overrun interrupt enable"]
pub type TRGOVIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GEN` reader - DMA request generator enable"]
pub type GEN_R = crate::BitReader;
#[doc = "Field `GEN` writer - DMA request generator enable"]
pub type GEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `GPOL` reader - DMA request generator trigger polarity"]
pub type GPOL_R = crate::FieldReader;
#[doc = "Field `GPOL` writer - DMA request generator trigger polarity"]
pub type GPOL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `GREQCNT` reader - Number of DMA requests to be generated"]
pub type GREQCNT_R = crate::FieldReader;
#[doc = "Field `GREQCNT` writer - Number of DMA requests to be generated"]
pub type GREQCNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
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
        f.debug_struct("MUXG4CTRL")
            .field("sigsel", &format_args!("{}", self.sigsel().bits()))
            .field("trgovien", &format_args!("{}", self.trgovien().bit()))
            .field("gen", &format_args!("{}", self.gen().bit()))
            .field("gpol", &format_args!("{}", self.gpol().bits()))
            .field("greqcnt", &format_args!("{}", self.greqcnt().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<MUXG4CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Signal select"]
    #[inline(always)]
    #[must_use]
    pub fn sigsel(&mut self) -> SIGSEL_W<MUXG4CTRL_SPEC, 0> {
        SIGSEL_W::new(self)
    }
    #[doc = "Bit 8 - Trigger overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgovien(&mut self) -> TRGOVIEN_W<MUXG4CTRL_SPEC, 8> {
        TRGOVIEN_W::new(self)
    }
    #[doc = "Bit 16 - DMA request generator enable"]
    #[inline(always)]
    #[must_use]
    pub fn gen(&mut self) -> GEN_W<MUXG4CTRL_SPEC, 16> {
        GEN_W::new(self)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger polarity"]
    #[inline(always)]
    #[must_use]
    pub fn gpol(&mut self) -> GPOL_W<MUXG4CTRL_SPEC, 17> {
        GPOL_W::new(self)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn greqcnt(&mut self) -> GREQCNT_W<MUXG4CTRL_SPEC, 19> {
        GREQCNT_W::new(self)
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
#[doc = "Generator 4 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxg4ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxg4ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MUXG4CTRL_SPEC;
impl crate::RegisterSpec for MUXG4CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxg4ctrl::R`](R) reader structure"]
impl crate::Readable for MUXG4CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`muxg4ctrl::W`](W) writer structure"]
impl crate::Writable for MUXG4CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MUXG4CTRL to value 0"]
impl crate::Resettable for MUXG4CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
