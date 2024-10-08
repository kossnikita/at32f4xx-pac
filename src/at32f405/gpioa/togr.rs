#[doc = "Register `TOGR` writer"]
pub type W = crate::W<TOGR_SPEC>;
#[doc = "Field `IOTB0` writer - toggle bit 0"]
pub type IOTB0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOTB1` writer - toggle bit 1"]
pub type IOTB1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOTB2` writer - toggle bit 2"]
pub type IOTB2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOTB3` writer - toggle bit 3"]
pub type IOTB3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOTB4` writer - toggle bit 4"]
pub type IOTB4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOTB5` writer - toggle bit 5"]
pub type IOTB5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOTB6` writer - toggle bit 6"]
pub type IOTB6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOTB7` writer - toggle bit 7"]
pub type IOTB7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOTB8` writer - toggle bit 8"]
pub type IOTB8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOTB9` writer - toggle bit 9"]
pub type IOTB9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOTB10` writer - toggle bit 10"]
pub type IOTB10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOTB11` writer - toggle bit 11"]
pub type IOTB11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOTB12` writer - toggle bit 12"]
pub type IOTB12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOTB13` writer - toggle bit 13"]
pub type IOTB13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOTB14` writer - toggle bit 14"]
pub type IOTB14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOTB15` writer - toggle bit 15"]
pub type IOTB15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<TOGR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - toggle bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn iotb0(&mut self) -> IOTB0_W<TOGR_SPEC> {
        IOTB0_W::new(self, 0)
    }
    #[doc = "Bit 1 - toggle bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn iotb1(&mut self) -> IOTB1_W<TOGR_SPEC> {
        IOTB1_W::new(self, 1)
    }
    #[doc = "Bit 2 - toggle bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn iotb2(&mut self) -> IOTB2_W<TOGR_SPEC> {
        IOTB2_W::new(self, 2)
    }
    #[doc = "Bit 3 - toggle bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn iotb3(&mut self) -> IOTB3_W<TOGR_SPEC> {
        IOTB3_W::new(self, 3)
    }
    #[doc = "Bit 4 - toggle bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn iotb4(&mut self) -> IOTB4_W<TOGR_SPEC> {
        IOTB4_W::new(self, 4)
    }
    #[doc = "Bit 5 - toggle bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn iotb5(&mut self) -> IOTB5_W<TOGR_SPEC> {
        IOTB5_W::new(self, 5)
    }
    #[doc = "Bit 6 - toggle bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn iotb6(&mut self) -> IOTB6_W<TOGR_SPEC> {
        IOTB6_W::new(self, 6)
    }
    #[doc = "Bit 7 - toggle bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn iotb7(&mut self) -> IOTB7_W<TOGR_SPEC> {
        IOTB7_W::new(self, 7)
    }
    #[doc = "Bit 8 - toggle bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn iotb8(&mut self) -> IOTB8_W<TOGR_SPEC> {
        IOTB8_W::new(self, 8)
    }
    #[doc = "Bit 9 - toggle bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn iotb9(&mut self) -> IOTB9_W<TOGR_SPEC> {
        IOTB9_W::new(self, 9)
    }
    #[doc = "Bit 10 - toggle bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn iotb10(&mut self) -> IOTB10_W<TOGR_SPEC> {
        IOTB10_W::new(self, 10)
    }
    #[doc = "Bit 11 - toggle bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn iotb11(&mut self) -> IOTB11_W<TOGR_SPEC> {
        IOTB11_W::new(self, 11)
    }
    #[doc = "Bit 12 - toggle bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn iotb12(&mut self) -> IOTB12_W<TOGR_SPEC> {
        IOTB12_W::new(self, 12)
    }
    #[doc = "Bit 13 - toggle bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn iotb13(&mut self) -> IOTB13_W<TOGR_SPEC> {
        IOTB13_W::new(self, 13)
    }
    #[doc = "Bit 14 - toggle bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn iotb14(&mut self) -> IOTB14_W<TOGR_SPEC> {
        IOTB14_W::new(self, 14)
    }
    #[doc = "Bit 15 - toggle bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn iotb15(&mut self) -> IOTB15_W<TOGR_SPEC> {
        IOTB15_W::new(self, 15)
    }
}
#[doc = "GPIO bit toggle register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`togr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOGR_SPEC;
impl crate::RegisterSpec for TOGR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`togr::W`](W) writer structure"]
impl crate::Writable for TOGR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOGR to value 0"]
impl crate::Resettable for TOGR_SPEC {
    const RESET_VALUE: u32 = 0;
}
