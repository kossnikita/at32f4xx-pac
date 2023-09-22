#[doc = "Register `SCR` writer"]
pub type W = crate::W<SCR_SPEC>;
#[doc = "Set bit %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOSB0W_AW {
    #[doc = "1: Set the corresponding ODT bit"]
    Set = 1,
}
impl From<IOSB0W_AW> for bool {
    #[inline(always)]
    fn from(variant: IOSB0W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOSB[0-15]` writer - Set bit %s"]
pub type IOSB_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O, IOSB0W_AW>;
impl<'a, REG, const O: u8> IOSB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set the corresponding ODT bit"]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(IOSB0W_AW::Set)
    }
}
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
impl W {
    #[doc = "Set bit [0-15]"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn iosb<const O: u8>(&mut self) -> IOSB_W<SCR_SPEC, O> {
        IOSB_W::new(self)
    }
    #[doc = "Bit 0 - Set bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn iosb0(&mut self) -> IOSB_W<SCR_SPEC, 0> {
        IOSB_W::new(self)
    }
    #[doc = "Bit 1 - Set bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn iosb1(&mut self) -> IOSB_W<SCR_SPEC, 1> {
        IOSB_W::new(self)
    }
    #[doc = "Bit 2 - Set bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn iosb2(&mut self) -> IOSB_W<SCR_SPEC, 2> {
        IOSB_W::new(self)
    }
    #[doc = "Bit 3 - Set bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn iosb3(&mut self) -> IOSB_W<SCR_SPEC, 3> {
        IOSB_W::new(self)
    }
    #[doc = "Bit 4 - Set bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn iosb4(&mut self) -> IOSB_W<SCR_SPEC, 4> {
        IOSB_W::new(self)
    }
    #[doc = "Bit 5 - Set bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn iosb5(&mut self) -> IOSB_W<SCR_SPEC, 5> {
        IOSB_W::new(self)
    }
    #[doc = "Bit 6 - Set bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn iosb6(&mut self) -> IOSB_W<SCR_SPEC, 6> {
        IOSB_W::new(self)
    }
    #[doc = "Bit 7 - Set bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn iosb7(&mut self) -> IOSB_W<SCR_SPEC, 7> {
        IOSB_W::new(self)
    }
    #[doc = "Bit 8 - Set bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn iosb8(&mut self) -> IOSB_W<SCR_SPEC, 8> {
        IOSB_W::new(self)
    }
    #[doc = "Bit 9 - Set bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn iosb9(&mut self) -> IOSB_W<SCR_SPEC, 9> {
        IOSB_W::new(self)
    }
    #[doc = "Bit 10 - Set bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn iosb10(&mut self) -> IOSB_W<SCR_SPEC, 10> {
        IOSB_W::new(self)
    }
    #[doc = "Bit 11 - Set bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn iosb11(&mut self) -> IOSB_W<SCR_SPEC, 11> {
        IOSB_W::new(self)
    }
    #[doc = "Bit 12 - Set bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn iosb12(&mut self) -> IOSB_W<SCR_SPEC, 12> {
        IOSB_W::new(self)
    }
    #[doc = "Bit 13 - Set bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn iosb13(&mut self) -> IOSB_W<SCR_SPEC, 13> {
        IOSB_W::new(self)
    }
    #[doc = "Bit 14 - Set bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn iosb14(&mut self) -> IOSB_W<SCR_SPEC, 14> {
        IOSB_W::new(self)
    }
    #[doc = "Bit 15 - Set bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn iosb15(&mut self) -> IOSB_W<SCR_SPEC, 15> {
        IOSB_W::new(self)
    }
    #[doc = "Clear bit [0-15]"]
    #[inline(always)]
    #[must_use]
    pub unsafe fn iocb<const O: u8>(&mut self) -> IOCB_W<SCR_SPEC, O> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 16 - Clear bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn iocb0(&mut self) -> IOCB_W<SCR_SPEC, 16> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 17 - Clear bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn iocb1(&mut self) -> IOCB_W<SCR_SPEC, 17> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 18 - Clear bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn iocb2(&mut self) -> IOCB_W<SCR_SPEC, 18> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 19 - Clear bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn iocb3(&mut self) -> IOCB_W<SCR_SPEC, 19> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 20 - Clear bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn iocb4(&mut self) -> IOCB_W<SCR_SPEC, 20> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 21 - Clear bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn iocb5(&mut self) -> IOCB_W<SCR_SPEC, 21> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 22 - Clear bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn iocb6(&mut self) -> IOCB_W<SCR_SPEC, 22> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 23 - Clear bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn iocb7(&mut self) -> IOCB_W<SCR_SPEC, 23> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 24 - Clear bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn iocb8(&mut self) -> IOCB_W<SCR_SPEC, 24> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 25 - Clear bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn iocb9(&mut self) -> IOCB_W<SCR_SPEC, 25> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 26 - Clear bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn iocb10(&mut self) -> IOCB_W<SCR_SPEC, 26> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 27 - Clear bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn iocb11(&mut self) -> IOCB_W<SCR_SPEC, 27> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 28 - Clear bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn iocb12(&mut self) -> IOCB_W<SCR_SPEC, 28> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 29 - Clear bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn iocb13(&mut self) -> IOCB_W<SCR_SPEC, 29> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 30 - Clear bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn iocb14(&mut self) -> IOCB_W<SCR_SPEC, 30> {
        IOCB_W::new(self)
    }
    #[doc = "Bit 31 - Clear bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn iocb15(&mut self) -> IOCB_W<SCR_SPEC, 31> {
        IOCB_W::new(self)
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0001_0001;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
