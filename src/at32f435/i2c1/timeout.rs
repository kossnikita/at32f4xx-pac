#[doc = "Register `TIMEOUT` reader"]
pub type R = crate::R<TIMEOUT_SPEC>;
#[doc = "Register `TIMEOUT` writer"]
pub type W = crate::W<TIMEOUT_SPEC>;
#[doc = "Field `TOTIME` reader - Clock timeout detection time"]
pub type TOTIME_R = crate::FieldReader<u16>;
#[doc = "Field `TOTIME` writer - Clock timeout detection time"]
pub type TOTIME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `TOMOED` reader - Clock timeout detection mode"]
pub type TOMOED_R = crate::BitReader;
#[doc = "Field `TOMOED` writer - Clock timeout detection mode"]
pub type TOMOED_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TOEN` reader - Detect clock low/high timeout enable"]
pub type TOEN_R = crate::BitReader;
#[doc = "Field `TOEN` writer - Detect clock low/high timeout enable"]
pub type TOEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `EXTTIME` reader - Cumulative clock low extend timeout value"]
pub type EXTTIME_R = crate::FieldReader<u16>;
#[doc = "Field `EXTTIME` writer - Cumulative clock low extend timeout value"]
pub type EXTTIME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `EXTEN` reader - Cumulative clock low extend timeout enable"]
pub type EXTEN_R = crate::BitReader;
#[doc = "Field `EXTEN` writer - Cumulative clock low extend timeout enable"]
pub type EXTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:11 - Clock timeout detection time"]
    #[inline(always)]
    pub fn totime(&self) -> TOTIME_R {
        TOTIME_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Clock timeout detection mode"]
    #[inline(always)]
    pub fn tomoed(&self) -> TOMOED_R {
        TOMOED_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Detect clock low/high timeout enable"]
    #[inline(always)]
    pub fn toen(&self) -> TOEN_R {
        TOEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:27 - Cumulative clock low extend timeout value"]
    #[inline(always)]
    pub fn exttime(&self) -> EXTTIME_R {
        EXTTIME_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Cumulative clock low extend timeout enable"]
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMEOUT")
            .field("totime", &format_args!("{}", self.totime().bits()))
            .field("tomoed", &format_args!("{}", self.tomoed().bit()))
            .field("toen", &format_args!("{}", self.toen().bit()))
            .field("exttime", &format_args!("{}", self.exttime().bits()))
            .field("exten", &format_args!("{}", self.exten().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<TIMEOUT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:11 - Clock timeout detection time"]
    #[inline(always)]
    #[must_use]
    pub fn totime(&mut self) -> TOTIME_W<TIMEOUT_SPEC, 0> {
        TOTIME_W::new(self)
    }
    #[doc = "Bit 12 - Clock timeout detection mode"]
    #[inline(always)]
    #[must_use]
    pub fn tomoed(&mut self) -> TOMOED_W<TIMEOUT_SPEC, 12> {
        TOMOED_W::new(self)
    }
    #[doc = "Bit 15 - Detect clock low/high timeout enable"]
    #[inline(always)]
    #[must_use]
    pub fn toen(&mut self) -> TOEN_W<TIMEOUT_SPEC, 15> {
        TOEN_W::new(self)
    }
    #[doc = "Bits 16:27 - Cumulative clock low extend timeout value"]
    #[inline(always)]
    #[must_use]
    pub fn exttime(&mut self) -> EXTTIME_W<TIMEOUT_SPEC, 16> {
        EXTTIME_W::new(self)
    }
    #[doc = "Bit 31 - Cumulative clock low extend timeout enable"]
    #[inline(always)]
    #[must_use]
    pub fn exten(&mut self) -> EXTEN_W<TIMEOUT_SPEC, 31> {
        EXTEN_W::new(self)
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
#[doc = "Timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timeout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMEOUT_SPEC;
impl crate::RegisterSpec for TIMEOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timeout::R`](R) reader structure"]
impl crate::Readable for TIMEOUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timeout::W`](W) writer structure"]
impl crate::Writable for TIMEOUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMEOUT to value 0"]
impl crate::Resettable for TIMEOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
