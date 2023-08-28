#[doc = "Register `SOFRNUM` reader"]
pub type R = crate::R<SOFRNUM_SPEC>;
#[doc = "Register `SOFRNUM` writer"]
pub type W = crate::W<SOFRNUM_SPEC>;
#[doc = "Field `SOFNUM` reader - Start of frame number"]
pub type SOFNUM_R = crate::FieldReader<u16>;
#[doc = "Field `SOFNUM` writer - Start of frame number"]
pub type SOFNUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
#[doc = "Field `LSOFNUM` reader - Lost start of frame number"]
pub type LSOFNUM_R = crate::FieldReader;
#[doc = "Field `LSOFNUM` writer - Lost start of frame number"]
pub type LSOFNUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `CLCK` reader - Connect locked"]
pub type CLCK_R = crate::BitReader;
#[doc = "Field `CLCK` writer - Connect locked"]
pub type CLCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DMSTS` reader - DM status"]
pub type DMSTS_R = crate::BitReader;
#[doc = "Field `DMSTS` writer - DM status"]
pub type DMSTS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DPSTS` reader - DP status"]
pub type DPSTS_R = crate::BitReader;
#[doc = "Field `DPSTS` writer - DP status"]
pub type DPSTS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
impl W {
    #[doc = "Bits 0:10 - Start of frame number"]
    #[inline(always)]
    #[must_use]
    pub fn sofnum(&mut self) -> SOFNUM_W<SOFRNUM_SPEC, 0> {
        SOFNUM_W::new(self)
    }
    #[doc = "Bits 11:12 - Lost start of frame number"]
    #[inline(always)]
    #[must_use]
    pub fn lsofnum(&mut self) -> LSOFNUM_W<SOFRNUM_SPEC, 11> {
        LSOFNUM_W::new(self)
    }
    #[doc = "Bit 13 - Connect locked"]
    #[inline(always)]
    #[must_use]
    pub fn clck(&mut self) -> CLCK_W<SOFRNUM_SPEC, 13> {
        CLCK_W::new(self)
    }
    #[doc = "Bit 14 - DM status"]
    #[inline(always)]
    #[must_use]
    pub fn dmsts(&mut self) -> DMSTS_W<SOFRNUM_SPEC, 14> {
        DMSTS_W::new(self)
    }
    #[doc = "Bit 15 - DP status"]
    #[inline(always)]
    #[must_use]
    pub fn dpsts(&mut self) -> DPSTS_W<SOFRNUM_SPEC, 15> {
        DPSTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOFRNUM to value 0"]
impl crate::Resettable for SOFRNUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
