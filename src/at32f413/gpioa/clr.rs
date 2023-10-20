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
pub type IOCB_W<'a, REG> = crate::BitWriter1C<'a, REG, IOCB0W_AW>;
impl<'a, REG> IOCB_W<'a, REG>
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
    pub fn iocb(&mut self, n: u8) -> IOCB_W<CLR_SPEC> {
        assert!(n < 16);
        IOCB_W::new(self, n)
    }
    #[doc = "Bit 0 - Clear bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn iocb0(&mut self) -> IOCB_W<CLR_SPEC> {
        IOCB_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn iocb1(&mut self) -> IOCB_W<CLR_SPEC> {
        IOCB_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn iocb2(&mut self) -> IOCB_W<CLR_SPEC> {
        IOCB_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn iocb3(&mut self) -> IOCB_W<CLR_SPEC> {
        IOCB_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn iocb4(&mut self) -> IOCB_W<CLR_SPEC> {
        IOCB_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn iocb5(&mut self) -> IOCB_W<CLR_SPEC> {
        IOCB_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn iocb6(&mut self) -> IOCB_W<CLR_SPEC> {
        IOCB_W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn iocb7(&mut self) -> IOCB_W<CLR_SPEC> {
        IOCB_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn iocb8(&mut self) -> IOCB_W<CLR_SPEC> {
        IOCB_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn iocb9(&mut self) -> IOCB_W<CLR_SPEC> {
        IOCB_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn iocb10(&mut self) -> IOCB_W<CLR_SPEC> {
        IOCB_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn iocb11(&mut self) -> IOCB_W<CLR_SPEC> {
        IOCB_W::new(self, 11)
    }
    #[doc = "Bit 12 - Clear bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn iocb12(&mut self) -> IOCB_W<CLR_SPEC> {
        IOCB_W::new(self, 12)
    }
    #[doc = "Bit 13 - Clear bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn iocb13(&mut self) -> IOCB_W<CLR_SPEC> {
        IOCB_W::new(self, 13)
    }
    #[doc = "Bit 14 - Clear bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn iocb14(&mut self) -> IOCB_W<CLR_SPEC> {
        IOCB_W::new(self, 14)
    }
    #[doc = "Bit 15 - Clear bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn iocb15(&mut self) -> IOCB_W<CLR_SPEC> {
        IOCB_W::new(self, 15)
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
#[doc = "Port bit reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLR_SPEC;
impl crate::RegisterSpec for CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clr::W`](W) writer structure"]
impl crate::Writable for CLR_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0x01;
}
#[doc = "`reset()` method sets CLR to value 0"]
impl crate::Resettable for CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
