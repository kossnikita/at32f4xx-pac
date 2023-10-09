#[doc = "Register `S6FCTRL` reader"]
pub type R = crate::R<S6FCTRL_SPEC>;
#[doc = "Register `S6FCTRL` writer"]
pub type W = crate::W<S6FCTRL_SPEC>;
#[doc = "Field `FTHSEL` reader - FIFO threshold selection"]
pub type FTHSEL_R = crate::FieldReader;
#[doc = "Field `FTHSEL` writer - FIFO threshold selection"]
pub type FTHSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `FEN` reader - FIFO mode enable"]
pub type FEN_R = crate::BitReader;
#[doc = "Field `FEN` writer - FIFO mode enable"]
pub type FEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FSTS` reader - FIFO status"]
pub type FSTS_R = crate::FieldReader;
#[doc = "Field `FERRIEN` reader - FIFO error interrupt enable"]
pub type FERRIEN_R = crate::BitReader;
#[doc = "Field `FERRIEN` writer - FIFO error interrupt enable"]
pub type FERRIEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - FIFO threshold selection"]
    #[inline(always)]
    pub fn fthsel(&self) -> FTHSEL_R {
        FTHSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - FIFO mode enable"]
    #[inline(always)]
    pub fn fen(&self) -> FEN_R {
        FEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - FIFO status"]
    #[inline(always)]
    pub fn fsts(&self) -> FSTS_R {
        FSTS_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - FIFO error interrupt enable"]
    #[inline(always)]
    pub fn ferrien(&self) -> FERRIEN_R {
        FERRIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S6FCTRL")
            .field("ferrien", &format_args!("{}", self.ferrien().bit()))
            .field("fsts", &format_args!("{}", self.fsts().bits()))
            .field("fen", &format_args!("{}", self.fen().bit()))
            .field("fthsel", &format_args!("{}", self.fthsel().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<S6FCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - FIFO threshold selection"]
    #[inline(always)]
    #[must_use]
    pub fn fthsel(&mut self) -> FTHSEL_W<S6FCTRL_SPEC, 0> {
        FTHSEL_W::new(self)
    }
    #[doc = "Bit 2 - FIFO mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn fen(&mut self) -> FEN_W<S6FCTRL_SPEC, 2> {
        FEN_W::new(self)
    }
    #[doc = "Bit 7 - FIFO error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ferrien(&mut self) -> FERRIEN_W<S6FCTRL_SPEC, 7> {
        FERRIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "stream 6 FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s6fctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s6fctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S6FCTRL_SPEC;
impl crate::RegisterSpec for S6FCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s6fctrl::R`](R) reader structure"]
impl crate::Readable for S6FCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`s6fctrl::W`](W) writer structure"]
impl crate::Writable for S6FCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets S6FCTRL to value 0x21"]
impl crate::Resettable for S6FCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x21;
}
