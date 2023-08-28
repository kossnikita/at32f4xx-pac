#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<CTRL2_SPEC>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<CTRL2_SPEC>;
#[doc = "Field `CBCTRL` reader - Channel buffer control"]
pub type CBCTRL_R = crate::BitReader;
#[doc = "Field `CBCTRL` writer - Channel buffer control"]
pub type CBCTRL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CCFS` reader - Channel control bit flash select"]
pub type CCFS_R = crate::BitReader;
#[doc = "Field `CCFS` writer - Channel control bit flash select"]
pub type CCFS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DRS` reader - DMA request source"]
pub type DRS_R = crate::BitReader;
#[doc = "Field `DRS` writer - DMA request source"]
pub type DRS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PTOS` reader - Primary TMR output selection"]
pub type PTOS_R = crate::FieldReader;
#[doc = "Field `PTOS` writer - Primary TMR output selection"]
pub type PTOS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `C1INSEL` reader - C1IN selection"]
pub type C1INSEL_R = crate::BitReader;
#[doc = "Field `C1INSEL` writer - C1IN selection"]
pub type C1INSEL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C1IOS` reader - Channel 1 idle output state"]
pub type C1IOS_R = crate::BitReader;
#[doc = "Field `C1IOS` writer - Channel 1 idle output state"]
pub type C1IOS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C1CIOS` reader - Channel 1 complementary idle output state"]
pub type C1CIOS_R = crate::BitReader;
#[doc = "Field `C1CIOS` writer - Channel 1 complementary idle output state"]
pub type C1CIOS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C2IOS` reader - Channel 2 idle output state"]
pub type C2IOS_R = crate::BitReader;
#[doc = "Field `C2IOS` writer - Channel 2 idle output state"]
pub type C2IOS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C2CIOS` reader - Channel 2 complementary idle output state"]
pub type C2CIOS_R = crate::BitReader;
#[doc = "Field `C2CIOS` writer - Channel 2 complementary idle output state"]
pub type C2CIOS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C3IOS` reader - Channel 3 idle output state"]
pub type C3IOS_R = crate::BitReader;
#[doc = "Field `C3IOS` writer - Channel 3 idle output state"]
pub type C3IOS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C3CIOS` reader - Channel 3 complementary idle output state"]
pub type C3CIOS_R = crate::BitReader;
#[doc = "Field `C3CIOS` writer - Channel 3 complementary idle output state"]
pub type C3CIOS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C4IOS` reader - Channel 4 idle output state"]
pub type C4IOS_R = crate::BitReader;
#[doc = "Field `C4IOS` writer - Channel 4 idle output state"]
pub type C4IOS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TRGOUT2EN` reader - TRGOUT2 enable"]
pub type TRGOUT2EN_R = crate::BitReader;
#[doc = "Field `TRGOUT2EN` writer - TRGOUT2 enable"]
pub type TRGOUT2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Channel buffer control"]
    #[inline(always)]
    pub fn cbctrl(&self) -> CBCTRL_R {
        CBCTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Channel control bit flash select"]
    #[inline(always)]
    pub fn ccfs(&self) -> CCFS_R {
        CCFS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA request source"]
    #[inline(always)]
    pub fn drs(&self) -> DRS_R {
        DRS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Primary TMR output selection"]
    #[inline(always)]
    pub fn ptos(&self) -> PTOS_R {
        PTOS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - C1IN selection"]
    #[inline(always)]
    pub fn c1insel(&self) -> C1INSEL_R {
        C1INSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 1 idle output state"]
    #[inline(always)]
    pub fn c1ios(&self) -> C1IOS_R {
        C1IOS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 1 complementary idle output state"]
    #[inline(always)]
    pub fn c1cios(&self) -> C1CIOS_R {
        C1CIOS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 2 idle output state"]
    #[inline(always)]
    pub fn c2ios(&self) -> C2IOS_R {
        C2IOS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 2 complementary idle output state"]
    #[inline(always)]
    pub fn c2cios(&self) -> C2CIOS_R {
        C2CIOS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 3 idle output state"]
    #[inline(always)]
    pub fn c3ios(&self) -> C3IOS_R {
        C3IOS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 3 complementary idle output state"]
    #[inline(always)]
    pub fn c3cios(&self) -> C3CIOS_R {
        C3CIOS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 4 idle output state"]
    #[inline(always)]
    pub fn c4ios(&self) -> C4IOS_R {
        C4IOS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 31 - TRGOUT2 enable"]
    #[inline(always)]
    pub fn trgout2en(&self) -> TRGOUT2EN_R {
        TRGOUT2EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel buffer control"]
    #[inline(always)]
    #[must_use]
    pub fn cbctrl(&mut self) -> CBCTRL_W<CTRL2_SPEC, 0> {
        CBCTRL_W::new(self)
    }
    #[doc = "Bit 2 - Channel control bit flash select"]
    #[inline(always)]
    #[must_use]
    pub fn ccfs(&mut self) -> CCFS_W<CTRL2_SPEC, 2> {
        CCFS_W::new(self)
    }
    #[doc = "Bit 3 - DMA request source"]
    #[inline(always)]
    #[must_use]
    pub fn drs(&mut self) -> DRS_W<CTRL2_SPEC, 3> {
        DRS_W::new(self)
    }
    #[doc = "Bits 4:6 - Primary TMR output selection"]
    #[inline(always)]
    #[must_use]
    pub fn ptos(&mut self) -> PTOS_W<CTRL2_SPEC, 4> {
        PTOS_W::new(self)
    }
    #[doc = "Bit 7 - C1IN selection"]
    #[inline(always)]
    #[must_use]
    pub fn c1insel(&mut self) -> C1INSEL_W<CTRL2_SPEC, 7> {
        C1INSEL_W::new(self)
    }
    #[doc = "Bit 8 - Channel 1 idle output state"]
    #[inline(always)]
    #[must_use]
    pub fn c1ios(&mut self) -> C1IOS_W<CTRL2_SPEC, 8> {
        C1IOS_W::new(self)
    }
    #[doc = "Bit 9 - Channel 1 complementary idle output state"]
    #[inline(always)]
    #[must_use]
    pub fn c1cios(&mut self) -> C1CIOS_W<CTRL2_SPEC, 9> {
        C1CIOS_W::new(self)
    }
    #[doc = "Bit 10 - Channel 2 idle output state"]
    #[inline(always)]
    #[must_use]
    pub fn c2ios(&mut self) -> C2IOS_W<CTRL2_SPEC, 10> {
        C2IOS_W::new(self)
    }
    #[doc = "Bit 11 - Channel 2 complementary idle output state"]
    #[inline(always)]
    #[must_use]
    pub fn c2cios(&mut self) -> C2CIOS_W<CTRL2_SPEC, 11> {
        C2CIOS_W::new(self)
    }
    #[doc = "Bit 12 - Channel 3 idle output state"]
    #[inline(always)]
    #[must_use]
    pub fn c3ios(&mut self) -> C3IOS_W<CTRL2_SPEC, 12> {
        C3IOS_W::new(self)
    }
    #[doc = "Bit 13 - Channel 3 complementary idle output state"]
    #[inline(always)]
    #[must_use]
    pub fn c3cios(&mut self) -> C3CIOS_W<CTRL2_SPEC, 13> {
        C3CIOS_W::new(self)
    }
    #[doc = "Bit 14 - Channel 4 idle output state"]
    #[inline(always)]
    #[must_use]
    pub fn c4ios(&mut self) -> C4IOS_W<CTRL2_SPEC, 14> {
        C4IOS_W::new(self)
    }
    #[doc = "Bit 31 - TRGOUT2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgout2en(&mut self) -> TRGOUT2EN_W<CTRL2_SPEC, 31> {
        TRGOUT2EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}