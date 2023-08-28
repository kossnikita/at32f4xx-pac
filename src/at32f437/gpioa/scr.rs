#[doc = "Register `SCR` writer"]
pub type W = crate::W<SCR_SPEC>;
#[doc = "Field `IOSB0` writer - Set bit 0"]
pub type IOSB0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOSB1` writer - Set bit 1"]
pub type IOSB1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOSB2` writer - Set bit 1"]
pub type IOSB2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOSB3` writer - Set bit 3"]
pub type IOSB3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOSB4` writer - Set bit 4"]
pub type IOSB4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOSB5` writer - Set bit 5"]
pub type IOSB5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOSB6` writer - Set bit 6"]
pub type IOSB6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOSB7` writer - Set bit 7"]
pub type IOSB7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOSB8` writer - Set bit 8"]
pub type IOSB8_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOSB9` writer - Set bit 9"]
pub type IOSB9_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOSB10` writer - Set bit 10"]
pub type IOSB10_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOSB11` writer - Set bit 11"]
pub type IOSB11_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOSB12` writer - Set bit 12"]
pub type IOSB12_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOSB13` writer - Set bit 13"]
pub type IOSB13_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOSB14` writer - Set bit 14"]
pub type IOSB14_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `IOSB15` writer - Set bit 15"]
pub type IOSB15_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[doc = "Bit 0 - Set bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn iosb0(&mut self) -> IOSB0_W<SCR_SPEC, 0> {
        IOSB0_W::new(self)
    }
    #[doc = "Bit 1 - Set bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn iosb1(&mut self) -> IOSB1_W<SCR_SPEC, 1> {
        IOSB1_W::new(self)
    }
    #[doc = "Bit 2 - Set bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn iosb2(&mut self) -> IOSB2_W<SCR_SPEC, 2> {
        IOSB2_W::new(self)
    }
    #[doc = "Bit 3 - Set bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn iosb3(&mut self) -> IOSB3_W<SCR_SPEC, 3> {
        IOSB3_W::new(self)
    }
    #[doc = "Bit 4 - Set bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn iosb4(&mut self) -> IOSB4_W<SCR_SPEC, 4> {
        IOSB4_W::new(self)
    }
    #[doc = "Bit 5 - Set bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn iosb5(&mut self) -> IOSB5_W<SCR_SPEC, 5> {
        IOSB5_W::new(self)
    }
    #[doc = "Bit 6 - Set bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn iosb6(&mut self) -> IOSB6_W<SCR_SPEC, 6> {
        IOSB6_W::new(self)
    }
    #[doc = "Bit 7 - Set bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn iosb7(&mut self) -> IOSB7_W<SCR_SPEC, 7> {
        IOSB7_W::new(self)
    }
    #[doc = "Bit 8 - Set bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn iosb8(&mut self) -> IOSB8_W<SCR_SPEC, 8> {
        IOSB8_W::new(self)
    }
    #[doc = "Bit 9 - Set bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn iosb9(&mut self) -> IOSB9_W<SCR_SPEC, 9> {
        IOSB9_W::new(self)
    }
    #[doc = "Bit 10 - Set bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn iosb10(&mut self) -> IOSB10_W<SCR_SPEC, 10> {
        IOSB10_W::new(self)
    }
    #[doc = "Bit 11 - Set bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn iosb11(&mut self) -> IOSB11_W<SCR_SPEC, 11> {
        IOSB11_W::new(self)
    }
    #[doc = "Bit 12 - Set bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn iosb12(&mut self) -> IOSB12_W<SCR_SPEC, 12> {
        IOSB12_W::new(self)
    }
    #[doc = "Bit 13 - Set bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn iosb13(&mut self) -> IOSB13_W<SCR_SPEC, 13> {
        IOSB13_W::new(self)
    }
    #[doc = "Bit 14 - Set bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn iosb14(&mut self) -> IOSB14_W<SCR_SPEC, 14> {
        IOSB14_W::new(self)
    }
    #[doc = "Bit 15 - Set bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn iosb15(&mut self) -> IOSB15_W<SCR_SPEC, 15> {
        IOSB15_W::new(self)
    }
    #[doc = "Bit 16 - Clear bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn iocb0(&mut self) -> IOCB0_W<SCR_SPEC, 16> {
        IOCB0_W::new(self)
    }
    #[doc = "Bit 17 - Clear bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn iocb1(&mut self) -> IOCB1_W<SCR_SPEC, 17> {
        IOCB1_W::new(self)
    }
    #[doc = "Bit 18 - Clear bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn iocb2(&mut self) -> IOCB2_W<SCR_SPEC, 18> {
        IOCB2_W::new(self)
    }
    #[doc = "Bit 19 - Clear bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn iocb3(&mut self) -> IOCB3_W<SCR_SPEC, 19> {
        IOCB3_W::new(self)
    }
    #[doc = "Bit 20 - Clear bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn iocb4(&mut self) -> IOCB4_W<SCR_SPEC, 20> {
        IOCB4_W::new(self)
    }
    #[doc = "Bit 21 - Clear bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn iocb5(&mut self) -> IOCB5_W<SCR_SPEC, 21> {
        IOCB5_W::new(self)
    }
    #[doc = "Bit 22 - Clear bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn iocb6(&mut self) -> IOCB6_W<SCR_SPEC, 22> {
        IOCB6_W::new(self)
    }
    #[doc = "Bit 23 - Clear bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn iocb7(&mut self) -> IOCB7_W<SCR_SPEC, 23> {
        IOCB7_W::new(self)
    }
    #[doc = "Bit 24 - Clear bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn iocb8(&mut self) -> IOCB8_W<SCR_SPEC, 24> {
        IOCB8_W::new(self)
    }
    #[doc = "Bit 25 - Clear bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn iocb9(&mut self) -> IOCB9_W<SCR_SPEC, 25> {
        IOCB9_W::new(self)
    }
    #[doc = "Bit 26 - Clear bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn iocb10(&mut self) -> IOCB10_W<SCR_SPEC, 26> {
        IOCB10_W::new(self)
    }
    #[doc = "Bit 27 - Clear bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn iocb11(&mut self) -> IOCB11_W<SCR_SPEC, 27> {
        IOCB11_W::new(self)
    }
    #[doc = "Bit 28 - Clear bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn iocb12(&mut self) -> IOCB12_W<SCR_SPEC, 28> {
        IOCB12_W::new(self)
    }
    #[doc = "Bit 29 - Clear bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn iocb13(&mut self) -> IOCB13_W<SCR_SPEC, 29> {
        IOCB13_W::new(self)
    }
    #[doc = "Bit 30 - Clear bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn iocb14(&mut self) -> IOCB14_W<SCR_SPEC, 30> {
        IOCB14_W::new(self)
    }
    #[doc = "Bit 31 - Clear bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn iocb15(&mut self) -> IOCB15_W<SCR_SPEC, 31> {
        IOCB15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port bit set/clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
