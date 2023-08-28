#[doc = "Register `CLR` writer"]
pub type W = crate::W<CLR_SPEC>;
#[doc = "Field `IOCB0` writer - Clear bit 0"]
pub type IOCB0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOCB1` writer - Clear bit 1"]
pub type IOCB1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOCB2` writer - Clear bit 2"]
pub type IOCB2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOCB3` writer - Clear bit 3"]
pub type IOCB3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOCB4` writer - Clear bit 4"]
pub type IOCB4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOCB5` writer - Clear bit 5"]
pub type IOCB5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOCB6` writer - Clear bit 6"]
pub type IOCB6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOCB7` writer - Clear bit 7"]
pub type IOCB7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOCB8` writer - Clear bit 8"]
pub type IOCB8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOCB9` writer - Clear bit 9"]
pub type IOCB9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOCB10` writer - Clear bit 10"]
pub type IOCB10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOCB11` writer - Clear bit 11"]
pub type IOCB11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOCB12` writer - Clear bit 12"]
pub type IOCB12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOCB13` writer - Clear bit 13"]
pub type IOCB13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOCB14` writer - Clear bit 14"]
pub type IOCB14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOCB15` writer - Clear bit 15"]
pub type IOCB15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Clear bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn iocb0(&mut self) -> IOCB0_W<CLR_SPEC, 0> {
        IOCB0_W::new(self)
    }
    #[doc = "Bit 1 - Clear bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn iocb1(&mut self) -> IOCB1_W<CLR_SPEC, 1> {
        IOCB1_W::new(self)
    }
    #[doc = "Bit 2 - Clear bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn iocb2(&mut self) -> IOCB2_W<CLR_SPEC, 2> {
        IOCB2_W::new(self)
    }
    #[doc = "Bit 3 - Clear bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn iocb3(&mut self) -> IOCB3_W<CLR_SPEC, 3> {
        IOCB3_W::new(self)
    }
    #[doc = "Bit 4 - Clear bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn iocb4(&mut self) -> IOCB4_W<CLR_SPEC, 4> {
        IOCB4_W::new(self)
    }
    #[doc = "Bit 5 - Clear bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn iocb5(&mut self) -> IOCB5_W<CLR_SPEC, 5> {
        IOCB5_W::new(self)
    }
    #[doc = "Bit 6 - Clear bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn iocb6(&mut self) -> IOCB6_W<CLR_SPEC, 6> {
        IOCB6_W::new(self)
    }
    #[doc = "Bit 7 - Clear bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn iocb7(&mut self) -> IOCB7_W<CLR_SPEC, 7> {
        IOCB7_W::new(self)
    }
    #[doc = "Bit 8 - Clear bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn iocb8(&mut self) -> IOCB8_W<CLR_SPEC, 8> {
        IOCB8_W::new(self)
    }
    #[doc = "Bit 9 - Clear bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn iocb9(&mut self) -> IOCB9_W<CLR_SPEC, 9> {
        IOCB9_W::new(self)
    }
    #[doc = "Bit 10 - Clear bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn iocb10(&mut self) -> IOCB10_W<CLR_SPEC, 10> {
        IOCB10_W::new(self)
    }
    #[doc = "Bit 11 - Clear bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn iocb11(&mut self) -> IOCB11_W<CLR_SPEC, 11> {
        IOCB11_W::new(self)
    }
    #[doc = "Bit 12 - Clear bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn iocb12(&mut self) -> IOCB12_W<CLR_SPEC, 12> {
        IOCB12_W::new(self)
    }
    #[doc = "Bit 13 - Clear bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn iocb13(&mut self) -> IOCB13_W<CLR_SPEC, 13> {
        IOCB13_W::new(self)
    }
    #[doc = "Bit 14 - Clear bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn iocb14(&mut self) -> IOCB14_W<CLR_SPEC, 14> {
        IOCB14_W::new(self)
    }
    #[doc = "Bit 15 - Clear bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn iocb15(&mut self) -> IOCB15_W<CLR_SPEC, 15> {
        IOCB15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPIO bit reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLR_SPEC;
impl crate::RegisterSpec for CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clr::W`](W) writer structure"]
impl crate::Writable for CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLR to value 0"]
impl crate::Resettable for CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
