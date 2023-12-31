#[doc = "Register `SOFRNUM` reader"]
pub type R = crate::R<SOFRNUM_SPEC>;
#[doc = "Register `SOFRNUM` writer"]
pub type W = crate::W<SOFRNUM_SPEC>;
#[doc = "Field `SOFNUM` reader - Start of frame number"]
pub type SOFNUM_R = crate::FieldReader<u16>;
#[doc = "Field `SOFNUM` writer - Start of frame number"]
pub type SOFNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `LSOFNUM` reader - Lost start of frame number"]
pub type LSOFNUM_R = crate::FieldReader;
#[doc = "Field `LSOFNUM` writer - Lost start of frame number"]
pub type LSOFNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLCK` reader - Connect locked"]
pub type CLCK_R = crate::BitReader;
#[doc = "Field `CLCK` writer - Connect locked"]
pub type CLCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMSTS` reader - DM status"]
pub type DMSTS_R = crate::BitReader;
#[doc = "Field `DMSTS` writer - DM status"]
pub type DMSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPSTS` reader - DP status"]
pub type DPSTS_R = crate::BitReader;
#[doc = "Field `DPSTS` writer - DP status"]
pub type DPSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Start of frame number"]
    #[inline(always)]
    pub fn sofnum(&self) -> SOFNUM_R {
        SOFNUM_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:12 - Lost start of frame number"]
    #[inline(always)]
    pub fn lsofnum(&self) -> LSOFNUM_R {
        LSOFNUM_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - Connect locked"]
    #[inline(always)]
    pub fn clck(&self) -> CLCK_R {
        CLCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DM status"]
    #[inline(always)]
    pub fn dmsts(&self) -> DMSTS_R {
        DMSTS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DP status"]
    #[inline(always)]
    pub fn dpsts(&self) -> DPSTS_R {
        DPSTS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SOFRNUM")
            .field("sofnum", &format_args!("{}", self.sofnum().bits()))
            .field("lsofnum", &format_args!("{}", self.lsofnum().bits()))
            .field("clck", &format_args!("{}", self.clck().bit()))
            .field("dmsts", &format_args!("{}", self.dmsts().bit()))
            .field("dpsts", &format_args!("{}", self.dpsts().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<SOFRNUM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:10 - Start of frame number"]
    #[inline(always)]
    #[must_use]
    pub fn sofnum(&mut self) -> SOFNUM_W<SOFRNUM_SPEC> {
        SOFNUM_W::new(self, 0)
    }
    #[doc = "Bits 11:12 - Lost start of frame number"]
    #[inline(always)]
    #[must_use]
    pub fn lsofnum(&mut self) -> LSOFNUM_W<SOFRNUM_SPEC> {
        LSOFNUM_W::new(self, 11)
    }
    #[doc = "Bit 13 - Connect locked"]
    #[inline(always)]
    #[must_use]
    pub fn clck(&mut self) -> CLCK_W<SOFRNUM_SPEC> {
        CLCK_W::new(self, 13)
    }
    #[doc = "Bit 14 - DM status"]
    #[inline(always)]
    #[must_use]
    pub fn dmsts(&mut self) -> DMSTS_W<SOFRNUM_SPEC> {
        DMSTS_W::new(self, 14)
    }
    #[doc = "Bit 15 - DP status"]
    #[inline(always)]
    #[must_use]
    pub fn dpsts(&mut self) -> DPSTS_W<SOFRNUM_SPEC> {
        DPSTS_W::new(self, 15)
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
#[doc = "frame number register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sofrnum::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sofrnum::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOFRNUM_SPEC;
impl crate::RegisterSpec for SOFRNUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sofrnum::R`](R) reader structure"]
impl crate::Readable for SOFRNUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sofrnum::W`](W) writer structure"]
impl crate::Writable for SOFRNUM_SPEC {
    const ZEROS_BITMAP: Self::Ux = 0;
    const ONES_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOFRNUM to value 0"]
impl crate::Resettable for SOFRNUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
