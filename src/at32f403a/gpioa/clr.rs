#[doc = "Register `CLR` writer"]
pub type W = crate::W<CLR_SPEC>;
#[doc = "Clear bit %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOCB0W_AW {
    #[doc = "1: Clear the corresponding ODT bit"]
    Clear = 1,
}
impl From<IOCB0W_AW> for bool {
    #[inline(always)]
    fn from(variant: IOCB0W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOCB[0-15]` writer - Clear bit %s"]
pub type IOCB_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O, IOCB0W_AW>;
impl<'a, REG, const O: u8> IOCB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the corresponding ODT bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(IOCB0W_AW::Clear)
    }
}
impl core::fmt::Debug for crate::generic::Reg<CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Clear bit [0-15]"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn iocb<const O: u8>(&mut self) -> IOCB_W<CLR_SPEC, O> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 0 - Clear bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn iocb0(&mut self) -> IOCB_W<CLR_SPEC, 0> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 1 - Clear bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn iocb1(&mut self) -> IOCB_W<CLR_SPEC, 1> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 2 - Clear bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn iocb2(&mut self) -> IOCB_W<CLR_SPEC, 2> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 3 - Clear bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn iocb3(&mut self) -> IOCB_W<CLR_SPEC, 3> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 4 - Clear bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn iocb4(&mut self) -> IOCB_W<CLR_SPEC, 4> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 5 - Clear bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn iocb5(&mut self) -> IOCB_W<CLR_SPEC, 5> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 6 - Clear bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn iocb6(&mut self) -> IOCB_W<CLR_SPEC, 6> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 7 - Clear bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn iocb7(&mut self) -> IOCB_W<CLR_SPEC, 7> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 8 - Clear bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn iocb8(&mut self) -> IOCB_W<CLR_SPEC, 8> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 9 - Clear bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn iocb9(&mut self) -> IOCB_W<CLR_SPEC, 9> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 10 - Clear bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn iocb10(&mut self) -> IOCB_W<CLR_SPEC, 10> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 11 - Clear bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn iocb11(&mut self) -> IOCB_W<CLR_SPEC, 11> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 12 - Clear bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn iocb12(&mut self) -> IOCB_W<CLR_SPEC, 12> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 13 - Clear bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn iocb13(&mut self) -> IOCB_W<CLR_SPEC, 13> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 14 - Clear bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn iocb14(&mut self) -> IOCB_W<CLR_SPEC, 14> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 15 - Clear bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn iocb15(&mut self) -> IOCB_W<CLR_SPEC, 15> {
        IOCB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port bit reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLR_SPEC;
impl crate::RegisterSpec for CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clr::W`](W) writer structure"]
impl crate::Writable for CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
}
#[doc = "`reset()` method sets CLR to value 0"]
impl crate::Resettable for CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
