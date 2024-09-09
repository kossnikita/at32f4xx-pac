#[doc = "Register `SCR` writer"]
pub type W = crate::W<SCR_SPEC>;
#[doc = "Set bit %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOSB0W_A {
    #[doc = "1: Set the corresponding ODT bit"]
    Set = 1,
}
impl From<IOSB0W_A> for bool {
    #[inline(always)]
    fn from(variant: IOSB0W_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOSB(0-15)` writer - Set bit %s"]
pub type IOSB_W<'a, REG> = crate::BitWriter1S<'a, REG, IOSB0W_A>;
impl<'a, REG> IOSB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set the corresponding ODT bit"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(IOSB0W_A::Set)
    }
}
#[doc = "Clear bit %s\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOCB0W_A {
    #[doc = "1: Clear the corresponding ODT bit"]
    Clear = 1,
}
impl From<IOCB0W_A> for bool {
    #[inline(always)]
    fn from(variant: IOCB0W_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOCB(0-15)` writer - Clear bit %s"]
pub type IOCB_W<'a, REG> = crate::BitWriter1C<'a, REG, IOCB0W_A>;
impl<'a, REG> IOCB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the corresponding ODT bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(IOCB0W_A::Clear)
    }
}
impl core::fmt::Debug for crate::generic::Reg<SCR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Set bit (0-15)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `IOSB0` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn iosb(&mut self, n: u8) -> IOSB_W<SCR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        IOSB_W::new(self, n)
    }
    #[doc = "Bit 0 - Set bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn iosb0(&mut self) -> IOSB_W<SCR_SPEC> {
        IOSB_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn iosb1(&mut self) -> IOSB_W<SCR_SPEC> {
        IOSB_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn iosb2(&mut self) -> IOSB_W<SCR_SPEC> {
        IOSB_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn iosb3(&mut self) -> IOSB_W<SCR_SPEC> {
        IOSB_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn iosb4(&mut self) -> IOSB_W<SCR_SPEC> {
        IOSB_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn iosb5(&mut self) -> IOSB_W<SCR_SPEC> {
        IOSB_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn iosb6(&mut self) -> IOSB_W<SCR_SPEC> {
        IOSB_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn iosb7(&mut self) -> IOSB_W<SCR_SPEC> {
        IOSB_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn iosb8(&mut self) -> IOSB_W<SCR_SPEC> {
        IOSB_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn iosb9(&mut self) -> IOSB_W<SCR_SPEC> {
        IOSB_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn iosb10(&mut self) -> IOSB_W<SCR_SPEC> {
        IOSB_W::new(self, 10)
    }
    #[doc = "Bit 11 - Set bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn iosb11(&mut self) -> IOSB_W<SCR_SPEC> {
        IOSB_W::new(self, 11)
    }
    #[doc = "Bit 12 - Set bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn iosb12(&mut self) -> IOSB_W<SCR_SPEC> {
        IOSB_W::new(self, 12)
    }
    #[doc = "Bit 13 - Set bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn iosb13(&mut self) -> IOSB_W<SCR_SPEC> {
        IOSB_W::new(self, 13)
    }
    #[doc = "Bit 14 - Set bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn iosb14(&mut self) -> IOSB_W<SCR_SPEC> {
        IOSB_W::new(self, 14)
    }
    #[doc = "Bit 15 - Set bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn iosb15(&mut self) -> IOSB_W<SCR_SPEC> {
        IOSB_W::new(self, 15)
    }
    #[doc = "Clear bit (0-15)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `IOCB0` field.</div>"]
    #[inline(always)]
    #[must_use]
    pub fn iocb(&mut self, n: u8) -> IOCB_W<SCR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        IOCB_W::new(self, n + 16)
    }
    #[doc = "Bit 16 - Clear bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn iocb0(&mut self) -> IOCB_W<SCR_SPEC> {
        IOCB_W::new(self, 16)
    }
    #[doc = "Bit 17 - Clear bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn iocb1(&mut self) -> IOCB_W<SCR_SPEC> {
        IOCB_W::new(self, 17)
    }
    #[doc = "Bit 18 - Clear bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn iocb2(&mut self) -> IOCB_W<SCR_SPEC> {
        IOCB_W::new(self, 18)
    }
    #[doc = "Bit 19 - Clear bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn iocb3(&mut self) -> IOCB_W<SCR_SPEC> {
        IOCB_W::new(self, 19)
    }
    #[doc = "Bit 20 - Clear bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn iocb4(&mut self) -> IOCB_W<SCR_SPEC> {
        IOCB_W::new(self, 20)
    }
    #[doc = "Bit 21 - Clear bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn iocb5(&mut self) -> IOCB_W<SCR_SPEC> {
        IOCB_W::new(self, 21)
    }
    #[doc = "Bit 22 - Clear bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn iocb6(&mut self) -> IOCB_W<SCR_SPEC> {
        IOCB_W::new(self, 22)
    }
    #[doc = "Bit 23 - Clear bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn iocb7(&mut self) -> IOCB_W<SCR_SPEC> {
        IOCB_W::new(self, 23)
    }
    #[doc = "Bit 24 - Clear bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn iocb8(&mut self) -> IOCB_W<SCR_SPEC> {
        IOCB_W::new(self, 24)
    }
    #[doc = "Bit 25 - Clear bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn iocb9(&mut self) -> IOCB_W<SCR_SPEC> {
        IOCB_W::new(self, 25)
    }
    #[doc = "Bit 26 - Clear bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn iocb10(&mut self) -> IOCB_W<SCR_SPEC> {
        IOCB_W::new(self, 26)
    }
    #[doc = "Bit 27 - Clear bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn iocb11(&mut self) -> IOCB_W<SCR_SPEC> {
        IOCB_W::new(self, 27)
    }
    #[doc = "Bit 28 - Clear bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn iocb12(&mut self) -> IOCB_W<SCR_SPEC> {
        IOCB_W::new(self, 28)
    }
    #[doc = "Bit 29 - Clear bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn iocb13(&mut self) -> IOCB_W<SCR_SPEC> {
        IOCB_W::new(self, 29)
    }
    #[doc = "Bit 30 - Clear bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn iocb14(&mut self) -> IOCB_W<SCR_SPEC> {
        IOCB_W::new(self, 30)
    }
    #[doc = "Bit 31 - Clear bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn iocb15(&mut self) -> IOCB_W<SCR_SPEC> {
        IOCB_W::new(self, 31)
    }
}
#[doc = "Port bit set/clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0001_0001;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
