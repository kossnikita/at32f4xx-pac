#[doc = "Register `CCTRL` reader"]
pub type R = crate::R<CCTRL_SPEC>;
#[doc = "Register `CCTRL` writer"]
pub type W = crate::W<CCTRL_SPEC>;
#[doc = "Field `C1EN` reader - Channel 1 enable"]
pub type C1EN_R = crate::BitReader;
#[doc = "Field `C1EN` writer - Channel 1 enable"]
pub type C1EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C1P` reader - Channel 1 Polarity"]
pub type C1P_R = crate::BitReader;
#[doc = "Field `C1P` writer - Channel 1 Polarity"]
pub type C1P_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C1CEN` reader - Channel 1 complementary enable"]
pub type C1CEN_R = crate::BitReader;
#[doc = "Field `C1CEN` writer - Channel 1 complementary enable"]
pub type C1CEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C1CP` reader - Channel 1 complementary polarity"]
pub type C1CP_R = crate::BitReader;
#[doc = "Field `C1CP` writer - Channel 1 complementary polarity"]
pub type C1CP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C2EN` reader - Channel 2 enable"]
pub type C2EN_R = crate::BitReader;
#[doc = "Field `C2EN` writer - Channel 2 enable"]
pub type C2EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C2P` reader - Channel 2 Polarity"]
pub type C2P_R = crate::BitReader;
#[doc = "Field `C2P` writer - Channel 2 Polarity"]
pub type C2P_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C2CEN` reader - Channel 2 complementary enable"]
pub type C2CEN_R = crate::BitReader;
#[doc = "Field `C2CEN` writer - Channel 2 complementary enable"]
pub type C2CEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C2CP` reader - Channel 2 complementary polarity"]
pub type C2CP_R = crate::BitReader;
#[doc = "Field `C2CP` writer - Channel 2 complementary polarity"]
pub type C2CP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C3EN` reader - Channel 3 enable"]
pub type C3EN_R = crate::BitReader;
#[doc = "Field `C3EN` writer - Channel 3 enable"]
pub type C3EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C3P` reader - Channel 3 Polarity"]
pub type C3P_R = crate::BitReader;
#[doc = "Field `C3P` writer - Channel 3 Polarity"]
pub type C3P_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C3CEN` reader - Channel 3 complementary enable"]
pub type C3CEN_R = crate::BitReader;
#[doc = "Field `C3CEN` writer - Channel 3 complementary enable"]
pub type C3CEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C3CP` reader - Channel 3 complementary polarity"]
pub type C3CP_R = crate::BitReader;
#[doc = "Field `C3CP` writer - Channel 3 complementary polarity"]
pub type C3CP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C4EN` reader - Channel 4 enable"]
pub type C4EN_R = crate::BitReader;
#[doc = "Field `C4EN` writer - Channel 4 enable"]
pub type C4EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `C4P` reader - Channel 4 Polarity"]
pub type C4P_R = crate::BitReader;
#[doc = "Field `C4P` writer - Channel 4 Polarity"]
pub type C4P_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Channel 1 enable"]
    #[inline(always)]
    pub fn c1en(&self) -> C1EN_R {
        C1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Polarity"]
    #[inline(always)]
    pub fn c1p(&self) -> C1P_R {
        C1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 1 complementary enable"]
    #[inline(always)]
    pub fn c1cen(&self) -> C1CEN_R {
        C1CEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 1 complementary polarity"]
    #[inline(always)]
    pub fn c1cp(&self) -> C1CP_R {
        C1CP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 2 enable"]
    #[inline(always)]
    pub fn c2en(&self) -> C2EN_R {
        C2EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 2 Polarity"]
    #[inline(always)]
    pub fn c2p(&self) -> C2P_R {
        C2P_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 2 complementary enable"]
    #[inline(always)]
    pub fn c2cen(&self) -> C2CEN_R {
        C2CEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 2 complementary polarity"]
    #[inline(always)]
    pub fn c2cp(&self) -> C2CP_R {
        C2CP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 3 enable"]
    #[inline(always)]
    pub fn c3en(&self) -> C3EN_R {
        C3EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 3 Polarity"]
    #[inline(always)]
    pub fn c3p(&self) -> C3P_R {
        C3P_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 3 complementary enable"]
    #[inline(always)]
    pub fn c3cen(&self) -> C3CEN_R {
        C3CEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 complementary polarity"]
    #[inline(always)]
    pub fn c3cp(&self) -> C3CP_R {
        C3CP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 4 enable"]
    #[inline(always)]
    pub fn c4en(&self) -> C4EN_R {
        C4EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 4 Polarity"]
    #[inline(always)]
    pub fn c4p(&self) -> C4P_R {
        C4P_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn c1en(&mut self) -> C1EN_W<CCTRL_SPEC, 0> {
        C1EN_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn c1p(&mut self) -> C1P_W<CCTRL_SPEC, 1> {
        C1P_W::new(self)
    }
    #[doc = "Bit 2 - Channel 1 complementary enable"]
    #[inline(always)]
    #[must_use]
    pub fn c1cen(&mut self) -> C1CEN_W<CCTRL_SPEC, 2> {
        C1CEN_W::new(self)
    }
    #[doc = "Bit 3 - Channel 1 complementary polarity"]
    #[inline(always)]
    #[must_use]
    pub fn c1cp(&mut self) -> C1CP_W<CCTRL_SPEC, 3> {
        C1CP_W::new(self)
    }
    #[doc = "Bit 4 - Channel 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn c2en(&mut self) -> C2EN_W<CCTRL_SPEC, 4> {
        C2EN_W::new(self)
    }
    #[doc = "Bit 5 - Channel 2 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn c2p(&mut self) -> C2P_W<CCTRL_SPEC, 5> {
        C2P_W::new(self)
    }
    #[doc = "Bit 6 - Channel 2 complementary enable"]
    #[inline(always)]
    #[must_use]
    pub fn c2cen(&mut self) -> C2CEN_W<CCTRL_SPEC, 6> {
        C2CEN_W::new(self)
    }
    #[doc = "Bit 7 - Channel 2 complementary polarity"]
    #[inline(always)]
    #[must_use]
    pub fn c2cp(&mut self) -> C2CP_W<CCTRL_SPEC, 7> {
        C2CP_W::new(self)
    }
    #[doc = "Bit 8 - Channel 3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn c3en(&mut self) -> C3EN_W<CCTRL_SPEC, 8> {
        C3EN_W::new(self)
    }
    #[doc = "Bit 9 - Channel 3 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn c3p(&mut self) -> C3P_W<CCTRL_SPEC, 9> {
        C3P_W::new(self)
    }
    #[doc = "Bit 10 - Channel 3 complementary enable"]
    #[inline(always)]
    #[must_use]
    pub fn c3cen(&mut self) -> C3CEN_W<CCTRL_SPEC, 10> {
        C3CEN_W::new(self)
    }
    #[doc = "Bit 11 - Channel 3 complementary polarity"]
    #[inline(always)]
    #[must_use]
    pub fn c3cp(&mut self) -> C3CP_W<CCTRL_SPEC, 11> {
        C3CP_W::new(self)
    }
    #[doc = "Bit 12 - Channel 4 enable"]
    #[inline(always)]
    #[must_use]
    pub fn c4en(&mut self) -> C4EN_W<CCTRL_SPEC, 12> {
        C4EN_W::new(self)
    }
    #[doc = "Bit 13 - Channel 4 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn c4p(&mut self) -> C4P_W<CCTRL_SPEC, 13> {
        C4P_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCTRL_SPEC;
impl crate::RegisterSpec for CCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cctrl::R`](R) reader structure"]
impl crate::Readable for CCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cctrl::W`](W) writer structure"]
impl crate::Writable for CCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCTRL to value 0"]
impl crate::Resettable for CCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
