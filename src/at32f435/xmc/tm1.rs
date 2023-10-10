#[doc = "Register `TM1` reader"]
pub type R = crate::R<TM1_SPEC>;
#[doc = "Register `TM1` writer"]
pub type W = crate::W<TM1_SPEC>;
#[doc = "Field `TMRD` reader - Mode register program to active delay"]
pub type TMRD_R = crate::FieldReader;
#[doc = "Field `TMRD` writer - Mode register program to active delay"]
pub type TMRD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `TXSR` reader - Exit Self-refresh to active delay"]
pub type TXSR_R = crate::FieldReader;
#[doc = "Field `TXSR` writer - Exit Self-refresh to active delay"]
pub type TXSR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `TRAS` reader - Self refresh time"]
pub type TRAS_R = crate::FieldReader;
#[doc = "Field `TRAS` writer - Self refresh time"]
pub type TRAS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `TRC` reader - Refresh to active delay"]
pub type TRC_R = crate::FieldReader;
#[doc = "Field `TRC` writer - Refresh to active delay"]
pub type TRC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `TWR` reader - Write Recovery delay"]
pub type TWR_R = crate::FieldReader;
#[doc = "Field `TWR` writer - Write Recovery delay"]
pub type TWR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `TRP` reader - Precharge to active delay"]
pub type TRP_R = crate::FieldReader;
#[doc = "Field `TRP` writer - Precharge to active delay"]
pub type TRP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `TRCD` reader - Row active to Read/Write delay"]
pub type TRCD_R = crate::FieldReader;
#[doc = "Field `TRCD` writer - Row active to Read/Write delay"]
pub type TRCD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Mode register program to active delay"]
    #[inline(always)]
    pub fn tmrd(&self) -> TMRD_R {
        TMRD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Exit Self-refresh to active delay"]
    #[inline(always)]
    pub fn txsr(&self) -> TXSR_R {
        TXSR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Self refresh time"]
    #[inline(always)]
    pub fn tras(&self) -> TRAS_R {
        TRAS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Refresh to active delay"]
    #[inline(always)]
    pub fn trc(&self) -> TRC_R {
        TRC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Write Recovery delay"]
    #[inline(always)]
    pub fn twr(&self) -> TWR_R {
        TWR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Precharge to active delay"]
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Row active to Read/Write delay"]
    #[inline(always)]
    pub fn trcd(&self) -> TRCD_R {
        TRCD_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TM1")
            .field("tmrd", &format_args!("{}", self.tmrd().bits()))
            .field("txsr", &format_args!("{}", self.txsr().bits()))
            .field("tras", &format_args!("{}", self.tras().bits()))
            .field("trc", &format_args!("{}", self.trc().bits()))
            .field("twr", &format_args!("{}", self.twr().bits()))
            .field("trp", &format_args!("{}", self.trp().bits()))
            .field("trcd", &format_args!("{}", self.trcd().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<TM1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Mode register program to active delay"]
    #[inline(always)]
    #[must_use]
    pub fn tmrd(&mut self) -> TMRD_W<TM1_SPEC, 0> {
        TMRD_W::new(self)
    }
    #[doc = "Bits 4:7 - Exit Self-refresh to active delay"]
    #[inline(always)]
    #[must_use]
    pub fn txsr(&mut self) -> TXSR_W<TM1_SPEC, 4> {
        TXSR_W::new(self)
    }
    #[doc = "Bits 8:11 - Self refresh time"]
    #[inline(always)]
    #[must_use]
    pub fn tras(&mut self) -> TRAS_W<TM1_SPEC, 8> {
        TRAS_W::new(self)
    }
    #[doc = "Bits 12:15 - Refresh to active delay"]
    #[inline(always)]
    #[must_use]
    pub fn trc(&mut self) -> TRC_W<TM1_SPEC, 12> {
        TRC_W::new(self)
    }
    #[doc = "Bits 16:19 - Write Recovery delay"]
    #[inline(always)]
    #[must_use]
    pub fn twr(&mut self) -> TWR_W<TM1_SPEC, 16> {
        TWR_W::new(self)
    }
    #[doc = "Bits 20:23 - Precharge to active delay"]
    #[inline(always)]
    #[must_use]
    pub fn trp(&mut self) -> TRP_W<TM1_SPEC, 20> {
        TRP_W::new(self)
    }
    #[doc = "Bits 24:27 - Row active to Read/Write delay"]
    #[inline(always)]
    #[must_use]
    pub fn trcd(&mut self) -> TRCD_W<TM1_SPEC, 24> {
        TRCD_W::new(self)
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
#[doc = "SDRAM Timing register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tm1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tm1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TM1_SPEC;
impl crate::RegisterSpec for TM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tm1::R`](R) reader structure"]
impl crate::Readable for TM1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tm1::W`](W) writer structure"]
impl crate::Writable for TM1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TM1 to value 0x0fff_ffff"]
impl crate::Resettable for TM1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff_ffff;
}
